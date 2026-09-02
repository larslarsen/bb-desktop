use bitbook_wallet_broker::xmr::rpc::{
    CONNECT_TIMEOUT_SECS, MAX_HTTP_BYTES, MAX_JSON_NESTING, MAX_REQUEST_BODY_BYTES,
    READ_TIMEOUT_SECS, READINESS_TIMEOUT_SECS, WALLET_RPC_VERSION, WRITE_TIMEOUT_SECS,
};
use bitbook_wallet_broker::xmr::test_support::{
    DigestVector, HttpFault, JsonFault, NodeInfo, NodeProbeRig, RpcFault, RpcMethod,
    RpcTransportRig, XmrNetwork,
};

#[test]
fn rfc_digest_vector_and_recorded_monero_challenge_match_independent_oracles() {
    assert_eq!(WALLET_RPC_VERSION, (1 << 16) | 31);
    let rfc = DigestVector {
        username: "Mufasa",
        password: "Circle Of Life",
        realm: "testrealm@host.com",
        nonce: "dcd98b7102dd2f0e8b11d0f600bfb0c093",
        uri: "/dir/index.html",
        method: "GET",
        qop: "auth",
        nc: "00000001",
        cnonce: "0a4f113b",
        algorithm: Some("MD5"),
        opaque: Some("5ccc069c403ebaf9f0171e9517f40e41"),
    };
    assert_eq!(
        RpcTransportRig::digest_response_for_test(&rfc).unwrap(),
        "6629fae49393a05397450978507c4ef1"
    );

    let mut rpc = RpcTransportRig::wallet();
    rpc.script_challenge(concat!(
        r#"Digest realm="monero-rpc", nonce="synthetic-nonce", "#,
        r#"qop="auth", algorithm=MD5, opaque="synthetic-opaque""#,
    ));
    rpc.call(RpcMethod::GetVersion).unwrap();
    let authorization = rpc.last_authorization().unwrap();
    assert_eq!(authorization.scheme, "Digest");
    assert_eq!(authorization.method, "POST");
    assert_eq!(authorization.uri, "/json_rpc");
    assert_eq!(authorization.qop, "auth");
    assert_eq!(authorization.nc, "00000001");
    assert_eq!(authorization.algorithm, "MD5");
    assert_eq!(authorization.opaque.as_deref(), Some("synthetic-opaque"));
}

#[test]
fn digest_challenge_without_algorithm_is_accepted_as_md5() {
    let mut rpc = RpcTransportRig::wallet();
    rpc.script_challenge(
        r#"Digest realm="monero-rpc", nonce="synthetic-no-algorithm", qop="auth""#,
    );

    rpc.call(RpcMethod::GetVersion).unwrap();

    let authorization = rpc.last_authorization().unwrap();
    assert_eq!(authorization.scheme, "Digest");
    assert_eq!(authorization.method, "POST");
    assert_eq!(authorization.uri, "/json_rpc");
    assert_eq!(authorization.qop, "auth");
    assert_eq!(authorization.nc, "00000001");
    assert_eq!(authorization.algorithm, "MD5");
    assert!(authorization.opaque.is_none());
    assert_eq!(rpc.request_count(), 2);
}

#[test]
fn wallet_digest_uses_fresh_sixteen_byte_cnonce_and_one_retry_only() {
    let mut rpc = RpcTransportRig::wallet();
    rpc.call(RpcMethod::GetVersion).unwrap();
    let first = rpc.last_authorization().unwrap();
    assert_eq!(first.cnonce.len(), 32);
    assert!(
        first
            .cnonce
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
    );
    assert_eq!(first.cnonce_source_bytes, 16);
    assert_eq!(rpc.request_count(), 2);

    rpc.call(RpcMethod::GetHeight).unwrap();
    let second = rpc.last_authorization().unwrap();
    assert_ne!(first.cnonce, second.cnonce);
    assert_eq!(second.nc, "00000001");
    assert_eq!(rpc.request_count(), 4);

    rpc.arm_fault(RpcFault::SecondChallenge);
    assert_eq!(
        rpc.call(RpcMethod::GetBalance).unwrap_err().code(),
        "UNAUTH"
    );
    assert_eq!(rpc.last_call_request_count(), 2);
}

