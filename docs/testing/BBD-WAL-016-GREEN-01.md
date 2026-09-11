# WAL016 manager, native UI, session and production lint verification

```json
{
  "accepted": true,
  "commands": [
    {
      "argv": [
        "hermes",
        "--version"
      ],
      "exit": 0,
      "output": "Hermes Agent v0.18.2 (2026.7.7.2) \u00b7 upstream 564aef29 \u00b7 local 10b6d1a9 (+1 carried commit)\nInstall directory: <home>/.hermes/hermes-agent\nInstall method: git\nPython: 3.11.15\nOpenAI SDK: 2.24.0\n",
      "timeout": false
    },
    {
      "argv": [
        "git",
        "rev-parse",
        "HEAD"
      ],
      "exit": 0,
      "output": "4cae91cb59635213ceecbae075c482ad8949aab2\n",
      "timeout": false
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
        "account_management"
      ],
      "exit": 0,
      "output": "   Compiling bitbook-wallet-broker v0.1.0 (<repo>/wallet-broker)\n    Finished `test` profile [unoptimized + debuginfo] target(s) in 8.24s\n     Running tests/account_management.rs (wallet-broker/target/debug/deps/account_management-c68a82dfe14ae99c)\n\nrunning 20 tests\ntest hostile_roots_and_catalog_entries_fail_closed_without_chmod ... ok\ntest second_manager_same_root_is_unavailable_until_first_drops ... ok\ntest entropy_failure_and_duplicate_id_never_overwrite_existing_vault ... ok\ntest catalog_cap_is_256_without_running_per_account_kdf ... ok\ntest locked_export_copies_exact_encrypted_bytes_and_refuses_existing_or_link ... ok\ntest restart_lists_persisted_locked_identities_without_secret_canaries ... ok\ntest wal014_foreign_ufvk_binding_refuses_without_advancing_issuance ... ok\ntest create_software_persists_locked_encrypted_account_with_scripted_seed ... ok\ntest restore_cancel_is_a_noop_and_confirm_commits_captured_not_replaced_bytes ... ok\ntest wal014_corrupt_viewing_db_and_symlink_directory_are_never_replaced ... ok\ntest nonnative_origins_refuse_privileged_apis_before_side_effects ... ok\ntest wal014_receiver_matches_seed_oracle_is_durable_private_and_preserves_vault ... ok\ntest wal014_receive_is_native_unlocked_valid_and_does_not_extend_idle_deadline ... ok\ntest wal015_live_cache_binding_failure_preserves_vault_receive_state_and_receiver_continuity ... ok\ntest wal015_sync_is_native_unlocked_bound_and_gated_before_source_access ... ok\ntest list_does_not_extend_idle_deadline_and_backwards_clock_locks_all ... ok\ntest restore_rejects_hostile_inputs_and_never_overwrites_existing_id ... ok\ntest wal015_one_worker_lock_expiry_and_drop_cancel_and_join_without_holding_manager ... ok\ntest wal016_idle_expiry_redacts_a_returned_copy_while_the_real_worker_finishes_and_unlock_reveals_it ... ok\ntest unlock_rejects_wrong_corrupt_and_short_material_then_valid_lock_cycle_works ... ok\n\ntest result: ok. 20 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 31.55s\n\n",
      "timeout": false
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
      "exit": 0,
      "output": "   Compiling bitbook-wallet-broker v0.1.0 (<repo>/wallet-broker)\n    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.33s\n     Running tests/account_native_ui.rs (wallet-broker/target/debug/deps/account_native_ui-e7381b3c5844547a)\n\nrunning 15 tests\ntest unlock_wrong_then_correct_clears_password_and_lock_calls ... ok\ntest empty_and_populated_list_paints_notices_and_selection_controls ... ok\ntest wal015_cancel_stale_job_lock_back_and_hide_invalidate_sync_results ... ok\ntest export_chooser_cancel_skips_port_and_chosen_path_reaches_port ... ok\ntest create_form_accepts_matching_unicode_and_rejects_invalid ... ok\ntest small_and_tall_layouts_keep_action_controls_usable ... ok\ntest copy_cut_undo_and_platform_events_never_expose_secret ... ok\ntest restore_requires_distinct_confirm_and_cannot_replay ... ok\ntest wal015_sync_pointer_flow_is_explicit_editable_and_paints_progress_then_received_funds ... ok\ntest wal016_idle_locked_sync_stays_visible_and_inline_unlock_reveals_the_same_completed_job ... ok\ntest window_control_reopen_quit_drop_and_closed_list_errors ... ok\ntest wal014_receive_scene_issues_once_paints_honestly_and_copies_release_output ... ok\ntest wal014_receive_state_clears_on_lock_catalog_change_error_selection_and_hide ... ok\ntest shared_port_and_local_manager_create_unlock_lock_and_restart ... ok\ntest wal014_shared_port_pointer_receive_and_copy_persist_real_issuance ... ok\n\ntest result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 7.19s\n\n",
      "timeout": false
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
        "vault_session"
      ],
      "exit": 0,
      "output": "   Compiling bitbook-wallet-broker v0.1.0 (<repo>/wallet-broker)\n    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.80s\n     Running tests/vault_session.rs (wallet-broker/target/debug/deps/vault_session-bce31faa69f47e02)\n\nrunning 13 tests\ntest backward_clock_fails_all_sessions_locked ... ok\ntest every_forced_lock_event_wipes_spend_material ... ok\ntest clock_failure_during_unlock_explicitly_wipes_supplied_material ... ok\ntest global_lock_events_ignore_malformed_account_but_scoped_events_reject_it ... ok\ntest late_native_authorization_at_existing_deadline_times_out_and_wipes ... ok\ntest idle_timeout_is_exactly_fifteen_minutes_and_locks_at_boundary ... ok\ntest invalid_account_unlock_is_schema_and_wipes_supplied_material ... ok\ntest monotonic_clock_error_fails_locked ... ok\ntest only_successful_native_authorization_resets_idle_deadline ... ok\ntest overflowing_deadline_fails_locked_instead_of_wrapping ... ok\ntest separate_accounts_have_isolated_deadlines_and_wipes ... ok\ntest polling_sync_backup_browsing_and_failed_or_cancelled_prompts_never_extend ... ok\ntest status_and_account_listing_never_request_spend_secret ... ok\n\ntest result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s\n\n",
      "timeout": false
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
        "--",
        "-D",
        "warnings"
      ],
      "exit": 0,
      "output": "    Checking bitbook-wallet-broker v0.1.0 (<repo>/wallet-broker)\n    Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.23s\n",
      "timeout": false
    }
  ],
  "session": {
    "id": "20260910_194204_94161a",
    "model": "meituan/longcat-2.0:free",
    "provider": "nous"
  },
  "version": "Hermes Agent v0.18.2 (2026.7.7.2) \u00b7 upstream 564aef29 \u00b7 local 10b6d1a9 (+1 carried commit)\nInstall directory: <home>/.hermes/hermes-agent\nInstall method: git\nPython: 3.11.15\nOpenAI SDK: 2.24.0",
  "head": "4cae91cb59635213ceecbae075c482ad8949aab2",
  "input_hashes": {
    "wallet-broker/src/accounts.rs": "8941884f837453bf71357f53341fae2c1334b8d10d017680053f9f8a44fb415d",
    "wallet-broker/src/account_ui.rs": "40eb0d8946901bd69d1e71ad894cfdf361490210b9c4f476359acfbada25d413",
    "wallet-broker/src/zec/test_support.rs": "088dba916fe6f91d9a543bf5c733f502394e15886029a1c4749444b1fbef9771",
    "wallet-broker/src/session.rs": "9f88b3b0eca91d341a1c025bc8394a25e456350be58e99d9ac9417e31eef21af",
    "wallet-broker/Cargo.toml": "435907f00d7819bdf7dbe9ff9e367522f7f6eb8b6019dbde9e58212bbaea041b",
    "wallet-broker/Cargo.lock": "a5d53b729433a06f60edcef51e608629c7365326d06fef2bbac8f2b23f048c2a",
    "wallet-broker/tests/account_management.rs": "c3520f0ec10f9fffd2366c99de06f078edf9ffd11b9e1cee0b81ecafe652dd0e",
    "wallet-broker/tests/account_native_ui.rs": "a6719c17ca25e00f4a4fc217aadc97bcc49800b423b8a5b13e2e90598c393797",
    "package.json": "76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5",
    "package-lock.json": "0e32de43f3c1a31ae3d9e9c8b172d10cd37a03577dc7d1eaf4ac916967cfcdb8",
    "scripts/security-policy.js": "414a5645d8621b5d81cdeba7bb7461ec25647a5a1e5532295d6f75e7b8762532",
    "test/securityPolicy.node.js": "ff449a86e04570f8ff07bd280db6787c62f645f301d4235a65e825f3af7bbb9c"
  },
  "formatted": [
    "wallet-broker/src/accounts.rs",
    "wallet-broker/src/account_ui.rs",
    "wallet-broker/src/zec/test_support.rs",
    "wallet-broker/tests/account_management.rs",
    "wallet-broker/tests/account_native_ui.rs"
  ],
  "final_hashes": {
    "wallet-broker/src/accounts.rs": "8941884f837453bf71357f53341fae2c1334b8d10d017680053f9f8a44fb415d",
    "wallet-broker/src/account_ui.rs": "67bf08a6eb809796349f225cb551f342e20be98b2a7eb95570dada6f5fa2ff44",
    "wallet-broker/src/zec/test_support.rs": "de157d4a78c4c452e0e19122200997e6676c82099ddb25bfebe84781470f0791",
    "wallet-broker/src/session.rs": "9f88b3b0eca91d341a1c025bc8394a25e456350be58e99d9ac9417e31eef21af",
    "wallet-broker/Cargo.toml": "435907f00d7819bdf7dbe9ff9e367522f7f6eb8b6019dbde9e58212bbaea041b",
    "wallet-broker/Cargo.lock": "a5d53b729433a06f60edcef51e608629c7365326d06fef2bbac8f2b23f048c2a",
    "wallet-broker/tests/account_management.rs": "f1f9c70dc89e330fca6f5d292cfbabd626ef0c760a8800f522b48c070e57ddf0",
    "wallet-broker/tests/account_native_ui.rs": "a6719c17ca25e00f4a4fc217aadc97bcc49800b423b8a5b13e2e90598c393797",
    "package.json": "76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5",
    "package-lock.json": "0e32de43f3c1a31ae3d9e9c8b172d10cd37a03577dc7d1eaf4ac916967cfcdb8",
    "scripts/security-policy.js": "414a5645d8621b5d81cdeba7bb7461ec25647a5a1e5532295d6f75e7b8762532",
    "test/securityPolicy.node.js": "ff449a86e04570f8ff07bd280db6787c62f645f301d4235a65e825f3af7bbb9c"
  }
}
```
