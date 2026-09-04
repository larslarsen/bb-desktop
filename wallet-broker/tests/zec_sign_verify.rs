use std::panic::{AssertUnwindSafe, catch_unwind};

use bitbook_wallet_broker::native::ActionOrigin;
use bitbook_wallet_broker::zec::test_support::{
    BarrierMutation, ConfirmationMutation, ExternalContributionMutation, FaultPoint, FrozenFixture,
    ManualClock, SignRoute, SignVerifyCanaries, SignVerifyHarness, SignVerifyMutation,
    SignVerifyObservations, SignVerifyPrerequisite, SyntheticKeystoneV2, TerminalExit,
    TestStateRoot, TouchedSecretClass, WipeExit,
};
use bitbook_wallet_broker::zec::{AccountId, PrepareZecV1};

const ACCOUNT: &str = "00112233445566778899aabbccddeeff";
const OTHER_ACCOUNT: &str = "ffeeddccbbaa99887766554433221100";
const REQUEST_ID: &str = "00112233445566778899aabbccddeeff";
const OTHER_REQUEST_ID: &str = "11112222333344445555666677778888";
const INTENT_HASH: &str = "ad55816f327c002be813a29d41f9a7ae429782b6856a4d3bb2e6c498c6f9e3c0";
const OTHER_INTENT_HASH: &str = "bd55816f327c002be813a29d41f9a7ae429782b6856a4d3bb2e6c498c6f9e3c0";
const MEMO_HASH: &str = "37290d74ac4d186e3a8e5785d259d2ec04fac91ae28092e7620ec8bc99e830aa";
const AMOUNT_ZAT: &str = "100000000";
const FEE_ZAT: &str = "10000";
const FEE_BOUND_ZAT: &str = "12000";
const EXPIRES_AT: &str = "2026-08-30T12:15:00Z";
const NOW: &str = "2026-08-30T12:00:30Z";
const FIXTURE_DIR: &str = "tests/fixtures/zec";

fn fixture() -> FrozenFixture {
    FrozenFixture::open(FIXTURE_DIR).unwrap()
}

fn input_for(account: &str, request_id: &str, intent_hash: &str) -> PrepareZecV1 {
    PrepareZecV1::new(
        account,
        "zec-local",
        request_id,
        intent_hash,
        fixture().expected_destination_receiver(),
        AMOUNT_ZAT,
        FEE_BOUND_ZAT,
        "coffee",
        EXPIRES_AT,
    )
    .unwrap()
}

fn input() -> PrepareZecV1 {
    input_for(ACCOUNT, REQUEST_ID, INTENT_HASH)
}

fn software_harness(label: &str) -> SignVerifyHarness {
    let mut harness = SignVerifyHarness::software_from_fixture(
        TestStateRoot::fresh(label),
        AccountId::parse(ACCOUNT).unwrap(),
        &fixture(),
    )
    .unwrap();
    harness.scan(&fixture()).unwrap();
    harness.unlock_with_fixture_seed().unwrap();
    harness
}

fn prepared(harness: &mut SignVerifyHarness) -> String {
    harness
        .prepare(input(), &mut ManualClock::at(NOW))
        .unwrap()
        .handle
}

fn assert_lower_hex(value: &str, length: usize) {
    assert_eq!(value.len(), length);
    assert!(
        value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    );
}

fn assert_no_secret_or_transaction_access(harness: &SignVerifyHarness) {
    let calls = harness.observed_calls();
    assert_eq!(calls.seed_accesses, 0);
    assert_eq!(calls.spend_authority_derivations, 0);
    assert_eq!(calls.authoritative_pczt_accesses, 0);
    assert_eq!(calls.signer_calls, 0);
    assert_eq!(calls.prover_calls, 0);
    assert_eq!(calls.finalizer_calls, 0);
    assert_eq!(calls.extractor_calls, 0);
    assert_eq!(calls.verifier_calls, 0);
    assert_eq!(calls.verified_publications, 0);
    assert_eq!(calls.broadcast_calls, 0);
}

fn assert_no_verified_or_broadcast(harness: &SignVerifyHarness) {
    let calls = harness.observed_calls();
    assert_eq!(calls.verified_publications, 0);
    assert_eq!(calls.broadcast_calls, 0);
    assert_eq!(harness.verified_handle_count(), 0);
}

fn software_classes() -> [TouchedSecretClass; 7] {
    [
        TouchedSecretClass::Seed,
        TouchedSecretClass::UnifiedSpendingAuthority,
        TouchedSecretClass::DerivedAuthorizingKey,
        TouchedSecretClass::ConfirmationCapability,
        TouchedSecretClass::AuthoritativePczt,
        TouchedSecretClass::ProofWorkspace,
        TouchedSecretClass::ExtractedTransaction,
    ]
}

fn assert_wiped(
    observations: &SignVerifyObservations,
    exit: WipeExit,
    classes: &[TouchedSecretClass],
) {
    for class in classes {
        assert!(
            observations.touch_count(*class, exit) > 0,
            "{class:?} was not touched for {exit:?}"
        );
        assert!(
            observations.positive_wipe_count(*class, exit) > 0,
            "{class:?} was not positively wiped for {exit:?}"
        );
        assert_eq!(
            observations.failed_wipe_count(*class, exit),
            0,
            "{class:?} had an incomplete wipe for {exit:?}"
        );
    }
}

