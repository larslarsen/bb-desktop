use std::panic::{AssertUnwindSafe, catch_unwind};

use bitbook_wallet_broker::zec::test_support::{
    CapabilityFlag, ClaimedRoute, DeviceFingerprint, DeviceVendor, FingerprintField,
    HardwareCanaries, HardwareCanarySlot, HardwareStateRoot, HardwareStoreFault,
    HardwareTestHarness, LiveProbe, PersistedDecisionMutation, ProbeMutation, ReviewedProfile,
    SigningPool, VerifiedField,
};

const SYNTHETIC_MODEL: &str = "BITBOOKSYNTHETICKEYSTONE";
const SYNTHETIC_APP: &str = "BITBOOKZECTESTAPP";
const SYNTHETIC_APP_VERSION: &str = "000TESTONLY";
const CONSENSUS_BRANCH: &str = "37a5165b";
const TRANSACTION_VERSION: &str = "6";
const PCZT_ENCODING_VERSION: &str = "2";

fn synthetic_profile() -> ReviewedProfile {
    let profile = ReviewedProfile::synthetic_keystone_test_only();
    assert!(profile.is_test_only());
    assert_eq!(profile.fingerprint().vendor(), DeviceVendor::Keystone);
    assert_eq!(profile.fingerprint().model(), SYNTHETIC_MODEL);
    assert_eq!(profile.fingerprint().app_name(), SYNTHETIC_APP);
    assert_eq!(profile.fingerprint().app_version(), SYNTHETIC_APP_VERSION);
    profile
}

fn synthetic_harness(label: &str) -> HardwareTestHarness {
    HardwareTestHarness::with_reviewed_profiles(
        HardwareStateRoot::fresh(label),
        vec![synthetic_profile()],
    )
    .unwrap()
}

fn exact_fingerprint() -> DeviceFingerprint {
    synthetic_profile().fingerprint().clone()
}

fn exact_probe() -> LiveProbe {
    LiveProbe::synthetic_keystone_test_only()
}

fn assert_no_private_spend_authority(
    decision: &bitbook_wallet_broker::zec::test_support::HardwareDecision,
) {
    assert!(!decision.capabilities.can_prepare_tx);
    assert!(!decision.capabilities.can_sign_spend);
    assert!(!decision.capabilities.can_sign_ironwood);
    assert!(!decision.capabilities.can_tx_v6);
    assert!(!decision.capabilities.can_verify_pczt_on_device);
    assert!(decision.capabilities.allowed_signing_pools.is_empty());
    assert!(decision.route.is_none());
}

fn bytes_contain(haystack: &[u8], needle: &[u8]) -> bool {
    !needle.is_empty()
        && haystack
            .windows(needle.len())
            .any(|window| window == needle)
}

fn assert_protocol_incompatible(mutation: ProbeMutation) {
    let mut harness = synthetic_harness("hardware-protocol-exactness");
    let probe = exact_probe().with_mutations(&[mutation]).unwrap();
    let decision = harness.decide(&exact_fingerprint(), &probe).unwrap();
    assert_eq!(decision.status.code(), "PROTOCOL_INCOMPATIBLE");
    assert_no_private_spend_authority(&decision);
    assert_eq!(harness.persistence_attempts(), 0);
}

#[test]
fn production_table_has_zero_positive_real_device_entries() {
    let mut harness = HardwareTestHarness::production(HardwareStateRoot::fresh(
        "hardware-empty-production-table",
    ))
    .unwrap();
    assert_eq!(harness.reviewed_profile_count(), 0);
    assert!(harness.reviewed_fingerprint_digests().is_empty());

    let decision = harness
        .decide(&exact_fingerprint(), &exact_probe())
        .unwrap();
    assert_eq!(decision.status.code(), "CAPABILITY_MISSING");
    assert_no_private_spend_authority(&decision);
    assert_eq!(harness.persistence_attempts(), 0);
}

