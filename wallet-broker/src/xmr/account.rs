use core::fmt;
use std::fs::{self, File, OpenOptions};
use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

#[cfg(target_os = "linux")]
use std::os::unix::fs::{FileTypeExt, MetadataExt, OpenOptionsExt, PermissionsExt};

use zeroize::{Zeroize, Zeroizing};

#[cfg(target_os = "linux")]
use crate::store::{LinuxStorePort, VaultStore};
use crate::vault::{
    Asset, EntropyPort, Network as VaultNetwork, OsEntropy, SecretBytes, VaultError, VaultMetadata,
    VaultWorkObserver, WipeEvent, WipeObserver, open_vault_bytes, parse_vault, seal_vault,
    valid_account_id,
};
#[cfg(target_os = "linux")]
use crate::xmr::distribution::InstallationVerifier;
use crate::xmr::model::{NodeState, WalletState, XmrError, XmrNetwork};
#[cfg(target_os = "linux")]
use crate::xmr::process::WalletRpcProcessPool;
#[cfg(target_os = "linux")]
use crate::xmr::receiver::{
    AddressClassification, CreatedSubaddress, ProductionViewInput, ReceiverPort,
    binding_from_stored, build_production_view, expected_nettype, issue_fresh_with_port,
    stored_from_binding,
};
use crate::xmr::receiver::{FreshReceiver, SanitizedAccountView};
#[cfg(target_os = "linux")]
use crate::xmr::rpc::{
    NodeState as RpcNodeState, SystemWalletRpcControl, probe_local_node_state,
    probe_local_node_view,
};
use crate::xmr::store::{
    AccountStore, DIRECTORY_MODE, PathSqliteSurface, ReceiverPersistenceProof, ReceiverSchemaView,
    STATE_FILE_MODE, SYNCHRONOUS_FULL, StoredIdentity, state_file_name,
};

pub const XMR_SECRET_MAGIC: [u8; 8] = *b"BBXMR001";
pub const PASSWORD_HEX_BYTES: usize = 64;
pub const PRIMARY_ADDRESS_BYTES: usize = 95;
pub const MNEMONIC_WORDS: usize = 25;
pub const VIEW_KEY_HEX_BYTES: usize = 64;
pub const MAX_SECRET_BYTES: usize = 2_048;
pub const RESTORE_SAFETY_MARGIN: u64 = 100;
pub const WALLET_FILE_MODE: u32 = 0o600;
pub const KEYS_FILE_MODE: u32 = 0o600;
const PASSWORD_SOURCE_BYTES: usize = 32;
const KIND_SOFTWARE: u8 = 1;
const KIND_WATCH_ONLY: u8 = 2;
const HEADER_BYTES: usize = 8 + 1 + 8 + 2 + 64 + 2 + 95 + 2;
const ENGLISH_LANGUAGE: &str = "English";
const MONERO_BASE58: &[u8] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
#[cfg(target_os = "linux")]
const LINUX_O_NOFOLLOW: i32 = 0o400_000;
#[cfg(target_os = "linux")]
const LINUX_O_DIRECTORY: i32 = 0o200_000;
const VAULT_EPOCH: u64 = 1;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AccountKind {
    Software,
    WatchOnly,
}

