use core::fmt;
use std::fs::{self, File, OpenOptions};
use std::os::unix::fs::{MetadataExt, OpenOptionsExt};
use std::path::Path;
use std::time::Instant;

use serde::Serialize;
use zeroize::{Zeroize, Zeroizing};

use crate::native::ActionOrigin;
use crate::session::{ClockError, MonotonicClock, SessionEvent, SessionManager};
use crate::store::{EntryKind, LinuxStorePort, StoreError, StorePort, VaultStore};
use crate::vault::{
    Asset, EntropyPort, MAX_ENVELOPE_BYTES, Network, OsEntropy, SecretBytes, VaultError,
    VaultMetadata, VaultWorkObserver, WipeEvent, WipeObserver, open_vault_bytes, parse_vault,
    seal_vault, valid_account_id,
};

const LINUX_O_NOFOLLOW: i32 = 0o400_000;
const LINUX_O_DIRECTORY: i32 = 0o200_000;
const MAX_ACTIVE_ACCOUNTS: usize = 256;
const MAX_DIRECTORY_ENTRIES: usize = 1_024;
const ACCOUNT_ID_BYTES: usize = 16;
const ACCOUNT_SEED_BYTES: usize = 32;
const STAGE_NAME_LEN: usize = 1 + 32 + 1 + 16 + 6;

pub struct SystemClock {
    origin: Instant,
}

impl SystemClock {
    pub fn new() -> Self {
        Self {
            origin: Instant::now(),
        }
    }
}

impl Default for SystemClock {
    fn default() -> Self {
        Self::new()
    }
}

impl MonotonicClock for SystemClock {
    fn now_millis(&mut self) -> Result<u64, ClockError> {
        u64::try_from(self.origin.elapsed().as_millis()).map_err(|_| ClockError::Unavailable)
    }
}

pub struct SilentWipes;

impl WipeObserver for SilentWipes {
    fn observe(&mut self, _event: WipeEvent) {}
}

struct IgnoreWork;

impl VaultWorkObserver for IgnoreWork {
    fn before_allocation(&mut self, _bytes: usize) -> Result<(), VaultError> {
        Ok(())
    }

    fn before_kdf(&mut self) {}
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct AccountSummary {
    pub account_id: String,
    pub asset: &'static str,
    pub network: &'static str,
    pub kind: &'static str,
    pub locked: bool,
}

pub struct AccountError {
    code: &'static str,
    message: &'static str,
}

impl AccountError {
    fn new(code: &'static str, message: &'static str) -> Self {
        Self { code, message }
    }

    fn unauth() -> Self {
        Self::new("UNAUTH", "Wallet action is not authorized")
    }

    fn schema() -> Self {
        Self::new("SCHEMA", "Wallet data is invalid")
    }

    fn locked() -> Self {
        Self::new("LOCKED", "Wallet locked")
    }

    fn unavailable() -> Self {
        Self::new("UNAVAILABLE", "Wallet unavailable")
    }

    fn limit() -> Self {
        Self::new("LIMIT", "Wallet data exceeds its limit")
    }

    fn already_exists() -> Self {
        Self::new("ALREADY_EXISTS", "Wallet destination already exists")
    }

    pub fn code(&self) -> &'static str {
        self.code
    }
}

impl fmt::Debug for AccountError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AccountError")
            .field("code", &self.code)
            .finish()
    }
}

impl fmt::Display for AccountError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.message)
    }
}

impl std::error::Error for AccountError {}

pub struct PreparedRestore {
    bytes: Vec<u8>,
    summary: AccountSummary,
}

impl PreparedRestore {
    pub fn summary(&self) -> &AccountSummary {
        &self.summary
    }
}

pub struct AccountManager<C: MonotonicClock, E: EntropyPort, W: WipeObserver> {
    _root_guard: File,
    accounts_dir: String,
    store: VaultStore<LinuxStorePort>,
    sessions: SessionManager<C, SilentWipes>,
    entropy: E,
    wipes: W,
}

pub type LocalAccountManager = AccountManager<SystemClock, OsEntropy, SilentWipes>;

impl LocalAccountManager {
    pub fn open(broker_root: &Path) -> Result<Self, AccountError> {
        Self::with_ports(broker_root, SystemClock::new(), OsEntropy, SilentWipes)
    }
}

