use std::cell::Cell;
use std::collections::BTreeMap;
use std::fs;
use std::io::ErrorKind;
use std::os::unix::fs::{PermissionsExt, symlink};
use std::path::PathBuf;
use std::rc::Rc;

use bitbook_wallet_broker::store::{
    EntryInfo, EntryKind, FULL_DIRECTORY_ROLLBACK_RESIDUAL, FaultPoint, LinuxStorePort,
    RestoreCandidate, RestoreContext, RestoreDecision, StoreError, StorePort, VaultStore,
    evaluate_restore, next_epoch,
};
use bitbook_wallet_broker::vault::{EntropyPort, VaultError};

const ROOT: &str = "target/wal004-scratch";
const ACCOUNT: &str = "00112233445566778899aabbccddeeff";
const SECOND_ACCOUNT: &str = "ffeeddccbbaa99887766554433221100";
const OLD: &[u8] = b"{\"synthetic\":\"old-authenticated-ciphertext\"}\n";
const NEW: &[u8] = b"{\"synthetic\":\"new-authenticated-ciphertext\"}\n";
const SECRET_CANARY: &[u8] = b"CANARY_WAL004_PLAINTEXT_NEVER_STAGED";

#[derive(Default)]
struct Entropy(u8);

impl EntropyPort for Entropy {
    fn fill(&mut self, label: &'static str, output: &mut [u8]) -> Result<(), VaultError> {
        assert_eq!(label, "staging-name");
        self.0 = self.0.wrapping_add(1);
        output.fill(self.0);
        Ok(())
    }
}

#[derive(Clone, Default)]
struct FaultState(Rc<Cell<Option<FaultPoint>>>);

impl FaultState {
    fn armed(point: FaultPoint) -> Self {
        Self(Rc::new(Cell::new(Some(point))))
    }

    fn set(&self, point: FaultPoint) {
        self.0.set(Some(point));
    }
}

#[derive(Default)]
struct FakePort {
    entries: BTreeMap<String, (EntryInfo, Vec<u8>)>,
    calls: Vec<String>,
    fail: FaultState,
    held: Vec<String>,
}

impl FakePort {
    fn record(&mut self, point: FaultPoint, call: String) -> Result<(), StoreError> {
        self.calls.push(call);
        if self.fail.0.get() == Some(point) {
            return Err(StoreError::injected(point));
        }
        Ok(())
    }

    fn active_path() -> String {
        format!("{ROOT}/{ACCOUNT}.vault")
    }

    fn install(&mut self, path: &str, kind: EntryKind, mode: u32, bytes: &[u8]) {
        self.entries.insert(
            path.to_owned(),
            (
                EntryInfo {
                    kind,
                    mode,
                    len: bytes.len() as u64,
                },
                bytes.to_vec(),
            ),
        );
    }
}

impl StorePort for FakePort {
    fn ensure_directory(&mut self, path: &str, mode: u32) -> Result<(), StoreError> {
        self.record(FaultPoint::Directory, format!("directory:{path}:{mode:o}"))
    }

    fn acquire_account(&mut self, account_id: &str) -> Result<(), StoreError> {
        self.record(FaultPoint::Acquire, format!("acquire:{account_id}"))?;
        if self.held.iter().any(|held| held == account_id) {
            return Err(StoreError::account_busy());
        }
        self.held.push(account_id.to_owned());
        Ok(())
    }

    fn release_account(&mut self, account_id: &str) {
        self.calls.push(format!("release:{account_id}"));
        self.held.retain(|held| held != account_id);
    }

    fn inspect(&mut self, path: &str) -> Result<EntryInfo, StoreError> {
        self.record(FaultPoint::Inspect, format!("inspect:{path}"))?;
        self.entries
            .get(path)
            .map(|entry| entry.0)
            .ok_or_else(StoreError::not_found)
    }

    fn read_bounded(&mut self, path: &str, maximum: usize) -> Result<Vec<u8>, StoreError> {
        self.record(FaultPoint::Read, format!("read:{path}:{maximum}"))?;
        let (_, bytes) = self.entries.get(path).ok_or_else(StoreError::not_found)?;
        if bytes.len() > maximum {
            return Err(StoreError::limit());
        }
        Ok(bytes.clone())
    }

