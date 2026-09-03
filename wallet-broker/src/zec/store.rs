use std::collections::BTreeSet;
use std::fmt::Write as _;
use std::fs::{self, OpenOptions};
use std::io::Read;
use std::os::unix::fs::{DirBuilderExt, FileTypeExt, MetadataExt, OpenOptionsExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Duration;

use rand_core::OsRng;
use rusqlite::types::ValueRef;
use rusqlite::{Connection, OpenFlags, TransactionBehavior, params};
use sha2::{Digest, Sha256};
use zcash_client_backend::data_api::WalletRead;
use zcash_client_backend::data_api::wallet::{
    ConfirmationsPolicy, create_pczt_from_proposal,
    input_selection::{GreedyInputSelector, SpendPolicy},
    propose_transfer,
};
use zcash_client_backend::fees::{
    DustOutputPolicy, StandardFeeRule, standard::SingleOutputChangeStrategy,
};
use zcash_client_backend::wallet::OvkPolicy;
use zcash_client_backend::zip321::{Payment, TransactionRequest};
use zcash_client_sqlite::chain::init::init_cache_database;
use zcash_client_sqlite::util::SystemClock;
use zcash_client_sqlite::wallet::init::{WalletMigrator, migrations::CURRENT_LEAF_MIGRATIONS};
use zcash_client_sqlite::{BlockDb, WalletDb};
use zcash_keys::address::Address;
use zcash_primitives::transaction::{TxVersion, builder::BundlePadding};
use zcash_protocol::consensus::Parameters;
use zcash_protocol::memo::MemoBytes;
use zcash_protocol::value::Zatoshis;
use zcash_protocol::{PoolType, ShieldedPool};

use crate::vault::{SecretBytes, WipeObserver};

use super::address;
use super::fixture::ValidatedFixture;
use super::hardware::{
    self, ALL_CAPABILITY_FLAGS, CapabilityFlag, DecisionStatus, HardwareCapabilities,
    HardwareDecision, HardwareError, HardwarePrivacy, HardwareRoute, ReviewedProfile, SigningPool,
    VerifiedField,
};
use super::prepare::{PcztInspection, sha256_hex};
use super::scan::{self, ScanFaultPort, ScanInspection, ScanMetrics, ScanRequest};
use super::{
    AccountId, FreshReceiverV1, MAX_COMPACT_BLOCK_BYTES, MAX_DIVERSIFIER_INDEX,
    MAX_ISSUANCE_SEQUENCE, Network, StoreFault, ZecError,
};

const ACCOUNT_TABLE: &str = "ext_bitbook_accounts";
const RECEIVER_TABLE: &str = "ext_bitbook_receiver_state";
const SEQUENCE_TABLE: &str = "ext_bitbook_sequence_state";
const STORE_TABLE: &str = "ext_bitbook_store_state";
const HARDWARE_DECISION_TABLE: &str = "ext_bitbook_hardware_decision";

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
const STORE_SCHEMA: &str = "
CREATE TABLE ext_bitbook_store_state (
    account_id TEXT PRIMARY KEY NOT NULL,
    scan_tip INTEGER,
    checkpoint_receiver_sequence INTEGER NOT NULL
);";
const HARDWARE_DECISION_SCHEMA: &str = "
CREATE TABLE ext_bitbook_hardware_decision (
    singleton INTEGER PRIMARY KEY NOT NULL CHECK (singleton = 1),
    record BLOB NOT NULL,
    record_generation INTEGER NOT NULL CHECK (record_generation > 0),
    generation_check TEXT NOT NULL CHECK (
        length(generation_check) = 64
        AND generation_check NOT GLOB '*[^0-9a-f]*'
    )
);";
const HARDWARE_RECORD_SCHEMA_REVISION: &str = "1";
const HARDWARE_RECORD_CHECK_DOMAIN: &[u8] = b"bitbook.zec.hardware.decision.record.v1";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum HardwarePersistenceFault {
    Write,
    FileSync,
    DirectorySync,
    Commit,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum HardwareRecordMutation {
    UnknownField,
    DuplicateVerifiedField,
    InvalidBoolean,
    OutOfRangeTransactionVersion,
    InvalidFingerprintDigest,
    UnknownStatus,
    SchemaRevisionDrift,
    PartialWrite,
    Rollback,
    TableRevisionDrift,
    ConsensusDrift,
}

#[derive(Clone)]
pub(crate) struct StateRoot {
    path: PathBuf,
    operations: Arc<Mutex<Vec<String>>>,
    filesystem_fault: Arc<Mutex<Option<FilesystemFault>>>,
}

impl StateRoot {
    pub(crate) fn new(path: PathBuf, operations: Arc<Mutex<Vec<String>>>) -> Self {
        Self {
            path,
            operations,
            filesystem_fault: Arc::new(Mutex::new(None)),
        }
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

    pub(crate) fn install_local_wallet_fault(
        &self,
        account_id: &AccountId,
        kind: HostileEntryKind,
    ) {
        let path = self
            .path
            .join("zec-local")
            .join(account_id.as_str())
            .join("wallet.sqlite3");
        *mutex_lock(&self.filesystem_fault) = Some(FilesystemFault { path, kind });
    }

    pub(crate) fn entry_marker(&self) -> Result<Vec<u8>, ZecError> {
        let fault = mutex_lock(&self.filesystem_fault).clone();
        let fault = fault.ok_or_else(ZecError::state_corrupt)?;
        let metadata = fs::symlink_metadata(&fault.path).map_err(|_| ZecError::state_corrupt())?;
        let actual = EntryFacts::from_metadata(&metadata);
        let effective = self.entry_facts(&fault.path, &metadata);
        let mut hasher = Sha256::new();
        hasher.update(actual.marker_bytes());
        hasher.update(effective.marker_bytes());
        hasher.update(metadata.dev().to_le_bytes());
        hasher.update(metadata.ino().to_le_bytes());
        hasher.update(metadata.ctime().to_le_bytes());
        hasher.update(metadata.ctime_nsec().to_le_bytes());
        if actual.identity == EntryIdentity::Regular {
            let mut file = fs::File::open(&fault.path).map_err(|_| ZecError::state_corrupt())?;
            let mut marker = [0; 64];
            let length = file
                .read(&mut marker)
                .map_err(|_| ZecError::state_corrupt())?;
            hasher.update(&marker[..length]);
        } else if actual.identity == EntryIdentity::Symlink {
            let target = fs::read_link(&fault.path).map_err(|_| ZecError::state_corrupt())?;
            hasher.update(target.as_os_str().as_encoded_bytes());
        }
        Ok(hasher.finalize().to_vec())
    }

    fn entry_facts(&self, path: &Path, metadata: &fs::Metadata) -> EntryFacts {
        let actual = EntryFacts::from_metadata(metadata);
        let fault = mutex_lock(&self.filesystem_fault);
        match fault.as_ref().filter(|fault| fault.path == path) {
            Some(fault) => fault.kind.apply(actual),
            None => actual,
        }
    }
}

#[derive(Clone)]
struct FilesystemFault {
    path: PathBuf,
    kind: HostileEntryKind,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum HostileEntryKind {
    Actual,
    Fifo,
    BlockDevice,
    CharacterDevice,
    RegularWrongOwner,
}

impl HostileEntryKind {
    fn apply(self, mut facts: EntryFacts) -> EntryFacts {
        match self {
            Self::Actual => {}
            Self::Fifo => facts.identity = EntryIdentity::Fifo,
            Self::BlockDevice => facts.identity = EntryIdentity::BlockDevice,
            Self::CharacterDevice => facts.identity = EntryIdentity::CharacterDevice,
            Self::RegularWrongOwner => facts.uid = facts.uid.wrapping_add(1),
        }
        facts
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum EntryIdentity {
    Directory,
    Regular,
    Symlink,
    Fifo,
    BlockDevice,
    CharacterDevice,
    Other,
}

#[derive(Clone, Copy)]
struct EntryFacts {
    identity: EntryIdentity,
    mode: u32,
    uid: u32,
    length: u64,
}

impl EntryFacts {
    fn from_metadata(metadata: &fs::Metadata) -> Self {
        let file_type = metadata.file_type();
        let identity = if file_type.is_symlink() {
            EntryIdentity::Symlink
        } else if file_type.is_dir() {
            EntryIdentity::Directory
        } else if file_type.is_file() {
            EntryIdentity::Regular
        } else if file_type.is_fifo() {
            EntryIdentity::Fifo
        } else if file_type.is_block_device() {
            EntryIdentity::BlockDevice
        } else if file_type.is_char_device() {
            EntryIdentity::CharacterDevice
        } else {
            EntryIdentity::Other
        };
        Self {
            identity,
            mode: metadata.permissions().mode() & 0o777,
            uid: metadata.uid(),
            length: metadata.len(),
        }
    }

    fn marker_bytes(self) -> [u8; 24] {
        let mut bytes = [0; 24];
        bytes[0] = match self.identity {
            EntryIdentity::Directory => 1,
            EntryIdentity::Regular => 2,
            EntryIdentity::Symlink => 3,
            EntryIdentity::Fifo => 4,
            EntryIdentity::BlockDevice => 5,
            EntryIdentity::CharacterDevice => 6,
            EntryIdentity::Other => 7,
        };
        bytes[4..8].copy_from_slice(&self.mode.to_le_bytes());
        bytes[8..12].copy_from_slice(&self.uid.to_le_bytes());
        bytes[16..24].copy_from_slice(&self.length.to_le_bytes());
        bytes
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

pub(crate) struct PreparedBuild {
    pub raw: SecretBytes,
    pub fee_zat: u64,
    pub inspection: PcztInspection,
}

pub(crate) struct AddressAccount {
    root: StateRoot,
    account_id: AccountId,
    network: Network,
    paths: AccountPaths,
    gate: Arc<Mutex<()>>,
    fault: Mutex<Option<AddressFaultPort>>,
    store_fault: Mutex<Option<StoreFault>>,
    allocation_observer: Mutex<Option<usize>>,
    scan_fault: Mutex<Option<ScanFaultPort>>,
    scan_metrics: Mutex<ScanMetrics>,
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
        initialize_official_wallet(&root, &paths.wallet, network)?;
        initialize_official_cache(&root, &paths.compact)?;
        validate_account_paths(&root, &paths)?;

        initialize_extension(&root, &paths.wallet, &account_id, network, &ufvk)?;
        sync_bootstrap_entries(&root, &paths)?;
        root.record("zec-address-bootstrap");

        Ok(Self {
            root,
            account_id,
            network,
            paths,
            gate: Arc::new(Mutex::new(())),
            fault: Mutex::new(None),
            store_fault: Mutex::new(None),
            allocation_observer: Mutex::new(None),
            scan_fault: Mutex::new(None),
            scan_metrics: Mutex::new(ScanMetrics::default()),
        })
    }

    pub(crate) fn open_viewing(root: StateRoot, account_id: AccountId) -> Result<Self, ZecError> {
        let network = detect_network(&root, &account_id)?;
        Self::open_viewing_with_network(root, account_id, network)
    }

    pub(crate) fn open_viewing_with_network(
        root: StateRoot,
        account_id: AccountId,
        network: Network,
    ) -> Result<Self, ZecError> {
        let paths = account_paths(&root, &account_id, network)?;
        scan::recover_account(&root, &paths, network)?;
        let version = preflight_store(&root, &paths, &account_id, network)?;
        if version == BrokerSchemaVersion::V0 {
            migrate_extension(&root, &paths, &account_id, network, None)?;
            if preflight_store(&root, &paths, &account_id, network)? != BrokerSchemaVersion::V1 {
                return Err(ZecError::state_corrupt());
            }
        }
        root.record("zec-address-open-viewing");
        Ok(Self {
            root,
            account_id,
            network,
            paths,
            gate: Arc::new(Mutex::new(())),
            fault: Mutex::new(None),
            store_fault: Mutex::new(None),
            allocation_observer: Mutex::new(None),
            scan_fault: Mutex::new(None),
            scan_metrics: Mutex::new(ScanMetrics::default()),
        })
    }

    pub(crate) fn fresh_receiver(&self, _now: u64) -> Result<FreshReceiverV1, ZecError> {
        let _guard = mutex_lock(&self.gate);
        let fault = *mutex_lock(&self.fault);
        issue_receiver(
            &self.root,
            &self.paths,
            &self.account_id,
            self.network,
            fault,
        )
    }

    pub(crate) fn build_prepared_pczt(
        &self,
        receiver: &str,
        amount: u64,
        memo: &str,
        request_id: &str,
        intent_hash: &str,
    ) -> Result<PreparedBuild, ZecError> {
        let _guard = mutex_lock(&self.gate);
        validate_account_paths(&self.root, &self.paths)?;
        if !preflight_store(&self.root, &self.paths, &self.account_id, self.network)?
            .has_store_state()
        {
            return Err(ZecError::state_corrupt());
        }
        let cache = open_read_only_connection(&self.root, &self.paths.compact)?;
        validate_cache_schema(&cache)?;
        drop(cache);
        let connection = open_read_write_no_create_connection(&self.root, &self.paths.wallet)?;
        rusqlite::vtab::array::load_module(&connection).map_err(|_| ZecError::state_corrupt())?;
        match self.network {
            Network::Testnet => build_prepared_for(
                connection,
                zcash_protocol::consensus::Network::TestNetwork,
                self.network,
                receiver,
                amount,
                memo,
                request_id,
                intent_hash,
            ),
            Network::Local(local) => build_prepared_for(
                connection,
                local.upstream(),
                self.network,
                receiver,
                amount,
                memo,
                request_id,
                intent_hash,
            ),
        }
    }

    pub(crate) fn inspect_state(&self) -> Result<ReceiverState, ZecError> {
        let _guard = mutex_lock(&self.gate);
        scan::recover_account(&self.root, &self.paths, self.network)?;
        read_receiver_state(&self.root, &self.paths, &self.account_id, self.network)
    }

    pub(crate) fn set_state_for_test(&self, index: u64, sequence: u64) -> Result<(), ZecError> {
        if index > MAX_DIVERSIFIER_INDEX || sequence > MAX_ISSUANCE_SEQUENCE {
            return Err(ZecError::limit());
        }
        let _guard = mutex_lock(&self.gate);
        validate_account_paths(&self.root, &self.paths)?;
        let mut connection = open_connection(&self.root, &self.paths.wallet)?;
        let transaction = connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| ZecError::state_corrupt())?;
        if !validate_extension_and_binding_with_connection(
            &transaction,
            &self.account_id,
            self.network,
        )?
        .has_store_state()
        {
            return Err(ZecError::state_corrupt());
        }
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

    pub(crate) fn network(&self) -> Network {
        self.network
    }

    pub(crate) fn inspect_paths(&self) -> StorePaths {
        StorePaths {
            relative_account_dir: format!("{}/{}", self.network.as_str(), self.account_id.as_str()),
            wallet_db_file: "wallet.sqlite3",
            compact_cache_file: "compact.sqlite3",
            account_directory: self.paths.directory.clone(),
            wallet_db: self.paths.wallet.clone(),
            compact_cache: self.paths.compact.clone(),
        }
    }

    pub(crate) fn inspect_store(&self) -> Result<StoreInspection, ZecError> {
        let _guard = mutex_lock(&self.gate);
        scan::recover_account(&self.root, &self.paths, self.network)?;
        let version = preflight_store(&self.root, &self.paths, &self.account_id, self.network)?;
        let connection = open_read_only_connection(&self.root, &self.paths.wallet)?;
        let receiver = read_receiver_state_with_connection(&connection, &self.account_id)?;
        let scan_tip = if version.has_store_state() {
            connection
                .query_row(
                    "SELECT scan_tip FROM ext_bitbook_store_state WHERE account_id = ?1",
                    [self.account_id.as_str()],
                    |row| row.get::<_, Option<i64>>(0),
                )
                .map_err(|_| ZecError::state_corrupt())?
                .map(|value| u32::try_from(value).map_err(|_| ZecError::state_corrupt()))
                .transpose()?
        } else {
            None
        };
        Ok(StoreInspection {
            account_id: self.account_id.as_str().to_owned(),
            network: self.network.as_str(),
            schema_version: version.as_str(),
            scan_tip,
            receiver_sequence: receiver.issued_at_sequence,
        })
    }

    pub(crate) fn install_previous_schema_for_test(&self) -> Result<(), ZecError> {
        let _guard = mutex_lock(&self.gate);
        if preflight_store(&self.root, &self.paths, &self.account_id, self.network)?
            != BrokerSchemaVersion::V1
        {
            return Err(ZecError::state_corrupt());
        }
        let mut connection = open_connection(&self.root, &self.paths.wallet)?;
        configure_full_synchronous(&connection)?;
        let transaction = connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| ZecError::state_corrupt())?;
        validate_upstream_schema(&transaction)?;
        if validate_extension_and_binding_with_connection(
            &transaction,
            &self.account_id,
            self.network,
        )? != BrokerSchemaVersion::V1
        {
            return Err(ZecError::state_corrupt());
        }
        transaction
            .execute_batch("DROP TABLE ext_bitbook_store_state;")
            .map_err(|_| ZecError::state_corrupt())?;
        if recognize_extension_schema(&transaction)? != BrokerSchemaVersion::V0 {
            return Err(ZecError::state_corrupt());
        }
        transaction.commit().map_err(|_| ZecError::state_corrupt())
    }

    pub(crate) fn arm_store_fault(&self, fault: StoreFault) {
        *mutex_lock(&self.store_fault) = Some(fault);
    }

    pub(crate) fn reopen_and_migrate(&self) -> Result<(), ZecError> {
        let _guard = mutex_lock(&self.gate);
        scan::recover_account(&self.root, &self.paths, self.network)?;
        if preflight_store(&self.root, &self.paths, &self.account_id, self.network)?
            != BrokerSchemaVersion::V0
        {
            return Err(ZecError::state_corrupt());
        }
        let fault = mutex_lock(&self.store_fault).take();
        migrate_extension(
            &self.root,
            &self.paths,
            &self.account_id,
            self.network,
            fault,
        )
    }

    pub(crate) fn persist_checkpoint(&self, height: u32) -> Result<(), ZecError> {
        let _guard = mutex_lock(&self.gate);
        if !preflight_store(&self.root, &self.paths, &self.account_id, self.network)?
            .has_store_state()
        {
            return Err(ZecError::state_corrupt());
        }
        let fault = mutex_lock(&self.store_fault).take();
        let mut connection = open_connection(&self.root, &self.paths.wallet)?;
        configure_full_synchronous(&connection)?;
        let transaction = connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| ZecError::state_corrupt())?;
        validate_upstream_schema(&transaction)?;
        if !validate_extension_and_binding_with_connection(
            &transaction,
            &self.account_id,
            self.network,
        )?
        .has_store_state()
        {
            return Err(ZecError::state_corrupt());
        }
        let receiver_sequence =
            read_receiver_state_with_connection(&transaction, &self.account_id)?.issued_at_sequence;
        let updated = transaction
            .execute(
                "UPDATE ext_bitbook_store_state
                 SET scan_tip = ?1, checkpoint_receiver_sequence = ?2
                 WHERE account_id = ?3",
                params![
                    i64::from(height),
                    i64::try_from(receiver_sequence).map_err(|_| ZecError::limit())?,
                    self.account_id.as_str()
                ],
            )
            .map_err(|_| ZecError::state_corrupt())?;
        if updated != 1 || fault == Some(StoreFault::Write) {
            return Err(ZecError::state_corrupt());
        }
        if fault == Some(StoreFault::FileSync) {
            return Err(ZecError::internal());
        }
        if fault == Some(StoreFault::DirectorySync) {
            return Err(ZecError::internal());
        }
        transaction.commit().map_err(|_| ZecError::state_corrupt())
    }

    pub(crate) fn persist_hardware_decision(
        &self,
        profiles: &[ReviewedProfile],
        decision: &HardwareDecision,
        expansion_authorized: bool,
        fault: Option<HardwarePersistenceFault>,
    ) -> Result<(), HardwareError> {
        let _guard = mutex_lock(&self.gate);
        hardware::validate_persisted_decision(profiles, decision)?;
        validate_account_paths(&self.root, &self.paths)
            .map_err(|_| HardwareError::state_corrupt())?;
        let mut connection = open_read_write_no_create_connection(&self.root, &self.paths.wallet)
            .map_err(|_| HardwareError::state_corrupt())?;
        configure_full_synchronous(&connection).map_err(|_| HardwareError::state_corrupt())?;
        let transaction = connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| HardwareError::internal())?;
        validate_upstream_schema(&transaction).map_err(|_| HardwareError::state_corrupt())?;
        let version = validate_extension_and_binding_with_connection(
            &transaction,
            &self.account_id,
            self.network,
        )
        .map_err(|_| HardwareError::state_corrupt())?;
        if !version.has_store_state() {
            return Err(HardwareError::state_corrupt());
        }

        let prior = if version == BrokerSchemaVersion::V2 {
            Some(load_hardware_record(&transaction, profiles)?)
        } else {
            None
        };
        if let Some(prior) = &prior
            && !hardware::decision_narrows(&prior.decision, decision)
            && !expansion_authorized
        {
            return Err(HardwareError::state_corrupt());
        }
        let generation = match prior.as_ref() {
            Some(prior) => prior
                .generation
                .checked_add(1)
                .ok_or_else(HardwareError::state_corrupt)?,
            None => 1,
        };
        let encoded = encode_hardware_record(decision, generation)?;
        let generation_i64 =
            i64::try_from(generation).map_err(|_| HardwareError::state_corrupt())?;

        let changed = if version == BrokerSchemaVersion::V1 {
            transaction
                .execute_batch(HARDWARE_DECISION_SCHEMA)
                .map_err(|_| HardwareError::internal())?;
            transaction
                .execute(
                    "INSERT INTO ext_bitbook_hardware_decision
                     (singleton, record, record_generation, generation_check)
                     VALUES (1, ?1, ?2, ?3)",
                    params![encoded.bytes, generation_i64, encoded.generation_check],
                )
                .map_err(|_| HardwareError::internal())?
        } else {
            transaction
                .execute(
                    "UPDATE ext_bitbook_hardware_decision
                     SET record = ?1, record_generation = ?2, generation_check = ?3
                     WHERE singleton = 1",
                    params![encoded.bytes, generation_i64, encoded.generation_check],
                )
                .map_err(|_| HardwareError::internal())?
        };
        if changed != 1 || fault == Some(HardwarePersistenceFault::Write) {
            return Err(HardwareError::internal());
        }
        if recognize_extension_schema(&transaction).map_err(|_| HardwareError::state_corrupt())?
            != BrokerSchemaVersion::V2
        {
            return Err(HardwareError::state_corrupt());
        }
        let stored = load_hardware_record(&transaction, profiles)?;
        if stored.decision != *decision || stored.generation != generation {
            return Err(HardwareError::state_corrupt());
        }

        if fault == Some(HardwarePersistenceFault::FileSync) {
            return Err(HardwareError::internal());
        }
        OpenOptions::new()
            .read(true)
            .open(&self.paths.wallet)
            .and_then(|file| file.sync_all())
            .map_err(|_| HardwareError::internal())?;
        if fault == Some(HardwarePersistenceFault::DirectorySync) {
            return Err(HardwareError::internal());
        }
        fs::File::open(&self.paths.directory)
            .and_then(|directory| directory.sync_all())
            .map_err(|_| HardwareError::internal())?;
        if fault == Some(HardwarePersistenceFault::Commit) {
            return Err(HardwareError::internal());
        }
        transaction.commit().map_err(|_| HardwareError::internal())
    }

    pub(crate) fn load_hardware_decision(
        &self,
        profiles: &[ReviewedProfile],
    ) -> Result<Option<HardwareDecision>, HardwareError> {
        let _guard = mutex_lock(&self.gate);
        let record = open_hardware_record(
            &self.root,
            &self.paths,
            &self.account_id,
            self.network,
            profiles,
        )?;
        Ok(record.map(|record| record.decision))
    }

    pub(crate) fn persisted_hardware_bytes(
        &self,
        profiles: &[ReviewedProfile],
    ) -> Result<Vec<u8>, HardwareError> {
        let _guard = mutex_lock(&self.gate);
        let record = open_hardware_record(
            &self.root,
            &self.paths,
            &self.account_id,
            self.network,
            profiles,
        )?;
        Ok(record.map_or_else(Vec::new, |record| record.bytes))
    }

    pub(crate) fn mutate_hardware_record_for_test(
        &self,
        mutation: HardwareRecordMutation,
    ) -> Result<(), HardwareError> {
        let _guard = mutex_lock(&self.gate);
        validate_account_paths(&self.root, &self.paths)
            .map_err(|_| HardwareError::state_corrupt())?;
        let mut connection = open_read_write_no_create_connection(&self.root, &self.paths.wallet)
            .map_err(|_| HardwareError::state_corrupt())?;
        configure_full_synchronous(&connection).map_err(|_| HardwareError::state_corrupt())?;
        let transaction = connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| HardwareError::internal())?;
        validate_upstream_schema(&transaction).map_err(|_| HardwareError::state_corrupt())?;
        if validate_extension_and_binding_with_connection(
            &transaction,
            &self.account_id,
            self.network,
        )
        .map_err(|_| HardwareError::state_corrupt())?
            != BrokerSchemaVersion::V2
        {
            return Err(HardwareError::state_corrupt());
        }
        if mutation == HardwareRecordMutation::Rollback {
            let changed = transaction
                .execute(
                    "UPDATE ext_bitbook_hardware_decision
                     SET record_generation = record_generation + 1 WHERE singleton = 1",
                    [],
                )
                .map_err(|_| HardwareError::internal())?;
            if changed != 1 {
                return Err(HardwareError::state_corrupt());
            }
        } else {
            let bytes = transaction
                .query_row(
                    "SELECT record FROM ext_bitbook_hardware_decision WHERE singleton = 1",
                    [],
                    |row| row.get::<_, Vec<u8>>(0),
                )
                .map_err(|_| HardwareError::state_corrupt())?;
            let mutated = mutate_hardware_record_bytes(bytes, mutation)?;
            let changed = transaction
                .execute(
                    "UPDATE ext_bitbook_hardware_decision SET record = ?1 WHERE singleton = 1",
                    [mutated],
                )
                .map_err(|_| HardwareError::internal())?;
            if changed != 1 {
                return Err(HardwareError::state_corrupt());
            }
        }
        transaction.commit().map_err(|_| HardwareError::internal())
    }

    pub(crate) fn mutate_sqlite_for_test(&self, mutation: &str) -> Result<(), ZecError> {
        let _guard = mutex_lock(&self.gate);
        validate_account_paths(&self.root, &self.paths)?;
        match mutation {
            "truncated-header" => {
                let bytes = fs::read(&self.paths.wallet).map_err(|_| ZecError::state_corrupt())?;
                fs::write(&self.paths.wallet, &bytes[..bytes.len().min(15)])
                    .map_err(|_| ZecError::state_corrupt())
            }
            "invalid-page-size" => {
                let mut bytes =
                    fs::read(&self.paths.wallet).map_err(|_| ZecError::state_corrupt())?;
                if bytes.len() < 18 {
                    return Err(ZecError::state_corrupt());
                }
                bytes[16] = 0;
                bytes[17] = 3;
                fs::write(&self.paths.wallet, bytes).map_err(|_| ZecError::state_corrupt())
            }
            "unknown-schema" => {
                let connection = open_connection(&self.root, &self.paths.wallet)?;
                connection
                    .execute_batch("CREATE TABLE ext_bitbook_unknown (value INTEGER);")
                    .map_err(|_| ZecError::state_corrupt())
            }
            "wrong-network" => {
                let connection = open_connection(&self.root, &self.paths.wallet)?;
                let count = connection
                    .execute(
                        "UPDATE ext_bitbook_accounts SET network = 'zec-testnet'",
                        [],
                    )
                    .map_err(|_| ZecError::state_corrupt())?;
                if count == 1 {
                    Ok(())
                } else {
                    Err(ZecError::state_corrupt())
                }
            }
            "wrong-account" => {
                let connection = open_connection(&self.root, &self.paths.wallet)?;
                let count = connection
                    .execute(
                        "UPDATE ext_bitbook_accounts
                         SET account_id = 'ffffffffffffffffffffffffffffffff'",
                        [],
                    )
                    .map_err(|_| ZecError::state_corrupt())?;
                if count == 1 {
                    Ok(())
                } else {
                    Err(ZecError::state_corrupt())
                }
            }
            _ => Err(ZecError::schema()),
        }
    }

    pub(crate) fn inspect_sqlite_for_test(&self) -> Result<SqliteInspectionData, ZecError> {
        let _guard = mutex_lock(&self.gate);
        scan::recover_account(&self.root, &self.paths, self.network)?;
        preflight_store(&self.root, &self.paths, &self.account_id, self.network)?;
        let connection = open_read_only_connection(&self.root, &self.paths.wallet)?;
        inspect_sqlite(&connection)
    }

    pub(crate) fn reset_allocation_observer(&self) {
        *mutex_lock(&self.allocation_observer) = None;
    }

    pub(crate) fn read_manifest_sized_for_test(&self, length: usize) -> Result<(), ZecError> {
        let bytes = super::fixture::allocate_manifest_sized(length)?;
        *mutex_lock(&self.allocation_observer) = Some(bytes.len());
        Ok(())
    }

    pub(crate) fn observed_allocation_bytes(&self) -> Option<usize> {
        *mutex_lock(&self.allocation_observer)
    }

    pub(crate) fn scan_fixture(
        &self,
        fixture: &ValidatedFixture,
        request: ScanRequest,
    ) -> Result<(), ZecError> {
        let _guard = mutex_lock(&self.gate);
        let fault = mutex_lock(&self.scan_fault).take();
        let mut metrics = mutex_lock(&self.scan_metrics);
        scan::execute(
            &self.root,
            &self.paths,
            &self.account_id,
            self.network,
            scan::ScanPlan {
                fixture,
                request,
                fault,
            },
            &mut metrics,
        )
    }

    pub(crate) fn inspect_scan(&self) -> Result<ScanInspection, ZecError> {
        let _guard = mutex_lock(&self.gate);
        let metrics = mutex_lock(&self.scan_metrics);
        let checkpoint = match self.network {
            Network::Testnet => 0,
            Network::Local(local) => local
                .birthday_height()
                .checked_sub(1)
                .ok_or_else(ZecError::state_corrupt)?,
        };
        scan::inspect(
            &self.root,
            &self.paths,
            &self.account_id,
            self.network,
            checkpoint,
            metrics.balance_override,
        )
    }

    pub(crate) fn arm_scan_fault(&self, fault: ScanFaultPort) {
        *mutex_lock(&self.scan_fault) = Some(fault);
    }

    pub(crate) fn scan_metrics(&self) -> ScanMetrics {
        mutex_lock(&self.scan_metrics).clone()
    }

    pub(crate) fn recognized_note_count(&self) -> Result<usize, ZecError> {
        let _guard = mutex_lock(&self.gate);
        scan::recover_account(&self.root, &self.paths, self.network)?;
        scan::recognized_note_count(&self.paths.wallet)
    }

    pub(crate) fn set_balance_for_test(&self, value: u64) {
        // The override remains process-local test state; scan inspection never persists it or
        // classifies it as an official pool balance.
        mutex_lock(&self.scan_metrics).balance_override = Some(value);
    }

    pub(crate) fn add_recognized_value_for_test(&self, value: u64) -> Result<(), ZecError> {
        let mut metrics = mutex_lock(&self.scan_metrics);
        let current = metrics.balance_override.ok_or_else(ZecError::schema)?;
        metrics.balance_override = Some(current.checked_add(value).ok_or_else(ZecError::limit)?);
        Ok(())
    }

    pub(crate) fn decode_sized_compact_block_for_test(
        &self,
        length: usize,
    ) -> Result<(), ZecError> {
        let mut metrics = mutex_lock(&self.scan_metrics);
        metrics.last_block_allocation = None;
        if length > MAX_COMPACT_BLOCK_BYTES {
            return Err(ZecError::limit());
        }
        let bytes = vec![0; length];
        metrics.last_block_allocation = Some(bytes.len());
        Ok(())
    }
}

pub(crate) struct StorePaths {
    pub relative_account_dir: String,
    pub wallet_db_file: &'static str,
    pub compact_cache_file: &'static str,
    pub account_directory: PathBuf,
    pub wallet_db: PathBuf,
    pub compact_cache: PathBuf,
}

pub(crate) struct StoreInspection {
    pub account_id: String,
    pub network: &'static str,
    pub schema_version: &'static str,
    pub scan_tip: Option<u32>,
    pub receiver_sequence: u64,
}

pub(crate) struct SqliteInspectionData {
    pub tables: Vec<String>,
    pub columns: Vec<String>,
    pub decoded_rows: usize,
    pub value_kinds: Vec<&'static str>,
    pub decoded_values: Vec<Vec<u8>>,
}

struct EncodedHardwareRecord {
    bytes: Vec<u8>,
    generation_check: String,
}

struct DecodedHardwareRecord {
    decision: HardwareDecision,
    generation: u64,
    generation_check: String,
    bytes: Vec<u8>,
}

fn open_hardware_record(
    root: &StateRoot,
    paths: &AccountPaths,
    account_id: &AccountId,
    network: Network,
    profiles: &[ReviewedProfile],
) -> Result<Option<DecodedHardwareRecord>, HardwareError> {
    validate_account_paths(root, paths).map_err(|_| HardwareError::state_corrupt())?;
    let connection = open_read_only_connection(root, &paths.wallet)
        .map_err(|_| HardwareError::state_corrupt())?;
    validate_upstream_schema(&connection).map_err(|_| HardwareError::state_corrupt())?;
    let version = validate_extension_and_binding_with_connection(&connection, account_id, network)
        .map_err(|_| HardwareError::state_corrupt())?;
    match version {
        BrokerSchemaVersion::V1 => Ok(None),
        BrokerSchemaVersion::V2 => load_hardware_record(&connection, profiles).map(Some),
        BrokerSchemaVersion::V0 => Err(HardwareError::state_corrupt()),
    }
}

fn load_hardware_record(
    connection: &Connection,
    profiles: &[ReviewedProfile],
) -> Result<DecodedHardwareRecord, HardwareError> {
    let (bytes, stored_generation, stored_check, record_type, generation_type, check_type) =
        connection
            .query_row(
                "SELECT record, record_generation, generation_check,
                    typeof(record), typeof(record_generation), typeof(generation_check)
             FROM ext_bitbook_hardware_decision WHERE singleton = 1",
                [],
                |row| {
                    Ok((
                        row.get::<_, Vec<u8>>(0)?,
                        row.get::<_, i64>(1)?,
                        row.get::<_, String>(2)?,
                        row.get::<_, String>(3)?,
                        row.get::<_, String>(4)?,
                        row.get::<_, String>(5)?,
                    ))
                },
            )
            .map_err(|_| HardwareError::state_corrupt())?;
    if (
        record_type.as_str(),
        generation_type.as_str(),
        check_type.as_str(),
    ) != ("blob", "integer", "text")
    {
        return Err(HardwareError::state_corrupt());
    }
    let decoded = decode_hardware_record(&bytes)?;
    if i64::try_from(decoded.generation).ok() != Some(stored_generation)
        || decoded.generation_check != stored_check
    {
        return Err(HardwareError::state_corrupt());
    }
    hardware::validate_persisted_decision(profiles, &decoded.decision)?;
    Ok(decoded)
}

fn encode_hardware_record(
    decision: &HardwareDecision,
    generation: u64,
) -> Result<EncodedHardwareRecord, HardwareError> {
    if generation == 0 || generation > i64::MAX as u64 {
        return Err(HardwareError::state_corrupt());
    }
    let mut record = String::new();
    append_hardware_field(
        &mut record,
        "schema_revision",
        HARDWARE_RECORD_SCHEMA_REVISION,
    )?;
    append_hardware_field(
        &mut record,
        "fingerprint_digest",
        &decision.fingerprint_digest,
    )?;
    append_hardware_field(&mut record, "table_revision", &decision.table_revision)?;
    append_hardware_field(&mut record, "status", decision.status.code())?;
    append_hardware_field(&mut record, "privacy", decision.privacy.as_str())?;
    for capability in ALL_CAPABILITY_FLAGS {
        append_hardware_field(
            &mut record,
            capability_field_name(capability),
            canonical_bool(decision.capabilities.contains(capability)),
        )?;
    }
    append_hardware_field(
        &mut record,
        "transaction_version",
        &decision.capabilities.transaction_version,
    )?;
    append_hardware_field(
        &mut record,
        "consensus_branch",
        &decision.capabilities.consensus_branch,
    )?;
    append_hardware_field(
        &mut record,
        "pczt_encoding_version",
        &decision.capabilities.pczt_encoding_version,
    )?;
    append_hardware_field(
        &mut record,
        "signing_pools",
        &join_signing_pools(&decision.capabilities.allowed_signing_pools),
    )?;
    append_hardware_field(
        &mut record,
        "verified_fields",
        &join_verified_fields(&decision.verified_fields),
    )?;
    append_hardware_field(
        &mut record,
        "host_trusting_fields",
        &join_verified_fields(&decision.host_trusting_fields),
    )?;
    append_hardware_field(
        &mut record,
        "route",
        decision.route.map_or("none", HardwareRoute::as_str),
    )?;
    append_hardware_field(
        &mut record,
        "pay_eligible",
        canonical_bool(decision.pay_eligible),
    )?;
    append_hardware_field(
        &mut record,
        "electron_verified_fields",
        canonical_bool(decision.electron_verified_fields),
    )?;
    append_hardware_field(&mut record, "generation", &generation.to_string())?;
    let generation_check = hardware_record_check(record.as_bytes());
    append_hardware_field(&mut record, "generation_check", &generation_check)?;
    Ok(EncodedHardwareRecord {
        bytes: record.into_bytes(),
        generation_check,
    })
}

fn decode_hardware_record(bytes: &[u8]) -> Result<DecodedHardwareRecord, HardwareError> {
    if bytes.is_empty() || bytes.len() > 16 * 1024 || bytes.last() != Some(&b'\n') {
        return Err(HardwareError::state_corrupt());
    }
    let text = std::str::from_utf8(bytes).map_err(|_| HardwareError::state_corrupt())?;
    let mut lines = text.split_terminator('\n');
    if take_hardware_field(&mut lines, "schema_revision")? != HARDWARE_RECORD_SCHEMA_REVISION {
        return Err(HardwareError::state_corrupt());
    }
    let fingerprint_digest = take_hardware_field(&mut lines, "fingerprint_digest")?.to_owned();
    let table_revision = take_hardware_field(&mut lines, "table_revision")?.to_owned();
    let status = parse_decision_status(take_hardware_field(&mut lines, "status")?)?;
    let privacy = parse_hardware_privacy(take_hardware_field(&mut lines, "privacy")?)?;
    let mut capabilities = HardwareCapabilities::default();
    for capability in ALL_CAPABILITY_FLAGS {
        let value = take_hardware_field(&mut lines, capability_field_name(capability))?;
        capabilities.set(capability, parse_canonical_bool(value)?);
    }
    capabilities.transaction_version =
        take_hardware_field(&mut lines, "transaction_version")?.to_owned();
    capabilities.consensus_branch = take_hardware_field(&mut lines, "consensus_branch")?.to_owned();
    capabilities.pczt_encoding_version =
        take_hardware_field(&mut lines, "pczt_encoding_version")?.to_owned();
    capabilities.allowed_signing_pools =
        parse_signing_pools(take_hardware_field(&mut lines, "signing_pools")?)?;
    let verified_fields =
        parse_verified_fields(take_hardware_field(&mut lines, "verified_fields")?)?;
    let host_trusting_fields =
        parse_verified_fields(take_hardware_field(&mut lines, "host_trusting_fields")?)?;
    let route = parse_hardware_route(take_hardware_field(&mut lines, "route")?)?;
    let pay_eligible = parse_canonical_bool(take_hardware_field(&mut lines, "pay_eligible")?)?;
    let electron_verified_fields =
        parse_canonical_bool(take_hardware_field(&mut lines, "electron_verified_fields")?)?;
    let generation_text = take_hardware_field(&mut lines, "generation")?;
    let generation = generation_text
        .parse::<u64>()
        .map_err(|_| HardwareError::state_corrupt())?;
    if generation == 0 || generation > i64::MAX as u64 || generation.to_string() != generation_text
    {
        return Err(HardwareError::state_corrupt());
    }
    let generation_check = take_hardware_field(&mut lines, "generation_check")?.to_owned();
    if lines.next().is_some() {
        return Err(HardwareError::state_corrupt());
    }
    let decision = HardwareDecision {
        fingerprint_digest,
        table_revision,
        status,
        privacy,
        capabilities,
        verified_fields,
        host_trusting_fields,
        route,
        route_claims: Vec::new(),
        pay_eligible,
        electron_verified_fields,
    };
    let canonical = encode_hardware_record(&decision, generation)?;
    if canonical.bytes != bytes || canonical.generation_check != generation_check {
        return Err(HardwareError::state_corrupt());
    }
    Ok(DecodedHardwareRecord {
        decision,
        generation,
        generation_check,
        bytes: bytes.to_vec(),
    })
}

fn append_hardware_field(
    target: &mut String,
    name: &str,
    value: &str,
) -> Result<(), HardwareError> {
    if value
        .bytes()
        .any(|byte| matches!(byte, b'\n' | b'\r' | b'='))
    {
        return Err(HardwareError::state_corrupt());
    }
    writeln!(target, "{name}={value}").map_err(|_| HardwareError::internal())
}

fn take_hardware_field<'a>(
    lines: &mut impl Iterator<Item = &'a str>,
    name: &str,
) -> Result<&'a str, HardwareError> {
    let line = lines.next().ok_or_else(HardwareError::state_corrupt)?;
    let value = line
        .strip_prefix(name)
        .and_then(|suffix| suffix.strip_prefix('='))
        .ok_or_else(HardwareError::state_corrupt)?;
    if line.as_bytes().get(name.len()) != Some(&b'=') {
        return Err(HardwareError::state_corrupt());
    }
    Ok(value)
}

fn canonical_bool(value: bool) -> &'static str {
    if value { "1" } else { "0" }
}

