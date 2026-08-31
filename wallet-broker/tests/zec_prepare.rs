use bitbook_wallet_broker::zec::test_support::{
    CanonicalNumericField, FrozenFixture, ManualClock, PoolInventory, PrepareBinding,
    PrepareMutation, PreparedInspection, TestAccount, TestStateRoot, parse_canonical_u64_for_test,
};
use bitbook_wallet_broker::zec::{AccountId, MAX_MEMO_BYTES, MAX_PREPARED_HANDLES, PrepareZecV1};

const ACCOUNT: &str = "00112233445566778899aabbccddeeff";
const OTHER_ACCOUNT: &str = "ffeeddccbbaa99887766554433221100";
const REQUEST_ID: &str = "00112233445566778899aabbccddeeff";
const INTENT_HASH: &str = "ad55816f327c002be813a29d41f9a7ae429782b6856a4d3bb2e6c498c6f9e3c0";
const MEMO_HASH: &str = "37290d74ac4d186e3a8e5785d259d2ec04fac91ae28092e7620ec8bc99e830aa";
const AMOUNT_ZAT: &str = "100000000";
const STANDARD_FEE_ZAT: &str = "10000";
const FEE_BOUND_ZAT: &str = "12000";
const EXPIRES_AT: &str = "2026-08-30T12:15:00Z";
const NOW: &str = "2026-08-30T12:00:30Z";
const FIXTURE_DIR: &str = "tests/fixtures/zec";

fn fixture() -> FrozenFixture {
    FrozenFixture::open(FIXTURE_DIR).unwrap()
}

fn wallet(label: &str) -> TestAccount {
    let mut wallet = TestAccount::bootstrap_from_fixture(
        TestStateRoot::fresh(label),
        AccountId::parse(ACCOUNT).unwrap(),
        &fixture(),
    )
    .unwrap();
    wallet.scan(&fixture()).unwrap();
    wallet.unlock_with_fixture_seed().unwrap();
    wallet
}

fn input() -> PrepareZecV1 {
    PrepareZecV1::new(
        ACCOUNT,
        "zec-local",
        REQUEST_ID,
        INTENT_HASH,
        fixture().expected_destination_receiver(),
        AMOUNT_ZAT,
        FEE_BOUND_ZAT,
        "coffee",
        EXPIRES_AT,
    )
    .unwrap()
}

fn assert_exact_inspection(inspection: &PreparedInspection) {
    assert_eq!(inspection.network, "zec-local");
    assert_eq!(inspection.consensus_branch, 0x37a5_165b);
    assert_eq!(inspection.transaction_version, 6);
    assert_eq!(
        inspection.destination,
        fixture().expected_destination_receiver()
    );
    assert_eq!(inspection.amount_zat, AMOUNT_ZAT);
    assert_eq!(inspection.memo_sha256, MEMO_HASH);
    assert_eq!(inspection.fee_zat, STANDARD_FEE_ZAT);
    assert_eq!(inspection.ironwood_inputs, 1);
    assert_eq!(inspection.ironwood_outputs, 2);
    assert!(!inspection.has_transparent_bundle);
    assert!(!inspection.has_sapling_bundle);
    assert!(!inspection.has_orchard_output_bundle);
    assert!(!inspection.has_signatures);
    assert!(!inspection.has_proofs);
    assert!(!inspection.finalized);
    assert!(!inspection.extractable);
}

#[test]
fn sufficient_confirmed_ironwood_prepares_sanitized_v6_handle_and_decoded_pczt() {
    let mut wallet = wallet("prepare-happy");
    let prepared = wallet.prepare(input(), &mut ManualClock::at(NOW)).unwrap();
    assert_eq!(prepared.handle.len(), 32);
    assert!(
        prepared
            .handle
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    );
    assert_eq!(prepared.account_id, ACCOUNT);
    assert_eq!(prepared.network, "zec-local");
    assert_eq!(prepared.request_id, REQUEST_ID);
    assert_eq!(prepared.intent_hash, INTENT_HASH);
    assert_eq!(prepared.receiver, fixture().expected_destination_receiver());
    assert_eq!(prepared.amount_zat, AMOUNT_ZAT);
    assert_eq!(prepared.fee_zat, STANDARD_FEE_ZAT);
    assert_eq!(prepared.fee_bound_zat, FEE_BOUND_ZAT);
    assert_eq!(prepared.expires_at, EXPIRES_AT);
    assert_eq!(prepared.tx_version, "6");
    assert_eq!(prepared.consensus_branch, "37a5165b");
    assert_eq!(prepared.spend_pool, "ironwood");
    assert_eq!(prepared.output_pool, "ironwood");
    assert!(!prepared.signed && !prepared.extractable && !prepared.broadcastable);
    assert_exact_inspection(&wallet.inspect_prepared_for_test(&prepared.handle).unwrap());
}

