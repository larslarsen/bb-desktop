# WAL-015 live sync initial expected red

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
      "output": "Hermes Agent v0.18.2 (2026.7.7.2) \u00b7 upstream 564aef29 \u00b7 local 10b6d1a9 (+1 carried commit)\nInstall directory: <home>/.hermes/hermes-agent\nInstall method: git\nPython: 3.11.15\nOpenAI SDK: 2.24.0\n"
    },
    {
      "argv": [
        "git",
        "rev-parse",
        "HEAD"
      ],
      "exit": 0,
      "output": "e85dfdc8b6e06fdd09f84cee3c6ac37037448437\n"
    },
    {
      "argv": [
        "git",
        "status",
        "--short"
      ],
      "exit": 0,
      "output": " M docs/architecture/BBD-WAL-001-REVIEW.md\n M docs/handoff/CURRENT_TASK.md\n M package-lock.json\n M package.json\n M scripts/security-policy.js\n M test/securityPolicy.node.js\n M wallet-broker/tests/account_management.rs\n M wallet-broker/tests/account_native_ui.rs\n?? docs/architecture/BBD-WAL-015-LIVE-SYNC-REVIEW.md\n?? docs/handoff/HERMES_BBD_WAL_015_RED_01.md\n?? docs/handoff/SOL_BBD_WAL_015_TESTS_01.md\n?? docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md\n?? docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md\n?? tickets/BBD-WAL-015.md\n?? wallet-broker/tests/zec_live_sync.rs\n"
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
        "zec_live_sync"
      ],
      "exit": 101,
      "output": "   Compiling bitbook-wallet-broker v0.1.0 (<repo>/wallet-broker)\nerror[E0432]: unresolved imports `bitbook_wallet_broker::zec::test_support::LiveEngineFault`, `bitbook_wallet_broker::zec::test_support::LiveSourceFault`, `bitbook_wallet_broker::zec::test_support::LiveStoreEntryKind`, `bitbook_wallet_broker::zec::test_support::LiveSyncHarness`, `bitbook_wallet_broker::zec::test_support::RecordedLiveSource`, `bitbook_wallet_broker::zec::test_support::validate_live_batch_shape_for_test`\n --> tests/zec_live_sync.rs:5:20\n  |\n5 |     FrozenFixture, LiveEngineFault, LiveSourceFault, LiveStoreEntryKind, LiveSyncHarness,\n  |                    ^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^ no `LiveSyncHarness` in `zec::test_support`\n  |                    |                |                |\n  |                    |                |                no `LiveStoreEntryKind` in `zec::test_support`\n  |                    |                no `LiveSourceFault` in `zec::test_support`\n  |                    no `LiveEngineFault` in `zec::test_support`\n6 |     RecordedLiveSource, validate_live_batch_shape_for_test,\n  |     ^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `validate_live_batch_shape_for_test` in `zec::test_support`\n  |     |\n  |     no `RecordedLiveSource` in `zec::test_support`\n  |\nhelp: a similar name exists in the module\n  |\n5 -     FrozenFixture, LiveEngineFault, LiveSourceFault, LiveStoreEntryKind, LiveSyncHarness,\n5 +     FrozenFixture, PipelineFault, LiveSourceFault, LiveStoreEntryKind, LiveSyncHarness,\n  |\nhelp: a similar name exists in the module\n  |\n5 -     FrozenFixture, LiveEngineFault, LiveSourceFault, LiveStoreEntryKind, LiveSyncHarness,\n5 +     FrozenFixture, LiveEngineFault, LiveSourceFault, StoreEntryKind, LiveSyncHarness,\n  |\n\nerror[E0432]: unresolved imports `bitbook_wallet_broker::zec::LiveCancellation`, `bitbook_wallet_broker::zec::LiveEndpoint`, `bitbook_wallet_broker::zec::LiveSyncOptions`, `bitbook_wallet_broker::zec::LiveSyncPhase`, `bitbook_wallet_broker::zec::MAX_LIVE_BATCH_BYTES`, `bitbook_wallet_broker::zec::MAX_LIVE_BATCH_BLOCKS`, `bitbook_wallet_broker::zec::MAX_LIVE_ENDPOINT_BYTES`\n  --> tests/zec_live_sync.rs:9:16\n   |\n 9 |     AccountId, LiveCancellation, LiveEndpoint, LiveSyncOptions, LiveSyncPhase,\n   |                ^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^  ^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^ no `LiveSyncPhase` in `zec`\n   |                |                 |             |\n   |                |                 |             no `LiveSyncOptions` in `zec`\n   |                |                 no `LiveEndpoint` in `zec`\n   |                no `LiveCancellation` in `zec`\n10 |     Network, MAX_COMPACT_BLOCK_BYTES, MAX_LIVE_BATCH_BYTES, MAX_LIVE_BATCH_BLOCKS,\n   |                                       ^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^ no `MAX_LIVE_BATCH_BLOCKS` in `zec`\n   |                                       |\n   |                                       no `MAX_LIVE_BATCH_BYTES` in `zec`\n11 |     MAX_LIVE_ENDPOINT_BYTES,\n   |     ^^^^^^^^^^^^^^^^^^^^^^^ no `MAX_LIVE_ENDPOINT_BYTES` in `zec`\n\nFor more information about this error, try `rustc --explain E0432`.\nerror: could not compile `bitbook-wallet-broker` (test \"zec_live_sync\") due to 2 previous errors\nwarning: build failed, waiting for other jobs to finish...\n"
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
        "wal015_"
      ],
      "exit": 101,
      "output": "   Compiling bitbook-wallet-broker v0.1.0 (<repo>/wallet-broker)\nerror[E0432]: unresolved imports `bitbook_wallet_broker::zec::test_support::RecordedLiveSource`, `bitbook_wallet_broker::zec::test_support::live_job_id`, `bitbook_wallet_broker::zec::test_support::replace_live_ufvk_for_test`\n  --> tests/account_management.rs:20:5\n   |\n20 |     RecordedLiveSource, decode_unified_address, live_job_id, replace_live_ufvk_for_test,\n   |     ^^^^^^^^^^^^^^^^^^                          ^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^^^^^^ no `replace_live_ufvk_for_test` in `zec::test_support`\n   |     |                                           |\n   |     |                                           no `live_job_id` in `zec::test_support`\n   |     no `RecordedLiveSource` in `zec::test_support`\n\nerror[E0432]: unresolved imports `bitbook_wallet_broker::zec::LiveEndpoint`, `bitbook_wallet_broker::zec::LiveSyncPhase`\n  --> tests/account_management.rs:22:51\n   |\n22 | use bitbook_wallet_broker::zec::{FreshReceiverV1, LiveEndpoint, LiveSyncPhase, Network as ZecNetwork};\n   |                                                   ^^^^^^^^^^^^  ^^^^^^^^^^^^^ no `LiveSyncPhase` in `zec`\n   |                                                   |\n   |                                                   no `LiveEndpoint` in `zec`\n\nerror[E0425]: cannot find type `LiveSourceProbe` in module `bitbook_wallet_broker::zec::test_support`\n   --> tests/account_management.rs:679:47\n    |\n679 |     bitbook_wallet_broker::zec::test_support::LiveSourceProbe,\n    |                                               ^^^^^^^^^^^^^^^ not found in `bitbook_wallet_broker::zec::test_support`\n\nerror[E0599]: no method named `start_live_sync_with_source_for_test` found for struct `AccountManager<C, E, W>` in the current scope\n    --> tests/account_management.rs:1938:18\n     |\n1937 |               &manager\n     |  ______________-\n1938 | |                 .start_live_sync_with_source_for_test(\n     | |                 -^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ method not found in `AccountManager<SharedClock, ScriptedEntropy, SharedWipes>`\n     | |_________________|\n     |\n\nerror[E0599]: no method named `start_live_sync_with_source_for_test` found for struct `AccountManager<C, E, W>` in the current scope\n    --> tests/account_management.rs:1954:14\n     |\n1953 |           &manager\n     |  __________-\n1954 | |             .start_live_sync_with_source_for_test(\n     | |             -^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ method not found in `AccountManager<SharedClock, ScriptedEntropy, SharedWipes>`\n     | |_____________|\n     |\n\nerror[E0599]: no method named `start_live_sync_with_source_for_test` found for struct `AccountManager<C, E, W>` in the current scope\n    --> tests/account_management.rs:1968:14\n     |\n1967 |           &manager\n     |  __________-\n1968 | |             .start_live_sync_with_source_for_test(\n     | |             -^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ method not found in `AccountManager<SharedClock, ScriptedEntropy, SharedWipes>`\n     | |_____________|\n     |\n\nerror[E0599]: no method named `start_live_sync_with_source_for_test` found for struct `AccountManager<C, E, W>` in the current scope\n    --> tests/account_management.rs:1985:10\n     |\n1984 |       let job = manager\n     |  _______________-\n1985 | |         .start_live_sync_with_source_for_test(\n     | |         -^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ method not found in `AccountManager<SharedClock, ScriptedEntropy, SharedWipes>`\n     | |_________|\n     |\n\nerror[E0599]: no method named `live_sync_status` found for struct `AccountManager<C, E, W>` in the current scope\n    --> tests/account_management.rs:1993:26\n     |\n1993 |     let status = manager.live_sync_status(&account_id).unwrap();\n     |                          ^^^^^^^^^^^^^^^^ method not found in `AccountManager<SharedClock, ScriptedEntropy, SharedWipes>`\n\nerror[E0599]: no method named `cancel_live_sync` found for struct `AccountManager<C, E, W>` in the current scope\n    --> tests/account_management.rs:2016:10\n     |\n2015 | /     manager\n2016 | |         .cancel_live_sync(ActionOrigin::NativeSurface, &account_id, job)\n     | |         -^^^^^^^^^^^^^^^^ method not found in `AccountManager<SharedClock, ScriptedEntropy, SharedWipes>`\n     | |_________|\n     |\n\nerror[E0599]: no method named `start_live_sync_with_source_for_test` found for struct `AccountManager<C, E, W>` in the current scope\n    --> tests/account_management.rs:2039:10\n     |\n2038 |       let first_job = manager\n     |  _____________________-\n2039 | |         .start_live_sync_with_source_for_test(\n     | |         -^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ method not found in `AccountManager<SharedClock, ScriptedEntropy, SharedWipes>`\n     | |_________|\n     |\n\nerror[E0599]: no method named `start_live_sync_with_source_for_test` found for struct `AccountManager<C, E, W>` in the current scope\n    --> tests/account_management.rs:2051:14\n     |\n2050 |           &manager\n     |  __________-\n2051 | |             .start_live_sync_with_source_for_test(\n     | |             -^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ method not found in `AccountManager<SharedClock, ScriptedEntropy, SharedWipes>`\n     | |_____________|\n     |\n\nerror[E0599]: no method named `cancel_live_sync` found for struct `AccountManager<C, E, W>` in the current scope\n    --> tests/account_management.rs:2063:14\n     |\n2062 |           &manager\n     |  __________-\n2063 | |             .cancel_live_sync(ActionOrigin::NativeSurface, &account_id, live_job_id(0))\n     | |             -^^^^^^^^^^^^^^^^ method not found in `AccountManager<SharedClock, ScriptedEntropy, SharedWipes>`\n     | |_____________|\n     |\n\nerror[E0599]: no method named `live_sync_status` found for struct `AccountManager<C, E, W>` in the current scope\n    --> tests/account_management.rs:2067:24\n     |\n2067 |     assert_eq!(manager.live_sync_status(&account_id).unwrap().job_id, Some(first_job));\n     |                        ^^^^^^^^^^^^^^^^ method not found in `AccountManager<SharedClock, ScriptedEntropy, SharedWipes>`\n\nerror[E0599]: no method named `live_sync_status` found for struct `AccountManager<C, E, W>` in the current scope\n    --> tests/account_management.rs:2071:29\n     |\n2071 |     let cancelled = manager.live_sync_status(&account_id).unwrap();\n     |                             ^^^^^^^^^^^^^^^^ method not found in `AccountManager<SharedClock, ScriptedEntropy, SharedWipes>`\n\nerror[E0599]: no method named `start_live_sync_with_source_for_test` found for struct `AccountManager<C, E, W>` in the current scope\n    --> tests/account_management.rs:2080:10\n     |\n2079 | /     manager\n2080 | |         .start_live_sync_with_source_for_test(\n     | |         -^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ method not found in `AccountManager<SharedClock, ScriptedEntropy, SharedWipes>`\n     | |_________|\n     |\n\nerror[E0599]: no method named `start_live_sync_with_source_for_test` found for struct `AccountManager<C, E, W>` in the current scope\n    --> tests/account_management.rs:2100:10\n     |\n2099 | /     manager\n2100 | |         .start_live_sync_with_source_for_test(\n     | |         -^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ method not found in `AccountManager<SharedClock, ScriptedEntropy, SharedWipes>`\n     | |_________|\n     |\n\nerror[E0599]: no method named `start_live_sync_with_source_for_test` found for struct `AccountManager<C, E, W>` in the current scope\n    --> tests/account_management.rs:2150:10\n     |\n2149 |       let job = manager\n     |  _______________-\n2150 | |         .start_live_sync_with_source_for_test(\n     | |         -^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ method not found in `AccountManager<SharedClock, ScriptedEntropy, SharedWipes>`\n     | |_________|\n     |\n\nerror[E0599]: no method named `cancel_live_sync` found for struct `AccountManager<C, E, W>` in the current scope\n    --> tests/account_management.rs:2161:10\n     |\n2160 | /     manager\n2161 | |         .cancel_live_sync(ActionOrigin::NativeSurface, &account_id, job)\n     | |         -^^^^^^^^^^^^^^^^ method not found in `AccountManager<SharedClock, ScriptedEntropy, SharedWipes>`\n     | |_________|\n     |\n\nerror[E0599]: no method named `start_live_sync_with_source_for_test` found for struct `AccountManager<C, E, W>` in the current scope\n    --> tests/account_management.rs:2173:14\n     |\n2172 |           &manager\n     |  __________-\n2173 | |             .start_live_sync_with_source_for_test(\n     | |             -^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ method not found in `AccountManager<SharedClock, ScriptedEntropy, SharedWipes>`\n     | |_____________|\n     |\n\nSome errors have detailed explanations: E0425, E0432, E0599.\nFor more information about an error, try `rustc --explain E0425`.\nerror: could not compile `bitbook-wallet-broker` (test \"account_management\") due to 19 previous errors\n"
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
        "wal015_"
      ],
      "exit": 101,
      "output": "   Compiling bitbook-wallet-broker v0.1.0 (<repo>/wallet-broker)\nerror[E0432]: unresolved import `bitbook_wallet_broker::zec::test_support::live_job_id`\n  --> tests/account_native_ui.rs:21:72\n   |\n21 | use bitbook_wallet_broker::zec::test_support::{decode_unified_address, live_job_id};\n   |                                                                        ^^^^^^^^^^^ no `live_job_id` in `zec::test_support`\n\nerror[E0432]: unresolved imports `bitbook_wallet_broker::zec::LiveSyncJobId`, `bitbook_wallet_broker::zec::LiveSyncPhase`, `bitbook_wallet_broker::zec::LiveSyncSnapshot`\n  --> tests/account_native_ui.rs:23:33\n   |\n23 |     AccountId, FreshReceiverV1, LiveSyncJobId, LiveSyncPhase, LiveSyncSnapshot,\n   |                                 ^^^^^^^^^^^^^  ^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^ no `LiveSyncSnapshot` in `zec`\n   |                                 |              |\n   |                                 |              no `LiveSyncPhase` in `zec`\n   |                                 no `LiveSyncJobId` in `zec`\n\nerror[E0407]: method `start_sync` is not a member of trait `AccountUiPort`\n   --> tests/account_native_ui.rs:550:5\n    |\n550 | /     fn start_sync(\n551 | |         &mut self,\n552 | |         id: &str,\n553 | |         endpoint: &str,\n...   |\n562 | |             .unwrap_or(Err(\"UNAVAILABLE\"))\n563 | |     }\n    | |_____^ not a member of trait `AccountUiPort`\n\nerror[E0407]: method `sync_status` is not a member of trait `AccountUiPort`\n   --> tests/account_native_ui.rs:565:5\n    |\n565 | /     fn sync_status(&mut self, id: &str) -> Result<LiveSyncSnapshot, &'static str> {\n566 | |         let mut state = self.inner.borrow_mut();\n567 | |         state.sync_status_calls.push(id.to_owned());\n568 | |         if let Some(error) = state.sync_status_error {\n...   |\n574 | |             .unwrap_or_else(|| sync_snapshot(id, None, LiveSyncPhase::Unsynced, None, None)))\n575 | |     }\n    | |_____^ not a member of trait `AccountUiPort`\n\nerror[E0407]: method `cancel_sync` is not a member of trait `AccountUiPort`\n   --> tests/account_native_ui.rs:577:5\n    |\n577 | /     fn cancel_sync(\n578 | |         &mut self,\n579 | |         id: &str,\n580 | |         job: LiveSyncJobId,\n...   |\n588 | |     }\n    | |_____^ not a member of trait `AccountUiPort`\n\nSome errors have detailed explanations: E0407, E0432.\nFor more information about an error, try `rustc --explain E0407`.\nerror: could not compile `bitbook-wallet-broker` (test \"account_native_ui\") due to 5 previous errors\n"
    }
  ],
  "accepted": true,
  "version": "Hermes Agent v0.18.2 (2026.7.7.2) \u00b7 upstream 564aef29 \u00b7 local 10b6d1a9 (+1 carried commit)\nInstall directory: <home>/.hermes/hermes-agent\nInstall method: git\nPython: 3.11.15\nOpenAI SDK: 2.24.0",
  "head": "e85dfdc8b6e06fdd09f84cee3c6ac37037448437",
  "initial_status": "M docs/architecture/BBD-WAL-001-REVIEW.md\n M docs/handoff/CURRENT_TASK.md\n M package-lock.json\n M package.json\n M scripts/security-policy.js\n M test/securityPolicy.node.js\n M wallet-broker/tests/account_management.rs\n M wallet-broker/tests/account_native_ui.rs\n?? docs/architecture/BBD-WAL-015-LIVE-SYNC-REVIEW.md\n?? docs/handoff/HERMES_BBD_WAL_015_RED_01.md\n?? docs/handoff/SOL_BBD_WAL_015_TESTS_01.md\n?? docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md\n?? docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md\n?? tickets/BBD-WAL-015.md\n?? wallet-broker/tests/zec_live_sync.rs",
  "session": {
    "id": "20260910_143940_a2efb3",
    "model": "meituan/longcat-2.0:free",
    "provider": "nous"
  },
  "input_hashes": {
    "wallet-broker/src/account_ui.rs": "fc671ebf02ecb92ef28101540ad78fad15a1b326cd198e41b85ede91be52be99",
    "wallet-broker/src/accounts.rs": "5768cef932b8926bdfc80810d799e81e99ba63370ad8f06a48fb7b7563a566e1",
    "wallet-broker/src/hygiene.rs": "7676aaad8ed78fb01fdb3cf2a763fd057693f5fe6f2721b385c3c8dd6d39bdbf",
    "wallet-broker/src/lib.rs": "e37713c4fd1a528a9260b8b11d740f1a317198f2eaf62a534729f4a566a10437",
    "wallet-broker/src/main.rs": "19b1651d88d85e5d968597eb26eeb62f76a11af56eaa311d60a55d3ef282736d",
    "wallet-broker/src/native.rs": "a0ccb0b11e3e636cc28f9576ec0cf6da2d8c2ef5344370e5a7a2a9a9b5cc32e5",
    "wallet-broker/src/native_ui.rs": "c6607af3f36278f8d7deeb2b4276017177bbb4c695017f18bf4d2bcb412b268f",
    "wallet-broker/src/native_ui/zec_native_app_tests.rs": "d1d405f8667c6c9d184104d2023ffcff209f6d013e3920dd5c5ccd7e5a2c40f0",
    "wallet-broker/src/native_ui/zec_review_tests.rs": "2eec3b60eb1b9fbd32235832a867d1189928126ca79373fa9f4fa7fdf2ee87bf",
    "wallet-broker/src/runtime.rs": "6788914524605f4669d20d65f7b6ef7282b0090a0696ea45f088ecc49dc1c962",
    "wallet-broker/src/session.rs": "9f88b3b0eca91d341a1c025bc8394a25e456350be58e99d9ac9417e31eef21af",
    "wallet-broker/src/store.rs": "611d837641069a98d05b9e68c14bf11a37a5076de58bf6516188870eeab19236",
    "wallet-broker/src/vault.rs": "f23247b0e46c68dfc125804fa981d16f7022464e1e4d94a8c1e62771a045be49",
    "wallet-broker/src/xmr.rs": "78107f241bb4cb8f02ab4168cbc81a01fc90cc75c80328a2677f819d7c06adce",
    "wallet-broker/src/xmr/account.rs": "5dcad3d450a2e5d8d780e7e490111c33ba06da6275d7d1ca84e5f76dde09cddb",
    "wallet-broker/src/xmr/distribution.rs": "163f8532bc7edfd80fc07966c0f8f32eebc0d12181fd273bc4e6c2870d86dea8",
    "wallet-broker/src/xmr/model.rs": "2a2d3ba1ce453aca65138df402bde3e7f1fee997d5d069024cb1beb8102152cb",
    "wallet-broker/src/xmr/process.rs": "66f0aae7fd0b507cbadc27628d0b1c26ee0033d90891c294721c11a00be9dd2d",
    "wallet-broker/src/xmr/receiver.rs": "daece8857b74eb7f369e0dfad7607dc418d397338cb311367448a632383df2b9",
    "wallet-broker/src/xmr/rpc.rs": "1bbfdf3ec58f89728b2eb169e9d49c53512eb3b108e5c17f7b02bf2634fada33",
    "wallet-broker/src/xmr/store.rs": "3a7f4d5b8cc7b33e3596910ce0b9b10d2f760f24c3ccff98fd2941c410ee2df4",
    "wallet-broker/src/xmr/test_support.rs": "18e6d410b0b5186d45db82105229c8473ce10cfa39a5a54e57a6bc7d0714c2fc",
    "wallet-broker/src/zec.rs": "2544d86b3022aacea6e64f75ce37c38da0ce27cb6f555e085fc76450b5760d1e",
    "wallet-broker/src/zec/address.rs": "d9d54e301d188e3d4cb539095c5c01bcb07aaf011fcb1a50129aadc18f24cbfe",
    "wallet-broker/src/zec/fixture.rs": "318820c6f125f2318ba0caf5ae44835b36b0f67856fc66b83401961f0c301b36",
    "wallet-broker/src/zec/hardware.rs": "9546188299a2b7225b65820f3022fbaa85c1b66acc5266c0c37cc858ae910760",
    "wallet-broker/src/zec/prepare.rs": "365ce7cc75219d616098512af900c6c34d0fd1784e844926be5946975a4e3468",
    "wallet-broker/src/zec/scan.rs": "54a890fd0b5a8ca8ce78dfb59c13424407341df25e47a5eacd2de8d6cc3317ad",
    "wallet-broker/src/zec/spend.rs": "086052983c0fae8bffb9461a599df809c92e0fe5b94adde413f2df07f1255b85",
    "wallet-broker/src/zec/spend/cleanup_lifecycle_tests.rs": "60d2a67a7c7fff912b0df4ec16bcbe251b276b6e990c33f5433a1492539b2f66",
    "wallet-broker/src/zec/spend/effects.rs": "54d1a2959f15f881a9e2bc56261b5090cad89aae06b97d7ace7747f2449a2a72",
    "wallet-broker/src/zec/spend/external_binding_tests.rs": "bb2d6873dd7262ad6e30f117f1947b7d4cf7685ab04e32a5b700d7cb5b112b6d",
    "wallet-broker/src/zec/spend/verification_context_tests.rs": "25db22a3273aa2fcc3da38055726ebe834cabea05183b8fbe9ca9c47f5ae7ee0",
    "wallet-broker/src/zec/store.rs": "531d0a6171ecd9b5012602ccb870f16eab96888c809b3ffdf4e6303576f17c90",
    "wallet-broker/src/zec/test_support.rs": "bd005c21c7a601fe9edd2357525d4f1e8eb29d0b799e57a672433b591581f12d",
    "wallet-broker/src/zec/test_support/cleanup_lifecycle_tests.rs": "5b01727fdc2d864e0ec31693a58ad7f25041891f5cab1bc091ec0b8743323cca",
    "wallet-broker/tests/zec_live_sync.rs": "ba5abca785c2fa40139afae74948174b1fdcdbd94c2301ed7f9280f03e9f99bf",
    "wallet-broker/tests/account_management.rs": "22a31f33cc589c99514b2e215596c5f843c02bfe28866eb6d5ae286743fa5773",
    "wallet-broker/tests/account_native_ui.rs": "ecdebfa97e0685df590cf69bbfcca5a04c6313da087d1190744667933aac98e6",
    "wallet-broker/Cargo.toml": "73e5e585eb2fd1ca867d66962460cfa010443886a4b59d8a33556ef2a4ff3503",
    "wallet-broker/Cargo.lock": "b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71",
    "package.json": "76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5",
    "package-lock.json": "0e32de43f3c1a31ae3d9e9c8b172d10cd37a03577dc7d1eaf4ac916967cfcdb8",
    "scripts/security-policy.js": "fec10759a73a3c675d8206da454abf4b0e8dca6b62da463f39a2db27b30dc078",
    "test/securityPolicy.node.js": "a58d85e5322f0cb01a78b636454f92547bd98e556013cc07712dc0cf64ea565c"
  },
  "final_hashes": {
    "wallet-broker/src/account_ui.rs": "fc671ebf02ecb92ef28101540ad78fad15a1b326cd198e41b85ede91be52be99",
    "wallet-broker/src/accounts.rs": "5768cef932b8926bdfc80810d799e81e99ba63370ad8f06a48fb7b7563a566e1",
    "wallet-broker/src/hygiene.rs": "7676aaad8ed78fb01fdb3cf2a763fd057693f5fe6f2721b385c3c8dd6d39bdbf",
    "wallet-broker/src/lib.rs": "e37713c4fd1a528a9260b8b11d740f1a317198f2eaf62a534729f4a566a10437",
    "wallet-broker/src/main.rs": "19b1651d88d85e5d968597eb26eeb62f76a11af56eaa311d60a55d3ef282736d",
    "wallet-broker/src/native.rs": "a0ccb0b11e3e636cc28f9576ec0cf6da2d8c2ef5344370e5a7a2a9a9b5cc32e5",
    "wallet-broker/src/native_ui.rs": "c6607af3f36278f8d7deeb2b4276017177bbb4c695017f18bf4d2bcb412b268f",
    "wallet-broker/src/native_ui/zec_native_app_tests.rs": "d1d405f8667c6c9d184104d2023ffcff209f6d013e3920dd5c5ccd7e5a2c40f0",
    "wallet-broker/src/native_ui/zec_review_tests.rs": "2eec3b60eb1b9fbd32235832a867d1189928126ca79373fa9f4fa7fdf2ee87bf",
    "wallet-broker/src/runtime.rs": "6788914524605f4669d20d65f7b6ef7282b0090a0696ea45f088ecc49dc1c962",
    "wallet-broker/src/session.rs": "9f88b3b0eca91d341a1c025bc8394a25e456350be58e99d9ac9417e31eef21af",
    "wallet-broker/src/store.rs": "611d837641069a98d05b9e68c14bf11a37a5076de58bf6516188870eeab19236",
    "wallet-broker/src/vault.rs": "f23247b0e46c68dfc125804fa981d16f7022464e1e4d94a8c1e62771a045be49",
    "wallet-broker/src/xmr.rs": "78107f241bb4cb8f02ab4168cbc81a01fc90cc75c80328a2677f819d7c06adce",
    "wallet-broker/src/xmr/account.rs": "5dcad3d450a2e5d8d780e7e490111c33ba06da6275d7d1ca84e5f76dde09cddb",
    "wallet-broker/src/xmr/distribution.rs": "163f8532bc7edfd80fc07966c0f8f32eebc0d12181fd273bc4e6c2870d86dea8",
    "wallet-broker/src/xmr/model.rs": "2a2d3ba1ce453aca65138df402bde3e7f1fee997d5d069024cb1beb8102152cb",
    "wallet-broker/src/xmr/process.rs": "66f0aae7fd0b507cbadc27628d0b1c26ee0033d90891c294721c11a00be9dd2d",
    "wallet-broker/src/xmr/receiver.rs": "daece8857b74eb7f369e0dfad7607dc418d397338cb311367448a632383df2b9",
    "wallet-broker/src/xmr/rpc.rs": "1bbfdf3ec58f89728b2eb169e9d49c53512eb3b108e5c17f7b02bf2634fada33",
    "wallet-broker/src/xmr/store.rs": "3a7f4d5b8cc7b33e3596910ce0b9b10d2f760f24c3ccff98fd2941c410ee2df4",
    "wallet-broker/src/xmr/test_support.rs": "18e6d410b0b5186d45db82105229c8473ce10cfa39a5a54e57a6bc7d0714c2fc",
    "wallet-broker/src/zec.rs": "2544d86b3022aacea6e64f75ce37c38da0ce27cb6f555e085fc76450b5760d1e",
    "wallet-broker/src/zec/address.rs": "d9d54e301d188e3d4cb539095c5c01bcb07aaf011fcb1a50129aadc18f24cbfe",
    "wallet-broker/src/zec/fixture.rs": "318820c6f125f2318ba0caf5ae44835b36b0f67856fc66b83401961f0c301b36",
    "wallet-broker/src/zec/hardware.rs": "9546188299a2b7225b65820f3022fbaa85c1b66acc5266c0c37cc858ae910760",
    "wallet-broker/src/zec/prepare.rs": "365ce7cc75219d616098512af900c6c34d0fd1784e844926be5946975a4e3468",
    "wallet-broker/src/zec/scan.rs": "54a890fd0b5a8ca8ce78dfb59c13424407341df25e47a5eacd2de8d6cc3317ad",
    "wallet-broker/src/zec/spend.rs": "086052983c0fae8bffb9461a599df809c92e0fe5b94adde413f2df07f1255b85",
    "wallet-broker/src/zec/spend/cleanup_lifecycle_tests.rs": "60d2a67a7c7fff912b0df4ec16bcbe251b276b6e990c33f5433a1492539b2f66",
    "wallet-broker/src/zec/spend/effects.rs": "54d1a2959f15f881a9e2bc56261b5090cad89aae06b97d7ace7747f2449a2a72",
    "wallet-broker/src/zec/spend/external_binding_tests.rs": "bb2d6873dd7262ad6e30f117f1947b7d4cf7685ab04e32a5b700d7cb5b112b6d",
    "wallet-broker/src/zec/spend/verification_context_tests.rs": "25db22a3273aa2fcc3da38055726ebe834cabea05183b8fbe9ca9c47f5ae7ee0",
    "wallet-broker/src/zec/store.rs": "531d0a6171ecd9b5012602ccb870f16eab96888c809b3ffdf4e6303576f17c90",
    "wallet-broker/src/zec/test_support.rs": "bd005c21c7a601fe9edd2357525d4f1e8eb29d0b799e57a672433b591581f12d",
    "wallet-broker/src/zec/test_support/cleanup_lifecycle_tests.rs": "5b01727fdc2d864e0ec31693a58ad7f25041891f5cab1bc091ec0b8743323cca",
    "wallet-broker/tests/zec_live_sync.rs": "ba5abca785c2fa40139afae74948174b1fdcdbd94c2301ed7f9280f03e9f99bf",
    "wallet-broker/tests/account_management.rs": "22a31f33cc589c99514b2e215596c5f843c02bfe28866eb6d5ae286743fa5773",
    "wallet-broker/tests/account_native_ui.rs": "ecdebfa97e0685df590cf69bbfcca5a04c6313da087d1190744667933aac98e6",
    "wallet-broker/Cargo.toml": "73e5e585eb2fd1ca867d66962460cfa010443886a4b59d8a33556ef2a4ff3503",
    "wallet-broker/Cargo.lock": "b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71",
    "package.json": "76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5",
    "package-lock.json": "0e32de43f3c1a31ae3d9e9c8b172d10cd37a03577dc7d1eaf4ac916967cfcdb8",
    "scripts/security-policy.js": "fec10759a73a3c675d8206da454abf4b0e8dca6b62da463f39a2db27b30dc078",
    "test/securityPolicy.node.js": "a58d85e5322f0cb01a78b636454f92547bd98e556013cc07712dc0cf64ea565c"
  }
}
```