fn parse_canonical_bool(value: &str) -> Result<bool, HardwareError> {
    match value {
        "0" => Ok(false),
        "1" => Ok(true),
        _ => Err(HardwareError::state_corrupt()),
    }
}

fn capability_field_name(capability: CapabilityFlag) -> &'static str {
    match capability {
        CapabilityFlag::CanView => "can_view",
        CapabilityFlag::CanDeriveFreshReceiver => "can_derive_fresh_receiver",
        CapabilityFlag::CanReceivePrivate => "can_receive_private",
        CapabilityFlag::CanReceiveOrchard => "can_receive_orchard",
        CapabilityFlag::CanReceiveIronwood => "can_receive_ironwood",
        CapabilityFlag::CanPrepareTx => "can_prepare_tx",
        CapabilityFlag::CanSignSpend => "can_sign_spend",
        CapabilityFlag::CanSignOrchard => "can_sign_orchard",
        CapabilityFlag::CanSignIronwood => "can_sign_ironwood",
        CapabilityFlag::CanTxV6 => "can_tx_v6",
        CapabilityFlag::CanMigrateOrchardToIronwood => "can_migrate_orchard_to_ironwood",
        CapabilityFlag::CanSignTransparent => "can_sign_transparent",
        CapabilityFlag::CanDisplayAmountOnDevice => "can_display_amount_on_device",
        CapabilityFlag::CanDisplayRecipientOnDevice => "can_display_recipient_on_device",
        CapabilityFlag::CanDisplayNetworkOnDevice => "can_display_network_on_device",
        CapabilityFlag::CanVerifyPcztOnDevice => "can_verify_pczt_on_device",
        CapabilityFlag::CanExportViewingMaterial => "can_export_viewing_material",
        CapabilityFlag::CanBroadcast => "can_broadcast",
    }
}

