use std::collections::BTreeSet;
use std::fs::{self, File, OpenOptions};
use std::path::{Path, PathBuf};

#[cfg(target_os = "linux")]
use std::os::unix::fs::{FileTypeExt, MetadataExt, OpenOptionsExt, PermissionsExt};

#[cfg(target_os = "linux")]
use rusqlite::OpenFlags;
use rusqlite::{Connection, OptionalExtension, params};
use zeroize::Zeroizing;

use crate::vault::SecretBytes;
use crate::xmr::model::{XmrError, XmrNetwork};

pub const SCHEMA_VERSION: i64 = 1;
pub const SYNCHRONOUS_FULL: &str = "FULL";
pub const STATE_FILE_MODE: u32 = 0o600;
pub const DIRECTORY_MODE: u32 = 0o700;

const SYNCHRONOUS_FULL_VALUE: i64 = 2;
#[cfg(target_os = "linux")]
const LINUX_O_NOFOLLOW: i32 = 0o400_000;
#[cfg(target_os = "linux")]
const LINUX_O_NONBLOCK: i32 = 0o4_000;
#[cfg(target_os = "linux")]
const LINUX_O_DIRECTORY: i32 = 0o200_000;
const STATE_FILE_NAME: &str = "state.sqlite";
const PRIMARY_ADDRESS_BYTES: usize = 95;
const MONERO_BASE58: &[u8] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

const SCHEMA_SQL: &str = "
CREATE TABLE account_identity (
    slot INTEGER PRIMARY KEY NOT NULL CHECK (slot = 1),
    schema_version INTEGER NOT NULL CHECK (schema_version = 1),
    account_id TEXT NOT NULL CHECK (length(account_id) = 32),
    network TEXT NOT NULL CHECK (network IN ('xmr-stagenet', 'xmr-testnet')),
    kind INTEGER NOT NULL CHECK (kind IN (1, 2)),
    primary_address TEXT NOT NULL CHECK (length(primary_address) = 95),
    restore_height BLOB NOT NULL CHECK (length(restore_height) = 8),
    greatest_issuance_sequence INTEGER NOT NULL CHECK (
        greatest_issuance_sequence >= 0
        AND greatest_issuance_sequence <= 9223372036854775807
    )
);
CREATE TABLE receivers (
    request_id TEXT NOT NULL,
    account_index INTEGER NOT NULL CHECK (account_index = 0),
    subaddress_index INTEGER NOT NULL CHECK (
        subaddress_index > 0
        AND subaddress_index <= 4294967295
    ),
    subaddress TEXT NOT NULL,
    issued_at_sequence INTEGER NOT NULL CHECK (
        issued_at_sequence > 0
        AND issued_at_sequence <= 9223372036854775807
    ),
    PRIMARY KEY (request_id),
    UNIQUE (account_index, subaddress_index),
    UNIQUE (subaddress),
    UNIQUE (issued_at_sequence)
);
";

const IDENTITY_COLUMNS: [ColumnSpec; 8] = [
    ColumnSpec {
        name: "slot",
        type_name: "INTEGER",
        notnull: true,
        pk: true,
    },
    ColumnSpec {
        name: "schema_version",
        type_name: "INTEGER",
        notnull: true,
        pk: false,
    },
    ColumnSpec {
        name: "account_id",
        type_name: "TEXT",
        notnull: true,
        pk: false,
    },
    ColumnSpec {
        name: "network",
        type_name: "TEXT",
        notnull: true,
        pk: false,
    },
    ColumnSpec {
        name: "kind",
        type_name: "INTEGER",
        notnull: true,
        pk: false,
    },
    ColumnSpec {
        name: "primary_address",
        type_name: "TEXT",
        notnull: true,
        pk: false,
    },
    ColumnSpec {
        name: "restore_height",
        type_name: "BLOB",
        notnull: true,
        pk: false,
    },
    ColumnSpec {
        name: "greatest_issuance_sequence",
        type_name: "INTEGER",
        notnull: true,
        pk: false,
    },
];
const RECEIVER_COLUMNS: [ColumnSpec; 5] = [
    ColumnSpec {
        name: "request_id",
        type_name: "TEXT",
        notnull: true,
        pk: true,
    },
    ColumnSpec {
        name: "account_index",
        type_name: "INTEGER",
        notnull: true,
        pk: false,
    },
    ColumnSpec {
        name: "subaddress_index",
        type_name: "INTEGER",
        notnull: true,
        pk: false,
    },
    ColumnSpec {
        name: "subaddress",
        type_name: "TEXT",
        notnull: true,
        pk: false,
    },
    ColumnSpec {
        name: "issued_at_sequence",
        type_name: "INTEGER",
        notnull: true,
        pk: false,
    },
];

#[derive(Clone, Copy)]
struct ColumnSpec {
    name: &'static str,
    type_name: &'static str,
    notnull: bool,
    pk: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ColumnInfo {
    pub name: String,
    pub type_name: String,
    pub notnull: bool,
    pub pk: bool,
}

pub(crate) struct StoredIdentity {
    schema_version: i64,
    account_id: String,
    network: String,
    kind: u8,
    primary_address: SecretBytes,
    restore_height: u64,
    greatest_issuance_sequence: i64,
}

impl StoredIdentity {
    pub fn new(
        account_id: String,
        network: XmrNetwork,
        kind: u8,
        primary_address: &str,
        restore_height: u64,
    ) -> Result<Self, XmrError> {
        let identity = Self {
            schema_version: SCHEMA_VERSION,
            account_id,
            network: network.name().to_owned(),
            kind,
            primary_address: SecretBytes::new(primary_address.as_bytes().to_vec())
                .map_err(|_| XmrError::state_corrupt())?,
            restore_height,
            greatest_issuance_sequence: 0,
        };
        identity.validate()?;
        Ok(identity)
    }

    pub fn validate(&self) -> Result<(), XmrError> {
        if self.schema_version != SCHEMA_VERSION {
            return Err(XmrError::state_corrupt());
        }
        if self.account_id.len() != 32
            || !self
                .account_id
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        {
            return Err(XmrError::state_corrupt());
        }
        if self.network != "xmr-stagenet" && self.network != "xmr-testnet" {
            return Err(XmrError::state_corrupt());
        }
        if self.kind != 1 && self.kind != 2 {
            return Err(XmrError::state_corrupt());
        }
        let primary_ok = self.primary_address.expose(primary_bytes_are_well_formed);
        if !primary_ok {
            return Err(XmrError::state_corrupt());
        }
        if self.greatest_issuance_sequence < 0 {
            return Err(XmrError::state_corrupt());
        }
        Ok(())
    }

    pub fn schema_version(&self) -> i64 {
        self.schema_version
    }

    pub fn account_id(&self) -> &str {
        &self.account_id
    }

    pub fn network(&self) -> &str {
        &self.network
    }

    pub fn kind(&self) -> u8 {
        self.kind
    }

    pub fn primary_address(&self) -> Result<Zeroizing<String>, XmrError> {
        self.primary_address
            .expose(|bytes| utf8(bytes).map(|value| Zeroizing::new(value.to_owned())))
    }

    pub fn restore_height(&self) -> u64 {
        self.restore_height
    }

    pub fn greatest_issuance_sequence(&self) -> i64 {
        self.greatest_issuance_sequence
    }

    pub fn set_greatest_issuance_sequence(&mut self, sequence: i64) -> Result<(), XmrError> {
        if sequence < 0 {
            return Err(XmrError::state_corrupt());
        }
        self.greatest_issuance_sequence = sequence;
        self.validate()
    }

