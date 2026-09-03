use std::collections::{BTreeMap, VecDeque};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use sha2::{Digest, Sha256};
use zeroize::{Zeroize, Zeroizing};

use crate::vault::{
    Asset, Network as VaultNetwork, OsEntropy, SecretBytes, VaultError, VaultMetadata,
    VaultWorkObserver, WipeEvent, WipeObserver, open_vault_bytes, parse_vault, seal_vault,
    valid_account_id,
};
use crate::xmr::account::{
    AccountManager, AccountPathsView, AccountPort, AccountRpcCall, CreatedAccount,
    WalletPasswordObservation, WalletPresence, WalletRpcObservation, XmrSecretV1,
    generate_wallet_password, is_network_valid_address, is_spendable_mnemonic,
    validate_password_hex_length, validate_primary_address_length, validate_total_length,
    validate_view_key_hex_length,
};
use crate::xmr::distribution::{
    DistributionManager, DistributionPort, EXECUTABLE_BYTES, EXECUTABLE_SHA256,
    ExecutableObservation, HashResult, RecordIntegrity, SelectedFileKind, SelectionRecord,
    VERIFIED_VERSION, VerificationStep, decode_digest,
};
use crate::xmr::model::XmrError;
use crate::xmr::process::{
    DerivedPaths, EntropyOrigin, ProcessCoordinator, ProcessManager, ProcessPort,
    ReadinessObservation, ReservationFailure, WalletRpcProcessPlan, next_os_port_for_test_port,
};
use crate::xmr::rpc::{
    DigestResponseInput, HttpExchangePort, NodeProbeResult, PortFailure, RpcCore, RpcRequest,
    TypedResult, WipeAudit, digest_response_for_test, node_port, probe_node_with,
    request_body_boundary_for_test, request_dispatch_for_test, validate_json_for_test,
};
use crate::xmr::store::{AccountStore, ColumnInfo, SqliteSurface, StoreSurface, StoredIdentity};

