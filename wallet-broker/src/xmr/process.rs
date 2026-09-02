use std::collections::BTreeMap;
#[cfg(target_os = "linux")]
use std::fs::{self, OpenOptions};
#[cfg(target_os = "linux")]
use std::io::Write;
#[cfg(target_os = "linux")]
use std::net::{Ipv4Addr, TcpListener};
#[cfg(target_os = "linux")]
use std::os::unix::fs::{MetadataExt, OpenOptionsExt, PermissionsExt};
#[cfg(target_os = "linux")]
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
#[cfg(target_os = "linux")]
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicU16, Ordering};
#[cfg(target_os = "linux")]
use std::thread;
use std::time::Duration;
#[cfg(target_os = "linux")]
use std::time::Instant;

use zeroize::{Zeroize, Zeroizing};

#[cfg(target_os = "linux")]
use crate::xmr::distribution::VERIFIED_VERSION;
use crate::xmr::distribution::VerifiedExecutable;
#[cfg(target_os = "linux")]
use crate::xmr::model::HostPlatform;
use crate::xmr::model::{XmrError, XmrNetwork};

pub const PORT_MIN: u16 = 49_152;
pub const PORT_MAX: u16 = 65_535;
pub const MAX_PORT_ATTEMPTS: usize = 16;
pub const CONNECT_TIMEOUT_SECS: u64 = 2;
pub const READINESS_TIMEOUT_SECS: u64 = 10;
pub const STOP_TIMEOUT_SECS: u64 = 2;
pub const MAX_ACTIVE_CHILDREN: usize = 4;
pub const MAX_LOG_FILE_BYTES: u64 = 1_048_576;

const CONFIG_RELATIVE_PATH: &str = "wallet-rpc.conf";
const LOG_RELATIVE_PATH: &str = "wallet-rpc.log";
const RPC_BIND_IP: &str = "127.0.0.1";
const ARGUMENT: &str = "--config-file=wallet-rpc.conf";
const ARGUMENT_ZERO: &str = "monero-wallet-rpc";
const ENTROPY_BYTES: usize = 16;
const WALLET_PASSWORD_BYTES: usize = 32;
const DIRECTORY_MODE: u32 = 0o700;
const PRIVATE_FILE_MODE: u32 = 0o600;

static LAST_RANDOM_PORT: AtomicU16 = AtomicU16::new(0);

#[derive(Clone, Copy, Eq, PartialEq)]
pub(crate) enum EntropyOrigin {
    Os,
}

struct SecretText {
    value: String,
}

impl SecretText {
    fn from_random(bytes: &[u8]) -> Self {
        let mut secret = Self {
            value: String::with_capacity(bytes.len() * 2),
        };
        for byte in bytes {
            use core::fmt::Write as _;
            let _ = write!(secret.value, "{byte:02x}");
        }
        secret
    }

    fn expose(&self) -> &str {
        &self.value
    }

    fn wipe(&mut self) {
        self.value.zeroize();
    }
}

impl Drop for SecretText {
    fn drop(&mut self) {
        self.wipe();
    }
}

struct ConfigEntry {
    key: &'static str,
    value: String,
}

impl Drop for ConfigEntry {
    fn drop(&mut self) {
        self.value.zeroize();
    }
}

pub(crate) struct DerivedPaths {
    root: PathBuf,
    namespace: PathBuf,
    network: PathBuf,
    base: PathBuf,
    runtime: PathBuf,
    wallet: PathBuf,
    ring: PathBuf,
    config: PathBuf,
    log: PathBuf,
}

impl DerivedPaths {
    fn new(root: &Path, account_id: &str, network: XmrNetwork) -> Result<Self, XmrError> {
        let root_text = root.to_str().ok_or_else(XmrError::request_schema)?;
        if !root.is_absolute() || root_text.chars().any(char::is_control) {
            return Err(XmrError::request_schema());
        }
        let namespace = root.join("xmr");
        let network_path = namespace.join(network.name());
        let base = network_path.join(account_id);
        let runtime = base.join("runtime");
        Ok(Self {
            root: root.to_path_buf(),
            wallet: base.join("wallet"),
            ring: base.join("shared-ringdb"),
            config: runtime.join(CONFIG_RELATIVE_PATH),
            log: runtime.join(LOG_RELATIVE_PATH),
            namespace,
            network: network_path,
            base,
            runtime,
        })
    }
}

pub struct WalletRpcProcessPlan {
    executable: VerifiedExecutable,
    account_id: String,
    network: XmrNetwork,
    paths: DerivedPaths,
    rpc_port: u16,
    rpc_username: SecretText,
    rpc_password: SecretText,
    wallet_password: SecretText,
    config: Vec<ConfigEntry>,
    config_text: SecretText,
    port_from_entropy: bool,
    username_from_entropy: bool,
    password_from_entropy: bool,
}

