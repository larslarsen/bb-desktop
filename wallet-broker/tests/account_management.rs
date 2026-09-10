use std::cell::{Cell, RefCell};
use std::collections::{BTreeMap, HashMap, VecDeque};
use std::fs;
use std::io::ErrorKind;
use std::os::unix::fs::{FileTypeExt, PermissionsExt, symlink};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};

use bitbook_wallet_broker::accounts::{AccountError, AccountManager, AccountSummary};
use bitbook_wallet_broker::native::ActionOrigin;
use bitbook_wallet_broker::session::{AUTHORIZATION_IDLE_MILLIS, ClockError, MonotonicClock};
use bitbook_wallet_broker::vault::{
    Asset, EntropyPort, MAX_ENVELOPE_BYTES, Network, SecretBytes, VaultError, VaultMetadata,
    VaultWorkObserver, WipeEvent, WipeObserver, open_vault_bytes, parse_vault, seal_vault,
};
use bitbook_wallet_broker::zec::test_support::decode_unified_address;
use bitbook_wallet_broker::zec::{FreshReceiverV1, Network as ZecNetwork};
use rusqlite::{Connection, params};
use zcash_keys::keys::{UnifiedAddressRequest, UnifiedSpendingKey};
use zcash_protocol::consensus::Network::TestNetwork;

const ID_A: [u8; 16] = [
    0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff,
];
const ID_B: [u8; 16] = [
    0xff, 0xee, 0xdd, 0xcc, 0xbb, 0xaa, 0x99, 0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11, 0x00,
];
const ID_C: [u8; 16] = [
    0x10, 0x32, 0x54, 0x76, 0x98, 0xba, 0xdc, 0xfe, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef,
];
const SEED_A: [u8; 32] = *b"CANARY_WAL013_SEED_A_32_BYTES!!!";
const SEED_B: [u8; 32] = *b"CANARY_WAL013_SEED_B_32_BYTES!!!";
const SHORT_SEED: [u8; 16] = *b"CANARY_WAL013_16";
const PASSPHRASE: &[u8] = b"CANARY_WAL013_PASSPHRASE";
const WRONG_PASSPHRASE: &[u8] = b"CANARY_WAL013_WRONG_PASS";
const FORBIDDEN: [ActionOrigin; 3] = [
    ActionOrigin::Electron,
    ActionOrigin::BrokerProtocol,
    ActionOrigin::Http,
];

struct Scratch {
    root: PathBuf,
    extras: Vec<PathBuf>,
}

impl Scratch {
    fn new(label: &str) -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(1);
        let token = NEXT.fetch_add(1, Ordering::Relaxed);
        let name = format!("wal013-accounts-{label}-{}-{token}", std::process::id());
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join(name);
        match fs::symlink_metadata(&root) {
            Err(error) if error.kind() == ErrorKind::NotFound => {}
            Ok(_) => panic!("refusing to reuse stale WAL-013 root {}", root.display()),
            Err(error) => panic!("cannot inspect WAL-013 root: {error}"),
        }
        fs::create_dir_all(&root).unwrap();
        fs::set_permissions(&root, fs::Permissions::from_mode(0o700)).unwrap();
        Self {
            root,
            extras: Vec::new(),
        }
    }

    fn accounts(&self) -> PathBuf {
        self.root.join("accounts")
    }

    fn sibling(&mut self, suffix: &str) -> PathBuf {
        let name = format!(
            "{}-{suffix}",
            self.root.file_name().unwrap().to_string_lossy()
        );
        let path = self.root.parent().unwrap().join(name);
        self.extras.push(path.clone());
        path
    }

    fn cleanup(&mut self) -> Result<(), String> {
        let mut errors = Vec::new();
        for extra in self.extras.iter().rev() {
            push_cleanup_error(&mut errors, unlink_dir_contents(extra));
            push_cleanup_error(&mut errors, unlink_any(extra));
        }
        let zec_network = self.root.join("zec-testnet");
        for account_id in [id_hex(&ID_A), id_hex(&ID_B), id_hex(&ID_C)] {
            push_cleanup_error(&mut errors, unlink_zec_account(&zec_network, &account_id));
        }
        push_cleanup_error(&mut errors, unlink_any(&zec_network));
        let accounts = self.root.join("accounts");
        push_cleanup_error(&mut errors, unlink_dir_contents(&accounts));
        push_cleanup_error(&mut errors, unlink_any(&accounts));
        push_cleanup_error(&mut errors, unlink_dir_contents(&self.root));
        push_cleanup_error(&mut errors, unlink_any(&self.root));
        for extra in &self.extras {
            push_cleanup_error(&mut errors, assert_absent(extra));
        }
        push_cleanup_error(&mut errors, assert_absent(&accounts));
        push_cleanup_error(&mut errors, assert_absent(&self.root));
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors.join("; "))
        }
    }
}

fn unlink_zec_account(network: &Path, account_id: &str) -> Result<(), String> {
    let network_metadata = match fs::symlink_metadata(network) {
        Err(error) if error.kind() == ErrorKind::NotFound => return Ok(()),
        Err(_) => return Err("cannot inspect owned Zcash network directory".to_owned()),
        Ok(metadata) => metadata,
    };
    if !network_metadata.file_type().is_dir() || network_metadata.file_type().is_symlink() {
        return Ok(());
    }
    let directory = network.join(account_id);
    let metadata = match fs::symlink_metadata(&directory) {
        Err(error) if error.kind() == ErrorKind::NotFound => return Ok(()),
        Err(error) => {
            return Err(format!("cannot inspect {}: {error}", directory.display()));
        }
        Ok(metadata) => metadata,
    };
    if metadata.file_type().is_symlink() || !metadata.file_type().is_dir() {
        return unlink_any(&directory);
    }
    for name in [
        "wallet.sqlite3",
        "wallet.sqlite3-journal",
        "wallet.sqlite3-wal",
        "wallet.sqlite3-shm",
        "compact.sqlite3",
        "compact.sqlite3-journal",
        "compact.sqlite3-wal",
        "compact.sqlite3-shm",
    ] {
        unlink_any(&directory.join(name))?;
    }
    unlink_any(&directory)
}

impl Drop for Scratch {
    fn drop(&mut self) {
        if let Err(error) = self.cleanup() {
            if std::thread::panicking() {
                eprintln!("WAL-013 scratch cleanup failed during unwind: {error}");
            } else {
                panic!("WAL-013 scratch cleanup failed: {error}");
            }
        }
    }
}

fn push_cleanup_error(errors: &mut Vec<String>, result: Result<(), String>) {
    if let Err(error) = result {
        errors.push(error);
    }
}

fn assert_absent(path: &Path) -> Result<(), String> {
    match fs::symlink_metadata(path) {
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(()),
        Err(error) => Err(format!("cannot inspect {}: {error}", path.display())),
        Ok(_) => Err(format!("owned temp leaked: {}", path.display())),
    }
}

fn unlink_any(path: &Path) -> Result<(), String> {
    let meta = match fs::symlink_metadata(path) {
        Err(error) if error.kind() == ErrorKind::NotFound => return Ok(()),
        Err(error) => return Err(format!("cannot inspect {}: {error}", path.display())),
        Ok(meta) => meta,
    };
    let result = if meta.file_type().is_dir() && !meta.file_type().is_symlink() {
        fs::remove_dir(path)
    } else {
        fs::remove_file(path)
    };
    match result {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(()),
        Err(error) => Err(format!("cannot remove {}: {error}", path.display())),
    }
}

fn unlink_dir_contents(dir: &Path) -> Result<(), String> {
    let meta = match fs::symlink_metadata(dir) {
        Err(error) if error.kind() == ErrorKind::NotFound => return Ok(()),
        Err(error) => return Err(format!("cannot inspect {}: {error}", dir.display())),
        Ok(meta) => meta,
    };
    if !meta.file_type().is_dir() || meta.file_type().is_symlink() {
        return Ok(());
    }
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(error) => return Err(format!("cannot read {}: {error}", dir.display())),
    };
    let mut errors = Vec::new();
    for entry in entries {
        match entry {
            Ok(entry) => push_cleanup_error(&mut errors, unlink_any(&entry.path())),
            Err(error) => errors.push(format!("cannot read entry in {}: {error}", dir.display())),
        }
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors.join("; "))
    }
}