#[test]
fn typed_prepare_validation_fails_before_spend_material_access() {
    let rows = [
        PrepareMutation::AccountId("00112233445566778899AABBCCDDEEFF".to_owned()),
        PrepareMutation::Network("zec-mainnet".to_owned()),
        PrepareMutation::RequestId("001122".to_owned()),
        PrepareMutation::IntentHash(
            "AD55816F327C002BE813A29D41F9A7AE429782B6856A4D3BB2E6C498C6F9E3C0".to_owned(),
        ),
        PrepareMutation::Amount("0".to_owned()),
        PrepareMutation::Amount("01".to_owned()),
        PrepareMutation::Amount("1e8".to_owned()),
        PrepareMutation::Amount("18446744073709551616".to_owned()),
        PrepareMutation::FeeBound("0".to_owned()),
        PrepareMutation::FeeBound("01".to_owned()),
        PrepareMutation::FeeBound("1.25".to_owned()),
        PrepareMutation::ExpiresAt("2026-02-30T12:15:00Z".to_owned()),
        PrepareMutation::ExpiresAt("2101-01-01T00:00:00Z".to_owned()),
    ];
    for mutation in rows {
        let mut wallet = wallet("prepare-validation");
        wallet.reset_spend_access_observer();
        let error = wallet
            .prepare_mutated_for_test(input(), mutation, &mut ManualClock::at(NOW))
            .unwrap_err();
        assert!(matches!(error.code(), "SCHEMA" | "NETWORK_DISABLED"));
        assert_eq!(wallet.spend_access_count(), 0);
        assert_eq!(wallet.prepared_handle_count(), 0);
    }
}

#[test]
fn closed_input_lengths_and_u64_parsing_cover_immediate_below_at_and_above() {
    for (length, accepted) in [(31, false), (32, true), (33, false)] {
        let mut wallet = wallet("prepare-request-id-length");
        wallet.reset_spend_access_observer();
        let result = wallet.prepare_mutated_for_test(
            input(),
            PrepareMutation::RequestId("a".repeat(length)),
            &mut ManualClock::at(NOW),
        );
        assert_eq!(result.is_ok(), accepted, "request ID length {length}");
        if accepted {
            assert!(wallet.spend_access_count() > 0);
            assert_eq!(wallet.prepared_handle_count(), 1);
        } else {
            assert_eq!(result.unwrap_err().code(), "SCHEMA");
            assert_eq!(wallet.spend_access_count(), 0);
            assert_eq!(wallet.prepared_handle_count(), 0);
        }
    }

    for (length, accepted) in [(63, false), (64, true), (65, false)] {
        let mut wallet = wallet("prepare-intent-hash-length");
        wallet.reset_spend_access_observer();
        let result = wallet.prepare_mutated_for_test(
            input(),
            PrepareMutation::IntentHash("b".repeat(length)),
            &mut ManualClock::at(NOW),
        );
        assert_eq!(result.is_ok(), accepted, "intent hash length {length}");
        if accepted {
            assert!(wallet.spend_access_count() > 0);
            assert_eq!(wallet.prepared_handle_count(), 1);
        } else {
            assert_eq!(result.unwrap_err().code(), "SCHEMA");
            assert_eq!(wallet.spend_access_count(), 0);
            assert_eq!(wallet.prepared_handle_count(), 0);
        }
    }

    for field in [
        CanonicalNumericField::Amount,
        CanonicalNumericField::FeeBound,
    ] {
        for (value, expected) in [
            ("18446744073709551614", Some(u64::MAX - 1)),
            ("18446744073709551615", Some(u64::MAX)),
            ("18446744073709551616", None),
            ("0", None),
            ("01", None),
            ("1e8", None),
        ] {
            let result = parse_canonical_u64_for_test(field, value);
            match expected {
                Some(expected) => assert_eq!(result.unwrap(), expected),
                None => assert!(matches!(result.unwrap_err().code(), "SCHEMA" | "LIMIT")),
            }
        }
    }
}

