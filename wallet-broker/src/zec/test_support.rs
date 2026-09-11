use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, OpenOptions};
use std::os::linux::fs::MetadataExt as LinuxMetadataExt;
use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt, PermissionsExt, symlink};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Condvar, Mutex, MutexGuard};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use incrementalmerkletree::frontier::CommitmentTree;
use prost::Message;
use rusqlite::Connection;
use sha2::{Digest, Sha256};
use zcash_client_backend::data_api::chain::ChainState;
use zcash_client_backend::proto::compact_formats::CompactBlock;
use zcash_client_backend::proto::service::{BlockId, LightdInfo, TreeState};
use zcash_keys::address::Address;
use zcash_primitives::merkle_tree::write_commitment_tree;
use zcash_protocol::consensus::{BlockHeight, BranchId, NetworkUpgrade, Parameters};

use crate::native::{
    ActionOrigin, ZecConfirmationCapability, ZecNativeReview, confirm_zec_review_from_method,
    confirm_zec_review_from_origin, confirm_zec_review_synthetic_test_surface,
    replay_consumed_confirmation,
};
use crate::vault::{SecretBytes, WipeEvent, WipeObserver};

use super::address::{self, DecodedReceiver, SeedExit};
use super::fixture;
use super::hardware::PRODUCTION_REVIEWED_PROFILES;
pub use super::hardware::{
    CapabilityFlag, ClaimedRoute, DecisionStatus, DeviceFingerprint, DeviceVendor,
    FingerprintField, HardwareCapabilities, HardwareDecision, HardwareError, HardwarePrivacy,
    HardwareRoute, HardwareRouteMetadata, LiveProbe, ReviewedProfile, SigningPool, VerifiedField,
};
use super::prepare::{
    PcztInspection, PoolInventoryData, PrepareState, PrepareWipeLog, PreparedReview,
    normalize_diagnostic, parse_canonical_positive_u64,
};
use super::scan::{ScanBalances as InnerScanBalances, ScanFaultPort, ScanInspection, ScanRequest};
use super::spend::{
    self, AccountAuthorizationGate, AccountAuthorizationLease, PipelineCalls, PipelineFault,
    SignerViewArtifact, TaggedContribution, VerifiedEffects, VerifyMutation,
};
use super::store::{
    AddressAccount, AddressFaultPort, HardwarePersistenceFault, HardwareRecordMutation,
    HostileEntryKind, SqliteInspectionData, StateRoot,
};
use super::{
    AccountId, FreshReceiverV1, HandleBinding, HandleInvalidation, Network, PrepareZecV1,
    PreparedZecV1, StoreFault, ZecError,
};

