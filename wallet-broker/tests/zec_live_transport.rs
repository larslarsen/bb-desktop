use std::collections::BTreeSet;
use std::fs::{self, OpenOptions};
use std::io::{ErrorKind, Read, Write};
use std::os::unix::fs::{MetadataExt, OpenOptionsExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender, TryRecvError};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use base64ct::{Base64, Encoding};
use bitbook_wallet_broker::zec::test_support::probe_live_transport_for_test;
use bitbook_wallet_broker::zec::{LiveCancellation, MAX_COMPACT_BLOCK_BYTES};
use prost::Message;
use serde::Serialize;
use serde_json::Value;
use zcash_client_backend::proto::compact_formats::{ChainMetadata, CompactBlock};
use zcash_client_backend::proto::service::{
    BlockId, BlockRange, ChainSpec, Empty, LightdInfo, TreeState,
};

const RPC_PREFIX: &str = "/cash.z.wallet.sdk.rpc.CompactTxStreamer/";
const INFO_PATH: &str = "/cash.z.wallet.sdk.rpc.CompactTxStreamer/GetLightdInfo";
const LATEST_PATH: &str = "/cash.z.wallet.sdk.rpc.CompactTxStreamer/GetLatestBlock";
const TREE_PATH: &str = "/cash.z.wallet.sdk.rpc.CompactTxStreamer/GetTreeState";
const RANGE_PATH: &str = "/cash.z.wallet.sdk.rpc.CompactTxStreamer/GetBlockRange";
const TIP: u32 = 1_842_421;
const EVENT_LIMIT: usize = 256 * 1024;
const LINE_LIMIT: usize = 64 * 1024;

#[derive(Serialize)]
struct FixtureConfig {
    cert_path: String,
    key_path: String,
    lightd_info: String,
    latest_block: String,
    tree_state: String,
    blocks: Vec<String>,
    range_mode: &'static str,
}

#[derive(Debug)]
struct ObservedRequest {
    path: String,
    body: String,
}

struct Scratch {
    root: PathBuf,
    cleaned: bool,
}

impl Scratch {
    fn new(label: &str) -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(1);
        let target = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target");
        let target_meta = fs::symlink_metadata(&target).expect("Cargo target directory must exist");
        assert!(target_meta.file_type().is_dir() && !target_meta.file_type().is_symlink());
        let root = target.join(format!(
            "wal015-live-transport-{label}-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        match fs::symlink_metadata(&root) {
            Err(error) if error.kind() == ErrorKind::NotFound => {}
            Ok(_) => panic!("refusing to reuse owned transport root {}", root.display()),
            Err(error) => panic!("cannot inspect owned transport root: {error}"),
        }
        fs::create_dir(&root).unwrap();
        fs::set_permissions(&root, fs::Permissions::from_mode(0o700)).unwrap();
        let mut scratch = Self {
            root,
            cleaned: false,
        };
        scratch.generate_credentials();
        scratch
    }

    fn path(&self, name: &str) -> PathBuf {
        self.root.join(name)
    }

