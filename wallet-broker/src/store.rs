use core::fmt;
use std::collections::{HashMap, HashSet};
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::os::unix::fs::{FileTypeExt, MetadataExt, OpenOptionsExt, PermissionsExt};
use std::path::Path;

use crate::vault::{EntropyPort, MAX_ENVELOPE_BYTES, valid_account_id};

const LINUX_O_NONBLOCK: i32 = 0o4_000;
const LINUX_O_NOFOLLOW: i32 = 0o400_000;

pub const FULL_DIRECTORY_ROLLBACK_RESIDUAL: &str =
    "A full rollback of the broker data directory can also roll back its local high-water record.";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EntryKind {
    Regular,
    Symlink,
    Fifo,
    Directory,
    BlockDevice,
    CharacterDevice,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EntryInfo {
    pub kind: EntryKind,
    pub mode: u32,
    pub len: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FaultPoint {
    Directory,
    Acquire,
    Inspect,
    Read,
    Create,
    Write,
    Permission,
    FileSync,
    Replace,
    DirectorySync,
}

#[derive(Clone, Eq, PartialEq)]
pub struct StoreError {
    code: &'static str,
    message: &'static str,
}

impl StoreError {
    fn new(code: &'static str, message: &'static str) -> Self {
        Self { code, message }
    }

    pub fn injected(_point: FaultPoint) -> Self {
        Self::new("INJECTED", "Wallet storage unavailable")
    }

    pub fn account_busy() -> Self {
        Self::new("ACCOUNT_BUSY", "Wallet account is busy")
    }

    pub fn not_found() -> Self {
        Self::new("NOT_FOUND", "Wallet data was not found")
    }

    pub fn limit() -> Self {
        Self::new("LIMIT", "Wallet data exceeds its limit")
    }

    pub fn already_exists() -> Self {
        Self::new("ALREADY_EXISTS", "Wallet destination already exists")
    }

    fn unavailable() -> Self {
        Self::new("UNAVAILABLE", "Wallet storage unavailable")
    }

    fn schema() -> Self {
        Self::new("SCHEMA", "Wallet storage request is invalid")
    }

    fn locked() -> Self {
        Self::new("LOCKED", "Wallet locked")
    }

    fn replay() -> Self {
        Self::new("REPLAY", "Wallet backup is stale")
    }

    fn wrong_network() -> Self {
        Self::new("WRONG_NETWORK", "Wallet backup does not match")
    }

    fn state_corrupt() -> Self {
        Self::new("STATE_CORRUPT", "Wallet state is unavailable")
    }

    pub fn code(&self) -> &'static str {
        self.code
    }

    pub fn public_message(&self) -> &'static str {
        self.message
    }

    pub fn metadata(&self) -> Option<&RestoreCandidate> {
        None
    }
}

impl fmt::Debug for StoreError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("StoreError")
            .field("code", &self.code)
            .finish()
    }
}

impl fmt::Display for StoreError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.message)
    }
}

impl std::error::Error for StoreError {}

pub trait StorePort {
    fn ensure_directory(&mut self, path: &str, mode: u32) -> Result<(), StoreError>;
    fn acquire_account(&mut self, account_id: &str) -> Result<(), StoreError>;
    fn release_account(&mut self, account_id: &str);
    fn inspect(&mut self, path: &str) -> Result<EntryInfo, StoreError>;
    fn read_bounded(&mut self, path: &str, maximum: usize) -> Result<Vec<u8>, StoreError>;
    fn create_exclusive(&mut self, path: &str, mode: u32) -> Result<(), StoreError>;
    fn write_all(&mut self, path: &str, bytes: &[u8]) -> Result<(), StoreError>;
    fn set_permissions(&mut self, path: &str, mode: u32) -> Result<(), StoreError>;
    fn sync_file(&mut self, path: &str) -> Result<(), StoreError>;
    fn replace_atomic(&mut self, staging: &str, active: &str) -> Result<(), StoreError>;
    fn sync_directory(&mut self, path: &str) -> Result<(), StoreError>;
}

