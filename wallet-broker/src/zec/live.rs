use std::collections::BTreeMap;
use std::fs::{self, OpenOptions};
use std::net::IpAddr;
use std::num::NonZeroU32;
use std::os::unix::fs::{MetadataExt, OpenOptionsExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use prost::Message;
use rand_core::OsRng;
use rusqlite::{Connection, OpenFlags, params};
use zcash_client_backend::data_api::chain::{BlockSource, ChainState, scan_cached_blocks};
use zcash_client_backend::data_api::wallet::ConfirmationsPolicy;
use zcash_client_backend::data_api::{
    Account, AccountBirthday, AccountPurpose, WalletRead, WalletWrite,
};
use zcash_client_backend::proto::compact_formats::CompactBlock;
use zcash_client_backend::proto::service::{BlockId, LightdInfo, TreeState};
use zcash_client_sqlite::WalletDb;
use zcash_client_sqlite::error::SqliteClientError;
use zcash_client_sqlite::util::SystemClock;
use zcash_client_sqlite::wallet::init::WalletMigrator;
use zcash_keys::keys::UnifiedFullViewingKey;
use zcash_protocol::consensus::{BlockHeight, BranchId, NetworkUpgrade, Parameters};

use super::{
    AccountId, MAX_COMPACT_BLOCK_BYTES, MAX_LIVE_BATCH_BLOCKS, MAX_LIVE_BATCH_BYTES,
    MAX_LIVE_ENDPOINT_BYTES, Network, ZecError,
};
use crate::vault::{SecretBytes, WipeObserver};

const LIVE_SCHEMA_VERSION: i64 = 1;
const RETAINED_CHECKPOINTS: u32 = 100;
const MAX_TREE_STATE_STRING_BYTES: usize = 4096;
const LIVE_SCHEMA: &str = "
CREATE TABLE ext_bitbook_live_state (
 singleton INTEGER NOT NULL PRIMARY KEY CHECK(singleton = 1),
 schema_version INTEGER NOT NULL, account_id TEXT NOT NULL, network TEXT NOT NULL,
 ufvk TEXT NOT NULL, birthday_height INTEGER NOT NULL, committed_height INTEGER,
 target_height INTEGER, target_hash BLOB, tip_hash BLOB,
 CHECK ((committed_height IS NULL AND target_height IS NULL AND target_hash IS NULL AND tip_hash IS NULL)
 OR (committed_height IS NOT NULL AND target_height IS NOT NULL AND target_hash IS NOT NULL
 AND length(target_hash)=32 AND tip_hash IS NOT NULL AND length(tip_hash)=32
 AND committed_height <= target_height))
);
CREATE TABLE ext_bitbook_live_hashes (
 height INTEGER NOT NULL PRIMARY KEY, hash BLOB NOT NULL CHECK(length(hash)=32)
);";

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct LiveSyncJobId(pub(crate) u64);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LiveSyncPhase {
    Unsynced,
    Syncing,
    Current,
    Stale,
    Cancelled,
    Failed,
    RescanRequired,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiveSyncSnapshot {
    pub account_id: AccountId,
    pub network: Network,
    pub job_id: Option<LiveSyncJobId>,
    pub phase: LiveSyncPhase,
    pub scanned_height: Option<u32>,
    pub target_height: Option<u32>,
    pub confirmed_received_zat: Option<u64>,
    pub pending_received_zat: Option<u64>,
}

#[derive(Clone)]
pub struct LiveCancellation(Arc<AtomicBool>);
impl LiveCancellation {
    pub fn new() -> Self {
        Self(Arc::new(AtomicBool::new(false)))
    }
    pub fn cancel(&self) {
        self.0.store(true, Ordering::Release);
    }
    pub fn is_cancelled(&self) -> bool {
        self.0.load(Ordering::Acquire)
    }
}
impl Default for LiveCancellation {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiveEndpoint {
    raw: String,
    host: String,
    port: u16,
}
impl LiveEndpoint {
    pub fn parse(value: &str) -> Result<Self, ZecError> {
        if value.is_empty()
            || value.len() > MAX_LIVE_ENDPOINT_BYTES
            || !value.is_ascii()
            || value.bytes().any(|byte| byte.is_ascii_control())
        {
            return Err(ZecError::schema());
        }
        let remainder = value
            .strip_prefix("https://")
            .ok_or_else(ZecError::schema)?;
        if remainder.contains(['?', '#', '@']) {
            return Err(ZecError::schema());
        }
        let (authority, path) = remainder
            .split_once('/')
            .map_or((remainder, ""), |(a, p)| (a, p));
        if authority.is_empty() || !path.is_empty() {
            return Err(ZecError::schema());
        }
        let (host, port) = parse_authority(authority)?;
        Ok(Self {
            raw: value.to_owned(),
            host,
            port,
        })
    }
    pub fn as_str(&self) -> &str {
        &self.raw
    }
    pub fn host(&self) -> &str {
        &self.host
    }
    pub fn port(&self) -> u16 {
        self.port
    }
}

fn parse_authority(authority: &str) -> Result<(String, u16), ZecError> {
    if let Some(rest) = authority.strip_prefix('[') {
        let close = rest.find(']').ok_or_else(ZecError::schema)?;
        let host = &rest[..close];
        let suffix = &rest[close + 1..];
        if !matches!(host.parse::<IpAddr>(), Ok(IpAddr::V6(_))) {
            return Err(ZecError::schema());
        }
        let port = if suffix.is_empty() {
            443
        } else {
            parse_port(suffix.strip_prefix(':').ok_or_else(ZecError::schema)?)?
        };
        return Ok((host.to_owned(), port));
    }
    if authority.contains(['[', ']']) || authority.matches(':').count() > 1 {
        return Err(ZecError::schema());
    }
    let (host, port) = match authority.rsplit_once(':') {
        Some((host, port)) => (host, parse_port(port)?),
        None => (authority, 443),
    };
    if !valid_host(host) {
        return Err(ZecError::schema());
    }
    Ok((host.to_owned(), port))
}
fn parse_port(value: &str) -> Result<u16, ZecError> {
    let port = value.parse::<u16>().map_err(|_| ZecError::schema())?;
    if port == 0 {
        Err(ZecError::schema())
    } else {
        Ok(port)
    }
}
fn valid_host(host: &str) -> bool {
    host.parse::<IpAddr>().is_ok() || valid_dns_name(host)
}
fn valid_dns_name(host: &str) -> bool {
    !host.is_empty()
        && host.len() <= 253
        && host.split('.').all(|label| {
            !label.is_empty()
                && label.len() <= 63
                && label
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
                && !label.starts_with('-')
                && !label.ends_with('-')
        })
}

#[derive(Clone, Copy)]
pub struct LiveSyncOptions {
    batch_blocks: usize,
}
impl LiveSyncOptions {
    pub fn for_test(batch_blocks: usize) -> Result<Self, ZecError> {
        if (1..=MAX_LIVE_BATCH_BLOCKS).contains(&batch_blocks) {
            Ok(Self { batch_blocks })
        } else {
            Err(ZecError::limit())
        }
    }
    pub fn batch_blocks(self) -> usize {
        self.batch_blocks
    }
}
impl Default for LiveSyncOptions {
    fn default() -> Self {
        Self {
            batch_blocks: MAX_LIVE_BATCH_BLOCKS,
        }
    }
}

pub(crate) struct SourceMetadata {
    pub info: LightdInfo,
    pub tip: BlockId,
}
pub(crate) trait LiveSource: Send + 'static {
    fn metadata(
        &mut self,
        network: Network,
        cancel: &LiveCancellation,
    ) -> Result<SourceMetadata, ZecError>;
    fn tree_state(&mut self, height: u32, cancel: &LiveCancellation)
    -> Result<TreeState, ZecError>;
    fn blocks(
        &mut self,
        from: u32,
        through: u32,
        cancel: &LiveCancellation,
    ) -> Result<Vec<CompactBlock>, ZecError>;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LiveEngineFault {
    ScanWrite,
    CheckpointWrite,
    Commit,
}
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct LiveEngineObservation {
    pub wallet_db_transactions: usize,
    pub scan_cached_blocks_calls: usize,
    pub decoded_recorded_blocks: usize,
    pub imported_viewing_accounts: usize,
    pub upstream_truncations: usize,
}
#[derive(Default)]
pub(crate) struct LiveEngineHooks {
    pub(crate) observation: LiveEngineObservation,
    pub(crate) fault: Option<LiveEngineFault>,
    pub(crate) on_commit: Option<Box<dyn FnMut(LiveSyncSnapshot) + Send>>,
}
impl LiveEngineHooks {
    fn take_fault(&mut self, expected: LiveEngineFault) -> bool {
        if self.fault == Some(expected) {
            self.fault = None;
            true
        } else {
            false
        }
    }
    fn publish(&mut self, snapshot: LiveSyncSnapshot) {
        if let Some(callback) = self.on_commit.as_mut() {
            callback(snapshot);
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct LiveStoredInspection {
    pub(crate) committed_height: Option<u32>,
    pub(crate) provisional_height: Option<u32>,
    pub(crate) phase: LiveSyncPhase,
    pub(crate) tip_hash: Vec<u8>,
}

#[derive(Clone)]
pub(crate) struct LivePrepared {
    pub account_id: AccountId,
    pub network: Network,
    pub path: PathBuf,
    pub ufvk: String,
    pub(crate) expected_owner: u32,
}

pub(crate) fn prepare_live_account(
    root: &Path,
    account_id: &str,
    seed: &SecretBytes,
    wipes: &mut dyn WipeObserver,
) -> Result<LivePrepared, ZecError> {
    let account_id = AccountId::parse(account_id)?;
    let network = Network::Testnet;
    super::prepare_viewing_account(root, &account_id, network, seed, wipes)?;
    let mut copied = super::copy_session_seed(seed)?;
    let ufvk = super::address::derive_ufvk(network, &mut copied, wipes)?;
    prepare_live_path(root, &account_id, network, &ufvk)
}

pub(crate) fn prepare_live_path(
    root: &Path,
    account_id: &AccountId,
    network: Network,
    ufvk: &str,
) -> Result<LivePrepared, ZecError> {
    let network_directory = root.join(network.as_str());
    let directory = network_directory.join(account_id.as_str());
    let path = directory.join("live.sqlite3");
    let owner = fs::symlink_metadata(root)
        .map_err(|_| ZecError::state_corrupt())?
        .uid();
    validate_dir(root, owner)?;
    validate_dir(&network_directory, owner)?;
    validate_dir(&directory, owner)?;
    match fs::symlink_metadata(&path) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            validate_sidecars(&directory)?;
            OpenOptions::new()
                .read(true)
                .write(true)
                .create_new(true)
                .mode(0o600)
                .open(&path)
                .map_err(|_| ZecError::state_corrupt())?;
            initialize_live(&path, account_id, network, ufvk)?;
        }
        Ok(_) => {}
        Err(_) => return Err(ZecError::state_corrupt()),
    }
    let prepared = LivePrepared {
        account_id: account_id.clone(),
        network,
        path,
        ufvk: ufvk.to_owned(),
        expected_owner: owner,
    };
    validate_prepared(&prepared)?;
    Ok(prepared)
}

fn validate_dir(path: &Path, owner: u32) -> Result<(), ZecError> {
    let metadata = fs::symlink_metadata(path).map_err(|_| ZecError::state_corrupt())?;
    if !metadata.is_dir()
        || metadata.file_type().is_symlink()
        || metadata.permissions().mode() & 0o777 != 0o700
        || metadata.uid() != owner
    {
        Err(ZecError::state_corrupt())
    } else {
        Ok(())
    }
}
fn validate_file(path: &Path, owner: u32) -> Result<(), ZecError> {
    let metadata = fs::symlink_metadata(path).map_err(|_| ZecError::state_corrupt())?;
    if !metadata.is_file()
        || metadata.file_type().is_symlink()
        || metadata.permissions().mode() & 0o777 != 0o600
        || metadata.uid() != owner
        || metadata.nlink() != 1
    {
        Err(ZecError::state_corrupt())
    } else {
        Ok(())
    }
}
fn validate_sidecars(directory: &Path) -> Result<(), ZecError> {
    for suffix in [
        "live.sqlite3-journal",
        "live.sqlite3-wal",
        "live.sqlite3-shm",
    ] {
        match fs::symlink_metadata(directory.join(suffix)) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            _ => return Err(ZecError::state_corrupt()),
        }
    }
    Ok(())
}
fn open_rw(path: &Path) -> Result<Connection, ZecError> {
    Connection::open_with_flags(
        path,
        OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_NO_MUTEX,
    )
    .map_err(|_| ZecError::state_corrupt())
}
fn open_ro(path: &Path) -> Result<Connection, ZecError> {
    Connection::open_with_flags(
        path,
        OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
    )
    .map_err(|_| ZecError::state_corrupt())
}

fn initialize_live(
    path: &Path,
    account_id: &AccountId,
    network: Network,
    ufvk: &str,
) -> Result<(), ZecError> {
    match network {
        Network::Testnet => init_wallet(path, zcash_protocol::consensus::Network::TestNetwork)?,
        Network::Local(local) => init_wallet(path, local.upstream())?,
    }
    let birthday = network_birthday(network)?;
    let connection = open_rw(path)?;
    connection
        .execute_batch("PRAGMA journal_mode=DELETE; PRAGMA synchronous=FULL;")
        .map_err(|_| ZecError::state_corrupt())?;
    connection
        .execute_batch(LIVE_SCHEMA)
        .map_err(|_| ZecError::state_corrupt())?;
    connection.execute(
        "INSERT INTO ext_bitbook_live_state (singleton,schema_version,account_id,network,ufvk,birthday_height) VALUES (1,?1,?2,?3,?4,?5)",
        params![LIVE_SCHEMA_VERSION, account_id.as_str(), network.as_str(), ufvk, birthday],
    ).map_err(|_| ZecError::state_corrupt())?;
    Ok(())
}
fn init_wallet<P: Parameters + 'static>(path: &Path, parameters: P) -> Result<(), ZecError> {
    let mut wallet = WalletDb::for_path(path, parameters, SystemClock, OsRng)
        .map_err(|_| ZecError::state_corrupt())?;
    WalletMigrator::new()
        .init_or_migrate(&mut wallet)
        .map_err(|_| ZecError::state_corrupt())
}
fn network_birthday(network: Network) -> Result<u32, ZecError> {
    match network {
        Network::Testnet => zcash_protocol::consensus::Network::TestNetwork
            .activation_height(NetworkUpgrade::Nu5)
            .map(u32::from)
            .ok_or_else(ZecError::protocol_incompatible),
        Network::Local(local) => Ok(local.birthday_height()),
    }
}

fn validate_prepared(prepared: &LivePrepared) -> Result<StoredState, ZecError> {
    let account_directory = prepared.path.parent().ok_or_else(ZecError::state_corrupt)?;
    let network_directory = account_directory
        .parent()
        .ok_or_else(ZecError::state_corrupt)?;
    let root = network_directory
        .parent()
        .ok_or_else(ZecError::state_corrupt)?;
    validate_dir(root, prepared.expected_owner)?;
    validate_dir(network_directory, prepared.expected_owner)?;
    validate_dir(account_directory, prepared.expected_owner)?;
    validate_file(&prepared.path, prepared.expected_owner)?;
    recover_private_rollback_journal(&prepared.path, account_directory, prepared.expected_owner)?;
    validate_sidecars(account_directory)?;
    let state = validate_extension_state(prepared)?;
    match prepared.network {
        Network::Testnet => validate_official_state(
            prepared,
            &state,
            zcash_protocol::consensus::Network::TestNetwork,
        )?,
        Network::Local(local) => validate_official_state(prepared, &state, local.upstream())?,
    }
    Ok(state)
}

fn recover_private_rollback_journal(
    database: &Path,
    directory: &Path,
    owner: u32,
) -> Result<(), ZecError> {
    for suffix in ["live.sqlite3-wal", "live.sqlite3-shm"] {
        match fs::symlink_metadata(directory.join(suffix)) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            _ => return Err(ZecError::state_corrupt()),
        }
    }
    let journal = directory.join("live.sqlite3-journal");
    match fs::symlink_metadata(&journal) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(_) => return Err(ZecError::state_corrupt()),
        Ok(_) => validate_file(&journal, owner)?,
    }
    let connection = open_rw(database)?;
    let _: i64 = connection
        .query_row("SELECT COUNT(*) FROM sqlite_master", [], |row| row.get(0))
        .map_err(|_| ZecError::state_corrupt())?;
    drop(connection);
    match fs::symlink_metadata(journal) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        _ => Err(ZecError::state_corrupt()),
    }
}