impl WalletRpcProcessPlan {
    fn build(
        executable: VerifiedExecutable,
        account_id: &str,
        network: XmrNetwork,
        root: &Path,
        rpc_port: u16,
        rpc_username: SecretText,
        rpc_password: SecretText,
        wallet_password: SecretText,
        port_from_entropy: bool,
        username_from_entropy: bool,
        password_from_entropy: bool,
    ) -> Result<Self, XmrError> {
        let paths = DerivedPaths::new(root, account_id, network)?;
        let wallet = path_text(&paths.wallet)?;
        let ring = path_text(&paths.ring)?;
        let log = path_text(&paths.log)?;
        let mut login = SecretText {
            value: String::new(),
        };
        login.value.push_str(rpc_username.expose());
        login.value.push(':');
        login.value.push_str(rpc_password.expose());
        let mut config = vec![
            entry("rpc-bind-ip", RPC_BIND_IP),
            entry("rpc-bind-port", &rpc_port.to_string()),
            entry("rpc-login", login.expose()),
            entry("rpc-ssl", "disabled"),
            entry(
                "daemon-address",
                &format!("http://127.0.0.1:{}", network.daemon_port()),
            ),
            entry("daemon-ssl", "disabled"),
            entry("untrusted-daemon", "1"),
            entry("wallet-dir", wallet),
            entry("shared-ringdb-dir", ring),
            entry("no-dns", "1"),
            entry("non-interactive", "1"),
            entry("log-file", log),
            entry("log-level", "0"),
            entry("max-log-file-size", "1048576"),
            entry("max-log-files", "1"),
            entry("rpc-max-connections", "4"),
            entry("rpc-max-connections-per-private-ip", "4"),
            entry("rpc-max-connections-per-public-ip", "1"),
            entry("rpc-response-soft-limit", "65536"),
        ];
        config.push(entry(network.flag(), "1"));
        let mut config_text = SecretText {
            value: String::new(),
        };
        for item in &config {
            config_text.value.push_str(item.key);
            config_text.value.push('=');
            config_text.value.push_str(&item.value);
            config_text.value.push('\n');
        }
        Ok(Self {
            executable,
            account_id: account_id.to_owned(),
            network,
            paths,
            rpc_port,
            rpc_username,
            rpc_password,
            wallet_password,
            config,
            config_text,
            port_from_entropy,
            username_from_entropy,
            password_from_entropy,
        })
    }

    fn wipe(&mut self) {
        self.rpc_username.wipe();
        self.rpc_password.wipe();
        self.wallet_password.wipe();
        self.config_text.wipe();
        for item in &mut self.config {
            item.value.zeroize();
        }
    }

    pub(crate) fn config_keys(&self) -> Vec<&'static str> {
        self.config.iter().map(|entry| entry.key).collect()
    }

    pub(crate) fn config_value(&self, key: &str) -> Option<&str> {
        self.config
            .iter()
            .find(|entry| entry.key == key)
            .map(|entry| entry.value.as_str())
    }

    pub(crate) fn selected_program_path(&self) -> &Path {
        self.executable.selected_path()
    }

    pub(crate) fn program_is_verified_selection(&self) -> bool {
        self.selected_program_path() == self.executable.selected_path()
    }

    pub(crate) fn current_directory_is_private_runtime(&self) -> bool {
        self.paths.config.parent() == Some(self.paths.runtime.as_path())
    }

    pub(crate) fn root_path(&self) -> &Path {
        &self.paths.root
    }

    pub(crate) fn paths_use_only_derived_components(&self) -> bool {
        let base = self
            .paths
            .root
            .join("xmr")
            .join(self.network.name())
            .join(&self.account_id);
        self.paths.base == base
            && self.paths.runtime == base.join("runtime")
            && self.paths.wallet == base.join("wallet")
            && self.paths.ring == base.join("shared-ringdb")
            && self.paths.config == base.join("runtime").join(CONFIG_RELATIVE_PATH)
            && self.paths.log == base.join("runtime").join(LOG_RELATIVE_PATH)
    }

    pub(crate) fn account_id(&self) -> &str {
        &self.account_id
    }

    pub(crate) fn network(&self) -> XmrNetwork {
        self.network
    }

    pub(crate) fn rpc_port(&self) -> u16 {
        self.rpc_port
    }

    pub(crate) fn rpc_username(&self) -> &str {
        self.rpc_username.expose()
    }

    pub(crate) fn rpc_password(&self) -> &str {
        self.rpc_password.expose()
    }

    pub(crate) fn runtime_path(&self) -> &Path {
        &self.paths.runtime
    }

    pub(crate) fn wallet_path(&self) -> &Path {
        &self.paths.wallet
    }

    pub(crate) fn ring_path(&self) -> &Path {
        &self.paths.ring
    }

    pub(crate) fn log_path(&self) -> &Path {
        &self.paths.log
    }

    pub(crate) fn port_from_entropy(&self) -> bool {
        self.port_from_entropy
    }

    pub(crate) fn username_from_entropy(&self) -> bool {
        self.username_from_entropy
    }

    pub(crate) fn password_from_entropy(&self) -> bool {
        self.password_from_entropy
    }

    pub(crate) fn argv0(&self) -> &'static str {
        ARGUMENT_ZERO
    }

    pub(crate) fn argv(&self) -> [&'static str; 1] {
        [ARGUMENT]
    }

    pub(crate) fn environment(&self) -> [(&'static str, &'static str); 1] {
        [("LANG", "C")]
    }

    pub(crate) fn config_relative_path(&self) -> &'static str {
        CONFIG_RELATIVE_PATH
    }

    pub(crate) fn directory_mode(&self) -> u32 {
        DIRECTORY_MODE
    }

    pub(crate) fn private_file_mode(&self) -> u32 {
        PRIVATE_FILE_MODE
    }
}