    fn generate_credentials(&mut self) {
        let extension = self.path("server.ext");
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(&extension)
            .unwrap();
        file.write_all(
            b"subjectAltName=DNS:localhost,IP:127.0.0.1\n\
basicConstraints=critical,CA:FALSE\n\
keyUsage=critical,digitalSignature,keyEncipherment\n\
extendedKeyUsage=serverAuth\n",
        )
        .unwrap();
        drop(file);

        let mut ca = Command::new("openssl");
        ca.args([
            "req",
            "-x509",
            "-newkey",
            "rsa:2048",
            "-sha256",
            "-nodes",
            "-days",
            "1",
            "-subj",
            "/CN=BitBook WAL-015 test CA",
            "-addext",
            "basicConstraints=critical,CA:TRUE",
            "-addext",
            "keyUsage=critical,keyCertSign,cRLSign",
            "-keyout",
        ])
        .arg(self.path("ca.key"))
        .arg("-out")
        .arg(self.path("ca.pem"));
        run_bounded(ca, "create test CA").unwrap();

        let mut request = Command::new("openssl");
        request
            .args([
                "req",
                "-new",
                "-newkey",
                "rsa:2048",
                "-sha256",
                "-nodes",
                "-subj",
                "/CN=localhost",
                "-addext",
                "subjectAltName=DNS:localhost,IP:127.0.0.1",
                "-keyout",
            ])
            .arg(self.path("server.key"))
            .arg("-out")
            .arg(self.path("server.csr"));
        run_bounded(request, "create localhost certificate request").unwrap();

        let mut sign = Command::new("openssl");
        sign.args([
            "x509",
            "-req",
            "-sha256",
            "-days",
            "1",
            "-set_serial",
            "1",
            "-in",
        ])
        .arg(self.path("server.csr"))
        .arg("-CA")
        .arg(self.path("ca.pem"))
        .arg("-CAkey")
        .arg(self.path("ca.key"))
        .arg("-extfile")
        .arg(&extension)
        .arg("-out")
        .arg(self.path("server.pem"));
        run_bounded(sign, "sign localhost certificate").unwrap();

        for name in CREDENTIAL_FILES {
            let path = self.path(name);
            let meta = fs::symlink_metadata(&path).unwrap();
            assert!(meta.file_type().is_file() && !meta.file_type().is_symlink());
            assert_eq!(meta.nlink(), 1);
            fs::set_permissions(path, fs::Permissions::from_mode(0o600)).unwrap();
        }
    }

    fn cleanup(&mut self) -> Result<(), String> {
        if self.cleaned {
            return Ok(());
        }
        let root_meta = fs::symlink_metadata(&self.root)
            .map_err(|error| format!("cannot inspect owned root: {error}"))?;
        if !root_meta.file_type().is_dir() || root_meta.file_type().is_symlink() {
            return Err("owned root was replaced".to_owned());
        }
        let expected = CREDENTIAL_FILES.iter().copied().collect::<BTreeSet<_>>();
        for entry in
            fs::read_dir(&self.root).map_err(|error| format!("cannot read root: {error}"))?
        {
            let entry = entry.map_err(|error| format!("cannot read root entry: {error}"))?;
            let name = entry
                .file_name()
                .to_str()
                .ok_or_else(|| "non-UTF-8 entry in owned root".to_owned())?
                .to_owned();
            if !expected.contains(name.as_str()) {
                return Err(format!("unexpected entry in owned root: {name}"));
            }
            let meta = fs::symlink_metadata(entry.path())
                .map_err(|error| format!("cannot inspect owned file: {error}"))?;
            if !meta.file_type().is_file() || meta.file_type().is_symlink() || meta.nlink() != 1 {
                return Err(format!("unsafe owned file: {name}"));
            }
            fs::remove_file(entry.path())
                .map_err(|error| format!("cannot remove owned file {name}: {error}"))?;
        }
        fs::remove_dir(&self.root).map_err(|error| format!("cannot remove owned root: {error}"))?;
        match fs::symlink_metadata(&self.root) {
            Err(error) if error.kind() == ErrorKind::NotFound => {}
            Err(error) => return Err(format!("cannot verify cleanup: {error}")),
            Ok(_) => return Err("owned root remains after cleanup".to_owned()),
        }
        self.cleaned = true;
        Ok(())
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        if let Err(error) = self.cleanup() {
            if std::thread::panicking() {
                eprintln!("WAL-015 transport scratch cleanup failed: {error}");
            } else {
                panic!("WAL-015 transport scratch cleanup failed: {error}");
            }
        }
    }
}

const CREDENTIAL_FILES: [&str; 6] = [
    "server.ext",
    "ca.key",
    "ca.pem",
    "server.key",
    "server.csr",
    "server.pem",
];