#[derive(Clone)]
struct StoredState {
    birthday_height: u32,
    committed_height: Option<u32>,
    target_height: Option<u32>,
    tip_hash: Option<Vec<u8>>,
    hashes: BTreeMap<u32, Vec<u8>>,
}

fn validate_extension_state(prepared: &LivePrepared) -> Result<StoredState, ZecError> {
    let connection = open_ro(&prepared.path)?;
    super::store::validate_upstream_schema(&connection)?;
    let integrity: String = connection
        .query_row("PRAGMA integrity_check(1)", [], |row| row.get(0))
        .map_err(|_| ZecError::state_corrupt())?;
    let journal_mode: String = connection
        .query_row("PRAGMA journal_mode", [], |row| row.get(0))
        .map_err(|_| ZecError::state_corrupt())?;
    let synchronous: i64 = connection
        .query_row("PRAGMA synchronous", [], |row| row.get(0))
        .map_err(|_| ZecError::state_corrupt())?;
    if integrity != "ok" || journal_mode != "delete" || synchronous != 2 {
        return Err(ZecError::state_corrupt());
    }
    validate_live_schema(&connection)?;

    type StateRow = (
        i64,
        String,
        String,
        String,
        i64,
        Option<i64>,
        Option<i64>,
        Option<Vec<u8>>,
        Option<Vec<u8>>,
    );
    let row: StateRow = connection.query_row(
        "SELECT schema_version,account_id,network,ufvk,birthday_height,committed_height,target_height,target_hash,tip_hash FROM ext_bitbook_live_state WHERE singleton=1",
        [],
        |row| Ok((row.get(0)?,row.get(1)?,row.get(2)?,row.get(3)?,row.get(4)?,row.get(5)?,row.get(6)?,row.get(7)?,row.get(8)?)),
    ).map_err(|_| ZecError::state_corrupt())?;
    let row_count: i64 = connection
        .query_row("SELECT COUNT(*) FROM ext_bitbook_live_state", [], |row| {
            row.get(0)
        })
        .map_err(|_| ZecError::state_corrupt())?;
    let birthday_height = u32::try_from(row.4).map_err(|_| ZecError::state_corrupt())?;
    if row_count != 1
        || row.0 != LIVE_SCHEMA_VERSION
        || row.1 != prepared.account_id.as_str()
        || row.2 != prepared.network.as_str()
        || row.3 != prepared.ufvk
        || birthday_height != network_birthday(prepared.network)?
    {
        return Err(ZecError::state_corrupt());
    }
    let committed_height = optional_height(row.5)?;
    let target_height = optional_height(row.6)?;
    if !optional_hash_valid(&row.7) || !optional_hash_valid(&row.8) {
        return Err(ZecError::state_corrupt());
    }

    let mut statement = connection
        .prepare("SELECT height,hash FROM ext_bitbook_live_hashes ORDER BY height LIMIT 101")
        .map_err(|_| ZecError::state_corrupt())?;
    let rows = statement
        .query_map([], |row| {
            Ok((row.get::<_, i64>(0)?, row.get::<_, Vec<u8>>(1)?))
        })
        .map_err(|_| ZecError::state_corrupt())?;
    let mut hashes = BTreeMap::new();
    for row in rows {
        let (height, hash) = row.map_err(|_| ZecError::state_corrupt())?;
        let height = u32::try_from(height).map_err(|_| ZecError::state_corrupt())?;
        if hash.len() != 32 || hashes.insert(height, hash).is_some() {
            return Err(ZecError::state_corrupt());
        }
    }
    if hashes.len() > RETAINED_CHECKPOINTS as usize {
        return Err(ZecError::state_corrupt());
    }
    match (committed_height, target_height, &row.7, &row.8) {
        (None, None, None, None) if hashes.is_empty() => {}
        (Some(committed), Some(target), Some(target_hash), Some(tip_hash))
            if committed >= birthday_height
                && committed <= target
                && target_hash.len() == 32
                && tip_hash.len() == 32 =>
        {
            if committed == target && target_hash != tip_hash {
                return Err(ZecError::state_corrupt());
            }
            let first = committed
                .saturating_sub(RETAINED_CHECKPOINTS - 1)
                .max(birthday_height);
            let expected_len =
                usize::try_from(committed - first + 1).map_err(|_| ZecError::state_corrupt())?;
            if hashes.len() != expected_len
                || hashes.keys().copied().ne(first..=committed)
                || hashes.get(&committed) != Some(tip_hash)
            {
                return Err(ZecError::state_corrupt());
            }
        }
        _ => return Err(ZecError::state_corrupt()),
    }
    Ok(StoredState {
        birthday_height,
        committed_height,
        target_height,
        tip_hash: row.8,
        hashes,
    })
}

