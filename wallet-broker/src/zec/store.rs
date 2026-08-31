use std::fs::{self, OpenOptions};
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Duration;

use rand_core::OsRng;
use rusqlite::{Connection, TransactionBehavior, params};
use zcash_client_sqlite::chain::init::init_cache_database;
use zcash_client_sqlite::util::SystemClock;
use zcash_client_sqlite::wallet::init::WalletMigrator;
use zcash_client_sqlite::{BlockDb, WalletDb};
use zcash_protocol::consensus::Parameters;

use crate::vault::{SecretBytes, WipeObserver};

use super::address;
use super::{
    AccountId, FreshReceiverV1, MAX_DIVERSIFIER_INDEX, MAX_ISSUANCE_SEQUENCE, Network, ZecError,
};

const ACCOUNT_TABLE: &str = "ext_bitbook_accounts";
const RECEIVER_TABLE: &str = "ext_bitbook_receiver_state";
const SEQUENCE_TABLE: &str = "ext_bitbook_sequence_state";

const ACCOUNT_SCHEMA: &str = "
CREATE TABLE ext_bitbook_accounts (
    account_id TEXT PRIMARY KEY NOT NULL,
    network TEXT NOT NULL,
    birthday_height INTEGER NOT NULL,
    nu6_3_height INTEGER NOT NULL,
    confirmation_height INTEGER NOT NULL,
    ufvk TEXT NOT NULL
);";
const RECEIVER_SCHEMA: &str = "
CREATE TABLE ext_bitbook_receiver_state (
    account_id TEXT PRIMARY KEY NOT NULL,
    last_diversifier_index INTEGER
);";
const SEQUENCE_SCHEMA: &str = "
CREATE TABLE ext_bitbook_sequence_state (
    account_id TEXT PRIMARY KEY NOT NULL,
    issued_at_sequence INTEGER NOT NULL
);";

#[derive(Clone)]
pub(crate) struct StateRoot {
    path: PathBuf,
    operations: Arc<Mutex<Vec<String>>>,
}

impl StateRoot {
    pub(crate) fn new(path: PathBuf, operations: Arc<Mutex<Vec<String>>>) -> Self {
        Self { path, operations }
    }

    pub(crate) fn path(&self) -> &Path {
        &self.path
    }

    pub(crate) fn operations(&self) -> Vec<String> {
        mutex_lock(&self.operations).clone()
    }

