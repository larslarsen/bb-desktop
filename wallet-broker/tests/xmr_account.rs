use bitbook_wallet_broker::vault::SecretBytes;
use bitbook_wallet_broker::xmr::account::{
    MAX_SECRET_BYTES, MNEMONIC_WORDS, PASSWORD_HEX_BYTES, PRIMARY_ADDRESS_BYTES,
    RESTORE_SAFETY_MARGIN, VIEW_KEY_HEX_BYTES, XMR_SECRET_MAGIC,
};
use bitbook_wallet_broker::xmr::test_support::{
    AccountFault, AccountKind, AccountRig, HostileWalletEntry, SecretExit, XmrNetwork,
    XmrSecretFixture,
};

const ACCOUNT: &str = "00112233445566778899aabbccddeeff";
const OTHER_ACCOUNT: &str = "ffeeddccbbaa99887766554433221100";
const PRIMARY: &str = concat!(
    "4AAAAAAAAAAAAAAAAAAA",
    "AAAAAAAAAAAAAAAAAAAA",
    "AAAAAAAAAAAAAAAAAAAA",
    "AAAAAAAAAAAAAAAAAAAA",
    "AAAAAAAAAAAAAAA",
);
const MNEMONIC: &str = concat!(
    "abbey abducts ability ablaze abnormal abort abrasive absorb abstract absurd abuse academy ",
    "aces ache acidic acoustic acquire across actress adapt adept adhesive adjusted adopt adorned",
);
const VIEW_KEY: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
const PASSWORD: &str = "abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789";

fn software() -> AccountRig {
    assert_eq!(PRIMARY.len(), PRIMARY_ADDRESS_BYTES);
    AccountRig::software(ACCOUNT, XmrNetwork::Stagenet)
}

fn synthetic_primary() -> &'static str {
    assert_eq!(PRIMARY.len(), PRIMARY_ADDRESS_BYTES);
    assert!(!XmrSecretFixture::is_network_valid_address_for_test(
        PRIMARY
    ));
    PRIMARY
}

fn assert_synthetic_mnemonic_is_not_spendable() {
    let words: Vec<&str> = MNEMONIC.split_ascii_whitespace().collect();
    assert_eq!(words.len(), 25);
    assert!(!words[..24].contains(&words[24]));
    assert!(!XmrSecretFixture::is_spendable_mnemonic_for_test(MNEMONIC));
}

#[test]
fn xmr_secret_v1_software_bytes_are_exact_closed_big_endian_and_round_trip() {
    assert_eq!(XMR_SECRET_MAGIC, *b"BBXMR001");
    assert_eq!(PASSWORD_HEX_BYTES, 64);
    assert_eq!(PRIMARY_ADDRESS_BYTES, 95);
    assert_eq!(PRIMARY.len(), 95);
    assert_eq!(MNEMONIC_WORDS, 25);
    assert_eq!(MAX_SECRET_BYTES, 2_048);
    assert_synthetic_mnemonic_is_not_spendable();
    let fixture = XmrSecretFixture::software(900, PASSWORD, synthetic_primary(), MNEMONIC);
    let encoded = fixture.encode().unwrap();
    assert!(encoded.len() <= MAX_SECRET_BYTES);
    assert_eq!(&encoded[0..8], b"BBXMR001");
    assert_eq!(encoded[8], 1);
    assert_eq!(&encoded[9..17], &900_u64.to_be_bytes());
    assert_eq!(&encoded[17..19], &64_u16.to_be_bytes());
    assert_eq!(&encoded[19..83], PASSWORD.as_bytes());
    assert_eq!(&encoded[83..85], &95_u16.to_be_bytes());
    assert_eq!(&encoded[85..180], PRIMARY.as_bytes());
    assert_eq!(
        &encoded[180..182],
        &(u16::try_from(MNEMONIC.len()).unwrap()).to_be_bytes()
    );
    assert_eq!(&encoded[182..], MNEMONIC.as_bytes());
    assert_eq!(XmrSecretFixture::decode(&encoded).unwrap(), fixture);
}

#[test]
fn xmr_secret_v1_watch_only_bytes_are_exact_and_reject_unknown_or_trailing_data() {
    assert_eq!(VIEW_KEY_HEX_BYTES, 64);
    assert_eq!(PRIMARY.len(), 95);
    let fixture = XmrSecretFixture::watch_only(123, PASSWORD, synthetic_primary(), VIEW_KEY);
    let encoded = fixture.encode().unwrap();
    assert_eq!(encoded[8], 2);
    assert_eq!(&encoded[182..], VIEW_KEY.as_bytes());
    assert_eq!(XmrSecretFixture::decode(&encoded).unwrap(), fixture);

    for mutation in [
        "bad-magic",
        "unknown-kind",
        "bad-password-length",
        "bad-address-length",
        "bad-secret-length",
        "uppercase-password",
        "uppercase-view-key",
        "trailing-byte",
        "truncated",
        "over-2048",
    ] {
        let bytes = fixture.mutated_encoding(mutation);
        assert_eq!(
            XmrSecretFixture::decode(&bytes).unwrap_err().code(),
            "STATE_CORRUPT",
            "mutation {mutation}"
        );
    }
}

