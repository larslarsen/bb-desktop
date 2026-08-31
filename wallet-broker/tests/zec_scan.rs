use std::collections::BTreeSet;

use bitbook_wallet_broker::zec::test_support::{
    FrozenFixture, ScanFault, TestAccount, TestStateRoot,
};
use bitbook_wallet_broker::zec::{AccountId, MAX_COMPACT_BLOCK_BYTES, ScanError};

const ACCOUNT: &str = "00112233445566778899aabbccddeeff";
const FIXTURE_DIR: &str = "tests/fixtures/zec";
const BIRTHDAY_HEIGHT: u32 = 100;
const CHECKPOINT_HEIGHT: u32 = 99;
const NU6_3_HEIGHT: u32 = 102;
const CONFIRMED_HEIGHT: u32 = 106;
const ORCHARD_MIGRATION_ZAT: &str = "40000000";
const IRONWOOD_SPENDABLE_ZAT: &str = "150000000";
const REORG_VICTIM_IRONWOOD_PENDING_ZAT: &str = "30000000";
const REORG_REPLACEMENT_IRONWOOD_PENDING_ZAT: &str = "120000000";

fn fixture() -> FrozenFixture {
    FrozenFixture::open(FIXTURE_DIR).expect("reviewer-frozen WAL-006 fixture")
}

fn wallet(label: &str) -> TestAccount {
    TestAccount::bootstrap_from_fixture(
        TestStateRoot::fresh(label),
        AccountId::parse(ACCOUNT).unwrap(),
        &fixture(),
    )
    .unwrap()
}

#[test]
fn fixture_manifest_is_closed_ordered_hashed_and_bound_before_scan() {
    let fixture = fixture();
    let manifest = fixture.manifest();
    assert_eq!(manifest.format, "bitbook-zec-compact-fixture");
    assert_eq!(manifest.version, 1);
    assert_eq!(manifest.network.discriminator, "zec-local");
    assert_eq!(manifest.generator.zcash_client_backend, "0.24.0");
    assert_eq!(manifest.generator.zcash_client_sqlite, "0.22.0");
    assert_eq!(manifest.generator.pczt, "0.9.3");
    assert_eq!(manifest.generator.zcash_primitives, "0.30.1");
    assert_eq!(manifest.generator.zcash_protocol, "0.10.5");
    assert_eq!(manifest.generator.zcash_keys, "0.16.1");
    assert_eq!(manifest.network.sapling, BIRTHDAY_HEIGHT);
    assert_eq!(manifest.network.nu6_3, NU6_3_HEIGHT);
    assert_eq!(manifest.network.birthday_height, BIRTHDAY_HEIGHT);
    assert_eq!(manifest.network.checkpoint_height, CHECKPOINT_HEIGHT);
    assert_eq!(manifest.expected.orchard_migration_required_zat, 40_000_000);
    assert_eq!(manifest.expected.ironwood_spendable_zat, 150_000_000);
    assert_eq!(
        manifest.expected.reorg_victim_ironwood_pending_zat,
        30_000_000
    );
    assert_eq!(
        manifest.expected.reorg_replacement_ironwood_pending_zat,
        120_000_000
    );
    assert_eq!(manifest.expected.confirmation_height, CONFIRMED_HEIGHT);
    assert!(!manifest.expected.orchard_only_receiver.is_empty());
    assert_eq!(
        fixture.expected_destination_receiver(),
        manifest.expected.orchard_only_receiver
    );
    let expected_canonical = [
        "blocks/canonical-000100.compact".to_owned(),
        "blocks/canonical-000101.compact".to_owned(),
        "blocks/canonical-000102.compact".to_owned(),
        "blocks/canonical-000103.compact".to_owned(),
        "blocks/canonical-000104.compact".to_owned(),
        "blocks/canonical-000105.compact".to_owned(),
        "blocks/canonical-000106.compact".to_owned(),
        "blocks/canonical-000107.compact".to_owned(),
    ];
    assert_eq!(
        manifest.scenarios.canonical.as_slice(),
        expected_canonical.as_slice()
    );
    assert_eq!(fixture.canonical_block_count(), 8);
    assert_eq!(manifest.files.len(), 15);
    let unique_files = manifest
        .files
        .iter()
        .map(|file| file.name.as_str())
        .collect::<BTreeSet<_>>();
    assert_eq!(unique_files.len(), 15);
    assert_eq!(
        manifest.scenarios.one_block_reorg,
        "blocks/reorg-replacement-000107.compact"
    );
    assert!(!manifest.files.is_empty());
    for block in &manifest.files {
        assert!(block.name.starts_with("blocks/"));
        assert!(block.name.ends_with(".compact"));
        assert!(!block.name.starts_with('/'));
        assert!(!block.name.contains(".."));
        assert_eq!(block.sha256.len(), 64);
        assert_eq!(
            fixture.bytes(block).unwrap().len() as u64,
            block.byte_length
        );
        assert_eq!(fixture.sha256(block).unwrap(), block.sha256);
    }
    assert_eq!(
        fixture.decode_block(NU6_3_HEIGHT).unwrap().consensus_branch,
        0x37a5_165b
    );
}