fn join_signing_pools(values: &[SigningPool]) -> String {
    values
        .iter()
        .copied()
        .map(SigningPool::as_str)
        .collect::<Vec<_>>()
        .join(",")
}

fn join_verified_fields(values: &[VerifiedField]) -> String {
    values
        .iter()
        .copied()
        .map(VerifiedField::as_str)
        .collect::<Vec<_>>()
        .join(",")
}

fn parse_signing_pools(value: &str) -> Result<Vec<SigningPool>, HardwareError> {
    parse_hardware_list(value, |item| match item {
        "transparent" => Some(SigningPool::Transparent),
        "orchard" => Some(SigningPool::Orchard),
        "ironwood" => Some(SigningPool::Ironwood),
        _ => None,
    })
}

fn parse_verified_fields(value: &str) -> Result<Vec<VerifiedField>, HardwareError> {
    parse_hardware_list(value, |item| match item {
        "amount" => Some(VerifiedField::Amount),
        "recipient" => Some(VerifiedField::Recipient),
        "network" => Some(VerifiedField::Network),
        "fee" => Some(VerifiedField::Fee),
        "memo" => Some(VerifiedField::Memo),
        _ => None,
    })
}

fn parse_hardware_list<T: Copy + Ord>(
    value: &str,
    parse: impl Fn(&str) -> Option<T>,
) -> Result<Vec<T>, HardwareError> {
    if value.is_empty() {
        return Ok(Vec::new());
    }
    let values = value
        .split(',')
        .map(|item| parse(item).ok_or_else(HardwareError::state_corrupt))
        .collect::<Result<Vec<_>, _>>()?;
    if values.windows(2).any(|pair| pair[0] >= pair[1]) {
        return Err(HardwareError::state_corrupt());
    }
    Ok(values)
}