#[test]
fn secret_frame_component_and_total_length_boundaries_are_exact() {
    for (length, accepted) in [(63, false), (64, true), (65, false)] {
        assert_eq!(
            XmrSecretFixture::validate_password_hex_length_for_test(length).is_ok(),
            accepted,
            "password length {length}"
        );
        assert_eq!(
            XmrSecretFixture::validate_view_key_hex_length_for_test(length).is_ok(),
            accepted,
            "view-key length {length}"
        );
    }
    for (length, accepted) in [(94, false), (95, true), (96, false)] {
        assert_eq!(
            XmrSecretFixture::validate_primary_address_length_for_test(length).is_ok(),
            accepted,
            "address length {length}"
        );
    }
    for (length, accepted) in [
        (MAX_SECRET_BYTES - 1, true),
        (MAX_SECRET_BYTES, true),
        (MAX_SECRET_BYTES + 1, false),
    ] {
        assert_eq!(
            XmrSecretFixture::validate_total_length_for_test(length).is_ok(),
            accepted,
            "secret total length {length}"
        );
    }
}

#[test]
fn software_creation_has_exact_rpc_order_and_seals_before_success() {
    let mut account = software();
    account.set_local_height_without_bootstrap(1_000);
    let created = account.create_software().unwrap();
    assert_eq!(
        account.rpc_calls(),
        [
            "create_wallet",
            "query_key:mnemonic",
            "get_address",
            "close_wallet",
        ]
    );
    assert_eq!(
        account.rpc_field_names("create_wallet"),
        ["filename", "password", "language"]
    );
    assert_eq!(
        account.rpc_argument("create_wallet", "filename"),
        Some(ACCOUNT)
    );
    assert_eq!(
        account.rpc_argument("create_wallet", "language"),
        Some("English")
    );
    let password = account.rpc_argument("create_wallet", "password").unwrap();
    assert_eq!(password.len(), 64);
    assert!(
        password
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    );
    assert_eq!(
        account.rpc_argument("create_wallet", "restore_height"),
        None
    );
    assert_eq!(account.rpc_argument("create_wallet", "spendkey"), None);
    assert_eq!(created.account_id, ACCOUNT);
    assert_eq!(created.kind.as_str(), "software");
    assert_eq!(created.network.as_str(), "xmr-stagenet");
    assert_eq!(created.restore_height, 900);
    assert_eq!(account.sealed_secret_for_test().restore_height, 900);
    assert!(account.vault_sealed_before_account_state());
    assert!(account.account_state_durable_before_return());
    assert!(!created.contains_primary_address());
    assert!(!created.contains_secret());
}

#[test]
fn software_creation_primary_address_mismatch_rolls_back_without_account_or_secret_success() {
    let mut account = software();
    account.arm_fault(AccountFault::ReturnedPrimaryMismatch);

    assert_eq!(
        account.create_software().unwrap_err().code(),
        "PROTOCOL_INCOMPATIBLE"
    );
    assert!(account.returned_account().is_none());
    assert!(!account.vault_committed());
    assert!(!account.account_state_committed());
    assert_eq!(account.active_child_count(), 0);
    assert_eq!(account.open_handle_count(), 0);
    assert!(account.generated_wallet_removed_or_quarantined());
    assert!(account.creation_secrets_wiped());
}

#[test]
fn restore_height_saturating_margin_and_watch_only_bounds_are_exact() {
    assert_eq!(RESTORE_SAFETY_MARGIN, 100);
    for (height, expected) in [
        (0, 0),
        (99, 0),
        (100, 0),
        (101, 1),
        (u64::MAX, u64::MAX - 100),
    ] {
        let mut account = software();
        account.set_local_height_without_bootstrap(height);
        let created = account.create_software().unwrap();
        assert_eq!(created.restore_height, expected, "height {height}");
    }
    for (restore, local, accepted) in [
        (999, 1_000, true),
        (1_000, 1_000, true),
        (1_001, 1_000, false),
    ] {
        let mut account = AccountRig::watch_only(ACCOUNT, XmrNetwork::Testnet);
        account.set_local_height_without_bootstrap(local);
        let result = account.import_watch_only(synthetic_primary(), VIEW_KEY, restore);
        assert_eq!(result.is_ok(), accepted, "restore {restore} local {local}");
        if !accepted {
            assert_eq!(result.unwrap_err().code(), "SCHEMA");
            assert!(account.rpc_calls().is_empty());
            assert!(account.returned_account().is_none());
        }
    }
}