fn validate_live_schema(connection: &Connection) -> Result<(), ZecError> {
    let mut statement = connection.prepare("SELECT type,name FROM sqlite_master WHERE name LIKE 'ext_bitbook_live_%' ORDER BY type,name").map_err(|_| ZecError::state_corrupt())?;
    let objects = statement
        .query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })
        .map_err(|_| ZecError::state_corrupt())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| ZecError::state_corrupt())?;
    if objects
        != [
            ("table".to_owned(), "ext_bitbook_live_hashes".to_owned()),
            ("table".to_owned(), "ext_bitbook_live_state".to_owned()),
        ]
    {
        return Err(ZecError::state_corrupt());
    }
    let attached_triggers: i64 = connection.query_row(
        "SELECT COUNT(*) FROM sqlite_master WHERE type='trigger' AND tbl_name IN ('ext_bitbook_live_state','ext_bitbook_live_hashes')",
        [], |row| row.get(0),
    ).map_err(|_| ZecError::state_corrupt())?;
    if attached_triggers != 0 {
        return Err(ZecError::state_corrupt());
    }
    validate_columns(
        connection,
        "ext_bitbook_live_state",
        &[
            ("singleton", "INTEGER", true, true),
            ("schema_version", "INTEGER", true, false),
            ("account_id", "TEXT", true, false),
            ("network", "TEXT", true, false),
            ("ufvk", "TEXT", true, false),
            ("birthday_height", "INTEGER", true, false),
            ("committed_height", "INTEGER", false, false),
            ("target_height", "INTEGER", false, false),
            ("target_hash", "BLOB", false, false),
            ("tip_hash", "BLOB", false, false),
        ],
    )?;
    validate_columns(
        connection,
        "ext_bitbook_live_hashes",
        &[
            ("height", "INTEGER", true, true),
            ("hash", "BLOB", true, false),
        ],
    )
}