struct TestServer {
    scratch: Option<Scratch>,
    child: Option<Child>,
    events: Receiver<Result<String, String>>,
    stdout_reader: Option<JoinHandle<Result<(), String>>>,
    stderr_reader: Option<JoinHandle<Result<Vec<u8>, String>>>,
    requests: Vec<ObservedRequest>,
    endpoint: String,
    ca_pem: Vec<u8>,
}

type ProbeOutcome = Result<(), (String, String)>;

struct ProbeWorker {
    cancellation: LiveCancellation,
    result: Receiver<ProbeOutcome>,
    thread: Option<JoinHandle<()>>,
}

impl ProbeWorker {
    fn start(endpoint: String, ca_pem: Vec<u8>) -> Self {
        let cancellation = LiveCancellation::new();
        let worker_cancellation = cancellation.clone();
        let (result_tx, result) = mpsc::channel();
        let thread = thread::spawn(move || {
            let outcome = probe_live_transport_for_test(
                &endpoint,
                Some(&ca_pem),
                worker_cancellation,
                Duration::from_secs(5),
            )
            .map(|_| ())
            .map_err(|error| (error.code().to_owned(), error.public_message().to_owned()));
            let _ = result_tx.send(outcome);
        });
        Self {
            cancellation,
            result,
            thread: Some(thread),
        }
    }

    fn cancel(&self) {
        self.cancellation.cancel();
    }

    fn finish(&mut self, timeout: Duration) -> ProbeOutcome {
        let outcome = self
            .result
            .recv_timeout(timeout)
            .expect("cancelled probe must return promptly");
        self.join().expect("cancelled probe thread must be reaped");
        outcome
    }

    fn join(&mut self) -> Result<(), String> {
        if let Some(thread) = self.thread.take() {
            thread
                .join()
                .map_err(|_| "probe worker panicked".to_owned())?;
        }
        Ok(())
    }

    fn cleanup(&mut self) -> Result<(), String> {
        if self.thread.is_none() {
            return Ok(());
        }
        self.cancellation.cancel();
        let signal = self.result.recv_timeout(Duration::from_secs(2));
        self.join()?;
        signal
            .map(|_| ())
            .map_err(|error| format!("probe worker missed cleanup bound: {error}"))
    }
}

impl Drop for ProbeWorker {
    fn drop(&mut self) {
        if let Err(error) = self.cleanup() {
            if std::thread::panicking() {
                eprintln!("WAL-015 probe worker cleanup failed: {error}");
            } else {
                panic!("WAL-015 probe worker cleanup failed: {error}");
            }
        }
    }
}

impl TestServer {
    fn start(label: &str, block_messages: Vec<Vec<u8>>, range_mode: &'static str) -> Self {
        let scratch = Scratch::new(label);
        let config = canonical_config(&scratch, block_messages, range_mode);
        let config = serde_json::to_vec(&config).unwrap();
        assert!(config.len() <= 4 * 1024 * 1024);
        let script =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/live-grpc-server.js");
        let script_meta = fs::symlink_metadata(&script).unwrap();
        assert!(script_meta.file_type().is_file() && !script_meta.file_type().is_symlink());

        let mut child = Command::new("node")
            .arg(script)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("start owned Node fixture");
        let stdin = child.stdin.take().unwrap();
        let stdout = child.stdout.take().unwrap();
        let stderr = child.stderr.take().unwrap();
        let (event_tx, events) = mpsc::channel();
        let stdout_reader = thread::spawn(move || read_event_lines(stdout, event_tx));
        let stderr_reader = thread::spawn(move || read_bounded(stderr, LINE_LIMIT));
        let ca_pem = fs::read(scratch.path("ca.pem")).unwrap();
        let mut server = Self {
            scratch: Some(scratch),
            child: Some(child),
            events,
            stdout_reader: Some(stdout_reader),
            stderr_reader: Some(stderr_reader),
            requests: Vec::new(),
            endpoint: String::new(),
            ca_pem,
        };
        let mut stdin = stdin;
        stdin
            .write_all(&config)
            .expect("configure owned Node fixture");
        drop(stdin);
        let ready = server.wait_event("ready", Duration::from_secs(3));
        let port = ready["port"].as_u64().expect("ready port");
        assert!((1..=u64::from(u16::MAX)).contains(&port));
        server.endpoint = format!("https://localhost:{port}");
        server
    }

