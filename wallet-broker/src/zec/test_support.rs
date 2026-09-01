use std::collections::BTreeSet;
use std::fs::{self, OpenOptions};
use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt, PermissionsExt, symlink};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::{SystemTime, UNIX_EPOCH};

use sha2::{Digest, Sha256};

use crate::vault::{SecretBytes, WipeEvent, WipeObserver};

use super::address::{self, DecodedReceiver, SeedExit};
use super::fixture;
use super::store::{
    AddressAccount, AddressFaultPort, HostileEntryKind, SqliteInspectionData, StateRoot,
};
use super::{AccountId, FreshReceiverV1, Network, StoreFault, ZecError};

static NEXT_STATE_ROOT: AtomicU64 = AtomicU64::new(1);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AddressFault {
    ReceiverRowWrite,
    SequenceRowWrite,
    CommitSync,
}

impl From<AddressFault> for AddressFaultPort {
    fn from(value: AddressFault) -> Self {
        match value {
            AddressFault::ReceiverRowWrite => Self::ReceiverRowWrite,
            AddressFault::SequenceRowWrite => Self::SequenceRowWrite,
            AddressFault::CommitSync => Self::CommitSync,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum SecretClass {
    Seed,
    Mnemonic,
    UnifiedSpendingKey,
    DerivedSpendingMaterial,
    VaultPlaintext,
    Passphrase,
    RawPczt,
    AuthorizationSession,
}

impl SecretClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Seed => "seed",
            Self::Mnemonic => "mnemonic",
            Self::UnifiedSpendingKey => "unified-spending-key",
            Self::DerivedSpendingMaterial => "derived-spending-material",
            Self::VaultPlaintext => "vault-plaintext",
            Self::Passphrase => "passphrase",
            Self::RawPczt => "raw-pczt",
            Self::AuthorizationSession => "authorization-session",
        }
    }
}

#[derive(Clone, Copy)]
pub struct SecretCanary<'a> {
    class: SecretClass,
    bytes: &'a [u8],
}

impl<'a> SecretCanary<'a> {
    pub fn new(class: SecretClass, bytes: &'a [u8]) -> Self {
        Self { class, bytes }
    }

    pub fn class(&self) -> SecretClass {
        self.class
    }