#[derive(Clone)]
struct SharedClock {
    now: Rc<Cell<u64>>,
    fail: Rc<Cell<bool>>,
}

impl SharedClock {
    fn new(now: u64) -> Self {
        Self {
            now: Rc::new(Cell::new(now)),
            fail: Rc::new(Cell::new(false)),
        }
    }

    fn set(&self, now: u64) {
        self.now.set(now);
    }

    fn fail(&self) {
        self.fail.set(true);
    }

    fn resume(&self) {
        self.fail.set(false);
    }
}

impl MonotonicClock for SharedClock {
    fn now_millis(&mut self) -> Result<u64, ClockError> {
        if self.fail.get() {
            Err(ClockError::Unavailable)
        } else {
            Ok(self.now.get())
        }
    }
}

struct EntropyState {
    by_label: HashMap<&'static str, VecDeque<Vec<u8>>>,
    fail_next: bool,
    calls: Vec<(&'static str, usize)>,
}

#[derive(Clone)]
struct ScriptedEntropy {
    inner: Rc<RefCell<EntropyState>>,
}

impl ScriptedEntropy {
    fn new() -> Self {
        Self {
            inner: Rc::new(RefCell::new(EntropyState {
                by_label: HashMap::new(),
                fail_next: false,
                calls: Vec::new(),
            })),
        }
    }

    fn push(&self, label: &'static str, bytes: Vec<u8>) {
        self.inner
            .borrow_mut()
            .by_label
            .entry(label)
            .or_default()
            .push_back(bytes);
    }

    fn script_create(&self, id: &[u8; 16], seed: &[u8; 32], salt: u8, nonce: u8, staging: u8) {
        self.push("account-id", id.to_vec());
        self.push("account-seed", seed.to_vec());
        self.push("vault-salt", vec![salt; 16]);
        self.push("vault-nonce", vec![nonce; 24]);
        self.push("staging-name", vec![staging; 8]);
    }

    fn fail_next(&self) {
        self.inner.borrow_mut().fail_next = true;
    }

    fn calls(&self) -> Vec<(&'static str, usize)> {
        self.inner.borrow().calls.clone()
    }
}

impl EntropyPort for ScriptedEntropy {
    fn fill(&mut self, label: &'static str, output: &mut [u8]) -> Result<(), VaultError> {
        let mut inner = self.inner.borrow_mut();
        inner.calls.push((label, output.len()));
        if inner.fail_next {
            inner.fail_next = false;
            return Err(VaultError::entropy());
        }
        let bytes = inner
            .by_label
            .get_mut(label)
            .and_then(|queue| queue.pop_front())
            .ok_or_else(VaultError::entropy)?;
        if bytes.len() != output.len() {
            return Err(VaultError::entropy());
        }
        output.copy_from_slice(&bytes);
        Ok(())
    }
}

#[derive(Clone, Default)]
struct SharedWipes(Rc<RefCell<Vec<WipeEvent>>>);

impl SharedWipes {
    fn clear(&self) {
        self.0.borrow_mut().clear();
    }

    fn passphrase_wiped(&self, length: usize) -> bool {
        self.0
            .borrow()
            .iter()
            .any(|event| event.label == "passphrase" && event.length == length && event.all_zero)
    }

    fn plaintext_wiped(&self, length: usize) -> bool {
        self.0
            .borrow()
            .iter()
            .any(|event| event.label == "plaintext" && event.length == length && event.all_zero)
    }

    fn labeled_wipe(&self, label: &str, length: usize) -> bool {
        self.0
            .borrow()
            .iter()
            .any(|event| event.label == label && event.length == length && event.all_zero)
    }
}

impl WipeObserver for SharedWipes {
    fn observe(&mut self, event: WipeEvent) {
        self.0.borrow_mut().push(event);
    }
}

struct Ignore;

impl WipeObserver for Ignore {
    fn observe(&mut self, _event: WipeEvent) {}
}

struct Work;

impl VaultWorkObserver for Work {
    fn before_allocation(&mut self, _bytes: usize) -> Result<(), VaultError> {
        Ok(())
    }

    fn before_kdf(&mut self) {}
}

struct FixedEntropy {
    salt: [u8; 16],
    nonce: [u8; 24],
}

impl EntropyPort for FixedEntropy {
    fn fill(&mut self, label: &'static str, output: &mut [u8]) -> Result<(), VaultError> {
        match label {
            "vault-salt" if output.len() == 16 => {
                output.copy_from_slice(&self.salt);
                Ok(())
            }
            "vault-nonce" if output.len() == 24 => {
                output.copy_from_slice(&self.nonce);
                Ok(())
            }
            _ => Err(VaultError::entropy()),
        }
    }
}

struct Harness {
    scratch: Scratch,
    clock: SharedClock,
    entropy: ScriptedEntropy,
    wipes: SharedWipes,
}

impl Harness {
    fn new(label: &str) -> Self {
        Self {
            scratch: Scratch::new(label),
            clock: SharedClock::new(1_000),
            entropy: ScriptedEntropy::new(),
            wipes: SharedWipes::default(),
        }
    }

    fn manager(&self) -> AccountManager<SharedClock, ScriptedEntropy, SharedWipes> {
        AccountManager::with_ports(
            &self.scratch.root,
            self.clock.clone(),
            self.entropy.clone(),
            self.wipes.clone(),
        )
        .unwrap()
    }
}

fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(HEX[(byte >> 4) as usize] as char);
        output.push(HEX[(byte & 0x0f) as usize] as char);
    }
    output
}

fn id_hex(id: &[u8; 16]) -> String {
    hex_encode(id)
}

fn write_private(path: &Path, bytes: &[u8]) {
    fs::write(path, bytes).unwrap();
    fs::set_permissions(path, fs::Permissions::from_mode(0o600)).unwrap();
}

fn plant_active(root: &Path, id: &str, bytes: &[u8]) {
    write_private(&root.join("accounts").join(format!("{id}.vault")), bytes);
}

fn mode(path: &Path) -> u32 {
    fs::symlink_metadata(path).unwrap().permissions().mode() & 0o777
}

fn catalog_files(root: &Path) -> BTreeMap<String, Vec<u8>> {
    let mut files = BTreeMap::new();
    let accounts = root.join("accounts");
    let Ok(meta) = fs::symlink_metadata(&accounts) else {
        return files;
    };
    if !meta.file_type().is_dir() || meta.file_type().is_symlink() {
        return files;
    }
    for entry in fs::read_dir(&accounts).unwrap().flatten() {
        let path = entry.path();
        let meta = fs::symlink_metadata(&path).unwrap();
        if meta.file_type().is_file() {
            files.insert(
                path.file_name().unwrap().to_str().unwrap().to_owned(),
                fs::read(&path).unwrap(),
            );
        }
    }
    files
}

fn vault_names(root: &Path) -> Vec<String> {
    catalog_files(root)
        .into_keys()
        .filter(|name| name.ends_with(".vault"))
        .collect()
}

fn walk_regular_files(root: &Path) -> Vec<(PathBuf, Vec<u8>)> {
    let mut stack = vec![root.to_path_buf()];
    let mut files = Vec::new();
    while let Some(dir) = stack.pop() {
        let Ok(meta) = fs::symlink_metadata(&dir) else {
            continue;
        };
        if !meta.file_type().is_dir() || meta.file_type().is_symlink() {
            continue;
        }
        for entry in fs::read_dir(&dir).unwrap().flatten() {
            let path = entry.path();
            let meta = fs::symlink_metadata(&path).unwrap();
            let file_type = meta.file_type();
            if file_type.is_symlink() {
                continue;
            }
            if file_type.is_dir() {
                stack.push(path);
            } else if file_type.is_file() {
                files.push((path.clone(), fs::read(&path).unwrap()));
            }
        }
    }
    files
}

fn assert_no_plaintext(root: &Path, secrets: &[&[u8]]) {
    for (path, bytes) in walk_regular_files(root) {
        for secret in secrets {
            assert!(
                !bytes.windows(secret.len()).any(|window| window == *secret),
                "plaintext leaked in {}",
                path.display()
            );
        }
    }
}

