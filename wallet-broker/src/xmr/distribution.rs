use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use crate::xmr::model::{HostPlatform, XmrError};

pub const ARCHIVE_NAME: &str = "monero-gui-linux-x64-v0.18.5.2.tar.bz2";
pub const ARCHIVE_SHA256: &str = "294017a5aa1ee86420b0c62fe4046000f42438375a8559d9ff55e41e5c6cbbcd";
pub const ARCHIVE_MEMBER: &str = "monero-gui-v0.18.5.2/extras/monero-wallet-rpc";
pub const EXECUTABLE_BYTES: u64 = 29_026_368;
pub const EXECUTABLE_SHA256: &str =
    "c1e3aff7c72837e6f29045c439b772a82b5cd7324c8b831fa825a6ce2019a656";
pub const VERIFIED_VERSION: &str = "Monero 'Fluorine Fermi' (v0.18.5.1-release)";
pub const MONEROD_ARCHIVE_MEMBER: &str = "monero-gui-v0.18.5.2/monerod";
pub const MONEROD_BYTES: u64 = 24_112_840;
pub const MONEROD_SHA256: &str = "9b3b2676ea7868c1a7186feea9569c2cf7683ae79d2fcc769c846a91c810a1f5";
pub const LINUX_PIN_ID: &str = "monero-gui-linux-x64-v0.18.5.2";
pub const MAX_SELECTED_PATH_BYTES: usize = 4_096;

const SELECTION_SCHEMA_VERSION: u32 = 1;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum VerificationStep {
    Lstat,
    Regular,
    Length,
    Executable,
    Sha256,
    Version,
    Record,
}

