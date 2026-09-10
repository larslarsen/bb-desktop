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
      "output": "4904df8816cf08756d37ceb1978a8c6192530d26\n"
    },
    {
      "argv": [
        "git",
        "status",
        "--short"
      ],
      "exit": 0,
      "output": " M package-lock.json\n M package.json\n M scripts/security-policy.js\n M test/securityPolicy.node.js\n M wallet-broker/src/account_ui.rs\n M wallet-broker/src/accounts.rs\n M wallet-broker/src/session.rs\n M wallet-broker/src/zec.rs\n M wallet-broker/tests/account_management.rs\n M wallet-broker/tests/account_native_ui.rs\n?? docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md\n?? docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md\n?? docs/testing/BBD-WAL-014-RECEIVE-GREEN-01.md\n?? docs/testing/BBD-WAL-014-RECEIVE-RED-01.md\n"
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
      "output": "    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.21s\n"
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
        "wal014_receive_is_native_unlocked_valid_and_does_not_extend_idle_deadline"
      ],
      "exit": 101,
      "output": "   Compiling bitbook-wallet-broker v0.1.0 (<repo>/wallet-broker)\n    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.03s\n     Running tests/account_management.rs (wallet-broker/target/debug/deps/account_management-aa95c72dd55220fd)\n\nrunning 1 test\ntest wal014_receive_is_native_unlocked_valid_and_does_not_extend_idle_deadline ... FAILED\n\nfailures:\n\n---- wal014_receive_is_native_unlocked_valid_and_does_not_extend_idle_deadline stdout ----\n\nthread 'wal014_receive_is_native_unlocked_valid_and_does_not_extend_idle_deadline' (4170840) panicked at tests/account_management.rs:539:5:\nassertion `left == right` failed\n  left: \"LOCKED\"\n right: \"UNAUTH\"\nnote: run with `RUST_BACKTRACE=1` environment variable to display a backtrace\n\n\nfailures:\n    wal014_receive_is_native_unlocked_valid_and_does_not_extend_idle_deadline\n\ntest result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 15 filtered out; finished in 4.08s\n\nerror: test failed, to rerun pass `--test account_management`\n"
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
        "wal014_foreign_ufvk_binding_refuses_without_advancing_issuance"
      ],
      "exit": 101,
      "output": "   Compiling bitbook-wallet-broker v0.1.0 (<repo>/wallet-broker)\n    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.58s\n     Running tests/account_management.rs (wallet-broker/target/debug/deps/account_management-aa95c72dd55220fd)\n\nrunning 1 test\ntest wal014_foreign_ufvk_binding_refuses_without_advancing_issuance ... FAILED\n\nfailures:\n\n---- wal014_foreign_ufvk_binding_refuses_without_advancing_issuance stdout ----\n\nthread 'wal014_foreign_ufvk_binding_refuses_without_advancing_issuance' (4171399) panicked at tests/account_management.rs:1895:14:\ncalled `Result::unwrap_err()` on an `Ok` value: FreshReceiverV1 { account_id: AccountId([REDACTED]), network: Testnet, receiver: \"utest1vck05sgawux0686sz5dafp3406gc9skt496vz8t0wl2q8gd0u0a75lylqcda9268ezt29d294fygnw0vp3un98ftmmx3mgeavvra0ttq\", diversifier_index: \"1\", issued_at_sequence: \"2\" }\nnote: run with `RUST_BACKTRACE=1` environment variable to display a backtrace\n\n\nfailures:\n    wal014_foreign_ufvk_binding_refuses_without_advancing_issuance\n\ntest result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 15 filtered out; finished in 5.01s\n\nerror: test failed, to rerun pass `--test account_management`\n"
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
        "account_management",
        "--test",
        "account_native_ui",
        "wal014_"
      ],
      "exit": 0,
      "output": "   Compiling bitbook-wallet-broker v0.1.0 (<repo>/wallet-broker)\n    Finished `test` profile [unoptimized + debuginfo] target(s) in 4.50s\n     Running tests/account_management.rs (wallet-broker/target/debug/deps/account_management-59a0af1fa4a04f72)\n\nrunning 4 tests\ntest wal014_foreign_ufvk_binding_refuses_without_advancing_issuance ... ok\ntest wal014_receiver_matches_seed_oracle_is_durable_private_and_preserves_vault ... ok\ntest wal014_receive_is_native_unlocked_valid_and_does_not_extend_idle_deadline ... ok\ntest wal014_corrupt_viewing_db_and_symlink_directory_are_never_replaced ... ok\n\ntest result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 12 filtered out; finished in 9.46s\n\n     Running tests/account_native_ui.rs (wallet-broker/target/debug/deps/account_native_ui-6dcf24c6022d9a39)\n\nrunning 3 tests\ntest wal014_receive_scene_issues_once_paints_honestly_and_copies_release_output ... ok\ntest wal014_receive_state_clears_on_lock_catalog_change_error_selection_and_hide ... ok\ntest wal014_shared_port_pointer_receive_and_copy_persist_real_issuance ... ok\n\ntest result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 7.44s\n\n"
    },
    {
      "argv": [
        "node",
        "scripts/build-wallet-broker.js"
      ],
      "exit": 1,
      "output": "Unable to build wallet broker\n"
    }
  ],
  "accepted": false,
  "version": "Hermes Agent v0.18.2 (2026.7.7.2) \u00b7 upstream 67764dc0 \u00b7 local 10b6d1a9 (+1 carried commit)\nInstall directory: <home>/.hermes/hermes-agent\nInstall method: git\nPython: 3.11.15\nOpenAI SDK: 2.24.0",
  "head": "4904df8816cf08756d37ceb1978a8c6192530d26",
  "initial_status": "M package-lock.json\n M package.json\n M scripts/security-policy.js\n M test/securityPolicy.node.js\n M wallet-broker/src/account_ui.rs\n M wallet-broker/src/accounts.rs\n M wallet-broker/src/session.rs\n M wallet-broker/src/zec.rs\n M wallet-broker/tests/account_management.rs\n M wallet-broker/tests/account_native_ui.rs\n?? docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md\n?? docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md\n?? docs/testing/BBD-WAL-014-RECEIVE-GREEN-01.md\n?? docs/testing/BBD-WAL-014-RECEIVE-RED-01.md",
  "session": {
    "id": "20260910_134750_2cf32f",
    "model": "meituan/longcat-2.0:free",
    "provider": "nous"
  },
  "input_hashes": {
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
    "wallet-broker/target/wal013-final-window-security-04.json": "45c1abe5ce23f32df5ac848c131f3c49ecb1c238f73604f2a704154eec4e20ed",
    "wallet-broker/target/wal014-receive-green-01.json": "f947d626af653d30c7381ccc2cc8ec241b62ea5c1b037b277cf701c1af853212",
    "docs/testing/BBD-WAL-014-RECEIVE-GREEN-01.md": "c091d139092d68b8c3c28f74605dced9fca1b86256cff6f77e7dc8bbd9112dc9"
  },
  "reused_rust_test_counts": [
    16,
    13,
    8,
    8,
    8,
    12,
    17
  ],
  "prior_test_helper_clippy_findings": 7,
  "prior_clippy_review": "All seven diagnostics point to byte-identical pre-WAL014 helper code; restore established production-only Clippy scope, no suppression or source change.",
  "native_origin_falsification_detected": true,
  "ufvk_binding_falsification_detected": true,
  "stop": "command failed: ['node', 'scripts/build-wallet-broker.js']",
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
    "wallet-broker/target/wal013-final-window-security-04.json": "45c1abe5ce23f32df5ac848c131f3c49ecb1c238f73604f2a704154eec4e20ed",
    "wallet-broker/target/wal014-receive-green-01.json": "f947d626af653d30c7381ccc2cc8ec241b62ea5c1b037b277cf701c1af853212",
    "docs/testing/BBD-WAL-014-RECEIVE-GREEN-01.md": "c091d139092d68b8c3c28f74605dced9fca1b86256cff6f77e7dc8bbd9112dc9"
  }
}
```