fn assert_no_secrets(text: &str) {
    for needle in [
        "CANARY_WAL013",
        core::str::from_utf8(&SEED_A).unwrap(),
        core::str::from_utf8(&SEED_B).unwrap(),
        core::str::from_utf8(PASSPHRASE).unwrap(),
        core::str::from_utf8(WRONG_PASSPHRASE).unwrap(),
        &hex_encode(&SEED_A),
        &hex_encode(&SEED_B),
    ] {
        assert!(
            !text.contains(needle),
            "secret leaked in public text: {text}"
        );
    }
}

fn assert_public_error(error: &AccountError, code: &str) {
    assert_eq!(error.code(), code);
    assert_no_secrets(&format!("{error:?}{error}"));
}

fn oracle_ufvk(seed: &[u8; 32]) -> String {
    UnifiedSpendingKey::from_seed(&TestNetwork, seed, Default::default())
        .expect("scripted seed must derive")
        .to_unified_full_viewing_key()
        .encode(&TestNetwork)
}

fn oracle_receiver(seed: &[u8; 32], index: u64) -> String {
    let viewing = UnifiedSpendingKey::from_seed(&TestNetwork, seed, Default::default())
        .expect("scripted seed must derive")
        .to_unified_full_viewing_key();
    let (address, actual_index) = viewing
        .find_address(index.into(), UnifiedAddressRequest::ORCHARD)
        .expect("reviewed Orchard-only request must derive");
    assert_eq!(u64::try_from(actual_index).unwrap(), index);
    address.encode(&TestNetwork)
}

fn assert_scripted_receiver(
    receiver: &FreshReceiverV1,
    account_id: &str,
    seed: &[u8; 32],
    index: u64,
    sequence: u64,
) {
    assert_eq!(receiver.account_id.as_str(), account_id);
    assert_eq!(receiver.network, ZecNetwork::Testnet);
    assert_eq!(receiver.receiver, oracle_receiver(seed, index));
    assert_eq!(receiver.diversifier_index, index.to_string());
    assert_eq!(receiver.issued_at_sequence, sequence.to_string());
    let decoded = decode_unified_address(&receiver.receiver).unwrap();
    assert_eq!(decoded.network, ZecNetwork::Testnet);
    assert_eq!(decoded.receivers.len(), 1);
    assert!(decoded.receivers[0].is_orchard_protocol());
    assert!(!decoded.receivers[0].is_p2pkh());
    assert!(!decoded.receivers[0].is_p2sh());
    assert!(!decoded.receivers[0].is_sapling());
    assert!(!decoded.receivers[0].is_tex());
    assert!(!decoded.receivers[0].is_unknown());
}

fn assert_locked_software(summary: &AccountSummary, account_id: &str) {
    assert_eq!(summary.account_id, account_id);
    assert_eq!(summary.asset, "ZEC");
    assert_eq!(summary.network, "zec-testnet");
    assert_eq!(summary.kind, "software");
    assert!(summary.locked);
}

fn decrypt_seed(bytes: &[u8], passphrase: &[u8]) -> Vec<u8> {
    let mut secret = SecretBytes::new(passphrase.to_vec()).unwrap();
    let opened = open_vault_bytes(bytes, &mut secret, &mut Work, &mut Ignore).unwrap();
    opened.expose(|plain| plain.to_vec())
}

fn seal_bytes(
    account_id: [u8; 16],
    asset: Asset,
    network: Network,
    epoch: u64,
    passphrase: &[u8],
    plaintext: &[u8],
    salt: u8,
    nonce: u8,
) -> Vec<u8> {
    let metadata = VaultMetadata::new(account_id, asset, network, epoch).unwrap();
    let mut entropy = FixedEntropy {
        salt: [salt; 16],
        nonce: [nonce; 24],
    };
    let mut pass = SecretBytes::new(passphrase.to_vec()).unwrap();
    let mut plain = SecretBytes::new(plaintext.to_vec()).unwrap();
    seal_vault(&metadata, &mut pass, &mut plain, &mut entropy, &mut Ignore)
        .unwrap()
        .into_bytes()
}

fn canonical_vault(account_id: &str, asset: &str, network: &str, epoch: u64) -> Vec<u8> {
    format!(
        "{{\"format\":\"bitbook-wallet-vault\",\"version\":1,\"account_id\":\"{account_id}\",\"asset\":\"{asset}\",\"network\":\"{network}\",\"epoch\":\"{epoch}\",\"kdf\":{{\"algorithm\":\"argon2id\",\"version\":19,\"m_cost_kib\":65536,\"t_cost\":3,\"p_cost\":1,\"salt_b64\":\"AAAAAAAAAAAAAAAAAAAAAA\"}},\"aead\":{{\"algorithm\":\"xchacha20poly1305\",\"nonce_b64\":\"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA\",\"ciphertext_b64\":\"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA\"}}}}\n"
    )
    .into_bytes()
}

fn mutate_ciphertext(bytes: &[u8]) -> Vec<u8> {
    let mut text = String::from_utf8(bytes.to_vec()).unwrap();
    let key = "\"ciphertext_b64\":\"";
    let index = text.find(key).unwrap() + key.len();
    let byte = text.as_bytes()[index];
    text.replace_range(index..index + 1, if byte == b'A' { "B" } else { "A" });
    text.into_bytes()
}

fn create_fifo(path: &Path) {
    let status = Command::new("mkfifo").arg(path).status().unwrap();
    assert!(status.success());
}

fn passphrase() -> SecretBytes {
    SecretBytes::new(PASSPHRASE.to_vec()).unwrap()
}

fn listed_ids(
    manager: &mut AccountManager<SharedClock, ScriptedEntropy, SharedWipes>,
) -> Vec<String> {
    manager
        .list()
        .unwrap()
        .into_iter()
        .map(|summary| summary.account_id)
        .collect()
}

fn unlocked(
    manager: &mut AccountManager<SharedClock, ScriptedEntropy, SharedWipes>,
    account_id: &str,
) -> bool {
    manager
        .list()
        .unwrap()
        .into_iter()
        .find(|summary| summary.account_id == account_id)
        .map(|summary| !summary.locked)
        .unwrap_or(false)
}

#[test]
fn create_software_persists_locked_encrypted_account_with_scripted_seed() {
    let harness = Harness::new("create");
    harness
        .entropy
        .script_create(&ID_A, &SEED_A, 0x11, 0x22, 0x01);
    harness
        .entropy
        .script_create(&ID_B, &SEED_B, 0x33, 0x44, 0x02);
    let mut manager = harness.manager();

    let first = manager
        .create_software(ActionOrigin::NativeSurface, passphrase())
        .unwrap();
    let second = manager
        .create_software(ActionOrigin::NativeSurface, passphrase())
        .unwrap();

    let id_a = id_hex(&ID_A);
    let id_b = id_hex(&ID_B);
    assert_locked_software(&first, &id_a);
    assert_locked_software(&second, &id_b);
    assert_eq!(mode(&harness.scratch.root), 0o700);
    assert_eq!(mode(&harness.scratch.accounts()), 0o700);

    let path_a = harness.scratch.accounts().join(format!("{id_a}.vault"));
    let path_b = harness.scratch.accounts().join(format!("{id_b}.vault"));
    let bytes_a = fs::read(&path_a).unwrap();
    let bytes_b = fs::read(&path_b).unwrap();
    assert_ne!(bytes_a, bytes_b);
    assert_eq!(mode(&path_a), 0o600);
    assert_eq!(mode(&path_b), 0o600);
    assert_eq!(decrypt_seed(&bytes_a, PASSPHRASE), SEED_A);
    assert_eq!(decrypt_seed(&bytes_b, PASSPHRASE), SEED_B);

    let parsed = parse_vault(&bytes_a, &mut Work).unwrap();
    assert_eq!(parsed.metadata().account_id_hex(), id_a);
    assert_eq!(parsed.metadata().asset(), Asset::Zec);
    assert_eq!(parsed.metadata().network(), Network::ZecTestnet);
    assert_eq!(parsed.metadata().epoch(), 1);
    assert_ne!(parsed.ciphertext(), SEED_A);

    let listed = manager.list().unwrap();
    assert_eq!(listed.len(), 2);
    assert!(listed.iter().all(|summary| summary.locked));
    assert_no_plaintext(&harness.scratch.root, &[&SEED_A, &SEED_B, PASSPHRASE]);
}

