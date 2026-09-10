use std::fs;
use std::os::unix::fs::PermissionsExt;

use bitbook_wallet_broker::zec::test_support::{
    FrozenFixture, LiveEngineFault, LiveSourceFault, LiveStoreEntryKind, LiveSyncHarness,
    RecordedLiveSource, validate_live_batch_shape_for_test,
};
use bitbook_wallet_broker::zec::{
    AccountId, LiveCancellation, LiveEndpoint, LiveSyncOptions, LiveSyncPhase,
    MAX_COMPACT_BLOCK_BYTES, MAX_LIVE_BATCH_BLOCKS, MAX_LIVE_BATCH_BYTES, MAX_LIVE_ENDPOINT_BYTES,
    Network,
};

const ACCOUNT: &str = "00112233445566778899aabbccddeeff";
const FIXTURE_DIR: &str = "tests/fixtures/zec";
const SUGGESTED_ENDPOINT: &str = "https://testnet.zec.rocks:443";
const TARGET_HEIGHT: u32 = 107;
const CONFIRMED_RECEIVED_ZAT: u64 = 190_000_000;
const PENDING_RECEIVED_ZAT: u64 = 30_000_000;

fn fixture() -> FrozenFixture {
    FrozenFixture::open(FIXTURE_DIR).expect("reviewer-frozen WAL-006 compact blocks")
}

fn wallet(label: &str) -> LiveSyncHarness {
    LiveSyncHarness::bootstrap_from_fixture(label, AccountId::parse(ACCOUNT).unwrap(), &fixture())
        .unwrap()
}

fn source() -> RecordedLiveSource {
    RecordedLiveSource::canonical(&fixture()).unwrap()
}

#[test]
fn endpoint_is_https_bounded_and_has_no_ambient_or_redirect_surface() {
    let endpoint = LiveEndpoint::parse(SUGGESTED_ENDPOINT).unwrap();
    assert_eq!(endpoint.as_str(), SUGGESTED_ENDPOINT);
    assert_eq!(endpoint.host(), "testnet.zec.rocks");
    assert_eq!(endpoint.port(), 443);

    for invalid in [
        "",
        "http://testnet.zec.rocks:443",
        "https://",
        "https://user@testnet.zec.rocks:443",
        "https://testnet.zec.rocks:0",
        "https://testnet.zec.rocks:65536",
        "https://testnet.zec.rocks:443/path",
        "https://testnet.zec.rocks:443/?query=yes",
        "https://testnet.zec.rocks:443/#fragment",
        "https://testnet.zec.rocks:443\n",
        "https://tést.example:443",
    ] {
        assert!(
            LiveEndpoint::parse(invalid).is_err(),
            "accepted {invalid:?}"
        );
    }
    let valid_host = format!(
        "{}.{}.{}.{}",
        "a".repeat(63),
        "b".repeat(63),
        "c".repeat(63),
        "d".repeat(61)
    );
    assert_eq!(valid_host.len(), 253);
    assert!(LiveEndpoint::parse(&format!("https://{valid_host}:443")).is_ok());
    let oversized = format!("https://{}:443", "a".repeat(MAX_LIVE_ENDPOINT_BYTES - 12));
    assert_eq!(oversized.len(), MAX_LIVE_ENDPOINT_BYTES);
    assert!(LiveEndpoint::parse(&oversized).is_err());
    assert!(LiveEndpoint::parse(&format!("{oversized}a")).is_err());
}