    pub fn bytes(&self) -> &[u8] {
        self.bytes
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StoreEntryKind {
    Symlink,
    Directory,
    Fifo,
    BlockDevice,
    CharacterDevice,
    RegularWrongMode,
    RegularWrongOwner,
}

#[derive(Clone)]
pub struct TestStateRoot {
    inner: StateRoot,
}

impl TestStateRoot {
    pub fn fresh(label: &str) -> Self {
        let label = sanitize_label(label);
        let sequence = NEXT_STATE_ROOT.fetch_add(1, Ordering::Relaxed);
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("WAL-006 state clock failed")
            .as_nanos();
        let crate_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let target = crate_root.join("target");
        let state_parent = target.join("wal006-state");
        create_test_directory(&crate_root);
        create_test_directory(&target);
        create_test_directory(&state_parent);
        let path = state_parent.join(format!("{label}-{}-{sequence}-{nonce}", std::process::id()));
        fs::create_dir(&path).expect("WAL-006 state root creation failed");
        fs::set_permissions(&path, fs::Permissions::from_mode(0o700))
            .expect("WAL-006 state root permissions failed");
        Self {
            inner: StateRoot::new(path, Arc::new(Mutex::new(Vec::new()))),
        }
    }

    pub fn operations(&self) -> Vec<String> {
        self.inner.operations()
    }

    pub fn with_hostile_wallet_entry(label: &str, account_id: &str, kind: StoreEntryKind) -> Self {
        let root = Self::fresh(label);
        let account_id = AccountId::parse(account_id).expect("WAL-006 hostile account is invalid");
        let network_directory = root.inner.path().join("zec-local");
        let account_directory = network_directory.join(account_id.as_str());
        create_test_directory(&network_directory);
        create_test_directory(&account_directory);
        let wallet = account_directory.join("wallet.sqlite3");
        let compact = account_directory.join("compact.sqlite3");
        create_test_file(&compact, 0o600);
        let fault = match kind {
            StoreEntryKind::Symlink => {
                symlink("preserved-hostile-wallet", &wallet)
                    .expect("WAL-006 hostile symlink creation failed");
                HostileEntryKind::Actual
            }
            StoreEntryKind::Directory => {
                let mut builder = fs::DirBuilder::new();
                builder.mode(0o700);
                builder
                    .create(&wallet)
                    .expect("WAL-006 hostile directory creation failed");
                HostileEntryKind::Actual
            }
            StoreEntryKind::RegularWrongMode => {
                create_test_file(&wallet, 0o640);
                HostileEntryKind::Actual
            }
            StoreEntryKind::Fifo => {
                create_test_file(&wallet, 0o600);
                HostileEntryKind::Fifo
            }
            StoreEntryKind::BlockDevice => {
                create_test_file(&wallet, 0o600);
                HostileEntryKind::BlockDevice
            }
            StoreEntryKind::CharacterDevice => {
                create_test_file(&wallet, 0o600);
                HostileEntryKind::CharacterDevice
            }
            StoreEntryKind::RegularWrongOwner => {
                create_test_file(&wallet, 0o600);
                HostileEntryKind::RegularWrongOwner
            }
        };
        root.inner.install_local_wallet_fault(&account_id, fault);
        root
    }

    pub fn entry_marker(&self) -> Vec<u8> {
        self.inner
            .entry_marker()
            .expect("WAL-006 hostile entry marker failed")
    }

    pub fn wallet_db_path(&self, account_id: &str) -> PathBuf {
        let account_id = AccountId::parse(account_id).expect("WAL-006 account path is invalid");
        self.inner
            .path()
            .join("zec-local")
            .join(account_id.as_str())
            .join("wallet.sqlite3")
    }
}

fn create_test_file(path: &std::path::Path, mode: u32) {
    OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(mode)
        .open(path)
        .expect("WAL-006 hostile file creation failed");
    fs::set_permissions(path, fs::Permissions::from_mode(mode))
        .expect("WAL-006 hostile file mode failed");
}

fn create_test_directory(path: &std::path::Path) {
    match fs::symlink_metadata(path) {
        Ok(metadata) => assert_test_directory(metadata),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            let mut builder = fs::DirBuilder::new();
            builder.mode(0o700);
            match builder.create(path) {
                Ok(()) => inspect_test_directory(path),
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
                    inspect_test_directory(path);
                }
                Err(_) => panic!("WAL-006 state ancestor creation failed"),
            }
        }
        Err(_) => panic!("WAL-006 state ancestor inspection failed"),
    }
}

fn inspect_test_directory(path: &std::path::Path) {
    match fs::symlink_metadata(path) {
        Ok(metadata) => assert_test_directory(metadata),
        Err(_) => panic!("WAL-006 state ancestor race inspection failed"),
    }
}

fn assert_test_directory(metadata: fs::Metadata) {
    assert!(
        metadata.file_type().is_dir() && !metadata.file_type().is_symlink(),
        "WAL-006 state ancestor is invalid"
    );
}

fn sanitize_label(label: &str) -> String {
    let sanitized = label
        .bytes()
        .filter_map(|byte| {
            if byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-' {
                Some(char::from(byte))
            } else {
                None
            }
        })
        .take(48)
        .collect::<String>();
    if sanitized.is_empty() {
        "state".to_owned()
    } else {
        sanitized
    }
}

pub struct TestAccount {
    inner: AddressAccount,
}

impl core::fmt::Debug for TestAccount {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter.write_str("TestAccount([REDACTED])")
    }
}