pub struct VaultStore<P: StorePort> {
    root: String,
    port: P,
    pending: HashMap<String, String>,
}

impl<P: StorePort> VaultStore<P> {
    pub fn new(root: impl AsRef<Path>, port: P) -> Result<Self, StoreError> {
        let root = root
            .as_ref()
            .to_str()
            .filter(|value| !value.is_empty())
            .ok_or_else(StoreError::schema)?
            .to_owned();
        Ok(Self {
            root,
            port,
            pending: HashMap::new(),
        })
    }

    pub fn port(&self) -> &P {
        &self.port
    }

    pub fn initialize(&mut self) -> Result<(), StoreError> {
        self.port.ensure_directory(&self.root, 0o700)
    }

    pub fn write_active(
        &mut self,
        account_id: &str,
        bytes: &[u8],
        entropy: &mut dyn EntropyPort,
    ) -> Result<(), StoreError> {
        validate_account(account_id)?;
        if bytes.is_empty() || bytes.len() > MAX_ENVELOPE_BYTES {
            return Err(StoreError::limit());
        }
        self.port.acquire_account(account_id)?;
        let result = self.write_active_locked(account_id, bytes, entropy);
        self.port.release_account(account_id);
        result
    }

    fn write_active_locked(
        &mut self,
        account_id: &str,
        bytes: &[u8],
        entropy: &mut dyn EntropyPort,
    ) -> Result<(), StoreError> {
        let active = self.active_path(account_id);
        let mut random = [0u8; 8];
        entropy
            .fill("staging-name", &mut random)
            .map_err(|_| StoreError::unavailable())?;
        let staging = format!("{}/.{}.{}.stage", self.root, account_id, hex(&random));
        match self.port.create_exclusive(&staging, 0o600) {
            Ok(()) => {}
            Err(error) if error.code() == "ALREADY_EXISTS" => {
                return Err(StoreError::account_busy());
            }
            Err(error) => return Err(error),
        }
        self.pending.insert(account_id.to_owned(), staging.clone());
        self.port.set_permissions(&staging, 0o600)?;
        self.port.write_all(&staging, bytes)?;
        self.port.sync_file(&staging)?;
        self.port.replace_atomic(&staging, &active)?;
        self.pending.remove(account_id);
        self.port.sync_directory(&self.root)
    }

    pub fn recover_account(&mut self, account_id: &str) -> Result<(), StoreError> {
        validate_account(account_id)?;
        self.port.acquire_account(account_id)?;
        let result = (|| {
            let staging = self
                .pending
                .get(account_id)
                .cloned()
                .ok_or_else(StoreError::not_found)?;
            self.port.sync_file(&staging)?;
            self.port
                .replace_atomic(&staging, &self.active_path(account_id))?;
            self.pending.remove(account_id);
            self.port.sync_directory(&self.root)
        })();
        self.port.release_account(account_id);
        result
    }

    pub fn read_active(&mut self, account_id: &str) -> Result<Vec<u8>, StoreError> {
        validate_account(account_id)?;
        let path = self.active_path(account_id);
        let info = self.port.inspect(&path)?;
        validate_regular(&info)?;
        if info.len as usize > MAX_ENVELOPE_BYTES {
            return Err(StoreError::limit());
        }
        self.port.read_bounded(&path, MAX_ENVELOPE_BYTES)
    }

    pub fn export_encrypted(
        &mut self,
        account_id: &str,
        destination: &str,
    ) -> Result<(), StoreError> {
        validate_account(account_id)?;
        let active = self.active_path(account_id);
        if destination.is_empty() || destination == active {
            return Err(StoreError::schema());
        }
        match self.port.inspect(destination) {
            Ok(_) => return Err(StoreError::already_exists()),
            Err(error) if error.code() == "NOT_FOUND" => {}
            Err(error) => return Err(error),
        }
        let bytes = self.read_active(account_id)?;
        self.port.create_exclusive(destination, 0o600)?;
        self.port.set_permissions(destination, 0o600)?;
        self.port.write_all(destination, &bytes)?;
        self.port.sync_file(destination)
    }