    fn record(&self, operation: &'static str) {
        mutex_lock(&self.operations).push(operation.to_owned());
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum AddressFaultPort {
    ReceiverRowWrite,
    SequenceRowWrite,
    CommitSync,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ReceiverState {
    pub last_diversifier_index: Option<u64>,
    pub issued_at_sequence: u64,
}

pub(crate) struct AddressAccount {
    root: StateRoot,
    account_id: AccountId,
    network: Network,
    paths: AccountPaths,
    gate: Arc<Mutex<()>>,
    fault: Mutex<Option<AddressFaultPort>>,
}

impl AddressAccount {
    pub(crate) fn bootstrap(
        root: StateRoot,
        account_id: AccountId,
        network: Network,
        mut seed: SecretBytes,
        observer: &mut dyn WipeObserver,
    ) -> Result<Self, ZecError> {
        let ufvk = address::derive_ufvk(network, &mut seed, observer)?;
        let paths = account_paths(&root, &account_id, network)?;
        prepare_account_paths(&root, &paths)?;
        initialize_official_wallet(&paths.wallet, network)?;
        initialize_official_cache(&paths.compact)?;
        set_file_mode(&paths.wallet)?;
        set_file_mode(&paths.compact)?;

        initialize_extension(&paths.wallet, &account_id, network, &ufvk)?;
        root.record("zec-address-bootstrap");

        Ok(Self {
            root,
            account_id,
            network,
            paths,
            gate: Arc::new(Mutex::new(())),
            fault: Mutex::new(None),
        })
    }

    pub(crate) fn open_viewing(root: StateRoot, account_id: AccountId) -> Result<Self, ZecError> {
        let network = detect_network(&root, &account_id)?;
        let paths = account_paths(&root, &account_id, network)?;
        validate_account_paths(&paths)?;
        initialize_official_wallet(&paths.wallet, network)?;
        validate_extension_and_binding(&paths.wallet, &account_id, network)?;
        root.record("zec-address-open-viewing");
        Ok(Self {
            root,
            account_id,
            network,
            paths,
            gate: Arc::new(Mutex::new(())),
            fault: Mutex::new(None),
        })
    }

    pub(crate) fn fresh_receiver(&self, _now: u64) -> Result<FreshReceiverV1, ZecError> {
        let _guard = mutex_lock(&self.gate);
        let fault = *mutex_lock(&self.fault);
        issue_receiver(&self.paths, &self.account_id, self.network, fault)
    }

    pub(crate) fn inspect_state(&self) -> Result<ReceiverState, ZecError> {
        let _guard = mutex_lock(&self.gate);
        read_receiver_state(&self.paths, &self.account_id, self.network)
    }

    pub(crate) fn set_state_for_test(&self, index: u64, sequence: u64) -> Result<(), ZecError> {
        if index > MAX_DIVERSIFIER_INDEX || sequence > MAX_ISSUANCE_SEQUENCE {
            return Err(ZecError::limit());
        }
        let _guard = mutex_lock(&self.gate);
        validate_account_paths(&self.paths)?;
        let mut connection = open_connection(&self.paths.wallet)?;
        let transaction = connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| ZecError::state_corrupt())?;
        validate_extension_and_binding_with_connection(
            &transaction,
            &self.account_id,
            self.network,
        )?;
        let receiver_count = transaction
            .execute(
                "UPDATE ext_bitbook_receiver_state
                 SET last_diversifier_index = ?1 WHERE account_id = ?2",
                params![index as i64, self.account_id.as_str()],
            )
            .map_err(|_| ZecError::state_corrupt())?;
        let sequence_count = transaction
            .execute(
                "UPDATE ext_bitbook_sequence_state
                 SET issued_at_sequence = ?1 WHERE account_id = ?2",
                params![sequence as i64, self.account_id.as_str()],
            )
            .map_err(|_| ZecError::state_corrupt())?;
        if receiver_count != 1 || sequence_count != 1 {
            return Err(ZecError::state_corrupt());
        }
        transaction.commit().map_err(|_| ZecError::state_corrupt())
    }

    pub(crate) fn arm_fault(&self, fault: AddressFaultPort) {
        *mutex_lock(&self.fault) = Some(fault);
    }

    pub(crate) fn clear_fault(&self) {
        *mutex_lock(&self.fault) = None;
    }

    pub(crate) fn account_id(&self) -> &AccountId {
        &self.account_id
    }

    pub(crate) fn root(&self) -> StateRoot {
        self.root.clone()
    }
}

struct AccountPaths {
    directory: PathBuf,
    wallet: PathBuf,
    compact: PathBuf,
}

fn account_paths(
    root: &StateRoot,
    account_id: &AccountId,
    network: Network,
) -> Result<AccountPaths, ZecError> {
    if !root.path().is_absolute() {
        return Err(ZecError::state_corrupt());
    }
    let directory = root.path().join(network.as_str()).join(account_id.as_str());
    Ok(AccountPaths {
        wallet: directory.join("wallet.sqlite3"),
        compact: directory.join("compact.sqlite3"),
        directory,
    })
}

fn prepare_account_paths(root: &StateRoot, paths: &AccountPaths) -> Result<(), ZecError> {
    validate_directory(root.path())?;
    let network_directory = paths
        .directory
        .parent()
        .ok_or_else(ZecError::state_corrupt)?;
    create_secure_directory(network_directory)?;
    create_secure_directory(&paths.directory)?;
    create_secure_file(&paths.wallet)?;
    create_secure_file(&paths.compact)?;
    Ok(())
}

fn validate_account_paths(paths: &AccountPaths) -> Result<(), ZecError> {
    let network_directory = paths
        .directory
        .parent()
        .ok_or_else(ZecError::state_corrupt)?;
    let root = network_directory
        .parent()
        .ok_or_else(ZecError::state_corrupt)?;
    validate_directory(root)?;
    validate_directory(network_directory)?;
    validate_directory(&paths.directory)?;
    validate_regular_file(&paths.wallet)?;
    validate_regular_file(&paths.compact)?;
    Ok(())
}

fn create_secure_directory(path: &Path) -> Result<(), ZecError> {
    match fs::symlink_metadata(path) {
        Ok(_) => validate_directory(path),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            fs::create_dir(path).map_err(|_| ZecError::state_corrupt())?;
            fs::set_permissions(path, fs::Permissions::from_mode(0o700))
                .map_err(|_| ZecError::state_corrupt())?;
            validate_directory(path)
        }
        Err(_) => Err(ZecError::state_corrupt()),
    }
}

fn validate_directory(path: &Path) -> Result<(), ZecError> {
    let metadata = fs::symlink_metadata(path).map_err(|_| ZecError::state_corrupt())?;
    if metadata.file_type().is_symlink()
        || !metadata.file_type().is_dir()
        || metadata.permissions().mode() & 0o777 != 0o700
    {
        return Err(ZecError::state_corrupt());
    }
    Ok(())
}

fn create_secure_file(path: &Path) -> Result<(), ZecError> {
    match fs::symlink_metadata(path) {
        Ok(_) => Err(ZecError::state_corrupt()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            OpenOptions::new()
                .write(true)
                .create_new(true)
                .mode(0o600)
                .open(path)
                .map_err(|_| ZecError::state_corrupt())?;
            validate_regular_file(path)
        }
        Err(_) => Err(ZecError::state_corrupt()),
    }
}

fn validate_regular_file(path: &Path) -> Result<(), ZecError> {
    let metadata = fs::symlink_metadata(path).map_err(|_| ZecError::state_corrupt())?;
    if metadata.file_type().is_symlink()
        || !metadata.file_type().is_file()
        || metadata.permissions().mode() & 0o777 != 0o600
    {
        return Err(ZecError::state_corrupt());
    }
    Ok(())
}

fn set_file_mode(path: &Path) -> Result<(), ZecError> {
    validate_regular_file(path)?;
    fs::set_permissions(path, fs::Permissions::from_mode(0o600))
        .map_err(|_| ZecError::state_corrupt())
}

fn initialize_official_wallet(path: &Path, network: Network) -> Result<(), ZecError> {
    validate_regular_file(path)?;
    match network {
        Network::Testnet => {
            initialize_wallet_for(path, zcash_protocol::consensus::Network::TestNetwork)
        }
        Network::Local(local) => initialize_wallet_for(path, local.upstream()),
    }
}

fn initialize_wallet_for<P: Parameters + 'static>(path: &Path, params: P) -> Result<(), ZecError> {
    let mut wallet = WalletDb::for_path(path, params, SystemClock, OsRng)
        .map_err(|_| ZecError::state_corrupt())?;
    WalletMigrator::new()
        .init_or_migrate(&mut wallet)
        .map_err(|_| ZecError::state_corrupt())
}