#[test]
fn recorded_blocks_use_real_walletdb_scan_and_publish_nonzero_received_funds() {
    let mut wallet = wallet("wal015-live-happy");
    let mut source = source();
    let snapshot = wallet
        .sync(
            &mut source,
            &LiveCancellation::new(),
            LiveSyncOptions::default(),
        )
        .unwrap();

    assert_eq!(snapshot.phase, LiveSyncPhase::Current);
    assert_eq!(snapshot.account_id.as_str(), ACCOUNT);
    assert!(matches!(snapshot.network, Network::Local(_)));
    assert_eq!(snapshot.scanned_height, Some(TARGET_HEIGHT));
    assert_eq!(snapshot.target_height, Some(TARGET_HEIGHT));
    assert_eq!(
        snapshot.confirmed_received_zat,
        Some(CONFIRMED_RECEIVED_ZAT)
    );
    assert_eq!(snapshot.pending_received_zat, Some(PENDING_RECEIVED_ZAT));

    let engine = wallet.engine_observation();
    assert_eq!(engine.wallet_db_transactions, 1);
    assert_eq!(engine.scan_cached_blocks_calls, 1);
    assert_eq!(
        engine.decoded_recorded_blocks,
        fixture().canonical_block_count()
    );
    assert_eq!(engine.imported_viewing_accounts, 1);
    let methods = &source.observation().rpc_methods;
    assert!(methods.iter().all(|method| matches!(
        method.as_str(),
        "GetLightdInfo" | "GetLatestBlock" | "GetTreeState" | "GetBlockRange"
    )));
    for required in [
        "GetLightdInfo",
        "GetLatestBlock",
        "GetTreeState",
        "GetBlockRange",
    ] {
        assert!(methods.iter().any(|method| method == required));
    }
}

#[test]
fn restart_resumes_committed_height_and_replay_is_idempotent_in_separate_live_cache() {
    let mut wallet = wallet("wal015-live-resume");
    let wallet_before = fs::read(wallet.receive_wallet_path()).unwrap();
    let compact_before = fs::read(wallet.fixture_cache_path()).unwrap();
    let receiver_before = wallet.receive_state().unwrap();

    let mut first = source();
    wallet
        .sync_through(
            &mut first,
            103,
            &LiveCancellation::new(),
            LiveSyncOptions::default(),
        )
        .unwrap();
    assert_eq!(wallet.inspect().unwrap().committed_height, Some(103));
    let restart = wallet.close().unwrap();
    let mut reopened = LiveSyncHarness::reopen(restart, &fixture()).unwrap();
    let mut resumed = source();
    reopened
        .sync(
            &mut resumed,
            &LiveCancellation::new(),
            LiveSyncOptions::default(),
        )
        .unwrap();
    assert_eq!(
        resumed.observation().requested_ranges,
        [(104, TARGET_HEIGHT)]
    );
    let committed = fs::read(reopened.live_cache_path()).unwrap();

    let mut replay = source();
    let replayed = reopened
        .sync(
            &mut replay,
            &LiveCancellation::new(),
            LiveSyncOptions::default(),
        )
        .unwrap();
    assert_eq!(replayed.phase, LiveSyncPhase::Current);
    assert!(replay.observation().requested_ranges.is_empty());
    assert_eq!(fs::read(reopened.live_cache_path()).unwrap(), committed);
    assert_eq!(
        fs::read(reopened.receive_wallet_path()).unwrap(),
        wallet_before
    );
    assert_eq!(
        fs::read(reopened.fixture_cache_path()).unwrap(),
        compact_before
    );
    assert_eq!(reopened.receive_state().unwrap(), receiver_before);
}

#[test]
fn cancellation_interrupts_source_wait_and_keeps_only_the_last_committed_batch() {
    let mut wallet = wallet("wal015-live-cancel");
    let mut source = source();
    let cancellation = LiveCancellation::new();
    source.cancel_before_range_request(2, cancellation.clone());
    let options = LiveSyncOptions::for_test(3).unwrap();

    let error = wallet
        .sync(&mut source, &cancellation, options)
        .unwrap_err();
    assert_eq!(error.code(), "CANCELLED");
    assert!(source.observation().transport_wait_interrupted);
    assert_eq!(wallet.inspect().unwrap().committed_height, Some(102));
    assert_eq!(wallet.inspect().unwrap().provisional_height, None);
    assert_eq!(wallet.inspect().unwrap().phase, LiveSyncPhase::Stale);

    let mut resumed = self::source();
    let completed = wallet
        .sync(
            &mut resumed,
            &LiveCancellation::new(),
            LiveSyncOptions::default(),
        )
        .unwrap();
    assert_eq!(
        resumed.observation().requested_ranges,
        [(103, TARGET_HEIGHT)]
    );
    assert_eq!(completed.phase, LiveSyncPhase::Current);
}