#[test]
fn malformed_duplicate_or_unsupported_digest_challenges_fail_closed() {
    for challenge in [
        "Basic realm=\"monero-rpc\"",
        "Digest nonce=\"n\", qop=\"auth\"",
        "Digest realm=\"r\", qop=\"auth\"",
        "Digest realm=\"r\", nonce=\"n\"",
        "Digest realm=\"r\", nonce=\"n\", qop=\"auth-int\"",
        "Digest realm=\"r\", nonce=\"n\", qop=\"auth,auth-int\"",
        "Digest realm=\"r\", nonce=\"n\", qop=\"auth, auth-int\"",
        "Digest realm=\"r\", nonce=\"n\", qop=\"auth\", algorithm=SHA-256",
        "Digest realm=\"r\", nonce=\"n\", qop=\"auth\", charset=UTF-8",
        "Digest realm=\"r\", nonce=\"n\", qop=\"auth\", userhash=true",
        "Digest realm=\"r\", nonce=\"n\", qop=\"auth\", synthetic=\"unknown\"",
        "Digest realm=\"r\", realm=\"r2\", nonce=\"n\", qop=\"auth\"",
        "Digest realm=\"r\", nonce=\"n\", nonce=\"n2\", qop=\"auth\"",
        "Digest realm=\"r\", nonce=\"n\", qop=\"auth\", stale=true",
        "Digest realm=r, nonce=\"n\", qop=\"auth\"",
        "Digest realm=\"unterminated, nonce=\"n\", qop=\"auth\"",
    ] {
        let mut rpc = RpcTransportRig::wallet();
        rpc.script_challenge(challenge);
        assert_eq!(
            rpc.call(RpcMethod::GetVersion).unwrap_err().code(),
            "UNAUTH"
        );
        assert!(rpc.returned_bytes().is_empty());
        assert_eq!(rpc.open_connection_count(), 0);
    }
}

#[test]
fn request_body_and_total_response_boundaries_are_exact() {
    assert_eq!(MAX_REQUEST_BODY_BYTES, 16 * 1_024);
    assert_eq!(MAX_HTTP_BYTES, 64 * 1_024);
    for (length, accepted) in [
        (MAX_REQUEST_BODY_BYTES - 1, true),
        (MAX_REQUEST_BODY_BYTES, true),
        (MAX_REQUEST_BODY_BYTES + 1, false),
    ] {
        let mut rpc = RpcTransportRig::wallet();
        let result = rpc.send_body_for_test(&vec![b'x'; length]);
        assert_eq!(result.is_ok(), accepted, "request length {length}");
        if !accepted {
            assert_eq!(result.unwrap_err().code(), "LIMIT");
            assert_eq!(rpc.bytes_written(), 0);
        }
    }
    for (length, accepted) in [
        (MAX_HTTP_BYTES - 1, true),
        (MAX_HTTP_BYTES, true),
        (MAX_HTTP_BYTES + 1, false),
    ] {
        let mut rpc = RpcTransportRig::wallet();
        rpc.script_response_total_bytes(length);
        let result = rpc.call(RpcMethod::GetVersion);
        assert_eq!(result.is_ok(), accepted, "response length {length}");
        if !accepted {
            assert_eq!(result.unwrap_err().code(), "LIMIT");
            assert!(rpc.returned_bytes().is_empty());
        }
    }
}