fn assert_every_touched_secret_wiped(observations: &SignVerifyObservations, exit: WipeExit) {
    let touched = observations.touched_classes(exit);
    assert!(
        !touched.is_empty(),
        "no sensitive class was observed for {exit:?}"
    );
    for class in touched {
        assert!(
            observations.positive_wipe_count(class, exit) > 0,
            "{class:?} was touched but not positively wiped for {exit:?}"
        );
        assert_eq!(
            observations.failed_wipe_count(class, exit),
            0,
            "{class:?} had an incomplete wipe for {exit:?}"
        );
    }
}

#[test]
fn software_signs_proves_finalizes_extracts_and_independently_decodes_exact_v6_effects() {
    let mut harness = software_harness("sign-verify-software-happy");
    let handle = prepared(&mut harness);
    let confirmation = harness
        .confirm_native(&handle, &mut ManualClock::at(NOW))
        .unwrap();
    let verified = harness
        .sign_and_verify(
            &handle,
            confirmation,
            SignRoute::Software,
            &mut ManualClock::at(NOW),
        )
        .unwrap();
    let effects = harness
        .independent_effects_observation(&verified.handle)
        .unwrap();

    assert_eq!(effects.network, "zec-local");
    assert!(!effects.mainnet);
    assert_eq!(effects.transaction_version, 6);
    assert_eq!(effects.consensus_branch, 0x37a5_165b);
    assert_eq!(
        effects.external_receiver_bytes,
        fixture().expected_destination_receiver_bytes().unwrap()
    );
    assert_eq!(
        effects.external_receiver,
        fixture().expected_destination_receiver()
    );
    assert_eq!(effects.external_amount_zat, AMOUNT_ZAT);
    assert_eq!(effects.fee_zat, FEE_ZAT);
    assert_eq!(effects.fee_bound_zat, FEE_BOUND_ZAT);
    assert!(effects.fee_zat_u64 <= effects.fee_bound_zat_u64);
    assert_eq!(effects.memo_sha256, MEMO_HASH);
    assert_eq!(effects.request_id_binding, REQUEST_ID);
    assert_eq!(effects.intent_hash_binding, INTENT_HASH);
    assert_eq!(effects.ironwood_real_spends, 1);
    assert_eq!(effects.ironwood_external_outputs, 1);
    assert_eq!(effects.ironwood_internal_change_outputs, 1);
    assert_eq!(effects.transparent_effects, 0);
    assert_eq!(effects.sapling_effects, 0);
    assert_eq!(effects.orchard_effects, 0);
    assert!(effects.proof_present && effects.proof_valid);
    assert!(effects.spend_authorization_present && effects.spend_authorization_valid);
    assert!(effects.binding_signature_present && effects.binding_signature_valid);
    assert_eq!(effects.derived_transaction_id, verified.transaction_id);

    let calls = harness.observed_calls();
    assert_eq!(calls.signer_calls, 1);
    assert_eq!(calls.prover_calls, 1);
    assert_eq!(calls.finalizer_calls, 1);
    assert_eq!(calls.extractor_calls, 1);
    assert_eq!(calls.independent_decoder_calls, 1);
    assert_eq!(calls.verifier_calls, 1);
    assert_eq!(calls.verified_publications, 1);
    assert_eq!(calls.broadcast_calls, 0);
}

#[test]
fn public_verified_result_is_bounded_redacted_derived_and_non_broadcastable() {
    let mut harness = software_harness("sign-verify-public-result");
    let handle = prepared(&mut harness);
    let untrusted_signer_transaction_id = "f".repeat(64);
    harness
        .set_untrusted_signer_transaction_id_for_test(&untrusted_signer_transaction_id)
        .unwrap();
    let confirmation = harness
        .confirm_native(&handle, &mut ManualClock::at(NOW))
        .unwrap();
    let verified = harness
        .sign_and_verify(
            &handle,
            confirmation,
            SignRoute::Software,
            &mut ManualClock::at(NOW),
        )
        .unwrap();

    assert_eq!(
        verified.field_names(),
        [
            "handle",
            "transaction_id",
            "state",
            "account_id",
            "request_id",
            "broadcastable",
        ]
    );
    assert_lower_hex(&verified.handle, 32);
    assert_lower_hex(&verified.transaction_id, 64);
    assert_eq!(verified.state, "verified");
    assert_eq!(verified.account_id, ACCOUNT);
    assert_eq!(verified.request_id, REQUEST_ID);
    assert!(!verified.broadcastable);
    assert_ne!(verified.transaction_id, untrusted_signer_transaction_id);
    assert_eq!(
        verified.transaction_id,
        harness
            .independently_derived_transaction_id(&verified.handle)
            .unwrap()
    );

    let public_json = verified.sanitized_json_for_test();
    let public_value: serde_json::Value = serde_json::from_str(&public_json).unwrap();
    assert_eq!(public_value.as_object().unwrap().len(), 6);
    for forbidden in [
        "raw_pczt",
        "raw_transaction",
        "seed",
        "spending_key",
        "memo",
        "receiver",
        "signature",
        "proof",
    ] {
        assert!(!public_json.contains(forbidden));
    }
    assert_eq!(harness.observed_calls().broadcast_calls, 0);
}

