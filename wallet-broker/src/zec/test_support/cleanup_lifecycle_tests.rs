use std::panic::{AssertUnwindSafe, catch_unwind};

use crate::vault::SecretBytes;
use crate::zec::prepare::PreparedSigningArtifact;
use crate::zec::{AccountId, HandleInvalidation, PrepareZecV1, ZecError};

use super::{
    AttemptOwner, FaultPoint, FrozenFixture, ManualClock, SignRoute, SignVerifyHarness,
    SignVerifyObservations, TestConfirmation, TestStateRoot, TouchedSecretClass, WipeExit,
};

const ACCOUNT: &str = "00112233445566778899aabbccddeeff";
const REQUEST_ID: &str = "00112233445566778899aabbccddeeff";
const INTENT_HASH: &str = "ad55816f327c002be813a29d41f9a7ae429782b6856a4d3bb2e6c498c6f9e3c0";
const AMOUNT_ZAT: &str = "100000000";
const FEE_BOUND_ZAT: &str = "12000";
const EXPIRES_AT: &str = "2026-08-30T12:15:00Z";
const NOW: &str = "2026-08-30T12:00:30Z";
const FIXTURE_DIR: &str = "tests/fixtures/zec";
const OPERATION_SEED_LABEL: &str = "zec-operation-seed";
const UNKNOWN_OWNER_LABEL: &str = "cleanup-lifecycle-unknown";
const INTENDED_PANIC: &str = "cleanup-lifecycle-intended-panic";
const SEED_17: [u8; 17] = [0xa1; 17];
const SEED_31: [u8; 31] = [0xb2; 31];
const UNKNOWN_BYTES: [u8; 11] = [0xc3; 11];

const SECRET_CLASSES: [TouchedSecretClass; 9] = [
    TouchedSecretClass::Seed,
    TouchedSecretClass::UnifiedSpendingAuthority,
    TouchedSecretClass::DerivedAuthorizingKey,
    TouchedSecretClass::ConfirmationCapability,
    TouchedSecretClass::AuthoritativePczt,
    TouchedSecretClass::ProofWorkspace,
    TouchedSecretClass::ExtractedTransaction,
    TouchedSecretClass::SignerView,
    TouchedSecretClass::SignatureContribution,
];