#[test]
fn http_framing_rejects_every_ambiguous_or_connection_widening_shape() {
    for fault in [
        HttpFault::MissingContentLength,
        HttpFault::DuplicateContentLength,
        HttpFault::ConflictingContentLength,
        HttpFault::TransferEncodingChunked,
        HttpFault::TransferEncodingIdentity,
        HttpFault::MissingConnectionClose,
        HttpFault::FoldedHeader,
        HttpFault::ControlByte,
        HttpFault::TrailingBytes,
        HttpFault::Redirect301,
        HttpFault::Redirect307,
        HttpFault::StatusUnknown,
        HttpFault::Http10,
    ] {
        let mut rpc = RpcTransportRig::wallet();
        rpc.arm_http_fault(fault);
        assert_eq!(
            rpc.call(RpcMethod::GetVersion).unwrap_err().code(),
            "PROTOCOL_INCOMPATIBLE"
        );
        assert!(rpc.returned_bytes().is_empty(), "fault {fault:?}");
        assert_eq!(rpc.open_connection_count(), 0, "fault {fault:?}");
        assert!(!rpc.followed_redirect(), "fault {fault:?}");
    }
}

#[test]
fn timeout_contract_is_two_second_connect_and_five_second_read_write() {
    assert_eq!(CONNECT_TIMEOUT_SECS, 2);
    assert_eq!(READ_TIMEOUT_SECS, 5);
    assert_eq!(WRITE_TIMEOUT_SECS, 5);
    assert_eq!(READINESS_TIMEOUT_SECS, 10);
    for (fault, boundary_millis) in [
        (RpcFault::ConnectDelay, 2_000),
        (RpcFault::ReadDelay, 5_000),
        (RpcFault::WriteDelay, 5_000),
    ] {
        for (millis, accepted) in [
            (boundary_millis - 1, true),
            (boundary_millis, true),
            (boundary_millis + 1, false),
        ] {
            let mut rpc = RpcTransportRig::wallet();
            rpc.set_timed_fault(fault, millis);
            let result = rpc.call(RpcMethod::GetVersion);
            assert_eq!(result.is_ok(), accepted, "{fault:?} {millis}");
            if !accepted {
                assert_eq!(result.unwrap_err().code(), "UNAVAILABLE");
                assert_eq!(rpc.open_connection_count(), 0);
            }
        }
    }

    let mut starting = RpcTransportRig::wallet();
    starting.script_startup_refusals(2, 125);
    starting.readiness().unwrap();
    assert_eq!(starting.readiness_attempts(), 3);
    assert!(starting.readiness_elapsed_millis() <= 10_000);
    assert!(
        starting
            .readiness_operation_timeouts_millis()
            .iter()
            .all(|timeout| *timeout <= 5_000)
    );

    let mut unavailable = RpcTransportRig::wallet();
    unavailable.script_startup_refusals(usize::MAX, 2_000);
    assert_eq!(unavailable.readiness().unwrap_err().code(), "UNAVAILABLE");
    assert_eq!(unavailable.readiness_elapsed_millis(), 10_000);

    let mut connect_timeout = RpcTransportRig::wallet();
    connect_timeout.set_timed_fault(RpcFault::ConnectDelay, 2_001);
    assert_eq!(
        connect_timeout.readiness().unwrap_err().code(),
        "UNAVAILABLE"
    );
    assert_eq!(connect_timeout.readiness_attempts(), 1);

    let mut other_connect_failure = RpcTransportRig::wallet();
    other_connect_failure.arm_fault(RpcFault::ConnectOtherFailure);
    assert_eq!(
        other_connect_failure.readiness().unwrap_err().code(),
        "UNAVAILABLE"
    );
    assert_eq!(other_connect_failure.readiness_attempts(), 1);

    let mut authenticated_refusal = RpcTransportRig::wallet();
    authenticated_refusal.arm_fault(RpcFault::AuthenticatedConnectionRefused);
    assert_eq!(
        authenticated_refusal.readiness().unwrap_err().code(),
        "UNAVAILABLE"
    );
    assert_eq!(authenticated_refusal.readiness_attempts(), 1);
    assert_eq!(authenticated_refusal.last_call_request_count(), 2);

    for (fault, code) in [
        (RpcFault::SecondChallenge, "UNAUTH"),
        (RpcFault::MalformedResponse, "PROTOCOL_INCOMPATIBLE"),
    ] {
        let mut rpc = RpcTransportRig::wallet();
        rpc.arm_fault(fault);
        assert_eq!(rpc.readiness().unwrap_err().code(), code);
        assert_eq!(rpc.readiness_attempts(), 1);
    }
}

