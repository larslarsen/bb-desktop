use core::fmt;
use std::collections::BTreeSet;
use std::io::{Read, Write};
use std::net::{Ipv4Addr, Shutdown, SocketAddrV4, TcpStream};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::{Duration, Instant};

use md5::{Digest, Md5};
use serde_json::{Map, Value};
use zeroize::{Zeroize, Zeroizing};

use crate::xmr::distribution::VERIFIED_VERSION;
use crate::xmr::model::{XmrError, XmrNetwork};
use crate::xmr::process::{ReadinessStatus, WalletRpcControl};

pub const CONNECT_TIMEOUT_SECS: u64 = 2;
pub const READ_TIMEOUT_SECS: u64 = 5;
pub const WRITE_TIMEOUT_SECS: u64 = 5;
pub const READINESS_TIMEOUT_SECS: u64 = 10;
pub const MAX_REQUEST_BODY_BYTES: usize = 16 * 1_024;
pub const MAX_HTTP_BYTES: usize = 64 * 1_024;
pub const MAX_JSON_NESTING: usize = 16;

const JSON_RPC_ID: &str = "bitbook-xmr-v1";
const JSON_RPC_PATH: &str = "/json_rpc";
const MAX_CHALLENGE_BYTES: usize = 4 * 1_024;
const MAX_CHALLENGE_VALUE_BYTES: usize = 1_024;
const MAX_NODE_VERSION_BYTES: usize = 128;
pub const WALLET_RPC_VERSION: u64 = (1 << 16) | 31;
const READINESS_RETRY_MILLIS: u64 = 25;
const ENGLISH_LANGUAGE: &str = "English";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum RpcMethod {
    GetVersion,
    CreateWallet,
    RestoreDeterministicWallet,
    GenerateFromKeys,
    OpenWallet,
    CloseWallet,
    StopWallet,
    QueryKey,
    Refresh,
    GetHeight,
    GetBalance,
    GetAddress,
    CreateAddress,
    ValidateAddress,
    GetInfo,
    HardForkInfo,
}

pub(crate) struct RpcSecret(Zeroizing<String>);

impl RpcSecret {
    fn new(value: &str) -> Result<Self, XmrError> {
        if value.is_empty() || value.len() > MAX_REQUEST_BODY_BYTES {
            return Err(XmrError::request_schema());
        }
        if value.bytes().any(|byte| byte < 0x20 || byte == 0x7f) {
            return Err(XmrError::request_schema());
        }
        Ok(Self(Zeroizing::new(value.to_owned())))
    }

    fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl fmt::Debug for RpcSecret {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("RpcSecret([REDACTED])")
    }
}

pub(crate) enum RpcRequest {
    GetVersion,
    CloseWallet,
    StopWallet,
    Refresh,
    GetHeight,
    GetBalance,
    CreateAddress,
    GetInfo,
    HardForkInfo,
    CreateWallet {
        filename: String,
        password: RpcSecret,
        language: &'static str,
    },
    QueryKeyMnemonic,
    GetAddress {
        account_index: u32,
        address_index: Vec<u32>,
    },
    ValidateAddress {
        address: RpcSecret,
    },
    GenerateFromKeys {
        filename: String,
        password: RpcSecret,
        address: RpcSecret,
        viewkey: RpcSecret,
        restore_height: u64,
        spendkey: &'static str,
        language: &'static str,
    },
    OpenWallet {
        filename: String,
        password: RpcSecret,
    },
    RestoreDeterministicWallet {
        filename: String,
        password: RpcSecret,
        seed: RpcSecret,
        restore_height: u64,
        language: &'static str,
        seed_offset: &'static str,
    },
}

impl fmt::Debug for RpcRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RpcRequest")
            .field("method", &self.method().name())
            .finish()
    }
}

impl RpcRequest {
    pub(crate) fn create_wallet(filename: &str, password: &str) -> Result<Self, XmrError> {
        Ok(Self::CreateWallet {
            filename: validated_wallet_filename(filename)?,
            password: RpcSecret::new(password)?,
            language: ENGLISH_LANGUAGE,
        })
    }

    pub(crate) fn query_key_mnemonic() -> Self {
        Self::QueryKeyMnemonic
    }

    pub(crate) fn get_address() -> Self {
        Self::GetAddress {
            account_index: 0,
            address_index: Vec::new(),
        }
    }

    pub(crate) fn get_address_at(account_index: u32, address_index: u32) -> Self {
        Self::GetAddress {
            account_index,
            address_index: vec![address_index],
        }
    }

    pub(crate) fn create_address() -> Self {
        Self::CreateAddress
    }

    pub(crate) fn validate_address(address: &str) -> Result<Self, XmrError> {
        Ok(Self::ValidateAddress {
            address: RpcSecret::new(address)?,
        })
    }

    pub(crate) fn generate_from_keys(
        filename: &str,
        password: &str,
        address: &str,
        viewkey: &str,
        restore_height: u64,
    ) -> Result<Self, XmrError> {
        Ok(Self::GenerateFromKeys {
            filename: validated_wallet_filename(filename)?,
            password: RpcSecret::new(password)?,
            address: RpcSecret::new(address)?,
            viewkey: RpcSecret::new(viewkey)?,
            restore_height,
            spendkey: "",
            language: "",
        })
    }

    pub(crate) fn open_wallet(filename: &str, password: &str) -> Result<Self, XmrError> {
        Ok(Self::OpenWallet {
            filename: validated_wallet_filename(filename)?,
            password: RpcSecret::new(password)?,
        })
    }

    pub(crate) fn restore_deterministic_wallet(
        filename: &str,
        password: &str,
        seed: &str,
        restore_height: u64,
    ) -> Result<Self, XmrError> {
        Ok(Self::RestoreDeterministicWallet {
            filename: validated_wallet_filename(filename)?,
            password: RpcSecret::new(password)?,
            seed: RpcSecret::new(seed)?,
            restore_height,
            language: ENGLISH_LANGUAGE,
            seed_offset: "",
        })
    }

    const fn method(&self) -> RpcMethod {
        match self {
            Self::GetVersion => RpcMethod::GetVersion,
            Self::CloseWallet => RpcMethod::CloseWallet,
            Self::StopWallet => RpcMethod::StopWallet,
            Self::Refresh => RpcMethod::Refresh,
            Self::GetHeight => RpcMethod::GetHeight,
            Self::GetBalance => RpcMethod::GetBalance,
            Self::CreateAddress => RpcMethod::CreateAddress,
            Self::GetInfo => RpcMethod::GetInfo,
            Self::HardForkInfo => RpcMethod::HardForkInfo,
            Self::CreateWallet { .. } => RpcMethod::CreateWallet,
            Self::QueryKeyMnemonic => RpcMethod::QueryKey,
            Self::GetAddress { .. } => RpcMethod::GetAddress,
            Self::ValidateAddress { .. } => RpcMethod::ValidateAddress,
            Self::GenerateFromKeys { .. } => RpcMethod::GenerateFromKeys,
            Self::OpenWallet { .. } => RpcMethod::OpenWallet,
            Self::RestoreDeterministicWallet { .. } => RpcMethod::RestoreDeterministicWallet,
        }
    }