fn initialize_official_cache(path: &Path) -> Result<(), ZecError> {
    validate_regular_file(path)?;
    let cache = BlockDb::for_path(path).map_err(|_| ZecError::state_corrupt())?;
    init_cache_database(&cache).map_err(|_| ZecError::state_corrupt())
}

fn initialize_extension(
    path: &Path,
    account_id: &AccountId,
    network: Network,
    ufvk: &str,
) -> Result<(), ZecError> {
    let mut connection = open_connection(path)?;
    let transaction = connection
        .transaction_with_behavior(TransactionBehavior::Immediate)
        .map_err(|_| ZecError::state_corrupt())?;
    let existing = extension_objects(&transaction)?;
    if existing.is_empty() {
        transaction
            .execute_batch(ACCOUNT_SCHEMA)
            .map_err(|_| ZecError::state_corrupt())?;
        transaction
            .execute_batch(RECEIVER_SCHEMA)
            .map_err(|_| ZecError::state_corrupt())?;
        transaction
            .execute_batch(SEQUENCE_SCHEMA)
            .map_err(|_| ZecError::state_corrupt())?;
    }
    validate_extension_schema(&transaction)?;

    let (birthday, nu6_3, confirmation) = network_heights(network);
    let inserted_account = transaction
        .execute(
            "INSERT INTO ext_bitbook_accounts
             (account_id, network, birthday_height, nu6_3_height, confirmation_height, ufvk)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                account_id.as_str(),
                network.as_str(),
                i64::from(birthday),
                i64::from(nu6_3),
                i64::from(confirmation),
                ufvk
            ],
        )
        .map_err(|_| ZecError::state_corrupt())?;
    let inserted_receiver = transaction
        .execute(
            "INSERT INTO ext_bitbook_receiver_state (account_id, last_diversifier_index)
             VALUES (?1, NULL)",
            [account_id.as_str()],
        )
        .map_err(|_| ZecError::state_corrupt())?;
    let inserted_sequence = transaction
        .execute(
            "INSERT INTO ext_bitbook_sequence_state (account_id, issued_at_sequence)
             VALUES (?1, 0)",
            [account_id.as_str()],
        )
        .map_err(|_| ZecError::state_corrupt())?;
    if inserted_account != 1 || inserted_receiver != 1 || inserted_sequence != 1 {
        return Err(ZecError::state_corrupt());
    }
    transaction.commit().map_err(|_| ZecError::state_corrupt())
}