    fn wait_event(&mut self, wanted: &str, timeout: Duration) -> Value {
        let deadline = Instant::now() + timeout;
        loop {
            let remaining = deadline.saturating_duration_since(Instant::now());
            let line = match self.events.recv_timeout(remaining) {
                Ok(Ok(line)) => line,
                Ok(Err(error)) => panic!("fixture stdout failed: {error}"),
                Err(RecvTimeoutError::Timeout) => panic!("fixture did not report {wanted}"),
                Err(RecvTimeoutError::Disconnected) => panic!("fixture exited before {wanted}"),
            };
            let value = self.record_event(line).expect("fixture JSON event");
            if value["event"] == wanted {
                return value;
            }
        }
    }

    fn record_event(&mut self, line: String) -> Result<Value, String> {
        let value: Value = serde_json::from_str(&line)
            .map_err(|error| format!("invalid fixture JSON event: {error}"))?;
        if value["event"] == "request" {
            self.requests.push(ObservedRequest {
                path: value["path"]
                    .as_str()
                    .ok_or_else(|| "fixture request omitted path".to_owned())?
                    .to_owned(),
                body: value["body"]
                    .as_str()
                    .ok_or_else(|| "fixture request omitted body".to_owned())?
                    .to_owned(),
            });
        }
        Ok(value)
    }

    fn drain_events(&mut self) -> Result<(), String> {
        let mut errors = Vec::new();
        loop {
            match self.events.try_recv() {
                Ok(Ok(line)) => {
                    if let Err(error) = self.record_event(line) {
                        errors.push(error);
                    }
                }
                Ok(Err(error)) => errors.push(error),
                Err(TryRecvError::Empty) => {
                    errors.push("fixture event channel remained open after reader join".to_owned());
                    break;
                }
                Err(TryRecvError::Disconnected) => break,
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors.join("; "))
        }
    }

    fn collect_requests(&mut self, count: usize) {
        while self.requests.len() < count {
            self.wait_event("request", Duration::from_secs(2));
        }
    }

    fn await_request(&mut self, path: &str) {
        while !self.requests.iter().any(|request| request.path == path) {
            self.wait_event("request", Duration::from_secs(2));
        }
    }

    fn root(&self) -> &Path {
        &self.scratch.as_ref().unwrap().root
    }

