use std::collections::BTreeSet;
use std::panic::{AssertUnwindSafe, catch_unwind};

use bitbook_wallet_broker::zec::test_support::{
    FrozenFixture, ManualClock, ObservableCanary, ObservableSecretClass, RecordingWipes,
    TestAccount, TestStateRoot,
};
use bitbook_wallet_broker::zec::{
    AccountId, HandleBinding, HandleInvalidation, MAX_DIAGNOSTIC_BYTES,
};
use sha2::{Digest, Sha256};

const ACCOUNT: &str = "00112233445566778899aabbccddeeff";
const OTHER_ACCOUNT: &str = "ffeeddccbbaa99887766554433221100";
const REQUEST_ID: &str = "00112233445566778899aabbccddeeff";
const OTHER_REQUEST: &str = "11112222333344445555666677778888";
const INTENT_HASH: &str = "ad55816f327c002be813a29d41f9a7ae429782b6856a4d3bb2e6c498c6f9e3c0";
const OTHER_INTENT: &str = "0d55816f327c002be813a29d41f9a7ae429782b6856a4d3bb2e6c498c6f9e3c0";
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

fn prepare(wallet: &mut TestAccount) -> String {
    wallet
        .prepare_fixture_payment(REQUEST_ID, INTENT_HASH, &mut ManualClock::at(NOW))
        .unwrap()
        .handle
}

fn sha256_hex(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let digest = Sha256::digest(bytes);
    let mut result = String::with_capacity(digest.len() * 2);
    for byte in digest {
        result.push(char::from(DIGITS[usize::from(byte >> 4)]));
        result.push(char::from(DIGITS[usize::from(byte & 0x0f)]));
    }
    result
}

#[test]
fn handle_lookup_is_bound_to_account_session_request_and_intent_with_constant_shape_miss() {
    let mut wallet = wallet("hygiene-binding");
    let handle = prepare(&mut wallet);
    let valid = HandleBinding::new(ACCOUNT, wallet.session_id(), REQUEST_ID, INTENT_HASH).unwrap();
    assert!(wallet.lookup_prepared(&handle, &valid).is_ok());
    for binding in [
        HandleBinding::new(OTHER_ACCOUNT, wallet.session_id(), REQUEST_ID, INTENT_HASH).unwrap(),
        HandleBinding::new(
            ACCOUNT,
            "ffffffffffffffffffffffffffffffff",
            REQUEST_ID,
            INTENT_HASH,
        )
        .unwrap(),
        HandleBinding::new(ACCOUNT, wallet.session_id(), OTHER_REQUEST, INTENT_HASH).unwrap(),
        HandleBinding::new(ACCOUNT, wallet.session_id(), REQUEST_ID, OTHER_INTENT).unwrap(),
    ] {
        wallet.reset_lookup_observer();
        assert_eq!(
            wallet
                .lookup_prepared(&handle, &binding)
                .unwrap_err()
                .code(),
            "LOCKED"
        );
        assert_eq!(
            wallet.lookup_observation().shape,
            wallet.constant_miss_shape()
        );
        assert_eq!(wallet.lookup_observation().returned_bytes, 0);
    }
}

#[test]
fn every_named_lifecycle_edge_invalidates_handle_and_wipes_prepared_state() {
    for edge in [
        HandleInvalidation::Lock,
        HandleInvalidation::Timeout,
        HandleInvalidation::Cancel,
        HandleInvalidation::Expiry,
        HandleInvalidation::AccountReplacement,
        HandleInvalidation::DatabaseRollback,
        HandleInvalidation::OperationError,
        HandleInvalidation::PanicUnwind,
        HandleInvalidation::BrokerExit,
    ] {
        let wipes = RecordingWipes::shared();
        let mut wallet = wallet("hygiene-invalidation");
        wallet.attach_wipe_observer(wipes.clone());
        let handle = prepare(&mut wallet);
        let raw_len = wallet.prepared_raw_length_for_test(&handle).unwrap();
        wallet.invalidate_for_test(edge).unwrap();
        assert!(!wallet.contains_prepared_handle(&handle), "edge {edge:?}");
        assert!(wipes.contains_post_wipe("zec-prepared-pczt", raw_len, edge.as_str()));
        assert_eq!(wallet.prepared_handle_count(), 0);
    }
}