#[test]
fn unknown_and_one_field_mismatched_fingerprints_deny_every_private_spend_route() {
    let exact = exact_fingerprint();
    let fingerprints = [
        exact.with_vendor_for_test(DeviceVendor::Ledger),
        exact
            .with_component_for_test(FingerprintField::Model, "UNKNOWNMODEL")
            .unwrap(),
        exact
            .with_component_for_test(FingerprintField::AppName, "UNKNOWNAPP")
            .unwrap(),
        exact
            .with_component_for_test(FingerprintField::AppVersion, "001TESTONLY")
            .unwrap(),
        exact
            .with_component_for_test(FingerprintField::Model, "bitbooksynthetickeystone")
            .unwrap(),
    ];

    for fingerprint in fingerprints {
        let mut harness = synthetic_harness("hardware-fingerprint-mismatch");
        let decision = harness.decide(&fingerprint, &exact_probe()).unwrap();
        assert_eq!(decision.status.code(), "CAPABILITY_MISSING");
        assert_no_private_spend_authority(&decision);
        assert_eq!(harness.persistence_attempts(), 0);
    }
}

#[test]
fn exact_complete_synthetic_keystone_intersection_selects_metadata_only_pczt_v2_route() {
    let profile = synthetic_profile();
    let mut harness = synthetic_harness("hardware-keystone-positive");
    assert_eq!(harness.reviewed_profile_count(), 1);
    assert_eq!(harness.positive_profile_count(), 1);
    let decision = harness
        .decide(&exact_fingerprint(), &exact_probe())
        .unwrap();

    assert_eq!(decision.status.code(), "READY");
    assert_eq!(
        decision.route.as_ref().unwrap().as_str(),
        "keystone_pczt_v2"
    );
    assert!(decision.capabilities.can_receive_private);
    assert!(decision.capabilities.can_prepare_tx);
    assert!(decision.capabilities.can_sign_spend);
    assert!(decision.capabilities.can_sign_ironwood);
    assert!(decision.capabilities.can_tx_v6);
    assert!(decision.capabilities.can_verify_pczt_on_device);
    assert_eq!(
        decision.capabilities.transaction_version,
        TRANSACTION_VERSION
    );
    assert_eq!(decision.capabilities.consensus_branch, CONSENSUS_BRANCH);
    assert_eq!(
        decision.capabilities.pczt_encoding_version,
        PCZT_ENCODING_VERSION
    );
    assert_eq!(
        decision.capabilities.allowed_signing_pools,
        [SigningPool::Ironwood]
    );
    assert_eq!(decision.table_revision, profile.table_revision());
    assert_eq!(decision.fingerprint_digest.len(), 64);
    assert!(
        decision
            .fingerprint_digest
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    );

    let route = harness.select_route(&decision).unwrap();
    assert_eq!(route.route.as_str(), "keystone_pczt_v2");
    assert_eq!(
        route.public_field_names(),
        [
            "route",
            "fingerprint_digest",
            "table_revision",
            "transaction_version",
            "consensus_branch",
            "pczt_encoding_version",
            "signing_pools",
            "verified_fields",
            "host_trusting_fields",
        ]
    );
    assert_eq!(route.accepted_artifact_bytes(), 0);
    assert_eq!(route.returned_artifact_bytes(), 0);
    assert!(!decision.capabilities.can_broadcast);
    assert!(!decision.capabilities.can_sign_transparent);
    assert!(!decision.capabilities.can_sign_orchard);
    assert!(!decision.capabilities.can_migrate_orchard_to_ironwood);
    assert_eq!(harness.software_fallback_count(), 0);
    assert_eq!(harness.other_device_fallback_count(), 0);
    assert_eq!(harness.pczt_mutation_count(), 0);
    assert_eq!(harness.proof_call_count(), 0);
    assert_eq!(harness.finalization_call_count(), 0);
    assert_eq!(harness.extraction_call_count(), 0);
    assert_eq!(harness.signing_call_count(), 0);
    assert_eq!(harness.broadcast_call_count(), 0);
    assert!(harness.forbidden_authority_observation().is_zero());

    for mutation in [
        ProbeMutation::Capability(CapabilityFlag::CanPrepareTx, false),
        ProbeMutation::Capability(CapabilityFlag::CanSignSpend, false),
        ProbeMutation::Capability(CapabilityFlag::CanSignIronwood, false),
        ProbeMutation::Capability(CapabilityFlag::CanTxV6, false),
        ProbeMutation::Capability(CapabilityFlag::CanVerifyPcztOnDevice, false),
        ProbeMutation::SigningPool(SigningPool::Ironwood, false),
    ] {
        let mut incomplete = synthetic_harness("hardware-keystone-incomplete");
        let probe = exact_probe().with_mutations(&[mutation]).unwrap();
        let denied = incomplete.decide(&exact_fingerprint(), &probe).unwrap();
        assert_ne!(denied.status.code(), "READY");
        assert!(denied.route.is_none());
        assert_eq!(incomplete.signing_call_count(), 0);
    }
}