    fn stop(&mut self) -> Result<(), String> {
        let mut errors = Vec::new();
        let mut child_reaped = true;
        if let Some(mut child) = self.child.take() {
            match child.kill() {
                Ok(()) => {}
                Err(error) if error.kind() == ErrorKind::InvalidInput => {}
                Err(error) => errors.push(format!("cannot stop Node fixture: {error}")),
            }
            if let Err(error) = wait_child(&mut child, Duration::from_secs(2)) {
                let _ = child.kill();
                match wait_child(&mut child, Duration::from_secs(2)) {
                    Ok(_) => errors.push(error),
                    Err(second) => {
                        child_reaped = false;
                        self.child = Some(child);
                        errors.push(format!("{error}; retry failed: {second}"));
                    }
                }
            }
        }

        if child_reaped {
            if let Some(reader) = self.stdout_reader.take() {
                match reader.join() {
                    Ok(Ok(())) => {}
                    Ok(Err(error)) => errors.push(error),
                    Err(_) => errors.push("stdout reader panicked".to_owned()),
                }
            }
            if let Some(reader) = self.stderr_reader.take() {
                match reader.join() {
                    Ok(Ok(stderr)) if stderr.len() <= LINE_LIMIT => {}
                    Ok(Ok(_)) => errors.push("stderr exceeded fixture limit".to_owned()),
                    Ok(Err(error)) => errors.push(error),
                    Err(_) => errors.push("stderr reader panicked".to_owned()),
                }
            }
            if let Err(error) = self.drain_events() {
                errors.push(error);
            }
            if let Some(mut scratch) = self.scratch.take() {
                if let Err(error) = scratch.cleanup() {
                    errors.push(error);
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors.join("; "))
        }
    }
}

impl Drop for TestServer {
    fn drop(&mut self) {
        if let Err(error) = self.stop() {
            if std::thread::panicking() {
                eprintln!("WAL-015 Node fixture cleanup failed: {error}");
            } else {
                panic!("WAL-015 Node fixture cleanup failed: {error}");
            }
        }
    }
}

#[test]
fn runtime_ca_is_the_only_trust_seam_and_real_rpc_bodies_are_account_free() {
    let block = canonical_block().encode_to_vec();
    let mut server = TestServer::start("trusted", vec![block], "complete");
    let result = probe_live_transport_for_test(
        &server.endpoint,
        Some(&server.ca_pem),
        LiveCancellation::new(),
        Duration::from_secs(2),
    )
    .unwrap();
    assert_eq!(result.tip_height, TIP);
    assert_eq!(result.block_count, 1);

    let untrusted = probe_live_transport_for_test(
        &server.endpoint,
        None,
        LiveCancellation::new(),
        Duration::from_secs(1),
    )
    .unwrap_err();
    assert_eq!(untrusted.code(), "UNAVAILABLE");
    assert!(!format!("{untrusted:?} {untrusted}").contains("CANARY"));

    server.collect_requests(4);
    let root = server.root().to_path_buf();
    server.stop().unwrap();
    assert_canonical_requests(&server.requests);
    assert!(
        matches!(fs::symlink_metadata(root), Err(error) if error.kind() == ErrorKind::NotFound)
    );
}

#[test]
fn oversized_compact_block_is_rejected_by_the_real_tonic_stream_limit() {
    let mut block = canonical_block();
    block.header = vec![b'X'; MAX_COMPACT_BLOCK_BYTES + 1];
    block.header[..28].copy_from_slice(b"CANARY_OVERSIZED_BLOCK_FRAME");
    let mut server = TestServer::start("oversized", vec![block.encode_to_vec()], "complete");
    let error = probe_live_transport_for_test(
        &server.endpoint,
        Some(&server.ca_pem),
        LiveCancellation::new(),
        Duration::from_secs(2),
    )
    .unwrap_err();
    assert_eq!(error.code(), "LIMIT");
    assert!(!format!("{error:?} {error}").contains("CANARY_OVERSIZED_BLOCK_FRAME"));
    server.collect_requests(4);
    assert_eq!(server.requests.last().unwrap().path, RANGE_PATH);
    server.stop().unwrap();
}

#[test]
fn complete_stream_deadline_and_explicit_cancellation_interrupt_a_stalled_range() {
    let block = canonical_block().encode_to_vec();
    let mut deadline_server = TestServer::start("deadline", vec![block.clone()], "stall");
    let started = Instant::now();
    let error = probe_live_transport_for_test(
        &deadline_server.endpoint,
        Some(&deadline_server.ca_pem),
        LiveCancellation::new(),
        Duration::from_millis(500),
    )
    .unwrap_err();
    assert_eq!(error.code(), "UNAVAILABLE");
    assert!(started.elapsed() < Duration::from_secs(2));
    deadline_server.collect_requests(4);
    assert_eq!(deadline_server.requests.last().unwrap().path, RANGE_PATH);
    let deadline_root = deadline_server.root().to_path_buf();
    deadline_server.stop().unwrap();
    assert!(
        matches!(fs::symlink_metadata(deadline_root), Err(error) if error.kind() == ErrorKind::NotFound)
    );

    let mut cancel_server = TestServer::start("cancel", vec![block], "stall");
    let mut worker =
        ProbeWorker::start(cancel_server.endpoint.clone(), cancel_server.ca_pem.clone());
    cancel_server.await_request(RANGE_PATH);
    let cancelled_at = Instant::now();
    worker.cancel();
    let result = worker.finish(Duration::from_secs(2));
    let (code, message) = result.unwrap_err();
    assert_eq!(code, "CANCELLED");
    assert!(!message.contains("CANARY"));
    assert!(cancelled_at.elapsed() < Duration::from_secs(2));
    let cancel_root = cancel_server.root().to_path_buf();
    cancel_server.stop().unwrap();
    assert!(
        matches!(fs::symlink_metadata(cancel_root), Err(error) if error.kind() == ErrorKind::NotFound)
    );
}

fn canonical_config(
    scratch: &Scratch,
    block_messages: Vec<Vec<u8>>,
    range_mode: &'static str,
) -> FixtureConfig {
    FixtureConfig {
        cert_path: scratch.path("server.pem").to_string_lossy().into_owned(),
        key_path: scratch.path("server.key").to_string_lossy().into_owned(),
        lightd_info: encode(&LightdInfo {
            version: "WAL-015 fixture".to_owned(),
            vendor: "BitBook".to_owned(),
            chain_name: "test".to_owned(),
            sapling_activation_height: 280_000,
            consensus_branch_id: "c2d6d0b4".to_owned(),
            block_height: u64::from(TIP),
            estimated_height: u64::from(TIP),
            lightwallet_protocol_version: "0.1".to_owned(),
            ..Default::default()
        }),
        latest_block: encode(&BlockId {
            height: u64::from(TIP),
            hash: vec![0x22; 32],
        }),
        tree_state: encode(&TreeState {
            network: "test".to_owned(),
            height: u64::from(TIP - 1),
            hash: "11".repeat(32),
            time: 1_700_000_000,
            sapling_tree: String::new(),
            orchard_tree: String::new(),
            ironwood_tree: String::new(),
        }),
        blocks: block_messages
            .into_iter()
            .map(|bytes| Base64::encode_string(&bytes))
            .collect(),
        range_mode,
    }
}

fn canonical_block() -> CompactBlock {
    CompactBlock {
        height: u64::from(TIP),
        hash: vec![0x22; 32],
        prev_hash: vec![0x11; 32],
        time: 1_700_000_075,
        header: Vec::new(),
        vtx: Vec::new(),
        chain_metadata: Some(ChainMetadata {
            sapling_commitment_tree_size: 0,
            orchard_commitment_tree_size: 0,
            ironwood_commitment_tree_size: 0,
        }),
    }
}

fn encode<M: Message>(message: &M) -> String {
    Base64::encode_string(&message.encode_to_vec())
}

fn assert_canonical_requests(requests: &[ObservedRequest]) {
    assert_eq!(requests.len(), 4);
    assert_eq!(
        requests
            .iter()
            .map(|request| request.path.as_str())
            .collect::<Vec<_>>(),
        [INFO_PATH, LATEST_PATH, TREE_PATH, RANGE_PATH]
    );
    assert!(
        requests
            .iter()
            .all(|request| request.path.starts_with(RPC_PREFIX))
    );
    assert!(!requests.iter().any(|request| {
        let lowered = request.path.to_ascii_lowercase();
        lowered.contains("address")
            || lowered.contains("transaction")
            || lowered.contains("mempool")
    }));

    let _: Empty = decode_exact_request(&requests[0]);
    let _: ChainSpec = decode_exact_request(&requests[1]);
    let tree: BlockId = decode_exact_request(&requests[2]);
    assert_eq!(tree.height, u64::from(TIP - 1));
    assert!(tree.hash.is_empty());
    let range: BlockRange = decode_exact_request(&requests[3]);
    assert_eq!(
        range.start.unwrap(),
        BlockId {
            height: u64::from(TIP),
            hash: Vec::new()
        }
    );
    assert_eq!(
        range.end.unwrap(),
        BlockId {
            height: u64::from(TIP),
            hash: Vec::new()
        }
    );
    assert!(range.pool_types.is_empty());
}

fn decode_exact_request<M: Message + Default>(request: &ObservedRequest) -> M {
    let framed = Base64::decode_vec(&request.body).expect("canonical base64 request");
    assert!(framed.len() >= 5);
    assert_eq!(framed[0], 0, "compressed gRPC request was not authorized");
    let declared = u32::from_be_bytes(framed[1..5].try_into().unwrap()) as usize;
    assert_eq!(declared, framed.len() - 5);
    let message = M::decode(&framed[5..]).expect("protobuf request");
    assert_eq!(
        message.encode_to_vec(),
        framed[5..],
        "request carried extra fields"
    );
    message
}

fn run_bounded(mut command: Command, label: &str) -> Result<(), String> {
    command
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    let mut child = command
        .spawn()
        .map_err(|error| format!("cannot {label}: {error}"))?;
    let status = wait_child(&mut child, Duration::from_secs(10)).or_else(|error| {
        let _ = child.kill();
        let _ = wait_child(&mut child, Duration::from_secs(2));
        Err(error)
    })?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("failed to {label}"))
    }
}

