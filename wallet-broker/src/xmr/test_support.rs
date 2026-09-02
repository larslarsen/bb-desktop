use std::ffi::OsStr;
use std::path::Path;

use crate::xmr::distribution::{
    DistributionManager, DistributionPort, EXECUTABLE_BYTES, EXECUTABLE_SHA256,
    ExecutableObservation, HashResult, RecordIntegrity, SelectedFileKind, SelectionRecord,
    VERIFIED_VERSION, VerificationStep, decode_digest,
};
pub use crate::xmr::model::HostPlatform;
use crate::xmr::model::XmrError;

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