    fn create_exclusive(&mut self, path: &str, mode: u32) -> Result<(), StoreError> {
        self.record(FaultPoint::Create, format!("create:{path}:{mode:o}"))?;
        if self.entries.contains_key(path) {
            return Err(StoreError::already_exists());
        }
        self.install(path, EntryKind::Regular, mode, &[]);
        Ok(())
    }

    fn write_all(&mut self, path: &str, bytes: &[u8]) -> Result<(), StoreError> {
        self.record(FaultPoint::Write, format!("write:{path}:{}", bytes.len()))?;
        assert!(
            !bytes
                .windows(SECRET_CANARY.len())
                .any(|part| part == SECRET_CANARY)
        );
        let entry = self
            .entries
            .get_mut(path)
            .ok_or_else(StoreError::not_found)?;
        entry.1 = bytes.to_vec();
        entry.0.len = bytes.len() as u64;
        Ok(())
    }

    fn set_permissions(&mut self, path: &str, mode: u32) -> Result<(), StoreError> {
        self.record(
            FaultPoint::Permission,
            format!("permission:{path}:{mode:o}"),
        )?;
        self.entries
            .get_mut(path)
            .ok_or_else(StoreError::not_found)?
            .0
            .mode = mode;
        Ok(())
    }

    fn sync_file(&mut self, path: &str) -> Result<(), StoreError> {
        self.record(FaultPoint::FileSync, format!("file-sync:{path}"))
    }

    fn replace_atomic(&mut self, staging: &str, active: &str) -> Result<(), StoreError> {
        self.record(FaultPoint::Replace, format!("replace:{staging}:{active}"))?;
        let staged = self
            .entries
            .remove(staging)
            .ok_or_else(StoreError::not_found)?;
        self.entries.insert(active.to_owned(), staged);
        Ok(())
    }

    fn sync_directory(&mut self, path: &str) -> Result<(), StoreError> {
        self.record(FaultPoint::DirectorySync, format!("directory-sync:{path}"))
    }
}

fn store(port: FakePort) -> VaultStore<FakePort> {
    VaultStore::new(ROOT, port).unwrap()
}

#[test]
fn private_directory_and_active_file_modes_are_exact() {
    let mut store = store(FakePort::default());
    store.initialize().unwrap();
    store
        .write_active(ACCOUNT, NEW, &mut Entropy::default())
        .unwrap();
    assert!(
        store
            .port()
            .calls
            .iter()
            .any(|call| call == "directory:target/wal004-scratch:700")
    );
    assert!(store.port().calls.iter().any(|call| call.contains(":600")));
    let active = store.port().entries.get(&FakePort::active_path()).unwrap();
    assert_eq!(active.0.mode, 0o600);
    assert_eq!(active.1, NEW);
}

#[test]
fn write_order_is_exclusive_complete_synced_atomic_and_directory_synced() {
    let mut store = store(FakePort::default());
    store.initialize().unwrap();
    store
        .write_active(ACCOUNT, NEW, &mut Entropy::default())
        .unwrap();
    let calls = &store.port().calls;
    let positions = [
        "create:",
        "permission:",
        "write:",
        "file-sync:",
        "replace:",
        "directory-sync:",
    ]
    .map(|prefix| {
        calls
            .iter()
            .position(|call| call.starts_with(prefix))
            .unwrap()
    });
    assert!(positions.windows(2).all(|pair| pair[0] < pair[1]));
    assert!(calls.last().unwrap().starts_with("release:"));
}