#[test]
fn json_nesting_utf8_bom_trailing_duplicate_and_id_fail_closed() {
    assert_eq!(MAX_JSON_NESTING, 16);
    for (nesting, accepted) in [
        (MAX_JSON_NESTING - 1, true),
        (MAX_JSON_NESTING, true),
        (MAX_JSON_NESTING + 1, false),
    ] {
        let mut rpc = RpcTransportRig::wallet();
        rpc.script_json_nesting(nesting);
        let result = rpc.call(RpcMethod::GetVersion);
        assert_eq!(result.is_ok(), accepted, "nesting {nesting}");
    }
    for fault in [
        JsonFault::InvalidUtf8,
        JsonFault::Bom,
        JsonFault::TrailingBytes,
        JsonFault::DuplicateKey,
        JsonFault::WrongVersion,
        JsonFault::WrongId,
        JsonFault::MissingId,
        JsonFault::TypeConfusion,
        JsonFault::IntegerOverflow,
        JsonFault::UnsupportedShape,
    ] {
        let mut rpc = RpcTransportRig::wallet();
        rpc.arm_json_fault(fault);
        assert_eq!(
            rpc.call(RpcMethod::GetVersion).unwrap_err().code(),
            "PROTOCOL_INCOMPATIBLE"
        );
        assert!(rpc.returned_bytes().is_empty(), "fault {fault:?}");
    }
}

#[test]
fn node_calls_never_send_authorization_and_wallet_calls_always_do() {
    let mut node = RpcTransportRig::node();
    node.call(RpcMethod::GetInfo).unwrap();
    node.call(RpcMethod::HardForkInfo).unwrap();
    assert!(
        node.requests()
            .iter()
            .all(|request| request.authorization.is_none())
    );

    let mut wallet = RpcTransportRig::wallet();
    wallet.call(RpcMethod::GetVersion).unwrap();
    wallet.call(RpcMethod::GetHeight).unwrap();
    wallet.call(RpcMethod::GetBalance).unwrap();
    let balance = wallet.last_result().unwrap();
    assert_eq!(balance.balance, Some(1_000));
    assert_eq!(balance.unlocked_balance, Some(900));
    assert_eq!(balance.raw_string_count, 0);
    assert_eq!(
        wallet.last_result_members(),
        [
            "balance",
            "blocks_to_unlock",
            "multisig_import_needed",
            "per_subaddress",
            "time_to_unlock",
            "unlocked_balance",
        ]
    );
    assert_eq!(
        wallet.last_nested_result_members(),
        [
            "account_index",
            "address",
            "address_index",
            "balance",
            "blocks_to_unlock",
            "label",
            "num_unspent_outputs",
            "time_to_unlock",
            "unlocked_balance",
        ]
    );
    wallet.call(RpcMethod::CreateAddress).unwrap();
    let created = wallet.last_result().unwrap();
    assert_eq!(created.address_index, Some(7));
    assert_eq!(created.address_count, Some(1));
    assert_eq!(created.raw_string_count, 0);
    assert_eq!(
        wallet.last_result_members(),
        ["address", "address_index", "address_indices", "addresses"]
    );
    assert_eq!(wallet.requests().len(), 8);
    for requests in wallet.requests().chunks_exact(2) {
        assert!(requests[0].authorization.is_none());
        assert!(requests[1].authorization.is_some());
    }
}

