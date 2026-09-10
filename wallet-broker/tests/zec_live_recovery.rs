use std::env;
use std::fs;
use std::io::ErrorKind;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitStatus, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use bitbook_wallet_broker::zec::test_support::{
    FrozenFixture, LiveSyncHarness, RecordedLiveSource,
};
use bitbook_wallet_broker::zec::{AccountId, LiveCancellation, LiveSyncOptions, LiveSyncPhase};
use rusqlite::{Connection, params};

const ACCOUNT: &str = "00112233445566778899aabbccddeeff";
const FIXTURE_DIR: &str = "tests/fixtures/zec";
const CHILD_TEST: &str = "wal015_hot_journal_crash_child";
const CHILD_DATABASE_ENV: &str = "BITBOOK_WAL015_HOT_JOURNAL_DATABASE";
const CRASH_EXIT: i32 = 73;
const TARGET_HEIGHT: u32 = 107;
const CONFIRMED_RECEIVED_ZAT: u64 = 190_000_000;
const PENDING_RECEIVED_ZAT: u64 = 30_000_000;
const MARKER_BYTES: usize = 256 * 1024;

#[test]
fn preparation_recovers_owned_hot_journal_without_losing_committed_state() {
    let fixture = FrozenFixture::open(FIXTURE_DIR).unwrap();
    let mut wallet = LiveSyncHarness::bootstrap_from_fixture(
        "wal015-hot-journal",
        AccountId::parse(ACCOUNT).unwrap(),
        &fixture,
    )
    .unwrap();
    let mut initial_source = RecordedLiveSource::canonical(&fixture).unwrap();
    wallet
        .sync_through(
            &mut initial_source,
            103,
            &LiveCancellation::new(),
            LiveSyncOptions::default(),
        )
        .unwrap();

    let inspection_before = wallet.inspect().unwrap();
    assert_eq!(inspection_before.committed_height, Some(103));
    let live_path = wallet.live_cache_path().to_path_buf();
    let journal_path = journal_path(&live_path);
    assert!(matches!(
        fs::symlink_metadata(&journal_path),
        Err(error) if error.kind() == ErrorKind::NotFound
    ));
    let live_before = fs::read(&live_path).unwrap();
    let receive_before = fs::read(wallet.receive_wallet_path()).unwrap();
    let restart = wallet.close().unwrap();

    let mut crash = OwnedChild::spawn(&live_path);
    let status = crash.wait_bounded(Duration::from_secs(5));
    assert_eq!(status.code(), Some(CRASH_EXIT));
    let journal_metadata = private_regular_file(&journal_path);
    assert!(
        journal_metadata.len() > 0,
        "SQLite rollback journal was empty"
    );
    assert_ne!(
        fs::read(&live_path).unwrap(),
        live_before,
        "large uncommitted update did not spill a dirty database page"
    );

    let mut reopened = LiveSyncHarness::reopen_via_preparation(restart, &fixture).unwrap();
    assert!(matches!(
        fs::symlink_metadata(&journal_path),
        Err(error) if error.kind() == ErrorKind::NotFound
    ));
    assert_eq!(fs::read(&live_path).unwrap(), live_before);
    assert_eq!(reopened.inspect().unwrap(), inspection_before);
    assert_eq!(
        fs::read(reopened.receive_wallet_path()).unwrap(),
        receive_before
    );

    let mut resumed_source = RecordedLiveSource::canonical(&fixture).unwrap();
    let completed = reopened
        .sync(
            &mut resumed_source,
            &LiveCancellation::new(),
            LiveSyncOptions::default(),
        )
        .unwrap();
    assert_eq!(completed.phase, LiveSyncPhase::Current);
    assert_eq!(completed.scanned_height, Some(TARGET_HEIGHT));
    assert_eq!(completed.target_height, Some(TARGET_HEIGHT));
    assert_eq!(
        completed.confirmed_received_zat,
        Some(CONFIRMED_RECEIVED_ZAT)
    );
    assert_eq!(completed.pending_received_zat, Some(PENDING_RECEIVED_ZAT));
}