#[test]
fn every_write_fault_reports_failure_and_never_installs_partial_plaintext() {
    for point in [
        FaultPoint::Create,
        FaultPoint::Permission,
        FaultPoint::Write,
        FaultPoint::FileSync,
        FaultPoint::Replace,
    ] {
        let mut port = FakePort {
            fail: FaultState::armed(point),
            ..FakePort::default()
        };
        port.install(&FakePort::active_path(), EntryKind::Regular, 0o600, OLD);
        let mut store = store(port);
        let result = store.write_active(ACCOUNT, NEW, &mut Entropy::default());
        assert!(result.is_err(), "{point:?} incorrectly reported success");
        assert_eq!(
            store
                .port()
                .entries
                .get(&FakePort::active_path())
                .unwrap()
                .1,
            OLD,
            "{point:?} changed the active bytes before replacement"
        );
        assert!(!store.port().held.iter().any(|held| held == ACCOUNT));
        assert!(
            store
                .port()
                .calls
                .iter()
                .any(|call| call == &format!("release:{ACCOUNT}"))
        );
        for (_, bytes) in store.port().entries.values() {
            assert!(
                !bytes
                    .windows(SECRET_CANARY.len())
                    .any(|part| part == SECRET_CANARY)
            );
        }
    }
}

#[test]
fn failure_during_recovery_remains_fail_closed_with_recoverable_staging() {
    let staging = format!("{ROOT}/.{ACCOUNT}.0101010101010101.stage");
    let fault = FaultState::armed(FaultPoint::FileSync);
    let mut port = FakePort {
        fail: fault.clone(),
        ..FakePort::default()
    };
    port.install(&FakePort::active_path(), EntryKind::Regular, 0o600, OLD);
    let mut store = store(port);
    assert!(
        store
            .write_active(ACCOUNT, NEW, &mut Entropy::default())
            .is_err()
    );
    assert_eq!(
        store
            .port()
            .entries
            .get(&FakePort::active_path())
            .unwrap()
            .1,
        OLD
    );
    assert_eq!(store.port().entries.get(&staging).unwrap().1, NEW);
    fault.set(FaultPoint::Replace);
    assert!(store.recover_account(ACCOUNT).is_err());
    assert_eq!(
        store
            .port()
            .entries
            .get(&FakePort::active_path())
            .unwrap()
            .1,
        OLD
    );
    assert_eq!(store.port().entries.get(&staging).unwrap().1, NEW);
    assert!(!store.port().held.iter().any(|held| held == ACCOUNT));
}

#[test]
fn directory_sync_failure_reports_failure_with_one_complete_active_value() {
    let mut port = FakePort {
        fail: FaultState::armed(FaultPoint::DirectorySync),
        ..FakePort::default()
    };
    port.install(&FakePort::active_path(), EntryKind::Regular, 0o600, OLD);
    let mut store = store(port);
    assert!(
        store
            .write_active(ACCOUNT, NEW, &mut Entropy::default())
            .is_err()
    );
    let active = &store
        .port()
        .entries
        .get(&FakePort::active_path())
        .unwrap()
        .1;
    assert!(active.as_slice() == OLD || active.as_slice() == NEW);
    assert!(!active.is_empty());
    assert!(!store.port().held.iter().any(|held| held == ACCOUNT));
    assert!(
        store
            .port()
            .calls
            .iter()
            .any(|call| call == &format!("release:{ACCOUNT}"))
    );
}

#[test]
fn reader_rejects_symlink_fifo_directory_device_and_oversize_before_allocation() {
    for kind in [
        EntryKind::Symlink,
        EntryKind::Fifo,
        EntryKind::Directory,
        EntryKind::BlockDevice,
        EntryKind::CharacterDevice,
    ] {
        let mut port = FakePort::default();
        port.install(&FakePort::active_path(), kind, 0o600, OLD);
        let mut store = store(port);
        assert_eq!(
            store.read_active(ACCOUNT).unwrap_err().code(),
            "UNAVAILABLE"
        );
        assert!(
            !store
                .port()
                .calls
                .iter()
                .any(|call| call.starts_with("read:"))
        );
    }
    let mut port = FakePort::default();
    port.install(
        &FakePort::active_path(),
        EntryKind::Regular,
        0o600,
        &vec![0u8; 128 * 1024 + 1],
    );
    let mut store = store(port);
    assert_eq!(store.read_active(ACCOUNT).unwrap_err().code(), "LIMIT");
    assert!(
        !store
            .port()
            .calls
            .iter()
            .any(|call| call.starts_with("read:"))
    );
}