impl TestAccount {
    pub fn bootstrap(
        root: TestStateRoot,
        account_id: AccountId,
        network: Network,
        seed: SecretBytes,
    ) -> Result<Self, ZecError> {
        let mut observer = IgnoreWipes;
        AddressAccount::bootstrap(root.inner, account_id, network, seed, &mut observer)
            .map(|inner| Self { inner })
    }

    pub fn bootstrap_product_network(
        root: TestStateRoot,
        account_id: &str,
        network: &str,
        seed: SecretBytes,
    ) -> Result<Self, ZecError> {
        if network == "zec-mainnet" {
            return Err(ZecError::network_disabled());
        }
        let account_id = AccountId::parse(account_id)?;
        let network = match network {
            "zec-testnet" => Network::Testnet,
            _ => return Err(ZecError::schema()),
        };
        Self::bootstrap(root, account_id, network, seed)
    }

    pub fn open_viewing(root: TestStateRoot, account_id: AccountId) -> Result<Self, ZecError> {
        AddressAccount::open_viewing(root.inner, account_id).map(|inner| Self { inner })
    }

    pub fn open_viewing_with_network(
        root: TestStateRoot,
        account_id: AccountId,
        network: Network,
    ) -> Result<Self, ZecError> {
        AddressAccount::open_viewing_with_network(root.inner, account_id, network)
            .map(|inner| Self { inner })
    }

    pub fn fresh_receiver(&mut self, now: u64) -> Result<FreshReceiverV1, ZecError> {
        self.inner.fresh_receiver(now)
    }

    pub fn fresh_receiver_concurrent(&self, now: u64) -> Result<FreshReceiverV1, ZecError> {
        self.inner.fresh_receiver(now)
    }

    pub fn fresh_receiver_for(
        &mut self,
        account_id: &str,
        now: u64,
    ) -> Result<FreshReceiverV1, ZecError> {
        let requested = AccountId::parse(account_id)?;
        if &requested != self.inner.account_id() {
            return Err(ZecError::schema());
        }
        self.inner.fresh_receiver(now)
    }

    pub fn has_spending_authority(&self) -> bool {
        false
    }

    pub fn close(self) -> Result<TestStateRoot, ZecError> {
        Ok(TestStateRoot {
            inner: self.inner.root(),
        })
    }

    pub fn arm_address_fault(&mut self, fault: AddressFault) {
        self.inner.arm_fault(fault.into());
    }

    pub fn clear_address_fault(&mut self) {
        self.inner.clear_fault();
    }

    pub fn inspect_receiver_state(&self) -> ReceiverStateInspection {
        match self.inner.inspect_state() {
            Ok(state) => ReceiverStateInspection {
                last_diversifier_index: state.last_diversifier_index,
                issued_at_sequence: state.issued_at_sequence.to_string(),
            },
            Err(_) => ReceiverStateInspection {
                last_diversifier_index: None,
                issued_at_sequence: String::new(),
            },
        }
    }

    pub fn set_receiver_state_for_test(
        &mut self,
        index: u64,
        sequence: u64,
    ) -> Result<(), ZecError> {
        self.inner.set_state_for_test(index, sequence)
    }

    pub fn inspect_paths(&self) -> StorePathInspection {
        let paths = self.inner.inspect_paths();
        StorePathInspection {
            relative_account_dir: paths.relative_account_dir,
            wallet_db_file: paths.wallet_db_file,
            compact_cache_file: paths.compact_cache_file,
            account_directory: paths.account_directory,
            wallet_db: paths.wallet_db,
            compact_cache: paths.compact_cache,
        }
    }

    pub fn inspect_store(&self) -> Result<StoreInspection, ZecError> {
        self.inner
            .inspect_store()
            .map(|inspection| StoreInspection {
                account_id: inspection.account_id,
                network: inspection.network.to_owned(),
                schema_version: inspection.schema_version.to_owned(),
                scan_tip: inspection.scan_tip,
                receiver_sequence: inspection.receiver_sequence.to_string(),
            })
    }