#[test]
fn restart_lists_persisted_locked_identities_without_secret_canaries() {
    let harness = Harness::new("restart");
    harness
        .entropy
        .script_create(&ID_B, &SEED_B, 0x21, 0x22, 0x03);
    harness
        .entropy
        .script_create(&ID_A, &SEED_A, 0x23, 0x24, 0x04);
    {
        let mut manager = harness.manager();
        manager
            .create_software(ActionOrigin::NativeSurface, passphrase())
            .unwrap();
        manager
            .create_software(ActionOrigin::NativeSurface, passphrase())
            .unwrap();
        manager
            .unlock(ActionOrigin::NativeSurface, &id_hex(&ID_A), passphrase())
            .unwrap();
        assert!(unlocked(&mut manager, &id_hex(&ID_A)));
    }

    let mut restarted = harness.manager();
    let listed = restarted.list().unwrap();
    assert_eq!(
        listed
            .iter()
            .map(|summary| summary.account_id.as_str())
            .collect::<Vec<_>>(),
        vec![id_hex(&ID_A), id_hex(&ID_B)]
    );
    for summary in &listed {
        assert_locked_software(summary, &summary.account_id);
        let json = serde_json::to_value(summary).unwrap();
        let object = json.as_object().unwrap();
        assert_eq!(object.len(), 5);
        for forbidden in [
            "seed",
            "passphrase",
            "balance",
            "address",
            "backup",
            "viewing",
            "ready",
        ] {
            assert!(!object.contains_key(forbidden));
        }
        assert_no_secrets(&serde_json::to_string(summary).unwrap());
        assert_no_secrets(&format!("{summary:?}"));
    }
}

#[test]
fn unlock_rejects_wrong_corrupt_and_short_material_then_valid_lock_cycle_works() {
    let harness = Harness::new("unlock");
    harness
        .entropy
        .script_create(&ID_A, &SEED_A, 0x31, 0x32, 0x05);
    let mut manager = harness.manager();
    manager
        .create_software(ActionOrigin::NativeSurface, passphrase())
        .unwrap();

    plant_active(
        &harness.scratch.root,
        &id_hex(&ID_B),
        &seal_bytes(
            ID_B,
            Asset::Zec,
            Network::ZecTestnet,
            1,
            PASSPHRASE,
            &SHORT_SEED,
            0x41,
            0x42,
        ),
    );
    plant_active(
        &harness.scratch.root,
        &id_hex(&ID_C),
        &mutate_ciphertext(&seal_bytes(
            ID_C,
            Asset::Zec,
            Network::ZecTestnet,
            1,
            PASSPHRASE,
            &SEED_B,
            0x43,
            0x44,
        )),
    );

    assert_public_error(
        &manager
            .unlock(
                ActionOrigin::NativeSurface,
                &id_hex(&ID_A),
                SecretBytes::new(WRONG_PASSPHRASE.to_vec()).unwrap(),
            )
            .unwrap_err(),
        "LOCKED",
    );
    assert_public_error(
        &manager
            .unlock(ActionOrigin::NativeSurface, &id_hex(&ID_B), passphrase())
            .unwrap_err(),
        "LOCKED",
    );
    assert_public_error(
        &manager
            .unlock(ActionOrigin::NativeSurface, &id_hex(&ID_C), passphrase())
            .unwrap_err(),
        "LOCKED",
    );
    assert!(!unlocked(&mut manager, &id_hex(&ID_A)));
    assert!(!unlocked(&mut manager, &id_hex(&ID_B)));
    assert!(!unlocked(&mut manager, &id_hex(&ID_C)));

    manager
        .unlock(ActionOrigin::NativeSurface, &id_hex(&ID_A), passphrase())
        .unwrap();
    assert!(unlocked(&mut manager, &id_hex(&ID_A)));

    harness.clock.set(1_000 + AUTHORIZATION_IDLE_MILLIS - 1);
    assert_public_error(
        &manager
            .unlock(
                ActionOrigin::NativeSurface,
                &id_hex(&ID_B),
                SecretBytes::new(WRONG_PASSPHRASE.to_vec()).unwrap(),
            )
            .unwrap_err(),
        "LOCKED",
    );
    assert!(unlocked(&mut manager, &id_hex(&ID_A)));
    harness.clock.set(1_000 + AUTHORIZATION_IDLE_MILLIS);
    manager.tick().unwrap();
    assert!(!unlocked(&mut manager, &id_hex(&ID_A)));

    manager
        .unlock(ActionOrigin::NativeSurface, &id_hex(&ID_A), passphrase())
        .unwrap();
    manager.lock(&id_hex(&ID_A)).unwrap();
    assert!(!unlocked(&mut manager, &id_hex(&ID_A)));
    manager
        .unlock(ActionOrigin::NativeSurface, &id_hex(&ID_A), passphrase())
        .unwrap();
    manager.lock_all();
    assert!(!unlocked(&mut manager, &id_hex(&ID_A)));
}

#[test]
fn list_does_not_extend_idle_deadline_and_backwards_clock_locks_all() {
    let harness = Harness::new("deadline");
    harness
        .entropy
        .script_create(&ID_A, &SEED_A, 0x51, 0x52, 0x06);
    harness
        .entropy
        .script_create(&ID_B, &SEED_B, 0x53, 0x54, 0x07);
    let mut manager = harness.manager();
    manager
        .create_software(ActionOrigin::NativeSurface, passphrase())
        .unwrap();
    manager
        .create_software(ActionOrigin::NativeSurface, passphrase())
        .unwrap();
    manager
        .unlock(ActionOrigin::NativeSurface, &id_hex(&ID_A), passphrase())
        .unwrap();
    manager
        .unlock(ActionOrigin::NativeSurface, &id_hex(&ID_B), passphrase())
        .unwrap();

    harness.clock.set(1_000 + AUTHORIZATION_IDLE_MILLIS - 1);
    let listed = manager.list().unwrap();
    assert!(
        listed
            .iter()
            .any(|summary| summary.account_id == id_hex(&ID_A) && !summary.locked)
    );
    harness.clock.set(1_000 + AUTHORIZATION_IDLE_MILLIS);
    let listed = manager.list().unwrap();
    assert!(listed.iter().all(|summary| summary.locked));

    manager
        .unlock(ActionOrigin::NativeSurface, &id_hex(&ID_A), passphrase())
        .unwrap();
    manager
        .unlock(ActionOrigin::NativeSurface, &id_hex(&ID_B), passphrase())
        .unwrap();
    harness.clock.set(1_000 + AUTHORIZATION_IDLE_MILLIS - 1);
    assert_public_error(&manager.tick().unwrap_err(), "UNAVAILABLE");
    harness.clock.set(1_000 + AUTHORIZATION_IDLE_MILLIS);
    assert!(!unlocked(&mut manager, &id_hex(&ID_A)));
    assert!(!unlocked(&mut manager, &id_hex(&ID_B)));

    manager
        .unlock(ActionOrigin::NativeSurface, &id_hex(&ID_A), passphrase())
        .unwrap();
    harness.clock.fail();
    assert_public_error(&manager.tick().unwrap_err(), "UNAVAILABLE");
    harness.clock.resume();
    assert!(!unlocked(&mut manager, &id_hex(&ID_A)));
}

