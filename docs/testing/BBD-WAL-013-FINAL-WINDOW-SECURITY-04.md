# WAL-013 actual window falsification and final security ratchet

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
      "output": "07170db4402b02d708d1f47f493cb437e64c3ed7\n"
    },
    {
      "argv": [
        "git",
        "status",
        "--short"
      ],
      "exit": 0,
      "output": " M package-lock.json\n M package.json\n M scripts/build-wallet-broker.js\n M scripts/security-policy.js\n M social-main.js\n M test/electronSecurity.node.js\n M test/securityPolicy.node.js\n M test/walletBrokerBuild.node.js\n M test/walletBrokerRuntime.node.js\n M test/walletStartup.node.js\n M test/walletStartupSmoke.node.js\n M test/walletSupervisor.node.js\n M test/walletSupervisorShutdown.node.js\n M wallet-broker/src/lib.rs\n M wallet-broker/src/runtime.rs\n M wallet-broker/supervisor.js\n?? docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md\n?? docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md\n?? docs/testing/BBD-WAL-013-ACCOUNTS-GREEN-UI-RED-01.md\n?? docs/testing/BBD-WAL-013-ACCOUNTS-RED-01.md\n?? docs/testing/BBD-WAL-013-FINAL-WINDOW-SECURITY-01.md\n?? docs/testing/BBD-WAL-013-FINAL-WINDOW-SECURITY-02.md\n?? docs/testing/BBD-WAL-013-FINAL-WINDOW-SECURITY-03.md\n?? docs/testing/BBD-WAL-013-NATIVE-RUNTIME-GREEN-01.md\n?? docs/testing/BBD-WAL-013-NATIVE-UI-DIAGNOSTIC-01.md\n?? docs/testing/BBD-WAL-013-NATIVE-UI-GREEN-01.md\n?? docs/testing/BBD-WAL-013-NATIVE-UI-GREEN-FINISH-01.md\n?? docs/testing/BBD-WAL-013-NATIVE-UI-GREEN-RESUME-01.md\n?? docs/testing/BBD-WAL-013-NATIVE-UI-GREEN-RESUME-02.md\n?? docs/testing/BBD-WAL-013-RUNTIME-JS-GREEN-01.md\n?? docs/testing/BBD-WAL-013-RUNTIME-JS-GREEN-FINISH-01.md\n?? docs/testing/BBD-WAL-013-RUNTIME-JS-RED-01.md\n?? test/fixtures/wallet-broker/x11-window.py\n?? test/walletAccountManagement.node.js\n?? test/walletAccountWindowSmoke.node.js\n?? wallet-broker/src/account_ui.rs\n?? wallet-broker/src/accounts.rs\n?? wallet-broker/tests/account_management.rs\n?? wallet-broker/tests/account_native_ui.rs\n"
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
        "account_management"
      ],
      "exit": 0,
      "output": "   Compiling bitbook-wallet-broker v0.1.0 (<repo>/wallet-broker)\n    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.36s\n     Running tests/account_management.rs (wallet-broker/target/debug/deps/account_management-aa95c72dd55220fd)\n\nrunning 12 tests\ntest hostile_roots_and_catalog_entries_fail_closed_without_chmod ... ok\ntest second_manager_same_root_is_unavailable_until_first_drops ... ok\ntest entropy_failure_and_duplicate_id_never_overwrite_existing_vault ... ok\ntest catalog_cap_is_256_without_running_per_account_kdf ... ok\ntest locked_export_copies_exact_encrypted_bytes_and_refuses_existing_or_link ... ok\ntest restart_lists_persisted_locked_identities_without_secret_canaries ... ok\ntest restore_cancel_is_a_noop_and_confirm_commits_captured_not_replaced_bytes ... ok\ntest create_software_persists_locked_encrypted_account_with_scripted_seed ... ok\ntest nonnative_origins_refuse_privileged_apis_before_side_effects ... ok\ntest list_does_not_extend_idle_deadline_and_backwards_clock_locks_all ... ok\ntest restore_rejects_hostile_inputs_and_never_overwrites_existing_id ... ok\ntest unlock_rejects_wrong_corrupt_and_short_material_then_valid_lock_cycle_works ... ok\n\ntest result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 25.34s\n\n"
    },
    {
      "argv": [
        "node",
        "scripts/build-wallet-broker.js"
      ],
      "exit": 0,
      "output": "    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.27s\n"
    },
    {
      "argv": [
        "git",
        "diff",
        "--check"
      ],
      "exit": 0,
      "output": ""
    },
    {
      "argv": [
        "node",
        "test/securityPolicy.node.js"
      ],
      "exit": 1,
      "output": "ok required policy, workflow, and validator sources exist\nok checker constants match the ticketed Action and tool pins\nok YAML parser keeps on: as a mapping key and preserves block scalars\nnot ok committed workflows satisfy the fail-closed checker\nPolicyError: wallet Rust source inventory is missing or extra\n    at checkRustWalletSourceInventory (<repo>/scripts/security-policy.js:2153:11)\n    at Object.checkRepository (<repo>/scripts/security-policy.js:2582:3)\n    at <repo>/test/securityPolicy.node.js:285:10\n    at run (<repo>/test/securityPolicy.node.js:3788:7)\n    at Object.<anonymous> (<repo>/test/securityPolicy.node.js:3803:3)\n    at Module._compile (node:internal/modules/cjs/loader:1781:14)\n    at Object..js (node:internal/modules/cjs/loader:1913:10)\n    at Module.load (node:internal/modules/cjs/loader:1505:32)\n    at Function._load (node:internal/modules/cjs/loader:1309:12)\n    at wrapModuleLoad (node:internal/modules/cjs/loader:254:19)\nok mutable Action tag is rejected\nok wrong Action SHA is rejected\nok unpinned CycloneDX tool is rejected\nok missing contents: read permissions are rejected\nok job-level write permissions are rejected\nok routine native packaging in the check job is rejected\nok routine artifact upload in the check job is rejected\nok package jobs without manual-only guards are rejected\nok security workflow push trigger is rejected\nok SBOM workflow push or pull_request trigger is rejected\nok missing relevant path filters are rejected\nok omitting the Windows native build script from routine path filters is rejected\nok documentation-only CI path is rejected\nok missing npm audit is rejected\nok missing maintained Electron test is rejected\nok missing policy test is rejected\nok pinned Electronegativity or ElectroNG reintroduction is rejected\nok obsolete Electron-SAST input of social-main.js, repository root, main.js, or js/ is rejected\nok obsolete Electron-SAST SARIF, CSV, exclusion, and eng-disable forms are rejected\nok missing complete-history Gitleaks checkout is rejected\nok missing pinned Gitleaks install or complete-history scan is rejected\nok wrong Gitleaks archive URL is rejected\nok wrong Gitleaks archive SHA-256 is rejected\nok wrong Gitleaks archive size is rejected\nok wrong Gitleaks version is rejected\nok mutable Gitleaks release name is rejected\nok Gitleaks extraction other than gitleaks under RUNNER_TEMP is rejected\nok Gitleaks install cleanup or deletion is rejected\nok Gitleaks scan must immediately follow install with the exact root command\nok Gitleaks range or log opts are rejected\nok non-blocking scanner behavior is rejected\nok ignore, baseline, and suppression flags are rejected\nok Gitleaks report path, artifact, or summary is rejected\nok Gitleaks Action, token, or comment environment is rejected\nok altered scanner exit behavior is rejected\nok altered Gitleaks exit behavior is rejected\nok SBOM output other than one validated CycloneDX JSON artifact is rejected\nok SBOM workflow npm ci, audit, generation, and validation stay required\nok SBOM workflow must not package a native application binary\nok security workflow does not upload artifacts or package binaries\nok CycloneDX validator accepts a bitbook-desktop JSON document\nok CycloneDX validator rejects SPDX, empty, and wrong-root documents\nok routine social check keeps offline syntax and Node tests only\nnot ok strict nine-line reviewed Gitleaks ratchet bytes and content are enforced\nPolicyError: wallet Rust source inventory is missing or extra\n    at checkRustWalletSourceInventory (<repo>/scripts/security-policy.js:2153:11)\n    at Object.checkRepository (<repo>/scripts/security-policy.js:2582:3)\n    at <repo>/test/securityPolicy.node.js:987:10\n    at run (<repo>/test/securityPolicy.node.js:3788:7)\n    at Object.<anonymous> (<repo>/test/securityPolicy.node.js:3803:3)\n    at Module._compile (node:internal/modules/cjs/loader:1781:14)\n    at Object..js (node:internal/modules/cjs/loader:1913:10)\n    at Module.load (node:internal/modules/cjs/loader:1505:32)\n    at Function._load (node:internal/modules/cjs/loader:1309:12)\n    at wrapModuleLoad (node:internal/modules/cjs/loader:254:19)\nok exact current-tree Gitleaks dir scan presence, order, and flags are enforced\nok inherited metrics and feedback loaders are structurally neutralized and unused by maintained social source\nok wallet contract package command and maintained-source policy are exact and fail closed\nok wallet maintained-source checker permits only exact literal pure sibling, crypto, and buffer module loads\nok wallet maintained-source filters are required on every routine social and security trigger\nok routine CI executes the exact wallet contract command and rejects its removal\nok WAL-005 Pay model package, syntax, top-level, and routine CI commands are exact\nok WAL-005 Pay model paths trigger routine workflows and remain policy-maintained\nok WAL-005 Pay model source policy permits no imports, I/O, authority, nondeterminism, or timers\nok wallet broker boundary package scripts and syntax checks are exact\nok wallet broker and preload paths are required on every routine workflow trigger\nok routine CI executes the named wallet broker suite and rejects omission\nok wallet boundary source policy allows only reviewed built-ins and forbids listeners, shell, and generic IPC\nok WAL-004 Rust test-harness manifest pins the exact toolchain, dependencies, and native features\nok WAL-004 Rust first-party source policy forbids unsafe and unreviewed authority\nok WAL-004 exact Rust test build lint and native compile commands are reserved\nok WAL-004 routine Linux CI is single-platform, locked, package-free, and path-filtered\nok WAL-004 RustSec and cargo-deny gates use exact tool versions and locked inputs\nok WAL-004 manual SBOM contains separately validated npm and Rust CycloneDX JSON artifacts\nok WAL-004 policy and validator changes trigger every applicable routine workflow\nnot ok WAL-004 Rust source inventory is exported closed and enumerated by repository policy\nPolicyError: wallet Rust source inventory is missing or extra\n    at Object.checkRustWalletSourceInventory (<repo>/scripts/security-policy.js:2153:11)\n    at <repo>/test/securityPolicy.node.js:2029:10\n    at run (<repo>/test/securityPolicy.node.js:3788:7)\n    at Object.<anonymous> (<repo>/test/securityPolicy.node.js:3803:3)\n    at Module._compile (node:internal/modules/cjs/loader:1781:14)\n    at Object..js (node:internal/modules/cjs/loader:1913:10)\n    at Module.load (node:internal/modules/cjs/loader:1505:32)\n    at Function._load (node:internal/modules/cjs/loader:1309:12)\n    at wrapModuleLoad (node:internal/modules/cjs/loader:254:19)\n    at Function.executeUserEntryPoint [as runMain] (node:internal/modules/run_main:171:5)\nok WAL-004 vault and native source policy requires reviewed secret and path primitives\nok WAL-004 cargo-deny policy is exact fail-closed and has no bypass lists\nok WAL-004 Rust SBOM validator accepts only a complete broker CycloneDX graph\nok WAL-006 manifest requires six exact defaults-off pins and the minimum direct feature union\nok WAL-006 manifest pins the exact production support APIs for RNG and SQLite\nok WAL-006 prepare NFC dependency is one exact defaults-off Unicode normalization pin\nok WAL-006 feature policy distinguishes compiled upstream PCZT capability from BitBook authority\nok WAL-006 preserves the exact historical seven-path Phase-C ZEC production inventory\nok WAL-006 policy rejects live-network and authority-bearing Rust snippets without denying upstream transitives\nnot ok BBD-WAL-008 closes the hardware target and current fourteen-path WAL-008/WAL-009 ZEC policy inventory\nAssertionError [ERR_ASSERTION]: Expected values to be strictly deep-equal:\n+ actual - expected\n\n  [\n    'wallet-broker/src/zec.rs',\n    'wallet-broker/src/zec/address.rs',\n    'wallet-broker/src/zec/fixture.rs',\n    'wallet-broker/src/zec/hardware.rs',\n    'wallet-broker/src/zec/prepare.rs',\n    'wallet-broker/src/zec/scan.rs',\n-   'wallet-broker/src/zec/spend.rs',\n-   'wallet-broker/src/zec/spend/cleanup_lifecycle_tests.rs',\n-   'wallet-broker/src/zec/spend/effects.rs',\n-   'wallet-broker/src/zec/spend/external_binding_tests.rs',\n-   'wallet-broker/src/zec/spend/verification_context_tests.rs',\n    'wallet-broker/src/zec/store.rs',\n    'wallet-broker/src/zec/test_support.rs',\n-   'wallet-broker/src/zec/test_support/cleanup_lifecycle_tests.rs'\n  ]\n\n    at <repo>/test/securityPolicy.node.js:2704:10\n    at run (<repo>/test/securityPolicy.node.js:3788:7)\n    at Object.<anonymous> (<repo>/test/securityPolicy.node.js:3803:3)\n    at Module._compile (node:internal/modules/cjs/loader:1781:14)\n    at Object..js (node:internal/modules/cjs/loader:1913:10)\n    at Module.load (node:internal/modules/cjs/loader:1505:32)\n    at Function._load (node:internal/modules/cjs/loader:1309:12)\n    at wrapModuleLoad (node:internal/modules/cjs/loader:254:19)\n    at Function.executeUserEntryPoint [as runMain] (node:internal/modules/run_main:171:5)\n    at node:internal/main/run_main_module:36:49\nok WAL-009 package policy accepts only exact maplibre-gl 6.8.0 and rejects Leaflet, extras, and unpinned versions\nok WAL-009 manifest requires the exact reviewed Orchard circuit pin and rejects independent mutations\nok WAL-009 signing target requires zec_sign_verify in the hardware/signing/xmr neighborhood\nnot ok WAL-009 production spend.rs accepts the exact reviewed TransactionExtractor statement and rejects independent mutations\nPolicyError: wallet Rust source wallet-broker/src/zec/spend.rs contains forbidden WAL-006 Zcash authority\n    at Object.checkRustWalletSource (<repo>/scripts/security-policy.js:2321:13)\n    at assertWal009ExtractorSourcePolicy (<repo>/test/securityPolicy.node.js:3183:10)\n    at <repo>/test/securityPolicy.node.js:3250:3\n    at run (<repo>/test/securityPolicy.node.js:3788:7)\n    at Object.<anonymous> (<repo>/test/securityPolicy.node.js:3803:3)\n    at Module._compile (node:internal/modules/cjs/loader:1781:14)\n    at Object..js (node:internal/modules/cjs/loader:1913:10)\n    at Module.load (node:internal/modules/cjs/loader:1505:32)\n    at Function._load (node:internal/modules/cjs/loader:1309:12)\n    at wrapModuleLoad (node:internal/modules/cjs/loader:254:19)\nnot ok WAL-009 verification-fixture spend/verification_context_tests.rs accepts the exact reviewed TransactionExtractor statement and rejects independent mutations\nPolicyError: wallet Rust source wallet-broker/src/zec/spend/verification_context_tests.rs contains forbidden WAL-006 Zcash authority\n    at Object.checkRustWalletSource (<repo>/scripts/security-policy.js:2321:13)\n    at assertWal009ExtractorSourcePolicy (<repo>/test/securityPolicy.node.js:3183:10)\n    at <repo>/test/securityPolicy.node.js:3260:3\n    at run (<repo>/test/securityPolicy.node.js:3788:7)\n    at Object.<anonymous> (<repo>/test/securityPolicy.node.js:3803:3)\n    at Module._compile (node:internal/modules/cjs/loader:1781:14)\n    at Object..js (node:internal/modules/cjs/loader:1913:10)\n    at Module.load (node:internal/modules/cjs/loader:1505:32)\n    at Function._load (node:internal/modules/cjs/loader:1309:12)\n    at wrapModuleLoad (node:internal/modules/cjs/loader:254:19)\nok RATE-001 quote-worker package, syntax, top-level, and routine CI commands are exact\nok RATE-001 quote-worker paths trigger routine workflows and remain policy-maintained\nok RATE-001 source policy permits only reviewed built-ins and forbids wallet, Electron, and default-on providers\nok RATE-001 policy exports exact provider pins and rejects unreviewed hosts and paths\nok WAL-007 manifest freezes one MD5 interop dependency, empty local-gate feature, and seven targets\nok WAL-007 permits no Phase-A XMR source and freezes the only Phase-C Rust inventory\nok WAL-007 production inventory grants no Electron or Node expansion\nok WAL-007 closed checker scans every present runtime source and rejects authority mutations\n6 security policy test(s) failed\n"
    },
    {
      "argv": [
        "node",
        "scripts/security-policy.js"
      ],
      "exit": 1,
      "output": "wallet Rust source inventory is missing or extra\n"
    },
    {
      "argv": [
        "target/security-tools/gitleaks-v8.30.1/gitleaks",
        "version"
      ],
      "exit": 0,
      "output": "8.30.1\n"
    },
    {
      "argv": [
        "target/security-tools/gitleaks-v8.30.1/gitleaks",
        "git",
        "--redact=100",
        "--no-banner",
        "."
      ],
      "exit": 0,
      "output": "11:49AM INF 5319 commits scanned.\n11:49AM INF scanned ~32962488 bytes (32.96 MB) in 2.58s\n11:49AM INF no leaks found\n"
    },
    {
      "argv": [
        "target/security-tools/gitleaks-v8.30.1/gitleaks",
        "dir",
        "--redact=100",
        "--no-banner",
        "."
      ],
      "exit": 0,
      "output": "11:50AM INF scanned ~1952492533 bytes (1.95 GB) in 18.4s\n11:50AM INF no leaks found\n"
    }
  ],
  "accepted": true,
  "version": "Hermes Agent v0.18.2 (2026.7.7.2) \u00b7 upstream 67764dc0 \u00b7 local 10b6d1a9 (+1 carried commit)\nInstall directory: <home>/.hermes/hermes-agent\nInstall method: git\nPython: 3.11.15\nOpenAI SDK: 2.24.0",
  "head": "07170db4402b02d708d1f47f493cb437e64c3ed7",
  "initial_status": "M package-lock.json\n M package.json\n M scripts/build-wallet-broker.js\n M scripts/security-policy.js\n M social-main.js\n M test/electronSecurity.node.js\n M test/securityPolicy.node.js\n M test/walletBrokerBuild.node.js\n M test/walletBrokerRuntime.node.js\n M test/walletStartup.node.js\n M test/walletStartupSmoke.node.js\n M test/walletSupervisor.node.js\n M test/walletSupervisorShutdown.node.js\n M wallet-broker/src/lib.rs\n M wallet-broker/src/runtime.rs\n M wallet-broker/supervisor.js\n?? docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md\n?? docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md\n?? docs/testing/BBD-WAL-013-ACCOUNTS-GREEN-UI-RED-01.md\n?? docs/testing/BBD-WAL-013-ACCOUNTS-RED-01.md\n?? docs/testing/BBD-WAL-013-FINAL-WINDOW-SECURITY-01.md\n?? docs/testing/BBD-WAL-013-FINAL-WINDOW-SECURITY-02.md\n?? docs/testing/BBD-WAL-013-FINAL-WINDOW-SECURITY-03.md\n?? docs/testing/BBD-WAL-013-NATIVE-RUNTIME-GREEN-01.md\n?? docs/testing/BBD-WAL-013-NATIVE-UI-DIAGNOSTIC-01.md\n?? docs/testing/BBD-WAL-013-NATIVE-UI-GREEN-01.md\n?? docs/testing/BBD-WAL-013-NATIVE-UI-GREEN-FINISH-01.md\n?? docs/testing/BBD-WAL-013-NATIVE-UI-GREEN-RESUME-01.md\n?? docs/testing/BBD-WAL-013-NATIVE-UI-GREEN-RESUME-02.md\n?? docs/testing/BBD-WAL-013-RUNTIME-JS-GREEN-01.md\n?? docs/testing/BBD-WAL-013-RUNTIME-JS-GREEN-FINISH-01.md\n?? docs/testing/BBD-WAL-013-RUNTIME-JS-RED-01.md\n?? test/fixtures/wallet-broker/x11-window.py\n?? test/walletAccountManagement.node.js\n?? test/walletAccountWindowSmoke.node.js\n?? wallet-broker/src/account_ui.rs\n?? wallet-broker/src/accounts.rs\n?? wallet-broker/tests/account_management.rs\n?? wallet-broker/tests/account_native_ui.rs",
  "session": {
    "id": "20260910_114808_10675d",
    "model": "meituan/longcat-2.0:free",
    "provider": "nous"
  },
  "input_hashes": {
    "social-main.js": "29fa41d88cb75d96b00bc2fe740d8701c9ddfbaf4ba2ae71be84b8755e192d30",
    "wallet-broker/supervisor.js": "1258444add1741dd6ba850c241871b8e68be71e8550dac14b0e8af6959b9f7a2",
    "scripts/build-wallet-broker.js": "4427ef3e713ec77c45daefe8d37b457ca8dc2c8529b30214dbf65628f43a1445",
    "wallet-broker/src/runtime.rs": "6788914524605f4669d20d65f7b6ef7282b0090a0696ea45f088ecc49dc1c962",
    "test/walletAccountManagement.node.js": "94a40ddbedea2298f707bb942218255ab6f55e7db9903c9401d18190f57eaaea",
    "test/walletStartup.node.js": "4a64be8628ff54e74f282703abfcaf286c9ae9cec96a15e1d13561a4fb0d501e",
    "test/walletBrokerBuild.node.js": "b1ac8ae3a1ca76c9580c92fec7cd1cbca0edb482246f072afcafb22a79e689de",
    "test/walletSupervisorShutdown.node.js": "f1156d8bf81a03a40ec9eb235666275ce85a3394b12f7b567bbb9da20a7543ef",
    "test/walletSupervisorTransport.node.js": "e69fdcfa4aaeeca03b4abf8ac100888ff0b42fbb6001f53e461a35a5b7550700",
    "test/walletSupervisor.node.js": "7fcb775d2db7ae8f28b6d9f7660c44a056e59891f3e552db80a248d843c76aa4",
    "test/electronSecurity.node.js": "d70d6337ffbe97dfeee1894895c757b804dc18ae819219e8742adda5358ab482",
    "test/walletPreload.node.js": "60b151344a01776e1d7f38238f69534426883503f9d627466bac7acbf4dc4f9e",
    "test/walletBrokerLaunchConfig.node.js": "90acd32c9863df0206364eb04f707d92a5979ee66c7e15b2ec7b8250a5fc2113",
    "wallet-preload.js": "3e6a18acf88dd5be4e6a88f326d6ace7a8071066480d9a70a2e8f89df035a1df",
    "wallet-broker/protocol.js": "79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4",
    "wallet-pay/model.js": "acf07238366f3e28253be8c9208fbb27e13e9c2687d374d1a6ae87e0d173fa5e",
    "wallet-broker/launch-config.js": "68adf92e2d543e5d51353477b1944c35d4189c5590ac581d3289f1bd72898af8",
    "package.json": "76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5",
    "package-lock.json": "0e32de43f3c1a31ae3d9e9c8b172d10cd37a03577dc7d1eaf4ac916967cfcdb8",
    "scripts/security-policy.js": "fec10759a73a3c675d8206da454abf4b0e8dca6b62da463f39a2db27b30dc078",
    "test/securityPolicy.node.js": "a58d85e5322f0cb01a78b636454f92547bd98e556013cc07712dc0cf64ea565c",
    "wallet-broker/src/accounts.rs": "f79eb3286cdccae2058e099c2d08b8e5a7167477a687e36609f91df4dc5b51f1",
    "wallet-broker/tests/account_management.rs": "7f876b1a9b7c4e65f8a6225d29867b2ae7dc4902f10cbbb829b4012675dff0d8",
    "wallet-broker/tests/account_native_ui.rs": "95584435ad547752481bfb2ebefbb83d734a6a0edbffb63a4ef8ec0681d166d5",
    "wallet-broker/src/lib.rs": "e37713c4fd1a528a9260b8b11d740f1a317198f2eaf62a534729f4a566a10437",
    "wallet-broker/Cargo.toml": "73e5e585eb2fd1ca867d66962460cfa010443886a4b59d8a33556ef2a4ff3503",
    "wallet-broker/Cargo.lock": "b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71",
    "wallet-broker/src/vault.rs": "f23247b0e46c68dfc125804fa981d16f7022464e1e4d94a8c1e62771a045be49",
    "wallet-broker/src/store.rs": "611d837641069a98d05b9e68c14bf11a37a5076de58bf6516188870eeab19236",
    "wallet-broker/src/session.rs": "42e4f335bb4080ad530d93dcc04d824b4ab54835be7f6c7cd68feba3f20ee227",
    "wallet-broker/src/native.rs": "a0ccb0b11e3e636cc28f9576ec0cf6da2d8c2ef5344370e5a7a2a9a9b5cc32e5",
    "wallet-broker/src/native_ui.rs": "c6607af3f36278f8d7deeb2b4276017177bbb4c695017f18bf4d2bcb412b268f",
    "wallet-broker/src/account_ui.rs": "f8a47778d00be2cd7a448aec33117172880bde6e70e220b9f783dcca1ad8c1c7",
    "wallet-broker/src/native_ui/zec_native_app_tests.rs": "d1d405f8667c6c9d184104d2023ffcff209f6d013e3920dd5c5ccd7e5a2c40f0",
    "wallet-broker/src/native_ui/zec_review_tests.rs": "2eec3b60eb1b9fbd32235832a867d1189928126ca79373fa9f4fa7fdf2ee87bf",
    "test/walletBrokerRuntime.node.js": "989c5f64716d22e8b2903d6bf736fdadcae253d9ed11c6c648aec0fc2de14592",
    "test/walletStartupSmoke.node.js": "a1c75efee5fcfdbeb9bf56dfc64f0e747ab6aa527d30ce416c5027bc2aa2a1ef",
    "test/walletAccountWindowSmoke.node.js": "c6e6ecc4616a28369f890255be2cc79f0d0d39909c561744f139aa6969549cbd",
    "test/fixtures/wallet-broker/x11-window.py": "040d8214487216e6b91ec514c9473e95cd66811c26306792f5ca7b081696b5d4",
    "wallet-broker/target/wal011-runtime-package-red-01.json": "5608cd092ae81d83e658aa9231e2751378f5f15fbde3a60c2b9dc176d5a370bd"
  },
  "final_artifact": {
    "sha256": "929751c8f956c8e764466279b9adb4ab8007c391206dd6478b18f1794853a11c",
    "bytes": 476621128,
    "mode": "0o755",
    "manifest_sha256": "f69decc1ccae3d28db169db96c655600acf2744fa1515e94cbd1b65f7487aa07"
  },
  "policy_baseline_failures": [
    "not ok committed workflows satisfy the fail-closed checker",
    "not ok strict nine-line reviewed Gitleaks ratchet bytes and content are enforced",
    "not ok WAL-004 Rust source inventory is exported closed and enumerated by repository policy",
    "not ok BBD-WAL-008 closes the hardware target and current fourteen-path WAL-008/WAL-009 ZEC policy inventory",
    "not ok WAL-009 production spend.rs accepts the exact reviewed TransactionExtractor statement and rejects independent mutations",
    "not ok WAL-009 verification-fixture spend/verification_context_tests.rs accepts the exact reviewed TransactionExtractor statement and rejects independent mutations"
  ],
  "policy_current_failures": [
    "not ok committed workflows satisfy the fail-closed checker",
    "not ok strict nine-line reviewed Gitleaks ratchet bytes and content are enforced",
    "not ok WAL-004 Rust source inventory is exported closed and enumerated by repository policy",
    "not ok BBD-WAL-008 closes the hardware target and current fourteen-path WAL-008/WAL-009 ZEC policy inventory",
    "not ok WAL-009 production spend.rs accepts the exact reviewed TransactionExtractor statement and rejects independent mutations",
    "not ok WAL-009 verification-fixture spend/verification_context_tests.rs accepts the exact reviewed TransactionExtractor statement and rejects independent mutations"
  ],
  "final_hashes": {
    "social-main.js": "29fa41d88cb75d96b00bc2fe740d8701c9ddfbaf4ba2ae71be84b8755e192d30",
    "wallet-broker/supervisor.js": "1258444add1741dd6ba850c241871b8e68be71e8550dac14b0e8af6959b9f7a2",
    "scripts/build-wallet-broker.js": "4427ef3e713ec77c45daefe8d37b457ca8dc2c8529b30214dbf65628f43a1445",
    "wallet-broker/src/runtime.rs": "6788914524605f4669d20d65f7b6ef7282b0090a0696ea45f088ecc49dc1c962",
    "test/walletAccountManagement.node.js": "94a40ddbedea2298f707bb942218255ab6f55e7db9903c9401d18190f57eaaea",
    "test/walletStartup.node.js": "4a64be8628ff54e74f282703abfcaf286c9ae9cec96a15e1d13561a4fb0d501e",
    "test/walletBrokerBuild.node.js": "b1ac8ae3a1ca76c9580c92fec7cd1cbca0edb482246f072afcafb22a79e689de",
    "test/walletSupervisorShutdown.node.js": "f1156d8bf81a03a40ec9eb235666275ce85a3394b12f7b567bbb9da20a7543ef",
    "test/walletSupervisorTransport.node.js": "e69fdcfa4aaeeca03b4abf8ac100888ff0b42fbb6001f53e461a35a5b7550700",
    "test/walletSupervisor.node.js": "7fcb775d2db7ae8f28b6d9f7660c44a056e59891f3e552db80a248d843c76aa4",
    "test/electronSecurity.node.js": "d70d6337ffbe97dfeee1894895c757b804dc18ae819219e8742adda5358ab482",
    "test/walletPreload.node.js": "60b151344a01776e1d7f38238f69534426883503f9d627466bac7acbf4dc4f9e",
    "test/walletBrokerLaunchConfig.node.js": "90acd32c9863df0206364eb04f707d92a5979ee66c7e15b2ec7b8250a5fc2113",
    "wallet-preload.js": "3e6a18acf88dd5be4e6a88f326d6ace7a8071066480d9a70a2e8f89df035a1df",
    "wallet-broker/protocol.js": "79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4",
    "wallet-pay/model.js": "acf07238366f3e28253be8c9208fbb27e13e9c2687d374d1a6ae87e0d173fa5e",
    "wallet-broker/launch-config.js": "68adf92e2d543e5d51353477b1944c35d4189c5590ac581d3289f1bd72898af8",
    "package.json": "76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5",
    "package-lock.json": "0e32de43f3c1a31ae3d9e9c8b172d10cd37a03577dc7d1eaf4ac916967cfcdb8",
    "scripts/security-policy.js": "fec10759a73a3c675d8206da454abf4b0e8dca6b62da463f39a2db27b30dc078",
    "test/securityPolicy.node.js": "a58d85e5322f0cb01a78b636454f92547bd98e556013cc07712dc0cf64ea565c",
    "wallet-broker/src/accounts.rs": "f79eb3286cdccae2058e099c2d08b8e5a7167477a687e36609f91df4dc5b51f1",
    "wallet-broker/tests/account_management.rs": "7f876b1a9b7c4e65f8a6225d29867b2ae7dc4902f10cbbb829b4012675dff0d8",
    "wallet-broker/tests/account_native_ui.rs": "95584435ad547752481bfb2ebefbb83d734a6a0edbffb63a4ef8ec0681d166d5",
    "wallet-broker/src/lib.rs": "e37713c4fd1a528a9260b8b11d740f1a317198f2eaf62a534729f4a566a10437",
    "wallet-broker/Cargo.toml": "73e5e585eb2fd1ca867d66962460cfa010443886a4b59d8a33556ef2a4ff3503",
    "wallet-broker/Cargo.lock": "b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71",
    "wallet-broker/src/vault.rs": "f23247b0e46c68dfc125804fa981d16f7022464e1e4d94a8c1e62771a045be49",
    "wallet-broker/src/store.rs": "611d837641069a98d05b9e68c14bf11a37a5076de58bf6516188870eeab19236",
    "wallet-broker/src/session.rs": "42e4f335bb4080ad530d93dcc04d824b4ab54835be7f6c7cd68feba3f20ee227",
    "wallet-broker/src/native.rs": "a0ccb0b11e3e636cc28f9576ec0cf6da2d8c2ef5344370e5a7a2a9a9b5cc32e5",
    "wallet-broker/src/native_ui.rs": "c6607af3f36278f8d7deeb2b4276017177bbb4c695017f18bf4d2bcb412b268f",
    "wallet-broker/src/account_ui.rs": "f8a47778d00be2cd7a448aec33117172880bde6e70e220b9f783dcca1ad8c1c7",
    "wallet-broker/src/native_ui/zec_native_app_tests.rs": "d1d405f8667c6c9d184104d2023ffcff209f6d013e3920dd5c5ccd7e5a2c40f0",
    "wallet-broker/src/native_ui/zec_review_tests.rs": "2eec3b60eb1b9fbd32235832a867d1189928126ca79373fa9f4fa7fdf2ee87bf",
    "test/walletBrokerRuntime.node.js": "989c5f64716d22e8b2903d6bf736fdadcae253d9ed11c6c648aec0fc2de14592",
    "test/walletStartupSmoke.node.js": "a1c75efee5fcfdbeb9bf56dfc64f0e747ab6aa527d30ce416c5027bc2aa2a1ef",
    "test/walletAccountWindowSmoke.node.js": "c6e6ecc4616a28369f890255be2cc79f0d0d39909c561744f139aa6969549cbd",
    "test/fixtures/wallet-broker/x11-window.py": "040d8214487216e6b91ec514c9473e95cd66811c26306792f5ca7b081696b5d4",
    "wallet-broker/target/wal011-runtime-package-red-01.json": "5608cd092ae81d83e658aa9231e2751378f5f15fbde3a60c2b9dc176d5a370bd"
  }
}
```
