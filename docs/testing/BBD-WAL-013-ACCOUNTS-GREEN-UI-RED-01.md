# WAL-013 account service green and native-window expected red

Actual bounded execution; inherited release blockers remain.

```json
{
  "commands": [
    {
      "argv": [
        "hermes",
        "--version"
      ],
      "exit": 0,
      "output": "Hermes Agent v0.18.2 (2026.7.7.2) \u00b7 upstream 67764dc0 \u00b7 local 10b6d1a9 (+1 carried commit)\nInstall directory: <home>/.hermes/hermes-agent\nInstall method: git\nPython: 3.11.15\nOpenAI SDK: 2.24.0\n"
    },
    {
      "argv": [
        "git",
        "rev-parse",
        "HEAD"
      ],
      "exit": 0,
      "output": "a6f77fa4df5a9d8eb018e4ccaf36483cfea74817\n"
    },
    {
      "argv": [
        "git",
        "status",
        "--short"
      ],
      "exit": 0,
      "output": " M package-lock.json\n M package.json\n M scripts/security-policy.js\n M test/electronSecurity.node.js\n M test/securityPolicy.node.js\n M test/walletBrokerBuild.node.js\n M test/walletBrokerRuntime.node.js\n M test/walletStartup.node.js\n M test/walletStartupSmoke.node.js\n M test/walletSupervisorShutdown.node.js\n M wallet-broker/Cargo.toml\n M wallet-broker/src/lib.rs\n?? docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md\n?? docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md\n?? docs/testing/BBD-WAL-013-ACCOUNTS-RED-01.md\n?? test/walletAccountManagement.node.js\n?? wallet-broker/src/accounts.rs\n?? wallet-broker/tests/account_management.rs\n?? wallet-broker/tests/account_native_ui.rs\n"
    },
    {
      "argv": [
        "<home>/.cargo/bin/rustup",
        "run",
        "1.98.0",
        "rustfmt",
        "--edition",
        "2024",
        "wallet-broker/src/accounts.rs",
        "wallet-broker/tests/account_management.rs",
        "wallet-broker/tests/account_native_ui.rs"
      ],
      "exit": 0,
      "output": ""
    },
    {
      "argv": [
        "<home>/.cargo/bin/rustup",
        "run",
        "1.98.0",
        "cargo",
        "test",
        "--manifest-path",
        "wallet-broker/Cargo.toml",
        "--locked",
        "--offline",
        "--no-default-features",
        "--features",
        "native-ui",
        "--test",
        "account_native_ui"
      ],
      "exit": 101,
      "output": "   Compiling bitbook-wallet-broker v0.1.0 (<repo>/wallet-broker)\nerror[E0432]: unresolved import `bitbook_wallet_broker::account_ui`\n  --> tests/account_native_ui.rs:12:28\n   |\n12 | use bitbook_wallet_broker::account_ui::{\n   |                            ^^^^^^^^^^ could not find `account_ui` in `bitbook_wallet_broker`\n\nFor more information about this error, try `rustc --explain E0432`.\nerror: could not compile `bitbook-wallet-broker` (test \"account_native_ui\") due to 1 previous error\nwarning: build failed, waiting for other jobs to finish...\n"
    },
    {
      "argv": [
        "<home>/.cargo/bin/rustup",
        "run",
        "1.98.0",
        "cargo",
        "test",
        "--manifest-path",
        "wallet-broker/Cargo.toml",
        "--locked",
        "--offline",
        "--no-default-features",
        "--test",
        "account_management",
        "--test",
        "vault_crypto",
        "--test",
        "vault_format",
        "--test",
        "vault_store",
        "--test",
        "vault_session",
        "--test",
        "native_surface"
      ],
      "exit": 0,
      "output": "   Compiling bitbook-wallet-broker v0.1.0 (<repo>/wallet-broker)\n    Finished `test` profile [unoptimized + debuginfo] target(s) in 8.86s\n     Running tests/account_management.rs (wallet-broker/target/debug/deps/account_management-aa95c72dd55220fd)\n\nrunning 12 tests\ntest hostile_roots_and_catalog_entries_fail_closed_without_chmod ... ok\ntest second_manager_same_root_is_unavailable_until_first_drops ... ok\ntest entropy_failure_and_duplicate_id_never_overwrite_existing_vault ... ok\ntest locked_export_copies_exact_encrypted_bytes_and_refuses_existing_or_link ... ok\ntest catalog_cap_is_256_without_running_per_account_kdf ... ok\ntest restart_lists_persisted_locked_identities_without_secret_canaries ... ok\ntest create_software_persists_locked_encrypted_account_with_scripted_seed ... ok\ntest restore_cancel_is_a_noop_and_confirm_commits_captured_not_replaced_bytes ... ok\ntest nonnative_origins_refuse_privileged_apis_before_side_effects ... ok\ntest list_does_not_extend_idle_deadline_and_backwards_clock_locks_all ... ok\ntest restore_rejects_hostile_inputs_and_never_overwrites_existing_id ... ok\ntest unlock_rejects_wrong_corrupt_and_short_material_then_valid_lock_cycle_works ... ok\n\ntest result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 24.03s\n\n     Running tests/native_surface.rs (wallet-broker/target/debug/deps/native_surface-fb091947b62a6c90)\n\nrunning 17 tests\ntest cancel_or_window_close_wipes_passphrase_and_performs_no_partial_action ... ok\ntest cancelled_native_xmr_installation_selection_stops_after_the_chooser ... ok\ntest generic_methods_cannot_reach_xmr_installation_selection ... ok\ntest export_and_restore_are_also_rejected_from_every_nonnative_origin ... ok\ntest generic_unlock_backup_and_future_payment_confirmation_methods_are_absent ... ok\ntest export_exchanges_only_a_path_with_dialog_and_ciphertext_stays_in_core ... ok\ntest invalid_passphrase_lengths_wipe_before_unlock_or_restore_custody ... ok\ntest invalid_unlock_and_export_accounts_fail_before_native_authority_moves ... ok\ntest invalid_utf8_restore_passphrase_wipes_before_custody_or_commit ... ok\ntest invalid_utf8_unlock_passphrase_wipes_before_custody ... ok\ntest native_error_text_is_closed_and_secret_free ... ok\ntest native_xmr_installation_selection_chooses_validates_verifies_and_persists_in_order ... ok\ntest password_prompt_is_masked_noncopyable_and_bounded ... ok\ntest restore_authenticates_before_metadata_and_requires_explicit_confirmation ... ok\ntest restore_cancel_never_commits_or_changes_active_state ... ok\ntest unlock_is_accepted_only_from_native_surface_origin ... ok\ntest xmr_installation_selection_is_rejected_before_every_nonnative_side_effect ... ok\n\ntest result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s\n\n     Running tests/vault_crypto.rs (wallet-broker/target/debug/deps/vault_crypto-d4d069d28d230122)\n\nrunning 11 tests\ntest crypto_constants_are_the_closed_v1_profile ... ok\ntest rfc5869_hkdf_sha256_vector_is_independent ... ok\ntest xchacha20poly1305_draft_vector_is_independent ... ok\ntest rfc9106_argon2id_v19_vector_is_independent ... ok\ntest zec_xmr_and_social_domain_substitution_cannot_open ... ok\ntest deterministic_entropy_produces_one_stable_envelope_and_is_fully_openable ... ok\ntest fresh_entropy_randomizes_both_salt_and_nonce_and_changes_ciphertext ... ok\ntest passphrases_are_exact_utf8_bytes_without_unicode_normalization ... ok\ntest wrong_passphrase_and_corrupt_tag_have_identical_public_failure ... ok\ntest salt_nonce_and_ciphertext_mutations_fail_closed ... ok\ntest authenticated_domain_mutations_all_fail_locked ... ok\n\ntest result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.09s\n\n     Running tests/vault_format.rs (wallet-broker/target/debug/deps/vault_format-ff3c927d0113ce69)\n\nrunning 11 tests\ntest asset_network_and_account_id_are_closed ... ok\ntest canonical_fixture_round_trips_byte_for_byte ... ok\ntest base64_is_unpadded_unspaced_and_exact_length ... ok\ntest golden_vault_fixture_is_exact_canonical_bytes ... ok\ntest metadata_constructor_rejects_zero_epoch_and_crossed_networks ... ok\ntest noncanonical_json_bom_crlf_trailing_and_invalid_utf8_are_rejected ... ok\ntest every_kdf_and_aead_parameter_downgrade_is_rejected_before_kdf ... ok\ntest wrong_json_types_and_noncanonical_epoch_are_rejected ... ok\ntest every_unknown_duplicate_missing_and_reordered_field_is_rejected ... ok\ntest envelope_limit_is_checked_before_body_allocation ... ok\ntest passphrase_and_plaintext_bounds_are_exact_before_entropy_or_kdf ... ok\n\ntest result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s\n\n     Running tests/vault_session.rs (wallet-broker/target/debug/deps/vault_session-b2e3540de753d46c)\n\nrunning 13 tests\ntest backward_clock_fails_all_sessions_locked ... ok\ntest clock_failure_during_unlock_explicitly_wipes_supplied_material ... ok\ntest idle_timeout_is_exactly_fifteen_minutes_and_locks_at_boundary ... ok\ntest late_native_authorization_at_existing_deadline_times_out_and_wipes ... ok\ntest invalid_account_unlock_is_schema_and_wipes_supplied_material ... ok\ntest global_lock_events_ignore_malformed_account_but_scoped_events_reject_it ... ok\ntest every_forced_lock_event_wipes_spend_material ... ok\ntest only_successful_native_authorization_resets_idle_deadline ... ok\ntest overflowing_deadline_fails_locked_instead_of_wrapping ... ok\ntest monotonic_clock_error_fails_locked ... ok\ntest polling_sync_backup_browsing_and_failed_or_cancelled_prompts_never_extend ... ok\ntest separate_accounts_have_isolated_deadlines_and_wipes ... ok\ntest status_and_account_listing_never_request_spend_secret ... ok\n\ntest result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s\n\n     Running tests/vault_store.rs (wallet-broker/target/debug/deps/vault_store-0bb86400311cbe86)\n\nrunning 20 tests\ntest active_filename_is_derived_only_from_validated_account_id ... ok\ntest directory_and_file_permission_mismatch_fail_before_read_or_replace ... ok\ntest concurrent_update_of_one_account_is_busy_but_other_accounts_are_isolated ... ok\ntest directory_sync_failure_reports_failure_with_one_complete_active_value ... ok\ntest export_is_ciphertext_only_exclusive_and_never_self_overwrites ... ok\ntest export_rejects_existing_and_non_regular_destinations ... ok\ntest every_write_fault_reports_failure_and_never_installs_partial_plaintext ... ok\ntest failure_during_recovery_remains_fail_closed_with_recoverable_staging ... ok\ntest first_restore_and_higher_epoch_each_require_explicit_native_confirmation ... ok\ntest full_directory_rollback_is_explicitly_residual_not_claimed_solved ... ok\ntest private_directory_and_active_file_modes_are_exact ... ok\ntest restore_rejects_account_asset_network_mismatch_and_corrupt_current_state ... ok\ntest restore_authenticates_before_metadata_or_confirmation_is_released ... ok\ntest staging_collision_is_not_reused_or_truncated ... ok\ntest stale_and_equal_restore_epochs_are_refused_even_when_confirmed ... ok\ntest linux_direct_operations_reject_wrong_mode_until_descriptor_repair ... ok\ntest successful_replacement_epoch_is_checked_and_strictly_increments ... ok\ntest reader_rejects_symlink_fifo_directory_device_and_oversize_before_allocation ... ok\ntest write_order_is_exclusive_complete_synced_atomic_and_directory_synced ... ok\ntest linux_store_enforces_real_modes_regular_files_and_symlink_rejection ... ok\n\ntest result: ok. 20 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s\n\n"
    },
    {
      "argv": [
        "<home>/.cargo/bin/rustup",
        "run",
        "1.98.0",
        "cargo",
        "test",
        "--manifest-path",
        "wallet-broker/Cargo.toml",
        "--locked",
        "--offline",
        "--no-default-features",
        "--test",
        "account_management",
        "nonnative_origins_refuse_privileged_apis_before_side_effects",
        "--",
        "--exact"
      ],
      "exit": 101,
      "output": "   Compiling bitbook-wallet-broker v0.1.0 (<repo>/wallet-broker)\nwarning: unused variable: `origin`\n   --> src/accounts.rs:437:30\n    |\n437 |     fn require_native(&self, origin: ActionOrigin) -> Result<(), AccountError> {\n    |                              ^^^^^^ help: if this is intentional, prefix it with an underscore: `_origin`\n    |\n    = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default\n\nwarning: `bitbook-wallet-broker` (lib) generated 1 warning (run `cargo fix --lib -p bitbook-wallet-broker` to apply 1 suggestion)\n    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.36s\n     Running tests/account_management.rs (wallet-broker/target/debug/deps/account_management-aa95c72dd55220fd)\n\nrunning 1 test\ntest nonnative_origins_refuse_privileged_apis_before_side_effects ... FAILED\n\nfailures:\n\n---- nonnative_origins_refuse_privileged_apis_before_side_effects stdout ----\n\nthread 'nonnative_origins_refuse_privileged_apis_before_side_effects' (4079125) panicked at tests/account_management.rs:861:60:\ncalled `Result::unwrap_err()` on an `Ok` value: AccountSummary { account_id: \"00112233445566778899aabbccddeeff\", asset: \"ZEC\", network: \"zec-testnet\", kind: \"software\", locked: true }\nnote: run with `RUST_BACKTRACE=1` environment variable to display a backtrace\n\n\nfailures:\n    nonnative_origins_refuse_privileged_apis_before_side_effects\n\ntest result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 11 filtered out; finished in 4.26s\n\nerror: test failed, to rerun pass `--test account_management`\n"
    },
    {
      "argv": [
        "<home>/.cargo/bin/rustup",
        "run",
        "1.98.0",
        "cargo",
        "test",
        "--manifest-path",
        "wallet-broker/Cargo.toml",
        "--locked",
        "--offline",
        "--no-default-features",
        "--test",
        "account_management",
        "nonnative_origins_refuse_privileged_apis_before_side_effects",
        "--",
        "--exact"
      ],
      "exit": 0,
      "output": "   Compiling bitbook-wallet-broker v0.1.0 (<repo>/wallet-broker)\n    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.04s\n     Running tests/account_management.rs (wallet-broker/target/debug/deps/account_management-aa95c72dd55220fd)\n\nrunning 1 test\ntest nonnative_origins_refuse_privileged_apis_before_side_effects ... ok\n\ntest result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 11 filtered out; finished in 11.22s\n\n"
    },
    {
      "argv": [
        "<home>/.cargo/bin/rustup",
        "run",
        "1.98.0",
        "cargo",
        "test",
        "--manifest-path",
        "wallet-broker/Cargo.toml",
        "--locked",
        "--offline",
        "--no-default-features",
        "--test",
        "account_management",
        "list_does_not_extend_idle_deadline_and_backwards_clock_locks_all",
        "--",
        "--exact"
      ],
      "exit": 101,
      "output": "   Compiling bitbook-wallet-broker v0.1.0 (<repo>/wallet-broker)\n    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.47s\n     Running tests/account_management.rs (wallet-broker/target/debug/deps/account_management-aa95c72dd55220fd)\n\nrunning 1 test\ntest list_does_not_extend_idle_deadline_and_backwards_clock_locks_all ... FAILED\n\nfailures:\n\n---- list_does_not_extend_idle_deadline_and_backwards_clock_locks_all stdout ----\n\nthread 'list_does_not_extend_idle_deadline_and_backwards_clock_locks_all' (4081212) panicked at tests/account_management.rs:808:5:\nassertion failed: listed.iter().all(|summary| summary.locked)\nnote: run with `RUST_BACKTRACE=1` environment variable to display a backtrace\n\n\nfailures:\n    list_does_not_extend_idle_deadline_and_backwards_clock_locks_all\n\ntest result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 11 filtered out; finished in 9.31s\n\nerror: test failed, to rerun pass `--test account_management`\n"
    },
    {
      "argv": [
        "<home>/.cargo/bin/rustup",
        "run",
        "1.98.0",
        "cargo",
        "test",
        "--manifest-path",
        "wallet-broker/Cargo.toml",
        "--locked",
        "--offline",
        "--no-default-features",
        "--test",
        "account_management",
        "list_does_not_extend_idle_deadline_and_backwards_clock_locks_all",
        "--",
        "--exact"
      ],
      "exit": 0,
      "output": "   Compiling bitbook-wallet-broker v0.1.0 (<repo>/wallet-broker)\n    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.52s\n     Running tests/account_management.rs (wallet-broker/target/debug/deps/account_management-aa95c72dd55220fd)\n\nrunning 1 test\ntest list_does_not_extend_idle_deadline_and_backwards_clock_locks_all ... ok\n\ntest result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 11 filtered out; finished in 14.95s\n\n"
    }
  ],
  "accepted": true,
  "version": "Hermes Agent v0.18.2 (2026.7.7.2) \u00b7 upstream 67764dc0 \u00b7 local 10b6d1a9 (+1 carried commit)\nInstall directory: <home>/.hermes/hermes-agent\nInstall method: git\nPython: 3.11.15\nOpenAI SDK: 2.24.0",
  "head": "a6f77fa4df5a9d8eb018e4ccaf36483cfea74817",
  "initial_status": "M package-lock.json\n M package.json\n M scripts/security-policy.js\n M test/electronSecurity.node.js\n M test/securityPolicy.node.js\n M test/walletBrokerBuild.node.js\n M test/walletBrokerRuntime.node.js\n M test/walletStartup.node.js\n M test/walletStartupSmoke.node.js\n M test/walletSupervisorShutdown.node.js\n M wallet-broker/Cargo.toml\n M wallet-broker/src/lib.rs\n?? docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md\n?? docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md\n?? docs/testing/BBD-WAL-013-ACCOUNTS-RED-01.md\n?? test/walletAccountManagement.node.js\n?? wallet-broker/src/accounts.rs\n?? wallet-broker/tests/account_management.rs\n?? wallet-broker/tests/account_native_ui.rs",
  "session": {
    "id": "20260910_104514_8fed69",
    "model": "poolside/laguna-s-2.1:free",
    "provider": "nous"
  },
  "input_hashes": {
    "wallet-broker/src/accounts.rs": "6100b5ae6dc9836f661b7da488a0c925bc0ed2dac60556e6858acf2a8e777bbd",
    "wallet-broker/tests/account_management.rs": "ddb292489e73b4f25d72133d98fdf204163f13254f5fa44a3a297bffbc3b6fae",
    "wallet-broker/tests/account_native_ui.rs": "d79859fdd0a8a1fd2707304688dcd325648d5d33a02ab0a6271af6b8c68ee919",
    "wallet-broker/src/lib.rs": "96bf49de102f16c026fb3d098cb3b447a269dc5f9176fd29c902ac55d44e7246",
    "wallet-broker/Cargo.toml": "c4e5700db87b30cd2da488fcc68c7305f8b59e2dd1b16f10da2fd6e59ab28cda",
    "wallet-broker/Cargo.lock": "b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71",
    "wallet-broker/src/vault.rs": "f23247b0e46c68dfc125804fa981d16f7022464e1e4d94a8c1e62771a045be49",
    "wallet-broker/src/store.rs": "611d837641069a98d05b9e68c14bf11a37a5076de58bf6516188870eeab19236",
    "wallet-broker/src/session.rs": "42e4f335bb4080ad530d93dcc04d824b4ab54835be7f6c7cd68feba3f20ee227",
    "wallet-broker/src/native.rs": "a0ccb0b11e3e636cc28f9576ec0cf6da2d8c2ef5344370e5a7a2a9a9b5cc32e5",
    "wallet-broker/src/native_ui.rs": "c6607af3f36278f8d7deeb2b4276017177bbb4c695017f18bf4d2bcb412b268f",
    "package.json": "76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5",
    "package-lock.json": "0e32de43f3c1a31ae3d9e9c8b172d10cd37a03577dc7d1eaf4ac916967cfcdb8",
    "scripts/security-policy.js": "fec10759a73a3c675d8206da454abf4b0e8dca6b62da463f39a2db27b30dc078",
    "test/securityPolicy.node.js": "a58d85e5322f0cb01a78b636454f92547bd98e556013cc07712dc0cf64ea565c"
  },
  "format_diffs": {
    "wallet-broker/src/accounts.rs": "--- wallet-broker/src/accounts.rs before\n+++ wallet-broker/src/accounts.rs formatted\n@@ -185,7 +185,9 @@\n         let ids = self.load_catalog()?;\n         Ok(ids\n             .into_iter()\n-            .map(|account_id| software_summary(&account_id, !self.sessions.is_unlocked(&account_id)))\n+            .map(|account_id| {\n+                software_summary(&account_id, !self.sessions.is_unlocked(&account_id))\n+            })\n             .collect())\n     }\n \n@@ -211,11 +213,7 @@\n         }\n \n         let mut id_bytes = [0u8; ACCOUNT_ID_BYTES];\n-        if self\n-            .entropy\n-            .fill(\"account-id\", &mut id_bytes)\n-            .is_err()\n-        {\n+        if self.entropy.fill(\"account-id\", &mut id_bytes).is_err() {\n             wipe_passphrase(&mut passphrase, &mut self.wipes);\n             return Err(AccountError::unavailable());\n         }\n@@ -226,11 +224,7 @@\n         }\n \n         let mut seed_bytes = Zeroizing::new([0u8; ACCOUNT_SEED_BYTES]);\n-        if self\n-            .entropy\n-            .fill(\"account-seed\", &mut *seed_bytes)\n-            .is_err()\n-        {\n+        if self.entropy.fill(\"account-seed\", &mut *seed_bytes).is_err() {\n             seed_bytes.zeroize();\n             wipe_passphrase(&mut passphrase, &mut self.wipes);\n             return Err(AccountError::unavailable());\n@@ -303,15 +297,11 @@\n             wipe_passphrase(&mut passphrase, &mut self.wipes);\n             return Err(AccountError::unavailable());\n         }\n-        let mut plaintext = match open_vault_bytes(\n-            &bytes,\n-            &mut passphrase,\n-            &mut IgnoreWork,\n-            &mut self.wipes,\n-        ) {\n-            Ok(plaintext) => plaintext,\n-            Err(error) => return Err(map_unlock_open(error)),\n-        };\n+        let mut plaintext =\n+            match open_vault_bytes(&bytes, &mut passphrase, &mut IgnoreWork, &mut self.wipes) {\n+                Ok(plaintext) => plaintext,\n+                Err(error) => return Err(map_unlock_open(error)),\n+            };\n         if plaintext.len() != ACCOUNT_SEED_BYTES {\n             plaintext.wipe_with(\"plaintext\", &mut self.wipes);\n             return Err(AccountError::locked());\n@@ -398,15 +388,11 @@\n             wipe_passphrase(&mut passphrase, &mut self.wipes);\n             return Err(AccountError::unavailable());\n         }\n-        let mut plaintext = match open_vault_bytes(\n-            &bytes,\n-            &mut passphrase,\n-            &mut IgnoreWork,\n-            &mut self.wipes,\n-        ) {\n-            Ok(plaintext) => plaintext,\n-            Err(error) => return Err(map_unlock_open(error)),\n-        };\n+        let mut plaintext =\n+            match open_vault_bytes(&bytes, &mut passphrase, &mut IgnoreWork, &mut self.wipes) {\n+                Ok(plaintext) => plaintext,\n+                Err(error) => return Err(map_unlock_open(error)),\n+            };\n         if plaintext.len() != ACCOUNT_SEED_BYTES {\n             plaintext.wipe_with(\"plaintext\", &mut self.wipes);\n             return Err(AccountError::locked());\n@@ -527,10 +513,13 @@\n         if info.len as usize > MAX_ENVELOPE_BYTES {\n             return Err(AccountError::unavailable());\n         }\n-        let bytes = self.store.read_active(account_id).map_err(map_catalog_store)?;\n-        let envelope =\n-            parse_vault(&bytes, &mut IgnoreWork).map_err(|_| AccountError::locked())?;\n-        if envelope.metadata().account_id_hex() != account_id || !supported_vault(envelope.metadata())\n+        let bytes = self\n+            .store\n+            .read_active(account_id)\n+            .map_err(map_catalog_store)?;\n+        let envelope = parse_vault(&bytes, &mut IgnoreWork).map_err(|_| AccountError::locked())?;\n+        if envelope.metadata().account_id_hex() != account_id\n+            || !supported_vault(envelope.metadata())\n         {\n             return Err(AccountError::unavailable());\n         }\n",
    "wallet-broker/tests/account_management.rs": "--- wallet-broker/tests/account_management.rs before\n+++ wallet-broker/tests/account_management.rs formatted\n@@ -161,10 +161,7 @@\n     for entry in entries {\n         match entry {\n             Ok(entry) => push_cleanup_error(&mut errors, unlink_any(&entry.path())),\n-            Err(error) => errors.push(format!(\n-                \"cannot read entry in {}: {error}\",\n-                dir.display()\n-            )),\n+            Err(error) => errors.push(format!(\"cannot read entry in {}: {error}\", dir.display())),\n         }\n     }\n     if errors.is_empty() {\n@@ -289,15 +286,17 @@\n     }\n \n     fn passphrase_wiped(&self, length: usize) -> bool {\n-        self.0.borrow().iter().any(|event| {\n-            event.label == \"passphrase\" && event.length == length && event.all_zero\n-        })\n+        self.0\n+            .borrow()\n+            .iter()\n+            .any(|event| event.label == \"passphrase\" && event.length == length && event.all_zero)\n     }\n \n     fn plaintext_wiped(&self, length: usize) -> bool {\n-        self.0.borrow().iter().any(|event| {\n-            event.label == \"plaintext\" && event.length == length && event.all_zero\n-        })\n+        self.0\n+            .borrow()\n+            .iter()\n+            .any(|event| event.label == \"plaintext\" && event.length == length && event.all_zero)\n     }\n }\n \n@@ -477,7 +476,10 @@\n         &hex_encode(&SEED_A),\n         &hex_encode(&SEED_B),\n     ] {\n-        assert!(!text.contains(needle), \"secret leaked in public text: {text}\");\n+        assert!(\n+            !text.contains(needle),\n+            \"secret leaked in public text: {text}\"\n+        );\n     }\n }\n \n@@ -517,15 +519,9 @@\n     };\n     let mut pass = SecretBytes::new(passphrase.to_vec()).unwrap();\n     let mut plain = SecretBytes::new(plaintext.to_vec()).unwrap();\n-    seal_vault(\n-        &metadata,\n-        &mut pass,\n-        &mut plain,\n-        &mut entropy,\n-        &mut Ignore,\n-    )\n-    .unwrap()\n-    .into_bytes()\n+    seal_vault(&metadata, &mut pass, &mut plain, &mut entropy, &mut Ignore)\n+        .unwrap()\n+        .into_bytes()\n }\n \n fn canonical_vault(account_id: &str, asset: &str, network: &str, epoch: u64) -> Vec<u8> {\n@@ -553,7 +549,9 @@\n     SecretBytes::new(PASSPHRASE.to_vec()).unwrap()\n }\n \n-fn listed_ids(manager: &mut AccountManager<SharedClock, ScriptedEntropy, SharedWipes>) -> Vec<String> {\n+fn listed_ids(\n+    manager: &mut AccountManager<SharedClock, ScriptedEntropy, SharedWipes>,\n+) -> Vec<String> {\n     manager\n         .list()\n         .unwrap()\n@@ -966,7 +964,13 @@\n         \"ALREADY_EXISTS\",\n     );\n     assert_eq!(\n-        fs::read(harness.scratch.accounts().join(format!(\"{}.vault\", id_hex(&ID_A)))).unwrap(),\n+        fs::read(\n+            harness\n+                .scratch\n+                .accounts()\n+                .join(format!(\"{}.vault\", id_hex(&ID_A)))\n+        )\n+        .unwrap(),\n         original\n     );\n     assert!(harness.wipes.passphrase_wiped(PASSPHRASE.len()));\n@@ -1017,7 +1021,12 @@\n         \"ALREADY_EXISTS\",\n     );\n     assert_eq!(fs::read(&dest).unwrap(), active);\n-    assert!(fs::symlink_metadata(&link).unwrap().file_type().is_symlink());\n+    assert!(\n+        fs::symlink_metadata(&link)\n+            .unwrap()\n+            .file_type()\n+            .is_symlink()\n+    );\n }\n \n #[test]\n@@ -1326,12 +1335,18 @@\n     assert_public_error(&manager.list().unwrap_err(), \"UNAVAILABLE\");\n     fs::remove_file(harness.scratch.accounts().join(\"readme.txt\")).unwrap();\n \n-    let link = harness.scratch.accounts().join(format!(\"{}.vault\", id_hex(&ID_B)));\n+    let link = harness\n+        .scratch\n+        .accounts()\n+        .join(format!(\"{}.vault\", id_hex(&ID_B)));\n     symlink(format!(\"{}.vault\", id_hex(&ID_A)), &link).unwrap();\n     assert_public_error(&manager.list().unwrap_err(), \"UNAVAILABLE\");\n     fs::remove_file(&link).unwrap();\n \n-    let fifo = harness.scratch.accounts().join(format!(\"{}.vault\", id_hex(&ID_B)));\n+    let fifo = harness\n+        .scratch\n+        .accounts()\n+        .join(format!(\"{}.vault\", id_hex(&ID_B)));\n     create_fifo(&fifo);\n     assert!(fs::symlink_metadata(&fifo).unwrap().file_type().is_fifo());\n     assert_public_error(&manager.list().unwrap_err(), \"UNAVAILABLE\");\n@@ -1357,7 +1372,13 @@\n         &canonical_vault(&id_hex(&ID_A), \"ZEC\", \"zec-testnet\", 1),\n     );\n     assert_public_error(&manager.list().unwrap_err(), \"UNAVAILABLE\");\n-    fs::remove_file(harness.scratch.accounts().join(format!(\"{}.vault\", id_hex(&ID_B)))).unwrap();\n+    fs::remove_file(\n+        harness\n+            .scratch\n+            .accounts()\n+            .join(format!(\"{}.vault\", id_hex(&ID_B))),\n+    )\n+    .unwrap();\n \n     plant_active(\n         &harness.scratch.root,\n@@ -1365,7 +1386,13 @@\n         &vec![b'x'; MAX_ENVELOPE_BYTES + 1],\n     );\n     assert_public_error(&manager.list().unwrap_err(), \"UNAVAILABLE\");\n-    fs::remove_file(harness.scratch.accounts().join(format!(\"{}.vault\", id_hex(&ID_B)))).unwrap();\n+    fs::remove_file(\n+        harness\n+            .scratch\n+            .accounts()\n+            .join(format!(\"{}.vault\", id_hex(&ID_B))),\n+    )\n+    .unwrap();\n \n     assert_eq!(listed_ids(&mut manager), vec![id_hex(&ID_A)]);\n \n@@ -1378,7 +1405,10 @@\n     );\n     for n in 0..1023u16 {\n         write_private(\n-            &bound.scratch.accounts().join(format!(\".{:032x}.{:016x}.stage\", n, n as u64)),\n+            &bound\n+                .scratch\n+                .accounts()\n+                .join(format!(\".{:032x}.{:016x}.stage\", n, n as u64)),\n             b\"stage\",\n         );\n     }\n@@ -1443,7 +1473,11 @@\n             && summary.network == \"zec-testnet\"\n             && summary.kind == \"software\"\n     }));\n-    assert!(listed.windows(2).all(|pair| pair[0].account_id < pair[1].account_id));\n+    assert!(\n+        listed\n+            .windows(2)\n+            .all(|pair| pair[0].account_id < pair[1].account_id)\n+    );\n \n     let before = vault_names(&harness.scratch.root);\n     assert_eq!(before.len(), 256);\n",
    "wallet-broker/tests/account_native_ui.rs": "--- wallet-broker/tests/account_native_ui.rs before\n+++ wallet-broker/tests/account_native_ui.rs formatted\n@@ -142,10 +142,7 @@\n         Self {\n             ctx,\n             time: 0.0,\n-            screen: egui::Rect::from_min_max(\n-                egui::pos2(0.0, 0.0),\n-                egui::pos2(size[0], size[1]),\n-            ),\n+            screen: egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(size[0], size[1])),\n             last_repaint_ms,\n         }\n     }\n@@ -217,7 +214,9 @@\n         leftover: (Vec<String>, Vec<String>),\n     ) -> Self {\n         let root = output.viewport_output.get(&egui::ViewportId::ROOT);\n-        let commands = root.map(|viewport| viewport.commands.as_slice()).unwrap_or(&[]);\n+        let commands = root\n+            .map(|viewport| viewport.commands.as_slice())\n+            .unwrap_or(&[]);\n         let title = root.and_then(|viewport| viewport.builder.title.clone());\n         let (leftover_events, leftover_raw_events) = leftover;\n         Self {\n@@ -256,7 +255,7 @@\n                 .iter()\n                 .map(|event| format!(\"{event:?}\"))\n                 .collect(),\n-      accesskit: format!(\"{:?}\", output.platform_output.accesskit_update),\n+            accesskit: format!(\"{:?}\", output.platform_output.accesskit_update),\n             events_description: output.platform_output.events_description(),\n             leftover_events,\n             leftover_raw_events,\n@@ -277,7 +276,10 @@\n         let (visual, clip) = *hits.last().expect(\"exact label must be painted\");\n         assert!(visual.is_finite());\n         assert!(visual.is_positive());\n-        assert!(clip.contains_rect(visual), \"label must be fully within clip\");\n+        assert!(\n+            clip.contains_rect(visual),\n+            \"label must be fully within clip\"\n+        );\n         assert!(\n             self.screen.contains_rect(visual),\n             \"label must be fully within screen\"\n@@ -314,7 +316,8 @@\n                 }\n             }\n         }\n-        let (_, field, clip) = best.expect(\"masked field must have painted geometry near its label\");\n+        let (_, field, clip) =\n+            best.expect(\"masked field must have painted geometry near its label\");\n         assert!(clip.contains_rect(field), \"field must be fully within clip\");\n         assert!(\n             self.screen.contains_rect(field),\n@@ -338,7 +341,10 @@\n \n     fn assert_no_substr(&self, needle: &str) {\n         for (text, _, _) in &self.painted {\n-            assert!(!text.contains(needle), \"painted text leaked {needle}: {text}\");\n+            assert!(\n+                !text.contains(needle),\n+                \"painted text leaked {needle}: {text}\"\n+            );\n         }\n         for text in &self.copy_texts {\n             assert!(!text.contains(needle), \"clipboard leaked {needle}: {text}\");\n@@ -500,10 +506,7 @@\n         prepared.summary.clone()\n     }\n \n-    fn confirm_restore(\n-        &mut self,\n-        prepared: Self::Restore,\n-    ) -> Result<AccountSummary, &'static str> {\n+    fn confirm_restore(&mut self, prepared: Self::Restore) -> Result<AccountSummary, &'static str> {\n         let mut state = self.inner.borrow_mut();\n         state.confirm_calls += 1;\n         state.accounts.push(prepared.summary.clone());\n@@ -781,12 +784,7 @@\n }\n \n fn leftover_event_text(ctx: &egui::Context) -> (Vec<String>, Vec<String>) {\n-    ctx.input(|input| {\n-        (\n-            event_texts(&input.events),\n-            event_texts(&input.raw.events),\n-        )\n-    })\n+    ctx.input(|input| (event_texts(&input.events), event_texts(&input.raw.events)))\n }\n \n fn event_texts(events: &[egui::Event]) -> Vec<String> {\n@@ -997,7 +995,10 @@\n     fill_matching(&mut ui, &mut app, &boundary);\n     let boundary_created = click_label(&mut ui, &mut app, CREATE);\n     assert_eq!(records.borrow().create_calls.len(), 1);\n-    assert_eq!(records.borrow().create_calls[0], boundary.as_bytes().to_vec());\n+    assert_eq!(\n+        records.borrow().create_calls[0],\n+        boundary.as_bytes().to_vec()\n+    );\n     boundary_created.assert_usable(CREATED_ID);\n     boundary_created.assert_no_substr(&boundary);\n \n@@ -1005,11 +1006,7 @@\n     let oversized = format!(\"{}\u00e9\", \"x\".repeat(1_023));\n     assert_eq!(oversized.len(), 1_025);\n     let _ = focus_field(&mut ui, &mut app, PASSPHRASE);\n-    let rejected_passphrase = ui.run(\n-        &mut app,\n-        vec![egui::Event::Paste(oversized.clone())],\n-        false,\n-    );\n+    let rejected_passphrase = ui.run(&mut app, vec![egui::Event::Paste(oversized.clone())], false);\n     assert_eq!(rejected_passphrase.bullet_count(), 0);\n     rejected_passphrase.assert_no_substr(&oversized);\n     let _ = focus_field(&mut ui, &mut app, CONFIRM_PASSPHRASE);\n@@ -1317,7 +1314,11 @@\n         false,\n     );\n     assert!(after_backspace.bullet_count() == 2);\n-    let _ = ui.run(&mut app, vec![key(egui::Key::Tab, egui::Modifiers::NONE)], false);\n+    let _ = ui.run(\n+        &mut app,\n+        vec![key(egui::Key::Tab, egui::Modifiers::NONE)],\n+        false,\n+    );\n     let _ = type_chars(&mut ui, &mut app, \"a\u00df\");\n     let created = click_label(&mut ui, &mut app, CREATE);\n     assert_eq!(records.borrow().create_calls.len(), 1);\n@@ -1430,9 +1431,7 @@\n \n     let vaults: Vec<_> = walk_regular_files(&scratch.accounts())\n         .into_iter()\n-        .filter(|(path, _)| {\n-            path.extension().and_then(|ext| ext.to_str()) == Some(\"vault\")\n-        })\n+        .filter(|(path, _)| path.extension().and_then(|ext| ext.to_str()) == Some(\"vault\"))\n         .collect();\n     assert_eq!(vaults.len(), 1);\n     let (vault_path, vault_bytes) = &vaults[0];\n"
  },
  "formatted_input_hashes": {
    "wallet-broker/src/accounts.rs": "f79eb3286cdccae2058e099c2d08b8e5a7167477a687e36609f91df4dc5b51f1",
    "wallet-broker/tests/account_management.rs": "7f876b1a9b7c4e65f8a6225d29867b2ae7dc4902f10cbbb829b4012675dff0d8",
    "wallet-broker/tests/account_native_ui.rs": "5a0facc561cd51b4f5ece6854df1a086ca8cd4a272cf92a45b08abfea0151abe",
    "wallet-broker/src/lib.rs": "96bf49de102f16c026fb3d098cb3b447a269dc5f9176fd29c902ac55d44e7246",
    "wallet-broker/Cargo.toml": "c4e5700db87b30cd2da488fcc68c7305f8b59e2dd1b16f10da2fd6e59ab28cda",
    "wallet-broker/Cargo.lock": "b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71",
    "wallet-broker/src/vault.rs": "f23247b0e46c68dfc125804fa981d16f7022464e1e4d94a8c1e62771a045be49",
    "wallet-broker/src/store.rs": "611d837641069a98d05b9e68c14bf11a37a5076de58bf6516188870eeab19236",
    "wallet-broker/src/session.rs": "42e4f335bb4080ad530d93dcc04d824b4ab54835be7f6c7cd68feba3f20ee227",
    "wallet-broker/src/native.rs": "a0ccb0b11e3e636cc28f9576ec0cf6da2d8c2ef5344370e5a7a2a9a9b5cc32e5",
    "wallet-broker/src/native_ui.rs": "c6607af3f36278f8d7deeb2b4276017177bbb4c695017f18bf4d2bcb412b268f",
    "package.json": "76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5",
    "package-lock.json": "0e32de43f3c1a31ae3d9e9c8b172d10cd37a03577dc7d1eaf4ac916967cfcdb8",
    "scripts/security-policy.js": "fec10759a73a3c675d8206da454abf4b0e8dca6b62da463f39a2db27b30dc078",
    "test/securityPolicy.node.js": "a58d85e5322f0cb01a78b636454f92547bd98e556013cc07712dc0cf64ea565c"
  },
  "ui_red_error_codes": [
    "E0432"
  ],
  "ui_expected_red": true,
  "distinct_green_tests": 84,
  "native_origin_detected": true,
  "native_origin_restored": true,
  "list_deadline_detected": true,
  "list_deadline_restored": true,
  "final_hashes": {
    "wallet-broker/src/accounts.rs": "f79eb3286cdccae2058e099c2d08b8e5a7167477a687e36609f91df4dc5b51f1",
    "wallet-broker/tests/account_management.rs": "7f876b1a9b7c4e65f8a6225d29867b2ae7dc4902f10cbbb829b4012675dff0d8",
    "wallet-broker/tests/account_native_ui.rs": "5a0facc561cd51b4f5ece6854df1a086ca8cd4a272cf92a45b08abfea0151abe",
    "wallet-broker/src/lib.rs": "96bf49de102f16c026fb3d098cb3b447a269dc5f9176fd29c902ac55d44e7246",
    "wallet-broker/Cargo.toml": "c4e5700db87b30cd2da488fcc68c7305f8b59e2dd1b16f10da2fd6e59ab28cda",
    "wallet-broker/Cargo.lock": "b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71",
    "wallet-broker/src/vault.rs": "f23247b0e46c68dfc125804fa981d16f7022464e1e4d94a8c1e62771a045be49",
    "wallet-broker/src/store.rs": "611d837641069a98d05b9e68c14bf11a37a5076de58bf6516188870eeab19236",
    "wallet-broker/src/session.rs": "42e4f335bb4080ad530d93dcc04d824b4ab54835be7f6c7cd68feba3f20ee227",
    "wallet-broker/src/native.rs": "a0ccb0b11e3e636cc28f9576ec0cf6da2d8c2ef5344370e5a7a2a9a9b5cc32e5",
    "wallet-broker/src/native_ui.rs": "c6607af3f36278f8d7deeb2b4276017177bbb4c695017f18bf4d2bcb412b268f",
    "package.json": "76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5",
    "package-lock.json": "0e32de43f3c1a31ae3d9e9c8b172d10cd37a03577dc7d1eaf4ac916967cfcdb8",
    "scripts/security-policy.js": "fec10759a73a3c675d8206da454abf4b0e8dca6b62da463f39a2db27b30dc078",
    "test/securityPolicy.node.js": "a58d85e5322f0cb01a78b636454f92547bd98e556013cc07712dc0cf64ea565c"
  }
}
```