    fn params(&self) -> Zeroizing<Vec<u8>> {
        match self {
            Self::GetVersion
            | Self::CloseWallet
            | Self::StopWallet
            | Self::Refresh
            | Self::GetHeight
            | Self::GetInfo
            | Self::HardForkInfo => Zeroizing::new(b"{}".to_vec()),
            Self::GetBalance => Zeroizing::new(
                br#"{"account_index":0,"address_indices":[],"all_accounts":false,"strict":true}"#
                    .to_vec(),
            ),
            Self::CreateAddress => {
                Zeroizing::new(br#"{"account_index":0,"label":"","count":1}"#.to_vec())
            }
            Self::QueryKeyMnemonic => Zeroizing::new(br#"{"key_type":"mnemonic"}"#.to_vec()),
            Self::GetAddress {
                account_index,
                address_index,
            } => json_get_address(*account_index, address_index.as_slice()),
            Self::CreateWallet {
                filename,
                password,
                language,
            } => json_object(&[
                ("filename", filename.as_str()),
                ("password", password.as_str()),
                ("language", language),
            ]),
            Self::ValidateAddress { address } => json_validate_address(address.as_str()),
            Self::GenerateFromKeys {
                filename,
                password,
                address,
                viewkey,
                restore_height,
                spendkey,
                language,
            } => json_generate_from_keys(
                filename,
                password.as_str(),
                address.as_str(),
                viewkey.as_str(),
                *restore_height,
                spendkey,
                language,
            ),
            Self::OpenWallet { filename, password } => json_object(&[
                ("filename", filename.as_str()),
                ("password", password.as_str()),
            ]),
            Self::RestoreDeterministicWallet {
                filename,
                password,
                seed,
                restore_height,
                language,
                seed_offset,
            } => json_restore_params(
                filename,
                password.as_str(),
                seed.as_str(),
                *restore_height,
                language,
                seed_offset,
            ),
        }
    }
}

impl RpcMethod {
    pub(crate) const fn name(self) -> &'static str {
        match self {
            Self::GetVersion => "get_version",
            Self::CreateWallet => "create_wallet",
            Self::RestoreDeterministicWallet => "restore_deterministic_wallet",
            Self::GenerateFromKeys => "generate_from_keys",
            Self::OpenWallet => "open_wallet",
            Self::CloseWallet => "close_wallet",
            Self::StopWallet => "stop_wallet",
            Self::QueryKey => "query_key",
            Self::Refresh => "refresh",
            Self::GetHeight => "get_height",
            Self::GetBalance => "get_balance",
            Self::GetAddress => "get_address",
            Self::CreateAddress => "create_address",
            Self::ValidateAddress => "validate_address",
            Self::GetInfo => "get_info",
            Self::HardForkInfo => "hard_fork_info",
        }
    }

    pub(crate) const fn is_wallet(self) -> bool {
        !matches!(self, Self::GetInfo | Self::HardForkInfo)
    }
}

pub(crate) fn request_dispatch_for_test(name: &str) -> bool {
    let method = match name {
        "get_version" => RpcMethod::GetVersion,
        "create_wallet" => RpcMethod::CreateWallet,
        "restore_deterministic_wallet" => RpcMethod::RestoreDeterministicWallet,
        "generate_from_keys" => RpcMethod::GenerateFromKeys,
        "open_wallet" => RpcMethod::OpenWallet,
        "close_wallet" => RpcMethod::CloseWallet,
        "stop_wallet" => RpcMethod::StopWallet,
        "query_key" => RpcMethod::QueryKey,
        "refresh" => RpcMethod::Refresh,
        "get_height" => RpcMethod::GetHeight,
        "get_balance" => RpcMethod::GetBalance,
        "get_address" => RpcMethod::GetAddress,
        "create_address" => RpcMethod::CreateAddress,
        "validate_address" => RpcMethod::ValidateAddress,
        "get_info" => RpcMethod::GetInfo,
        "hard_fork_info" => RpcMethod::HardForkInfo,
        _ => return false,
    };
    matches!(
        method,
        RpcMethod::GetVersion
            | RpcMethod::CloseWallet
            | RpcMethod::StopWallet
            | RpcMethod::Refresh
            | RpcMethod::GetHeight
            | RpcMethod::GetBalance
            | RpcMethod::CreateAddress
            | RpcMethod::GetInfo
            | RpcMethod::HardForkInfo
            | RpcMethod::CreateWallet
            | RpcMethod::QueryKey
            | RpcMethod::GetAddress
            | RpcMethod::ValidateAddress
            | RpcMethod::GenerateFromKeys
            | RpcMethod::OpenWallet
            | RpcMethod::RestoreDeterministicWallet
    )
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PortFailure {
    Unavailable,
    Limit,
}

pub(crate) trait HttpExchangePort {
    fn exchange(
        &mut self,
        port: u16,
        request: &[u8],
        connect_timeout: Duration,
        read_timeout: Duration,
        write_timeout: Duration,
    ) -> Result<Vec<u8>, PortFailure>;

    fn fill_entropy(&mut self, output: &mut [u8]) -> Result<(), PortFailure>;
    fn begin_readiness(&mut self);
    fn readiness_elapsed(&self) -> Duration;
    fn readiness_failure_retryable(&self) -> bool;
    fn wait_readiness_retry(&mut self, duration: Duration);
    fn end_readiness(&mut self);
    fn response_consumed(&mut self);
    fn close_all(&mut self);
}

pub(crate) struct SystemHttpPort {
    readiness_started: Option<Instant>,
    last_failure_not_listening: bool,
    readiness_exchange_succeeded: bool,
}

impl SystemHttpPort {
    pub(crate) fn new() -> Self {
        Self {
            readiness_started: None,
            last_failure_not_listening: false,
            readiness_exchange_succeeded: false,
        }
    }

    fn remaining_timeout(&self, maximum: Duration) -> Duration {
        self.readiness_started.map_or(maximum, |started| {
            maximum
                .min(Duration::from_secs(READINESS_TIMEOUT_SECS).saturating_sub(started.elapsed()))
        })
    }
}

impl HttpExchangePort for SystemHttpPort {
    fn exchange(
        &mut self,
        port: u16,
        request: &[u8],
        connect_timeout: Duration,
        read_timeout: Duration,
        write_timeout: Duration,
    ) -> Result<Vec<u8>, PortFailure> {
        let endpoint = SocketAddrV4::new(Ipv4Addr::LOCALHOST, port);
        self.last_failure_not_listening = false;
        let mut stream = match TcpStream::connect_timeout(&endpoint.into(), connect_timeout) {
            Ok(stream) => stream,
            Err(error) => {
                self.last_failure_not_listening = error.kind()
                    == std::io::ErrorKind::ConnectionRefused
                    && !self.readiness_exchange_succeeded;
                return Err(PortFailure::Unavailable);
            }
        };
        let result = (|| {
            let mut written = 0usize;
            while written < request.len() {
                let timeout = self.remaining_timeout(write_timeout);
                if timeout.is_zero() {
                    return Err(PortFailure::Unavailable);
                }
                stream
                    .set_write_timeout(Some(timeout))
                    .map_err(|_| PortFailure::Unavailable)?;
                let count = stream
                    .write(&request[written..])
                    .map_err(|_| PortFailure::Unavailable)?;
                if count == 0 {
                    return Err(PortFailure::Unavailable);
                }
                written = written.saturating_add(count);
            }
            let timeout = self.remaining_timeout(write_timeout);
            if timeout.is_zero() {
                return Err(PortFailure::Unavailable);
            }
            stream
                .set_write_timeout(Some(timeout))
                .map_err(|_| PortFailure::Unavailable)?;
            stream.flush().map_err(|_| PortFailure::Unavailable)?;
            let mut response = Vec::new();
            let mut chunk = [0u8; 4 * 1_024];
            loop {
                let timeout = self.remaining_timeout(read_timeout);
                if timeout.is_zero() {
                    return Err(PortFailure::Unavailable);
                }
                stream
                    .set_read_timeout(Some(timeout))
                    .map_err(|_| PortFailure::Unavailable)?;
                let count = stream
                    .read(&mut chunk)
                    .map_err(|_| PortFailure::Unavailable)?;
                if count == 0 {
                    break;
                }
                if response.len().saturating_add(count) > MAX_HTTP_BYTES {
                    return Err(PortFailure::Limit);
                }
                response.extend_from_slice(&chunk[..count]);
            }
            Ok(response)
        })();
        let _ = stream.shutdown(Shutdown::Both);
        if result.is_ok() {
            self.readiness_exchange_succeeded = true;
        }
        result
    }

    fn fill_entropy(&mut self, output: &mut [u8]) -> Result<(), PortFailure> {
        getrandom::fill(output).map_err(|_| PortFailure::Unavailable)
    }

    fn begin_readiness(&mut self) {
        self.readiness_started = Some(Instant::now());
        self.readiness_exchange_succeeded = false;
    }

    fn readiness_elapsed(&self) -> Duration {
        self.readiness_started
            .map_or(Duration::ZERO, |started| started.elapsed())
    }

    fn readiness_failure_retryable(&self) -> bool {
        self.last_failure_not_listening
    }

    fn wait_readiness_retry(&mut self, duration: Duration) {
        thread::sleep(duration);
    }

    fn end_readiness(&mut self) {
        self.readiness_started = None;
    }

    fn response_consumed(&mut self) {}

    fn close_all(&mut self) {}
}

#[derive(Default)]
pub(crate) struct WipeAudit {
    created: AtomicUsize,
    wiped: AtomicUsize,
}

impl WipeAudit {
    pub(crate) fn complete(&self) -> bool {
        self.created.load(Ordering::SeqCst) == self.wiped.load(Ordering::SeqCst)
    }
}

struct SecretBytes {
    bytes: Vec<u8>,
    audit: Option<Arc<WipeAudit>>,
}

impl SecretBytes {
    fn new(bytes: impl Into<Vec<u8>>, audit: Option<&Arc<WipeAudit>>) -> Self {
        if let Some(audit) = audit {
            audit.created.fetch_add(1, Ordering::SeqCst);
        }
        Self {
            bytes: bytes.into(),
            audit: audit.cloned(),
        }
    }

    fn text(&self) -> Result<&str, XmrError> {
        core::str::from_utf8(&self.bytes).map_err(|_| XmrError::protocol_incompatible())
    }
}

impl Drop for SecretBytes {
    fn drop(&mut self) {
        self.bytes.zeroize();
        if let Some(audit) = &self.audit {
            audit.wiped.fetch_add(1, Ordering::SeqCst);
        }
    }
}

struct DigestChallenge {
    realm: SecretBytes,
    nonce: SecretBytes,
    opaque: Option<SecretBytes>,
}

struct WalletCredentials {
    username: SecretBytes,
    password: SecretBytes,
}

struct HttpResponse {
    status: u16,
    challenge: Option<Zeroizing<Vec<u8>>>,
    body: Zeroizing<Vec<u8>>,
}

#[derive(Clone, Eq, PartialEq)]
pub(crate) struct NodeInfoResult {
    pub status: String,
    pub nettype: String,
    pub mainnet: bool,
    pub stagenet: bool,
    pub testnet: bool,
    pub offline: bool,
    pub untrusted: bool,
    pub bootstrap_daemon_address: String,
    pub was_bootstrap_ever_used: bool,
    pub synchronized: bool,
    pub height: u64,
    pub target_height: u64,
    pub height_without_bootstrap: u64,
}

#[derive(Clone, Eq, PartialEq)]
pub(crate) struct HardForkInfoResult {
    pub status: String,
    pub earliest_height: u64,
    pub enabled: bool,
    pub untrusted: bool,
    pub version: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum NodeState {
    Syncing,
    Ready,
}

impl NodeState {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Syncing => "NODE_SYNCING",
            Self::Ready => "READY",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct NodeProbeResult {
    pub state: NodeState,
    pub height: u64,
    pub height_without_bootstrap: u64,
}

#[derive(Clone, Eq, PartialEq)]
pub(crate) struct NodeViewProbeResult {
    pub state: NodeState,
    pub height: u64,
    pub hard_fork: HardForkInfoResult,
}

pub(crate) struct SensitiveRpcText(SecretBytes);

impl SensitiveRpcText {
    pub(crate) fn expose(&self) -> Result<&str, XmrError> {
        self.0.text()
    }
}

pub(crate) struct AddressEntryResult {
    pub address: SensitiveRpcText,
    pub address_index: u32,
    pub label: SensitiveRpcText,
    pub used: bool,
}

pub(crate) enum TypedResult {
    Version(u64),
    Height(u64),
    Balance {
        total: u64,
        unlocked: u64,
    },
    NodeInfo(NodeInfoResult),
    HardForkInfo(HardForkInfoResult),
    Empty,
    Restore {
        address: SensitiveRpcText,
        seed: SensitiveRpcText,
        was_deprecated: bool,
    },
    Generated {
        address: SensitiveRpcText,
    },
    Key(SensitiveRpcText),
    Refreshed {
        blocks_fetched: u64,
        received_money: bool,
    },
    Addresses {
        primary: SensitiveRpcText,
        addresses: Vec<AddressEntryResult>,
    },
    CreatedAddress {
        address: SensitiveRpcText,
        address_index: u32,
        address_count: usize,
    },
    AddressValidation {
        valid: bool,
        integrated: bool,
        subaddress: bool,
        nettype: String,
    },
}

pub(crate) struct RpcCore<P: HttpExchangePort> {
    port: P,
    audit: Option<Arc<WipeAudit>>,
}

impl<P: HttpExchangePort> RpcCore<P> {
    pub(crate) fn new(port: P) -> Self {
        Self { port, audit: None }
    }

    pub(crate) fn with_audit(port: P, audit: Arc<WipeAudit>) -> Self {
        Self {
            port,
            audit: Some(audit),
        }
    }

    pub(crate) fn port(&self) -> &P {
        &self.port
    }

    pub(crate) fn port_mut(&mut self) -> &mut P {
        &mut self.port
    }

    pub(crate) fn close_all(&mut self) {
        self.port.close_all();
    }

    pub(crate) fn call_node(
        &mut self,
        port: u16,
        request: RpcRequest,
    ) -> Result<TypedResult, XmrError> {
        let method = request.method();
        if method.is_wallet() {
            return Err(XmrError::request_schema());
        }
        let body = request_body(&request)?;
        let response = self.exchange(port, &body, None)?;
        if response.status != 200 || response.challenge.is_some() {
            return Err(XmrError::protocol_incompatible());
        }
        parse_rpc_body(&response.body, method)
    }

    pub(crate) fn call_wallet(
        &mut self,
        port: u16,
        username: &str,
        password: &str,
        request: RpcRequest,
    ) -> Result<TypedResult, XmrError> {
        let method = request.method();
        if !method.is_wallet() {
            return Err(XmrError::request_schema());
        }
        let credentials = WalletCredentials {
            username: SecretBytes::new(username.as_bytes().to_vec(), self.audit.as_ref()),
            password: SecretBytes::new(password.as_bytes().to_vec(), self.audit.as_ref()),
        };
        let body = request_body(&request)?;
        let first = self.exchange(port, &body, None)?;
        if first.status != 401 {
            return Err(XmrError::unauth());
        }
        let challenge = parse_digest_challenge(
            first.challenge.as_deref().ok_or_else(XmrError::unauth)?,
            self.audit.as_ref(),
        )?;
        let authorization = build_authorization(
            &mut self.port,
            &credentials,
            &challenge,
            method,
            self.audit.as_ref(),
        )?;
        let second = self.exchange(port, &body, Some(&authorization))?;
        if second.status == 401 || second.challenge.is_some() {
            return Err(XmrError::unauth());
        }
        if second.status != 200 {
            return Err(XmrError::protocol_incompatible());
        }
        parse_rpc_body(&second.body, method)
    }

    pub(crate) fn readiness_wallet(
        &mut self,
        port: u16,
        username: &str,
        password: &str,
    ) -> Result<TypedResult, XmrError> {
        self.port.begin_readiness();
        let deadline = Duration::from_secs(READINESS_TIMEOUT_SECS);
        let result = loop {
            if self.port.readiness_elapsed() >= deadline {
                break Err(XmrError::unavailable());
            }
            match self.call_wallet(port, username, password, RpcRequest::GetVersion) {
                Ok(result) if self.port.readiness_elapsed() <= deadline => break Ok(result),
                Ok(_) => break Err(XmrError::unavailable()),
                Err(error)
                    if error.code() == "UNAVAILABLE" && self.port.readiness_failure_retryable() =>
                {
                    let remaining = deadline.saturating_sub(self.port.readiness_elapsed());
                    if remaining.is_zero() {
                        break Err(error);
                    }
                    self.port.wait_readiness_retry(
                        remaining.min(Duration::from_millis(READINESS_RETRY_MILLIS)),
                    );
                }
                Err(error) => break Err(error),
            }
        };
        self.port.end_readiness();
        result
    }

    fn exchange(
        &mut self,
        port: u16,
        body: &[u8],
        authorization: Option<&SecretBytes>,
    ) -> Result<HttpResponse, XmrError> {
        let request = build_http_request(port, body, authorization)?;
        let readiness_remaining = Duration::from_secs(READINESS_TIMEOUT_SECS)
            .saturating_sub(self.port.readiness_elapsed());
        if readiness_remaining.is_zero() {
            return Err(XmrError::unavailable());
        }
        let raw = Zeroizing::new(
            self.port
                .exchange(
                    port,
                    &request,
                    readiness_remaining.min(Duration::from_secs(CONNECT_TIMEOUT_SECS)),
                    readiness_remaining.min(Duration::from_secs(READ_TIMEOUT_SECS)),
                    readiness_remaining.min(Duration::from_secs(WRITE_TIMEOUT_SECS)),
                )
                .map_err(port_error)?,
        );
        let response = parse_http_response(&raw);
        self.port.response_consumed();
        response
    }
}

fn port_error(error: PortFailure) -> XmrError {
    match error {
        PortFailure::Unavailable => XmrError::unavailable(),
        PortFailure::Limit => XmrError::limit(),
    }
}

fn request_body(request: &RpcRequest) -> Result<Zeroizing<Vec<u8>>, XmrError> {
    let method = request.method();
    let params = request.params();
    let mut body = Zeroizing::new(Vec::with_capacity(256));
    body.extend_from_slice(b"{\"jsonrpc\":\"2.0\",\"id\":\"");
    body.extend_from_slice(JSON_RPC_ID.as_bytes());
    body.extend_from_slice(b"\",\"method\":\"");
    body.extend_from_slice(method.name().as_bytes());
    body.extend_from_slice(b"\",\"params\":");
    body.extend_from_slice(&params);
    body.push(b'}');
    if body.len() > MAX_REQUEST_BODY_BYTES {
        Err(XmrError::limit())
    } else {
        Ok(body)
    }
}

fn validated_wallet_filename(filename: &str) -> Result<String, XmrError> {
    if filename.len() == 32
        && filename
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        && !filename.contains('/')
        && !filename.contains("..")
    {
        Ok(filename.to_owned())
    } else {
        Err(XmrError::request_schema())
    }
}

fn json_object(fields: &[(&str, &str)]) -> Zeroizing<Vec<u8>> {
    let mut params = Zeroizing::new(Vec::new());
    params.push(b'{');
    for (index, (key, value)) in fields.iter().enumerate() {
        if index > 0 {
            params.push(b',');
        }
        json_string(&mut params, key);
        params.push(b':');
        json_string(&mut params, value);
    }
    params.push(b'}');
    params
}

fn json_validate_address(address: &str) -> Zeroizing<Vec<u8>> {
    let mut params = Zeroizing::new(Vec::new());
    params.extend_from_slice(b"{\"address\":");
    json_string(&mut params, address);
    params.extend_from_slice(b",\"any_net_type\":false,\"allow_openalias\":false}");
    params
}

fn json_get_address(account_index: u32, address_index: &[u32]) -> Zeroizing<Vec<u8>> {
    let mut params = Zeroizing::new(Vec::new());
    params.extend_from_slice(b"{\"account_index\":");
    params.extend_from_slice(account_index.to_string().as_bytes());
    params.extend_from_slice(b",\"address_index\":[");
    for (index, value) in address_index.iter().enumerate() {
        if index > 0 {
            params.push(b',');
        }
        params.extend_from_slice(value.to_string().as_bytes());
    }
    params.extend_from_slice(b"]}");
    params
}

fn json_generate_from_keys(
    filename: &str,
    password: &str,
    address: &str,
    viewkey: &str,
    restore_height: u64,
    spendkey: &str,
    language: &str,
) -> Zeroizing<Vec<u8>> {
    let mut params = Zeroizing::new(Vec::new());
    params.extend_from_slice(b"{\"filename\":");
    json_string(&mut params, filename);
    params.extend_from_slice(b",\"password\":");
    json_string(&mut params, password);
    params.extend_from_slice(b",\"address\":");
    json_string(&mut params, address);
    params.extend_from_slice(b",\"spendkey\":");
    json_string(&mut params, spendkey);
    params.extend_from_slice(b",\"viewkey\":");
    json_string(&mut params, viewkey);
    params.extend_from_slice(b",\"restore_height\":");
    params.extend_from_slice(restore_height.to_string().as_bytes());
    params.extend_from_slice(b",\"language\":");
    json_string(&mut params, language);
    params.push(b'}');
    params
}

fn json_restore_params(
    filename: &str,
    password: &str,
    seed: &str,
    restore_height: u64,
    language: &str,
    seed_offset: &str,
) -> Zeroizing<Vec<u8>> {
    let mut params = Zeroizing::new(Vec::new());
    params.extend_from_slice(b"{\"filename\":");
    json_string(&mut params, filename);
    params.extend_from_slice(b",\"password\":");
    json_string(&mut params, password);
    params.extend_from_slice(b",\"seed\":");
    json_string(&mut params, seed);
    params.extend_from_slice(b",\"seed_offset\":");
    json_string(&mut params, seed_offset);
    params.extend_from_slice(b",\"restore_height\":");
    params.extend_from_slice(restore_height.to_string().as_bytes());
    params.extend_from_slice(b",\"language\":");
    json_string(&mut params, language);
    params.push(b'}');
    params
}

fn json_string(output: &mut Vec<u8>, value: &str) {
    output.push(b'"');
    for byte in value.bytes() {
        match byte {
            b'"' | b'\\' => {
                output.push(b'\\');
                output.push(byte);
            }
            b'\n' => output.extend_from_slice(b"\\n"),
            b'\r' => output.extend_from_slice(b"\\r"),
            b'\t' => output.extend_from_slice(b"\\t"),
            byte if byte < 0x20 => {
                const HEX: &[u8; 16] = b"0123456789abcdef";
                output.extend_from_slice(b"\\u00");
                output.push(HEX[(byte >> 4) as usize]);
                output.push(HEX[(byte & 0x0f) as usize]);
            }
            byte => output.push(byte),
        }
    }
    output.push(b'"');
}

fn build_http_request(
    port: u16,
    body: &[u8],
    authorization: Option<&SecretBytes>,
) -> Result<Zeroizing<Vec<u8>>, XmrError> {
    if body.len() > MAX_REQUEST_BODY_BYTES {
        return Err(XmrError::limit());
    }
    let mut request = Zeroizing::new(Vec::with_capacity(body.len() + 512));
    write!(
        request,
        "POST {JSON_RPC_PATH} HTTP/1.1\r\nHost: 127.0.0.1:{port}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n",
        body.len()
    )
    .map_err(|_| XmrError::internal())?;
    if let Some(authorization) = authorization {
        request.extend_from_slice(b"Authorization: ");
        request.extend_from_slice(&authorization.bytes);
        request.extend_from_slice(b"\r\n");
    }
    request.extend_from_slice(b"\r\n");
    request.extend_from_slice(body);
    Ok(request)
}

fn parse_http_response(raw: &[u8]) -> Result<HttpResponse, XmrError> {
    if raw.len() > MAX_HTTP_BYTES {
        return Err(XmrError::limit());
    }
    let boundary = raw
        .windows(4)
        .position(|window| window == b"\r\n\r\n")
        .ok_or_else(XmrError::protocol_incompatible)?;
    let header_end = boundary + 4;
    let header = &raw[..boundary];
    if header
        .iter()
        .any(|byte| (*byte < 0x20 && *byte != b'\r' && *byte != b'\n') || *byte == 0x7f)
    {
        return Err(XmrError::protocol_incompatible());
    }
    let text = core::str::from_utf8(header).map_err(|_| XmrError::protocol_incompatible())?;
    let mut lines = text.split("\r\n");
    let status_line = lines.next().ok_or_else(XmrError::protocol_incompatible)?;
    let status = match status_line {
        "HTTP/1.1 200 OK" => 200,
        "HTTP/1.1 401 Unauthorized" => 401,
        _ => return Err(XmrError::protocol_incompatible()),
    };
    let mut content_length = None;
    let mut connection_close = false;
    let mut challenge = None;
    let mut seen = BTreeSet::new();
    for line in lines {
        if line.is_empty() || line.starts_with(' ') || line.starts_with('\t') {
            return Err(XmrError::protocol_incompatible());
        }
        let (name, value) = line
            .split_once(": ")
            .ok_or_else(XmrError::protocol_incompatible)?;
        if name.is_empty()
            || !name
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
            || value.is_empty()
            || value.bytes().any(|byte| byte.is_ascii_control())
        {
            return Err(XmrError::protocol_incompatible());
        }
        let name = name.to_ascii_lowercase();
        if !seen.insert(name.clone()) {
            return Err(XmrError::protocol_incompatible());
        }
        match name.as_str() {
            "content-length" => {
                if (value.len() > 1 && value.starts_with('0'))
                    || !value.bytes().all(|byte| byte.is_ascii_digit())
                {
                    return Err(XmrError::protocol_incompatible());
                }
                content_length = Some(
                    value
                        .parse::<usize>()
                        .map_err(|_| XmrError::protocol_incompatible())?,
                );
            }
            "connection" if value.eq_ignore_ascii_case("close") => connection_close = true,
            "connection" => return Err(XmrError::protocol_incompatible()),
            "www-authenticate" => {
                if value.len() > MAX_CHALLENGE_BYTES {
                    return Err(XmrError::unauth());
                }
                challenge = Some(Zeroizing::new(value.as_bytes().to_vec()));
            }
            "content-type" if value.eq_ignore_ascii_case("application/json") => {}
            "server" | "date" => {}
            _ => return Err(XmrError::protocol_incompatible()),
        }
    }
    if !connection_close {
        return Err(XmrError::protocol_incompatible());
    }
    let length = content_length.ok_or_else(XmrError::protocol_incompatible)?;
    let end = header_end
        .checked_add(length)
        .ok_or_else(XmrError::protocol_incompatible)?;
    if end != raw.len() {
        return Err(XmrError::protocol_incompatible());
    }
    if status == 200 && challenge.is_some() {
        return Err(XmrError::protocol_incompatible());
    }
    Ok(HttpResponse {
        status,
        challenge,
        body: Zeroizing::new(raw[header_end..end].to_vec()),
    })
}

fn parse_digest_challenge(
    input: &[u8],
    audit: Option<&Arc<WipeAudit>>,
) -> Result<DigestChallenge, XmrError> {
    let text = core::str::from_utf8(input).map_err(|_| XmrError::unauth())?;
    let directives = text.strip_prefix("Digest ").ok_or_else(XmrError::unauth)?;
    let mut realm = None;
    let mut nonce = None;
    let mut opaque = None;
    let mut qop = None;
    let mut algorithm = None;
    let mut seen = BTreeSet::new();
    for item in split_directives(directives)? {
        let (key, raw_value) = item.split_once('=').ok_or_else(XmrError::unauth)?;
        let key = key.trim().to_ascii_lowercase();
        if key.is_empty() || !seen.insert(key.clone()) {
            return Err(XmrError::unauth());
        }
        match key.as_str() {
            "realm" => realm = Some(parse_quoted(raw_value, audit)?),
            "nonce" => nonce = Some(parse_quoted(raw_value, audit)?),
            "opaque" => opaque = Some(parse_quoted(raw_value, audit)?),
            "qop" => qop = Some(parse_token_or_quoted(raw_value)?),
            "algorithm" => algorithm = Some(parse_token(raw_value)?),
            _ => return Err(XmrError::unauth()),
        }
    }
    if qop.as_deref() != Some("auth")
        || algorithm
            .as_deref()
            .is_some_and(|value| !value.eq_ignore_ascii_case("MD5"))
    {
        return Err(XmrError::unauth());
    }
    Ok(DigestChallenge {
        realm: realm.ok_or_else(XmrError::unauth)?,
        nonce: nonce.ok_or_else(XmrError::unauth)?,
        opaque,
    })
}

fn split_directives(input: &str) -> Result<Vec<&str>, XmrError> {
    let bytes = input.as_bytes();
    let mut items = Vec::new();
    let mut start = 0usize;
    let mut quoted = false;
    let mut escaped = false;
    for (index, byte) in bytes.iter().copied().enumerate() {
        if escaped {
            escaped = false;
        } else if quoted && byte == b'\\' {
            escaped = true;
        } else if byte == b'"' {
            quoted = !quoted;
        } else if byte == b',' && !quoted {
            let item = input[start..index].trim();
            if item.is_empty() {
                return Err(XmrError::unauth());
            }
            items.push(item);
            start = index + 1;
        }
    }
    if quoted || escaped {
        return Err(XmrError::unauth());
    }
    let last = input[start..].trim();
    if last.is_empty() {
        return Err(XmrError::unauth());
    }
    items.push(last);
    Ok(items)
}

fn parse_quoted(input: &str, audit: Option<&Arc<WipeAudit>>) -> Result<SecretBytes, XmrError> {
    let value = input.trim();
    if value.len() < 2 || !value.starts_with('"') || !value.ends_with('"') {
        return Err(XmrError::unauth());
    }
    let mut output = Vec::with_capacity(value.len() - 2);
    let mut escaped = false;
    for byte in value.as_bytes()[1..value.len() - 1].iter().copied() {
        if escaped {
            if !matches!(byte, b'"' | b'\\') {
                return Err(XmrError::unauth());
            }
            output.push(byte);
            escaped = false;
        } else if byte == b'\\' {
            escaped = true;
        } else if byte == b'"' || byte.is_ascii_control() {
            return Err(XmrError::unauth());
        } else {
            output.push(byte);
        }
        if output.len() > MAX_CHALLENGE_VALUE_BYTES {
            return Err(XmrError::unauth());
        }
    }
    if escaped || output.is_empty() {
        return Err(XmrError::unauth());
    }
    Ok(SecretBytes::new(output, audit))
}

fn parse_token_or_quoted(input: &str) -> Result<String, XmrError> {
    let value = input.trim();
    if value.starts_with('"') {
        let secret = parse_quoted(value, None)?;
        Ok(secret.text()?.to_owned())
    } else {
        parse_token(value)
    }
}

fn parse_token(input: &str) -> Result<String, XmrError> {
    let value = input.trim();
    if value.is_empty()
        || value.len() > 32
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
    {
        return Err(XmrError::unauth());
    }
    Ok(value.to_owned())
}

fn build_authorization<P: HttpExchangePort>(
    port: &mut P,
    credentials: &WalletCredentials,
    challenge: &DigestChallenge,
    method: RpcMethod,
    audit: Option<&Arc<WipeAudit>>,
) -> Result<SecretBytes, XmrError> {
    let mut source = Zeroizing::new([0u8; 16]);
    port.fill_entropy(&mut *source).map_err(port_error)?;
    let cnonce = SecretBytes::new(hex_bytes(&*source).into_bytes(), audit);
    let username = credentials.username.text()?;
    let password = credentials.password.text()?;
    let realm = challenge.realm.text()?;
    let nonce = challenge.nonce.text()?;
    let ha1 = md5_hex(
        &[
            username.as_bytes(),
            b":",
            realm.as_bytes(),
            b":",
            password.as_bytes(),
        ],
        audit,
    );
    let ha2 = md5_hex(&[b"POST", b":", JSON_RPC_PATH.as_bytes()], audit);
    let response = md5_hex(
        &[
            &ha1.bytes,
            b":",
            nonce.as_bytes(),
            b":00000001:",
            &cnonce.bytes,
            b":auth:",
            &ha2.bytes,
        ],
        audit,
    );
    let mut authorization = SecretBytes::new(Vec::with_capacity(512), audit);
    append_text(&mut authorization.bytes, "Digest username=");
    append_quoted(&mut authorization.bytes, username)?;
    append_text(&mut authorization.bytes, ", realm=");
    append_quoted(&mut authorization.bytes, realm)?;
    append_text(&mut authorization.bytes, ", nonce=");
    append_quoted(&mut authorization.bytes, nonce)?;
    append_text(&mut authorization.bytes, ", uri=\"/json_rpc\", response=\"");
    authorization.bytes.extend_from_slice(&response.bytes);
    append_text(
        &mut authorization.bytes,
        "\", algorithm=MD5, qop=auth, nc=00000001, cnonce=\"",
    );
    authorization.bytes.extend_from_slice(&cnonce.bytes);
    authorization.bytes.push(b'"');
    if let Some(opaque) = &challenge.opaque {
        append_text(&mut authorization.bytes, ", opaque=");
        append_quoted(&mut authorization.bytes, opaque.text()?)?;
    }
    let _ = method;
    Ok(authorization)
}

fn append_text(output: &mut Vec<u8>, text: &str) {
    output.extend_from_slice(text.as_bytes());
}

fn append_quoted(output: &mut Vec<u8>, value: &str) -> Result<(), XmrError> {
    output.push(b'"');
    for byte in value.bytes() {
        if byte.is_ascii_control() {
            return Err(XmrError::unauth());
        }
        if matches!(byte, b'"' | b'\\') {
            output.push(b'\\');
        }
        output.push(byte);
    }
    output.push(b'"');
    Ok(())
}

fn md5_hex(parts: &[&[u8]], audit: Option<&Arc<WipeAudit>>) -> SecretBytes {
    let mut hasher = Md5::new();
    for part in parts {
        hasher.update(part);
    }
    let mut digest = hasher.finalize();
    let output = hex_bytes(&digest).into_bytes();
    digest.zeroize();
    SecretBytes::new(output, audit)
}

fn hex_bytes(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(HEX[(byte >> 4) as usize] as char);
        output.push(HEX[(byte & 0x0f) as usize] as char);
    }
    output
}

fn parse_rpc_body(body: &[u8], method: RpcMethod) -> Result<TypedResult, XmrError> {
    validate_json_document(body)?;
    let mut value: Value =
        serde_json::from_slice(body).map_err(|_| XmrError::protocol_incompatible())?;
    let parsed = (|| {
        let object = value
            .as_object()
            .ok_or_else(XmrError::protocol_incompatible)?;
        if !matches!(object.get("jsonrpc"), Some(Value::String(value)) if value == "2.0")
            || !matches!(object.get("id"), Some(Value::String(value)) if value == JSON_RPC_ID)
        {
            return Err(XmrError::protocol_incompatible());
        }
        let result = object.get("result");
        let error = object.get("error");
        if result.is_some() == error.is_some() || object.len() != 3 {
            return Err(XmrError::protocol_incompatible());
        }
        if let Some(error) = error {
            validate_upstream_error(error)?;
            return Err(XmrError::protocol_incompatible());
        }
        let result = result.ok_or_else(XmrError::protocol_incompatible)?;
        parse_typed_result(result, method)
    })();
    zeroize_json_value(&mut value);
    parsed
}

fn zeroize_json_value(value: &mut Value) {
    match value {
        Value::String(text) => text.zeroize(),
        Value::Array(values) => {
            for value in values {
                zeroize_json_value(value);
            }
        }
        Value::Object(object) => {
            let entries = core::mem::take(object);
            for (mut key, mut value) in entries {
                key.zeroize();
                zeroize_json_value(&mut value);
            }
        }
        Value::Null | Value::Bool(_) | Value::Number(_) => {}
    }
}

fn validate_upstream_error(value: &Value) -> Result<(), XmrError> {
    let object = value
        .as_object()
        .ok_or_else(XmrError::protocol_incompatible)?;
    if object.len() != 2
        || object.get("code").and_then(Value::as_i64).is_none()
        || object.get("message").and_then(Value::as_str).is_none()
    {
        return Err(XmrError::protocol_incompatible());
    }
    Ok(())
}

pub(crate) fn parse_typed_result(
    value: &Value,
    method: RpcMethod,
) -> Result<TypedResult, XmrError> {
    let object = value
        .as_object()
        .ok_or_else(XmrError::protocol_incompatible)?;
    match method {
        RpcMethod::GetVersion => {
            exact_keys(object, &["version", "release"])?;
            let version = required_u64(object, "version")?;
            if version != WALLET_RPC_VERSION || !required_bool(object, "release")? {
                return Err(XmrError::protocol_incompatible());
            }
            Ok(TypedResult::Version(version))
        }
        RpcMethod::GetHeight => {
            exact_keys(object, &["height"])?;
            Ok(TypedResult::Height(required_u64(object, "height")?))
        }
        RpcMethod::GetBalance => {
            exact_keys(
                object,
                &[
                    "balance",
                    "unlocked_balance",
                    "multisig_import_needed",
                    "per_subaddress",
                    "blocks_to_unlock",
                    "time_to_unlock",
                ],
            )?;
            let total = required_u64(object, "balance")?;
            let unlocked = required_u64(object, "unlocked_balance")?;
            if unlocked > total {
                return Err(XmrError::protocol_incompatible());
            }
            let _ = required_bool(object, "multisig_import_needed")?;
            let _ = required_u64(object, "blocks_to_unlock")?;
            let _ = required_u64(object, "time_to_unlock")?;
            let subaddresses = object
                .get("per_subaddress")
                .and_then(Value::as_array)
                .ok_or_else(XmrError::protocol_incompatible)?;
            for value in subaddresses {
                let entry = value
                    .as_object()
                    .ok_or_else(XmrError::protocol_incompatible)?;
                exact_keys(
                    entry,
                    &[
                        "account_index",
                        "address_index",
                        "address",
                        "balance",
                        "unlocked_balance",
                        "label",
                        "num_unspent_outputs",
                        "blocks_to_unlock",
                        "time_to_unlock",
                    ],
                )?;
                let entry_total = required_u64(entry, "balance")?;
                let entry_unlocked = required_u64(entry, "unlocked_balance")?;
                if entry_unlocked > entry_total {
                    return Err(XmrError::protocol_incompatible());
                }
                let _ = required_u32(entry, "account_index")?;
                let _ = required_u32(entry, "address_index")?;
                let _ = required_str(entry, "address")?;
                let _ = required_str(entry, "label")?;
                let _ = required_u64(entry, "num_unspent_outputs")?;
                let _ = required_u64(entry, "blocks_to_unlock")?;
                let _ = required_u64(entry, "time_to_unlock")?;
            }
            Ok(TypedResult::Balance { total, unlocked })
        }
        RpcMethod::GetInfo => parse_node_info(object).map(TypedResult::NodeInfo),
        RpcMethod::HardForkInfo => parse_hard_fork_info(object).map(TypedResult::HardForkInfo),
        RpcMethod::CreateWallet
        | RpcMethod::OpenWallet
        | RpcMethod::CloseWallet
        | RpcMethod::StopWallet => {
            exact_keys(object, &[])?;
            Ok(TypedResult::Empty)
        }
        RpcMethod::RestoreDeterministicWallet => {
            exact_keys(object, &["address", "info", "seed", "was_deprecated"])?;
            let _ = required_str(object, "info")?;
            Ok(TypedResult::Restore {
                address: required_sensitive(object, "address")?,
                seed: required_sensitive(object, "seed")?,
                was_deprecated: required_bool(object, "was_deprecated")?,
            })
        }
        RpcMethod::GenerateFromKeys => {
            exact_keys(object, &["address", "info"])?;
            let _ = required_str(object, "info")?;
            Ok(TypedResult::Generated {
                address: required_sensitive(object, "address")?,
            })
        }
        RpcMethod::QueryKey => {
            exact_keys(object, &["key"])?;
            Ok(TypedResult::Key(required_sensitive(object, "key")?))
        }
        RpcMethod::Refresh => {
            exact_keys(object, &["blocks_fetched", "received_money"])?;
            Ok(TypedResult::Refreshed {
                blocks_fetched: required_u64(object, "blocks_fetched")?,
                received_money: required_bool(object, "received_money")?,
            })
        }
        RpcMethod::GetAddress => parse_addresses(object),
        RpcMethod::CreateAddress => {
            exact_keys(
                object,
                &["address", "address_index", "addresses", "address_indices"],
            )?;
            let address_text = required_str(object, "address")?;
            let address_index = required_u32(object, "address_index")?;
            let addresses = object
                .get("addresses")
                .and_then(Value::as_array)
                .ok_or_else(XmrError::protocol_incompatible)?;
            let address_indices = object
                .get("address_indices")
                .and_then(Value::as_array)
                .ok_or_else(XmrError::protocol_incompatible)?;
            if addresses.len() != 1
                || address_indices.len() != 1
                || addresses.first().and_then(Value::as_str) != Some(address_text)
                || address_indices.first().and_then(Value::as_u64) != Some(u64::from(address_index))
            {
                return Err(XmrError::protocol_incompatible());
            }
            Ok(TypedResult::CreatedAddress {
                address: required_sensitive(object, "address")?,
                address_index,
                address_count: addresses.len(),
            })
        }
        RpcMethod::ValidateAddress => {
            allowed_keys(
                object,
                &[
                    "valid",
                    "integrated",
                    "subaddress",
                    "nettype",
                    "openalias_address",
                ],
            )?;
            Ok(TypedResult::AddressValidation {
                valid: required_bool(object, "valid")?,
                integrated: required_bool(object, "integrated")?,
                subaddress: required_bool(object, "subaddress")?,
                nettype: required_string(object, "nettype")?,
            })
        }
    }
}

fn parse_addresses(object: &Map<String, Value>) -> Result<TypedResult, XmrError> {
    exact_keys(object, &["address", "addresses"])?;
    let values = object
        .get("addresses")
        .and_then(Value::as_array)
        .ok_or_else(XmrError::protocol_incompatible)?;
    let mut addresses = Vec::with_capacity(values.len());
    for value in values {
        let entry = value
            .as_object()
            .ok_or_else(XmrError::protocol_incompatible)?;
        exact_keys(entry, &["address", "address_index", "label", "used"])?;
        addresses.push(AddressEntryResult {
            address: required_sensitive(entry, "address")?,
            address_index: required_u32(entry, "address_index")?,
            label: required_sensitive(entry, "label")?,
            used: required_bool(entry, "used")?,
        });
    }
    Ok(TypedResult::Addresses {
        primary: required_sensitive(object, "address")?,
        addresses,
    })
}

fn parse_node_info(object: &Map<String, Value>) -> Result<NodeInfoResult, XmrError> {
    required_and_optional_keys(
        object,
        &[
            "adjusted_time",
            "alt_blocks_count",
            "block_size_limit",
            "block_size_median",
            "bootstrap_daemon_address",
            "busy_syncing",
            "credits",
            "cumulative_difficulty",
            "cumulative_difficulty_top64",
            "database_size",
            "difficulty",
            "difficulty_top64",
            "free_space",
            "grey_peerlist_size",
            "height",
            "height_without_bootstrap",
            "incoming_connections_count",
            "mainnet",
            "nettype",
            "offline",
            "outgoing_connections_count",
            "restricted",
            "rpc_connections_count",
            "stagenet",
            "start_time",
            "status",
            "synchronized",
            "target",
            "target_height",
            "testnet",
            "top_block_hash",
            "top_hash",
            "tx_count",
            "tx_pool_size",
            "untrusted",
            "update_available",
            "version",
            "was_bootstrap_ever_used",
            "white_peerlist_size",
            "wide_cumulative_difficulty",
            "wide_difficulty",
        ],
        &["block_weight_limit", "block_weight_median"],
    )?;
    for key in [
        "adjusted_time",
        "alt_blocks_count",
        "block_size_limit",
        "block_size_median",
        "credits",
        "cumulative_difficulty",
        "cumulative_difficulty_top64",
        "database_size",
        "difficulty",
        "difficulty_top64",
        "free_space",
        "grey_peerlist_size",
        "height",
        "height_without_bootstrap",
        "incoming_connections_count",
        "outgoing_connections_count",
        "rpc_connections_count",
        "start_time",
        "target",
        "target_height",
        "tx_count",
        "tx_pool_size",
        "white_peerlist_size",
    ] {
        let _ = required_u64(object, key)?;
    }
    for key in ["block_weight_limit", "block_weight_median"] {
        if object
            .get(key)
            .is_some_and(|value| value.as_u64().is_none())
        {
            return Err(XmrError::protocol_incompatible());
        }
    }
    for key in [
        "busy_syncing",
        "mainnet",
        "offline",
        "restricted",
        "stagenet",
        "synchronized",
        "testnet",
        "untrusted",
        "update_available",
        "was_bootstrap_ever_used",
    ] {
        let _ = required_bool(object, key)?;
    }
    for key in [
        "bootstrap_daemon_address",
        "nettype",
        "status",
        "top_block_hash",
        "top_hash",
        "wide_cumulative_difficulty",
        "wide_difficulty",
    ] {
        let _ = required_str(object, key)?;
    }
    let _ = required_bounded_str(object, "version", MAX_NODE_VERSION_BYTES)?;
    Ok(NodeInfoResult {
        status: required_string(object, "status")?,
        nettype: required_string(object, "nettype")?,
        mainnet: required_bool(object, "mainnet")?,
        stagenet: required_bool(object, "stagenet")?,
        testnet: required_bool(object, "testnet")?,
        offline: required_bool(object, "offline")?,
        untrusted: required_bool(object, "untrusted")?,
        bootstrap_daemon_address: required_string(object, "bootstrap_daemon_address")?,
        was_bootstrap_ever_used: required_bool(object, "was_bootstrap_ever_used")?,
        synchronized: required_bool(object, "synchronized")?,
        height: required_u64(object, "height")?,
        target_height: required_u64(object, "target_height")?,
        height_without_bootstrap: required_u64(object, "height_without_bootstrap")?,
    })
}

fn parse_hard_fork_info(object: &Map<String, Value>) -> Result<HardForkInfoResult, XmrError> {
    exact_keys(
        object,
        &[
            "credits",
            "earliest_height",
            "enabled",
            "state",
            "status",
            "threshold",
            "top_hash",
            "untrusted",
            "version",
            "votes",
            "voting",
            "window",
        ],
    )?;
    for key in ["credits", "earliest_height"] {
        let _ = required_u64(object, key)?;
    }
    for key in ["threshold", "votes", "window"] {
        let _ = required_u32(object, key)?;
    }
    for key in ["version", "voting"] {
        let _ = required_u8(object, key)?;
    }
    let state = required_u32(object, "state")?;
    if state > 2 {
        return Err(XmrError::protocol_incompatible());
    }
    let _ = required_str(object, "top_hash")?;
    Ok(HardForkInfoResult {
        status: required_string(object, "status")?,
        earliest_height: required_u64(object, "earliest_height")?,
        enabled: required_bool(object, "enabled")?,
        untrusted: required_bool(object, "untrusted")?,
        version: required_u8(object, "version")?,
    })
}

fn exact_keys(object: &Map<String, Value>, keys: &[&str]) -> Result<(), XmrError> {
    if object.len() == keys.len() && keys.iter().all(|key| object.contains_key(*key)) {
        Ok(())
    } else {
        Err(XmrError::protocol_incompatible())
    }
}

fn required_and_optional_keys(
    object: &Map<String, Value>,
    required: &[&str],
    optional: &[&str],
) -> Result<(), XmrError> {
    if required.iter().all(|key| object.contains_key(*key))
        && object
            .keys()
            .all(|key| required.contains(&key.as_str()) || optional.contains(&key.as_str()))
    {
        Ok(())
    } else {
        Err(XmrError::protocol_incompatible())
    }
}

fn allowed_keys(object: &Map<String, Value>, keys: &[&str]) -> Result<(), XmrError> {
    if object.keys().all(|key| keys.contains(&key.as_str())) {
        Ok(())
    } else {
        Err(XmrError::protocol_incompatible())
    }
}

fn required_u64(object: &Map<String, Value>, key: &str) -> Result<u64, XmrError> {
    object
        .get(key)
        .and_then(Value::as_u64)
        .ok_or_else(XmrError::protocol_incompatible)
}

fn required_u32(object: &Map<String, Value>, key: &str) -> Result<u32, XmrError> {
    u32::try_from(required_u64(object, key)?).map_err(|_| XmrError::protocol_incompatible())
}

fn required_u8(object: &Map<String, Value>, key: &str) -> Result<u8, XmrError> {
    u8::try_from(required_u64(object, key)?).map_err(|_| XmrError::protocol_incompatible())
}

fn required_bool(object: &Map<String, Value>, key: &str) -> Result<bool, XmrError> {
    object
        .get(key)
        .and_then(Value::as_bool)
        .ok_or_else(XmrError::protocol_incompatible)
}

fn required_string(object: &Map<String, Value>, key: &str) -> Result<String, XmrError> {
    required_str(object, key).map(str::to_owned)
}

fn required_str<'a>(object: &'a Map<String, Value>, key: &str) -> Result<&'a str, XmrError> {
    object
        .get(key)
        .and_then(Value::as_str)
        .ok_or_else(XmrError::protocol_incompatible)
}

fn required_bounded_str<'a>(
    object: &'a Map<String, Value>,
    key: &str,
    maximum_bytes: usize,
) -> Result<&'a str, XmrError> {
    required_str(object, key).and_then(|value| {
        if !value.is_empty() && value.len() <= maximum_bytes {
            Ok(value)
        } else {
            Err(XmrError::protocol_incompatible())
        }
    })
}