#[test]
fn panic_unwind_wipes_seed_derived_material_and_prepared_artifact_before_return() {
    let wipes = RecordingWipes::shared();
    let mut wallet = wallet("hygiene-unwind");
    wallet.attach_wipe_observer(wipes.clone());
    let handle = prepare(&mut wallet);
    let prepared_len = wallet.prepared_raw_length_for_test(&handle).unwrap();
    let spend_len = wallet.spend_material_length_for_test();
    let unwind = catch_unwind(AssertUnwindSafe(|| wallet.panic_inside_prepare_for_test()));
    assert!(unwind.is_err());
    assert!(!wallet.contains_prepared_handle(&handle));
    assert!(wipes.contains_post_wipe("zec-prepared-pczt", prepared_len, "panic-unwind"));
    assert!(wipes.contains_post_wipe("zec-derived-spend", spend_len, "panic-unwind"));
}

#[test]
fn debug_display_diagnostics_and_logs_omit_every_secret_class() {
    let canaries = [
        ObservableCanary::new(ObservableSecretClass::Seed, "CANARY_WAL006_SEED_40a1"),
        ObservableCanary::new(
            ObservableSecretClass::SpendingKey,
            "CANARY_WAL006_SPEND_KEY_40a2",
        ),
        ObservableCanary::new(
            ObservableSecretClass::VaultPlaintext,
            "CANARY_WAL006_VAULT_PLAINTEXT_40a3",
        ),
        ObservableCanary::new(ObservableSecretClass::Ufvk, "CANARY_WAL006_UFVK_40a4"),
        ObservableCanary::new(
            ObservableSecretClass::ReceiverInternals,
            "CANARY_WAL006_RECEIVER_INTERNALS_40a5",
        ),
        ObservableCanary::new(ObservableSecretClass::Memo, "CANARY_WAL006_MEMO_40a6"),
        ObservableCanary::new(
            ObservableSecretClass::NotePlaintext,
            "CANARY_WAL006_NOTE_PLAINTEXT_40a7",
        ),
        ObservableCanary::new(
            ObservableSecretClass::Nullifier,
            "CANARY_WAL006_NULLIFIER_40a8",
        ),
        ObservableCanary::new(
            ObservableSecretClass::CompactBlock,
            "CANARY_WAL006_COMPACT_BLOCK_40a9",
        ),
        ObservableCanary::new(
            ObservableSecretClass::SqliteRow,
            "CANARY_WAL006_SQLITE_ROW_40aa",
        ),
        ObservableCanary::new(ObservableSecretClass::RawPczt, "CANARY_WAL006_PCZT_40ab"),
        ObservableCanary::new(
            ObservableSecretClass::Transaction,
            "CANARY_WAL006_TRANSACTION_40ac",
        ),
        ObservableCanary::new(
            ObservableSecretClass::UserPath,
            "CANARY_WAL006_USER_PATH_40ad",
        ),
    ];
    let mut wallet = wallet("hygiene-redaction");
    let receipt = wallet
        .install_observable_canaries_for_test(&canaries)
        .unwrap();
    assert!(canaries.iter().all(|canary| !canary.value().is_empty()));
    assert!(receipt.is_closed());
    assert_eq!(
        receipt.class_names(),
        [
            "seed",
            "spending-key",
            "vault-plaintext",
            "ufvk",
            "receiver-internals",
            "memo",
            "note-plaintext",
            "nullifier",
            "compact-block",
            "sqlite-row",
            "raw-pczt",
            "transaction",
            "user-path",
        ]
    );
    assert_eq!(receipt.commitments().len(), canaries.len());
    let mut commitment_hashes = BTreeSet::new();
    for (commitment, canary) in receipt.commitments().iter().zip(&canaries) {
        assert_eq!(commitment.class, canary.class().as_str());
        assert_eq!(commitment.byte_length, canary.value().len());
        assert_eq!(commitment.sha256, sha256_hex(canary.value().as_bytes()));
        assert!(commitment_hashes.insert(commitment.sha256.as_str()));
    }
    let prepared = wallet
        .prepare_fixture_payment(REQUEST_ID, INTENT_HASH, &mut ManualClock::at(NOW))
        .unwrap();
    let observable = format!(
        "wallet={wallet:?};prepared={prepared:?};error={:?};logs={:?};diagnostics={:?}",
        wallet.synthetic_failure_for_test(),
        wallet.captured_logs(),
        wallet.diagnostics(),
    );
    for canary in &canaries {
        assert!(!observable.contains(canary.value()));
    }
    assert_eq!(
        wallet.diagnostic_field_names(),
        ["operation", "account_id", "network", "code"]
    );
}