const WIPE_EXITS: [WipeExit; 14] = [
    WipeExit::Success,
    WipeExit::Error,
    WipeExit::Cancellation,
    WipeExit::Expiry,
    WipeExit::Lock,
    WipeExit::PanicUnwind,
    WipeExit::AccountReplacement,
    WipeExit::BrokerExit,
    WipeExit::SignerError,
    WipeExit::ProverError,
    WipeExit::FinalizerError,
    WipeExit::ExtractorError,
    WipeExit::VerifierError,
    WipeExit::CleanupError,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ClassExitCounts {
    touch: usize,
    positive: usize,
    failed: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ObservationSnapshot {
    cells: [[ClassExitCounts; 14]; 9],
    unclassified: usize,
}

fn observation_snapshot(observations: &SignVerifyObservations) -> ObservationSnapshot {
    let mut cells = [[ClassExitCounts {
        touch: 0,
        positive: 0,
        failed: 0,
    }; 14]; 9];
    for (class_index, class) in SECRET_CLASSES.iter().enumerate() {
        for (exit_index, exit) in WIPE_EXITS.iter().enumerate() {
            cells[class_index][exit_index] = ClassExitCounts {
                touch: observations.touch_count(*class, *exit),
                positive: observations.positive_wipe_count(*class, *exit),
                failed: observations.failed_wipe_count(*class, *exit),
            };
        }
    }
    ObservationSnapshot {
        cells,
        unclassified: observations.unclassified_event_count(),
    }
}

fn expected_counts(
    expected: &[(TouchedSecretClass, WipeExit, usize, usize, usize)],
    class: TouchedSecretClass,
    exit: WipeExit,
) -> (usize, usize, usize) {
    expected
        .iter()
        .find(|(expected_class, expected_exit, _, _, _)| {
            *expected_class == class && *expected_exit == exit
        })
        .map(|(_, _, touch, positive, failed)| (*touch, *positive, *failed))
        .unwrap_or((0, 0, 0))
}

fn assert_exact_observations(
    observations: &SignVerifyObservations,
    expected: &[(TouchedSecretClass, WipeExit, usize, usize, usize)],
    unclassified: usize,
) {
    for class in SECRET_CLASSES {
        for exit in WIPE_EXITS {
            let (touch, positive, failed) = expected_counts(expected, class, exit);
            let actual_touch = observations.touch_count(class, exit);
            let actual_positive = observations.positive_wipe_count(class, exit);
            let actual_failed = observations.failed_wipe_count(class, exit);
            assert_eq!(
                actual_touch, touch,
                "class={class:?} exit={exit:?} touch={actual_touch} expected={touch}"
            );
            assert_eq!(
                actual_positive, positive,
                "class={class:?} exit={exit:?} positive={actual_positive} expected={positive}"
            );
            assert_eq!(
                actual_failed, failed,
                "class={class:?} exit={exit:?} failed={actual_failed} expected={failed}"
            );
        }
    }
    let actual_unclassified = observations.unclassified_event_count();
    assert_eq!(
        actual_unclassified, unclassified,
        "unclassified={actual_unclassified} expected={unclassified}"
    );
}

fn assert_no_published_observations(observations: &SignVerifyObservations) {
    assert_exact_observations(observations, &[], 0);
}

fn assert_unchanged_snapshot(
    observations: &SignVerifyObservations,
    snapshot: ObservationSnapshot,
) {
    assert_eq!(snapshot, observation_snapshot(observations));
}

fn owned_nonzero(secret: &SecretBytes) -> bool {
    secret.expose(|bytes| !bytes.is_empty() && bytes.iter().all(|byte| *byte != 0))
}

fn observed_seed(attempt: &AttemptOwner, bytes: &[u8]) -> crate::vault::ObservedSecretBytes {
    SecretBytes::new_observed(
        bytes.to_vec(),
        OPERATION_SEED_LABEL,
        Box::new(attempt.collector()),
    )
    .expect("observed seed owner")
}

fn drop_nonzero_seed(attempt: &AttemptOwner, bytes: &[u8]) {
    let owner = observed_seed(attempt, bytes);
    assert!(
        owned_nonzero(owner.as_secret()),
        "class={:?} length={}",
        TouchedSecretClass::Seed,
        bytes.len()
    );
    drop(owner);
}

fn is_intended_panic(payload: &(dyn std::any::Any + Send)) -> bool {
    payload
        .downcast_ref::<&str>()
        .is_some_and(|value| *value == INTENDED_PANIC)
        || payload
            .downcast_ref::<String>()
            .is_some_and(|value| value == INTENDED_PANIC)
}

fn fixture() -> FrozenFixture {
    FrozenFixture::open(FIXTURE_DIR).expect("open zec fixture")
}

fn ordinary_intent() -> PrepareZecV1 {
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
    .expect("ordinary coffee intent")
}

fn software_harness(label: &str) -> SignVerifyHarness {
    let frozen = fixture();
    let mut harness = SignVerifyHarness::software_from_fixture(
        TestStateRoot::fresh(label),
        AccountId::parse(ACCOUNT).expect("fixture account id"),
        &frozen,
    )
    .expect("software harness");
    harness.scan(&frozen).expect("scan fixture");
    harness
        .unlock_with_fixture_seed()
        .expect("unlock fixture seed");
    harness
}

fn confirmed_software(label: &str) -> (SignVerifyHarness, String, TestConfirmation) {
    let mut harness = software_harness(label);
    let prepared = harness
        .prepare(ordinary_intent(), &mut ManualClock::at(NOW))
        .expect("prepare ordinary fixture");
    let handle = prepared.handle;
    let confirmation = harness
        .confirm_native(&handle, &mut ManualClock::at(NOW))
        .expect("confirm ordinary fixture");
    (harness, handle, confirmation)
}

pub(crate) fn prepared_artifact(label: &str) -> PreparedSigningArtifact {
    let (harness, _handle, confirmation) = confirmed_software(label);
    let account = harness
        .accounts
        .get(&harness.primary_account)
        .expect("fixture account");
    account
        .prepare
        .consume(&confirmation.review, NOW)
        .expect("consume prepared artifact")
}

fn assert_two_positive_seeds(observations: &SignVerifyObservations, exit: WipeExit) {
    assert_exact_observations(
        observations,
        &[(TouchedSecretClass::Seed, exit, 2, 2, 0)],
        0,
    );
}

fn assert_preconsume_calls(harness: &SignVerifyHarness) {
    let calls = harness.observed_calls();
    assert_eq!(calls.seed_accesses, 1);
    assert_eq!(calls.spend_authority_derivations, 1);
    assert_eq!(calls.authoritative_pczt_accesses, 0);
    assert_eq!(calls.signer_calls, 0);
    assert_eq!(calls.prover_calls, 0);
    assert_eq!(calls.finalizer_calls, 0);
    assert_eq!(calls.extractor_calls, 0);
    assert_eq!(calls.independent_decoder_calls, 0);
    assert_eq!(calls.verifier_calls, 0);
    assert_eq!(calls.post_sign_status_reads, 0);
    assert_eq!(calls.post_sign_clock_reads, 0);
    assert_eq!(calls.verified_publications, 0);
    assert_eq!(calls.broadcast_calls, 0);
    assert_eq!(harness.verified_handle_count(), 0);
    assert_eq!(harness.account_lock_count(ACCOUNT), 0);
}

enum PreconsumeSetup {
    Cancel,
    None,
    MissingHandle,
}

#[test]
fn attempt_defers_classification_until_owner_drop() {
    let success = SignVerifyObservations::shared();
    {
        let mut attempt = AttemptOwner::new(success.clone());
        drop_nonzero_seed(&attempt, &SEED_17);
        assert_no_published_observations(&success);
        attempt.finish(WipeExit::Success);
        assert_no_published_observations(&success);
        let second = observed_seed(&attempt, &SEED_31);
        assert!(
            owned_nonzero(second.as_secret()),
            "class={:?} length=31",
            TouchedSecretClass::Seed
        );
        assert_no_published_observations(&success);
    }
    assert_two_positive_seeds(&success, WipeExit::Success);
    let frozen = observation_snapshot(&success);
    assert_unchanged_snapshot(&success, frozen);

    let unfinished = SignVerifyObservations::shared();
    let error = {
        let attempt = AttemptOwner::new(unfinished.clone());
        drop_nonzero_seed(&attempt, &SEED_17);
        assert_no_published_observations(&unfinished);
        let second = observed_seed(&attempt, &SEED_31);
        assert!(
            owned_nonzero(second.as_secret()),
            "class={:?} length=31",
            TouchedSecretClass::Seed
        );
        assert_no_published_observations(&unfinished);
        Err::<(), ZecError>(ZecError::internal())
    };
    assert_eq!(error.expect_err("unfinished attempt").code(), "INTERNAL");
    assert_two_positive_seeds(&unfinished, WipeExit::Error);
    let frozen = observation_snapshot(&unfinished);
    assert_unchanged_snapshot(&unfinished, frozen);
}

#[test]
fn panic_overrides_selected_outcome() {
    let observations = SignVerifyObservations::shared();
    let mut reached = false;
    let result = catch_unwind(AssertUnwindSafe(|| {
        let mut attempt = AttemptOwner::new(observations.clone());
        drop_nonzero_seed(&attempt, &SEED_17);
        attempt.finish(WipeExit::Success);
        let second = observed_seed(&attempt, &SEED_31);
        assert!(
            owned_nonzero(second.as_secret()),
            "class={:?} length=31",
            TouchedSecretClass::Seed
        );
        assert_no_published_observations(&observations);
        reached = true;
        panic!("{INTENDED_PANIC}");
    }));
    assert!(reached, "exit={:?}", WipeExit::PanicUnwind);
    assert!(
        is_intended_panic(result.expect_err("intended panic").as_ref()),
        "exit={:?}",
        WipeExit::PanicUnwind
    );
    assert_two_positive_seeds(&observations, WipeExit::PanicUnwind);
    let frozen = observation_snapshot(&observations);
    assert_unchanged_snapshot(&observations, frozen);
}

#[test]
fn preconsume_errors_use_actual_exit() {
    for (label, setup, clock, code, exit) in [
        (
            "cleanup-lifecycle-cancel",
            PreconsumeSetup::Cancel,
            NOW,
            "CANCELLED",
            WipeExit::Cancellation,
        ),
        (
            "cleanup-lifecycle-expiry",
            PreconsumeSetup::None,
            EXPIRES_AT,
            "EXPIRED",
            WipeExit::Expiry,
        ),
        (
            "cleanup-lifecycle-missing-handle",
            PreconsumeSetup::MissingHandle,
            NOW,
            "LOCKED",
            WipeExit::Lock,
        ),
        (
            "cleanup-lifecycle-invalid-timestamp",
            PreconsumeSetup::None,
            "invalid-timestamp",
            "SCHEMA",
            WipeExit::Error,
        ),
    ] {
        let (mut harness, handle, confirmation) = confirmed_software(label);
        match setup {
            PreconsumeSetup::Cancel => {
                harness
                    .accounts
                    .get(&harness.primary_account)
                    .expect("fixture account")
                    .prepare
                    .cancel_request(REQUEST_ID);
            }
            PreconsumeSetup::None => {}
            PreconsumeSetup::MissingHandle => {
                harness
                    .accounts
                    .get(&harness.primary_account)
                    .expect("fixture account")
                    .prepare
                    .invalidate(HandleInvalidation::Expiry);
            }
        }
        harness.reset_sign_verify_observations();
        let observations = SignVerifyObservations::shared();
        harness.attach_sign_verify_observations(observations.clone());
        let error = harness
            .sign_with_fault_for_test(
                &handle,
                confirmation,
                SignRoute::Software,
                FaultPoint::Signer,
                &mut ManualClock::at(clock),
            )
            .expect_err("pre-consume operation error");
        assert_eq!(error.code(), code, "exit={exit:?}");
        assert_preconsume_calls(&harness);
        assert_exact_observations(
            &observations,
            &[(TouchedSecretClass::Seed, exit, 1, 1, 0)],
            0,
        );
        let frozen = observation_snapshot(&observations);
        drop(harness);
        assert_unchanged_snapshot(&observations, frozen);
    }
}

#[test]
fn empty_and_unknown_owners_never_report_positive_known_wipes() {
    let observations = SignVerifyObservations::shared();
    {
        let mut attempt = AttemptOwner::new(observations.clone());
        let empty = SecretBytes::new_observed(
            Vec::new(),
            OPERATION_SEED_LABEL,
            Box::new(attempt.collector()),
        )
        .expect("empty seed owner");
        let unknown = SecretBytes::new_observed(
            UNKNOWN_BYTES.to_vec(),
            UNKNOWN_OWNER_LABEL,
            Box::new(attempt.collector()),
        )
        .expect("unknown owner");
        assert!(
            owned_nonzero(unknown.as_secret()),
            "length={}",
            UNKNOWN_BYTES.len()
        );
        drop(empty);
        drop(unknown);
        attempt.finish(WipeExit::Success);
        assert_no_published_observations(&observations);
    }
    assert_exact_observations(
        &observations,
        &[(TouchedSecretClass::Seed, WipeExit::Success, 1, 0, 1)],
        1,
    );
    let frozen = observation_snapshot(&observations);
    assert_unchanged_snapshot(&observations, frozen);
}