impl AccountKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Software => "software",
            Self::WatchOnly => "watch_only",
        }
    }

    pub fn parse(value: &str) -> Result<Self, XmrError> {
        match value {
            "software" => Ok(Self::Software),
            "watch_only" => Ok(Self::WatchOnly),
            _ => Err(XmrError::request_schema()),
        }
    }

    pub(crate) fn code(self) -> u8 {
        match self {
            Self::Software => KIND_SOFTWARE,
            Self::WatchOnly => KIND_WATCH_ONLY,
        }
    }

    pub(crate) fn from_code(code: u8) -> Result<Self, XmrError> {
        match code {
            KIND_SOFTWARE => Ok(Self::Software),
            KIND_WATCH_ONLY => Ok(Self::WatchOnly),
            _ => Err(XmrError::state_corrupt()),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SecretExit {
    Success,
    Error,
    Cancellation,
    PanicUnwind,
    Drop,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HostileWalletEntry {
    Symlink,
    Directory,
    Fifo,
    WrongOwner,
    WrongMode,
    CrossAccount,
    CrossNetwork,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AccountNetwork(XmrNetwork);

impl AccountNetwork {
    pub fn as_str(self) -> &'static str {
        self.0.name()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AccountCapabilities {
    pub can_view: bool,
    pub can_derive_fresh_receiver: bool,
    pub can_receive_private: bool,
    pub can_receive_orchard: bool,
    pub can_receive_ironwood: bool,
    pub can_prepare_tx: bool,
    pub can_sign_spend: bool,
    pub can_sign_orchard: bool,
    pub can_sign_ironwood: bool,
    pub can_tx_v6: bool,
    pub can_migrate_orchard_to_ironwood: bool,
    pub can_sign_transparent: bool,
    pub can_display_amount_on_device: bool,
    pub can_display_recipient_on_device: bool,
    pub can_display_network_on_device: bool,
    pub can_verify_pczt_on_device: bool,
    pub can_export_viewing_material: bool,
    pub can_broadcast: bool,
}

impl AccountCapabilities {
    pub fn viewing_only() -> Self {
        Self {
            can_view: true,
            can_derive_fresh_receiver: true,
            can_receive_private: true,
            can_receive_orchard: false,
            can_receive_ironwood: false,
            can_prepare_tx: false,
            can_sign_spend: false,
            can_sign_orchard: false,
            can_sign_ironwood: false,
            can_tx_v6: false,
            can_migrate_orchard_to_ironwood: false,
            can_sign_transparent: false,
            can_display_amount_on_device: false,
            can_display_recipient_on_device: false,
            can_display_network_on_device: false,
            can_verify_pczt_on_device: false,
            can_export_viewing_material: false,
            can_broadcast: false,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreatedAccount {
    pub account_id: String,
    pub kind: AccountKind,
    pub network: AccountNetwork,
    pub restore_height: u64,
    pub capabilities: AccountCapabilities,
}

impl CreatedAccount {
    pub fn contains_primary_address(&self) -> bool {
        let rendered = format!("{self:?}").to_ascii_lowercase();
        rendered.contains("primary_address") || rendered.contains("primary-address")
    }

    pub fn contains_secret(&self) -> bool {
        let rendered = format!("{self:?}").to_ascii_lowercase();
        [
            "mnemonic",
            "view_key",
            "wallet_password",
            "seed",
            "spendkey",
            "private_view",
        ]
        .iter()
        .any(|needle| rendered.contains(needle))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccountPathsView {
    pub relative_account_directory: String,
    pub wallet_filename: String,
    pub directory_mode: u32,
    pub wallet_file_mode: u32,
    pub keys_file_mode: u32,
    pub state_db_mode: u32,
    pub state_db_synchronous: &'static str,
}

pub(crate) struct WalletPassword {
    encoded: SecretBytes,
    source: &'static str,
    source_bytes: usize,
}

impl WalletPassword {
    pub fn observation(&self) -> WalletPasswordObservation {
        WalletPasswordObservation {
            source: self.source,
            source_bytes: self.source_bytes,
            encoded: self
                .encoded
                .expose(|bytes| utf8(bytes).map(str::to_owned).unwrap_or_default()),
        }
    }

    fn copy_text(&self) -> Result<Zeroizing<String>, XmrError> {
        self.encoded
            .expose(|bytes| utf8(bytes).map(|value| Zeroizing::new(value.to_owned())))
    }

    fn wipe(&mut self, observer: &mut dyn WipeObserver) {
        self.encoded.wipe_with("xmr-wallet-password", observer);
    }
}

impl Drop for WalletPassword {
    fn drop(&mut self) {
        let mut observer = IgnoringWipe;
        self.wipe(&mut observer);
    }
}

impl fmt::Debug for WalletPassword {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("WalletPassword([REDACTED])")
    }
}

impl fmt::Display for WalletPassword {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("[REDACTED]")
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct WalletPasswordObservation {
    pub source: &'static str,
    pub source_bytes: usize,
    pub encoded: String,
}

impl Drop for WalletPasswordObservation {
    fn drop(&mut self) {
        self.encoded.zeroize();
    }
}

impl fmt::Debug for WalletPasswordObservation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("WalletPasswordObservation([REDACTED])")
    }
}

pub(crate) struct XmrSecretV1 {
    kind: AccountKind,
    restore_height: u64,
    wallet_password: SecretBytes,
    primary_address: SecretBytes,
    secret: SecretBytes,
}

impl XmrSecretV1 {
    pub fn software(
        restore_height: u64,
        password: &str,
        primary: &str,
        mnemonic: &str,
    ) -> Result<Self, XmrError> {
        Self::new(
            AccountKind::Software,
            restore_height,
            password,
            primary,
            mnemonic,
        )
    }

    pub fn watch_only(
        restore_height: u64,
        password: &str,
        primary: &str,
        view_key: &str,
    ) -> Result<Self, XmrError> {
        Self::new(
            AccountKind::WatchOnly,
            restore_height,
            password,
            primary,
            view_key,
        )
    }

    fn new(
        kind: AccountKind,
        restore_height: u64,
        password: &str,
        primary: &str,
        secret: &str,
    ) -> Result<Self, XmrError> {
        validate_password_hex_length(password.len())?;
        validate_password_hex(password.as_bytes())?;
        validate_primary_address_length(primary.len())?;
        validate_secret_payload(kind, secret.as_bytes())?;
        Ok(Self {
            kind,
            restore_height,
            wallet_password: SecretBytes::new(password.as_bytes().to_vec())
                .map_err(|_| XmrError::state_corrupt())?,
            primary_address: SecretBytes::new(primary.as_bytes().to_vec())
                .map_err(|_| XmrError::state_corrupt())?,
            secret: SecretBytes::new(secret.as_bytes().to_vec())
                .map_err(|_| XmrError::state_corrupt())?,
        })
    }

    pub fn kind(&self) -> AccountKind {
        self.kind
    }

    pub fn restore_height(&self) -> u64 {
        self.restore_height
    }

    pub fn expose_wallet_password<T>(&self, expose: impl FnOnce(&str) -> T) -> Result<T, XmrError> {
        self.wallet_password.expose(|bytes| utf8(bytes).map(expose))
    }

    pub fn expose_primary_address<T>(&self, expose: impl FnOnce(&str) -> T) -> Result<T, XmrError> {
        self.primary_address.expose(|bytes| utf8(bytes).map(expose))
    }

    pub fn expose_mnemonic<T>(
        &self,
        expose: impl FnOnce(&str) -> T,
    ) -> Result<Option<T>, XmrError> {
        if self.kind != AccountKind::Software {
            return Ok(None);
        }
        self.secret
            .expose(|bytes| utf8(bytes).map(|value| Some(expose(value))))
    }

    pub fn expose_private_view_key<T>(
        &self,
        expose: impl FnOnce(&str) -> T,
    ) -> Result<Option<T>, XmrError> {
        if self.kind != AccountKind::WatchOnly {
            return Ok(None);
        }
        self.secret
            .expose(|bytes| utf8(bytes).map(|value| Some(expose(value))))
    }

    pub fn copy_wallet_password(&self) -> Result<Zeroizing<String>, XmrError> {
        self.expose_wallet_password(|value| Zeroizing::new(value.to_owned()))
    }

    pub fn copy_primary_address(&self) -> Result<Zeroizing<String>, XmrError> {
        self.expose_primary_address(|value| Zeroizing::new(value.to_owned()))
    }

    pub fn copy_mnemonic(&self) -> Result<Option<Zeroizing<String>>, XmrError> {
        self.expose_mnemonic(|value| Zeroizing::new(value.to_owned()))
    }

    pub fn copy_private_view_key(&self) -> Result<Option<Zeroizing<String>>, XmrError> {
        self.expose_private_view_key(|value| Zeroizing::new(value.to_owned()))
    }

    pub fn encode(&self) -> Result<Zeroizing<Vec<u8>>, XmrError> {
        self.wallet_password.expose(|password| {
            self.primary_address.expose(|primary| {
                self.secret.expose(|secret| {
                    encode_frame(self.kind, self.restore_height, password, primary, secret)
                })
            })
        })
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, XmrError> {
        validate_total_length(bytes.len())?;
        if bytes.len() < HEADER_BYTES {
            return Err(XmrError::state_corrupt());
        }
        if bytes[0..8] != XMR_SECRET_MAGIC {
            return Err(XmrError::state_corrupt());
        }
        let kind = AccountKind::from_code(bytes[8])?;
        let restore_height = u64::from_be_bytes(
            bytes[9..17]
                .try_into()
                .map_err(|_| XmrError::state_corrupt())?,
        );
        let password_len = u16::from_be_bytes(
            bytes[17..19]
                .try_into()
                .map_err(|_| XmrError::state_corrupt())?,
        ) as usize;
        if password_len != PASSWORD_HEX_BYTES {
            return Err(XmrError::state_corrupt());
        }
        let password = &bytes[19..83];
        validate_password_hex(password)?;
        let primary_len = u16::from_be_bytes(
            bytes[83..85]
                .try_into()
                .map_err(|_| XmrError::state_corrupt())?,
        ) as usize;
        if primary_len != PRIMARY_ADDRESS_BYTES {
            return Err(XmrError::state_corrupt());
        }
        let primary = &bytes[85..180];
        let secret_len = u16::from_be_bytes(
            bytes[180..182]
                .try_into()
                .map_err(|_| XmrError::state_corrupt())?,
        ) as usize;
        if bytes.len() != HEADER_BYTES + secret_len {
            return Err(XmrError::state_corrupt());
        }
        let secret = &bytes[182..];
        if secret.len() != secret_len {
            return Err(XmrError::state_corrupt());
        }
        validate_secret_payload(kind, secret)?;
        Ok(Self {
            kind,
            restore_height,
            wallet_password: SecretBytes::new(password.to_vec())
                .map_err(|_| XmrError::state_corrupt())?,
            primary_address: SecretBytes::new(primary.to_vec())
                .map_err(|_| XmrError::state_corrupt())?,
            secret: SecretBytes::new(secret.to_vec()).map_err(|_| XmrError::state_corrupt())?,
        })
    }

    fn wipe(&mut self, observer: &mut dyn WipeObserver) {
        self.wallet_password
            .wipe_with("xmr-wallet-password", observer);
        self.primary_address
            .wipe_with("xmr-primary-address", observer);
        self.secret.wipe_with("xmr-account-secret", observer);
    }
}

impl fmt::Debug for XmrSecretV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("XmrSecretV1([REDACTED])")
    }
}

impl fmt::Display for XmrSecretV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("[REDACTED]")
    }
}

impl Drop for XmrSecretV1 {
    fn drop(&mut self) {
        self.wipe(&mut IgnoringWipe);
    }
}

pub(crate) fn validate_password_hex_length(length: usize) -> Result<(), XmrError> {
    if length == PASSWORD_HEX_BYTES {
        Ok(())
    } else {
        Err(XmrError::state_corrupt())
    }
}

pub(crate) fn validate_view_key_hex_length(length: usize) -> Result<(), XmrError> {
    if length == VIEW_KEY_HEX_BYTES {
        Ok(())
    } else {
        Err(XmrError::state_corrupt())
    }
}

pub(crate) fn validate_primary_address_length(length: usize) -> Result<(), XmrError> {
    if length == PRIMARY_ADDRESS_BYTES {
        Ok(())
    } else {
        Err(XmrError::state_corrupt())
    }
}

pub(crate) fn validate_total_length(length: usize) -> Result<(), XmrError> {
    if length <= MAX_SECRET_BYTES {
        Ok(())
    } else {
        Err(XmrError::state_corrupt())
    }
}

pub(crate) fn primary_address_syntax_is_valid(address: &str) -> bool {
    address.len() == PRIMARY_ADDRESS_BYTES
        && address
            .as_bytes()
            .iter()
            .all(|byte| MONERO_BASE58.contains(byte))
}

pub(crate) fn is_network_valid_address(address: &str) -> bool {
    primary_address_syntax_is_valid(address)
        && matches!(address.as_bytes().first(), Some(b'5' | b'7' | b'9' | b'B'))
}

pub(crate) fn is_spendable_mnemonic(mnemonic: &str) -> bool {
    let words: Vec<&str> = mnemonic.split_ascii_whitespace().collect();
    words.len() == MNEMONIC_WORDS
        && words
            .iter()
            .all(|word| !word.is_empty() && word.bytes().all(|byte| byte.is_ascii_lowercase()))
        && words[..24].contains(&words[24])
}

pub(crate) fn restore_height_from_local(local_height: u64) -> u64 {
    local_height.saturating_sub(RESTORE_SAFETY_MARGIN)
}

pub(crate) fn generate_wallet_password<P: AccountPort>(
    port: &mut P,
) -> Result<WalletPassword, XmrError> {
    let mut bytes = Zeroizing::new([0u8; PASSWORD_SOURCE_BYTES]);
    if let Err(error) = port.fill_entropy(&mut *bytes) {
        bytes.zeroize();
        return Err(error);
    }
    let mut encoded = Zeroizing::new(String::with_capacity(PASSWORD_HEX_BYTES));
    for byte in &*bytes {
        use core::fmt::Write as _;
        let _ = write!(encoded, "{byte:02x}");
    }
    bytes.zeroize();
    let secret = SecretBytes::new(encoded.as_bytes().to_vec()).map_err(|_| XmrError::internal())?;
    encoded.zeroize();
    Ok(WalletPassword {
        encoded: secret,
        source: "os-entropy",
        source_bytes: PASSWORD_SOURCE_BYTES,
    })
}

pub(crate) fn derived_paths(
    network: XmrNetwork,
    account_id: &str,
) -> Result<AccountPathsView, XmrError> {
    if !valid_account_id(account_id) || account_id.contains('/') || account_id.contains("..") {
        return Err(XmrError::request_schema());
    }
    Ok(AccountPathsView {
        relative_account_directory: format!("{}/{}", network.name(), account_id),
        wallet_filename: account_id.to_owned(),
        directory_mode: DIRECTORY_MODE,
        wallet_file_mode: WALLET_FILE_MODE,
        keys_file_mode: KEYS_FILE_MODE,
        state_db_mode: STATE_FILE_MODE,
        state_db_synchronous: SYNCHRONOUS_FULL,
    })
}

pub(crate) fn run_secret_exit(
    secrets: Vec<SecretBytes>,
    exit: SecretExit,
    events: Arc<Mutex<Vec<WipeEvent>>>,
) -> Result<(), XmrError> {
    let mut guard = SecretExitGuard::new(secrets, events);
    match exit {
        SecretExit::Success => {
            guard.wipe_now();
            Ok(())
        }
        SecretExit::Error | SecretExit::Cancellation => {
            guard.wipe_now();
            Err(XmrError::internal())
        }
        SecretExit::Drop => Ok(()),
        SecretExit::PanicUnwind => {
            let _ = catch_unwind(AssertUnwindSafe(|| {
                guard.force_panic();
            }));
            Ok(())
        }
    }
}

struct SecretExitGuard {
    secrets: Vec<SecretBytes>,
    events: Arc<Mutex<Vec<WipeEvent>>>,
    wiped: bool,
}

impl SecretExitGuard {
    fn new(secrets: Vec<SecretBytes>, events: Arc<Mutex<Vec<WipeEvent>>>) -> Self {
        Self {
            secrets,
            events,
            wiped: false,
        }
    }

    fn wipe_now(&mut self) {
        self.wipe_with_observer();
    }

    fn force_panic(&mut self) {
        panic!("sanitized account secret exit");
    }

    fn wipe_with_observer(&mut self) {
        if self.wiped {
            return;
        }
        let mut observer = SharedWipeObserver {
            events: self.events.clone(),
        };
        for secret in &mut self.secrets {
            secret.wipe_with("xmr-account-secret", &mut observer);
        }
        self.wiped = true;
    }
}

impl Drop for SecretExitGuard {
    fn drop(&mut self) {
        self.wipe_with_observer();
    }
}

struct SharedWipeObserver {
    events: Arc<Mutex<Vec<WipeEvent>>>,
}

impl WipeObserver for SharedWipeObserver {
    fn observe(&mut self, event: WipeEvent) {
        if let Ok(mut events) = self.events.lock() {
            events.push(event);
        }
    }
}

struct IgnoringWipe;

impl WipeObserver for IgnoringWipe {
    fn observe(&mut self, _event: WipeEvent) {}
}

struct SilentWork;

impl VaultWorkObserver for SilentWork {
    fn before_allocation(&mut self, _bytes: usize) -> Result<(), VaultError> {
        Ok(())
    }

    fn before_kdf(&mut self) {}
}

pub(crate) struct AccountRpcCall {
    method: &'static str,
    fields: Vec<(&'static str, Zeroizing<String>)>,
}

impl AccountRpcCall {
    pub(crate) fn create_wallet(filename: &str, password: &str) -> Self {
        Self {
            method: "create_wallet",
            fields: vec![
                ("filename", Zeroizing::new(filename.to_owned())),
                ("password", Zeroizing::new(password.to_owned())),
                ("language", Zeroizing::new(ENGLISH_LANGUAGE.to_owned())),
            ],
        }
    }

    pub(crate) fn query_key_mnemonic() -> Self {
        Self {
            method: "query_key:mnemonic",
            fields: vec![("key_type", Zeroizing::new("mnemonic".to_owned()))],
        }
    }

    pub(crate) fn get_address() -> Self {
        Self {
            method: "get_address",
            fields: vec![("account_index", Zeroizing::new("0".to_owned()))],
        }
    }

    pub(crate) fn close_wallet() -> Self {
        Self {
            method: "close_wallet",
            fields: Vec::new(),
        }
    }

    pub(crate) fn stop_wallet() -> Self {
        Self {
            method: "stop_wallet",
            fields: Vec::new(),
        }
    }

    pub(crate) fn generate_from_keys(
        filename: &str,
        password: &str,
        address: &str,
        viewkey: &str,
        restore_height: u64,
    ) -> Self {
        Self {
            method: "generate_from_keys",
            fields: vec![
                ("filename", Zeroizing::new(filename.to_owned())),
                ("password", Zeroizing::new(password.to_owned())),
                ("address", Zeroizing::new(address.to_owned())),
                ("viewkey", Zeroizing::new(viewkey.to_owned())),
                ("restore_height", Zeroizing::new(restore_height.to_string())),
            ],
        }
    }

    pub(crate) fn open_wallet(filename: &str, password: &str) -> Self {
        Self {
            method: "open_wallet",
            fields: vec![
                ("filename", Zeroizing::new(filename.to_owned())),
                ("password", Zeroizing::new(password.to_owned())),
            ],
        }
    }

    pub(crate) fn restore_deterministic_wallet(
        filename: &str,
        password: &str,
        seed: &str,
        restore_height: u64,
    ) -> Self {
        Self {
            method: "restore_deterministic_wallet",
            fields: vec![
                ("filename", Zeroizing::new(filename.to_owned())),
                ("password", Zeroizing::new(password.to_owned())),
                ("seed", Zeroizing::new(seed.to_owned())),
                ("restore_height", Zeroizing::new(restore_height.to_string())),
                ("language", Zeroizing::new(ENGLISH_LANGUAGE.to_owned())),
            ],
        }
    }

    pub fn method(&self) -> &'static str {
        self.method
    }

    pub fn field_names(&self) -> Vec<&'static str> {
        self.fields.iter().map(|(name, _)| *name).collect()
    }

    pub fn argument(&self, field: &str) -> Option<&str> {
        self.fields
            .iter()
            .find(|(name, _)| *name == field)
            .map(|(_, value)| value.as_str())
    }
}

impl Drop for AccountRpcCall {
    fn drop(&mut self) {
        for (_, value) in &mut self.fields {
            value.zeroize();
        }
    }
}

impl fmt::Debug for AccountRpcCall {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AccountRpcCall")
            .field("method", &self.method)
            .field("fields", &"[REDACTED]")
            .finish()
    }
}

pub(crate) struct WalletRpcObservation {
    pub primary: Zeroizing<String>,
    pub verified_primary: Zeroizing<String>,
    pub watch_only: bool,
}

impl fmt::Debug for WalletRpcObservation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("WalletRpcObservation([REDACTED])")
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum WalletPresence {
    Missing,
    Complete,
    Partial,
    Hostile(HostileWalletEntry),
}

pub(crate) trait AccountPort {
    fn note(&mut self, operation: &'static str);
    fn operations(&self) -> &[&'static str];
    fn fill_entropy(&mut self, output: &mut [u8]) -> Result<(), XmrError>;
    fn local_height_without_bootstrap(&mut self) -> Result<u64, XmrError>;
    fn begin_attempt(&mut self);
    fn commit_attempt(&mut self);
    fn preflight_create_new(&mut self, paths: &AccountPathsView) -> Result<(), XmrError>;
    fn preflight_open_existing(
        &mut self,
        paths: &AccountPathsView,
    ) -> Result<WalletPresence, XmrError>;
    fn create_private_layout_after_preflight(&mut self) -> Result<(), XmrError>;
    fn create_missing_wallet_layout(&mut self) -> Result<(), XmrError>;
    fn inspect_wallet(&mut self, paths: &AccountPathsView) -> Result<WalletPresence, XmrError>;
    fn create_wallet_files(&mut self, paths: &AccountPathsView) -> Result<(), XmrError>;
    fn rollback_owned_artifacts(&mut self) -> Result<(), XmrError>;
    fn start_child(&mut self) -> Result<(), XmrError>;
    fn create_wallet(&mut self, filename: &str, password: &str) -> Result<(), XmrError>;
    fn query_mnemonic(&mut self) -> Result<Zeroizing<String>, XmrError>;
    fn get_primary_address(&mut self) -> Result<WalletRpcObservation, XmrError>;
    fn validate_primary_for_network(&mut self, address: &str) -> Result<(), XmrError>;
    fn generate_from_keys(
        &mut self,
        filename: &str,
        password: &str,
        address: &str,
        viewkey: &str,
        restore_height: u64,
    ) -> Result<WalletRpcObservation, XmrError>;
    fn open_wallet(
        &mut self,
        filename: &str,
        password: &str,
    ) -> Result<WalletRpcObservation, XmrError>;
    fn restore_deterministic_wallet(
        &mut self,
        filename: &str,
        password: &str,
        seed: &str,
        restore_height: u64,
    ) -> Result<WalletRpcObservation, XmrError>;
    fn close_wallet(&mut self) -> Result<(), XmrError>;
    fn recorded_calls(&self) -> &[AccountRpcCall];
    fn seal_vault(&mut self, secret: &XmrSecretV1) -> Result<(), XmrError>;
    fn open_vault(&mut self) -> Result<XmrSecretV1, XmrError>;
    fn persist_state(&mut self, identity: &StoredIdentity) -> Result<(), XmrError>;
    fn load_state(&mut self) -> Result<StoredIdentity, XmrError>;
    fn stop_and_reap_child(&mut self) -> Result<(), XmrError>;
    fn teardown_owned(&mut self) -> Result<(), XmrError>;
    fn prove_owned_session(
        &mut self,
        account_id: &str,
        network: XmrNetwork,
    ) -> Result<(), XmrError>;
    fn wipe_wallet_password(&mut self);
    fn wipe_events(&self) -> Vec<WipeEvent>;
    fn active_child_count(&self) -> usize;
    fn open_handle_count(&self) -> usize;
    fn replaced_hostile_entry(&self) -> bool;
}

pub(crate) struct AccountManager<P: AccountPort> {
    account_id: String,
    network_text: String,
    kind_text: String,
    port: P,
    paths: Option<AccountPathsView>,
    returned: Option<CreatedAccount>,
    vault_committed: bool,
    state_committed: bool,
    vault_before_state: bool,
    state_before_return: bool,
    generated_removed: bool,
    secrets_wiped: bool,
    identity_verified: bool,
    vault_authenticated: bool,
    recovery_created: bool,
    rpc_watch_only: bool,
    password_wiped: bool,
    may_retain: bool,
    requires_vault_password: bool,
    unavailable: bool,
    attempt_active: bool,
    last_rpc: Vec<&'static str>,
}

struct VerifyOpenIdentity<'a> {
    account_id: &'a str,
    network: XmrNetwork,
    kind: AccountKind,
    sealed: &'a XmrSecretV1,
    stored: &'a StoredIdentity,
    observation: &'a WalletRpcObservation,
    paths: &'a AccountPathsView,
}

impl<P: AccountPort> AccountManager<P> {
    pub fn new(account_id: &str, network: &str, kind: &str, port: P) -> Self {
        Self {
            account_id: account_id.to_owned(),
            network_text: network.to_owned(),
            kind_text: kind.to_owned(),
            port,
            paths: None,
            returned: None,
            vault_committed: false,
            state_committed: false,
            vault_before_state: false,
            state_before_return: false,
            generated_removed: false,
            secrets_wiped: false,
            identity_verified: false,
            vault_authenticated: false,
            recovery_created: false,
            rpc_watch_only: false,
            password_wiped: false,
            may_retain: false,
            requires_vault_password: true,
            unavailable: false,
            attempt_active: false,
            last_rpc: Vec::new(),
        }
    }

    pub fn port(&self) -> &P {
        &self.port
    }

    pub fn port_mut(&mut self) -> &mut P {
        &mut self.port
    }

    pub fn returned_account(&self) -> Option<&CreatedAccount> {
        self.returned.as_ref()
    }

    pub fn vault_committed(&self) -> bool {
        self.vault_committed
    }

    pub fn account_state_committed(&self) -> bool {
        self.state_committed
    }

    pub fn vault_sealed_before_account_state(&self) -> bool {
        self.vault_before_state
    }

    pub fn account_state_durable_before_return(&self) -> bool {
        self.state_before_return
    }

    pub fn generated_wallet_removed_or_quarantined(&self) -> bool {
        self.generated_removed
    }

    pub fn creation_secrets_wiped(&self) -> bool {
        self.secrets_wiped
            && self.port.wipe_events().iter().any(|event| {
                event.label == "xmr-wallet-password" && event.all_zero && event.length > 0
            })
    }

    pub fn identity_verified_after_open(&self) -> bool {
        self.identity_verified
    }

    pub fn vault_authenticated_before_recovery(&self) -> bool {
        self.vault_authenticated
    }

    pub fn recovery_created_files(&self) -> bool {
        self.recovery_created
    }

    pub fn rpc_reported_watch_only(&self) -> bool {
        self.rpc_watch_only
    }

    pub fn wallet_password_wiped(&self) -> bool {
        self.password_wiped
            && self
                .port
                .wipe_events()
                .iter()
                .any(|event| event.label == "xmr-wallet-password" && event.all_zero)
    }

    pub fn may_retain_process(&self) -> bool {
        self.may_retain
    }

    pub fn requires_authenticated_vault_for_password(&self) -> bool {
        self.requires_vault_password
    }

    pub fn account_unavailable(&self) -> bool {
        self.unavailable
    }

    pub fn last_rpc_calls(&self) -> &[&'static str] {
        &self.last_rpc
    }

    pub fn operations(&self) -> &[&'static str] {
        self.port.operations()
    }

    pub fn active_child_count(&self) -> usize {
        self.port.active_child_count()
    }

    pub fn open_handle_count(&self) -> usize {
        self.port.open_handle_count()
    }

    pub fn replaced_hostile_entry(&self) -> bool {
        self.port.replaced_hostile_entry()
    }

    pub fn inspect_paths(&self) -> Result<AccountPathsView, XmrError> {
        let (account_id, network, _) = self.validated_identity()?;
        derived_paths(network, &account_id)
    }

    pub fn create(&mut self) -> Result<CreatedAccount, XmrError> {
        self.require_available()?;
        let (_, _, kind) = self.validated_identity()?;
        match kind {
            AccountKind::Software => self.create_software(),
            AccountKind::WatchOnly => Err(XmrError::request_schema()),
        }
    }

    pub fn create_software(&mut self) -> Result<CreatedAccount, XmrError> {
        self.require_available()?;
        let (account_id, network, kind) = self.validated_identity()?;
        if kind != AccountKind::Software {
            return Err(XmrError::request_schema());
        }
        self.begin_attempt()?;
        match self.create_software_inner(&account_id, network) {
            Ok(account) => Ok(account),
            Err(error) => self.finish_failed_attempt(error),
        }
    }

    pub fn import_watch_only(
        &mut self,
        primary: &str,
        view_key: &str,
        restore_height: u64,
    ) -> Result<CreatedAccount, XmrError> {
        self.require_available()?;
        let (account_id, network, kind) = self.validated_identity()?;
        if kind != AccountKind::WatchOnly {
            return Err(XmrError::request_schema());
        }
        self.begin_attempt()?;
        match self.import_watch_only_inner(&account_id, network, primary, view_key, restore_height)
        {
            Ok(account) => Ok(account),
            Err(error) => self.finish_failed_attempt(error),
        }
    }

    pub fn open(&mut self) -> Result<CreatedAccount, XmrError> {
        self.require_available()?;
        let (account_id, network, kind) = self.validated_identity()?;
        self.begin_attempt()?;
        match self.open_inner(&account_id, network, kind) {
            Ok(account) => Ok(account),
            Err(error) => self.finish_failed_attempt(error),
        }
    }

    pub fn lock(&mut self) -> Result<(), XmrError> {
        self.require_available()?;
        let (_, _, kind) = self.validated_identity()?;
        self.last_rpc.clear();
        let teardown = if kind == AccountKind::Software || self.port.active_child_count() > 0 {
            Some(self.port.teardown_owned())
        } else {
            None
        };
        self.port.wipe_wallet_password();
        self.password_wiped = true;
        self.may_retain = false;
        self.requires_vault_password = true;
        match teardown {
            Some(Ok(())) => {
                self.last_rpc.push("close_wallet");
                self.last_rpc.push("stop_wallet");
                Ok(())
            }
            Some(Err(_)) => {
                self.unavailable = true;
                Err(XmrError::internal())
            }
            None => Ok(()),
        }
    }

    pub fn simulate_cold_restart(&mut self) {
        self.may_retain = false;
        self.requires_vault_password = true;
        self.password_wiped = true;
        if self.port.stop_and_reap_child().is_err() {
            self.unavailable = true;
        }
        self.port.wipe_wallet_password();
    }

    fn create_software_inner(
        &mut self,
        account_id: &str,
        network: XmrNetwork,
    ) -> Result<CreatedAccount, XmrError> {
        let paths = self.prepare_paths(account_id, network)?;
        self.port.preflight_create_new(&paths)?;
        let local_height = self.port.local_height_without_bootstrap()?;
        let restore_height = restore_height_from_local(local_height);
        self.port.create_private_layout_after_preflight()?;
        self.port.create_wallet_files(&paths)?;
        if let Err(error) = self.port.start_child() {
            return self.rollback_and(error);
        }
        let mut password = match generate_wallet_password(&mut self.port) {
            Ok(password) => password,
            Err(error) => return self.rollback_and(error),
        };
        let password_text = match password.copy_text() {
            Ok(value) => value,
            Err(error) => return self.rollback_and(error),
        };
        if let Err(error) = self.port.create_wallet(account_id, password_text.as_str()) {
            password.wipe(&mut IgnoringWipe);
            return self.rollback_and(error);
        }
        let mnemonic = match self.port.query_mnemonic() {
            Ok(mnemonic) => mnemonic,
            Err(error) => {
                password.wipe(&mut IgnoringWipe);
                return self.rollback_and(error);
            }
        };
        let addressed = match self.port.get_primary_address() {
            Ok(observation) => observation,
            Err(error) => {
                password.wipe(&mut IgnoringWipe);
                return self.rollback_and(error);
            }
        };
        if addressed.primary.as_str() != addressed.verified_primary.as_str()
            || validate_primary_address_length(addressed.primary.len()).is_err()
        {
            password.wipe(&mut IgnoringWipe);
            return self.rollback_and(XmrError::protocol_incompatible());
        }
        if let Err(error) = self.port.close_wallet() {
            password.wipe(&mut IgnoringWipe);
            return self.rollback_and(error);
        }
        self.port.wipe_wallet_password();
        let secret = match XmrSecretV1::software(
            restore_height,
            password_text.as_str(),
            addressed.primary.as_str(),
            mnemonic.as_str(),
        ) {
            Ok(secret) => secret,
            Err(error) => {
                password.wipe(&mut IgnoringWipe);
                return self.rollback_and(error);
            }
        };
        password.wipe(&mut SharedWipeObserver {
            events: Arc::new(Mutex::new(Vec::new())),
        });
        self.port.wipe_wallet_password();
        self.seal_and_persist(
            account_id,
            network,
            AccountKind::Software,
            restore_height,
            &secret,
        )?;
        self.wipe_creation_secrets();
        Ok(self.success(account_id, AccountKind::Software, network, restore_height))
    }

    fn import_watch_only_inner(
        &mut self,
        account_id: &str,
        network: XmrNetwork,
        primary: &str,
        view_key: &str,
        restore_height: u64,
    ) -> Result<CreatedAccount, XmrError> {
        validate_primary_address_length(primary.len()).map_err(|_| XmrError::request_schema())?;
        validate_view_key_hex_length(view_key.len()).map_err(|_| XmrError::request_schema())?;
        if !is_lowercase_hex(view_key.as_bytes()) {
            return Err(XmrError::request_schema());
        }
        let paths = self.prepare_paths(account_id, network)?;
        self.port.preflight_create_new(&paths)?;
        let local_height = self.port.local_height_without_bootstrap()?;
        if restore_height > local_height {
            return Err(XmrError::request_schema());
        }
        self.port.create_private_layout_after_preflight()?;
        self.port.create_wallet_files(&paths)?;
        if let Err(error) = self.port.start_child() {
            return self.rollback_and(error);
        }
        if let Err(error) = self.port.validate_primary_for_network(primary) {
            return self.rollback_and(error);
        }
        let mut password = match generate_wallet_password(&mut self.port) {
            Ok(password) => password,
            Err(error) => return self.rollback_and(error),
        };
        let password_text = match password.copy_text() {
            Ok(value) => value,
            Err(error) => return self.rollback_and(error),
        };
        let generated = match self.port.generate_from_keys(
            account_id,
            password_text.as_str(),
            primary,
            view_key,
            restore_height,
        ) {
            Ok(observation) => observation,
            Err(error) => {
                password.wipe(&mut IgnoringWipe);
                return self.rollback_and(error);
            }
        };
        let addressed = match self.port.get_primary_address() {
            Ok(observation) => observation,
            Err(error) => {
                password.wipe(&mut IgnoringWipe);
                return self.rollback_and(error);
            }
        };
        self.rpc_watch_only = generated.watch_only;
        if !self.rpc_watch_only
            || addressed.primary.as_str() != primary
            || generated.primary.as_str() != primary
            || addressed.primary.as_str() != addressed.verified_primary.as_str()
        {
            password.wipe(&mut IgnoringWipe);
            return self.rollback_and(XmrError::protocol_incompatible());
        }
        if let Err(error) = self.port.close_wallet() {
            password.wipe(&mut IgnoringWipe);
            return self.rollback_and(error);
        }
        self.port.wipe_wallet_password();
        let secret = match XmrSecretV1::watch_only(
            restore_height,
            password_text.as_str(),
            primary,
            view_key,
        ) {
            Ok(secret) => secret,
            Err(error) => {
                password.wipe(&mut IgnoringWipe);
                return self.rollback_and(error);
            }
        };
        password.wipe(&mut IgnoringWipe);
        self.port.wipe_wallet_password();
        self.seal_and_persist(
            account_id,
            network,
            AccountKind::WatchOnly,
            restore_height,
            &secret,
        )?;
        self.wipe_creation_secrets();
        self.may_retain = true;
        self.requires_vault_password = false;
        Ok(self.success(account_id, AccountKind::WatchOnly, network, restore_height))
    }

    fn open_inner(
        &mut self,
        account_id: &str,
        network: XmrNetwork,
        kind: AccountKind,
    ) -> Result<CreatedAccount, XmrError> {
        let paths = self.prepare_paths(account_id, network)?;
        let presence = match self.port.preflight_open_existing(&paths) {
            Ok(presence) => presence,
            Err(error) => return self.rollback_and(error),
        };
        let sealed = match self.port.open_vault() {
            Ok(sealed) => sealed,
            Err(error) => return self.rollback_and(error),
        };
        self.vault_authenticated = true;
        let stored = match self.port.load_state() {
            Ok(stored) => stored,
            Err(error) => return self.rollback_and(error),
        };
        stored.validate()?;
        if sealed.kind() != kind
            || stored.kind() != kind.code()
            || stored.account_id() != account_id
            || stored.network() != network.name()
            || stored.restore_height() != sealed.restore_height()
        {
            return self.rollback_and(XmrError::state_corrupt());
        }
        if let Err(error) = self.require_matching_primaries_before_child(&sealed, &stored) {
            return self.rollback_and(error);
        }
        match presence {
            WalletPresence::Hostile(_) | WalletPresence::Partial => {
                return self.rollback_and(XmrError::state_corrupt());
            }
            WalletPresence::Missing | WalletPresence::Complete => {}
        }
        if presence == WalletPresence::Missing
            && let Err(error) = self.port.create_missing_wallet_layout()
        {
            return self.rollback_and(error);
        }
        if let Err(error) = self.port.start_child() {
            return self.rollback_and(error);
        }
        let password = sealed.copy_wallet_password()?;
        let observation = match presence {
            WalletPresence::Complete => {
                self.recovery_created = false;
                match self.port.open_wallet(account_id, password.as_str()) {
                    Ok(observation) => observation,
                    Err(error) => return self.rollback_and(error),
                }
            }
            WalletPresence::Missing => {
                self.recovery_created = true;
                match kind {
                    AccountKind::Software => {
                        let mnemonic = sealed
                            .copy_mnemonic()?
                            .ok_or_else(XmrError::state_corrupt)?;
                        match self.port.restore_deterministic_wallet(
                            account_id,
                            password.as_str(),
                            mnemonic.as_str(),
                            sealed.restore_height(),
                        ) {
                            Ok(observation) => observation,
                            Err(error) => return self.rollback_and(error),
                        }
                    }
                    AccountKind::WatchOnly => {
                        let view_key = sealed
                            .copy_private_view_key()?
                            .ok_or_else(XmrError::state_corrupt)?;
                        let primary = sealed.copy_primary_address()?;
                        match self.port.generate_from_keys(
                            account_id,
                            password.as_str(),
                            primary.as_str(),
                            view_key.as_str(),
                            sealed.restore_height(),
                        ) {
                            Ok(observation) => observation,
                            Err(error) => return self.rollback_and(error),
                        }
                    }
                }
            }
            WalletPresence::Partial | WalletPresence::Hostile(_) => {
                return Err(XmrError::state_corrupt());
            }
        };
        self.port.wipe_wallet_password();
        if let Err(error) = self.verify_open_identity(VerifyOpenIdentity {
            account_id,
            network,
            kind,
            sealed: &sealed,
            stored: &stored,
            observation: &observation,
            paths: &paths,
        }) {
            return self.rollback_and(error);
        }
        self.identity_verified = true;
        if kind == AccountKind::WatchOnly {
            self.may_retain = true;
            self.requires_vault_password = false;
        } else {
            self.may_retain = false;
            self.requires_vault_password = true;
        }
        Ok(self.success(account_id, kind, network, sealed.restore_height()))
    }

    fn verify_open_identity(&mut self, identity: VerifyOpenIdentity<'_>) -> Result<(), XmrError> {
        let VerifyOpenIdentity {
            account_id,
            network,
            kind,
            sealed,
            stored,
            observation,
            paths,
        } = identity;
        let sealed_primary = sealed.copy_primary_address()?;
        let stored_primary = stored.primary_address()?;
        self.port.prove_owned_session(account_id, network)?;
        if observation.primary.as_str() != sealed_primary.as_str()
            || observation.primary.as_str() != observation.verified_primary.as_str()
            || stored_primary.as_str() != sealed_primary.as_str()
            || stored.network() != network.name()
            || stored.kind() != kind.code()
            || sealed.kind() != kind
            || stored.restore_height() != sealed.restore_height()
            || paths.wallet_filename != account_id
            || stored.account_id() != account_id
        {
            return Err(XmrError::state_corrupt());
        }
        Ok(())
    }

    fn seal_and_persist(
        &mut self,
        account_id: &str,
        network: XmrNetwork,
        kind: AccountKind,
        restore_height: u64,
        secret: &XmrSecretV1,
    ) -> Result<(), XmrError> {
        self.port.note("vault_seal");
        if let Err(error) = self.port.seal_vault(secret) {
            return self.rollback_and(error);
        }
        self.vault_committed = true;
        let primary = secret.copy_primary_address()?;
        let identity = match StoredIdentity::new(
            account_id.to_owned(),
            network,
            kind.code(),
            primary.as_str(),
            restore_height,
        ) {
            Ok(identity) => identity,
            Err(error) => return self.rollback_and(error),
        };
        self.port.note("account_state");
        if let Err(error) = self.port.persist_state(&identity) {
            return self.rollback_and(error);
        }
        self.state_committed = true;
        self.vault_before_state = true;
        self.state_before_return = true;
        Ok(())
    }

    fn success(
        &mut self,
        account_id: &str,
        kind: AccountKind,
        network: XmrNetwork,
        restore_height: u64,
    ) -> CreatedAccount {
        let created = CreatedAccount {
            account_id: account_id.to_owned(),
            kind,
            network: AccountNetwork(network),
            restore_height,
            capabilities: AccountCapabilities::viewing_only(),
        };
        self.returned = Some(created.clone());
        self.port.commit_attempt();
        self.attempt_active = false;
        created
    }

    fn prepare_paths(
        &mut self,
        account_id: &str,
        network: XmrNetwork,
    ) -> Result<AccountPathsView, XmrError> {
        self.port.note("derive_paths");
        let paths = derived_paths(network, account_id)?;
        self.paths = Some(paths.clone());
        Ok(paths)
    }

    fn validated_identity(&self) -> Result<(String, XmrNetwork, AccountKind), XmrError> {
        if !valid_account_id(&self.account_id) {
            return Err(XmrError::request_schema());
        }
        let network = XmrNetwork::parse(&self.network_text)?;
        let kind = AccountKind::parse(&self.kind_text)?;
        Ok((self.account_id.clone(), network, kind))
    }

    fn rollback_and<T>(&mut self, error: XmrError) -> Result<T, XmrError> {
        match self.rollback() {
            Ok(()) => Err(error),
            Err(cleanup) => Err(cleanup),
        }
    }

    fn require_available(&self) -> Result<(), XmrError> {
        if self.unavailable {
            Err(XmrError::internal())
        } else {
            Ok(())
        }
    }

    fn begin_attempt(&mut self) -> Result<(), XmrError> {
        if self.unavailable {
            return Err(XmrError::internal());
        }
        if self.attempt_active {
            self.unavailable = true;
            return Err(XmrError::internal());
        }
        self.returned = None;
        self.vault_committed = false;
        self.state_committed = false;
        self.vault_before_state = false;
        self.state_before_return = false;
        self.generated_removed = false;
        self.secrets_wiped = false;
        self.identity_verified = false;
        self.vault_authenticated = false;
        self.recovery_created = false;
        self.rpc_watch_only = false;
        self.attempt_active = true;
        self.port.begin_attempt();
        Ok(())
    }

    fn rollback(&mut self) -> Result<(), XmrError> {
        self.wipe_creation_secrets();
        let mut cleanup_failed = false;
        if self.port.teardown_owned().is_err() {
            cleanup_failed = true;
        }
        match self.port.rollback_owned_artifacts() {
            Ok(()) => self.generated_removed = true,
            Err(_) => {
                cleanup_failed = true;
                self.generated_removed = true;
            }
        }
        self.vault_committed = false;
        self.state_committed = false;
        self.returned = None;
        self.may_retain = false;
        if cleanup_failed {
            self.unavailable = true;
            Err(XmrError::internal())
        } else {
            self.attempt_active = false;
            self.port.commit_attempt();
            Ok(())
        }
    }

    fn finish_failed_attempt<T>(&mut self, error: XmrError) -> Result<T, XmrError> {
        self.returned = None;
        if self.unavailable {
            Err(XmrError::internal())
        } else if self.attempt_active {
            self.rollback_and(error)
        } else {
            Err(error)
        }
    }

    fn reconcile_on_unwind(&mut self) {
        self.wipe_creation_secrets();
        if self.attempt_active {
            let _ = self.rollback();
        } else if self.fail_closed().is_err() {
            self.unavailable = true;
        }
    }

    fn mark_unavailable(&mut self) {
        self.unavailable = true;
    }

    fn require_matching_primaries_before_child(
        &self,
        sealed: &XmrSecretV1,
        stored: &StoredIdentity,
    ) -> Result<(), XmrError> {
        let sealed_primary = sealed.copy_primary_address()?;
        let stored_primary = stored.primary_address()?;
        if !primary_address_syntax_is_valid(sealed_primary.as_str())
            || !primary_address_syntax_is_valid(stored_primary.as_str())
            || sealed_primary.as_str() != stored_primary.as_str()
        {
            return Err(XmrError::state_corrupt());
        }
        Ok(())
    }

    fn fail_closed(&mut self) -> Result<(), XmrError> {
        let teardown = self.port.teardown_owned();
        self.port.wipe_wallet_password();
        self.password_wiped = true;
        self.may_retain = false;
        teardown
    }

    fn wipe_creation_secrets(&mut self) {
        self.port.wipe_wallet_password();
        self.secrets_wiped = true;
        self.password_wiped = true;
    }
}

#[cfg(target_os = "linux")]
pub(crate) struct SystemAccountPort {
    account_id: String,
    network: XmrNetwork,
    _kind: AccountKind,
    root: PathBuf,
    executable_path: PathBuf,
    passphrase: Option<SecretBytes>,
    pool: WalletRpcProcessPool<SystemWalletRpcControl>,
    vault: Option<VaultStore<LinuxStorePort>>,
    store: Option<AccountStore<PathSqliteSurface>>,
    operations: Vec<&'static str>,
    wipe_events: Arc<Mutex<Vec<WipeEvent>>>,
    child_count: usize,
    handles: usize,
    replaced_hostile: bool,
    owner: u32,
    last_password: Option<SecretBytes>,
    attempt: AttemptOwnership,
    receiver_unavailable: bool,
    receiver_node_state: NodeState,
    receiver_wallet_state: WalletState,
    receiver_last_index: u32,
}

#[cfg(target_os = "linux")]
#[derive(Clone, Debug)]
struct ArtifactIdentity {
    path: PathBuf,
    dev: u64,
    ino: u64,
}

#[cfg(target_os = "linux")]
#[derive(Clone, Default, Debug)]
struct AttemptOwnership {
    vault: Option<ArtifactIdentity>,
    state: Option<ArtifactIdentity>,
    wallet: Option<ArtifactIdentity>,
    keys: Option<ArtifactIdentity>,
    uncertain: bool,
    provisional_uncertainty: bool,
}

#[cfg(target_os = "linux")]
impl AttemptOwnership {
    fn record_provisional_uncertainty(&mut self) {
        self.provisional_uncertainty = true;
    }

    fn publish_vault(&mut self, identity: ArtifactIdentity) {
        self.vault = Some(identity);
        self.provisional_uncertainty = false;
    }

    fn publish_state(&mut self, identity: ArtifactIdentity) {
        self.state = Some(identity);
        self.provisional_uncertainty = false;
    }

    fn cleanup_uncertain(&self) -> bool {
        self.uncertain || self.provisional_uncertainty
    }
}

#[cfg(target_os = "linux")]
pub struct SystemAccount {
    manager: AccountManager<SystemAccountPort>,
}

#[cfg(not(target_os = "linux"))]
pub struct SystemAccount {
    _private: (),
}

impl SystemAccount {
    pub fn new(
        root: &Path,
        executable: &Path,
        account_id: &str,
        network: &str,
        kind: &str,
    ) -> Result<Self, XmrError> {
        #[cfg(not(target_os = "linux"))]
        {
            let _ = (root, executable, account_id, network, kind);
            return Err(XmrError::unavailable());
        }
        #[cfg(target_os = "linux")]
        {
            let port = SystemAccountPort::new(root, executable, account_id, network, kind)?;
            Ok(Self {
                manager: AccountManager::new(account_id, network, kind, port),
            })
        }
    }

    pub fn create_software(&mut self, passphrase: SecretBytes) -> Result<CreatedAccount, XmrError> {
        #[cfg(not(target_os = "linux"))]
        {
            let mut passphrase = passphrase;
            passphrase.wipe_with("vault-passphrase", &mut IgnoringWipe);
            return Err(XmrError::unavailable());
        }
        #[cfg(target_os = "linux")]
        {
            self.manager.port_mut().install_passphrase(passphrase)?;
            let result = catch_unwind(AssertUnwindSafe(|| self.manager.create_software()));
            self.manager.port_mut().wipe_passphrase();
            self.manager.port_mut().wipe_wallet_password();
            self.resume_or_reconcile(result)
        }
    }

    pub fn import_watch_only(
        &mut self,
        passphrase: SecretBytes,
        mut primary: SecretBytes,
        mut view_key: SecretBytes,
        restore_height: u64,
    ) -> Result<CreatedAccount, XmrError> {
        #[cfg(not(target_os = "linux"))]
        {
            let _ = restore_height;
            let mut passphrase = passphrase;
            passphrase.wipe_with("vault-passphrase", &mut IgnoringWipe);
            primary.wipe_with("xmr-primary-address", &mut IgnoringWipe);
            view_key.wipe_with("xmr-account-secret", &mut IgnoringWipe);
            return Err(XmrError::unavailable());
        }
        #[cfg(target_os = "linux")]
        {
            let primary_text =
                primary.expose(|bytes| utf8(bytes).map(|value| Zeroizing::new(value.to_owned())));
            let view_text =
                view_key.expose(|bytes| utf8(bytes).map(|value| Zeroizing::new(value.to_owned())));
            let converted = match (primary_text, view_text) {
                (Ok(primary_text), Ok(view_text)) => Ok((primary_text, view_text)),
                (Err(error), _) | (_, Err(error)) => Err(error),
            };
            let result = match converted {
                Ok((primary_text, view_text)) => {
                    self.manager.port_mut().install_passphrase(passphrase)?;
                    catch_unwind(AssertUnwindSafe(|| {
                        self.manager.import_watch_only(
                            primary_text.as_str(),
                            view_text.as_str(),
                            restore_height,
                        )
                    }))
                }
                Err(error) => {
                    let mut passphrase = passphrase;
                    passphrase.wipe_with("vault-passphrase", &mut IgnoringWipe);
                    Ok(Err(error))
                }
            };
            primary.wipe_with("xmr-primary-address", &mut IgnoringWipe);
            view_key.wipe_with("xmr-account-secret", &mut IgnoringWipe);
            self.manager.port_mut().wipe_passphrase();
            self.manager.port_mut().wipe_wallet_password();
            self.resume_or_reconcile(result)
        }
    }

    pub fn open(&mut self, passphrase: SecretBytes) -> Result<CreatedAccount, XmrError> {
        #[cfg(not(target_os = "linux"))]
        {
            let mut passphrase = passphrase;
            passphrase.wipe_with("vault-passphrase", &mut IgnoringWipe);
            return Err(XmrError::unavailable());
        }
        #[cfg(target_os = "linux")]
        {
            self.manager.port_mut().install_passphrase(passphrase)?;
            let result = catch_unwind(AssertUnwindSafe(|| self.manager.open()));
            self.manager.port_mut().wipe_passphrase();
            self.manager.port_mut().wipe_wallet_password();
            let opened = self.resume_or_reconcile(result);
            if opened.is_ok() {
                self.manager.port_mut().receiver_wallet_state = WalletState::Ready;
            }
            opened
        }
    }

    pub fn lock(&mut self) -> Result<(), XmrError> {
        #[cfg(not(target_os = "linux"))]
        {
            return Err(XmrError::unavailable());
        }
        #[cfg(target_os = "linux")]
        {
            let result = catch_unwind(AssertUnwindSafe(|| self.manager.lock()));
            self.manager.port_mut().wipe_passphrase();
            self.manager.port_mut().wipe_wallet_password();
            let locked = self.resume_or_reconcile(result);
            self.manager.port_mut().receiver_wallet_state = WalletState::Locked;
            locked
        }
    }

    pub fn view(
        &mut self,
        account_id: &str,
        network: &str,
    ) -> Result<SanitizedAccountView, XmrError> {
        #[cfg(not(target_os = "linux"))]
        {
            let _ = (account_id, network);
            Err(XmrError::unavailable())
        }
        #[cfg(target_os = "linux")]
        {
            self.manager.require_available()?;
            let parsed = XmrNetwork::parse(network)?;
            if !valid_account_id(account_id) || account_id != self.manager.account_id {
                return Err(XmrError::request_schema());
            }
            if parsed != self.manager.port().network {
                return Err(XmrError::wrong_network());
            }
            self.manager.port_mut().production_view(account_id, parsed)
        }
    }

    pub fn fresh_receiver(
        &mut self,
        account_id: &str,
        network: &str,
        request_id: &str,
    ) -> Result<FreshReceiver, XmrError> {
        #[cfg(not(target_os = "linux"))]
        {
            let _ = (account_id, network, request_id);
            Err(XmrError::unavailable())
        }
        #[cfg(target_os = "linux")]
        {
            self.manager.require_available()?;
            let owned_account = self.manager.account_id.clone();
            let owned_network = self.manager.port().network;
            issue_fresh_with_port(
                &owned_account,
                owned_network,
                account_id,
                network,
                request_id,
                self.manager.port_mut(),
            )
        }
    }

    #[cfg(target_os = "linux")]
    fn resume_or_reconcile<T>(&mut self, result: std::thread::Result<T>) -> T {
        match result {
            Ok(value) => value,
            Err(payload) => {
                if catch_unwind(AssertUnwindSafe(|| self.manager.reconcile_on_unwind())).is_err() {
                    self.manager.mark_unavailable();
                }
                resume_unwind(payload)
            }
        }
    }
}

#[cfg(target_os = "linux")]
impl SystemAccountPort {
    pub fn new(
        root: &Path,
        executable: &Path,
        account_id: &str,
        network: &str,
        kind: &str,
    ) -> Result<Self, XmrError> {
        if !valid_account_id(account_id) {
            return Err(XmrError::request_schema());
        }
        let network = XmrNetwork::parse(network)?;
        let kind = AccountKind::parse(kind)?;
        let root_text = root.to_str().ok_or_else(XmrError::request_schema)?;
        if !root.is_absolute() || root_text.chars().any(char::is_control) {
            return Err(XmrError::request_schema());
        }
        let owner = current_uid()?;
        preflight_broker_tree(root, root, owner, true)?;
        Ok(Self {
            account_id: account_id.to_owned(),
            network,
            _kind: kind,
            root: root.to_path_buf(),
            executable_path: executable.to_path_buf(),
            passphrase: None,
            pool: WalletRpcProcessPool::new()?,
            vault: None,
            store: None,
            operations: Vec::new(),
            wipe_events: Arc::new(Mutex::new(Vec::new())),
            child_count: 0,
            handles: 0,
            replaced_hostile: false,
            owner,
            last_password: None,
            attempt: AttemptOwnership::default(),
            receiver_unavailable: false,
            receiver_node_state: NodeState::Unavailable,
            receiver_wallet_state: WalletState::Locked,
            receiver_last_index: 0,
        })
    }

    fn install_passphrase(&mut self, passphrase: SecretBytes) -> Result<(), XmrError> {
        self.wipe_passphrase();
        self.passphrase = Some(passphrase);
        Ok(())
    }

    fn wipe_passphrase(&mut self) {
        if let Some(mut passphrase) = self.passphrase.take() {
            let mut observer = self.observer();
            passphrase.wipe_with("vault-passphrase", &mut observer);
        }
    }

    fn account_base(&self) -> PathBuf {
        self.root
            .join("xmr")
            .join(self.network.name())
            .join(&self.account_id)
    }

    fn wallet_dir(&self) -> PathBuf {
        self.account_base().join("wallet")
    }

    fn wallet_file(&self) -> PathBuf {
        self.wallet_dir().join(&self.account_id)
    }

    fn keys_file(&self) -> PathBuf {
        let mut path = self.wallet_file().into_os_string();
        path.push(".keys");
        PathBuf::from(path)
    }

    fn observer(&self) -> SharedWipeObserver {
        SharedWipeObserver {
            events: self.wipe_events.clone(),
        }
    }

    fn vault_file(&self) -> PathBuf {
        self.root
            .join("vault")
            .join(format!("{}.vault", self.account_id))
    }

    fn state_file(&self) -> PathBuf {
        self.account_base().join(state_file_name())
    }

    fn remember_password(&mut self, password: &str) -> Result<(), XmrError> {
        self.last_password =
            Some(SecretBytes::new(password.as_bytes().to_vec()).map_err(|_| XmrError::internal())?);
        Ok(())
    }

    fn ensure_layout(&self) -> Result<(), XmrError> {
        preflight_broker_tree(&self.root, &self.root, self.owner, true)?;
        let namespace = self.root.join("xmr");
        ensure_private_directory(&namespace, self.owner)?;
        preflight_broker_tree(&self.root, &namespace, self.owner, true)?;
        let network_dir = namespace.join(self.network.name());
        ensure_private_directory(&network_dir, self.owner)?;
        preflight_broker_tree(&self.root, &network_dir, self.owner, true)?;
        ensure_private_directory(&self.account_base(), self.owner)?;
        preflight_broker_tree(&self.root, &self.account_base(), self.owner, true)?;
        ensure_private_directory(&self.wallet_dir(), self.owner)?;
        preflight_broker_tree(&self.root, &self.wallet_dir(), self.owner, true)
    }

    fn open_store_new(&mut self) -> Result<(), XmrError> {
        if self.store.is_some() {
            return Err(XmrError::state_corrupt());
        }
        if path_exists(&self.state_file())? {
            return Err(XmrError::state_corrupt());
        }
        preflight_broker_tree(&self.root, &self.account_base(), self.owner, true)?;
        let handle = PathSqliteSurface::exclusive_create_file(&self.state_file(), self.owner)?;
        self.attempt.record_provisional_uncertainty();
        let identity = PathSqliteSurface::created_file_identity(&handle)?;
        self.attempt.publish_state(ArtifactIdentity {
            path: self.state_file(),
            dev: identity.0,
            ino: identity.1,
        });
        let surface =
            PathSqliteSurface::bind_created(&self.account_base(), self.owner, handle, identity)?;
        self.store = Some(AccountStore::new(surface));
        Ok(())
    }

    fn open_store_existing(&mut self) -> Result<(), XmrError> {
        self.store = None;
        preflight_broker_tree(&self.root, &self.account_base(), self.owner, true)?;
        let surface = PathSqliteSurface::open_existing(&self.account_base(), self.owner)?;
        self.store = Some(AccountStore::attach_existing(surface)?);
        Ok(())
    }

    fn vault_metadata(&self) -> Result<VaultMetadata, XmrError> {
        VaultMetadata::new(
            account_id_bytes(&self.account_id)?,
            Asset::Xmr,
            vault_network(self.network),
            VAULT_EPOCH,
        )
        .map_err(map_vault_error)
    }

    fn capture_regular(&self, path: &Path) -> Result<Option<ArtifactIdentity>, XmrError> {
        match fs::symlink_metadata(path) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(_) => Err(XmrError::internal()),
            Ok(metadata) => {
                if metadata.file_type().is_symlink() || !metadata.file_type().is_file() {
                    return Err(XmrError::state_corrupt());
                }
                let opened = OpenOptions::new()
                    .read(true)
                    .custom_flags(LINUX_O_NOFOLLOW)
                    .open(path)
                    .map_err(|_| XmrError::internal())?;
                let opened_meta = opened.metadata().map_err(|_| XmrError::internal())?;
                if !opened_meta.file_type().is_file()
                    || opened_meta.dev() != metadata.dev()
                    || opened_meta.ino() != metadata.ino()
                {
                    return Err(XmrError::internal());
                }
                Ok(Some(ArtifactIdentity {
                    path: path.to_path_buf(),
                    dev: opened_meta.dev(),
                    ino: opened_meta.ino(),
                }))
            }
        }
    }

    fn capture_wallet_artifacts(&mut self) -> Result<(), XmrError> {
        let wallet = self.capture_regular(&self.wallet_file());
        let keys = self.capture_regular(&self.keys_file());
        let mut failed = false;
        match wallet {
            Ok(Some(identity)) if self.attempt.wallet.is_none() => {
                self.attempt.wallet = Some(identity);
            }
            Ok(_) => {}
            Err(_) => {
                self.attempt.uncertain = true;
                failed = true;
            }
        }
        match keys {
            Ok(Some(identity)) if self.attempt.keys.is_none() => {
                self.attempt.keys = Some(identity);
            }
            Ok(_) => {}
            Err(_) => {
                self.attempt.uncertain = true;
                failed = true;
            }
        }
        if failed {
            Err(XmrError::internal())
        } else {
            Ok(())
        }
    }

    fn require_complete_wallet_pair(&self) -> Result<(), XmrError> {
        let wallet = self
            .attempt
            .wallet
            .as_ref()
            .ok_or_else(XmrError::internal)?;
        let keys = self.attempt.keys.as_ref().ok_or_else(XmrError::internal)?;
        self.revalidate_owned_secret_file(wallet)?;
        self.revalidate_owned_secret_file(keys)?;
        preflight_broker_tree(&self.root, &self.wallet_dir(), self.owner, true)
    }

    fn revalidate_owned_secret_file(&self, artifact: &ArtifactIdentity) -> Result<(), XmrError> {
        let opened = OpenOptions::new()
            .read(true)
            .custom_flags(LINUX_O_NOFOLLOW)
            .open(&artifact.path)
            .map_err(|_| XmrError::internal())?;
        let metadata = opened.metadata().map_err(|_| XmrError::internal())?;
        if !metadata.file_type().is_file()
            || metadata.uid() != self.owner
            || metadata.permissions().mode() & 0o777 != WALLET_FILE_MODE
            || metadata.dev() != artifact.dev
            || metadata.ino() != artifact.ino
        {
            return Err(XmrError::internal());
        }
        Ok(())
    }

    fn after_wallet_rpc<T>(&mut self, result: Result<T, XmrError>) -> Result<T, XmrError> {
        if self.capture_wallet_artifacts().is_err() {
            self.attempt.uncertain = true;
        }
        match result {
            Ok(value) => {
                if self.attempt.uncertain || self.require_complete_wallet_pair().is_err() {
                    self.attempt.uncertain = true;
                    Err(XmrError::internal())
                } else {
                    Ok(value)
                }
            }
            Err(error) => Err(error),
        }
    }

    fn attach_existing_vault(&mut self) -> Result<(), XmrError> {
        if self.vault.is_some() {
            return Ok(());
        }
        let vault_root = self.root.join("vault");
        preflight_broker_tree(&self.root, &vault_root, self.owner, true)?;
        let vault = VaultStore::new(&vault_root, LinuxStorePort::new())
            .map_err(|_| XmrError::internal())?;
        self.vault = Some(vault);
        Ok(())
    }

    fn attach_or_create_vault_dir(&mut self) -> Result<(), XmrError> {
        if self.vault.is_some() {
            return Ok(());
        }
        let vault_root = self.root.join("vault");
        if path_exists(&vault_root)? {
            preflight_broker_tree(&self.root, &vault_root, self.owner, true)?;
        } else {
            ensure_private_directory(&vault_root, self.owner)?;
        }
        let vault = VaultStore::new(&vault_root, LinuxStorePort::new())
            .map_err(|_| XmrError::internal())?;
        self.vault = Some(vault);
        Ok(())
    }

    fn preflight_all_existing(&self) -> Result<(), XmrError> {
        preflight_broker_tree(&self.root, &self.root, self.owner, true)?;
        for path in [
            self.root.join("vault"),
            self.root.join("xmr"),
            self.root.join("xmr").join(self.network.name()),
            self.account_base(),
            self.wallet_dir(),
            self.account_base().join("runtime"),
            self.account_base().join("shared-ringdb"),
        ] {
            if path_exists(&path)? {
                preflight_broker_tree(&self.root, &path, self.owner, true)?;
            }
        }
        for path in [
            self.vault_file(),
            self.state_file(),
            self.wallet_file(),
            self.keys_file(),
        ] {
            if path_exists(&path)? {
                inspect_secret_file(&path, self.owner)?
                    .ok_or_else(XmrError::state_corrupt)?
                    .map_err(|_| XmrError::state_corrupt())?;
                let opened = OpenOptions::new()
                    .read(true)
                    .custom_flags(LINUX_O_NOFOLLOW)
                    .open(&path)
                    .map_err(|_| XmrError::state_corrupt())?;
                let metadata = opened.metadata().map_err(|_| XmrError::state_corrupt())?;
                let listed = fs::symlink_metadata(&path).map_err(|_| XmrError::state_corrupt())?;
                if metadata.dev() != listed.dev() || metadata.ino() != listed.ino() {
                    return Err(XmrError::state_corrupt());
                }
            }
        }
        Ok(())
    }

    fn live_store(&mut self) -> Result<&mut AccountStore<PathSqliteSurface>, XmrError> {
        self.store.as_mut().ok_or_else(XmrError::state_corrupt)
    }

    fn production_view(
        &mut self,
        account_id: &str,
        network: XmrNetwork,
    ) -> Result<SanitizedAccountView, XmrError> {
        if self.receiver_unavailable {
            return Err(XmrError::state_corrupt());
        }
        let identity = match self.live_store()?.load_identity() {
            Ok(identity) => identity,
            Err(error) => {
                if error.code() == "STATE_CORRUPT" {
                    self.receiver_unavailable = true;
                }
                return Err(error);
            }
        };
        if identity.account_id() != account_id || identity.network() != network.name() {
            self.receiver_unavailable = true;
            return Err(XmrError::state_corrupt());
        }
        let kind = AccountKind::from_code(identity.kind())?;
        let (node_state, node_height, hard_fork) = match probe_local_node_view(network) {
            Ok(node) => (
                match node.state {
                    RpcNodeState::Syncing => NodeState::Syncing,
                    RpcNodeState::Ready => NodeState::Ready,
                },
                Some(node.height),
                Some(node.hard_fork),
            ),
            Err(error) if error.code() == "NODE_UNAVAILABLE" => {
                (NodeState::Unavailable, None, None)
            }
            Err(error) => return Err(error),
        };
        let wallet_locked = self.receiver_wallet_state == WalletState::Locked;
        let wallet_snapshot = if wallet_locked || node_state == NodeState::Unavailable {
            None
        } else {
            (|| -> Result<Option<(u64, u64, u64)>, XmrError> {
                if let Err(error) = self.pool.prove_owned_session(account_id, network) {
                    return if error.code() == "UNAVAILABLE" {
                        Ok(None)
                    } else {
                        Err(error)
                    };
                }
                if let Err(error) = self.pool.refresh(account_id) {
                    return if error.code() == "UNAVAILABLE" {
                        Ok(None)
                    } else {
                        Err(error)
                    };
                }
                let height = match self.pool.get_height(account_id) {
                    Ok(height) => height,
                    Err(error) if error.code() == "UNAVAILABLE" => return Ok(None),
                    Err(error) => return Err(error),
                };
                match self.pool.get_balance(account_id) {
                    Ok((total, unlocked)) => Ok(Some((height, total, unlocked))),
                    Err(error) if error.code() == "UNAVAILABLE" => Ok(None),
                    Err(error) => Err(error),
                }
            })()?
        };
        let (wallet_available, wallet_height, total_atomic, unlocked_atomic) = match wallet_snapshot
        {
            Some((height, total, unlocked)) => (true, Some(height), Some(total), Some(unlocked)),
            None => (false, None, None, None),
        };
        build_production_view(ProductionViewInput {
            account_id,
            network,
            kind,
            node_state,
            node_height,
            wallet_available,
            wallet_locked,
            wallet_height,
            total_atomic,
            unlocked_atomic,
            hard_fork: hard_fork.as_ref(),
        })
    }
}

#[cfg(target_os = "linux")]
impl AccountPort for SystemAccountPort {
    fn note(&mut self, operation: &'static str) {
        self.operations.push(operation);
    }

    fn operations(&self) -> &[&'static str] {
        &self.operations
    }

    fn fill_entropy(&mut self, output: &mut [u8]) -> Result<(), XmrError> {
        OsEntropy
            .fill("xmr-wallet-password", output)
            .map_err(|_| XmrError::internal())
    }

    fn local_height_without_bootstrap(&mut self) -> Result<u64, XmrError> {
        probe_local_node_state(self.network).map(|result| result.height_without_bootstrap)
    }

    fn begin_attempt(&mut self) {
        self.attempt = AttemptOwnership::default();
    }

    fn commit_attempt(&mut self) {
        self.attempt = AttemptOwnership::default();
    }

    fn preflight_create_new(&mut self, paths: &AccountPathsView) -> Result<(), XmrError> {
        self.preflight_all_existing()?;
        if path_exists(&self.vault_file())?
            || path_exists(&self.state_file())?
            || path_exists(&self.wallet_file())?
            || path_exists(&self.keys_file())?
        {
            return Err(XmrError::state_corrupt());
        }
        match self.inspect_wallet(paths)? {
            WalletPresence::Missing => Ok(()),
            _ => Err(XmrError::state_corrupt()),
        }
    }

    fn preflight_open_existing(
        &mut self,
        paths: &AccountPathsView,
    ) -> Result<WalletPresence, XmrError> {
        self.preflight_all_existing()?;
        if !path_exists(&self.vault_file())? || !path_exists(&self.state_file())? {
            return Err(XmrError::state_corrupt());
        }
        inspect_secret_file(&self.vault_file(), self.owner)?
            .ok_or_else(XmrError::state_corrupt)?
            .map_err(|_| XmrError::state_corrupt())?;
        inspect_secret_file(&self.state_file(), self.owner)?
            .ok_or_else(XmrError::state_corrupt)?
            .map_err(|_| XmrError::state_corrupt())?;
        match self.inspect_wallet(paths)? {
            WalletPresence::Hostile(_) | WalletPresence::Partial => Err(XmrError::state_corrupt()),
            presence => Ok(presence),
        }
    }

    fn create_private_layout_after_preflight(&mut self) -> Result<(), XmrError> {
        self.ensure_layout()
    }

    fn create_missing_wallet_layout(&mut self) -> Result<(), XmrError> {
        preflight_broker_tree(&self.root, &self.account_base(), self.owner, true)?;
        ensure_private_directory(&self.wallet_dir(), self.owner)?;
        preflight_broker_tree(&self.root, &self.wallet_dir(), self.owner, true)
    }

    fn inspect_wallet(&mut self, paths: &AccountPathsView) -> Result<WalletPresence, XmrError> {
        inspect_wallet_layout(
            &self.root,
            &self.wallet_dir(),
            &paths.wallet_filename,
            self.network,
            &self.account_id,
            self.owner,
        )
    }

    fn create_wallet_files(&mut self, paths: &AccountPathsView) -> Result<(), XmrError> {
        match self.inspect_wallet(paths)? {
            WalletPresence::Missing => Ok(()),
            _ => Err(XmrError::state_corrupt()),
        }
    }

    fn rollback_owned_artifacts(&mut self) -> Result<(), XmrError> {
        let mut failed = self.attempt.cleanup_uncertain();
        if self.attempt.state.is_some() {
            self.store = None;
        }
        if let Some(wallet) = self.attempt.wallet.clone() {
            match quarantine_identified(&wallet, self.owner) {
                Ok(()) => self.attempt.wallet = None,
                Err(_) => failed = true,
            }
        }
        if let Some(keys) = self.attempt.keys.clone() {
            match quarantine_identified(&keys, self.owner) {
                Ok(()) => self.attempt.keys = None,
                Err(_) => failed = true,
            }
        }
        if let Some(state) = self.attempt.state.clone() {
            match quarantine_identified(&state, self.owner) {
                Ok(()) => {
                    self.attempt.state = None;
                    self.store = None;
                }
                Err(_) => failed = true,
            }
        }
        if let Some(vault) = self.attempt.vault.clone() {
            match quarantine_identified(&vault, self.owner) {
                Ok(()) => self.attempt.vault = None,
                Err(_) => failed = true,
            }
        }
        if failed {
            Err(XmrError::internal())
        } else {
            Ok(())
        }
    }

    fn start_child(&mut self) -> Result<(), XmrError> {
        preflight_broker_tree(&self.root, &self.root, self.owner, true)?;
        let executable =
            InstallationVerifier::linux_x86_64().verify_selected(&self.executable_path)?;
        self.pool.start_account(
            &self.account_id,
            self.network,
            executable,
            &self.root,
            SystemWalletRpcControl::new(),
        )?;
        self.child_count = 1;
        Ok(())
    }

    fn create_wallet(&mut self, filename: &str, password: &str) -> Result<(), XmrError> {
        self.remember_password(password)?;
        let result = self.pool.create_wallet(&self.account_id, password);
        self.after_wallet_rpc(result)?;
        self.handles = 1;
        let _ = filename;
        Ok(())
    }

    fn query_mnemonic(&mut self) -> Result<Zeroizing<String>, XmrError> {
        self.pool.query_mnemonic(&self.account_id)
    }

    fn get_primary_address(&mut self) -> Result<WalletRpcObservation, XmrError> {
        let primary = self.pool.get_primary_address(&self.account_id)?;
        Ok(WalletRpcObservation {
            verified_primary: Zeroizing::new(primary.as_str().to_owned()),
            primary,
            watch_only: false,
        })
    }

    fn validate_primary_for_network(&mut self, address: &str) -> Result<(), XmrError> {
        self.pool
            .validate_primary_address(&self.account_id, address)
    }

    fn generate_from_keys(
        &mut self,
        filename: &str,
        password: &str,
        address: &str,
        viewkey: &str,
        restore_height: u64,
    ) -> Result<WalletRpcObservation, XmrError> {
        let _ = filename;
        self.remember_password(password)?;
        let result = self.pool.generate_from_keys(
            &self.account_id,
            password,
            address,
            viewkey,
            restore_height,
        );
        let primary = self.after_wallet_rpc(result)?;
        self.handles = 1;
        Ok(WalletRpcObservation {
            verified_primary: Zeroizing::new(primary.as_str().to_owned()),
            primary,
            watch_only: true,
        })
    }

    fn open_wallet(
        &mut self,
        filename: &str,
        password: &str,
    ) -> Result<WalletRpcObservation, XmrError> {
        let _ = filename;
        self.remember_password(password)?;
        self.pool.open_wallet(&self.account_id, password)?;
        self.handles = 1;
        self.get_primary_address()
    }

    fn restore_deterministic_wallet(
        &mut self,
        filename: &str,
        password: &str,
        seed: &str,
        restore_height: u64,
    ) -> Result<WalletRpcObservation, XmrError> {
        let _ = filename;
        self.remember_password(password)?;
        let result = self.pool.restore_deterministic_wallet(
            &self.account_id,
            password,
            seed,
            restore_height,
        );
        let primary = self.after_wallet_rpc(result)?;
        self.handles = 1;
        Ok(WalletRpcObservation {
            verified_primary: Zeroizing::new(primary.as_str().to_owned()),
            primary,
            watch_only: false,
        })
    }

    fn close_wallet(&mut self) -> Result<(), XmrError> {
        let result = self.pool.close_wallet(&self.account_id);
        if result.is_ok() {
            self.handles = 0;
        }
        result
    }

    fn recorded_calls(&self) -> &[AccountRpcCall] {
        &[]
    }

    fn seal_vault(&mut self, secret: &XmrSecretV1) -> Result<(), XmrError> {
        self.attach_or_create_vault_dir()?;
        if path_exists(&self.vault_file())? {
            return Err(XmrError::state_corrupt());
        }
        let metadata = self.vault_metadata()?;
        let mut plaintext = SecretBytes::new(secret.encode()?.to_vec()).map_err(map_vault_error)?;
        let mut passphrase = self
            .passphrase
            .as_ref()
            .ok_or_else(XmrError::unauth)?
            .expose(|bytes| SecretBytes::new(bytes.to_vec()))
            .map_err(map_vault_error)?;
        let mut entropy = OsEntropy;
        let mut observer = self.observer();
        let envelope = seal_vault(
            &metadata,
            &mut passphrase,
            &mut plaintext,
            &mut entropy,
            &mut observer,
        )
        .map_err(map_vault_error)?;
        let vault_root = self.root.join("vault");
        let file = exclusive_create_active_envelope(&vault_root, &self.account_id, self.owner)?;
        self.attempt.record_provisional_uncertainty();
        let identity = created_file_artifact(&file, self.vault_file())?;
        self.attempt.publish_vault(identity.clone());
        write_active_envelope(
            file,
            envelope.as_bytes(),
            &identity,
            &vault_root,
            self.owner,
        )?;
        Ok(())
    }

    fn open_vault(&mut self) -> Result<XmrSecretV1, XmrError> {
        self.attach_existing_vault()?;
        let bytes = self
            .vault
            .as_mut()
            .ok_or_else(XmrError::internal)?
            .read_active(&self.account_id)
            .map_err(map_store_error)?;
        let mut work = SilentWork;
        let envelope = parse_vault(&bytes, &mut work).map_err(map_vault_error)?;
        if envelope.metadata() != &self.vault_metadata()? {
            return Err(XmrError::state_corrupt());
        }
        let mut passphrase = self
            .passphrase
            .as_ref()
            .ok_or_else(XmrError::unauth)?
            .expose(|value| SecretBytes::new(value.to_vec()))
            .map_err(map_vault_error)?;
        let mut observer = self.observer();
        let plaintext = open_vault_bytes(&bytes, &mut passphrase, &mut work, &mut observer)
            .map_err(map_vault_error)?;
        plaintext.expose(XmrSecretV1::decode)
    }

    fn persist_state(&mut self, identity: &StoredIdentity) -> Result<(), XmrError> {
        self.open_store_new()?;
        self.store
            .as_mut()
            .ok_or_else(XmrError::internal)?
            .persist_identity(identity)
    }

    fn load_state(&mut self) -> Result<StoredIdentity, XmrError> {
        self.open_store_existing()?;
        self.store
            .as_mut()
            .ok_or_else(XmrError::internal)?
            .load_identity()
    }

    fn stop_and_reap_child(&mut self) -> Result<(), XmrError> {
        self.teardown_owned()
    }

    fn teardown_owned(&mut self) -> Result<(), XmrError> {
        if self.child_count == 0 {
            self.wipe_wallet_password();
            return Ok(());
        }
        let result = self.pool.stop_account(&self.account_id);
        self.child_count = 0;
        self.handles = 0;
        self.wipe_wallet_password();
        result
    }

    fn prove_owned_session(
        &mut self,
        account_id: &str,
        network: XmrNetwork,
    ) -> Result<(), XmrError> {
        self.pool.prove_owned_session(account_id, network)
    }

    fn wipe_wallet_password(&mut self) {
        let mut observer = self.observer();
        if let Some(mut password) = self.last_password.take() {
            password.wipe_with("xmr-wallet-password", &mut observer);
        }
    }

    fn wipe_events(&self) -> Vec<WipeEvent> {
        self.wipe_events
            .lock()
            .map(|events| events.clone())
            .unwrap_or_default()
    }

    fn active_child_count(&self) -> usize {
        self.child_count
    }

    fn open_handle_count(&self) -> usize {
        self.handles
    }

    fn replaced_hostile_entry(&self) -> bool {
        self.replaced_hostile
    }
}

#[cfg(target_os = "linux")]
impl ReceiverPort for SystemAccountPort {
    fn rpc_calls(&self) -> Vec<String> {
        Vec::new()
    }

    fn clear_rpc_calls(&mut self) {}

    fn lookup_binding(&mut self, request_id: &str) -> Result<Option<FreshReceiver>, XmrError> {
        let account_id = self.account_id.clone();
        let network = self.network;
        match self.live_store()?.lookup_receiver(request_id)? {
            Some(stored) => binding_from_stored(&account_id, network, &stored).map(Some),
            None => Ok(None),
        }
    }

    fn lookup_all(&mut self) -> Result<Vec<FreshReceiver>, XmrError> {
        let account_id = self.account_id.clone();
        let network = self.network;
        self.live_store()?
            .list_receivers()?
            .iter()
            .map(|stored| binding_from_stored(&account_id, network, stored))
            .collect()
    }

    fn load_identity(&mut self) -> Result<StoredIdentity, XmrError> {
        self.live_store()?.load_identity()
    }

    fn create_address(&mut self) -> Result<CreatedSubaddress, XmrError> {
        let (address, subaddress_index) = self.pool.create_address(&self.account_id)?;
        self.receiver_last_index = subaddress_index;
        Ok(CreatedSubaddress {
            address,
            account_index: 0,
            subaddress_index,
        })
    }

    fn validate_subaddress(
        &mut self,
        address: &str,
        network: XmrNetwork,
    ) -> Result<AddressClassification, XmrError> {
        if network != self.network {
            return Err(XmrError::wrong_network());
        }
        self.pool.validate_subaddress(&self.account_id, address)?;
        Ok(AddressClassification {
            valid: true,
            integrated: false,
            subaddress: true,
            nettype: expected_nettype(network).to_owned(),
        })
    }

    fn get_indexed_address(
        &mut self,
        account_index: u32,
        address_index: u32,
    ) -> Result<Zeroizing<String>, XmrError> {
        self.pool
            .get_indexed_address(&self.account_id, account_index, address_index)
    }

    fn persist_binding(
        &mut self,
        binding: &FreshReceiver,
    ) -> Result<ReceiverPersistenceProof, XmrError> {
        self.live_store()?
            .persist_receiver(&stored_from_binding(binding)?)
    }

    fn set_issuance_watermarks(&mut self, index: u64, sequence: u64) -> Result<(), XmrError> {
        if index > u64::from(u32::MAX) || sequence > i64::MAX as u64 {
            return Err(XmrError::limit());
        }
        self.receiver_last_index = u32::try_from(index).map_err(|_| XmrError::limit())?;
        let sequence = i64::try_from(sequence).map_err(|_| XmrError::limit())?;
        self.live_store()?.set_issuance_sequence(sequence)
    }

    fn greatest_sequence(&mut self) -> Result<i64, XmrError> {
        Ok(self
            .live_store()?
            .load_identity()?
            .greatest_issuance_sequence())
    }

    fn last_subaddress_index(&mut self) -> Result<u32, XmrError> {
        let retained = self.receiver_last_index;
        Ok(self
            .live_store()?
            .max_subaddress_index()?
            .unwrap_or(retained)
            .max(retained))
    }

    fn inspect_schema(&mut self) -> Result<ReceiverSchemaView, XmrError> {
        let mode = self.live_store()?.surface_mut().state_file_mode()?;
        self.live_store()?.inspect_receiver_schema(mode)
    }

    fn reopen(&mut self) -> Result<(), XmrError> {
        self.live_store()?.reopen().map(|_| ())
    }

    fn begin_create_address(&mut self) {}

    fn end_create_address(&mut self) {}

    fn max_in_flight_create_address(&self) -> usize {
        usize::from(self.child_count > 0)
    }

    fn wallet_state(&self) -> WalletState {
        self.receiver_wallet_state
    }

    fn node_state(&self) -> NodeState {
        self.receiver_node_state
    }

    fn watch_only_initialization_failed(&self) -> bool {
        false
    }

    fn prove_owned_identity(&mut self) -> Result<(), XmrError> {
        if self.receiver_unavailable {
            return Err(XmrError::state_corrupt());
        }
        if self.receiver_wallet_state == WalletState::Locked {
            return Err(XmrError::locked());
        }
        let account_id = self.account_id.clone();
        self.pool.prove_owned_session(&account_id, self.network)
    }

    fn prepare_receiver(&mut self) -> Result<(), XmrError> {
        if self.receiver_unavailable {
            return Err(XmrError::state_corrupt());
        }
        let account_id = self.account_id.clone();
        self.pool.prove_owned_session(&account_id, self.network)?;
        let node = probe_local_node_view(self.network)?;
        self.receiver_node_state = match node.state {
            RpcNodeState::Syncing => NodeState::Syncing,
            RpcNodeState::Ready => NodeState::Ready,
        };
        if self.receiver_node_state == NodeState::Syncing {
            return Ok(());
        }
        self.pool.refresh(&account_id)?;
        let wallet_height = self.pool.get_height(&account_id)?;
        self.receiver_wallet_state = if wallet_height < node.height {
            WalletState::Refreshing
        } else if wallet_height == node.height {
            WalletState::Ready
        } else {
            return Err(XmrError::protocol_incompatible());
        };
        Ok(())
    }

    fn latch_unavailable(&mut self) {
        self.receiver_unavailable = true;
    }

    fn authority_unavailable(&self) -> bool {
        self.receiver_unavailable
    }
}

fn encode_frame(
    kind: AccountKind,
    restore_height: u64,
    password: &[u8],
    primary: &[u8],
    secret: &[u8],
) -> Result<Zeroizing<Vec<u8>>, XmrError> {
    validate_password_hex_length(password.len())?;
    validate_password_hex(password)?;
    validate_primary_address_length(primary.len())?;
    validate_secret_payload(kind, secret)?;
    let secret_len = u16::try_from(secret.len()).map_err(|_| XmrError::state_corrupt())?;
    let total = HEADER_BYTES + secret.len();
    validate_total_length(total)?;
    let mut output = Zeroizing::new(Vec::with_capacity(total));
    output.extend_from_slice(&XMR_SECRET_MAGIC);
    output.push(kind.code());
    output.extend_from_slice(&restore_height.to_be_bytes());
    output.extend_from_slice(&(PASSWORD_HEX_BYTES as u16).to_be_bytes());
    output.extend_from_slice(password);
    output.extend_from_slice(&(PRIMARY_ADDRESS_BYTES as u16).to_be_bytes());
    output.extend_from_slice(primary);
    output.extend_from_slice(&secret_len.to_be_bytes());
    output.extend_from_slice(secret);
    Ok(output)
}

fn validate_password_hex(bytes: &[u8]) -> Result<(), XmrError> {
    validate_password_hex_length(bytes.len())?;
    if is_lowercase_hex(bytes) {
        Ok(())
    } else {
        Err(XmrError::state_corrupt())
    }
}

fn validate_secret_payload(kind: AccountKind, secret: &[u8]) -> Result<(), XmrError> {
    match kind {
        AccountKind::Software => validate_mnemonic(secret),
        AccountKind::WatchOnly => {
            validate_view_key_hex_length(secret.len())?;
            if is_lowercase_hex(secret) {
                Ok(())
            } else {
                Err(XmrError::state_corrupt())
            }
        }
    }
}

fn validate_mnemonic(secret: &[u8]) -> Result<(), XmrError> {
    let text = utf8(secret)?;
    let words: Vec<&str> = text.split_ascii_whitespace().collect();
    if words.len() == MNEMONIC_WORDS
        && words
            .iter()
            .all(|word| !word.is_empty() && word.bytes().all(|byte| byte.is_ascii_lowercase()))
    {
        Ok(())
    } else {
        Err(XmrError::state_corrupt())
    }
}

fn is_lowercase_hex(bytes: &[u8]) -> bool {
    bytes
        .iter()
        .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(byte))
}

fn utf8(bytes: &[u8]) -> Result<&str, XmrError> {
    core::str::from_utf8(bytes).map_err(|_| XmrError::state_corrupt())
}

#[cfg(target_os = "linux")]
pub(crate) fn current_uid() -> Result<u32, XmrError> {
    fs::metadata("/proc/self")
        .map(|metadata| metadata.uid())
        .map_err(|_| XmrError::internal())
}

fn map_store_error(error: crate::store::StoreError) -> XmrError {
    match error.code() {
        "NOT_FOUND" | "UNAVAILABLE" | "STATE_CORRUPT" | "SCHEMA" => XmrError::state_corrupt(),
        "LOCKED" => XmrError::unauth(),
        "LIMIT" => XmrError::limit(),
        _ => XmrError::internal(),
    }
}

fn path_exists(path: &Path) -> Result<bool, XmrError> {
    match fs::symlink_metadata(path) {
        Ok(_) => Ok(true),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(_) => Err(XmrError::state_corrupt()),
    }
}

fn account_id_bytes(account_id: &str) -> Result<[u8; 16], XmrError> {
    if !valid_account_id(account_id) {
        return Err(XmrError::request_schema());
    }
    let mut output = [0u8; 16];
    for (index, chunk) in account_id.as_bytes().as_chunks::<2>().0.iter().enumerate() {
        output[index] = (hex_nibble(chunk[0])? << 4) | hex_nibble(chunk[1])?;
    }
    Ok(output)
}

fn hex_nibble(value: u8) -> Result<u8, XmrError> {
    match value {
        b'0'..=b'9' => Ok(value - b'0'),
        b'a'..=b'f' => Ok(value - b'a' + 10),
        _ => Err(XmrError::request_schema()),
    }
}

fn vault_network(network: XmrNetwork) -> VaultNetwork {
    match network {
        XmrNetwork::Stagenet => VaultNetwork::XmrStagenet,
        XmrNetwork::Testnet => VaultNetwork::XmrTestnet,
    }
}

fn map_vault_error(error: VaultError) -> XmrError {
    match error.code() {
        "SCHEMA" | "WRONG_NETWORK" => XmrError::state_corrupt(),
        "LIMIT" => XmrError::limit(),
        "LOCKED" => XmrError::unauth(),
        _ => XmrError::internal(),
    }
}

#[cfg(target_os = "linux")]
fn preflight_broker_tree(
    root: &Path,
    path: &Path,
    owner: u32,
    require_directory: bool,
) -> Result<(), XmrError> {
    if !root.is_absolute() || !path.is_absolute() || !path.starts_with(root) {
        return Err(XmrError::state_corrupt());
    }
    let mut current = PathBuf::new();
    for component in path.components() {
        current.push(component);
        if !current.starts_with(root) && current != root {
            continue;
        }
        let metadata = fs::symlink_metadata(&current).map_err(|_| XmrError::state_corrupt())?;
        if metadata.file_type().is_symlink()
            || metadata.file_type().is_fifo()
            || metadata.file_type().is_socket()
            || metadata.file_type().is_block_device()
            || metadata.file_type().is_char_device()
        {
            return Err(XmrError::state_corrupt());
        }
        if current == path && !require_directory {
            if !metadata.file_type().is_file()
                || metadata.permissions().mode() & 0o777 != WALLET_FILE_MODE
                || metadata.uid() != owner
            {
                return Err(XmrError::state_corrupt());
            }
        } else if current.starts_with(root)
            && (!metadata.file_type().is_dir()
                || metadata.permissions().mode() & 0o777 != DIRECTORY_MODE
                || metadata.uid() != owner)
        {
            return Err(XmrError::state_corrupt());
        }
    }
    Ok(())
}

#[cfg(target_os = "linux")]
pub(crate) fn ensure_private_directory(path: &Path, owner: u32) -> Result<(), XmrError> {
    let created = match fs::create_dir(path) {
        Ok(()) => {
            fs::set_permissions(path, fs::Permissions::from_mode(DIRECTORY_MODE))
                .map_err(|_| XmrError::internal())?;
            true
        }
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => false,
        Err(_) => return Err(XmrError::internal()),
    };
    let metadata = fs::symlink_metadata(path).map_err(|_| XmrError::state_corrupt())?;
    if metadata.file_type().is_symlink()
        || !metadata.file_type().is_dir()
        || metadata.uid() != owner
        || metadata.permissions().mode() & 0o777 != DIRECTORY_MODE
    {
        return Err(XmrError::state_corrupt());
    }
    let directory = OpenOptions::new()
        .read(true)
        .custom_flags(LINUX_O_NOFOLLOW | LINUX_O_DIRECTORY)
        .open(path)
        .map_err(|_| XmrError::state_corrupt())?;
    let opened = directory
        .metadata()
        .map_err(|_| XmrError::state_corrupt())?;
    if !opened.file_type().is_dir()
        || opened.uid() != owner
        || opened.permissions().mode() & 0o777 != DIRECTORY_MODE
        || opened.dev() != metadata.dev()
        || opened.ino() != metadata.ino()
    {
        return Err(XmrError::state_corrupt());
    }
    if created {
        let parent = path.parent().ok_or_else(XmrError::internal)?;
        sync_directory_nofollow(parent, owner)?;
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn inspect_wallet_layout(
    root: &Path,
    wallet_dir: &Path,
    filename: &str,
    network: XmrNetwork,
    account_id: &str,
    owner: u32,
) -> Result<WalletPresence, XmrError> {
    if filename != account_id || filename.contains('/') {
        return Ok(WalletPresence::Hostile(HostileWalletEntry::CrossAccount));
    }
    if let Some(parent) = wallet_dir.parent()
        && let Some(network_dir) = parent.parent()
        && network_dir.file_name().and_then(|name| name.to_str()) != Some(network.name())
    {
        return Ok(WalletPresence::Hostile(HostileWalletEntry::CrossNetwork));
    }
    match fs::symlink_metadata(wallet_dir) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            if wallet_dir.exists() {
                return Ok(WalletPresence::Hostile(HostileWalletEntry::Symlink));
            }
            return Ok(WalletPresence::Missing);
        }
        Err(_) => return Err(XmrError::state_corrupt()),
        Ok(metadata) => {
            if metadata.file_type().is_symlink() {
                return Ok(WalletPresence::Hostile(HostileWalletEntry::Symlink));
            }
            if !metadata.file_type().is_dir() {
                return Ok(WalletPresence::Hostile(HostileWalletEntry::Directory));
            }
            if metadata.uid() != owner {
                return Ok(WalletPresence::Hostile(HostileWalletEntry::WrongOwner));
            }
            if metadata.permissions().mode() & 0o777 != DIRECTORY_MODE {
                return Ok(WalletPresence::Hostile(HostileWalletEntry::WrongMode));
            }
        }
    }
    if preflight_broker_tree(root, wallet_dir, owner, true).is_err() {
        return Ok(WalletPresence::Hostile(HostileWalletEntry::Symlink));
    }
    let wallet = wallet_dir.join(filename);
    let mut keys = wallet.clone().into_os_string();
    keys.push(".keys");
    let keys = PathBuf::from(keys);
    let wallet_meta = inspect_secret_file(&wallet, owner)?;
    let keys_meta = inspect_secret_file(&keys, owner)?;
    match (wallet_meta, keys_meta) {
        (None, None) => Ok(WalletPresence::Missing),
        (Some(Ok(())), Some(Ok(()))) => Ok(WalletPresence::Complete),
        (Some(Err(kind)), _) | (_, Some(Err(kind))) => Ok(WalletPresence::Hostile(kind)),
        _ => Ok(WalletPresence::Partial),
    }
}

#[cfg(target_os = "linux")]
fn inspect_secret_file(
    path: &Path,
    owner: u32,
) -> Result<Option<Result<(), HostileWalletEntry>>, XmrError> {
    match fs::symlink_metadata(path) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(_) => Err(XmrError::state_corrupt()),
        Ok(metadata) => {
            if metadata.file_type().is_symlink() {
                return Ok(Some(Err(HostileWalletEntry::Symlink)));
            }
            if metadata.file_type().is_dir() {
                return Ok(Some(Err(HostileWalletEntry::Directory)));
            }
            if metadata.file_type().is_fifo() {
                return Ok(Some(Err(HostileWalletEntry::Fifo)));
            }
            if !metadata.file_type().is_file() {
                return Ok(Some(Err(HostileWalletEntry::Directory)));
            }
            if metadata.uid() != owner {
                return Ok(Some(Err(HostileWalletEntry::WrongOwner)));
            }
            if metadata.permissions().mode() & 0o777 != WALLET_FILE_MODE {
                return Ok(Some(Err(HostileWalletEntry::WrongMode)));
            }
            let opened = OpenOptions::new()
                .read(true)
                .custom_flags(LINUX_O_NOFOLLOW)
                .open(path)
                .map_err(|_| XmrError::state_corrupt())?;
            let opened_meta = opened.metadata().map_err(|_| XmrError::state_corrupt())?;
            if !opened_meta.file_type().is_file()
                || opened_meta.uid() != owner
                || opened_meta.permissions().mode() & 0o777 != WALLET_FILE_MODE
            {
                return Ok(Some(Err(HostileWalletEntry::WrongMode)));
            }
            Ok(Some(Ok(())))
        }
    }
}

#[cfg(target_os = "linux")]
fn exclusive_create_active_envelope(
    vault_root: &Path,
    account_id: &str,
    owner: u32,
) -> Result<File, XmrError> {
    preflight_broker_tree(vault_root, vault_root, owner, true)?;
    let active = vault_root.join(format!("{account_id}.vault"));
    OpenOptions::new()
        .read(true)
        .write(true)
        .create_new(true)
        .mode(WALLET_FILE_MODE)
        .custom_flags(LINUX_O_NOFOLLOW)
        .open(&active)
        .map_err(|error| {
            if error.kind() == std::io::ErrorKind::AlreadyExists {
                XmrError::state_corrupt()
            } else {
                XmrError::internal()
            }
        })
}

#[cfg(target_os = "linux")]
fn created_file_artifact(file: &File, path: PathBuf) -> Result<ArtifactIdentity, XmrError> {
    let metadata = file.metadata().map_err(|_| XmrError::internal())?;
    if !metadata.file_type().is_file() {
        return Err(XmrError::internal());
    }
    Ok(ArtifactIdentity {
        path,
        dev: metadata.dev(),
        ino: metadata.ino(),
    })
}

#[cfg(target_os = "linux")]
fn write_active_envelope(
    mut file: File,
    bytes: &[u8],
    identity: &ArtifactIdentity,
    vault_root: &Path,
    owner: u32,
) -> Result<(), XmrError> {
    if bytes.is_empty() {
        return Err(XmrError::internal());
    }
    use std::io::Write as _;
    file.write_all(bytes).map_err(|_| XmrError::internal())?;
    file.sync_all().map_err(|_| XmrError::internal())?;
    let metadata = file.metadata().map_err(|_| XmrError::internal())?;
    if !metadata.file_type().is_file()
        || metadata.uid() != owner
        || metadata.permissions().mode() & 0o777 != WALLET_FILE_MODE
        || metadata.dev() != identity.dev
        || metadata.ino() != identity.ino
    {
        return Err(XmrError::internal());
    }
    sync_directory_nofollow(vault_root, owner)
}

#[cfg(target_os = "linux")]
fn quarantine_identified(artifact: &ArtifactIdentity, owner: u32) -> Result<(), XmrError> {
    let opened = match OpenOptions::new()
        .read(true)
        .custom_flags(LINUX_O_NOFOLLOW)
        .open(&artifact.path)
    {
        Ok(file) => file,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(_) => return Err(XmrError::internal()),
    };
    let metadata = opened.metadata().map_err(|_| XmrError::internal())?;
    if !metadata.file_type().is_file()
        || metadata.uid() != owner
        || metadata.dev() != artifact.dev
        || metadata.ino() != artifact.ino
    {
        return Err(XmrError::internal());
    }
    let parent = artifact.path.parent().ok_or_else(XmrError::internal)?;
    let name = artifact
        .path
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(XmrError::internal)?;
    let mut random = [0u8; 8];
    getrandom::fill(&mut random).map_err(|_| XmrError::internal())?;
    let mut suffix = String::with_capacity(16);
    for byte in random {
        suffix.push_str(&format!("{byte:02x}"));
    }
    let destination = parent.join(format!("{name}.quarantine.{suffix}"));
    fs::hard_link(&artifact.path, &destination).map_err(|_| XmrError::internal())?;
    let dest = match OpenOptions::new()
        .read(true)
        .custom_flags(LINUX_O_NOFOLLOW)
        .open(&destination)
    {
        Ok(file) => file,
        Err(_) => return Err(XmrError::internal()),
    };
    let dest_meta = dest.metadata().map_err(|_| XmrError::internal())?;
    if !dest_meta.file_type().is_file() {
        return Err(XmrError::internal());
    }
    let dest_identity = (dest_meta.dev(), dest_meta.ino());
    if dest_identity != (artifact.dev, artifact.ino) {
        unlink_identified_path(&destination, dest_identity)?;
        sync_directory_nofollow(parent, owner)?;
        return Err(XmrError::internal());
    }
    match fs::symlink_metadata(&artifact.path) {
        Ok(listed) if listed.dev() == artifact.dev && listed.ino() == artifact.ino => {
            unlink_identified_path(&artifact.path, (artifact.dev, artifact.ino))?;
        }
        Ok(_) => return Err(XmrError::internal()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(_) => return Err(XmrError::internal()),
    }
    sync_directory_nofollow(parent, owner)?;
    Ok(())
}

#[cfg(target_os = "linux")]
fn unlink_identified_path(path: &Path, identity: (u64, u64)) -> Result<(), XmrError> {
    let listed = fs::symlink_metadata(path).map_err(|_| XmrError::internal())?;
    if listed.file_type().is_symlink()
        || !listed.file_type().is_file()
        || listed.dev() != identity.0
        || listed.ino() != identity.1
    {
        return Err(XmrError::internal());
    }
    fs::remove_file(path).map_err(|_| XmrError::internal())
}

#[cfg(target_os = "linux")]
fn sync_directory_nofollow(path: &Path, owner: u32) -> Result<(), XmrError> {
    let directory = OpenOptions::new()
        .read(true)
        .custom_flags(LINUX_O_NOFOLLOW | LINUX_O_DIRECTORY)
        .open(path)
        .map_err(|_| XmrError::internal())?;
    let metadata = directory.metadata().map_err(|_| XmrError::internal())?;
    if !metadata.file_type().is_dir()
        || metadata.uid() != owner
        || metadata.permissions().mode() & 0o777 != DIRECTORY_MODE
    {
        return Err(XmrError::internal());
    }
    directory.sync_all().map_err(|_| XmrError::internal())
}
