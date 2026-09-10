use core::fmt;
use std::fs;
use std::io::ErrorKind;
use std::path::Path;
use std::sync::{Arc, Mutex};

use crate::vault::{SecretBytes, WipeObserver};

mod address;
mod fixture;
mod hardware;
mod prepare;
mod scan;
mod spend;
mod store;

#[doc(hidden)]
pub mod test_support;

pub const MAX_DIVERSIFIER_INDEX: u64 = i64::MAX as u64;
pub const MAX_ISSUANCE_SEQUENCE: u64 = i64::MAX as u64;
pub const MAX_FIXTURE_MANIFEST_BYTES: usize = 256 * 1024;
pub const MAX_COMPACT_BLOCK_BYTES: usize = 2 * 1024 * 1024;
pub const MAX_MEMO_BYTES: usize = 512;
pub const MAX_PREPARED_HANDLES: usize = 64;
pub const MAX_DIAGNOSTIC_BYTES: usize = 4096;

pub use prepare::{HandleBinding, HandleInvalidation, PrepareZecV1, PreparedZecV1};

pub type ScanError = ZecError;

pub(crate) fn fresh_receiver_for_account(
    broker_root: &Path,
    account_id: &str,
    session_seed: &SecretBytes,
    observer: &mut dyn WipeObserver,
) -> Result<FreshReceiverV1, ZecError> {
    let account_id = AccountId::parse(account_id)?;
    let network = Network::Testnet;
    let root = store::StateRoot::new(broker_root.to_path_buf(), Arc::new(Mutex::new(Vec::new())));
    let network_directory = broker_root.join(network.as_str());
    let account_directory = network_directory.join(account_id.as_str());

    let account_present = match fs::symlink_metadata(&network_directory) {
        Err(error) if error.kind() == ErrorKind::NotFound => false,
        Err(_) => return Err(ZecError::state_corrupt()),
        Ok(metadata) if metadata.file_type().is_dir() && !metadata.file_type().is_symlink() => {
            match fs::symlink_metadata(&account_directory) {
                Ok(_) => true,
                Err(error) if error.kind() == ErrorKind::NotFound => false,
                Err(_) => return Err(ZecError::state_corrupt()),
            }
        }
        Ok(_) => return Err(ZecError::state_corrupt()),
    };

    let mut expected_seed = copy_session_seed(session_seed)?;
    let expected_ufvk = address::derive_ufvk(network, &mut expected_seed, observer)?;
    let account = if account_present {
        store::AddressAccount::open_viewing_with_network(root, account_id, network)?
    } else {
        store::AddressAccount::bootstrap(
            root,
            account_id,
            network,
            copy_session_seed(session_seed)?,
            observer,
        )?
    };
    if account.viewing_key_binding()? != expected_ufvk {
        return Err(ZecError::state_corrupt());
    }
    account.fresh_receiver(0)
}