#[test]
fn native_confirmation_is_one_shot_and_every_binding_mismatch_precedes_secret_access() {
    for origin in [
        ActionOrigin::Electron,
        ActionOrigin::BrokerProtocol,
        ActionOrigin::Http,
    ] {
        let mut harness = software_harness("sign-verify-confirm-origin");
        let handle = prepared(&mut harness);
        harness.reset_sign_verify_observations();
        let error = harness
            .confirm_from_for_test(origin, &handle, &mut ManualClock::at(NOW))
            .unwrap_err();
        assert_eq!(error.code(), "UNAUTH");
        assert_no_secret_or_transaction_access(&harness);
    }

    for method in [
        "intent.confirm",
        "signer.sign",
        "wallet.invoke",
        "http://synthetic.invalid/confirm",
    ] {
        let mut harness = software_harness("sign-verify-confirm-string");
        let handle = prepared(&mut harness);
        harness.reset_sign_verify_observations();
        let error = harness
            .confirm_reconstructed_for_test(method, &handle)
            .unwrap_err();
        assert_eq!(error.code(), "UNAUTH");
        assert_no_secret_or_transaction_access(&harness);
    }

    for mutation in [
        ConfirmationMutation::Handle,
        ConfirmationMutation::Session,
        ConfirmationMutation::Account,
        ConfirmationMutation::Network,
        ConfirmationMutation::RequestId,
        ConfirmationMutation::IntentHash,
        ConfirmationMutation::ReviewHash,
        ConfirmationMutation::Receiver,
        ConfirmationMutation::Amount,
        ConfirmationMutation::Fee,
        ConfirmationMutation::FeeBound,
        ConfirmationMutation::MemoHash,
        ConfirmationMutation::Expiry,
    ] {
        let mut harness = software_harness("sign-verify-confirm-binding");
        let handle = prepared(&mut harness);
        harness.reset_sign_verify_observations();
        let error = harness
            .confirm_mutated_for_test(
                ActionOrigin::NativeSurface,
                &handle,
                mutation,
                &mut ManualClock::at(NOW),
            )
            .unwrap_err();
        assert_eq!(error.code(), "INTENT_MISMATCH", "mutation {mutation:?}");
        assert_no_secret_or_transaction_access(&harness);
    }

    let mut harness = software_harness("sign-verify-confirm-replay");
    let handle = prepared(&mut harness);
    let confirmation = harness
        .confirm_native(&handle, &mut ManualClock::at(NOW))
        .unwrap();
    let consumed = harness
        .sign_and_verify_with_receipt(
            &handle,
            confirmation,
            SignRoute::Software,
            &mut ManualClock::at(NOW),
        )
        .unwrap();
    assert_eq!(harness.observed_calls().verified_publications, 1);
    let replay = harness
        .replay_consumed_confirmation_for_test(consumed.confirmation_receipt)
        .unwrap_err();
    assert_eq!(replay.code(), "UNAUTH");
    assert_eq!(harness.observed_calls().signer_calls, 1);
    assert_eq!(harness.observed_calls().verified_publications, 1);
    assert_eq!(harness.observed_calls().broadcast_calls, 0);
}

#[test]
fn custody_network_and_session_failures_stop_before_pczt_signing_or_proving() {
    let rows = [
        (SignVerifyPrerequisite::WrongSeed, "LOCKED"),
        (SignVerifyPrerequisite::WrongFullViewingKey, "LOCKED"),
        (SignVerifyPrerequisite::WrongAccount, "LOCKED"),
        (SignVerifyPrerequisite::Locked, "LOCKED"),
        (SignVerifyPrerequisite::WatchOnly, "WATCH_ONLY"),
        (SignVerifyPrerequisite::WrongNetwork, "SCHEMA"),
        (SignVerifyPrerequisite::Mainnet, "NETWORK_DISABLED"),
        (SignVerifyPrerequisite::StaleSession, "LOCKED"),
    ];
    for (prerequisite, code) in rows {
        let mut harness = software_harness("sign-verify-prerequisite");
        let handle = prepared(&mut harness);
        let confirmation = harness
            .confirm_native(&handle, &mut ManualClock::at(NOW))
            .unwrap();
        harness.reset_sign_verify_observations();
        let error = harness
            .sign_with_prerequisite_for_test(
                &handle,
                confirmation,
                prerequisite,
                &mut ManualClock::at(NOW),
            )
            .unwrap_err();
        assert_eq!(error.code(), code, "prerequisite {prerequisite:?}");
        let calls = harness.observed_calls();
        assert_eq!(calls.authoritative_pczt_accesses, 0);
        assert_eq!(calls.signer_calls, 0);
        assert_eq!(calls.prover_calls, 0);
        assert_eq!(calls.finalizer_calls, 0);
        assert_eq!(calls.extractor_calls, 0);
        assert_eq!(calls.verifier_calls, 0);
        assert_no_verified_or_broadcast(&harness);
    }
}