#[test]
fn hostile_manifest_families_fail_before_any_scan_or_database_advance() {
    for mutation in [
        "unknown-field",
        "duplicate-entry",
        "path-traversal",
        "absolute-path",
        "wrong-length",
        "wrong-sha256",
        "wrong-network",
        "unsupported-version",
        "duplicate-json-key",
    ] {
        let hostile = fixture().mutated_manifest_for_test(mutation);
        let mut wallet = wallet("scan-hostile-manifest");
        let before = wallet.inspect_scan_state().unwrap();
        assert!(matches!(
            wallet.scan(&hostile).unwrap_err().code(),
            "SCHEMA" | "STATE_CORRUPT"
        ));
        assert_eq!(wallet.inspect_scan_state().unwrap(), before);
        assert_eq!(wallet.scan_calls(), 0, "{mutation} reached the scanner");
    }
}

#[test]
fn birthday_continuity_confirmation_and_unrelated_output_are_non_vacuous() {
    let mut wallet = wallet("scan-happy");
    assert_eq!(
        wallet.inspect_scan_state().unwrap().tip_height,
        CHECKPOINT_HEIGHT
    );
    wallet
        .scan_through(&fixture(), CONFIRMED_HEIGHT - 1)
        .unwrap();
    let before_confirmation = wallet.balances().unwrap();
    assert_eq!(
        before_confirmation.ironwood_pending_zat,
        IRONWOOD_SPENDABLE_ZAT
    );
    assert_eq!(before_confirmation.ironwood_spendable_zat, "0");
    assert_eq!(
        before_confirmation.orchard_migration_required_zat,
        ORCHARD_MIGRATION_ZAT
    );

    wallet.scan_through(&fixture(), CONFIRMED_HEIGHT).unwrap();
    let confirmed = wallet.balances().unwrap();
    assert_eq!(confirmed.ironwood_pending_zat, "0");
    assert_eq!(confirmed.ironwood_spendable_zat, IRONWOOD_SPENDABLE_ZAT);
    assert_eq!(
        confirmed.orchard_migration_required_zat,
        ORCHARD_MIGRATION_ZAT
    );
    assert_eq!(confirmed.transparent_zat, "0");
    assert_eq!(confirmed.sapling_zat, "0");
    assert_eq!(confirmed.total_zat, "190000000");
    assert_eq!(wallet.recognized_note_count(), 2);
    assert_eq!(wallet.unrelated_output_count_seen(), 1);
}

#[test]
fn replay_is_idempotent_and_close_reopen_preserves_exact_state() {
    let mut wallet = wallet("scan-replay");
    wallet.scan(&fixture()).unwrap();
    let before = wallet.inspect_scan_state().unwrap();
    wallet.scan(&fixture()).unwrap();
    assert_eq!(wallet.inspect_scan_state().unwrap(), before);
    assert_eq!(
        wallet.applied_block_count(),
        fixture().canonical_block_count()
    );

    let root = wallet.close().unwrap();
    let reopened = TestAccount::open_viewing(root, AccountId::parse(ACCOUNT).unwrap()).unwrap();
    let after = reopened.inspect_scan_state().unwrap();
    assert_eq!(after.tip_height, before.tip_height);
    assert_eq!(after.tip_hash, before.tip_hash);
    assert_eq!(after.tree_root, before.tree_root);
    assert_eq!(after.receiver_sequence, before.receiver_sequence);
    assert_eq!(after.balances, before.balances);
    assert_eq!(after.pool_classification, before.pool_classification);
}

#[test]
fn malformed_discontinuous_wrong_branch_and_impossible_tree_fail_without_tip_advance() {
    for scenario in [
        "truncated",
        "malformed",
        "wrong-previous-hash",
        "height-gap",
        "wrong-branch",
        "wrong-network",
        "impossible-tree-state",
    ] {
        let mut wallet = wallet("scan-corrupt-block");
        wallet.scan_through(&fixture(), 103).unwrap();
        let before = wallet.inspect_scan_state().unwrap();
        let error: ScanError = wallet.scan_scenario(&fixture(), scenario).unwrap_err();
        assert!(matches!(
            error.code(),
            "STATE_CORRUPT" | "PROTOCOL_INCOMPATIBLE"
        ));
        assert_eq!(
            wallet.inspect_scan_state().unwrap(),
            before,
            "scenario {scenario}"
        );
    }
}