#[test]
fn wallet_password_uses_fresh_thirty_two_byte_os_entropy_and_never_repeats() {
    let first = AccountRig::fresh_wallet_password_for_test().unwrap();
    let second = AccountRig::fresh_wallet_password_for_test().unwrap();
    for password in [&first, &second] {
        assert_eq!(password.source, "os-entropy");
        assert_eq!(password.source_bytes, 32);
        assert_eq!(password.encoded.len(), 64);
        assert!(
            password
                .encoded
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        );
    }
    assert_ne!(first.encoded, second.encoded);
}

#[test]
fn watch_only_import_uses_generate_from_keys_and_never_expands_spend_capability() {
    let mut account = AccountRig::watch_only(ACCOUNT, XmrNetwork::Testnet);
    account.set_local_height_without_bootstrap(500);
    let imported = account
        .import_watch_only(synthetic_primary(), VIEW_KEY, 400)
        .unwrap();
    assert_eq!(
        account.rpc_calls(),
        ["generate_from_keys", "get_address", "close_wallet"]
    );
    assert_eq!(
        account.rpc_field_names("generate_from_keys"),
        [
            "filename",
            "password",
            "address",
            "viewkey",
            "restore_height"
        ]
    );
    assert_eq!(
        account.rpc_argument("generate_from_keys", "address"),
        Some(PRIMARY)
    );
    assert_eq!(
        account.rpc_argument("generate_from_keys", "viewkey"),
        Some(VIEW_KEY)
    );
    assert_eq!(
        account.rpc_argument("generate_from_keys", "restore_height"),
        Some("400")
    );
    assert_eq!(account.rpc_argument("generate_from_keys", "spendkey"), None);
    assert_eq!(imported.kind.as_str(), "watch_only");
    assert!(account.rpc_reported_watch_only());
    assert!(!imported.capabilities.can_prepare_tx);
    assert!(!imported.capabilities.can_sign_spend);
    assert!(!imported.capabilities.can_broadcast);
}

#[test]
fn watch_only_rpc_kind_or_primary_mismatch_rolls_back_without_account_or_secret_success() {
    for fault in [
        AccountFault::ReportedNotWatchOnly,
        AccountFault::ReturnedPrimaryMismatch,
    ] {
        let mut account = AccountRig::watch_only(ACCOUNT, XmrNetwork::Testnet);
        account.set_local_height_without_bootstrap(500);
        account.arm_fault(fault);

        assert_eq!(
            account
                .import_watch_only(synthetic_primary(), VIEW_KEY, 400)
                .unwrap_err()
                .code(),
            "PROTOCOL_INCOMPATIBLE",
            "fault {fault:?}"
        );
        assert!(account.returned_account().is_none(), "fault {fault:?}");
        assert!(!account.vault_committed(), "fault {fault:?}");
        assert!(!account.account_state_committed(), "fault {fault:?}");
        assert_eq!(account.active_child_count(), 0, "fault {fault:?}");
        assert_eq!(account.open_handle_count(), 0, "fault {fault:?}");
        assert!(
            account.generated_wallet_removed_or_quarantined(),
            "fault {fault:?}"
        );
        assert!(account.creation_secrets_wiped(), "fault {fault:?}");
    }
}