    pub fn matches(&self, other: &Self) -> bool {
        self.schema_version == other.schema_version
            && self.account_id == other.account_id
            && self.network == other.network
            && self.kind == other.kind
            && self.restore_height == other.restore_height
            && self.greatest_issuance_sequence == other.greatest_issuance_sequence
            && self
                .primary_address
                .expose(|left| other.primary_address.expose(|right| left == right))
    }
}

pub(crate) struct StoredReceiver {
    request_id: String,
    account_index: u32,
    subaddress_index: u32,
    subaddress: SecretBytes,
    issued_at_sequence: i64,
}

impl StoredReceiver {
    pub fn new(
        request_id: String,
        account_index: u32,
        subaddress_index: u32,
        subaddress: &str,
        issued_at_sequence: i64,
    ) -> Result<Self, XmrError> {
        let receiver = Self {
            request_id,
            account_index,
            subaddress_index,
            subaddress: SecretBytes::new(subaddress.as_bytes().to_vec())
                .map_err(|_| XmrError::state_corrupt())?,
            issued_at_sequence,
        };
        receiver.validate()?;
        Ok(receiver)
    }

    pub fn validate(&self) -> Result<(), XmrError> {
        if self.request_id.len() != 32
            || !self
                .request_id
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        {
            return Err(XmrError::state_corrupt());
        }
        if self.account_index != 0 || self.subaddress_index == 0 || self.issued_at_sequence <= 0 {
            return Err(XmrError::state_corrupt());
        }
        let address_ok = self.subaddress.expose(primary_bytes_are_well_formed);
        if !address_ok {
            return Err(XmrError::state_corrupt());
        }
        Ok(())
    }

    pub fn request_id(&self) -> &str {
        &self.request_id
    }

    pub fn account_index(&self) -> u32 {
        self.account_index
    }

    pub fn subaddress_index(&self) -> u32 {
        self.subaddress_index
    }

    pub fn issued_at_sequence(&self) -> i64 {
        self.issued_at_sequence
    }

    pub fn subaddress_text(&self) -> Result<Zeroizing<String>, XmrError> {
        self.subaddress
            .expose(|bytes| utf8(bytes).map(|value| Zeroizing::new(value.to_owned())))
    }
}

impl std::fmt::Debug for StoredReceiver {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("StoredReceiver")
            .field("request_id", &"[REDACTED]")
            .field("account_index", &self.account_index)
            .field("subaddress_index", &self.subaddress_index)
            .field("subaddress", &"[REDACTED]")
            .field("issued_at_sequence", &self.issued_at_sequence)
            .finish()
    }
}

impl std::fmt::Debug for StoredIdentity {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("StoredIdentity")
            .field("schema_version", &self.schema_version)
            .field("account_id", &self.account_id)
            .field("network", &self.network)
            .field("kind", &self.kind)
            .field("primary_address", &"[REDACTED]")
            .field("restore_height", &self.restore_height)
            .finish()
    }
}

pub(crate) trait StoreSurface {
    fn execute_batch(&mut self, sql: &str) -> Result<(), XmrError>;
    fn query_i64(&mut self, sql: &str) -> Result<i64, XmrError>;
    fn insert_identity(&mut self, identity: &StoredIdentity) -> Result<(), XmrError>;
    fn load_identity(&mut self) -> Result<StoredIdentity, XmrError>;
    fn table_names(&mut self) -> Result<Vec<String>, XmrError>;
    fn table_sql(&mut self, table: &str) -> Result<String, XmrError>;
    fn column_info(&mut self, table: &str) -> Result<Vec<ColumnInfo>, XmrError>;
    fn schema_objects(&mut self) -> Result<Vec<(String, String, String)>, XmrError>;
    fn unique_column_sets(&mut self, table: &str) -> Result<Vec<Vec<String>>, XmrError>;
    fn begin(&mut self) -> Result<(), XmrError>;
    fn commit(&mut self) -> Result<(), XmrError>;
    fn rollback(&mut self) -> Result<(), XmrError>;
    fn sync_file(&mut self) -> Result<(), XmrError>;
    fn sync_directory(&mut self) -> Result<(), XmrError>;
    fn reopen_existing(&mut self) -> Result<(), XmrError>;
    fn lookup_receiver(&mut self, request_id: &str) -> Result<Option<StoredReceiver>, XmrError>;
    fn insert_receiver(&mut self, receiver: &StoredReceiver) -> Result<(), XmrError>;
    fn update_issuance_sequence(&mut self, sequence: i64) -> Result<(), XmrError>;
    fn list_receivers(&mut self) -> Result<Vec<StoredReceiver>, XmrError>;
    fn max_subaddress_index(&mut self) -> Result<Option<u32>, XmrError>;
    fn delete_receiver(&mut self, request_id: &str) -> Result<(), XmrError>;
}

pub(crate) struct AccountStore<S: StoreSurface> {
    surface: S,
    initialized: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ReceiverPersistenceProof {
    committed: bool,
    file_synced: bool,
    directory_synced: bool,
    reopened: bool,
    binding_proved: bool,
}

impl ReceiverPersistenceProof {
    pub(crate) fn committed(self) -> bool {
        self.committed
    }

    pub(crate) fn durable_and_proved(self) -> bool {
        self.committed
            && self.file_synced
            && self.directory_synced
            && self.reopened
            && self.binding_proved
    }
}

impl<S: StoreSurface> AccountStore<S> {
    pub fn new(surface: S) -> Self {
        Self {
            surface,
            initialized: false,
        }
    }

    pub fn attach_existing(mut surface: S) -> Result<Self, XmrError> {
        require_full_synchronous(&mut surface)?;
        verify_schema(&mut surface)?;
        Ok(Self {
            surface,
            initialized: true,
        })
    }

    pub fn surface_mut(&mut self) -> &mut S {
        &mut self.surface
    }

    pub(crate) fn into_surface(self) -> S {
        self.surface
    }

    pub fn initialize(&mut self) -> Result<(), XmrError> {
        configure_full_synchronous(&mut self.surface)?;
        self.surface.begin()?;
        let result = (|| {
            self.surface.execute_batch(SCHEMA_SQL)?;
            self.surface
                .execute_batch(&format!("PRAGMA user_version = {SCHEMA_VERSION};"))?;
            verify_schema(&mut self.surface)?;
            Ok(())
        })();
        match result {
            Ok(()) => self.surface.commit()?,
            Err(error) => {
                let _ = self.surface.rollback();
                return Err(error);
            }
        }
        self.initialized = true;
        Ok(())
    }

    pub fn persist_identity(&mut self, identity: &StoredIdentity) -> Result<(), XmrError> {
        identity.validate()?;
        if !self.initialized {
            self.initialize()?;
        } else {
            configure_full_synchronous(&mut self.surface)?;
            verify_schema(&mut self.surface)?;
        }
        self.surface.begin()?;
        let result = self.surface.insert_identity(identity);
        match result {
            Ok(()) => self.surface.commit()?,
            Err(error) => {
                let _ = self.surface.rollback();
                return Err(error);
            }
        }
        self.surface.sync_file()?;
        self.surface.sync_directory()?;
        self.surface.reopen_existing()?;
        configure_full_synchronous(&mut self.surface)?;
        verify_schema(&mut self.surface)?;
        let loaded = self.surface.load_identity()?;
        loaded.validate()?;
        if loaded.matches(identity) {
            Ok(())
        } else {
            Err(XmrError::state_corrupt())
        }
    }

    pub fn load_identity(&mut self) -> Result<StoredIdentity, XmrError> {
        require_full_synchronous(&mut self.surface)?;
        verify_schema(&mut self.surface)?;
        let identity = self.surface.load_identity()?;
        identity.validate()?;
        Ok(identity)
    }