fn validate_extension_and_binding(
    path: &Path,
    account_id: &AccountId,
    network: Network,
) -> Result<(), ZecError> {
    let connection = open_connection(path)?;
    validate_extension_and_binding_with_connection(&connection, account_id, network)
}

fn validate_extension_and_binding_with_connection(
    connection: &Connection,
    account_id: &AccountId,
    network: Network,
) -> Result<(), ZecError> {
    validate_extension_schema(connection)?;
    let row_counts = connection
        .query_row(
            "SELECT
                (SELECT COUNT(*) FROM ext_bitbook_accounts),
                (SELECT COUNT(*) FROM ext_bitbook_receiver_state),
                (SELECT COUNT(*) FROM ext_bitbook_sequence_state)",
            [],
            |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, i64>(1)?,
                    row.get::<_, i64>(2)?,
                ))
            },
        )
        .map_err(|_| ZecError::state_corrupt())?;
    if row_counts != (1, 1, 1) {
        return Err(ZecError::state_corrupt());
    }
    let binding = connection
        .query_row(
            "SELECT network, birthday_height, nu6_3_height, confirmation_height, ufvk
             FROM ext_bitbook_accounts WHERE account_id = ?1",
            [account_id.as_str()],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, i64>(1)?,
                    row.get::<_, i64>(2)?,
                    row.get::<_, i64>(3)?,
                    row.get::<_, String>(4)?,
                ))
            },
        )
        .map_err(|_| ZecError::state_corrupt())?;
    let (birthday, nu6_3, confirmation) = network_heights(network);
    let (stored_network, stored_birthday, stored_nu6_3, stored_confirmation, ufvk) = binding;
    if (
        stored_network.as_str(),
        stored_birthday,
        stored_nu6_3,
        stored_confirmation,
    ) != (
        network.as_str(),
        i64::from(birthday),
        i64::from(nu6_3),
        i64::from(confirmation),
    ) || ufvk.is_empty()
    {
        return Err(ZecError::state_corrupt());
    }
    read_receiver_state_with_connection(connection, account_id)?;
    Ok(())
}