#[test]
fn existing_files_open_while_missing_files_recover_from_authenticated_vault() {
    for kind in [AccountKind::Software, AccountKind::WatchOnly] {
        let mut existing = AccountRig::sealed(ACCOUNT, XmrNetwork::Stagenet, kind);
        let existing_sealed = existing.authenticated_sealed_record_for_test().unwrap();
        existing.open().unwrap();
        assert_eq!(existing.rpc_calls()[0], "open_wallet");
        assert_eq!(
            existing.rpc_field_names("open_wallet"),
            ["filename", "password"]
        );
        assert_eq!(
            existing.rpc_argument("open_wallet", "filename"),
            Some(ACCOUNT)
        );
        assert!(existing.rpc_argument_matches_secret(
            "open_wallet",
            "password",
            existing_sealed.wallet_password(),
        ));
        assert!(!existing.recovery_created_files());

        let mut missing = AccountRig::sealed(ACCOUNT, XmrNetwork::Stagenet, kind);
        let missing_sealed = missing.authenticated_sealed_record_for_test().unwrap();
        let expected_restore_height = missing_sealed.restore_height().to_string();
        missing.remove_wallet_files_for_test();
        missing.open().unwrap();
        assert!(missing.vault_authenticated_before_recovery());
        match kind {
            AccountKind::Software => {
                assert_eq!(missing.rpc_calls()[0], "restore_deterministic_wallet");
                assert_eq!(
                    missing.rpc_field_names("restore_deterministic_wallet"),
                    ["filename", "password", "seed", "restore_height", "language"]
                );
                assert_eq!(
                    missing.rpc_argument("restore_deterministic_wallet", "filename"),
                    Some(ACCOUNT)
                );
                assert!(missing.rpc_argument_matches_secret(
                    "restore_deterministic_wallet",
                    "password",
                    missing_sealed.wallet_password(),
                ));
                assert!(missing.rpc_argument_matches_secret(
                    "restore_deterministic_wallet",
                    "seed",
                    missing_sealed.mnemonic().unwrap(),
                ));
                assert_eq!(
                    missing.rpc_argument("restore_deterministic_wallet", "restore_height"),
                    Some(expected_restore_height.as_str())
                );
                assert_eq!(
                    missing.rpc_argument("restore_deterministic_wallet", "language"),
                    Some("English")
                );
                assert_eq!(
                    missing.rpc_argument("restore_deterministic_wallet", "spendkey"),
                    None
                );
                assert_eq!(
                    missing.rpc_argument("restore_deterministic_wallet", "address"),
                    None
                );
                assert_eq!(
                    missing.rpc_argument("restore_deterministic_wallet", "viewkey"),
                    None
                );
            }
            AccountKind::WatchOnly => {
                assert_eq!(missing.rpc_calls()[0], "generate_from_keys");
                assert_eq!(
                    missing.rpc_field_names("generate_from_keys"),
                    [
                        "filename",
                        "password",
                        "address",
                        "viewkey",
                        "restore_height"
                    ]
                );
                assert_eq!(
                    missing.rpc_argument("generate_from_keys", "filename"),
                    Some(ACCOUNT)
                );
                assert!(missing.rpc_argument_matches_secret(
                    "generate_from_keys",
                    "password",
                    missing_sealed.wallet_password(),
                ));
                assert_eq!(
                    missing.rpc_argument("generate_from_keys", "address"),
                    Some(missing_sealed.primary_address())
                );
                assert!(missing.rpc_argument_matches_secret(
                    "generate_from_keys",
                    "viewkey",
                    missing_sealed.private_view_key().unwrap(),
                ));
                assert_eq!(
                    missing.rpc_argument("generate_from_keys", "restore_height"),
                    Some(expected_restore_height.as_str())
                );
                assert_eq!(missing.rpc_argument("generate_from_keys", "spendkey"), None);
                assert_eq!(missing.rpc_argument("generate_from_keys", "seed"), None);
                assert_eq!(missing.rpc_argument("generate_from_keys", "language"), None);
            }
        }
        assert!(missing.recovery_created_files());
        assert!(missing.identity_verified_after_open());
    }
}

#[test]
fn primary_network_kind_restore_height_and_wallet_file_substitution_fail_closed() {
    for mutation in [
        "primary-address",
        "network",
        "kind",
        "restore-height",
        "account-id",
        "partial-wallet-set",
    ] {
        let mut account = AccountRig::sealed(ACCOUNT, XmrNetwork::Stagenet, AccountKind::Software);
        account.mutate_open_identity(mutation);
        assert_eq!(
            account.open().unwrap_err().code(),
            "STATE_CORRUPT",
            "mutation {mutation}"
        );
        assert!(account.returned_account().is_none());
        assert_eq!(account.active_child_count(), 0);
        assert_eq!(account.open_handle_count(), 0);
    }
}

#[test]
fn wallet_paths_are_derived_network_bound_private_and_hostile_entries_are_rejected() {
    let account = AccountRig::sealed(ACCOUNT, XmrNetwork::Testnet, AccountKind::Software);
    let paths = account.inspect_paths();
    assert_eq!(
        paths.relative_account_directory,
        format!("xmr-testnet/{ACCOUNT}")
    );
    assert_eq!(paths.wallet_filename, ACCOUNT);
    assert_eq!(paths.directory_mode, 0o700);
    assert_eq!(paths.wallet_file_mode, 0o600);
    assert_eq!(paths.keys_file_mode, 0o600);
    assert_eq!(paths.state_db_mode, 0o600);
    assert_eq!(paths.state_db_synchronous, "FULL");
    assert!(!paths.relative_account_directory.contains(".."));
    assert!(!paths.wallet_filename.contains('/'));

    for entry in [
        HostileWalletEntry::Symlink,
        HostileWalletEntry::Directory,
        HostileWalletEntry::Fifo,
        HostileWalletEntry::WrongOwner,
        HostileWalletEntry::WrongMode,
        HostileWalletEntry::CrossAccount,
        HostileWalletEntry::CrossNetwork,
    ] {
        let mut hostile = AccountRig::with_hostile_entry(ACCOUNT, XmrNetwork::Testnet, entry);
        assert_eq!(hostile.open().unwrap_err().code(), "STATE_CORRUPT");
        assert!(!hostile.replaced_hostile_entry());
        assert_eq!(hostile.active_child_count(), 0);
    }
}