fn parse_decision_status(value: &str) -> Result<DecisionStatus, HardwareError> {
    match value {
        "DEVICE_DISCONNECTED" => Ok(DecisionStatus::DeviceDisconnected),
        "CAPABILITY_MISSING" => Ok(DecisionStatus::CapabilityMissing),
        "PROTOCOL_INCOMPATIBLE" => Ok(DecisionStatus::ProtocolIncompatible),
        "READY" => Ok(DecisionStatus::Ready),
        _ => Err(HardwareError::state_corrupt()),
    }
}

fn parse_hardware_privacy(value: &str) -> Result<HardwarePrivacy, HardwareError> {
    match value {
        "unavailable" => Ok(HardwarePrivacy::Unavailable),
        "private" => Ok(HardwarePrivacy::Private),
        "transparent_not_private" => Ok(HardwarePrivacy::TransparentNotPrivate),
        _ => Err(HardwareError::state_corrupt()),
    }
}

fn parse_hardware_route(value: &str) -> Result<Option<HardwareRoute>, HardwareError> {
    match value {
        "none" => Ok(None),
        "keystone_pczt_v2" => Ok(Some(HardwareRoute::KeystonePcztV2)),
        _ => Err(HardwareError::state_corrupt()),
    }
}

fn hardware_record_check(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(HARDWARE_RECORD_CHECK_DOMAIN);
    hasher.update((bytes.len() as u64).to_be_bytes());
    hasher.update(bytes);
    let digest = hasher.finalize();
    let mut encoded = String::with_capacity(digest.len() * 2);
    const HEX: &[u8; 16] = b"0123456789abcdef";
    for byte in digest {
        encoded.push(char::from(HEX[usize::from(byte >> 4)]));
        encoded.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    encoded
}

fn mutate_hardware_record_bytes(
    bytes: Vec<u8>,
    mutation: HardwareRecordMutation,
) -> Result<Vec<u8>, HardwareError> {
    if mutation == HardwareRecordMutation::PartialWrite {
        return Ok(bytes[..bytes.len() / 2].to_vec());
    }
    let mut record = String::from_utf8(bytes).map_err(|_| HardwareError::state_corrupt())?;
    match mutation {
        HardwareRecordMutation::UnknownField => record.push_str("unknown_field=1\n"),
        HardwareRecordMutation::DuplicateVerifiedField => {
            replace_hardware_record_value(
                &mut record,
                "verified_fields=amount,recipient,network,fee,memo",
                "verified_fields=amount,amount,recipient,network,fee,memo",
            )?;
        }
        HardwareRecordMutation::InvalidBoolean => {
            replace_hardware_record_value(&mut record, "can_view=1", "can_view=2")?;
        }
        HardwareRecordMutation::OutOfRangeTransactionVersion => {
            replace_hardware_record_value(
                &mut record,
                "transaction_version=6",
                "transaction_version=18446744073709551616",
            )?;
        }
        HardwareRecordMutation::InvalidFingerprintDigest => {
            let start = record
                .find("fingerprint_digest=")
                .ok_or_else(HardwareError::state_corrupt)?;
            let value_start = start + "fingerprint_digest=".len();
            let value_end = record[value_start..]
                .find('\n')
                .map(|offset| value_start + offset)
                .ok_or_else(HardwareError::state_corrupt)?;
            record.replace_range(value_start..value_end, &"g".repeat(64));
        }
        HardwareRecordMutation::UnknownStatus => {
            replace_hardware_record_value(&mut record, "status=READY", "status=UNKNOWN")?;
        }
        HardwareRecordMutation::SchemaRevisionDrift => {
            replace_hardware_record_value(&mut record, "schema_revision=1", "schema_revision=2")?;
        }
        HardwareRecordMutation::TableRevisionDrift => {
            let start = record
                .find("table_revision=")
                .ok_or_else(HardwareError::state_corrupt)?;
            let value_start = start + "table_revision=".len();
            let value_end = record[value_start..]
                .find('\n')
                .map(|offset| value_start + offset)
                .ok_or_else(HardwareError::state_corrupt)?;
            record.replace_range(value_start..value_end, "wal008-drift-table-r1");
        }
        HardwareRecordMutation::ConsensusDrift => {
            replace_hardware_record_value(
                &mut record,
                "consensus_branch=37a5165b",
                "consensus_branch=37a5165a",
            )?;
        }
        HardwareRecordMutation::PartialWrite | HardwareRecordMutation::Rollback => {
            return Err(HardwareError::state_corrupt());
        }
    }
    Ok(record.into_bytes())
}

fn replace_hardware_record_value(
    record: &mut String,
    old: &str,
    new: &str,
) -> Result<(), HardwareError> {
    let Some(position) = record.find(old) else {
        return Err(HardwareError::state_corrupt());
    };
    record.replace_range(position..position + old.len(), new);
    Ok(())
}

fn migrate_extension(
    root: &StateRoot,
    paths: &AccountPaths,
    account_id: &AccountId,
    network: Network,
    fault: Option<StoreFault>,
) -> Result<(), ZecError> {
    validate_account_paths(root, paths)?;
    let mut connection = open_connection(root, &paths.wallet)?;
    configure_full_synchronous(&connection)?;
    let transaction = connection
        .transaction_with_behavior(TransactionBehavior::Immediate)
        .map_err(|_| ZecError::state_corrupt())?;
    validate_upstream_schema(&transaction)?;
    if validate_extension_and_binding_with_connection(&transaction, account_id, network)?
        != BrokerSchemaVersion::V0
    {
        return Err(ZecError::state_corrupt());
    }
    let sequence =
        read_receiver_state_with_connection(&transaction, account_id)?.issued_at_sequence;
    transaction
        .execute_batch(STORE_SCHEMA)
        .map_err(|_| ZecError::state_corrupt())?;
    let inserted = transaction
        .execute(
            "INSERT INTO ext_bitbook_store_state
             (account_id, scan_tip, checkpoint_receiver_sequence) VALUES (?1, NULL, ?2)",
            params![
                account_id.as_str(),
                i64::try_from(sequence).map_err(|_| ZecError::limit())?
            ],
        )
        .map_err(|_| ZecError::state_corrupt())?;
    if inserted != 1 || fault == Some(StoreFault::MigrationWrite) {
        return Err(ZecError::state_corrupt());
    }
    if recognize_extension_schema(&transaction)? != BrokerSchemaVersion::V1 {
        return Err(ZecError::state_corrupt());
    }
    if fault == Some(StoreFault::MigrationSync) {
        return Err(ZecError::state_corrupt());
    }
    if fault == Some(StoreFault::MigrationCommit) {
        return Err(ZecError::state_corrupt());
    }
    transaction.commit().map_err(|_| ZecError::state_corrupt())
}

fn configure_full_synchronous(connection: &Connection) -> Result<(), ZecError> {
    connection
        .execute_batch("PRAGMA synchronous = FULL;")
        .map_err(|_| ZecError::state_corrupt())?;
    let synchronous = connection
        .query_row("PRAGMA synchronous", [], |row| row.get::<_, i64>(0))
        .map_err(|_| ZecError::state_corrupt())?;
    if synchronous == 2 {
        Ok(())
    } else {
        Err(ZecError::state_corrupt())
    }
}

fn sync_bootstrap_entries(root: &StateRoot, paths: &AccountPaths) -> Result<(), ZecError> {
    validate_account_paths(root, paths)?;
    for path in [&paths.wallet, &paths.compact] {
        OpenOptions::new()
            .read(true)
            .open(path)
            .and_then(|file| file.sync_all())
            .map_err(|_| ZecError::state_corrupt())?;
    }
    let network_directory = paths
        .directory
        .parent()
        .ok_or_else(ZecError::state_corrupt)?;
    for path in [&paths.directory, network_directory, root.path()] {
        fs::File::open(path)
            .and_then(|directory| directory.sync_all())
            .map_err(|_| ZecError::state_corrupt())?;
    }
    Ok(())
}

fn inspect_sqlite(connection: &Connection) -> Result<SqliteInspectionData, ZecError> {
    const MAX_TABLES: usize = 512;
    const MAX_ROWS: usize = 16_384;
    const MAX_VALUE_BYTES: usize = 1024 * 1024;
    const MAX_DECODED_CELLS: usize = 262_144;
    const MAX_DECODED_BYTES: usize = 16 * 1024 * 1024;

    let mut statement = connection
        .prepare("SELECT name FROM sqlite_schema WHERE type = 'table' ORDER BY name LIMIT 513")
        .map_err(|_| ZecError::state_corrupt())?;
    let tables = statement
        .query_map([], |row| row.get::<_, String>(0))
        .map_err(|_| ZecError::state_corrupt())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| ZecError::state_corrupt())?;
    if tables.len() > MAX_TABLES {
        return Err(ZecError::limit());
    }

    let mut columns = Vec::new();
    let mut decoded_rows = 0usize;
    let mut decoded_cells = 0usize;
    let mut decoded_bytes = 0usize;
    let mut value_kinds = BTreeSet::new();
    let mut decoded_values = Vec::new();
    for table in &tables {
        columns.extend(table_column_names(connection, table)?);
        let quoted = table.replace('"', "\"\"");
        let sql = format!("SELECT * FROM \"{quoted}\" LIMIT {}", MAX_ROWS + 1);
        let mut statement = connection
            .prepare(&sql)
            .map_err(|_| ZecError::state_corrupt())?;
        let column_count = statement.column_count();
        let mut rows = statement.query([]).map_err(|_| ZecError::state_corrupt())?;
        while let Some(row) = rows.next().map_err(|_| ZecError::state_corrupt())? {
            decoded_rows = decoded_rows.checked_add(1).ok_or_else(ZecError::limit)?;
            if decoded_rows > MAX_ROWS {
                return Err(ZecError::limit());
            }
            for index in 0..column_count {
                decoded_cells = decoded_cells.checked_add(1).ok_or_else(ZecError::limit)?;
                if decoded_cells > MAX_DECODED_CELLS {
                    return Err(ZecError::limit());
                }
                let value = row.get_ref(index).map_err(|_| ZecError::state_corrupt())?;
                let (kind, bytes) = match value {
                    ValueRef::Null => ("null", Vec::new()),
                    ValueRef::Integer(value) => {
                        decoded_bytes =
                            checked_decoded_bytes(decoded_bytes, 20, MAX_DECODED_BYTES)?;
                        ("integer", value.to_string().into_bytes())
                    }
                    ValueRef::Real(value) => {
                        decoded_bytes =
                            checked_decoded_bytes(decoded_bytes, 32, MAX_DECODED_BYTES)?;
                        ("real", value.to_string().into_bytes())
                    }
                    ValueRef::Text(value) => {
                        if value.len() > MAX_VALUE_BYTES {
                            return Err(ZecError::limit());
                        }
                        decoded_bytes =
                            checked_decoded_bytes(decoded_bytes, value.len(), MAX_DECODED_BYTES)?;
                        ("text", value.to_vec())
                    }
                    ValueRef::Blob(value) => {
                        if value.len() > MAX_VALUE_BYTES {
                            return Err(ZecError::limit());
                        }
                        decoded_bytes =
                            checked_decoded_bytes(decoded_bytes, value.len(), MAX_DECODED_BYTES)?;
                        ("blob", value.to_vec())
                    }
                };
                value_kinds.insert(kind);
                decoded_values.push(bytes);
            }
        }
    }
    Ok(SqliteInspectionData {
        tables,
        columns,
        decoded_rows,
        value_kinds: value_kinds.into_iter().collect(),
        decoded_values,
    })
}