fn required_sensitive(
    object: &Map<String, Value>,
    key: &str,
) -> Result<SensitiveRpcText, XmrError> {
    object
        .get(key)
        .and_then(Value::as_str)
        .map(|value| SensitiveRpcText(SecretBytes::new(value.as_bytes().to_vec(), None)))
        .ok_or_else(XmrError::protocol_incompatible)
}

pub(crate) fn evaluate_node_info(
    network: XmrNetwork,
    info: NodeInfoResult,
) -> Result<NodeProbeResult, XmrError> {
    let network_exact = match network {
        XmrNetwork::Stagenet => {
            info.nettype == "stagenet" && !info.mainnet && info.stagenet && !info.testnet
        }
        XmrNetwork::Testnet => {
            info.nettype == "testnet" && !info.mainnet && !info.stagenet && info.testnet
        }
    };
    if info.status != "OK"
        || !network_exact
        || info.offline
        || info.untrusted
        || !info.bootstrap_daemon_address.is_empty()
        || info.height_without_bootstrap > info.height
    {
        return Err(XmrError::node_unavailable());
    }
    Ok(NodeProbeResult {
        state: if info.synchronized {
            NodeState::Ready
        } else {
            NodeState::Syncing
        },
        height: info.height,
        height_without_bootstrap: info.height_without_bootstrap,
    })
}

