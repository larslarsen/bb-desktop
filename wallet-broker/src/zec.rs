use core::fmt;

mod address;
mod fixture;
mod scan;
mod store;

#[doc(hidden)]
pub mod test_support;

pub const MAX_DIVERSIFIER_INDEX: u64 = i64::MAX as u64;
pub const MAX_ISSUANCE_SEQUENCE: u64 = i64::MAX as u64;
pub const MAX_FIXTURE_MANIFEST_BYTES: usize = 256 * 1024;
pub const MAX_COMPACT_BLOCK_BYTES: usize = 2 * 1024 * 1024;

pub type ScanError = ZecError;

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
