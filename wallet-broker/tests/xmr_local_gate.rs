use std::env;
use std::ffi::{OsStr, OsString};
use std::fs::{self, File};
use std::io::Read;
use std::net::{Ipv4Addr, TcpListener};
use std::os::unix::fs::{PermissionsExt, symlink};
use std::path::{Component, Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::Mutex;
use std::thread;
use std::time::{Duration, Instant};

use bitbook_wallet_broker::xmr::account::{SoftwareAccountRequest, XmrAccount};
use bitbook_wallet_broker::xmr::distribution::{
    EXECUTABLE_BYTES, EXECUTABLE_SHA256, InstallationVerifier, MONEROD_BYTES, MONEROD_SHA256,
};
use bitbook_wallet_broker::xmr::model::XmrNetwork;
use bitbook_wallet_broker::xmr::process::{WalletRpcProcess, WalletRpcProcessPlan};
use bitbook_wallet_broker::xmr::receiver::FreshXmrReceiverV1;
use bitbook_wallet_broker::xmr::rpc::{LoopbackEndpoint, NodeRpcClient, WalletRpcClient};
use sha2::{Digest, Sha256};

const ACCOUNT: &str = "00112233445566778899aabbccddeeff";
const REQUEST: &str = "102132435465768798a9bacbdcedfe0f";
const WALLET_RPC_MEMBER: &str = "monero-gui-v0.18.5.2/extras/monero-wallet-rpc";
const MONEROD_MEMBER: &str = "monero-gui-v0.18.5.2/monerod";
const READY_TIMEOUT: Duration = Duration::from_secs(10);
const READY_POLL: Duration = Duration::from_millis(25);

static REAL_GATE_LOCK: Mutex<()> = Mutex::new(());

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum GateFailure {
    Environment,
    UnsafeRoot,
    UnsafeScratch,
    ExistingScratch,
    BinaryPin,
    Port,
    Spawn,
    Readiness,
    ProductBoundary,
    Cleanup,
}

fn exact_scratch_leaf() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/wal007-local-gate")
}

fn map_io<T>(result: std::io::Result<T>, failure: GateFailure) -> Result<T, GateFailure> {
    result.map_err(|_| failure)
}

fn reject_symlink_components(path: &Path) -> Result<(), GateFailure> {
    if !path.is_absolute() {
        return Err(GateFailure::UnsafeScratch);
    }
    let mut walked = PathBuf::new();
    for component in path.components() {
        match component {
            Component::RootDir => walked.push(component.as_os_str()),
            Component::Normal(part) => walked.push(part),
            _ => return Err(GateFailure::UnsafeScratch),
        }
        let metadata = map_io(fs::symlink_metadata(&walked), GateFailure::UnsafeScratch)?;
        if metadata.file_type().is_symlink() {
            return Err(GateFailure::UnsafeScratch);
        }
    }
    Ok(())
}

struct ExactScratch {
    manifest_real: PathBuf,
    target_real: PathBuf,
    leaf: PathBuf,
    created: bool,
}

impl ExactScratch {
    fn preflight() -> Result<Self, GateFailure> {
        let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let manifest_real = map_io(fs::canonicalize(&manifest), GateFailure::UnsafeScratch)?;
        if manifest != manifest_real {
            return Err(GateFailure::UnsafeScratch);
        }
        reject_symlink_components(&manifest_real)?;

        let target = manifest_real.join("target");
        let target_real = map_io(fs::canonicalize(&target), GateFailure::UnsafeScratch)?;
        if target != target_real {
            return Err(GateFailure::UnsafeScratch);
        }
        reject_symlink_components(&target_real)?;

        let leaf = exact_scratch_leaf();
        if leaf != target_real.join("wal007-local-gate") {
            return Err(GateFailure::UnsafeScratch);
        }
        if fs::symlink_metadata(&leaf).is_ok() {
            return Err(GateFailure::ExistingScratch);
        }
        Ok(Self {
            manifest_real,
            target_real,
            leaf,
            created: false,
        })
    }

