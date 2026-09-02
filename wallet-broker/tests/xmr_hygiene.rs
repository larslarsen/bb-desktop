use std::collections::BTreeSet;

use bitbook_wallet_broker::xmr::test_support::{
    AuthorityRig, HygieneExit, HygieneRig, ObservableCanary, ObservableSecretClass, RpcMethod,
    XmrNetwork,
};

const ACCOUNT: &str = "00112233445566778899aabbccddeeff";

#[test]
fn phase_bound_typed_authority_has_no_raw_generic_spend_or_daemon_switch_route() {
    let authority = AuthorityRig::xmr_phase_seven();
    assert_eq!(
        authority.public_operations(),
        [
            "xmr.installation.select-native",
            "xmr.account.create-native",
            "xmr.account.import-watch-only-native",
            "xmr.account.open",
            "xmr.account.lock",
            "xmr.account.view",
            "xmr.receiver.fresh",
        ]
    );
    for forbidden in [
        "raw",
        "generic",
        "transfer",
        "sweep",
        "describe",
        "sign",
        "submit",
        "relay",
        "proof",
        "key_image",
        "multisig",
        "address_book",
        "mining",
        "set_daemon",
        "spend_key",
        "view_key",
        "broadcast",
        "mainnet",
        "remote",
        "endpoint",
        "url",
    ] {
        assert!(
            !authority
                .public_operations()
                .iter()
                .any(|operation| operation.contains(forbidden))
        );
        assert_eq!(
            authority.invoke_for_test(forbidden).unwrap_err().code(),
            "SCHEMA"
        );
        assert_eq!(authority.side_effect_count(), 0);
    }
}

#[test]
fn fake_endpoint_records_exact_typed_calls_and_panics_immediately_on_every_unlisted_method() {
    let mut authority = AuthorityRig::xmr_phase_seven();
    authority.call(RpcMethod::GetVersion).unwrap();
    authority.call(RpcMethod::GetInfo).unwrap();
    assert_eq!(
        authority.typed_calls(),
        ["wallet:get_version", "node:get_info"]
    );
    assert_eq!(authority.raw_request_count(), 0);

    for method in [
        "transfer",
        "transfer_split",
        "sweep_all",
        "sign_transfer",
        "submit_transfer",
        "relay_tx",
        "export_key_images",
        "set_daemon",
        "json_rpc",
    ] {
        let failure = authority.invoke_fake_unlisted_and_capture(method);
        assert!(failure.failed_immediately);
        assert_eq!(failure.method, method);
        assert_eq!(failure.bytes_read_after_method, 0);
        assert_eq!(failure.state_transitions_after_method, 0);
        assert_eq!(failure.returned_bytes, 0);
    }
}

#[test]
fn query_key_is_mnemonic_only_during_fresh_software_creation() {
    let mut authority = AuthorityRig::xmr_phase_seven();
    authority
        .begin_fresh_software_creation(ACCOUNT, XmrNetwork::Stagenet)
        .unwrap();
    authority.query_mnemonic_once().unwrap();
    for key_type in ["spend_key", "view_key", "mnemonic"] {
        assert_eq!(
            authority.query_key_for_test(key_type).unwrap_err().code(),
            "SCHEMA"
        );
        assert_eq!(authority.side_effect_count_since_last_observation(), 0);
    }
    authority.finish_fresh_software_creation().unwrap();
    assert_eq!(
        authority.query_mnemonic_once().unwrap_err().code(),
        "SCHEMA"
    );
    assert_eq!(authority.mnemonic_query_count(), 1);
}

#[test]
fn mainnet_is_rejected_before_node_path_database_vault_socket_or_process_side_effect() {
    let mut authority = AuthorityRig::from_product_input(ACCOUNT, "xmr-mainnet");
    assert_eq!(authority.open().unwrap_err().code(), "NETWORK_DISABLED");
    assert!(authority.operations().is_empty());
    assert_eq!(authority.node_request_count(), 0);
    assert_eq!(authority.wallet_request_count(), 0);
    assert_eq!(authority.child_count(), 0);
    assert_eq!(authority.open_handle_count(), 0);
    assert!(authority.returned_values().is_empty());
}