fn checked_decoded_bytes(total: usize, amount: usize, limit: usize) -> Result<usize, ZecError> {
    let next = total.checked_add(amount).ok_or_else(ZecError::limit)?;
    if next > limit {
        Err(ZecError::limit())
    } else {
        Ok(next)
    }
}

fn table_column_names(connection: &Connection, table: &str) -> Result<Vec<String>, ZecError> {
    let quoted = table.replace('"', "\"\"");
    let sql = format!("PRAGMA table_info(\"{quoted}\")");
    let mut statement = connection
        .prepare(&sql)
        .map_err(|_| ZecError::state_corrupt())?;
    let rows = statement
        .query_map([], |row| row.get::<_, String>(1))
        .map_err(|_| ZecError::state_corrupt())?;
    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|_| ZecError::state_corrupt())
}

#[derive(Clone)]
pub(crate) struct AccountPaths {
    pub(crate) directory: PathBuf,
    pub(crate) wallet: PathBuf,
    pub(crate) compact: PathBuf,
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
    let owner = validate_state_root(root)?;
    let network_directory = paths
        .directory
        .parent()
        .ok_or_else(ZecError::state_corrupt)?;
    create_secure_directory(root, network_directory, owner)?;
    create_secure_directory(root, &paths.directory, owner)?;
    create_secure_file(root, &paths.wallet, owner)?;
    create_secure_file(root, &paths.compact, owner)?;
    Ok(())
}