#[test]
fn malformed_server_observations_fail_before_descendant_scan_or_checkpoint_commit() {
    for fault in [
        LiveSourceFault::WrongNetwork,
        LiveSourceFault::ProtocolBeforeNu5,
        LiveSourceFault::TipHeightOverflow,
        LiveSourceFault::TipHashWrongLength,
        LiveSourceFault::TreeStateNetwork,
        LiveSourceFault::TreeStateHeight,
        LiveSourceFault::TreeStateHash,
        LiveSourceFault::TreeStateOversized,
        LiveSourceFault::MissingBlock,
        LiveSourceFault::DuplicateBlock,
        LiveSourceFault::ExtraBlock,
        LiveSourceFault::WrongPreviousHash,
        LiveSourceFault::BlockOversized,
    ] {
        let mut wallet = wallet("wal015-live-hostile-source");
        let before = wallet.inspect().unwrap();
        let mut source = source();
        source.arm_fault(fault);
        let error = wallet
            .sync(
                &mut source,
                &LiveCancellation::new(),
                LiveSyncOptions::default(),
            )
            .unwrap_err();
        assert!(
            matches!(
                error.code(),
                "PROTOCOL_INCOMPATIBLE" | "LIMIT" | "RESCAN_REQUIRED"
            ),
            "unexpected code for {fault:?}: {}",
            error.code()
        );
        assert_eq!(source.observation().mutated_response_count, 1);
        assert_eq!(wallet.inspect().unwrap(), before, "fault {fault:?}");
        assert_eq!(wallet.engine_observation().scan_cached_blocks_calls, 0);
    }
}

#[test]
fn batch_defaults_and_size_boundaries_are_exact_and_bounded() {
    assert_eq!(
        LiveSyncOptions::default().batch_blocks(),
        MAX_LIVE_BATCH_BLOCKS
    );
    assert_eq!(MAX_LIVE_BATCH_BLOCKS, 100);
    assert_eq!(MAX_LIVE_BATCH_BYTES, 32 * 1024 * 1024);
    for (blocks, accepted) in [(1, true), (100, true), (101, false), (0, false)] {
        assert_eq!(LiveSyncOptions::for_test(blocks).is_ok(), accepted);
    }
    assert!(validate_live_batch_shape_for_test(&[MAX_COMPACT_BLOCK_BYTES; 16], 16).is_ok());
    assert_eq!(
        validate_live_batch_shape_for_test(&[MAX_COMPACT_BLOCK_BYTES; 17], 17)
            .unwrap_err()
            .code(),
        "LIMIT"
    );
    assert_eq!(
        validate_live_batch_shape_for_test(&[MAX_COMPACT_BLOCK_BYTES + 1], 1)
            .unwrap_err()
            .code(),
        "LIMIT"
    );
    assert_eq!(
        validate_live_batch_shape_for_test(&[1, 1], 3)
            .unwrap_err()
            .code(),
        "PROTOCOL_INCOMPATIBLE"
    );

    let mut wallet = wallet("wal015-live-bounds");
    let mut source = source();
    wallet
        .sync(
            &mut source,
            &LiveCancellation::new(),
            LiveSyncOptions::for_test(3).unwrap(),
        )
        .unwrap();
    assert_eq!(
        source.observation().requested_ranges,
        [(100, 102), (103, 105), (106, 107)]
    );
    assert_eq!(source.observation().returned_counts, [3, 3, 2]);
    assert!(
        source
            .observation()
            .returned_bytes
            .iter()
            .all(|bytes| *bytes <= MAX_LIVE_BATCH_BYTES)
    );
}

#[test]
fn transaction_failure_discards_provisional_scan_and_preserves_durable_bytes() {
    for fault in [
        LiveEngineFault::ScanWrite,
        LiveEngineFault::CheckpointWrite,
        LiveEngineFault::Commit,
    ] {
        let mut wallet = wallet("wal015-live-atomicity");
        let mut prefix = source();
        wallet
            .sync_through(
                &mut prefix,
                103,
                &LiveCancellation::new(),
                LiveSyncOptions::default(),
            )
            .unwrap();
        let before = wallet.inspect().unwrap();
        let bytes_before = fs::read(wallet.live_cache_path()).unwrap();
        wallet.arm_engine_fault(fault);
        let mut tail = source();
        assert_eq!(
            wallet
                .sync(
                    &mut tail,
                    &LiveCancellation::new(),
                    LiveSyncOptions::default(),
                )
                .unwrap_err()
                .code(),
            "STATE_CORRUPT"
        );
        assert_eq!(wallet.inspect().unwrap(), before, "fault {fault:?}");
        assert_eq!(fs::read(wallet.live_cache_path()).unwrap(), bytes_before);
    }
}