#[test]
fn other_account_stale_session_and_mismatched_binding_fail_before_spend_access() {
    let mut other_account = wallet("prepare-other-account");
    other_account.reset_spend_access_observer();
    let error = other_account
        .prepare_mutated_for_test(
            input(),
            PrepareMutation::AccountId(OTHER_ACCOUNT.to_owned()),
            &mut ManualClock::at(NOW),
        )
        .unwrap_err();
    assert_eq!(error.code(), "SCHEMA");
    assert_eq!(other_account.spend_access_count(), 0);
    assert_eq!(other_account.prepared_handle_count(), 0);

    for mismatch in ["other-account", "stale-session"] {
        let mut wallet = wallet("prepare-session-binding");
        wallet.reset_spend_access_observer();
        let binding = if mismatch == "other-account" {
            PrepareBinding::new(OTHER_ACCOUNT, wallet.session_id(), REQUEST_ID, INTENT_HASH)
                .unwrap()
        } else {
            PrepareBinding::new(
                ACCOUNT,
                "ffffffffffffffffffffffffffffffff",
                REQUEST_ID,
                INTENT_HASH,
            )
            .unwrap()
        };
        let error = wallet
            .prepare_with_binding(input(), binding, &mut ManualClock::at(NOW))
            .unwrap_err();
        assert_eq!(error.code(), "LOCKED");
        assert_eq!(wallet.spend_access_count(), 0);
        assert_eq!(wallet.prepared_handle_count(), 0);
    }
}

#[test]
fn receiver_network_and_composition_reject_downgrade_without_fallback() {
    for receiver in [
        fixture().wrong_network_receiver(),
        fixture().orchard_plus_p2pkh_receiver(),
        fixture().orchard_plus_sapling_receiver(),
        fixture().unknown_item_receiver(),
    ] {
        let mut wallet = wallet("prepare-receiver");
        let error = wallet
            .prepare_with_receiver(input(), receiver, &mut ManualClock::at(NOW))
            .unwrap_err();
        assert!(matches!(
            error.code(),
            "SCHEMA" | "TRANSPARENT_DOWNGRADE" | "PROTOCOL_INCOMPATIBLE"
        ));
        assert_eq!(wallet.spend_access_count(), 0);
        assert_eq!(wallet.prepared_handle_count(), 0);
    }
}

#[test]
fn memo_boundaries_are_utf8_nfc_and_reject_controls_before_prepare() {
    assert_eq!(MAX_MEMO_BYTES, 512);
    for (memo, accepted) in [
        ("m".repeat(511), true),
        ("m".repeat(512), true),
        ("m".repeat(513), false),
        ("cafe\u{301}".to_owned(), false),
        ("safe\u{202e}text".to_owned(), false),
        ("safe\u{200b}text".to_owned(), false),
    ] {
        let mut wallet = wallet("prepare-memo-boundary");
        let result = wallet.prepare_with_memo(input(), memo, &mut ManualClock::at(NOW));
        assert_eq!(result.is_ok(), accepted);
        if accepted {
            assert_eq!(wallet.prepared_handle_count(), 1);
        } else {
            assert_eq!(result.unwrap_err().code(), "SCHEMA");
            assert_eq!(wallet.spend_access_count(), 0);
            assert_eq!(wallet.prepared_handle_count(), 0);
        }
    }
}

#[test]
fn pool_outcome_table_never_substitutes_total_for_ironwood_spendable() {
    let rows = [
        (PoolInventory::confirmed_ironwood("150010000"), None),
        (
            PoolInventory::mixed("90000000", "20010000"),
            Some("MIGRATION_REQUIRED"),
        ),
        (
            PoolInventory::orchard("150010000"),
            Some("MIGRATION_REQUIRED"),
        ),
        (
            PoolInventory::transparent("150010000"),
            Some("CAPABILITY_MISSING"),
        ),
        (
            PoolInventory::sapling("150010000"),
            Some("CAPABILITY_MISSING"),
        ),
        (
            PoolInventory::mixed_with_sufficient_ironwood("150010000", "80000000"),
            None,
        ),
        (
            PoolInventory::unconfirmed_ironwood("150010000"),
            Some("INSUFFICIENT_FUNDS"),
        ),
        (
            PoolInventory::locked_ironwood("150010000"),
            Some("INSUFFICIENT_FUNDS"),
        ),
    ];
    for (inventory, expected_error) in rows {
        let mut wallet = wallet("prepare-pool-table");
        wallet.replace_inventory_for_test(inventory);
        let result = wallet.prepare(input(), &mut ManualClock::at(NOW));
        match expected_error {
            Some(code) => assert_eq!(result.unwrap_err().code(), code),
            None => {
                let prepared = result.unwrap();
                let inspected = wallet.inspect_prepared_for_test(&prepared.handle).unwrap();
                assert_eq!(inspected.spend_pool, "ironwood");
                assert_eq!(inspected.legacy_input_value_zat, "0");
            }
        }
    }
}