impl VerificationStep {
    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::Lstat => "lstat",
            Self::Regular => "regular",
            Self::Length => "length",
            Self::Executable => "executable",
            Self::Sha256 => "sha256",
            Self::Version => "version",
            Self::Record => "record",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SelectedFileKind {
    Regular,
    Symlink,
    Directory,
    Fifo,
    Socket,
    BlockDevice,
    CharacterDevice,
    Other,
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub(crate) struct ExecutableObservation {
    pub(crate) kind: SelectedFileKind,
    pub(crate) device: u64,
    pub(crate) inode: u64,
    pub(crate) length: u64,
    pub(crate) modified_seconds: i64,
    pub(crate) modified_nanoseconds: i64,
    pub(crate) mode: u32,
    pub(crate) owner: u32,
    pub(crate) group: u32,
}

impl ExecutableObservation {
    fn same_selected_file(self, other: Self) -> bool {
        self.kind == SelectedFileKind::Regular
            && other.kind == SelectedFileKind::Regular
            && self.device == other.device
            && self.inode == other.inode
            && self.length == other.length
            && self.modified_seconds == other.modified_seconds
            && self.modified_nanoseconds == other.modified_nanoseconds
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum RecordIntegrity {
    Complete,
    Partial,
    UnknownField,
    Symlink,
}

#[derive(Clone, Eq, PartialEq)]
pub struct SelectionRecord {
    pub schema_version: u32,
    pub platform_pin_id: String,
    pub selected_path: String,
    pub mode: u32,
    pub was_atomically_replaced: bool,
    pub parent_was_scanned: bool,
    integrity: RecordIntegrity,
}

impl SelectionRecord {
    pub(crate) fn complete(selected_path: &str) -> Self {
        Self {
            schema_version: SELECTION_SCHEMA_VERSION,
            platform_pin_id: LINUX_PIN_ID.to_owned(),
            selected_path: selected_path.to_owned(),
            mode: 0o600,
            was_atomically_replaced: true,
            parent_was_scanned: false,
            integrity: RecordIntegrity::Complete,
        }
    }

    pub(crate) fn set_integrity(&mut self, integrity: RecordIntegrity) {
        self.integrity = integrity;
    }
}

pub struct VerifiedExecutable {
    selected_path: PathBuf,
    observation: ExecutableObservation,
}

impl VerifiedExecutable {
    pub(crate) fn selected_path(&self) -> &Path {
        &self.selected_path
    }

    pub(crate) fn observation(&self) -> ExecutableObservation {
        self.observation
    }
}

pub struct InstallationVerifier {
    platform: HostPlatform,
}

impl InstallationVerifier {
    pub fn linux_x86_64() -> Self {
        Self {
            platform: HostPlatform::current(),
        }
    }

    pub fn verify_selected(&self, selected_path: &Path) -> Result<VerifiedExecutable, XmrError> {
        if !self.platform.supports_distribution() {
            return Err(XmrError::unavailable());
        }

        #[cfg(target_os = "linux")]
        {
            let port = linux::LinuxDistributionPort::verification_only();
            let mut manager = DistributionManager::new(self.platform, port);
            manager.verify_selected_path(selected_path)
        }

        #[cfg(not(target_os = "linux"))]
        {
            let _ = selected_path;
            Err(XmrError::unavailable())
        }
    }
}

pub(crate) struct HashResult {
    pub(crate) digest: [u8; 32],
    pub(crate) bytes_read: u64,
}

pub(crate) trait DistributionPort {
    fn note_step(&mut self, step: VerificationStep);
    fn lstat(&mut self, selected_path: &Path) -> Result<ExecutableObservation, XmrError>;
    fn effective_user_can_execute(
        &mut self,
        observation: &ExecutableObservation,
    ) -> Result<bool, XmrError>;
    fn hash_executable(
        &mut self,
        selected_path: &Path,
        observation: &ExecutableObservation,
    ) -> Result<HashResult, XmrError>;
    fn probe_version(&mut self, selected_path: &Path) -> Result<String, XmrError>;
    fn persist_selection(&mut self, selected_path: &str) -> Result<SelectionRecord, XmrError>;
    fn load_selection(&mut self) -> Result<SelectionRecord, XmrError>;
}

struct ValidatedSelectedPath {
    text: String,
    path: PathBuf,
}

impl ValidatedSelectedPath {
    fn from_os_str(value: &OsStr) -> Result<Self, XmrError> {
        let text = value.to_str().ok_or_else(XmrError::schema)?;
        if text.is_empty()
            || text.len() > MAX_SELECTED_PATH_BYTES
            || text.as_bytes().contains(&0)
            || !Path::new(text).is_absolute()
        {
            return Err(XmrError::schema());
        }
        Ok(Self {
            text: text.to_owned(),
            path: PathBuf::from(text),
        })
    }
}

pub(crate) struct DistributionManager<P: DistributionPort> {
    platform: HostPlatform,
    port: P,
    selection: Option<SelectionRecord>,
    observation: Option<ExecutableObservation>,
    stop_required: bool,
    teardown_reason: Option<&'static str>,
}

impl<P: DistributionPort> DistributionManager<P> {
    pub(crate) fn new(platform: HostPlatform, port: P) -> Self {
        Self {
            platform,
            port,
            selection: None,
            observation: None,
            stop_required: false,
            teardown_reason: None,
        }
    }

    pub(crate) fn port(&self) -> &P {
        &self.port
    }

    pub(crate) fn port_mut(&mut self) -> &mut P {
        &mut self.port
    }

    pub(crate) fn selection(&self) -> Option<&SelectionRecord> {
        self.selection.as_ref()
    }

    pub(crate) fn stop_required(&self) -> bool {
        self.stop_required
    }

    pub(crate) fn teardown_reason(&self) -> Option<&'static str> {
        self.teardown_reason
    }

    pub(crate) fn enroll(&mut self, selected_path: &str) -> Result<(), XmrError> {
        self.enroll_os(OsStr::new(selected_path))
    }

    pub(crate) fn enroll_os(&mut self, selected_path: &OsStr) -> Result<(), XmrError> {
        self.require_supported_platform()?;
        self.selection = None;
        self.observation = None;
        let selected_path = ValidatedSelectedPath::from_os_str(selected_path)?;
        let verified = self.verify(&selected_path)?;
        self.port.note_step(VerificationStep::Record);
        let record = self.port.persist_selection(&selected_path.text)?;
        validate_selection_record(&record, &selected_path.text)?;
        self.selection = Some(record);
        self.observation = Some(verified.observation());
        self.stop_required = false;
        self.teardown_reason = None;
        Ok(())
    }

    pub(crate) fn authorize_launch(&mut self) -> Result<VerifiedExecutable, XmrError> {
        self.require_supported_platform()?;
        let selected_path = self
            .selection
            .as_ref()
            .ok_or_else(XmrError::unavailable)?
            .selected_path
            .clone();
        let selected_path = ValidatedSelectedPath::from_os_str(OsStr::new(&selected_path))?;
        let verified = self.verify(&selected_path)?;
        self.observation = Some(verified.observation());
        self.stop_required = false;
        self.teardown_reason = None;
        Ok(verified)
    }

    pub(crate) fn poll_selected_file(&mut self) -> Result<(), XmrError> {
        self.require_supported_platform()?;
        let selected_path = self
            .selection
            .as_ref()
            .ok_or_else(XmrError::unavailable)?
            .selected_path
            .clone();
        let expected = self.observation.ok_or_else(XmrError::unavailable)?;
        let selected_path = ValidatedSelectedPath::from_os_str(OsStr::new(&selected_path))?;
        self.port.note_step(VerificationStep::Lstat);
        let current = self.port.lstat(&selected_path.path);
        if !matches!(current, Ok(observed) if expected.same_selected_file(observed)) {
            self.stop_required = true;
            self.teardown_reason = Some("selected-executable-changed");
            return Err(XmrError::unavailable());
        }
        Ok(())
    }

    pub(crate) fn restore_selection(&mut self) -> Result<(), XmrError> {
        self.require_supported_platform()?;
        self.selection = None;
        self.observation = None;
        let record = self.port.load_selection()?;
        validate_selection_record(&record, &record.selected_path)?;
        ValidatedSelectedPath::from_os_str(OsStr::new(&record.selected_path))
            .map_err(|_| XmrError::state_corrupt())?;
        self.selection = Some(record);
        Ok(())
    }

    fn require_supported_platform(&self) -> Result<(), XmrError> {
        if self.platform.supports_distribution() {
            Ok(())
        } else {
            Err(XmrError::unavailable())
        }
    }

    fn verify_selected_path(
        &mut self,
        selected_path: &Path,
    ) -> Result<VerifiedExecutable, XmrError> {
        self.require_supported_platform()?;
        let selected_path = ValidatedSelectedPath::from_os_str(selected_path.as_os_str())?;
        self.verify(&selected_path)
    }

    fn verify(
        &mut self,
        selected_path: &ValidatedSelectedPath,
    ) -> Result<VerifiedExecutable, XmrError> {
        self.port.note_step(VerificationStep::Lstat);
        let observation = self.port.lstat(&selected_path.path)?;
        if observation.kind != SelectedFileKind::Regular {
            return Err(XmrError::protocol_incompatible());
        }

        self.port.note_step(VerificationStep::Regular);
        self.port.note_step(VerificationStep::Length);
        if observation.length != EXECUTABLE_BYTES {
            return Err(XmrError::protocol_incompatible());
        }

        self.port.note_step(VerificationStep::Executable);
        if !self.port.effective_user_can_execute(&observation)? {
            return Err(XmrError::protocol_incompatible());
        }

        self.port.note_step(VerificationStep::Sha256);
        let hashed = self
            .port
            .hash_executable(&selected_path.path, &observation)?;
        if hashed.bytes_read != EXECUTABLE_BYTES || !constant_time_expected_digest(&hashed.digest) {
            return Err(XmrError::protocol_incompatible());
        }

        self.port.note_step(VerificationStep::Version);
        if self.port.probe_version(&selected_path.path)? != VERIFIED_VERSION {
            return Err(XmrError::protocol_incompatible());
        }

        Ok(VerifiedExecutable {
            selected_path: selected_path.path.clone(),
            observation,
        })
    }
}

fn validate_selection_record(
    record: &SelectionRecord,
    selected_path: &str,
) -> Result<(), XmrError> {
    if record.integrity != RecordIntegrity::Complete
        || record.schema_version != SELECTION_SCHEMA_VERSION
        || record.platform_pin_id != LINUX_PIN_ID
        || record.selected_path != selected_path
        || record.mode != 0o600
        || !record.was_atomically_replaced
        || record.parent_was_scanned
    {
        return Err(XmrError::state_corrupt());
    }
    Ok(())
}

fn constant_time_expected_digest(actual: &[u8; 32]) -> bool {
    let Some(expected) = decode_digest(EXECUTABLE_SHA256) else {
        return false;
    };
    actual
        .iter()
        .zip(expected.iter())
        .fold(0u8, |difference, (left, right)| {
            difference | (*left ^ *right)
        })
        == 0
}

pub(crate) fn decode_digest(value: &str) -> Option<[u8; 32]> {
    if value.len() != 64 {
        return None;
    }
    let mut decoded = [0u8; 32];
    for (index, pair) in value.as_bytes().as_chunks::<2>().0.iter().enumerate() {
        decoded[index] = (hex_nibble(pair[0])? << 4) | hex_nibble(pair[1])?;
    }
    Some(decoded)
}

fn hex_nibble(value: u8) -> Option<u8> {
    match value {
        b'0'..=b'9' => Some(value - b'0'),
        b'a'..=b'f' => Some(value - b'a' + 10),
        _ => None,
    }
}

#[cfg(target_os = "linux")]
mod linux {
    use std::fs::{self, File, OpenOptions};
    use std::io::{Read, Write};
    use std::os::unix::fs::{FileTypeExt, MetadataExt, OpenOptionsExt, PermissionsExt};
    use std::os::unix::process::CommandExt;
    use std::path::{Path, PathBuf};
    use std::process::{Command, Stdio};
    use std::thread;
    use std::time::{Duration, Instant};

    use sha2::{Digest, Sha256};

    use super::{
        DistributionPort, ExecutableObservation, HashResult, LINUX_PIN_ID,
        SELECTION_SCHEMA_VERSION, SelectedFileKind, SelectionRecord, VERIFIED_VERSION,
        VerificationStep,
    };
    use crate::xmr::model::XmrError;

    const RECORD_MAGIC: &[u8; 8] = b"BBXMRSEL";
    const RECORD_FILE: &str = "xmr-installation-v1";
    const RECORD_TEMP_FILE: &str = ".xmr-installation-v1.tmp";
    const MAX_RECORD_BYTES: usize = 8 + 4 + 2 + 128 + 2 + super::MAX_SELECTED_PATH_BYTES;
    const MAX_PROCESS_STATUS_BYTES: usize = 64 * 1_024;
    const MAX_VERSION_OUTPUT_BYTES: usize = 4 * 1_024;
    const VERSION_PROBE_TIMEOUT: Duration = Duration::from_secs(5);

    pub struct LinuxDistributionPort {
        data_directory: PathBuf,
    }

    impl LinuxDistributionPort {
        pub fn new(data_directory: impl AsRef<Path>) -> Self {
            Self {
                data_directory: data_directory.as_ref().to_path_buf(),
            }
        }

        pub(super) fn verification_only() -> Self {
            Self {
                data_directory: PathBuf::new(),
            }
        }

        fn ensure_private_directory(&self) -> Result<(), XmrError> {
            let metadata = fs::symlink_metadata(&self.data_directory)
                .map_err(|_| XmrError::state_corrupt())?;
            if !metadata.file_type().is_dir()
                || metadata.permissions().mode() & 0o777 != 0o700
                || metadata.uid() != effective_identity()?.user
            {
                return Err(XmrError::state_corrupt());
            }
            Ok(())
        }

        fn record_path(&self) -> PathBuf {
            self.data_directory.join(RECORD_FILE)
        }

        fn temporary_record_path(&self) -> PathBuf {
            self.data_directory.join(RECORD_TEMP_FILE)
        }

        fn write_and_load_record(&self, selected_path: &str) -> Result<SelectionRecord, XmrError> {
            self.ensure_private_directory()?;
            let active = self.record_path();
            let temporary = self.temporary_record_path();
            if fs::symlink_metadata(&temporary).is_ok() {
                return Err(XmrError::internal());
            }
            let bytes = encode_record(selected_path)?;
            let mut file = OpenOptions::new()
                .write(true)
                .create_new(true)
                .mode(0o600)
                .open(&temporary)
                .map_err(|_| XmrError::internal())?;
            let result = (|| {
                file.set_permissions(fs::Permissions::from_mode(0o600))
                    .map_err(|_| XmrError::internal())?;
                file.write_all(&bytes).map_err(|_| XmrError::internal())?;
                file.sync_all().map_err(|_| XmrError::internal())?;
                drop(file);
                fs::rename(&temporary, &active).map_err(|_| XmrError::internal())?;
                File::open(&self.data_directory)
                    .and_then(|directory| directory.sync_all())
                    .map_err(|_| XmrError::internal())?;
                self.load_record()
            })();
            if result.is_err() {
                let _ = fs::remove_file(&temporary);
            }
            result
        }

        fn load_record(&self) -> Result<SelectionRecord, XmrError> {
            let path = self.record_path();
            let metadata = fs::symlink_metadata(&path).map_err(|_| XmrError::state_corrupt())?;
            if !metadata.file_type().is_file()
                || metadata.permissions().mode() & 0o777 != 0o600
                || metadata.len() > MAX_RECORD_BYTES as u64
            {
                return Err(XmrError::state_corrupt());
            }
            let mut file = File::open(&path).map_err(|_| XmrError::state_corrupt())?;
            let opened = file.metadata().map_err(|_| XmrError::state_corrupt())?;
            if metadata.dev() != opened.dev()
                || metadata.ino() != opened.ino()
                || metadata.len() != opened.len()
            {
                return Err(XmrError::state_corrupt());
            }
            let bytes =
                read_bounded(&mut file, MAX_RECORD_BYTES).map_err(|_| XmrError::state_corrupt())?;
            let selected_path = decode_record(&bytes)?;
            let mut record = SelectionRecord::complete(&selected_path);
            record.mode = metadata.permissions().mode() & 0o777;
            Ok(record)
        }
    }

    impl DistributionPort for LinuxDistributionPort {
        fn note_step(&mut self, _step: VerificationStep) {}

        fn lstat(&mut self, selected_path: &Path) -> Result<ExecutableObservation, XmrError> {
            let metadata = fs::symlink_metadata(selected_path).map_err(|error| {
                if error.kind() == std::io::ErrorKind::NotFound {
                    XmrError::unavailable()
                } else {
                    XmrError::protocol_incompatible()
                }
            })?;
            let kind = selected_file_kind(&metadata.file_type());
            Ok(ExecutableObservation {
                kind,
                device: metadata.dev(),
                inode: metadata.ino(),
                length: metadata.len(),
                modified_seconds: metadata.mtime(),
                modified_nanoseconds: metadata.mtime_nsec(),
                mode: metadata.permissions().mode(),
                owner: metadata.uid(),
                group: metadata.gid(),
            })
        }

        fn effective_user_can_execute(
            &mut self,
            observation: &ExecutableObservation,
        ) -> Result<bool, XmrError> {
            let identity = effective_identity()?;
            let execute_bits = observation.mode & 0o111;
            if identity.user == 0 {
                return Ok(execute_bits != 0);
            }
            if identity.user == observation.owner {
                return Ok(observation.mode & 0o100 != 0);
            }
            if identity.group == observation.group || identity.groups.contains(&observation.group) {
                return Ok(observation.mode & 0o010 != 0);
            }
            Ok(observation.mode & 0o001 != 0)
        }

        fn hash_executable(
            &mut self,
            selected_path: &Path,
            observation: &ExecutableObservation,
        ) -> Result<HashResult, XmrError> {
            let mut file = File::open(selected_path).map_err(|_| XmrError::unavailable())?;
            let metadata = file
                .metadata()
                .map_err(|_| XmrError::protocol_incompatible())?;
            let opened = observation_from_metadata(&metadata);
            if !observation.same_selected_file(opened) {
                return Err(XmrError::protocol_incompatible());
            }
            let mut hasher = Sha256::new();
            let mut buffer = [0u8; 64 * 1_024];
            let mut bytes_read = 0u64;
            loop {
                let count = file
                    .read(&mut buffer)
                    .map_err(|_| XmrError::protocol_incompatible())?;
                if count == 0 {
                    break;
                }
                bytes_read = bytes_read
                    .checked_add(count as u64)
                    .ok_or_else(XmrError::protocol_incompatible)?;
                if bytes_read > super::EXECUTABLE_BYTES {
                    return Err(XmrError::protocol_incompatible());
                }
                hasher.update(&buffer[..count]);
            }
            Ok(HashResult {
                digest: hasher.finalize().into(),
                bytes_read,
            })
        }

        fn probe_version(&mut self, selected_path: &Path) -> Result<String, XmrError> {
            let mut child = Command::new(selected_path)
                .arg0("monero-wallet-rpc")
                .arg("--version")
                .env_clear()
                .stdin(Stdio::null())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .map_err(|_| XmrError::protocol_incompatible())?;
            let stdout = child
                .stdout
                .take()
                .ok_or_else(XmrError::protocol_incompatible)?;
            let stderr = child
                .stderr
                .take()
                .ok_or_else(XmrError::protocol_incompatible)?;
            let stdout_reader =
                thread::spawn(move || read_bounded(stdout, MAX_VERSION_OUTPUT_BYTES));
            let stderr_reader =
                thread::spawn(move || read_bounded(stderr, MAX_VERSION_OUTPUT_BYTES));
            let started = Instant::now();
            let status = loop {
                if let Some(status) = child
                    .try_wait()
                    .map_err(|_| XmrError::protocol_incompatible())?
                {
                    break status;
                }
                if started.elapsed() >= VERSION_PROBE_TIMEOUT {
                    let _ = child.kill();
                    let _ = child.wait();
                    let _ = stdout_reader.join();
                    let _ = stderr_reader.join();
                    return Err(XmrError::protocol_incompatible());
                }
                thread::sleep(Duration::from_millis(10));
            };
            let stdout = stdout_reader
                .join()
                .map_err(|_| XmrError::protocol_incompatible())?
                .map_err(|_| XmrError::protocol_incompatible())?;
            let stderr = stderr_reader
                .join()
                .map_err(|_| XmrError::protocol_incompatible())?
                .map_err(|_| XmrError::protocol_incompatible())?;
            if stdout.len().saturating_add(stderr.len()) > MAX_VERSION_OUTPUT_BYTES
                || !status.success()
                || !stderr.is_empty()
            {
                return Err(XmrError::protocol_incompatible());
            }
            normalize_version_output(stdout)
        }

        fn persist_selection(&mut self, selected_path: &str) -> Result<SelectionRecord, XmrError> {
            self.write_and_load_record(selected_path)
        }

        fn load_selection(&mut self) -> Result<SelectionRecord, XmrError> {
            self.ensure_private_directory()?;
            self.load_record()
        }
    }

    fn selected_file_kind(file_type: &fs::FileType) -> SelectedFileKind {
        if file_type.is_symlink() {
            SelectedFileKind::Symlink
        } else if file_type.is_file() {
            SelectedFileKind::Regular
        } else if file_type.is_dir() {
            SelectedFileKind::Directory
        } else if file_type.is_fifo() {
            SelectedFileKind::Fifo
        } else if file_type.is_socket() {
            SelectedFileKind::Socket
        } else if file_type.is_block_device() {
            SelectedFileKind::BlockDevice
        } else if file_type.is_char_device() {
            SelectedFileKind::CharacterDevice
        } else {
            SelectedFileKind::Other
        }
    }

    fn observation_from_metadata(metadata: &fs::Metadata) -> ExecutableObservation {
        ExecutableObservation {
            kind: selected_file_kind(&metadata.file_type()),
            device: metadata.dev(),
            inode: metadata.ino(),
            length: metadata.len(),
            modified_seconds: metadata.mtime(),
            modified_nanoseconds: metadata.mtime_nsec(),
            mode: metadata.permissions().mode(),
            owner: metadata.uid(),
            group: metadata.gid(),
        }
    }

    struct EffectiveIdentity {
        user: u32,
        group: u32,
        groups: Vec<u32>,
    }

    fn effective_identity() -> Result<EffectiveIdentity, XmrError> {
        let mut file = File::open("/proc/self/status").map_err(|_| XmrError::unavailable())?;
        let bytes = read_bounded(&mut file, MAX_PROCESS_STATUS_BYTES)
            .map_err(|_| XmrError::unavailable())?;
        let text = core::str::from_utf8(&bytes).map_err(|_| XmrError::unavailable())?;
        let mut user = None;
        let mut group = None;
        let mut groups = None;
        for line in text.lines() {
            if let Some(values) = line.strip_prefix("Uid:") {
                user = parse_effective_id(values);
            } else if let Some(values) = line.strip_prefix("Gid:") {
                group = parse_effective_id(values);
            } else if let Some(values) = line.strip_prefix("Groups:") {
                groups = Some(
                    values
                        .split_ascii_whitespace()
                        .map(str::parse)
                        .collect::<Result<Vec<u32>, _>>()
                        .map_err(|_| XmrError::unavailable())?,
                );
            }
        }
        Ok(EffectiveIdentity {
            user: user.ok_or_else(XmrError::unavailable)?,
            group: group.ok_or_else(XmrError::unavailable)?,
            groups: groups.ok_or_else(XmrError::unavailable)?,
        })
    }

    fn parse_effective_id(values: &str) -> Option<u32> {
        values.split_ascii_whitespace().nth(1)?.parse().ok()
    }

    fn normalize_version_output(mut bytes: Vec<u8>) -> Result<String, XmrError> {
        if bytes.last() == Some(&b'\n') {
            bytes.pop();
            if bytes.last() == Some(&b'\r') {
                bytes.pop();
            }
        }
        if bytes.contains(&b'\n') || bytes.contains(&b'\r') {
            return Err(XmrError::protocol_incompatible());
        }
        let text = String::from_utf8(bytes).map_err(|_| XmrError::protocol_incompatible())?;
        if text == VERIFIED_VERSION {
            Ok(text)
        } else {
            Err(XmrError::protocol_incompatible())
        }
    }

    fn read_bounded(mut reader: impl Read, maximum: usize) -> std::io::Result<Vec<u8>> {
        let mut output = Vec::new();
        let mut buffer = [0u8; 4 * 1_024];
        loop {
            let count = reader.read(&mut buffer)?;
            if count == 0 {
                return Ok(output);
            }
            if output.len().saturating_add(count) > maximum {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "bounded input exceeded",
                ));
            }
            output.extend_from_slice(&buffer[..count]);
        }
    }

    fn encode_record(selected_path: &str) -> Result<Vec<u8>, XmrError> {
        let pin_length = u16::try_from(LINUX_PIN_ID.len()).map_err(|_| XmrError::internal())?;
        let path_length = u16::try_from(selected_path.len()).map_err(|_| XmrError::internal())?;
        let mut bytes = Vec::with_capacity(
            RECORD_MAGIC.len() + 4 + 2 + LINUX_PIN_ID.len() + 2 + selected_path.len(),
        );
        bytes.extend_from_slice(RECORD_MAGIC);
        bytes.extend_from_slice(&SELECTION_SCHEMA_VERSION.to_be_bytes());
        bytes.extend_from_slice(&pin_length.to_be_bytes());
        bytes.extend_from_slice(LINUX_PIN_ID.as_bytes());
        bytes.extend_from_slice(&path_length.to_be_bytes());
        bytes.extend_from_slice(selected_path.as_bytes());
        Ok(bytes)
    }

    fn decode_record(bytes: &[u8]) -> Result<String, XmrError> {
        let mut cursor = 0usize;
        if take(bytes, &mut cursor, RECORD_MAGIC.len())? != RECORD_MAGIC {
            return Err(XmrError::state_corrupt());
        }
        let schema = u32::from_be_bytes(
            take(bytes, &mut cursor, 4)?
                .try_into()
                .map_err(|_| XmrError::state_corrupt())?,
        );
        if schema != SELECTION_SCHEMA_VERSION {
            return Err(XmrError::state_corrupt());
        }
        let pin_length = read_u16(bytes, &mut cursor)?;
        let pin = take(bytes, &mut cursor, pin_length)?;
        if pin != LINUX_PIN_ID.as_bytes() {
            return Err(XmrError::state_corrupt());
        }
        let path_length = read_u16(bytes, &mut cursor)?;
        let selected_path = take(bytes, &mut cursor, path_length)?;
        if cursor != bytes.len() {
            return Err(XmrError::state_corrupt());
        }
        let selected_path = core::str::from_utf8(selected_path)
            .map_err(|_| XmrError::state_corrupt())?
            .to_owned();
        super::ValidatedSelectedPath::from_os_str(std::ffi::OsStr::new(&selected_path))
            .map_err(|_| XmrError::state_corrupt())?;
        Ok(selected_path)
    }

    fn read_u16(bytes: &[u8], cursor: &mut usize) -> Result<usize, XmrError> {
        let value = take(bytes, cursor, 2)?;
        Ok(u16::from_be_bytes([value[0], value[1]]) as usize)
    }

    fn take<'a>(bytes: &'a [u8], cursor: &mut usize, length: usize) -> Result<&'a [u8], XmrError> {
        let end = cursor
            .checked_add(length)
            .ok_or_else(XmrError::state_corrupt)?;
        let value = bytes
            .get(*cursor..end)
            .ok_or_else(XmrError::state_corrupt)?;
        *cursor = end;
        Ok(value)
    }
}