#[test]
fn active_filename_is_derived_only_from_validated_account_id() {
    for invalid in [
        "",
        "../00112233445566778899aabbccddeeff",
        "00112233445566778899AABBCCDDEEFF",
        "00112233445566778899aabbccddeefg",
        "00112233445566778899aabbccddee",
        "/target/wal004-scratch/account",
    ] {
        let mut store = store(FakePort::default());
        assert_eq!(store.read_active(invalid).unwrap_err().code(), "SCHEMA");
        assert!(store.port().calls.is_empty());
    }
}

#[test]
fn staging_collision_is_not_reused_or_truncated() {
    let mut port = FakePort::default();
    let collision = format!("{ROOT}/.{ACCOUNT}.0101010101010101.stage");
    port.install(&collision, EntryKind::Regular, 0o600, OLD);
    let mut store = store(port);
    assert_eq!(
        store
            .write_active(ACCOUNT, NEW, &mut Entropy::default())
            .unwrap_err()
            .code(),
        "ACCOUNT_BUSY"
    );
    assert_eq!(store.port().entries.get(&collision).unwrap().1, OLD);
}

#[test]
fn concurrent_update_of_one_account_is_busy_but_other_accounts_are_isolated() {
    let mut port = FakePort::default();
    port.held.push(ACCOUNT.to_owned());
    let mut store = store(port);
    assert_eq!(
        store
            .write_active(ACCOUNT, NEW, &mut Entropy::default())
            .unwrap_err()
            .code(),
        "ACCOUNT_BUSY"
    );
    assert!(
        store
            .write_active(
                "ffeeddccbbaa99887766554433221100",
                NEW,
                &mut Entropy::default()
            )
            .is_ok()
    );
}

#[test]
fn export_is_ciphertext_only_exclusive_and_never_self_overwrites() {
    let mut port = FakePort::default();
    port.install(&FakePort::active_path(), EntryKind::Regular, 0o600, NEW);
    let mut store = store(port);
    store
        .export_encrypted(ACCOUNT, "target/wal004-scratch/export.vault")
        .unwrap();
    assert_eq!(
        store
            .port()
            .entries
            .get("target/wal004-scratch/export.vault")
            .unwrap()
            .1,
        NEW
    );
    assert!(!store.port().entries.values().any(|entry| {
        entry
            .1
            .windows(SECRET_CANARY.len())
            .any(|part| part == SECRET_CANARY)
    }));
    assert_eq!(
        store
            .export_encrypted(ACCOUNT, &FakePort::active_path())
            .unwrap_err()
            .code(),
        "SCHEMA"
    );
}

#[test]
fn export_rejects_existing_and_non_regular_destinations() {
    for kind in [
        EntryKind::Regular,
        EntryKind::Symlink,
        EntryKind::Directory,
        EntryKind::Fifo,
        EntryKind::BlockDevice,
        EntryKind::CharacterDevice,
    ] {
        let destination = "target/wal004-scratch/existing.vault";
        let mut port = FakePort::default();
        port.install(&FakePort::active_path(), EntryKind::Regular, 0o600, NEW);
        port.install(destination, kind, 0o600, OLD);
        let mut store = store(port);
        assert!(store.export_encrypted(ACCOUNT, destination).is_err());
        assert_eq!(store.port().entries.get(destination).unwrap().1, OLD);
    }
}

#[test]
fn directory_and_file_permission_mismatch_fail_before_read_or_replace() {
    let mut port = FakePort::default();
    port.install(&FakePort::active_path(), EntryKind::Regular, 0o644, OLD);
    let mut store = store(port);
    assert_eq!(
        store.read_active(ACCOUNT).unwrap_err().code(),
        "UNAVAILABLE"
    );
    assert!(
        !store
            .port()
            .calls
            .iter()
            .any(|call| call.starts_with("read:"))
    );
}