fn validate_columns(
    connection: &Connection,
    table: &str,
    expected: &[(&str, &str, bool, bool)],
) -> Result<(), ZecError> {
    let mut statement = connection
        .prepare(&format!("PRAGMA table_info({table})"))
        .map_err(|_| ZecError::state_corrupt())?;
    let columns = statement
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, i64>(3)? != 0,
                row.get::<_, i64>(5)? != 0,
            ))
        })
        .map_err(|_| ZecError::state_corrupt())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| ZecError::state_corrupt())?;
    if columns
        .iter()
        .map(|(name, ty, required, primary)| (name.as_str(), ty.as_str(), *required, *primary))
        .eq(expected.iter().copied())
    {
        Ok(())
    } else {
        Err(ZecError::state_corrupt())
    }
}
fn optional_height(value: Option<i64>) -> Result<Option<u32>, ZecError> {
    value
        .map(|height| u32::try_from(height).map_err(|_| ZecError::state_corrupt()))
        .transpose()
}
fn optional_hash_valid(value: &Option<Vec<u8>>) -> bool {
    value.as_ref().is_none_or(|hash| hash.len() == 32)
}

fn validate_official_state<P: Parameters + Clone + 'static>(
    prepared: &LivePrepared,
    state: &StoredState,
    parameters: P,
) -> Result<(), ZecError> {
    let wallet = WalletDb::for_path(&prepared.path, parameters.clone(), SystemClock, OsRng)
        .map_err(|_| ZecError::state_corrupt())?;
    let ids = wallet
        .get_account_ids()
        .map_err(|_| ZecError::state_corrupt())?;
    match (ids.as_slice(), state.committed_height) {
        ([], None) => {
            if wallet
                .get_max_height_hash()
                .map_err(|_| ZecError::state_corrupt())?
                .is_some()
            {
                return Err(ZecError::state_corrupt());
            }
        }
        ([id], Some(committed)) => {
            let account = wallet
                .get_account(*id)
                .map_err(|_| ZecError::state_corrupt())?
                .ok_or_else(ZecError::state_corrupt)?;
            if account.purpose() != AccountPurpose::ViewOnly
                || account.birthday_height() != BlockHeight::from_u32(state.birthday_height)
                || account
                    .ufvk()
                    .map(|ufvk| ufvk.encode(&parameters))
                    .as_deref()
                    != Some(prepared.ufvk.as_str())
            {
                return Err(ZecError::state_corrupt());
            }
            let (wallet_height, wallet_hash) = wallet
                .get_max_height_hash()
                .map_err(|_| ZecError::state_corrupt())?
                .ok_or_else(ZecError::state_corrupt)?;
            if u32::from(wallet_height) != committed
                || Some(wallet_hash.0.as_slice()) != state.tip_hash.as_deref()
            {
                return Err(ZecError::state_corrupt());
            }
        }
        _ => return Err(ZecError::state_corrupt()),
    }
    Ok(())
}