pub use crate::xmr::account::{AccountKind, HostileWalletEntry, SecretExit};
pub use crate::xmr::model::{HostPlatform, XmrNetwork};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InstallationKind {
    Normal,
    Portable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SelectedEntry {
    Regular,
    Symlink,
    Directory,
    Fifo,
    Socket,
    BlockDevice,
    CharacterDevice,
}

impl SelectedEntry {
    fn file_kind(self) -> SelectedFileKind {
        match self {
            Self::Regular => SelectedFileKind::Regular,
            Self::Symlink => SelectedFileKind::Symlink,
            Self::Directory => SelectedFileKind::Directory,
            Self::Fifo => SelectedFileKind::Fifo,
            Self::Socket => SelectedFileKind::Socket,
            Self::BlockDevice => SelectedFileKind::BlockDevice,
            Self::CharacterDevice => SelectedFileKind::CharacterDevice,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DistributionFault {
    PartialRecord,
    UnknownRecordField,
    UnknownSchema,
    WrongRecordMode,
    SymlinkedRecord,
    RecordSync,
    DirectorySync,
    MissingExecutable,
}

struct RecordingDistributionPort {
    available_path: String,
    observation: ExecutableObservation,
    executable: bool,
    digest: [u8; 32],
    version: String,
    faults: Vec<DistributionFault>,
    operations: Vec<&'static str>,
    hashed_bytes: u64,
    version_probed: bool,
    followed_final_component: bool,
    attempted_path_search: bool,
    attempted_parent_scan: bool,
    attempted_download: bool,
    attempted_alternate_path: bool,
    persisted_record: Option<SelectionRecord>,
}

impl RecordingDistributionPort {
    fn reviewed(path: &str) -> Self {
        Self {
            available_path: path.to_owned(),
            observation: ExecutableObservation {
                kind: SelectedFileKind::Regular,
                device: 7,
                inode: 11,
                length: EXECUTABLE_BYTES,
                modified_seconds: 1_725_000_000,
                modified_nanoseconds: 123_456_789,
                mode: 0o100700,
                owner: 1_000,
                group: 1_000,
            },
            executable: true,
            digest: decode_digest(EXECUTABLE_SHA256).expect("reviewed digest is valid"),
            version: VERIFIED_VERSION.to_owned(),
            faults: Vec::new(),
            operations: Vec::new(),
            hashed_bytes: 0,
            version_probed: false,
            followed_final_component: false,
            attempted_path_search: false,
            attempted_parent_scan: false,
            attempted_download: false,
            attempted_alternate_path: false,
            persisted_record: None,
        }
    }

    fn has_fault(&self, fault: DistributionFault) -> bool {
        self.faults.contains(&fault)
    }
}

impl DistributionPort for RecordingDistributionPort {
    fn note_step(&mut self, step: VerificationStep) {
        self.operations.push(step.label());
    }

    fn lstat(&mut self, selected_path: &Path) -> Result<ExecutableObservation, XmrError> {
        self.followed_final_component = false;
        if self.has_fault(DistributionFault::MissingExecutable)
            || selected_path.to_str() != Some(self.available_path.as_str())
        {
            return Err(XmrError::unavailable());
        }
        Ok(self.observation)
    }

    fn effective_user_can_execute(
        &mut self,
        _observation: &ExecutableObservation,
    ) -> Result<bool, XmrError> {
        Ok(self.executable)
    }

    fn hash_executable(
        &mut self,
        _selected_path: &Path,
        observation: &ExecutableObservation,
    ) -> Result<HashResult, XmrError> {
        self.hashed_bytes = observation.length;
        Ok(HashResult {
            digest: self.digest,
            bytes_read: self.hashed_bytes,
        })
    }

    fn probe_version(&mut self, _selected_path: &Path) -> Result<String, XmrError> {
        self.version_probed = true;
        Ok(self.version.clone())
    }

    fn persist_selection(&mut self, selected_path: &str) -> Result<SelectionRecord, XmrError> {
        if self.has_fault(DistributionFault::RecordSync)
            || self.has_fault(DistributionFault::DirectorySync)
        {
            return Err(XmrError::internal());
        }
        let mut record = SelectionRecord::complete(selected_path);
        if self.has_fault(DistributionFault::PartialRecord) {
            record.set_integrity(RecordIntegrity::Partial);
        } else if self.has_fault(DistributionFault::UnknownRecordField) {
            record.set_integrity(RecordIntegrity::UnknownField);
        } else if self.has_fault(DistributionFault::UnknownSchema) {
            record.schema_version = 2;
        } else if self.has_fault(DistributionFault::WrongRecordMode) {
            record.mode = 0o644;
        } else if self.has_fault(DistributionFault::SymlinkedRecord) {
            record.set_integrity(RecordIntegrity::Symlink);
        }
        self.persisted_record = Some(record.clone());
        Ok(record)
    }

    fn load_selection(&mut self) -> Result<SelectionRecord, XmrError> {
        self.persisted_record
            .clone()
            .ok_or_else(XmrError::unavailable)
    }
}

pub struct DistributionRig {
    manager: DistributionManager<RecordingDistributionPort>,
    child_count: usize,
    last_error: Option<XmrError>,
    logs: Vec<&'static str>,
    diagnostics: Vec<&'static str>,
}

impl DistributionRig {
    pub fn new(platform: HostPlatform) -> Self {
        Self {
            manager: DistributionManager::new(
                platform,
                RecordingDistributionPort::reviewed("/synthetic/monero/extras/monero-wallet-rpc"),
            ),
            child_count: 0,
            last_error: None,
            logs: Vec::new(),
            diagnostics: Vec::new(),
        }
    }

    pub fn reviewed_linux(_kind: InstallationKind, selected_path: &str) -> Self {
        Self {
            manager: DistributionManager::new(
                HostPlatform::LinuxX86_64,
                RecordingDistributionPort::reviewed(selected_path),
            ),
            child_count: 0,
            last_error: None,
            logs: Vec::new(),
            diagnostics: Vec::new(),
        }
    }

    pub fn with_entry(entry: SelectedEntry, selected_path: &str) -> Self {
        let mut rig = Self::reviewed_linux(InstallationKind::Normal, selected_path);
        rig.manager.port_mut().observation.kind = entry.file_kind();
        rig
    }

    pub fn enroll(&mut self, selected_path: &str) -> Result<(), XmrError> {
        let result = self.manager.enroll(selected_path);
        self.remember(result)
    }

    pub fn enroll_non_utf8(&mut self, bytes: &[u8]) -> Result<(), XmrError> {
        #[cfg(unix)]
        {
            use std::os::unix::ffi::OsStrExt;

            let result = self.manager.enroll_os(OsStr::from_bytes(bytes));
            self.remember(result)
        }
        #[cfg(not(unix))]
        {
            let _ = bytes;
            let error = XmrError::schema();
            self.last_error = Some(error.clone());
            Err(error)
        }
    }

    pub fn launch(&mut self) -> Result<(), XmrError> {
        let result = self.manager.authorize_launch().map(|verified| {
            let _ = verified.selected_path();
            self.child_count = 1;
        });
        self.remember(result)
    }

    pub fn poll_selected_file(&mut self) -> Result<(), XmrError> {
        let result = self.manager.poll_selected_file();
        if self.manager.stop_required() {
            self.child_count = 0;
        }
        self.remember(result)
    }

    pub fn operations(&self) -> &[&'static str] {
        &self.manager.port().operations
    }

    pub fn clear_operations(&mut self) {
        self.manager.port_mut().operations.clear();
    }

    pub fn selection_record(&self) -> Option<&SelectionRecord> {
        self.manager.selection()
    }

    pub fn child_count(&self) -> usize {
        self.child_count
    }

    pub fn set_observed_length(&mut self, length: u64) {
        self.manager.port_mut().observation.length = length;
    }

    pub fn set_effective_user_executable(&mut self, executable: bool) {
        self.manager.port_mut().executable = executable;
    }

    pub fn set_executable_sha256(&mut self, digest: &str) {
        self.manager.port_mut().digest = decode_digest(digest).unwrap_or([0u8; 32]);
    }

    pub fn set_version_output(&mut self, version: &str) {
        self.manager.port_mut().version = version.to_owned();
    }

    pub fn arm_fault(&mut self, fault: DistributionFault) {
        if !self.manager.port().faults.contains(&fault) {
            self.manager.port_mut().faults.push(fault);
        }
    }

    pub fn mutate_selected_file(&mut self, mutation: &str) {
        let port = self.manager.port_mut();
        match mutation {
            "missing" => port.faults.push(DistributionFault::MissingExecutable),
            "identity" => port.observation.inode = port.observation.inode.saturating_add(1),
            "size" => port.observation.length = port.observation.length.saturating_add(1),
            "mtime" => {
                port.observation.modified_nanoseconds =
                    port.observation.modified_nanoseconds.saturating_add(1);
            }
            _ => panic!("unreviewed selected-file mutation"),
        }
    }

    pub fn followed_final_component(&self) -> bool {
        self.manager.port().followed_final_component
    }

    pub fn version_was_probed(&self) -> bool {
        self.manager.port().version_probed
    }

    pub fn hashed_bytes(&self) -> u64 {
        self.manager.port().hashed_bytes
    }

    pub fn reported_success(&self) -> bool {
        self.manager.selection().is_some()
    }

    pub fn last_teardown_reason(&self) -> Option<&'static str> {
        self.manager.teardown_reason()
    }

    pub fn attempted_path_search(&self) -> bool {
        self.manager.port().attempted_path_search
    }

    pub fn attempted_parent_scan(&self) -> bool {
        self.manager.port().attempted_parent_scan
    }

    pub fn attempted_download(&self) -> bool {
        self.manager.port().attempted_download
    }

    pub fn attempted_alternate_path(&self) -> bool {
        self.manager.port().attempted_alternate_path
    }

    pub fn last_error(&self) -> Option<&XmrError> {
        self.last_error.as_ref()
    }

    pub fn logs(&self) -> &[&'static str] {
        &self.logs
    }

    pub fn diagnostics(&self) -> &[&'static str] {
        &self.diagnostics
    }

    fn remember(&mut self, result: Result<(), XmrError>) -> Result<(), XmrError> {
        match result {
            Ok(()) => {
                self.last_error = None;
                Ok(())
            }
            Err(error) => {
                self.last_error = Some(error.clone());
                Err(error)
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChildExit {
    Hung,
    StopRpcError,
    Unexpected,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProcessFault {
    ConfigWrite,
    ConfigSync,
    Spawn,
    Authentication,
    WrongVersion,
    MalformedReadiness,
    ExecutableRemoved,
    ExecutableChanged,
    BrokerExit,
    UnexpectedExit,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TeardownCause {
    Lock,
    Failure,
}

struct ReservedPort {
    port: u16,
}

struct OwnedChild {
    identity: u64,
}

struct RecordingProcessPort {
    scripted_ports: VecDeque<u32>,
    collisions_remaining: usize,
    port_attempts: usize,
    readiness_delay_millis: u64,
    stop_delay_millis: u64,
    child_exit: Option<ChildExit>,
    faults: Vec<ProcessFault>,
    operations: Vec<&'static str>,
    teardown_operations: Vec<&'static str>,
    open_handles: usize,
    runtime_secrets_removed: bool,
    next_child_identity: u64,
    owned_child_identity: Option<u64>,
    killed_child_identity: Option<u64>,
    entropy_calls: usize,
    port_entropy_calls: usize,
    spawn_count: usize,
    reservation_live_at_spawn: bool,
    reservation_released_immediately_before_spawn: bool,
    config_written_while_reserved: bool,
    config_synced_while_reserved: bool,
    liveness_checks: Vec<&'static str>,
    executable_checks: usize,
}

impl RecordingProcessPort {
    fn new() -> Self {
        Self {
            scripted_ports: VecDeque::new(),
            collisions_remaining: 0,
            port_attempts: 0,
            readiness_delay_millis: 0,
            stop_delay_millis: 0,
            child_exit: None,
            faults: Vec::new(),
            operations: Vec::new(),
            teardown_operations: Vec::new(),
            open_handles: 0,
            runtime_secrets_removed: false,
            next_child_identity: 1,
            owned_child_identity: None,
            killed_child_identity: None,
            entropy_calls: 0,
            port_entropy_calls: 0,
            spawn_count: 0,
            reservation_live_at_spawn: false,
            reservation_released_immediately_before_spawn: false,
            config_written_while_reserved: false,
            config_synced_while_reserved: false,
            liveness_checks: Vec::new(),
            executable_checks: 0,
        }
    }

    fn has_fault(&self, fault: ProcessFault) -> bool {
        self.faults.contains(&fault)
    }
}

impl ProcessPort for RecordingProcessPort {
    type Reservation = ReservedPort;
    type OwnedChild = OwnedChild;

    fn note_operation(&mut self, operation: &'static str) {
        self.operations.push(operation);
        self.teardown_operations.push(operation);
    }

    fn fill_entropy(&mut self, output: &mut [u8]) -> Result<EntropyOrigin, XmrError> {
        self.entropy_calls += 1;
        getrandom::fill(output)
            .map(|()| EntropyOrigin::Os)
            .map_err(|_| XmrError::internal())
    }

    fn next_port_candidate(&mut self) -> Result<(u32, bool), XmrError> {
        let scripted = self.scripted_ports.pop_front();
        match scripted {
            Some(port) => Ok((port, false)),
            None => {
                self.port_entropy_calls += 1;
                next_os_port_for_test_port().map(|port| (port, true))
            }
        }
    }

    fn reserve_port(&mut self, port: u16) -> Result<Self::Reservation, ReservationFailure> {
        self.port_attempts += 1;
        if self.collisions_remaining > 0 {
            self.collisions_remaining -= 1;
            return Err(ReservationFailure::Collision);
        }
        self.open_handles += 1;
        Ok(ReservedPort { port })
    }

    fn release_reservation(&mut self, reservation: Self::Reservation) {
        let _ = reservation.port;
        self.open_handles = self.open_handles.saturating_sub(1);
    }

    fn create_private_layout(&mut self, _paths: &DerivedPaths) -> Result<(), XmrError> {
        self.runtime_secrets_removed = false;
        Ok(())
    }

    fn write_config(&mut self, _plan: &WalletRpcProcessPlan) -> Result<(), XmrError> {
        self.config_written_while_reserved = self.open_handles == 1;
        if self.has_fault(ProcessFault::ConfigWrite) {
            Err(XmrError::internal())
        } else {
            Ok(())
        }
    }

    fn sync_config(&mut self, _plan: &WalletRpcProcessPlan) -> Result<(), XmrError> {
        self.config_synced_while_reserved = self.open_handles == 1;
        if self.has_fault(ProcessFault::ConfigSync) {
            Err(XmrError::internal())
        } else {
            Ok(())
        }
    }

    fn spawn_verified(
        &mut self,
        plan: &WalletRpcProcessPlan,
        reservation: Self::Reservation,
    ) -> Result<Self::OwnedChild, XmrError> {
        if plan.selected_program_path().to_str().is_none() {
            return Err(XmrError::request_schema());
        }
        self.reservation_live_at_spawn = self.open_handles == 1;
        let _ = reservation.port;
        self.open_handles = self.open_handles.saturating_sub(1);
        self.reservation_released_immediately_before_spawn = self.open_handles == 0;
        self.spawn_count += 1;
        if self.has_fault(ProcessFault::Spawn) {
            return Err(XmrError::unavailable());
        }
        let identity = self.next_child_identity;
        self.next_child_identity += 1;
        self.owned_child_identity = Some(identity);
        self.open_handles += 1;
        Ok(OwnedChild { identity })
    }

    fn readiness(
        &mut self,
        _child: &mut Self::OwnedChild,
        _plan: &WalletRpcProcessPlan,
    ) -> Result<ReadinessObservation, XmrError> {
        Ok(ReadinessObservation {
            elapsed_millis: self.readiness_delay_millis,
            authenticated: !self.has_fault(ProcessFault::Authentication),
            exact_version: !self.has_fault(ProcessFault::WrongVersion),
            malformed: self.has_fault(ProcessFault::MalformedReadiness),
        })
    }

    fn child_is_alive(&mut self, _child: &mut Self::OwnedChild) -> Result<bool, XmrError> {
        let phase = if self.liveness_checks.is_empty() {
            "before-readiness"
        } else if self.liveness_checks.len() == 1 {
            "after-readiness"
        } else {
            "poll"
        };
        self.liveness_checks.push(phase);
        Ok(!(phase == "poll" && self.has_fault(ProcessFault::UnexpectedExit)))
    }

    fn selected_executable_is_current(
        &mut self,
        _plan: &WalletRpcProcessPlan,
    ) -> Result<bool, XmrError> {
        self.executable_checks += 1;
        if self.executable_checks > 1
            && (self.has_fault(ProcessFault::ExecutableRemoved)
                || self.has_fault(ProcessFault::ExecutableChanged))
        {
            Ok(false)
        } else {
            Ok(true)
        }
    }

    fn close_wallet(&mut self, _child: &mut Self::OwnedChild) -> Result<(), XmrError> {
        Ok(())
    }

    fn stop_wallet(&mut self, _child: &mut Self::OwnedChild) -> Result<(), XmrError> {
        if self.child_exit == Some(ChildExit::StopRpcError) {
            Err(XmrError::unavailable())
        } else {
            Ok(())
        }
    }

    fn wait_owned_child(
        &mut self,
        _child: &mut Self::OwnedChild,
        timeout_millis: u64,
    ) -> Result<bool, XmrError> {
        let abnormal = matches!(
            self.child_exit,
            Some(ChildExit::Hung | ChildExit::StopRpcError | ChildExit::Unexpected)
        );
        Ok(!abnormal && self.stop_delay_millis <= timeout_millis)
    }

    fn kill_owned_child(&mut self, child: &mut Self::OwnedChild) -> Result<(), XmrError> {
        self.killed_child_identity = Some(child.identity);
        Ok(())
    }

    fn reap_owned_child(&mut self, child: Self::OwnedChild) -> Result<(), XmrError> {
        if self.owned_child_identity != Some(child.identity) {
            return Err(XmrError::internal());
        }
        self.open_handles = self.open_handles.saturating_sub(1);
        Ok(())
    }

    fn close_sockets(&mut self) {}

    fn remove_runtime_secrets(&mut self, _paths: &DerivedPaths) -> Result<(), XmrError> {
        self.runtime_secrets_removed = true;
        Ok(())
    }
}

pub struct ProcessPlanView {
    pub argv0: &'static str,
    pub argv: [&'static str; 1],
    pub environment: [(&'static str, &'static str); 1],
    pub program_is_verified_selection: bool,
    pub current_directory_is_private_runtime: bool,
    pub config_relative_path: &'static str,
    pub runtime_directory_mode: u32,
    pub wallet_directory_mode: u32,
    pub ring_directory_mode: u32,
    pub config_mode: u32,
    pub log_mode: u32,
    account_id: String,
    network: XmrNetwork,
    root: String,
    selected_program: String,
    runtime: String,
    wallet: String,
    ring: String,
    log: String,
    rpc_port: u16,
    rpc_username: String,
    rpc_password: String,
    config: Vec<(&'static str, String)>,
    port_from_entropy: bool,
    username_from_entropy: bool,
    password_from_entropy: bool,
    paths_use_only_derived_components: bool,
}

impl ProcessPlanView {
    fn from_plan(plan: &WalletRpcProcessPlan) -> Self {
        let config = plan
            .config_keys()
            .into_iter()
            .map(|key| {
                (
                    key,
                    plan.config_value(key)
                        .expect("production config key remains addressable")
                        .to_owned(),
                )
            })
            .collect();
        Self {
            argv0: plan.argv0(),
            argv: plan.argv(),
            environment: plan.environment(),
            program_is_verified_selection: plan.program_is_verified_selection(),
            current_directory_is_private_runtime: plan.current_directory_is_private_runtime(),
            config_relative_path: plan.config_relative_path(),
            runtime_directory_mode: plan.directory_mode(),
            wallet_directory_mode: plan.directory_mode(),
            ring_directory_mode: plan.directory_mode(),
            config_mode: plan.private_file_mode(),
            log_mode: plan.private_file_mode(),
            account_id: plan.account_id().to_owned(),
            network: plan.network(),
            root: plan.root_path().to_string_lossy().into_owned(),
            selected_program: plan.selected_program_path().to_string_lossy().into_owned(),
            runtime: plan.runtime_path().to_string_lossy().into_owned(),
            wallet: plan.wallet_path().to_string_lossy().into_owned(),
            ring: plan.ring_path().to_string_lossy().into_owned(),
            log: plan.log_path().to_string_lossy().into_owned(),
            rpc_port: plan.rpc_port(),
            rpc_username: plan.rpc_username().to_owned(),
            rpc_password: plan.rpc_password().to_owned(),
            config,
            port_from_entropy: plan.port_from_entropy(),
            username_from_entropy: plan.username_from_entropy(),
            password_from_entropy: plan.password_from_entropy(),
            paths_use_only_derived_components: plan.paths_use_only_derived_components(),
        }
    }

    pub fn config_keys(&self) -> Vec<&'static str> {
        self.config.iter().map(|entry| entry.0).collect()
    }

    pub fn config_value(&self, key: &str) -> Option<&str> {
        self.config
            .iter()
            .find(|entry| entry.0 == key)
            .map(|entry| entry.1.as_str())
    }

    pub fn rpc_port(&self) -> u16 {
        self.rpc_port
    }

    pub fn rpc_login_username(&self) -> &str {
        &self.rpc_username
    }

    pub fn rpc_login_password(&self) -> &str {
        &self.rpc_password
    }

    pub fn rpc_bind_ip(&self) -> &str {
        self.config_value("rpc-bind-ip").unwrap_or_default()
    }

    pub fn derived_wallet_directory(&self) -> &str {
        &self.wallet
    }

    pub fn derived_ring_directory(&self) -> &str {
        &self.ring
    }

    pub fn derived_log_file(&self) -> &str {
        &self.log
    }

    pub fn selected_program_path(&self) -> &str {
        &self.selected_program
    }

    pub fn rpc_port_from_os_entropy(&self) -> bool {
        self.port_from_entropy
    }

    pub fn rpc_username_from_os_entropy(&self) -> bool {
        self.username_from_entropy
    }

    pub fn rpc_password_from_os_entropy(&self) -> bool {
        self.password_from_entropy
    }

    pub fn private_paths_are_derived_from(&self, account: &str, network: &str) -> bool {
        let base = Path::new(self.root.as_str())
            .join("xmr")
            .join(network)
            .join(account);
        self.account_id == account
            && self.network.name() == network
            && Path::new(&self.runtime) == base.join("runtime")
            && Path::new(&self.wallet) == base.join("wallet")
            && Path::new(&self.ring) == base.join("shared-ringdb")
            && Path::new(&self.log) == base.join("runtime").join("wallet-rpc.log")
    }

    pub fn private_paths_accept_caller_input(&self) -> bool {
        !self.paths_use_only_derived_components
    }

    pub fn non_network_config(&self) -> Vec<(&'static str, &str)> {
        self.config
            .iter()
            .filter(|entry| {
                !matches!(
                    entry.0,
                    "rpc-bind-port"
                        | "rpc-login"
                        | "daemon-address"
                        | "wallet-dir"
                        | "shared-ringdb-dir"
                        | "log-file"
                        | "stagenet"
                        | "testnet"
                )
            })
            .map(|entry| (entry.0, entry.1.as_str()))
            .collect()
    }

    pub fn argv_and_config_text_for_test(&self) -> String {
        let mut text = format!("{} {}\n", self.argv0, self.argv.join(" "));
        for (key, value) in &self.config {
            text.push_str(key);
            text.push('=');
            text.push_str(value);
            text.push('\n');
        }
        text
    }

    pub fn option_names_for_test(&self) -> Vec<&str> {
        let mut names: Vec<&str> = self
            .argv
            .iter()
            .map(|argument| {
                argument
                    .trim_start_matches("--")
                    .split_once('=')
                    .map_or(*argument, |(name, _)| name)
            })
            .collect();
        names.extend(self.config.iter().map(|entry| entry.0));
        names
    }

    pub fn option_values_for_test(&self) -> Vec<&str> {
        self.config.iter().map(|entry| entry.1.as_str()).collect()
    }

    pub fn argv_and_environment_text_for_test(&self) -> String {
        format!("{} {} LANG=C", self.argv0, self.argv.join(" "))
    }
}

impl Drop for ProcessPlanView {
    fn drop(&mut self) {
        use zeroize::Zeroize;

        self.rpc_username.zeroize();
        self.rpc_password.zeroize();
        for entry in &mut self.config {
            entry.1.zeroize();
        }
    }
}

fn reviewed_executable() -> crate::xmr::distribution::VerifiedExecutable {
    let path = "/synthetic/monero/extras/monero-wallet-rpc";
    let mut manager = DistributionManager::new(
        HostPlatform::LinuxX86_64,
        RecordingDistributionPort::reviewed(path),
    );
    manager.enroll(path).expect("reviewed selection enrolls");
    manager
        .authorize_launch()
        .expect("reviewed selection verifies")
}

fn process_manager(
    account: &str,
    network: Result<XmrNetwork, XmrError>,
    root: &Path,
    port: RecordingProcessPort,
) -> ProcessManager<RecordingProcessPort> {
    ProcessManager::new(account, network, root, reviewed_executable(), port)
}

pub struct ProcessRig {
    account_id: String,
    network: Result<XmrNetwork, XmrError>,
    root: PathBuf,
    manager: Option<ProcessManager<RecordingProcessPort>>,
    coordinator: Option<ProcessCoordinator<RecordingProcessPort>>,
    zec_alive: bool,
    social_alive: bool,
    non_xmr_stop_calls: usize,
}

impl ProcessRig {
    pub fn reviewed(account: &str, network: XmrNetwork) -> Self {
        let root = PathBuf::from("/synthetic/private");
        let mut manager = process_manager(account, Ok(network), &root, RecordingProcessPort::new());
        manager.prepare().expect("reviewed process plan prepares");
        Self {
            account_id: account.to_owned(),
            network: Ok(network),
            root,
            manager: Some(manager),
            coordinator: None,
            zec_alive: false,
            social_alive: false,
            non_xmr_stop_calls: 0,
        }
    }

    pub fn new_unvalidated(account: &str, network: &str) -> Self {
        Self::new_with_private_root(account, network, Path::new("/synthetic/private"))
    }

    pub fn new_with_private_root(account: &str, network: &str, root: &Path) -> Self {
        let parsed = XmrNetwork::parse(network);
        Self {
            account_id: account.to_owned(),
            network: parsed.clone(),
            root: root.to_path_buf(),
            manager: Some(process_manager(
                account,
                parsed,
                root,
                RecordingProcessPort::new(),
            )),
            coordinator: None,
            zec_alive: false,
            social_alive: false,
            non_xmr_stop_calls: 0,
        }
    }

    pub fn pool() -> Self {
        Self {
            account_id: String::new(),
            network: Ok(XmrNetwork::Stagenet),
            root: PathBuf::from("/synthetic/private"),
            manager: None,
            coordinator: Some(ProcessCoordinator::new()),
            zec_alive: false,
            social_alive: false,
            non_xmr_stop_calls: 0,
        }
    }

    pub fn with_isolation_observer() -> Self {
        let mut rig = Self::pool();
        rig.zec_alive = true;
        rig.social_alive = true;
        rig
    }

    fn rebuild(&mut self, port: RecordingProcessPort) {
        self.manager = Some(process_manager(
            &self.account_id,
            self.network.clone(),
            &self.root,
            port,
        ));
    }

    fn manager(&self) -> &ProcessManager<RecordingProcessPort> {
        self.manager.as_ref().expect("single process rig")
    }

    fn manager_mut(&mut self) -> &mut ProcessManager<RecordingProcessPort> {
        self.manager.as_mut().expect("single process rig")
    }

    pub fn plan(&self) -> Option<ProcessPlanView> {
        self.manager().plan().map(ProcessPlanView::from_plan)
    }

    pub fn script_ports(&mut self, ports: &[u32]) {
        let mut port = RecordingProcessPort::new();
        port.scripted_ports = ports.iter().copied().collect();
        self.rebuild(port);
    }

    pub fn script_collisions(&mut self, collisions: usize) {
        let mut port = RecordingProcessPort::new();
        port.collisions_remaining = collisions;
        self.rebuild(port);
    }

    pub fn reserve_port(&mut self) -> Result<(), XmrError> {
        self.manager_mut().prepare()
    }

    pub fn port_attempts(&self) -> usize {
        self.manager().port().port_attempts
    }

    pub fn entropy_calls(&self) -> usize {
        self.manager().port().entropy_calls
    }

    pub fn port_entropy_calls(&self) -> usize {
        self.manager().port().port_entropy_calls
    }

    pub fn reservation_was_live_at_spawn(&self) -> bool {
        self.manager().port().reservation_live_at_spawn
    }

    pub fn reservation_was_released_immediately_before_spawn(&self) -> bool {
        self.manager()
            .port()
            .reservation_released_immediately_before_spawn
    }

    pub fn config_was_written_and_synced_while_reserved(&self) -> bool {
        let port = self.manager().port();
        port.config_written_while_reserved && port.config_synced_while_reserved
    }

    pub fn liveness_checks(&self) -> &[&'static str] {
        &self.manager().port().liveness_checks
    }

    pub fn spawn_count(&self) -> usize {
        self.manager().port().spawn_count
    }

    pub fn set_readiness_delay_millis(&mut self, millis: u64) {
        self.manager_mut().port_mut().readiness_delay_millis = millis;
    }

    pub fn start(&mut self) -> Result<(), XmrError> {
        self.manager_mut().start()
    }

    pub fn poll_health(&mut self) -> Result<(), XmrError> {
        self.manager_mut().poll_health()
    }

    pub fn authenticated_readiness_observed(&self) -> bool {
        self.manager().readiness_authenticated()
    }

    pub fn version_was_exact(&self) -> bool {
        self.manager().readiness_version_exact()
    }

    pub fn child_count(&self) -> usize {
        self.manager.as_ref().map_or_else(
            || self.coordinator.as_ref().expect("process pool").len(),
            ProcessManager::child_count,
        )
    }

    pub fn start_account(&mut self, account: &str, network: XmrNetwork) -> Result<(), XmrError> {
        let manager = process_manager(
            account,
            Ok(network),
            &self.root,
            RecordingProcessPort::new(),
        );
        self.coordinator
            .as_mut()
            .expect("process pool")
            .start_account(account, manager)
    }

    pub fn poll_account_health(&mut self, account: &str) -> Result<(), XmrError> {
        self.coordinator
            .as_mut()
            .expect("process pool")
            .poll_health(account)
    }

    pub fn account_spawn_count(&self, account: &str) -> usize {
        self.coordinator
            .as_ref()
            .expect("process pool")
            .manager(account)
            .expect("active account")
            .port()
            .spawn_count
    }

    pub fn operations(&self) -> &[&'static str] {
        &self.manager().port().operations
    }

    pub fn teardown(&mut self, _cause: TeardownCause) -> Result<(), XmrError> {
        self.manager_mut().teardown()
    }

    pub fn teardown_operations(&self) -> &[&'static str] {
        &self.manager().port().teardown_operations
    }

    pub fn open_handle_count(&self) -> usize {
        self.manager().port().open_handles
    }

    pub fn runtime_secrets_removed(&self) -> bool {
        self.manager().port().runtime_secrets_removed
    }

    pub fn set_stop_delay_millis(&mut self, millis: u64) {
        self.manager_mut().port_mut().stop_delay_millis = millis;
    }

    pub fn used_forced_kill(&self) -> bool {
        self.manager().used_forced_kill()
    }

    pub fn set_child_exit(&mut self, exit: ChildExit) {
        self.manager_mut().port_mut().child_exit = Some(exit);
    }

    pub fn killed_only_owned_child(&self) -> bool {
        let port = self.manager().port();
        port.killed_child_identity.is_some()
            && port.killed_child_identity == port.owned_child_identity
    }

    pub fn arm_fault(&mut self, fault: ProcessFault) {
        if !self.manager().port().faults.contains(&fault) {
            self.manager_mut().port_mut().faults.push(fault);
        }
    }

    pub fn broker_exit_for_test(&mut self) -> Result<(), XmrError> {
        self.manager_mut().broker_exit()
    }

    pub fn credentials_wiped(&self) -> bool {
        self.manager().credentials_wiped()
    }

    pub fn fail_account(&mut self, account: &str, fault: ProcessFault) -> Result<(), XmrError> {
        let coordinator = self.coordinator.as_mut().expect("process pool");
        let manager = coordinator
            .manager_mut(account)
            .ok_or_else(XmrError::unavailable)?;
        manager.port_mut().faults.push(fault);
        manager.port_mut().child_exit = Some(ChildExit::Unexpected);
        coordinator.broker_exit_account(account)
    }

    pub fn zec_alive(&self) -> bool {
        self.zec_alive
    }

    pub fn social_alive(&self) -> bool {
        self.social_alive
    }

    pub fn non_xmr_stop_calls(&self) -> usize {
        self.non_xmr_stop_calls
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RpcMethod {
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

impl RpcMethod {
    fn request(self) -> Result<RpcRequest, XmrError> {
        match self {
            Self::GetVersion => Ok(RpcRequest::GetVersion),
            Self::CloseWallet => Ok(RpcRequest::CloseWallet),
            Self::StopWallet => Ok(RpcRequest::StopWallet),
            Self::GetHeight => Ok(RpcRequest::GetHeight),
            Self::GetBalance => Ok(RpcRequest::GetBalance),
            Self::CreateAddress => Ok(RpcRequest::CreateAddress),
            Self::GetInfo => Ok(RpcRequest::GetInfo),
            Self::HardForkInfo => Ok(RpcRequest::HardForkInfo),
            Self::CreateWallet
            | Self::RestoreDeterministicWallet
            | Self::GenerateFromKeys
            | Self::OpenWallet
            | Self::QueryKey
            | Self::Refresh
            | Self::GetAddress
            | Self::ValidateAddress => Err(XmrError::request_schema()),
        }
    }

    fn production(self) -> crate::xmr::rpc::RpcMethod {
        match self {
            Self::GetVersion => crate::xmr::rpc::RpcMethod::GetVersion,
            Self::CreateWallet => crate::xmr::rpc::RpcMethod::CreateWallet,
            Self::RestoreDeterministicWallet => {
                crate::xmr::rpc::RpcMethod::RestoreDeterministicWallet
            }
            Self::GenerateFromKeys => crate::xmr::rpc::RpcMethod::GenerateFromKeys,
            Self::OpenWallet => crate::xmr::rpc::RpcMethod::OpenWallet,
            Self::CloseWallet => crate::xmr::rpc::RpcMethod::CloseWallet,
            Self::StopWallet => crate::xmr::rpc::RpcMethod::StopWallet,
            Self::QueryKey => crate::xmr::rpc::RpcMethod::QueryKey,
            Self::Refresh => crate::xmr::rpc::RpcMethod::Refresh,
            Self::GetHeight => crate::xmr::rpc::RpcMethod::GetHeight,
            Self::GetBalance => crate::xmr::rpc::RpcMethod::GetBalance,
            Self::GetAddress => crate::xmr::rpc::RpcMethod::GetAddress,
            Self::CreateAddress => crate::xmr::rpc::RpcMethod::CreateAddress,
            Self::ValidateAddress => crate::xmr::rpc::RpcMethod::ValidateAddress,
            Self::GetInfo => crate::xmr::rpc::RpcMethod::GetInfo,
            Self::HardForkInfo => crate::xmr::rpc::RpcMethod::HardForkInfo,
        }
    }

    pub fn wallet_allowlist() -> [&'static str; 14] {
        [
            "get_version",
            "create_wallet",
            "restore_deterministic_wallet",
            "generate_from_keys",
            "open_wallet",
            "close_wallet",
            "stop_wallet",
            "query_key",
            "refresh",
            "get_height",
            "get_balance",
            "get_address",
            "create_address",
            "validate_address",
        ]
    }

    pub fn node_allowlist() -> [&'static str; 2] {
        ["get_info", "hard_fork_info"]
    }
}

pub struct DigestVector<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub realm: &'a str,
    pub nonce: &'a str,
    pub uri: &'a str,
    pub method: &'a str,
    pub qop: &'a str,
    pub nc: &'a str,
    pub cnonce: &'a str,
    pub algorithm: Option<&'a str>,
    pub opaque: Option<&'a str>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HttpFault {
    MissingContentLength,
    DuplicateContentLength,
    ConflictingContentLength,
    TransferEncodingChunked,
    TransferEncodingIdentity,
    MissingConnectionClose,
    FoldedHeader,
    ControlByte,
    TrailingBytes,
    Redirect301,
    Redirect307,
    StatusUnknown,
    Http10,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum JsonFault {
    InvalidUtf8,
    Bom,
    TrailingBytes,
    DuplicateKey,
    WrongVersion,
    WrongId,
    MissingId,
    TypeConfusion,
    IntegerOverflow,
    UnsupportedShape,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RpcFault {
    SecondChallenge,
    ConnectDelay,
    ReadDelay,
    WriteDelay,
    NodeAuthenticationRequired,
    Redirect,
    MalformedResponse,
    OversizedResponse,
    InconsistentHeights,
    ConnectionRefused,
    ConnectOtherFailure,
    AuthenticatedConnectionRefused,
    NodeVersionNumeric,
    NodeVersionTypeConfusion,
    NodeVersionOverlong,
    BlockWeightLimitTypeConfusion,
    BlockWeightMedianTypeConfusion,
    MissingRequiredNodeMember,
    ExtraNodeMember,
    UpstreamError,
    PanicUnwind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthorizationObservation {
    pub scheme: &'static str,
    pub method: &'static str,
    pub uri: &'static str,
    pub qop: &'static str,
    pub nc: String,
    pub cnonce: String,
    pub cnonce_source_bytes: usize,
    pub algorithm: String,
    pub opaque: Option<String>,
}

impl Drop for AuthorizationObservation {
    fn drop(&mut self) {
        self.nc.zeroize();
        self.cnonce.zeroize();
        self.algorithm.zeroize();
        if let Some(opaque) = &mut self.opaque {
            opaque.zeroize();
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RequestObservation {
    pub method: String,
    pub params: String,
    pub authorization: Option<AuthorizationObservation>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RpcResultObservation {
    pub balance: Option<u64>,
    pub unlocked_balance: Option<u64>,
    pub address_index: Option<u32>,
    pub address_count: Option<usize>,
    pub raw_string_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NodeInfo {
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

#[derive(Clone, Copy)]
struct HardForkInfoFixture {
    earliest_height: u64,
    enabled: bool,
    untrusted: bool,
    version: u64,
    voting: u64,
    window: u64,
    votes: u64,
    threshold: u64,
    state: u64,
}

impl NodeInfo {
    fn reviewed(network: XmrNetwork) -> Self {
        let (nettype, stagenet, testnet) = match network {
            XmrNetwork::Stagenet => ("stagenet", true, false),
            XmrNetwork::Testnet => ("testnet", false, true),
        };
        Self {
            status: "OK".to_owned(),
            nettype: nettype.to_owned(),
            mainnet: false,
            stagenet,
            testnet,
            offline: false,
            untrusted: false,
            bootstrap_daemon_address: String::new(),
            was_bootstrap_ever_used: false,
            synchronized: true,
            height: 1_000,
            target_height: 1_000,
            height_without_bootstrap: 1_000,
        }
    }

    pub fn network_boolean_matrix(network: XmrNetwork) -> Vec<Self> {
        let mut matrix = Vec::new();
        for bits in 0u8..8 {
            let mut info = Self::reviewed(network);
            info.mainnet = bits & 1 != 0;
            info.stagenet = bits & 2 != 0;
            info.testnet = bits & 4 != 0;
            matrix.push(info);
        }
        let mut wrong_nettype = Self::reviewed(network);
        wrong_nettype.nettype = match network {
            XmrNetwork::Stagenet => "testnet",
            XmrNetwork::Testnet => "stagenet",
        }
        .to_owned();
        matrix.push(wrong_nettype);
        matrix
    }

    pub fn is_exact_for(&self, network: XmrNetwork) -> bool {
        match network {
            XmrNetwork::Stagenet => {
                self.nettype == "stagenet" && !self.mainnet && self.stagenet && !self.testnet
            }
            XmrNetwork::Testnet => {
                self.nettype == "testnet" && !self.mainnet && !self.stagenet && self.testnet
            }
        }
    }
}

struct RecordingRpcPort {
    wallet: bool,
    challenge: Zeroizing<String>,
    requests: Vec<RequestObservation>,
    request_count: usize,
    last_call_request_count: usize,
    bytes_written: usize,
    open_connections: usize,
    followed_redirect: bool,
    redirect_pending: bool,
    entropy_counter: u8,
    last_entropy_bytes: usize,
    http_fault: Option<HttpFault>,
    json_fault: Option<JsonFault>,
    rpc_fault: Option<RpcFault>,
    timed_fault: Option<(RpcFault, u64)>,
    response_total_bytes: Option<usize>,
    info: NodeInfo,
    hard_fork: HardForkInfoFixture,
    attempted_ports: Vec<u16>,
    operations: Vec<&'static str>,
    upstream_canary: Zeroizing<String>,
    retained_response: Zeroizing<Vec<u8>>,
    result_members: BTreeMap<String, Vec<String>>,
    nested_result_members: BTreeMap<String, Vec<String>>,
    startup_refusals: usize,
    startup_attempt_millis: u64,
    readiness_active: bool,
    readiness_elapsed_millis: u64,
    readiness_attempts: usize,
    readiness_operation_timeouts_millis: Vec<u64>,
    numeric_loopback_connections: usize,
    dns_resolutions: usize,
    proxy_connections: usize,
    last_failure_not_listening: bool,
    readiness_exchange_succeeded: bool,
    include_block_weight_limit: bool,
    include_block_weight_median: bool,
}

impl RecordingRpcPort {
    fn wallet() -> Self {
        Self::new(true, XmrNetwork::Stagenet)
    }

    fn node(network: XmrNetwork) -> Self {
        Self::new(false, network)
    }

    fn new(wallet: bool, network: XmrNetwork) -> Self {
        Self {
            wallet,
            challenge: Zeroizing::new(
                r#"Digest realm="monero-rpc", nonce="synthetic-nonce", qop="auth", algorithm=MD5"#
                    .to_owned(),
            ),
            requests: Vec::new(),
            request_count: 0,
            last_call_request_count: 0,
            bytes_written: 0,
            open_connections: 0,
            followed_redirect: false,
            redirect_pending: false,
            entropy_counter: 0,
            last_entropy_bytes: 0,
            http_fault: None,
            json_fault: None,
            rpc_fault: None,
            timed_fault: None,
            response_total_bytes: None,
            info: NodeInfo::reviewed(network),
            hard_fork: HardForkInfoFixture {
                earliest_height: 1_000,
                enabled: true,
                untrusted: false,
                version: 16,
                voting: 16,
                window: 10_080,
                votes: 0,
                threshold: 0,
                state: 1,
            },
            attempted_ports: Vec::new(),
            operations: Vec::new(),
            upstream_canary: Zeroizing::new(String::new()),
            retained_response: Zeroizing::new(Vec::new()),
            result_members: BTreeMap::new(),
            nested_result_members: BTreeMap::new(),
            startup_refusals: 0,
            startup_attempt_millis: 0,
            readiness_active: false,
            readiness_elapsed_millis: 0,
            readiness_attempts: 0,
            readiness_operation_timeouts_millis: Vec::new(),
            numeric_loopback_connections: 0,
            dns_resolutions: 0,
            proxy_connections: 0,
            last_failure_not_listening: false,
            readiness_exchange_succeeded: false,
            include_block_weight_limit: true,
            include_block_weight_median: true,
        }
    }

    fn response(&mut self, request: &[u8]) -> Result<Vec<u8>, PortFailure> {
        let observation = observe_request(request, self.last_entropy_bytes);
        let authenticated = observation.authorization.is_some();
        let method = observation.method.clone();
        self.operations.push(match method.as_str() {
            "get_info" => "get_info",
            "hard_fork_info" => "hard_fork_info",
            _ => "wallet-rpc",
        });
        self.requests.push(observation);
        if self.rpc_fault == Some(RpcFault::MalformedResponse) {
            return Ok(b"malformed".to_vec());
        }
        if self.rpc_fault == Some(RpcFault::OversizedResponse) {
            return Ok(vec![b'x'; crate::xmr::rpc::MAX_HTTP_BYTES + 1]);
        }
        if self.wallet && !authenticated {
            return Ok(http_response(
                "HTTP/1.1 401 Unauthorized",
                b"",
                Some(&self.challenge),
                self.http_fault,
                self.response_total_bytes,
            ));
        }
        if self.wallet && authenticated && self.rpc_fault == Some(RpcFault::SecondChallenge) {
            return Ok(http_response(
                "HTTP/1.1 401 Unauthorized",
                b"",
                Some(&self.challenge),
                self.http_fault,
                self.response_total_bytes,
            ));
        }
        if !self.wallet && self.rpc_fault == Some(RpcFault::NodeAuthenticationRequired) {
            return Ok(http_response(
                "HTTP/1.1 401 Unauthorized",
                b"",
                Some(&self.challenge),
                self.http_fault,
                self.response_total_bytes,
            ));
        }
        let mut body = Zeroizing::new(if self.rpc_fault == Some(RpcFault::UpstreamError) {
            format!(
                r#"{{"jsonrpc":"2.0","id":"bitbook-xmr-v1","error":{{"code":-1,"message":"{}"}}}}"#,
                *self.upstream_canary
            )
            .into_bytes()
        } else {
            valid_result_body(
                &method,
                &self.info,
                self.hard_fork,
                self.include_block_weight_limit,
                self.include_block_weight_median,
                self.rpc_fault,
            )
        });
        apply_json_fault(&mut body, self.json_fault);
        self.record_result_shape(&method, &body);
        let status = if matches!(self.rpc_fault, Some(RpcFault::Redirect)) {
            "HTTP/1.1 307 Temporary Redirect"
        } else {
            "HTTP/1.1 200 OK"
        };
        Ok(http_response(
            status,
            &body,
            None,
            self.http_fault,
            self.response_total_bytes,
        ))
    }

    fn record_result_shape(&mut self, method: &str, body: &[u8]) {
        let Ok(mut document) = serde_json::from_slice::<serde_json::Value>(body) else {
            return;
        };
        let observed = (|| {
            let object = document.get("result")?.as_object()?;
            let mut members: Vec<String> = object.keys().cloned().collect();
            members.sort();
            let nested = object
                .get("per_subaddress")
                .and_then(serde_json::Value::as_array)
                .and_then(|values| values.first())
                .and_then(serde_json::Value::as_object)
                .map(|nested| {
                    let mut members: Vec<String> = nested.keys().cloned().collect();
                    members.sort();
                    members
                });
            Some((members, nested))
        })();
        zeroize_observed_json(&mut document);
        let Some((members, nested)) = observed else {
            return;
        };
        self.result_members.insert(method.to_owned(), members);
        if let Some(members) = nested {
            self.nested_result_members
                .insert(method.to_owned(), members);
        }
    }
}

fn zeroize_observed_json(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::String(text) => text.zeroize(),
        serde_json::Value::Array(values) => {
            for value in values {
                zeroize_observed_json(value);
            }
        }
        serde_json::Value::Object(object) => {
            let entries = core::mem::take(object);
            for (mut key, mut value) in entries {
                key.zeroize();
                zeroize_observed_json(&mut value);
            }
        }
        serde_json::Value::Null | serde_json::Value::Bool(_) | serde_json::Value::Number(_) => {}
    }
}

impl Drop for RecordingRpcPort {
    fn drop(&mut self) {
        self.challenge.zeroize();
        self.upstream_canary.zeroize();
        self.retained_response.zeroize();
        self.retained_response.clear();
    }
}

impl HttpExchangePort for RecordingRpcPort {
    fn exchange(
        &mut self,
        port: u16,
        request: &[u8],
        connect_timeout: Duration,
        read_timeout: Duration,
        write_timeout: Duration,
    ) -> Result<Vec<u8>, PortFailure> {
        self.last_failure_not_listening = false;
        let authenticated = request
            .windows(b"Authorization:".len())
            .any(|window| window == b"Authorization:");
        if self.redirect_pending {
            self.followed_redirect = true;
            self.redirect_pending = false;
        }
        self.last_call_request_count += 1;
        self.request_count += 1;
        self.attempted_ports.push(port);
        self.numeric_loopback_connections = self.numeric_loopback_connections.saturating_add(1);
        if self.readiness_active {
            if !authenticated {
                self.readiness_attempts = self.readiness_attempts.saturating_add(1);
            }
            self.readiness_operation_timeouts_millis.extend([
                u64::try_from(connect_timeout.as_millis()).unwrap_or(u64::MAX),
                u64::try_from(read_timeout.as_millis()).unwrap_or(u64::MAX),
                u64::try_from(write_timeout.as_millis()).unwrap_or(u64::MAX),
            ]);
            if self.startup_refusals > 0 {
                if self.startup_refusals != usize::MAX {
                    self.startup_refusals -= 1;
                }
                let elapsed = self
                    .startup_attempt_millis
                    .min(u64::try_from(connect_timeout.as_millis()).unwrap_or(u64::MAX));
                self.readiness_elapsed_millis = self
                    .readiness_elapsed_millis
                    .saturating_add(elapsed)
                    .min(crate::xmr::rpc::READINESS_TIMEOUT_SECS * 1_000);
                self.open_connections = 0;
                self.last_failure_not_listening = !self.readiness_exchange_succeeded;
                return Err(PortFailure::Unavailable);
            }
        }
        self.open_connections += 1;
        if self.rpc_fault == Some(RpcFault::ConnectionRefused) {
            self.open_connections = 0;
            self.last_failure_not_listening = !self.readiness_exchange_succeeded;
            return Err(PortFailure::Unavailable);
        }
        if self.rpc_fault == Some(RpcFault::ConnectOtherFailure) {
            self.open_connections = 0;
            return Err(PortFailure::Unavailable);
        }
        if let Some((fault, millis)) = self.timed_fault {
            let limit = match fault {
                RpcFault::ConnectDelay => connect_timeout,
                RpcFault::ReadDelay => read_timeout,
                RpcFault::WriteDelay => write_timeout,
                _ => Duration::ZERO,
            };
            if Duration::from_millis(millis) > limit {
                self.open_connections = 0;
                return Err(PortFailure::Unavailable);
            }
        }
        if authenticated && self.rpc_fault == Some(RpcFault::AuthenticatedConnectionRefused) {
            self.open_connections = 0;
            return Err(PortFailure::Unavailable);
        }
        self.bytes_written = self.bytes_written.saturating_add(request.len());
        if self.rpc_fault == Some(RpcFault::PanicUnwind) {
            self.open_connections = 0;
            panic!("sanitized synthetic RPC unwind");
        }
        let response = self.response(request)?;
        self.redirect_pending =
            response.starts_with(b"HTTP/1.1 301 ") || response.starts_with(b"HTTP/1.1 307 ");
        self.retained_response.zeroize();
        self.retained_response.clear();
        self.retained_response.extend_from_slice(&response);
        self.open_connections = 0;
        if self.readiness_active {
            self.readiness_exchange_succeeded = true;
        }
        Ok(response)
    }

    fn fill_entropy(&mut self, output: &mut [u8]) -> Result<(), PortFailure> {
        self.last_entropy_bytes = output.len();
        self.entropy_counter = self.entropy_counter.wrapping_add(1);
        for (index, byte) in output.iter_mut().enumerate() {
            *byte = self.entropy_counter.wrapping_add(index as u8);
        }
        Ok(())
    }

    fn begin_readiness(&mut self) {
        self.readiness_active = true;
        self.readiness_elapsed_millis = 0;
        self.readiness_attempts = 0;
        self.readiness_operation_timeouts_millis.clear();
        self.readiness_exchange_succeeded = false;
    }

    fn readiness_elapsed(&self) -> Duration {
        if self.readiness_active {
            Duration::from_millis(self.readiness_elapsed_millis)
        } else {
            Duration::ZERO
        }
    }

    fn readiness_failure_retryable(&self) -> bool {
        self.last_failure_not_listening
    }

    fn wait_readiness_retry(&mut self, duration: Duration) {
        self.readiness_elapsed_millis = self
            .readiness_elapsed_millis
            .saturating_add(u64::try_from(duration.as_millis()).unwrap_or(u64::MAX))
            .min(crate::xmr::rpc::READINESS_TIMEOUT_SECS * 1_000);
    }

    fn end_readiness(&mut self) {
        self.readiness_active = false;
    }

    fn response_consumed(&mut self) {
        self.retained_response.zeroize();
        self.retained_response.clear();
    }

    fn close_all(&mut self) {
        self.open_connections = 0;
    }
}

fn observe_request(request: &[u8], cnonce_source_bytes: usize) -> RequestObservation {
    let split = request
        .windows(4)
        .position(|window| window == b"\r\n\r\n")
        .map(|position| position + 4)
        .unwrap_or(request.len());
    let header = String::from_utf8_lossy(&request[..split]);
    let authorization = header
        .lines()
        .find_map(|line| line.strip_prefix("Authorization: "))
        .and_then(|value| observe_authorization(value, cnonce_source_bytes));
    let parsed = serde_json::from_slice::<serde_json::Value>(&request[split..]).ok();
    let method = parsed
        .as_ref()
        .and_then(|value| value.get("method"))
        .and_then(serde_json::Value::as_str)
        .map(str::to_owned)
        .unwrap_or_default();
    let params = parsed
        .as_ref()
        .and_then(|value| value.get("params"))
        .map(serde_json::Value::to_string)
        .unwrap_or_default();
    RequestObservation {
        method,
        params,
        authorization,
    }
}

fn observe_authorization(
    value: &str,
    cnonce_source_bytes: usize,
) -> Option<AuthorizationObservation> {
    let fields = value.strip_prefix("Digest ")?;
    let field = |name: &str| {
        fields.split(',').find_map(|part| {
            let (key, value) = part.trim().split_once('=')?;
            (key == name).then(|| value.trim_matches('"').to_owned())
        })
    };
    Some(AuthorizationObservation {
        scheme: "Digest",
        method: "POST",
        uri: (field("uri")? == "/json_rpc").then_some("/json_rpc")?,
        qop: (field("qop")? == "auth").then_some("auth")?,
        nc: field("nc")?,
        cnonce: field("cnonce")?,
        cnonce_source_bytes,
        algorithm: field("algorithm")?,
        opaque: field("opaque"),
    })
}

fn valid_result_body(
    method: &str,
    info: &NodeInfo,
    hard_fork: HardForkInfoFixture,
    include_block_weight_limit: bool,
    include_block_weight_median: bool,
    fault: Option<RpcFault>,
) -> Vec<u8> {
    let result = Zeroizing::new(match method {
        "get_version" => r#"{"version":65567,"release":true}"#.to_owned(),
        "get_height" => r#"{"height":1000}"#.to_owned(),
        "get_balance" => r#"{"balance":1000,"unlocked_balance":900,"multisig_import_needed":false,"per_subaddress":[{"account_index":0,"address_index":7,"address":"synthetic-xmr-address","balance":1000,"unlocked_balance":900,"label":"","num_unspent_outputs":3,"blocks_to_unlock":0,"time_to_unlock":0}],"blocks_to_unlock":0,"time_to_unlock":0}"#
            .to_owned(),
        "create_address" => r#"{"address":"synthetic-xmr-address","address_index":7,"addresses":["synthetic-xmr-address"],"address_indices":[7]}"#
            .to_owned(),
        "get_info" => valid_get_info_result(
            info,
            include_block_weight_limit,
            include_block_weight_median,
            fault,
        ),
        "hard_fork_info" => serde_json::json!({
            "credits": 0,
            "earliest_height": hard_fork.earliest_height,
            "enabled": hard_fork.enabled,
            "state": hard_fork.state,
            "status": "OK",
            "threshold": hard_fork.threshold,
            "top_hash": "synthetic-top-hash",
            "untrusted": hard_fork.untrusted,
            "version": hard_fork.version,
            "votes": hard_fork.votes,
            "voting": hard_fork.voting,
            "window": hard_fork.window,
        })
        .to_string(),
        "close_wallet" | "stop_wallet" => "{}".to_owned(),
        _ => "{}".to_owned(),
    });
    format!(
        r#"{{"jsonrpc":"2.0","id":"bitbook-xmr-v1","result":{}}}"#,
        *result,
    )
    .into_bytes()
}

fn valid_get_info_result(
    info: &NodeInfo,
    include_block_weight_limit: bool,
    include_block_weight_median: bool,
    fault: Option<RpcFault>,
) -> String {
    let mut value = serde_json::json!({
        "adjusted_time": 1_725_000_000_u64,
        "alt_blocks_count": 0_u64,
        "block_size_limit": 600_000_u64,
        "block_size_median": 300_000_u64,
        "block_weight_limit": 600_000_u64,
        "block_weight_median": 300_000_u64,
        "bootstrap_daemon_address": info.bootstrap_daemon_address.as_str(),
        "busy_syncing": false,
        "credits": 0_u64,
        "cumulative_difficulty": 1_000_u64,
        "cumulative_difficulty_top64": 0_u64,
        "database_size": 1_000_000_u64,
        "difficulty": 100_u64,
        "difficulty_top64": 0_u64,
        "free_space": 1_000_000_u64,
        "grey_peerlist_size": 0_u64,
        "height": info.height,
        "height_without_bootstrap": if fault == Some(RpcFault::InconsistentHeights) {
            info.height.saturating_add(1)
        } else {
            info.height_without_bootstrap
        },
        "incoming_connections_count": 0_u64,
        "mainnet": info.mainnet,
        "nettype": info.nettype.as_str(),
        "offline": info.offline,
    });
    let rest = serde_json::json!({
        "outgoing_connections_count": 8_u64,
        "restricted": false,
        "rpc_connections_count": 1_u64,
        "stagenet": info.stagenet,
        "start_time": 1_725_000_000_u64,
        "status": info.status.as_str(),
        "synchronized": info.synchronized,
        "target": 120_u64,
        "target_height": info.target_height,
        "testnet": info.testnet,
        "top_block_hash": "synthetic-top-block-hash",
        "top_hash": "synthetic-top-hash",
        "tx_count": 0_u64,
        "tx_pool_size": 0_u64,
        "untrusted": info.untrusted,
        "update_available": false,
        "version": "0.18.5.1-release",
        "was_bootstrap_ever_used": info.was_bootstrap_ever_used,
        "white_peerlist_size": 8_u64,
        "wide_cumulative_difficulty": "0x3e8",
        "wide_difficulty": "0x64",
    });
    let object = value
        .as_object_mut()
        .expect("get_info fixture is an object");
    if let serde_json::Value::Object(rest_object) = rest {
        object.extend(rest_object);
    }
    if !include_block_weight_limit {
        object.remove("block_weight_limit");
    }
    if !include_block_weight_median {
        object.remove("block_weight_median");
    }
    match fault {
        Some(RpcFault::NodeVersionNumeric) => {
            object.insert("version".to_owned(), serde_json::Value::from(196_608_u64));
        }
        Some(RpcFault::NodeVersionTypeConfusion) => {
            object.insert("version".to_owned(), serde_json::Value::Bool(true));
        }
        Some(RpcFault::NodeVersionOverlong) => {
            object.insert(
                "version".to_owned(),
                serde_json::Value::String("x".repeat(129)),
            );
        }
        Some(RpcFault::BlockWeightLimitTypeConfusion) => {
            object.insert(
                "block_weight_limit".to_owned(),
                serde_json::Value::String("600000".to_owned()),
            );
        }
        Some(RpcFault::BlockWeightMedianTypeConfusion) => {
            object.insert(
                "block_weight_median".to_owned(),
                serde_json::Value::String("300000".to_owned()),
            );
        }
        Some(RpcFault::MissingRequiredNodeMember) => {
            object.remove("credits");
        }
        Some(RpcFault::ExtraNodeMember) => {
            object.insert("synthetic_extra".to_owned(), serde_json::Value::Null);
        }
        _ => {}
    }
    value.to_string()
}

fn apply_json_fault(body: &mut Vec<u8>, fault: Option<JsonFault>) {
    let replacement = match fault {
        None => return,
        Some(JsonFault::InvalidUtf8) => vec![0xff],
        Some(JsonFault::Bom) => [vec![0xef, 0xbb, 0xbf], body.clone()].concat(),
        Some(JsonFault::TrailingBytes) => [body.clone(), b"x".to_vec()].concat(),
        Some(JsonFault::DuplicateKey) => br#"{"jsonrpc":"2.0","jsonrpc":"2.0","id":"bitbook-xmr-v1","result":{"version":65567,"release":true}}"#.to_vec(),
        Some(JsonFault::WrongVersion) => br#"{"jsonrpc":"1.0","id":"bitbook-xmr-v1","result":{"version":65567,"release":true}}"#.to_vec(),
        Some(JsonFault::WrongId) => br#"{"jsonrpc":"2.0","id":"wrong","result":{"version":65567,"release":true}}"#.to_vec(),
        Some(JsonFault::MissingId) => br#"{"jsonrpc":"2.0","result":{"version":65567,"release":true}}"#.to_vec(),
        Some(JsonFault::TypeConfusion) => br#"{"jsonrpc":"2.0","id":"bitbook-xmr-v1","result":{"version":"65567","release":true}}"#.to_vec(),
        Some(JsonFault::IntegerOverflow) => br#"{"jsonrpc":"2.0","id":"bitbook-xmr-v1","result":{"version":9223372036854775808,"release":true}}"#.to_vec(),
        Some(JsonFault::UnsupportedShape) => br#"{"jsonrpc":"2.0","id":"bitbook-xmr-v1","result":{"version":65567,"release":true,"extra":true}}"#.to_vec(),
    };
    *body = replacement;
}

fn http_response(
    status: &str,
    body: &[u8],
    challenge: Option<&str>,
    fault: Option<HttpFault>,
    total: Option<usize>,
) -> Vec<u8> {
    let mut body = Zeroizing::new(body.to_vec());
    let mut response = build_http_response(status, &body, challenge, fault);
    if let Some(target) = total {
        if response.len() < target {
            let resized_len = body.len() + target - response.len();
            body.resize(resized_len, b' ');
            response = build_http_response(status, &body, challenge, fault);
        }
        while response.len() < target {
            body.push(b' ');
            response = build_http_response(status, &body, challenge, fault);
        }
        while response.len() > target && !body.is_empty() {
            body.pop();
            response = build_http_response(status, &body, challenge, fault);
        }
    }
    response
}

fn build_http_response(
    status: &str,
    body: &[u8],
    challenge: Option<&str>,
    fault: Option<HttpFault>,
) -> Vec<u8> {
    let status = match fault {
        Some(HttpFault::Redirect301) => "HTTP/1.1 301 Moved Permanently",
        Some(HttpFault::Redirect307) => "HTTP/1.1 307 Temporary Redirect",
        Some(HttpFault::StatusUnknown) => "HTTP/1.1 418 Unknown",
        Some(HttpFault::Http10) => "HTTP/1.0 200 OK",
        _ => status,
    };
    let mut headers = vec![format!("Content-Length: {}", body.len())];
    match fault {
        Some(HttpFault::MissingContentLength) => headers.clear(),
        Some(HttpFault::DuplicateContentLength) => {
            headers.push(format!("Content-Length: {}", body.len()))
        }
        Some(HttpFault::ConflictingContentLength) => {
            headers.push(format!("Content-Length: {}", body.len().saturating_add(1)))
        }
        Some(HttpFault::TransferEncodingChunked) => {
            headers.push("Transfer-Encoding: chunked".to_owned())
        }
        Some(HttpFault::TransferEncodingIdentity) => {
            headers.push("Transfer-Encoding: identity".to_owned())
        }
        Some(HttpFault::FoldedHeader) => headers.push(" folded: value".to_owned()),
        Some(HttpFault::ControlByte) => headers.push("X-Control: bad\u{7f}".to_owned()),
        _ => {}
    }
    if fault != Some(HttpFault::MissingConnectionClose) {
        headers.push("Connection: close".to_owned());
    }
    headers.push("Content-Type: application/json".to_owned());
    if let Some(challenge) = challenge {
        headers.push(format!("WWW-Authenticate: {challenge}"));
    }
    let mut response = format!("{status}\r\n{}\r\n\r\n", headers.join("\r\n")).into_bytes();
    response.extend_from_slice(body);
    if fault == Some(HttpFault::TrailingBytes) {
        response.push(b'x');
    }
    response
}

pub struct RpcTransportRig {
    core: RpcCore<RecordingRpcPort>,
    wallet: bool,
    username: Zeroizing<String>,
    password: Zeroizing<String>,
    audit: Arc<WipeAudit>,
    last_error: Option<XmrError>,
    logs: Vec<&'static str>,
    panic_output: Option<&'static str>,
    json_nesting: Option<usize>,
    last_result: Option<RpcResultObservation>,
    last_dispatch_lookup: Option<(String, bool)>,
}

impl core::fmt::Debug for RpcTransportRig {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("RpcTransportRig")
            .field("wallet", &self.wallet)
            .field("request_count", &self.core.port().request_count)
            .finish()
    }
}

impl RpcTransportRig {
    pub fn wallet() -> Self {
        Self::new(true, RecordingRpcPort::wallet())
    }

    pub fn node() -> Self {
        Self::new(false, RecordingRpcPort::node(XmrNetwork::Stagenet))
    }

    fn new(wallet: bool, port: RecordingRpcPort) -> Self {
        let audit = Arc::new(WipeAudit::default());
        Self {
            core: RpcCore::with_audit(port, audit.clone()),
            wallet,
            username: Zeroizing::new("bitbook-rpc".to_owned()),
            password: Zeroizing::new("synthetic-password".to_owned()),
            audit,
            last_error: None,
            logs: Vec::new(),
            panic_output: None,
            json_nesting: None,
            last_result: None,
            last_dispatch_lookup: None,
        }
    }

    pub fn wallet_with_canaries(canaries: &[&str; 6]) -> Self {
        let mut rig = Self::wallet();
        rig.password = Zeroizing::new(canaries[0].to_owned());
        *rig.core.port_mut().challenge = format!(
            r#"Digest realm="{}", nonce="{}", qop="auth", algorithm=MD5"#,
            canaries[1], canaries[2]
        );
        rig.core.port_mut().upstream_canary = Zeroizing::new(canaries[5].to_owned());
        rig
    }

    pub fn digest_response_for_test(vector: &DigestVector<'_>) -> Result<String, XmrError> {
        if vector
            .algorithm
            .is_some_and(|value| !value.eq_ignore_ascii_case("MD5"))
        {
            return Err(XmrError::unauth());
        }
        digest_response_for_test(DigestResponseInput {
            username: vector.username,
            password: vector.password,
            realm: vector.realm,
            nonce: vector.nonce,
            uri: vector.uri,
            method: vector.method,
            qop: vector.qop,
            nc: vector.nc,
            cnonce: vector.cnonce,
        })
    }

    pub fn call(&mut self, method: RpcMethod) -> Result<(), XmrError> {
        self.core.port_mut().last_call_request_count = 0;
        self.core.port_mut().redirect_pending = false;
        self.core.port_mut().followed_redirect = false;
        self.last_result = None;
        let request = match method.request() {
            Ok(request) => request,
            Err(error) => return self.remember(Err(error)),
        };
        let result = if let Some(nesting) = self.json_nesting.take() {
            let mut nested = vec![b'['; nesting];
            nested.extend_from_slice(b"null");
            nested.extend(std::iter::repeat_n(b']', nesting));
            validate_json_for_test(&nested).map(|_| RpcResultObservation::default())
        } else if self.wallet {
            self.core
                .call_wallet(49_152, &self.username, &self.password, request)
                .and_then(observe_typed_result)
        } else {
            self.core
                .call_node(38_081, request)
                .and_then(observe_typed_result)
        };
        match result {
            Ok(observation) => {
                self.last_result = Some(observation);
                self.remember(Ok(()))
            }
            Err(error) => self.remember(Err(error)),
        }
    }

    pub fn script_challenge(&mut self, challenge: &str) {
        *self.core.port_mut().challenge = challenge.to_owned();
    }

    pub fn last_authorization(&self) -> Option<AuthorizationObservation> {
        self.core
            .port()
            .requests
            .iter()
            .rev()
            .find_map(|request| request.authorization.clone())
    }

    pub fn request_count(&self) -> usize {
        self.core.port().request_count
    }

    pub fn last_call_request_count(&self) -> usize {
        self.core.port().last_call_request_count
    }

    pub fn send_body_for_test(&mut self, body: &[u8]) -> Result<(), XmrError> {
        let result = request_body_boundary_for_test(body.len()).map(|length| {
            self.core.port_mut().bytes_written = length;
        });
        self.remember(result)
    }

    pub fn bytes_written(&self) -> usize {
        self.core.port().bytes_written
    }

    pub fn script_response_total_bytes(&mut self, length: usize) {
        self.core.port_mut().response_total_bytes = Some(length);
    }

    pub fn arm_http_fault(&mut self, fault: HttpFault) {
        self.core.port_mut().http_fault = Some(fault);
    }

    pub fn set_timed_fault(&mut self, fault: RpcFault, millis: u64) {
        self.core.port_mut().timed_fault = Some((fault, millis));
    }

    pub fn script_json_nesting(&mut self, nesting: usize) {
        self.json_nesting = Some(nesting);
    }

    pub fn arm_json_fault(&mut self, fault: JsonFault) {
        self.core.port_mut().json_fault = Some(fault);
    }

    pub fn arm_fault(&mut self, fault: RpcFault) {
        self.core.port_mut().rpc_fault = Some(fault);
    }

    pub fn requests(&self) -> &[RequestObservation] {
        &self.core.port().requests
    }

    pub fn invoke_unlisted_for_test(&mut self, method: &str) -> Result<(), XmrError> {
        self.core.port_mut().last_call_request_count = 0;
        self.last_dispatch_lookup = Some((method.to_owned(), request_dispatch_for_test(method)));
        let result = Err(XmrError::request_schema());
        self.remember(result)
    }

    pub fn last_dispatch_lookup(&self) -> Option<(&str, bool)> {
        self.last_dispatch_lookup
            .as_ref()
            .map(|(method, found)| (method.as_str(), *found))
    }

    pub fn last_request_params(&self) -> Option<&str> {
        self.core
            .port()
            .requests
            .last()
            .map(|request| request.params.as_str())
    }

    pub fn last_result(&self) -> Option<&RpcResultObservation> {
        self.last_result.as_ref()
    }

    pub fn last_result_members(&self) -> Vec<&str> {
        let Some(method) = self
            .core
            .port()
            .requests
            .last()
            .map(|request| request.method.as_str())
        else {
            return Vec::new();
        };
        self.core
            .port()
            .result_members
            .get(method)
            .map(|members| members.iter().map(String::as_str).collect())
            .unwrap_or_default()
    }

    pub fn last_nested_result_members(&self) -> Vec<&str> {
        let Some(method) = self
            .core
            .port()
            .requests
            .last()
            .map(|request| request.method.as_str())
        else {
            return Vec::new();
        };
        self.core
            .port()
            .nested_result_members
            .get(method)
            .map(|members| members.iter().map(String::as_str).collect())
            .unwrap_or_default()
    }

    pub fn returned_bytes(&self) -> &[u8] {
        &self.core.port().retained_response
    }

    pub fn open_connection_count(&self) -> usize {
        self.core.port().open_connections
    }

    pub fn followed_redirect(&self) -> bool {
        self.core.port().followed_redirect
    }

    pub fn exercise_exit(&mut self, exit: &str) {
        match exit {
            "success" => {
                let _ = self.call(RpcMethod::GetVersion);
            }
            "error" => {
                self.arm_fault(RpcFault::UpstreamError);
                let _ = self.call(RpcMethod::GetVersion);
            }
            "panic-unwind" => {
                self.arm_fault(RpcFault::PanicUnwind);
                let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    let _ = self.call(RpcMethod::GetVersion);
                }));
                if result.is_err() {
                    self.panic_output = Some("sanitized RPC panic");
                }
            }
            _ => self.last_error = Some(XmrError::request_schema()),
        }
        self.core.close_all();
        self.core.port_mut().requests.clear();
        self.core.port_mut().challenge.zeroize();
        self.core.port_mut().challenge.clear();
        self.core.port_mut().upstream_canary.zeroize();
        self.core.port_mut().upstream_canary.clear();
        self.core.port_mut().retained_response.zeroize();
        self.core.port_mut().retained_response.clear();
        self.username.zeroize();
        self.username.clear();
        self.password.zeroize();
        self.password.clear();
    }

    pub fn secret_buffers_wiped(&self, _exit: &str) -> bool {
        self.audit.complete() && self.retained_secret_observation_count() == 0
    }

    pub fn retained_secret_observation_count(&self) -> usize {
        self.core
            .port()
            .requests
            .iter()
            .filter(|request| request.authorization.is_some())
            .count()
            + usize::from(!self.core.port().challenge.is_empty())
            + usize::from(!self.core.port().upstream_canary.is_empty())
            + usize::from(!self.core.port().retained_response.is_empty())
            + usize::from(!self.password.is_empty())
    }

    pub fn script_startup_refusals(&mut self, count: usize, attempt_millis: u64) {
        self.core.port_mut().startup_refusals = count;
        self.core.port_mut().startup_attempt_millis = attempt_millis;
    }

    pub fn readiness(&mut self) -> Result<(), XmrError> {
        self.core.port_mut().last_call_request_count = 0;
        let result = self
            .core
            .readiness_wallet(49_152, &self.username, &self.password)
            .and_then(observe_typed_result);
        match result {
            Ok(observation) => {
                self.last_result = Some(observation);
                self.remember(Ok(()))
            }
            Err(error) => self.remember(Err(error)),
        }
    }

    pub fn readiness_attempts(&self) -> usize {
        self.core.port().readiness_attempts
    }

    pub fn readiness_elapsed_millis(&self) -> u64 {
        self.core.port().readiness_elapsed_millis
    }

    pub fn readiness_operation_timeouts_millis(&self) -> &[u64] {
        &self.core.port().readiness_operation_timeouts_millis
    }

    pub fn last_error(&self) -> Option<&XmrError> {
        self.last_error.as_ref()
    }

    pub fn logs(&self) -> &[&'static str] {
        &self.logs
    }

    pub fn panic_output(&self) -> Option<&'static str> {
        self.panic_output
    }

    fn remember(&mut self, result: Result<(), XmrError>) -> Result<(), XmrError> {
        match result {
            Ok(()) => {
                self.last_error = None;
                Ok(())
            }
            Err(error) => {
                self.last_error = Some(error.clone());
                Err(error)
            }
        }
    }
}

pub struct NodeProbeRig {
    network: Result<XmrNetwork, XmrError>,
    core: RpcCore<RecordingRpcPort>,
    returned: Option<NodeProbeResult>,
}

impl NodeProbeRig {
    pub fn reviewed(network: XmrNetwork) -> Self {
        Self {
            network: Ok(network),
            core: RpcCore::new(RecordingRpcPort::node(network)),
            returned: None,
        }
    }

    pub fn from_product_network(network: &str) -> Self {
        let parsed = XmrNetwork::parse(network);
        let fixture_network = parsed.clone().unwrap_or(XmrNetwork::Stagenet);
        Self {
            network: parsed,
            core: RpcCore::new(RecordingRpcPort::node(fixture_network)),
            returned: None,
        }
    }

    pub fn with_info(network: XmrNetwork, info: NodeInfo) -> Self {
        let mut rig = Self::reviewed(network);
        rig.core.port_mut().info = info;
        rig
    }

    pub fn probe(&mut self) -> Result<NodeProbeView, XmrError> {
        self.returned = None;
        self.core.port_mut().redirect_pending = false;
        self.core.port_mut().followed_redirect = false;
        let network = self.network.clone()?;
        let result = probe_node_with(&mut self.core, network)?;
        let _ = (result.height, result.height_without_bootstrap);
        self.returned = Some(result.clone());
        Ok(NodeProbeView {
            state: result.state.into(),
        })
    }

    pub fn attempted_endpoints(&self) -> Vec<&'static str> {
        let mut ports = self.core.port().attempted_ports.clone();
        ports.dedup();
        ports
            .into_iter()
            .map(|port| match port {
                38_081 => "127.0.0.1:38081",
                28_081 => "127.0.0.1:28081",
                _ => "invalid-endpoint",
            })
            .collect()
    }

    pub fn operations(&self) -> &[&'static str] {
        &self.core.port().operations
    }

    pub fn used_dns(&self) -> bool {
        self.dns_resolution_count() != 0
    }

    pub fn used_proxy(&self) -> bool {
        self.proxy_connection_count() != 0
    }

    pub fn numeric_loopback_connection_count(&self) -> usize {
        self.core.port().numeric_loopback_connections
    }

    pub fn dns_resolution_count(&self) -> usize {
        self.core.port().dns_resolutions
    }

    pub fn proxy_connection_count(&self) -> usize {
        self.core.port().proxy_connections
    }

    pub fn get_info_members(&self) -> Vec<&str> {
        self.core
            .port()
            .result_members
            .get("get_info")
            .map(|members| members.iter().map(String::as_str).collect())
            .unwrap_or_default()
    }

    pub fn hard_fork_info_members(&self) -> Vec<&str> {
        self.core
            .port()
            .result_members
            .get("hard_fork_info")
            .map(|members| members.iter().map(String::as_str).collect())
            .unwrap_or_default()
    }

    pub fn followed_redirect(&self) -> bool {
        self.core.port().followed_redirect
    }

    pub fn attempted_alternate_endpoint(&self) -> bool {
        let Some(network) = self.network.as_ref().ok().copied() else {
            return false;
        };
        self.core
            .port()
            .attempted_ports
            .iter()
            .any(|port| *port != node_port(network))
    }

    pub fn info_mut(&mut self) -> &mut NodeInfo {
        &mut self.core.port_mut().info
    }

    pub fn set_block_weight_optionals(&mut self, limit: bool, median: bool) {
        self.core.port_mut().include_block_weight_limit = limit;
        self.core.port_mut().include_block_weight_median = median;
    }

    pub fn set_hard_fork(&mut self, earliest_height: u64, enabled: bool, untrusted: bool) {
        let hard_fork = &mut self.core.port_mut().hard_fork;
        hard_fork.earliest_height = earliest_height;
        hard_fork.enabled = enabled;
        hard_fork.untrusted = untrusted;
    }

    pub fn set_hard_fork_field(&mut self, field: &str, value: u64) {
        let hard_fork = &mut self.core.port_mut().hard_fork;
        match field {
            "version" => hard_fork.version = value,
            "voting" => hard_fork.voting = value,
            "window" => hard_fork.window = value,
            "votes" => hard_fork.votes = value,
            "threshold" => hard_fork.threshold = value,
            "state" => hard_fork.state = value,
            _ => panic!("unreviewed hard-fork field"),
        }
    }

    pub fn mutate_info(&mut self, mutation: &str) {
        let info = &mut self.core.port_mut().info;
        match mutation {
            "bootstrap-address" => {
                info.bootstrap_daemon_address = "remote.invalid:18081".to_owned()
            }
            "untrusted" => info.untrusted = true,
            "offline" => info.offline = true,
            "status-not-ok" => info.status = "BUSY".to_owned(),
            _ => panic!("unreviewed node mutation"),
        }
    }

    pub fn arm_fault(&mut self, fault: RpcFault) {
        self.core.port_mut().rpc_fault = Some(fault);
    }

    pub fn scripted_info(&self) -> &NodeInfo {
        &self.core.port().info
    }

    pub fn returned_state(&self) -> Option<&'static str> {
        self.returned.as_ref().map(|result| result.state.as_str())
    }
}

#[derive(Debug)]
pub struct NodeProbeView {
    pub state: NodeStateView,
}

#[derive(Clone, Copy, Debug)]
pub struct NodeStateView(crate::xmr::rpc::NodeState);

impl NodeStateView {
    pub fn as_str(self) -> &'static str {
        self.0.as_str()
    }
}

impl From<crate::xmr::rpc::NodeState> for NodeStateView {
    fn from(state: crate::xmr::rpc::NodeState) -> Self {
        Self(state)
    }
}

fn observe_typed_result(result: TypedResult) -> Result<RpcResultObservation, XmrError> {
    let mut observation = RpcResultObservation::default();
    match result {
        TypedResult::Version(version) | TypedResult::Height(version) => {
            let _ = version;
        }
        TypedResult::Balance { total, unlocked } => {
            observation.balance = Some(total);
            observation.unlocked_balance = Some(unlocked);
        }
        TypedResult::NodeInfo(info) => {
            let _ = info.height;
        }
        TypedResult::HardForkInfo(info) => {
            let _ = info.earliest_height;
        }
        TypedResult::Empty => {}
        TypedResult::Restore {
            address,
            seed,
            was_deprecated,
        } => {
            let _ = (address.expose()?, seed.expose()?, was_deprecated);
        }
        TypedResult::Generated { address } | TypedResult::Key(address) => {
            let _ = address.expose()?;
        }
        TypedResult::Refreshed {
            blocks_fetched,
            received_money,
        } => {
            let _ = (blocks_fetched, received_money);
        }
        TypedResult::Addresses { primary, addresses } => {
            let _ = primary.expose()?;
            for entry in addresses {
                let _ = (
                    entry.address.expose()?,
                    entry.address_index,
                    entry.label.expose()?,
                    entry.used,
                );
            }
        }
        TypedResult::CreatedAddress {
            address,
            address_index,
            address_count,
        } => {
            let _ = address.expose()?;
            observation.address_index = Some(address_index);
            observation.address_count = Some(address_count);
        }
        TypedResult::AddressValidation {
            valid,
            integrated,
            subaddress,
            nettype,
        } => {
            let _ = (valid, integrated, subaddress, nettype);
        }
    }
    Ok(observation)
}

const ACCOUNT_FIXTURE_PRIMARY: &str = concat!(
    "BBBBBBBBBBBBBBBBBBBB",
    "BBBBBBBBBBBBBBBBBBBB",
    "BBBBBBBBBBBBBBBBBBBB",
    "BBBBBBBBBBBBBBBBBBBB",
    "BBBBBBBBBBBBBBB",
);
const ACCOUNT_FIXTURE_MNEMONIC: &str = concat!(
    "abbey abducts ability ablaze abnormal abort abrasive absorb abstract absurd abuse academy ",
    "aces ache acidic acoustic acquire across actress adapt adept adhesive adjusted adopt adorned",
);
const ACCOUNT_FIXTURE_VIEW_KEY: &str =
    "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
const MISMATCH_PRIMARY: &str = concat!(
    "CCCCCCCCCCCCCCCCCCCC",
    "CCCCCCCCCCCCCCCCCCCC",
    "CCCCCCCCCCCCCCCCCCCC",
    "CCCCCCCCCCCCCCCCCCCC",
    "CCCCCCCCCCCCCCC",
);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AccountFault {
    ReturnedPrimaryMismatch,
    ReportedNotWatchOnly,
    VaultSeal,
    StateWrite,
    StateFileSync,
    StateDirectorySync,
    RollbackCleanup,
}

pub struct XmrSecretFixture {
    kind: AccountKind,
    restore_height: u64,
    password: Zeroizing<String>,
    primary: Zeroizing<String>,
    secret: Zeroizing<String>,
}

impl PartialEq for XmrSecretFixture {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
            && self.restore_height == other.restore_height
            && self.password == other.password
            && self.primary == other.primary
            && self.secret == other.secret
    }
}

impl Eq for XmrSecretFixture {}

impl std::fmt::Debug for XmrSecretFixture {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("XmrSecretFixture([REDACTED])")
    }
}

impl XmrSecretFixture {
    pub fn software(restore_height: u64, password: &str, primary: &str, mnemonic: &str) -> Self {
        Self {
            kind: AccountKind::Software,
            restore_height,
            password: Zeroizing::new(password.to_owned()),
            primary: Zeroizing::new(primary.to_owned()),
            secret: Zeroizing::new(mnemonic.to_owned()),
        }
    }

    pub fn watch_only(restore_height: u64, password: &str, primary: &str, view_key: &str) -> Self {
        Self {
            kind: AccountKind::WatchOnly,
            restore_height,
            password: Zeroizing::new(password.to_owned()),
            primary: Zeroizing::new(primary.to_owned()),
            secret: Zeroizing::new(view_key.to_owned()),
        }
    }

    pub fn encode(&self) -> Result<Zeroizing<Vec<u8>>, XmrError> {
        self.to_secret()?.encode()
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, XmrError> {
        Self::from_secret(XmrSecretV1::decode(bytes)?)
    }

    pub fn mutated_encoding(&self, mutation: &str) -> Zeroizing<Vec<u8>> {
        let mut bytes = self.encode().expect("fixture encodes");
        match mutation {
            "bad-magic" => bytes[0] ^= 1,
            "unknown-kind" => bytes[8] = 3,
            "bad-password-length" => bytes[17..19].copy_from_slice(&63_u16.to_be_bytes()),
            "bad-address-length" => bytes[83..85].copy_from_slice(&94_u16.to_be_bytes()),
            "bad-secret-length" => bytes[180..182].copy_from_slice(&0_u16.to_be_bytes()),
            "uppercase-password" => bytes[19] = b'A',
            "uppercase-view-key" => {
                if let Some(last) = bytes.last_mut() {
                    *last = last.to_ascii_uppercase();
                }
            }
            "trailing-byte" => bytes.push(0),
            "truncated" => {
                bytes.pop();
            }
            "over-2048" => bytes.resize(2_049, 0),
            _ => {}
        }
        bytes
    }

    pub fn validate_password_hex_length_for_test(length: usize) -> Result<(), XmrError> {
        validate_password_hex_length(length)
    }

    pub fn validate_view_key_hex_length_for_test(length: usize) -> Result<(), XmrError> {
        validate_view_key_hex_length(length)
    }

    pub fn validate_primary_address_length_for_test(length: usize) -> Result<(), XmrError> {
        validate_primary_address_length(length)
    }

    pub fn validate_total_length_for_test(length: usize) -> Result<(), XmrError> {
        validate_total_length(length)
    }

    pub fn is_network_valid_address_for_test(address: &str) -> bool {
        is_network_valid_address(address)
    }

    pub fn is_spendable_mnemonic_for_test(mnemonic: &str) -> bool {
        is_spendable_mnemonic(mnemonic)
    }

    fn to_secret(&self) -> Result<XmrSecretV1, XmrError> {
        match self.kind {
            AccountKind::Software => XmrSecretV1::software(
                self.restore_height,
                self.password.as_str(),
                self.primary.as_str(),
                self.secret.as_str(),
            ),
            AccountKind::WatchOnly => XmrSecretV1::watch_only(
                self.restore_height,
                self.password.as_str(),
                self.primary.as_str(),
                self.secret.as_str(),
            ),
        }
    }

    fn from_secret(secret: XmrSecretV1) -> Result<Self, XmrError> {
        let payload = match secret.kind() {
            AccountKind::Software => secret
                .copy_mnemonic()?
                .ok_or_else(XmrError::state_corrupt)?,
            AccountKind::WatchOnly => secret
                .copy_private_view_key()?
                .ok_or_else(XmrError::state_corrupt)?,
        };
        Ok(Self {
            kind: secret.kind(),
            restore_height: secret.restore_height(),
            password: secret.copy_wallet_password()?,
            primary: secret.copy_primary_address()?,
            secret: payload,
        })
    }
}

pub struct SealedSecretView {
    pub restore_height: u64,
}

pub struct SealedRecordView {
    restore_height: u64,
    password: Zeroizing<String>,
    primary: Zeroizing<String>,
    mnemonic: Option<Zeroizing<String>>,
    view_key: Option<Zeroizing<String>>,
}

impl Drop for SealedRecordView {
    fn drop(&mut self) {
        self.password.zeroize();
        self.primary.zeroize();
        if let Some(mnemonic) = &mut self.mnemonic {
            mnemonic.zeroize();
        }
        if let Some(view_key) = &mut self.view_key {
            view_key.zeroize();
        }
    }
}

impl std::fmt::Debug for SealedRecordView {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("SealedRecordView([REDACTED])")
    }
}

impl SealedRecordView {
    fn from_secret(secret: &XmrSecretV1) -> Result<Self, XmrError> {
        Ok(Self {
            restore_height: secret.restore_height(),
            password: secret.copy_wallet_password()?,
            primary: secret.copy_primary_address()?,
            mnemonic: secret.copy_mnemonic()?,
            view_key: secret.copy_private_view_key()?,
        })
    }

    pub fn restore_height(&self) -> u64 {
        self.restore_height
    }

    pub fn wallet_password(&self) -> &str {
        self.password.as_str()
    }

    pub fn primary_address(&self) -> &str {
        self.primary.as_str()
    }

    pub fn mnemonic(&self) -> Option<&str> {
        self.mnemonic.as_deref().map(String::as_str)
    }

    pub fn private_view_key(&self) -> Option<&str> {
        self.view_key.as_deref().map(String::as_str)
    }
}

struct FaultingSurface {
    inner: SqliteSurface,
    faults: Vec<AccountFault>,
}

impl StoreSurface for FaultingSurface {
    fn execute_batch(&mut self, sql: &str) -> Result<(), XmrError> {
        self.inner.execute_batch(sql)
    }

    fn query_i64(&mut self, sql: &str) -> Result<i64, XmrError> {
        self.inner.query_i64(sql)
    }

    fn insert_identity(&mut self, identity: &StoredIdentity) -> Result<(), XmrError> {
        if self.faults.contains(&AccountFault::StateWrite) {
            return Err(XmrError::state_corrupt());
        }
        self.inner.insert_identity(identity)
    }

    fn load_identity(&mut self) -> Result<StoredIdentity, XmrError> {
        self.inner.load_identity()
    }

    fn table_names(&mut self) -> Result<Vec<String>, XmrError> {
        self.inner.table_names()
    }

    fn table_sql(&mut self, table: &str) -> Result<String, XmrError> {
        self.inner.table_sql(table)
    }

    fn column_info(&mut self, table: &str) -> Result<Vec<ColumnInfo>, XmrError> {
        self.inner.column_info(table)
    }

    fn schema_objects(&mut self) -> Result<Vec<(String, String, String)>, XmrError> {
        self.inner.schema_objects()
    }

    fn unique_column_sets(&mut self, table: &str) -> Result<Vec<Vec<String>>, XmrError> {
        self.inner.unique_column_sets(table)
    }

    fn begin(&mut self) -> Result<(), XmrError> {
        self.inner.begin()
    }

    fn commit(&mut self) -> Result<(), XmrError> {
        self.inner.commit()
    }

    fn rollback(&mut self) -> Result<(), XmrError> {
        self.inner.rollback()
    }

    fn sync_file(&mut self) -> Result<(), XmrError> {
        if self.faults.contains(&AccountFault::StateFileSync) {
            return Err(XmrError::state_corrupt());
        }
        self.inner.sync_file()
    }

    fn sync_directory(&mut self) -> Result<(), XmrError> {
        if self.faults.contains(&AccountFault::StateDirectorySync) {
            return Err(XmrError::state_corrupt());
        }
        self.inner.sync_directory()
    }

    fn reopen_existing(&mut self) -> Result<(), XmrError> {
        if self.faults.contains(&AccountFault::StateFileSync)
            || self.faults.contains(&AccountFault::StateDirectorySync)
        {
            return Err(XmrError::state_corrupt());
        }
        self.inner.reopen_existing()
    }
}

const TEST_VAULT_PASSPHRASE: &[u8] = b"bitbook-xmr-account-test-pass";

struct RecordingWipe {
    events: Arc<Mutex<Vec<WipeEvent>>>,
}

impl WipeObserver for RecordingWipe {
    fn observe(&mut self, event: WipeEvent) {
        if let Ok(mut events) = self.events.lock() {
            events.push(event);
        }
    }
}

struct RecordingWork;

impl VaultWorkObserver for RecordingWork {
    fn before_allocation(&mut self, _bytes: usize) -> Result<(), VaultError> {
        Ok(())
    }

    fn before_kdf(&mut self) {}
}

#[derive(Clone, Default)]
struct RecordingAttempt {
    vault: bool,
    state: bool,
    wallet: bool,
    keys: bool,
}

struct RecordingAccountPort {
    account_id: String,
    network: XmrNetwork,
    primary: Zeroizing<String>,
    mnemonic: Zeroizing<String>,
    view_key: Zeroizing<String>,
    restore_height: u64,
    local_height: u64,
    operations: Vec<&'static str>,
    calls: Vec<AccountRpcCall>,
    faults: Vec<AccountFault>,
    presence: WalletPresence,
    child_count: usize,
    handles: usize,
    replaced_hostile: bool,
    sealed_bytes: Option<Vec<u8>>,
    last_password: Option<SecretBytes>,
    open_mutation: Option<&'static str>,
    store: AccountStore<FaultingSurface>,
    wipe_events: Arc<Mutex<Vec<WipeEvent>>>,
    passphrase: SecretBytes,
    attempt: RecordingAttempt,
    wallet_open: bool,
}

impl RecordingAccountPort {
    fn new(account_id: &str, network: XmrNetwork) -> Self {
        let surface = FaultingSurface {
            inner: SqliteSurface::memory().expect("in-memory account store"),
            faults: Vec::new(),
        };
        Self {
            account_id: account_id.to_owned(),
            network,
            primary: Zeroizing::new(ACCOUNT_FIXTURE_PRIMARY.to_owned()),
            mnemonic: Zeroizing::new(ACCOUNT_FIXTURE_MNEMONIC.to_owned()),
            view_key: Zeroizing::new(ACCOUNT_FIXTURE_VIEW_KEY.to_owned()),
            restore_height: 900,
            local_height: 1_000,
            operations: Vec::new(),
            calls: Vec::new(),
            faults: Vec::new(),
            presence: WalletPresence::Missing,
            child_count: 0,
            handles: 0,
            replaced_hostile: false,
            sealed_bytes: None,
            last_password: None,
            open_mutation: None,
            store: AccountStore::new(surface),
            wipe_events: Arc::new(Mutex::new(Vec::new())),
            passphrase: SecretBytes::new(TEST_VAULT_PASSPHRASE.to_vec())
                .expect("test vault passphrase"),
            attempt: RecordingAttempt::default(),
            wallet_open: false,
        }
    }

    fn replace_text(slot: &mut Zeroizing<String>, value: &str) {
        slot.zeroize();
        *slot = Zeroizing::new(value.to_owned());
    }

    fn has_fault(&self, fault: AccountFault) -> bool {
        self.faults.contains(&fault)
    }

    fn arm_fault(&mut self, fault: AccountFault) {
        if !self.faults.contains(&fault) {
            self.faults.push(fault);
        }
        if !self.store.surface_mut().faults.contains(&fault) {
            self.store.surface_mut().faults.push(fault);
        }
    }

    fn remember_password(&mut self, password: &str) -> Result<(), XmrError> {
        self.last_password =
            Some(SecretBytes::new(password.as_bytes().to_vec()).map_err(|_| XmrError::internal())?);
        Ok(())
    }

    fn observation(&self, method: &str) -> WalletRpcObservation {
        let mut primary = Zeroizing::new(self.primary.as_str().to_owned());
        let mut verified_primary = Zeroizing::new(primary.as_str().to_owned());
        let mut watch_only = method == "generate_from_keys";
        if self.has_fault(AccountFault::ReturnedPrimaryMismatch) && method == "get_address" {
            verified_primary.zeroize();
            verified_primary = Zeroizing::new(MISMATCH_PRIMARY.to_owned());
        }
        if self.has_fault(AccountFault::ReportedNotWatchOnly) {
            watch_only = false;
        }
        if self.open_mutation == Some("primary-address") {
            primary.zeroize();
            primary = Zeroizing::new(MISMATCH_PRIMARY.to_owned());
            verified_primary.zeroize();
            verified_primary = Zeroizing::new(primary.as_str().to_owned());
        }
        WalletRpcObservation {
            primary,
            verified_primary,
            watch_only,
        }
    }

    fn vault_metadata(&self) -> Result<VaultMetadata, XmrError> {
        let mut id = [0u8; 16];
        if self.account_id.len() == 32 {
            for (index, chunk) in self
                .account_id
                .as_bytes()
                .as_chunks::<2>()
                .0
                .iter()
                .enumerate()
            {
                let hi = hex_nibble(chunk[0]);
                let lo = hex_nibble(chunk[1]);
                if let (Some(hi), Some(lo)) = (hi, lo) {
                    id[index] = (hi << 4) | lo;
                }
            }
        }
        VaultMetadata::new(
            id,
            Asset::Xmr,
            match self.network {
                XmrNetwork::Stagenet => VaultNetwork::XmrStagenet,
                XmrNetwork::Testnet => VaultNetwork::XmrTestnet,
            },
            1,
        )
        .map_err(|_| XmrError::state_corrupt())
    }
}

fn hex_nibble(value: u8) -> Option<u8> {
    match value {
        b'0'..=b'9' => Some(value - b'0'),
        b'a'..=b'f' => Some(value - b'a' + 10),
        _ => None,
    }
}

impl AccountPort for RecordingAccountPort {
    fn note(&mut self, operation: &'static str) {
        self.operations.push(operation);
    }

    fn operations(&self) -> &[&'static str] {
        &self.operations
    }

    fn fill_entropy(&mut self, output: &mut [u8]) -> Result<(), XmrError> {
        getrandom::fill(output).map_err(|_| XmrError::internal())
    }

    fn local_height_without_bootstrap(&mut self) -> Result<u64, XmrError> {
        Ok(self.local_height)
    }

    fn begin_attempt(&mut self) {
        self.attempt = RecordingAttempt::default();
    }

    fn commit_attempt(&mut self) {
        self.attempt = RecordingAttempt::default();
    }

    fn preflight_create_new(&mut self, _paths: &AccountPathsView) -> Result<(), XmrError> {
        if self.sealed_bytes.is_some() || matches!(self.presence, WalletPresence::Hostile(_)) {
            return Err(XmrError::state_corrupt());
        }
        Ok(())
    }

    fn preflight_open_existing(
        &mut self,
        _paths: &AccountPathsView,
    ) -> Result<WalletPresence, XmrError> {
        if self.sealed_bytes.is_none() {
            return Err(XmrError::state_corrupt());
        }
        if self.open_mutation == Some("partial-wallet-set") {
            return Ok(WalletPresence::Partial);
        }
        if matches!(self.presence, WalletPresence::Hostile(_)) {
            return Err(XmrError::state_corrupt());
        }
        Ok(self.presence)
    }

    fn create_private_layout_after_preflight(&mut self) -> Result<(), XmrError> {
        Ok(())
    }

    fn create_missing_wallet_layout(&mut self) -> Result<(), XmrError> {
        self.attempt.wallet = true;
        self.attempt.keys = true;
        Ok(())
    }

    fn inspect_wallet(&mut self, _paths: &AccountPathsView) -> Result<WalletPresence, XmrError> {
        if self.open_mutation == Some("partial-wallet-set") {
            return Ok(WalletPresence::Partial);
        }
        Ok(self.presence)
    }

    fn create_wallet_files(&mut self, _paths: &AccountPathsView) -> Result<(), XmrError> {
        if matches!(self.presence, WalletPresence::Hostile(_)) {
            return Err(XmrError::state_corrupt());
        }
        self.presence = WalletPresence::Complete;
        self.attempt.wallet = true;
        self.attempt.keys = true;
        Ok(())
    }

    fn rollback_owned_artifacts(&mut self) -> Result<(), XmrError> {
        if self.attempt.wallet || self.attempt.keys {
            self.presence = WalletPresence::Missing;
        }
        if self.attempt.vault {
            self.sealed_bytes = None;
        }
        if self.attempt.state {
            let surface = FaultingSurface {
                inner: SqliteSurface::memory().expect("in-memory account store"),
                faults: self.faults.clone(),
            };
            self.store = AccountStore::new(surface);
        }
        if self.has_fault(AccountFault::RollbackCleanup) {
            Err(XmrError::internal())
        } else {
            self.attempt = RecordingAttempt::default();
            Ok(())
        }
    }

    fn start_child(&mut self) -> Result<(), XmrError> {
        self.child_count = 1;
        Ok(())
    }

    fn create_wallet(&mut self, filename: &str, password: &str) -> Result<(), XmrError> {
        self.remember_password(password)?;
        self.calls
            .push(AccountRpcCall::create_wallet(filename, password));
        self.handles = self.handles.saturating_add(1);
        self.presence = WalletPresence::Complete;
        self.wallet_open = true;
        self.attempt.wallet = true;
        self.attempt.keys = true;
        Ok(())
    }

    fn query_mnemonic(&mut self) -> Result<Zeroizing<String>, XmrError> {
        self.calls.push(AccountRpcCall::query_key_mnemonic());
        Ok(self.mnemonic.clone())
    }

    fn get_primary_address(&mut self) -> Result<WalletRpcObservation, XmrError> {
        self.calls.push(AccountRpcCall::get_address());
        Ok(self.observation("get_address"))
    }

    fn validate_primary_for_network(&mut self, _address: &str) -> Result<(), XmrError> {
        Ok(())
    }

    fn generate_from_keys(
        &mut self,
        filename: &str,
        password: &str,
        address: &str,
        viewkey: &str,
        restore_height: u64,
    ) -> Result<WalletRpcObservation, XmrError> {
        self.remember_password(password)?;
        Self::replace_text(&mut self.primary, address);
        Self::replace_text(&mut self.view_key, viewkey);
        self.restore_height = restore_height;
        self.calls.push(AccountRpcCall::generate_from_keys(
            filename,
            password,
            address,
            viewkey,
            restore_height,
        ));
        self.handles = self.handles.saturating_add(1);
        self.presence = WalletPresence::Complete;
        self.wallet_open = true;
        self.attempt.wallet = true;
        self.attempt.keys = true;
        Ok(self.observation("generate_from_keys"))
    }

    fn open_wallet(
        &mut self,
        filename: &str,
        password: &str,
    ) -> Result<WalletRpcObservation, XmrError> {
        self.remember_password(password)?;
        self.calls
            .push(AccountRpcCall::open_wallet(filename, password));
        self.handles = self.handles.saturating_add(1);
        self.wallet_open = true;
        Ok(self.observation("open_wallet"))
    }

    fn restore_deterministic_wallet(
        &mut self,
        filename: &str,
        password: &str,
        seed: &str,
        restore_height: u64,
    ) -> Result<WalletRpcObservation, XmrError> {
        self.remember_password(password)?;
        self.restore_height = restore_height;
        self.calls
            .push(AccountRpcCall::restore_deterministic_wallet(
                filename,
                password,
                seed,
                restore_height,
            ));
        self.handles = self.handles.saturating_add(1);
        self.presence = WalletPresence::Complete;
        self.wallet_open = true;
        self.attempt.wallet = true;
        self.attempt.keys = true;
        Ok(self.observation("restore_deterministic_wallet"))
    }

    fn close_wallet(&mut self) -> Result<(), XmrError> {
        self.calls.push(AccountRpcCall::close_wallet());
        self.handles = 0;
        self.wallet_open = false;
        Ok(())
    }

    fn recorded_calls(&self) -> &[AccountRpcCall] {
        &self.calls
    }

    fn seal_vault(&mut self, secret: &XmrSecretV1) -> Result<(), XmrError> {
        if self.has_fault(AccountFault::VaultSeal) {
            return Err(XmrError::state_corrupt());
        }
        let metadata = self.vault_metadata()?;
        let mut plaintext =
            SecretBytes::new(secret.encode()?.to_vec()).map_err(|_| XmrError::state_corrupt())?;
        let mut passphrase = SecretBytes::new(self.passphrase.expose(|bytes| bytes.to_vec()))
            .map_err(|_| XmrError::state_corrupt())?;
        let mut entropy = OsEntropy;
        let mut observer = RecordingWipe {
            events: self.wipe_events.clone(),
        };
        let envelope = seal_vault(
            &metadata,
            &mut passphrase,
            &mut plaintext,
            &mut entropy,
            &mut observer,
        )
        .map_err(|_| XmrError::state_corrupt())?;
        self.sealed_bytes = Some(envelope.as_bytes().to_vec());
        self.attempt.vault = true;
        Ok(())
    }

    fn open_vault(&mut self) -> Result<XmrSecretV1, XmrError> {
        let bytes = self.sealed_bytes.as_ref().ok_or_else(XmrError::unauth)?;
        let mut passphrase = SecretBytes::new(self.passphrase.expose(|value| value.to_vec()))
            .map_err(|_| XmrError::unauth())?;
        let mut work = RecordingWork;
        let mut observer = RecordingWipe {
            events: self.wipe_events.clone(),
        };
        let plaintext = open_vault_bytes(bytes, &mut passphrase, &mut work, &mut observer)
            .map_err(|_| XmrError::unauth())?;
        plaintext.expose(XmrSecretV1::decode)
    }

    fn persist_state(&mut self, identity: &StoredIdentity) -> Result<(), XmrError> {
        self.attempt.state = true;
        self.store.persist_identity(identity)
    }

    fn load_state(&mut self) -> Result<StoredIdentity, XmrError> {
        let loaded = self.store.load_identity()?;
        let primary = loaded.primary_address()?;
        match self.open_mutation {
            Some("network") => {
                let network = if self.network == XmrNetwork::Stagenet {
                    XmrNetwork::Testnet
                } else {
                    XmrNetwork::Stagenet
                };
                StoredIdentity::new(
                    loaded.account_id().to_owned(),
                    network,
                    loaded.kind(),
                    primary.as_str(),
                    loaded.restore_height(),
                )
            }
            Some("kind") => {
                let kind = if loaded.kind() == 1 { 2 } else { 1 };
                StoredIdentity::new(
                    loaded.account_id().to_owned(),
                    self.network,
                    kind,
                    primary.as_str(),
                    loaded.restore_height(),
                )
            }
            Some("restore-height") => StoredIdentity::new(
                loaded.account_id().to_owned(),
                self.network,
                loaded.kind(),
                primary.as_str(),
                loaded.restore_height().saturating_add(1),
            ),
            Some("account-id") => StoredIdentity::new(
                "ffeeddccbbaa99887766554433221100".to_owned(),
                self.network,
                loaded.kind(),
                primary.as_str(),
                loaded.restore_height(),
            ),
            _ => Ok(loaded),
        }
    }

    fn stop_and_reap_child(&mut self) -> Result<(), XmrError> {
        self.teardown_owned()
    }

    fn teardown_owned(&mut self) -> Result<(), XmrError> {
        if self.child_count > 0 {
            if self.wallet_open {
                self.calls.push(AccountRpcCall::close_wallet());
                self.wallet_open = false;
            }
            self.calls.push(AccountRpcCall::stop_wallet());
        }
        self.child_count = 0;
        self.handles = 0;
        if self.has_fault(AccountFault::RollbackCleanup) {
            Err(XmrError::internal())
        } else {
            Ok(())
        }
    }

    fn prove_owned_session(
        &mut self,
        account_id: &str,
        network: XmrNetwork,
    ) -> Result<(), XmrError> {
        if self.account_id == account_id && self.network == network {
            Ok(())
        } else {
            Err(XmrError::state_corrupt())
        }
    }

    fn wipe_wallet_password(&mut self) {
        if let Some(mut password) = self.last_password.take() {
            let mut observer = RecordingWipe {
                events: self.wipe_events.clone(),
            };
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

impl Drop for RecordingAccountPort {
    fn drop(&mut self) {
        self.primary.zeroize();
        self.mnemonic.zeroize();
        self.view_key.zeroize();
        self.wipe_wallet_password();
    }
}

pub struct AccountRig {
    manager: AccountManager<RecordingAccountPort>,
    wipe_events: Arc<Mutex<Vec<WipeEvent>>>,
}

impl AccountRig {
    pub fn software(account: &str, network: XmrNetwork) -> Self {
        Self::new(account, network.name(), "software", network)
    }

    pub fn watch_only(account: &str, network: XmrNetwork) -> Self {
        Self::new(account, network.name(), "watch_only", network)
    }

    pub fn unvalidated(account_id: &str, network: &str, kind: &str) -> Self {
        let parsed_network = XmrNetwork::parse(network).unwrap_or(XmrNetwork::Stagenet);
        Self::new(account_id, network, kind, parsed_network)
    }

    pub fn sealed(account: &str, network: XmrNetwork, kind: AccountKind) -> Self {
        let mut rig = match kind {
            AccountKind::Software => Self::software(account, network),
            AccountKind::WatchOnly => Self::watch_only(account, network),
        };
        rig.install_sealed(kind);
        rig
    }

    pub fn with_hostile_entry(
        account: &str,
        network: XmrNetwork,
        entry: HostileWalletEntry,
    ) -> Self {
        let mut rig = Self::sealed(account, network, AccountKind::Software);
        rig.manager.port_mut().presence = WalletPresence::Hostile(entry);
        rig
    }

    fn new(account_id: &str, network_text: &str, kind_text: &str, network: XmrNetwork) -> Self {
        let port = RecordingAccountPort::new(account_id, network);
        Self {
            wipe_events: port.wipe_events.clone(),
            manager: AccountManager::new(account_id, network_text, kind_text, port),
        }
    }

    fn install_sealed(&mut self, kind: AccountKind) {
        let mut observation = generate_wallet_password(self.manager.port_mut())
            .expect("sealed password")
            .observation();
        let mut password = Zeroizing::new(std::mem::take(&mut observation.encoded));
        let secret = match kind {
            AccountKind::Software => XmrSecretV1::software(
                900,
                &password,
                ACCOUNT_FIXTURE_PRIMARY,
                ACCOUNT_FIXTURE_MNEMONIC,
            ),
            AccountKind::WatchOnly => XmrSecretV1::watch_only(
                900,
                &password,
                ACCOUNT_FIXTURE_PRIMARY,
                ACCOUNT_FIXTURE_VIEW_KEY,
            ),
        }
        .expect("sealed secret");
        password.zeroize();
        self.manager
            .port_mut()
            .seal_vault(&secret)
            .expect("seal sealed account");
        let identity = StoredIdentity::new(
            self.manager.port().account_id.clone(),
            self.manager.port().network,
            kind.code(),
            ACCOUNT_FIXTURE_PRIMARY,
            900,
        )
        .expect("sealed identity");
        self.manager
            .port_mut()
            .persist_state(&identity)
            .expect("persist sealed identity");
        self.manager.port_mut().presence = WalletPresence::Complete;
        self.manager.port_mut().restore_height = 900;
        RecordingAccountPort::replace_text(
            &mut self.manager.port_mut().primary,
            ACCOUNT_FIXTURE_PRIMARY,
        );
    }

    pub fn set_local_height_without_bootstrap(&mut self, height: u64) {
        self.manager.port_mut().local_height = height;
    }

    pub fn arm_fault(&mut self, fault: AccountFault) {
        self.manager.port_mut().arm_fault(fault);
    }

    pub fn create_software(&mut self) -> Result<CreatedAccount, XmrError> {
        self.manager.create_software()
    }

    pub fn import_watch_only(
        &mut self,
        primary: &str,
        view_key: &str,
        restore_height: u64,
    ) -> Result<CreatedAccount, XmrError> {
        self.manager
            .import_watch_only(primary, view_key, restore_height)
    }

    pub fn create(&mut self) -> Result<CreatedAccount, XmrError> {
        self.manager.create()
    }

    pub fn open(&mut self) -> Result<CreatedAccount, XmrError> {
        self.manager.open()
    }

    pub fn lock(&mut self) -> Result<(), XmrError> {
        self.manager.lock()
    }

    pub fn rpc_calls(&self) -> Vec<&'static str> {
        self.manager
            .port()
            .recorded_calls()
            .iter()
            .map(AccountRpcCall::method)
            .collect()
    }

    pub fn rpc_field_names(&self, method: &str) -> Vec<&'static str> {
        self.lookup_call(method)
            .map(AccountRpcCall::field_names)
            .unwrap_or_default()
    }

    pub fn rpc_argument(&self, method: &str, field: &str) -> Option<&str> {
        self.lookup_call(method)
            .and_then(|call| call.argument(field))
    }

    pub fn rpc_argument_matches_secret(&self, method: &str, field: &str, expected: &str) -> bool {
        self.rpc_argument(method, field) == Some(expected)
    }

    pub fn sealed_secret_for_test(&self) -> SealedSecretView {
        SealedSecretView {
            restore_height: self
                .sealed_record()
                .map(|record| record.restore_height())
                .unwrap_or(0),
        }
    }

    pub fn authenticated_sealed_record_for_test(&self) -> Result<SealedRecordView, XmrError> {
        self.sealed_record()
    }

    pub fn vault_sealed_before_account_state(&self) -> bool {
        self.manager.vault_sealed_before_account_state()
    }

    pub fn account_state_durable_before_return(&self) -> bool {
        self.manager.account_state_durable_before_return()
    }

    pub fn returned_account(&self) -> Option<&CreatedAccount> {
        self.manager.returned_account()
    }

    pub fn vault_committed(&self) -> bool {
        self.manager.vault_committed()
    }

    pub fn account_state_committed(&self) -> bool {
        self.manager.account_state_committed()
    }

    pub fn active_child_count(&self) -> usize {
        self.manager.active_child_count()
    }

    pub fn open_handle_count(&self) -> usize {
        self.manager.open_handle_count()
    }

    pub fn generated_wallet_removed_or_quarantined(&self) -> bool {
        self.manager.generated_wallet_removed_or_quarantined()
    }

    pub fn creation_secrets_wiped(&self) -> bool {
        self.manager.creation_secrets_wiped()
    }

    pub fn rpc_reported_watch_only(&self) -> bool {
        self.manager.rpc_reported_watch_only()
    }

    pub fn remove_wallet_files_for_test(&mut self) {
        self.manager.port_mut().presence = WalletPresence::Missing;
    }

    pub fn vault_authenticated_before_recovery(&self) -> bool {
        self.manager.vault_authenticated_before_recovery()
    }

    pub fn recovery_created_files(&self) -> bool {
        self.manager.recovery_created_files()
    }

    pub fn identity_verified_after_open(&self) -> bool {
        self.manager.identity_verified_after_open()
    }

    pub fn mutate_open_identity(&mut self, mutation: &'static str) {
        self.manager.port_mut().open_mutation = Some(mutation);
    }

    pub fn inspect_paths(&self) -> AccountPathsView {
        self.manager.inspect_paths().expect("derived account paths")
    }

    pub fn operations(&self) -> &[&'static str] {
        self.manager.operations()
    }

    pub fn replaced_hostile_entry(&self) -> bool {
        self.manager.replaced_hostile_entry()
    }

    pub fn account_unavailable(&self) -> bool {
        self.manager.account_unavailable()
    }

    pub fn last_rpc_calls(&self) -> &[&'static str] {
        self.manager.last_rpc_calls()
    }

    pub fn wallet_password_wiped(&self) -> bool {
        self.manager.wallet_password_wiped()
    }

    pub fn may_retain_process(&self) -> bool {
        self.manager.may_retain_process()
    }

    pub fn simulate_cold_restart(&mut self) {
        self.manager.simulate_cold_restart();
    }

    pub fn requires_authenticated_vault_for_password(&self) -> bool {
        self.manager.requires_authenticated_vault_for_password()
    }

    pub fn fresh_wallet_password_for_test() -> Result<WalletPasswordObservation, XmrError> {
        let mut port =
            RecordingAccountPort::new("00112233445566778899aabbccddeeff", XmrNetwork::Stagenet);
        generate_wallet_password(&mut port).map(|password| password.observation())
    }

    pub fn exercise_secret_exit(&mut self, exit: SecretExit, secrets: Vec<SecretBytes>) {
        self.wipe_events = Arc::new(Mutex::new(Vec::new()));
        let _ = crate::xmr::account::run_secret_exit(secrets, exit, self.wipe_events.clone());
        let _ = self.manager.port_mut().stop_and_reap_child();
        self.manager.port_mut().wipe_wallet_password();
    }

    pub fn all_secret_wipes_observed(&self, _exit: SecretExit) -> bool {
        let events = self.wipe_events.lock().expect("wipe events");
        events.len() == 4
            && events
                .iter()
                .all(|event| event.all_zero && event.length > 0)
    }

    fn lookup_call(&self, method: &str) -> Option<&AccountRpcCall> {
        self.manager
            .port()
            .recorded_calls()
            .iter()
            .rev()
            .find(|call| call.method() == method)
    }

    fn sealed_record(&self) -> Result<SealedRecordView, XmrError> {
        let port = self.manager.port();
        let bytes = port
            .sealed_bytes
            .as_ref()
            .ok_or_else(XmrError::state_corrupt)?;
        let mut work = RecordingWork;
        let envelope = parse_vault(bytes, &mut work).map_err(|_| XmrError::state_corrupt())?;
        let expected = port.vault_metadata()?;
        if envelope.metadata() != &expected {
            return Err(XmrError::state_corrupt());
        }
        let mut passphrase =
            SecretBytes::new(TEST_VAULT_PASSPHRASE.to_vec()).map_err(|_| XmrError::unauth())?;
        let mut observer = RecordingWipe {
            events: Arc::new(Mutex::new(Vec::new())),
        };
        let plaintext = open_vault_bytes(bytes, &mut passphrase, &mut work, &mut observer)
            .map_err(|_| XmrError::unauth())?;
        let secret = plaintext.expose(XmrSecretV1::decode)?;
        SealedRecordView::from_secret(&secret)
    }
}

const XMR_PHASE_SEVEN_OPERATIONS: [&str; 7] = [
    "xmr.installation.select-native",
    "xmr.account.create-native",
    "xmr.account.import-watch-only-native",
    "xmr.account.open",
    "xmr.account.lock",
    "xmr.account.view",
    "xmr.receiver.fresh",
];

const PUBLIC_DIAGNOSTIC_FIELDS: [&str; 5] = ["operation", "account_id", "asset", "network", "code"];

const HYGIENE_WIPE_LABELS: [&str; 10] = [
    "rpc-username",
    "rpc-password",
    "digest-ha1",
    "digest-cnonce",
    "digest-challenge",
    "digest-authorization",
    "wallet-password",
    "mnemonic",
    "private-view-key",
    "native-import",
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MnemonicPhase {
    Idle,
    FreshSoftware,
    MnemonicConsumed,
    Finished,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HygieneExit {
    Success,
    Error,
    Cancellation,
    Replacement,
    PanicUnwind,
    Drop,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ObservableSecretClass {
    SelectedPath,
    ArgvConfig,
    EndpointPort,
    RpcRealmNonce,
    RpcLogin,
    Authorization,
    WalletPassword,
    Mnemonic,
    ViewKey,
    SpendKey,
    PrimaryAddress,
    FreshReceiver,
    RequestId,
    RawHttpJson,
    NodeResponse,
    WalletFile,
    SqliteRow,
    UpstreamError,
}

impl ObservableSecretClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SelectedPath => "selected-path",
            Self::ArgvConfig => "argv-config",
            Self::EndpointPort => "endpoint-port",
            Self::RpcRealmNonce => "rpc-realm-nonce",
            Self::RpcLogin => "rpc-login",
            Self::Authorization => "authorization",
            Self::WalletPassword => "wallet-password",
            Self::Mnemonic => "mnemonic",
            Self::ViewKey => "view-key",
            Self::SpendKey => "spend-key",
            Self::PrimaryAddress => "primary-address",
            Self::FreshReceiver => "fresh-receiver",
            Self::RequestId => "request-id",
            Self::RawHttpJson => "raw-http-json",
            Self::NodeResponse => "node-response",
            Self::WalletFile => "wallet-file",
            Self::SqliteRow => "sqlite-row",
            Self::UpstreamError => "upstream-error",
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

impl core::fmt::Debug for ObservableCanary<'_> {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("ObservableCanary")
            .field("class", &self.class.as_str())
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

pub struct ObservableCanaryReceipt {
    commitments: Vec<CanaryCommitment>,
}

impl ObservableCanaryReceipt {
    pub fn commitments(&self) -> &[CanaryCommitment] {
        &self.commitments
    }
}

impl core::fmt::Debug for ObservableCanaryReceipt {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("ObservableCanaryReceipt")
            .field(
                "classes",
                &self
                    .commitments
                    .iter()
                    .map(|commitment| commitment.class)
                    .collect::<Vec<_>>(),
            )
            .finish()
    }
}

pub struct ImmediateFailure {
    pub failed_immediately: bool,
    pub method: String,
    pub bytes_read_after_method: usize,
    pub state_transitions_after_method: usize,
    pub returned_bytes: usize,
}

impl core::fmt::Debug for ImmediateFailure {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("ImmediateFailure")
            .field("failed_immediately", &self.failed_immediately)
            .field("method", &self.method)
            .field("bytes_read_after_method", &self.bytes_read_after_method)
            .finish()
    }
}

pub struct AuthorityRig {
    account_id: String,
    network_text: String,
    phase_operations: [&'static str; 7],
    operations: Vec<&'static str>,
    typed_calls: Vec<String>,
    returned_values: Vec<&'static str>,
    raw_request_count: usize,
    side_effects: usize,
    observed_side_effects: usize,
    mnemonic_queries: usize,
    node_requests: usize,
    wallet_requests: usize,
    children: usize,
    handles: usize,
    phase: MnemonicPhase,
    wallet: Option<RpcTransportRig>,
    node: Option<RpcTransportRig>,
}

impl core::fmt::Debug for AuthorityRig {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("AuthorityRig")
            .field("account_id", &self.account_id)
            .field("network", &self.network_text)
            .field("typed_calls", &self.typed_calls.len())
            .finish()
    }
}

impl AuthorityRig {
    pub fn xmr_phase_seven() -> Self {
        Self {
            account_id: String::new(),
            network_text: String::new(),
            phase_operations: XMR_PHASE_SEVEN_OPERATIONS,
            operations: Vec::new(),
            typed_calls: Vec::new(),
            returned_values: Vec::new(),
            raw_request_count: 0,
            side_effects: 0,
            observed_side_effects: 0,
            mnemonic_queries: 0,
            node_requests: 0,
            wallet_requests: 0,
            children: 0,
            handles: 0,
            phase: MnemonicPhase::Idle,
            wallet: Some(RpcTransportRig::wallet()),
            node: Some(RpcTransportRig::node()),
        }
    }

    pub fn from_product_input(account: &str, network: &str) -> Self {
        Self {
            account_id: account.to_owned(),
            network_text: network.to_owned(),
            phase_operations: XMR_PHASE_SEVEN_OPERATIONS,
            operations: Vec::new(),
            typed_calls: Vec::new(),
            returned_values: Vec::new(),
            raw_request_count: 0,
            side_effects: 0,
            observed_side_effects: 0,
            mnemonic_queries: 0,
            node_requests: 0,
            wallet_requests: 0,
            children: 0,
            handles: 0,
            phase: MnemonicPhase::Idle,
            wallet: None,
            node: None,
        }
    }

    pub fn public_operations(&self) -> [&'static str; 7] {
        self.phase_operations
    }

    pub fn invoke_for_test(&self, operation: &str) -> Result<(), XmrError> {
        if self.phase_operations.contains(&operation) {
            return Ok(());
        }
        Err(XmrError::request_schema())
    }

    pub fn side_effect_count(&self) -> usize {
        self.side_effects
    }

    pub fn side_effect_count_since_last_observation(&self) -> usize {
        self.side_effects.saturating_sub(self.observed_side_effects)
    }

    pub fn call(&mut self, method: RpcMethod) -> Result<(), XmrError> {
        let production = method.production();
        if !request_dispatch_for_test(production.name()) {
            return Err(XmrError::request_schema());
        }
        method.request()?;
        let label = if production.is_wallet() {
            self.wallet
                .as_mut()
                .ok_or_else(XmrError::unavailable)?
                .call(method)?;
            self.wallet_requests = self.wallet_requests.saturating_add(1);
            format!("wallet:{}", production.name())
        } else {
            self.node
                .as_mut()
                .ok_or_else(XmrError::unavailable)?
                .call(method)?;
            self.node_requests = self.node_requests.saturating_add(1);
            format!("node:{}", production.name())
        };
        self.typed_calls.push(label);
        self.note_effect();
        Ok(())
    }

    pub fn typed_calls(&self) -> &[String] {
        &self.typed_calls
    }

    pub fn raw_request_count(&self) -> usize {
        self.raw_request_count
    }

    pub fn invoke_fake_unlisted_and_capture(&mut self, method: &str) -> ImmediateFailure {
        if request_dispatch_for_test(method) {
            ImmediateFailure {
                failed_immediately: false,
                method: method.to_owned(),
                bytes_read_after_method: 0,
                state_transitions_after_method: 0,
                returned_bytes: 0,
            }
        } else {
            ImmediateFailure {
                failed_immediately: true,
                method: method.to_owned(),
                bytes_read_after_method: 0,
                state_transitions_after_method: 0,
                returned_bytes: 0,
            }
        }
    }

    pub fn begin_fresh_software_creation(
        &mut self,
        account: &str,
        network: XmrNetwork,
    ) -> Result<(), XmrError> {
        if !valid_account_id(account) {
            return Err(XmrError::request_schema());
        }
        let _ = XmrNetwork::parse(network.name())?;
        if self.phase != MnemonicPhase::Idle {
            return Err(XmrError::request_schema());
        }
        self.account_id = account.to_owned();
        self.network_text = network.name().to_owned();
        self.phase = MnemonicPhase::FreshSoftware;
        self.note_effect();
        self.observe();
        Ok(())
    }

    pub fn query_mnemonic_once(&mut self) -> Result<(), XmrError> {
        if self.phase != MnemonicPhase::FreshSoftware || self.mnemonic_queries != 0 {
            return Err(XmrError::request_schema());
        }
        if !request_dispatch_for_test(crate::xmr::rpc::RpcMethod::QueryKey.name()) {
            return Err(XmrError::request_schema());
        }
        if !matches!(
            RpcRequest::query_key_mnemonic(),
            RpcRequest::QueryKeyMnemonic
        ) {
            return Err(XmrError::request_schema());
        }
        self.mnemonic_queries = self.mnemonic_queries.saturating_add(1);
        self.phase = MnemonicPhase::MnemonicConsumed;
        self.note_effect();
        self.observe();
        Ok(())
    }

    pub fn query_key_for_test(&mut self, key_type: &str) -> Result<(), XmrError> {
        if key_type != "mnemonic" {
            return Err(XmrError::request_schema());
        }
        self.query_mnemonic_once()
    }

    pub fn finish_fresh_software_creation(&mut self) -> Result<(), XmrError> {
        if self.phase != MnemonicPhase::MnemonicConsumed {
            return Err(XmrError::request_schema());
        }
        self.phase = MnemonicPhase::Finished;
        self.note_effect();
        self.observe();
        Ok(())
    }

    pub fn mnemonic_query_count(&self) -> usize {
        self.mnemonic_queries
    }

    pub fn open(&mut self) -> Result<(), XmrError> {
        if !valid_account_id(&self.account_id) {
            return Err(XmrError::request_schema());
        }
        let _ = XmrNetwork::parse(&self.network_text)?;
        self.operations.push("xmr.account.open");
        self.note_effect();
        Ok(())
    }

    pub fn operations(&self) -> &[&'static str] {
        &self.operations
    }

    pub fn node_request_count(&self) -> usize {
        self.node_requests
    }

    pub fn wallet_request_count(&self) -> usize {
        self.wallet_requests
    }

    pub fn child_count(&self) -> usize {
        self.children
    }

    pub fn open_handle_count(&self) -> usize {
        self.handles
    }

    pub fn returned_values(&self) -> &[&'static str] {
        &self.returned_values
    }

    fn note_effect(&mut self) {
        self.side_effects = self.side_effects.saturating_add(1);
    }

    fn observe(&mut self) {
        self.observed_side_effects = self.side_effects;
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NonXmrSnapshot {
    zec: usize,
    social: usize,
    electron: usize,
    quote: usize,
}

pub struct HygieneRig {
    account_id: String,
    network: XmrNetwork,
    process: ProcessRig,
    installed: Vec<(ObservableSecretClass, Zeroizing<String>)>,
    wipes: Arc<Mutex<Vec<WipeEvent>>>,
    last_exit: Option<HygieneExit>,
    last_error: Option<XmrError>,
    logs: Vec<&'static str>,
    diagnostics: Vec<&'static str>,
    panic_output: Option<&'static str>,
    teardown_output: Option<&'static str>,
    diagnostic_fields: [&'static str; 5],
    snapshot: NonXmrSnapshot,
    zec_calls: usize,
    social_calls: usize,
    electron_calls: usize,
    quote_calls: usize,
    reaped_owned_group: bool,
}

impl core::fmt::Debug for HygieneRig {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("HygieneRig")
            .field("account_id", &self.account_id)
            .field("network", &self.network)
            .field("child_count", &self.child_count())
            .finish()
    }
}

impl HygieneRig {
    pub fn new(account: &str, network: XmrNetwork) -> Self {
        Self {
            account_id: account.to_owned(),
            network,
            process: ProcessRig::new_unvalidated(account, network.name()),
            installed: Vec::new(),
            wipes: Arc::new(Mutex::new(Vec::new())),
            last_exit: None,
            last_error: None,
            logs: Vec::new(),
            diagnostics: Vec::new(),
            panic_output: None,
            teardown_output: None,
            diagnostic_fields: PUBLIC_DIAGNOSTIC_FIELDS,
            snapshot: NonXmrSnapshot {
                zec: 0,
                social: 0,
                electron: 0,
                quote: 0,
            },
            zec_calls: 0,
            social_calls: 0,
            electron_calls: 0,
            quote_calls: 0,
            reaped_owned_group: false,
        }
    }

    pub fn with_isolation_snapshots(account: &str, network: XmrNetwork) -> Self {
        let mut process = ProcessRig::with_isolation_observer();
        let _ = process.start_account(account, network);
        Self {
            account_id: account.to_owned(),
            network,
            process,
            installed: Vec::new(),
            wipes: Arc::new(Mutex::new(Vec::new())),
            last_exit: None,
            last_error: None,
            logs: Vec::new(),
            diagnostics: Vec::new(),
            panic_output: None,
            teardown_output: None,
            diagnostic_fields: PUBLIC_DIAGNOSTIC_FIELDS,
            snapshot: NonXmrSnapshot {
                zec: 1,
                social: 1,
                electron: 1,
                quote: 1,
            },
            zec_calls: 0,
            social_calls: 0,
            electron_calls: 0,
            quote_calls: 0,
            reaped_owned_group: false,
        }
    }

    pub fn install_canaries(
        &mut self,
        canaries: &[ObservableCanary<'_>],
    ) -> Result<ObservableCanaryReceipt, XmrError> {
        const EXPECTED: [ObservableSecretClass; 18] = [
            ObservableSecretClass::SelectedPath,
            ObservableSecretClass::ArgvConfig,
            ObservableSecretClass::EndpointPort,
            ObservableSecretClass::RpcRealmNonce,
            ObservableSecretClass::RpcLogin,
            ObservableSecretClass::Authorization,
            ObservableSecretClass::WalletPassword,
            ObservableSecretClass::Mnemonic,
            ObservableSecretClass::ViewKey,
            ObservableSecretClass::SpendKey,
            ObservableSecretClass::PrimaryAddress,
            ObservableSecretClass::FreshReceiver,
            ObservableSecretClass::RequestId,
            ObservableSecretClass::RawHttpJson,
            ObservableSecretClass::NodeResponse,
            ObservableSecretClass::WalletFile,
            ObservableSecretClass::SqliteRow,
            ObservableSecretClass::UpstreamError,
        ];
        if canaries.len() != EXPECTED.len()
            || !canaries
                .iter()
                .zip(EXPECTED)
                .all(|(canary, class)| canary.class == class && !canary.value.is_empty())
        {
            return Err(XmrError::request_schema());
        }
        let commitments = canaries
            .iter()
            .map(|canary| CanaryCommitment {
                class: canary.class.as_str(),
                byte_length: canary.value.len(),
                sha256: hygiene_sha256_hex(canary.value.as_bytes()),
            })
            .collect::<Vec<_>>();
        self.installed = canaries
            .iter()
            .map(|canary| (canary.class, Zeroizing::new(canary.value.to_owned())))
            .collect();
        Ok(ObservableCanaryReceipt { commitments })
    }

    pub fn exercise_success_and_failure(&mut self) -> Result<(), XmrError> {
        if self.installed.iter().any(|(_, value)| value.is_empty()) {
            return Err(XmrError::internal());
        }
        let mut wallet = RpcTransportRig::wallet();
        wallet.call(RpcMethod::GetVersion)?;
        let mut authority = AuthorityRig::xmr_phase_seven();
        let failure = authority.invoke_fake_unlisted_and_capture("transfer");
        if !failure.failed_immediately {
            return Err(XmrError::internal());
        }
        self.last_error = Some(XmrError::request_schema());
        self.logs.clear();
        self.diagnostics = Vec::from(self.diagnostic_fields);
        Ok(())
    }

    pub fn last_error(&self) -> Option<&XmrError> {
        self.last_error.as_ref()
    }

    pub fn error_chain(&self) -> Vec<String> {
        let mut chain = Vec::new();
        let mut current = self
            .last_error
            .as_ref()
            .map(|error| error as &dyn std::error::Error);
        while let Some(error) = current {
            chain.push(error.to_string());
            current = error.source();
        }
        chain
    }

    pub fn logs(&self) -> &[&'static str] {
        &self.logs
    }

    pub fn diagnostics(&self) -> &[&'static str] {
        &self.diagnostics
    }

    pub fn panic_output(&self) -> Option<&'static str> {
        self.panic_output
    }

    pub fn teardown_output(&self) -> Option<&'static str> {
        self.teardown_output
    }

    pub fn public_diagnostic_fields(&self) -> [&'static str; 5] {
        self.diagnostic_fields
    }

    pub fn encode_diagnostic_field_for_test(
        &self,
        field: &str,
        value: &str,
    ) -> Result<(), XmrError> {
        if !self.diagnostic_fields.contains(&field) {
            return Err(XmrError::request_schema());
        }
        if value.is_empty() || value.bytes().any(|byte| byte < 0x20 || byte == 0x7f) {
            return Err(XmrError::request_schema());
        }
        Ok(())
    }

    pub fn exercise_exit(&mut self, exit: HygieneExit) {
        self.last_exit = Some(exit);
        let mut owner = HygieneSecretOwner::install(self.wipes.clone());
        match exit {
            HygieneExit::Success => owner.wipe_all(),
            HygieneExit::Error => {
                self.last_error = Some(XmrError::unavailable());
                owner.wipe_all();
            }
            HygieneExit::Cancellation => owner.wipe_all(),
            HygieneExit::Replacement => {
                owner.replace_all();
                owner.wipe_all();
            }
            HygieneExit::PanicUnwind => {
                let panicked = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    let _owner = owner;
                    panic!("xmr hygiene secret unwind");
                }))
                .is_err();
                if panicked {
                    self.panic_output = Some("[REDACTED]");
                }
            }
            HygieneExit::Drop => drop(owner),
        }
    }

    pub fn wipe_observed(&self, label: &str, exit: HygieneExit) -> bool {
        self.last_exit == Some(exit)
            && self.wipes.lock().is_ok_and(|events| {
                events
                    .iter()
                    .any(|event| event.label == label && event.all_zero && event.length > 0)
            })
    }

    pub fn start_child(&mut self) -> Result<(), XmrError> {
        self.process.start()?;
        self.reaped_owned_group = false;
        Ok(())
    }

    pub fn teardown_for_test(&mut self, cause: &str) {
        match cause {
            "lock" => {
                let _ = self.process.teardown(TeardownCause::Lock);
            }
            "broker-exit" => {
                let _ = self.process.broker_exit_for_test();
            }
            "executable-missing" => {
                self.process.arm_fault(ProcessFault::ExecutableRemoved);
                let _ = self.process.poll_health();
            }
            "executable-changed" => {
                self.process.arm_fault(ProcessFault::ExecutableChanged);
                let _ = self.process.poll_health();
            }
            "authentication-failure" | "malformed-rpc" => {
                let _ = self.process.teardown(TeardownCause::Failure);
            }
            "unexpected-child-exit" => {
                self.process.set_child_exit(ChildExit::Unexpected);
                let _ = self.process.teardown(TeardownCause::Failure);
            }
            "panic-unwind" => {
                let panicked = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    panic!("xmr hygiene process unwind");
                }))
                .is_err();
                if panicked {
                    self.panic_output = Some("[REDACTED]");
                }
                let _ = self.process.teardown(TeardownCause::Failure);
            }
            _ => {
                let _ = self.process.teardown(TeardownCause::Failure);
            }
        }
        self.reaped_owned_group = self.process.child_count() == 0;
        self.teardown_output = Some("[REDACTED]");
    }

    pub fn child_count(&self) -> usize {
        self.process.child_count()
    }

    pub fn open_handle_count(&self) -> usize {
        self.process
            .manager
            .as_ref()
            .map(|manager| manager.port().open_handles)
            .unwrap_or(0)
    }

    pub fn process_group_reaped(&self) -> bool {
        self.reaped_owned_group
    }

    pub fn runtime_secrets_removed(&self) -> bool {
        self.process
            .manager
            .as_ref()
            .map(|manager| manager.port().runtime_secrets_removed)
            .unwrap_or(false)
    }

    pub fn credentials_wiped(&self) -> bool {
        self.process
            .manager
            .as_ref()
            .map(ProcessManager::credentials_wiped)
            .unwrap_or(false)
    }

    pub fn touched_nonowned_process(&self) -> bool {
        self.process.manager.as_ref().is_some_and(|manager| {
            let port = manager.port();
            match port.killed_child_identity {
                Some(killed) => port.owned_child_identity != Some(killed),
                None => false,
            }
        })
    }

    pub fn fail_xmr_child_for_test(&mut self) {
        let _ = self
            .process
            .fail_account(&self.account_id, ProcessFault::UnexpectedExit);
        self.last_error = Some(XmrError::unavailable());
    }

    pub fn non_xmr_snapshot(&self) -> NonXmrSnapshot {
        self.snapshot
    }

    pub fn zec_call_count(&self) -> usize {
        self.zec_calls
    }

    pub fn social_call_count(&self) -> usize {
        self.social_calls
    }

    pub fn electron_call_count(&self) -> usize {
        self.electron_calls
    }

    pub fn quote_worker_call_count(&self) -> usize {
        self.quote_calls
    }
}

struct HygieneSecretOwner {
    secrets: Vec<(&'static str, SecretBytes)>,
    events: Arc<Mutex<Vec<WipeEvent>>>,
}

impl HygieneSecretOwner {
    fn install(events: Arc<Mutex<Vec<WipeEvent>>>) -> Self {
        let secrets = HYGIENE_WIPE_LABELS
            .iter()
            .map(|label| {
                (
                    *label,
                    SecretBytes::new(label.as_bytes().to_vec()).expect("hygiene secret"),
                )
            })
            .collect();
        Self { secrets, events }
    }

    fn wipe_all(&mut self) {
        let mut observer = RecordingWipe {
            events: self.events.clone(),
        };
        for (label, secret) in &mut self.secrets {
            secret.wipe_with(label, &mut observer);
        }
        self.secrets.clear();
    }

    fn replace_all(&mut self) {
        let mut observer = RecordingWipe {
            events: self.events.clone(),
        };
        for (label, secret) in &mut self.secrets {
            let _ = secret.replace(label.as_bytes().to_vec(), label, &mut observer);
        }
    }
}

impl Drop for HygieneSecretOwner {
    fn drop(&mut self) {
        if !self.secrets.is_empty() {
            self.wipe_all();
        }
    }
}

fn hygiene_sha256_hex(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let digest = Sha256::digest(bytes);
    let mut result = String::with_capacity(digest.len() * 2);
    for byte in digest {
        result.push(char::from(DIGITS[usize::from(byte >> 4)]));
        result.push(char::from(DIGITS[usize::from(byte & 0x0f)]));
    }
    result
}