#[test]
fn live_claims_cannot_expand_reviewed_booleans_pools_routes_or_fields() {
    for capability in [
        CapabilityFlag::CanView,
        CapabilityFlag::CanDeriveFreshReceiver,
        CapabilityFlag::CanReceivePrivate,
        CapabilityFlag::CanReceiveOrchard,
        CapabilityFlag::CanReceiveIronwood,
        CapabilityFlag::CanPrepareTx,
        CapabilityFlag::CanSignSpend,
        CapabilityFlag::CanSignOrchard,
        CapabilityFlag::CanSignIronwood,
        CapabilityFlag::CanTxV6,
        CapabilityFlag::CanMigrateOrchardToIronwood,
        CapabilityFlag::CanSignTransparent,
        CapabilityFlag::CanDisplayAmountOnDevice,
        CapabilityFlag::CanDisplayRecipientOnDevice,
        CapabilityFlag::CanDisplayNetworkOnDevice,
        CapabilityFlag::CanVerifyPcztOnDevice,
        CapabilityFlag::CanExportViewingMaterial,
        CapabilityFlag::CanBroadcast,
    ] {
        let profile = synthetic_profile().without_capability_for_test(capability);
        let mut harness = HardwareTestHarness::with_reviewed_profiles(
            HardwareStateRoot::fresh("hardware-boolean-expansion"),
            vec![profile],
        )
        .unwrap();
        let probe = exact_probe()
            .with_mutations(&[ProbeMutation::Capability(capability, true)])
            .unwrap();
        let decision = harness.decide(&exact_fingerprint(), &probe).unwrap();
        assert!(
            !decision.capabilities.contains(capability),
            "{capability:?}"
        );
    }

    let profile = synthetic_profile()
        .without_capability_for_test(CapabilityFlag::CanBroadcast)
        .without_capability_for_test(CapabilityFlag::CanSignTransparent)
        .without_signing_pool_for_test(SigningPool::Orchard)
        .without_verified_field_for_test(VerifiedField::Memo);
    let mut harness = HardwareTestHarness::with_reviewed_profiles(
        HardwareStateRoot::fresh("hardware-live-expansion"),
        vec![profile],
    )
    .unwrap();
    let probe = exact_probe()
        .with_mutations(&[
            ProbeMutation::Capability(CapabilityFlag::CanBroadcast, true),
            ProbeMutation::Capability(CapabilityFlag::CanSignTransparent, true),
            ProbeMutation::SigningPool(SigningPool::Orchard, true),
            ProbeMutation::VerifiedField(VerifiedField::Memo, true),
            ProbeMutation::ClaimedRoute(ClaimedRoute::Software),
            ProbeMutation::ClaimedRoute(ClaimedRoute::OtherDevice),
        ])
        .unwrap();

    let decision = harness.decide(&exact_fingerprint(), &probe).unwrap();
    assert_eq!(decision.status.code(), "READY");
    assert!(!decision.capabilities.can_broadcast);
    assert!(!decision.capabilities.can_sign_transparent);
    assert!(
        !decision
            .capabilities
            .allowed_signing_pools
            .contains(&SigningPool::Orchard)
    );
    assert!(!decision.verified_fields.contains(&VerifiedField::Memo));
    assert_eq!(
        decision.route.as_ref().unwrap().as_str(),
        "keystone_pczt_v2"
    );
    assert!(!decision.route_claims.contains(&ClaimedRoute::Software));
    assert!(!decision.route_claims.contains(&ClaimedRoute::OtherDevice));
}