#[test]
fn nonnative_origins_refuse_privileged_apis_before_side_effects() {
    let mut harness = Harness::new("unauth");
    harness
        .entropy
        .script_create(&ID_A, &SEED_A, 0x61, 0x62, 0x08);
    let mut manager = harness.manager();
    let export_dest = harness.scratch.sibling("export.vault");
    let restore_link = harness.scratch.sibling("restore-link");
    let restore_target = harness.scratch.sibling("restore-target.vault");
    write_private(
        &restore_target,
        &seal_bytes(
            ID_B,
            Asset::Zec,
            Network::ZecTestnet,
            1,
            PASSPHRASE,
            &SEED_B,
            0x63,
            0x64,
        ),
    );
    symlink(&restore_target, &restore_link).unwrap();

    for origin in FORBIDDEN {
        harness.wipes.clear();
        let before_calls = harness.entropy.calls().len();
        let before_catalog = catalog_files(&harness.scratch.root);
        assert_public_error(
            &manager.create_software(origin, passphrase()).unwrap_err(),
            "UNAUTH",
        );
        assert!(harness.wipes.passphrase_wiped(PASSPHRASE.len()));
        assert_eq!(harness.entropy.calls().len(), before_calls);
        assert_eq!(catalog_files(&harness.scratch.root), before_catalog);

        harness.wipes.clear();
        assert_public_error(
            &manager
                .unlock(origin, "NOT_AN_ACCOUNT_ID", passphrase())
                .unwrap_err(),
            "UNAUTH",
        );
        assert!(harness.wipes.passphrase_wiped(PASSPHRASE.len()));

        assert_public_error(
            &manager
                .export_encrypted(origin, "NOT_AN_ACCOUNT_ID", Path::new("relative.vault"))
                .unwrap_err(),
            "UNAUTH",
        );
        assert!(!export_dest.exists());

        harness.wipes.clear();
        assert_public_error(
            &manager
                .prepare_restore(origin, &restore_link, passphrase())
                .err()
                .expect("forbidden origin prepare_restore should fail"),
            "UNAUTH",
        );
        assert!(harness.wipes.passphrase_wiped(PASSPHRASE.len()));
    }

    manager
        .create_software(ActionOrigin::NativeSurface, passphrase())
        .unwrap();
    for origin in FORBIDDEN {
        let prepared = manager
            .prepare_restore(ActionOrigin::NativeSurface, &restore_target, passphrase())
            .unwrap();
        assert_eq!(prepared.summary().account_id, id_hex(&ID_B));
        assert!(prepared.summary().locked);
        let before = catalog_files(&harness.scratch.root);
        assert_public_error(
            &manager.confirm_restore(origin, prepared, true).unwrap_err(),
            "UNAUTH",
        );
        assert_eq!(catalog_files(&harness.scratch.root), before);
    }
    assert_eq!(listed_ids(&mut manager), vec![id_hex(&ID_A)]);
}

#[test]
fn entropy_failure_and_duplicate_id_never_overwrite_existing_vault() {
    let harness = Harness::new("entropy");
    let mut manager = harness.manager();
    harness.entropy.fail_next();
    harness.wipes.clear();
    assert_public_error(
        &manager
            .create_software(ActionOrigin::NativeSurface, passphrase())
            .unwrap_err(),
        "UNAVAILABLE",
    );
    assert!(harness.wipes.passphrase_wiped(PASSPHRASE.len()));
    assert!(vault_names(&harness.scratch.root).is_empty());

    let salt_fail = Harness::new("entropy-salt");
    salt_fail.entropy.push("account-id", ID_B.to_vec());
    salt_fail.entropy.push("account-seed", SEED_B.to_vec());
    let mut salt_manager = salt_fail.manager();
    salt_fail.wipes.clear();
    assert_public_error(
        &salt_manager
            .create_software(ActionOrigin::NativeSurface, passphrase())
            .unwrap_err(),
        "UNAVAILABLE",
    );
    assert!(vault_names(&salt_fail.scratch.root).is_empty());
    assert!(salt_fail.wipes.passphrase_wiped(PASSPHRASE.len()));
    assert!(salt_fail.wipes.plaintext_wiped(SEED_B.len()));

    let original = seal_bytes(
        ID_A,
        Asset::Zec,
        Network::ZecTestnet,
        1,
        PASSPHRASE,
        &SEED_A,
        0x71,
        0x72,
    );
    plant_active(&harness.scratch.root, &id_hex(&ID_A), &original);
    harness
        .entropy
        .script_create(&ID_A, &SEED_B, 0x73, 0x74, 0x09);
    harness.wipes.clear();
    assert_public_error(
        &manager
            .create_software(ActionOrigin::NativeSurface, passphrase())
            .unwrap_err(),
        "ALREADY_EXISTS",
    );
    assert_eq!(
        fs::read(
            harness
                .scratch
                .accounts()
                .join(format!("{}.vault", id_hex(&ID_A)))
        )
        .unwrap(),
        original
    );
    assert!(harness.wipes.passphrase_wiped(PASSPHRASE.len()));
}

#[test]
fn locked_export_copies_exact_encrypted_bytes_and_refuses_existing_or_link() {
    let mut harness = Harness::new("export");
    harness
        .entropy
        .script_create(&ID_A, &SEED_A, 0x81, 0x82, 0x0a);
    let mut manager = harness.manager();
    manager
        .create_software(ActionOrigin::NativeSurface, passphrase())
        .unwrap();
    assert!(!unlocked(&mut manager, &id_hex(&ID_A)));

    let active = fs::read(
        harness
            .scratch
            .accounts()
            .join(format!("{}.vault", id_hex(&ID_A))),
    )
    .unwrap();
    let dest = harness.scratch.sibling("export.vault");
    manager
        .export_encrypted(ActionOrigin::NativeSurface, &id_hex(&ID_A), &dest)
        .unwrap();
    assert_eq!(fs::read(&dest).unwrap(), active);
    assert_eq!(mode(&dest), 0o600);
    assert_eq!(decrypt_seed(&fs::read(&dest).unwrap(), PASSPHRASE), SEED_A);
    assert!(!unlocked(&mut manager, &id_hex(&ID_A)));

    assert_public_error(
        &manager
            .export_encrypted(ActionOrigin::NativeSurface, &id_hex(&ID_A), &dest)
            .unwrap_err(),
        "ALREADY_EXISTS",
    );
    assert_eq!(fs::read(&dest).unwrap(), active);

    let link = harness.scratch.sibling("export-link");
    symlink(&dest, &link).unwrap();
    assert_public_error(
        &manager
            .export_encrypted(ActionOrigin::NativeSurface, &id_hex(&ID_A), &link)
            .unwrap_err(),
        "ALREADY_EXISTS",
    );
    assert_eq!(fs::read(&dest).unwrap(), active);
    assert!(
        fs::symlink_metadata(&link)
            .unwrap()
            .file_type()
            .is_symlink()
    );
}

#[test]
fn restore_cancel_is_a_noop_and_confirm_commits_captured_not_replaced_bytes() {
    let mut harness = Harness::new("restore-capture");
    harness.entropy.push("staging-name", vec![0x0b; 8]);
    let mut manager = harness.manager();
    let captured = seal_bytes(
        ID_A,
        Asset::Zec,
        Network::ZecTestnet,
        1,
        PASSPHRASE,
        &SEED_A,
        0x91,
        0x92,
    );
    let replacement = seal_bytes(
        ID_B,
        Asset::Zec,
        Network::ZecTestnet,
        1,
        PASSPHRASE,
        &SEED_B,
        0x93,
        0x94,
    );
    let source = harness.scratch.sibling("restore-source.vault");
    write_private(&source, &captured);

    let prepared = manager
        .prepare_restore(ActionOrigin::NativeSurface, &source, passphrase())
        .unwrap();
    assert_locked_software(prepared.summary(), &id_hex(&ID_A));
    let before = catalog_files(&harness.scratch.root);
    assert!(
        manager
            .confirm_restore(ActionOrigin::NativeSurface, prepared, false)
            .unwrap()
            .is_none()
    );
    assert_eq!(catalog_files(&harness.scratch.root), before);

    let prepared = manager
        .prepare_restore(ActionOrigin::NativeSurface, &source, passphrase())
        .unwrap();
    write_private(&source, &replacement);
    let restored = manager
        .confirm_restore(ActionOrigin::NativeSurface, prepared, true)
        .unwrap()
        .unwrap();
    assert_locked_software(&restored, &id_hex(&ID_A));
    let imported = fs::read(
        harness
            .scratch
            .accounts()
            .join(format!("{}.vault", id_hex(&ID_A))),
    )
    .unwrap();
    assert_eq!(imported, captured);
    assert_ne!(imported, replacement);
    assert_eq!(fs::read(&source).unwrap(), replacement);
    assert_eq!(listed_ids(&mut manager), vec![id_hex(&ID_A)]);
    assert!(!unlocked(&mut manager, &id_hex(&ID_A)));
}