    fn active_path(&self, account_id: &str) -> String {
        format!("{}/{}.vault", self.root, account_id)
    }
}

fn validate_account(account_id: &str) -> Result<(), StoreError> {
    if valid_account_id(account_id) {
        Ok(())
    } else {
        Err(StoreError::schema())
    }
}

fn validate_regular(info: &EntryInfo) -> Result<(), StoreError> {
    if info.kind != EntryKind::Regular || info.mode & 0o777 != 0o600 {
        return Err(StoreError::unavailable());
    }
    Ok(())
}

fn hex(bytes: &[u8]) -> String {
    const TABLE: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(TABLE[(byte >> 4) as usize] as char);
        output.push(TABLE[(byte & 15) as usize] as char);
    }
    output
}

#[derive(Default)]
pub struct LinuxStorePort {
    held: HashSet<String>,
}

impl LinuxStorePort {
    pub fn new() -> Self {
        Self::default()
    }
}

impl StorePort for LinuxStorePort {
    fn ensure_directory(&mut self, path: &str, mode: u32) -> Result<(), StoreError> {
        match fs::symlink_metadata(path) {
            Ok(metadata) if metadata.file_type().is_dir() => {}
            Ok(_) => return Err(StoreError::unavailable()),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                fs::create_dir_all(path).map_err(|_| StoreError::unavailable())?;
            }
            Err(_) => return Err(StoreError::unavailable()),
        }
        fs::set_permissions(path, fs::Permissions::from_mode(mode))
            .map_err(|_| StoreError::unavailable())
    }

    fn acquire_account(&mut self, account_id: &str) -> Result<(), StoreError> {
        if !self.held.insert(account_id.to_owned()) {
            return Err(StoreError::account_busy());
        }
        Ok(())
    }

    fn release_account(&mut self, account_id: &str) {
        self.held.remove(account_id);
    }

    fn inspect(&mut self, path: &str) -> Result<EntryInfo, StoreError> {
        let metadata = fs::symlink_metadata(path).map_err(map_io)?;
        let file_type = metadata.file_type();
        let kind = if file_type.is_symlink() {
            EntryKind::Symlink
        } else if file_type.is_file() {
            EntryKind::Regular
        } else if file_type.is_dir() {
            EntryKind::Directory
        } else if file_type.is_fifo() {
            EntryKind::Fifo
        } else if file_type.is_block_device() {
            EntryKind::BlockDevice
        } else {
            EntryKind::CharacterDevice
        };
        Ok(EntryInfo {
            kind,
            mode: metadata.mode() & 0o777,
            len: metadata.len(),
        })
    }

    fn read_bounded(&mut self, path: &str, maximum: usize) -> Result<Vec<u8>, StoreError> {
        let file = open_existing_regular(path, true, false, true)?;
        let mut bytes = Vec::new();
        file.take(maximum as u64 + 1)
            .read_to_end(&mut bytes)
            .map_err(|_| StoreError::unavailable())?;
        if bytes.len() > maximum {
            return Err(StoreError::limit());
        }
        Ok(bytes)
    }

    fn create_exclusive(&mut self, path: &str, mode: u32) -> Result<(), StoreError> {
        OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(mode)
            .open(path)
            .map(|_| ())
            .map_err(map_io)
    }

    fn write_all(&mut self, path: &str, bytes: &[u8]) -> Result<(), StoreError> {
        let mut file = open_existing_regular(path, false, true, true)?;
        file.set_len(0).map_err(|_| StoreError::unavailable())?;
        file.write_all(bytes).map_err(|_| StoreError::unavailable())
    }

    fn set_permissions(&mut self, path: &str, mode: u32) -> Result<(), StoreError> {
        open_existing_regular(path, true, false, false)?
            .set_permissions(fs::Permissions::from_mode(mode))
            .map_err(|_| StoreError::unavailable())
    }

    fn sync_file(&mut self, path: &str) -> Result<(), StoreError> {
        open_existing_regular(path, true, true, true)?
            .sync_all()
            .map_err(|_| StoreError::unavailable())
    }

    fn replace_atomic(&mut self, staging: &str, active: &str) -> Result<(), StoreError> {
        fs::rename(staging, active).map_err(|_| StoreError::unavailable())
    }

    fn sync_directory(&mut self, path: &str) -> Result<(), StoreError> {
        File::open(path)
            .and_then(|directory| directory.sync_all())
            .map_err(|_| StoreError::unavailable())
    }
}