#[test]
fn production_hardware_denies_without_a_positive_route_or_pczt_export() {
    let mut harness = SignVerifyHarness::production_hardware_from_fixture(
        TestStateRoot::fresh("sign-verify-production-hardware"),
        AccountId::parse(ACCOUNT).unwrap(),
        &fixture(),
    )
    .unwrap();
    harness.scan(&fixture()).unwrap();
    let handle = harness
        .prepare(input(), &mut ManualClock::at(NOW))
        .unwrap()
        .handle;
    let confirmation = harness
        .confirm_native(&handle, &mut ManualClock::at(NOW))
        .unwrap();
    harness.reset_sign_verify_observations();
    let error = harness
        .sign_and_verify(
            &handle,
            confirmation,
            SignRoute::ProductionHardware,
            &mut ManualClock::at(NOW),
        )
        .unwrap_err();

    assert_eq!(error.code(), "CAPABILITY_MISSING");
    assert_eq!(harness.production_positive_hardware_routes(), 0);
    let calls = harness.observed_calls();
    assert_eq!(calls.hardware_view_exports, 0);
    assert_eq!(calls.exported_pczt_bytes, 0);
    assert_eq!(calls.external_contributions_received, 0);
    assert_eq!(calls.signer_calls, 0);
    assert_eq!(calls.prover_calls, 0);
    assert_no_verified_or_broadcast(&harness);
}

#[test]
fn synthetic_keystone_v2_returns_tagged_ironwood_contributions_for_retained_pczt() {
    let mut harness = SignVerifyHarness::synthetic_keystone_from_fixture(
        TestStateRoot::fresh("sign-verify-keystone-v2"),
        AccountId::parse(ACCOUNT).unwrap(),
        &fixture(),
    )
    .unwrap();
    harness.scan(&fixture()).unwrap();
    let handle = harness
        .prepare(input(), &mut ManualClock::at(NOW))
        .unwrap()
        .handle;
    let retained_commitment = harness.authoritative_effects_commitment(&handle).unwrap();
    let confirmation = harness
        .confirm_native(&handle, &mut ManualClock::at(NOW))
        .unwrap();
    let view = harness
        .reviewed_signer_view_for_test(&handle, &confirmation)
        .unwrap();

    assert_eq!(view.route, "keystone_pczt_v2");
    assert_eq!(view.pczt_encoding_version, 2);
    assert_eq!(view.batch_len, 1);
    assert_eq!(
        view.field_names(),
        [
            "route",
            "pczt_encoding_version",
            "batch_id",
            "intent_hash",
            "review_hash",
            "network",
            "transaction_version",
            "consensus_branch",
            "signing_pool",
            "actions",
        ]
    );
    assert_eq!(view.raw_pczt_bytes(), 0);
    assert_eq!(view.transaction_bytes(), 0);
    assert_eq!(view.actions.len(), 1);
    assert_eq!(view.actions[0].pool, "ironwood");
    assert_eq!(view.actions[0].action_index, 0);
    assert_lower_hex(&view.actions[0].randomized_key, 64);

    let contributions = SyntheticKeystoneV2::sign_for_test(&view).unwrap();
    assert_eq!(contributions.route(), "keystone_pczt_v2");
    assert_eq!(contributions.len(), 1);
    assert_eq!(contributions[0].pool, "ironwood");
    assert_eq!(contributions[0].action_index, 0);
    assert_eq!(
        contributions[0].randomized_key,
        view.actions[0].randomized_key
    );
    assert_eq!(contributions[0].signature_bytes().len(), 64);
    assert!(contributions[0].is_unmistakably_test_only());

    let verified = harness
        .apply_external_contributions_and_verify(
            &handle,
            confirmation,
            contributions,
            &mut ManualClock::at(NOW),
        )
        .unwrap();
    assert_eq!(
        harness
            .verified_effects_commitment(&verified.handle)
            .unwrap(),
        retained_commitment
    );
    let calls = harness.observed_calls();
    assert_eq!(calls.hardware_view_exports, 1);
    assert_eq!(calls.exported_pczt_bytes, 0);
    assert_eq!(calls.external_contributions_received, 1);
    assert_eq!(calls.external_contributions_applied, 1);
    assert_eq!(calls.external_signatures_verified, 1);
    assert_eq!(calls.software_fallbacks, 0);
    assert_eq!(calls.other_device_fallbacks, 0);
    assert_eq!(calls.verified_publications, 1);
    assert_eq!(calls.broadcast_calls, 0);
}