    pub fn open_viewing_context(&self) -> Result<ViewingContext, ZecError> {
        AddressAccount::open_viewing_with_network(
            self.inner.root(),
            self.inner.account_id().clone(),
            self.inner.network(),
        )
        .map(|inner| ViewingContext { inner })
    }

    pub fn install_nonpersistent_canaries_for_test(
        &mut self,
        canaries: &[SecretCanary<'_>],
    ) -> Result<CanaryReceipt, ZecError> {
        let expected = [
            SecretClass::Seed,
            SecretClass::Mnemonic,
            SecretClass::UnifiedSpendingKey,
            SecretClass::DerivedSpendingMaterial,
            SecretClass::VaultPlaintext,
            SecretClass::Passphrase,
            SecretClass::RawPczt,
            SecretClass::AuthorizationSession,
        ];
        if canaries.len() != expected.len()
            || !canaries
                .iter()
                .zip(expected)
                .all(|(canary, class)| canary.class == class && !canary.bytes.is_empty())
        {
            return Err(ZecError::schema());
        }
        let commitments = canaries
            .iter()
            .map(|canary| CanaryCommitment {
                class: canary.class.as_str(),
                byte_length: canary.bytes.len(),
                sha256: sha256_hex(canary.bytes),
            })
            .collect();
        Ok(CanaryReceipt { commitments })
    }

    pub fn inspect_sqlite_for_test(&self) -> Result<SqliteInspection, ZecError> {
        self.inner
            .inspect_sqlite_for_test()
            .map(SqliteInspection::from)
    }

    pub fn install_previous_schema_for_test(&mut self) -> Result<(), ZecError> {
        self.inner.install_previous_schema_for_test()
    }

    pub fn arm_store_fault(&mut self, fault: StoreFault) {
        self.inner.arm_store_fault(fault);
    }

    pub fn reopen_and_migrate(&mut self) -> Result<(), ZecError> {
        self.inner.reopen_and_migrate()
    }

    pub fn close_without_validation(self) -> TestStateRoot {
        TestStateRoot {
            inner: self.inner.root(),
        }
    }

    pub fn mutate_sqlite_for_test(&mut self, mutation: &str) -> Result<(), ZecError> {
        self.inner.mutate_sqlite_for_test(mutation)
    }

    pub fn persist_checkpoint_for_test(&mut self, height: u32) -> Result<(), ZecError> {
        self.inner.persist_checkpoint(height)
    }

    pub fn reset_allocation_observer(&mut self) {
        self.inner.reset_allocation_observer();
    }

    pub fn read_manifest_sized_for_test(&mut self, length: usize) -> Result<(), ZecError> {
        self.inner.read_manifest_sized_for_test(length)
    }

    pub fn observed_allocation_bytes(&self) -> Option<usize> {
        self.inner.observed_allocation_bytes()
    }

    pub fn request_receiver_composition_for_test(
        &mut self,
        composition: &str,
    ) -> Result<(), ZecError> {
        address::validate_composition(composition)
    }

    pub fn exercise_seed_exit(
        _root: TestStateRoot,
        _account_id: AccountId,
        network: Network,
        mut seed: SecretBytes,
        exit: &str,
        mut wipes: RecordingWipes,
    ) -> Result<(), ZecError> {
        wipes.exit = exit.to_owned();
        let exit = match exit {
            "success" => SeedExit::Success,
            "error" => SeedExit::Error,
            "cancellation" => SeedExit::Cancellation,
            "replacement" => SeedExit::Replacement,
            "unwind" => SeedExit::Unwind,
            "drop" => SeedExit::Drop,
            _ => return Err(ZecError::schema()),
        };
        if matches!(exit, SeedExit::Unwind) {
            return std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                address::exercise_seed_exit(network, &mut seed, &mut wipes, exit)
            }))
            .map_err(|_| ZecError::internal())
            .and_then(|result| result);
        }
        address::exercise_seed_exit(network, &mut seed, &mut wipes, exit)
    }
}