#[derive(Debug, Eq, PartialEq)]
struct SchemaObject {
    object_type: String,
    name: String,
    table_name: String,
    sql: Option<String>,
}

fn extension_objects(connection: &Connection) -> Result<Vec<SchemaObject>, ZecError> {
    let mut statement = connection
        .prepare(
            "SELECT type, name, tbl_name, sql FROM sqlite_schema
             WHERE lower(name) GLOB 'ext_bitbook_*'
                OR lower(tbl_name) GLOB 'ext_bitbook_*'
                OR instr(lower(COALESCE(sql, '')), 'ext_bitbook_') > 0
             ORDER BY type, name",
        )
        .map_err(|_| ZecError::state_corrupt())?;
    let rows = statement
        .query_map([], |row| {
            Ok(SchemaObject {
                object_type: row.get(0)?,
                name: row.get(1)?,
                table_name: row.get(2)?,
                sql: row
                    .get::<_, Option<String>>(3)?
                    .map(|sql| normalize_sql(&sql)),
            })
        })
        .map_err(|_| ZecError::state_corrupt())?;
    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|_| ZecError::state_corrupt())
}

fn expected_extension_objects() -> Vec<SchemaObject> {
    vec![
        expected_autoindex(ACCOUNT_TABLE),
        expected_autoindex(RECEIVER_TABLE),
        expected_autoindex(SEQUENCE_TABLE),
        expected_table(ACCOUNT_TABLE, ACCOUNT_SCHEMA),
        expected_table(RECEIVER_TABLE, RECEIVER_SCHEMA),
        expected_table(SEQUENCE_TABLE, SEQUENCE_SCHEMA),
    ]
}

fn expected_autoindex(table: &str) -> SchemaObject {
    SchemaObject {
        object_type: "index".to_owned(),
        name: format!("sqlite_autoindex_{table}_1"),
        table_name: table.to_owned(),
        sql: None,
    }
}

fn expected_table(table: &str, sql: &str) -> SchemaObject {
    SchemaObject {
        object_type: "table".to_owned(),
        name: table.to_owned(),
        table_name: table.to_owned(),
        sql: Some(normalize_sql(sql)),
    }
}

fn normalize_sql(sql: &str) -> String {
    let compact = sql
        .chars()
        .filter(|character| !character.is_ascii_whitespace())
        .collect::<String>();
    compact.strip_suffix(';').unwrap_or(&compact).to_owned()
}

fn validate_extension_schema(connection: &Connection) -> Result<(), ZecError> {
    if extension_objects(connection)? != expected_extension_objects() {
        return Err(ZecError::state_corrupt());
    }
    validate_columns(
        connection,
        ACCOUNT_TABLE,
        &[
            ("account_id", "TEXT", true, true),
            ("network", "TEXT", true, false),
            ("birthday_height", "INTEGER", true, false),
            ("nu6_3_height", "INTEGER", true, false),
            ("confirmation_height", "INTEGER", true, false),
            ("ufvk", "TEXT", true, false),
        ],
    )?;
    validate_columns(
        connection,
        RECEIVER_TABLE,
        &[
            ("account_id", "TEXT", true, true),
            ("last_diversifier_index", "INTEGER", false, false),
        ],
    )?;
    validate_columns(
        connection,
        SEQUENCE_TABLE,
        &[
            ("account_id", "TEXT", true, true),
            ("issued_at_sequence", "INTEGER", true, false),
        ],
    )
}