#[test]
fn every_prohibited_secret_class_is_committed_then_absent_from_all_observables() {
    let canaries = [
        ObservableCanary::new(
            ObservableSecretClass::SelectedPath,
            "CANARY_WAL007_PATH_8301",
        ),
        ObservableCanary::new(
            ObservableSecretClass::ArgvConfig,
            "CANARY_WAL007_ARGV_CONFIG_8302",
        ),
        ObservableCanary::new(
            ObservableSecretClass::EndpointPort,
            "CANARY_WAL007_ENDPOINT_PORT_8303",
        ),
        ObservableCanary::new(
            ObservableSecretClass::RpcRealmNonce,
            "CANARY_WAL007_REALM_NONCE_8304",
        ),
        ObservableCanary::new(
            ObservableSecretClass::RpcLogin,
            "CANARY_WAL007_RPC_LOGIN_8305",
        ),
        ObservableCanary::new(
            ObservableSecretClass::Authorization,
            "CANARY_WAL007_AUTHORIZATION_8306",
        ),
        ObservableCanary::new(
            ObservableSecretClass::WalletPassword,
            "CANARY_WAL007_WALLET_PASSWORD_8307",
        ),
        ObservableCanary::new(
            ObservableSecretClass::Mnemonic,
            "CANARY_WAL007_MNEMONIC_8308",
        ),
        ObservableCanary::new(
            ObservableSecretClass::ViewKey,
            "CANARY_WAL007_VIEW_KEY_8309",
        ),
        ObservableCanary::new(
            ObservableSecretClass::SpendKey,
            "CANARY_WAL007_SPEND_KEY_830a",
        ),
        ObservableCanary::new(
            ObservableSecretClass::PrimaryAddress,
            "CANARY_WAL007_PRIMARY_830b",
        ),
        ObservableCanary::new(
            ObservableSecretClass::FreshReceiver,
            "CANARY_WAL007_RECEIVER_830c",
        ),
        ObservableCanary::new(
            ObservableSecretClass::RequestId,
            "CANARY_WAL007_REQUEST_830d",
        ),
        ObservableCanary::new(
            ObservableSecretClass::RawHttpJson,
            "CANARY_WAL007_RAW_HTTP_JSON_830e",
        ),
        ObservableCanary::new(
            ObservableSecretClass::NodeResponse,
            "CANARY_WAL007_NODE_RESPONSE_830f",
        ),
        ObservableCanary::new(
            ObservableSecretClass::WalletFile,
            "CANARY_WAL007_WALLET_FILE_8310",
        ),
        ObservableCanary::new(
            ObservableSecretClass::SqliteRow,
            "CANARY_WAL007_SQLITE_ROW_8311",
        ),
        ObservableCanary::new(
            ObservableSecretClass::UpstreamError,
            "CANARY_WAL007_UPSTREAM_ERROR_8312",
        ),
    ];
    let mut hygiene = HygieneRig::new(ACCOUNT, XmrNetwork::Stagenet);
    let receipt = hygiene.install_canaries(&canaries).unwrap();
    assert_eq!(receipt.commitments().len(), canaries.len());
    let mut hashes = BTreeSet::new();
    for (commitment, canary) in receipt.commitments().iter().zip(&canaries) {
        assert_eq!(commitment.class, canary.class().as_str());
        assert_eq!(commitment.byte_length, canary.value().len());
        assert!(hashes.insert(commitment.sha256.as_str()));
    }
    hygiene.exercise_success_and_failure().unwrap();
    let observable = format!(
        "rig={hygiene:?};error={:?};chain={:?};logs={:?};diagnostics={:?};panic={:?};teardown={:?}",
        hygiene.last_error(),
        hygiene.error_chain(),
        hygiene.logs(),
        hygiene.diagnostics(),
        hygiene.panic_output(),
        hygiene.teardown_output(),
    );
    for canary in &canaries {
        assert!(
            !observable.contains(canary.value()),
            "class {:?}",
            canary.class()
        );
    }
    assert_eq!(
        hygiene.public_diagnostic_fields(),
        ["operation", "account_id", "asset", "network", "code"]
    );
}