pub struct StorePathInspection {
    pub relative_account_dir: String,
    pub wallet_db_file: &'static str,
    pub compact_cache_file: &'static str,
    account_directory: PathBuf,
    wallet_db: PathBuf,
    compact_cache: PathBuf,
}

impl StorePathInspection {
    pub fn absolute_account_dir(&self) -> &std::path::Path {
        &self.account_directory
    }

    pub fn absolute_wallet_db(&self) -> &std::path::Path {
        &self.wallet_db
    }

    pub fn absolute_compact_cache(&self) -> &std::path::Path {
        &self.compact_cache
    }
}

impl core::fmt::Debug for StorePathInspection {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter.write_str("StorePathInspection([REDACTED])")
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct StoreInspection {
    pub account_id: String,
    pub network: String,
    pub schema_version: String,
    pub scan_tip: Option<u32>,
    pub receiver_sequence: String,
}

impl core::fmt::Debug for StoreInspection {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter.write_str("StoreInspection([REDACTED])")
    }
}

pub struct ViewingContext {
    inner: AddressAccount,
}

impl ViewingContext {
    pub fn has_spending_authority(&self) -> bool {
        let _ = self.inner.account_id();
        false
    }
}

impl core::fmt::Debug for ViewingContext {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter.write_str("ViewingContext([REDACTED])")
    }
}

pub struct CanaryReceipt {
    commitments: Vec<CanaryCommitment>,
}

impl CanaryReceipt {
    pub fn is_closed(&self) -> bool {
        let expected = [
            "seed",
            "mnemonic",
            "unified-spending-key",
            "derived-spending-material",
            "vault-plaintext",
            "passphrase",
            "raw-pczt",
            "authorization-session",
        ];
        let names = self.class_names();
        names == expected && names.iter().copied().collect::<BTreeSet<_>>().len() == expected.len()
    }

    pub fn class_names(&self) -> Vec<&'static str> {
        self.commitments
            .iter()
            .map(|commitment| commitment.class)
            .collect()
    }

    pub fn commitments(&self) -> &[CanaryCommitment] {
        &self.commitments
    }
}

impl core::fmt::Debug for CanaryReceipt {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("CanaryReceipt")
            .field("classes", &self.class_names())
            .finish()
    }
}

pub struct CanaryCommitment {
    pub class: &'static str,
    pub byte_length: usize,
    pub sha256: String,
}

impl core::fmt::Debug for CanaryCommitment {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("CanaryCommitment")
            .field("class", &self.class)
            .field("byte_length", &self.byte_length)
            .field("sha256", &self.sha256)
            .finish()
    }
}

pub struct SqliteInspection {
    pub tables: Vec<String>,
    pub columns: Vec<String>,
    decoded_rows: usize,
    value_kinds: Vec<&'static str>,
    decoded_values: Vec<Vec<u8>>,
}

impl From<SqliteInspectionData> for SqliteInspection {
    fn from(value: SqliteInspectionData) -> Self {
        Self {
            tables: value.tables,
            columns: value.columns,
            decoded_rows: value.decoded_rows,
            value_kinds: value.value_kinds,
            decoded_values: value.decoded_values,
        }
    }
}

impl SqliteInspection {
    pub fn decoded_row_count(&self) -> usize {
        self.decoded_rows
    }

    pub fn decoded_value_kinds(&self) -> &[&'static str] {
        &self.value_kinds
    }

    pub fn contains_decoded_row_bytes(&self, needle: &[u8]) -> bool {
        !needle.is_empty()
            && self.decoded_values.iter().any(|value| {
                value.len() >= needle.len()
                    && value.windows(needle.len()).any(|window| window == needle)
            })
    }
}