    pub fn lookup_receiver(
        &mut self,
        request_id: &str,
    ) -> Result<Option<StoredReceiver>, XmrError> {
        require_full_synchronous(&mut self.surface)?;
        verify_schema(&mut self.surface)?;
        let receiver = self.surface.lookup_receiver(request_id)?;
        if let Some(receiver) = &receiver {
            receiver.validate()?;
        }
        Ok(receiver)
    }

    pub fn list_receivers(&mut self) -> Result<Vec<StoredReceiver>, XmrError> {
        require_full_synchronous(&mut self.surface)?;
        verify_schema(&mut self.surface)?;
        let receivers = self.surface.list_receivers()?;
        for receiver in &receivers {
            receiver.validate()?;
        }
        Ok(receivers)
    }

    pub fn max_subaddress_index(&mut self) -> Result<Option<u32>, XmrError> {
        require_full_synchronous(&mut self.surface)?;
        verify_schema(&mut self.surface)?;
        self.surface.max_subaddress_index()
    }

    pub fn set_issuance_sequence(&mut self, sequence: i64) -> Result<(), XmrError> {
        if sequence < 0 {
            return Err(XmrError::limit());
        }
        require_full_synchronous(&mut self.surface)?;
        verify_schema(&mut self.surface)?;
        self.surface.begin()?;
        let result = self.surface.update_issuance_sequence(sequence);
        match result {
            Ok(()) => self.surface.commit()?,
            Err(error) => {
                let _ = self.surface.rollback();
                return Err(error);
            }
        }
        self.surface.sync_file()?;
        self.surface.sync_directory()?;
        Ok(())
    }

    pub fn persist_receiver(
        &mut self,
        receiver: &StoredReceiver,
    ) -> Result<ReceiverPersistenceProof, XmrError> {
        receiver.validate()?;
        require_full_synchronous(&mut self.surface)?;
        verify_schema(&mut self.surface)?;
        let mut identity = self.surface.load_identity()?;
        identity.validate()?;
        if receiver.issued_at_sequence() <= identity.greatest_issuance_sequence() {
            return Err(XmrError::state_corrupt());
        }
        identity.set_greatest_issuance_sequence(receiver.issued_at_sequence())?;
        self.surface
            .begin()
            .map_err(|_| XmrError::state_corrupt())?;
        let result: Result<(), XmrError> = (|| {
            self.surface.insert_receiver(receiver)?;
            self.surface
                .update_issuance_sequence(receiver.issued_at_sequence())?;
            Ok(())
        })();
        match result {
            Ok(()) => {
                if self.surface.commit().is_err() {
                    let _ = self.surface.rollback();
                    return Err(XmrError::state_corrupt());
                }
            }
            Err(_) => {
                let _ = self.surface.rollback();
                return Err(XmrError::state_corrupt());
            }
        }
        self.surface
            .sync_file()
            .map_err(|_| XmrError::state_corrupt())?;
        self.surface
            .sync_directory()
            .map_err(|_| XmrError::state_corrupt())?;
        self.surface
            .reopen_existing()
            .map_err(|_| XmrError::state_corrupt())?;
        require_full_synchronous(&mut self.surface).map_err(|_| XmrError::state_corrupt())?;
        verify_schema(&mut self.surface).map_err(|_| XmrError::state_corrupt())?;
        let loaded = self
            .surface
            .lookup_receiver(receiver.request_id())?
            .ok_or_else(XmrError::state_corrupt)?;
        loaded.validate()?;
        let loaded_address = loaded.subaddress_text()?;
        let candidate_address = receiver.subaddress_text()?;
        let proved_identity = self.surface.load_identity()?;
        proved_identity.validate()?;
        if loaded.request_id() != receiver.request_id()
            || loaded.subaddress_index() != receiver.subaddress_index()
            || loaded.issued_at_sequence() != receiver.issued_at_sequence()
            || loaded.account_index() != receiver.account_index()
            || loaded_address.as_str() != candidate_address.as_str()
            || !proved_identity.matches(&identity)
        {
            return Err(XmrError::state_corrupt());
        }
        Ok(ReceiverPersistenceProof {
            committed: true,
            file_synced: true,
            directory_synced: true,
            reopened: true,
            binding_proved: true,
        })
    }

    pub fn delete_uncommitted_receiver(&mut self, request_id: &str) -> Result<(), XmrError> {
        self.surface.begin()?;
        let result = self.surface.delete_receiver(request_id);
        match result {
            Ok(()) => self.surface.commit()?,
            Err(error) => {
                let _ = self.surface.rollback();
                return Err(error);
            }
        }
        self.surface.sync_file()?;
        self.surface.sync_directory()?;
        Ok(())
    }

    pub fn prove_durability(&mut self) -> Result<(), XmrError> {
        require_full_synchronous(&mut self.surface)
    }

    pub fn reopen(&mut self) -> Result<StoredIdentity, XmrError> {
        self.surface.reopen_existing()?;
        require_full_synchronous(&mut self.surface)?;
        verify_schema(&mut self.surface)?;
        let identity = self.surface.load_identity()?;
        identity.validate()?;
        Ok(identity)
    }