impl<C: MonotonicClock, E: EntropyPort, W: WipeObserver> AccountManager<C, E, W> {
    pub fn with_ports(
        broker_root: &Path,
        clock: C,
        entropy: E,
        wipes: W,
    ) -> Result<Self, AccountError> {
        let root_guard = lock_broker_root(broker_root)?;
        let accounts_dir = accounts_dir_path(broker_root)?;
        ensure_accounts_directory(&accounts_dir)?;
        let store = VaultStore::new(&accounts_dir, LinuxStorePort::new()).map_err(map_store)?;
        Ok(Self {
            _root_guard: root_guard,
            accounts_dir,
            store,
            sessions: SessionManager::new(clock, SilentWipes),
            entropy,
            wipes,
        })
    }

    pub fn list(&mut self) -> Result<Vec<AccountSummary>, AccountError> {
        self.sessions
            .check_deadlines()
            .map_err(|_| AccountError::unavailable())?;
        let ids = self.load_catalog()?;
        Ok(ids
            .into_iter()
            .map(|account_id| {
                software_summary(&account_id, !self.sessions.is_unlocked(&account_id))
            })
            .collect())
    }

    pub fn create_software(
        &mut self,
        origin: ActionOrigin,
        mut passphrase: SecretBytes,
    ) -> Result<AccountSummary, AccountError> {
        if let Err(error) = self.require_native(origin) {
            wipe_passphrase(&mut passphrase, &mut self.wipes);
            return Err(error);
        }
        let catalog = match self.load_catalog() {
            Ok(catalog) => catalog,
            Err(error) => {
                wipe_passphrase(&mut passphrase, &mut self.wipes);
                return Err(error);
            }
        };
        if catalog.len() >= MAX_ACTIVE_ACCOUNTS {
            wipe_passphrase(&mut passphrase, &mut self.wipes);
            return Err(AccountError::limit());
        }

        let mut id_bytes = [0u8; ACCOUNT_ID_BYTES];
        if self.entropy.fill("account-id", &mut id_bytes).is_err() {
            wipe_passphrase(&mut passphrase, &mut self.wipes);
            return Err(AccountError::unavailable());
        }
        let account_id = hex_encode(&id_bytes);
        if catalog.iter().any(|existing| existing == &account_id) {
            wipe_passphrase(&mut passphrase, &mut self.wipes);
            return Err(AccountError::already_exists());
        }

        let mut seed_bytes = Zeroizing::new([0u8; ACCOUNT_SEED_BYTES]);
        if self.entropy.fill("account-seed", &mut *seed_bytes).is_err() {
            seed_bytes.zeroize();
            wipe_passphrase(&mut passphrase, &mut self.wipes);
            return Err(AccountError::unavailable());
        }
        let mut plaintext = match SecretBytes::new(seed_bytes.to_vec()) {
            Ok(secret) => secret,
            Err(_) => {
                seed_bytes.zeroize();
                wipe_passphrase(&mut passphrase, &mut self.wipes);
                return Err(AccountError::unavailable());
            }
        };
        seed_bytes.zeroize();

        let metadata = match VaultMetadata::new(id_bytes, Asset::Zec, Network::ZecTestnet, 1) {
            Ok(metadata) => metadata,
            Err(_) => {
                wipe_passphrase(&mut passphrase, &mut self.wipes);
                plaintext.wipe_with("plaintext", &mut self.wipes);
                return Err(AccountError::unavailable());
            }
        };
        let envelope = match seal_vault(
            &metadata,
            &mut passphrase,
            &mut plaintext,
            &mut self.entropy,
            &mut self.wipes,
        ) {
            Ok(envelope) => envelope,
            Err(_) => return Err(AccountError::unavailable()),
        };
        self.store
            .write_active(&account_id, envelope.as_bytes(), &mut self.entropy)
            .map_err(map_store)?;
        Ok(software_summary(&account_id, true))
    }

    pub fn unlock(
        &mut self,
        origin: ActionOrigin,
        account_id: &str,
        mut passphrase: SecretBytes,
    ) -> Result<(), AccountError> {
        if let Err(error) = self.require_native(origin) {
            wipe_passphrase(&mut passphrase, &mut self.wipes);
            return Err(error);
        }
        if !valid_account_id(account_id) {
            wipe_passphrase(&mut passphrase, &mut self.wipes);
            return Err(AccountError::schema());
        }
        let bytes = match self.read_active_account(account_id) {
            Ok(bytes) => bytes,
            Err(error) => {
                wipe_passphrase(&mut passphrase, &mut self.wipes);
                return Err(error);
            }
        };
        let envelope = match parse_vault(&bytes, &mut IgnoreWork) {
            Ok(envelope) => envelope,
            Err(_) => {
                wipe_passphrase(&mut passphrase, &mut self.wipes);
                return Err(AccountError::locked());
            }
        };
        if envelope.metadata().account_id_hex() != account_id
            || !supported_vault(envelope.metadata())
        {
            wipe_passphrase(&mut passphrase, &mut self.wipes);
            return Err(AccountError::unavailable());
        }
        let mut plaintext =
            match open_vault_bytes(&bytes, &mut passphrase, &mut IgnoreWork, &mut self.wipes) {
                Ok(plaintext) => plaintext,
                Err(error) => return Err(map_unlock_open(error)),
            };
        if plaintext.len() != ACCOUNT_SEED_BYTES {
            plaintext.wipe_with("plaintext", &mut self.wipes);
            return Err(AccountError::locked());
        }
        self.sessions
            .unlock(account_id, plaintext)
            .map_err(|_| AccountError::unavailable())
    }