#[test]
fn credentials_and_secrets_wipe_on_success_error_cancel_replacement_unwind_and_drop() {
    for exit in [
        HygieneExit::Success,
        HygieneExit::Error,
        HygieneExit::Cancellation,
        HygieneExit::Replacement,
        HygieneExit::PanicUnwind,
        HygieneExit::Drop,
    ] {
        let mut hygiene = HygieneRig::new(ACCOUNT, XmrNetwork::Testnet);
        hygiene.exercise_exit(exit);
        for label in [
            "rpc-username",
            "rpc-password",
            "digest-ha1",
            "digest-cnonce",
            "digest-challenge",
            "digest-authorization",
            "wallet-password",
            "mnemonic",
            "private-view-key",
            "native-import",
        ] {
            assert!(hygiene.wipe_observed(label, exit), "{label} {exit:?}");
        }
        assert_eq!(hygiene.open_handle_count(), 0);
        assert_eq!(hygiene.child_count(), 0);
    }
}

#[test]
fn every_teardown_cause_reaps_child_process_group_closes_handles_and_removes_secrets() {
    for cause in [
        "lock",
        "broker-exit",
        "executable-missing",
        "executable-changed",
        "authentication-failure",
        "malformed-rpc",
        "unexpected-child-exit",
        "panic-unwind",
    ] {
        let mut hygiene = HygieneRig::new(ACCOUNT, XmrNetwork::Stagenet);
        hygiene.start_child().unwrap();
        hygiene.teardown_for_test(cause);
        assert_eq!(hygiene.child_count(), 0, "cause {cause}");
        assert_eq!(hygiene.open_handle_count(), 0, "cause {cause}");
        assert!(hygiene.process_group_reaped(), "cause {cause}");
        assert!(hygiene.runtime_secrets_removed(), "cause {cause}");
        assert!(hygiene.credentials_wiped(), "cause {cause}");
        assert!(!hygiene.touched_nonowned_process(), "cause {cause}");
    }
}

#[test]
fn diagnostics_are_closed_stable_fields_without_free_text_or_upstream_values() {
    let hygiene = HygieneRig::new(ACCOUNT, XmrNetwork::Testnet);
    assert_eq!(
        hygiene.public_diagnostic_fields(),
        ["operation", "account_id", "asset", "network", "code"]
    );
    for forbidden in [
        "message",
        "detail",
        "cause",
        "upstream",
        "path",
        "endpoint",
        "port",
        "request_id",
        "receiver",
        "raw",
    ] {
        assert_eq!(
            hygiene
                .encode_diagnostic_field_for_test(forbidden, "CANARY_WAL007_FREE_TEXT")
                .unwrap_err()
                .code(),
            "SCHEMA"
        );
    }
}

#[test]
fn xmr_failure_does_not_mutate_zec_social_electron_or_quote_state() {
    let mut hygiene = HygieneRig::with_isolation_snapshots(ACCOUNT, XmrNetwork::Stagenet);
    let before = hygiene.non_xmr_snapshot();
    hygiene.fail_xmr_child_for_test();
    assert_eq!(hygiene.non_xmr_snapshot(), before);
    assert_eq!(hygiene.zec_call_count(), 0);
    assert_eq!(hygiene.social_call_count(), 0);
    assert_eq!(hygiene.electron_call_count(), 0);
    assert_eq!(hygiene.quote_worker_call_count(), 0);
}