#[test]
fn branch_transaction_and_pczt_versions_require_exact_values() {
    for mutation in [
        ProbeMutation::ConsensusBranch("37a5165".to_owned()),
        ProbeMutation::ConsensusBranch("37A5165B".to_owned()),
        ProbeMutation::ConsensusBranch("37a5165b-37a5165c".to_owned()),
        ProbeMutation::ConsensusBranch("37a5165a".to_owned()),
        ProbeMutation::TransactionVersion("5".to_owned()),
        ProbeMutation::TransactionVersion("06".to_owned()),
        ProbeMutation::TransactionVersion("6-7".to_owned()),
        ProbeMutation::PcztEncodingVersion("1".to_owned()),
        ProbeMutation::PcztEncodingVersion("02".to_owned()),
        ProbeMutation::PcztEncodingVersion("2-3".to_owned()),
    ] {
        assert_protocol_incompatible(mutation);
    }
}

#[test]
fn malformed_probe_and_absence_follow_fail_closed_decision_precedence() {
    let mut malformed = synthetic_harness("hardware-malformed-probe");
    let probe = exact_probe()
        .with_mutations(&[ProbeMutation::TransactionVersion("six".to_owned())])
        .unwrap();
    let error = malformed.decide(&exact_fingerprint(), &probe).unwrap_err();
    assert_eq!(error.code(), "SCHEMA");
    assert_eq!(malformed.persistence_attempts(), 0);

    let mut absent = synthetic_harness("hardware-absent-precedence");
    let fingerprint = exact_fingerprint().with_vendor_for_test(DeviceVendor::Ledger);
    let probe = exact_probe()
        .with_mutations(&[ProbeMutation::Present(false)])
        .unwrap();
    let decision = absent.decide(&fingerprint, &probe).unwrap();
    assert_eq!(decision.status.code(), "DEVICE_DISCONNECTED");
    assert_no_private_spend_authority(&decision);
    assert_eq!(absent.persistence_attempts(), 0);
}

#[test]
fn disconnected_device_never_falls_back_to_software_or_another_device() {
    let mut harness = synthetic_harness("hardware-disconnect");
    let probe = exact_probe()
        .with_mutations(&[
            ProbeMutation::Present(false),
            ProbeMutation::ClaimedRoute(ClaimedRoute::Software),
            ProbeMutation::ClaimedRoute(ClaimedRoute::OtherDevice),
        ])
        .unwrap();
    let decision = harness.decide(&exact_fingerprint(), &probe).unwrap();
    assert_eq!(decision.status.code(), "DEVICE_DISCONNECTED");
    assert_no_private_spend_authority(&decision);
    assert!(decision.route_claims.is_empty());
    assert_eq!(harness.software_fallback_count(), 0);
    assert_eq!(harness.other_device_fallback_count(), 0);
}

#[test]
fn transparent_only_trezor_is_not_private_or_pay_eligible() {
    let profile = ReviewedProfile::synthetic_trezor_transparent_negative();
    assert!(profile.is_test_only());
    let fingerprint = profile.fingerprint().clone();
    let mut harness = HardwareTestHarness::with_reviewed_profiles(
        HardwareStateRoot::fresh("hardware-trezor-transparent"),
        vec![profile],
    )
    .unwrap();
    let decision = harness
        .decide(&fingerprint, &LiveProbe::synthetic_trezor_transparent())
        .unwrap();

    assert_eq!(decision.status.code(), "CAPABILITY_MISSING");
    assert_eq!(decision.privacy.as_str(), "transparent_not_private");
    assert!(decision.capabilities.can_sign_transparent);
    assert!(!decision.capabilities.can_receive_private);
    assert_no_private_spend_authority(&decision);
    assert!(!decision.pay_eligible);
}