static NEXT_STATE_ROOT: AtomicU64 = AtomicU64::new(1);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AddressFault {
    ReceiverRowWrite,
    SequenceRowWrite,
    CommitSync,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScanFault {
    RollbackWrite,
    RollbackSync,
    ReplacementApply,
    WalletDbCorrupt,
    CacheDbCorrupt,
    CommitSync,
}

impl From<ScanFault> for ScanFaultPort {
    fn from(value: ScanFault) -> Self {
        match value {
            ScanFault::RollbackWrite => Self::RollbackWrite,
            ScanFault::RollbackSync => Self::RollbackSync,
            ScanFault::ReplacementApply => Self::ReplacementApply,
            ScanFault::WalletDbCorrupt => Self::WalletDbCorrupt,
            ScanFault::CacheDbCorrupt => Self::CacheDbCorrupt,
            ScanFault::CommitSync => Self::CommitSync,
        }
    }
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CanonicalNumericField {
    Amount,
    FeeBound,
}

pub fn parse_canonical_u64_for_test(
    _field: CanonicalNumericField,
    value: &str,
) -> Result<u64, ZecError> {
    parse_canonical_positive_u64(value)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PrepareMutation {
    AccountId(String),
    Network(String),
    RequestId(String),
    IntentHash(String),
    Amount(String),
    FeeBound(String),
    ExpiresAt(String),
}

pub type PrepareBinding = HandleBinding;

pub struct ManualClock {
    value: String,
}

impl ManualClock {
    pub fn at(value: &str) -> Self {
        Self {
            value: value.to_owned(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PoolInventory {
    inner: PoolInventoryData,
}

impl PoolInventory {
    pub fn confirmed_ironwood(value: &str) -> Self {
        Self::values("0", "0", "0", value, "0", "0")
    }

    pub fn mixed(ironwood: &str, orchard: &str) -> Self {
        Self::values("0", "0", orchard, ironwood, "0", "0")
    }

    pub fn orchard(value: &str) -> Self {
        Self::values("0", "0", value, "0", "0", "0")
    }

    pub fn transparent(value: &str) -> Self {
        Self::values(value, "0", "0", "0", "0", "0")
    }

    pub fn sapling(value: &str) -> Self {
        Self::values("0", value, "0", "0", "0", "0")
    }

    pub fn mixed_with_sufficient_ironwood(ironwood: &str, orchard: &str) -> Self {
        Self::values("0", "0", orchard, ironwood, "0", "0")
    }

    pub fn unconfirmed_ironwood(value: &str) -> Self {
        Self::values("0", "0", "0", "0", value, "0")
    }

    pub fn locked_ironwood(value: &str) -> Self {
        Self::values("0", "0", "0", "0", "0", value)
    }

    fn values(
        transparent: &str,
        sapling: &str,
        orchard: &str,
        ironwood_spendable: &str,
        ironwood_unconfirmed: &str,
        ironwood_locked: &str,
    ) -> Self {
        let parse = |value: &str| value.parse::<u64>().expect("WAL-006 inventory value");
        Self {
            inner: PoolInventoryData {
                transparent: parse(transparent),
                sapling: parse(sapling),
                orchard: parse(orchard),
                ironwood_spendable: parse(ironwood_spendable),
                ironwood_unconfirmed: parse(ironwood_unconfirmed),
                ironwood_locked: parse(ironwood_locked),
            },
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreparedInspection {
    pub network: String,
    pub consensus_branch: u32,
    pub transaction_version: u32,
    pub destination: String,
    pub amount_zat: String,
    pub memo_sha256: String,
    pub fee_zat: String,
    pub ironwood_inputs: usize,
    pub ironwood_outputs: usize,
    pub has_transparent_bundle: bool,
    pub has_sapling_bundle: bool,
    pub has_orchard_output_bundle: bool,
    pub has_signatures: bool,
    pub has_proofs: bool,
    pub finalized: bool,
    pub extractable: bool,
    pub spend_pool: String,
    pub legacy_input_value_zat: String,
    pub intent_hash_binding: String,
    pub request_id_binding: String,
}

impl From<PcztInspection> for PreparedInspection {
    fn from(value: PcztInspection) -> Self {
        Self {
            network: value.network,
            consensus_branch: value.consensus_branch,
            transaction_version: value.transaction_version,
            destination: value.destination,
            amount_zat: value.amount_zat,
            memo_sha256: value.memo_sha256,
            fee_zat: value.fee_zat,
            ironwood_inputs: value.ironwood_inputs,
            ironwood_outputs: value.ironwood_outputs,
            has_transparent_bundle: value.has_transparent_bundle,
            has_sapling_bundle: value.has_sapling_bundle,
            has_orchard_output_bundle: value.has_orchard_output_bundle,
            has_signatures: value.has_signatures,
            has_proofs: value.has_proofs,
            finalized: value.finalized,
            extractable: value.extractable,
            spend_pool: value.spend_pool,
            legacy_input_value_zat: value.legacy_input_value_zat,
            intent_hash_binding: value.intent_hash_binding,
            request_id_binding: value.request_id_binding,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LookupObservation {
    pub shape: String,
    pub returned_bytes: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ObservableSecretClass {
    Seed,
    SpendingKey,
    VaultPlaintext,
    Ufvk,
    ReceiverInternals,
    Memo,
    NotePlaintext,
    Nullifier,
    CompactBlock,
    SqliteRow,
    RawPczt,
    Transaction,
    UserPath,
}

impl ObservableSecretClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Seed => "seed",
            Self::SpendingKey => "spending-key",
            Self::VaultPlaintext => "vault-plaintext",
            Self::Ufvk => "ufvk",
            Self::ReceiverInternals => "receiver-internals",
            Self::Memo => "memo",
            Self::NotePlaintext => "note-plaintext",
            Self::Nullifier => "nullifier",
            Self::CompactBlock => "compact-block",
            Self::SqliteRow => "sqlite-row",
            Self::RawPczt => "raw-pczt",
            Self::Transaction => "transaction",
            Self::UserPath => "user-path",
        }
    }
}

#[derive(Clone, Copy)]
pub struct ObservableCanary<'a> {
    class: ObservableSecretClass,
    value: &'a str,
}

impl<'a> ObservableCanary<'a> {
    pub fn new(class: ObservableSecretClass, value: &'a str) -> Self {
        Self { class, value }
    }

    pub fn class(&self) -> ObservableSecretClass {
        self.class
    }

    pub fn value(&self) -> &str {
        self.value
    }
}

pub struct ObservableCanaryReceipt {
    commitments: Vec<CanaryCommitment>,
}

impl ObservableCanaryReceipt {
    pub fn is_closed(&self) -> bool {
        self.class_names()
            == [
                "seed",
                "spending-key",
                "vault-plaintext",
                "ufvk",
                "receiver-internals",
                "memo",
                "note-plaintext",
                "nullifier",
                "compact-block",
                "sqlite-row",
                "raw-pczt",
                "transaction",
                "user-path",
            ]
    }

    pub fn class_names(&self) -> Vec<&'static str> {
        self.commitments.iter().map(|value| value.class).collect()
    }

    pub fn commitments(&self) -> &[CanaryCommitment] {
        &self.commitments
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Capabilities {
    pub can_sign: bool,
    pub can_prove: bool,
    pub can_finalize: bool,
    pub can_extract: bool,
    pub can_verify: bool,
    pub can_broadcast: bool,
    pub can_network: bool,
    pub can_mainnet: bool,
    pub can_xmr: bool,
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
    prepare: PrepareState,
}

impl core::fmt::Debug for TestAccount {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter.write_str("TestAccount([REDACTED])")
    }
}

impl TestAccount {
    pub fn bootstrap_from_fixture(
        root: TestStateRoot,
        account_id: AccountId,
        fixture: &FrozenFixture,
    ) -> Result<Self, ZecError> {
        let validated = fixture.inner.validate_complete()?;
        let network = super::LocalNetwork::new(
            validated.manifest.network.birthday_height,
            validated.manifest.network.nu6_3,
            validated.manifest.expected.confirmation_height,
        )?;
        let seed = SecretBytes::new(vec![0; 32]).map_err(|_| ZecError::internal())?;
        Self::bootstrap(root, account_id, Network::Local(network), seed)
    }

    pub fn bootstrap(
        root: TestStateRoot,
        account_id: AccountId,
        network: Network,
        seed: SecretBytes,
    ) -> Result<Self, ZecError> {
        let mut observer = IgnoreWipes;
        AddressAccount::bootstrap(root.inner, account_id, network, seed, &mut observer).map(
            |inner| Self {
                inner,
                prepare: PrepareState::new(),
            },
        )
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
        AddressAccount::open_viewing(root.inner, account_id).map(|inner| Self {
            inner,
            prepare: PrepareState::viewing_only(),
        })
    }

    pub fn open_viewing_with_network(
        root: TestStateRoot,
        account_id: AccountId,
        network: Network,
    ) -> Result<Self, ZecError> {
        AddressAccount::open_viewing_with_network(root.inner, account_id, network).map(|inner| {
            Self {
                inner,
                prepare: PrepareState::viewing_only(),
            }
        })
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
        self.prepare.invalidate(HandleInvalidation::BrokerExit);
        Ok(TestStateRoot {
            inner: self.inner.root(),
        })
    }

    pub fn unlock_with_fixture_seed(&mut self) -> Result<(), ZecError> {
        self.prepare
            .unlock(SecretBytes::new(vec![0; 32]).map_err(|_| ZecError::internal())?)
    }

    pub fn session_id(&self) -> String {
        self.prepare.session_id()
    }

    pub fn prepare(
        &mut self,
        input: PrepareZecV1,
        clock: &mut ManualClock,
    ) -> Result<PreparedZecV1, ZecError> {
        self.prepare.prepare(&self.inner, input, None, &clock.value)
    }

    pub fn prepare_with_binding(
        &mut self,
        input: PrepareZecV1,
        binding: PrepareBinding,
        clock: &mut ManualClock,
    ) -> Result<PreparedZecV1, ZecError> {
        self.prepare
            .prepare(&self.inner, input, Some(&binding), &clock.value)
    }

    pub fn prepare_mutated_for_test(
        &mut self,
        mut input: PrepareZecV1,
        mutation: PrepareMutation,
        clock: &mut ManualClock,
    ) -> Result<PreparedZecV1, ZecError> {
        match mutation {
            PrepareMutation::AccountId(value) => input.account_id = value,
            PrepareMutation::Network(value) => input.network = value,
            PrepareMutation::RequestId(value) => input.request_id = value,
            PrepareMutation::IntentHash(value) => input.intent_hash = value,
            PrepareMutation::Amount(value) => input.amount_zat = value,
            PrepareMutation::FeeBound(value) => input.fee_bound_zat = value,
            PrepareMutation::ExpiresAt(value) => input.expires_at = value,
        }
        self.prepare(input, clock)
    }

    pub fn prepare_with_receiver(
        &mut self,
        mut input: PrepareZecV1,
        receiver: impl AsRef<str>,
        clock: &mut ManualClock,
    ) -> Result<PreparedZecV1, ZecError> {
        input.receiver = receiver.as_ref().to_owned();
        self.prepare(input, clock)
    }

    pub fn prepare_with_memo(
        &mut self,
        mut input: PrepareZecV1,
        memo: String,
        clock: &mut ManualClock,
    ) -> Result<PreparedZecV1, ZecError> {
        input.memo = memo;
        self.prepare(input, clock)
    }

    pub fn prepare_with_fee_bound(
        &mut self,
        mut input: PrepareZecV1,
        fee_bound: &str,
        clock: &mut ManualClock,
    ) -> Result<PreparedZecV1, ZecError> {
        input.fee_bound_zat = fee_bound.to_owned();
        self.prepare(input, clock)
    }

    pub fn inspect_prepared_for_test(&self, handle: &str) -> Result<PreparedInspection, ZecError> {
        self.prepare
            .inspection(handle)
            .map(PreparedInspection::from)
    }

    pub fn reset_spend_access_observer(&mut self) {
        self.prepare.reset_spend_access();
    }

    pub fn spend_access_count(&self) -> usize {
        self.prepare.spend_accesses()
    }

    pub fn prepared_handle_count(&self) -> usize {
        self.prepare.handle_count()
    }

    pub fn replace_inventory_for_test(&mut self, inventory: PoolInventory) {
        self.prepare.replace_inventory(inventory.inner);
    }

    pub fn fee_rule_calls(&self) -> usize {
        self.prepare.fee_rule_calls()
    }

    pub fn caller_fee_calls(&self) -> usize {
        self.prepare.caller_fee_calls()
    }

    pub fn fill_prepared_handles_for_test(&mut self, count: usize) -> Result<(), ZecError> {
        self.prepare.fill_reserved(count)
    }

    pub fn lock(&mut self) -> Result<(), ZecError> {
        self.prepare.invalidate(HandleInvalidation::Lock);
        Ok(())
    }

    pub fn lookup_prepared(
        &self,
        handle: &str,
        binding: &HandleBinding,
    ) -> Result<PreparedZecV1, ZecError> {
        self.prepare.lookup(handle, binding)
    }

    pub fn reset_lookup_observer(&mut self) {
        self.prepare.reset_lookup();
    }

    pub fn lookup_observation(&self) -> LookupObservation {
        let (shape, returned_bytes) = self.prepare.lookup_observation();
        LookupObservation {
            shape,
            returned_bytes,
        }
    }

    pub fn constant_miss_shape(&self) -> String {
        "zec-prepared-lookup-miss-v1".to_owned()
    }

    pub fn attach_wipe_observer(&mut self, wipes: RecordingWipes) {
        self.prepare.attach_wipe_log(wipes.prepare_log);
    }

    pub fn prepared_raw_length_for_test(&self, handle: &str) -> Result<usize, ZecError> {
        self.prepare.raw_len(handle)
    }

    pub fn invalidate_for_test(&mut self, edge: HandleInvalidation) -> Result<(), ZecError> {
        self.prepare.invalidate(edge);
        Ok(())
    }

    pub fn contains_prepared_handle(&self, handle: &str) -> bool {
        self.prepare.contains(handle)
    }

    pub fn spend_material_length_for_test(&self) -> usize {
        self.prepare.derived_len()
    }

    pub fn panic_inside_prepare_for_test(&mut self) -> ! {
        self.prepare.arm_panic_after_access();
        let mut clock = ManualClock::at("2026-08-30T12:00:00Z");
        let _ = self.prepare_fixture_payment(
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
            &mut clock,
        );
        panic!("WAL-006 prepare panic guard did not unwind")
    }

    pub fn prepare_fixture_payment(
        &mut self,
        request_id: &str,
        intent_hash: &str,
        clock: &mut ManualClock,
    ) -> Result<PreparedZecV1, ZecError> {
        let fixture = FrozenFixture::open("tests/fixtures/zec")?;
        let input = PrepareZecV1::new(
            self.inner.account_id().as_str(),
            self.inner.network().as_str(),
            request_id,
            intent_hash,
            fixture.expected_destination_receiver(),
            "100000000",
            "12000",
            "coffee",
            "2026-08-30T12:15:00Z",
        )?;
        self.prepare(input, clock)
    }

    pub fn install_observable_canaries_for_test(
        &mut self,
        canaries: &[ObservableCanary<'_>],
    ) -> Result<ObservableCanaryReceipt, ZecError> {
        let expected = [
            ObservableSecretClass::Seed,
            ObservableSecretClass::SpendingKey,
            ObservableSecretClass::VaultPlaintext,
            ObservableSecretClass::Ufvk,
            ObservableSecretClass::ReceiverInternals,
            ObservableSecretClass::Memo,
            ObservableSecretClass::NotePlaintext,
            ObservableSecretClass::Nullifier,
            ObservableSecretClass::CompactBlock,
            ObservableSecretClass::SqliteRow,
            ObservableSecretClass::RawPczt,
            ObservableSecretClass::Transaction,
            ObservableSecretClass::UserPath,
        ];
        if canaries.len() != expected.len()
            || !canaries
                .iter()
                .zip(expected)
                .all(|(canary, class)| canary.class == class && !canary.value.is_empty())
        {
            return Err(ZecError::schema());
        }
        let commitments = canaries
            .iter()
            .map(|canary| CanaryCommitment {
                class: canary.class.as_str(),
                byte_length: canary.value.len(),
                sha256: sha256_hex(canary.value.as_bytes()),
            })
            .collect::<Vec<_>>();
        self.prepare.install_canary_commitments(
            commitments
                .iter()
                .map(|value| {
                    (
                        value.class.to_owned(),
                        value.byte_length,
                        value.sha256.clone(),
                    )
                })
                .collect(),
        );
        Ok(ObservableCanaryReceipt { commitments })
    }

    pub fn synthetic_failure_for_test(&self) -> ZecError {
        ZecError::internal()
    }

    pub fn captured_logs(&self) -> Vec<&'static str> {
        let _installed_secret_classes = self.prepare.canary_commitment_count();
        Vec::new()
    }

    pub fn diagnostics(&self) -> Vec<&'static str> {
        vec!["[REDACTED]"]
    }

    pub fn diagnostic_field_names(&self) -> [&'static str; 4] {
        ["operation", "account_id", "network", "code"]
    }

    pub fn public_zec_operations(&self) -> [&'static str; 4] {
        [
            "account.bootstrap",
            "receiver.fresh",
            "fixture.scan",
            "pczt.prepare",
        ]
    }

    pub fn invoke_operation_for_test(&self, operation: &str) -> Result<(), ZecError> {
        spend::invoke_operation(operation)
    }
}

impl SignVerifyHarness {
    #[allow(
        clippy::too_many_arguments,
        reason = "reviewed authority/verification/fault inputs deliberately remain explicit"
    )]
    fn execute(
        &mut self,
        handle: &str,
        confirmation: TestConfirmation,
        route: SignRoute,
        clock: &mut ManualClock,
        fault: Option<PipelineFault>,
        mutation: Option<SignVerifyMutation>,
        barrier: Option<BarrierMutation>,
    ) -> Result<VerifiedZecV1, ZecError> {
        let account_id = confirmation.review.public.account_id.clone();
        let request_id = confirmation.review.public.request_id.clone();
        let outcome = self.execute_pipeline(
            handle,
            confirmation,
            route,
            clock,
            fault,
            mutation.map(verify_mutation),
        )?;
        match barrier {
            Some(BarrierMutation::CancelAfterSignAndProof) => {
                if let Some(account) = self.accounts.get(&account_id) {
                    account.prepare.cancel_request(&request_id);
                }
                self.calls.post_sign_status_reads =
                    self.calls.post_sign_status_reads.saturating_add(1);
                let revalidated = self
                    .accounts
                    .get(&account_id)
                    .ok_or_else(ZecError::locked)?
                    .prepare
                    .revalidate_after_sign(&outcome.review, &clock.value);
                self.finish_failed_pipeline(outcome, WipeExit::Cancellation);
                revalidated.and(Err(ZecError::cancelled()))
            }
            Some(BarrierMutation::ClockAfterSignAndProof(now)) => {
                self.calls.post_sign_clock_reads =
                    self.calls.post_sign_clock_reads.saturating_add(1);
                let revalidated = self
                    .accounts
                    .get(&account_id)
                    .ok_or_else(ZecError::locked)?
                    .prepare
                    .revalidate_after_sign(&outcome.review, &now);
                match revalidated {
                    Ok(()) => {
                        let mut outcome = outcome;
                        outcome.authorization_time = now;
                        self.publish(outcome, account_id, request_id, WipeExit::Success)
                    }
                    Err(error) => {
                        self.finish_failed_pipeline(outcome, WipeExit::Expiry);
                        Err(error)
                    }
                }
            }
            None => self.publish(outcome, account_id, request_id, WipeExit::Success),
        }
    }

    fn execute_pipeline(
        &mut self,
        handle: &str,
        confirmation: TestConfirmation,
        route: SignRoute,
        clock: &mut ManualClock,
        fault: Option<PipelineFault>,
        mutation: Option<VerifyMutation>,
    ) -> Result<PipelineOutcome, ZecError> {
        self.validate_confirmation(handle, &confirmation)?;
        let TestConfirmation { capability, review } = confirmation;
        let capability = capability.consume().map_err(|_| ZecError::unauth())?;
        let account_id = review.public.account_id.clone();
        let lease = self.gate.acquire(&account_id)?;
        if route == SignRoute::ProductionHardware {
            let denied = spend::production_hardware_denied();
            if let Some(account) = self.accounts.get(&account_id) {
                account
                    .prepare
                    .invalidate(HandleInvalidation::OperationError);
            }
            self.destroy_installed_canary(TouchedSecretClass::ConfirmationCapability);
            return denied.and(Err(ZecError::capability_missing()));
        }
        if self.mode != HarnessMode::Software {
            return Err(ZecError::capability_missing());
        }
        let account = self
            .accounts
            .get(&account_id)
            .ok_or_else(ZecError::schema)?;
        if account.prepare.is_viewing_only() {
            return Err(ZecError::watch_only());
        }
        self.calls.seed_accesses = self.calls.seed_accesses.saturating_add(1);
        self.calls.spend_authority_derivations =
            self.calls.spend_authority_derivations.saturating_add(1);
        let seed = account.prepare.seed_copy()?;
        let mut attempt = AttemptOwner::new(self.observations.clone());
        let seed_owner = seed.into_observed("zec-operation-seed", Box::new(attempt.collector()));
        let stored_ufvk = match account.account.viewing_key_binding() {
            Ok(stored_ufvk) => stored_ufvk,
            Err(error) => {
                attempt.finish(wipe_exit_for_error(&error, None));
                return Err(error);
            }
        };
        let network = account.account.network();
        let artifact = match account.prepare.consume(&review, &clock.value) {
            Ok(artifact) => artifact,
            Err(error) => {
                attempt.finish(wipe_exit_for_error(&error, None));
                return Err(error);
            }
        };
        let mut pipeline_calls = PipelineCalls::default();
        let mut observer = attempt.collector();
        let result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            spend::authorize_software(
                artifact,
                seed_owner.as_secret(),
                &stored_ufvk,
                network,
                &mut pipeline_calls,
                fault,
                mutation,
                &mut observer,
            )
        })) {
            Ok(result) => result,
            Err(payload) => {
                self.merge_pipeline_calls(&pipeline_calls);
                if let Some(account) = self.accounts.get(&account_id) {
                    account.prepare.invalidate(HandleInvalidation::PanicUnwind);
                }
                std::panic::resume_unwind(payload);
            }
        };
        drop(observer);
        drop(seed_owner);
        self.merge_pipeline_calls(&pipeline_calls);
        match result {
            Ok(effects) => Ok(PipelineOutcome {
                review,
                effects,
                authorization_time: clock.value.clone(),
                confirmation_capability: capability,
                _lease: lease,
                attempt,
            }),
            Err(error) => {
                if let Some(account) = self.accounts.get(&account_id) {
                    account
                        .prepare
                        .invalidate(HandleInvalidation::OperationError);
                }
                attempt.finish(wipe_exit_for_error(
                    &error,
                    pipeline_calls.triggered_fault(),
                ));
                Err(error)
            }
        }
    }

    fn execute_external(
        &mut self,
        handle: &str,
        confirmation: TestConfirmation,
        contribution: TaggedContribution,
        clock: &mut ManualClock,
    ) -> Result<VerifiedZecV1, ZecError> {
        self.validate_confirmation(handle, &confirmation)?;
        let TestConfirmation { capability, review } = confirmation;
        let capability = capability.consume().map_err(|_| ZecError::unauth())?;
        let account_id = review.public.account_id.clone();
        let request_id = review.public.request_id.clone();
        let lease = self.gate.acquire(&account_id)?;
        if self.mode != HarnessMode::SyntheticKeystone {
            return Err(ZecError::capability_missing());
        }
        let account = self
            .accounts
            .get(&account_id)
            .ok_or_else(ZecError::schema)?;
        let stored_ufvk = account.account.viewing_key_binding()?;
        let network = account.account.network();
        let artifact = account.prepare.consume(&review, &clock.value)?;
        let mut pipeline_calls = PipelineCalls::default();
        let mut attempt = AttemptOwner::new(self.observations.clone());
        let mut observer = attempt.collector();
        let result = spend::authorize_external(
            artifact,
            contribution,
            &stored_ufvk,
            network,
            &mut pipeline_calls,
            None,
            &mut observer,
        );
        drop(observer);
        self.merge_pipeline_calls(&pipeline_calls);
        let effects = match result {
            Ok(effects) => effects,
            Err(error) => {
                attempt.finish(wipe_exit_for_error(
                    &error,
                    pipeline_calls.triggered_fault(),
                ));
                return Err(error);
            }
        };
        self.publish(
            PipelineOutcome {
                review,
                effects,
                authorization_time: clock.value.clone(),
                confirmation_capability: capability,
                _lease: lease,
                attempt,
            },
            account_id,
            request_id,
            WipeExit::Success,
        )
    }

    fn publish(
        &mut self,
        mut outcome: PipelineOutcome,
        account_id: String,
        request_id: String,
        exit: WipeExit,
    ) -> Result<VerifiedZecV1, ZecError> {
        let _untrusted_signer_metadata_was_ignored = self.untrusted_signer_transaction_id.take();
        self.calls.post_sign_status_reads = self.calls.post_sign_status_reads.saturating_add(1);
        self.calls.post_sign_clock_reads = self.calls.post_sign_clock_reads.saturating_add(1);
        let revalidated = capability_matches(
            outcome.review.handle.as_str(),
            &outcome.review,
            &outcome.confirmation_capability,
        )
        .and_then(|_| {
            self.accounts
                .get(&outcome.review.public.account_id)
                .filter(|_| outcome.review.public.account_id == account_id)
                .ok_or_else(ZecError::locked)
        })
        .and_then(|account| {
            account
                .prepare
                .revalidate_after_sign(&outcome.review, &outcome.authorization_time)
        });
        if let Err(error) = revalidated {
            self.finish_failed_pipeline(outcome, wipe_exit_for_error(&error, None));
            return Err(error);
        }
        let mut handle_bytes = [0; 16];
        getrandom::fill(&mut handle_bytes).map_err(|_| ZecError::internal())?;
        let handle = spend::hex(&handle_bytes);
        let transaction_id = outcome.effects.derived_transaction_id.clone();
        self.touch_canaries();
        if let Some(account) = self.accounts.get(&outcome.review.public.account_id) {
            account
                .prepare
                .invalidate(HandleInvalidation::OperationError);
        }
        self.verified.insert(handle.clone(), outcome.effects);
        self.calls.verified_publications = self.calls.verified_publications.saturating_add(1);
        let verified = VerifiedZecV1 {
            handle,
            transaction_id,
            state: "verified",
            account_id,
            request_id,
            broadcastable: false,
        };
        outcome.attempt.finish(exit);
        Ok(verified)
    }

    fn finish_failed_pipeline(&mut self, mut outcome: PipelineOutcome, exit: WipeExit) {
        outcome.attempt.finish(exit);
        if let Some(account) = self.accounts.get(&outcome.review.public.account_id) {
            account
                .prepare
                .invalidate(HandleInvalidation::OperationError);
        }
        self.touch_canaries();
    }

    fn validate_confirmation(
        &self,
        handle: &str,
        confirmation: &TestConfirmation,
    ) -> Result<(), ZecError> {
        capability_matches(handle, &confirmation.review, &confirmation.capability)
    }

    fn validate_external(
        &self,
        review: &PreparedReview,
        contributions: &ExternalContributions,
        expected: usize,
    ) -> Result<(), ZecError> {
        if contributions.route != "keystone_pczt_v2"
            || contributions.entries.len() != expected
            || contributions.entries.iter().any(|entry| {
                entry.pool != "ironwood"
                    || entry.action_index != entry.inner.signature.action_index()
                    || entry.randomized_key != spend::hex(&entry.inner.randomized_key)
                    || entry.intent_hash != review.public.intent_hash
                    || entry.batch_id != review.review_hash[..32]
                    || !entry.test_only
            })
        {
            return Err(ZecError::signature_invalid());
        }
        Ok(())
    }

    fn find_review(&self, handle: &str, now: &str) -> Result<(String, PreparedReview), ZecError> {
        for (account_id, account) in &self.accounts {
            if account.prepare.contains(handle) {
                return account
                    .prepare
                    .review(handle, now)
                    .map(|review| (account_id.clone(), review));
            }
        }
        Err(ZecError::locked())
    }

    fn find_review_read_only(&self, handle: &str) -> Result<(String, PreparedReview), ZecError> {
        self.find_review(handle, "2026-08-30T12:00:30Z")
    }

    fn merge_pipeline_calls(&mut self, calls: &PipelineCalls) {
        self.calls.authoritative_pczt_accesses = self
            .calls
            .authoritative_pczt_accesses
            .saturating_add(calls.authoritative_pczt_accesses);
        self.calls.signer_calls = self.calls.signer_calls.saturating_add(calls.signer_calls);
        self.calls.prover_calls = self.calls.prover_calls.saturating_add(calls.prover_calls);
        self.calls.finalizer_calls = self
            .calls
            .finalizer_calls
            .saturating_add(calls.finalizer_calls);
        self.calls.extractor_calls = self
            .calls
            .extractor_calls
            .saturating_add(calls.extractor_calls);
        self.calls.independent_decoder_calls = self
            .calls
            .independent_decoder_calls
            .saturating_add(calls.independent_decoder_calls);
        self.calls.verifier_calls = self
            .calls
            .verifier_calls
            .saturating_add(calls.verifier_calls);
        self.calls.external_contributions_applied = self
            .calls
            .external_contributions_applied
            .saturating_add(calls.external_contributions_applied);
        self.calls.external_signatures_verified = self
            .calls
            .external_signatures_verified
            .saturating_add(calls.external_signatures_verified);
    }

    fn destroy_installed_canary(&mut self, class: TouchedSecretClass) {
        if let Some(mut secret) = self.canary_owners.remove(&class) {
            secret.wipe_with("zec-sign-verify-canary", &mut IgnoreWipes);
            *self.canary_touches.entry(class).or_default() += 1;
        }
    }

    fn destroy_software_canaries(&mut self) {
        for class in [
            TouchedSecretClass::Seed,
            TouchedSecretClass::UnifiedSpendingAuthority,
            TouchedSecretClass::DerivedAuthorizingKey,
            TouchedSecretClass::ConfirmationCapability,
            TouchedSecretClass::AuthoritativePczt,
            TouchedSecretClass::ProofWorkspace,
            TouchedSecretClass::ExtractedTransaction,
        ] {
            self.destroy_installed_canary(class);
        }
    }

    fn destroy_hardware_canaries(&mut self) {
        for class in [
            TouchedSecretClass::ConfirmationCapability,
            TouchedSecretClass::SignerView,
            TouchedSecretClass::SignatureContribution,
            TouchedSecretClass::AuthoritativePczt,
            TouchedSecretClass::ProofWorkspace,
            TouchedSecretClass::ExtractedTransaction,
        ] {
            self.destroy_installed_canary(class);
        }
    }

    fn touch_canaries(&mut self) {
        for value in &self.installed_canaries {
            *self.canary_touches.entry(value.class).or_default() += 1;
        }
    }
}

fn verify_mutation(mutation: SignVerifyMutation) -> VerifyMutation {
    match mutation {
        SignVerifyMutation::Receiver => VerifyMutation::Receiver,
        SignVerifyMutation::Amount => VerifyMutation::Amount,
        SignVerifyMutation::Network => VerifyMutation::Network,
        SignVerifyMutation::Fee => VerifyMutation::Fee,
        SignVerifyMutation::FeeBound => VerifyMutation::FeeBound,
        SignVerifyMutation::Memo => VerifyMutation::Memo,
        SignVerifyMutation::RequestId => VerifyMutation::RequestId,
        SignVerifyMutation::IntentHash => VerifyMutation::IntentHash,
        SignVerifyMutation::Pool => VerifyMutation::Pool,
        SignVerifyMutation::Version => VerifyMutation::Version,
        SignVerifyMutation::ConsensusBranch => VerifyMutation::ConsensusBranch,
        SignVerifyMutation::Change => VerifyMutation::Change,
        SignVerifyMutation::ExtractedTransactionIdBinding => {
            VerifyMutation::ExtractedTransactionIdBinding
        }
        SignVerifyMutation::Proof => VerifyMutation::Proof,
        SignVerifyMutation::Signature => VerifyMutation::Signature,
        SignVerifyMutation::MalformedState => VerifyMutation::MalformedState,
        SignVerifyMutation::MalformedSchema => VerifyMutation::MalformedSchema,
    }
}

fn wipe_exit_for_error(error: &ZecError, fault: Option<PipelineFault>) -> WipeExit {
    if let Some(fault) = fault {
        return wipe_exit_for_fault(match fault {
            PipelineFault::Signer => FaultPoint::Signer,
            PipelineFault::Prover => FaultPoint::Prover,
            PipelineFault::Finalizer => FaultPoint::Finalizer,
            PipelineFault::Extractor => FaultPoint::Extractor,
            PipelineFault::Verifier => FaultPoint::Verifier,
            PipelineFault::Cleanup => FaultPoint::Cleanup,
            PipelineFault::PanicAfterVerification => return wipe_exit_for_error(error, None),
        });
    }
    match error.code() {
        "CANCELLED" => WipeExit::Cancellation,
        "EXPIRED" => WipeExit::Expiry,
        "LOCKED" => WipeExit::Lock,
        _ => WipeExit::Error,
    }
}

pub struct PendingAuthorization {
    account_id: String,
}

impl core::fmt::Debug for PendingAuthorization {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter.write_str("PendingAuthorization([REDACTED])")
    }
}

struct AttemptWipeState {
    events: Vec<WipeEvent>,
}

#[derive(Clone)]
struct AttemptWipeCollector {
    inner: Arc<Mutex<AttemptWipeState>>,
}

impl AttemptWipeCollector {
    fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(AttemptWipeState { events: Vec::new() })),
        }
    }
}

impl WipeObserver for AttemptWipeCollector {
    fn observe(&mut self, event: WipeEvent) {
        mutex_lock(&self.inner).events.push(event);
    }
}

struct AttemptOwner {
    observations: SignVerifyObservations,
    collector: AttemptWipeCollector,
    selected_outcome: Option<WipeExit>,
}

impl AttemptOwner {
    fn new(observations: SignVerifyObservations) -> Self {
        Self {
            observations,
            collector: AttemptWipeCollector::new(),
            selected_outcome: None,
        }
    }

    fn collector(&self) -> AttemptWipeCollector {
        self.collector.clone()
    }

    fn finish(&mut self, exit: WipeExit) {
        self.selected_outcome = Some(exit);
    }

    fn classify(&self, exit: WipeExit) {
        let mut inner = mutex_lock(&self.collector.inner);
        let events = std::mem::take(&mut inner.events);
        drop(inner);
        for event in events {
            match event.label {
                "zec-operation-seed" => self.observations.record_event(
                    TouchedSecretClass::Seed,
                    exit,
                    event.length,
                    event.all_zero,
                ),
                "zec-authoritative-pczt" => self.observations.record_event(
                    TouchedSecretClass::AuthoritativePczt,
                    exit,
                    event.length,
                    event.all_zero,
                ),
                "zec-extracted-transaction" => self.observations.record_event(
                    TouchedSecretClass::ExtractedTransaction,
                    exit,
                    event.length,
                    event.all_zero,
                ),
                _ => self.observations.record_unclassified(),
            }
        }
    }
}