fn wait_child(child: &mut Child, timeout: Duration) -> Result<std::process::ExitStatus, String> {
    let deadline = Instant::now() + timeout;
    loop {
        match child.try_wait() {
            Ok(Some(status)) => return Ok(status),
            Ok(None) if Instant::now() < deadline => thread::sleep(Duration::from_millis(10)),
            Ok(None) => return Err("owned child did not exit within bound".to_owned()),
            Err(error) => return Err(format!("cannot reap owned child: {error}")),
        }
    }
}

fn read_event_lines<R: Read>(
    mut reader: R,
    sender: Sender<Result<String, String>>,
) -> Result<(), String> {
    let mut chunk = [0_u8; 4096];
    let mut pending = Vec::new();
    let mut total = 0_usize;
    loop {
        let read = reader
            .read(&mut chunk)
            .map_err(|error| format!("read stdout: {error}"))?;
        if read == 0 {
            if !pending.is_empty() {
                return Err("fixture stdout ended without newline".to_owned());
            }
            return Ok(());
        }
        total = total
            .checked_add(read)
            .ok_or_else(|| "stdout length overflow".to_owned())?;
        if total > EVENT_LIMIT {
            let error = "fixture stdout exceeded limit".to_owned();
            let _ = sender.send(Err(error.clone()));
            return Err(error);
        }
        pending.extend_from_slice(&chunk[..read]);
        while let Some(index) = pending.iter().position(|byte| *byte == b'\n') {
            if index > LINE_LIMIT {
                let error = "fixture stdout line exceeded limit".to_owned();
                let _ = sender.send(Err(error.clone()));
                return Err(error);
            }
            let line = pending.drain(..=index).collect::<Vec<_>>();
            let text = std::str::from_utf8(&line[..line.len() - 1])
                .map_err(|_| "fixture stdout was not UTF-8".to_owned())?
                .to_owned();
            if sender.send(Ok(text)).is_err() {
                return Ok(());
            }
        }
        if pending.len() > LINE_LIMIT {
            return Err("fixture stdout line exceeded limit".to_owned());
        }
    }
}

fn read_bounded<R: Read>(mut reader: R, limit: usize) -> Result<Vec<u8>, String> {
    let mut output = Vec::new();
    let mut chunk = [0_u8; 4096];
    loop {
        let read = reader
            .read(&mut chunk)
            .map_err(|error| format!("read stderr: {error}"))?;
        if read == 0 {
            return Ok(output);
        }
        if output.len().saturating_add(read) > limit {
            return Err("fixture stderr exceeded limit".to_owned());
        }
        output.extend_from_slice(&chunk[..read]);
    }
}