#[test]
fn invalid_account_and_mainnet_fail_before_path_node_wallet_or_vault_side_effect() {
    for account_id in [
        "",
        "00112233445566778899AABBCCDDEEFF",
        "00112233445566778899aabbccddeefg",
        "00112233445566778899aabbccddee",
        "../112233445566778899aabbccddeeff",
    ] {
        let mut account = AccountRig::unvalidated(account_id, "xmr-stagenet", "software");
        assert_eq!(account.create().unwrap_err().code(), "SCHEMA");
        assert!(account.operations().is_empty());
    }
    let mut mainnet = AccountRig::unvalidated(ACCOUNT, "xmr-mainnet", "software");
    assert_eq!(mainnet.create().unwrap_err().code(), "NETWORK_DISABLED");
    assert!(mainnet.operations().is_empty());
}

#[test]
fn failed_vault_or_state_persistence_rolls_back_and_cleanup_failure_is_compound_internal() {
    for fault in [
        AccountFault::VaultSeal,
        AccountFault::StateWrite,
        AccountFault::StateFileSync,
        AccountFault::StateDirectorySync,
    ] {
        let mut account = software();
        account.arm_fault(fault);
        let error = account.create_software().unwrap_err();
        assert!(matches!(error.code(), "STATE_CORRUPT" | "INTERNAL"));
        assert!(account.returned_account().is_none());
        assert_eq!(account.active_child_count(), 0);
        assert_eq!(account.open_handle_count(), 0);
        assert!(account.generated_wallet_removed_or_quarantined());
    }

    let mut compound = software();
    compound.arm_fault(AccountFault::VaultSeal);
    compound.arm_fault(AccountFault::RollbackCleanup);
    assert_eq!(compound.create_software().unwrap_err().code(), "INTERNAL");
    assert!(compound.account_unavailable());
    assert!(compound.returned_account().is_none());
    assert_eq!(compound.active_child_count(), 0);
}

#[test]
fn software_lock_closes_and_stops_child_while_watch_only_retention_is_post_open_only() {
    let mut software = AccountRig::sealed(ACCOUNT, XmrNetwork::Stagenet, AccountKind::Software);
    software.open().unwrap();
    software.lock().unwrap();
    assert_eq!(software.last_rpc_calls(), ["close_wallet", "stop_wallet"]);
    assert_eq!(software.active_child_count(), 0);
    assert!(software.wallet_password_wiped());

    let mut watch = AccountRig::sealed(OTHER_ACCOUNT, XmrNetwork::Stagenet, AccountKind::WatchOnly);
    assert!(!watch.may_retain_process());
    watch.open().unwrap();
    assert!(watch.may_retain_process());
    watch.simulate_cold_restart();
    assert!(!watch.may_retain_process());
    assert!(watch.requires_authenticated_vault_for_password());
}

#[test]
fn mnemonic_view_key_password_and_native_import_values_wipe_on_every_exit() {
    let canaries = [
        b"CANARY_WAL007_MNEMONIC_7101".as_slice(),
        b"CANARY_WAL007_VIEW_KEY_7102".as_slice(),
        b"CANARY_WAL007_WALLET_PASSWORD_7103".as_slice(),
        b"CANARY_WAL007_NATIVE_IMPORT_7104".as_slice(),
    ];
    for exit in [
        SecretExit::Success,
        SecretExit::Error,
        SecretExit::Cancellation,
        SecretExit::PanicUnwind,
        SecretExit::Drop,
    ] {
        let mut account = software();
        let secrets = canaries
            .iter()
            .map(|bytes| SecretBytes::new(bytes.to_vec()).unwrap())
            .collect();
        account.exercise_secret_exit(exit, secrets);
        assert!(account.all_secret_wipes_observed(exit));
        assert_eq!(account.open_handle_count(), 0);
        assert_eq!(account.active_child_count(), 0);
    }
}