#[cfg(target_os = "linux")]
pub use linux::LinuxDistributionPort;

#[cfg(target_os = "linux")]
pub struct SystemDistribution {
    manager: DistributionManager<LinuxDistributionPort>,
}

#[cfg(target_os = "linux")]
impl SystemDistribution {
    pub fn new(data_directory: impl AsRef<Path>) -> Self {
        Self {
            manager: DistributionManager::new(
                HostPlatform::current(),
                LinuxDistributionPort::new(data_directory),
            ),
        }
    }

    pub fn enroll(&mut self, selected_path: &str) -> Result<(), XmrError> {
        self.manager.enroll(selected_path)
    }

    pub fn restore_selection(&mut self) -> Result<(), XmrError> {
        self.manager.restore_selection()
    }

    pub fn authorize_launch(&mut self) -> Result<VerifiedExecutable, XmrError> {
        self.manager.authorize_launch()
    }

    pub fn poll_selected_file(&mut self) -> Result<(), XmrError> {
        self.manager.poll_selected_file()
    }

    pub fn selection_record(&self) -> Option<&SelectionRecord> {
        self.manager.selection()
    }

    pub fn stop_required(&self) -> bool {
        self.manager.stop_required()
    }

    pub fn teardown_reason(&self) -> Option<&'static str> {
        self.manager.teardown_reason()
    }
}