#[test]
fn restore_rejects_hostile_inputs_and_never_overwrites_existing_id() {
    let mut harness = Harness::new("restore-reject");
    harness
        .entropy
        .script_create(&ID_A, &SEED_A, 0xa1, 0xa2, 0x0c);
    harness.entropy.push("staging-name", vec![0x0d; 8]);
    let mut manager = harness.manager();
    manager
        .create_software(ActionOrigin::NativeSurface, passphrase())
        .unwrap();
    let original = fs::read(
        harness
            .scratch
            .accounts()
            .join(format!("{}.vault", id_hex(&ID_A))),
    )
    .unwrap();

    let wrong_pass = harness.scratch.sibling("CANARY_WAL013_PATH-wrong.vault");
    write_private(
        &wrong_pass,
        &seal_bytes(
            ID_B,
            Asset::Zec,
            Network::ZecTestnet,
            1,
            PASSPHRASE,
            &SEED_B,
            0xa3,
            0xa4,
        ),
    );
    assert_public_error(
        &manager
            .prepare_restore(
                ActionOrigin::NativeSurface,
                &wrong_pass,
                SecretBytes::new(WRONG_PASSPHRASE.to_vec()).unwrap(),
            )
            .err()
            .expect("wrong passphrase prepare_restore should fail"),
        "LOCKED",
    );

    let wrong_network = harness.scratch.sibling("CANARY_WAL013_PATH-network.vault");
    write_private(
        &wrong_network,
        &seal_bytes(
            ID_B,
            Asset::Zec,
            Network::ZecRegtest,
            1,
            PASSPHRASE,
            &SEED_B,
            0xa5,
            0xa6,
        ),
    );
    let error = manager
        .prepare_restore(ActionOrigin::NativeSurface, &wrong_network, passphrase())
        .err()
        .expect("wrong network prepare_restore should fail");
    assert_public_error(&error, "UNAVAILABLE");
    assert!(!format!("{error:?}{error}").contains("CANARY_WAL013_PATH"));

    let short = harness.scratch.sibling("short.vault");
    write_private(
        &short,
        &seal_bytes(
            ID_B,
            Asset::Zec,
            Network::ZecTestnet,
            1,
            PASSPHRASE,
            &SHORT_SEED,
            0xa7,
            0xa8,
        ),
    );
    assert_public_error(
        &manager
            .prepare_restore(ActionOrigin::NativeSurface, &short, passphrase())
            .err()
            .expect("short seed prepare_restore should fail"),
        "LOCKED",
    );

    let malformed = harness.scratch.sibling("malformed.vault");
    write_private(&malformed, b"{not-a-vault");
    assert_public_error(
        &manager
            .prepare_restore(ActionOrigin::NativeSurface, &malformed, passphrase())
            .err()
            .expect("malformed prepare_restore should fail"),
        "SCHEMA",
    );

    let newer = harness.scratch.sibling("newer.vault");
    write_private(
        &newer,
        &seal_bytes(
            ID_A,
            Asset::Zec,
            Network::ZecTestnet,
            9,
            PASSPHRASE,
            &SEED_B,
            0xa9,
            0xaa,
        ),
    );
    let prepared = manager
        .prepare_restore(ActionOrigin::NativeSurface, &newer, passphrase())
        .unwrap();
    assert_eq!(prepared.summary().account_id, id_hex(&ID_A));
    assert_public_error(
        &manager
            .confirm_restore(ActionOrigin::NativeSurface, prepared, true)
            .unwrap_err(),
        "ALREADY_EXISTS",
    );
    assert_eq!(
        fs::read(
            harness
                .scratch
                .accounts()
                .join(format!("{}.vault", id_hex(&ID_A)))
        )
        .unwrap(),
        original
    );
    assert_eq!(listed_ids(&mut manager), vec![id_hex(&ID_A)]);
}

#[test]
fn hostile_roots_and_catalog_entries_fail_closed_without_chmod() {
    let root_case = Scratch::new("hostile-root");
    fs::set_permissions(&root_case.root, fs::Permissions::from_mode(0o755)).unwrap();
    assert_public_error(
        &AccountManager::with_ports(
            &root_case.root,
            SharedClock::new(1_000),
            ScriptedEntropy::new(),
            SharedWipes::default(),
        )
        .err()
        .expect("hostile broker root should be rejected"),
        "UNAVAILABLE",
    );
    assert_eq!(mode(&root_case.root), 0o755);

    let accounts_case = Scratch::new("hostile-accounts");
    fs::create_dir(accounts_case.accounts()).unwrap();
    fs::set_permissions(&accounts_case.accounts(), fs::Permissions::from_mode(0o755)).unwrap();
    assert_public_error(
        &AccountManager::with_ports(
            &accounts_case.root,
            SharedClock::new(1_000),
            ScriptedEntropy::new(),
            SharedWipes::default(),
        )
        .err()
        .expect("hostile accounts directory should be rejected"),
        "UNAVAILABLE",
    );
    assert_eq!(mode(&accounts_case.accounts()), 0o755);

    let mut symlink_root = Scratch::new("symlink-root-target");
    let keep_root = symlink_root.root.join("keep");
    write_private(&keep_root, b"keep-root");
    let root_mode = mode(&symlink_root.root);
    let root_link = symlink_root.sibling("symlink-root");
    symlink(&symlink_root.root, &root_link).unwrap();
    assert_public_error(
        &AccountManager::with_ports(
            &root_link,
            SharedClock::new(1_000),
            ScriptedEntropy::new(),
            SharedWipes::default(),
        )
        .err()
        .expect("symlink broker root should be rejected"),
        "UNAVAILABLE",
    );
    assert_eq!(mode(&symlink_root.root), root_mode);
    assert_eq!(fs::read(&keep_root).unwrap(), b"keep-root");
    assert!(matches!(
        fs::symlink_metadata(&symlink_root.accounts()),
        Err(error) if error.kind() == ErrorKind::NotFound
    ));

    let mut symlink_accounts = Scratch::new("symlink-accounts-target");
    let real_accounts = symlink_accounts.sibling("real-accounts");
    fs::create_dir(&real_accounts).unwrap();
    fs::set_permissions(&real_accounts, fs::Permissions::from_mode(0o700)).unwrap();
    let keep_accounts = real_accounts.join("keep");
    write_private(&keep_accounts, b"keep-accounts");
    let accounts_mode = mode(&real_accounts);
    symlink(&real_accounts, &symlink_accounts.accounts()).unwrap();
    assert_public_error(
        &AccountManager::with_ports(
            &symlink_accounts.root,
            SharedClock::new(1_000),
            ScriptedEntropy::new(),
            SharedWipes::default(),
        )
        .err()
        .expect("symlink accounts directory should be rejected"),
        "UNAVAILABLE",
    );
    assert_eq!(mode(&real_accounts), accounts_mode);
    assert_eq!(fs::read(&keep_accounts).unwrap(), b"keep-accounts");
    assert_eq!(
        fs::read_dir(&real_accounts)
            .unwrap()
            .map(|entry| entry.expect("symlink accounts target readdir").file_name())
            .collect::<Vec<_>>(),
        vec![std::ffi::OsString::from("keep")]
    );

    let harness = Harness::new("catalog");
    let mut manager = harness.manager();
    plant_active(
        &harness.scratch.root,
        &id_hex(&ID_A),
        &canonical_vault(&id_hex(&ID_A), "ZEC", "zec-testnet", 1),
    );
    write_private(
        &harness.scratch.accounts().join(format!(
            ".{}.{}.stage",
            id_hex(&ID_A),
            "0123456789abcdef"
        )),
        b"ignored-stage",
    );
    assert_eq!(listed_ids(&mut manager), vec![id_hex(&ID_A)]);

    fs::write(harness.scratch.accounts().join("readme.txt"), b"nope").unwrap();
    assert_public_error(&manager.list().unwrap_err(), "UNAVAILABLE");
    fs::remove_file(harness.scratch.accounts().join("readme.txt")).unwrap();

    let link = harness
        .scratch
        .accounts()
        .join(format!("{}.vault", id_hex(&ID_B)));
    symlink(format!("{}.vault", id_hex(&ID_A)), &link).unwrap();
    assert_public_error(&manager.list().unwrap_err(), "UNAVAILABLE");
    fs::remove_file(&link).unwrap();

    let fifo = harness
        .scratch
        .accounts()
        .join(format!("{}.vault", id_hex(&ID_B)));
    create_fifo(&fifo);
    assert!(fs::symlink_metadata(&fifo).unwrap().file_type().is_fifo());
    assert_public_error(&manager.list().unwrap_err(), "UNAVAILABLE");
    fs::remove_file(&fifo).unwrap();

    plant_active(
        &harness.scratch.root,
        &id_hex(&ID_B),
        &canonical_vault(&id_hex(&ID_B), "ZEC", "zec-testnet", 1),
    );
    let wrong_mode = harness
        .scratch
        .accounts()
        .join(format!("{}.vault", id_hex(&ID_B)));
    fs::set_permissions(&wrong_mode, fs::Permissions::from_mode(0o644)).unwrap();
    assert_public_error(&manager.list().unwrap_err(), "UNAVAILABLE");
    assert_eq!(mode(&wrong_mode), 0o644);
    fs::remove_file(&wrong_mode).unwrap();

    plant_active(
        &harness.scratch.root,
        &id_hex(&ID_B),
        &canonical_vault(&id_hex(&ID_A), "ZEC", "zec-testnet", 1),
    );
    assert_public_error(&manager.list().unwrap_err(), "UNAVAILABLE");
    fs::remove_file(
        harness
            .scratch
            .accounts()
            .join(format!("{}.vault", id_hex(&ID_B))),
    )
    .unwrap();

    plant_active(
        &harness.scratch.root,
        &id_hex(&ID_B),
        &vec![b'x'; MAX_ENVELOPE_BYTES + 1],
    );
    assert_public_error(&manager.list().unwrap_err(), "UNAVAILABLE");
    fs::remove_file(
        harness
            .scratch
            .accounts()
            .join(format!("{}.vault", id_hex(&ID_B))),
    )
    .unwrap();

    assert_eq!(listed_ids(&mut manager), vec![id_hex(&ID_A)]);

    let bound = Harness::new("entry-bound");
    let mut bound_manager = bound.manager();
    plant_active(
        &bound.scratch.root,
        &id_hex(&ID_A),
        &canonical_vault(&id_hex(&ID_A), "ZEC", "zec-testnet", 1),
    );
    for n in 0..1023u16 {
        write_private(
            &bound
                .scratch
                .accounts()
                .join(format!(".{:032x}.{:016x}.stage", n, n as u64)),
            b"stage",
        );
    }
    assert_eq!(listed_ids(&mut bound_manager), vec![id_hex(&ID_A)]);
    write_private(
        &bound
            .scratch
            .accounts()
            .join(format!(".{:032x}.{:016x}.stage", 1023u16, 1023u64)),
        b"overflow-stage",
    );
    assert_public_error(&bound_manager.list().unwrap_err(), "LIMIT");
}