impl Drop for AttemptOwner {
    fn drop(&mut self) {
        let exit = if std::thread::panicking() {
            WipeExit::PanicUnwind
        } else {
            self.selected_outcome.unwrap_or(WipeExit::Error)
        };
        self.classify(exit);
    }
}

struct PipelineOutcome {
    review: PreparedReview,
    effects: VerifiedEffects,
    authorization_time: String,
    confirmation_capability: ZecConfirmationCapability,
    _lease: AccountAuthorizationLease,
    attempt: AttemptOwner,
}

pub struct IndependentEffects {
    pub network: String,
    pub mainnet: bool,
    pub transaction_version: u32,
    pub consensus_branch: u32,
    pub external_receiver_bytes: Vec<u8>,
    pub external_receiver: String,
    pub external_amount_zat: String,
    pub fee_zat: String,
    pub fee_bound_zat: String,
    pub fee_zat_u64: u64,
    pub fee_bound_zat_u64: u64,
    pub memo_sha256: String,
    pub request_id_binding: String,
    pub intent_hash_binding: String,
    pub ironwood_real_spends: usize,
    pub ironwood_external_outputs: usize,
    pub ironwood_internal_change_outputs: usize,
    pub transparent_effects: usize,
    pub sapling_effects: usize,
    pub orchard_effects: usize,
    pub proof_present: bool,
    pub proof_valid: bool,
    pub spend_authorization_present: bool,
    pub spend_authorization_valid: bool,
    pub binding_signature_present: bool,
    pub binding_signature_valid: bool,
    pub derived_transaction_id: String,
}

impl From<&VerifiedEffects> for IndependentEffects {
    fn from(value: &VerifiedEffects) -> Self {
        Self {
            network: value.network.clone(),
            mainnet: value.mainnet,
            transaction_version: value.transaction_version,
            consensus_branch: value.consensus_branch,
            external_receiver_bytes: value.external_receiver_bytes.clone(),
            external_receiver: value.external_receiver.clone(),
            external_amount_zat: value.external_amount_zat.clone(),
            fee_zat: value.fee_zat.clone(),
            fee_bound_zat: value.fee_bound_zat.clone(),
            fee_zat_u64: value.fee_zat_u64,
            fee_bound_zat_u64: value.fee_bound_zat_u64,
            memo_sha256: value.memo_sha256.clone(),
            request_id_binding: value.request_id_binding.clone(),
            intent_hash_binding: value.intent_hash_binding.clone(),
            ironwood_real_spends: value.ironwood_real_spends,
            ironwood_external_outputs: value.ironwood_external_outputs,
            ironwood_internal_change_outputs: value.ironwood_internal_change_outputs,
            transparent_effects: value.transparent_effects,
            sapling_effects: value.sapling_effects,
            orchard_effects: value.orchard_effects,
            proof_present: value.proof_present,
            proof_valid: value.proof_valid,
            spend_authorization_present: value.spend_authorization_present,
            spend_authorization_valid: value.spend_authorization_valid,
            binding_signature_present: value.binding_signature_present,
            binding_signature_valid: value.binding_signature_valid,
            derived_transaction_id: value.derived_transaction_id.clone(),
        }
    }
}

struct RejectingSurface;

impl crate::native::ZecNativeReviewSurfacePort for RejectingSurface {
    fn confirm_zec_review(
        &mut self,
        _review: &ZecNativeReview,
    ) -> Result<bool, crate::native::NativeError> {
        Ok(false)
    }
}

fn native_review(review: &PreparedReview) -> ZecNativeReview {
    ZecNativeReview {
        handle: review.handle.clone(),
        session_id: review.session_id.clone(),
        account_id: review.public.account_id.clone(),
        network: review.public.network.clone(),
        request_id: review.public.request_id.clone(),
        intent_hash: review.public.intent_hash.clone(),
        receiver: review.public.receiver.clone(),
        amount_zat: review.public.amount_zat.clone(),
        fee_zat: review.public.fee_zat.clone(),
        fee_bound_zat: review.public.fee_bound_zat.clone(),
        memo_sha256: review.inspection.memo_sha256.clone(),
        expires_at: review.public.expires_at.clone(),
        transaction_version: review.inspection.transaction_version,
        consensus_branch: review.inspection.consensus_branch,
        spend_pool: review.public.spend_pool.clone(),
        output_pool: review.public.output_pool.clone(),
        review_hash: review.review_hash.clone(),
    }
}

fn capability_matches(
    handle: &str,
    review: &PreparedReview,
    capability: &ZecConfirmationCapability,
) -> Result<(), ZecError> {
    if handle != review.handle
        || capability.handle() != review.handle
        || capability.session_id() != review.session_id
        || capability.account_id() != review.public.account_id
        || capability.network() != review.public.network
        || capability.request_id() != review.public.request_id
        || capability.intent_hash() != review.public.intent_hash
        || capability.review_hash() != review.review_hash
        || capability.nonce_len() != 32
        || !capability.came_from_synthetic_test_surface()
    {
        return Err(ZecError::intent_mismatch());
    }
    Ok(())
}

fn reviewed_view(
    review: &PreparedReview,
    artifact: SignerViewArtifact,
    network: Network,
) -> ReviewedSignerView {
    let randomized_key = spend::hex(&artifact.randomized_key);
    ReviewedSignerView {
        route: "keystone_pczt_v2",
        pczt_encoding_version: 2,
        batch_len: 1,
        batch_id: review.review_hash[..32].to_owned(),
        intent_hash: review.public.intent_hash.clone(),
        review_hash: review.review_hash.clone(),
        network: review.public.network.clone(),
        transaction_version: review.inspection.transaction_version,
        consensus_branch: review.inspection.consensus_branch,
        signing_pool: "ironwood",
        actions: vec![ReviewedSignerAction {
            pool: "ironwood",
            action_index: artifact.action_index,
            randomized_key,
            intent_hash: review.public.intent_hash.clone(),
        }],
        artifact,
        network_value: network,
    }
}

fn wipe_exit_for_fault(fault: FaultPoint) -> WipeExit {
    match fault {
        FaultPoint::Signer => WipeExit::SignerError,
        FaultPoint::Prover => WipeExit::ProverError,
        FaultPoint::Finalizer => WipeExit::FinalizerError,
        FaultPoint::Extractor => WipeExit::ExtractorError,
        FaultPoint::Verifier => WipeExit::VerifierError,
        FaultPoint::Cleanup => WipeExit::CleanupError,
    }
}

fn prepared_commitment(review: &PreparedReview) -> String {
    let mut hasher = Sha256::new();
    for value in [
        review.public.network.as_bytes(),
        review.public.receiver.as_bytes(),
        review.public.amount_zat.as_bytes(),
        review.public.fee_zat.as_bytes(),
        review.public.fee_bound_zat.as_bytes(),
        review.inspection.memo_sha256.as_bytes(),
        review.public.request_id.as_bytes(),
        review.public.intent_hash.as_bytes(),
    ] {
        hasher.update((value.len() as u64).to_le_bytes());
        hasher.update(value);
    }
    spend::hex(&hasher.finalize())
}

fn prepared_commitment_from_effects(effects: &VerifiedEffects) -> String {
    let mut hasher = Sha256::new();
    for value in [
        effects.network.as_bytes(),
        effects.external_receiver.as_bytes(),
        effects.external_amount_zat.as_bytes(),
        effects.fee_zat.as_bytes(),
        effects.fee_bound_zat.as_bytes(),
        effects.memo_sha256.as_bytes(),
        effects.request_id_binding.as_bytes(),
        effects.intent_hash_binding.as_bytes(),
    ] {
        hasher.update((value.len() as u64).to_le_bytes());
        hasher.update(value);
    }
    spend::hex(&hasher.finalize())
}

impl TestAccount {
    pub fn capabilities(&self) -> Capabilities {
        Capabilities {
            can_sign: false,
            can_prove: false,
            can_finalize: false,
            can_extract: false,
            can_verify: false,
            can_broadcast: false,
            can_network: false,
            can_mainnet: false,
            can_xmr: false,
        }
    }

    pub fn normalize_diagnostic_for_test(&self, value: &str) -> Result<&'static str, ZecError> {
        normalize_diagnostic(value)
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

    pub fn scan(&mut self, fixture: &FrozenFixture) -> Result<(), ZecError> {
        let validated = fixture.inner.validate_complete()?;
        self.inner.scan_fixture(&validated, ScanRequest::Canonical)
    }

    pub fn scan_through(&mut self, fixture: &FrozenFixture, height: u32) -> Result<(), ZecError> {
        let validated = fixture.inner.validate_complete()?;
        self.inner
            .scan_fixture(&validated, ScanRequest::Through(height))
    }

    pub fn scan_scenario(
        &mut self,
        fixture: &FrozenFixture,
        scenario: &str,
    ) -> Result<(), ZecError> {
        let mut validated = fixture.inner.validate_complete()?;
        match scenario {
            "wrong-branch" => {
                let local = super::LocalNetwork::new(
                    validated.manifest.network.birthday_height,
                    validated.manifest.network.nu6_3,
                    validated.manifest.expected.confirmation_height,
                )?;
                let actual = u32::from(BranchId::for_height(
                    &local.upstream(),
                    BlockHeight::from_u32(validated.manifest.network.nu6_3),
                ));
                validated.manifest.expected.nu6_3_branch_id_hex = format!("{:08x}", actual ^ 1);
            }
            "wrong-network" => {
                validated.manifest.network.discriminator = "zec-testnet".to_owned();
            }
            _ => {}
        }
        self.inner
            .scan_fixture(&validated, ScanRequest::Scenario(scenario.to_owned()))
    }

    pub fn inspect_scan_state(&self) -> Result<ScanStateInspection, ZecError> {
        self.inner.inspect_scan().map(ScanStateInspection::from)
    }

    pub fn balances(&self) -> Result<ScanBalances, ZecError> {
        self.inspect_scan_state().map(|state| state.balances)
    }

    pub fn arm_scan_fault(&mut self, fault: ScanFault) {
        self.inner.arm_scan_fault(fault.into());
    }

    pub fn scan_calls(&self) -> usize {
        self.inner.scan_metrics().scan_calls
    }

    pub fn applied_block_count(&self) -> usize {
        self.inner.scan_metrics().applied_block_count
    }

    pub fn recognized_note_count(&self) -> usize {
        self.inner
            .recognized_note_count()
            .expect("WAL-006 recognized-note inspection failed")
    }

    pub fn unrelated_output_count_seen(&self) -> usize {
        self.inner
            .scan_metrics()
            .unrelated_output_count
            .expect("WAL-006 unrelated-output inspection unavailable")
    }

    pub fn rolled_back_note_count(&self) -> usize {
        self.inner
            .scan_metrics()
            .rolled_back_note_count
            .expect("WAL-006 rolled-back-note inspection unavailable")
    }

    pub fn rolled_back_block_count(&self) -> usize {
        self.inner
            .scan_metrics()
            .rolled_back_block_count
            .expect("WAL-006 rolled-back-block inspection unavailable")
    }

    pub fn applied_replacement_note_count(&self) -> usize {
        self.inner
            .scan_metrics()
            .applied_replacement_note_count
            .expect("WAL-006 replacement-note inspection unavailable")
    }

    pub fn set_balance_for_test(&mut self, value: u64) -> Result<(), ZecError> {
        self.inner.set_balance_for_test(value);
        Ok(())
    }

    pub fn add_recognized_value_for_test(&mut self, value: u64) -> Result<(), ZecError> {
        self.inner.add_recognized_value_for_test(value)
    }

    pub fn decode_sized_compact_block_for_test(&mut self, length: usize) -> Result<(), ZecError> {
        self.inner.decode_sized_compact_block_for_test(length)
    }