    fn create(&mut self) -> Result<(), GateFailure> {
        map_io(fs::create_dir(&self.leaf), GateFailure::UnsafeScratch)?;
        map_io(
            fs::set_permissions(&self.leaf, fs::Permissions::from_mode(0o700)),
            GateFailure::UnsafeScratch,
        )?;
        self.created = true;
        let metadata = map_io(fs::symlink_metadata(&self.leaf), GateFailure::UnsafeScratch)?;
        if !metadata.is_dir()
            || metadata.file_type().is_symlink()
            || metadata.permissions().mode() & 0o777 != 0o700
        {
            return Err(GateFailure::UnsafeScratch);
        }
        Ok(())
    }

    fn child(&self, name: &str) -> Result<PathBuf, GateFailure> {
        if name.is_empty() || name.contains('/') || name == "." || name == ".." {
            return Err(GateFailure::UnsafeScratch);
        }
        Ok(self.leaf.join(name))
    }

    fn remove_exact(&mut self) -> Result<(), GateFailure> {
        if !self.created {
            return Ok(());
        }
        if self.manifest_real != PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            || self.target_real != self.manifest_real.join("target")
            || self.leaf != self.target_real.join("wal007-local-gate")
        {
            return Err(GateFailure::Cleanup);
        }
        reject_symlink_components(&self.target_real)?;
        let metadata = map_io(fs::symlink_metadata(&self.leaf), GateFailure::Cleanup)?;
        if !metadata.is_dir() || metadata.file_type().is_symlink() {
            return Err(GateFailure::Cleanup);
        }
        map_io(fs::remove_dir_all(&self.leaf), GateFailure::Cleanup)?;
        self.created = false;
        if fs::symlink_metadata(&self.leaf).is_ok() {
            return Err(GateFailure::Cleanup);
        }
        Ok(())
    }
}

impl Drop for ExactScratch {
    fn drop(&mut self) {
        let _ = self.remove_exact();
    }
}

struct TestOwnedMonerod {
    child: Option<Child>,
}

impl TestOwnedMonerod {
    fn spawn(program: &Path, args: &[OsString], current_dir: &Path) -> Result<Self, GateFailure> {
        let child = Command::new(program)
            .args(args)
            .current_dir(current_dir)
            .env_clear()
            .env("LANG", "C")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|_| GateFailure::Spawn)?;
        Ok(Self { child: Some(child) })
    }

    fn reap(&mut self) -> Result<(), GateFailure> {
        if let Some(mut child) = self.child.take() {
            let _ = child.kill();
            child.wait().map_err(|_| GateFailure::Cleanup)?;
        }
        Ok(())
    }

    fn is_reaped(&self) -> bool {
        self.child.is_none()
    }
}

impl Drop for TestOwnedMonerod {
    fn drop(&mut self) {
        let _ = self.reap();
    }
}

fn sha256_hex(path: &Path) -> Result<String, GateFailure> {
    let mut file = map_io(File::open(path), GateFailure::BinaryPin)?;
    let mut digest = Sha256::new();
    let mut buffer = [0_u8; 64 * 1_024];
    loop {
        let read = map_io(file.read(&mut buffer), GateFailure::BinaryPin)?;
        if read == 0 {
            break;
        }
        digest.update(&buffer[..read]);
    }
    Ok(format!("{:x}", digest.finalize()))
}

fn verify_inner_binary(
    path: &Path,
    expected_bytes: u64,
    expected_sha256: &str,
) -> Result<(), GateFailure> {
    reject_symlink_components(path)?;
    let metadata = map_io(fs::symlink_metadata(path), GateFailure::BinaryPin)?;
    if !metadata.is_file()
        || metadata.file_type().is_symlink()
        || metadata.len() != expected_bytes
        || sha256_hex(path)? != expected_sha256
    {
        return Err(GateFailure::BinaryPin);
    }
    Ok(())
}

fn reserve_random_loopback_port() -> Result<u16, GateFailure> {
    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).map_err(|_| GateFailure::Port)?;
    let port = listener.local_addr().map_err(|_| GateFailure::Port)?.port();
    drop(listener);
    Ok(port)
}