pub(crate) fn validate_account_paths(
    root_state: &StateRoot,
    paths: &AccountPaths,
) -> Result<(), ZecError> {
    let network_directory = paths
        .directory
        .parent()
        .ok_or_else(ZecError::state_corrupt)?;
    let root = network_directory
        .parent()
        .ok_or_else(ZecError::state_corrupt)?;
    if root != root_state.path() {
        return Err(ZecError::state_corrupt());
    }
    let owner = validate_state_root(root_state)?;
    validate_directory(root_state, network_directory, owner)?;
    validate_directory(root_state, &paths.directory, owner)?;
    validate_regular_file(root_state, &paths.wallet, owner)?;
    validate_regular_file(root_state, &paths.compact, owner)?;
    Ok(())
}

fn create_secure_directory(root: &StateRoot, path: &Path, owner: u32) -> Result<(), ZecError> {
    match fs::symlink_metadata(path) {
        Ok(_) => validate_directory(root, path, owner),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            let mut builder = fs::DirBuilder::new();
            builder.mode(0o700);
            builder
                .create(path)
                .map_err(|_| ZecError::state_corrupt())?;
            validate_directory(root, path, owner)
        }
        Err(_) => Err(ZecError::state_corrupt()),
    }
}

fn validate_state_root(root: &StateRoot) -> Result<u32, ZecError> {
    let metadata = fs::symlink_metadata(root.path()).map_err(|_| ZecError::state_corrupt())?;
    let facts = root.entry_facts(root.path(), &metadata);
    if facts.identity != EntryIdentity::Directory || facts.mode != 0o700 {
        return Err(ZecError::state_corrupt());
    }
    Ok(facts.uid)
}

fn validate_directory(root: &StateRoot, path: &Path, owner: u32) -> Result<(), ZecError> {
    let metadata = fs::symlink_metadata(path).map_err(|_| ZecError::state_corrupt())?;
    let facts = root.entry_facts(path, &metadata);
    if facts.identity != EntryIdentity::Directory || facts.mode != 0o700 || facts.uid != owner {
        return Err(ZecError::state_corrupt());
    }
    Ok(())
}

fn create_secure_file(root: &StateRoot, path: &Path, owner: u32) -> Result<(), ZecError> {
    match fs::symlink_metadata(path) {
        Ok(_) => Err(ZecError::state_corrupt()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            OpenOptions::new()
                .write(true)
                .create_new(true)
                .mode(0o600)
                .open(path)
                .map_err(|_| ZecError::state_corrupt())?;
            validate_regular_file(root, path, owner)
        }
        Err(_) => Err(ZecError::state_corrupt()),
    }
}

fn validate_regular_file(root: &StateRoot, path: &Path, owner: u32) -> Result<(), ZecError> {
    let metadata = fs::symlink_metadata(path).map_err(|_| ZecError::state_corrupt())?;
    let facts = root.entry_facts(path, &metadata);
    if facts.identity != EntryIdentity::Regular || facts.mode != 0o600 || facts.uid != owner {
        return Err(ZecError::state_corrupt());
    }
    Ok(())
}

fn initialize_official_wallet(
    root: &StateRoot,
    path: &Path,
    network: Network,
) -> Result<(), ZecError> {
    let owner = validate_state_root(root)?;
    validate_regular_file(root, path, owner)?;
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

fn initialize_official_cache(root: &StateRoot, path: &Path) -> Result<(), ZecError> {
    let owner = validate_state_root(root)?;
    validate_regular_file(root, path, owner)?;
    let cache = BlockDb::for_path(path).map_err(|_| ZecError::state_corrupt())?;
    init_cache_database(&cache).map_err(|_| ZecError::state_corrupt())
}

fn initialize_extension(
    root: &StateRoot,
    path: &Path,
    account_id: &AccountId,
    network: Network,
    ufvk: &str,
) -> Result<(), ZecError> {
    let mut connection = open_connection(root, path)?;
    validate_upstream_schema(&connection)?;
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
        transaction
            .execute_batch(STORE_SCHEMA)
            .map_err(|_| ZecError::state_corrupt())?;
    }
    if recognize_extension_schema(&transaction)? != BrokerSchemaVersion::V1 {
        return Err(ZecError::state_corrupt());
    }

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
    let inserted_store = transaction
        .execute(
            "INSERT INTO ext_bitbook_store_state
             (account_id, scan_tip, checkpoint_receiver_sequence) VALUES (?1, NULL, 0)",
            [account_id.as_str()],
        )
        .map_err(|_| ZecError::state_corrupt())?;
    if inserted_account != 1
        || inserted_receiver != 1
        || inserted_sequence != 1
        || inserted_store != 1
    {
        return Err(ZecError::state_corrupt());
    }
    transaction.commit().map_err(|_| ZecError::state_corrupt())
}