#[test]
fn supported_one_block_reorg_rolls_back_exact_effects_and_applies_replacement() {
    let mut wallet = wallet("scan-reorg");
    wallet.scan(&fixture()).unwrap();
    let old = wallet.inspect_scan_state().unwrap();
    assert_eq!(old.balances.ironwood_spendable_zat, IRONWOOD_SPENDABLE_ZAT);
    assert_eq!(
        old.balances.ironwood_pending_zat,
        REORG_VICTIM_IRONWOOD_PENDING_ZAT
    );
    assert_eq!(old.balances.total_zat, "220000000");
    assert_eq!(wallet.recognized_note_count(), 3);
    wallet.scan_scenario(&fixture(), "one-block-reorg").unwrap();
    let replaced = wallet.inspect_scan_state().unwrap();
    assert_eq!(replaced.tip_height, old.tip_height);
    assert_ne!(replaced.tip_hash, old.tip_hash);
    assert_ne!(replaced.tree_root, old.tree_root);
    assert_eq!(
        replaced.balances.ironwood_spendable_zat,
        IRONWOOD_SPENDABLE_ZAT
    );
    assert_eq!(
        replaced.balances.ironwood_pending_zat,
        REORG_REPLACEMENT_IRONWOOD_PENDING_ZAT
    );
    assert_eq!(
        replaced.balances.orchard_migration_required_zat,
        ORCHARD_MIGRATION_ZAT
    );
    assert_eq!(replaced.balances.total_zat, "310000000");
    assert_eq!(wallet.recognized_note_count(), 3);
    assert_eq!(wallet.rolled_back_note_count(), 1);
    assert_eq!(wallet.rolled_back_block_count(), 1);
    assert_eq!(wallet.applied_replacement_note_count(), 1);
}

#[test]
fn deep_reorg_and_compound_rollback_failure_never_partially_mutate() {
    for (scenario, fault) in [
        ("two-block-reorg", None),
        ("one-block-reorg", Some(ScanFault::RollbackWrite)),
        ("one-block-reorg", Some(ScanFault::RollbackSync)),
        ("one-block-reorg", Some(ScanFault::ReplacementApply)),
    ] {
        let mut wallet = wallet("scan-compound-reorg");
        wallet.scan(&fixture()).unwrap();
        let before = wallet.inspect_scan_state().unwrap();
        if let Some(fault) = fault {
            wallet.arm_scan_fault(fault);
        }
        let error = wallet.scan_scenario(&fixture(), scenario).unwrap_err();
        assert!(matches!(
            error.code(),
            "STATE_CORRUPT" | "LIMIT" | "INTERNAL"
        ));
        assert_eq!(wallet.inspect_scan_state().unwrap(), before);
    }
}

#[test]
fn sqlite_corruption_during_scan_fails_closed_without_cache_or_tip_commit() {
    for fault in [
        ScanFault::WalletDbCorrupt,
        ScanFault::CacheDbCorrupt,
        ScanFault::CommitSync,
    ] {
        let mut wallet = wallet("scan-sqlite-corruption");
        wallet.scan_through(&fixture(), 103).unwrap();
        let before = wallet.inspect_scan_state().unwrap();
        wallet.arm_scan_fault(fault);
        assert_eq!(
            wallet.scan_through(&fixture(), 104).unwrap_err().code(),
            "STATE_CORRUPT"
        );
        assert_eq!(wallet.inspect_scan_state().unwrap(), before);
    }
}

#[test]
fn checked_balance_and_compact_block_limits_cover_below_at_and_above() {
    let mut wallet = wallet("scan-limits");
    for (value, accepted) in [(u64::MAX - 1, true), (u64::MAX, true)] {
        wallet.set_balance_for_test(value).unwrap();
        assert_eq!(wallet.balances().unwrap().total_zat, value.to_string());
    }
    wallet.set_balance_for_test(u64::MAX).unwrap();
    assert_eq!(
        wallet.add_recognized_value_for_test(1).unwrap_err().code(),
        "LIMIT"
    );
    assert_eq!(wallet.balances().unwrap().total_zat, u64::MAX.to_string());

    for (length, accepted) in [
        (MAX_COMPACT_BLOCK_BYTES - 1, true),
        (MAX_COMPACT_BLOCK_BYTES, true),
        (MAX_COMPACT_BLOCK_BYTES + 1, false),
    ] {
        let result = wallet.decode_sized_compact_block_for_test(length);
        assert_eq!(result.is_ok(), accepted);
        if !accepted {
            assert_eq!(result.unwrap_err().code(), "LIMIT");
            assert_eq!(wallet.last_block_allocation(), None);
        }
    }
}