fn distinct_random_loopback_ports() -> Result<(u16, u16), GateFailure> {
    for _ in 0..16 {
        let rpc = reserve_random_loopback_port()?;
        let p2p = reserve_random_loopback_port()?;
        if rpc != p2p {
            return Ok((rpc, p2p));
        }
    }
    Err(GateFailure::Port)
}

fn monerod_args(
    scratch: &ExactScratch,
    rpc_port: u16,
    p2p_port: u16,
) -> Result<Vec<OsString>, GateFailure> {
    let data_dir = scratch.child("monerod-data")?;
    let log_file = scratch.child("monerod.log")?;
    Ok(vec![
        "--stagenet".into(),
        "--offline".into(),
        "--non-interactive".into(),
        "--rpc-bind-ip".into(),
        "127.0.0.1".into(),
        "--rpc-bind-port".into(),
        rpc_port.to_string().into(),
        "--p2p-bind-ip".into(),
        "127.0.0.1".into(),
        "--p2p-bind-port".into(),
        p2p_port.to_string().into(),
        "--data-dir".into(),
        data_dir.into_os_string(),
        "--log-file".into(),
        log_file.into_os_string(),
        "--hide-my-port".into(),
        "--no-igd".into(),
    ])
}

fn wait_for_node(endpoint: LoopbackEndpoint) -> Result<NodeRpcClient, GateFailure> {
    let started = Instant::now();
    while started.elapsed() <= READY_TIMEOUT {
        if let Ok(mut node) = NodeRpcClient::connect_test_owned(endpoint) {
            if node.probe_stagenet().is_ok() {
                return Ok(node);
            }
        }
        thread::sleep(READY_POLL);
    }
    Err(GateFailure::Readiness)
}

