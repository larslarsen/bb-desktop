use std::collections::BTreeSet;
use std::sync::{Arc, Barrier};
use std::thread;

use bitbook_wallet_broker::xmr::model::{NodeState, WalletState};
use bitbook_wallet_broker::xmr::receiver::{MAX_ISSUANCE_SEQUENCE, MAX_SUBADDRESS_INDEX};
use bitbook_wallet_broker::xmr::test_support::{
    BalanceFault, HardForkInfoFixture, ReceiverFault, ReceiverRig, RpcAddress, ViewRig, XmrNetwork,
};

const ACCOUNT: &str = "00112233445566778899aabbccddeeff";
const REQUEST: &str = "102132435465768798a9bacbdcedfe0f";
const OTHER_REQUEST: &str = "ffeeddccbbaa99887766554433221100";
const PRIMARY: &str = concat!(
    "4AAAAAAAAAAAAAAAAAAA",
    "AAAAAAAAAAAAAAAAAAAA",
    "AAAAAAAAAAAAAAAAAAAA",
    "AAAAAAAAAAAAAAAAAAAA",
    "AAAAAAAAAAAAAAA",
);
const SUBADDRESS_ONE: &str = concat!(
    "8BBBBBBBBBBBBBBBBBBB",
    "BBBBBBBBBBBBBBBBBBBB",
    "BBBBBBBBBBBBBBBBBBBB",
    "BBBBBBBBBBBBBBBBBBBB",
    "BBBBBBBBBBBBBBB",
);
const SUBADDRESS_TWO: &str = concat!(
    "8CCCCCCCCCCCCCCCCCCC",
    "CCCCCCCCCCCCCCCCCCCC",
    "CCCCCCCCCCCCCCCCCCCC",
    "CCCCCCCCCCCCCCCCCCCC",
    "CCCCCCCCCCCCCCC",
);

fn receiver(label: &str) -> ReceiverRig {
    assert_eq!(PRIMARY.len(), 95);
    assert_eq!(SUBADDRESS_ONE.len(), 95);
    assert_eq!(SUBADDRESS_TWO.len(), 95);
    assert!(!ReceiverRig::is_network_valid_address_for_test(PRIMARY));
    assert!(!ReceiverRig::is_network_valid_address_for_test(
        SUBADDRESS_ONE
    ));
    assert!(!ReceiverRig::is_network_valid_address_for_test(
        SUBADDRESS_TWO
    ));
    ReceiverRig::ready(label, ACCOUNT, XmrNetwork::Stagenet, PRIMARY)
}

#[test]
fn node_wallet_and_device_states_are_independent() {
    for (node_state, wallet_state, wallet_height, node_height, expected) in [
        (
            NodeState::Unavailable,
            WalletState::Locked,
            None,
            None,
            ("NODE_UNAVAILABLE", "LOCKED"),
        ),
        (
            NodeState::Ready,
            WalletState::Unavailable,
            None,
            Some(100),
            ("READY", "UNAVAILABLE"),
        ),
        (
            NodeState::Syncing,
            WalletState::Refreshing,
            Some(99),
            Some(100),
            ("NODE_SYNCING", "WALLET_REFRESHING"),
        ),
        (
            NodeState::Syncing,
            WalletState::Ready,
            Some(100),
            Some(100),
            ("NODE_SYNCING", "READY"),
        ),
        (
            NodeState::Ready,
            WalletState::Locked,
            None,
            Some(100),
            ("READY", "LOCKED"),
        ),
        (
            NodeState::Ready,
            WalletState::Ready,
            Some(100),
            Some(100),
            ("READY", "READY"),
        ),
    ] {
        let view = ViewRig::with_explicit_states(
            ACCOUNT,
            XmrNetwork::Stagenet,
            node_state,
            wallet_state,
            node_height,
            wallet_height,
        )
        .snapshot()
        .unwrap();
        assert_eq!(view.node_state, expected.0);
        assert_eq!(view.wallet_state, expected.1);
        assert_eq!(view.device_state, "NOT_APPLICABLE");
        assert_eq!(
            view.node_height,
            node_height.map(|height| height.to_string())
        );
        assert_eq!(
            view.wallet_height,
            wallet_height.map(|height| height.to_string())
        );
    }
}