pub(crate) fn inspect_live(prepared: &LivePrepared) -> Result<LiveStoredInspection, ZecError> {
    let state = validate_prepared(prepared)?;
    let phase = match (state.committed_height, state.target_height) {
        (None, None) => LiveSyncPhase::Unsynced,
        (Some(committed), Some(target)) if committed == target => LiveSyncPhase::Current,
        (Some(_), Some(_)) => LiveSyncPhase::Stale,
        _ => return Err(ZecError::state_corrupt()),
    };
    Ok(LiveStoredInspection {
        committed_height: state.committed_height,
        provisional_height: None,
        phase,
        tip_hash: state.tip_hash.unwrap_or_default(),
    })
}

pub(crate) fn validate_batch(
    blocks: &[CompactBlock],
    from: u32,
    through: u32,
) -> Result<(), ZecError> {
    validate_batch_with_prior(blocks, from, through, None)
}
fn validate_batch_with_prior(
    blocks: &[CompactBlock],
    from: u32,
    through: u32,
    expected_prior: Option<&[u8]>,
) -> Result<(), ZecError> {
    let expected = through
        .checked_sub(from)
        .and_then(|count| count.checked_add(1))
        .ok_or_else(ZecError::protocol_incompatible)?;
    let expected = usize::try_from(expected).map_err(|_| ZecError::limit())?;
    let sizes = blocks.iter().map(Message::encoded_len).collect::<Vec<_>>();
    validate_batch_shape(&sizes, expected)?;
    let mut prior = expected_prior.map(ToOwned::to_owned);
    for (offset, block) in blocks.iter().enumerate() {
        let height = u32::try_from(block.height).map_err(|_| ZecError::protocol_incompatible())?;
        let expected_height = from
            .checked_add(u32::try_from(offset).map_err(|_| ZecError::limit())?)
            .ok_or_else(ZecError::protocol_incompatible)?;
        if height != expected_height
            || block.hash.len() != 32
            || block.prev_hash.len() != 32
            || prior.as_ref().is_some_and(|hash| hash != &block.prev_hash)
        {
            return Err(ZecError::protocol_incompatible());
        }
        if !block.header.is_empty() {
            let header = block.header().ok_or_else(ZecError::protocol_incompatible)?;
            if header.hash().0.as_slice() != block.hash.as_slice()
                || header.prev_block.0.as_slice() != block.prev_hash.as_slice()
            {
                return Err(ZecError::protocol_incompatible());
            }
        }
        if block
            .vtx
            .iter()
            .any(|transaction| transaction.txid.len() != 32)
        {
            return Err(ZecError::protocol_incompatible());
        }
        prior = Some(block.hash.clone());
    }
    Ok(())
}
pub(crate) fn validate_batch_shape(sizes: &[usize], expected: usize) -> Result<(), ZecError> {
    if sizes.len() != expected {
        return Err(ZecError::protocol_incompatible());
    }
    if sizes.len() > MAX_LIVE_BATCH_BLOCKS
        || sizes.iter().any(|size| *size > MAX_COMPACT_BLOCK_BYTES)
        || sizes
            .iter()
            .try_fold(0usize, |total, size| total.checked_add(*size))
            .ok_or_else(ZecError::limit)?
            > MAX_LIVE_BATCH_BYTES
    {
        return Err(ZecError::limit());
    }
    Ok(())
}