#[test]
fn unverified_ledger_never_signs_ironwood() {
    let profile = ReviewedProfile::synthetic_ledger_unverified_negative();
    assert!(profile.is_test_only());
    let fingerprint = profile.fingerprint().clone();
    let mut harness = HardwareTestHarness::with_reviewed_profiles(
        HardwareStateRoot::fresh("hardware-ledger-unverified"),
        vec![profile],
    )
    .unwrap();
    let probe = LiveProbe::synthetic_ledger_unverified()
        .with_mutations(&[
            ProbeMutation::Capability(CapabilityFlag::CanSignIronwood, true),
            ProbeMutation::Capability(CapabilityFlag::CanVerifyPcztOnDevice, true),
        ])
        .unwrap();
    let decision = harness.decide(&fingerprint, &probe).unwrap();

    assert_eq!(decision.status.code(), "CAPABILITY_MISSING");
    assert!(!decision.capabilities.can_sign_ironwood);
    assert!(!decision.capabilities.can_sign_spend);
    assert!(!decision.capabilities.can_verify_pczt_on_device);
    assert!(!decision.pay_eligible);
    assert!(decision.route.is_none());
}

#[test]
fn persisted_narrowing_reopens_without_expansion_and_requires_a_fresh_exact_restoration() {
    let mut harness = synthetic_harness("hardware-persist-reopen");
    let previously_wide = harness
        .decide(&exact_fingerprint(), &exact_probe())
        .unwrap();
    assert!(previously_wide.capabilities.can_display_recipient_on_device);
    assert!(
        previously_wide
            .verified_fields
            .contains(&VerifiedField::Recipient)
    );

    let probe = exact_probe()
        .with_mutations(&[
            ProbeMutation::Capability(CapabilityFlag::CanDisplayRecipientOnDevice, false),
            ProbeMutation::VerifiedField(VerifiedField::Recipient, false),
        ])
        .unwrap();
    let narrowed = harness.decide(&exact_fingerprint(), &probe).unwrap();
    assert!(!narrowed.capabilities.can_display_recipient_on_device);
    assert!(!narrowed.verified_fields.contains(&VerifiedField::Recipient));
    assert!(
        narrowed
            .host_trusting_fields
            .contains(&VerifiedField::Recipient)
    );
    harness.persist(&narrowed).unwrap();
    let persisted_before = harness.persisted_bytes().unwrap();
    assert!(!persisted_before.is_empty());
    for raw_fingerprint_component in [SYNTHETIC_MODEL, SYNTHETIC_APP, SYNTHETIC_APP_VERSION] {
        assert!(!bytes_contain(
            &persisted_before,
            raw_fingerprint_component.as_bytes()
        ));
    }

    let mut reopened = harness.reopen().unwrap();
    assert_eq!(reopened.ready_decision().unwrap(), &narrowed);
    assert_eq!(reopened.persisted_bytes().unwrap(), persisted_before);
    assert_eq!(reopened.published_ready_count(), 1);

    let error = reopened.persist(&previously_wide).unwrap_err();
    assert_eq!(error.code(), "STATE_CORRUPT");
    assert_eq!(reopened.ready_decision().unwrap(), &narrowed);
    assert_eq!(reopened.persisted_bytes().unwrap(), persisted_before);

    let freshly_restored = reopened
        .decide(&exact_fingerprint(), &exact_probe())
        .unwrap();
    assert!(
        freshly_restored
            .capabilities
            .can_display_recipient_on_device
    );
    assert!(
        freshly_restored
            .verified_fields
            .contains(&VerifiedField::Recipient)
    );
    reopened.persist(&freshly_restored).unwrap();
    let restored_bytes = reopened.persisted_bytes().unwrap();
    assert_ne!(restored_bytes, persisted_before);

    let restored_reopen = reopened.reopen().unwrap();
    assert_eq!(restored_reopen.ready_decision().unwrap(), &freshly_restored);
    assert_eq!(restored_reopen.persisted_bytes().unwrap(), restored_bytes);
}