fn run_real_gate() -> Result<(), GateFailure> {
    let root_value = env::var_os("BITBOOK_MONERO_TEST_ROOT").ok_or(GateFailure::Environment)?;
    let root = PathBuf::from(root_value);
    let root_real = map_io(fs::canonicalize(&root), GateFailure::UnsafeRoot)?;
    if root != root_real {
        return Err(GateFailure::UnsafeRoot);
    }
    reject_symlink_components(&root_real).map_err(|_| GateFailure::UnsafeRoot)?;

    let wallet_rpc_path = root_real.join(WALLET_RPC_MEMBER);
    let monerod_path = root_real.join(MONEROD_MEMBER);
    verify_inner_binary(&wallet_rpc_path, EXECUTABLE_BYTES, EXECUTABLE_SHA256)?;
    verify_inner_binary(&monerod_path, MONEROD_BYTES, MONEROD_SHA256)?;

    let mut scratch = ExactScratch::preflight()?;
    let noncanonical = scratch.target_real.join("../target/wal007-local-gate");
    if noncanonical == scratch.leaf {
        return Err(GateFailure::UnsafeScratch);
    }
    scratch.create()?;
    if !matches!(ExactScratch::preflight(), Err(GateFailure::ExistingScratch)) {
        return Err(GateFailure::ExistingScratch);
    }

    let symlink_target = scratch.child("symlink-target")?;
    let symlink_component = scratch.child("symlink-component")?;
    map_io(fs::create_dir(&symlink_target), GateFailure::UnsafeScratch)?;
    map_io(
        symlink(&symlink_target, &symlink_component),
        GateFailure::UnsafeScratch,
    )?;
    if reject_symlink_components(&symlink_component).is_ok() {
        return Err(GateFailure::UnsafeScratch);
    }

    let (node_rpc_port, node_p2p_port) = distinct_random_loopback_ports()?;
    let args = monerod_args(&scratch, node_rpc_port, node_p2p_port)?;
    let expected_flag_names = [
        "--stagenet",
        "--offline",
        "--non-interactive",
        "--rpc-bind-ip",
        "--rpc-bind-port",
        "--p2p-bind-ip",
        "--p2p-bind-port",
        "--data-dir",
        "--log-file",
        "--hide-my-port",
        "--no-igd",
    ];
    for required in expected_flag_names {
        if !args
            .iter()
            .any(|arg| arg.as_os_str() == OsStr::new(required))
        {
            return Err(GateFailure::ProductBoundary);
        }
    }
    for forbidden in [
        "--bootstrap-daemon-address",
        "--proxy",
        "--tx-proxy",
        "--mainnet",
        "--public-node",
        "--restricted-rpc",
    ] {
        if args
            .iter()
            .any(|arg| arg.as_os_str() == OsStr::new(forbidden))
        {
            return Err(GateFailure::ProductBoundary);
        }
    }

    let mut monerod = TestOwnedMonerod::spawn(&monerod_path, &args, &scratch.leaf)?;
    let node_endpoint =
        LoopbackEndpoint::numeric_ipv4(node_rpc_port).map_err(|_| GateFailure::ProductBoundary)?;
    let node = wait_for_node(node_endpoint.clone())?;
    if !node.last_probe_was_local_nonbootstrap_stagenet() || node.outbound_connection_count() != 0 {
        return Err(GateFailure::ProductBoundary);
    }

    let installation = InstallationVerifier::linux_x86_64()
        .verify_selected(&wallet_rpc_path)
        .map_err(|_| GateFailure::ProductBoundary)?;
    let runtime_root = scratch.child("wallet-rpc-runtime")?;
    let plan = WalletRpcProcessPlan::for_test_owned_stagenet(
        ACCOUNT,
        installation,
        node_endpoint,
        &runtime_root,
    )
    .map_err(|_| GateFailure::ProductBoundary)?;
    if plan.contains_restricted_rpc()
        || !plan.digest_login_enabled()
        || !plan.binds_numeric_ipv4_loopback()
    {
        return Err(GateFailure::ProductBoundary);
    }

    let mut wallet_process =
        WalletRpcProcess::start(plan).map_err(|_| GateFailure::ProductBoundary)?;
    let mut wallet_rpc = WalletRpcClient::connect(wallet_process.connection())
        .map_err(|_| GateFailure::ProductBoundary)?;
    if !wallet_rpc.authenticated_with_digest()
        || wallet_rpc.process_identity() == node.process_identity()
    {
        return Err(GateFailure::ProductBoundary);
    }

    let account_root = scratch.child("account")?;
    let request = SoftwareAccountRequest::ephemeral_stagenet(ACCOUNT, &account_root)
        .map_err(|_| GateFailure::ProductBoundary)?;
    let mut account = XmrAccount::create_software(request, &node, &mut wallet_rpc)
        .map_err(|_| GateFailure::ProductBoundary)?;
    if account.network() != XmrNetwork::Stagenet
        || !account.is_software()
        || !account.all_paths_are_within(&account_root)
    {
        return Err(GateFailure::ProductBoundary);
    }
    let primary = account
        .primary_address_for_internal_verification()
        .to_owned();
    let receiver: FreshXmrReceiverV1 = account
        .fresh_receiver(REQUEST, &mut wallet_rpc)
        .map_err(|_| GateFailure::ProductBoundary)?;
    if receiver.account_index != 0
        || receiver.subaddress_index == 0
        || receiver.receiver == primary
        || !wallet_rpc.last_receiver_was_validated_subaddress()
    {
        return Err(GateFailure::ProductBoundary);
    }

    account
        .close(&mut wallet_rpc)
        .map_err(|_| GateFailure::Cleanup)?;
    wallet_process
        .stop(wallet_rpc)
        .map_err(|_| GateFailure::Cleanup)?;
    if !wallet_process.is_reaped() {
        return Err(GateFailure::Cleanup);
    }
    monerod.reap()?;
    if !monerod.is_reaped() {
        return Err(GateFailure::Cleanup);
    }
    scratch.remove_exact()?;
    if fs::symlink_metadata(exact_scratch_leaf()).is_ok() {
        return Err(GateFailure::Cleanup);
    }
    Ok(())
}

#[test]
fn real_offline_local_monero_gate_is_serialized_owned_bounded_and_exactly_cleaned() {
    let _serial = REAL_GATE_LOCK
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let result = run_real_gate();
    let scratch_is_absent = fs::symlink_metadata(exact_scratch_leaf()).is_err();
    if result.is_err() || !scratch_is_absent {
        panic!("WAL-007 real offline local gate failed without secret diagnostics");
    }
}