#[test]
fn exact_u64_balances_are_canonical_and_unlocked_never_exceeds_total() {
    for (total, unlocked) in [(0, 0), (1, 0), (1, 1), (u64::MAX, u64::MAX)] {
        let mut view = ViewRig::ready(ACCOUNT, XmrNetwork::Testnet);
        view.script_balances(total, unlocked);
        let snapshot = view.snapshot().unwrap();
        assert_eq!(snapshot.balance_atomic, total.to_string());
        assert_eq!(snapshot.unlocked_balance_atomic, unlocked.to_string());
    }
    for fault in [
        BalanceFault::MissingTotal,
        BalanceFault::MissingUnlocked,
        BalanceFault::Stale,
        BalanceFault::Negative,
        BalanceFault::Floating,
        BalanceFault::Overflow,
        BalanceFault::LeadingZero,
        BalanceFault::UnlockedAboveTotal,
    ] {
        let mut view = ViewRig::ready(ACCOUNT, XmrNetwork::Testnet);
        view.arm_balance_fault(fault);
        assert_eq!(view.snapshot().unwrap_err().code(), "PROTOCOL_INCOMPATIBLE");
        assert!(view.returned_snapshot().is_none());
        assert!(!view.substituted_total_for_unlocked());
    }
}

#[test]
fn sanitized_xmr_view_has_exact_fields_and_all_spend_zec_hardware_caps_negative() {
    let view = ViewRig::ready(ACCOUNT, XmrNetwork::Stagenet)
        .snapshot()
        .unwrap();
    assert_eq!(
        view.field_names(),
        [
            "account_id",
            "asset",
            "network",
            "kind",
            "privacy",
            "node_state",
            "wallet_state",
            "device_state",
            "node_height",
            "wallet_height",
            "balance_atomic",
            "unlocked_balance_atomic",
            "capabilities",
        ]
    );
    assert_eq!(view.asset, "XMR");
    assert_eq!(view.kind, "software");
    assert_eq!(view.privacy, "private");
    assert_eq!(
        view.capabilities.field_names(),
        [
            "can_view",
            "can_derive_fresh_receiver",
            "can_receive_private",
            "can_receive_orchard",
            "can_receive_ironwood",
            "can_prepare_tx",
            "can_sign_spend",
            "can_sign_orchard",
            "can_sign_ironwood",
            "can_tx_v6",
            "can_migrate_orchard_to_ironwood",
            "can_sign_transparent",
            "can_display_amount_on_device",
            "can_display_recipient_on_device",
            "can_display_network_on_device",
            "can_verify_pczt_on_device",
            "can_export_viewing_material",
            "can_broadcast",
            "consensus_branch",
            "pczt_version",
            "tx_version_max",
        ]
    );
    assert!(view.capabilities.can_view);
    assert!(view.capabilities.can_derive_fresh_receiver);
    assert!(view.capabilities.can_receive_private);
    assert!(!view.capabilities.can_receive_orchard);
    assert!(!view.capabilities.can_receive_ironwood);
    assert!(!view.capabilities.can_prepare_tx);
    assert!(!view.capabilities.can_sign_spend);
    assert!(!view.capabilities.can_sign_orchard);
    assert!(!view.capabilities.can_sign_ironwood);
    assert!(!view.capabilities.can_tx_v6);
    assert!(!view.capabilities.can_migrate_orchard_to_ironwood);
    assert!(!view.capabilities.can_sign_transparent);
    assert!(!view.capabilities.can_display_amount_on_device);
    assert!(!view.capabilities.can_display_recipient_on_device);
    assert!(!view.capabilities.can_display_network_on_device);
    assert!(!view.capabilities.can_verify_pczt_on_device);
    assert!(!view.capabilities.can_export_viewing_material);
    assert!(!view.capabilities.can_broadcast);
    assert!(view.capabilities.pczt_version.is_none());
    assert!(view.capabilities.tx_version_max.is_none());
    let encoded = view.sanitized_json();
    for forbidden in [
        "primary_address",
        "receiver_history",
        "mnemonic",
        "view_key",
        "wallet_password",
        "rpc_login",
        "endpoint",
        "port",
        "path",
        "process_id",
        "raw_node",
        "upstream",
    ] {
        assert!(!encoded.contains(forbidden));
    }
}