impl Drop for WalletRpcProcessPlan {
    fn drop(&mut self) {
        self.wipe();
    }
}

fn entry(key: &'static str, value: &str) -> ConfigEntry {
    let mut entry = ConfigEntry {
        key,
        value: String::new(),
    };
    entry.value.push_str(value);
    entry
}

fn path_text(path: &Path) -> Result<&str, XmrError> {
    path.to_str().ok_or_else(XmrError::request_schema)
}

fn valid_account_id(value: &str) -> bool {
    value.len() == 32
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub(crate) enum ReservationFailure {
    Collision,
    Internal,
}

pub(crate) struct ReadinessObservation {
    pub(crate) elapsed_millis: u64,
    pub(crate) authenticated: bool,
    pub(crate) exact_version: bool,
    pub(crate) malformed: bool,
}

pub(crate) trait ProcessPort {
    type Reservation;
    type OwnedChild;

    fn note_operation(&mut self, operation: &'static str);
    fn fill_entropy(&mut self, output: &mut [u8]) -> Result<EntropyOrigin, XmrError>;
    fn next_port_candidate(&mut self) -> Result<(u32, bool), XmrError>;
    fn reserve_port(&mut self, port: u16) -> Result<Self::Reservation, ReservationFailure>;
    fn release_reservation(&mut self, reservation: Self::Reservation);
    fn create_private_layout(&mut self, paths: &DerivedPaths) -> Result<(), XmrError>;
    fn write_config(&mut self, plan: &WalletRpcProcessPlan) -> Result<(), XmrError>;
    fn sync_config(&mut self, plan: &WalletRpcProcessPlan) -> Result<(), XmrError>;
    fn spawn_verified(
        &mut self,
        plan: &WalletRpcProcessPlan,
        reservation: Self::Reservation,
    ) -> Result<Self::OwnedChild, XmrError>;
    fn readiness(
        &mut self,
        child: &mut Self::OwnedChild,
        plan: &WalletRpcProcessPlan,
    ) -> Result<ReadinessObservation, XmrError>;
    fn child_is_alive(&mut self, child: &mut Self::OwnedChild) -> Result<bool, XmrError>;
    fn selected_executable_is_current(
        &mut self,
        plan: &WalletRpcProcessPlan,
    ) -> Result<bool, XmrError>;
    fn close_wallet(&mut self, child: &mut Self::OwnedChild) -> Result<(), XmrError>;
    fn stop_wallet(&mut self, child: &mut Self::OwnedChild) -> Result<(), XmrError>;
    fn wait_owned_child(
        &mut self,
        child: &mut Self::OwnedChild,
        timeout_millis: u64,
    ) -> Result<bool, XmrError>;
    fn kill_owned_child(&mut self, child: &mut Self::OwnedChild) -> Result<(), XmrError>;
    fn reap_owned_child(&mut self, child: Self::OwnedChild) -> Result<(), XmrError>;
    fn close_sockets(&mut self);
    fn remove_runtime_secrets(&mut self, paths: &DerivedPaths) -> Result<(), XmrError>;
}

pub(crate) struct ProcessManager<P: ProcessPort> {
    account_id: String,
    network: Result<XmrNetwork, XmrError>,
    root: PathBuf,
    executable: Option<VerifiedExecutable>,
    port: P,
    plan: Option<WalletRpcProcessPlan>,
    reservation: Option<P::Reservation>,
    child: Option<P::OwnedChild>,
    credentials_wiped: bool,
    readiness_authenticated: bool,
    readiness_version_exact: bool,
    forced_kill: bool,
}

impl<P: ProcessPort> ProcessManager<P> {
    pub(crate) fn new(
        account_id: &str,
        network: Result<XmrNetwork, XmrError>,
        root: &Path,
        executable: VerifiedExecutable,
        port: P,
    ) -> Self {
        Self {
            account_id: account_id.to_owned(),
            network,
            root: root.to_path_buf(),
            executable: Some(executable),
            port,
            plan: None,
            reservation: None,
            child: None,
            credentials_wiped: false,
            readiness_authenticated: false,
            readiness_version_exact: false,
            forced_kill: false,
        }
    }

    pub(crate) fn port(&self) -> &P {
        &self.port
    }

    pub(crate) fn port_mut(&mut self) -> &mut P {
        &mut self.port
    }

    pub(crate) fn plan(&self) -> Option<&WalletRpcProcessPlan> {
        self.plan.as_ref()
    }

    pub(crate) fn child_count(&self) -> usize {
        usize::from(self.child.is_some())
    }

    pub(crate) fn credentials_wiped(&self) -> bool {
        self.credentials_wiped
    }

    pub(crate) fn readiness_authenticated(&self) -> bool {
        self.readiness_authenticated
    }

    pub(crate) fn readiness_version_exact(&self) -> bool {
        self.readiness_version_exact
    }

    pub(crate) fn used_forced_kill(&self) -> bool {
        self.forced_kill
    }

    pub(crate) fn prepare(&mut self) -> Result<(), XmrError> {
        if self.plan.is_some() {
            return Ok(());
        }
        if !valid_account_id(&self.account_id) {
            return Err(XmrError::request_schema());
        }
        let network = self.network.clone()?;
        let paths = DerivedPaths::new(&self.root, &self.account_id, network)?;
        self.port.create_private_layout(&paths)?;

        let (rpc_username, username_origin) = random_secret(&mut self.port, ENTROPY_BYTES)?;
        let (mut rpc_password, mut password_origin) = random_secret(&mut self.port, ENTROPY_BYTES)?;
        while rpc_password.expose() == rpc_username.expose() {
            (rpc_password, password_origin) = random_secret(&mut self.port, ENTROPY_BYTES)?;
        }
        let (wallet_password, _) = random_secret(&mut self.port, WALLET_PASSWORD_BYTES)?;
        let (rpc_port, reservation, port_from_entropy) = reserve_port(&mut self.port)?;
        let executable = self.executable.take().ok_or_else(XmrError::internal)?;
        let plan = WalletRpcProcessPlan::build(
            executable,
            &self.account_id,
            network,
            &self.root,
            rpc_port,
            rpc_username,
            rpc_password,
            wallet_password,
            port_from_entropy,
            username_origin == EntropyOrigin::Os,
            password_origin == EntropyOrigin::Os,
        )?;
        self.plan = Some(plan);
        self.reservation = Some(reservation);
        self.credentials_wiped = false;
        Ok(())
    }

    pub(crate) fn start(&mut self) -> Result<(), XmrError> {
        if self.credentials_wiped {
            return Err(XmrError::unavailable());
        }
        self.prepare()?;
        let result = self.start_prepared();
        if result.is_err() {
            if self.child.is_some() {
                let _ = self.teardown();
            } else {
                self.cleanup_failure();
            }
        }
        result
    }

    fn start_prepared(&mut self) -> Result<(), XmrError> {
        let plan = self.plan.as_ref().ok_or_else(XmrError::internal)?;
        self.port.write_config(plan)?;
        self.port.sync_config(plan)?;
        let reservation = self.reservation.take().ok_or_else(XmrError::internal)?;
        let child = self.port.spawn_verified(plan, reservation)?;
        self.child = Some(child);
        if !self
            .port
            .child_is_alive(self.child.as_mut().ok_or_else(XmrError::internal)?)?
        {
            return Err(XmrError::unavailable());
        }
        let observation = self
            .port
            .readiness(self.child.as_mut().ok_or_else(XmrError::internal)?, plan)?;
        if observation.elapsed_millis > READINESS_TIMEOUT_SECS * 1_000 {
            return Err(XmrError::unavailable());
        }
        if observation.malformed || !observation.authenticated || !observation.exact_version {
            return Err(XmrError::protocol_incompatible());
        }
        self.readiness_authenticated = true;
        self.readiness_version_exact = true;
        if !self
            .port
            .child_is_alive(self.child.as_mut().ok_or_else(XmrError::internal)?)?
        {
            return Err(XmrError::unavailable());
        }
        if !self.port.selected_executable_is_current(plan)? {
            return Err(XmrError::unavailable());
        }
        Ok(())
    }

    pub(crate) fn teardown(&mut self) -> Result<(), XmrError> {
        if self.child.is_none() {
            self.cleanup_failure();
            return Ok(());
        }
        self.forced_kill = false;
        self.port.note_operation("stop-new-calls");
        let child = self.child.as_mut().ok_or_else(XmrError::internal)?;
        self.port.note_operation("close-wallet");
        let _ = self.port.close_wallet(child);
        self.port.note_operation("stop-wallet");
        let _stop_result = self.port.stop_wallet(child);
        self.port.note_operation("wait-2s");
        let exited = self
            .port
            .wait_owned_child(child, STOP_TIMEOUT_SECS * 1_000)
            .unwrap_or(false);
        let mut lifecycle_error = None;
        if !exited {
            self.port.note_operation("kill-exact-owned-child");
            if let Err(error) = self.port.kill_owned_child(child) {
                lifecycle_error = Some(error);
            }
            self.forced_kill = true;
        }
        self.port.note_operation("reap");
        let child = self.child.take().ok_or_else(XmrError::internal)?;
        if let Err(error) = self.port.reap_owned_child(child) {
            lifecycle_error.get_or_insert(error);
        }
        if let Err(error) = self.finish_cleanup() {
            lifecycle_error.get_or_insert(error);
        }
        lifecycle_error.map_or(Ok(()), Err)
    }

    pub(crate) fn poll_health(&mut self) -> Result<(), XmrError> {
        let health = (|| {
            let child = self.child.as_mut().ok_or_else(XmrError::unavailable)?;
            if !self.port.child_is_alive(child)? {
                return Err(XmrError::unavailable());
            }
            let plan = self.plan.as_ref().ok_or_else(XmrError::internal)?;
            if !self.port.selected_executable_is_current(plan)? {
                return Err(XmrError::unavailable());
            }
            Ok(())
        })();
        if let Err(error) = health {
            let _ = self.teardown();
            return Err(error);
        }
        Ok(())
    }

    pub(crate) fn broker_exit(&mut self) -> Result<(), XmrError> {
        self.teardown()
    }

    fn cleanup_failure(&mut self) {
        if let Some(mut child) = self.child.take() {
            let _ = self.port.kill_owned_child(&mut child);
            let _ = self.port.reap_owned_child(child);
        }
        if let Some(reservation) = self.reservation.take() {
            self.port.release_reservation(reservation);
        }
        let _ = self.finish_cleanup();
    }

    fn finish_cleanup(&mut self) -> Result<(), XmrError> {
        self.port.note_operation("wipe-rpc-login");
        self.port.note_operation("wipe-wallet-password");
        let cleanup = if let Some(plan) = self.plan.as_mut() {
            plan.wipe();
            self.port.note_operation("close-sockets");
            self.port.close_sockets();
            self.port.note_operation("remove-runtime-secrets");
            self.port.remove_runtime_secrets(&plan.paths)
        } else {
            self.port.close_sockets();
            Ok(())
        };
        self.credentials_wiped = true;
        cleanup
    }
}

impl<P: ProcessPort> Drop for ProcessManager<P> {
    fn drop(&mut self) {
        if self.child.is_some() {
            let _ = self.teardown();
        } else if !self.credentials_wiped {
            self.cleanup_failure();
        }
    }
}

fn random_secret<P: ProcessPort>(
    port: &mut P,
    length: usize,
) -> Result<(SecretText, EntropyOrigin), XmrError> {
    let mut bytes = Zeroizing::new(vec![0u8; length]);
    let origin = port.fill_entropy(&mut bytes)?;
    let secret = SecretText::from_random(&bytes);
    if secret.expose().is_empty() {
        Err(XmrError::internal())
    } else {
        Ok((secret, origin))
    }
}

fn reserve_port<P: ProcessPort>(port: &mut P) -> Result<(u16, P::Reservation, bool), XmrError> {
    for _ in 0..MAX_PORT_ATTEMPTS {
        let (candidate, from_entropy) = port.next_port_candidate()?;
        if !(u32::from(PORT_MIN)..=u32::from(PORT_MAX)).contains(&candidate) {
            return Err(XmrError::internal());
        }
        let candidate = candidate as u16;
        match port.reserve_port(candidate) {
            Ok(reservation) => return Ok((candidate, reservation, from_entropy)),
            Err(ReservationFailure::Collision) => {}
            Err(ReservationFailure::Internal) => return Err(XmrError::internal()),
        }
    }
    Err(XmrError::unavailable())
}

pub(crate) struct ProcessCoordinator<P: ProcessPort> {
    active: BTreeMap<String, ProcessManager<P>>,
}

impl<P: ProcessPort> ProcessCoordinator<P> {
    pub(crate) fn new() -> Self {
        Self {
            active: BTreeMap::new(),
        }
    }

    pub(crate) fn start_account(
        &mut self,
        account_id: &str,
        mut manager: ProcessManager<P>,
    ) -> Result<(), XmrError> {
        if !valid_account_id(account_id) || manager.account_id != account_id {
            return Err(XmrError::request_schema());
        }
        if self.active.len() >= MAX_ACTIVE_CHILDREN || self.active.contains_key(account_id) {
            return Err(XmrError::limit());
        }
        manager.start()?;
        self.active.insert(account_id.to_owned(), manager);
        Ok(())
    }

    pub(crate) fn poll_health(&mut self, account_id: &str) -> Result<(), XmrError> {
        let result = self
            .active
            .get_mut(account_id)
            .ok_or_else(XmrError::unavailable)?
            .poll_health();
        if result.is_err() {
            self.active.remove(account_id);
        }
        result
    }

    pub(crate) fn stop_account(&mut self, account_id: &str) -> Result<(), XmrError> {
        let mut manager = self
            .active
            .remove(account_id)
            .ok_or_else(XmrError::unavailable)?;
        manager.teardown()
    }

    pub(crate) fn broker_exit_account(&mut self, account_id: &str) -> Result<(), XmrError> {
        let mut manager = self
            .active
            .remove(account_id)
            .ok_or_else(XmrError::unavailable)?;
        manager.broker_exit()
    }

    pub(crate) fn broker_exit_all(&mut self) -> Result<(), XmrError> {
        let active = core::mem::take(&mut self.active);
        let mut first_error = None;
        for (_, mut manager) in active {
            if let Err(error) = manager.broker_exit() {
                first_error.get_or_insert(error);
            }
        }
        first_error.map_or(Ok(()), Err)
    }

    pub(crate) fn len(&self) -> usize {
        self.active.len()
    }

    pub(crate) fn manager(&self, account_id: &str) -> Option<&ProcessManager<P>> {
        self.active.get(account_id)
    }

    pub(crate) fn manager_mut(&mut self, account_id: &str) -> Option<&mut ProcessManager<P>> {
        self.active.get_mut(account_id)
    }
}

fn os_port_candidate() -> Result<u32, XmrError> {
    loop {
        let mut random = Zeroizing::new([0u8; 2]);
        getrandom::fill(&mut *random).map_err(|_| XmrError::internal())?;
        let offset = u16::from_be_bytes(*random) & (PORT_MAX - PORT_MIN);
        let candidate = PORT_MIN + offset;
        let previous = LAST_RANDOM_PORT.swap(candidate, Ordering::SeqCst);
        if candidate != previous {
            return Ok(u32::from(candidate));
        }
    }
}

pub struct ReadinessStatus {
    pub authenticated: bool,
    pub version: String,
    pub elapsed_millis: u64,
}

pub trait WalletRpcControl {
    fn readiness(
        &mut self,
        rpc_port: u16,
        username: &str,
        password: &str,
        connect_timeout: Duration,
    ) -> Result<ReadinessStatus, XmrError>;
    fn close_wallet(&mut self) -> Result<(), XmrError>;
    fn stop_wallet(&mut self) -> Result<(), XmrError>;
    fn close_sockets(&mut self);
}

#[cfg(target_os = "linux")]
pub struct SystemProcessPort<C: WalletRpcControl> {
    control: C,
}

#[cfg(target_os = "linux")]
impl<C: WalletRpcControl> SystemProcessPort<C> {
    pub fn new(control: C) -> Self {
        Self { control }
    }
}

#[cfg(target_os = "linux")]
impl<C: WalletRpcControl> ProcessPort for SystemProcessPort<C> {
    type Reservation = TcpListener;
    type OwnedChild = Child;

    fn note_operation(&mut self, _operation: &'static str) {}

    fn fill_entropy(&mut self, output: &mut [u8]) -> Result<EntropyOrigin, XmrError> {
        getrandom::fill(output)
            .map(|()| EntropyOrigin::Os)
            .map_err(|_| XmrError::internal())
    }

    fn next_port_candidate(&mut self) -> Result<(u32, bool), XmrError> {
        os_port_candidate().map(|port| (port, true))
    }

    fn reserve_port(&mut self, port: u16) -> Result<Self::Reservation, ReservationFailure> {
        TcpListener::bind((Ipv4Addr::LOCALHOST, port)).map_err(|error| {
            if error.kind() == std::io::ErrorKind::AddrInUse {
                ReservationFailure::Collision
            } else {
                ReservationFailure::Internal
            }
        })
    }

    fn child_is_alive(&mut self, child: &mut Self::OwnedChild) -> Result<bool, XmrError> {
        child
            .try_wait()
            .map(|status| status.is_none())
            .map_err(|_| XmrError::internal())
    }

    fn selected_executable_is_current(
        &mut self,
        plan: &WalletRpcProcessPlan,
    ) -> Result<bool, XmrError> {
        let metadata = fs::symlink_metadata(plan.executable.selected_path())
            .map_err(|_| XmrError::unavailable())?;
        let expected = plan.executable.observation();
        if !metadata.file_type().is_file()
            || metadata.dev() != expected.device
            || metadata.ino() != expected.inode
            || metadata.len() != expected.length
            || metadata.mtime() != expected.modified_seconds
            || metadata.mtime_nsec() != i64::from(expected.modified_nanoseconds)
            || metadata.mode() != expected.mode
            || metadata.uid() != expected.owner
            || metadata.gid() != expected.group
        {
            return Ok(false);
        }
        Ok(true)
    }

    fn release_reservation(&mut self, reservation: Self::Reservation) {
        drop(reservation);
    }

    fn create_private_layout(&mut self, paths: &DerivedPaths) -> Result<(), XmrError> {
        ensure_private_root(&paths.base)?;
        for directory in [
            &paths.namespace,
            &paths.network,
            &paths.base,
            &paths.runtime,
            &paths.wallet,
            &paths.ring,
        ] {
            create_private_directory(directory)?;
        }
        Ok(())
    }

    fn write_config(&mut self, plan: &WalletRpcProcessPlan) -> Result<(), XmrError> {
        let mut config = OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(PRIVATE_FILE_MODE)
            .open(&plan.paths.config)
            .map_err(|_| XmrError::internal())?;
        config
            .set_permissions(fs::Permissions::from_mode(PRIVATE_FILE_MODE))
            .map_err(|_| XmrError::internal())?;
        config
            .write_all(plan.config_text.expose().as_bytes())
            .map_err(|_| XmrError::internal())?;
        let log = OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(PRIVATE_FILE_MODE)
            .open(&plan.paths.log)
            .map_err(|_| XmrError::internal())?;
        log.set_permissions(fs::Permissions::from_mode(PRIVATE_FILE_MODE))
            .map_err(|_| XmrError::internal())
    }

    fn sync_config(&mut self, plan: &WalletRpcProcessPlan) -> Result<(), XmrError> {
        OpenOptions::new()
            .write(true)
            .open(&plan.paths.config)
            .and_then(|file| file.sync_all())
            .map_err(|_| XmrError::internal())
    }

    fn spawn_verified(
        &mut self,
        plan: &WalletRpcProcessPlan,
        reservation: Self::Reservation,
    ) -> Result<Self::OwnedChild, XmrError> {
        let mut command = Command::new(plan.executable.selected_path());
        command
            .arg0(ARGUMENT_ZERO)
            .arg(ARGUMENT)
            .env_clear()
            .env("LANG", "C")
            .current_dir(&plan.paths.runtime)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null());
        drop(reservation);
        command.spawn().map_err(|_| XmrError::unavailable())
    }

    fn readiness(
        &mut self,
        _child: &mut Self::OwnedChild,
        plan: &WalletRpcProcessPlan,
    ) -> Result<ReadinessObservation, XmrError> {
        let status = self.control.readiness(
            plan.rpc_port,
            plan.rpc_username.expose(),
            plan.rpc_password.expose(),
            Duration::from_secs(CONNECT_TIMEOUT_SECS),
        )?;
        Ok(ReadinessObservation {
            elapsed_millis: status.elapsed_millis,
            authenticated: status.authenticated,
            exact_version: status.version == VERIFIED_VERSION,
            malformed: false,
        })
    }

    fn close_wallet(&mut self, _child: &mut Self::OwnedChild) -> Result<(), XmrError> {
        self.control.close_wallet()
    }

    fn stop_wallet(&mut self, _child: &mut Self::OwnedChild) -> Result<(), XmrError> {
        self.control.stop_wallet()
    }

    fn wait_owned_child(
        &mut self,
        child: &mut Self::OwnedChild,
        timeout_millis: u64,
    ) -> Result<bool, XmrError> {
        let started = Instant::now();
        let timeout = Duration::from_millis(timeout_millis);
        loop {
            if child
                .try_wait()
                .map_err(|_| XmrError::internal())?
                .is_some()
            {
                return Ok(true);
            }
            if started.elapsed() >= timeout {
                return Ok(false);
            }
            thread::sleep(Duration::from_millis(10));
        }
    }

    fn kill_owned_child(&mut self, child: &mut Self::OwnedChild) -> Result<(), XmrError> {
        child.kill().map_err(|_| XmrError::internal())
    }

    fn reap_owned_child(&mut self, mut child: Self::OwnedChild) -> Result<(), XmrError> {
        child.wait().map(|_| ()).map_err(|_| XmrError::internal())
    }

    fn close_sockets(&mut self) {
        self.control.close_sockets();
    }

    fn remove_runtime_secrets(&mut self, paths: &DerivedPaths) -> Result<(), XmrError> {
        remove_if_file(&paths.config)?;
        remove_if_file(&paths.log)
    }
}

#[cfg(target_os = "linux")]
fn ensure_private_root(derived_base: &Path) -> Result<(), XmrError> {
    let root = derived_base
        .ancestors()
        .nth(3)
        .ok_or_else(XmrError::request_schema)?;
    let metadata = fs::symlink_metadata(root).map_err(|_| XmrError::state_corrupt())?;
    if !metadata.file_type().is_dir() || metadata.permissions().mode() & 0o777 != DIRECTORY_MODE {
        return Err(XmrError::state_corrupt());
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn create_private_directory(path: &Path) -> Result<(), XmrError> {
    match fs::create_dir(path) {
        Ok(()) => {}
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
        Err(_) => return Err(XmrError::internal()),
    }
    let metadata = fs::symlink_metadata(path).map_err(|_| XmrError::internal())?;
    if !metadata.file_type().is_dir() {
        return Err(XmrError::state_corrupt());
    }
    fs::set_permissions(path, fs::Permissions::from_mode(DIRECTORY_MODE))
        .map_err(|_| XmrError::internal())?;
    let metadata = fs::symlink_metadata(path).map_err(|_| XmrError::internal())?;
    if metadata.file_type().is_dir() && metadata.permissions().mode() & 0o777 == DIRECTORY_MODE {
        Ok(())
    } else {
        Err(XmrError::state_corrupt())
    }
}

#[cfg(target_os = "linux")]
fn remove_if_file(path: &Path) -> Result<(), XmrError> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_file() => {
            fs::remove_file(path).map_err(|_| XmrError::internal())
        }
        Ok(_) => Err(XmrError::state_corrupt()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(_) => Err(XmrError::internal()),
    }
}

#[cfg(target_os = "linux")]
pub struct WalletRpcProcessPool<C: WalletRpcControl> {
    coordinator: ProcessCoordinator<SystemProcessPort<C>>,
}

#[cfg(target_os = "linux")]
impl<C: WalletRpcControl> WalletRpcProcessPool<C> {
    pub fn new() -> Result<Self, XmrError> {
        if !HostPlatform::current().supports_distribution() {
            return Err(XmrError::unavailable());
        }
        Ok(Self {
            coordinator: ProcessCoordinator::new(),
        })
    }

    pub fn start_account(
        &mut self,
        account_id: &str,
        network: XmrNetwork,
        executable: VerifiedExecutable,
        private_root: &Path,
        control: C,
    ) -> Result<(), XmrError> {
        let manager = ProcessManager::new(
            account_id,
            Ok(network),
            private_root,
            executable,
            SystemProcessPort::new(control),
        );
        self.coordinator.start_account(account_id, manager)
    }

    pub fn poll_health(&mut self, account_id: &str) -> Result<(), XmrError> {
        self.coordinator.poll_health(account_id)
    }

    pub fn stop_account(&mut self, account_id: &str) -> Result<(), XmrError> {
        self.coordinator.stop_account(account_id)
    }

    pub fn broker_exit_account(&mut self, account_id: &str) -> Result<(), XmrError> {
        self.coordinator.broker_exit_account(account_id)
    }

    pub fn broker_exit(&mut self) -> Result<(), XmrError> {
        self.coordinator.broker_exit_all()
    }

    pub fn child_count(&self) -> usize {
        self.coordinator.len()
    }
}

#[cfg(not(target_os = "linux"))]
pub struct WalletRpcProcessPool<C: WalletRpcControl> {
    _marker: core::marker::PhantomData<C>,
}

#[cfg(not(target_os = "linux"))]
impl<C: WalletRpcControl> WalletRpcProcessPool<C> {
    pub fn new() -> Result<Self, XmrError> {
        Err(XmrError::unavailable())
    }

    pub fn start_account(
        &mut self,
        account_id: &str,
        network: XmrNetwork,
        executable: VerifiedExecutable,
        private_root: &Path,
        control: C,
    ) -> Result<(), XmrError> {
        let _ = (account_id, network, executable, private_root, control);
        Err(XmrError::unavailable())
    }

    pub fn poll_health(&mut self, account_id: &str) -> Result<(), XmrError> {
        let _ = account_id;
        Err(XmrError::unavailable())
    }

    pub fn stop_account(&mut self, account_id: &str) -> Result<(), XmrError> {
        let _ = account_id;
        Err(XmrError::unavailable())
    }

    pub fn broker_exit_account(&mut self, account_id: &str) -> Result<(), XmrError> {
        let _ = account_id;
        Err(XmrError::unavailable())
    }

    pub fn broker_exit(&mut self) -> Result<(), XmrError> {
        Err(XmrError::unavailable())
    }

    pub fn child_count(&self) -> usize {
        0
    }
}

pub(crate) fn next_os_port_for_test_port() -> Result<u32, XmrError> {
    os_port_candidate()
}