pub(crate) fn evaluate_node_policy(
    network: XmrNetwork,
    info: NodeInfoResult,
    hard_fork: HardForkInfoResult,
) -> Result<NodeProbeResult, XmrError> {
    if hard_fork.status != "OK" || hard_fork.untrusted {
        return Err(XmrError::node_unavailable());
    }
    evaluate_node_info(network, info)
}

pub(crate) fn node_port(network: XmrNetwork) -> u16 {
    network.daemon_port()
}

pub(crate) fn probe_node_with<P: HttpExchangePort>(
    core: &mut RpcCore<P>,
    network: XmrNetwork,
) -> Result<NodeProbeResult, XmrError> {
    let port = node_port(network);
    let info = match core.call_node(port, RpcRequest::GetInfo) {
        Ok(TypedResult::NodeInfo(info)) => info,
        _ => return Err(XmrError::node_unavailable()),
    };
    let hard_fork = match core.call_node(port, RpcRequest::HardForkInfo) {
        Ok(TypedResult::HardForkInfo(info)) => info,
        _ => return Err(XmrError::node_unavailable()),
    };
    evaluate_node_policy(network, info, hard_fork)
}

pub(crate) fn probe_node_view_with<P: HttpExchangePort>(
    core: &mut RpcCore<P>,
    network: XmrNetwork,
) -> Result<NodeViewProbeResult, XmrError> {
    let port = node_port(network);
    let info = match core.call_node(port, RpcRequest::GetInfo) {
        Ok(TypedResult::NodeInfo(info)) => info,
        _ => return Err(XmrError::node_unavailable()),
    };
    let hard_fork = match core.call_node(port, RpcRequest::HardForkInfo) {
        Ok(TypedResult::HardForkInfo(info)) => info,
        _ => return Err(XmrError::node_unavailable()),
    };
    let node = evaluate_node_policy(network, info, hard_fork.clone())?;
    Ok(NodeViewProbeResult {
        state: node.state,
        height: node.height,
        hard_fork,
    })
}