#[test]
fn write_file_sync_directory_sync_and_commit_faults_publish_nothing_and_preserve_prior_bytes() {
    for fault in [
        HardwareStoreFault::Write,
        HardwareStoreFault::FileSync,
        HardwareStoreFault::DirectorySync,
        HardwareStoreFault::Commit,
    ] {
        let mut harness = synthetic_harness("hardware-persistence-fault");
        let decision = harness
            .decide(&exact_fingerprint(), &exact_probe())
            .unwrap();
        let before = harness.persisted_bytes().unwrap();
        let error = harness.persist_with_fault(&decision, fault).unwrap_err();
        assert_eq!(error.code(), "INTERNAL");
        assert_eq!(
            harness.persisted_bytes().unwrap(),
            before,
            "fault {fault:?}"
        );
        assert_eq!(harness.published_ready_count(), 0, "fault {fault:?}");
    }
}

#[test]
fn invalid_records_and_reopen_drift_fail_closed_without_ready_publication() {
    for mutation in [
        PersistedDecisionMutation::UnknownField,
        PersistedDecisionMutation::DuplicateVerifiedField,
        PersistedDecisionMutation::InvalidBoolean,
        PersistedDecisionMutation::OutOfRangeTransactionVersion,
        PersistedDecisionMutation::InvalidFingerprintDigest,
        PersistedDecisionMutation::UnknownStatus,
        PersistedDecisionMutation::SchemaRevisionDrift,
        PersistedDecisionMutation::PartialWrite,
        PersistedDecisionMutation::Rollback,
        PersistedDecisionMutation::TableRevisionDrift,
        PersistedDecisionMutation::ConsensusDrift,
    ] {
        let mut harness = synthetic_harness("hardware-persisted-corruption");
        let decision = harness
            .decide(&exact_fingerprint(), &exact_probe())
            .unwrap();
        harness.persist(&decision).unwrap();
        harness.mutate_persisted_for_test(mutation).unwrap();
        let error = harness.reopen_in_place().unwrap_err();
        assert_eq!(error.code(), "STATE_CORRUPT", "mutation {mutation:?}");
        assert_eq!(harness.published_ready_count(), 0, "mutation {mutation:?}");
    }
}

#[test]
fn fingerprint_component_lengths_cover_both_immediate_boundaries() {
    for field in [
        FingerprintField::Model,
        FingerprintField::AppName,
        FingerprintField::AppVersion,
    ] {
        for (length, accepted) in [
            (0, false),
            (1, true),
            (2, true),
            (63, true),
            (64, true),
            (65, false),
        ] {
            let result = exact_fingerprint().with_component_for_test(field, &"A".repeat(length));
            assert_eq!(result.is_ok(), accepted, "field {field:?}, length {length}");
            if !accepted {
                assert_eq!(result.unwrap_err().code(), "SCHEMA");
            }
        }
    }
}

#[test]
fn fingerprint_components_accept_exact_allowed_ascii_alphabet() {
    const ALLOWED: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789._+-";
    assert_eq!(ALLOWED.len(), 66);

    for field in [
        FingerprintField::Model,
        FingerprintField::AppName,
        FingerprintField::AppVersion,
    ] {
        for byte in 0_u8..=127 {
            let value = char::from(byte).to_string();
            let result = exact_fingerprint().with_component_for_test(field, &value);
            let accepted = ALLOWED.contains(&byte);
            assert_eq!(
                result.is_ok(),
                accepted,
                "field {field:?}, byte {byte:#04x}"
            );
            if !accepted {
                assert_eq!(result.unwrap_err().code(), "SCHEMA");
            }
        }
    }
}