#[test]
fn local_node_endpoints_are_internal_fixed_and_mainnet_has_no_side_effect() {
    for (network, endpoint) in [
        (XmrNetwork::Stagenet, "127.0.0.1:38081"),
        (XmrNetwork::Testnet, "127.0.0.1:28081"),
    ] {
        let mut probe = NodeProbeRig::reviewed(network);
        probe.probe().unwrap();
        assert_eq!(probe.attempted_endpoints(), [endpoint]);
        assert_eq!(probe.numeric_loopback_connection_count(), 2);
        assert_eq!(probe.dns_resolution_count(), 0);
        assert_eq!(probe.proxy_connection_count(), 0);
        assert!(!probe.followed_redirect());
        assert_eq!(
            probe.get_info_members(),
            [
                "adjusted_time",
                "alt_blocks_count",
                "block_size_limit",
                "block_size_median",
                "block_weight_limit",
                "block_weight_median",
                "bootstrap_daemon_address",
                "busy_syncing",
                "credits",
                "cumulative_difficulty",
                "cumulative_difficulty_top64",
                "database_size",
                "difficulty",
                "difficulty_top64",
                "free_space",
                "grey_peerlist_size",
                "height",
                "height_without_bootstrap",
                "incoming_connections_count",
                "mainnet",
                "nettype",
                "offline",
                "outgoing_connections_count",
                "restricted",
                "rpc_connections_count",
                "stagenet",
                "start_time",
                "status",
                "synchronized",
                "target",
                "target_height",
                "testnet",
                "top_block_hash",
                "top_hash",
                "tx_count",
                "tx_pool_size",
                "untrusted",
                "update_available",
                "version",
                "was_bootstrap_ever_used",
                "white_peerlist_size",
                "wide_cumulative_difficulty",
                "wide_difficulty",
            ]
        );
        assert_eq!(
            probe.hard_fork_info_members(),
            [
                "credits",
                "earliest_height",
                "enabled",
                "state",
                "status",
                "threshold",
                "top_hash",
                "untrusted",
                "version",
                "votes",
                "voting",
                "window",
            ]
        );
    }
    let mut mainnet = NodeProbeRig::from_product_network("xmr-mainnet");
    assert_eq!(mainnet.probe().unwrap_err().code(), "NETWORK_DISABLED");
    assert!(mainnet.operations().is_empty());
    assert!(mainnet.attempted_endpoints().is_empty());

    for (limit, median) in [(true, true), (false, true), (true, false), (false, false)] {
        let mut probe = NodeProbeRig::reviewed(XmrNetwork::Stagenet);
        probe.set_block_weight_optionals(limit, median);
        probe.probe().unwrap();
        assert_eq!(
            probe.get_info_members().contains(&"block_weight_limit"),
            limit
        );
        assert_eq!(
            probe.get_info_members().contains(&"block_weight_median"),
            median
        );
    }

    for fault in [
        RpcFault::NodeVersionNumeric,
        RpcFault::NodeVersionTypeConfusion,
        RpcFault::NodeVersionOverlong,
        RpcFault::BlockWeightLimitTypeConfusion,
        RpcFault::BlockWeightMedianTypeConfusion,
        RpcFault::MissingRequiredNodeMember,
        RpcFault::ExtraNodeMember,
    ] {
        let mut probe = NodeProbeRig::reviewed(XmrNetwork::Stagenet);
        probe.arm_fault(fault);
        assert_eq!(probe.probe().unwrap_err().code(), "NODE_UNAVAILABLE");
    }
}

#[test]
fn node_network_boolean_matrix_requires_exact_mutual_consistency() {
    for network in [XmrNetwork::Stagenet, XmrNetwork::Testnet] {
        for info in NodeInfo::network_boolean_matrix(network) {
            let expected = info.is_exact_for(network);
            let mut probe = NodeProbeRig::with_info(network, info);
            let result = probe.probe();
            assert_eq!(
                result.is_ok(),
                expected,
                "{network:?} {:?}",
                probe.scripted_info()
            );
            if !expected {
                assert_eq!(result.unwrap_err().code(), "NODE_UNAVAILABLE");
                assert_eq!(probe.attempted_endpoints().len(), 1);
            }
        }
    }
}