pub fn probe_local_node(network: &str) -> Result<(), XmrError> {
    let network = XmrNetwork::parse(network)?;
    probe_local_node_state(network).map(|_| ())
}

pub(crate) fn probe_local_node_state(network: XmrNetwork) -> Result<NodeProbeResult, XmrError> {
    let mut core = RpcCore::new(SystemHttpPort::new());
    probe_node_with(&mut core, network)
}

pub(crate) fn probe_local_node_view(network: XmrNetwork) -> Result<NodeViewProbeResult, XmrError> {
    let mut core = RpcCore::new(SystemHttpPort::new());
    probe_node_view_with(&mut core, network)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum WalletRpcPhase {
    Authenticated,
    FreshSoftware,
    MnemonicConsumed,
    WalletBound,
}

struct WalletSession {
    port: u16,
    credentials: WalletCredentials,
    phase: WalletRpcPhase,
}

pub(crate) struct SystemWalletRpcControl {
    core: RpcCore<SystemHttpPort>,
    session: Option<WalletSession>,
}

impl SystemWalletRpcControl {
    pub fn new() -> Self {
        Self {
            core: RpcCore::new(SystemHttpPort::new()),
            session: None,
        }
    }

    fn session_call(&mut self, request: RpcRequest) -> Result<TypedResult, XmrError> {
        let session = self.session.as_ref().ok_or_else(XmrError::unavailable)?;
        self.core.call_wallet(
            session.port,
            session.credentials.username.text()?,
            session.credentials.password.text()?,
            request,
        )
    }

    fn require_phase(&self, allowed: &[WalletRpcPhase]) -> Result<WalletRpcPhase, XmrError> {
        let phase = self
            .session
            .as_ref()
            .map(|session| session.phase)
            .ok_or_else(XmrError::unavailable)?;
        if allowed.contains(&phase) {
            Ok(phase)
        } else {
            Err(XmrError::request_schema())
        }
    }

    fn set_phase(&mut self, phase: WalletRpcPhase) -> Result<(), XmrError> {
        let session = self.session.as_mut().ok_or_else(XmrError::unavailable)?;
        session.phase = phase;
        Ok(())
    }

    fn require_empty(&mut self, request: RpcRequest) -> Result<(), XmrError> {
        match self.session_call(request)? {
            TypedResult::Empty => Ok(()),
            _ => Err(XmrError::protocol_incompatible()),
        }
    }

    fn primary_from_result(result: TypedResult) -> Result<Zeroizing<String>, XmrError> {
        match result {
            TypedResult::Addresses { primary, addresses } => {
                let text = Zeroizing::new(primary.expose()?.to_owned());
                drop(addresses);
                if text.len() != 95 {
                    return Err(XmrError::protocol_incompatible());
                }
                Ok(text)
            }
            TypedResult::Generated { address } | TypedResult::Restore { address, .. } => {
                let text = Zeroizing::new(address.expose()?.to_owned());
                if text.len() != 95 {
                    return Err(XmrError::protocol_incompatible());
                }
                Ok(text)
            }
            _ => Err(XmrError::protocol_incompatible()),
        }
    }

    fn expected_nettype(network: XmrNetwork) -> &'static str {
        match network {
            XmrNetwork::Stagenet => "stagenet",
            XmrNetwork::Testnet => "testnet",
        }
    }
}

