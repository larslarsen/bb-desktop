use std::collections::VecDeque;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use crate::xmr::distribution::{
    DistributionManager, DistributionPort, EXECUTABLE_BYTES, EXECUTABLE_SHA256,
    ExecutableObservation, HashResult, RecordIntegrity, SelectedFileKind, SelectionRecord,
    VERIFIED_VERSION, VerificationStep, decode_digest,
};
use crate::xmr::model::XmrError;
pub use crate::xmr::model::{HostPlatform, XmrNetwork};
use crate::xmr::process::{
    DerivedPaths, EntropyOrigin, ProcessCoordinator, ProcessManager, ProcessPort,
    ReadinessObservation, ReservationFailure, WalletRpcProcessPlan, next_os_port_for_test_port,
};

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