    pub fn inspect_receiver_schema(
        &mut self,
        file_mode: u32,
    ) -> Result<ReceiverSchemaView, XmrError> {
        require_full_synchronous(&mut self.surface)?;
        verify_schema(&mut self.surface)?;
        let identity = self.surface.load_identity()?;
        identity.validate()?;
        let receiver_sql = self.surface.table_sql("receivers")?;
        if !receiver_sql.contains("CHECK (account_index = 0)")
            || !receiver_sql.contains("subaddress_index > 0")
            || !receiver_sql.contains("issued_at_sequence > 0")
            || !receiver_sql.contains("issued_at_sequence <= 9223372036854775807")
        {
            return Err(XmrError::state_corrupt());
        }
        Ok(ReceiverSchemaView {
            schema_version: identity.schema_version(),
            synchronous: SYNCHRONOUS_FULL,
            file_mode,
            account_columns: [
                "schema_version",
                "account_id",
                "network",
                "primary_address",
                "greatest_issuance_sequence",
            ],
            receiver_columns: [
                "request_id",
                "account_index",
                "subaddress_index",
                "subaddress",
                "issued_at_sequence",
            ],
            account_id: identity.account_id().to_owned(),
            network: identity.network().to_owned(),
            primary_address: identity.primary_address()?.as_str().to_owned(),
            independent_unique_constraints: vec![
                vec!["request_id"],
                vec!["account_index", "subaddress_index"],
                vec!["subaddress"],
                vec!["issued_at_sequence"],
            ],
            account_index_check: "account_index = 0",
            subaddress_index_check: "subaddress_index > 0",
            issuance_sequence_check: "issued_at_sequence > 0 AND issued_at_sequence <= 9223372036854775807",
        })
    }
}

pub struct ReceiverSchemaView {
    pub schema_version: i64,
    pub synchronous: &'static str,
    pub file_mode: u32,
    pub account_columns: [&'static str; 5],
    pub receiver_columns: [&'static str; 5],
    pub account_id: String,
    pub network: String,
    pub primary_address: String,
    pub independent_unique_constraints: Vec<Vec<&'static str>>,
    pub account_index_check: &'static str,
    pub subaddress_index_check: &'static str,
    pub issuance_sequence_check: &'static str,
}

pub(crate) struct SqliteSurface {
    connection: Connection,
}

impl SqliteSurface {
    pub fn memory() -> Result<Self, XmrError> {
        Connection::open_in_memory()
            .map_err(|_| XmrError::state_corrupt())
            .map(|connection| Self { connection })
    }
}

impl StoreSurface for SqliteSurface {
    fn execute_batch(&mut self, sql: &str) -> Result<(), XmrError> {
        self.connection
            .execute_batch(sql)
            .map_err(|_| XmrError::state_corrupt())
    }

    fn query_i64(&mut self, sql: &str) -> Result<i64, XmrError> {
        self.connection
            .query_row(sql, [], |row| row.get(0))
            .map_err(|_| XmrError::state_corrupt())
    }

    fn insert_identity(&mut self, identity: &StoredIdentity) -> Result<(), XmrError> {
        insert_identity(&self.connection, identity)
    }

    fn load_identity(&mut self) -> Result<StoredIdentity, XmrError> {
        load_identity(&self.connection)
    }

    fn table_names(&mut self) -> Result<Vec<String>, XmrError> {
        table_names(&self.connection)
    }

    fn table_sql(&mut self, table: &str) -> Result<String, XmrError> {
        table_sql(&self.connection, table)
    }

    fn column_info(&mut self, table: &str) -> Result<Vec<ColumnInfo>, XmrError> {
        column_info(&self.connection, table)
    }

    fn schema_objects(&mut self) -> Result<Vec<(String, String, String)>, XmrError> {
        schema_objects(&self.connection)
    }

    fn unique_column_sets(&mut self, table: &str) -> Result<Vec<Vec<String>>, XmrError> {
        unique_column_sets(&self.connection, table)
    }

    fn begin(&mut self) -> Result<(), XmrError> {
        self.connection
            .execute_batch("BEGIN IMMEDIATE;")
            .map_err(|_| XmrError::state_corrupt())
    }

    fn commit(&mut self) -> Result<(), XmrError> {
        self.connection
            .execute_batch("COMMIT;")
            .map_err(|_| XmrError::state_corrupt())
    }

    fn rollback(&mut self) -> Result<(), XmrError> {
        self.connection
            .execute_batch("ROLLBACK;")
            .map_err(|_| XmrError::state_corrupt())
    }

    fn sync_file(&mut self) -> Result<(), XmrError> {
        Ok(())
    }

    fn sync_directory(&mut self) -> Result<(), XmrError> {
        Ok(())
    }

    fn reopen_existing(&mut self) -> Result<(), XmrError> {
        Ok(())
    }

    fn lookup_receiver(&mut self, request_id: &str) -> Result<Option<StoredReceiver>, XmrError> {
        lookup_receiver(&self.connection, request_id)
    }

    fn insert_receiver(&mut self, receiver: &StoredReceiver) -> Result<(), XmrError> {
        insert_receiver(&self.connection, receiver)
    }

    fn update_issuance_sequence(&mut self, sequence: i64) -> Result<(), XmrError> {
        update_issuance_sequence(&self.connection, sequence)
    }

    fn list_receivers(&mut self) -> Result<Vec<StoredReceiver>, XmrError> {
        list_receivers(&self.connection)
    }

    fn max_subaddress_index(&mut self) -> Result<Option<u32>, XmrError> {
        max_subaddress_index(&self.connection)
    }

    fn delete_receiver(&mut self, request_id: &str) -> Result<(), XmrError> {
        delete_receiver(&self.connection, request_id)
    }
}

pub(crate) struct PathSqliteSurface {
    #[cfg(target_os = "linux")]
    connection: Connection,
    file: PathBuf,
    directory: PathBuf,
    expected_owner: u32,
    #[cfg(target_os = "linux")]
    state_handle: File,
    #[cfg(target_os = "linux")]
    state_identity: (u64, u64),
    #[cfg(target_os = "linux")]
    directory_handle: File,
    #[cfg(target_os = "linux")]
    directory_identity: (u64, u64),
}

impl PathSqliteSurface {
    pub fn exclusive_create_file(path: &Path, expected_owner: u32) -> Result<File, XmrError> {
        #[cfg(not(target_os = "linux"))]
        {
            let _ = (path, expected_owner);
            Err(XmrError::unavailable())
        }
        #[cfg(target_os = "linux")]
        exclusive_create_state_file(path, expected_owner)
    }

    pub fn created_file_identity(file: &File) -> Result<(u64, u64), XmrError> {
        #[cfg(not(target_os = "linux"))]
        {
            let _ = file;
            Err(XmrError::unavailable())
        }
        #[cfg(target_os = "linux")]
        created_state_file_identity(file)
    }

    pub fn bind_created(
        account_directory: &Path,
        expected_owner: u32,
        state_handle: File,
        state_identity: (u64, u64),
    ) -> Result<Self, XmrError> {
        #[cfg(not(target_os = "linux"))]
        {
            let _ = (
                account_directory,
                expected_owner,
                state_handle,
                state_identity,
            );
            Err(XmrError::unavailable())
        }
        #[cfg(target_os = "linux")]
        {
            validate_account_directory(account_directory, expected_owner)?;
            let file = account_directory.join(STATE_FILE_NAME);
            let (directory_handle, directory_identity) =
                open_account_directory(account_directory, expected_owner)?;
            let opened = revalidate_opened_file(&state_handle, expected_owner, STATE_FILE_MODE)?;
            if opened != state_identity {
                return Err(XmrError::state_corrupt());
            }
            let (connection, state_handle, bound_identity) =
                bind_sqlite(state_handle, &file, expected_owner, true)?;
            if bound_identity != state_identity {
                return Err(XmrError::state_corrupt());
            }
            let mut surface = Self {
                connection,
                file,
                directory: account_directory.to_path_buf(),
                expected_owner,
                state_handle,
                state_identity: bound_identity,
                directory_handle,
                directory_identity,
            };
            configure_full_synchronous(&mut surface)?;
            Ok(surface)
        }
    }

    pub fn state_file_mode(&self) -> Result<u32, XmrError> {
        #[cfg(not(target_os = "linux"))]
        {
            let _ = self;
            Err(XmrError::unavailable())
        }
        #[cfg(target_os = "linux")]
        {
            let metadata = self
                .state_handle
                .metadata()
                .map_err(|_| XmrError::state_corrupt())?;
            Ok(metadata.permissions().mode() & 0o777)
        }
    }

    pub fn open_existing(account_directory: &Path, expected_owner: u32) -> Result<Self, XmrError> {
        #[cfg(not(target_os = "linux"))]
        {
            let _ = (account_directory, expected_owner);
            return Err(XmrError::unavailable());
        }
        #[cfg(target_os = "linux")]
        {
            validate_account_directory(account_directory, expected_owner)?;
            let file = account_directory.join(STATE_FILE_NAME);
            let (directory_handle, directory_identity) =
                open_account_directory(account_directory, expected_owner)?;
            let state_handle = OpenOptions::new()
                .read(true)
                .write(true)
                .custom_flags(LINUX_O_NOFOLLOW | LINUX_O_NONBLOCK)
                .open(&file)
                .map_err(|_| XmrError::state_corrupt())?;
            let (connection, state_handle, state_identity) =
                bind_sqlite(state_handle, &file, expected_owner, false)?;
            let mut surface = Self {
                connection,
                file,
                directory: account_directory.to_path_buf(),
                expected_owner,
                state_handle,
                state_identity,
                directory_handle,
                directory_identity,
            };
            let version = surface.query_i64("PRAGMA user_version")?;
            if version != SCHEMA_VERSION {
                return Err(XmrError::state_corrupt());
            }
            verify_schema(&mut surface)?;
            Ok(surface)
        }
    }

    #[cfg(target_os = "linux")]
    fn reopen_linux(&mut self) -> Result<(), XmrError> {
        validate_account_directory(&self.directory, self.expected_owner)?;
        let directory_identity =
            revalidate_opened_directory(&self.directory_handle, self.expected_owner)?;
        if directory_identity != self.directory_identity {
            return Err(XmrError::state_corrupt());
        }
        let identity =
            revalidate_opened_file(&self.state_handle, self.expected_owner, STATE_FILE_MODE)?;
        if identity != self.state_identity {
            return Err(XmrError::state_corrupt());
        }
        let (connection, state_handle, state_identity) = bind_sqlite(
            self.state_handle
                .try_clone()
                .map_err(|_| XmrError::internal())?,
            &self.file,
            self.expected_owner,
            false,
        )?;
        if state_identity != self.state_identity {
            return Err(XmrError::state_corrupt());
        }
        self.connection = connection;
        self.state_handle = state_handle;
        self.state_identity = state_identity;
        require_full_synchronous(self)?;
        verify_schema(self)
    }
}

impl StoreSurface for PathSqliteSurface {
    fn execute_batch(&mut self, sql: &str) -> Result<(), XmrError> {
        #[cfg(not(target_os = "linux"))]
        {
            let _ = sql;
            return Err(XmrError::unavailable());
        }
        #[cfg(target_os = "linux")]
        {
            self.connection
                .execute_batch(sql)
                .map_err(|_| XmrError::state_corrupt())
        }
    }

    fn query_i64(&mut self, sql: &str) -> Result<i64, XmrError> {
        #[cfg(not(target_os = "linux"))]
        {
            let _ = sql;
            return Err(XmrError::unavailable());
        }
        #[cfg(target_os = "linux")]
        {
            self.connection
                .query_row(sql, [], |row| row.get(0))
                .map_err(|_| XmrError::state_corrupt())
        }
    }

    fn insert_identity(&mut self, identity: &StoredIdentity) -> Result<(), XmrError> {
        #[cfg(not(target_os = "linux"))]
        {
            let _ = identity;
            return Err(XmrError::unavailable());
        }
        #[cfg(target_os = "linux")]
        insert_identity(&self.connection, identity)
    }

    fn load_identity(&mut self) -> Result<StoredIdentity, XmrError> {
        #[cfg(not(target_os = "linux"))]
        {
            return Err(XmrError::unavailable());
        }
        #[cfg(target_os = "linux")]
        load_identity(&self.connection)
    }

    fn table_names(&mut self) -> Result<Vec<String>, XmrError> {
        #[cfg(not(target_os = "linux"))]
        {
            return Err(XmrError::unavailable());
        }
        #[cfg(target_os = "linux")]
        table_names(&self.connection)
    }

    fn table_sql(&mut self, table: &str) -> Result<String, XmrError> {
        #[cfg(not(target_os = "linux"))]
        {
            let _ = table;
            return Err(XmrError::unavailable());
        }
        #[cfg(target_os = "linux")]
        table_sql(&self.connection, table)
    }

    fn column_info(&mut self, table: &str) -> Result<Vec<ColumnInfo>, XmrError> {
        #[cfg(not(target_os = "linux"))]
        {
            let _ = table;
            return Err(XmrError::unavailable());
        }
        #[cfg(target_os = "linux")]
        column_info(&self.connection, table)
    }

    fn schema_objects(&mut self) -> Result<Vec<(String, String, String)>, XmrError> {
        #[cfg(not(target_os = "linux"))]
        {
            return Err(XmrError::unavailable());
        }
        #[cfg(target_os = "linux")]
        schema_objects(&self.connection)
    }

    fn unique_column_sets(&mut self, table: &str) -> Result<Vec<Vec<String>>, XmrError> {
        #[cfg(not(target_os = "linux"))]
        {
            let _ = table;
            return Err(XmrError::unavailable());
        }
        #[cfg(target_os = "linux")]
        unique_column_sets(&self.connection, table)
    }

    fn begin(&mut self) -> Result<(), XmrError> {
        self.execute_batch("BEGIN IMMEDIATE;")
    }

    fn commit(&mut self) -> Result<(), XmrError> {
        self.execute_batch("COMMIT;")
    }

    fn rollback(&mut self) -> Result<(), XmrError> {
        self.execute_batch("ROLLBACK;")
    }

    fn sync_file(&mut self) -> Result<(), XmrError> {
        #[cfg(not(target_os = "linux"))]
        {
            return Err(XmrError::unavailable());
        }
        #[cfg(target_os = "linux")]
        {
            let identity =
                revalidate_opened_file(&self.state_handle, self.expected_owner, STATE_FILE_MODE)?;
            if identity != self.state_identity {
                return Err(XmrError::state_corrupt());
            }
            self.state_handle
                .sync_all()
                .map_err(|_| XmrError::state_corrupt())
        }
    }

    fn sync_directory(&mut self) -> Result<(), XmrError> {
        #[cfg(not(target_os = "linux"))]
        {
            return Err(XmrError::unavailable());
        }
        #[cfg(target_os = "linux")]
        {
            let retained =
                revalidate_opened_directory(&self.directory_handle, self.expected_owner)?;
            if retained != self.directory_identity {
                return Err(XmrError::state_corrupt());
            }
            let listed =
                fs::symlink_metadata(&self.directory).map_err(|_| XmrError::state_corrupt())?;
            if listed.file_type().is_symlink()
                || !listed.file_type().is_dir()
                || listed.uid() != self.expected_owner
                || listed.permissions().mode() & 0o777 != DIRECTORY_MODE
                || listed.dev() != self.directory_identity.0
                || listed.ino() != self.directory_identity.1
            {
                return Err(XmrError::state_corrupt());
            }
            self.directory_handle
                .sync_all()
                .map_err(|_| XmrError::state_corrupt())
        }
    }

    fn reopen_existing(&mut self) -> Result<(), XmrError> {
        #[cfg(not(target_os = "linux"))]
        {
            return Err(XmrError::unavailable());
        }
        #[cfg(target_os = "linux")]
        self.reopen_linux()
    }

    fn lookup_receiver(&mut self, request_id: &str) -> Result<Option<StoredReceiver>, XmrError> {
        #[cfg(not(target_os = "linux"))]
        {
            let _ = request_id;
            return Err(XmrError::unavailable());
        }
        #[cfg(target_os = "linux")]
        lookup_receiver(&self.connection, request_id)
    }

    fn insert_receiver(&mut self, receiver: &StoredReceiver) -> Result<(), XmrError> {
        #[cfg(not(target_os = "linux"))]
        {
            let _ = receiver;
            return Err(XmrError::unavailable());
        }
        #[cfg(target_os = "linux")]
        insert_receiver(&self.connection, receiver)
    }

    fn update_issuance_sequence(&mut self, sequence: i64) -> Result<(), XmrError> {
        #[cfg(not(target_os = "linux"))]
        {
            let _ = sequence;
            return Err(XmrError::unavailable());
        }
        #[cfg(target_os = "linux")]
        update_issuance_sequence(&self.connection, sequence)
    }

    fn list_receivers(&mut self) -> Result<Vec<StoredReceiver>, XmrError> {
        #[cfg(not(target_os = "linux"))]
        {
            return Err(XmrError::unavailable());
        }
        #[cfg(target_os = "linux")]
        list_receivers(&self.connection)
    }

    fn max_subaddress_index(&mut self) -> Result<Option<u32>, XmrError> {
        #[cfg(not(target_os = "linux"))]
        {
            return Err(XmrError::unavailable());
        }
        #[cfg(target_os = "linux")]
        max_subaddress_index(&self.connection)
    }

    fn delete_receiver(&mut self, request_id: &str) -> Result<(), XmrError> {
        #[cfg(not(target_os = "linux"))]
        {
            let _ = request_id;
            return Err(XmrError::unavailable());
        }
        #[cfg(target_os = "linux")]
        delete_receiver(&self.connection, request_id)
    }
}

pub(crate) fn state_file_name() -> &'static str {
    STATE_FILE_NAME
}

fn insert_identity(connection: &Connection, identity: &StoredIdentity) -> Result<(), XmrError> {
    identity.validate()?;
    let restore_height = identity.restore_height().to_be_bytes();
    let primary = identity.primary_address()?;
    connection
        .execute(
            "INSERT INTO account_identity (
                    slot,
                    schema_version,
                    account_id,
                    network,
                    kind,
                    primary_address,
                    restore_height,
                    greatest_issuance_sequence
                ) VALUES (1, ?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                identity.schema_version(),
                identity.account_id(),
                identity.network(),
                i64::from(identity.kind()),
                primary.as_str(),
                restore_height.as_slice(),
                identity.greatest_issuance_sequence(),
            ],
        )
        .map_err(|_| XmrError::state_corrupt())?;
    Ok(())
}

fn insert_receiver(connection: &Connection, receiver: &StoredReceiver) -> Result<(), XmrError> {
    receiver.validate()?;
    let subaddress = receiver.subaddress_text()?;
    connection
        .execute(
            "INSERT INTO receivers (
                    request_id,
                    account_index,
                    subaddress_index,
                    subaddress,
                    issued_at_sequence
                ) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                receiver.request_id(),
                i64::from(receiver.account_index()),
                i64::from(receiver.subaddress_index()),
                subaddress.as_str(),
                receiver.issued_at_sequence(),
            ],
        )
        .map_err(|_| XmrError::state_corrupt())?;
    Ok(())
}