impl Default for SystemWalletRpcControl {
    fn default() -> Self {
        Self::new()
    }
}

impl WalletRpcControl for SystemWalletRpcControl {
    fn readiness(
        &mut self,
        rpc_port: u16,
        username: &str,
        password: &str,
        connect_timeout: Duration,
    ) -> Result<ReadinessStatus, XmrError> {
        if connect_timeout != Duration::from_secs(CONNECT_TIMEOUT_SECS) {
            return Err(XmrError::unavailable());
        }
        self.close_sockets();
        self.session = Some(WalletSession {
            port: rpc_port,
            credentials: WalletCredentials {
                username: SecretBytes::new(username.as_bytes().to_vec(), None),
                password: SecretBytes::new(password.as_bytes().to_vec(), None),
            },
            phase: WalletRpcPhase::Authenticated,
        });
        let started = Instant::now();
        let session = self.session.as_ref().ok_or_else(XmrError::unavailable)?;
        let result = self.core.readiness_wallet(
            session.port,
            session.credentials.username.text()?,
            session.credentials.password.text()?,
        );
        match result {
            Ok(TypedResult::Version(version)) if version == WALLET_RPC_VERSION => {
                Ok(ReadinessStatus {
                    authenticated: true,
                    // Process construction is already gated by the verified distribution capability;
                    // this exact authenticated RPC proof bridges to its accepted sentinel.
                    version: VERIFIED_VERSION.to_owned(),
                    elapsed_millis: u64::try_from(started.elapsed().as_millis())
                        .unwrap_or(u64::MAX),
                })
            }
            Ok(_) => {
                self.close_sockets();
                Err(XmrError::protocol_incompatible())
            }
            Err(error) => {
                self.close_sockets();
                Err(error)
            }
        }
    }