#[test]
fn malformed_replayed_or_misbound_external_contributions_fail_closed() {
    for mutation in [
        ExternalContributionMutation::Missing,
        ExternalContributionMutation::Duplicate,
        ExternalContributionMutation::Extra,
        ExternalContributionMutation::InvalidSignature,
        ExternalContributionMutation::WrongPool,
        ExternalContributionMutation::WrongActionIndex,
        ExternalContributionMutation::WrongRandomizedKey,
        ExternalContributionMutation::Replayed,
    ] {
        let mut harness = SignVerifyHarness::synthetic_keystone_from_fixture(
            TestStateRoot::fresh("sign-verify-keystone-invalid"),
            AccountId::parse(ACCOUNT).unwrap(),
            &fixture(),
        )
        .unwrap();
        harness.scan(&fixture()).unwrap();
        let handle = harness
            .prepare(input(), &mut ManualClock::at(NOW))
            .unwrap()
            .handle;
        let confirmation = harness
            .confirm_native(&handle, &mut ManualClock::at(NOW))
            .unwrap();
        let view = harness
            .reviewed_signer_view_for_test(&handle, &confirmation)
            .unwrap();
        let contributions = SyntheticKeystoneV2::sign_for_test(&view)
            .unwrap()
            .with_mutation_for_test(mutation, OTHER_INTENT_HASH);
        let error = harness
            .apply_external_contributions_and_verify(
                &handle,
                confirmation,
                contributions,
                &mut ManualClock::at(NOW),
            )
            .unwrap_err();
        assert_eq!(
            error.code(),
            "SIGNATURE_INVALID",
            "contribution mutation {mutation:?}"
        );
        assert_eq!(harness.observed_calls().exported_pczt_bytes, 0);
        assert_no_verified_or_broadcast(&harness);
    }

    for mutation in [
        ExternalContributionMutation::Reordered,
        ExternalContributionMutation::CrossIntent,
    ] {
        let mut harness = SignVerifyHarness::synthetic_keystone_from_fixture(
            TestStateRoot::fresh("sign-verify-keystone-invalid-batch"),
            AccountId::parse(ACCOUNT).unwrap(),
            &fixture(),
        )
        .unwrap();
        harness
            .add_synthetic_keystone_fixture_account(
                AccountId::parse(OTHER_ACCOUNT).unwrap(),
                &fixture(),
            )
            .unwrap();
        harness.scan(&fixture()).unwrap();
        harness.scan_account(OTHER_ACCOUNT, &fixture()).unwrap();
        let first_handle = harness
            .prepare(input(), &mut ManualClock::at(NOW))
            .unwrap()
            .handle;
        let second_handle = harness
            .prepare(
                input_for(OTHER_ACCOUNT, OTHER_REQUEST_ID, OTHER_INTENT_HASH),
                &mut ManualClock::at(NOW),
            )
            .unwrap()
            .handle;
        let first_confirmation = harness
            .confirm_native(&first_handle, &mut ManualClock::at(NOW))
            .unwrap();
        let second_confirmation = harness
            .confirm_native(&second_handle, &mut ManualClock::at(NOW))
            .unwrap();
        let batch = harness
            .reviewed_signer_batch_for_test([
                (&first_handle, &first_confirmation),
                (&second_handle, &second_confirmation),
            ])
            .unwrap();
        assert_eq!(batch.batch_len, 2);
        assert_eq!(batch.actions.len(), 2);
        assert_ne!(batch.actions[0].intent_hash, batch.actions[1].intent_hash);
        let contributions = SyntheticKeystoneV2::sign_batch_for_test(&batch)
            .unwrap()
            .with_mutation_for_test(mutation, OTHER_INTENT_HASH);
        let error = harness
            .apply_external_batch_and_verify(
                [
                    (first_handle, first_confirmation),
                    (second_handle, second_confirmation),
                ],
                contributions,
                &mut ManualClock::at(NOW),
            )
            .unwrap_err();
        assert_eq!(error.code(), "SIGNATURE_INVALID", "mutation {mutation:?}");
        assert_eq!(harness.observed_calls().exported_pczt_bytes, 0);
        assert_no_verified_or_broadcast(&harness);
    }
}

#[test]
fn every_post_sign_effect_and_authorization_mutation_fails_independent_verification() {
    let intent_mismatches = [
        SignVerifyMutation::Receiver,
        SignVerifyMutation::Amount,
        SignVerifyMutation::Network,
        SignVerifyMutation::Fee,
        SignVerifyMutation::FeeBound,
        SignVerifyMutation::Memo,
        SignVerifyMutation::RequestId,
        SignVerifyMutation::IntentHash,
        SignVerifyMutation::Pool,
        SignVerifyMutation::Version,
        SignVerifyMutation::ConsensusBranch,
        SignVerifyMutation::Change,
        SignVerifyMutation::ExtractedTransactionIdBinding,
    ];
    for mutation in intent_mismatches {
        let mut harness = software_harness("sign-verify-effect-mutation");
        let handle = prepared(&mut harness);
        let confirmation = harness
            .confirm_native(&handle, &mut ManualClock::at(NOW))
            .unwrap();
        harness.reset_sign_verify_observations();
        let error = harness
            .sign_mutate_and_verify_for_test(
                &handle,
                confirmation,
                mutation,
                &mut ManualClock::at(NOW),
            )
            .unwrap_err();
        assert_eq!(error.code(), "INTENT_MISMATCH", "mutation {mutation:?}");
        let calls = harness.observed_calls();
        assert_eq!(calls.signer_calls, 1);
        assert_eq!(calls.prover_calls, 1);
        assert_eq!(calls.finalizer_calls, 1);
        assert_eq!(calls.extractor_calls, 1);
        assert_eq!(calls.verifier_calls, 1);
        assert_no_verified_or_broadcast(&harness);
    }

    for mutation in [SignVerifyMutation::Proof, SignVerifyMutation::Signature] {
        let mut harness = software_harness("sign-verify-authorization-mutation");
        let handle = prepared(&mut harness);
        let confirmation = harness
            .confirm_native(&handle, &mut ManualClock::at(NOW))
            .unwrap();
        let error = harness
            .sign_mutate_and_verify_for_test(
                &handle,
                confirmation,
                mutation,
                &mut ManualClock::at(NOW),
            )
            .unwrap_err();
        assert_eq!(error.code(), "SIGNATURE_INVALID");
        assert_no_verified_or_broadcast(&harness);
    }

    for (mutation, code) in [
        (SignVerifyMutation::MalformedState, "STATE_CORRUPT"),
        (SignVerifyMutation::MalformedSchema, "SCHEMA"),
    ] {
        let mut harness = software_harness("sign-verify-malformed-state");
        let handle = prepared(&mut harness);
        let confirmation = harness
            .confirm_native(&handle, &mut ManualClock::at(NOW))
            .unwrap();
        let error = harness
            .sign_mutate_and_verify_for_test(
                &handle,
                confirmation,
                mutation,
                &mut ManualClock::at(NOW),
            )
            .unwrap_err();
        assert_eq!(error.code(), code);
        assert_no_verified_or_broadcast(&harness);
    }
}

