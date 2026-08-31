use std::fs;
use std::os::unix::fs::{DirBuilderExt, PermissionsExt};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::vault::{SecretBytes, WipeEvent, WipeObserver};

use super::address::{self, DecodedReceiver, SeedExit};
use super::fixture;
use super::store::{AddressAccount, AddressFaultPort, StateRoot};
use super::{AccountId, FreshReceiverV1, Network, ZecError};

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