#[test]
fn bounded_reorg_uses_upstream_truncation_and_deep_fork_requires_explicit_rescan() {
    let mut wallet = wallet("wal015-live-reorg");
    let mut canonical = source();
    wallet
        .sync(
            &mut canonical,
            &LiveCancellation::new(),
            LiveSyncOptions::default(),
        )
        .unwrap();
    let old = wallet.inspect().unwrap();

    let mut replacement = RecordedLiveSource::one_block_reorg(&fixture()).unwrap();
    let replaced = wallet
        .sync(
            &mut replacement,
            &LiveCancellation::new(),
            LiveSyncOptions::default(),
        )
        .unwrap();
    assert_eq!(replaced.phase, LiveSyncPhase::Current);
    assert_eq!(replaced.scanned_height, Some(TARGET_HEIGHT));
    assert_ne!(wallet.inspect().unwrap().tip_hash, old.tip_hash);
    assert_eq!(wallet.engine_observation().upstream_truncations, 1);
    assert_eq!(
        replaced.confirmed_received_zat,
        Some(CONFIRMED_RECEIVED_ZAT)
    );
    assert_eq!(replaced.pending_received_zat, Some(120_000_000));

    let before_deep = wallet.inspect().unwrap();
    let mut deep = RecordedLiveSource::fork_beyond_retained_checkpoint(101);
    let error = wallet
        .sync(
            &mut deep,
            &LiveCancellation::new(),
            LiveSyncOptions::default(),
        )
        .unwrap_err();
    assert_eq!(error.code(), "RESCAN_REQUIRED");
    assert_eq!(deep.observation().reported_ancestor_depth, Some(101));
    assert!((8..=100).contains(&deep.observation().tree_states_returned));
    assert!(
        deep.observation()
            .requested_tree_heights
            .iter()
            .all(|height| *height >= 99)
    );
    assert_eq!(wallet.inspect().unwrap(), before_deep);
}

#[test]
fn live_cache_binding_permissions_and_hostile_entries_fail_closed_without_replacement() {
    let wallet = wallet("wal015-live-paths");
    assert_eq!(
        wallet.live_cache_path().file_name().unwrap(),
        "live.sqlite3"
    );
    assert_ne!(wallet.live_cache_path(), wallet.receive_wallet_path());
    assert_ne!(wallet.live_cache_path(), wallet.fixture_cache_path());
    assert_eq!(
        fs::symlink_metadata(wallet.account_directory())
            .unwrap()
            .permissions()
            .mode()
            & 0o777,
        0o700
    );
    assert_eq!(
        fs::symlink_metadata(wallet.live_cache_path())
            .unwrap()
            .permissions()
            .mode()
            & 0o777,
        0o600
    );
    drop(wallet);

    for kind in [
        LiveStoreEntryKind::Symlink,
        LiveStoreEntryKind::HardLink,
        LiveStoreEntryKind::WrongMode,
        LiveStoreEntryKind::WrongOwner,
        LiveStoreEntryKind::HostileJournal,
        LiveStoreEntryKind::HostileWal,
        LiveStoreEntryKind::HostileShm,
        LiveStoreEntryKind::CorruptDatabase,
        LiveStoreEntryKind::PartialSchema,
        LiveStoreEntryKind::ForeignUfvk,
    ] {
        let mut wallet = LiveSyncHarness::with_hostile_live_entry(
            "wal015-live-hostile-path",
            AccountId::parse(ACCOUNT).unwrap(),
            &fixture(),
            kind,
        );
        let marker = wallet.hostile_entry_marker();
        let mut source = source();
        assert_eq!(
            wallet
                .sync(
                    &mut source,
                    &LiveCancellation::new(),
                    LiveSyncOptions::default(),
                )
                .unwrap_err()
                .code(),
            "STATE_CORRUPT",
            "kind {kind:?}"
        );
        assert_eq!(wallet.hostile_entry_marker(), marker, "kind {kind:?}");
        assert!(source.observation().rpc_methods.is_empty(), "kind {kind:?}");
    }
}