#[test]
fn standard_fee_is_authoritative_and_bound_covers_below_at_and_above() {
    for (bound, expected) in [
        ("9999", Some("FEE_BOUND")),
        ("10000", None),
        ("10001", None),
    ] {
        let mut wallet = wallet("prepare-fee-boundary");
        let result = wallet.prepare_with_fee_bound(input(), bound, &mut ManualClock::at(NOW));
        match expected {
            Some(code) => {
                assert_eq!(result.unwrap_err().code(), code);
                assert_eq!(wallet.prepared_handle_count(), 0);
            }
            None => assert_eq!(result.unwrap().fee_zat, STANDARD_FEE_ZAT),
        }
        assert_eq!(wallet.fee_rule_calls(), 1);
        assert_eq!(wallet.caller_fee_calls(), 0);
    }
}

#[test]
fn expiry_and_lock_are_rechecked_at_exact_boundary() {
    for (now, expected) in [
        ("2026-08-30T12:14:59Z", None),
        (EXPIRES_AT, Some("EXPIRED")),
        ("2026-08-30T12:15:01Z", Some("EXPIRED")),
    ] {
        let mut wallet = wallet("prepare-expiry");
        let result = wallet.prepare(input(), &mut ManualClock::at(now));
        match expected {
            Some(code) => assert_eq!(result.unwrap_err().code(), code),
            None => assert!(result.is_ok()),
        }
    }
    let mut locked = wallet("prepare-locked");
    locked.reset_spend_access_observer();
    locked.lock().unwrap();
    assert_eq!(
        locked
            .prepare(input(), &mut ManualClock::at(NOW))
            .unwrap_err()
            .code(),
        "LOCKED"
    );
    assert_eq!(locked.spend_access_count(), 0);
    assert_eq!(locked.prepared_handle_count(), 0);
}

#[test]
fn exact_wal002_intent_values_and_independent_hashes_survive_sanitization() {
    let mut wallet = wallet("prepare-intent-vector");
    let prepared = wallet.prepare(input(), &mut ManualClock::at(NOW)).unwrap();
    assert_eq!(prepared.request_id, REQUEST_ID);
    assert_eq!(prepared.intent_hash, INTENT_HASH);
    assert_eq!(prepared.amount_zat, AMOUNT_ZAT);
    assert_eq!(prepared.fee_bound_zat, FEE_BOUND_ZAT);
    assert_eq!(prepared.expires_at, EXPIRES_AT);
    let inspected = wallet.inspect_prepared_for_test(&prepared.handle).unwrap();
    assert_eq!(inspected.memo_sha256, MEMO_HASH);
    assert_eq!(inspected.intent_hash_binding, INTENT_HASH);
    assert_eq!(inspected.request_id_binding, REQUEST_ID);
}

#[test]
fn prepared_handle_limit_covers_immediate_below_at_and_above_without_eviction() {
    let mut wallet = wallet("prepare-handle-limit");
    wallet
        .fill_prepared_handles_for_test(MAX_PREPARED_HANDLES - 1)
        .unwrap();
    assert!(wallet.prepare(input(), &mut ManualClock::at(NOW)).is_ok());
    assert_eq!(wallet.prepared_handle_count(), MAX_PREPARED_HANDLES);
    assert_eq!(
        wallet
            .prepare(input(), &mut ManualClock::at(NOW))
            .unwrap_err()
            .code(),
        "LIMIT"
    );
    assert_eq!(wallet.prepared_handle_count(), MAX_PREPARED_HANDLES);
    assert_eq!(
        wallet
            .fill_prepared_handles_for_test(MAX_PREPARED_HANDLES + 1)
            .unwrap_err()
            .code(),
        "LIMIT"
    );
}