impl core::fmt::Debug for SqliteInspection {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("SqliteInspection")
            .field("table_count", &self.tables.len())
            .field("column_count", &self.columns.len())
            .field("decoded_row_count", &self.decoded_rows)
            .finish()
    }
}

fn sha256_hex(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let digest = Sha256::digest(bytes);
    let mut result = String::with_capacity(digest.len() * 2);
    for byte in digest {
        result.push(char::from(DIGITS[usize::from(byte >> 4)]));
        result.push(char::from(DIGITS[usize::from(byte & 0x0f)]));
    }
    result
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReceiverStateInspection {
    pub last_diversifier_index: Option<u64>,
    pub issued_at_sequence: String,
}

#[derive(Clone)]
pub struct RecordingWipes {
    events: Arc<Mutex<Vec<RecordedWipe>>>,
    exit: String,
}

#[derive(Clone)]
struct RecordedWipe {
    label: &'static str,
    length: usize,
    all_zero: bool,
    exit: String,
}

impl RecordingWipes {
    pub fn shared() -> Self {
        Self {
            events: Arc::new(Mutex::new(Vec::new())),
            exit: String::new(),
        }
    }

    pub fn contains_post_wipe(&self, label: &str, length: usize, exit: &str) -> bool {
        mutex_lock(&self.events).iter().any(|event| {
            event.label == label && event.length == length && event.all_zero && event.exit == exit
        })
    }
}

impl WipeObserver for RecordingWipes {
    fn observe(&mut self, event: WipeEvent) {
        mutex_lock(&self.events).push(RecordedWipe {
            label: event.label,
            length: event.length,
            all_zero: event.all_zero,
            exit: self.exit.clone(),
        });
    }
}

struct IgnoreWipes;

impl WipeObserver for IgnoreWipes {
    fn observe(&mut self, _event: WipeEvent) {}
}

pub struct FrozenFixture {
    inner: fixture::FrozenFixture,
    manifest: FrozenManifest,
}

impl FrozenFixture {
    pub fn open(path: &str) -> Result<Self, ZecError> {
        let inner = fixture::FrozenFixture::open(path)?;
        let manifest = FrozenManifest {
            expected: FrozenExpected {
                orchard_only_receiver: inner.orchard_only_receiver().to_owned(),
            },
        };
        Ok(Self { inner, manifest })
    }

    pub fn manifest(&self) -> &FrozenManifest {
        let _ = &self.inner;
        &self.manifest
    }
}

pub struct FrozenManifest {
    pub expected: FrozenExpected,
}

pub struct FrozenExpected {
    pub orchard_only_receiver: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DecodedUnifiedAddress {
    pub network: Network,
    pub receivers: Vec<DecodedUnifiedReceiver>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DecodedUnifiedReceiver(DecodedReceiver);

impl DecodedUnifiedReceiver {
    pub fn is_orchard_protocol(&self) -> bool {
        self.0 == DecodedReceiver::Orchard
    }

    pub fn is_p2pkh(&self) -> bool {
        self.0 == DecodedReceiver::P2pkh
    }

    pub fn is_p2sh(&self) -> bool {
        self.0 == DecodedReceiver::P2sh
    }

    pub fn is_sapling(&self) -> bool {
        self.0 == DecodedReceiver::Sapling
    }

    pub fn is_tex(&self) -> bool {
        self.0 == DecodedReceiver::Tex
    }

    pub fn is_unknown(&self) -> bool {
        self.0 == DecodedReceiver::Unknown
    }
}

pub fn decode_unified_address(encoded: &str) -> Result<DecodedUnifiedAddress, ZecError> {
    let decoded = address::decode_unified_address(encoded)?;
    Ok(DecodedUnifiedAddress {
        network: decoded.network,
        receivers: decoded
            .receivers
            .into_iter()
            .map(DecodedUnifiedReceiver)
            .collect(),
    })
}

fn mutex_lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}