fn lookup_receiver(
    connection: &Connection,
    request_id: &str,
) -> Result<Option<StoredReceiver>, XmrError> {
    let row = connection
        .query_row(
            "SELECT request_id, account_index, subaddress_index, subaddress, issued_at_sequence
             FROM receivers WHERE request_id = ?1",
            [request_id],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, i64>(1)?,
                    row.get::<_, i64>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, i64>(4)?,
                ))
            },
        )
        .optional()
        .map_err(|_| XmrError::state_corrupt())?;
    row.map(
        |(request_id, account_index, subaddress_index, subaddress, sequence)| {
            let account_index =
                u32::try_from(account_index).map_err(|_| XmrError::state_corrupt())?;
            let subaddress_index =
                u32::try_from(subaddress_index).map_err(|_| XmrError::state_corrupt())?;
            StoredReceiver::new(
                request_id,
                account_index,
                subaddress_index,
                &subaddress,
                sequence,
            )
        },
    )
    .transpose()
}

fn list_receivers(connection: &Connection) -> Result<Vec<StoredReceiver>, XmrError> {
    let mut statement = connection
        .prepare(
            "SELECT request_id, account_index, subaddress_index, subaddress, issued_at_sequence
             FROM receivers ORDER BY issued_at_sequence",
        )
        .map_err(|_| XmrError::state_corrupt())?;
    let rows = statement
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, i64>(1)?,
                row.get::<_, i64>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, i64>(4)?,
            ))
        })
        .map_err(|_| XmrError::state_corrupt())?;
    let mut receivers = Vec::new();
    for row in rows {
        let (request_id, account_index, subaddress_index, subaddress, sequence) =
            row.map_err(|_| XmrError::state_corrupt())?;
        let account_index = u32::try_from(account_index).map_err(|_| XmrError::state_corrupt())?;
        let subaddress_index =
            u32::try_from(subaddress_index).map_err(|_| XmrError::state_corrupt())?;
        receivers.push(StoredReceiver::new(
            request_id,
            account_index,
            subaddress_index,
            &subaddress,
            sequence,
        )?);
    }
    Ok(receivers)
}