#[test]
fn cancellation_and_expiry_are_reread_after_proving_at_exact_boundaries() {
    let mut cancelled = software_harness("sign-verify-cancel-after-proof");
    let handle = prepared(&mut cancelled);
    let confirmation = cancelled
        .confirm_native(&handle, &mut ManualClock::at(NOW))
        .unwrap();
    cancelled.reset_sign_verify_observations();
    let error = cancelled
        .sign_with_barrier_for_test(
            &handle,
            confirmation,
            SignRoute::Software,
            BarrierMutation::CancelAfterSignAndProof,
            &mut ManualClock::at(NOW),
        )
        .unwrap_err();
    assert_eq!(error.code(), "CANCELLED");
    assert_eq!(cancelled.observed_calls().signer_calls, 1);
    assert_eq!(cancelled.observed_calls().prover_calls, 1);
    assert_eq!(cancelled.observed_calls().post_sign_status_reads, 1);
    assert_no_verified_or_broadcast(&cancelled);

    for (now, expected) in [
        ("2026-08-30T12:14:59Z", None),
        (EXPIRES_AT, Some("EXPIRED")),
        ("2026-08-30T12:15:01Z", Some("EXPIRED")),
    ] {
        let mut harness = software_harness("sign-verify-expiry-after-proof");
        let handle = prepared(&mut harness);
        let confirmation = harness
            .confirm_native(&handle, &mut ManualClock::at(NOW))
            .unwrap();
        harness.reset_sign_verify_observations();
        let result = harness.sign_with_barrier_for_test(
            &handle,
            confirmation,
            SignRoute::Software,
            BarrierMutation::ClockAfterSignAndProof(now.to_owned()),
            &mut ManualClock::at(NOW),
        );
        assert_eq!(harness.observed_calls().signer_calls, 1);
        assert_eq!(harness.observed_calls().prover_calls, 1);
        assert_eq!(harness.observed_calls().post_sign_clock_reads, 1);
        match expected {
            Some(code) => {
                assert_eq!(result.unwrap_err().code(), code);
                assert_no_verified_or_broadcast(&harness);
            }
            None => {
                assert_eq!(result.unwrap().state, "verified");
                assert_eq!(harness.observed_calls().verified_publications, 1);
                assert_eq!(harness.observed_calls().broadcast_calls, 0);
            }
        }
    }
}

#[test]
fn account_authorization_lock_is_scoped_and_released_on_every_exit() {
    let mut harness = software_harness("sign-verify-account-lock");
    harness
        .add_software_fixture_account(AccountId::parse(OTHER_ACCOUNT).unwrap(), &fixture())
        .unwrap();
    harness.scan_account(OTHER_ACCOUNT, &fixture()).unwrap();
    harness
        .unlock_account_with_fixture_seed(OTHER_ACCOUNT)
        .unwrap();
    let handle = prepared(&mut harness);
    let confirmation = harness
        .confirm_native(&handle, &mut ManualClock::at(NOW))
        .unwrap();
    let pending = harness
        .pause_after_account_lock_for_test(
            &handle,
            confirmation,
            SignRoute::Software,
            &mut ManualClock::at(NOW),
        )
        .unwrap();

    let same_account = harness
        .begin_authorization_for_test(ACCOUNT, REQUEST_ID, INTENT_HASH)
        .unwrap_err();
    assert_eq!(same_account.code(), "ACCOUNT_BUSY");
    assert_eq!(harness.account_lock_count(ACCOUNT), 1);
    let other = harness
        .begin_authorization_for_test(OTHER_ACCOUNT, REQUEST_ID, OTHER_INTENT_HASH)
        .unwrap();
    assert_eq!(harness.account_lock_count(OTHER_ACCOUNT), 1);
    harness.cancel_authorization_for_test(other).unwrap();
    assert_eq!(harness.account_lock_count(OTHER_ACCOUNT), 0);
    harness.cancel_paused_for_test(pending).unwrap();
    assert_eq!(harness.account_lock_count(ACCOUNT), 0);
    assert_eq!(harness.observed_calls().broadcast_calls, 0);

    for exit in [
        TerminalExit::Success,
        TerminalExit::Error,
        TerminalExit::Cancellation,
        TerminalExit::Expiry,
        TerminalExit::Lock,
        TerminalExit::PanicUnwind,
        TerminalExit::AccountReplacement,
        TerminalExit::BrokerExit,
    ] {
        let mut harness = software_harness("sign-verify-lock-release");
        harness.exercise_terminal_exit_for_test(exit).unwrap();
        assert_eq!(harness.account_lock_count(ACCOUNT), 0, "exit {exit:?}");
        let next = harness
            .begin_authorization_for_test(ACCOUNT, REQUEST_ID, INTENT_HASH)
            .unwrap();
        harness.cancel_authorization_for_test(next).unwrap();
        assert_eq!(harness.account_lock_count(ACCOUNT), 0);
        assert_eq!(harness.observed_calls().broadcast_calls, 0);
    }

    let mut harness = software_harness("sign-verify-panic-lock-release");
    let handle = prepared(&mut harness);
    let confirmation = harness
        .confirm_native(&handle, &mut ManualClock::at(NOW))
        .unwrap();
    let panic = catch_unwind(AssertUnwindSafe(|| {
        harness.panic_during_sign_for_test(
            &handle,
            confirmation,
            SignRoute::Software,
            &mut ManualClock::at(NOW),
        )
    }));
    assert!(panic.is_err());
    assert_eq!(harness.account_lock_count(ACCOUNT), 0);
    assert_no_verified_or_broadcast(&harness);
    let next = harness
        .begin_authorization_for_test(ACCOUNT, REQUEST_ID, INTENT_HASH)
        .unwrap();
    harness.cancel_authorization_for_test(next).unwrap();
    assert_eq!(harness.account_lock_count(ACCOUNT), 0);
}