#[test]
fn linux_store_enforces_real_modes_regular_files_and_symlink_rejection() {
    let root = PathBuf::from(format!("target/wal004-scratch/os-{}", std::process::id()));
    match fs::symlink_metadata(&root) {
        Err(error) if error.kind() == ErrorKind::NotFound => {}
        Ok(_) => panic!(
            "refusing to reuse stale WAL-004 OS boundary root: {}",
            root.display()
        ),
        Err(error) => panic!("cannot inspect WAL-004 OS boundary root: {error}"),
    }

    let active_path = root.join(format!("{ACCOUNT}.vault"));
    let symlink_path = root.join(format!("{SECOND_ACCOUNT}.vault"));
    let mut store = VaultStore::new(root.clone(), LinuxStorePort::new()).unwrap();
    store.initialize().unwrap();
    store
        .write_active(ACCOUNT, NEW, &mut Entropy::default())
        .unwrap();

    let directory = fs::symlink_metadata(&root).unwrap();
    assert!(directory.file_type().is_dir());
    assert_eq!(directory.permissions().mode() & 0o777, 0o700);

    let active = fs::symlink_metadata(&active_path).unwrap();
    assert!(active.file_type().is_file());
    assert!(!active.file_type().is_symlink());
    assert_eq!(active.permissions().mode() & 0o777, 0o600);

    symlink(active_path.file_name().unwrap(), &symlink_path).unwrap();
    assert!(
        fs::symlink_metadata(&symlink_path)
            .unwrap()
            .file_type()
            .is_symlink()
    );
    assert_eq!(
        store.read_active(SECOND_ACCOUNT).unwrap_err().code(),
        "UNAVAILABLE"
    );

    drop(store);
    let symlink_text = symlink_path.to_str().unwrap();
    let mut direct = LinuxStorePort::new();
    let read_result = direct.read_bounded(symlink_text, 128 * 1024);
    let write_result = direct.write_all(symlink_text, OLD);
    let permission_result = direct.set_permissions(symlink_text, 0o644);
    let sync_result = direct.sync_file(symlink_text);
    let target_bytes = fs::read(&active_path).unwrap();
    let target_mode = fs::symlink_metadata(&active_path)
        .unwrap()
        .permissions()
        .mode()
        & 0o777;
    fs::remove_file(&symlink_path).unwrap();
    fs::remove_file(&active_path).unwrap();
    fs::remove_dir(&root).unwrap();

    for result in [
        read_result.map(|_| ()),
        write_result,
        permission_result,
        sync_result,
    ] {
        assert_eq!(result.unwrap_err().code(), "UNAVAILABLE");
    }
    assert_eq!(target_bytes, NEW);
    assert_eq!(target_mode, 0o600);
}

#[test]
fn linux_direct_operations_reject_wrong_mode_until_descriptor_repair() {
    let root = PathBuf::from(format!("target/wal004-scratch/mode-{}", std::process::id()));
    match fs::symlink_metadata(&root) {
        Err(error) if error.kind() == ErrorKind::NotFound => {}
        Ok(_) => panic!(
            "refusing to reuse stale WAL-004 mode root: {}",
            root.display()
        ),
        Err(error) => panic!("cannot inspect WAL-004 mode root: {error}"),
    }

    fs::create_dir_all(&root).unwrap();
    fs::set_permissions(&root, fs::Permissions::from_mode(0o700)).unwrap();
    let path = root.join("wrong-mode.vault");
    fs::write(&path, NEW).unwrap();
    fs::set_permissions(&path, fs::Permissions::from_mode(0o644)).unwrap();
    let path_text = path.to_str().unwrap();
    let mut direct = LinuxStorePort::new();

    let read_result = direct.read_bounded(path_text, 128 * 1024);
    let write_result = direct.write_all(path_text, OLD);
    let sync_result = direct.sync_file(path_text);
    let bytes_after_rejections = fs::read(&path).unwrap();
    let mode_after_rejections = fs::symlink_metadata(&path).unwrap().permissions().mode() & 0o777;
    let permission_result = direct.set_permissions(path_text, 0o600);
    let repaired_mode = fs::symlink_metadata(&path).unwrap().permissions().mode() & 0o777;

    fs::remove_file(&path).unwrap();
    fs::remove_dir(&root).unwrap();

    for result in [read_result.map(|_| ()), write_result, sync_result] {
        assert_eq!(result.unwrap_err().code(), "UNAVAILABLE");
    }
    assert_eq!(bytes_after_rejections, NEW);
    assert_eq!(mode_after_rejections, 0o644);
    permission_result.unwrap();
    assert_eq!(repaired_mode, 0o600);
}