#[test]
fn catalog_cap_is_256_without_running_per_account_kdf() {
    let mut harness = Harness::new("cap");
    harness
        .entropy
        .script_create(&ID_A, &SEED_A, 0xb1, 0xb2, 0x0e);
    let backup = harness.scratch.sibling("cap-backup.vault");
    write_private(
        &backup,
        &seal_bytes(
            ID_C,
            Asset::Zec,
            Network::ZecTestnet,
            1,
            PASSPHRASE,
            &SEED_B,
            0xb5,
            0xb6,
        ),
    );
    let mut manager = harness.manager();
    let prepared = manager
        .prepare_restore(ActionOrigin::NativeSurface, &backup, passphrase())
        .unwrap();
    assert_locked_software(prepared.summary(), &id_hex(&ID_C));

    for n in 0..256u16 {
        let id = format!("{n:032x}");
        plant_active(
            &harness.scratch.root,
            &id,
            &canonical_vault(&id, "ZEC", "zec-testnet", 1),
        );
    }
    write_private(
        &harness
            .scratch
            .accounts()
            .join(format!(".{:032x}.{:016x}.stage", 0u16, 1u64)),
        b"ignored",
    );

    let listed = manager.list().unwrap();
    assert_eq!(listed.len(), 256);
    assert!(listed.iter().all(|summary| {
        summary.locked
            && summary.asset == "ZEC"
            && summary.network == "zec-testnet"
            && summary.kind == "software"
    }));
    assert!(
        listed
            .windows(2)
            .all(|pair| pair[0].account_id < pair[1].account_id)
    );

    let before = vault_names(&harness.scratch.root);
    assert_eq!(before.len(), 256);
    assert_public_error(
        &manager
            .confirm_restore(ActionOrigin::NativeSurface, prepared, true)
            .unwrap_err(),
        "LIMIT",
    );
    assert_eq!(vault_names(&harness.scratch.root), before);

    assert_public_error(
        &manager
            .create_software(ActionOrigin::NativeSurface, passphrase())
            .unwrap_err(),
        "LIMIT",
    );
    assert_eq!(vault_names(&harness.scratch.root).len(), 256);

    let overflow = format!("{:032x}", 256u16);
    plant_active(
        &harness.scratch.root,
        &overflow,
        &canonical_vault(&overflow, "ZEC", "zec-testnet", 1),
    );
    assert_public_error(&manager.list().unwrap_err(), "LIMIT");
}

#[test]
fn second_manager_same_root_is_unavailable_until_first_drops() {
    let harness = Harness::new("root-lock");
    harness
        .entropy
        .script_create(&ID_A, &SEED_A, 0xc1, 0xc2, 0x10);
    let mut first = harness.manager();
    first
        .create_software(ActionOrigin::NativeSurface, passphrase())
        .unwrap();
    assert_eq!(listed_ids(&mut first), vec![id_hex(&ID_A)]);
    assert!(!unlocked(&mut first, &id_hex(&ID_A)));

    assert_public_error(
        &AccountManager::with_ports(
            &harness.scratch.root,
            SharedClock::new(1_000),
            ScriptedEntropy::new(),
            SharedWipes::default(),
        )
        .err()
        .expect("second manager for the same root should be rejected"),
        "UNAVAILABLE",
    );

    let unrelated = Harness::new("root-lock-other");
    let _other = unrelated.manager();

    drop(first);
    let mut reopened = harness.manager();
    let listed = reopened.list().unwrap();
    assert_eq!(listed.len(), 1);
    assert_locked_software(&listed[0], &id_hex(&ID_A));
}

#[test]
fn wal014_receiver_matches_seed_oracle_is_durable_private_and_preserves_vault() {
    let harness = Harness::new("wal014-receiver");
    harness
        .entropy
        .script_create(&ID_A, &SEED_A, 0xd1, 0xd2, 0x11);
    let account_id = id_hex(&ID_A);
    let vault_path = harness
        .scratch
        .accounts()
        .join(format!("{account_id}.vault"));

    let mut manager = harness.manager();
    manager
        .create_software(ActionOrigin::NativeSurface, passphrase())
        .unwrap();
    let vault_before = fs::read(&vault_path).unwrap();
    manager
        .unlock(ActionOrigin::NativeSurface, &account_id, passphrase())
        .unwrap();
    harness.wipes.clear();
    let first = manager
        .fresh_receiver(ActionOrigin::NativeSurface, &account_id)
        .unwrap();
    assert!(harness.wipes.labeled_wipe("zec-seed", SEED_A.len()));
    let second = manager
        .fresh_receiver(ActionOrigin::NativeSurface, &account_id)
        .unwrap();
    assert_scripted_receiver(&first, &account_id, &SEED_A, 0, 1);
    assert_scripted_receiver(&second, &account_id, &SEED_A, 1, 2);
    assert_ne!(first.receiver, second.receiver);
    assert_eq!(fs::read(&vault_path).unwrap(), vault_before);

    let zec_network = harness.scratch.root.join("zec-testnet");
    let zec_account = zec_network.join(&account_id);
    assert_eq!(mode(&zec_network), 0o700);
    assert_eq!(mode(&zec_account), 0o700);
    for file in ["wallet.sqlite3", "compact.sqlite3"] {
        let path = zec_account.join(file);
        assert!(fs::symlink_metadata(&path).unwrap().file_type().is_file());
        assert_eq!(mode(&path), 0o600);
    }
    assert_no_plaintext(&harness.scratch.root, &[&SEED_A, PASSPHRASE]);

    drop(manager);
    let mut restarted = harness.manager();
    restarted
        .unlock(ActionOrigin::NativeSurface, &account_id, passphrase())
        .unwrap();
    harness.wipes.clear();
    let third = restarted
        .fresh_receiver(ActionOrigin::NativeSurface, &account_id)
        .unwrap();
    assert!(harness.wipes.labeled_wipe("zec-seed", SEED_A.len()));
    assert_scripted_receiver(&third, &account_id, &SEED_A, 2, 3);
    assert_eq!(fs::read(&vault_path).unwrap(), vault_before);
    assert_no_plaintext(&harness.scratch.root, &[&SEED_A, PASSPHRASE]);
}