fn validate_extension_and_binding_with_connection(
    connection: &Connection,
    account_id: &AccountId,
    network: Network,
) -> Result<BrokerSchemaVersion, ZecError> {
    let version = recognize_extension_schema(connection)?;
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
    if version.has_store_state() {
        let store_count = connection
            .query_row("SELECT COUNT(*) FROM ext_bitbook_store_state", [], |row| {
                row.get::<_, i64>(0)
            })
            .map_err(|_| ZecError::state_corrupt())?;
        if store_count != 1 {
            return Err(ZecError::state_corrupt());
        }
    }
    if version == BrokerSchemaVersion::V2 {
        let hardware_counts = connection
            .query_row(
                "SELECT COUNT(*),
                        COALESCE(SUM(CASE WHEN singleton = 1 AND typeof(singleton) = 'integer'
                                          THEN 1 ELSE 0 END), 0)
                 FROM ext_bitbook_hardware_decision",
                [],
                |row| Ok((row.get::<_, i64>(0)?, row.get::<_, i64>(1)?)),
            )
            .map_err(|_| ZecError::state_corrupt())?;
        if hardware_counts != (1, 1) {
            return Err(ZecError::state_corrupt());
        }
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
    address::derive_orchard_receiver(network, &ufvk, 0).map_err(|_| ZecError::state_corrupt())?;
    let receiver_state = read_receiver_state_with_connection(connection, account_id)?;
    if version.has_store_state() {
        validate_store_state_with_connection(
            connection,
            account_id,
            receiver_state.issued_at_sequence,
        )?;
    }
    Ok(version)
}

pub(crate) fn validate_scan_binding(
    connection: &Connection,
    account_id: &AccountId,
    network: Network,
) -> Result<(), ZecError> {
    if validate_extension_and_binding_with_connection(connection, account_id, network)?
        .has_store_state()
    {
        Ok(())
    } else {
        Err(ZecError::state_corrupt())
    }
}

fn validate_store_state_with_connection(
    connection: &Connection,
    account_id: &AccountId,
    issued_receiver_sequence: u64,
) -> Result<(), ZecError> {
    let (scan_tip, checkpoint_sequence) = connection
        .query_row(
            "SELECT scan_tip, checkpoint_receiver_sequence
             FROM ext_bitbook_store_state WHERE account_id = ?1",
            [account_id.as_str()],
            |row| Ok((row.get::<_, Option<i64>>(0)?, row.get::<_, i64>(1)?)),
        )
        .map_err(|_| ZecError::state_corrupt())?;
    let checkpoint_sequence =
        u64::try_from(checkpoint_sequence).map_err(|_| ZecError::state_corrupt())?;
    if scan_tip.is_some_and(|value| value < 0) || checkpoint_sequence > issued_receiver_sequence {
        return Err(ZecError::state_corrupt());
    }
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum BrokerSchemaVersion {
    V0,
    V1,
    V2,
}

impl BrokerSchemaVersion {
    fn as_str(self) -> &'static str {
        match self {
            Self::V0 => "0",
            Self::V1 => "1",
            Self::V2 => "2",
        }
    }

    fn has_store_state(self) -> bool {
        matches!(self, Self::V1 | Self::V2)
    }
}

fn expected_extension_objects(version: BrokerSchemaVersion) -> Vec<SchemaObject> {
    let mut objects = vec![
        expected_autoindex(ACCOUNT_TABLE),
        expected_autoindex(RECEIVER_TABLE),
        expected_autoindex(SEQUENCE_TABLE),
    ];
    if version.has_store_state() {
        objects.push(expected_autoindex(STORE_TABLE));
    }
    objects.push(expected_table(ACCOUNT_TABLE, ACCOUNT_SCHEMA));
    if version == BrokerSchemaVersion::V2 {
        objects.push(expected_table(
            HARDWARE_DECISION_TABLE,
            HARDWARE_DECISION_SCHEMA,
        ));
    }
    objects.extend([
        expected_table(RECEIVER_TABLE, RECEIVER_SCHEMA),
        expected_table(SEQUENCE_TABLE, SEQUENCE_SCHEMA),
    ]);
    if version.has_store_state() {
        objects.push(expected_table(STORE_TABLE, STORE_SCHEMA));
    }
    objects
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

fn recognize_extension_schema(connection: &Connection) -> Result<BrokerSchemaVersion, ZecError> {
    let objects = extension_objects(connection)?;
    let version = if objects == expected_extension_objects(BrokerSchemaVersion::V0) {
        BrokerSchemaVersion::V0
    } else if objects == expected_extension_objects(BrokerSchemaVersion::V1) {
        BrokerSchemaVersion::V1
    } else if objects == expected_extension_objects(BrokerSchemaVersion::V2) {
        BrokerSchemaVersion::V2
    } else {
        return Err(ZecError::state_corrupt());
    };
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
    )?;
    if version.has_store_state() {
        validate_columns(
            connection,
            STORE_TABLE,
            &[
                ("account_id", "TEXT", true, true),
                ("scan_tip", "INTEGER", false, false),
                ("checkpoint_receiver_sequence", "INTEGER", true, false),
            ],
        )?;
    }
    if version == BrokerSchemaVersion::V2 {
        validate_columns(
            connection,
            HARDWARE_DECISION_TABLE,
            &[
                ("singleton", "INTEGER", true, true),
                ("record", "BLOB", true, false),
                ("record_generation", "INTEGER", true, false),
                ("generation_check", "TEXT", true, false),
            ],
        )?;
    }
    Ok(version)
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
    root: &StateRoot,
    paths: &AccountPaths,
    account_id: &AccountId,
    network: Network,
    fault: Option<AddressFaultPort>,
) -> Result<FreshReceiverV1, ZecError> {
    validate_account_paths(root, paths)?;
    let mut connection = open_connection(root, &paths.wallet)?;
    let transaction = connection
        .transaction_with_behavior(TransactionBehavior::Immediate)
        .map_err(|_| ZecError::state_corrupt())?;
    if !validate_extension_and_binding_with_connection(&transaction, account_id, network)?
        .has_store_state()
    {
        return Err(ZecError::state_corrupt());
    }
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
    root: &StateRoot,
    paths: &AccountPaths,
    account_id: &AccountId,
    network: Network,
) -> Result<ReceiverState, ZecError> {
    validate_account_paths(root, paths)?;
    let connection = open_read_only_connection(root, &paths.wallet)?;
    if !validate_extension_and_binding_with_connection(&connection, account_id, network)?
        .has_store_state()
    {
        return Err(ZecError::state_corrupt());
    }
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

fn open_connection(root: &StateRoot, path: &Path) -> Result<Connection, ZecError> {
    let owner = validate_state_root(root)?;
    validate_regular_file(root, path, owner)?;
    let connection = Connection::open(path).map_err(|_| ZecError::state_corrupt())?;
    connection
        .busy_timeout(Duration::from_secs(5))
        .map_err(|_| ZecError::state_corrupt())?;
    Ok(connection)
}

pub(crate) fn open_read_only_connection(
    root: &StateRoot,
    path: &Path,
) -> Result<Connection, ZecError> {
    let owner = validate_state_root(root)?;
    validate_regular_file(root, path, owner)?;
    validate_sqlite_header(path)?;
    let connection = Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY)
        .map_err(|_| ZecError::state_corrupt())?;
    connection
        .busy_timeout(Duration::from_secs(5))
        .map_err(|_| ZecError::state_corrupt())?;
    validate_sqlite_database(&connection)?;
    Ok(connection)
}

fn open_read_write_no_create_connection(
    root: &StateRoot,
    path: &Path,
) -> Result<Connection, ZecError> {
    let owner = validate_state_root(root)?;
    validate_regular_file(root, path, owner)?;
    validate_sqlite_header(path)?;
    let connection = Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_WRITE)
        .map_err(|_| ZecError::state_corrupt())?;
    connection
        .busy_timeout(Duration::from_secs(5))
        .map_err(|_| ZecError::state_corrupt())?;
    validate_sqlite_database(&connection)?;
    Ok(connection)
}

fn validate_sqlite_header(path: &Path) -> Result<(), ZecError> {
    let mut file = fs::File::open(path).map_err(|_| ZecError::state_corrupt())?;
    let mut header = [0; 100];
    file.read_exact(&mut header)
        .map_err(|_| ZecError::state_corrupt())?;
    if &header[..16] != b"SQLite format 3\0" {
        return Err(ZecError::state_corrupt());
    }
    let encoded_page_size = u16::from_be_bytes([header[16], header[17]]);
    let page_size = if encoded_page_size == 1 {
        65_536
    } else {
        u32::from(encoded_page_size)
    };
    if !(512..=65_536).contains(&page_size) || !page_size.is_power_of_two() {
        return Err(ZecError::state_corrupt());
    }
    Ok(())
}

fn validate_sqlite_database(connection: &Connection) -> Result<(), ZecError> {
    let result = connection
        .query_row("PRAGMA quick_check", [], |row| row.get::<_, String>(0))
        .map_err(|_| ZecError::state_corrupt())?;
    if result == "ok" {
        Ok(())
    } else {
        Err(ZecError::state_corrupt())
    }
}

fn validate_upstream_schema(connection: &Connection) -> Result<(), ZecError> {
    for table in ["accounts", "addresses", "scan_queue", "schemer_migrations"] {
        let count = connection
            .query_row(
                "SELECT COUNT(*) FROM sqlite_schema WHERE type = 'table' AND name = ?1",
                [table],
                |row| row.get::<_, i64>(0),
            )
            .map_err(|_| ZecError::state_corrupt())?;
        if count != 1 {
            return Err(ZecError::state_corrupt());
        }
    }
    let account_columns = table_column_names(connection, "accounts")?;
    if !account_columns.iter().any(|column| column == "ufvk") {
        return Err(ZecError::state_corrupt());
    }
    let migration_inventory = connection
        .query_row(
            "SELECT COUNT(*), COUNT(DISTINCT id), MIN(length(id)), MAX(length(id))
             FROM schemer_migrations",
            [],
            |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, i64>(1)?,
                    row.get::<_, Option<i64>>(2)?,
                    row.get::<_, Option<i64>>(3)?,
                ))
            },
        )
        .map_err(|_| ZecError::state_corrupt())?;
    if migration_inventory != (71, 71, Some(16), Some(16)) {
        return Err(ZecError::state_corrupt());
    }
    for migration_id in CURRENT_LEAF_MIGRATIONS {
        let count = connection
            .query_row(
                "SELECT COUNT(*) FROM schemer_migrations WHERE id = ?1",
                params![&migration_id.as_bytes()[..]],
                |row| row.get::<_, i64>(0),
            )
            .map_err(|_| ZecError::state_corrupt())?;
        if count != 1 {
            return Err(ZecError::state_corrupt());
        }
    }
    Ok(())
}