fn validate_columns(
    connection: &Connection,
    table: &'static str,
    expected: &[(&str, &str, bool, bool)],
) -> Result<(), ZecError> {
    let sql = format!("PRAGMA table_info({table})");
    let mut statement = connection
        .prepare(&sql)
        .map_err(|_| ZecError::state_corrupt())?;
    let rows = statement
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, i64>(3)? != 0,
                row.get::<_, i64>(5)? != 0,
            ))
        })
        .map_err(|_| ZecError::state_corrupt())?;
    let actual = rows
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| ZecError::state_corrupt())?;
    let expected = expected
        .iter()
        .map(|(name, kind, not_null, primary)| {
            ((*name).to_owned(), (*kind).to_owned(), *not_null, *primary)
        })
        .collect::<Vec<_>>();
    if actual == expected {
        Ok(())
    } else {
        Err(ZecError::state_corrupt())
    }
}

fn issue_receiver(
    paths: &AccountPaths,
    account_id: &AccountId,
    network: Network,
    fault: Option<AddressFaultPort>,
) -> Result<FreshReceiverV1, ZecError> {
    validate_account_paths(paths)?;
    let mut connection = open_connection(&paths.wallet)?;
    let transaction = connection
        .transaction_with_behavior(TransactionBehavior::Immediate)
        .map_err(|_| ZecError::state_corrupt())?;
    validate_extension_and_binding_with_connection(&transaction, account_id, network)?;
    let ufvk = transaction
        .query_row(
            "SELECT ufvk FROM ext_bitbook_accounts WHERE account_id = ?1",
            [account_id.as_str()],
            |row| row.get::<_, String>(0),
        )
        .map_err(|_| ZecError::state_corrupt())?;
    let state = read_receiver_state_with_connection(&transaction, account_id)?;
    let next_index = match state.last_diversifier_index {
        None => 0,
        Some(index) => index.checked_add(1).ok_or_else(ZecError::limit)?,
    };
    let next_sequence = state
        .issued_at_sequence
        .checked_add(1)
        .ok_or_else(ZecError::limit)?;
    if next_index > MAX_DIVERSIFIER_INDEX || next_sequence > MAX_ISSUANCE_SEQUENCE {
        return Err(ZecError::limit());
    }
    let receiver = address::derive_orchard_receiver(network, &ufvk, next_index)?;

    let receiver_count = transaction
        .execute(
            "UPDATE ext_bitbook_receiver_state SET last_diversifier_index = ?1
             WHERE account_id = ?2",
            params![next_index as i64, account_id.as_str()],
        )
        .map_err(|_| ZecError::state_corrupt())?;
    if receiver_count != 1 || fault == Some(AddressFaultPort::ReceiverRowWrite) {
        return Err(ZecError::state_corrupt());
    }
    let sequence_count = transaction
        .execute(
            "UPDATE ext_bitbook_sequence_state SET issued_at_sequence = ?1
             WHERE account_id = ?2",
            params![next_sequence as i64, account_id.as_str()],
        )
        .map_err(|_| ZecError::state_corrupt())?;
    if sequence_count != 1 || fault == Some(AddressFaultPort::SequenceRowWrite) {
        return Err(ZecError::state_corrupt());
    }
    if fault == Some(AddressFaultPort::CommitSync) {
        return Err(ZecError::internal());
    }
    transaction
        .commit()
        .map_err(|_| ZecError::state_corrupt())?;
    Ok(FreshReceiverV1 {
        account_id: account_id.clone(),
        network,
        receiver,
        diversifier_index: next_index.to_string(),
        issued_at_sequence: next_sequence.to_string(),
    })
}

fn read_receiver_state(
    paths: &AccountPaths,
    account_id: &AccountId,
    network: Network,
) -> Result<ReceiverState, ZecError> {
    validate_account_paths(paths)?;
    let connection = open_connection(&paths.wallet)?;
    validate_extension_and_binding_with_connection(&connection, account_id, network)?;
    read_receiver_state_with_connection(&connection, account_id)
}