#[test]
fn node_syncing_is_distinct_from_bootstrap_remote_and_unavailable() {
    let mut syncing = NodeProbeRig::reviewed(XmrNetwork::Stagenet);
    syncing.info_mut().synchronized = false;
    assert_eq!(syncing.probe().unwrap().state.as_str(), "NODE_SYNCING");

    let mut past_bootstrap = NodeProbeRig::reviewed(XmrNetwork::Stagenet);
    past_bootstrap.info_mut().was_bootstrap_ever_used = true;
    assert_eq!(past_bootstrap.probe().unwrap().state.as_str(), "READY");

    let mut zero_target = NodeProbeRig::reviewed(XmrNetwork::Stagenet);
    zero_target.info_mut().target_height = 0;
    assert_eq!(zero_target.probe().unwrap().state.as_str(), "READY");

    let mut lower_target = NodeProbeRig::reviewed(XmrNetwork::Stagenet);
    lower_target.info_mut().target_height = 900;
    assert_eq!(lower_target.probe().unwrap().state.as_str(), "READY");

    let mut future_fork = NodeProbeRig::reviewed(XmrNetwork::Stagenet);
    future_fork.set_hard_fork(2_000, false, false);
    assert_eq!(future_fork.probe().unwrap().state.as_str(), "READY");

    let mut maximum_widths = NodeProbeRig::reviewed(XmrNetwork::Stagenet);
    for (field, value) in [
        ("version", u64::from(u8::MAX)),
        ("voting", u64::from(u8::MAX)),
        ("window", u64::from(u32::MAX)),
        ("votes", u64::from(u32::MAX)),
        ("threshold", u64::from(u32::MAX)),
        ("state", 2),
    ] {
        maximum_widths.set_hard_fork_field(field, value);
    }
    assert_eq!(maximum_widths.probe().unwrap().state.as_str(), "READY");

    for (field, value) in [
        ("version", u64::from(u8::MAX) + 1),
        ("voting", u64::from(u8::MAX) + 1),
        ("window", u64::from(u32::MAX) + 1),
        ("votes", u64::from(u32::MAX) + 1),
        ("threshold", u64::from(u32::MAX) + 1),
        ("state", u64::from(u32::MAX) + 1),
        ("state", 3),
    ] {
        let mut probe = NodeProbeRig::reviewed(XmrNetwork::Stagenet);
        probe.set_hard_fork_field(field, value);
        assert_eq!(probe.probe().unwrap_err().code(), "NODE_UNAVAILABLE");
    }

    let mut untrusted_fork = NodeProbeRig::reviewed(XmrNetwork::Stagenet);
    untrusted_fork.set_hard_fork(1_000, true, true);
    assert_eq!(
        untrusted_fork.probe().unwrap_err().code(),
        "NODE_UNAVAILABLE"
    );

    for mutation in ["bootstrap-address", "untrusted", "offline", "status-not-ok"] {
        let mut probe = NodeProbeRig::reviewed(XmrNetwork::Stagenet);
        probe.mutate_info(mutation);
        assert_eq!(probe.probe().unwrap_err().code(), "NODE_UNAVAILABLE");
        assert_eq!(probe.attempted_endpoints(), ["127.0.0.1:38081"]);
        assert!(!probe.attempted_alternate_endpoint());
    }
}

#[test]
fn node_auth_redirect_malformed_oversized_and_inconsistent_height_never_fallback() {
    for fault in [
        RpcFault::NodeAuthenticationRequired,
        RpcFault::Redirect,
        RpcFault::MalformedResponse,
        RpcFault::OversizedResponse,
        RpcFault::InconsistentHeights,
        RpcFault::ConnectionRefused,
    ] {
        let mut probe = NodeProbeRig::reviewed(XmrNetwork::Testnet);
        probe.arm_fault(fault);
        assert_eq!(probe.probe().unwrap_err().code(), "NODE_UNAVAILABLE");
        assert_eq!(probe.attempted_endpoints(), ["127.0.0.1:28081"]);
        assert!(!probe.attempted_alternate_endpoint());
        assert!(probe.returned_state().is_none());
    }
}