#[test]
fn public_capability_surface_has_no_raw_sign_prove_finalize_extract_broadcast_or_network_authority()
{
    let wallet = wallet("hygiene-capabilities");
    assert_eq!(
        wallet.public_zec_operations(),
        [
            "account.bootstrap",
            "receiver.fresh",
            "fixture.scan",
            "pczt.prepare"
        ]
    );
    for forbidden in [
        "raw",
        "pczt.raw",
        "transaction.raw",
        "sign",
        "prove",
        "finalize",
        "extract",
        "serialize",
        "txid",
        "broadcast",
        "submit",
        "sync",
        "connect",
        "endpoint",
        "http",
        "https",
        "dns",
        "tor",
        "proxy",
        "socket",
        "lightwalletd",
        "mainnet",
    ] {
        assert!(
            !wallet
                .public_zec_operations()
                .iter()
                .any(|operation| operation.contains(forbidden))
        );
        assert_eq!(
            wallet
                .invoke_operation_for_test(forbidden)
                .unwrap_err()
                .code(),
            "CAPABILITY_MISSING"
        );
    }
    assert!(!wallet.capabilities().can_sign);
    assert!(!wallet.capabilities().can_prove);
    assert!(!wallet.capabilities().can_extract);
    assert!(!wallet.capabilities().can_broadcast);
    assert!(!wallet.capabilities().can_network);
    assert!(!wallet.capabilities().can_mainnet);
}

#[test]
fn sanitized_prepared_value_has_exact_closed_fields_and_no_raw_artifact() {
    let mut wallet = wallet("hygiene-prepared-fields");
    let prepared = wallet
        .prepare_fixture_payment(REQUEST_ID, INTENT_HASH, &mut ManualClock::at(NOW))
        .unwrap();
    assert_eq!(
        prepared.field_names(),
        [
            "handle",
            "account_id",
            "network",
            "request_id",
            "intent_hash",
            "receiver",
            "amount_zat",
            "fee_zat",
            "fee_bound_zat",
            "expires_at",
            "tx_version",
            "consensus_branch",
            "spend_pool",
            "output_pool",
            "signed",
            "extractable",
            "broadcastable",
        ]
    );
    let encoded = prepared.sanitized_json_for_test();
    for forbidden in [
        "pczt",
        "transaction",
        "proof",
        "key",
        "txid",
        "endpoint",
        "path",
        "diagnostic",
        "rate",
        "fiat",
        "signature",
        "nullifier",
    ] {
        assert!(!encoded.to_ascii_lowercase().contains(forbidden));
    }
    assert!(encoded.len() <= MAX_DIAGNOSTIC_BYTES);
}

#[test]
fn diagnostics_limit_covers_immediate_below_at_and_above_without_secret_echo() {
    let wallet = wallet("hygiene-diagnostic-limit");
    for (length, accepted) in [
        (MAX_DIAGNOSTIC_BYTES - 1, true),
        (MAX_DIAGNOSTIC_BYTES, true),
        (MAX_DIAGNOSTIC_BYTES + 1, false),
    ] {
        let canary = "X".repeat(length);
        let result = wallet.normalize_diagnostic_for_test(&canary);
        assert_eq!(result.is_ok(), accepted);
        if accepted {
            assert!(!result.unwrap().contains(&canary));
        } else {
            assert_eq!(result.unwrap_err().code(), "LIMIT");
        }
    }
}

#[test]
fn raw_prepared_state_is_memory_only_and_absent_after_close() {
    let mut wallet = wallet("hygiene-memory-only");
    let paths = wallet.inspect_paths();
    let before_wallet = std::fs::read(paths.absolute_wallet_db()).unwrap();
    let before_cache = std::fs::read(paths.absolute_compact_cache()).unwrap();
    let handle = prepare(&mut wallet);
    assert!(wallet.prepared_raw_length_for_test(&handle).unwrap() > 0);
    assert_eq!(
        std::fs::read(paths.absolute_wallet_db()).unwrap(),
        before_wallet
    );
    assert_eq!(
        std::fs::read(paths.absolute_compact_cache()).unwrap(),
        before_cache
    );
    wallet.close().unwrap();
    assert_eq!(
        std::fs::read(paths.absolute_wallet_db()).unwrap(),
        before_wallet
    );
    assert_eq!(
        std::fs::read(paths.absolute_compact_cache()).unwrap(),
        before_cache
    );
}