fn max_subaddress_index(connection: &Connection) -> Result<Option<u32>, XmrError> {
    let value = connection
        .query_row("SELECT MAX(subaddress_index) FROM receivers", [], |row| {
            row.get::<_, Option<i64>>(0)
        })
        .map_err(|_| XmrError::state_corrupt())?;
    match value {
        Some(index) => u32::try_from(index)
            .map(Some)
            .map_err(|_| XmrError::state_corrupt()),
        None => Ok(None),
    }
}

fn update_issuance_sequence(connection: &Connection, sequence: i64) -> Result<(), XmrError> {
    if sequence < 0 {
        return Err(XmrError::state_corrupt());
    }
    let updated = connection
        .execute(
            "UPDATE account_identity SET greatest_issuance_sequence = ?1 WHERE slot = 1",
            params![sequence],
        )
        .map_err(|_| XmrError::state_corrupt())?;
    if updated == 1 {
        Ok(())
    } else {
        Err(XmrError::state_corrupt())
    }
}

fn delete_receiver(connection: &Connection, request_id: &str) -> Result<(), XmrError> {
    connection
        .execute(
            "DELETE FROM receivers WHERE request_id = ?1",
            params![request_id],
        )
        .map_err(|_| XmrError::state_corrupt())?;
    Ok(())
}

fn load_identity(connection: &Connection) -> Result<StoredIdentity, XmrError> {
    let row = connection
        .query_row(
            "SELECT schema_version, account_id, network, kind, primary_address, restore_height, greatest_issuance_sequence
             FROM account_identity WHERE slot = 1",
            [],
            |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, i64>(3)?,
                    row.get::<_, String>(4)?,
                    row.get::<_, rusqlite::types::Value>(5)?,
                    row.get::<_, i64>(6)?,
                ))
            },
        )
        .optional()
        .map_err(|_| XmrError::state_corrupt())?
        .ok_or_else(XmrError::state_corrupt)?;
    let kind = u8::try_from(row.3).map_err(|_| XmrError::state_corrupt())?;
    let restore_height = match row.5 {
        rusqlite::types::Value::Blob(bytes) => {
            let exact: [u8; 8] = bytes.try_into().map_err(|_| XmrError::state_corrupt())?;
            u64::from_be_bytes(exact)
        }
        _ => return Err(XmrError::state_corrupt()),
    };
    let identity = StoredIdentity {
        schema_version: row.0,
        account_id: row.1,
        network: row.2,
        kind,
        primary_address: SecretBytes::new(row.4.into_bytes())
            .map_err(|_| XmrError::state_corrupt())?,
        restore_height,
        greatest_issuance_sequence: row.6,
    };
    identity.validate()?;
    Ok(identity)
}

fn table_names(connection: &Connection) -> Result<Vec<String>, XmrError> {
    let mut statement = connection
        .prepare("SELECT name FROM sqlite_schema WHERE type = 'table' AND name NOT LIKE 'sqlite_%' ORDER BY name")
        .map_err(|_| XmrError::state_corrupt())?;
    statement
        .query_map([], |row| row.get::<_, String>(0))
        .map_err(|_| XmrError::state_corrupt())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| XmrError::state_corrupt())
}

fn column_info(connection: &Connection, table: &str) -> Result<Vec<ColumnInfo>, XmrError> {
    if table != "account_identity" && table != "receivers" {
        return Err(XmrError::state_corrupt());
    }
    let mut statement = connection
        .prepare(&format!("PRAGMA table_info({table})"))
        .map_err(|_| XmrError::state_corrupt())?;
    statement
        .query_map([], |row| {
            Ok(ColumnInfo {
                name: row.get::<_, String>(1)?,
                type_name: row.get::<_, String>(2)?,
                notnull: row.get::<_, i64>(3)? != 0,
                pk: row.get::<_, i64>(5)? != 0,
            })
        })
        .map_err(|_| XmrError::state_corrupt())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| XmrError::state_corrupt())
}