struct MemoryBlocks(Vec<CompactBlock>);
impl BlockSource for MemoryBlocks {
    type Error = std::convert::Infallible;
    fn with_blocks<F, W>(
        &self,
        from: Option<BlockHeight>,
        limit: Option<usize>,
        mut callback: F,
    ) -> Result<(), zcash_client_backend::data_api::chain::error::Error<W, Self::Error>>
    where
        F: FnMut(
            CompactBlock,
        )
            -> Result<(), zcash_client_backend::data_api::chain::error::Error<W, Self::Error>>,
    {
        let from = from.map(u32::from).unwrap_or(0);
        for block in self
            .0
            .iter()
            .filter(|block| block.height >= u64::from(from))
            .take(limit.unwrap_or(usize::MAX))
        {
            callback(block.clone())?;
        }
        Ok(())
    }
}

pub(crate) fn run_live_sync_with_hooks(
    prepared: &LivePrepared,
    source: &mut dyn LiveSource,
    cancel: &LiveCancellation,
    options: LiveSyncOptions,
    job: Option<LiveSyncJobId>,
    hooks: &mut LiveEngineHooks,
) -> Result<LiveSyncSnapshot, ZecError> {
    match prepared.network {
        Network::Testnet => run_with(
            prepared,
            source,
            cancel,
            options,
            job,
            zcash_protocol::consensus::Network::TestNetwork,
            hooks,
        ),
        Network::Local(local) => run_with(
            prepared,
            source,
            cancel,
            options,
            job,
            local.upstream(),
            hooks,
        ),
    }
}