    pub fn lock(&mut self, account_id: &str) -> Result<(), AccountError> {
        if !valid_account_id(account_id) {
            return Err(AccountError::schema());
        }
        let catalog = self.load_catalog()?;
        if !catalog.iter().any(|existing| existing == account_id) {
            return Err(AccountError::unavailable());
        }
        self.sessions
            .handle(account_id, SessionEvent::ManualLock)
            .map_err(|_| AccountError::unavailable())
    }

    pub fn lock_all(&mut self) {
        let _ = self.sessions.handle("", SessionEvent::BrokerQuit);
    }

    pub fn tick(&mut self) -> Result<(), AccountError> {
        self.sessions
            .check_deadlines()
            .map_err(|_| AccountError::unavailable())
    }

    pub fn export_encrypted(
        &mut self,
        origin: ActionOrigin,
        account_id: &str,
        destination: &Path,
    ) -> Result<(), AccountError> {
        self.require_native(origin)?;
        if !valid_account_id(account_id) {
            return Err(AccountError::schema());
        }
        let destination = absolute_path(destination)?;
        self.validate_accounts_dir()?;
        self.read_active_account(account_id)
            .map_err(|_| AccountError::unavailable())?;
        self.store
            .export_encrypted(account_id, &destination)
            .map_err(map_store)
    }

    pub fn prepare_restore(
        &mut self,
        origin: ActionOrigin,
        source: &Path,
        mut passphrase: SecretBytes,
    ) -> Result<PreparedRestore, AccountError> {
        if let Err(error) = self.require_native(origin) {
            wipe_passphrase(&mut passphrase, &mut self.wipes);
            return Err(error);
        }
        let source = match absolute_path(source) {
            Ok(source) => source,
            Err(error) => {
                wipe_passphrase(&mut passphrase, &mut self.wipes);
                return Err(error);
            }
        };
        let bytes = match read_restore_source(&source) {
            Ok(bytes) => bytes,
            Err(error) => {
                wipe_passphrase(&mut passphrase, &mut self.wipes);
                return Err(error);
            }
        };
        let envelope = match parse_vault(&bytes, &mut IgnoreWork) {
            Ok(envelope) => envelope,
            Err(error) => {
                wipe_passphrase(&mut passphrase, &mut self.wipes);
                return Err(map_prepare_parse(error));
            }
        };
        if !supported_vault(envelope.metadata()) {
            wipe_passphrase(&mut passphrase, &mut self.wipes);
            return Err(AccountError::unavailable());
        }
        let mut plaintext =
            match open_vault_bytes(&bytes, &mut passphrase, &mut IgnoreWork, &mut self.wipes) {
                Ok(plaintext) => plaintext,
                Err(error) => return Err(map_unlock_open(error)),
            };
        if plaintext.len() != ACCOUNT_SEED_BYTES {
            plaintext.wipe_with("plaintext", &mut self.wipes);
            return Err(AccountError::locked());
        }
        plaintext.wipe_with("plaintext", &mut self.wipes);
        Ok(PreparedRestore {
            bytes,
            summary: software_summary(&envelope.metadata().account_id_hex(), true),
        })
    }

    pub fn confirm_restore(
        &mut self,
        origin: ActionOrigin,
        prepared: PreparedRestore,
        confirmed: bool,
    ) -> Result<Option<AccountSummary>, AccountError> {
        self.require_native(origin)?;
        if !confirmed {
            return Ok(None);
        }
        let catalog = self.load_catalog()?;
        if catalog
            .iter()
            .any(|existing| existing == &prepared.summary.account_id)
        {
            return Err(AccountError::already_exists());
        }
        if catalog.len() >= MAX_ACTIVE_ACCOUNTS {
            return Err(AccountError::limit());
        }
        self.store
            .write_active(
                &prepared.summary.account_id,
                &prepared.bytes,
                &mut self.entropy,
            )
            .map_err(map_store)?;
        Ok(Some(prepared.summary))
    }