fn table_sql(connection: &Connection, table: &str) -> Result<String, XmrError> {
    if table != "account_identity" && table != "receivers" {
        return Err(XmrError::state_corrupt());
    }
    connection
        .query_row(
            "SELECT sql FROM sqlite_schema WHERE type = 'table' AND name = ?1",
            [table],
            |row| row.get(0),
        )
        .map_err(|_| XmrError::state_corrupt())
}

fn schema_objects(connection: &Connection) -> Result<Vec<(String, String, String)>, XmrError> {
    let mut statement = connection
        .prepare("SELECT type, name, COALESCE(tbl_name, '') FROM sqlite_schema ORDER BY type, name")
        .map_err(|_| XmrError::state_corrupt())?;
    statement
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
            ))
        })
        .map_err(|_| XmrError::state_corrupt())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| XmrError::state_corrupt())
}

fn unique_column_sets(connection: &Connection, table: &str) -> Result<Vec<Vec<String>>, XmrError> {
    if table != "account_identity" && table != "receivers" {
        return Err(XmrError::state_corrupt());
    }
    let mut index_statement = connection
        .prepare(&format!("PRAGMA index_list({table})"))
        .map_err(|_| XmrError::state_corrupt())?;
    let indexes = index_statement
        .query_map([], |row| {
            Ok((row.get::<_, String>(1)?, row.get::<_, i64>(2)?))
        })
        .map_err(|_| XmrError::state_corrupt())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| XmrError::state_corrupt())?;
    let mut sets = Vec::new();
    for (name, unique) in indexes {
        if unique != 1 {
            continue;
        }
        let mut info = connection
            .prepare(&format!("PRAGMA index_info({name})"))
            .map_err(|_| XmrError::state_corrupt())?;
        let mut columns = info
            .query_map([], |row| row.get::<_, String>(2))
            .map_err(|_| XmrError::state_corrupt())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| XmrError::state_corrupt())?;
        columns.sort();
        sets.push(columns);
    }
    let pk: Vec<String> = column_info(connection, table)?
        .into_iter()
        .filter(|column| column.pk)
        .map(|column| column.name)
        .collect();
    if !pk.is_empty() && !sets.iter().any(|set| set == &pk) {
        sets.push(pk);
    }
    Ok(sets)
}

fn configure_full_synchronous<S: StoreSurface>(surface: &mut S) -> Result<(), XmrError> {
    surface.execute_batch("PRAGMA synchronous = FULL;")?;
    require_full_synchronous(surface)
}

fn require_full_synchronous<S: StoreSurface>(surface: &mut S) -> Result<(), XmrError> {
    let value = surface.query_i64("PRAGMA synchronous")?;
    if value == SYNCHRONOUS_FULL_VALUE {
        Ok(())
    } else {
        Err(XmrError::state_corrupt())
    }
}

fn verify_schema<S: StoreSurface>(surface: &mut S) -> Result<(), XmrError> {
    let version = surface.query_i64("PRAGMA user_version")?;
    if version != SCHEMA_VERSION {
        return Err(XmrError::state_corrupt());
    }
    let names = surface.table_names()?;
    if names.as_slice() != ["account_identity", "receivers"] {
        return Err(XmrError::state_corrupt());
    }
    let mut tables = BTreeSet::new();
    let mut receiver_autoindexes = 0usize;
    for (kind, name, tbl) in surface.schema_objects()? {
        match kind.as_str() {
            "table" if name == "account_identity" || name == "receivers" => {
                tables.insert(name);
            }
            "index" if tbl == "receivers" && name.starts_with("sqlite_autoindex_receivers_") => {
                receiver_autoindexes += 1;
            }
            _ => return Err(XmrError::state_corrupt()),
        }
    }
    if tables.len() != 2 || receiver_autoindexes != 4 {
        return Err(XmrError::state_corrupt());
    }
    verify_columns(surface, "account_identity", &IDENTITY_COLUMNS)?;
    verify_columns(surface, "receivers", &RECEIVER_COLUMNS)?;
    let identity_sql = surface.table_sql("account_identity")?;
    let receiver_sql = surface.table_sql("receivers")?;
    if !sql_matches_schema(&identity_sql, "account_identity")
        || !sql_matches_schema(&receiver_sql, "receivers")
    {
        return Err(XmrError::state_corrupt());
    }
    for required in [
        "CHECK (slot = 1)",
        "CHECK (schema_version = 1)",
        "CHECK (length(account_id) = 32)",
        "CHECK (network IN ('xmr-stagenet', 'xmr-testnet'))",
        "CHECK (kind IN (1, 2))",
        "CHECK (length(primary_address) = 95)",
        "CHECK (length(restore_height) = 8)",
        "greatest_issuance_sequence >= 0",
        "greatest_issuance_sequence <= 9223372036854775807",
        "PRIMARY KEY",
    ] {
        if !identity_sql.contains(required) {
            return Err(XmrError::state_corrupt());
        }
    }
    for required in [
        "CHECK (account_index = 0)",
        "subaddress_index > 0",
        "subaddress_index <= 4294967295",
        "issued_at_sequence > 0",
        "issued_at_sequence <= 9223372036854775807",
        "PRIMARY KEY (request_id)",
        "UNIQUE (account_index, subaddress_index)",
        "UNIQUE (subaddress)",
        "UNIQUE (issued_at_sequence)",
    ] {
        if !receiver_sql.contains(required) {
            return Err(XmrError::state_corrupt());
        }
    }
    let receiver_uniques: BTreeSet<Vec<String>> = surface
        .unique_column_sets("receivers")?
        .into_iter()
        .map(|mut set| {
            set.sort();
            set
        })
        .collect();
    let required_uniques: BTreeSet<Vec<String>> = [
        vec!["request_id".to_owned()],
        vec!["account_index".to_owned(), "subaddress_index".to_owned()],
        vec!["subaddress".to_owned()],
        vec!["issued_at_sequence".to_owned()],
    ]
    .into_iter()
    .map(|mut set| {
        set.sort();
        set
    })
    .collect();
    if receiver_uniques != required_uniques {
        return Err(XmrError::state_corrupt());
    }
    Ok(())
}

fn verify_columns<S: StoreSurface>(
    surface: &mut S,
    table: &str,
    expected: &[ColumnSpec],
) -> Result<(), XmrError> {
    let columns = surface.column_info(table)?;
    if columns.len() != expected.len() {
        return Err(XmrError::state_corrupt());
    }
    for (index, (actual, spec)) in columns.iter().zip(expected.iter()).enumerate() {
        if actual.name != spec.name || actual.type_name != spec.type_name || actual.pk != spec.pk {
            return Err(XmrError::state_corrupt());
        }
        if spec.pk {
            if actual.pk != spec.pk {
                return Err(XmrError::state_corrupt());
            }
            let _ = index;
        }
        if spec.notnull && !actual.notnull && !(spec.pk && spec.type_name == "INTEGER") {
            return Err(XmrError::state_corrupt());
        }
        if spec.notnull && spec.pk {
            let sql = surface.table_sql(table)?;
            if !column_sql_declares_not_null(&sql, spec.name) {
                return Err(XmrError::state_corrupt());
            }
        } else if actual.notnull != spec.notnull {
            return Err(XmrError::state_corrupt());
        }
    }
    Ok(())
}