    fn close_wallet(&mut self) -> Result<(), XmrError> {
        let result = self.require_empty(RpcRequest::CloseWallet);
        if result.is_ok() {
            let _ = self.set_phase(WalletRpcPhase::Authenticated);
        }
        result
    }

    fn stop_wallet(&mut self) -> Result<(), XmrError> {
        self.require_empty(RpcRequest::StopWallet)
    }

    fn refresh(&mut self) -> Result<(), XmrError> {
        self.require_phase(&[WalletRpcPhase::WalletBound])?;
        match self.session_call(RpcRequest::Refresh)? {
            TypedResult::Refreshed { .. } => Ok(()),
            _ => Err(XmrError::protocol_incompatible()),
        }
    }

    fn close_sockets(&mut self) {
        self.core.close_all();
        self.session = None;
    }

    fn create_wallet(&mut self, filename: &str, password: &str) -> Result<(), XmrError> {
        self.require_phase(&[WalletRpcPhase::Authenticated])?;
        self.require_empty(RpcRequest::create_wallet(filename, password)?)?;
        self.set_phase(WalletRpcPhase::FreshSoftware)
    }

    fn query_mnemonic(&mut self) -> Result<Zeroizing<String>, XmrError> {
        self.require_phase(&[WalletRpcPhase::FreshSoftware])?;
        let mnemonic = match self.session_call(RpcRequest::query_key_mnemonic())? {
            TypedResult::Key(key) => Zeroizing::new(key.expose()?.to_owned()),
            _ => return Err(XmrError::protocol_incompatible()),
        };
        self.set_phase(WalletRpcPhase::MnemonicConsumed)?;
        Ok(mnemonic)
    }

    fn get_primary_address(&mut self, network: XmrNetwork) -> Result<Zeroizing<String>, XmrError> {
        self.require_phase(&[
            WalletRpcPhase::FreshSoftware,
            WalletRpcPhase::MnemonicConsumed,
            WalletRpcPhase::WalletBound,
        ])?;
        let address = Self::primary_from_result(self.session_call(RpcRequest::get_address())?)?;
        self.validate_primary_address(&address, network)?;
        Ok(address)
    }

    fn validate_primary_address(
        &mut self,
        address: &str,
        network: XmrNetwork,
    ) -> Result<(), XmrError> {
        self.require_phase(&[
            WalletRpcPhase::Authenticated,
            WalletRpcPhase::FreshSoftware,
            WalletRpcPhase::MnemonicConsumed,
            WalletRpcPhase::WalletBound,
        ])?;
        match self.session_call(RpcRequest::validate_address(address)?)? {
            TypedResult::AddressValidation {
                valid,
                integrated,
                subaddress,
                nettype,
            } if valid
                && !integrated
                && !subaddress
                && nettype == Self::expected_nettype(network) =>
            {
                Ok(())
            }
            TypedResult::AddressValidation { .. } => Err(XmrError::protocol_incompatible()),
            _ => Err(XmrError::protocol_incompatible()),
        }
    }

    fn generate_from_keys(
        &mut self,
        filename: &str,
        password: &str,
        address: &str,
        viewkey: &str,
        restore_height: u64,
    ) -> Result<Zeroizing<String>, XmrError> {
        self.require_phase(&[WalletRpcPhase::Authenticated])?;
        let result = self.session_call(RpcRequest::generate_from_keys(
            filename,
            password,
            address,
            viewkey,
            restore_height,
        )?)?;
        let primary = Self::primary_from_result(result)?;
        self.set_phase(WalletRpcPhase::WalletBound)?;
        Ok(primary)
    }

    fn open_wallet(&mut self, filename: &str, password: &str) -> Result<(), XmrError> {
        self.require_phase(&[WalletRpcPhase::Authenticated])?;
        self.require_empty(RpcRequest::open_wallet(filename, password)?)?;
        self.set_phase(WalletRpcPhase::WalletBound)
    }

    fn restore_deterministic_wallet(
        &mut self,
        filename: &str,
        password: &str,
        seed: &str,
        restore_height: u64,
    ) -> Result<Zeroizing<String>, XmrError> {
        self.require_phase(&[WalletRpcPhase::Authenticated])?;
        let result = self.session_call(RpcRequest::restore_deterministic_wallet(
            filename,
            password,
            seed,
            restore_height,
        )?)?;
        let primary = Self::primary_from_result(result)?;
        self.set_phase(WalletRpcPhase::WalletBound)?;
        Ok(primary)
    }

    fn get_height(&mut self) -> Result<u64, XmrError> {
        self.require_phase(&[
            WalletRpcPhase::FreshSoftware,
            WalletRpcPhase::MnemonicConsumed,
            WalletRpcPhase::WalletBound,
        ])?;
        match self.session_call(RpcRequest::GetHeight)? {
            TypedResult::Height(height) => Ok(height),
            _ => Err(XmrError::protocol_incompatible()),
        }
    }