fn run_with<P: Parameters + Clone + Send + 'static>(
    prepared: &LivePrepared,
    source: &mut dyn LiveSource,
    cancel: &LiveCancellation,
    options: LiveSyncOptions,
    job: Option<LiveSyncJobId>,
    parameters: P,
    hooks: &mut LiveEngineHooks,
) -> Result<LiveSyncSnapshot, ZecError> {
    if cancel.is_cancelled() {
        return Err(ZecError::cancelled());
    }
    let stored = validate_prepared(prepared)?;
    let metadata = source.metadata(prepared.network, cancel)?;
    if cancel.is_cancelled() {
        return Err(ZecError::cancelled());
    }
    let target = validate_metadata(
        prepared.network,
        &parameters,
        metadata,
        stored.birthday_height,
    )?;
    if stored
        .committed_height
        .is_some_and(|committed| target.height < committed)
    {
        return Err(ZecError::protocol_incompatible());
    }

    let mut wallet = WalletDb::for_path(&prepared.path, parameters.clone(), SystemClock, OsRng)
        .map_err(|_| ZecError::state_corrupt())?;
    let ufvk = UnifiedFullViewingKey::decode(&parameters, &prepared.ufvk)
        .map_err(|_| ZecError::state_corrupt())?;
    let (mut next, mut prior_state, mut rewind_height) = if let Some(committed) =
        stored.committed_height
    {
        let saved_hash = stored
            .tip_hash
            .as_deref()
            .ok_or_else(ZecError::state_corrupt)?;
        let raw = source.tree_state(committed, cancel)?;
        let current = validate_tree_state(prepared.network, raw, committed, None)?;
        if target.height == committed && current.block_hash().0.as_slice() != target.hash.as_slice()
        {
            return Err(ZecError::protocol_incompatible());
        }
        if current.block_hash().0.as_slice() == saved_hash {
            (
                committed
                    .checked_add(1)
                    .ok_or_else(ZecError::protocol_incompatible)?,
                current,
                None,
            )
        } else {
            let mut common = None;
            for (&height, hash) in stored.hashes.iter().rev().skip(1) {
                if cancel.is_cancelled() {
                    return Err(ZecError::cancelled());
                }
                let raw = source.tree_state(height, cancel)?;
                let candidate = validate_tree_state(prepared.network, raw, height, None)?;
                if candidate.block_hash().0.as_slice() == hash.as_slice() {
                    common = Some((height, candidate));
                    break;
                }
            }
            let (height, state) = common.ok_or_else(ZecError::rescan_required)?;
            (
                height
                    .checked_add(1)
                    .ok_or_else(ZecError::protocol_incompatible)?,
                state,
                Some(height),
            )
        }
    } else {
        let prior_height = stored
            .birthday_height
            .checked_sub(1)
            .ok_or_else(ZecError::protocol_incompatible)?;
        let raw = source.tree_state(prior_height, cancel)?;
        let state = validate_tree_state(prepared.network, raw, prior_height, None)?;
        (stored.birthday_height, state, None)
    };

    while next <= target.height {
        if cancel.is_cancelled() {
            return Err(ZecError::cancelled());
        }
        let batch_span = u32::try_from(options.batch_blocks)
            .map_err(|_| ZecError::limit())?
            .checked_sub(1)
            .ok_or_else(ZecError::limit)?;
        let through = target.height.min(
            next.checked_add(batch_span)
                .ok_or_else(ZecError::protocol_incompatible)?,
        );
        let expected_prior = prior_state.block_hash();
        let blocks = source.blocks(next, through, cancel)?;
        validate_batch_with_prior(&blocks, next, through, Some(expected_prior.0.as_slice()))?;
        let last_hash = blocks
            .last()
            .map(|block| block.hash.clone())
            .ok_or_else(ZecError::protocol_incompatible)?;
        if through == target.height && last_hash != target.hash {
            return Err(ZecError::protocol_incompatible());
        }
        let raw_final = source.tree_state(through, cancel)?;
        let final_state = validate_tree_state(
            prepared.network,
            raw_final,
            through,
            Some(last_hash.as_slice()),
        )?;
        if cancel.is_cancelled() {
            return Err(ZecError::cancelled());
        }

        let source_blocks = MemoryBlocks(blocks.clone());
        hooks.observation.wallet_db_transactions += 1;
        wallet.transactionally_with_extension::<_, _, SqliteClientError>(|wallet, extension| {
            if let Some(height) = rewind_height {
                if u32::from(prior_state.block_height()) != height { return Err(SqliteClientError::CorruptedData("live rewind checkpoint mismatch".to_owned())); }
                wallet.truncate_to_chain_state(prior_state.clone())?;
                hooks.observation.upstream_truncations += 1;
                extension.execute("DELETE FROM ext_bitbook_live_hashes WHERE height > ?1", [height])?;
            }
            let ids = wallet.get_account_ids()?;
            if ids.is_empty() {
                wallet.import_account_ufvk("BitBook viewing account", &ufvk, &AccountBirthday::from_parts(prior_state.clone(), None), AccountPurpose::ViewOnly, None)?;
                hooks.observation.imported_viewing_accounts += 1;
            } else if ids.len() != 1 { return Err(SqliteClientError::CorruptedData("live account count".to_owned())); }
            wallet.update_chain_tip(BlockHeight::from_u32(target.height))?;
            scan_cached_blocks(&parameters, &source_blocks, wallet, BlockHeight::from_u32(next), &prior_state, blocks.len())
                .map_err(|_| SqliteClientError::CorruptedData("live scan".to_owned()))?;
            hooks.observation.scan_cached_blocks_calls += 1;
            hooks.observation.decoded_recorded_blocks += blocks.len();
            if hooks.take_fault(LiveEngineFault::ScanWrite) { return Err(SqliteClientError::CorruptedData("live injected scan failure".to_owned())); }
            extension.execute(
                "UPDATE ext_bitbook_live_state SET committed_height=?1,target_height=?2,target_hash=?3,tip_hash=?4 WHERE singleton=1",
                params![through, target.height, &target.hash, &last_hash],
            )?;
            for block in &blocks {
                extension.execute("INSERT OR REPLACE INTO ext_bitbook_live_hashes(height,hash) VALUES (?1,?2)", params![block.height, &block.hash])?;
            }
            extension.execute("DELETE FROM ext_bitbook_live_hashes WHERE height < ?1", [through.saturating_sub(RETAINED_CHECKPOINTS - 1)])?;
            if hooks.take_fault(LiveEngineFault::CheckpointWrite) { return Err(SqliteClientError::CorruptedData("live injected checkpoint failure".to_owned())); }
            if hooks.take_fault(LiveEngineFault::Commit) { return Err(SqliteClientError::CorruptedData("live injected commit failure".to_owned())); }
            Ok(())
        }).map_err(|_| ZecError::state_corrupt())?;

        rewind_height = None;
        prior_state = final_state;
        let phase = if through == target.height {
            LiveSyncPhase::Current
        } else {
            LiveSyncPhase::Syncing
        };
        let mut snapshot = LiveSyncSnapshot {
            account_id: prepared.account_id.clone(),
            network: prepared.network,
            job_id: job,
            phase,
            scanned_height: Some(through),
            target_height: Some(target.height),
            confirmed_received_zat: None,
            pending_received_zat: None,
        };
        if phase == LiveSyncPhase::Current {
            let (confirmed, pending) =
                shielded_received_balances(&wallet, &prepared.path, through)?;
            snapshot.confirmed_received_zat = Some(confirmed);
            snapshot.pending_received_zat = Some(pending);
        }
        hooks.publish(snapshot);
        next = through
            .checked_add(1)
            .ok_or_else(ZecError::protocol_incompatible)?;
    }

    let (confirmed, pending) = shielded_received_balances(&wallet, &prepared.path, target.height)?;
    let snapshot = LiveSyncSnapshot {
        account_id: prepared.account_id.clone(),
        network: prepared.network,
        job_id: job,
        phase: LiveSyncPhase::Current,
        scanned_height: Some(target.height),
        target_height: Some(target.height),
        confirmed_received_zat: Some(confirmed),
        pending_received_zat: Some(pending),
    };
    if stored.committed_height == Some(target.height)
        && stored.tip_hash.as_deref() == Some(target.hash.as_slice())
    {
        hooks.publish(snapshot.clone());
    }
    Ok(snapshot)
}