#[test]
fn wal014_receive_is_native_unlocked_valid_and_does_not_extend_idle_deadline() {
    let harness = Harness::new("wal014-gates");
    harness
        .entropy
        .script_create(&ID_A, &SEED_A, 0xd3, 0xd4, 0x12);
    harness
        .entropy
        .script_create(&ID_B, &SEED_B, 0xd5, 0xd6, 0x13);
    let account_a = id_hex(&ID_A);
    let account_b = id_hex(&ID_B);
    let zec_network = harness.scratch.root.join("zec-testnet");
    let mut manager = harness.manager();
    manager
        .create_software(ActionOrigin::NativeSurface, passphrase())
        .unwrap();
    manager
        .create_software(ActionOrigin::NativeSurface, passphrase())
        .unwrap();

    for origin in FORBIDDEN {
        harness.wipes.clear();
        assert_public_error(
            &manager.fresh_receiver(origin, &account_a).unwrap_err(),
            "UNAUTH",
        );
        assert!(!zec_network.exists());
        assert!(!harness.wipes.labeled_wipe("zec-seed", SEED_A.len()));
    }
    assert_public_error(
        &manager
            .fresh_receiver(ActionOrigin::NativeSurface, "../not-an-account")
            .unwrap_err(),
        "SCHEMA",
    );
    assert_public_error(
        &manager
            .fresh_receiver(ActionOrigin::NativeSurface, &id_hex(&ID_C))
            .unwrap_err(),
        "UNAVAILABLE",
    );
    assert_public_error(
        &manager
            .fresh_receiver(ActionOrigin::NativeSurface, &account_a)
            .unwrap_err(),
        "LOCKED",
    );
    assert!(!zec_network.exists());

    manager
        .unlock(ActionOrigin::NativeSurface, &account_a, passphrase())
        .unwrap();
    manager
        .unlock(ActionOrigin::NativeSurface, &account_b, passphrase())
        .unwrap();
    harness.clock.set(1_000 + AUTHORIZATION_IDLE_MILLIS - 1);
    let first = manager
        .fresh_receiver(ActionOrigin::NativeSurface, &account_a)
        .unwrap();
    assert_scripted_receiver(&first, &account_a, &SEED_A, 0, 1);

    harness.clock.set(1_000 + AUTHORIZATION_IDLE_MILLIS);
    assert_public_error(
        &manager
            .fresh_receiver(ActionOrigin::NativeSurface, &account_a)
            .unwrap_err(),
        "LOCKED",
    );
    assert_public_error(
        &manager
            .fresh_receiver(ActionOrigin::NativeSurface, &account_b)
            .unwrap_err(),
        "LOCKED",
    );
    assert!(!zec_network.join(&account_b).exists());
}

#[test]
fn wal014_corrupt_viewing_db_and_symlink_directory_are_never_replaced() {
    let mut harness = Harness::new("wal014-hostile-state");
    harness
        .entropy
        .script_create(&ID_A, &SEED_A, 0xd7, 0xd8, 0x14);
    harness
        .entropy
        .script_create(&ID_B, &SEED_B, 0xd9, 0xda, 0x15);
    let account_a = id_hex(&ID_A);
    let account_b = id_hex(&ID_B);
    let mut manager = harness.manager();
    manager
        .create_software(ActionOrigin::NativeSurface, passphrase())
        .unwrap();
    manager
        .create_software(ActionOrigin::NativeSurface, passphrase())
        .unwrap();
    manager
        .unlock(ActionOrigin::NativeSurface, &account_a, passphrase())
        .unwrap();
    manager
        .unlock(ActionOrigin::NativeSurface, &account_b, passphrase())
        .unwrap();
    manager
        .fresh_receiver(ActionOrigin::NativeSurface, &account_a)
        .unwrap();

    let wallet_db = harness
        .scratch
        .root
        .join("zec-testnet")
        .join(&account_a)
        .join("wallet.sqlite3");
    let corrupt = b"WAL014 intentionally corrupt existing wallet DB";
    fs::write(&wallet_db, corrupt).unwrap();
    fs::set_permissions(&wallet_db, fs::Permissions::from_mode(0o600)).unwrap();
    assert_public_error(
        &manager
            .fresh_receiver(ActionOrigin::NativeSurface, &account_a)
            .unwrap_err(),
        "UNAVAILABLE",
    );
    assert_eq!(fs::read(&wallet_db).unwrap(), corrupt);

    let foreign = harness.scratch.sibling("wal014-zec-symlink-target");
    fs::create_dir(&foreign).unwrap();
    fs::set_permissions(&foreign, fs::Permissions::from_mode(0o700)).unwrap();
    let sentinel = foreign.join("sentinel");
    write_private(&sentinel, b"do-not-touch");
    let linked_account = harness.scratch.root.join("zec-testnet").join(&account_b);
    symlink(&foreign, &linked_account).unwrap();
    assert_public_error(
        &manager
            .fresh_receiver(ActionOrigin::NativeSurface, &account_b)
            .unwrap_err(),
        "UNAVAILABLE",
    );
    assert!(
        fs::symlink_metadata(&linked_account)
            .unwrap()
            .file_type()
            .is_symlink()
    );
    assert_eq!(fs::read(&sentinel).unwrap(), b"do-not-touch");
    assert_eq!(fs::read_dir(&foreign).unwrap().count(), 1);
}

#[test]
fn wal014_foreign_ufvk_binding_refuses_without_advancing_issuance() {
    let harness = Harness::new("wal014-foreign-ufvk");
    harness
        .entropy
        .script_create(&ID_A, &SEED_A, 0xdb, 0xdc, 0x16);
    let account_id = id_hex(&ID_A);
    let mut manager = harness.manager();
    manager
        .create_software(ActionOrigin::NativeSurface, passphrase())
        .unwrap();
    manager
        .unlock(ActionOrigin::NativeSurface, &account_id, passphrase())
        .unwrap();
    manager
        .fresh_receiver(ActionOrigin::NativeSurface, &account_id)
        .unwrap();

    let wallet_db = harness
        .scratch
        .root
        .join("zec-testnet")
        .join(&account_id)
        .join("wallet.sqlite3");
    let connection = Connection::open(&wallet_db).unwrap();
    assert_eq!(
        connection
            .execute(
                "UPDATE ext_bitbook_accounts SET ufvk = ?1 WHERE account_id = ?2",
                params![oracle_ufvk(&SEED_B), &account_id],
            )
            .unwrap(),
        1
    );
    let read_state = |connection: &Connection| {
        connection
            .query_row(
                "SELECT r.last_diversifier_index, s.issued_at_sequence
                 FROM ext_bitbook_receiver_state r
                 JOIN ext_bitbook_sequence_state s USING (account_id)
                 WHERE r.account_id = ?1",
                [&account_id],
                |row| Ok((row.get::<_, Option<i64>>(0)?, row.get::<_, i64>(1)?)),
            )
            .unwrap()
    };
    let before = read_state(&connection);
    assert_eq!(before, (Some(0), 1));
    drop(connection);

    harness.wipes.clear();
    assert_public_error(
        &manager
            .fresh_receiver(ActionOrigin::NativeSurface, &account_id)
            .unwrap_err(),
        "UNAVAILABLE",
    );
    assert!(harness.wipes.labeled_wipe("zec-seed", SEED_A.len()));
    let connection = Connection::open(&wallet_db).unwrap();
    assert_eq!(read_state(&connection), before);
}