#[test]
fn closed_typed_rpc_allowlist_rejects_every_unlisted_method_immediately() {
    assert_eq!(
        RpcMethod::wallet_allowlist(),
        [
            "get_version",
            "create_wallet",
            "restore_deterministic_wallet",
            "generate_from_keys",
            "open_wallet",
            "close_wallet",
            "stop_wallet",
            "query_key",
            "refresh",
            "get_height",
            "get_balance",
            "get_address",
            "create_address",
            "validate_address",
        ]
    );
    assert_eq!(RpcMethod::node_allowlist(), ["get_info", "hard_fork_info"]);
    for (method, expected_params) in [
        (RpcMethod::GetVersion, "{}"),
        (RpcMethod::GetHeight, "{}"),
        (
            RpcMethod::GetBalance,
            r#"{"account_index":0,"address_indices":[],"all_accounts":false,"strict":true}"#,
        ),
        (
            RpcMethod::CreateAddress,
            r#"{"account_index":0,"count":1,"label":""}"#,
        ),
        (RpcMethod::CloseWallet, "{}"),
        (RpcMethod::StopWallet, "{}"),
    ] {
        let mut rpc = RpcTransportRig::wallet();
        rpc.call(method).unwrap();
        assert_eq!(rpc.last_request_params(), Some(expected_params));
    }
    for method in [RpcMethod::GetInfo, RpcMethod::HardForkInfo] {
        let mut rpc = RpcTransportRig::node();
        rpc.call(method).unwrap();
        assert_eq!(rpc.last_request_params(), Some("{}"));
    }
    for method in [
        RpcMethod::CreateWallet,
        RpcMethod::RestoreDeterministicWallet,
        RpcMethod::GenerateFromKeys,
        RpcMethod::OpenWallet,
        RpcMethod::QueryKey,
        RpcMethod::Refresh,
        RpcMethod::GetAddress,
        RpcMethod::ValidateAddress,
    ] {
        let mut rpc = RpcTransportRig::wallet();
        assert_eq!(rpc.call(method).unwrap_err().code(), "SCHEMA");
        assert_eq!(rpc.last_call_request_count(), 0);
    }
    let mut rpc = RpcTransportRig::wallet();
    for method in [
        "transfer",
        "transfer_split",
        "sweep_all",
        "describe_transfer",
        "sign_transfer",
        "submit_transfer",
        "relay_tx",
        "get_spend_proof",
        "export_key_images",
        "import_key_images",
        "make_multisig",
        "add_address_book",
        "start_mining",
        "set_daemon",
        "rpc.raw",
    ] {
        assert_eq!(
            rpc.invoke_unlisted_for_test(method).unwrap_err().code(),
            "SCHEMA"
        );
        assert_eq!(rpc.last_call_request_count(), 0, "method {method}");
        assert_eq!(rpc.last_dispatch_lookup(), Some((method, false)));
    }
}

#[test]
fn digest_secrets_and_raw_upstream_errors_are_wiped_and_never_observable() {
    let canaries = [
        "CANARY_WAL007_RPC_PASSWORD_6a01",
        "CANARY_WAL007_REALM_6a02",
        "CANARY_WAL007_NONCE_6a03",
        "CANARY_WAL007_CNONCE_6a04",
        "CANARY_WAL007_AUTHORIZATION_6a05",
        "CANARY_WAL007_UPSTREAM_ERROR_6a06",
    ];
    for exit in ["success", "error", "panic-unwind"] {
        let mut rpc = RpcTransportRig::wallet_with_canaries(&canaries);
        rpc.exercise_exit(exit);
        assert!(rpc.secret_buffers_wiped(exit));
        assert_eq!(rpc.retained_secret_observation_count(), 0);
        let observable = format!(
            "rpc={rpc:?};error={:?};logs={:?};panic={:?}",
            rpc.last_error(),
            rpc.logs(),
            rpc.panic_output()
        );
        for canary in canaries {
            assert!(!observable.contains(canary), "exit {exit}");
        }
        assert_eq!(rpc.open_connection_count(), 0);
    }
}
