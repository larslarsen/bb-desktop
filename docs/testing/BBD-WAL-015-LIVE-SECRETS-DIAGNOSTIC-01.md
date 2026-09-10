# WAL-015 redacted secret-scan finding locations

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
      "output": " M docs/architecture/BBD-WAL-001-REVIEW.md\n M docs/handoff/CURRENT_TASK.md\n M package-lock.json\n M package.json\n M scripts/security-policy.js\n M test/securityPolicy.node.js\n M wallet-broker/Cargo.lock\n M wallet-broker/Cargo.toml\n M wallet-broker/src/account_ui.rs\n M wallet-broker/src/accounts.rs\n M wallet-broker/src/zec.rs\n M wallet-broker/src/zec/scan.rs\n M wallet-broker/src/zec/store.rs\n M wallet-broker/src/zec/test_support.rs\n M wallet-broker/tests/account_management.rs\n M wallet-broker/tests/account_native_ui.rs\n?? docs/architecture/BBD-WAL-015-LIVE-SYNC-REVIEW.md\n?? docs/handoff/HERMES_BBD_WAL_015_AUDIT_01.md\n?? docs/handoff/HERMES_BBD_WAL_015_BUILD_01.md\n?? docs/handoff/HERMES_BBD_WAL_015_BUILD_01.py\n?? docs/handoff/HERMES_BBD_WAL_015_COMPILE_01.md\n?? docs/handoff/HERMES_BBD_WAL_015_COMPILE_01.py\n?? docs/handoff/HERMES_BBD_WAL_015_COMPILE_02.md\n?? docs/handoff/HERMES_BBD_WAL_015_COMPILE_02.py\n?? docs/handoff/HERMES_BBD_WAL_015_COMPILE_03.md\n?? docs/handoff/HERMES_BBD_WAL_015_COMPILE_03.py\n?? docs/handoff/HERMES_BBD_WAL_015_DEPS_01.md\n?? docs/handoff/HERMES_BBD_WAL_015_GREEN_01.md\n?? docs/handoff/HERMES_BBD_WAL_015_GREEN_01.py\n?? docs/handoff/HERMES_BBD_WAL_015_GREEN_02.md\n?? docs/handoff/HERMES_BBD_WAL_015_GREEN_02.py\n?? docs/handoff/HERMES_BBD_WAL_015_LOCK_EDGE_01.md\n?? docs/handoff/HERMES_BBD_WAL_015_LOCK_EDGE_01.py\n?? docs/handoff/HERMES_BBD_WAL_015_POLICY_GREEN_01.md\n?? docs/handoff/HERMES_BBD_WAL_015_POLICY_GREEN_01.py\n?? docs/handoff/HERMES_BBD_WAL_015_POLICY_RED_01.md\n?? docs/handoff/HERMES_BBD_WAL_015_POLICY_RED_01.py\n?? docs/handoff/HERMES_BBD_WAL_015_RED_01.md\n?? docs/handoff/HERMES_BBD_WAL_015_RUNTIME_01.md\n?? docs/handoff/HERMES_BBD_WAL_015_RUNTIME_01.py\n?? docs/handoff/HERMES_BBD_WAL_015_RUNTIME_02.md\n?? docs/handoff/HERMES_BBD_WAL_015_RUNTIME_02.py\n?? docs/handoff/HERMES_BBD_WAL_015_RUNTIME_03.md\n?? docs/handoff/HERMES_BBD_WAL_015_RUNTIME_03.py\n?? docs/handoff/HERMES_BBD_WAL_015_SECRETS_DIAGNOSTIC_01.md\n?? docs/handoff/HERMES_BBD_WAL_015_SECRETS_DIAGNOSTIC_01.py\n?? docs/handoff/SOL_BBD_WAL_015_CORE_01.md\n?? docs/handoff/SOL_BBD_WAL_015_HELPERS_01.md\n?? docs/handoff/SOL_BBD_WAL_015_LINT_01.md\n?? docs/handoff/SOL_BBD_WAL_015_POLICY_01.md\n?? docs/handoff/SOL_BBD_WAL_015_PRODUCTION_01.md\n?? docs/handoff/SOL_BBD_WAL_015_RECOVERY_TEST_01.md\n?? docs/handoff/SOL_BBD_WAL_015_REORG_CORRECTION_01.md\n?? docs/handoff/SOL_BBD_WAL_015_RUNTIME_CORRECTIONS_01.md\n?? docs/handoff/SOL_BBD_WAL_015_TESTS_01.md\n?? docs/handoff/SOL_BBD_WAL_015_TRANSPORT_TESTS_01.md\n?? docs/handoff/SOL_BBD_WAL_015_VALIDATION_TESTS_01.md\n?? docs/handoff/WAL_015_POLICY_REVIEWED_DELTAS.json\n?? docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md\n?? docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md\n?? docs/testing/BBD-WAL-015-LIVE-AUDIT-01.md\n?? docs/testing/BBD-WAL-015-LIVE-COMPILE-01.md\n?? docs/testing/BBD-WAL-015-LIVE-COMPILE-02.md\n?? docs/testing/BBD-WAL-015-LIVE-COMPILE-03.md\n?? docs/testing/BBD-WAL-015-LIVE-DEPS-01.md\n?? docs/testing/BBD-WAL-015-LIVE-GREEN-01.md\n?? docs/testing/BBD-WAL-015-LIVE-GREEN-02.md\n?? docs/testing/BBD-WAL-015-LIVE-LOCK-EDGE-01.md\n?? docs/testing/BBD-WAL-015-LIVE-POLICY-GREEN-01.md\n?? docs/testing/BBD-WAL-015-LIVE-POLICY-RED-01.md\n?? docs/testing/BBD-WAL-015-LIVE-RED-01.md\n?? docs/testing/BBD-WAL-015-LIVE-RUNTIME-01.md\n?? docs/testing/BBD-WAL-015-LIVE-RUNTIME-02.md\n?? docs/testing/BBD-WAL-015-LIVE-RUNTIME-03.md\n?? test/walletLiveSyncPolicy.node.js\n?? tickets/BBD-WAL-015.md\n?? wallet-broker/src/zec/live.rs\n?? wallet-broker/src/zec/live_transport.rs\n?? wallet-broker/tests/fixtures/live-grpc-server.js\n?? wallet-broker/tests/zec_live_recovery.rs\n?? wallet-broker/tests/zec_live_sync.rs\n?? wallet-broker/tests/zec_live_transport.rs\n?? wallet-broker/tests/zec_live_validation.rs\n"
    },
    {
      "argv": [
        "target/security-tools/gitleaks-v8.30.1/gitleaks",
        "dir",
        "--redact=100",
        "--no-banner",
        "--report-format",
        "json",
        "--report-path",
        "wallet-broker/target/wal015-gitleaks-redacted-01.json",
        "."
      ],
      "exit": 1,
      "output": "4:01PM INF scanned ~2162597607 bytes (2.16 GB) in 13.9s\n4:01PM WRN leaks found: 64\n"
    }
  ],
  "accepted": true,
  "version": "Hermes Agent v0.18.2 (2026.7.7.2) \u00b7 upstream 564aef29 \u00b7 local 10b6d1a9 (+1 carried commit)\nInstall directory: <home>/.hermes/hermes-agent\nInstall method: git\nPython: 3.11.15\nOpenAI SDK: 2.24.0",
  "head": "e85dfdc8b6e06fdd09f84cee3c6ac37037448437",
  "initial_status": "M docs/architecture/BBD-WAL-001-REVIEW.md\n M docs/handoff/CURRENT_TASK.md\n M package-lock.json\n M package.json\n M scripts/security-policy.js\n M test/securityPolicy.node.js\n M wallet-broker/Cargo.lock\n M wallet-broker/Cargo.toml\n M wallet-broker/src/account_ui.rs\n M wallet-broker/src/accounts.rs\n M wallet-broker/src/zec.rs\n M wallet-broker/src/zec/scan.rs\n M wallet-broker/src/zec/store.rs\n M wallet-broker/src/zec/test_support.rs\n M wallet-broker/tests/account_management.rs\n M wallet-broker/tests/account_native_ui.rs\n?? docs/architecture/BBD-WAL-015-LIVE-SYNC-REVIEW.md\n?? docs/handoff/HERMES_BBD_WAL_015_AUDIT_01.md\n?? docs/handoff/HERMES_BBD_WAL_015_BUILD_01.md\n?? docs/handoff/HERMES_BBD_WAL_015_BUILD_01.py\n?? docs/handoff/HERMES_BBD_WAL_015_COMPILE_01.md\n?? docs/handoff/HERMES_BBD_WAL_015_COMPILE_01.py\n?? docs/handoff/HERMES_BBD_WAL_015_COMPILE_02.md\n?? docs/handoff/HERMES_BBD_WAL_015_COMPILE_02.py\n?? docs/handoff/HERMES_BBD_WAL_015_COMPILE_03.md\n?? docs/handoff/HERMES_BBD_WAL_015_COMPILE_03.py\n?? docs/handoff/HERMES_BBD_WAL_015_DEPS_01.md\n?? docs/handoff/HERMES_BBD_WAL_015_GREEN_01.md\n?? docs/handoff/HERMES_BBD_WAL_015_GREEN_01.py\n?? docs/handoff/HERMES_BBD_WAL_015_GREEN_02.md\n?? docs/handoff/HERMES_BBD_WAL_015_GREEN_02.py\n?? docs/handoff/HERMES_BBD_WAL_015_LOCK_EDGE_01.md\n?? docs/handoff/HERMES_BBD_WAL_015_LOCK_EDGE_01.py\n?? docs/handoff/HERMES_BBD_WAL_015_POLICY_GREEN_01.md\n?? docs/handoff/HERMES_BBD_WAL_015_POLICY_GREEN_01.py\n?? docs/handoff/HERMES_BBD_WAL_015_POLICY_RED_01.md\n?? docs/handoff/HERMES_BBD_WAL_015_POLICY_RED_01.py\n?? docs/handoff/HERMES_BBD_WAL_015_RED_01.md\n?? docs/handoff/HERMES_BBD_WAL_015_RUNTIME_01.md\n?? docs/handoff/HERMES_BBD_WAL_015_RUNTIME_01.py\n?? docs/handoff/HERMES_BBD_WAL_015_RUNTIME_02.md\n?? docs/handoff/HERMES_BBD_WAL_015_RUNTIME_02.py\n?? docs/handoff/HERMES_BBD_WAL_015_RUNTIME_03.md\n?? docs/handoff/HERMES_BBD_WAL_015_RUNTIME_03.py\n?? docs/handoff/HERMES_BBD_WAL_015_SECRETS_DIAGNOSTIC_01.md\n?? docs/handoff/HERMES_BBD_WAL_015_SECRETS_DIAGNOSTIC_01.py\n?? docs/handoff/SOL_BBD_WAL_015_CORE_01.md\n?? docs/handoff/SOL_BBD_WAL_015_HELPERS_01.md\n?? docs/handoff/SOL_BBD_WAL_015_LINT_01.md\n?? docs/handoff/SOL_BBD_WAL_015_POLICY_01.md\n?? docs/handoff/SOL_BBD_WAL_015_PRODUCTION_01.md\n?? docs/handoff/SOL_BBD_WAL_015_RECOVERY_TEST_01.md\n?? docs/handoff/SOL_BBD_WAL_015_REORG_CORRECTION_01.md\n?? docs/handoff/SOL_BBD_WAL_015_RUNTIME_CORRECTIONS_01.md\n?? docs/handoff/SOL_BBD_WAL_015_TESTS_01.md\n?? docs/handoff/SOL_BBD_WAL_015_TRANSPORT_TESTS_01.md\n?? docs/handoff/SOL_BBD_WAL_015_VALIDATION_TESTS_01.md\n?? docs/handoff/WAL_015_POLICY_REVIEWED_DELTAS.json\n?? docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md\n?? docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md\n?? docs/testing/BBD-WAL-015-LIVE-AUDIT-01.md\n?? docs/testing/BBD-WAL-015-LIVE-COMPILE-01.md\n?? docs/testing/BBD-WAL-015-LIVE-COMPILE-02.md\n?? docs/testing/BBD-WAL-015-LIVE-COMPILE-03.md\n?? docs/testing/BBD-WAL-015-LIVE-DEPS-01.md\n?? docs/testing/BBD-WAL-015-LIVE-GREEN-01.md\n?? docs/testing/BBD-WAL-015-LIVE-GREEN-02.md\n?? docs/testing/BBD-WAL-015-LIVE-LOCK-EDGE-01.md\n?? docs/testing/BBD-WAL-015-LIVE-POLICY-GREEN-01.md\n?? docs/testing/BBD-WAL-015-LIVE-POLICY-RED-01.md\n?? docs/testing/BBD-WAL-015-LIVE-RED-01.md\n?? docs/testing/BBD-WAL-015-LIVE-RUNTIME-01.md\n?? docs/testing/BBD-WAL-015-LIVE-RUNTIME-02.md\n?? docs/testing/BBD-WAL-015-LIVE-RUNTIME-03.md\n?? test/walletLiveSyncPolicy.node.js\n?? tickets/BBD-WAL-015.md\n?? wallet-broker/src/zec/live.rs\n?? wallet-broker/src/zec/live_transport.rs\n?? wallet-broker/tests/fixtures/live-grpc-server.js\n?? wallet-broker/tests/zec_live_recovery.rs\n?? wallet-broker/tests/zec_live_sync.rs\n?? wallet-broker/tests/zec_live_transport.rs\n?? wallet-broker/tests/zec_live_validation.rs",
  "session": {
    "id": "20260910_160100_bf9397",
    "model": "meituan/longcat-2.0:free",
    "provider": "nous"
  },
  "input_hashes": {
    "wallet-broker/src/account_ui.rs": "ae8aa3b7bd14390244c5e73475195c33a1ba69079ac9ba6b0d86d869941fda75",
    "wallet-broker/src/accounts.rs": "f386f28f44c12bbdf1015713e4c736e81a86f8c0a27024f376b18a9eb8751d2f",
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
    "wallet-broker/src/zec.rs": "95335e042f2ab67368701f049098682697a7d77b29e2315c6e198dffd8c8755c",
    "wallet-broker/src/zec/address.rs": "d9d54e301d188e3d4cb539095c5c01bcb07aaf011fcb1a50129aadc18f24cbfe",
    "wallet-broker/src/zec/fixture.rs": "318820c6f125f2318ba0caf5ae44835b36b0f67856fc66b83401961f0c301b36",
    "wallet-broker/src/zec/hardware.rs": "9546188299a2b7225b65820f3022fbaa85c1b66acc5266c0c37cc858ae910760",
    "wallet-broker/src/zec/live.rs": "17be2ab62cdda883940d301d9701ed14c507ff351c4356d46a523a045c3f149f",
    "wallet-broker/src/zec/live_transport.rs": "a9f3b50e2694746ec6f443d7a35ded4c035baab4b6003946ac29d1d88e09b90e",
    "wallet-broker/src/zec/prepare.rs": "365ce7cc75219d616098512af900c6c34d0fd1784e844926be5946975a4e3468",
    "wallet-broker/src/zec/scan.rs": "0c4b75aa1e5b7b1b63b710a85a128dadea7dde0fb619189a787fa6bf7b42c3f7",
    "wallet-broker/src/zec/spend.rs": "086052983c0fae8bffb9461a599df809c92e0fe5b94adde413f2df07f1255b85",
    "wallet-broker/src/zec/spend/cleanup_lifecycle_tests.rs": "60d2a67a7c7fff912b0df4ec16bcbe251b276b6e990c33f5433a1492539b2f66",
    "wallet-broker/src/zec/spend/effects.rs": "54d1a2959f15f881a9e2bc56261b5090cad89aae06b97d7ace7747f2449a2a72",
    "wallet-broker/src/zec/spend/external_binding_tests.rs": "bb2d6873dd7262ad6e30f117f1947b7d4cf7685ab04e32a5b700d7cb5b112b6d",
    "wallet-broker/src/zec/spend/verification_context_tests.rs": "25db22a3273aa2fcc3da38055726ebe834cabea05183b8fbe9ca9c47f5ae7ee0",
    "wallet-broker/src/zec/store.rs": "d98f4641f64e931d79fdcdf5e87713fc3a5d0479eae7210b7c955b9465780910",
    "wallet-broker/src/zec/test_support.rs": "7b4dad2fef0c505f610bdcfe7b87b9075bde0673c8713e34ed57b7714326ce57",
    "wallet-broker/src/zec/test_support/cleanup_lifecycle_tests.rs": "5b01727fdc2d864e0ec31693a58ad7f25041891f5cab1bc091ec0b8743323cca",
    "wallet-broker/tests/account_management.rs": "03834e502e133f00a374601bffa3730d38177742df69b4a05001fa8a15dcbd11",
    "wallet-broker/tests/account_native_ui.rs": "1193b6181fb1f54b491386037d075f0681a1ff3fde155b9889adcdfc52d4e05e",
    "wallet-broker/tests/native_surface.rs": "349f3a019a0e7a5c37aaae727192b0ddcecb02192a5fae6df9291eaf1357276d",
    "wallet-broker/tests/secret_hygiene.rs": "dcebe361c7061b06b9ec6bb6fbea88ccb208069f7360ae14e2d7c6ca83b433a4",
    "wallet-broker/tests/vault_crypto.rs": "a83bc4d1bf30201ad8c9bfa09556e8bae029b8eefebf2eeeab4db5fed03e561b",
    "wallet-broker/tests/vault_format.rs": "5c07a7a52a5be52d852e5c5d45bf62e2f86913324d8dcf642a455d6483b6f193",
    "wallet-broker/tests/vault_session.rs": "67487db86d6788633e031418da71f6080409ac57a144ad362e311cb22519be6b",
    "wallet-broker/tests/vault_store.rs": "582dd24bb91b30db8ec3f38bca6103994b8896f3b3a351c63ae00a7187a838c5",
    "wallet-broker/tests/xmr_account.rs": "5e8afabaa8c820be84a010a88fef0e07617e67934c06b32ab445901b1ddad35b",
    "wallet-broker/tests/xmr_distribution.rs": "481042e6dabe705fb7771c0cf692f0561298a9e0e8aa6d0cfd1d8f2ab9deb6f8",
    "wallet-broker/tests/xmr_hygiene.rs": "3653eba660be481d71e75185e024ae7e0a17f8754089c83b21edb397da84230f",
    "wallet-broker/tests/xmr_local_gate.rs": "00a1c7f7e4d01254a94f35b9d38b4a7374d0b74fe3c80d42ef258d7fdcc8728d",
    "wallet-broker/tests/xmr_process.rs": "25166407a510fef113c5c1db948243ef60de884658f7a9545f49524af49da833",
    "wallet-broker/tests/xmr_receiver.rs": "39d438a767214f31fe07d68a844b217e41bcd73ead1a90ab666b596085b6583e",
    "wallet-broker/tests/xmr_rpc.rs": "a1face1660ca0daf66002671ba3d794058b4acd497de2f0a3e25f2ca57597d0b",
    "wallet-broker/tests/zec_address.rs": "2c5012e6884c8c2a81236266c6861b6e4e4fd6b124656dad2ab438add5848ee3",
    "wallet-broker/tests/zec_fixture_builder.rs": "40cc2b56132b42a765c86482e9915b0422adc0154c1e2edcfda4623760ec5d09",
    "wallet-broker/tests/zec_hardware.rs": "32959949c9da01834fe10ab1328777ab906fb9f8c7bc3e8ef66945f6961ad7a7",
    "wallet-broker/tests/zec_hygiene.rs": "aad7c95a2ef661063661f2ec0f16a216d80328096b049d636b88fa0252ba1be6",
    "wallet-broker/tests/zec_live_sync.rs": "60d0093857a5a22a1cbb113b97fd8504827cc6a4a060b628543d0bccb71b7c8e",
    "wallet-broker/tests/zec_live_transport.rs": "f586257cb8827fcda68e7d38ebe4f4ea52852457e6599b0d231a266687756613",
    "wallet-broker/tests/zec_prepare.rs": "c38339ab88a954f725c7341b4384f178078116de1c700e16892409c18eb2f3fa",
    "wallet-broker/tests/zec_scan.rs": "87ed1c3e8db8219ad126efb4de7f681cf909cb404b88df6c2dfadec471a3701f",
    "wallet-broker/tests/zec_sign_verify.rs": "47d6457f2031132efe09282ca38d2b0141b937a9ec48d6acdecbcb618f244f81",
    "wallet-broker/tests/zec_store.rs": "1c230a2a9cf51c841a0df6514393861387422e5d0b2a83e80af47022728e2225",
    "wallet-broker/tests/fixtures/live-grpc-server.js": "bd208bc877070e6cf27a45e5b7129107190bbd74390ac03c648bf9cd80b64c3b",
    "wallet-broker/Cargo.toml": "435907f00d7819bdf7dbe9ff9e367522f7f6eb8b6019dbde9e58212bbaea041b",
    "wallet-broker/Cargo.lock": "a5d53b729433a06f60edcef51e608629c7365326d06fef2bbac8f2b23f048c2a",
    "package.json": "76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5",
    "package-lock.json": "0e32de43f3c1a31ae3d9e9c8b172d10cd37a03577dc7d1eaf4ac916967cfcdb8",
    "scripts/security-policy.js": "414a5645d8621b5d81cdeba7bb7461ec25647a5a1e5532295d6f75e7b8762532",
    "test/securityPolicy.node.js": "ff449a86e04570f8ff07bd280db6787c62f645f301d4235a65e825f3af7bbb9c",
    "wallet-broker/tests/zec_live_validation.rs": "f84e970072700e14cb4ffb1af2330043c315df7c631ac6302efda51a136fd3e7",
    "wallet-broker/tests/zec_live_recovery.rs": "86a1ca797cc84e9ce56a8f8ac737015efafca8b217182ad1701c9e0c0335153f",
    "wallet-broker/supervisor.js": "115e0e51425e7b25d8f6178b206f592165cb14dacde8cb544b970839833c4721",
    "social-main.js": "d959e6adffc2579fa3f2f447f60a165f2b6bcdc279afde2f6ddaa91bc9d9aa4a",
    "scripts/build-wallet-broker.js": "4427ef3e713ec77c45daefe8d37b457ca8dc2c8529b30214dbf65628f43a1445",
    "test/walletAccountManagement.node.js": "6a449335e1101b8dd6340d640514e8fc083a09235210c17dd582b57230de9475",
    "test/walletStartup.node.js": "448fb88268655b979c7c724ab21d13d09fee29a219628cd70f576daae4589de1",
    "test/walletSupervisor.node.js": "7fcb775d2db7ae8f28b6d9f7660c44a056e59891f3e552db80a248d843c76aa4",
    "test/walletStartupSmoke.node.js": "a1c75efee5fcfdbeb9bf56dfc64f0e747ab6aa527d30ce416c5027bc2aa2a1ef",
    "test/walletAccountWindowSmoke.node.js": "3590096cafed5596378488e8ebca8c916e62352d4410e5748a2033ab0ec9fa3a",
    "test/fixtures/wallet-broker/x11-window.py": "d192f036781e5623b88a8ba3f31b9ee7c781f7bdffa46982aa431f8ada5330d3",
    "test/walletLiveSyncPolicy.node.js": "653c906645abf519f75d036467d65b2aaa705d9f3c775ee8638db644ed47700c",
    "docs/handoff/WAL_015_POLICY_REVIEWED_DELTAS.json": "eaac41ea9c717f0535a52886e596e2e8a5dc14ede7722509908d6d36ffc1ecbd"
  },
  "findings": [
    {
      "RuleID": "generic-api-key",
      "File": "docs/handoff/HERMES_BBD_WAL_015_POLICY_RED_01.py",
      "StartLine": 55,
      "EndLine": 55,
      "Fingerprint": "docs/handoff/HERMES_BBD_WAL_015_POLICY_RED_01.py:generic-api-key:55",
      "Description": "Detected a Generic API Key, potentially exposing access to various services and sensitive operations."
    },
    {
      "RuleID": "generic-api-key",
      "File": "docs/handoff/HERMES_BBD_WAL_015_BUILD_01.py",
      "StartLine": 55,
      "EndLine": 55,
      "Fingerprint": "docs/handoff/HERMES_BBD_WAL_015_BUILD_01.py:generic-api-key:55",
      "Description": "Detected a Generic API Key, potentially exposing access to various services and sensitive operations."
    },
    {
      "RuleID": "generic-api-key",
      "File": "docs/handoff/HERMES_BBD_WAL_015_POLICY_GREEN_01.py",
      "StartLine": 55,
      "EndLine": 55,
      "Fingerprint": "docs/handoff/HERMES_BBD_WAL_015_POLICY_GREEN_01.py:generic-api-key:55",
      "Description": "Detected a Generic API Key, potentially exposing access to various services and sensitive operations."
    },
    {
      "RuleID": "generic-api-key",
      "File": "docs/handoff/HERMES_BBD_WAL_015_GREEN_02.py",
      "StartLine": 55,
      "EndLine": 55,
      "Fingerprint": "docs/handoff/HERMES_BBD_WAL_015_GREEN_02.py:generic-api-key:55",
      "Description": "Detected a Generic API Key, potentially exposing access to various services and sensitive operations."
    },
    {
      "RuleID": "generic-api-key",
      "File": "docs/handoff/HERMES_BBD_WAL_015_RUNTIME_03.py",
      "StartLine": 55,
      "EndLine": 55,
      "Fingerprint": "docs/handoff/HERMES_BBD_WAL_015_RUNTIME_03.py:generic-api-key:55",
      "Description": "Detected a Generic API Key, potentially exposing access to various services and sensitive operations."
    },
    {
      "RuleID": "generic-api-key",
      "File": "docs/handoff/HERMES_BBD_WAL_015_COMPILE_01.py",
      "StartLine": 56,
      "EndLine": 56,
      "Fingerprint": "docs/handoff/HERMES_BBD_WAL_015_COMPILE_01.py:generic-api-key:56",
      "Description": "Detected a Generic API Key, potentially exposing access to various services and sensitive operations."
    },
    {
      "RuleID": "generic-api-key",
      "File": "docs/handoff/HERMES_BBD_WAL_015_RUNTIME_01.py",
      "StartLine": 55,
      "EndLine": 55,
      "Fingerprint": "docs/handoff/HERMES_BBD_WAL_015_RUNTIME_01.py:generic-api-key:55",
      "Description": "Detected a Generic API Key, potentially exposing access to various services and sensitive operations."
    },
    {
      "RuleID": "generic-api-key",
      "File": "docs/handoff/HERMES_BBD_WAL_015_RUNTIME_02.py",
      "StartLine": 55,
      "EndLine": 55,
      "Fingerprint": "docs/handoff/HERMES_BBD_WAL_015_RUNTIME_02.py:generic-api-key:55",
      "Description": "Detected a Generic API Key, potentially exposing access to various services and sensitive operations."
    },
    {
      "RuleID": "generic-api-key",
      "File": "docs/handoff/HERMES_BBD_WAL_015_COMPILE_03.py",
      "StartLine": 56,
      "EndLine": 56,
      "Fingerprint": "docs/handoff/HERMES_BBD_WAL_015_COMPILE_03.py:generic-api-key:56",
      "Description": "Detected a Generic API Key, potentially exposing access to various services and sensitive operations."
    },
    {
      "RuleID": "generic-api-key",
      "File": "docs/handoff/HERMES_BBD_WAL_015_SECRETS_DIAGNOSTIC_01.py",
      "StartLine": 55,
      "EndLine": 55,
      "Fingerprint": "docs/handoff/HERMES_BBD_WAL_015_SECRETS_DIAGNOSTIC_01.py:generic-api-key:55",
      "Description": "Detected a Generic API Key, potentially exposing access to various services and sensitive operations."
    },
    {
      "RuleID": "generic-api-key",
      "File": "docs/handoff/HERMES_BBD_WAL_015_COMPILE_02.py",
      "StartLine": 56,
      "EndLine": 56,
      "Fingerprint": "docs/handoff/HERMES_BBD_WAL_015_COMPILE_02.py:generic-api-key:56",
      "Description": "Detected a Generic API Key, potentially exposing access to various services and sensitive operations."
    },
    {
      "RuleID": "generic-api-key",
      "File": "docs/handoff/HERMES_BBD_WAL_015_GREEN_01.py",
      "StartLine": 55,
      "EndLine": 55,
      "Fingerprint": "docs/handoff/HERMES_BBD_WAL_015_GREEN_01.py:generic-api-key:55",
      "Description": "Detected a Generic API Key, potentially exposing access to various services and sensitive operations."
    },
    {
      "RuleID": "generic-api-key",
      "File": "docs/testing/BBD-WAL-015-LIVE-RUNTIME-03.md",
      "StartLine": 146,
      "EndLine": 146,
      "Fingerprint": "docs/testing/BBD-WAL-015-LIVE-RUNTIME-03.md:generic-api-key:146",
      "Description": "Detected a Generic API Key, potentially exposing access to various services and sensitive operations."
    },
    {
      "RuleID": "generic-api-key",
      "File": "docs/testing/BBD-WAL-015-LIVE-RUNTIME-03.md",
      "StartLine": 220,
      "EndLine": 220,
      "Fingerprint": "docs/testing/BBD-WAL-015-LIVE-RUNTIME-03.md:generic-api-key:220",
      "Description": "Detected a Generic API Key, potentially exposing access to various services and sensitive operations."
    },
    {
      "RuleID": "generic-api-key",
      "File": "docs/testing/BBD-WAL-015-LIVE-COMPILE-01.md",
      "StartLine": 176,
      "EndLine": 176,
      "Fingerprint": "docs/testing/BBD-WAL-015-LIVE-COMPILE-01.md:generic-api-key:176",
      "Description": "Detected a Generic API Key, potentially exposing access to various services and sensitive operations."
    },
    {
      "RuleID": "generic-api-key",
      "File": "docs/testing/BBD-WAL-015-LIVE-COMPILE-01.md",
      "StartLine": 256,
      "EndLine": 256,
      "Fingerprint": "docs/testing/BBD-WAL-015-LIVE-COMPILE-01.md:generic-api-key:256",
      "Description": "Detected a Generic API Key, potentially exposing access to various services and sensitive operations."
    },
    {
      "RuleID": "generic-api-key",
      "File": "docs/testing/BBD-WAL-015-LIVE-COMPILE-03.md",
      "StartLine": 177,
      "EndLine": 177,
      "Fingerprint": "docs/testing/BBD-WAL-015-LIVE-COMPILE-03.md:generic-api-key:177",
      "Description": "Detected a Generic API Key, potentially exposing access to various services and sensitive operations."
    },
    {
      "RuleID": "generic-api-key",
      "File": "docs/testing/BBD-WAL-015-LIVE-COMPILE-03.md",
      "StartLine": 257,
      "EndLine": 257,
      "Fingerprint": "docs/testing/BBD-WAL-015-LIVE-COMPILE-03.md:generic-api-key:257",
      "Description": "Detected a Generic API Key, potentially exposing access to various services and sensitive operations."
    },
    {
      "RuleID": "generic-api-key",
      "File": "docs/testing/BBD-WAL-015-LIVE-COMPILE-03.md",
      "StartLine": 333,
      "EndLine": 333,
      "Fingerprint": "docs/testing/BBD-WAL-015-LIVE-COMPILE-03.md:generic-api-key:333",
      "Description": "Detected a Generic API Key, potentially exposing access to various services and sensitive operations."
    },
    {
      "RuleID": "generic-api-key",
      "File": "docs/testing/BBD-WAL-015-LIVE-POLICY-GREEN-01.md",
      "StartLine": 121,
      "EndLine": 121,
      "Fingerprint": "docs/testing/BBD-WAL-015-LIVE-POLICY-GREEN-01.md:generic-api-key:121",
      "Description": "Detected a Generic API Key, potentially exposing access to various services and sensitive operations."
    },
    {
      "RuleID": "generic-api-key",
      "File": "docs/testing/BBD-WAL-015-LIVE-POLICY-GREEN-01.md",
      "StartLine": 216,
      "EndLine": 216,
      "Fingerprint": "docs/testing/BBD-WAL-015-LIVE-POLICY-GREEN-01.md:generic-api-key:216",
      "Description": "Detected a Generic API Key, potentially exposing access to various services and sensitive operations."
    },
    {
      "RuleID": "generic-api-key",
      "File": "docs/testing/BBD-WAL-015-LIVE-RUNTIME-02.md",
      "StartLine": 166,
      "EndLine": 166,
      "Fingerprint": "docs/testing/BBD-WAL-015-LIVE-RUNTIME-02.md:generic-api-key:166",
      "Description": "Detected a Generic API Key, potentially exposing access to various services and sensitive operations."
    },
    {
      "RuleID": "generic-api-key",
      "File": "docs/testing/BBD-WAL-015-LIVE-RUNTIME-02.md",
      "StartLine": 246,
      "EndLine": 246,
      "Fingerprint": "docs/testing/BBD-WAL-015-LIVE-RUNTIME-02.md:generic-api-key:246",
      "Description": "Detected a Generic API Key, potentially exposing access to various services and sensitive operations."
    },
    {
      "RuleID": "generic-api-key",
      "File": "docs/testing/BBD-WAL-015-LIVE-RUNTIME-02.md",
      "StartLine": 321,
      "EndLine": 321,
      "Fingerprint": "docs/testing/BBD-WAL-015-LIVE-RUNTIME-02.md:generic-api-key:321",
      "Description": "Detected a Generic API Key, potentially exposing access to various services and sensitive operations."
    },
    {
      "RuleID": "generic-api-key",
      "File": "docs/testing/BBD-WAL-015-LIVE-POLICY-RED-01.md",
      "StartLine": 94,
      "EndLine": 94,
      "Fingerprint": "docs/testing/BBD-WAL-015-LIVE-POLICY-RED-01.md:generic-api-key:94",
      "Description": "Detected a Generic API Key, potentially exposing access to various services and sensitive operations."
    },
    {
      "RuleID": "generic-api-key",
      "File": "docs/testing/BBD-WAL-015-LIVE-POLICY-RED-01.md",
      "StartLine": 179,
      "EndLine": 179,
      "Fingerprint": "docs/testing/BBD-WAL-015-LIVE-POLICY-RED-01.md:generic-api-key:179",
      "Description": "Detected a Generic API Key, potentially exposing access to various services and sensitive operations."
    },
    {
      "RuleID": "generic-api-key",
      "File": "docs/testing/BBD-WAL-015-LIVE-COMPILE-02.md",
      "StartLine": 188,
      "EndLine": 188,
      "Fingerprint": "docs/testing/BBD-WAL-015-LIVE-COMPILE-02.md:generic-api-key:188",
      "Description": "Detected a Generic API Key, potentially exposing access to various services and sensitive operations."
    },
    {
      "RuleID": "generic-api-key",
      "File": "docs/testing/BBD-WAL-015-LIVE-COMPILE-02.md",
      "StartLine": 267,
      "EndLine": 267,
      "Fingerprint": "docs/testing/BBD-WAL-015-LIVE-COMPILE-02.md:generic-api-key:267",
      "Description": "Detected a Generic API Key, potentially exposing access to various services and sensitive operations."
    },
    {
      "RuleID": "generic-api-key",
      "File": "docs/testing/BBD-WAL-015-LIVE-COMPILE-02.md",
      "StartLine": 341,
      "EndLine": 341,
      "Fingerprint": "docs/testing/BBD-WAL-015-LIVE-COMPILE-02.md:generic-api-key:341",
      "Description": "Detected a Generic API Key, potentially exposing access to various services and sensitive operations."
    },
    {
      "RuleID": "generic-api-key",
      "File": "docs/testing/BBD-WAL-015-LIVE-GREEN-02.md",
      "StartLine": 262,
      "EndLine": 262,
      "Fingerprint": "docs/testing/BBD-WAL-015-LIVE-GREEN-02.md:generic-api-key:262",
      "Description": "Detected a Generic API Key, potentially exposing access to various services and sensitive operations."
    },
    {
      "RuleID": "generic-api-key",
      "File": "docs/testing/BBD-WAL-015-LIVE-GREEN-02.md",
      "StartLine": 349,
      "EndLine": 349,
      "Fingerprint": "docs/testing/BBD-WAL-015-LIVE-GREEN-02.md:generic-api-key:349",
      "Description": "Detected a Generic API Key, potentially exposing access to various services and sensitive operations."
    },
    {
      "RuleID": "generic-api-key",
      "File": "docs/testing/BBD-WAL-015-LIVE-GREEN-02.md",
      "StartLine": 444,
      "EndLine": 444,
      "Fingerprint": "docs/testing/BBD-WAL-015-LIVE-GREEN-02.md:generic-api-key:444",
      "Description": "Detected a Generic API Key, potentially exposing access to various services and sensitive operations."
    },
    {
      "RuleID": "generic-api-key",
      "File": "docs/testing/BBD-WAL-015-LIVE-RUNTIME-01.md",
      "StartLine": 283,
      "EndLine": 283,
      "Fingerprint": "docs/testing/BBD-WAL-015-LIVE-RUNTIME-01.md:generic-api-key:283",
      "Description": "Detected a Generic API Key, potentially exposing access to various services and sensitive operations."
    },
    {
      "RuleID": "generic-api-key",
      "File": "docs/testing/BBD-WAL-015-LIVE-RUNTIME-01.md",
      "StartLine": 364,
      "EndLine": 364,
      "Fingerprint": "docs/testing/BBD-WAL-015-LIVE-RUNTIME-01.md:generic-api-key:364",
      "Description": "Detected a Generic API Key, potentially exposing access to various services and sensitive operations."
    },
    {
      "RuleID": "generic-api-key",
      "File": "docs/testing/BBD-WAL-015-LIVE-RUNTIME-01.md",
      "StartLine": 440,
      "EndLine": 440,
      "Fingerprint": "docs/testing/BBD-WAL-015-LIVE-RUNTIME-01.md:generic-api-key:440",
      "Description": "Detected a Generic API Key, potentially exposing access to various services and sensitive operations."
    },
    {
      "RuleID": "generic-api-key",
      "File": "docs/testing/BBD-WAL-015-LIVE-GREEN-01.md",
      "StartLine": 290,
      "EndLine": 290,
      "Fingerprint": "docs/testing/BBD-WAL-015-LIVE-GREEN-01.md:generic-api-key:290",
      "Description": "Detected a Generic API Key, potentially exposing access to various services and sensitive operations."
    },
    {
      "RuleID": "generic-api-key",
      "File": "docs/testing/BBD-WAL-015-LIVE-GREEN-01.md",
      "StartLine": 368,
      "EndLine": 368,
      "Fingerprint": "docs/testing/BBD-WAL-015-LIVE-GREEN-01.md:generic-api-key:368",
      "Description": "Detected a Generic API Key, potentially exposing access to various services and sensitive operations."
    },
    {
      "RuleID": "generic-api-key",
      "File": "docs/testing/BBD-WAL-015-LIVE-GREEN-01.md",
      "StartLine": 443,
      "EndLine": 443,
      "Fingerprint": "docs/testing/BBD-WAL-015-LIVE-GREEN-01.md:generic-api-key:443",
      "Description": "Detected a Generic API Key, potentially exposing access to various services and sensitive operations."
    },
    {
      "RuleID": "generic-api-key",
      "File": "wallet-broker/target/wal015-live-policy-green-01.json",
      "StartLine": 116,
      "EndLine": 116,
      "Fingerprint": "wallet-broker/target/wal015-live-policy-green-01.json:generic-api-key:116",
      "Description": "Detected a Generic API Key, potentially exposing access to various services and sensitive operations."
    },
    {
      "RuleID": "generic-api-key",
      "File": "wallet-broker/target/wal015-live-policy-green-01.json",
      "StartLine": 211,
      "EndLine": 211,
      "Fingerprint": "wallet-broker/target/wal015-live-policy-green-01.json:generic-api-key:211",
      "Description": "Detected a Generic API Key, potentially exposing access to various services and sensitive operations."
    },
    {
      "RuleID": "generic-api-key",
      "File": "wallet-broker/target/wal015-live-policy-red-01.json",
      "StartLine": 89,
      "EndLine": 89,
      "Fingerprint": "wallet-broker/target/wal015-live-policy-red-01.json:generic-api-key:89",
      "Description": "Detected a Generic API Key, potentially exposing access to various services and sensitive operations."
    },
    {
      "RuleID": "generic-api-key",
      "File": "wallet-broker/target/wal015-live-policy-red-01.json",
      "StartLine": 174,
      "EndLine": 174,
      "Fingerprint": "wallet-broker/target/wal015-live-policy-red-01.json:generic-api-key:174",
      "Description": "Detected a Generic API Key, potentially exposing access to various services and sensitive operations."
    },
    {
      "RuleID": "generic-api-key",
      "File": "wallet-broker/target/wal015-live-compile-01.json",
      "StartLine": 171,
      "EndLine": 171,
      "Fingerprint": "wallet-broker/target/wal015-live-compile-01.json:generic-api-key:171",
      "Description": "Detected a Generic API Key, potentially exposing access to various services and sensitive operations."
    },
    {
      "RuleID": "generic-api-key",
      "File": "wallet-broker/target/wal015-live-compile-01.json",
      "StartLine": 251,
      "EndLine": 251,
      "Fingerprint": "wallet-broker/target/wal015-live-compile-01.json:generic-api-key:251",
      "Description": "Detected a Generic API Key, potentially exposing access to various services and sensitive operations."
    },
    {
      "RuleID": "generic-api-key",
      "File": "wallet-broker/target/wal015-live-runtime-01.json",
      "StartLine": 278,
      "EndLine": 278,
      "Fingerprint": "wallet-broker/target/wal015-live-runtime-01.json:generic-api-key:278",
      "Description": "Detected a Generic API Key, potentially exposing access to various services and sensitive operations."
    },
    {
      "RuleID": "generic-api-key",
      "File": "wallet-broker/target/wal015-live-runtime-01.json",
      "StartLine": 359,
      "EndLine": 359,
      "Fingerprint": "wallet-broker/target/wal015-live-runtime-01.json:generic-api-key:359",
      "Description": "Detected a Generic API Key, potentially exposing access to various services and sensitive operations."
    },
    {
      "RuleID": "generic-api-key",
      "File": "wallet-broker/target/wal015-live-runtime-01.json",
      "StartLine": 435,
      "EndLine": 435,
      "Fingerprint": "wallet-broker/target/wal015-live-runtime-01.json:generic-api-key:435",
      "Description": "Detected a Generic API Key, potentially exposing access to various services and sensitive operations."
    },
    {
      "RuleID": "generic-api-key",
      "File": "wallet-broker/target/wal015-live-runtime-03.json",
      "StartLine": 141,
      "EndLine": 141,
      "Fingerprint": "wallet-broker/target/wal015-live-runtime-03.json:generic-api-key:141",
      "Description": "Detected a Generic API Key, potentially exposing access to various services and sensitive operations."
    },
    {
      "RuleID": "generic-api-key",
      "File": "wallet-broker/target/wal015-live-runtime-03.json",
      "StartLine": 215,
      "EndLine": 215,
      "Fingerprint": "wallet-broker/target/wal015-live-runtime-03.json:generic-api-key:215",
      "Description": "Detected a Generic API Key, potentially exposing access to various services and sensitive operations."
    },
    {
      "RuleID": "generic-api-key",
      "File": "wallet-broker/target/wal015-live-compile-02.json",
      "StartLine": 183,
      "EndLine": 183,
      "Fingerprint": "wallet-broker/target/wal015-live-compile-02.json:generic-api-key:183",
      "Description": "Detected a Generic API Key, potentially exposing access to various services and sensitive operations."
    },
    {
      "RuleID": "generic-api-key",
      "File": "wallet-broker/target/wal015-live-compile-02.json",
      "StartLine": 262,
      "EndLine": 262,
      "Fingerprint": "wallet-broker/target/wal015-live-compile-02.json:generic-api-key:262",
      "Description": "Detected a Generic API Key, potentially exposing access to various services and sensitive operations."
    },
    {
      "RuleID": "generic-api-key",
      "File": "wallet-broker/target/wal015-live-compile-02.json",
      "StartLine": 336,
      "EndLine": 336,
      "Fingerprint": "wallet-broker/target/wal015-live-compile-02.json:generic-api-key:336",
      "Description": "Detected a Generic API Key, potentially exposing access to various services and sensitive operations."
    },
    {
      "RuleID": "generic-api-key",
      "File": "wallet-broker/target/wal015-live-compile-03.json",
      "StartLine": 172,
      "EndLine": 172,
      "Fingerprint": "wallet-broker/target/wal015-live-compile-03.json:generic-api-key:172",
      "Description": "Detected a Generic API Key, potentially exposing access to various services and sensitive operations."
    },
    {
      "RuleID": "generic-api-key",
      "File": "wallet-broker/target/wal015-live-compile-03.json",
      "StartLine": 252,
      "EndLine": 252,
      "Fingerprint": "wallet-broker/target/wal015-live-compile-03.json:generic-api-key:252",
      "Description": "Detected a Generic API Key, potentially exposing access to various services and sensitive operations."
    },
    {
      "RuleID": "generic-api-key",
      "File": "wallet-broker/target/wal015-live-compile-03.json",
      "StartLine": 328,
      "EndLine": 328,
      "Fingerprint": "wallet-broker/target/wal015-live-compile-03.json:generic-api-key:328",
      "Description": "Detected a Generic API Key, potentially exposing access to various services and sensitive operations."
    },
    {
      "RuleID": "generic-api-key",
      "File": "wallet-broker/target/wal015-live-runtime-02.json",
      "StartLine": 161,
      "EndLine": 161,
      "Fingerprint": "wallet-broker/target/wal015-live-runtime-02.json:generic-api-key:161",
      "Description": "Detected a Generic API Key, potentially exposing access to various services and sensitive operations."
    },
    {
      "RuleID": "generic-api-key",
      "File": "wallet-broker/target/wal015-live-runtime-02.json",
      "StartLine": 241,
      "EndLine": 241,
      "Fingerprint": "wallet-broker/target/wal015-live-runtime-02.json:generic-api-key:241",
      "Description": "Detected a Generic API Key, potentially exposing access to various services and sensitive operations."
    },
    {
      "RuleID": "generic-api-key",
      "File": "wallet-broker/target/wal015-live-runtime-02.json",
      "StartLine": 316,
      "EndLine": 316,
      "Fingerprint": "wallet-broker/target/wal015-live-runtime-02.json:generic-api-key:316",
      "Description": "Detected a Generic API Key, potentially exposing access to various services and sensitive operations."
    },
    {
      "RuleID": "generic-api-key",
      "File": "wallet-broker/target/wal015-live-green-01.json",
      "StartLine": 285,
      "EndLine": 285,
      "Fingerprint": "wallet-broker/target/wal015-live-green-01.json:generic-api-key:285",
      "Description": "Detected a Generic API Key, potentially exposing access to various services and sensitive operations."
    },
    {
      "RuleID": "generic-api-key",
      "File": "wallet-broker/target/wal015-live-green-01.json",
      "StartLine": 363,
      "EndLine": 363,
      "Fingerprint": "wallet-broker/target/wal015-live-green-01.json:generic-api-key:363",
      "Description": "Detected a Generic API Key, potentially exposing access to various services and sensitive operations."
    },
    {
      "RuleID": "generic-api-key",
      "File": "wallet-broker/target/wal015-live-green-01.json",
      "StartLine": 438,
      "EndLine": 438,
      "Fingerprint": "wallet-broker/target/wal015-live-green-01.json:generic-api-key:438",
      "Description": "Detected a Generic API Key, potentially exposing access to various services and sensitive operations."
    },
    {
      "RuleID": "generic-api-key",
      "File": "wallet-broker/target/wal015-live-green-02.json",
      "StartLine": 257,
      "EndLine": 257,
      "Fingerprint": "wallet-broker/target/wal015-live-green-02.json:generic-api-key:257",
      "Description": "Detected a Generic API Key, potentially exposing access to various services and sensitive operations."
    },
    {
      "RuleID": "generic-api-key",
      "File": "wallet-broker/target/wal015-live-green-02.json",
      "StartLine": 344,
      "EndLine": 344,
      "Fingerprint": "wallet-broker/target/wal015-live-green-02.json:generic-api-key:344",
      "Description": "Detected a Generic API Key, potentially exposing access to various services and sensitive operations."
    },
    {
      "RuleID": "generic-api-key",
      "File": "wallet-broker/target/wal015-live-green-02.json",
      "StartLine": 439,
      "EndLine": 439,
      "Fingerprint": "wallet-broker/target/wal015-live-green-02.json:generic-api-key:439",
      "Description": "Detected a Generic API Key, potentially exposing access to various services and sensitive operations."
    }
  ],
  "redacted_report_sha256": "880bc24bd0d796994c706a31cefe2fca238e581e1ddfe7585c42df5a6e876682",
  "diagnostic_complete": true,
  "final_hashes": {
    "wallet-broker/src/account_ui.rs": "ae8aa3b7bd14390244c5e73475195c33a1ba69079ac9ba6b0d86d869941fda75",
    "wallet-broker/src/accounts.rs": "f386f28f44c12bbdf1015713e4c736e81a86f8c0a27024f376b18a9eb8751d2f",
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
    "wallet-broker/src/zec.rs": "95335e042f2ab67368701f049098682697a7d77b29e2315c6e198dffd8c8755c",
    "wallet-broker/src/zec/address.rs": "d9d54e301d188e3d4cb539095c5c01bcb07aaf011fcb1a50129aadc18f24cbfe",
    "wallet-broker/src/zec/fixture.rs": "318820c6f125f2318ba0caf5ae44835b36b0f67856fc66b83401961f0c301b36",
    "wallet-broker/src/zec/hardware.rs": "9546188299a2b7225b65820f3022fbaa85c1b66acc5266c0c37cc858ae910760",
    "wallet-broker/src/zec/live.rs": "17be2ab62cdda883940d301d9701ed14c507ff351c4356d46a523a045c3f149f",
    "wallet-broker/src/zec/live_transport.rs": "a9f3b50e2694746ec6f443d7a35ded4c035baab4b6003946ac29d1d88e09b90e",
    "wallet-broker/src/zec/prepare.rs": "365ce7cc75219d616098512af900c6c34d0fd1784e844926be5946975a4e3468",
    "wallet-broker/src/zec/scan.rs": "0c4b75aa1e5b7b1b63b710a85a128dadea7dde0fb619189a787fa6bf7b42c3f7",
    "wallet-broker/src/zec/spend.rs": "086052983c0fae8bffb9461a599df809c92e0fe5b94adde413f2df07f1255b85",
    "wallet-broker/src/zec/spend/cleanup_lifecycle_tests.rs": "60d2a67a7c7fff912b0df4ec16bcbe251b276b6e990c33f5433a1492539b2f66",
    "wallet-broker/src/zec/spend/effects.rs": "54d1a2959f15f881a9e2bc56261b5090cad89aae06b97d7ace7747f2449a2a72",
    "wallet-broker/src/zec/spend/external_binding_tests.rs": "bb2d6873dd7262ad6e30f117f1947b7d4cf7685ab04e32a5b700d7cb5b112b6d",
    "wallet-broker/src/zec/spend/verification_context_tests.rs": "25db22a3273aa2fcc3da38055726ebe834cabea05183b8fbe9ca9c47f5ae7ee0",
    "wallet-broker/src/zec/store.rs": "d98f4641f64e931d79fdcdf5e87713fc3a5d0479eae7210b7c955b9465780910",
    "wallet-broker/src/zec/test_support.rs": "7b4dad2fef0c505f610bdcfe7b87b9075bde0673c8713e34ed57b7714326ce57",
    "wallet-broker/src/zec/test_support/cleanup_lifecycle_tests.rs": "5b01727fdc2d864e0ec31693a58ad7f25041891f5cab1bc091ec0b8743323cca",
    "wallet-broker/tests/account_management.rs": "03834e502e133f00a374601bffa3730d38177742df69b4a05001fa8a15dcbd11",
    "wallet-broker/tests/account_native_ui.rs": "1193b6181fb1f54b491386037d075f0681a1ff3fde155b9889adcdfc52d4e05e",
    "wallet-broker/tests/native_surface.rs": "349f3a019a0e7a5c37aaae727192b0ddcecb02192a5fae6df9291eaf1357276d",
    "wallet-broker/tests/secret_hygiene.rs": "dcebe361c7061b06b9ec6bb6fbea88ccb208069f7360ae14e2d7c6ca83b433a4",
    "wallet-broker/tests/vault_crypto.rs": "a83bc4d1bf30201ad8c9bfa09556e8bae029b8eefebf2eeeab4db5fed03e561b",
    "wallet-broker/tests/vault_format.rs": "5c07a7a52a5be52d852e5c5d45bf62e2f86913324d8dcf642a455d6483b6f193",
    "wallet-broker/tests/vault_session.rs": "67487db86d6788633e031418da71f6080409ac57a144ad362e311cb22519be6b",
    "wallet-broker/tests/vault_store.rs": "582dd24bb91b30db8ec3f38bca6103994b8896f3b3a351c63ae00a7187a838c5",
    "wallet-broker/tests/xmr_account.rs": "5e8afabaa8c820be84a010a88fef0e07617e67934c06b32ab445901b1ddad35b",
    "wallet-broker/tests/xmr_distribution.rs": "481042e6dabe705fb7771c0cf692f0561298a9e0e8aa6d0cfd1d8f2ab9deb6f8",
    "wallet-broker/tests/xmr_hygiene.rs": "3653eba660be481d71e75185e024ae7e0a17f8754089c83b21edb397da84230f",
    "wallet-broker/tests/xmr_local_gate.rs": "00a1c7f7e4d01254a94f35b9d38b4a7374d0b74fe3c80d42ef258d7fdcc8728d",
    "wallet-broker/tests/xmr_process.rs": "25166407a510fef113c5c1db948243ef60de884658f7a9545f49524af49da833",
    "wallet-broker/tests/xmr_receiver.rs": "39d438a767214f31fe07d68a844b217e41bcd73ead1a90ab666b596085b6583e",
    "wallet-broker/tests/xmr_rpc.rs": "a1face1660ca0daf66002671ba3d794058b4acd497de2f0a3e25f2ca57597d0b",
    "wallet-broker/tests/zec_address.rs": "2c5012e6884c8c2a81236266c6861b6e4e4fd6b124656dad2ab438add5848ee3",
    "wallet-broker/tests/zec_fixture_builder.rs": "40cc2b56132b42a765c86482e9915b0422adc0154c1e2edcfda4623760ec5d09",
    "wallet-broker/tests/zec_hardware.rs": "32959949c9da01834fe10ab1328777ab906fb9f8c7bc3e8ef66945f6961ad7a7",
    "wallet-broker/tests/zec_hygiene.rs": "aad7c95a2ef661063661f2ec0f16a216d80328096b049d636b88fa0252ba1be6",
    "wallet-broker/tests/zec_live_sync.rs": "60d0093857a5a22a1cbb113b97fd8504827cc6a4a060b628543d0bccb71b7c8e",
    "wallet-broker/tests/zec_live_transport.rs": "f586257cb8827fcda68e7d38ebe4f4ea52852457e6599b0d231a266687756613",
    "wallet-broker/tests/zec_prepare.rs": "c38339ab88a954f725c7341b4384f178078116de1c700e16892409c18eb2f3fa",
    "wallet-broker/tests/zec_scan.rs": "87ed1c3e8db8219ad126efb4de7f681cf909cb404b88df6c2dfadec471a3701f",
    "wallet-broker/tests/zec_sign_verify.rs": "47d6457f2031132efe09282ca38d2b0141b937a9ec48d6acdecbcb618f244f81",
    "wallet-broker/tests/zec_store.rs": "1c230a2a9cf51c841a0df6514393861387422e5d0b2a83e80af47022728e2225",
    "wallet-broker/tests/fixtures/live-grpc-server.js": "bd208bc877070e6cf27a45e5b7129107190bbd74390ac03c648bf9cd80b64c3b",
    "wallet-broker/Cargo.toml": "435907f00d7819bdf7dbe9ff9e367522f7f6eb8b6019dbde9e58212bbaea041b",
    "wallet-broker/Cargo.lock": "a5d53b729433a06f60edcef51e608629c7365326d06fef2bbac8f2b23f048c2a",
    "package.json": "76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5",
    "package-lock.json": "0e32de43f3c1a31ae3d9e9c8b172d10cd37a03577dc7d1eaf4ac916967cfcdb8",
    "scripts/security-policy.js": "414a5645d8621b5d81cdeba7bb7461ec25647a5a1e5532295d6f75e7b8762532",
    "test/securityPolicy.node.js": "ff449a86e04570f8ff07bd280db6787c62f645f301d4235a65e825f3af7bbb9c",
    "wallet-broker/tests/zec_live_validation.rs": "f84e970072700e14cb4ffb1af2330043c315df7c631ac6302efda51a136fd3e7",
    "wallet-broker/tests/zec_live_recovery.rs": "86a1ca797cc84e9ce56a8f8ac737015efafca8b217182ad1701c9e0c0335153f",
    "wallet-broker/supervisor.js": "115e0e51425e7b25d8f6178b206f592165cb14dacde8cb544b970839833c4721",
    "social-main.js": "d959e6adffc2579fa3f2f447f60a165f2b6bcdc279afde2f6ddaa91bc9d9aa4a",
    "scripts/build-wallet-broker.js": "4427ef3e713ec77c45daefe8d37b457ca8dc2c8529b30214dbf65628f43a1445",
    "test/walletAccountManagement.node.js": "6a449335e1101b8dd6340d640514e8fc083a09235210c17dd582b57230de9475",
    "test/walletStartup.node.js": "448fb88268655b979c7c724ab21d13d09fee29a219628cd70f576daae4589de1",
    "test/walletSupervisor.node.js": "7fcb775d2db7ae8f28b6d9f7660c44a056e59891f3e552db80a248d843c76aa4",
    "test/walletStartupSmoke.node.js": "a1c75efee5fcfdbeb9bf56dfc64f0e747ab6aa527d30ce416c5027bc2aa2a1ef",
    "test/walletAccountWindowSmoke.node.js": "3590096cafed5596378488e8ebca8c916e62352d4410e5748a2033ab0ec9fa3a",
    "test/fixtures/wallet-broker/x11-window.py": "d192f036781e5623b88a8ba3f31b9ee7c781f7bdffa46982aa431f8ada5330d3",
    "test/walletLiveSyncPolicy.node.js": "653c906645abf519f75d036467d65b2aaa705d9f3c775ee8638db644ed47700c",
    "docs/handoff/WAL_015_POLICY_REVIEWED_DELTAS.json": "eaac41ea9c717f0535a52886e596e2e8a5dc14ede7722509908d6d36ffc1ecbd"
  }
}
```
