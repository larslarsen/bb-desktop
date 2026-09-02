use bitbook_wallet_broker::xmr::process::{
    CONNECT_TIMEOUT_SECS, MAX_ACTIVE_CHILDREN, MAX_LOG_FILE_BYTES, MAX_PORT_ATTEMPTS, PORT_MAX,
    PORT_MIN, READINESS_TIMEOUT_SECS, STOP_TIMEOUT_SECS,
};
use bitbook_wallet_broker::xmr::test_support::{
    ChildExit, ProcessFault, ProcessRig, TeardownCause, XmrNetwork,
};

const ACCOUNT: &str = "00112233445566778899aabbccddeeff";

fn rig(network: XmrNetwork) -> ProcessRig {
    ProcessRig::reviewed(ACCOUNT, network)
}

#[test]
fn stagenet_process_plan_has_exact_program_argv_environment_and_config() {
    let process = rig(XmrNetwork::Stagenet);
    let plan = process.plan().unwrap();
    assert_eq!(plan.argv0, "monero-wallet-rpc");
    assert_eq!(plan.argv, ["--config-file=wallet-rpc.conf"]);
    assert_eq!(plan.environment, [("LANG", "C")]);
    assert!(plan.program_is_verified_selection);
    assert!(plan.current_directory_is_private_runtime);
    assert_eq!(plan.config_relative_path, "wallet-rpc.conf");
    assert_eq!(plan.runtime_directory_mode, 0o700);
    assert_eq!(plan.wallet_directory_mode, 0o700);
    assert_eq!(plan.ring_directory_mode, 0o700);
    assert_eq!(plan.config_mode, 0o600);
    assert_eq!(plan.log_mode, 0o600);
    assert_eq!(
        plan.config_keys(),
        [
            "rpc-bind-ip",
            "rpc-bind-port",
            "rpc-login",
            "rpc-ssl",
            "daemon-address",
            "daemon-ssl",
            "untrusted-daemon",
            "wallet-dir",
            "shared-ringdb-dir",
            "no-dns",
            "non-interactive",
            "log-file",
            "log-level",
            "max-log-file-size",
            "max-log-files",
            "rpc-max-connections",
            "rpc-max-connections-per-private-ip",
            "rpc-max-connections-per-public-ip",
            "rpc-response-soft-limit",
            "stagenet",
        ]
    );
    assert_eq!(plan.config_value("rpc-bind-ip"), Some("127.0.0.1"));
    let rpc_port = plan.rpc_port().to_string();
    let rpc_login = format!(
        "{}:{}",
        plan.rpc_login_username(),
        plan.rpc_login_password()
    );
    assert_eq!(plan.config_value("rpc-bind-port"), Some(rpc_port.as_str()));
    assert_eq!(plan.config_value("rpc-login"), Some(rpc_login.as_str()));
    assert_eq!(plan.config_value("rpc-ssl"), Some("disabled"));
    assert_eq!(
        plan.config_value("daemon-address"),
        Some("http://127.0.0.1:38081")
    );
    assert_eq!(plan.config_value("daemon-ssl"), Some("disabled"));
    assert_eq!(plan.config_value("untrusted-daemon"), Some("1"));
    assert_eq!(
        plan.config_value("wallet-dir"),
        Some(plan.derived_wallet_directory())
    );
    assert_eq!(
        plan.config_value("shared-ringdb-dir"),
        Some(plan.derived_ring_directory())
    );
    assert_eq!(plan.config_value("no-dns"), Some("1"));
    assert_eq!(plan.config_value("non-interactive"), Some("1"));
    assert_eq!(plan.config_value("log-file"), Some(plan.derived_log_file()));
    assert_eq!(plan.config_value("log-level"), Some("0"));
    assert_eq!(MAX_LOG_FILE_BYTES, 1_048_576);
    assert_eq!(plan.config_value("max-log-file-size"), Some("1048576"));
    assert_eq!(plan.config_value("max-log-files"), Some("1"));
    assert_eq!(plan.config_value("rpc-max-connections"), Some("4"));
    assert_eq!(
        plan.config_value("rpc-max-connections-per-private-ip"),
        Some("4")
    );
    assert_eq!(
        plan.config_value("rpc-max-connections-per-public-ip"),
        Some("1")
    );
    assert_eq!(plan.config_value("rpc-response-soft-limit"), Some("65536"));
    assert_eq!(plan.config_value("stagenet"), Some("1"));
    assert_eq!(plan.config_value("testnet"), None);
    assert!(plan.private_paths_are_derived_from(ACCOUNT, "xmr-stagenet"));
    assert!(!plan.private_paths_accept_caller_input());
}

#[test]
fn testnet_changes_only_network_flag_and_fixed_local_daemon_port() {
    let stage = rig(XmrNetwork::Stagenet).plan().unwrap();
    let test = rig(XmrNetwork::Testnet).plan().unwrap();
    assert_eq!(
        test.config_value("daemon-address"),
        Some("http://127.0.0.1:28081")
    );
    assert_eq!(test.config_value("testnet"), Some("1"));
    assert_eq!(test.config_value("stagenet"), None);
    assert_eq!(stage.non_network_config(), test.non_network_config());
}