#[test]
fn component_and_cleanup_faults_return_stable_closed_errors() {
    for fault in [
        FaultPoint::Signer,
        FaultPoint::Prover,
        FaultPoint::Finalizer,
        FaultPoint::Extractor,
        FaultPoint::Verifier,
        FaultPoint::Cleanup,
    ] {
        let mut harness = software_harness("sign-verify-component-fault");
        let handle = prepared(&mut harness);
        let confirmation = harness
            .confirm_native(&handle, &mut ManualClock::at(NOW))
            .unwrap();
        harness.reset_sign_verify_observations();
        let error = harness
            .sign_with_fault_for_test(
                &handle,
                confirmation,
                SignRoute::Software,
                fault,
                &mut ManualClock::at(NOW),
            )
            .unwrap_err();
        assert_eq!(error.code(), "INTERNAL", "fault {fault:?}");
        assert_eq!(error.public_message(), "Zcash operation failed");
        assert_eq!(format!("{error:?}"), "ZecError { code: \"INTERNAL\" }");
        assert_no_verified_or_broadcast(&harness);
        assert_eq!(harness.account_lock_count(ACCOUNT), 0);
    }
}

#[test]
fn every_sensitive_class_touched_is_positively_wiped_on_every_exit() {
    let classes = software_classes();
    for (exit, wipe_exit) in [
        (TerminalExit::Success, WipeExit::Success),
        (TerminalExit::Error, WipeExit::Error),
        (TerminalExit::Cancellation, WipeExit::Cancellation),
        (TerminalExit::Expiry, WipeExit::Expiry),
        (TerminalExit::Lock, WipeExit::Lock),
        (TerminalExit::PanicUnwind, WipeExit::PanicUnwind),
        (
            TerminalExit::AccountReplacement,
            WipeExit::AccountReplacement,
        ),
        (TerminalExit::BrokerExit, WipeExit::BrokerExit),
    ] {
        let observations = SignVerifyObservations::shared();
        let mut harness = software_harness("sign-verify-wipes");
        harness.attach_sign_verify_observations(observations.clone());
        harness.exercise_terminal_exit_for_test(exit).unwrap();
        drop(harness);
        assert_every_touched_secret_wiped(&observations, wipe_exit);
        if wipe_exit == WipeExit::Success {
            assert_wiped(&observations, wipe_exit, &classes);
        }
    }

    for (fault, wipe_exit) in [
        (FaultPoint::Signer, WipeExit::SignerError),
        (FaultPoint::Prover, WipeExit::ProverError),
        (FaultPoint::Finalizer, WipeExit::FinalizerError),
        (FaultPoint::Extractor, WipeExit::ExtractorError),
        (FaultPoint::Verifier, WipeExit::VerifierError),
        (FaultPoint::Cleanup, WipeExit::CleanupError),
    ] {
        let observations = SignVerifyObservations::shared();
        let mut harness = software_harness("sign-verify-fault-wipes");
        harness.attach_sign_verify_observations(observations.clone());
        harness.exercise_fault_exit_for_test(fault).unwrap();
        drop(harness);
        assert_every_touched_secret_wiped(&observations, wipe_exit);
    }

    let observations = SignVerifyObservations::shared();
    let mut hardware = SignVerifyHarness::synthetic_keystone_from_fixture(
        TestStateRoot::fresh("sign-verify-hardware-wipes"),
        AccountId::parse(ACCOUNT).unwrap(),
        &fixture(),
    )
    .unwrap();
    hardware.attach_sign_verify_observations(observations.clone());
    hardware.exercise_hardware_success_for_test().unwrap();
    drop(hardware);
    assert_wiped(
        &observations,
        WipeExit::Success,
        &[
            TouchedSecretClass::ConfirmationCapability,
            TouchedSecretClass::SignerView,
            TouchedSecretClass::SignatureContribution,
            TouchedSecretClass::AuthoritativePczt,
            TouchedSecretClass::ProofWorkspace,
            TouchedSecretClass::ExtractedTransaction,
        ],
    );
}