#[test]
fn validated_hard_fork_info_sets_the_xmr_consensus_branch_capability() {
    let hard_fork = HardForkInfoFixture::valid_stagenet(16);
    let mut view = ViewRig::ready(ACCOUNT, XmrNetwork::Stagenet);
    view.script_hard_fork_info(hard_fork);
    let snapshot = view.snapshot().unwrap();
    assert!(view.hard_fork_info_was_validated());
    assert_eq!(
        snapshot.capabilities.consensus_branch.as_deref(),
        Some("16")
    );

    for mutation in [
        "status",
        "enabled",
        "version-zero",
        "version-overflow",
        "wrong-network",
    ] {
        let mut view = ViewRig::ready(ACCOUNT, XmrNetwork::Stagenet);
        view.script_hard_fork_info(HardForkInfoFixture::mutated_stagenet(16, mutation));
        assert_eq!(
            view.snapshot().unwrap_err().code(),
            "PROTOCOL_INCOMPATIBLE",
            "mutation {mutation}"
        );
        assert!(view.returned_snapshot().is_none());
    }
}

#[test]
fn fresh_receiver_input_lengths_and_closed_network_values_are_exact() {
    for (length, accepted) in [(31, false), (32, true), (33, false)] {
        let value = "a".repeat(length);
        assert_eq!(
            ReceiverRig::validate_account_id_for_test(&value).is_ok(),
            accepted,
            "account length {length}"
        );
        assert_eq!(
            ReceiverRig::validate_request_id_for_test(&value).is_ok(),
            accepted,
            "request length {length}"
        );
    }
    for invalid in [
        "00112233445566778899AABBCCDDEEFF",
        "00112233445566778899aabbccddeefg",
        "../112233445566778899aabbccddeeff",
    ] {
        assert_eq!(
            ReceiverRig::validate_account_id_for_test(invalid)
                .unwrap_err()
                .code(),
            "SCHEMA"
        );
        assert_eq!(
            ReceiverRig::validate_request_id_for_test(invalid)
                .unwrap_err()
                .code(),
            "SCHEMA"
        );
    }
    for (network, expected) in [
        ("xmr-stagenet", Ok(())),
        ("xmr-testnet", Ok(())),
        ("xmr-mainnet", Err("NETWORK_DISABLED")),
        ("stagenet", Err("SCHEMA")),
        ("", Err("SCHEMA")),
    ] {
        let result = ReceiverRig::validate_network_for_test(network);
        match expected {
            Ok(()) => assert!(result.is_ok()),
            Err(code) => assert_eq!(result.unwrap_err().code(), code),
        }
    }
}

#[test]
fn exact_replay_returns_durable_binding_without_any_rpc_call() {
    let mut wallet = receiver("receiver-replay");
    wallet.script_address(RpcAddress::valid(1, SUBADDRESS_ONE));
    let first = wallet.fresh(ACCOUNT, "xmr-stagenet", REQUEST).unwrap();
    assert_eq!(
        wallet.rpc_calls(),
        ["create_address:0:", "validate_address", "get_address:0:1"]
    );
    wallet.clear_rpc_calls();
    let replay = wallet.fresh(ACCOUNT, "xmr-stagenet", REQUEST).unwrap();
    assert_eq!(replay, first);
    assert!(wallet.rpc_calls().is_empty());
    assert!(wallet.last_lookup_was_durable());
}

#[test]
fn new_receiver_is_nonprimary_validated_equal_and_committed_before_return() {
    let mut wallet = receiver("receiver-validation");
    wallet.script_address(RpcAddress::valid(1, SUBADDRESS_ONE));
    let issued = wallet.fresh(ACCOUNT, "xmr-stagenet", REQUEST).unwrap();
    assert_eq!(issued.account_id, ACCOUNT);
    assert_eq!(issued.network, "xmr-stagenet");
    assert_eq!(issued.request_id, REQUEST);
    assert_eq!(issued.receiver, SUBADDRESS_ONE);
    assert_eq!(issued.account_index, 0);
    assert_eq!(issued.subaddress_index, 1);
    assert_eq!(issued.issued_at_sequence, 1);
    assert_ne!(issued.receiver, PRIMARY);
    assert!(wallet.validate_reported_subaddress());
    assert!(wallet.get_address_equal());
    assert!(wallet.binding_committed_before_return());
    assert!(wallet.binding_file_synced_before_return());
}

#[test]
fn rpc_mismatch_primary_zero_index_wrong_network_or_non_subaddress_returns_nothing() {
    for fault in [
        ReceiverFault::PrimaryAddress,
        ReceiverFault::ZeroIndex,
        ReceiverFault::WrongNetwork,
        ReceiverFault::ValidateSaysPrimary,
        ReceiverFault::ValidateSaysInvalid,
        ReceiverFault::GetAddressMismatch,
        ReceiverFault::WrongAccountIndex,
    ] {
        let mut wallet = receiver("receiver-rpc-mismatch");
        wallet.arm_fault(fault);
        assert_eq!(
            wallet
                .fresh(ACCOUNT, "xmr-stagenet", REQUEST)
                .unwrap_err()
                .code(),
            "PROTOCOL_INCOMPATIBLE"
        );
        assert!(wallet.returned_receiver().is_none(), "fault {fault:?}");
        assert!(wallet.persisted_bindings().is_empty(), "fault {fault:?}");
        assert!(!wallet.used_primary_or_stale_fallback(), "fault {fault:?}");
    }
}