#[test]
fn full_wallet_rpc_is_authenticated_ipv4_loopback_without_forbidden_options() {
    let plan = rig(XmrNetwork::Stagenet).plan().unwrap();
    assert_eq!(plan.rpc_bind_ip(), "127.0.0.1");
    assert!(!plan.rpc_login_username().is_empty());
    assert!(!plan.rpc_login_password().is_empty());
    assert_ne!(plan.rpc_login_username(), plan.rpc_login_password());
    for forbidden in [
        "disable-rpc-login",
        "restricted-rpc",
        "confirm-external-bind",
        "rpc-access-control-origins",
        "trusted-daemon",
        "proxy",
        "tx-notify",
        "detach",
        "pidfile",
        "wallet-file",
        "wallet-password",
        "hw-device",
        "mainnet",
        "::1",
    ] {
        assert!(!plan.argv_and_config_text_for_test().contains(forbidden));
    }
    assert!(
        !plan
            .argv_and_environment_text_for_test()
            .contains(plan.rpc_login_password())
    );
    assert!(
        !plan
            .argv_and_environment_text_for_test()
            .contains(plan.selected_program_path())
    );
}

#[test]
fn rpc_login_and_port_use_fresh_os_entropy_and_do_not_repeat() {
    let first = rig(XmrNetwork::Stagenet).plan().unwrap();
    let second = rig(XmrNetwork::Stagenet).plan().unwrap();
    for plan in [&first, &second] {
        assert!(plan.rpc_port_from_os_entropy());
        assert!(plan.rpc_username_from_os_entropy());
        assert!(plan.rpc_password_from_os_entropy());
        assert!((PORT_MIN..=PORT_MAX).contains(&plan.rpc_port()));
        assert!(!plan.rpc_login_username().is_empty());
        assert!(!plan.rpc_login_password().is_empty());
    }
    assert_ne!(first.rpc_port(), second.rpc_port());
    assert_ne!(first.rpc_login_username(), second.rpc_login_username());
    assert_ne!(first.rpc_login_password(), second.rpc_login_password());
}

#[test]
fn random_port_range_and_collision_budget_cover_immediate_boundaries() {
    assert_eq!(PORT_MIN, 49_152);
    assert_eq!(PORT_MAX, 65_535);
    assert_eq!(MAX_PORT_ATTEMPTS, 16);
    for (port, accepted) in [
        (u32::from(PORT_MIN) - 1, false),
        (u32::from(PORT_MIN), true),
        (u32::from(PORT_MAX), true),
        (u32::from(PORT_MAX) + 1, false),
    ] {
        let mut process = rig(XmrNetwork::Stagenet);
        process.script_ports(&[port]);
        let result = process.reserve_port();
        assert_eq!(result.is_ok(), accepted, "port {port}");
        if !accepted {
            assert_eq!(result.unwrap_err().code(), "INTERNAL");
            assert_eq!(process.child_count(), 0);
        }
    }
    for (collisions, accepted, attempts) in [
        (MAX_PORT_ATTEMPTS - 1, true, MAX_PORT_ATTEMPTS),
        (MAX_PORT_ATTEMPTS, false, MAX_PORT_ATTEMPTS),
        (MAX_PORT_ATTEMPTS + 1, false, MAX_PORT_ATTEMPTS),
    ] {
        let mut process = rig(XmrNetwork::Stagenet);
        process.script_collisions(collisions);
        let result = process.reserve_port();
        assert_eq!(result.is_ok(), accepted, "collisions {collisions}");
        if !accepted {
            assert_eq!(result.unwrap_err().code(), "UNAVAILABLE");
            assert_eq!(process.child_count(), 0);
        }
        assert_eq!(process.port_attempts(), attempts);
    }
}

#[test]
fn startup_requires_authenticated_exact_version_before_ten_second_deadline() {
    assert_eq!(CONNECT_TIMEOUT_SECS, 2);
    assert_eq!(READINESS_TIMEOUT_SECS, 10);
    for (millis, accepted) in [(9_999, true), (10_000, true), (10_001, false)] {
        let mut process = rig(XmrNetwork::Stagenet);
        process.set_readiness_delay_millis(millis);
        let result = process.start();
        assert_eq!(result.is_ok(), accepted, "delay {millis}");
        if accepted {
            assert!(process.authenticated_readiness_observed());
            assert!(process.version_was_exact());
        } else {
            assert_eq!(result.unwrap_err().code(), "UNAVAILABLE");
            assert_eq!(process.child_count(), 0);
            assert!(process.runtime_secrets_removed());
        }
    }
}