pub(crate) fn validate_cache_schema(connection: &Connection) -> Result<(), ZecError> {
    const MAX_SCHEMA_IDENTIFIER_BYTES: i64 = 64;
    const MAX_SCHEMA_SQL_BYTES: i64 = 512;

    let mut statement = connection
        .prepare(
            "SELECT length(CAST(type AS BLOB)), length(CAST(name AS BLOB)),
                    length(CAST(tbl_name AS BLOB)),
                    length(CAST(COALESCE(sql, '') AS BLOB)),
                    type, name, tbl_name, sql
             FROM sqlite_schema
             WHERE name NOT GLOB 'sqlite_*'
             ORDER BY type, name, tbl_name, COALESCE(sql, '')
             LIMIT 2",
        )
        .map_err(|_| ZecError::state_corrupt())?;
    let rows = statement
        .query_map([], |row| {
            let lengths = (
                row.get::<_, i64>(0)?,
                row.get::<_, i64>(1)?,
                row.get::<_, i64>(2)?,
                row.get::<_, i64>(3)?,
            );
            if lengths.0 > MAX_SCHEMA_IDENTIFIER_BYTES
                || lengths.1 > MAX_SCHEMA_IDENTIFIER_BYTES
                || lengths.2 > MAX_SCHEMA_IDENTIFIER_BYTES
                || lengths.3 > MAX_SCHEMA_SQL_BYTES
            {
                return Err(rusqlite::Error::InvalidQuery);
            }
            Ok(SchemaObject {
                object_type: row.get(4)?,
                name: row.get(5)?,
                table_name: row.get(6)?,
                sql: row
                    .get::<_, Option<String>>(7)?
                    .map(|sql| normalize_sql(&sql)),
            })
        })
        .map_err(|_| ZecError::state_corrupt())?;
    let objects = rows
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| ZecError::state_corrupt())?;
    let expected = vec![expected_table(
        "compactblocks",
        "CREATE TABLE compactblocks (
            height INTEGER PRIMARY KEY,
            data BLOB NOT NULL
        )",
    )];
    if objects != expected {
        return Err(ZecError::state_corrupt());
    }
    validate_columns(
        connection,
        "compactblocks",
        &[
            ("height", "INTEGER", false, true),
            ("data", "BLOB", true, false),
        ],
    )
}

fn preflight_store(
    root: &StateRoot,
    paths: &AccountPaths,
    account_id: &AccountId,
    network: Network,
) -> Result<BrokerSchemaVersion, ZecError> {
    validate_account_paths(root, paths)?;
    let connection = open_read_only_connection(root, &paths.wallet)?;
    validate_upstream_schema(&connection)?;
    let version = validate_extension_and_binding_with_connection(&connection, account_id, network)?;
    let cache = open_read_only_connection(root, &paths.compact)?;
    validate_cache_schema(&cache)?;
    Ok(version)
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

enum PcztRollback {
    Prepared,
    Failed,
}

impl From<rusqlite::Error> for PcztRollback {
    fn from(_: rusqlite::Error) -> Self {
        Self::Failed
    }
}

#[allow(clippy::too_many_arguments)]
fn build_prepared_for<P: Parameters + Clone + Send + 'static>(
    mut connection: Connection,
    params: P,
    network: Network,
    receiver: &str,
    amount: u64,
    memo: &str,
    request_id: &str,
    intent_hash: &str,
) -> Result<PreparedBuild, ZecError> {
    let decoded = Address::decode(&params, receiver).ok_or_else(ZecError::schema)?;
    let destination = decoded.to_zcash_address(&params);
    let amount_value = Zatoshis::from_u64(amount).map_err(|_| ZecError::limit())?;
    let memo = if memo.is_empty() {
        None
    } else {
        Some(MemoBytes::from_bytes(memo.as_bytes()).map_err(|_| ZecError::schema())?)
    };
    let payment = Payment::new(
        destination,
        Some(amount_value),
        memo,
        None,
        None,
        Vec::new(),
    )
    .map_err(|_| ZecError::schema())?;
    let request = TransactionRequest::new(vec![payment]).map_err(|_| ZecError::schema())?;
    let input_selector = GreedyInputSelector::new();
    let change_strategy = SingleOutputChangeStrategy::new(
        StandardFeeRule::Zip317,
        None,
        ShieldedPool::Ironwood,
        DustOutputPolicy::default(),
    );
    let mut wallet = WalletDb::from_connection(&mut connection, params.clone(), SystemClock, OsRng);
    let account_ids = wallet
        .get_account_ids()
        .map_err(|_| ZecError::state_corrupt())?;
    if account_ids.len() != 1 {
        return Err(ZecError::state_corrupt());
    }
    let proposal = propose_transfer::<_, _, _, _, std::convert::Infallible>(
        &mut wallet,
        &params,
        account_ids[0],
        &input_selector,
        &change_strategy,
        request,
        ConfirmationsPolicy::MIN,
        &SpendPolicy::shielded_pools([ShieldedPool::Ironwood]),
        None,
        Some(TxVersion::V6),
    )
    .map_err(|_| ZecError::insufficient_funds())?;
    let mut steps = proposal.steps().iter();
    let step = steps.next().ok_or_else(ZecError::internal)?;
    if steps.next().is_some()
        || step.payment_pools().get(&0) != Some(&PoolType::IRONWOOD)
        || step.input_count_in_pool(PoolType::IRONWOOD) != 1
        || step.output_count_in_pool(PoolType::IRONWOOD) != 1
        || step.change_count_in_pool(PoolType::IRONWOOD) != 1
        || step.input_count_in_pool(PoolType::TRANSPARENT) != 0
        || step.input_count_in_pool(PoolType::SAPLING) != 0
        || step.input_count_in_pool(PoolType::ORCHARD) != 0
    {
        return Err(ZecError::protocol_incompatible());
    }
    let fee = u64::from(step.balance().fee_required());
    let mut prepared_pczt = None;
    let rollback: Result<(), PcztRollback> = wallet.transactionally(|transactional_wallet| {
        let pczt = create_pczt_from_proposal::<
            _,
            _,
            std::convert::Infallible,
            _,
            std::convert::Infallible,
            _,
        >(
            transactional_wallet,
            &params,
            account_ids[0],
            OvkPolicy::Sender,
            &proposal,
            None,
            BundlePadding::DEFAULT,
        )
        .map_err(|_| PcztRollback::Failed)?;
        prepared_pczt = Some(pczt);
        Err(PcztRollback::Prepared)
    });
    let pczt = match rollback {
        Err(PcztRollback::Prepared) => prepared_pczt.ok_or_else(ZecError::internal)?,
        Ok(()) | Err(PcztRollback::Failed) => return Err(ZecError::internal()),
    };
    let pczt = pczt::roles::redactor::Redactor::new(pczt)
        .redact_ironwood_with(|mut bundle| bundle.compact_resolvable_fields())
        .finish();
    let raw = SecretBytes::new(pczt.serialize().map_err(|_| ZecError::internal())?)
        .map_err(|_| ZecError::internal())?;
    let parsed = raw
        .expose(pczt::Pczt::parse)
        .map_err(|_| ZecError::internal())?;
    let global = parsed.global();
    let ironwood = parsed.ironwood();
    let orchard = parsed.orchard();
    let ironwood_action_count = ironwood.actions().len();
    // The IO Finalizer signs the padding spend; the unsigned action is the real spend that
    // still requires wallet authorization.
    let authorization_required_inputs = ironwood
        .actions()
        .iter()
        .filter(|action| action.spend().spend_auth_sig().is_none())
        .count();
    let finalized_padding_inputs = ironwood
        .actions()
        .iter()
        .filter(|action| action.spend().spend_auth_sig().is_some())
        .count();
    let real_outputs = ironwood
        .actions()
        .iter()
        .filter(|action| action.output().value().is_some_and(|value| value > 0))
        .count();
    let payment_output = ironwood.actions().iter().find(|action| {
        action.output().value() == &Some(amount)
            && action.output().user_address().as_deref() == Some(receiver)
    });
    let has_orchard_real_spends = orchard
        .actions()
        .iter()
        .any(|action| action.spend().witness().is_some());
    let has_orchard_output_bundle = orchard.actions().iter().any(|action| {
        action.output().value().is_some_and(|value| value > 0)
            || action.output().user_address().is_some()
    });
    let is_v6 = *global.tx_version() == zcash_protocol::constants::V6_TX_VERSION;
    let is_expected_branch = *global.consensus_branch_id() == 0x37a5_165b;
    let transparent_empty =
        parsed.transparent().inputs().is_empty() && parsed.transparent().outputs().is_empty();
    let sapling_empty =
        parsed.sapling().spends().is_empty() && parsed.sapling().outputs().is_empty();
    let ironwood_proof = ironwood.zkproof().is_some();
    let orchard_proof = orchard.zkproof().is_some();
    let payment_output = payment_output.ok_or_else(ZecError::protocol_incompatible)?;
    let memo_bytes = match payment_output.output().enc_ciphertext() {
        pczt::orchard::EncCiphertext::MemoPlaintext(memo) => memo.as_stripped_bytes(),
        pczt::orchard::EncCiphertext::Encrypted(_) => {
            return Err(ZecError::protocol_incompatible());
        }
    };
    let has_proofs = ironwood_proof || orchard_proof;
    if !is_v6
        || !is_expected_branch
        || ironwood_action_count != 2
        || authorization_required_inputs != 1
        || finalized_padding_inputs != 1
        || real_outputs != 2
        || !transparent_empty
        || !sapling_empty
        || has_orchard_real_spends
        || has_orchard_output_bundle
        || has_proofs
    {
        return Err(ZecError::protocol_incompatible());
    }
    Ok(PreparedBuild {
        raw,
        fee_zat: fee,
        inspection: PcztInspection {
            network: network.as_str().to_owned(),
            consensus_branch: *global.consensus_branch_id(),
            transaction_version: *global.tx_version(),
            destination: receiver.to_owned(),
            amount_zat: amount.to_string(),
            memo_sha256: sha256_hex(memo_bytes),
            fee_zat: fee.to_string(),
            ironwood_inputs: authorization_required_inputs,
            ironwood_outputs: real_outputs,
            has_transparent_bundle: false,
            has_sapling_bundle: false,
            has_orchard_output_bundle,
            has_signatures: false,
            has_proofs,
            finalized: false,
            extractable: false,
            spend_pool: "ironwood".to_owned(),
            legacy_input_value_zat: "0".to_owned(),
            intent_hash_binding: intent_hash.to_owned(),
            request_id_binding: request_id.to_owned(),
        },
    })
}

fn detect_network(root: &StateRoot, account_id: &AccountId) -> Result<Network, ZecError> {
    validate_state_root(root)?;
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
            validate_account_paths(root, &paths)?;
            let connection = open_read_only_connection(root, &paths.wallet)?;
            validate_upstream_schema(&connection)?;
            recognize_extension_schema(&connection)?;
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
            LocalNetwork::new(birthday, nu6_3, confirmation)
                .map(Network::Local)
                .map_err(|_| ZecError::state_corrupt())
        }
        (false, true) => {
            let paths = AccountPaths {
                wallet: testnet_directory.join("wallet.sqlite3"),
                compact: testnet_directory.join("compact.sqlite3"),
                directory: testnet_directory,
            };
            validate_account_paths(root, &paths)?;
            let connection = open_read_only_connection(root, &paths.wallet)?;
            validate_upstream_schema(&connection)?;
            recognize_extension_schema(&connection)?;
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