fn candidate(epoch: u64) -> RestoreCandidate {
    RestoreCandidate {
        authenticated: true,
        account_id: ACCOUNT.to_owned(),
        asset: "ZEC".to_owned(),
        network: "zec-testnet".to_owned(),
        epoch,
    }
}

#[test]
fn restore_authenticates_before_metadata_or_confirmation_is_released() {
    let mut unauthenticated = candidate(9);
    unauthenticated.authenticated = false;
    let error = evaluate_restore(
        &RestoreContext::empty(ACCOUNT, "ZEC", "zec-testnet"),
        &unauthenticated,
        true,
    )
    .unwrap_err();
    assert_eq!(error.code(), "LOCKED");
    assert!(error.metadata().is_none());
    assert_eq!(error.public_message(), "Wallet locked");
}

#[test]
fn first_restore_and_higher_epoch_each_require_explicit_native_confirmation() {
    let first = RestoreContext::empty(ACCOUNT, "ZEC", "zec-testnet");
    assert_eq!(
        evaluate_restore(&first, &candidate(9), false).unwrap(),
        RestoreDecision::Cancelled
    );
    assert_eq!(
        evaluate_restore(&first, &candidate(9), true).unwrap(),
        RestoreDecision::Replace
    );

    let existing = RestoreContext::authenticated(ACCOUNT, "ZEC", "zec-testnet", 8);
    assert_eq!(
        evaluate_restore(&existing, &candidate(9), false).unwrap(),
        RestoreDecision::Cancelled
    );
    assert_eq!(
        evaluate_restore(&existing, &candidate(9), true).unwrap(),
        RestoreDecision::Replace
    );
}

#[test]
fn stale_and_equal_restore_epochs_are_refused_even_when_confirmed() {
    let existing = RestoreContext::authenticated(ACCOUNT, "ZEC", "zec-testnet", 9);
    for epoch in [1, 8, 9] {
        assert_eq!(
            evaluate_restore(&existing, &candidate(epoch), true)
                .unwrap_err()
                .code(),
            "REPLAY"
        );
    }
}

#[test]
fn restore_rejects_account_asset_network_mismatch_and_corrupt_current_state() {
    let existing = RestoreContext::authenticated(ACCOUNT, "ZEC", "zec-testnet", 8);
    for changed in [
        RestoreCandidate {
            account_id: "ffeeddccbbaa99887766554433221100".to_owned(),
            ..candidate(9)
        },
        RestoreCandidate {
            asset: "XMR".to_owned(),
            network: "xmr-stagenet".to_owned(),
            ..candidate(9)
        },
        RestoreCandidate {
            network: "zec-regtest".to_owned(),
            ..candidate(9)
        },
    ] {
        assert!(evaluate_restore(&existing, &changed, true).is_err());
    }
    let corrupt = RestoreContext::corrupt(ACCOUNT, "ZEC", "zec-testnet");
    assert_eq!(
        evaluate_restore(&corrupt, &candidate(9), true)
            .unwrap_err()
            .code(),
        "STATE_CORRUPT"
    );
}

#[test]
fn full_directory_rollback_is_explicitly_residual_not_claimed_solved() {
    assert_eq!(
        FULL_DIRECTORY_ROLLBACK_RESIDUAL,
        "A full rollback of the broker data directory can also roll back its local high-water record."
    );
}

#[test]
fn successful_replacement_epoch_is_checked_and_strictly_increments() {
    assert_eq!(next_epoch(None).unwrap(), 1);
    assert_eq!(next_epoch(Some(7)).unwrap(), 8);
    assert_eq!(
        next_epoch(Some(u64::MAX)).unwrap_err().code(),
        "STATE_CORRUPT"
    );
}