#[test]
fn one_child_per_account_and_four_child_cap_are_fail_closed() {
    assert_eq!(MAX_ACTIVE_CHILDREN, 4);
    let mut process = ProcessRig::pool();
    for index in 0..MAX_ACTIVE_CHILDREN {
        process
            .start_account(&format!("{index:032x}"), XmrNetwork::Stagenet)
            .unwrap();
    }
    assert_eq!(process.child_count(), 4);
    assert_eq!(
        process
            .start_account("ffffffffffffffffffffffffffffffff", XmrNetwork::Testnet)
            .unwrap_err()
            .code(),
        "LIMIT"
    );
    assert_eq!(process.child_count(), 4);
    assert_eq!(
        process
            .start_account(&format!("{:032x}", 0), XmrNetwork::Stagenet)
            .unwrap_err()
            .code(),
        "LIMIT"
    );
    assert_eq!(process.child_count(), 4);
}

#[test]
fn invalid_account_network_and_mainnet_fail_before_directory_socket_or_process() {
    for account in [
        "",
        "00112233445566778899AABBCCDDEEFF",
        "00112233445566778899aabbccddeefg",
        "../112233445566778899aabbccddeeff",
    ] {
        let mut process = ProcessRig::new_unvalidated(account, "xmr-stagenet");
        assert_eq!(process.start().unwrap_err().code(), "SCHEMA");
        assert!(process.operations().is_empty());
    }
    let mut mainnet = ProcessRig::new_unvalidated(ACCOUNT, "xmr-mainnet");
    assert_eq!(mainnet.start().unwrap_err().code(), "NETWORK_DISABLED");
    assert!(mainnet.operations().is_empty());
}

#[test]
fn graceful_teardown_has_exact_closed_order_and_two_second_wait() {
    assert_eq!(STOP_TIMEOUT_SECS, 2);
    let mut process = rig(XmrNetwork::Stagenet);
    process.start().unwrap();
    process.teardown(TeardownCause::Lock).unwrap();
    assert_eq!(
        process.teardown_operations(),
        [
            "stop-new-calls",
            "close-wallet",
            "stop-wallet",
            "wait-2s",
            "reap",
            "wipe-rpc-login",
            "wipe-wallet-password",
            "close-sockets",
            "remove-runtime-secrets",
        ]
    );
    assert_eq!(process.child_count(), 0);
    assert_eq!(process.open_handle_count(), 0);
    assert!(process.runtime_secrets_removed());

    for (millis, killed) in [(1_999, false), (2_000, false), (2_001, true)] {
        let mut process = rig(XmrNetwork::Stagenet);
        process.start().unwrap();
        process.set_stop_delay_millis(millis);
        process.teardown(TeardownCause::Lock).unwrap();
        assert_eq!(process.used_forced_kill(), killed, "stop delay {millis}");
        assert_eq!(process.child_count(), 0);
        assert_eq!(process.open_handle_count(), 0);
    }
}

#[test]
fn hung_or_failed_child_is_killed_as_exact_process_group_then_reaped() {
    for exit in [
        ChildExit::Hung,
        ChildExit::StopRpcError,
        ChildExit::Unexpected,
    ] {
        let mut process = rig(XmrNetwork::Testnet);
        process.start().unwrap();
        process.set_child_exit(exit);
        let _ = process.teardown(TeardownCause::Failure);
        assert_eq!(
            process.teardown_operations(),
            [
                "stop-new-calls",
                "close-wallet",
                "stop-wallet",
                "wait-2s",
                "kill-exact-process-group",
                "reap",
                "wipe-rpc-login",
                "wipe-wallet-password",
                "close-sockets",
                "remove-runtime-secrets",
            ]
        );
        assert!(process.killed_only_owned_process_group());
        assert_eq!(process.child_count(), 0);
        assert_eq!(process.open_handle_count(), 0);
    }
}

#[test]
fn every_lifecycle_failure_reaps_and_removes_private_config() {
    for fault in [
        ProcessFault::ConfigWrite,
        ProcessFault::ConfigSync,
        ProcessFault::Spawn,
        ProcessFault::Authentication,
        ProcessFault::WrongVersion,
        ProcessFault::MalformedReadiness,
        ProcessFault::ExecutableRemoved,
        ProcessFault::ExecutableChanged,
        ProcessFault::BrokerExit,
    ] {
        let mut process = rig(XmrNetwork::Stagenet);
        process.arm_fault(fault);
        let _ = process.start_or_poll_for_test();
        assert_eq!(process.child_count(), 0, "fault {fault:?}");
        assert_eq!(process.open_handle_count(), 0, "fault {fault:?}");
        assert!(process.runtime_secrets_removed(), "fault {fault:?}");
        assert!(process.credentials_wiped(), "fault {fault:?}");
    }
}

#[test]
fn xmr_child_failure_does_not_stop_zec_or_social_supervisor() {
    let mut process = ProcessRig::with_isolation_observer();
    process
        .start_account(ACCOUNT, XmrNetwork::Stagenet)
        .unwrap();
    process
        .fail_account(ACCOUNT, ProcessFault::UnexpectedExit)
        .unwrap();
    assert_eq!(process.child_count(), 0);
    assert!(process.zec_alive());
    assert!(process.social_alive());
    assert_eq!(process.non_xmr_stop_calls(), 0);
}