    fn get_balance(&mut self) -> Result<(u64, u64), XmrError> {
        self.require_phase(&[
            WalletRpcPhase::FreshSoftware,
            WalletRpcPhase::MnemonicConsumed,
            WalletRpcPhase::WalletBound,
        ])?;
        match self.session_call(RpcRequest::GetBalance)? {
            TypedResult::Balance { total, unlocked } => Ok((total, unlocked)),
            _ => Err(XmrError::protocol_incompatible()),
        }
    }

    fn create_address(&mut self) -> Result<(Zeroizing<String>, u32), XmrError> {
        self.require_phase(&[WalletRpcPhase::WalletBound])?;
        match self.session_call(RpcRequest::create_address())? {
            TypedResult::CreatedAddress {
                address,
                address_index,
                address_count,
            } => {
                if address_count != 1 || address_index == 0 {
                    return Err(XmrError::protocol_incompatible());
                }
                Ok((Zeroizing::new(address.expose()?.to_owned()), address_index))
            }
            _ => Err(XmrError::protocol_incompatible()),
        }
    }

    fn validate_subaddress(&mut self, address: &str, network: XmrNetwork) -> Result<(), XmrError> {
        self.require_phase(&[WalletRpcPhase::WalletBound])?;
        match self.session_call(RpcRequest::validate_address(address)?)? {
            TypedResult::AddressValidation {
                valid,
                integrated,
                subaddress,
                nettype,
            } if valid
                && !integrated
                && subaddress
                && nettype == Self::expected_nettype(network) =>
            {
                Ok(())
            }
            TypedResult::AddressValidation { .. } => Err(XmrError::protocol_incompatible()),
            _ => Err(XmrError::protocol_incompatible()),
        }
    }

    fn get_indexed_address(
        &mut self,
        account_index: u32,
        address_index: u32,
    ) -> Result<Zeroizing<String>, XmrError> {
        self.require_phase(&[WalletRpcPhase::WalletBound])?;
        match self.session_call(RpcRequest::get_address_at(account_index, address_index))? {
            TypedResult::Addresses { primary, addresses } => {
                drop(primary);
                match addresses.as_slice() {
                    [entry] if entry.address_index == address_index => {
                        Ok(Zeroizing::new(entry.address.expose()?.to_owned()))
                    }
                    _ => Err(XmrError::protocol_incompatible()),
                }
            }
            _ => Err(XmrError::protocol_incompatible()),
        }
    }
}

impl Drop for SystemWalletRpcControl {
    fn drop(&mut self) {
        self.close_sockets();
    }
}

pub(crate) struct DigestResponseInput<'a> {
    pub(crate) username: &'a str,
    pub(crate) password: &'a str,
    pub(crate) realm: &'a str,
    pub(crate) nonce: &'a str,
    pub(crate) uri: &'a str,
    pub(crate) method: &'a str,
    pub(crate) qop: &'a str,
    pub(crate) nc: &'a str,
    pub(crate) cnonce: &'a str,
}

pub(crate) fn digest_response_for_test(input: DigestResponseInput<'_>) -> Result<String, XmrError> {
    let DigestResponseInput {
        username,
        password,
        realm,
        nonce,
        uri,
        method,
        qop,
        nc,
        cnonce,
    } = input;
    if qop != "auth" || nc != "00000001" {
        return Err(XmrError::unauth());
    }
    let ha1 = md5_hex(
        &[
            username.as_bytes(),
            b":",
            realm.as_bytes(),
            b":",
            password.as_bytes(),
        ],
        None,
    );
    let ha2 = md5_hex(&[method.as_bytes(), b":", uri.as_bytes()], None);
    let response = md5_hex(
        &[
            &ha1.bytes,
            b":",
            nonce.as_bytes(),
            b":",
            nc.as_bytes(),
            b":",
            cnonce.as_bytes(),
            b":",
            qop.as_bytes(),
            b":",
            &ha2.bytes,
        ],
        None,
    );
    response.text().map(str::to_owned)
}

pub(crate) fn request_body_boundary_for_test(length: usize) -> Result<usize, XmrError> {
    build_http_request(49_152, &vec![b'x'; length], None).map(|request| request.len())
}

pub(crate) fn validate_json_for_test(input: &[u8]) -> Result<(), XmrError> {
    validate_json_document(input)
}

fn validate_json_document(input: &[u8]) -> Result<(), XmrError> {
    if input.starts_with(&[0xef, 0xbb, 0xbf]) {
        return Err(XmrError::protocol_incompatible());
    }
    core::str::from_utf8(input).map_err(|_| XmrError::protocol_incompatible())?;
    let mut cursor = JsonCursor { input, offset: 0 };
    cursor.whitespace();
    cursor.value(0)?;
    cursor.whitespace();
    if cursor.offset == input.len() {
        Ok(())
    } else {
        Err(XmrError::protocol_incompatible())
    }
}

struct JsonCursor<'a> {
    input: &'a [u8],
    offset: usize,
}

impl JsonCursor<'_> {
    fn whitespace(&mut self) {
        while matches!(self.peek(), Some(b' ' | b'\n' | b'\r' | b'\t')) {
            self.offset += 1;
        }
    }

    fn peek(&self) -> Option<u8> {
        self.input.get(self.offset).copied()
    }

    fn value(&mut self, depth: usize) -> Result<(), XmrError> {
        self.whitespace();
        match self.peek() {
            Some(b'{') => self.object(depth + 1),
            Some(b'[') => self.array(depth + 1),
            Some(b'"') => self.string().map(|_| ()),
            Some(b't') => self.literal(b"true"),
            Some(b'f') => self.literal(b"false"),
            Some(b'n') => self.literal(b"null"),
            Some(b'-' | b'0'..=b'9') => self.number(),
            _ => Err(XmrError::protocol_incompatible()),
        }
    }

    fn object(&mut self, depth: usize) -> Result<(), XmrError> {
        if depth > MAX_JSON_NESTING {
            return Err(XmrError::protocol_incompatible());
        }
        self.offset += 1;
        self.whitespace();
        let mut keys = BTreeSet::new();
        if self.peek() == Some(b'}') {
            self.offset += 1;
            return Ok(());
        }
        loop {
            let key_slice = self.string()?;
            let key: String =
                serde_json::from_slice(key_slice).map_err(|_| XmrError::protocol_incompatible())?;
            if !keys.insert(key) {
                return Err(XmrError::protocol_incompatible());
            }
            self.whitespace();
            if self.peek() != Some(b':') {
                return Err(XmrError::protocol_incompatible());
            }
            self.offset += 1;
            self.value(depth)?;
            self.whitespace();
            match self.peek() {
                Some(b',') => {
                    self.offset += 1;
                    self.whitespace();
                }
                Some(b'}') => {
                    self.offset += 1;
                    return Ok(());
                }
                _ => return Err(XmrError::protocol_incompatible()),
            }
        }
    }

    fn array(&mut self, depth: usize) -> Result<(), XmrError> {
        if depth > MAX_JSON_NESTING {
            return Err(XmrError::protocol_incompatible());
        }
        self.offset += 1;
        self.whitespace();
        if self.peek() == Some(b']') {
            self.offset += 1;
            return Ok(());
        }
        loop {
            self.value(depth)?;
            self.whitespace();
            match self.peek() {
                Some(b',') => {
                    self.offset += 1;
                    self.whitespace();
                }
                Some(b']') => {
                    self.offset += 1;
                    return Ok(());
                }
                _ => return Err(XmrError::protocol_incompatible()),
            }
        }
    }

    fn string(&mut self) -> Result<&[u8], XmrError> {
        let start = self.offset;
        if self.peek() != Some(b'"') {
            return Err(XmrError::protocol_incompatible());
        }
        self.offset += 1;
        let mut escaped = false;
        while let Some(byte) = self.peek() {
            self.offset += 1;
            if escaped {
                escaped = false;
                if byte == b'u' {
                    for _ in 0..4 {
                        let hex = self.peek().ok_or_else(XmrError::protocol_incompatible)?;
                        if !hex.is_ascii_hexdigit() {
                            return Err(XmrError::protocol_incompatible());
                        }
                        self.offset += 1;
                    }
                } else if !matches!(byte, b'"' | b'\\' | b'/' | b'b' | b'f' | b'n' | b'r' | b't') {
                    return Err(XmrError::protocol_incompatible());
                }
            } else if byte == b'\\' {
                escaped = true;
            } else if byte == b'"' {
                return Ok(&self.input[start..self.offset]);
            } else if byte < 0x20 {
                return Err(XmrError::protocol_incompatible());
            }
        }
        Err(XmrError::protocol_incompatible())
    }

    fn literal(&mut self, expected: &[u8]) -> Result<(), XmrError> {
        if self.input.get(self.offset..self.offset + expected.len()) == Some(expected) {
            self.offset += expected.len();
            Ok(())
        } else {
            Err(XmrError::protocol_incompatible())
        }
    }

    fn number(&mut self) -> Result<(), XmrError> {
        let start = self.offset;
        if self.peek() == Some(b'-') {
            self.offset += 1;
        }
        match self.peek() {
            Some(b'0') => self.offset += 1,
            Some(b'1'..=b'9') => {
                self.offset += 1;
                while matches!(self.peek(), Some(b'0'..=b'9')) {
                    self.offset += 1;
                }
            }
            _ => return Err(XmrError::protocol_incompatible()),
        }
        if self.peek() == Some(b'.') {
            self.offset += 1;
            let fraction = self.offset;
            while matches!(self.peek(), Some(b'0'..=b'9')) {
                self.offset += 1;
            }
            if self.offset == fraction {
                return Err(XmrError::protocol_incompatible());
            }
        }
        if matches!(self.peek(), Some(b'e' | b'E')) {
            self.offset += 1;
            if matches!(self.peek(), Some(b'+' | b'-')) {
                self.offset += 1;
            }
            let exponent = self.offset;
            while matches!(self.peek(), Some(b'0'..=b'9')) {
                self.offset += 1;
            }
            if self.offset == exponent {
                return Err(XmrError::protocol_incompatible());
            }
        }
        if self.offset == start {
            Err(XmrError::protocol_incompatible())
        } else {
            Ok(())
        }
    }
}