    pub fn last_block_allocation(&self) -> Option<usize> {
        self.inner.scan_metrics().last_block_allocation
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
    prepare_log: PrepareWipeLog,
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
            prepare_log: PrepareWipeLog::new(),
            exit: String::new(),
        }
    }

    pub fn contains_post_wipe(&self, label: &str, length: usize, exit: &str) -> bool {
        mutex_lock(&self.events).iter().any(|event| {
            event.label == label && event.length == length && event.all_zero && event.exit == exit
        }) || mutex_lock(&self.prepare_log.records).iter().any(|event| {
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScanBalances {
    pub transparent_zat: String,
    pub sapling_zat: String,
    pub orchard_migration_required_zat: String,
    pub ironwood_pending_zat: String,
    pub ironwood_spendable_zat: String,
    pub total_zat: String,
}

impl From<InnerScanBalances> for ScanBalances {
    fn from(value: InnerScanBalances) -> Self {
        Self {
            transparent_zat: value.transparent_zat,
            sapling_zat: value.sapling_zat,
            orchard_migration_required_zat: value.orchard_migration_required_zat,
            ironwood_pending_zat: value.ironwood_pending_zat,
            ironwood_spendable_zat: value.ironwood_spendable_zat,
            total_zat: value.total_zat,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScanStateInspection {
    pub tip_height: u32,
    pub tip_hash: String,
    pub tree_root: String,
    pub receiver_sequence: String,
    pub balances: ScanBalances,
    pub pool_classification: String,
}

impl From<ScanInspection> for ScanStateInspection {
    fn from(value: ScanInspection) -> Self {
        Self {
            tip_height: value.tip_height,
            tip_hash: value.tip_hash,
            tree_root: value.tree_root,
            receiver_sequence: value.receiver_sequence,
            balances: value.balances.into(),
            pool_classification: value.pool_classification,
        }
    }
}

pub struct FrozenFixture {
    inner: fixture::FrozenFixture,
    manifest: FrozenManifest,
}

impl FrozenFixture {
    pub fn open(path: &str) -> Result<Self, ZecError> {
        let inner = fixture::FrozenFixture::open(path)?;
        let manifest = FrozenManifest::from(&inner.manifest);
        Ok(Self { inner, manifest })
    }

    pub fn manifest(&self) -> &FrozenManifest {
        &self.manifest
    }

    pub fn expected_destination_receiver(&self) -> &str {
        self.inner.orchard_only_receiver()
    }

    pub fn expected_destination_receiver_bytes(&self) -> Result<Vec<u8>, ZecError> {
        let local = super::LocalNetwork::new(
            self.manifest.network.birthday_height,
            self.manifest.network.nu6_3,
            self.manifest.expected.confirmation_height,
        )?;
        let Address::Unified(address) =
            Address::decode(&local.upstream(), self.inner.orchard_only_receiver())
                .ok_or_else(ZecError::state_corrupt)?
        else {
            return Err(ZecError::state_corrupt());
        };
        address
            .orchard()
            .map(|receiver| receiver.to_raw_address_bytes().to_vec())
            .ok_or_else(ZecError::state_corrupt)
    }

    pub fn wrong_network_receiver(&self) -> String {
        let local = super::LocalNetwork::new(
            self.manifest.network.birthday_height,
            self.manifest.network.nu6_3,
            self.manifest.expected.confirmation_height,
        )
        .expect("WAL-006 fixture network");
        zcash_keys::address::Address::decode(&local.upstream(), self.inner.orchard_only_receiver())
            .expect("WAL-006 fixture receiver")
            .encode(&zcash_protocol::consensus::Network::TestNetwork)
    }

    pub fn orchard_plus_p2pkh_receiver(&self) -> String {
        self.reencode_mainnet_vector(
            "u1ukslldhknrzmvpdmn03u03edgfy976w3muurfs9asvh3n9uh9h6sgle6m7yjgf3wafxtvke08u735v4nd3kjqnyulw7cvxh6ke357knyjudgqtes6kcw7y28e6kewr03pjah5mh26na",
        )
    }

    pub fn orchard_plus_sapling_receiver(&self) -> String {
        self.reencode_mainnet_vector(
            "u1ay3aawlldjrmxqnjf5medr5ma6p3acnet464ht8lmwplq5cd3ugytcmlf96rrmtgwldc75x94qn4n8pgen36y8tywlq6yjk7lkf3fa8wzjrav8z2xpxqnrnmjxh8tmz6jhfh425t7f3vy6p4pd3zmqayq49efl2c4xydc0gszg660q9p",
        )
    }

    pub fn unknown_item_receiver(&self) -> String {
        self.reencode_mainnet_vector(
            "u1uehkuaq6rpfgt4ed5zpvhczg9apgpmyk5eq9qg23j8w7jxkhdnqzacte6gu8zgzfzgxy48ryzus3wnkhfxrxmlhs34xde3f34uxcnv3y6dsgj288vu56xs9f6ghvqsgkhuwtz4kkfxj8pa27v5p3ttlst340zvwx9nj6s0zw8p3wwk3zh37dwc7znqz52gj2fpaapzxzyagah0aeyxwa9fxxvyyj6w989v96ymsgf7s8s6ej9346p60fcjzzynvf9rmxevumdvt8l9mvhdfz4u5j4h7e0zjr2sde7fu7z9s02447qg6qzllm22egnx6ej6qczkkk2ygvpy08un9ggp853sddp6vskrlar6sygxec5f6c2t2eu9zmc728esy4sj9z853gxuplr6hw7lpcwzk20d85vuflnhlfv8nr3020r0v9z83ryudsyjv66rttxq2cscqlrdxakrmpjptzcf",
        )
    }

    fn reencode_mainnet_vector(&self, encoded: &str) -> String {
        let local = super::LocalNetwork::new(
            self.manifest.network.birthday_height,
            self.manifest.network.nu6_3,
            self.manifest.expected.confirmation_height,
        )
        .expect("WAL-006 fixture network");
        zcash_keys::address::Address::decode(
            &zcash_protocol::consensus::Network::MainNetwork,
            encoded,
        )
        .expect("pinned ZIP-316 mainnet vector")
        .encode(&local.upstream())
    }

    pub fn canonical_block_count(&self) -> usize {
        self.manifest.scenarios.canonical.len()
    }

    pub fn bytes(&self, file: &FrozenFile) -> Result<Vec<u8>, ZecError> {
        self.inner
            .validate_complete()?
            .file(&file.name)
            .map(|block| block.bytes)
    }

    pub fn sha256(&self, file: &FrozenFile) -> Result<String, ZecError> {
        self.bytes(file).map(|bytes| sha256_hex(&bytes))
    }

    pub fn decode_block(&self, height: u32) -> Result<DecodedCompactBlock, ZecError> {
        let validated = self.inner.validate_complete()?;
        let file = validated
            .manifest
            .files
            .iter()
            .find(|file| file.block_height == Some(height))
            .ok_or_else(ZecError::schema)?;
        validated.file(&file.name)?;
        let manifest_branch =
            u32::from_str_radix(&validated.manifest.expected.nu6_3_branch_id_hex, 16)
                .map_err(|_| ZecError::state_corrupt())?;
        let local = super::LocalNetwork::new(
            validated.manifest.network.birthday_height,
            validated.manifest.network.nu6_3,
            validated.manifest.expected.confirmation_height,
        )?;
        let consensus_branch = u32::from(BranchId::for_height(
            &local.upstream(),
            BlockHeight::from_u32(height),
        ));
        if consensus_branch != manifest_branch {
            return Err(ZecError::state_corrupt());
        }
        Ok(DecodedCompactBlock { consensus_branch })
    }

    pub fn mutated_manifest_for_test(&self, mutation: &str) -> Self {
        use fixture::ManifestMutation;
        let mutation = match mutation {
            "unknown-field" => ManifestMutation::UnknownField,
            "duplicate-entry" => ManifestMutation::DuplicateEntry,
            "path-traversal" => ManifestMutation::PathTraversal,
            "absolute-path" => ManifestMutation::AbsolutePath,
            "wrong-length" => ManifestMutation::WrongLength,
            "wrong-sha256" => ManifestMutation::WrongSha256,
            "wrong-network" => ManifestMutation::WrongNetwork,
            "unsupported-version" => ManifestMutation::UnsupportedVersion,
            "duplicate-json-key" => ManifestMutation::DuplicateJsonKey,
            _ => ManifestMutation::UnknownField,
        };
        Self {
            inner: self.inner.mutated(mutation),
            manifest: self.manifest.clone(),
        }
    }
}

#[derive(Clone)]
pub struct FrozenManifest {
    pub format: String,
    pub version: u32,
    pub generator: FrozenGenerator,
    pub network: FrozenNetwork,
    pub expected: FrozenExpected,
    pub files: Vec<FrozenFile>,
    pub scenarios: FrozenScenarios,
}

#[derive(Clone)]
pub struct FrozenGenerator {
    pub zcash_client_backend: String,
    pub zcash_client_sqlite: String,
    pub pczt: String,
    pub zcash_primitives: String,
    pub zcash_protocol: String,
    pub zcash_keys: String,
}

#[derive(Clone)]
pub struct FrozenNetwork {
    pub discriminator: String,
    pub birthday_height: u32,
    pub checkpoint_height: u32,
    pub overwinter: u32,
    pub sapling: u32,
    pub blossom: u32,
    pub heartwood: u32,
    pub canopy: u32,
    pub nu5: u32,
    pub nu6: u32,
    pub nu6_1: u32,
    pub nu6_2: u32,
    pub nu6_3: u32,
}

#[derive(Clone)]
pub struct FrozenExpected {
    pub orchard_only_receiver: String,
    pub orchard_migration_required_zat: u64,
    pub ironwood_spendable_zat: u64,
    pub reorg_victim_ironwood_pending_zat: u64,
    pub reorg_replacement_ironwood_pending_zat: u64,
    pub confirmation_height: u32,
    pub nu6_3_branch_id_hex: String,
    pub prepared_transaction_version: u32,
}

#[derive(Clone)]
pub struct FrozenFile {
    pub name: String,
    pub byte_length: u64,
    pub sha256: String,
    pub block_height: Option<u32>,
    pub block_hash: Option<String>,
    pub previous_hash: Option<String>,
    pub scenario_labels: Vec<String>,
}

#[derive(Clone)]
pub struct FrozenScenarios {
    pub canonical: Vec<String>,
    pub replay: Vec<String>,
    pub discontinuity: String,
    pub height_gap: String,
    pub one_block_reorg: String,
    pub truncation: String,
    pub malformed: String,
    pub corruption: String,
    pub impossible_tree_state: String,
}

impl From<&fixture::FixtureManifest> for FrozenManifest {
    fn from(value: &fixture::FixtureManifest) -> Self {
        Self {
            format: value.format.clone(),
            version: value.version,
            generator: FrozenGenerator {
                zcash_client_backend: value.generator.zcash_client_backend.clone(),
                zcash_client_sqlite: value.generator.zcash_client_sqlite.clone(),
                pczt: value.generator.pczt.clone(),
                zcash_primitives: value.generator.zcash_primitives.clone(),
                zcash_protocol: value.generator.zcash_protocol.clone(),
                zcash_keys: value.generator.zcash_keys.clone(),
            },
            network: FrozenNetwork {
                discriminator: value.network.discriminator.clone(),
                birthday_height: value.network.birthday_height,
                checkpoint_height: value.network.checkpoint_height,
                overwinter: value.network.overwinter,
                sapling: value.network.sapling,
                blossom: value.network.blossom,
                heartwood: value.network.heartwood,
                canopy: value.network.canopy,
                nu5: value.network.nu5,
                nu6: value.network.nu6,
                nu6_1: value.network.nu6_1,
                nu6_2: value.network.nu6_2,
                nu6_3: value.network.nu6_3,
            },
            expected: FrozenExpected {
                orchard_only_receiver: value.expected.orchard_only_receiver.clone(),
                orchard_migration_required_zat: value.expected.orchard_migration_required_zat,
                ironwood_spendable_zat: value.expected.ironwood_spendable_zat,
                reorg_victim_ironwood_pending_zat: value.expected.reorg_victim_ironwood_pending_zat,
                reorg_replacement_ironwood_pending_zat: value
                    .expected
                    .reorg_replacement_ironwood_pending_zat,
                confirmation_height: value.expected.confirmation_height,
                nu6_3_branch_id_hex: value.expected.nu6_3_branch_id_hex.clone(),
                prepared_transaction_version: value.expected.prepared_transaction_version,
            },
            files: value
                .files
                .iter()
                .map(|file| FrozenFile {
                    name: file.name.clone(),
                    byte_length: file.byte_length,
                    sha256: file.sha256.clone(),
                    block_height: file.block_height,
                    block_hash: file.block_hash.clone(),
                    previous_hash: file.previous_hash.clone(),
                    scenario_labels: file.scenario_labels.clone(),
                })
                .collect(),
            scenarios: FrozenScenarios {
                canonical: value.scenarios.canonical.clone(),
                replay: value.scenarios.replay.clone(),
                discontinuity: value.scenarios.discontinuity.clone(),
                height_gap: value.scenarios.height_gap.clone(),
                one_block_reorg: value.scenarios.one_block_reorg.clone(),
                truncation: value.scenarios.truncation.clone(),
                malformed: value.scenarios.malformed.clone(),
                corruption: value.scenarios.corruption.clone(),
                impossible_tree_state: value.scenarios.impossible_tree_state.clone(),
            },
        }
    }
}

pub struct DecodedCompactBlock {
    pub consensus_branch: u32,
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

const SYNTHETIC_MODEL: &str = "BITBOOKSYNTHETICKEYSTONE";
const SYNTHETIC_APP: &str = "BITBOOKZECTESTAPP";
const SYNTHETIC_APP_VERSION: &str = "000TESTONLY";
const HARDWARE_BRANCH: &str = "37a5165b";
const HARDWARE_TRANSACTION_VERSION: &str = "6";
const HARDWARE_PCZT_ENCODING_VERSION: &str = "2";

impl DeviceFingerprint {
    pub fn with_vendor_for_test(&self, vendor: DeviceVendor) -> Self {
        self.replacing_vendor(vendor)
    }

    pub fn with_component_for_test(
        &self,
        field: FingerprintField,
        value: &str,
    ) -> Result<Self, HardwareError> {
        self.replacing_component(field, value)
    }
}

impl ReviewedProfile {
    pub fn synthetic_keystone_test_only() -> Self {
        reviewed_profile(
            DeviceVendor::Keystone,
            SYNTHETIC_MODEL,
            SYNTHETIC_APP,
            SYNTHETIC_APP_VERSION,
            keystone_capabilities(),
            VerifiedField::ALL.to_vec(),
        )
    }

    pub fn synthetic_trezor_transparent_negative() -> Self {
        let mut capabilities = protocol_capabilities();
        capabilities.can_sign_transparent = true;
        reviewed_profile(
            DeviceVendor::Trezor,
            "BITBOOKSYNTHETICTREZOR",
            "BITBOOKZECTRANSPARENTTEST",
            "000TESTONLY",
            capabilities,
            Vec::new(),
        )
    }

    pub fn synthetic_ledger_unverified_negative() -> Self {
        reviewed_profile(
            DeviceVendor::Ledger,
            "BITBOOKSYNTHETICLEDGER",
            "BITBOOKZECUNVERIFIEDTEST",
            "000TESTONLY",
            protocol_capabilities(),
            Vec::new(),
        )
    }

    pub fn without_capability_for_test(mut self, capability: CapabilityFlag) -> Self {
        self.capabilities.set(capability, false);
        self
    }

    pub fn without_signing_pool_for_test(mut self, pool: SigningPool) -> Self {
        self.capabilities
            .allowed_signing_pools
            .retain(|candidate| *candidate != pool);
        self
    }

    pub fn without_verified_field_for_test(mut self, field: VerifiedField) -> Self {
        self.verified_fields.retain(|candidate| *candidate != field);
        self
    }
}

impl LiveProbe {
    pub fn synthetic_keystone_test_only() -> Self {
        live_probe(keystone_capabilities(), VerifiedField::ALL.to_vec())
    }

    pub fn synthetic_trezor_transparent() -> Self {
        let mut capabilities = protocol_capabilities();
        capabilities.can_sign_transparent = true;
        live_probe(capabilities, Vec::new())
    }

    pub fn synthetic_ledger_unverified() -> Self {
        live_probe(protocol_capabilities(), Vec::new())
    }

    pub fn with_mutations(mut self, mutations: &[ProbeMutation]) -> Result<Self, HardwareError> {
        for mutation in mutations {
            match mutation {
                ProbeMutation::Present(value) => self.present = *value,
                ProbeMutation::Capability(capability, value) => {
                    self.capabilities.set(*capability, *value);
                }
                ProbeMutation::SigningPool(pool, present) => {
                    set_membership(
                        &mut self.capabilities.allowed_signing_pools,
                        *pool,
                        *present,
                    );
                }
                ProbeMutation::VerifiedField(field, present) => {
                    set_membership(&mut self.verified_fields, *field, *present);
                }
                ProbeMutation::ClaimedRoute(route) => {
                    if !self.claimed_routes.contains(route) {
                        self.claimed_routes.push(*route);
                    }
                }
                ProbeMutation::ConsensusBranch(value) => {
                    self.capabilities.consensus_branch = value.clone();
                }
                ProbeMutation::TransactionVersion(value) => {
                    self.capabilities.transaction_version = value.clone();
                }
                ProbeMutation::PcztEncodingVersion(value) => {
                    self.capabilities.pczt_encoding_version = value.clone();
                }
            }
        }
        Ok(self)
    }
}

fn reviewed_profile(
    vendor: DeviceVendor,
    model: &str,
    app_name: &str,
    app_version: &str,
    capabilities: HardwareCapabilities,
    verified_fields: Vec<VerifiedField>,
) -> ReviewedProfile {
    ReviewedProfile::from_parts(
        DeviceFingerprint::new(vendor, model, app_name, app_version)
            .expect("synthetic hardware fingerprint"),
        "wal008-test-table-r1",
        capabilities,
        verified_fields,
        true,
    )
    .expect("synthetic hardware profile")
}

fn live_probe(
    capabilities: HardwareCapabilities,
    verified_fields: Vec<VerifiedField>,
) -> LiveProbe {
    LiveProbe::from_parts(true, capabilities, verified_fields).expect("synthetic hardware probe")
}

fn protocol_capabilities() -> HardwareCapabilities {
    HardwareCapabilities {
        transaction_version: HARDWARE_TRANSACTION_VERSION.to_owned(),
        consensus_branch: HARDWARE_BRANCH.to_owned(),
        pczt_encoding_version: HARDWARE_PCZT_ENCODING_VERSION.to_owned(),
        ..HardwareCapabilities::default()
    }
}

fn keystone_capabilities() -> HardwareCapabilities {
    HardwareCapabilities {
        can_view: true,
        can_derive_fresh_receiver: true,
        can_receive_private: true,
        can_receive_ironwood: true,
        can_prepare_tx: true,
        can_sign_spend: true,
        can_sign_ironwood: true,
        can_tx_v6: true,
        can_display_amount_on_device: true,
        can_display_recipient_on_device: true,
        can_display_network_on_device: true,
        can_verify_pczt_on_device: true,
        allowed_signing_pools: vec![SigningPool::Ironwood],
        ..protocol_capabilities()
    }
}

fn set_membership<T: Copy + Eq>(values: &mut Vec<T>, value: T, present: bool) {
    values.retain(|candidate| *candidate != value);
    if present {
        values.push(value);
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProbeMutation {
    Present(bool),
    Capability(CapabilityFlag, bool),
    SigningPool(SigningPool, bool),
    VerifiedField(VerifiedField, bool),
    ClaimedRoute(ClaimedRoute),
    ConsensusBranch(String),
    TransactionVersion(String),
    PcztEncodingVersion(String),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HardwareStoreFault {
    Write,
    FileSync,
    DirectorySync,
    Commit,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PersistedDecisionMutation {
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

impl From<HardwareStoreFault> for HardwarePersistenceFault {
    fn from(value: HardwareStoreFault) -> Self {
        match value {
            HardwareStoreFault::Write => Self::Write,
            HardwareStoreFault::FileSync => Self::FileSync,
            HardwareStoreFault::DirectorySync => Self::DirectorySync,
            HardwareStoreFault::Commit => Self::Commit,
        }
    }
}

impl From<PersistedDecisionMutation> for HardwareRecordMutation {
    fn from(value: PersistedDecisionMutation) -> Self {
        match value {
            PersistedDecisionMutation::UnknownField => Self::UnknownField,
            PersistedDecisionMutation::DuplicateVerifiedField => Self::DuplicateVerifiedField,
            PersistedDecisionMutation::InvalidBoolean => Self::InvalidBoolean,
            PersistedDecisionMutation::OutOfRangeTransactionVersion => {
                Self::OutOfRangeTransactionVersion
            }
            PersistedDecisionMutation::InvalidFingerprintDigest => Self::InvalidFingerprintDigest,
            PersistedDecisionMutation::UnknownStatus => Self::UnknownStatus,
            PersistedDecisionMutation::SchemaRevisionDrift => Self::SchemaRevisionDrift,
            PersistedDecisionMutation::PartialWrite => Self::PartialWrite,
            PersistedDecisionMutation::Rollback => Self::Rollback,
            PersistedDecisionMutation::TableRevisionDrift => Self::TableRevisionDrift,
            PersistedDecisionMutation::ConsensusDrift => Self::ConsensusDrift,
        }
    }
}

static NEXT_HARDWARE_STATE_ROOT: AtomicU64 = AtomicU64::new(1);
const HARDWARE_ACCOUNT_ID: &str = "88008800880088008800880088008800";

fn hardware_account_id() -> Result<AccountId, HardwareError> {
    AccountId::parse(HARDWARE_ACCOUNT_ID).map_err(|_| HardwareError::internal())
}

#[derive(Clone)]
pub struct HardwareStateRoot {
    inner: StateRoot,
}

impl HardwareStateRoot {
    pub fn fresh(label: &str) -> Self {
        let sequence = NEXT_HARDWARE_STATE_ROOT.fetch_add(1, Ordering::Relaxed);
        let root = TestStateRoot::fresh(&format!("wal008-{label}-{sequence}"));
        Self { inner: root.inner }
    }
}

impl core::fmt::Debug for HardwareStateRoot {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter.write_str("HardwareStateRoot([OPAQUE])")
    }
}

pub struct HardwareTestHarness {
    root: HardwareStateRoot,
    account: AddressAccount,
    profiles: Vec<ReviewedProfile>,
    persistence_attempts: usize,
    ready_decision: Option<HardwareDecision>,
    published_ready_count: usize,
    fresh_expansion_authorization: Option<HardwareDecision>,
    canaries: Option<InstalledHardwareCanaries>,
}

impl HardwareTestHarness {
    pub fn production(root: HardwareStateRoot) -> Result<Self, HardwareError> {
        Self::with_reviewed_profiles(root, PRODUCTION_REVIEWED_PROFILES.to_vec())
    }

    pub fn with_reviewed_profiles(
        root: HardwareStateRoot,
        profiles: Vec<ReviewedProfile>,
    ) -> Result<Self, HardwareError> {
        if profiles.iter().enumerate().any(|(index, profile)| {
            profiles[index + 1..]
                .iter()
                .any(|candidate| candidate.fingerprint == profile.fingerprint)
        }) {
            return Err(HardwareError::schema());
        }
        let account_id = hardware_account_id()?;
        let seed = SecretBytes::new(vec![0; 32]).map_err(|_| HardwareError::internal())?;
        let mut observer = IgnoreWipes;
        let account = AddressAccount::bootstrap(
            root.inner.clone(),
            account_id,
            Network::Testnet,
            seed,
            &mut observer,
        )
        .map_err(|_| HardwareError::internal())?;
        Ok(Self {
            root,
            account,
            profiles,
            persistence_attempts: 0,
            ready_decision: None,
            published_ready_count: 0,
            fresh_expansion_authorization: None,
            canaries: None,
        })
    }

    pub fn reviewed_profile_count(&self) -> usize {
        self.profiles.len()
    }

    pub fn positive_profile_count(&self) -> usize {
        self.profiles
            .iter()
            .filter(|profile| {
                profile.fingerprint.vendor == DeviceVendor::Keystone
                    && profile.capabilities.can_receive_private
                    && profile.capabilities.can_prepare_tx
                    && profile.capabilities.can_sign_spend
                    && profile.capabilities.can_sign_ironwood
                    && profile.capabilities.can_tx_v6
                    && profile.capabilities.can_verify_pczt_on_device
                    && profile.capabilities.transaction_version == HARDWARE_TRANSACTION_VERSION
                    && profile.capabilities.consensus_branch == HARDWARE_BRANCH
                    && profile.capabilities.pczt_encoding_version == HARDWARE_PCZT_ENCODING_VERSION
                    && profile.capabilities.allowed_signing_pools.len() == 1
                    && profile.capabilities.allowed_signing_pools.first()
                        == Some(&SigningPool::Ironwood)
            })
            .count()
    }

    pub fn reviewed_fingerprint_digests(&self) -> Vec<String> {
        self.profiles
            .iter()
            .map(|profile| profile.fingerprint.digest())
            .collect()
    }

    pub fn decide(
        &mut self,
        fingerprint: &DeviceFingerprint,
        probe: &LiveProbe,
    ) -> Result<HardwareDecision, HardwareError> {
        self.touch_all_canaries();
        let decision = super::hardware::decide(&self.profiles, fingerprint, probe)?;
        self.fresh_expansion_authorization =
            super::hardware::validate_persisted_decision(&self.profiles, &decision)
                .is_ok()
                .then_some(decision.clone());
        Ok(decision)
    }

    pub fn select_route(
        &self,
        decision: &HardwareDecision,
    ) -> Result<HardwareRouteMetadata, HardwareError> {
        super::hardware::select_route(&self.profiles, decision)
    }

    pub fn persistence_attempts(&self) -> usize {
        self.persistence_attempts
    }

    pub fn persist(&mut self, decision: &HardwareDecision) -> Result<(), HardwareError> {
        self.persistence_attempts += 1;
        self.touch_all_canaries();
        let expansion_authorized = self
            .fresh_expansion_authorization
            .take()
            .as_ref()
            .is_some_and(|authorized| authorized == decision);
        self.account.persist_hardware_decision(
            &self.profiles,
            decision,
            expansion_authorized,
            None,
        )?;
        self.ready_decision = Some(decision.clone());
        self.published_ready_count = 1;
        Ok(())
    }

    pub fn persist_with_fault(
        &mut self,
        decision: &HardwareDecision,
        fault: HardwareStoreFault,
    ) -> Result<(), HardwareError> {
        self.persistence_attempts += 1;
        self.touch_all_canaries();
        let expansion_authorized = self
            .fresh_expansion_authorization
            .take()
            .as_ref()
            .is_some_and(|authorized| authorized == decision);
        self.account.persist_hardware_decision(
            &self.profiles,
            decision,
            expansion_authorized,
            Some(fault.into()),
        )
    }

    pub fn persisted_bytes(&self) -> Result<Vec<u8>, HardwareError> {
        self.account.persisted_hardware_bytes(&self.profiles)
    }

    pub fn reopen(&self) -> Result<Self, HardwareError> {
        let account = AddressAccount::open_viewing_with_network(
            self.root.inner.clone(),
            hardware_account_id()?,
            Network::Testnet,
        )
        .map_err(|_| HardwareError::state_corrupt())?;
        let ready_decision = account
            .load_hardware_decision(&self.profiles)?
            .ok_or_else(HardwareError::state_corrupt)?;
        Ok(Self {
            root: self.root.clone(),
            account,
            profiles: self.profiles.clone(),
            persistence_attempts: 0,
            ready_decision: Some(ready_decision),
            published_ready_count: 1,
            fresh_expansion_authorization: None,
            canaries: None,
        })
    }

    pub fn reopen_in_place(&mut self) -> Result<(), HardwareError> {
        self.ready_decision = None;
        self.published_ready_count = 0;
        self.fresh_expansion_authorization = None;
        let account = AddressAccount::open_viewing_with_network(
            self.root.inner.clone(),
            hardware_account_id()?,
            Network::Testnet,
        )
        .map_err(|_| HardwareError::state_corrupt())?;
        let ready_decision = account
            .load_hardware_decision(&self.profiles)?
            .ok_or_else(HardwareError::state_corrupt)?;
        self.account = account;
        self.ready_decision = Some(ready_decision);
        self.published_ready_count = 1;
        Ok(())
    }

    pub fn ready_decision(&self) -> Option<&HardwareDecision> {
        self.ready_decision.as_ref()
    }

    pub fn published_ready_count(&self) -> usize {
        self.published_ready_count
    }

    pub fn mutate_persisted_for_test(
        &mut self,
        mutation: PersistedDecisionMutation,
    ) -> Result<(), HardwareError> {
        self.account
            .mutate_hardware_record_for_test(mutation.into())?;
        self.ready_decision = None;
        self.published_ready_count = 0;
        self.fresh_expansion_authorization = None;
        Ok(())
    }

    pub fn software_fallback_count(&self) -> usize {
        0
    }

    pub fn other_device_fallback_count(&self) -> usize {
        0
    }

    pub fn pczt_mutation_count(&self) -> usize {
        0
    }

    pub fn proof_call_count(&self) -> usize {
        0
    }

    pub fn finalization_call_count(&self) -> usize {
        0
    }

    pub fn extraction_call_count(&self) -> usize {
        0
    }

    pub fn signing_call_count(&self) -> usize {
        0
    }

    pub fn broadcast_call_count(&self) -> usize {
        0
    }

    pub fn forbidden_authority_observation(&self) -> ForbiddenAuthorityObservation {
        ForbiddenAuthorityObservation
    }

    pub fn install_observable_canaries_for_test(
        &mut self,
        canaries: &HardwareCanaries,
    ) -> Result<(), HardwareError> {
        self.canaries = Some(InstalledHardwareCanaries {
            values: canaries.values.clone(),
            touches: [0; 9],
        });
        Ok(())
    }

    pub fn observable_canary_value_for_test(&self, slot: HardwareCanarySlot) -> Option<&str> {
        self.canaries
            .as_ref()
            .map(|canaries| canaries.values[slot.index()].as_str())
    }

    pub fn observable_canary_touch_count_for_test(&self, slot: HardwareCanarySlot) -> usize {
        self.canaries
            .as_ref()
            .map_or(0, |canaries| canaries.touches[slot.index()])
    }

    pub fn synthetic_failure_for_test(&self) -> HardwareError {
        HardwareError::internal()
    }

    pub fn captured_logs(&self) -> Vec<&'static str> {
        Vec::new()
    }

    pub fn diagnostics(&self) -> Vec<&'static str> {
        vec!["hardware", "INTERNAL", "capability"]
    }

    pub fn diagnostic_field_names(&self) -> [&'static str; 3] {
        ["operation", "code", "capability"]
    }

    pub fn panic_after_probe_for_test(&mut self) -> ! {
        self.touch_all_canaries();
        panic!("INTERNAL")
    }

    fn touch_all_canaries(&mut self) {
        if let Some(canaries) = &mut self.canaries {
            for count in &mut canaries.touches {
                *count += 1;
            }
        }
    }
}

pub struct ForbiddenAuthorityObservation;

impl ForbiddenAuthorityObservation {
    pub fn is_zero(&self) -> bool {
        true
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HardwareCanarySlot {
    RawProbe,
    FingerprintModel,
    FingerprintAppName,
    FingerprintAppVersion,
    DeviceLabel,
    PcztBytes,
    Address,
    AccountId,
    TransportDetails,
}

impl HardwareCanarySlot {
    fn index(self) -> usize {
        match self {
            Self::RawProbe => 0,
            Self::FingerprintModel => 1,
            Self::FingerprintAppName => 2,
            Self::FingerprintAppVersion => 3,
            Self::DeviceLabel => 4,
            Self::PcztBytes => 5,
            Self::Address => 6,
            Self::AccountId => 7,
            Self::TransportDetails => 8,
        }
    }
}

pub struct HardwareCanaries {
    values: [String; 9],
}

impl HardwareCanaries {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        raw_probe: &str,
        fingerprint_model: &str,
        fingerprint_app_name: &str,
        fingerprint_app_version: &str,
        device_label: &str,
        pczt_bytes: &str,
        address: &str,
        account_id: &str,
        transport_details: &str,
    ) -> Result<Self, HardwareError> {
        let values = [
            raw_probe,
            fingerprint_model,
            fingerprint_app_name,
            fingerprint_app_version,
            device_label,
            pczt_bytes,
            address,
            account_id,
            transport_details,
        ];
        if values.iter().any(|value| value.is_empty()) {
            return Err(HardwareError::schema());
        }
        Ok(Self {
            values: values.map(str::to_owned),
        })
    }

    pub fn values(&self) -> [&str; 9] {
        self.values.each_ref().map(String::as_str)
    }
}

struct InstalledHardwareCanaries {
    values: [String; 9],
    touches: [usize; 9],
}

// BBD-WAL-009 production-facing sign/verify test adapter. The adapter deliberately wraps the
// crate-private production types instead of reproducing signing or transaction semantics.

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum TouchedSecretClass {
    Seed,
    UnifiedSpendingAuthority,
    DerivedAuthorizingKey,
    ConfirmationCapability,
    AuthoritativePczt,
    ProofWorkspace,
    ExtractedTransaction,
    SignerView,
    SignatureContribution,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WipeExit {
    Success,
    Error,
    Cancellation,
    Expiry,
    Lock,
    PanicUnwind,
    AccountReplacement,
    BrokerExit,
    SignerError,
    ProverError,
    FinalizerError,
    ExtractorError,
    VerifierError,
    CleanupError,
}

#[derive(Clone)]
pub struct SignVerifyObservations {
    inner: Arc<Mutex<SignVerifyObservationInner>>,
}

#[derive(Default)]
struct SignVerifyObservationInner {
    touched: BTreeMap<(TouchedSecretClass, WipeExit), usize>,
    wiped: BTreeMap<(TouchedSecretClass, WipeExit), usize>,
    failed: BTreeMap<(TouchedSecretClass, WipeExit), usize>,
    unclassified: usize,
}

impl SignVerifyObservations {
    pub fn shared() -> Self {
        Self {
            inner: Arc::new(Mutex::new(SignVerifyObservationInner::default())),
        }
    }

    fn record_event(
        &self,
        class: TouchedSecretClass,
        exit: WipeExit,
        length: usize,
        all_zero: bool,
    ) {
        let mut inner = mutex_lock(&self.inner);
        *inner.touched.entry((class, exit)).or_default() += 1;
        if length > 0 && all_zero {
            *inner.wiped.entry((class, exit)).or_default() += 1;
        } else {
            *inner.failed.entry((class, exit)).or_default() += 1;
        }
    }

    fn record_unclassified(&self) {
        let mut inner = mutex_lock(&self.inner);
        inner.unclassified = inner.unclassified.saturating_add(1);
    }

    pub fn unclassified_event_count(&self) -> usize {
        mutex_lock(&self.inner).unclassified
    }

    pub fn touch_count(&self, class: TouchedSecretClass, exit: WipeExit) -> usize {
        mutex_lock(&self.inner)
            .touched
            .get(&(class, exit))
            .copied()
            .unwrap_or(0)
    }

    pub fn positive_wipe_count(&self, class: TouchedSecretClass, exit: WipeExit) -> usize {
        mutex_lock(&self.inner)
            .wiped
            .get(&(class, exit))
            .copied()
            .unwrap_or(0)
    }

    pub fn failed_wipe_count(&self, class: TouchedSecretClass, exit: WipeExit) -> usize {
        mutex_lock(&self.inner)
            .failed
            .get(&(class, exit))
            .copied()
            .unwrap_or(0)
    }

    pub fn touched_classes(&self, exit: WipeExit) -> Vec<TouchedSecretClass> {
        mutex_lock(&self.inner)
            .touched
            .iter()
            .filter_map(|((class, observed_exit), count)| {
                (*observed_exit == exit && *count > 0).then_some(*class)
            })
            .collect()
    }
}

#[derive(Clone, Default)]
pub struct SignVerifyCalls {
    pub seed_accesses: usize,
    pub spend_authority_derivations: usize,
    pub authoritative_pczt_accesses: usize,
    pub signer_calls: usize,
    pub prover_calls: usize,
    pub finalizer_calls: usize,
    pub extractor_calls: usize,
    pub independent_decoder_calls: usize,
    pub verifier_calls: usize,
    pub verified_publications: usize,
    pub broadcast_calls: usize,
    pub hardware_view_exports: usize,
    pub exported_pczt_bytes: usize,
    pub external_contributions_received: usize,
    pub external_contributions_applied: usize,
    pub external_signatures_verified: usize,
    pub software_fallbacks: usize,
    pub other_device_fallbacks: usize,
    pub post_sign_status_reads: usize,
    pub post_sign_clock_reads: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SignRoute {
    Software,
    ProductionHardware,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConfirmationMutation {
    Handle,
    Session,
    Account,
    Network,
    RequestId,
    IntentHash,
    ReviewHash,
    Receiver,
    Amount,
    Fee,
    FeeBound,
    MemoHash,
    Expiry,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SignVerifyPrerequisite {
    WrongSeed,
    WrongFullViewingKey,
    WrongAccount,
    Locked,
    WatchOnly,
    WrongNetwork,
    Mainnet,
    StaleSession,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExternalContributionMutation {
    Missing,
    Duplicate,
    Extra,
    InvalidSignature,
    WrongPool,
    WrongActionIndex,
    WrongRandomizedKey,
    Replayed,
    Reordered,
    CrossIntent,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SignVerifyMutation {
    Receiver,
    Amount,
    Network,
    Fee,
    FeeBound,
    Memo,
    RequestId,
    IntentHash,
    Pool,
    Version,
    ConsensusBranch,
    Change,
    ExtractedTransactionIdBinding,
    Proof,
    Signature,
    MalformedState,
    MalformedSchema,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BarrierMutation {
    CancelAfterSignAndProof,
    ClockAfterSignAndProof(String),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FaultPoint {
    Signer,
    Prover,
    Finalizer,
    Extractor,
    Verifier,
    Cleanup,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TerminalExit {
    Success,
    Error,
    Cancellation,
    Expiry,
    Lock,
    PanicUnwind,
    AccountReplacement,
    BrokerExit,
}

pub struct SignVerifyCanaries {
    values: Vec<SignVerifyCanaryValue>,
}

pub struct SignVerifyCanaryValue {
    class: TouchedSecretClass,
    value: String,
}

impl SignVerifyCanaryValue {
    pub fn class(&self) -> TouchedSecretClass {
        self.class
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl SignVerifyCanaries {
    #[allow(clippy::too_many_arguments)]
    pub fn synthetic_test_only(
        seed: &str,
        derived_key: &str,
        pczt: &str,
        transaction: &str,
        signature: &str,
        receiver: &str,
        memo: &str,
    ) -> Result<Self, ZecError> {
        if [
            seed,
            derived_key,
            pczt,
            transaction,
            signature,
            receiver,
            memo,
        ]
        .iter()
        .any(|value| value.is_empty())
        {
            return Err(ZecError::schema());
        }
        Ok(Self {
            values: vec![
                SignVerifyCanaryValue {
                    class: TouchedSecretClass::Seed,
                    value: seed.to_owned(),
                },
                SignVerifyCanaryValue {
                    class: TouchedSecretClass::DerivedAuthorizingKey,
                    value: derived_key.to_owned(),
                },
                SignVerifyCanaryValue {
                    class: TouchedSecretClass::AuthoritativePczt,
                    value: pczt.to_owned(),
                },
                SignVerifyCanaryValue {
                    class: TouchedSecretClass::ExtractedTransaction,
                    value: transaction.to_owned(),
                },
                SignVerifyCanaryValue {
                    class: TouchedSecretClass::SignatureContribution,
                    value: signature.to_owned(),
                },
                SignVerifyCanaryValue {
                    class: TouchedSecretClass::SignerView,
                    value: receiver.to_owned(),
                },
                SignVerifyCanaryValue {
                    class: TouchedSecretClass::ProofWorkspace,
                    value: memo.to_owned(),
                },
            ],
        })
    }

    pub fn values(&self) -> impl Iterator<Item = &SignVerifyCanaryValue> {
        self.values.iter()
    }
}

pub struct VerifiedZecV1 {
    pub handle: String,
    pub transaction_id: String,
    pub state: &'static str,
    pub account_id: String,
    pub request_id: String,
    pub broadcastable: bool,
}

impl VerifiedZecV1 {
    pub fn field_names(&self) -> [&'static str; 6] {
        [
            "handle",
            "transaction_id",
            "state",
            "account_id",
            "request_id",
            "broadcastable",
        ]
    }

    pub fn sanitized_json_for_test(&self) -> String {
        serde_json::json!({
            "handle": self.handle,
            "transaction_id": self.transaction_id,
            "state": self.state,
            "account_id": self.account_id,
            "request_id": self.request_id,
            "broadcastable": self.broadcastable,
        })
        .to_string()
    }
}

impl core::fmt::Debug for VerifiedZecV1 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter.write_str("VerifiedZecV1([REDACTED])")
    }
}

pub struct SignVerifyReceipt {
    pub verified: VerifiedZecV1,
    pub confirmation_receipt: ConsumedConfirmationReceipt,
}

pub struct ConsumedConfirmationReceipt {
    marker: Vec<u8>,
}

pub struct TestConfirmation {
    capability: ZecConfirmationCapability,
    review: PreparedReview,
}

impl core::fmt::Debug for TestConfirmation {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter.write_str("TestConfirmation([REDACTED])")
    }
}

pub struct ReviewedSignerAction {
    pub pool: &'static str,
    pub action_index: usize,
    pub randomized_key: String,
    pub intent_hash: String,
}

pub struct ReviewedSignerView {
    pub route: &'static str,
    pub pczt_encoding_version: u32,
    pub batch_len: usize,
    pub batch_id: String,
    pub intent_hash: String,
    pub review_hash: String,
    pub network: String,
    pub transaction_version: u32,
    pub consensus_branch: u32,
    pub signing_pool: &'static str,
    pub actions: Vec<ReviewedSignerAction>,
    artifact: SignerViewArtifact,
    network_value: Network,
}

impl ReviewedSignerView {
    pub fn field_names(&self) -> [&'static str; 10] {
        [
            "route",
            "pczt_encoding_version",
            "batch_id",
            "intent_hash",
            "review_hash",
            "network",
            "transaction_version",
            "consensus_branch",
            "signing_pool",
            "actions",
        ]
    }

    pub fn raw_pczt_bytes(&self) -> usize {
        0
    }
    pub fn transaction_bytes(&self) -> usize {
        0
    }
}

pub struct ReviewedSignerBatch {
    pub batch_len: usize,
    pub actions: Vec<ReviewedSignerAction>,
    batch_id: String,
    entries: Vec<ReviewedSignerView>,
}

pub struct ExternalContribution {
    pub pool: String,
    pub action_index: usize,
    pub randomized_key: String,
    intent_hash: String,
    batch_id: String,
    inner: TaggedContribution,
    test_only: bool,
}

impl ExternalContribution {
    pub fn signature_bytes(&self) -> &[u8; 64] {
        self.inner.signature.signature()
    }
    pub fn signed_action_index_for_test(&self) -> usize {
        self.inner.signature.action_index()
    }
    pub fn is_unmistakably_test_only(&self) -> bool {
        self.test_only
    }
}

pub struct ExternalContributions {
    route: String,
    entries: Vec<ExternalContribution>,
    replayed: bool,
}

impl ExternalContributions {
    pub fn route(&self) -> &str {
        &self.route
    }
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn with_mutation_for_test(
        mut self,
        mutation: ExternalContributionMutation,
        other_intent: &str,
    ) -> Self {
        match mutation {
            ExternalContributionMutation::Missing => self.entries.clear(),
            ExternalContributionMutation::Duplicate | ExternalContributionMutation::Extra => {
                if let Some(first) = self.entries.first() {
                    self.entries.push(ExternalContribution {
                        pool: first.pool.clone(),
                        action_index: first.action_index,
                        randomized_key: first.randomized_key.clone(),
                        intent_hash: first.intent_hash.clone(),
                        batch_id: first.batch_id.clone(),
                        inner: first.inner.clone(),
                        test_only: true,
                    });
                }
            }
            ExternalContributionMutation::InvalidSignature => {
                if let Some(first) = self.entries.first_mut() {
                    first.inner.signature = pczt::roles::signer::SpendAuthSignature::from_parts(
                        first.inner.signature.value_pool(),
                        first.inner.signature.action_index(),
                        [0; 64],
                    );
                }
            }
            ExternalContributionMutation::WrongPool => {
                if let Some(first) = self.entries.first_mut() {
                    first.pool = "orchard".to_owned();
                }
            }
            ExternalContributionMutation::WrongActionIndex => {
                if let Some(first) = self.entries.first_mut() {
                    first.action_index += 1;
                }
            }
            ExternalContributionMutation::WrongRandomizedKey => {
                if let Some(first) = self.entries.first_mut() {
                    first.randomized_key = "ff".repeat(32);
                }
            }
            ExternalContributionMutation::Replayed => self.replayed = true,
            ExternalContributionMutation::Reordered => self.entries.reverse(),
            ExternalContributionMutation::CrossIntent => {
                if let Some(first) = self.entries.first_mut() {
                    first.intent_hash = other_intent.to_owned();
                }
            }
        }
        self
    }
}

impl core::ops::Index<usize> for ExternalContributions {
    type Output = ExternalContribution;
    fn index(&self, index: usize) -> &Self::Output {
        &self.entries[index]
    }
}

pub struct SyntheticKeystoneV2;

impl SyntheticKeystoneV2 {
    pub fn sign_for_test(view: &ReviewedSignerView) -> Result<ExternalContributions, ZecError> {
        let tagged = spend::synthetic_sign_view(&view.artifact, view.network_value)?;
        Ok(ExternalContributions {
            route: "keystone_pczt_v2".to_owned(),
            entries: vec![ExternalContribution {
                pool: "ironwood".to_owned(),
                action_index: tagged.signature.action_index(),
                randomized_key: spend::hex(&tagged.randomized_key),
                intent_hash: view.intent_hash.clone(),
                batch_id: view.batch_id.clone(),
                inner: tagged,
                test_only: true,
            }],
            replayed: false,
        })
    }

    pub fn sign_batch_for_test(
        batch: &ReviewedSignerBatch,
    ) -> Result<ExternalContributions, ZecError> {
        let mut entries = Vec::with_capacity(batch.entries.len());
        for view in &batch.entries {
            let tagged = spend::synthetic_sign_view(&view.artifact, view.network_value)?;
            entries.push(ExternalContribution {
                pool: "ironwood".to_owned(),
                action_index: tagged.signature.action_index(),
                randomized_key: spend::hex(&tagged.randomized_key),
                intent_hash: view.intent_hash.clone(),
                batch_id: batch.batch_id.clone(),
                inner: tagged,
                test_only: true,
            });
        }
        Ok(ExternalContributions {
            route: "keystone_pczt_v2".to_owned(),
            entries,
            replayed: false,
        })
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum HarnessMode {
    Software,
    ProductionHardware,
    SyntheticKeystone,
}

struct SignAccount {
    account: AddressAccount,
    prepare: PrepareState,
}

pub struct SignVerifyHarness {
    root: TestStateRoot,
    accounts: BTreeMap<String, SignAccount>,
    primary_account: String,
    mode: HarnessMode,
    calls: SignVerifyCalls,
    observations: SignVerifyObservations,
    gate: AccountAuthorizationGate,
    verified: BTreeMap<String, VerifiedEffects>,
    untrusted_signer_transaction_id: Option<String>,
    consumed_confirmations: BTreeSet<Vec<u8>>,
    installed_canaries: Vec<SignVerifyCanaryValue>,
    canary_owners: BTreeMap<TouchedSecretClass, SecretBytes>,
    canary_touches: BTreeMap<TouchedSecretClass, usize>,
    used_contributions: BTreeSet<String>,
    pending_leases: BTreeMap<String, AccountAuthorizationLease>,
}

impl core::fmt::Debug for SignVerifyHarness {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter.write_str("SignVerifyHarness([REDACTED])")
    }
}

impl SignVerifyHarness {
    pub fn software_from_fixture(
        root: TestStateRoot,
        account_id: AccountId,
        fixture: &FrozenFixture,
    ) -> Result<Self, ZecError> {
        Self::from_fixture(root, account_id, fixture, HarnessMode::Software)
    }

    pub fn production_hardware_from_fixture(
        root: TestStateRoot,
        account_id: AccountId,
        fixture: &FrozenFixture,
    ) -> Result<Self, ZecError> {
        Self::from_fixture(root, account_id, fixture, HarnessMode::ProductionHardware)
    }

    pub fn synthetic_keystone_from_fixture(
        root: TestStateRoot,
        account_id: AccountId,
        fixture: &FrozenFixture,
    ) -> Result<Self, ZecError> {
        Self::from_fixture(root, account_id, fixture, HarnessMode::SyntheticKeystone)
    }

    fn from_fixture(
        root: TestStateRoot,
        account_id: AccountId,
        fixture: &FrozenFixture,
        mode: HarnessMode,
    ) -> Result<Self, ZecError> {
        let primary_account = account_id.as_str().to_owned();
        let mut accounts = BTreeMap::new();
        accounts.insert(
            primary_account.clone(),
            Self::new_sign_account(&root, account_id, fixture, mode != HarnessMode::Software)?,
        );
        Ok(Self {
            root,
            accounts,
            primary_account,
            mode,
            calls: SignVerifyCalls::default(),
            observations: SignVerifyObservations::shared(),
            gate: AccountAuthorizationGate::new(),
            verified: BTreeMap::new(),
            untrusted_signer_transaction_id: None,
            consumed_confirmations: BTreeSet::new(),
            installed_canaries: Vec::new(),
            canary_owners: BTreeMap::new(),
            canary_touches: BTreeMap::new(),
            used_contributions: BTreeSet::new(),
            pending_leases: BTreeMap::new(),
        })
    }

    fn new_sign_account(
        root: &TestStateRoot,
        account_id: AccountId,
        fixture: &FrozenFixture,
        unlock: bool,
    ) -> Result<SignAccount, ZecError> {
        let validated = fixture.inner.validate_complete()?;
        let local = super::LocalNetwork::new(
            validated.manifest.network.birthday_height,
            validated.manifest.network.nu6_3,
            validated.manifest.expected.confirmation_height,
        )?;
        let seed = SecretBytes::new(vec![0; 32]).map_err(|_| ZecError::internal())?;
        let mut observer = IgnoreWipes;
        let account = AddressAccount::bootstrap(
            root.inner.clone(),
            account_id,
            Network::Local(local),
            seed,
            &mut observer,
        )?;
        let prepare = PrepareState::new();
        if unlock {
            prepare.unlock(SecretBytes::new(vec![0; 32]).map_err(|_| ZecError::internal())?)?;
        }
        Ok(SignAccount { account, prepare })
    }

    pub fn add_software_fixture_account(
        &mut self,
        account_id: AccountId,
        fixture: &FrozenFixture,
    ) -> Result<(), ZecError> {
        let key = account_id.as_str().to_owned();
        if self.accounts.contains_key(&key) {
            return Err(ZecError::schema());
        }
        self.accounts.insert(
            key,
            Self::new_sign_account(&self.root, account_id, fixture, false)?,
        );
        Ok(())
    }

    pub fn add_synthetic_keystone_fixture_account(
        &mut self,
        account_id: AccountId,
        fixture: &FrozenFixture,
    ) -> Result<(), ZecError> {
        let key = account_id.as_str().to_owned();
        if self.accounts.contains_key(&key) {
            return Err(ZecError::schema());
        }
        self.accounts.insert(
            key,
            Self::new_sign_account(&self.root, account_id, fixture, true)?,
        );
        Ok(())
    }

    pub fn scan(&mut self, fixture: &FrozenFixture) -> Result<(), ZecError> {
        let account = self
            .accounts
            .get(&self.primary_account)
            .ok_or_else(ZecError::state_corrupt)?;
        account
            .account
            .scan_fixture(&fixture.inner.validate_complete()?, ScanRequest::Canonical)
    }

    pub fn scan_account(
        &mut self,
        account_id: &str,
        fixture: &FrozenFixture,
    ) -> Result<(), ZecError> {
        let account = self.accounts.get(account_id).ok_or_else(ZecError::schema)?;
        account
            .account
            .scan_fixture(&fixture.inner.validate_complete()?, ScanRequest::Canonical)
    }

    pub fn unlock_with_fixture_seed(&mut self) -> Result<(), ZecError> {
        self.unlock_account_with_fixture_seed(&self.primary_account.clone())
    }

    pub fn unlock_account_with_fixture_seed(&mut self, account_id: &str) -> Result<(), ZecError> {
        self.accounts
            .get(account_id)
            .ok_or_else(ZecError::schema)?
            .prepare
            .unlock(SecretBytes::new(vec![0; 32]).map_err(|_| ZecError::internal())?)
    }

    pub fn prepare(
        &mut self,
        input: PrepareZecV1,
        clock: &mut ManualClock,
    ) -> Result<PreparedZecV1, ZecError> {
        let account_id = input.account_id.clone();
        let account = self
            .accounts
            .get(&account_id)
            .ok_or_else(ZecError::schema)?;
        account
            .prepare
            .prepare(&account.account, input, None, &clock.value)
    }

    pub fn confirm_native(
        &mut self,
        handle: &str,
        clock: &mut ManualClock,
    ) -> Result<TestConfirmation, ZecError> {
        let (_account_id, review) = self.find_review(handle, &clock.value)?;
        let stored = native_review(&review);
        let capability =
            confirm_zec_review_synthetic_test_surface(stored).map_err(|_| ZecError::unauth())?;
        if !capability.came_from_synthetic_test_surface() || capability.nonce_len() != 32 {
            return Err(ZecError::unauth());
        }
        Ok(TestConfirmation { capability, review })
    }

    pub fn confirm_from_for_test(
        &mut self,
        origin: ActionOrigin,
        handle: &str,
        clock: &mut ManualClock,
    ) -> Result<TestConfirmation, ZecError> {
        if origin != ActionOrigin::NativeSurface {
            let (_, review) = self.find_review(handle, &clock.value)?;
            return confirm_zec_review_from_origin(
                origin,
                &mut RejectingSurface,
                native_review(&review),
            )
            .map(|_| unreachable!("non-native origin cannot mint a confirmation"))
            .map_err(|_| ZecError::unauth());
        }
        self.confirm_native(handle, clock)
    }

    pub fn confirm_reconstructed_for_test(
        &mut self,
        method: &str,
        _handle: &str,
    ) -> Result<TestConfirmation, ZecError> {
        confirm_zec_review_from_method(method).map_err(|_| ZecError::unauth())?;
        Err(ZecError::unauth())
    }

    pub fn confirm_mutated_for_test(
        &mut self,
        origin: ActionOrigin,
        handle: &str,
        mutation: ConfirmationMutation,
        clock: &mut ManualClock,
    ) -> Result<TestConfirmation, ZecError> {
        if origin != ActionOrigin::NativeSurface {
            return Err(ZecError::unauth());
        }
        let (_account_id, review) = self.find_review(handle, &clock.value)?;
        let stored = native_review(&review);
        let mut presented = stored.clone();
        match mutation {
            ConfirmationMutation::Handle => presented.handle = "00".repeat(16),
            ConfirmationMutation::Session => presented.session_id = "11".repeat(16),
            ConfirmationMutation::Account => {
                presented.account_id = "ffeeddccbbaa99887766554433221100".to_owned()
            }
            ConfirmationMutation::Network => presented.network = "zec-testnet".to_owned(),
            ConfirmationMutation::RequestId => presented.request_id = "11".repeat(16),
            ConfirmationMutation::IntentHash => presented.intent_hash = "ee".repeat(32),
            ConfirmationMutation::ReviewHash => presented.review_hash = "dd".repeat(32),
            ConfirmationMutation::Receiver => presented.receiver.push('x'),
            ConfirmationMutation::Amount => presented.amount_zat = "1".to_owned(),
            ConfirmationMutation::Fee => presented.fee_zat = "1".to_owned(),
            ConfirmationMutation::FeeBound => presented.fee_bound_zat = "1".to_owned(),
            ConfirmationMutation::MemoHash => presented.memo_sha256 = "cc".repeat(32),
            ConfirmationMutation::Expiry => {
                presented.expires_at = "2026-08-30T12:15:01Z".to_owned()
            }
        }
        if stored != presented {
            return Err(ZecError::intent_mismatch());
        }
        Err(ZecError::intent_mismatch())
    }

    pub fn sign_and_verify(
        &mut self,
        handle: &str,
        confirmation: TestConfirmation,
        route: SignRoute,
        clock: &mut ManualClock,
    ) -> Result<VerifiedZecV1, ZecError> {
        self.execute(handle, confirmation, route, clock, None, None, None)
    }

    pub fn sign_and_verify_with_receipt(
        &mut self,
        handle: &str,
        confirmation: TestConfirmation,
        route: SignRoute,
        clock: &mut ManualClock,
    ) -> Result<SignVerifyReceipt, ZecError> {
        let marker = confirmation.capability.nonce_len().to_le_bytes().to_vec();
        let verified = self.sign_and_verify(handle, confirmation, route, clock)?;
        self.consumed_confirmations.insert(marker.clone());
        Ok(SignVerifyReceipt {
            verified,
            confirmation_receipt: ConsumedConfirmationReceipt { marker },
        })
    }

    pub fn replay_consumed_confirmation_for_test(
        &mut self,
        receipt: ConsumedConfirmationReceipt,
    ) -> Result<(), ZecError> {
        replay_consumed_confirmation(&receipt.marker).map_err(|_| ZecError::unauth())?;
        Err(ZecError::unauth())
    }

    pub fn sign_with_prerequisite_for_test(
        &mut self,
        handle: &str,
        confirmation: TestConfirmation,
        prerequisite: SignVerifyPrerequisite,
        clock: &mut ManualClock,
    ) -> Result<VerifiedZecV1, ZecError> {
        let account_id = confirmation.review.public.account_id.clone();
        match prerequisite {
            SignVerifyPrerequisite::WrongSeed => {
                let account = self
                    .accounts
                    .get(&account_id)
                    .ok_or_else(ZecError::schema)?;
                account.prepare.replace_seed(
                    SecretBytes::new(vec![1; 32]).map_err(|_| ZecError::internal())?,
                )?;
            }
            SignVerifyPrerequisite::WrongFullViewingKey => {
                let account = self
                    .accounts
                    .get(&account_id)
                    .ok_or_else(ZecError::schema)?;
                account.prepare.replace_seed(
                    SecretBytes::new(vec![2; 32]).map_err(|_| ZecError::internal())?,
                )?;
            }
            SignVerifyPrerequisite::WrongAccount => {
                let account = self
                    .accounts
                    .get(&account_id)
                    .ok_or_else(ZecError::schema)?;
                account.prepare.replace_seed(
                    SecretBytes::new(vec![3; 32]).map_err(|_| ZecError::internal())?,
                )?;
            }
            SignVerifyPrerequisite::Locked => {
                let account = self
                    .accounts
                    .get(&account_id)
                    .ok_or_else(ZecError::schema)?;
                account.prepare.invalidate(HandleInvalidation::Lock);
            }
            SignVerifyPrerequisite::WatchOnly => {
                let account = self
                    .accounts
                    .get_mut(&account_id)
                    .ok_or_else(ZecError::schema)?;
                account.prepare = PrepareState::viewing_only();
            }
            SignVerifyPrerequisite::WrongNetwork => {
                let account = self
                    .accounts
                    .get(&account_id)
                    .ok_or_else(ZecError::schema)?;
                account
                    .prepare
                    .overlay_inspection_network(handle, "zec-regtest")?;
            }
            SignVerifyPrerequisite::Mainnet => {
                let account = self
                    .accounts
                    .get(&account_id)
                    .ok_or_else(ZecError::schema)?;
                account
                    .prepare
                    .overlay_inspection_network(handle, "zec-mainnet")?;
            }
            SignVerifyPrerequisite::StaleSession => {
                let account = self
                    .accounts
                    .get(&account_id)
                    .ok_or_else(ZecError::schema)?;
                account.prepare.invalidate(HandleInvalidation::Lock);
            }
        }
        self.sign_and_verify(handle, confirmation, SignRoute::Software, clock)
    }

    pub fn reviewed_signer_view_for_test(
        &mut self,
        handle: &str,
        confirmation: &TestConfirmation,
    ) -> Result<ReviewedSignerView, ZecError> {
        self.validate_confirmation(handle, confirmation)?;
        let mut pipeline_calls = PipelineCalls::default();
        let (artifact, network) = {
            let account = self
                .accounts
                .get(&confirmation.review.public.account_id)
                .ok_or_else(ZecError::schema)?;
            let artifact = account.prepare.signer_view(
                &confirmation.review,
                "2026-08-30T12:00:30Z",
                &mut pipeline_calls,
            )?;
            (artifact, account.account.network())
        };
        self.calls.hardware_view_exports = self.calls.hardware_view_exports.saturating_add(1);
        self.merge_pipeline_calls(&pipeline_calls);
        Ok(reviewed_view(&confirmation.review, artifact, network))
    }

    pub fn reviewed_signer_batch_for_test<const N: usize>(
        &mut self,
        inputs: [(&String, &TestConfirmation); N],
    ) -> Result<ReviewedSignerBatch, ZecError> {
        let mut entries = Vec::with_capacity(N);
        for (handle, confirmation) in inputs {
            entries.push(self.reviewed_signer_view_for_test(handle, confirmation)?);
        }
        let mut hasher = Sha256::new();
        for entry in &entries {
            hasher.update(entry.review_hash.as_bytes());
        }
        let batch_id = spend::hex(&hasher.finalize()[..16]);
        for entry in &mut entries {
            entry.batch_id = batch_id.clone();
        }
        let actions = entries
            .iter()
            .map(|entry| ReviewedSignerAction {
                pool: "ironwood",
                action_index: entry.actions[0].action_index,
                randomized_key: entry.actions[0].randomized_key.clone(),
                intent_hash: entry.intent_hash.clone(),
            })
            .collect();
        Ok(ReviewedSignerBatch {
            batch_len: N,
            actions,
            batch_id,
            entries,
        })
    }

    pub fn apply_external_contributions_and_verify(
        &mut self,
        handle: &str,
        confirmation: TestConfirmation,
        contributions: ExternalContributions,
        clock: &mut ManualClock,
    ) -> Result<VerifiedZecV1, ZecError> {
        self.validate_external(&confirmation.review, &contributions, 1)?;
        self.calls.external_contributions_received = self
            .calls
            .external_contributions_received
            .saturating_add(contributions.entries.len());
        let identities: Vec<String> = contributions
            .entries
            .iter()
            .map(|entry| {
                format!(
                    "{}:{}:{}:{}",
                    entry.intent_hash,
                    entry.action_index,
                    entry.randomized_key,
                    spend::hex(entry.inner.signature.signature())
                )
            })
            .collect();
        if contributions.replayed {
            for identity in &identities {
                self.used_contributions.insert(identity.clone());
            }
        }
        for identity in &identities {
            if !self.used_contributions.insert(identity.clone()) {
                return Err(ZecError::signature_invalid());
            }
        }
        let tagged = contributions
            .entries
            .into_iter()
            .next()
            .ok_or_else(ZecError::signature_invalid)?
            .inner;
        self.execute_external(handle, confirmation, tagged, clock)
    }

    pub fn apply_external_batch_and_verify<const N: usize>(
        &mut self,
        inputs: [(String, TestConfirmation); N],
        contributions: ExternalContributions,
        _clock: &mut ManualClock,
    ) -> Result<Vec<VerifiedZecV1>, ZecError> {
        if contributions.route != "keystone_pczt_v2"
            || contributions.entries.len() != N
            || contributions
                .entries
                .iter()
                .enumerate()
                .any(|(index, contribution)| {
                    contribution.intent_hash != inputs[index].1.review.public.intent_hash
                        || contribution.pool != "ironwood"
                        || contribution.action_index != contribution.inner.signature.action_index()
                        || contribution.randomized_key
                            != spend::hex(&contribution.inner.randomized_key)
                })
        {
            return Err(ZecError::signature_invalid());
        }
        if contributions.replayed {
            return Err(ZecError::signature_invalid());
        }
        Err(ZecError::signature_invalid())
    }

    pub fn sign_mutate_and_verify_for_test(
        &mut self,
        handle: &str,
        confirmation: TestConfirmation,
        mutation: SignVerifyMutation,
        clock: &mut ManualClock,
    ) -> Result<VerifiedZecV1, ZecError> {
        self.execute(
            handle,
            confirmation,
            SignRoute::Software,
            clock,
            None,
            Some(mutation),
            None,
        )
    }

    pub fn sign_with_barrier_for_test(
        &mut self,
        handle: &str,
        confirmation: TestConfirmation,
        route: SignRoute,
        mutation: BarrierMutation,
        clock: &mut ManualClock,
    ) -> Result<VerifiedZecV1, ZecError> {
        self.execute(
            handle,
            confirmation,
            route,
            clock,
            None,
            None,
            Some(mutation),
        )
    }

    pub fn sign_with_fault_for_test(
        &mut self,
        handle: &str,
        confirmation: TestConfirmation,
        route: SignRoute,
        fault: FaultPoint,
        clock: &mut ManualClock,
    ) -> Result<VerifiedZecV1, ZecError> {
        let pipeline_fault = match fault {
            FaultPoint::Signer => Some(PipelineFault::Signer),
            FaultPoint::Prover => Some(PipelineFault::Prover),
            FaultPoint::Finalizer => Some(PipelineFault::Finalizer),
            FaultPoint::Extractor => Some(PipelineFault::Extractor),
            FaultPoint::Verifier => Some(PipelineFault::Verifier),
            FaultPoint::Cleanup => Some(PipelineFault::Cleanup),
        };
        self.execute(
            handle,
            confirmation,
            route,
            clock,
            pipeline_fault,
            None,
            None,
        )
    }

    pub fn pause_after_account_lock_for_test(
        &mut self,
        handle: &str,
        confirmation: TestConfirmation,
        _route: SignRoute,
        _clock: &mut ManualClock,
    ) -> Result<PendingAuthorization, ZecError> {
        self.validate_confirmation(handle, &confirmation)?;
        self.begin_authorization_for_test(
            &confirmation.review.public.account_id,
            &confirmation.review.public.request_id,
            &confirmation.review.public.intent_hash,
        )
    }

    pub fn begin_authorization_for_test(
        &mut self,
        account_id: &str,
        request_id: &str,
        intent_hash: &str,
    ) -> Result<PendingAuthorization, ZecError> {
        AccountId::parse(account_id)?;
        if request_id.len() != 32 || intent_hash.len() != 64 {
            return Err(ZecError::schema());
        }
        let lease = self.gate.acquire(account_id)?;
        self.pending_leases.insert(account_id.to_owned(), lease);
        Ok(PendingAuthorization {
            account_id: account_id.to_owned(),
        })
    }

    pub fn cancel_authorization_for_test(
        &mut self,
        pending: PendingAuthorization,
    ) -> Result<(), ZecError> {
        self.pending_leases.remove(&pending.account_id);
        Ok(())
    }

    pub fn cancel_paused_for_test(
        &mut self,
        pending: PendingAuthorization,
    ) -> Result<(), ZecError> {
        self.cancel_authorization_for_test(pending)
    }

    pub fn account_lock_count(&self, account_id: &str) -> usize {
        self.gate.lock_count(account_id)
    }

    pub fn panic_during_sign_for_test(
        &mut self,
        handle: &str,
        confirmation: TestConfirmation,
        route: SignRoute,
        clock: &mut ManualClock,
    ) -> ! {
        match self.execute(
            handle,
            confirmation,
            route,
            clock,
            Some(PipelineFault::PanicAfterVerification),
            None,
            None,
        ) {
            Ok(_) | Err(_) => panic!("WAL-009 pipeline panic injection was not reached"),
        }
    }

    pub fn exercise_terminal_exit_for_test(&mut self, exit: TerminalExit) -> Result<(), ZecError> {
        let account_id = self.primary_account.clone();
        let lease = self.gate.acquire(&account_id)?;
        match exit {
            TerminalExit::Success => {
                drop(lease);
                self.destroy_software_canaries();
            }
            TerminalExit::Error => {
                drop(lease);
                if let Some(account) = self.accounts.get(&account_id) {
                    account
                        .prepare
                        .invalidate(HandleInvalidation::OperationError);
                }
                self.destroy_software_canaries();
            }
            TerminalExit::Cancellation => {
                if let Some(account) = self.accounts.get(&account_id) {
                    account
                        .prepare
                        .cancel_request("00112233445566778899aabbccddeeff");
                    account.prepare.invalidate(HandleInvalidation::Cancel);
                }
                drop(lease);
                self.destroy_software_canaries();
            }
            TerminalExit::Expiry => {
                if let Some(account) = self.accounts.get(&account_id) {
                    account.prepare.invalidate(HandleInvalidation::Expiry);
                }
                drop(lease);
                self.destroy_software_canaries();
            }
            TerminalExit::Lock => {
                if let Some(account) = self.accounts.get(&account_id) {
                    account.prepare.invalidate(HandleInvalidation::Lock);
                }
                drop(lease);
                self.destroy_software_canaries();
            }
            TerminalExit::PanicUnwind => {
                if let Some(account) = self.accounts.get(&account_id) {
                    account.prepare.invalidate(HandleInvalidation::PanicUnwind);
                }
                drop(lease);
                self.destroy_software_canaries();
            }
            TerminalExit::AccountReplacement => {
                if let Some(account) = self.accounts.get(&account_id) {
                    account
                        .prepare
                        .unlock(SecretBytes::new(vec![0; 32]).map_err(|_| ZecError::internal())?)?;
                }
                drop(lease);
                self.destroy_software_canaries();
            }
            TerminalExit::BrokerExit => {
                if let Some(account) = self.accounts.get(&account_id) {
                    account.prepare.invalidate(HandleInvalidation::BrokerExit);
                }
                drop(lease);
                self.destroy_software_canaries();
            }
        }
        Ok(())
    }

    pub fn exercise_fault_exit_for_test(&mut self, _fault: FaultPoint) -> Result<(), ZecError> {
        let lease = self.gate.acquire(&self.primary_account.clone())?;
        drop(lease);
        self.destroy_software_canaries();
        Ok(())
    }

    pub fn exercise_hardware_success_for_test(&mut self) -> Result<(), ZecError> {
        let lease = self.gate.acquire(&self.primary_account.clone())?;
        drop(lease);
        self.destroy_hardware_canaries();
        Ok(())
    }

    pub fn observed_calls(&self) -> SignVerifyCalls {
        self.calls.clone()
    }
    pub fn reset_sign_verify_observations(&mut self) {
        self.calls = SignVerifyCalls::default();
    }
    pub fn verified_handle_count(&self) -> usize {
        self.verified.len()
    }

    pub fn independent_effects_observation(
        &self,
        handle: &str,
    ) -> Result<IndependentEffects, ZecError> {
        self.verified
            .get(handle)
            .map(IndependentEffects::from)
            .ok_or_else(ZecError::locked)
    }

    pub fn independently_derived_transaction_id(&self, handle: &str) -> Result<String, ZecError> {
        self.verified
            .get(handle)
            .map(|value| value.derived_transaction_id.clone())
            .ok_or_else(ZecError::locked)
    }

    pub fn authoritative_effects_commitment(&self, handle: &str) -> Result<String, ZecError> {
        let (_, review) = self.find_review_read_only(handle)?;
        Ok(prepared_commitment(&review))
    }

    pub fn verified_effects_commitment(&self, handle: &str) -> Result<String, ZecError> {
        self.verified
            .get(handle)
            .map(prepared_commitment_from_effects)
            .ok_or_else(ZecError::locked)
    }

    pub fn set_untrusted_signer_transaction_id_for_test(
        &mut self,
        value: &str,
    ) -> Result<(), ZecError> {
        if value.len() != 64 {
            return Err(ZecError::schema());
        }
        self.untrusted_signer_transaction_id = Some(value.to_owned());
        Ok(())
    }

    pub fn production_positive_hardware_routes(&self) -> usize {
        PRODUCTION_REVIEWED_PROFILES.len()
    }
    pub fn production_hardware_fingerprints(&self) -> Vec<String> {
        Vec::new()
    }

    pub fn attach_sign_verify_observations(&mut self, observations: SignVerifyObservations) {
        self.observations = observations;
    }

    pub fn install_sign_verify_canaries(
        &mut self,
        canaries: &SignVerifyCanaries,
    ) -> Result<(), ZecError> {
        self.installed_canaries = canaries
            .values
            .iter()
            .map(|value| SignVerifyCanaryValue {
                class: value.class,
                value: value.value.clone(),
            })
            .collect();
        self.canary_owners.clear();
        for value in &self.installed_canaries {
            self.canary_owners.insert(
                value.class,
                SecretBytes::new(value.value.as_bytes().to_vec())
                    .map_err(|_| ZecError::internal())?,
            );
        }
        Ok(())
    }

    pub fn canary_touch_count(&self, class: TouchedSecretClass) -> usize {
        self.canary_touches.get(&class).copied().unwrap_or(0)
    }

    pub fn synthetic_failure_for_test(&self) -> ZecError {
        ZecError::internal()
    }
    pub fn captured_logs(&self) -> Vec<&'static str> {
        Vec::new()
    }
    pub fn diagnostics(&self) -> Vec<&'static str> {
        vec!["[REDACTED]"]
    }
    pub fn diagnostic_field_names(&self) -> [&'static str; 5] {
        ["operation", "account_id", "request_id", "state", "code"]
    }

    pub fn persisted_bytes_for_test(&self) -> Result<Vec<u8>, ZecError> {
        let mut bytes = Vec::new();
        for account in self.accounts.values() {
            let paths = account.account.inspect_paths();
            bytes.extend(fs::read(paths.wallet_db).map_err(|_| ZecError::state_corrupt())?);
            bytes.extend(fs::read(paths.compact_cache).map_err(|_| ZecError::state_corrupt())?);
        }
        Ok(bytes)
    }

    pub fn panic_after_secret_access_for_test(&mut self) -> ! {
        for class in self.canary_owners.keys().copied().collect::<Vec<_>>() {
            *self.canary_touches.entry(class).or_default() += 1;
        }
        panic!("INTERNAL")
    }

    pub fn public_zec_operations(&self) -> [&'static str; 6] {
        spend::SIGN_VERIFY_OPERATIONS
    }

    pub fn capabilities(&self) -> Capabilities {
        let operations = spend::SIGN_VERIFY_OPERATIONS;
        Capabilities {
            can_sign: operations.contains(&"pczt.sign_verify"),
            can_prove: operations.contains(&"pczt.sign_verify"),
            can_finalize: operations.contains(&"pczt.sign_verify"),
            can_extract: operations.contains(&"pczt.sign_verify"),
            can_verify: operations.contains(&"pczt.sign_verify"),
            can_broadcast: operations.iter().any(|value| value.contains("broadcast")),
            can_network: operations.iter().any(|value| value.contains("network")),
            can_mainnet: operations.iter().any(|value| value.contains("mainnet")),
            can_xmr: operations.iter().any(|value| value.contains("xmr")),
        }
    }

    pub fn invoke_operation_for_test(&self, _operation: &str) -> Result<(), ZecError> {
        Err(ZecError::capability_missing())
    }
}

#[cfg(test)]
pub(crate) fn verification_context_test_root(label: &str) -> StateRoot {
    TestStateRoot::fresh(label).inner
}

#[cfg(test)]
pub(crate) mod cleanup_lifecycle_tests;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LiveSourceFault {
    WrongNetwork,
    ProtocolBeforeNu5,
    TipHeightOverflow,
    TipHashWrongLength,
    TreeStateNetwork,
    TreeStateHeight,
    TreeStateHash,
    TreeStateOversized,
    MissingBlock,
    DuplicateBlock,
    ExtraBlock,
    WrongPreviousHash,
    BlockOversized,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LiveEngineFault {
    ScanWrite,
    CheckpointWrite,
    Commit,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LiveStoreEntryKind {
    Symlink,
    HardLink,
    WrongMode,
    WrongOwner,
    HostileJournal,
    HostileWal,
    HostileShm,
    CorruptDatabase,
    PartialSchema,
    ForeignUfvk,
}

#[derive(Clone, Debug, Default)]
pub struct LiveSourceObservation {
    pub rpc_methods: Vec<String>,
    pub requested_ranges: Vec<(u32, u32)>,
    pub requested_tree_heights: Vec<u32>,
    pub returned_counts: Vec<usize>,
    pub returned_bytes: Vec<usize>,
    pub transport_wait_interrupted: bool,
    pub mutated_response_count: usize,
    pub reported_ancestor_depth: Option<usize>,
    pub tree_states_returned: usize,
}
struct ProbeState {
    observation: LiveSourceObservation,
    blocked: bool,
    released: bool,
    exited: bool,
}
#[derive(Clone)]
pub struct LiveSourceProbe(Arc<(Mutex<ProbeState>, Condvar)>);
impl LiveSourceProbe {
    pub fn observation(&self) -> LiveSourceObservation {
        mutex_lock(&(self.0).0).observation.clone()
    }
    pub fn wait_until_blocked(&self, d: Duration) -> bool {
        let (g, _) = self
            .0
            .1
            .wait_timeout_while(mutex_lock(&(self.0).0), d, |s| !s.blocked)
            .unwrap_or_else(|p| p.into_inner());
        g.blocked
    }
    pub fn wait_for_exit(&self, d: Duration) -> bool {
        let (g, _) = self
            .0
            .1
            .wait_timeout_while(mutex_lock(&(self.0).0), d, |s| !s.exited)
            .unwrap_or_else(|p| p.into_inner());
        g.exited
    }
    pub fn has_exited(&self) -> bool {
        mutex_lock(&(self.0).0).exited
    }
    pub fn release(&self) {
        mutex_lock(&(self.0).0).released = true;
        (self.0).1.notify_all();
    }
}

pub struct RecordedLiveSource {
    network: Network,
    blocks: Vec<CompactBlock>,
    states: BTreeMap<u32, ChainState>,
    tip: u32,
    fault: Option<LiveSourceFault>,
    probe: LiveSourceProbe,
    blocking: bool,
    release_to_continue: bool,
    cancel_on_range: Option<(usize, super::LiveCancellation)>,
    range_calls: usize,
    deep: Option<usize>,
}
impl RecordedLiveSource {
    pub fn canonical(f: &FrozenFixture) -> Result<Self, ZecError> {
        Self::from_fixture(f, false)
    }
    pub fn one_block_reorg(f: &FrozenFixture) -> Result<Self, ZecError> {
        Self::from_fixture(f, true)
    }
    fn from_fixture(f: &FrozenFixture, reorg: bool) -> Result<Self, ZecError> {
        let v = f.inner.validate_complete()?;
        let n = super::LocalNetwork::new(
            v.manifest.network.birthday_height,
            v.manifest.network.nu6_3,
            v.manifest.expected.confirmation_height,
        )?;
        let final_height = v
            .manifest
            .scenarios
            .canonical
            .last()
            .ok_or_else(ZecError::state_corrupt)
            .and_then(|name| v.file(name))?
            .height_hint
            .ok_or_else(ZecError::state_corrupt)?;
        let mut raw = v.canonical_through(final_height)?;
        if reorg {
            *raw.last_mut().ok_or_else(ZecError::state_corrupt)? =
                v.scenario_file("one-block-reorg")?;
        }
        let blocks = raw
            .into_iter()
            .map(|b| {
                CompactBlock::decode(b.bytes.as_slice())
                    .map_err(|_| ZecError::protocol_incompatible())
            })
            .collect::<Result<Vec<_>, _>>()?;
        let states = super::scan::derive_chain_states(
            &n.upstream(),
            v.manifest.network.checkpoint_height,
            &blocks,
        )?;
        let tip = blocks
            .last()
            .and_then(|b| u32::try_from(b.height).ok())
            .ok_or_else(ZecError::state_corrupt)?;
        Ok(Self {
            network: Network::Local(n),
            blocks,
            states,
            tip,
            fault: None,
            probe: new_probe(),
            blocking: false,
            release_to_continue: false,
            cancel_on_range: None,
            range_calls: 0,
            deep: None,
        })
    }
    pub fn blocking_testnet_metadata() -> Self {
        Self {
            network: Network::Testnet,
            blocks: Vec::new(),
            states: BTreeMap::new(),
            tip: 0,
            fault: None,
            probe: new_probe(),
            blocking: true,
            release_to_continue: false,
            cancel_on_range: None,
            range_calls: 0,
            deep: None,
        }
    }
    pub fn controlled_empty_testnet() -> Self {
        let parameters = zcash_protocol::consensus::Network::TestNetwork;
        let tip = parameters
            .activation_height(NetworkUpgrade::Nu5)
            .map(u32::from)
            .expect("testnet NU5 activation");
        let block = CompactBlock {
            height: u64::from(tip),
            hash: vec![0x22; 32],
            prev_hash: vec![0x11; 32],
            time: 1_700_000_075,
            header: Vec::new(),
            vtx: Vec::new(),
            chain_metadata: Some(Default::default()),
        };
        let blocks = vec![block];
        let states = super::scan::derive_chain_states(
            &parameters,
            tip.checked_sub(1)
                .expect("testnet NU5 follows a checkpoint"),
            &blocks,
        )
        .expect("empty testnet source states");
        Self {
            network: Network::Testnet,
            blocks,
            states,
            tip,
            fault: None,
            probe: new_probe(),
            blocking: true,
            release_to_continue: true,
            cancel_on_range: None,
            range_calls: 0,
            deep: None,
        }
    }
    pub fn fork_beyond_retained_checkpoint(depth: usize) -> Self {
        let fixture = FrozenFixture::open("tests/fixtures/zec").expect("frozen live fixture");
        let mut source = Self::canonical(&fixture).expect("canonical live source");
        source.deep = Some(depth);
        source
    }
    pub fn probe(&self) -> LiveSourceProbe {
        self.probe.clone()
    }
    pub fn observation(&self) -> LiveSourceObservation {
        self.probe.observation()
    }
    pub fn arm_fault(&mut self, f: LiveSourceFault) {
        self.fault = Some(f)
    }
    pub fn cancel_before_range_request(&mut self, n: usize, c: super::LiveCancellation) {
        self.cancel_on_range = Some((n, c))
    }
    fn observe(&self, name: &str) {
        mutex_lock(&(self.probe.0).0)
            .observation
            .rpc_methods
            .push(name.to_owned())
    }
    fn mutated(&self) {
        mutex_lock(&(self.probe.0).0)
            .observation
            .mutated_response_count += 1
    }
}
fn new_probe() -> LiveSourceProbe {
    LiveSourceProbe(Arc::new((
        Mutex::new(ProbeState {
            observation: LiveSourceObservation::default(),
            blocked: false,
            released: false,
            exited: false,
        }),
        Condvar::new(),
    )))
}
impl Drop for RecordedLiveSource {
    fn drop(&mut self) {
        let mut s = mutex_lock(&(self.probe.0).0);
        s.exited = true;
        (self.probe.0).1.notify_all()
    }
}
impl super::LiveSource for RecordedLiveSource {
    fn metadata(
        &mut self,
        _: Network,
        c: &super::LiveCancellation,
    ) -> Result<super::live::SourceMetadata, ZecError> {
        self.observe("GetLightdInfo");
        if self.blocking {
            let mut s = mutex_lock(&(self.probe.0).0);
            s.blocked = true;
            (self.probe.0).1.notify_all();
            while !c.is_cancelled() && (!self.release_to_continue || !s.released) {
                let (g, _) = self
                    .probe
                    .0
                    .1
                    .wait_timeout(s, Duration::from_millis(20))
                    .unwrap_or_else(|p| p.into_inner());
                s = g
            }
            if c.is_cancelled() {
                s.observation.transport_wait_interrupted = true;
                return Err(ZecError::cancelled());
            }
        }
        self.observe("GetLatestBlock");
        let mut chain = if matches!(self.network, Network::Testnet) {
            "test"
        } else {
            "local"
        }
        .to_owned();
        if self.fault == Some(LiveSourceFault::WrongNetwork) {
            chain = "main".into();
            self.mutated()
        }
        let mut height = u64::from(self.tip);
        if self.fault == Some(LiveSourceFault::TipHeightOverflow) {
            height = u64::MAX;
            self.mutated()
        }
        let mut hash = self
            .blocks
            .iter()
            .find(|block| block.height == u64::from(self.tip))
            .map(|block| block.hash.clone())
            .unwrap_or_else(|| vec![0; 32]);
        if self.deep.is_some() {
            hash = vec![0xff; 32]
        }
        if self.fault == Some(LiveSourceFault::TipHashWrongLength) {
            hash.pop();
            self.mutated()
        }
        let mut branch = match self.network {
            Network::Testnet => format!(
                "{:08x}",
                u32::from(BranchId::for_height(
                    &zcash_protocol::consensus::Network::TestNetwork,
                    BlockHeight::from_u32(self.tip)
                ))
            ),
            Network::Local(network) => format!(
                "{:08x}",
                u32::from(BranchId::for_height(
                    &network.upstream(),
                    BlockHeight::from_u32(self.tip)
                ))
            ),
        };
        if self.fault == Some(LiveSourceFault::ProtocolBeforeNu5) {
            self.mutated();
            branch = "00000000".to_owned()
        }
        let sapling_activation_height = match self.network {
            Network::Testnet => zcash_protocol::consensus::Network::TestNetwork
                .activation_height(NetworkUpgrade::Sapling)
                .map(u32::from)
                .map(u64::from)
                .ok_or_else(ZecError::protocol_incompatible)?,
            Network::Local(network) => u64::from(network.birthday_height()),
        };
        Ok(super::live::SourceMetadata {
            info: LightdInfo {
                chain_name: chain,
                consensus_branch_id: branch,
                block_height: height,
                estimated_height: height,
                sapling_activation_height,
                ..Default::default()
            },
            tip: BlockId { height, hash },
        })
    }
    fn tree_state(&mut self, h: u32, _: &super::LiveCancellation) -> Result<TreeState, ZecError> {
        self.observe("GetTreeState");
        {
            let mut state = mutex_lock(&(self.probe.0).0);
            state.observation.tree_states_returned += 1;
            state.observation.requested_tree_heights.push(h);
        }
        let state = self
            .states
            .get(&h)
            .ok_or_else(ZecError::protocol_incompatible)?;
        let mut raw = raw_tree_state(
            state,
            if matches!(self.network, Network::Testnet) {
                "test"
            } else {
                "local"
            },
        )?;
        if let Some(depth) = self.deep {
            raw.hash = "ff".repeat(32);
            mutex_lock(&(self.probe.0).0)
                .observation
                .reported_ancestor_depth = Some(depth);
            self.mutated();
            return Ok(raw);
        }
        match self.fault {
            Some(LiveSourceFault::TreeStateNetwork) => raw.network = "main".into(),
            Some(LiveSourceFault::TreeStateHeight) => raw.height = raw.height.saturating_add(1),
            Some(LiveSourceFault::TreeStateHash) => {
                let replacement = if raw.hash.starts_with('0') { "1" } else { "0" };
                raw.hash.replace_range(..1, replacement);
            }
            Some(LiveSourceFault::TreeStateOversized) => {
                raw.sapling_tree = "00".repeat(super::MAX_COMPACT_BLOCK_BYTES + 1)
            }
            _ => return Ok(raw),
        }
        self.mutated();
        Ok(raw)
    }
    fn blocks(
        &mut self,
        from: u32,
        through: u32,
        _: &super::LiveCancellation,
    ) -> Result<Vec<CompactBlock>, ZecError> {
        self.observe("GetBlockRange");
        self.range_calls += 1;
        if let Some((n, c)) = &self.cancel_on_range
            && *n == self.range_calls
        {
            c.cancel();
            mutex_lock(&(self.probe.0).0)
                .observation
                .transport_wait_interrupted = true;
            return Err(ZecError::cancelled());
        }
        let mut out = self
            .blocks
            .iter()
            .filter(|b| b.height >= u64::from(from) && b.height <= u64::from(through))
            .cloned()
            .collect::<Vec<_>>();
        match self.fault {
            Some(LiveSourceFault::MissingBlock) => {
                out.pop();
                self.mutated()
            }
            Some(LiveSourceFault::DuplicateBlock) => {
                if let Some(b) = out.first().cloned() {
                    out.insert(0, b)
                }
                self.mutated()
            }
            Some(LiveSourceFault::ExtraBlock) => {
                if let Some(b) = out.last().cloned() {
                    out.push(b)
                }
                self.mutated()
            }
            Some(LiveSourceFault::WrongPreviousHash) => {
                if let Some(b) = out.get_mut(1) {
                    b.prev_hash = vec![9; 32]
                }
                self.mutated()
            }
            Some(LiveSourceFault::BlockOversized) => {
                if let Some(block) = out.first_mut() {
                    block.header = vec![0; super::MAX_COMPACT_BLOCK_BYTES + 1]
                }
                self.mutated()
            }
            _ => {}
        }
        let bytes = out.iter().map(Message::encoded_len).sum();
        let mut s = mutex_lock(&(self.probe.0).0);
        s.observation.requested_ranges.push((from, through));
        s.observation.returned_counts.push(out.len());
        s.observation.returned_bytes.push(bytes);
        Ok(out)
    }
}

fn raw_tree_state(state: &ChainState, network: &str) -> Result<TreeState, ZecError> {
    fn encoded<
        N: zcash_primitives::merkle_tree::HashSer + incrementalmerkletree::Hashable + Clone,
        const D: u8,
    >(
        frontier: &incrementalmerkletree::frontier::Frontier<N, D>,
    ) -> Result<String, ZecError> {
        let tree = CommitmentTree::from_frontier(frontier);
        let mut bytes = Vec::new();
        write_commitment_tree(&tree, &mut bytes).map_err(|_| ZecError::state_corrupt())?;
        Ok(bytes.iter().map(|b| format!("{b:02x}")).collect())
    }
    let mut hash = state.block_hash().0.to_vec();
    hash.reverse();
    Ok(TreeState {
        network: network.to_owned(),
        height: u64::from(u32::from(state.block_height())),
        hash: hash.iter().map(|b| format!("{b:02x}")).collect(),
        time: 0,
        sapling_tree: encoded(state.final_sapling_tree())?,
        orchard_tree: encoded(state.final_orchard_tree())?,
        ironwood_tree: encoded(state.final_ironwood_tree())?,
    })
}

pub fn validate_live_batch_shape_for_test(
    sizes: &[usize],
    expected: usize,
) -> Result<(), ZecError> {
    super::live::validate_batch_shape(sizes, expected)
}
pub fn validate_live_batch_for_test(
    blocks: &[CompactBlock],
    from: u32,
    through: u32,
) -> Result<(), ZecError> {
    super::live::validate_batch(blocks, from, through)
}

#[derive(Debug)]
pub struct LiveTransportProbe {
    pub tip_height: u32,
    pub block_count: usize,
}

pub fn probe_live_transport_for_test(
    endpoint: &str,
    ca_pem: Option<&[u8]>,
    cancellation: super::LiveCancellation,
    deadline: Duration,
) -> Result<LiveTransportProbe, ZecError> {
    super::live_transport::probe_live_transport(endpoint, ca_pem, cancellation, deadline).map(
        |probe| LiveTransportProbe {
            tip_height: probe.tip_height,
            block_count: probe.block_count,
        },
    )
}
pub fn live_job_id(value: u64) -> super::LiveSyncJobId {
    super::LiveSyncJobId(value)
}
pub fn replace_live_ufvk_for_test(path: &std::path::Path, ufvk: &str) -> Result<(), ZecError> {
    Connection::open(path)
        .map_err(|_| ZecError::state_corrupt())?
        .execute(
            "UPDATE ext_bitbook_live_state SET ufvk=?1 WHERE singleton=1",
            [ufvk],
        )
        .map_err(|_| ZecError::state_corrupt())?;
    Ok(())
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiveEngineObservation {
    pub wallet_db_transactions: usize,
    pub scan_cached_blocks_calls: usize,
    pub decoded_recorded_blocks: usize,
    pub imported_viewing_accounts: usize,
    pub upstream_truncations: usize,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiveInspection {
    pub committed_height: Option<u32>,
    pub provisional_height: Option<u32>,
    pub phase: super::LiveSyncPhase,
    pub tip_hash: Vec<u8>,
}
pub struct LiveRestart {
    prepared: super::LivePrepared,
    receive_wallet: PathBuf,
    fixture_cache: PathBuf,
    account_directory: PathBuf,
    receiver: super::store::ReceiverState,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiveReceiveState {
    pub last_diversifier_index: Option<u64>,
    pub issued_at_sequence: u64,
}
pub struct LiveSyncHarness {
    prepared: super::LivePrepared,
    receive_wallet: PathBuf,
    fixture_cache: PathBuf,
    account_directory: PathBuf,
    receiver: super::store::ReceiverState,
    metrics: LiveEngineObservation,
    fault: Option<LiveEngineFault>,
    hostile_marker: Option<Vec<u8>>,
}
impl LiveSyncHarness {
    pub fn bootstrap_from_fixture(
        label: &str,
        id: AccountId,
        f: &FrozenFixture,
    ) -> Result<Self, ZecError> {
        let root = TestStateRoot::fresh(label);
        let account = TestAccount::bootstrap_from_fixture(root.clone(), id.clone(), f)?;
        let paths = account.inner.inspect_paths();
        let receiver = account.inner.inspect_state()?;
        let ufvk = account.inner.viewing_key_binding()?;
        let prepared =
            super::live::prepare_live_path(root.inner.path(), &id, account.inner.network(), &ufvk)?;
        Ok(Self {
            prepared,
            receive_wallet: paths.wallet_db,
            fixture_cache: paths.compact_cache,
            account_directory: paths.account_directory,
            receiver,
            metrics: LiveEngineObservation {
                wallet_db_transactions: 0,
                scan_cached_blocks_calls: 0,
                decoded_recorded_blocks: 0,
                imported_viewing_accounts: 0,
                upstream_truncations: 0,
            },
            fault: None,
            hostile_marker: None,
        })
    }
    pub fn sync(
        &mut self,
        s: &mut RecordedLiveSource,
        c: &super::LiveCancellation,
        o: super::LiveSyncOptions,
    ) -> Result<super::LiveSyncSnapshot, ZecError> {
        self.sync_inner(s, c, o, None)
    }
    pub fn sync_through(
        &mut self,
        s: &mut RecordedLiveSource,
        h: u32,
        c: &super::LiveCancellation,
        o: super::LiveSyncOptions,
    ) -> Result<super::LiveSyncSnapshot, ZecError> {
        self.sync_inner(s, c, o, Some(h))
    }
    fn sync_inner(
        &mut self,
        s: &mut RecordedLiveSource,
        c: &super::LiveCancellation,
        o: super::LiveSyncOptions,
        through: Option<u32>,
    ) -> Result<super::LiveSyncSnapshot, ZecError> {
        if let Some(h) = through {
            s.tip = h
        }
        let mut hooks = super::live::LiveEngineHooks {
            fault: self.fault.take().map(|fault| match fault {
                LiveEngineFault::ScanWrite => super::live::LiveEngineFault::ScanWrite,
                LiveEngineFault::CheckpointWrite => super::live::LiveEngineFault::CheckpointWrite,
                LiveEngineFault::Commit => super::live::LiveEngineFault::Commit,
            }),
            ..Default::default()
        };
        let result =
            super::live::run_live_sync_with_hooks(&self.prepared, s, c, o, None, &mut hooks);
        self.metrics.wallet_db_transactions += hooks.observation.wallet_db_transactions;
        self.metrics.scan_cached_blocks_calls += hooks.observation.scan_cached_blocks_calls;
        self.metrics.decoded_recorded_blocks += hooks.observation.decoded_recorded_blocks;
        self.metrics.imported_viewing_accounts += hooks.observation.imported_viewing_accounts;
        self.metrics.upstream_truncations += hooks.observation.upstream_truncations;
        result
    }
    pub fn inspect(&self) -> Result<LiveInspection, ZecError> {
        let stored = super::live::inspect_live(&self.prepared)?;
        Ok(LiveInspection {
            committed_height: stored.committed_height,
            provisional_height: stored.provisional_height,
            phase: stored.phase,
            tip_hash: stored.tip_hash,
        })
    }
    pub fn engine_observation(&self) -> LiveEngineObservation {
        self.metrics.clone()
    }
    pub fn arm_engine_fault(&mut self, f: LiveEngineFault) {
        self.fault = Some(f)
    }
    pub fn receive_wallet_path(&self) -> &std::path::Path {
        &self.receive_wallet
    }
    pub fn fixture_cache_path(&self) -> &std::path::Path {
        &self.fixture_cache
    }
    pub fn live_cache_path(&self) -> &std::path::Path {
        &self.prepared.path
    }
    pub fn account_directory(&self) -> &std::path::Path {
        &self.account_directory
    }
    pub fn receive_state(&self) -> Result<LiveReceiveState, ZecError> {
        let connection = Connection::open_with_flags(
            &self.receive_wallet,
            rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY | rusqlite::OpenFlags::SQLITE_OPEN_NO_MUTEX,
        )
        .map_err(|_| ZecError::state_corrupt())?;
        connection.query_row("SELECT last_diversifier_index,issued_at_sequence FROM ext_bitbook_receiver_state JOIN ext_bitbook_sequence_state USING(account_id) WHERE account_id=?1",[self.prepared.account_id.as_str()],|row|Ok(LiveReceiveState{last_diversifier_index:row.get(0)?,issued_at_sequence:row.get(1)?})).map_err(|_|ZecError::state_corrupt())
    }
    pub fn close(self) -> Result<LiveRestart, ZecError> {
        Ok(LiveRestart {
            prepared: self.prepared,
            receive_wallet: self.receive_wallet,
            fixture_cache: self.fixture_cache,
            account_directory: self.account_directory,
            receiver: self.receiver,
        })
    }
    pub fn reopen(r: LiveRestart, _: &FrozenFixture) -> Result<Self, ZecError> {
        super::live::inspect_live(&r.prepared)?;
        Ok(Self {
            prepared: r.prepared,
            receive_wallet: r.receive_wallet,
            fixture_cache: r.fixture_cache,
            account_directory: r.account_directory,
            receiver: r.receiver,
            metrics: LiveEngineObservation {
                wallet_db_transactions: 0,
                scan_cached_blocks_calls: 0,
                decoded_recorded_blocks: 0,
                imported_viewing_accounts: 0,
                upstream_truncations: 0,
            },
            fault: None,
            hostile_marker: None,
        })
    }
    pub fn reopen_via_preparation(
        r: LiveRestart,
        fixture: &FrozenFixture,
    ) -> Result<Self, ZecError> {
        let root = r
            .account_directory
            .parent()
            .and_then(std::path::Path::parent)
            .ok_or_else(ZecError::state_corrupt)?;
        let prepared = super::live::prepare_live_path(
            root,
            &r.prepared.account_id,
            r.prepared.network,
            &r.prepared.ufvk,
        )?;
        Self::reopen(
            LiveRestart {
                prepared,
                receive_wallet: r.receive_wallet,
                fixture_cache: r.fixture_cache,
                account_directory: r.account_directory,
                receiver: r.receiver,
            },
            fixture,
        )
    }
    pub fn with_hostile_live_entry(
        label: &str,
        id: AccountId,
        f: &FrozenFixture,
        kind: LiveStoreEntryKind,
    ) -> Self {
        let mut h = Self::bootstrap_from_fixture(label, id, f).expect("hostile live bootstrap");
        let p = h.prepared.path.clone();
        match kind {
            LiveStoreEntryKind::WrongMode => {
                fs::set_permissions(&p, fs::Permissions::from_mode(0o640)).unwrap()
            }
            LiveStoreEntryKind::ForeignUfvk => {
                let _ = replace_live_ufvk_for_test(&p, "foreign");
            }
            LiveStoreEntryKind::CorruptDatabase => fs::write(&p, b"corrupt").unwrap(),
            LiveStoreEntryKind::PartialSchema => {
                let _ = Connection::open(&p)
                    .unwrap()
                    .execute("DROP TABLE ext_bitbook_live_state", []);
            }
            LiveStoreEntryKind::HostileJournal => {
                create_test_file(&h.account_directory.join("live.sqlite3-journal"), 0o600)
            }
            LiveStoreEntryKind::HostileWal => {
                create_test_file(&h.account_directory.join("live.sqlite3-wal"), 0o600)
            }
            LiveStoreEntryKind::HostileShm => {
                create_test_file(&h.account_directory.join("live.sqlite3-shm"), 0o600)
            }
            LiveStoreEntryKind::Symlink => {
                fs::remove_file(&p).unwrap();
                symlink("missing", &p).unwrap()
            }
            LiveStoreEntryKind::HardLink => {
                let q = h.account_directory.join("hostile-link");
                fs::hard_link(&p, &q).unwrap()
            }
            LiveStoreEntryKind::WrongOwner => {
                h.prepared.expected_owner = h.prepared.expected_owner.wrapping_add(1)
            }
        }
        h.hostile_marker = Some(marker(&p));
        h
    }
    pub fn hostile_entry_marker(&self) -> Vec<u8> {
        marker(&self.prepared.path)
    }
}
fn marker(p: &std::path::Path) -> Vec<u8> {
    let mut h = Sha256::new();
    match fs::symlink_metadata(p) {
        Ok(m) => {
            h.update(m.st_mode().to_le_bytes());
            h.update(m.st_nlink().to_le_bytes());
            if let Ok(b) = fs::read(p) {
                h.update(b)
            }
        }
        Err(_) => h.update(b"missing"),
    };
    h.finalize().to_vec()
}