#[test]
fn canaries_are_touched_but_absent_from_every_observable_representation() {
    let canaries = SignVerifyCanaries::synthetic_test_only(
        "CANARY_WAL009_SEED_BYTES",
        "CANARY_WAL009_DERIVED_KEY",
        "CANARY_WAL009_PCZT_BYTES",
        "CANARY_WAL009_TRANSACTION_BYTES",
        "CANARY_WAL009_SIGNATURE_BYTES",
        "CANARY_WAL009_RECEIVER_BYTES",
        "CANARY_WAL009_MEMO_BYTES",
    )
    .unwrap();
    let mut harness = software_harness("sign-verify-canaries");
    harness.install_sign_verify_canaries(&canaries).unwrap();
    let handle = prepared(&mut harness);
    let confirmation = harness
        .confirm_native(&handle, &mut ManualClock::at(NOW))
        .unwrap();
    let verified = harness
        .sign_and_verify(
            &handle,
            confirmation,
            SignRoute::Software,
            &mut ManualClock::at(NOW),
        )
        .unwrap();
    let error = harness.synthetic_failure_for_test();
    let panic = catch_unwind(AssertUnwindSafe(|| {
        harness.panic_after_secret_access_for_test()
    }))
    .unwrap_err();
    let panic_text = if let Some(value) = panic.downcast_ref::<&str>() {
        (*value).to_owned()
    } else if let Some(value) = panic.downcast_ref::<String>() {
        value.clone()
    } else {
        panic!("sign/verify panic payload must be a bounded string")
    };
    let observable = format!(
        "verified-debug={verified:?};verified-json={};error-debug={error:?};error-display={error};logs={:?};diagnostics={:?}",
        verified.sanitized_json_for_test(),
        harness.captured_logs(),
        harness.diagnostics(),
    );
    let persisted = harness.persisted_bytes_for_test().unwrap();

    for value in canaries.values() {
        assert!(harness.canary_touch_count(value.class()) > 0);
        assert!(!observable.contains(value.value()));
        assert!(!panic_text.contains(value.value()));
        assert!(
            !persisted
                .windows(value.value().len())
                .any(|window| { window == value.value().as_bytes() })
        );
    }
    assert_eq!(panic_text, "INTERNAL");
    assert_eq!(
        harness.diagnostic_field_names(),
        ["operation", "account_id", "request_id", "state", "code"]
    );
    assert_eq!(harness.observed_calls().broadcast_calls, 0);
}

#[test]
fn operation_and_source_inventories_exclude_broadcast_network_mainnet_xmr_and_real_hardware() {
    let harness = software_harness("sign-verify-authority-inventory");
    assert_eq!(
        harness.public_zec_operations(),
        [
            "account.bootstrap",
            "receiver.fresh",
            "fixture.scan",
            "pczt.prepare",
            "intent.confirm.native",
            "pczt.sign_verify",
        ]
    );
    let capabilities = harness.capabilities();
    assert!(capabilities.can_sign);
    assert!(capabilities.can_prove);
    assert!(capabilities.can_finalize);
    assert!(capabilities.can_extract);
    assert!(capabilities.can_verify);
    assert!(!capabilities.can_broadcast);
    assert!(!capabilities.can_network);
    assert!(!capabilities.can_mainnet);
    assert!(!capabilities.can_xmr);

    for operation in [
        "tx.broadcast",
        "intent.broadcast",
        "network.submit",
        "http.request",
        "grpc.submit",
        "xmr.sign",
        "mainnet.enable",
        "hardware.enumerate",
        "hardware.transport",
    ] {
        assert_eq!(
            harness
                .invoke_operation_for_test(operation)
                .unwrap_err()
                .code(),
            "CAPABILITY_MISSING"
        );
    }

    let manifest = include_str!("../Cargo.toml");
    let spend_source = include_str!("../src/zec/spend.rs");
    let native_source = include_str!("../src/native.rs");
    for forbidden_dependency in [
        "reqwest",
        "ureq",
        "hyper",
        "tonic",
        "tokio",
        "hidapi",
        "rusb",
        "serialport",
        "monero",
    ] {
        assert!(!manifest.contains(forbidden_dependency));
    }
    for forbidden_authority in [
        "std::net",
        "TcpStream",
        "UdpSocket",
        "reqwest",
        "ureq",
        "tonic",
        "broadcast_transaction",
        "submit_transaction",
        "xmr::",
        "hidapi",
        "rusb",
        "serialport",
    ] {
        assert!(
            !spend_source.contains(forbidden_authority),
            "forbidden Phase-A1 authority {forbidden_authority:?}"
        );
    }
    for generic_method in [
        "\"intent.confirm\" =>",
        "\"signer.sign\" =>",
        "\"tx.broadcast\" =>",
    ] {
        assert!(!native_source.contains(generic_method));
    }

    let production = SignVerifyHarness::production_hardware_from_fixture(
        TestStateRoot::fresh("sign-verify-final-empty-production-table"),
        AccountId::parse(ACCOUNT).unwrap(),
        &fixture(),
    )
    .unwrap();
    assert_eq!(production.production_positive_hardware_routes(), 0);
    assert!(production.production_hardware_fingerprints().is_empty());
    assert_eq!(production.observed_calls().broadcast_calls, 0);
}