#[test]
fn concurrent_distinct_requests_serialize_with_unique_increasing_indices_and_sequences() {
    let wallet = Arc::new(receiver("receiver-concurrent"));
    wallet.script_concurrent_addresses([
        RpcAddress::valid(1, SUBADDRESS_ONE),
        RpcAddress::valid(2, SUBADDRESS_TWO),
    ]);
    let barrier = Arc::new(Barrier::new(3));
    let mut joins = Vec::new();
    for request_id in [REQUEST, OTHER_REQUEST] {
        let wallet = Arc::clone(&wallet);
        let barrier = Arc::clone(&barrier);
        joins.push(thread::spawn(move || {
            barrier.wait();
            wallet
                .fresh_concurrent(ACCOUNT, "xmr-stagenet", request_id)
                .unwrap()
        }));
    }
    barrier.wait();
    let issued: Vec<_> = joins.into_iter().map(|join| join.join().unwrap()).collect();
    assert_eq!(
        issued
            .iter()
            .map(|item| item.subaddress_index)
            .collect::<BTreeSet<_>>(),
        BTreeSet::from([1, 2])
    );
    assert_eq!(
        issued
            .iter()
            .map(|item| item.issued_at_sequence)
            .collect::<BTreeSet<_>>(),
        BTreeSet::from([1, 2])
    );
    assert_eq!(
        issued
            .iter()
            .map(|item| item.receiver.as_str())
            .collect::<BTreeSet<_>>()
            .len(),
        2
    );
    assert_eq!(wallet.maximum_concurrent_create_address_calls(), 1);
}

#[test]
fn reopen_preserves_every_returned_request_index_address_and_sequence_binding() {
    let mut wallet = receiver("receiver-reopen");
    wallet.script_address(RpcAddress::valid(1, SUBADDRESS_ONE));
    let first = wallet.fresh(ACCOUNT, "xmr-stagenet", REQUEST).unwrap();
    let root = wallet.close().unwrap();
    let mut reopened = ReceiverRig::open(root, ACCOUNT, XmrNetwork::Stagenet).unwrap();
    let replay = reopened.fresh(ACCOUNT, "xmr-stagenet", REQUEST).unwrap();
    assert_eq!(replay, first);
    assert!(reopened.rpc_calls().is_empty());
    assert_eq!(reopened.greatest_issuance_sequence(), 1);
}

#[test]
fn sqlite_identity_columns_and_independent_uniqueness_constraints_are_exact() {
    let wallet = receiver("receiver-schema");
    let schema = wallet.inspect_schema().unwrap();
    assert_eq!(schema.schema_version, 1);
    assert_eq!(schema.synchronous, "FULL");
    assert_eq!(schema.file_mode, 0o600);
    assert_eq!(
        schema.account_columns,
        [
            "schema_version",
            "account_id",
            "network",
            "primary_address",
            "greatest_issuance_sequence",
        ]
    );
    assert_eq!(
        schema.receiver_columns,
        [
            "request_id",
            "account_index",
            "subaddress_index",
            "subaddress",
            "issued_at_sequence",
        ]
    );
    assert_eq!(schema.account_id, ACCOUNT);
    assert_eq!(schema.network, "xmr-stagenet");
    assert_eq!(schema.primary_address, PRIMARY);
    assert_eq!(
        schema.independent_unique_constraints,
        vec![
            vec!["request_id"],
            vec!["account_index", "subaddress_index"],
            vec!["subaddress"],
            vec!["issued_at_sequence"],
        ]
    );
    assert_eq!(schema.account_index_check, "account_index = 0");
    assert_eq!(schema.subaddress_index_check, "subaddress_index > 0");
    assert_eq!(
        schema.issuance_sequence_check,
        "issued_at_sequence > 0 AND issued_at_sequence <= 9223372036854775807"
    );
}