fn open_existing_regular(
    path: &str,
    read: bool,
    write: bool,
    require_private_mode: bool,
) -> Result<File, StoreError> {
    let file = OpenOptions::new()
        .read(read)
        .write(write)
        .custom_flags(LINUX_O_NOFOLLOW | LINUX_O_NONBLOCK)
        .open(path)
        .map_err(|_| StoreError::unavailable())?;
    let metadata = file.metadata().map_err(|_| StoreError::unavailable())?;
    if !metadata.file_type().is_file() || (require_private_mode && metadata.mode() & 0o777 != 0o600)
    {
        return Err(StoreError::unavailable());
    }
    Ok(file)
}

fn map_io(error: std::io::Error) -> StoreError {
    match error.kind() {
        std::io::ErrorKind::NotFound => StoreError::not_found(),
        std::io::ErrorKind::AlreadyExists => StoreError::already_exists(),
        _ => StoreError::unavailable(),
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RestoreCandidate {
    pub authenticated: bool,
    pub account_id: String,
    pub asset: String,
    pub network: String,
    pub epoch: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum RestoreState {
    Empty,
    Authenticated(u64),
    Corrupt,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RestoreContext {
    account_id: String,
    asset: String,
    network: String,
    state: RestoreState,
}

impl RestoreContext {
    pub fn empty(account_id: &str, asset: &str, network: &str) -> Self {
        Self {
            account_id: account_id.to_owned(),
            asset: asset.to_owned(),
            network: network.to_owned(),
            state: RestoreState::Empty,
        }
    }

    pub fn authenticated(account_id: &str, asset: &str, network: &str, epoch: u64) -> Self {
        Self {
            account_id: account_id.to_owned(),
            asset: asset.to_owned(),
            network: network.to_owned(),
            state: RestoreState::Authenticated(epoch),
        }
    }

    pub fn corrupt(account_id: &str, asset: &str, network: &str) -> Self {
        Self {
            account_id: account_id.to_owned(),
            asset: asset.to_owned(),
            network: network.to_owned(),
            state: RestoreState::Corrupt,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RestoreDecision {
    Cancelled,
    Replace,
}

pub fn evaluate_restore(
    context: &RestoreContext,
    candidate: &RestoreCandidate,
    confirmed: bool,
) -> Result<RestoreDecision, StoreError> {
    if !candidate.authenticated {
        return Err(StoreError::locked());
    }
    if context.state == RestoreState::Corrupt {
        return Err(StoreError::state_corrupt());
    }
    if candidate.account_id != context.account_id
        || candidate.asset != context.asset
        || candidate.network != context.network
    {
        return Err(StoreError::wrong_network());
    }
    if let RestoreState::Authenticated(current) = context.state
        && candidate.epoch <= current
    {
        return Err(StoreError::replay());
    }
    if confirmed {
        Ok(RestoreDecision::Replace)
    } else {
        Ok(RestoreDecision::Cancelled)
    }
}

pub fn next_epoch(current: Option<u64>) -> Result<u64, StoreError> {
    match current {
        None => Ok(1),
        Some(value) => value.checked_add(1).ok_or_else(StoreError::state_corrupt),
    }
}
