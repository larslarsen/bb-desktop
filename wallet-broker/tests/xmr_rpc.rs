use bitbook_wallet_broker::xmr::rpc::{
    CONNECT_TIMEOUT_SECS, MAX_HTTP_BYTES, MAX_JSON_NESTING, MAX_REQUEST_BODY_BYTES,
    READ_TIMEOUT_SECS, WRITE_TIMEOUT_SECS,
};
use bitbook_wallet_broker::xmr::test_support::{
    DigestVector, HttpFault, JsonFault, NodeInfo, NodeProbeRig, RpcFault, RpcMethod,
    RpcTransportRig, XmrNetwork,
};

#[test]
fn rfc_digest_vector_and_recorded_monero_challenge_match_independent_oracles() {
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
    assert_eq!(wallet.requests().len(), 4);
    assert!(wallet.requests()[0].authorization.is_none());
    assert!(wallet.requests()[1].authorization.is_some());
    assert!(wallet.requests()[2].authorization.is_none());
    assert!(wallet.requests()[3].authorization.is_some());
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
        assert!(!probe.used_dns());
        assert!(!probe.used_proxy());
        assert!(!probe.followed_redirect());
    }
    let mut mainnet = NodeProbeRig::from_product_network("xmr-mainnet");
    assert_eq!(mainnet.probe().unwrap_err().code(), "NETWORK_DISABLED");
    assert!(mainnet.operations().is_empty());
    assert!(mainnet.attempted_endpoints().is_empty());
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