    fn require_native(&self, origin: ActionOrigin) -> Result<(), AccountError> {
        if origin == ActionOrigin::NativeSurface {
            Ok(())
        } else {
            Err(AccountError::unauth())
        }
    }

    fn validate_accounts_dir(&self) -> Result<(), AccountError> {
        match inspect_path(&self.accounts_dir) {
            Ok(info) if info.kind == EntryKind::Directory && info.mode == 0o700 => Ok(()),
            _ => Err(AccountError::unavailable()),
        }
    }

    fn load_catalog(&mut self) -> Result<Vec<String>, AccountError> {
        self.validate_accounts_dir()?;
        let mut names = Vec::new();
        let entries = fs::read_dir(&self.accounts_dir).map_err(|_| AccountError::unavailable())?;
        for entry in entries {
            let entry = entry.map_err(|_| AccountError::unavailable())?;
            let name = entry
                .file_name()
                .into_string()
                .map_err(|_| AccountError::unavailable())?;
            if name.is_empty() || name.contains('/') || name == "." || name == ".." {
                return Err(AccountError::unavailable());
            }
            names.push(name);
            if names.len() > MAX_DIRECTORY_ENTRIES {
                self.validate_accounts_dir()?;
                return Err(AccountError::limit());
            }
        }
        self.validate_accounts_dir()?;

        let mut ids = Vec::new();
        for name in names {
            if is_stage_name(&name) {
                continue;
            }
            let Some(account_id) = vault_account_id(&name) else {
                return Err(AccountError::unavailable());
            };
            if ids.len() >= MAX_ACTIVE_ACCOUNTS {
                return Err(AccountError::limit());
            }
            let path = format!("{}/{}", self.accounts_dir, name);
            let info = inspect_path(&path).map_err(map_catalog_store)?;
            if info.kind != EntryKind::Regular || info.mode != 0o600 {
                return Err(AccountError::unavailable());
            }
            if info.len as usize > MAX_ENVELOPE_BYTES {
                return Err(AccountError::unavailable());
            }
            let bytes = read_bounded_path(&path, MAX_ENVELOPE_BYTES).map_err(map_catalog_store)?;
            let envelope =
                parse_vault(&bytes, &mut IgnoreWork).map_err(|_| AccountError::unavailable())?;
            if envelope.metadata().account_id_hex() != account_id
                || !supported_vault(envelope.metadata())
            {
                return Err(AccountError::unavailable());
            }
            ids.push(account_id);
        }
        ids.sort();
        Ok(ids)
    }

    fn read_active_account(&mut self, account_id: &str) -> Result<Vec<u8>, AccountError> {
        self.validate_accounts_dir()?;
        let path = format!("{}/{}.vault", self.accounts_dir, account_id);
        let info = inspect_path(&path).map_err(map_catalog_store)?;
        if info.kind != EntryKind::Regular || info.mode != 0o600 {
            return Err(AccountError::unavailable());
        }
        if info.len as usize > MAX_ENVELOPE_BYTES {
            return Err(AccountError::unavailable());
        }
        let bytes = self
            .store
            .read_active(account_id)
            .map_err(map_catalog_store)?;
        let envelope = parse_vault(&bytes, &mut IgnoreWork).map_err(|_| AccountError::locked())?;
        if envelope.metadata().account_id_hex() != account_id
            || !supported_vault(envelope.metadata())
        {
            return Err(AccountError::unavailable());
        }
        Ok(bytes)
    }
}

impl<C: MonotonicClock, E: EntropyPort, W: WipeObserver> Drop for AccountManager<C, E, W> {
    fn drop(&mut self) {
        self.lock_all();
    }
}

fn lock_broker_root(path: &Path) -> Result<File, AccountError> {
    if path.as_os_str().is_empty() || !path.is_absolute() || path.to_str().is_none() {
        return Err(AccountError::unavailable());
    }
    let metadata = fs::symlink_metadata(path).map_err(|_| AccountError::unavailable())?;
    if metadata.file_type().is_symlink()
        || !metadata.file_type().is_dir()
        || metadata.mode() & 0o777 != 0o700
    {
        return Err(AccountError::unavailable());
    }
    let file = OpenOptions::new()
        .read(true)
        .custom_flags(LINUX_O_DIRECTORY | LINUX_O_NOFOLLOW)
        .open(path)
        .map_err(|_| AccountError::unavailable())?;
    let opened = file.metadata().map_err(|_| AccountError::unavailable())?;
    if !opened.file_type().is_dir() || opened.mode() & 0o777 != 0o700 {
        return Err(AccountError::unavailable());
    }
    file.try_lock().map_err(|_| AccountError::unavailable())?;
    Ok(file)
}