#[test]
#[ignore]
fn wal015_hot_journal_crash_child() {
    let database = PathBuf::from(
        env::var_os(CHILD_DATABASE_ENV).expect("owned live database path was not supplied"),
    );
    assert_eq!(database.file_name().unwrap(), "live.sqlite3");
    let owned_parent = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("wal006-state");
    assert!(database.starts_with(&owned_parent));
    let parent_metadata = fs::symlink_metadata(database.parent().unwrap()).unwrap();
    assert!(parent_metadata.file_type().is_dir() && !parent_metadata.file_type().is_symlink());
    assert_eq!(parent_metadata.permissions().mode() & 0o777, 0o700);
    private_regular_file(&database);

    let connection = Connection::open(&database).unwrap();
    connection
        .pragma_update(None, "journal_mode", "DELETE")
        .unwrap();
    connection
        .pragma_update(None, "synchronous", "FULL")
        .unwrap();
    connection.pragma_update(None, "cache_size", 1).unwrap();
    connection.pragma_update(None, "cache_spill", "ON").unwrap();
    connection.execute_batch("BEGIN IMMEDIATE;").unwrap();

    let prefix = "WAL015-HOT-JOURNAL-MARKER:";
    let mut marker = String::with_capacity(MARKER_BYTES);
    marker.push_str(prefix);
    marker.extend(std::iter::repeat_n('R', MARKER_BYTES - prefix.len()));
    assert_eq!(marker.len(), MARKER_BYTES);
    assert_eq!(
        connection
            .execute(
                "UPDATE ext_bitbook_live_state SET ufvk=?1 WHERE singleton=1",
                params![marker],
            )
            .unwrap(),
        1
    );
    assert!(private_regular_file(&journal_path(&database)).len() > 0);
    std::process::exit(CRASH_EXIT);
}

struct OwnedChild {
    child: Option<Child>,
}

impl OwnedChild {
    fn spawn(database: &Path) -> Self {
        let executable = env::current_exe().unwrap();
        let child = Command::new(executable)
            .args(["--ignored", "--exact", CHILD_TEST, "--test-threads=1"])
            .env(CHILD_DATABASE_ENV, database)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("spawn owned hot-journal crash child");
        Self { child: Some(child) }
    }

    fn wait_bounded(&mut self, timeout: Duration) -> ExitStatus {
        let deadline = Instant::now() + timeout;
        loop {
            match self.child.as_mut().unwrap().try_wait() {
                Ok(Some(status)) => {
                    self.child.take();
                    return status;
                }
                Ok(None) if Instant::now() < deadline => {
                    thread::sleep(Duration::from_millis(10));
                }
                Ok(None) => panic!("hot-journal crash child exceeded wait bound"),
                Err(error) => panic!("cannot wait for hot-journal crash child: {error}"),
            }
        }
    }
}

impl Drop for OwnedChild {
    fn drop(&mut self) {
        if let Some(mut child) = self.child.take() {
            let _ = child.kill();
            let deadline = Instant::now() + Duration::from_secs(2);
            loop {
                match child.try_wait() {
                    Ok(Some(_)) => return,
                    Ok(None) if Instant::now() < deadline => {
                        thread::sleep(Duration::from_millis(10));
                    }
                    Ok(None) | Err(_) => {
                        let _ = child.kill();
                        let _ = child.wait();
                        return;
                    }
                }
            }
        }
    }
}

fn private_regular_file(path: &Path) -> fs::Metadata {
    let metadata = fs::symlink_metadata(path).unwrap();
    assert!(metadata.file_type().is_file() && !metadata.file_type().is_symlink());
    assert_eq!(metadata.nlink(), 1);
    assert_eq!(metadata.permissions().mode() & 0o777, 0o600);
    metadata
}

fn journal_path(database: &Path) -> PathBuf {
    let name = format!(
        "{}-journal",
        database.file_name().unwrap().to_string_lossy()
    );
    database.with_file_name(name)
}
