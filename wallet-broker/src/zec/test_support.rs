use std::collections::BTreeSet;
use std::fs::{self, OpenOptions};
use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt, PermissionsExt, symlink};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::{SystemTime, UNIX_EPOCH};

use sha2::{Digest, Sha256};
use zcash_protocol::consensus::{BlockHeight, BranchId};

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
    PcztInspection, PoolInventoryData, PrepareState, PrepareWipeLog, normalize_diagnostic,
    parse_canonical_positive_u64,
};
use super::scan::{ScanBalances as InnerScanBalances, ScanFaultPort, ScanInspection, ScanRequest};
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
    pub can_extract: bool,
    pub can_broadcast: bool,
    pub can_network: bool,
    pub can_mainnet: bool,
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

    pub fn invoke_operation_for_test(&self, _operation: &str) -> Result<(), ZecError> {
        Err(ZecError::capability_missing())
    }

    pub fn capabilities(&self) -> Capabilities {
        Capabilities {
            can_sign: false,
            can_prove: false,
            can_extract: false,
            can_broadcast: false,
            can_network: false,
            can_mainnet: false,
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