fn copy_session_seed(seed: &SecretBytes) -> Result<SecretBytes, ZecError> {
    seed.expose(|bytes| SecretBytes::new(bytes.to_vec()).map_err(|_| ZecError::internal()))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StoreFault {
    MigrationWrite,
    MigrationSync,
    MigrationCommit,
    Write,
    FileSync,
    DirectorySync,
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct AccountId(String);

impl AccountId {
    pub fn parse(value: &str) -> Result<Self, ZecError> {
        if value.len() == 32
            && value
                .as_bytes()
                .iter()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(byte))
        {
            Ok(Self(value.to_owned()))
        } else {
            Err(ZecError::schema())
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for AccountId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("AccountId([REDACTED])")
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LocalNetwork {
    birthday_height: u32,
    nu6_3_height: u32,
    confirmation_height: u32,
}

impl LocalNetwork {
    pub fn new(
        birthday_height: u32,
        nu6_3_height: u32,
        confirmation_height: u32,
    ) -> Result<Self, ZecError> {
        if birthday_height == 0
            || birthday_height > nu6_3_height
            || nu6_3_height > confirmation_height
        {
            return Err(ZecError::schema());
        }
        Ok(Self {
            birthday_height,
            nu6_3_height,
            confirmation_height,
        })
    }

    pub(crate) fn birthday_height(self) -> u32 {
        self.birthday_height
    }

    pub(crate) fn nu6_3_height(self) -> u32 {
        self.nu6_3_height
    }

    pub(crate) fn confirmation_height(self) -> u32 {
        self.confirmation_height
    }

    pub(crate) fn upstream(self) -> zcash_protocol::local_consensus::LocalNetwork {
        use zcash_protocol::consensus::BlockHeight;

        let birthday = Some(BlockHeight::from_u32(self.birthday_height));
        zcash_protocol::local_consensus::LocalNetwork {
            overwinter: Some(BlockHeight::from_u32(1)),
            sapling: birthday,
            blossom: birthday,
            heartwood: birthday,
            canopy: birthday,
            nu5: birthday,
            nu6: birthday,
            nu6_1: birthday,
            nu6_2: birthday,
            nu6_3: Some(BlockHeight::from_u32(self.nu6_3_height)),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq)]
pub enum Network {
    Testnet,
    Local(LocalNetwork),
}

impl Network {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Testnet => "zec-testnet",
            Self::Local(_) => "zec-local",
        }
    }
}

impl PartialEq for Network {
    fn eq(&self, other: &Self) -> bool {
        // A Unified Address exposes only its network discriminator. Exact local
        // activation-height equality lives on LocalNetwork and is independently
        // enforced by every SQLite account-binding validation.
        matches!(
            (self, other),
            (Self::Testnet, Self::Testnet) | (Self::Local(_), Self::Local(_))
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FreshReceiverV1 {
    pub account_id: AccountId,
    pub network: Network,
    pub receiver: String,
    pub diversifier_index: String,
    pub issued_at_sequence: String,
}

#[derive(Clone, Eq, PartialEq)]
pub struct ZecError {
    code: &'static str,
    message: &'static str,
}

impl ZecError {
    pub(crate) fn new(code: &'static str, message: &'static str) -> Self {
        Self { code, message }
    }

    pub(crate) fn schema() -> Self {
        Self::new("SCHEMA", "Zcash request is invalid")
    }

    pub(crate) fn limit() -> Self {
        Self::new("LIMIT", "Zcash limit reached")
    }

    pub(crate) fn state_corrupt() -> Self {
        Self::new("STATE_CORRUPT", "Zcash state is unavailable")
    }

    pub(crate) fn internal() -> Self {
        Self::new("INTERNAL", "Zcash operation failed")
    }

    pub(crate) fn network_disabled() -> Self {
        Self::new("NETWORK_DISABLED", "Zcash network is disabled")
    }

    pub(crate) fn transparent_downgrade() -> Self {
        Self::new(
            "TRANSPARENT_DOWNGRADE",
            "Transparent receiver composition is disabled",
        )
    }

    pub(crate) fn protocol_incompatible() -> Self {
        Self::new(
            "PROTOCOL_INCOMPATIBLE",
            "Receiver composition is unsupported",
        )
    }

    pub(crate) fn locked() -> Self {
        Self::new("LOCKED", "Zcash account is locked")
    }

    pub(crate) fn watch_only() -> Self {
        Self::new("WATCH_ONLY", "Zcash account is viewing-only")
    }

    pub(crate) fn capability_missing() -> Self {
        Self::new("CAPABILITY_MISSING", "Zcash capability is unavailable")
    }

    pub(crate) fn migration_required() -> Self {
        Self::new("MIGRATION_REQUIRED", "Zcash value requires migration")
    }

    pub(crate) fn insufficient_funds() -> Self {
        Self::new("INSUFFICIENT_FUNDS", "Insufficient eligible Zcash funds")
    }

    pub(crate) fn fee_bound() -> Self {
        Self::new("FEE_BOUND", "Zcash fee exceeds the approved bound")
    }

    pub(crate) fn expired() -> Self {
        Self::new("EXPIRED", "Zcash request has expired")
    }

    pub(crate) fn unauth() -> Self {
        Self::new("UNAUTH", "Zcash action requires the native surface")
    }

    pub(crate) fn intent_mismatch() -> Self {
        Self::new("INTENT_MISMATCH", "Zcash intent verification failed")
    }

    pub(crate) fn signature_invalid() -> Self {
        Self::new("SIGNATURE_INVALID", "Zcash authorization is invalid")
    }

    pub(crate) fn cancelled() -> Self {
        Self::new("CANCELLED", "Zcash request was cancelled")
    }

    pub(crate) fn account_busy() -> Self {
        Self::new("ACCOUNT_BUSY", "Zcash account is busy")
    }

    pub fn code(&self) -> &'static str {
        self.code
    }

    pub fn public_message(&self) -> &'static str {
        self.message
    }
}

impl fmt::Debug for ZecError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ZecError")
            .field("code", &self.code)
            .finish()
    }
}

impl fmt::Display for ZecError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.message)
    }
}

impl std::error::Error for ZecError {}