#[test]
fn subaddress_and_sequence_limits_cover_immediately_below_at_and_above_without_wrap() {
    assert_eq!(MAX_SUBADDRESS_INDEX, u32::MAX);
    assert_eq!(MAX_ISSUANCE_SEQUENCE, i64::MAX);
    let max_index = u64::from(MAX_SUBADDRESS_INDEX);
    let max_sequence = u64::try_from(MAX_ISSUANCE_SEQUENCE).unwrap();
    for (index, sequence, accepted) in [
        (max_index - 1, max_sequence - 1, true),
        (max_index, max_sequence, false),
        (max_index + 1, max_sequence, false),
        (max_index, max_sequence + 1, false),
    ] {
        let mut wallet = receiver("receiver-limits");
        let configured = wallet.set_receiver_state_for_test(index, sequence);
        if index > max_index || sequence > max_sequence {
            assert_eq!(configured.unwrap_err().code(), "LIMIT");
            assert!(wallet.persisted_bindings().is_empty());
        } else {
            configured.unwrap();
            let result = wallet.fresh(ACCOUNT, "xmr-stagenet", REQUEST);
            assert_eq!(result.is_ok(), accepted);
            if !accepted {
                assert_eq!(result.unwrap_err().code(), "LIMIT");
                assert!(wallet.returned_receiver().is_none());
            }
        }
    }
}

#[test]
fn post_rpc_persistence_failure_consumes_gap_but_never_returns_or_reuses_address() {
    for fault in [
        ReceiverFault::ReceiverRowWrite,
        ReceiverFault::SequenceWrite,
        ReceiverFault::FileSync,
        ReceiverFault::Commit,
    ] {
        let mut wallet = receiver("receiver-consumed-gap");
        wallet.script_addresses([
            RpcAddress::valid(1, SUBADDRESS_ONE),
            RpcAddress::valid(2, SUBADDRESS_TWO),
        ]);
        wallet.arm_fault(fault);
        assert_eq!(
            wallet
                .fresh(ACCOUNT, "xmr-stagenet", REQUEST)
                .unwrap_err()
                .code(),
            "STATE_CORRUPT"
        );
        assert!(wallet.returned_receiver().is_none());
        assert!(wallet.persisted_bindings().is_empty());
        wallet.clear_faults();
        let next = wallet
            .fresh(ACCOUNT, "xmr-stagenet", OTHER_REQUEST)
            .unwrap();
        assert_eq!(next.subaddress_index, 2);
        assert_ne!(next.receiver, SUBADDRESS_ONE);
        assert!(!wallet.address_was_reused(SUBADDRESS_ONE));
    }
}

#[test]
fn schema_uniqueness_corruption_rollback_and_sync_failure_are_state_corrupt_not_reconstruction() {
    for fault in [
        ReceiverFault::SchemaDrift,
        ReceiverFault::DuplicateRequest,
        ReceiverFault::DuplicateIndex,
        ReceiverFault::DuplicateAddress,
        ReceiverFault::DuplicateSequence,
        ReceiverFault::Rollback,
        ReceiverFault::CorruptDatabase,
        ReceiverFault::WrongIdentity,
        ReceiverFault::SynchronousNotFull,
    ] {
        let mut wallet = receiver("receiver-state-corrupt");
        wallet.arm_fault(fault);
        assert_eq!(
            wallet.reopen_for_test().unwrap_err().code(),
            "STATE_CORRUPT"
        );
        assert!(!wallet.reconstructed_from_wallet_output());
        assert!(wallet.returned_receiver().is_none());
        assert_eq!(wallet.rpc_call_count(), 0);
    }
}

#[test]
fn invalid_locked_syncing_wrong_network_and_watch_initialization_have_no_fallback() {
    for (fault, expected_code) in [
        (ReceiverFault::Locked, "LOCKED"),
        (ReceiverFault::NodeSyncing, "NODE_SYNCING"),
        (ReceiverFault::WalletRefreshing, "UNAVAILABLE"),
        (ReceiverFault::WrongNetworkRequest, "WRONG_NETWORK"),
        (ReceiverFault::WatchOnlyInitialization, "WATCH_ONLY"),
        (ReceiverFault::Exhausted, "LIMIT"),
    ] {
        let mut wallet = receiver("receiver-ineligible");
        wallet.arm_fault(fault);
        let error = wallet.fresh(ACCOUNT, "xmr-stagenet", REQUEST).unwrap_err();
        assert_eq!(error.code(), expected_code, "fault {fault:?}");
        assert!(wallet.returned_receiver().is_none());
        assert!(!wallet.used_primary_or_stale_fallback());
        assert!(wallet.persisted_bindings().is_empty());
    }
}
