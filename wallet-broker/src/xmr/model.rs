use core::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HostPlatform {
    LinuxX86_64,
    LinuxAarch64,
    WindowsX86_64,
    MacosX86_64,
    MacosAarch64,
    Unknown,
}

impl HostPlatform {
    pub fn current() -> Self {
        if cfg!(all(target_os = "linux", target_arch = "x86_64")) {
            Self::LinuxX86_64
        } else if cfg!(all(target_os = "linux", target_arch = "aarch64")) {
            Self::LinuxAarch64
        } else if cfg!(all(target_os = "windows", target_arch = "x86_64")) {
            Self::WindowsX86_64
        } else if cfg!(all(target_os = "macos", target_arch = "x86_64")) {
            Self::MacosX86_64
        } else if cfg!(all(target_os = "macos", target_arch = "aarch64")) {
            Self::MacosAarch64
        } else {
            Self::Unknown
        }
    }

    pub(crate) fn supports_distribution(self) -> bool {
        self == Self::LinuxX86_64
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum XmrNetwork {
    Stagenet,
    Testnet,
}

impl XmrNetwork {
    pub(crate) fn parse(value: &str) -> Result<Self, XmrError> {
        match value {
            "xmr-stagenet" => Ok(Self::Stagenet),
            "xmr-testnet" => Ok(Self::Testnet),
            "xmr-mainnet" => Err(XmrError::network_disabled()),
            _ => Err(XmrError::request_schema()),
        }
    }

    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::Stagenet => "xmr-stagenet",
            Self::Testnet => "xmr-testnet",
        }
    }

    pub(crate) fn daemon_port(self) -> u16 {
        match self {
            Self::Stagenet => 38_081,
            Self::Testnet => 28_081,
        }
    }

    pub(crate) fn flag(self) -> &'static str {
        match self {
            Self::Stagenet => "stagenet",
            Self::Testnet => "testnet",
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct XmrError {
    code: &'static str,
    message: &'static str,
}

impl XmrError {
    fn new(code: &'static str, message: &'static str) -> Self {
        Self { code, message }
    }

    pub(crate) fn schema() -> Self {
        Self::new("SCHEMA", "Monero installation selection is invalid")
    }

    pub(crate) fn request_schema() -> Self {
        Self::new("SCHEMA", "Monero wallet request is invalid")
    }

    pub(crate) fn unavailable() -> Self {
        Self::new("UNAVAILABLE", "Monero wallet is unavailable")
    }

    pub(crate) fn protocol_incompatible() -> Self {
        Self::new(
            "PROTOCOL_INCOMPATIBLE",
            "Monero installation is incompatible",
        )
    }

    pub(crate) fn state_corrupt() -> Self {
        Self::new("STATE_CORRUPT", "Monero installation state is unavailable")
    }

    pub(crate) fn internal() -> Self {
        Self::new("INTERNAL", "Monero installation could not be saved")
    }

    pub(crate) fn limit() -> Self {
        Self::new("LIMIT", "Monero wallet process limit reached")
    }

    pub(crate) fn network_disabled() -> Self {
        Self::new("NETWORK_DISABLED", "Monero network is disabled")
    }

    pub fn code(&self) -> &'static str {
        self.code
    }

    pub fn public_message(&self) -> &'static str {
        self.message
    }
}

impl fmt::Debug for XmrError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("XmrError")
            .field("code", &self.code)
            .finish()
    }
}

impl fmt::Display for XmrError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.message)
    }
}

impl std::error::Error for XmrError {}