struct ValidatedTarget {
    height: u32,
    hash: Vec<u8>,
}
fn validate_metadata<P: Parameters>(
    network: Network,
    parameters: &P,
    metadata: SourceMetadata,
    birthday_height: u32,
) -> Result<ValidatedTarget, ZecError> {
    let tip_height =
        u32::try_from(metadata.tip.height).map_err(|_| ZecError::protocol_incompatible())?;
    let info_height =
        u32::try_from(metadata.info.block_height).map_err(|_| ZecError::protocol_incompatible())?;
    let sapling_height = parameters
        .activation_height(NetworkUpgrade::Sapling)
        .map(u32::from)
        .ok_or_else(ZecError::protocol_incompatible)?;
    let reported_sapling = u32::try_from(metadata.info.sapling_activation_height)
        .map_err(|_| ZecError::protocol_incompatible())?;
    let expected_branch = format!(
        "{:08x}",
        u32::from(BranchId::for_height(
            parameters,
            BlockHeight::from_u32(tip_height)
        ))
    );
    if metadata.info.chain_name != rpc_network(network)
        || metadata.info.chain_name.len() > MAX_TREE_STATE_STRING_BYTES
        || metadata.info.consensus_branch_id != expected_branch
        || metadata.info.consensus_branch_id.len() > MAX_TREE_STATE_STRING_BYTES
        || info_height != tip_height
        || reported_sapling != sapling_height
        || tip_height < birthday_height
        || metadata.tip.hash.len() != 32
    {
        return Err(ZecError::protocol_incompatible());
    }
    Ok(ValidatedTarget {
        height: tip_height,
        hash: metadata.tip.hash,
    })
}

fn validate_tree_state(
    network: Network,
    raw: TreeState,
    requested_height: u32,
    expected_hash: Option<&[u8]>,
) -> Result<ChainState, ZecError> {
    if raw.network != rpc_network(network)
        || raw.network.len() > MAX_TREE_STATE_STRING_BYTES
        || raw.height != u64::from(requested_height)
        || raw.hash.len() > MAX_TREE_STATE_STRING_BYTES
        || raw.sapling_tree.len() > MAX_TREE_STATE_STRING_BYTES
        || raw.orchard_tree.len() > MAX_TREE_STATE_STRING_BYTES
        || raw.ironwood_tree.len() > MAX_TREE_STATE_STRING_BYTES
    {
        return Err(ZecError::protocol_incompatible());
    }
    let state = raw
        .to_chain_state()
        .map_err(|_| ZecError::protocol_incompatible())?;
    if u32::from(state.block_height()) != requested_height
        || expected_hash.is_some_and(|hash| state.block_hash().0.as_slice() != hash)
    {
        return Err(ZecError::protocol_incompatible());
    }
    Ok(state)
}
fn rpc_network(network: Network) -> &'static str {
    match network {
        Network::Testnet => "test",
        Network::Local(_) => "local",
    }
}

fn shielded_received_balances<P: Parameters + Clone + 'static>(
    wallet: &WalletDb<Connection, P, SystemClock, OsRng>,
    path: &Path,
    scanned_tip: u32,
) -> Result<(u64, u64), ZecError> {
    let ids = wallet
        .get_account_ids()
        .map_err(|_| ZecError::state_corrupt())?;
    if ids.len() != 1 {
        return Err(ZecError::state_corrupt());
    }
    let confirmations = ConfirmationsPolicy::new_symmetrical(
        NonZeroU32::new(3).ok_or_else(ZecError::internal)?,
        false,
    );
    let summary = wallet
        .get_wallet_summary(confirmations)
        .map_err(|_| ZecError::state_corrupt())?
        .ok_or_else(ZecError::state_corrupt)?;
    let account = summary
        .account_balances()
        .get(&ids[0])
        .ok_or_else(ZecError::state_corrupt)?;
    let orchard = account.orchard_balance();
    let ironwood = account.ironwood_balance();
    let orchard_pending = orchard
        .change_pending_confirmation()
        .into_u64()
        .checked_add(orchard.value_pending_spendability().into_u64())
        .ok_or_else(ZecError::state_corrupt)?;
    let ironwood_pending = ironwood
        .change_pending_confirmation()
        .into_u64()
        .checked_add(ironwood.value_pending_spendability().into_u64())
        .ok_or_else(ZecError::state_corrupt)?;
    let projection_height = scanned_tip
        .checked_add(1)
        .ok_or_else(ZecError::state_corrupt)?;
    let connection = open_ro(path)?;
    let orphans = super::scan::orphan_projection(&connection, ids[0], projection_height)?;
    if orphans.orchard > orchard_pending || orphans.ironwood > ironwood_pending {
        return Err(ZecError::state_corrupt());
    }
    let total = orchard
        .total()
        .into_u64()
        .checked_sub(orphans.orchard)
        .and_then(|value| {
            ironwood
                .total()
                .into_u64()
                .checked_sub(orphans.ironwood)
                .and_then(|ironwood| value.checked_add(ironwood))
        })
        .ok_or_else(ZecError::state_corrupt)?;
    let pending = orchard_pending
        .checked_sub(orphans.orchard)
        .and_then(|value| {
            ironwood_pending
                .checked_sub(orphans.ironwood)
                .and_then(|ironwood| value.checked_add(ironwood))
        })
        .ok_or_else(ZecError::state_corrupt)?;
    Ok((
        total
            .checked_sub(pending)
            .ok_or_else(ZecError::state_corrupt)?,
        pending,
    ))
}
