# WAL-014 receive validation, native rendering and unchanged policy baseline

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
      "output": "800a08587e943d9a8f31faa1e0c4d54a7b77e44c\n"
    },
    {
      "argv": [
        "git",
        "status",
        "--short"
      ],
      "exit": 0,
      "output": " M package-lock.json\n M package.json\n M scripts/security-policy.js\n M test/securityPolicy.node.js\n M wallet-broker/src/account_ui.rs\n M wallet-broker/src/accounts.rs\n M wallet-broker/src/session.rs\n M wallet-broker/src/zec.rs\n M wallet-broker/tests/account_management.rs\n M wallet-broker/tests/account_native_ui.rs\n?? docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md\n?? docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md\n?? docs/testing/BBD-WAL-014-RECEIVE-RED-01.md\n"
    },
    {
      "argv": [
        "<home>/.cargo/bin/rustup",
        "run",
        "1.98.0",
        "rustfmt",
        "--edition",
        "2024",
        "--emit",
        "stdout"
      ],
      "stdin_path": "wallet-broker/src/accounts.rs",
      "stdin_sha256": "8a710331002001ca3c5af6ec794b2f2fe3f21ededa1c87440896ab682e511850",
      "exit": 0,
      "output": "",
      "formatted_sha256": "5768cef932b8926bdfc80810d799e81e99ba63370ad8f06a48fb7b7563a566e1"
    },
    {
      "argv": [
        "<home>/.cargo/bin/rustup",
        "run",
        "1.98.0",
        "rustfmt",
        "--edition",
        "2024",
        "--emit",
        "stdout"
      ],
      "stdin_path": "wallet-broker/src/session.rs",
      "stdin_sha256": "9f88b3b0eca91d341a1c025bc8394a25e456350be58e99d9ac9417e31eef21af",
      "exit": 0,
      "output": "",
      "formatted_sha256": "9f88b3b0eca91d341a1c025bc8394a25e456350be58e99d9ac9417e31eef21af"
    },
    {
      "argv": [
        "<home>/.cargo/bin/rustup",
        "run",
        "1.98.0",
        "rustfmt",
        "--edition",
        "2024",
        "--emit",
        "stdout"
      ],
      "stdin_path": "wallet-broker/src/zec.rs",
      "stdin_sha256": "76e26abc01494340d660ee4891dbf671174b07a59673038702713e5861bb683d",
      "exit": 0,
      "output": "",
      "formatted_sha256": "2544d86b3022aacea6e64f75ce37c38da0ce27cb6f555e085fc76450b5760d1e"
    },
    {
      "argv": [
        "<home>/.cargo/bin/rustup",
        "run",
        "1.98.0",
        "rustfmt",
        "--edition",
        "2024",
        "--emit",
        "stdout"
      ],
      "stdin_path": "wallet-broker/src/account_ui.rs",
      "stdin_sha256": "3c38c7fe201c53a6d586a5fb7d0d44554c4d9a8a8fcf7b2ef80afcc66f3794a8",
      "exit": 0,
      "output": "",
      "formatted_sha256": "fc671ebf02ecb92ef28101540ad78fad15a1b326cd198e41b85ede91be52be99"
    },
    {
      "argv": [
        "<home>/.cargo/bin/rustup",
        "run",
        "1.98.0",
        "rustfmt",
        "--edition",
        "2024",
        "--emit",
        "stdout"
      ],
      "stdin_path": "wallet-broker/tests/account_management.rs",
      "stdin_sha256": "f7a81c67b9dc428a9a1ce26b9c0463deaaa3f59bc93ccfc5f1ef34aef0fe0a3b",
      "exit": 0,
      "output": "",
      "formatted_sha256": "817fa7972be4aebec3799a83ac998e572dd5b6f54c93683c5f16077069382e77"
    },
    {
      "argv": [
        "<home>/.cargo/bin/rustup",
        "run",
        "1.98.0",
        "rustfmt",
        "--edition",
        "2024",
        "--emit",
        "stdout"
      ],
      "stdin_path": "wallet-broker/tests/account_native_ui.rs",
      "stdin_sha256": "397c905889dc0ace02e8b51f28e6ca7a147b5e5504ec5de7b2e3ec017970ade4",
      "exit": 0,
      "output": "",
      "formatted_sha256": "eaa1dbb0a9db410c41be7daaac6663dc07d369a6a0769b6460ecf7983382d560"
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
        "vault_session",
        "--test",
        "zec_address",
        "--test",
        "zec_store",
        "--test",
        "zec_hygiene"
      ],
      "exit": 0,
      "output": "   Compiling bitbook-wallet-broker v0.1.0 (<repo>/wallet-broker)\n    Finished `test` profile [unoptimized + debuginfo] target(s) in 5.30s\n     Running tests/account_management.rs (wallet-broker/target/debug/deps/account_management-aa95c72dd55220fd)\n\nrunning 16 tests\ntest hostile_roots_and_catalog_entries_fail_closed_without_chmod ... ok\ntest entropy_failure_and_duplicate_id_never_overwrite_existing_vault ... ok\ntest second_manager_same_root_is_unavailable_until_first_drops ... ok\ntest catalog_cap_is_256_without_running_per_account_kdf ... ok\ntest locked_export_copies_exact_encrypted_bytes_and_refuses_existing_or_link ... ok\ntest restart_lists_persisted_locked_identities_without_secret_canaries ... ok\ntest wal014_foreign_ufvk_binding_refuses_without_advancing_issuance ... ok\ntest restore_cancel_is_a_noop_and_confirm_commits_captured_not_replaced_bytes ... ok\ntest create_software_persists_locked_encrypted_account_with_scripted_seed ... ok\ntest wal014_corrupt_viewing_db_and_symlink_directory_are_never_replaced ... ok\ntest nonnative_origins_refuse_privileged_apis_before_side_effects ... ok\ntest wal014_receive_is_native_unlocked_valid_and_does_not_extend_idle_deadline ... ok\ntest wal014_receiver_matches_seed_oracle_is_durable_private_and_preserves_vault ... ok\ntest list_does_not_extend_idle_deadline_and_backwards_clock_locks_all ... ok\ntest restore_rejects_hostile_inputs_and_never_overwrites_existing_id ... ok\ntest unlock_rejects_wrong_corrupt_and_short_material_then_valid_lock_cycle_works ... ok\n\ntest result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 28.85s\n\n     Running tests/vault_session.rs (wallet-broker/target/debug/deps/vault_session-b2e3540de753d46c)\n\nrunning 13 tests\ntest backward_clock_fails_all_sessions_locked ... ok\ntest clock_failure_during_unlock_explicitly_wipes_supplied_material ... ok\ntest every_forced_lock_event_wipes_spend_material ... ok\ntest idle_timeout_is_exactly_fifteen_minutes_and_locks_at_boundary ... ok\ntest invalid_account_unlock_is_schema_and_wipes_supplied_material ... ok\ntest late_native_authorization_at_existing_deadline_times_out_and_wipes ... ok\ntest monotonic_clock_error_fails_locked ... ok\ntest only_successful_native_authorization_resets_idle_deadline ... ok\ntest global_lock_events_ignore_malformed_account_but_scoped_events_reject_it ... ok\ntest overflowing_deadline_fails_locked_instead_of_wrapping ... ok\ntest separate_accounts_have_isolated_deadlines_and_wipes ... ok\ntest polling_sync_backup_browsing_and_failed_or_cancelled_prompts_never_extend ... ok\ntest status_and_account_listing_never_request_spend_secret ... ok\n\ntest result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s\n\n     Running tests/zec_address.rs (wallet-broker/target/debug/deps/zec_address-defdc746026cd429)\n\nrunning 8 tests\ntest seed_is_wiped_on_success_error_cancellation_replacement_unwind_and_drop ... ok\ntest two_concurrent_issuers_serialize_one_account_without_duplicates ... ok\ntest fresh_receiver_decodes_to_exactly_one_orchard_protocol_receiver ... ok\ntest account_network_and_mainnet_validation_precede_database_or_derivation ... ok\ntest receiver_issuance_is_monotonic_durable_and_viewing_only_after_reopen ... ok\ntest unsupported_receiver_composition_never_falls_back ... ok\ntest coupled_receiver_state_write_failure_returns_nothing_and_advances_neither_record ... ok\ntest receiver_limits_cover_immediate_below_at_and_above_without_wrap ... ok\n\ntest result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.88s\n\n     Running tests/zec_hygiene.rs (wallet-broker/target/debug/deps/zec_hygiene-10e748b72dc7e7fd)\n\nrunning 8 tests\ntest public_capability_surface_has_no_raw_sign_prove_finalize_extract_broadcast_or_network_authority ... ok\ntest diagnostics_limit_covers_immediate_below_at_and_above_without_secret_echo ... ok\ntest panic_unwind_wipes_seed_derived_material_and_prepared_artifact_before_return ... ok\ntest sanitized_prepared_value_has_exact_closed_fields_and_no_raw_artifact ... ok\ntest raw_prepared_state_is_memory_only_and_absent_after_close ... ok\ntest handle_lookup_is_bound_to_account_session_request_and_intent_with_constant_shape_miss ... ok\ntest debug_display_diagnostics_and_logs_omit_every_secret_class ... ok\ntest every_named_lifecycle_edge_invalidates_handle_and_wipes_prepared_state ... ok\n\ntest result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 15.65s\n\n     Running tests/zec_store.rs (wallet-broker/target/debug/deps/zec_store-70a070239af5d78b)\n\nrunning 8 tests\ntest symlink_nonregular_and_wrong_mode_state_are_rejected_without_replacement ... ok\ntest sqlite_paths_are_closed_account_network_derived_and_linux_private ... ok\ntest store_limits_cover_immediate_below_at_and_above_before_allocation ... ok\ntest initialization_and_reopen_bind_exact_account_network_and_schema ... ok\ntest sqlite_schema_and_rows_contain_viewing_state_but_no_spend_secrets ... ok\ntest schema_migration_is_atomic_across_write_sync_and_commit_failures ... ok\ntest failed_write_file_sync_and_directory_sync_never_report_durable_state ... ok\ntest corrupt_wrong_schema_and_truncated_sqlite_fail_closed_without_empty_recreation ... ok\n\ntest result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 7.59s\n\n"
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
        "account_native_ui",
        "--test",
        "native_surface"
      ],
      "exit": 0,
      "output": "   Compiling bitbook-wallet-broker v0.1.0 (<repo>/wallet-broker)\n    Finished `test` profile [unoptimized + debuginfo] target(s) in 25.44s\n     Running tests/account_native_ui.rs (wallet-broker/target/debug/deps/account_native_ui-6dcf24c6022d9a39)\n\nrunning 12 tests\ntest export_chooser_cancel_skips_port_and_chosen_path_reaches_port ... ok\ntest empty_and_populated_list_paints_notices_and_selection_controls ... ok\ntest copy_cut_undo_and_platform_events_never_expose_secret ... ok\ntest restore_requires_distinct_confirm_and_cannot_replay ... ok\ntest window_control_reopen_quit_drop_and_closed_list_errors ... ok\ntest unlock_wrong_then_correct_clears_password_and_lock_calls ... ok\ntest small_and_tall_layouts_keep_action_controls_usable ... ok\ntest create_form_accepts_matching_unicode_and_rejects_invalid ... ok\ntest wal014_receive_scene_issues_once_paints_honestly_and_copies_release_output ... ok\ntest wal014_receive_state_clears_on_lock_catalog_change_error_selection_and_hide ... ok\ntest shared_port_and_local_manager_create_unlock_lock_and_restart ... ok\ntest wal014_shared_port_pointer_receive_and_copy_persist_real_issuance ... ok\n\ntest result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 11.38s\n\n     Running tests/native_surface.rs (wallet-broker/target/debug/deps/native_surface-4cf4948bd503fe83)\n\nrunning 17 tests\ntest cancel_or_window_close_wipes_passphrase_and_performs_no_partial_action ... ok\ntest cancelled_native_xmr_installation_selection_stops_after_the_chooser ... ok\ntest export_and_restore_are_also_rejected_from_every_nonnative_origin ... ok\ntest export_exchanges_only_a_path_with_dialog_and_ciphertext_stays_in_core ... ok\ntest generic_unlock_backup_and_future_payment_confirmation_methods_are_absent ... ok\ntest generic_methods_cannot_reach_xmr_installation_selection ... ok\ntest invalid_unlock_and_export_accounts_fail_before_native_authority_moves ... ok\ntest invalid_passphrase_lengths_wipe_before_unlock_or_restore_custody ... ok\ntest invalid_utf8_restore_passphrase_wipes_before_custody_or_commit ... ok\ntest invalid_utf8_unlock_passphrase_wipes_before_custody ... ok\ntest native_error_text_is_closed_and_secret_free ... ok\ntest native_xmr_installation_selection_chooses_validates_verifies_and_persists_in_order ... ok\ntest password_prompt_is_masked_noncopyable_and_bounded ... ok\ntest restore_authenticates_before_metadata_and_requires_explicit_confirmation ... ok\ntest restore_cancel_never_commits_or_changes_active_state ... ok\ntest unlock_is_accepted_only_from_native_surface_origin ... ok\ntest xmr_installation_selection_is_rejected_before_every_nonnative_side_effect ... ok\n\ntest result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s\n\n"
    },
    {
      "argv": [
        "<home>/.cargo/bin/rustup",
        "run",
        "1.98.0",
        "cargo",
        "clippy",
        "--manifest-path",
        "wallet-broker/Cargo.toml",
        "--locked",
        "--offline",
        "--no-default-features",
        "--features",
        "native-ui",
        "--lib",
        "--bin",
        "bitbook-wallet-broker",
        "--test",
        "account_management",
        "--test",
        "account_native_ui",
        "--",
        "-D",
        "warnings"
      ],
      "exit": 101,
      "output": "    Checking bitbook-wallet-broker v0.1.0 (<repo>/wallet-broker)\nerror: this function has too many arguments (8/7)\n   --> tests/account_management.rs:598:1\n    |\n598 | / fn seal_bytes(\n599 | |     account_id: [u8; 16],\n600 | |     asset: Asset,\n601 | |     network: Network,\n...   |\n606 | |     nonce: u8,\n607 | | ) -> Vec<u8> {\n    | |____________^\n    |\n    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.98.0/index.html#too_many_arguments\n    = note: `-D clippy::too-many-arguments` implied by `-D warnings`\n    = help: to override `-D warnings` add `#[allow(clippy::too_many_arguments)]`\n\nerror: the borrowed expression implements the required traits\n    --> tests/account_management.rs:1343:25\n     |\n1343 |     fs::set_permissions(&accounts_case.accounts(), fs::Permissions::from_mode(0o755)).unwrap();\n     |                         ^^^^^^^^^^^^^^^^^^^^^^^^^ help: change this to: `accounts_case.accounts()`\n     |\n     = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.98.0/index.html#needless_borrows_for_generic_args\n     = note: `-D clippy::needless-borrows-for-generic-args` implied by `-D warnings`\n     = help: to override `-D warnings` add `#[allow(clippy::needless_borrows_for_generic_args)]`\n\nerror: the borrowed expression implements the required traits\n    --> tests/account_management.rs:1377:30\n     |\n1377 |         fs::symlink_metadata(&symlink_root.accounts()),\n     |                              ^^^^^^^^^^^^^^^^^^^^^^^^ help: change this to: `symlink_root.accounts()`\n     |\n     = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.98.0/index.html#needless_borrows_for_generic_args\n\nerror: the borrowed expression implements the required traits\n    --> tests/account_management.rs:1388:29\n     |\n1388 |     symlink(&real_accounts, &symlink_accounts.accounts()).unwrap();\n     |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: change this to: `symlink_accounts.accounts()`\n     |\n     = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.98.0/index.html#needless_borrows_for_generic_args\n\nerror: this `impl` can be derived\n   --> tests/account_native_ui.rs:553:1\n    |\n553 | / impl Default for DialogState {\n554 | |     fn default() -> Self {\n555 | |         Self {\n556 | |             export_queue: Vec::new(),\n...   |\n563 | | }\n    | |_^\n    |\n    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.98.0/index.html#derivable_impls\n    = note: `-D clippy::derivable-impls` implied by `-D warnings`\n    = help: to override `-D warnings` add `#[allow(clippy::derivable_impls)]`\nhelp: replace the manual implementation with a derive attribute\n    |\n129 + #[derive(Default)]\n130 | struct DialogState {\n    |\n\nerror: could not compile `bitbook-wallet-broker` (test \"account_management\") due to 4 previous errors\nwarning: build failed, waiting for other jobs to finish...\nerror: very complex type used. Consider factoring parts into `type` definitions\n    --> tests/account_native_ui.rs:1016:6\n     |\n1016 |   ) -> (\n     |  ______^\n1017 | |     AccountWindow<FakePort, FakeDialogs>,\n1018 | |     Rc<RefCell<PortState>>,\n1019 | |     Rc<RefCell<DialogState>>,\n1020 | | ) {\n     | |_^\n     |\n     = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.98.0/index.html#type_complexity\n     = note: `-D clippy::type-complexity` implied by `-D warnings`\n     = help: to override `-D warnings` add `#[allow(clippy::type_complexity)]`\n\nerror: field assignment outside of initializer for an instance created with Default::default()\n    --> tests/account_native_ui.rs:1022:5\n     |\n1022 |     state.accounts = accounts;\n     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^\n     |\nnote: consider initializing the variable with `PortState { accounts: accounts, ..Default::default() }` and removing relevant reassignments\n    --> tests/account_native_ui.rs:1021:5\n     |\n1021 |     let mut state = PortState::default();\n     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n     = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.98.0/index.html#field_reassign_with_default\n     = note: `-D clippy::field-reassign-with-default` implied by `-D warnings`\n     = help: to override `-D warnings` add `#[allow(clippy::field_reassign_with_default)]`\n\nerror: could not compile `bitbook-wallet-broker` (test \"account_native_ui\") due to 3 previous errors\n"
    }
  ],
  "accepted": false,
  "version": "Hermes Agent v0.18.2 (2026.7.7.2) \u00b7 upstream 67764dc0 \u00b7 local 10b6d1a9 (+1 carried commit)\nInstall directory: <home>/.hermes/hermes-agent\nInstall method: git\nPython: 3.11.15\nOpenAI SDK: 2.24.0",
  "head": "800a08587e943d9a8f31faa1e0c4d54a7b77e44c",
  "initial_status": "M package-lock.json\n M package.json\n M scripts/security-policy.js\n M test/securityPolicy.node.js\n M wallet-broker/src/account_ui.rs\n M wallet-broker/src/accounts.rs\n M wallet-broker/src/session.rs\n M wallet-broker/src/zec.rs\n M wallet-broker/tests/account_management.rs\n M wallet-broker/tests/account_native_ui.rs\n?? docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md\n?? docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md\n?? docs/testing/BBD-WAL-014-RECEIVE-RED-01.md",
  "session": {
    "id": "20260910_134304_2a1d4e",
    "model": "meituan/longcat-2.0:free",
    "provider": "nous"
  },
  "input_hashes": {
    "wallet-broker/tests/account_management.rs": "f7a81c67b9dc428a9a1ce26b9c0463deaaa3f59bc93ccfc5f1ef34aef0fe0a3b",
    "wallet-broker/tests/account_native_ui.rs": "397c905889dc0ace02e8b51f28e6ca7a147b5e5504ec5de7b2e3ec017970ade4",
    "wallet-broker/src/accounts.rs": "8a710331002001ca3c5af6ec794b2f2fe3f21ededa1c87440896ab682e511850",
    "wallet-broker/src/session.rs": "9f88b3b0eca91d341a1c025bc8394a25e456350be58e99d9ac9417e31eef21af",
    "wallet-broker/src/zec.rs": "76e26abc01494340d660ee4891dbf671174b07a59673038702713e5861bb683d",
    "wallet-broker/src/account_ui.rs": "3c38c7fe201c53a6d586a5fb7d0d44554c4d9a8a8fcf7b2ef80afcc66f3794a8",
    "wallet-broker/src/lib.rs": "e37713c4fd1a528a9260b8b11d740f1a317198f2eaf62a534729f4a566a10437",
    "wallet-broker/src/vault.rs": "f23247b0e46c68dfc125804fa981d16f7022464e1e4d94a8c1e62771a045be49",
    "wallet-broker/src/store.rs": "611d837641069a98d05b9e68c14bf11a37a5076de58bf6516188870eeab19236",
    "wallet-broker/src/zec/store.rs": "531d0a6171ecd9b5012602ccb870f16eab96888c809b3ffdf4e6303576f17c90",
    "wallet-broker/src/zec/address.rs": "d9d54e301d188e3d4cb539095c5c01bcb07aaf011fcb1a50129aadc18f24cbfe",
    "wallet-broker/src/zec/scan.rs": "54a890fd0b5a8ca8ce78dfb59c13424407341df25e47a5eacd2de8d6cc3317ad",
    "wallet-broker/Cargo.toml": "73e5e585eb2fd1ca867d66962460cfa010443886a4b59d8a33556ef2a4ff3503",
    "wallet-broker/Cargo.lock": "b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71",
    "package.json": "76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5",
    "package-lock.json": "0e32de43f3c1a31ae3d9e9c8b172d10cd37a03577dc7d1eaf4ac916967cfcdb8",
    "scripts/security-policy.js": "fec10759a73a3c675d8206da454abf4b0e8dca6b62da463f39a2db27b30dc078",
    "test/securityPolicy.node.js": "a58d85e5322f0cb01a78b636454f92547bd98e556013cc07712dc0cf64ea565c",
    "wallet-broker/src/runtime.rs": "6788914524605f4669d20d65f7b6ef7282b0090a0696ea45f088ecc49dc1c962",
    "wallet-broker/supervisor.js": "115e0e51425e7b25d8f6178b206f592165cb14dacde8cb544b970839833c4721",
    "social-main.js": "d959e6adffc2579fa3f2f447f60a165f2b6bcdc279afde2f6ddaa91bc9d9aa4a",
    "scripts/build-wallet-broker.js": "4427ef3e713ec77c45daefe8d37b457ca8dc2c8529b30214dbf65628f43a1445",
    "test/walletAccountManagement.node.js": "6a449335e1101b8dd6340d640514e8fc083a09235210c17dd582b57230de9475",
    "test/walletStartup.node.js": "448fb88268655b979c7c724ab21d13d09fee29a219628cd70f576daae4589de1",
    "test/walletSupervisor.node.js": "7fcb775d2db7ae8f28b6d9f7660c44a056e59891f3e552db80a248d843c76aa4",
    "test/walletStartupSmoke.node.js": "a1c75efee5fcfdbeb9bf56dfc64f0e747ab6aa527d30ce416c5027bc2aa2a1ef",
    "test/walletAccountWindowSmoke.node.js": "3590096cafed5596378488e8ebca8c916e62352d4410e5748a2033ab0ec9fa3a",
    "test/fixtures/wallet-broker/x11-window.py": "d192f036781e5623b88a8ba3f31b9ee7c781f7bdffa46982aa431f8ada5330d3",
    "wallet-broker/tests/vault_session.rs": "67487db86d6788633e031418da71f6080409ac57a144ad362e311cb22519be6b",
    "wallet-broker/tests/zec_address.rs": "2c5012e6884c8c2a81236266c6861b6e4e4fd6b124656dad2ab438add5848ee3",
    "wallet-broker/tests/zec_store.rs": "1c230a2a9cf51c841a0df6514393861387422e5d0b2a83e80af47022728e2225",
    "wallet-broker/tests/zec_hygiene.rs": "aad7c95a2ef661063661f2ec0f16a216d80328096b049d636b88fa0252ba1be6",
    "wallet-broker/tests/native_surface.rs": "349f3a019a0e7a5c37aaae727192b0ddcecb02192a5fae6df9291eaf1357276d",
    "wallet-broker/target/wal013-final-window-security-04.json": "45c1abe5ce23f32df5ac848c131f3c49ecb1c238f73604f2a704154eec4e20ed"
  },
  "formatted_hashes": {
    "wallet-broker/tests/account_management.rs": "817fa7972be4aebec3799a83ac998e572dd5b6f54c93683c5f16077069382e77",
    "wallet-broker/tests/account_native_ui.rs": "eaa1dbb0a9db410c41be7daaac6663dc07d369a6a0769b6460ecf7983382d560",
    "wallet-broker/src/accounts.rs": "5768cef932b8926bdfc80810d799e81e99ba63370ad8f06a48fb7b7563a566e1",
    "wallet-broker/src/session.rs": "9f88b3b0eca91d341a1c025bc8394a25e456350be58e99d9ac9417e31eef21af",
    "wallet-broker/src/zec.rs": "2544d86b3022aacea6e64f75ce37c38da0ce27cb6f555e085fc76450b5760d1e",
    "wallet-broker/src/account_ui.rs": "fc671ebf02ecb92ef28101540ad78fad15a1b326cd198e41b85ede91be52be99",
    "wallet-broker/src/lib.rs": "e37713c4fd1a528a9260b8b11d740f1a317198f2eaf62a534729f4a566a10437",
    "wallet-broker/src/vault.rs": "f23247b0e46c68dfc125804fa981d16f7022464e1e4d94a8c1e62771a045be49",
    "wallet-broker/src/store.rs": "611d837641069a98d05b9e68c14bf11a37a5076de58bf6516188870eeab19236",
    "wallet-broker/src/zec/store.rs": "531d0a6171ecd9b5012602ccb870f16eab96888c809b3ffdf4e6303576f17c90",
    "wallet-broker/src/zec/address.rs": "d9d54e301d188e3d4cb539095c5c01bcb07aaf011fcb1a50129aadc18f24cbfe",
    "wallet-broker/src/zec/scan.rs": "54a890fd0b5a8ca8ce78dfb59c13424407341df25e47a5eacd2de8d6cc3317ad",
    "wallet-broker/Cargo.toml": "73e5e585eb2fd1ca867d66962460cfa010443886a4b59d8a33556ef2a4ff3503",
    "wallet-broker/Cargo.lock": "b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71",
    "package.json": "76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5",
    "package-lock.json": "0e32de43f3c1a31ae3d9e9c8b172d10cd37a03577dc7d1eaf4ac916967cfcdb8",
    "scripts/security-policy.js": "fec10759a73a3c675d8206da454abf4b0e8dca6b62da463f39a2db27b30dc078",
    "test/securityPolicy.node.js": "a58d85e5322f0cb01a78b636454f92547bd98e556013cc07712dc0cf64ea565c",
    "wallet-broker/src/runtime.rs": "6788914524605f4669d20d65f7b6ef7282b0090a0696ea45f088ecc49dc1c962",
    "wallet-broker/supervisor.js": "115e0e51425e7b25d8f6178b206f592165cb14dacde8cb544b970839833c4721",
    "social-main.js": "d959e6adffc2579fa3f2f447f60a165f2b6bcdc279afde2f6ddaa91bc9d9aa4a",
    "scripts/build-wallet-broker.js": "4427ef3e713ec77c45daefe8d37b457ca8dc2c8529b30214dbf65628f43a1445",
    "test/walletAccountManagement.node.js": "6a449335e1101b8dd6340d640514e8fc083a09235210c17dd582b57230de9475",
    "test/walletStartup.node.js": "448fb88268655b979c7c724ab21d13d09fee29a219628cd70f576daae4589de1",
    "test/walletSupervisor.node.js": "7fcb775d2db7ae8f28b6d9f7660c44a056e59891f3e552db80a248d843c76aa4",
    "test/walletStartupSmoke.node.js": "a1c75efee5fcfdbeb9bf56dfc64f0e747ab6aa527d30ce416c5027bc2aa2a1ef",
    "test/walletAccountWindowSmoke.node.js": "3590096cafed5596378488e8ebca8c916e62352d4410e5748a2033ab0ec9fa3a",
    "test/fixtures/wallet-broker/x11-window.py": "d192f036781e5623b88a8ba3f31b9ee7c781f7bdffa46982aa431f8ada5330d3",
    "wallet-broker/tests/vault_session.rs": "67487db86d6788633e031418da71f6080409ac57a144ad362e311cb22519be6b",
    "wallet-broker/tests/zec_address.rs": "2c5012e6884c8c2a81236266c6861b6e4e4fd6b124656dad2ab438add5848ee3",
    "wallet-broker/tests/zec_store.rs": "1c230a2a9cf51c841a0df6514393861387422e5d0b2a83e80af47022728e2225",
    "wallet-broker/tests/zec_hygiene.rs": "aad7c95a2ef661063661f2ec0f16a216d80328096b049d636b88fa0252ba1be6",
    "wallet-broker/tests/native_surface.rs": "349f3a019a0e7a5c37aaae727192b0ddcecb02192a5fae6df9291eaf1357276d",
    "wallet-broker/target/wal013-final-window-security-04.json": "45c1abe5ce23f32df5ac848c131f3c49ecb1c238f73604f2a704154eec4e20ed"
  },
  "stop": "command failed: ['<home>/.cargo/bin/rustup', 'run', '1.98.0', 'cargo', 'clippy', '--manifest-path', 'wallet-broker/Cargo.toml', '--locked', '--offline', '--no-default-features', '--features', 'native-ui', '--lib', '--bin', 'bitbook-wallet-broker', '--test', 'account_management', '--test', 'account_native_ui', '--', '-D', 'warnings']",
  "final_hashes": {
    "wallet-broker/tests/account_management.rs": "817fa7972be4aebec3799a83ac998e572dd5b6f54c93683c5f16077069382e77",
    "wallet-broker/tests/account_native_ui.rs": "eaa1dbb0a9db410c41be7daaac6663dc07d369a6a0769b6460ecf7983382d560",
    "wallet-broker/src/accounts.rs": "5768cef932b8926bdfc80810d799e81e99ba63370ad8f06a48fb7b7563a566e1",
    "wallet-broker/src/session.rs": "9f88b3b0eca91d341a1c025bc8394a25e456350be58e99d9ac9417e31eef21af",
    "wallet-broker/src/zec.rs": "2544d86b3022aacea6e64f75ce37c38da0ce27cb6f555e085fc76450b5760d1e",
    "wallet-broker/src/account_ui.rs": "fc671ebf02ecb92ef28101540ad78fad15a1b326cd198e41b85ede91be52be99",
    "wallet-broker/src/lib.rs": "e37713c4fd1a528a9260b8b11d740f1a317198f2eaf62a534729f4a566a10437",
    "wallet-broker/src/vault.rs": "f23247b0e46c68dfc125804fa981d16f7022464e1e4d94a8c1e62771a045be49",
    "wallet-broker/src/store.rs": "611d837641069a98d05b9e68c14bf11a37a5076de58bf6516188870eeab19236",
    "wallet-broker/src/zec/store.rs": "531d0a6171ecd9b5012602ccb870f16eab96888c809b3ffdf4e6303576f17c90",
    "wallet-broker/src/zec/address.rs": "d9d54e301d188e3d4cb539095c5c01bcb07aaf011fcb1a50129aadc18f24cbfe",
    "wallet-broker/src/zec/scan.rs": "54a890fd0b5a8ca8ce78dfb59c13424407341df25e47a5eacd2de8d6cc3317ad",
    "wallet-broker/Cargo.toml": "73e5e585eb2fd1ca867d66962460cfa010443886a4b59d8a33556ef2a4ff3503",
    "wallet-broker/Cargo.lock": "b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71",
    "package.json": "76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5",
    "package-lock.json": "0e32de43f3c1a31ae3d9e9c8b172d10cd37a03577dc7d1eaf4ac916967cfcdb8",
    "scripts/security-policy.js": "fec10759a73a3c675d8206da454abf4b0e8dca6b62da463f39a2db27b30dc078",
    "test/securityPolicy.node.js": "a58d85e5322f0cb01a78b636454f92547bd98e556013cc07712dc0cf64ea565c",
    "wallet-broker/src/runtime.rs": "6788914524605f4669d20d65f7b6ef7282b0090a0696ea45f088ecc49dc1c962",
    "wallet-broker/supervisor.js": "115e0e51425e7b25d8f6178b206f592165cb14dacde8cb544b970839833c4721",
    "social-main.js": "d959e6adffc2579fa3f2f447f60a165f2b6bcdc279afde2f6ddaa91bc9d9aa4a",
    "scripts/build-wallet-broker.js": "4427ef3e713ec77c45daefe8d37b457ca8dc2c8529b30214dbf65628f43a1445",
    "test/walletAccountManagement.node.js": "6a449335e1101b8dd6340d640514e8fc083a09235210c17dd582b57230de9475",
    "test/walletStartup.node.js": "448fb88268655b979c7c724ab21d13d09fee29a219628cd70f576daae4589de1",
    "test/walletSupervisor.node.js": "7fcb775d2db7ae8f28b6d9f7660c44a056e59891f3e552db80a248d843c76aa4",
    "test/walletStartupSmoke.node.js": "a1c75efee5fcfdbeb9bf56dfc64f0e747ab6aa527d30ce416c5027bc2aa2a1ef",
    "test/walletAccountWindowSmoke.node.js": "3590096cafed5596378488e8ebca8c916e62352d4410e5748a2033ab0ec9fa3a",
    "test/fixtures/wallet-broker/x11-window.py": "d192f036781e5623b88a8ba3f31b9ee7c781f7bdffa46982aa431f8ada5330d3",
    "wallet-broker/tests/vault_session.rs": "67487db86d6788633e031418da71f6080409ac57a144ad362e311cb22519be6b",
    "wallet-broker/tests/zec_address.rs": "2c5012e6884c8c2a81236266c6861b6e4e4fd6b124656dad2ab438add5848ee3",
    "wallet-broker/tests/zec_store.rs": "1c230a2a9cf51c841a0df6514393861387422e5d0b2a83e80af47022728e2225",
    "wallet-broker/tests/zec_hygiene.rs": "aad7c95a2ef661063661f2ec0f16a216d80328096b049d636b88fa0252ba1be6",
    "wallet-broker/tests/native_surface.rs": "349f3a019a0e7a5c37aaae727192b0ddcecb02192a5fae6df9291eaf1357276d",
    "wallet-broker/target/wal013-final-window-security-04.json": "45c1abe5ce23f32df5ac848c131f3c49ecb1c238f73604f2a704154eec4e20ed"
  }
}
```