fn normalize_sql(sql: &str) -> String {
    sql.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn schema_table_sql(table: &str) -> Result<String, XmrError> {
    for statement in SCHEMA_SQL.split("CREATE TABLE") {
        let statement = statement.trim();
        if statement.is_empty() {
            continue;
        }
        let full = format!("CREATE TABLE {statement}");
        if full.contains(&format!("CREATE TABLE {table} ("))
            || full.contains(&format!("CREATE TABLE {table}("))
        {
            return Ok(full.trim_end_matches(';').trim().to_owned());
        }
    }
    Err(XmrError::state_corrupt())
}

fn sql_matches_schema(actual: &str, table: &str) -> bool {
    match schema_table_sql(table) {
        Ok(expected) => normalize_sql(actual) == normalize_sql(&expected),
        Err(_) => false,
    }
}

fn column_sql_declares_not_null(sql: &str, column: &str) -> bool {
    let normalized = normalize_sql(sql);
    let patterns = [
        format!("{column} INTEGER PRIMARY KEY NOT NULL"),
        format!("{column} TEXT NOT NULL"),
        format!("{column} INTEGER NOT NULL"),
        format!("{column} BLOB NOT NULL"),
    ];
    patterns
        .iter()
        .any(|pattern| normalized.contains(pattern.as_str()))
}

fn validate_account_directory(path: &Path, expected_owner: u32) -> Result<(), XmrError> {
    validate_path_components(path, expected_owner, true)
}

fn validate_path_components(
    path: &Path,
    expected_owner: u32,
    require_directory: bool,
) -> Result<(), XmrError> {
    #[cfg(not(target_os = "linux"))]
    {
        let _ = (path, expected_owner, require_directory);
        return Err(XmrError::unavailable());
    }
    #[cfg(target_os = "linux")]
    {
        if !path.is_absolute() {
            return Err(XmrError::state_corrupt());
        }
        let mut current = PathBuf::new();
        for component in path.components() {
            current.push(component);
            let metadata = fs::symlink_metadata(&current).map_err(|_| XmrError::state_corrupt())?;
            if metadata.file_type().is_symlink()
                || metadata.file_type().is_fifo()
                || metadata.file_type().is_socket()
                || metadata.file_type().is_block_device()
                || metadata.file_type().is_char_device()
            {
                return Err(XmrError::state_corrupt());
            }
            if current == path {
                if require_directory {
                    if !metadata.file_type().is_dir()
                        || metadata.permissions().mode() & 0o777 != DIRECTORY_MODE
                        || metadata.uid() != expected_owner
                    {
                        return Err(XmrError::state_corrupt());
                    }
                } else if !metadata.file_type().is_file()
                    || metadata.permissions().mode() & 0o777 != STATE_FILE_MODE
                    || metadata.uid() != expected_owner
                {
                    return Err(XmrError::state_corrupt());
                }
            } else if !metadata.file_type().is_dir() {
                return Err(XmrError::state_corrupt());
            }
        }
        Ok(())
    }
}

fn primary_bytes_are_well_formed(bytes: &[u8]) -> bool {
    if bytes.len() != PRIMARY_ADDRESS_BYTES {
        return false;
    }
    let Ok(text) = core::str::from_utf8(bytes) else {
        return false;
    };
    text.as_bytes()
        .iter()
        .all(|byte| MONERO_BASE58.contains(byte))
}

#[cfg(target_os = "linux")]
fn exclusive_create_state_file(path: &Path, _expected_owner: u32) -> Result<File, XmrError> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create_new(true)
        .mode(STATE_FILE_MODE)
        .custom_flags(LINUX_O_NOFOLLOW)
        .open(path)
        .map_err(|error| {
            if error.kind() == std::io::ErrorKind::AlreadyExists {
                XmrError::state_corrupt()
            } else {
                XmrError::internal()
            }
        })
}

#[cfg(target_os = "linux")]
fn created_state_file_identity(file: &File) -> Result<(u64, u64), XmrError> {
    let metadata = file.metadata().map_err(|_| XmrError::internal())?;
    if !metadata.file_type().is_file() {
        return Err(XmrError::internal());
    }
    Ok((metadata.dev(), metadata.ino()))
}

#[cfg(target_os = "linux")]
fn open_account_directory(
    path: &Path,
    expected_owner: u32,
) -> Result<(File, (u64, u64)), XmrError> {
    let listed = fs::symlink_metadata(path).map_err(|_| XmrError::state_corrupt())?;
    if listed.file_type().is_symlink()
        || !listed.file_type().is_dir()
        || listed.uid() != expected_owner
        || listed.permissions().mode() & 0o777 != DIRECTORY_MODE
    {
        return Err(XmrError::state_corrupt());
    }
    let directory = OpenOptions::new()
        .read(true)
        .custom_flags(LINUX_O_NOFOLLOW | LINUX_O_DIRECTORY)
        .open(path)
        .map_err(|_| XmrError::state_corrupt())?;
    let identity = revalidate_opened_directory(&directory, expected_owner)?;
    if identity != (listed.dev(), listed.ino()) {
        return Err(XmrError::state_corrupt());
    }
    Ok((directory, identity))
}

#[cfg(target_os = "linux")]
fn revalidate_opened_directory(
    directory: &File,
    expected_owner: u32,
) -> Result<(u64, u64), XmrError> {
    let metadata = directory
        .metadata()
        .map_err(|_| XmrError::state_corrupt())?;
    if !metadata.file_type().is_dir()
        || metadata.uid() != expected_owner
        || metadata.permissions().mode() & 0o777 != DIRECTORY_MODE
    {
        return Err(XmrError::state_corrupt());
    }
    Ok((metadata.dev(), metadata.ino()))
}

#[cfg(target_os = "linux")]
fn bind_sqlite(
    file: File,
    path: &Path,
    expected_owner: u32,
    newly_created: bool,
) -> Result<(Connection, File, (u64, u64)), XmrError> {
    let before = revalidate_opened_file(&file, expected_owner, STATE_FILE_MODE)?;
    let mut flags = OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_NOFOLLOW;
    if newly_created {
        flags |= OpenFlags::SQLITE_OPEN_CREATE;
    }
    let connection =
        Connection::open_with_flags(path, flags).map_err(|_| XmrError::state_corrupt())?;
    let after_handle = revalidate_opened_file(&file, expected_owner, STATE_FILE_MODE)?;
    if after_handle != before {
        return Err(XmrError::state_corrupt());
    }
    let after = fs::symlink_metadata(path).map_err(|_| XmrError::state_corrupt())?;
    if after.file_type().is_symlink()
        || !after.file_type().is_file()
        || after.uid() != expected_owner
        || after.permissions().mode() & 0o777 != STATE_FILE_MODE
        || after.dev() != before.0
        || after.ino() != before.1
    {
        return Err(XmrError::state_corrupt());
    }
    Ok((connection, file, before))
}

#[cfg(target_os = "linux")]
fn revalidate_opened_file(
    file: &File,
    expected_owner: u32,
    expected_mode: u32,
) -> Result<(u64, u64), XmrError> {
    let metadata = file.metadata().map_err(|_| XmrError::state_corrupt())?;
    if !metadata.file_type().is_file()
        || metadata.uid() != expected_owner
        || metadata.permissions().mode() & 0o777 != expected_mode
    {
        return Err(XmrError::state_corrupt());
    }
    Ok((metadata.dev(), metadata.ino()))
}

fn utf8(bytes: &[u8]) -> Result<&str, XmrError> {
    core::str::from_utf8(bytes).map_err(|_| XmrError::state_corrupt())
}