#[test]
fn fingerprint_components_reject_non_visible_or_ambiguous_bytes_without_normalization() {
    for value in [
        "MODEL WITH SPACE",
        "MODEL\tTAB",
        "MODEL\nLINE",
        "MODEL\0NUL",
        "MODEL/SLASH",
        "MODEL:COLON",
        "MODEL|PIPE",
        "MODEL,COMMA",
        "MODEL=EQUALS",
        "MODEL*WILDCARD",
        "MODEL?WILDCARD",
        "MÖDEL",
    ] {
        let error = exact_fingerprint()
            .with_component_for_test(FingerprintField::Model, value)
            .unwrap_err();
        assert_eq!(error.code(), "SCHEMA", "value {value:?}");
    }

    let lower = exact_fingerprint()
        .with_component_for_test(FingerprintField::Model, "bitbooksynthetickeystone")
        .unwrap();
    assert_ne!(lower, exact_fingerprint());
}

#[test]
fn verified_fields_are_intersected_and_every_omission_is_host_trusting() {
    let profile = synthetic_profile().without_verified_field_for_test(VerifiedField::Memo);
    let mut harness = HardwareTestHarness::with_reviewed_profiles(
        HardwareStateRoot::fresh("hardware-verified-fields"),
        vec![profile],
    )
    .unwrap();
    let probe = exact_probe()
        .with_mutations(&[
            ProbeMutation::VerifiedField(VerifiedField::Recipient, false),
            ProbeMutation::VerifiedField(VerifiedField::Fee, false),
            ProbeMutation::VerifiedField(VerifiedField::Memo, true),
        ])
        .unwrap();
    let decision = harness.decide(&exact_fingerprint(), &probe).unwrap();

    assert_eq!(
        decision.verified_fields,
        [VerifiedField::Amount, VerifiedField::Network]
    );
    assert_eq!(
        decision.host_trusting_fields,
        [
            VerifiedField::Recipient,
            VerifiedField::Fee,
            VerifiedField::Memo,
        ]
    );
    assert!(!decision.electron_verified_fields);

    harness.persist(&decision).unwrap();
    let reopened = harness.reopen().unwrap();
    assert_eq!(
        reopened.ready_decision().unwrap().verified_fields,
        [VerifiedField::Amount, VerifiedField::Network]
    );
    assert_eq!(
        reopened.ready_decision().unwrap().host_trusting_fields,
        [
            VerifiedField::Recipient,
            VerifiedField::Fee,
            VerifiedField::Memo,
        ]
    );
}