fn accounts_dir_path(root: &Path) -> Result<String, AccountError> {
    root.join("accounts")
        .to_str()
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
        .ok_or_else(AccountError::unavailable)
}

fn ensure_accounts_directory(accounts_dir: &str) -> Result<(), AccountError> {
    let mut port = LinuxStorePort::new();
    match port.inspect(accounts_dir) {
        Err(error) if error.code() == "NOT_FOUND" => {
            port.ensure_directory(accounts_dir, 0o700)
                .map_err(map_store)?;
        }
        Err(_) => return Err(AccountError::unavailable()),
        Ok(info) => {
            if info.kind == EntryKind::Directory && info.mode == 0o700 {
                return Ok(());
            }
            return Err(AccountError::unavailable());
        }
    }
    match inspect_path(accounts_dir) {
        Ok(info) if info.kind == EntryKind::Directory && info.mode == 0o700 => Ok(()),
        _ => Err(AccountError::unavailable()),
    }
}

fn inspect_path(path: &str) -> Result<crate::store::EntryInfo, StoreError> {
    LinuxStorePort::new().inspect(path)
}

fn read_bounded_path(path: &str, maximum: usize) -> Result<Vec<u8>, StoreError> {
    LinuxStorePort::new().read_bounded(path, maximum)
}

fn read_restore_source(path: &str) -> Result<Vec<u8>, AccountError> {
    let info = inspect_path(path).map_err(map_store)?;
    if info.kind != EntryKind::Regular || info.mode != 0o600 {
        return Err(AccountError::unavailable());
    }
    if info.len as usize > MAX_ENVELOPE_BYTES {
        return Err(AccountError::limit());
    }
    read_bounded_path(path, MAX_ENVELOPE_BYTES).map_err(map_store)
}

fn absolute_path(path: &Path) -> Result<String, AccountError> {
    path.to_str()
        .filter(|value| !value.is_empty() && path.is_absolute())
        .map(ToOwned::to_owned)
        .ok_or_else(AccountError::schema)
}

fn software_summary(account_id: &str, locked: bool) -> AccountSummary {
    AccountSummary {
        account_id: account_id.to_owned(),
        asset: "ZEC",
        network: "zec-testnet",
        kind: "software",
        locked,
    }
}

fn supported_vault(metadata: &VaultMetadata) -> bool {
    metadata.asset() == Asset::Zec && metadata.network() == Network::ZecTestnet
}

fn wipe_passphrase(passphrase: &mut SecretBytes, wipes: &mut dyn WipeObserver) {
    passphrase.wipe_with("passphrase", wipes);
}

fn is_stage_name(name: &str) -> bool {
    let bytes = name.as_bytes();
    bytes.len() == STAGE_NAME_LEN
        && bytes[0] == b'.'
        && bytes[33] == b'.'
        && name.ends_with(".stage")
        && is_lower_hex(&name[1..33])
        && is_lower_hex(&name[34..50])
}

fn vault_account_id(name: &str) -> Option<String> {
    let account_id = name.strip_suffix(".vault")?;
    valid_account_id(account_id).then(|| account_id.to_owned())
}

fn is_lower_hex(value: &str) -> bool {
    !value.is_empty()
        && value
            .bytes()
            .all(|byte| matches!(byte, b'0'..=b'9' | b'a'..=b'f'))
}

fn hex_encode(bytes: &[u8]) -> String {
    const TABLE: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(TABLE[(byte >> 4) as usize] as char);
        output.push(TABLE[(byte & 0x0f) as usize] as char);
    }
    output
}

fn map_store(error: StoreError) -> AccountError {
    match error.code() {
        "ALREADY_EXISTS" => AccountError::already_exists(),
        "LIMIT" => AccountError::limit(),
        "SCHEMA" => AccountError::schema(),
        "LOCKED" => AccountError::locked(),
        _ => AccountError::unavailable(),
    }
}

fn map_catalog_store(_: StoreError) -> AccountError {
    AccountError::unavailable()
}

fn map_prepare_parse(error: VaultError) -> AccountError {
    match error.code() {
        "SCHEMA" => AccountError::schema(),
        "LIMIT" => AccountError::limit(),
        "LOCKED" => AccountError::locked(),
        _ => AccountError::unavailable(),
    }
}

fn map_unlock_open(error: VaultError) -> AccountError {
    match error.code() {
        "LOCKED" => AccountError::locked(),
        "SCHEMA" => AccountError::schema(),
        "LIMIT" => AccountError::limit(),
        _ => AccountError::unavailable(),
    }
}