fn read_receiver_state_with_connection(
    connection: &Connection,
    account_id: &AccountId,
) -> Result<ReceiverState, ZecError> {
    let (last_index, sequence) = connection
        .query_row(
            "SELECT r.last_diversifier_index, s.issued_at_sequence
             FROM ext_bitbook_receiver_state r
             JOIN ext_bitbook_sequence_state s ON s.account_id = r.account_id
             WHERE r.account_id = ?1",
            [account_id.as_str()],
            |row| Ok((row.get::<_, Option<i64>>(0)?, row.get::<_, i64>(1)?)),
        )
        .map_err(|_| ZecError::state_corrupt())?;
    if last_index.is_some_and(|value| value < 0) || sequence < 0 {
        return Err(ZecError::state_corrupt());
    }
    Ok(ReceiverState {
        last_diversifier_index: last_index.map(|value| value as u64),
        issued_at_sequence: sequence as u64,
    })
}

fn open_connection(path: &Path) -> Result<Connection, ZecError> {
    validate_regular_file(path)?;
    let connection = Connection::open(path).map_err(|_| ZecError::state_corrupt())?;
    connection
        .busy_timeout(Duration::from_secs(5))
        .map_err(|_| ZecError::state_corrupt())?;
    Ok(connection)
}

fn network_heights(network: Network) -> (u32, u32, u32) {
    match network {
        Network::Testnet => (0, 0, 0),
        Network::Local(local) => (
            local.birthday_height(),
            local.nu6_3_height(),
            local.confirmation_height(),
        ),
    }
}

fn detect_network(root: &StateRoot, account_id: &AccountId) -> Result<Network, ZecError> {
    validate_directory(root.path())?;
    let local_directory = root.path().join("zec-local").join(account_id.as_str());
    let testnet_directory = root.path().join("zec-testnet").join(account_id.as_str());
    let local_exists = entry_exists(&local_directory)?;
    let testnet_exists = entry_exists(&testnet_directory)?;
    match (local_exists, testnet_exists) {
        (true, false) => {
            let paths = AccountPaths {
                wallet: local_directory.join("wallet.sqlite3"),
                compact: local_directory.join("compact.sqlite3"),
                directory: local_directory,
            };
            validate_account_paths(&paths)?;
            let connection = open_connection(&paths.wallet)?;
            validate_extension_schema(&connection)?;
            let heights = connection
                .query_row(
                    "SELECT birthday_height, nu6_3_height, confirmation_height
                     FROM ext_bitbook_accounts WHERE account_id = ?1 AND network = 'zec-local'",
                    [account_id.as_str()],
                    |row| {
                        Ok((
                            row.get::<_, i64>(0)?,
                            row.get::<_, i64>(1)?,
                            row.get::<_, i64>(2)?,
                        ))
                    },
                )
                .map_err(|_| ZecError::state_corrupt())?;
            let birthday = u32::try_from(heights.0).map_err(|_| ZecError::state_corrupt())?;
            let nu6_3 = u32::try_from(heights.1).map_err(|_| ZecError::state_corrupt())?;
            let confirmation = u32::try_from(heights.2).map_err(|_| ZecError::state_corrupt())?;
            Ok(Network::Local(LocalNetwork::new(
                birthday,
                nu6_3,
                confirmation,
            )?))
        }
        (false, true) => {
            let paths = AccountPaths {
                wallet: testnet_directory.join("wallet.sqlite3"),
                compact: testnet_directory.join("compact.sqlite3"),
                directory: testnet_directory,
            };
            validate_account_paths(&paths)?;
            Ok(Network::Testnet)
        }
        _ => Err(ZecError::state_corrupt()),
    }
}

fn entry_exists(path: &Path) -> Result<bool, ZecError> {
    match fs::symlink_metadata(path) {
        Ok(_) => Ok(true),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(_) => Err(ZecError::state_corrupt()),
    }
}

fn mutex_lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

use super::LocalNetwork;