#[test]
fn success_error_debug_panic_and_persistence_representations_are_redacted() {
    const CANARY_INPUTS: [(HardwareCanarySlot, &str); 9] = [
        (HardwareCanarySlot::RawProbe, "CANARY_WAL008_RAW_PROBE_8101"),
        (
            HardwareCanarySlot::FingerprintModel,
            "CANARYWAL008MODEL8102",
        ),
        (
            HardwareCanarySlot::FingerprintAppName,
            "CANARYWAL008APP8103",
        ),
        (
            HardwareCanarySlot::FingerprintAppVersion,
            "CANARYWAL008VERSION8104",
        ),
        (
            HardwareCanarySlot::DeviceLabel,
            "CANARY_WAL008_DEVICE_LABEL_8105",
        ),
        (
            HardwareCanarySlot::PcztBytes,
            "CANARY_WAL008_PCZT_BYTES_8106",
        ),
        (HardwareCanarySlot::Address, "CANARY_WAL008_ADDRESS_8107"),
        (HardwareCanarySlot::AccountId, "CANARY_WAL008_ACCOUNT_8108"),
        (
            HardwareCanarySlot::TransportDetails,
            "CANARY_WAL008_TRANSPORT_8109",
        ),
    ];
    let canaries = HardwareCanaries::new(
        CANARY_INPUTS[0].1,
        CANARY_INPUTS[1].1,
        CANARY_INPUTS[2].1,
        CANARY_INPUTS[3].1,
        CANARY_INPUTS[4].1,
        CANARY_INPUTS[5].1,
        CANARY_INPUTS[6].1,
        CANARY_INPUTS[7].1,
        CANARY_INPUTS[8].1,
    )
    .unwrap();
    assert_eq!(canaries.values().len(), 9);
    assert!(canaries.values().iter().all(|value| !value.is_empty()));
    let mut distinct_canaries = canaries.values().to_vec();
    distinct_canaries.sort_unstable();
    distinct_canaries.dedup();
    assert_eq!(distinct_canaries.len(), CANARY_INPUTS.len());

    let mut harness = synthetic_harness("hardware-redaction");
    harness
        .install_observable_canaries_for_test(&canaries)
        .unwrap();
    for (slot, canary) in CANARY_INPUTS {
        assert_eq!(
            harness.observable_canary_value_for_test(slot),
            Some(canary),
            "sensitive slot {slot:?}"
        );
        assert_eq!(
            harness.observable_canary_touch_count_for_test(slot),
            0,
            "sensitive slot {slot:?}"
        );
    }
    let decision = harness
        .decide(&exact_fingerprint(), &exact_probe())
        .unwrap();
    harness.persist(&decision).unwrap();
    let error = harness.synthetic_failure_for_test();
    let success_and_error = format!(
        "decision={decision:?};error-debug={error:?};error-display={error};logs={:?};diagnostics={:?}",
        harness.captured_logs(),
        harness.diagnostics(),
    );
    let persisted = harness.persisted_bytes().unwrap();
    let panic =
        catch_unwind(AssertUnwindSafe(|| harness.panic_after_probe_for_test())).unwrap_err();
    let panic_text = if let Some(value) = panic.downcast_ref::<&str>() {
        (*value).to_owned()
    } else if let Some(value) = panic.downcast_ref::<String>() {
        value.clone()
    } else {
        panic!("hardware panic payload must be a bounded string")
    };

    for (slot, canary) in CANARY_INPUTS {
        assert_eq!(
            harness.observable_canary_value_for_test(slot),
            Some(canary),
            "sensitive slot {slot:?}"
        );
        assert!(
            harness.observable_canary_touch_count_for_test(slot) > 0,
            "sensitive slot {slot:?}"
        );
    }

    for canary in canaries.values() {
        assert!(!success_and_error.contains(canary));
        assert!(!bytes_contain(&persisted, canary.as_bytes()));
        assert!(!panic_text.contains(canary));
    }
    assert_eq!(error.code(), "INTERNAL");
    assert_eq!(panic_text, "INTERNAL");
    assert_eq!(
        harness.diagnostic_field_names(),
        ["operation", "code", "capability"]
    );
}

#[test]
fn production_inventory_has_no_transport_parser_signing_or_fallback_authority() {
    let manifest = include_str!("../Cargo.toml");
    let hardware_source = include_str!("../src/zec/hardware.rs");

    for forbidden_dependency in [
        "hidapi",
        "rusb",
        "serialport",
        "qrcode",
        "reqwest",
        "ureq",
        "tokio",
        "tonic",
        "hyper",
    ] {
        assert!(!manifest.contains(forbidden_dependency));
    }
    for forbidden_source in [
        "std::net",
        "TcpStream",
        "UdpSocket",
        "std::env",
        "env::var",
        "File::open",
        "read_to_string",
        "serde_json",
        "use pczt",
        "pczt::Pczt",
        "fn sign(",
        "fn sign_",
        "fn prove",
        "fn finalize",
        "fn extract",
        "fn broadcast",
        "software_fallback",
        "fallback_to_software",
    ] {
        assert!(
            !hardware_source.contains(forbidden_source),
            "forbidden production authority {forbidden_source:?}"
        );
    }
    for synthetic_pin in [SYNTHETIC_MODEL, SYNTHETIC_APP, SYNTHETIC_APP_VERSION] {
        assert!(!hardware_source.contains(synthetic_pin));
    }
}
