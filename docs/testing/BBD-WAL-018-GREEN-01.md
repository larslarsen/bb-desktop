# WAL018 cleanup, endpoint GREEN, falsification and rebuilt broker

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
      "output": "Hermes Agent v0.18.2 (2026.7.7.2) \u00b7 upstream ad03f20d \u00b7 local 10b6d1a9 (+1 carried commit)\nInstall directory: <home>/.hermes/hermes-agent\nInstall method: git\nPython: 3.11.15\nOpenAI SDK: 2.24.0\n",
      "timeout": false
    },
    {
      "argv": [
        "git",
        "rev-parse",
        "HEAD"
      ],
      "exit": 0,
      "output": "acf53cf26c41992f198bdb6a4cde5fb04ccffb8b\n",
      "timeout": false
    },
    {
      "argv": [
        "python3",
        "docs/handoff/WAL018_QUIET.py"
      ],
      "exit": 0,
      "output": "{\"owned_diagnostic_processes\": 0}\n",
      "timeout": false
    },
    {
      "argv": [
        "python3",
        "docs/handoff/WAL018_RESTORE.py"
      ],
      "exit": 0,
      "output": "temporary_helper_restored=true\n",
      "timeout": false
    },
    {
      "argv": [
        "python3",
        "docs/handoff/WAL018_CLEANUP.py"
      ],
      "exit": 0,
      "output": "{\"private_clone_removed\": true, \"temporary_source_removed\": true, \"production_helper_restored\": true}\n",
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
      "output": "   Compiling bitbook-wallet-broker v0.1.0 (<repo>/wallet-broker)\n    Finished `test` profile [unoptimized + debuginfo] target(s) in 6.87s\n     Running tests/account_native_ui.rs (wallet-broker/target/debug/deps/account_native_ui-e7381b3c5844547a)\n\nrunning 16 tests\ntest export_chooser_cancel_skips_port_and_chosen_path_reaches_port ... ok\ntest empty_and_populated_list_paints_notices_and_selection_controls ... ok\ntest copy_cut_undo_and_platform_events_never_expose_secret ... ok\ntest unlock_wrong_then_correct_clears_password_and_lock_calls ... ok\ntest wal015_cancel_stale_job_lock_back_and_hide_invalidate_sync_results ... ok\ntest small_and_tall_layouts_keep_action_controls_usable ... ok\ntest create_form_accepts_matching_unicode_and_rejects_invalid ... ok\ntest restore_requires_distinct_confirm_and_cannot_replay ... ok\ntest wal015_sync_pointer_flow_is_explicit_editable_and_paints_progress_then_received_funds ... ok\ntest wal018_default_server_survives_reentering_sync ... ok\ntest wal016_idle_locked_sync_stays_visible_and_inline_unlock_reveals_the_same_completed_job ... ok\ntest window_control_reopen_quit_drop_and_closed_list_errors ... ok\ntest wal014_receive_scene_issues_once_paints_honestly_and_copies_release_output ... ok\ntest wal014_receive_state_clears_on_lock_catalog_change_error_selection_and_hide ... ok\ntest shared_port_and_local_manager_create_unlock_lock_and_restart ... ok\ntest wal014_shared_port_pointer_receive_and_copy_persist_real_issuance ... ok\n\ntest result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 7.28s\n\n",
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
        "account_native_ui",
        "wal018_default_server_survives_reentering_sync",
        "--",
        "--exact"
      ],
      "exit": 101,
      "output": "   Compiling bitbook-wallet-broker v0.1.0 (<repo>/wallet-broker)\n    Finished `test` profile [unoptimized + debuginfo] target(s) in 20.78s\n     Running tests/account_native_ui.rs (wallet-broker/target/debug/deps/account_native_ui-e7381b3c5844547a)\n\nrunning 1 test\ntest wal018_default_server_survives_reentering_sync ... FAILED\n\nfailures:\n\n---- wal018_default_server_survives_reentering_sync stdout ----\n\nthread 'wal018_default_server_survives_reentering_sync' (274331) panicked at tests/account_native_ui.rs:317:43:\nexact label must be painted\nnote: run with `RUST_BACKTRACE=1` environment variable to display a backtrace\n\n\nfailures:\n    wal018_default_server_survives_reentering_sync\n\ntest result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.06s\n\nerror: test failed, to rerun pass `--test account_native_ui`\n",
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
        "account_native_ui",
        "wal018_default_server_survives_reentering_sync",
        "--",
        "--exact"
      ],
      "exit": 0,
      "output": "   Compiling bitbook-wallet-broker v0.1.0 (<repo>/wallet-broker)\n    Finished `test` profile [unoptimized + debuginfo] target(s) in 6.09s\n     Running tests/account_native_ui.rs (wallet-broker/target/debug/deps/account_native_ui-e7381b3c5844547a)\n\nrunning 1 test\ntest wal018_default_server_survives_reentering_sync ... ok\n\ntest result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.07s\n\n",
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
      "output": "    Checking bitbook-wallet-broker v0.1.0 (<repo>/wallet-broker)\n    Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.20s\n",
      "timeout": false
    },
    {
      "argv": [
        "node",
        "scripts/build-wallet-broker.js"
      ],
      "exit": 0,
      "output": "    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.27s\n",
      "timeout": false
    },
    {
      "argv": [
        "node",
        "test/securityPolicy.node.js"
      ],
      "exit": 1,
      "output": "ok required policy, workflow, and validator sources exist\nok checker constants match the ticketed Action and tool pins\nok YAML parser keeps on: as a mapping key and preserves block scalars\nnot ok committed workflows satisfy the fail-closed checker\nPolicyError: wallet Rust source inventory is missing or extra\n    at checkRustWalletSourceInventory (<repo>/scripts/security-policy.js:2161:11)\n    at Object.checkRepository (<repo>/scripts/security-policy.js:2590:3)\n    at <repo>/test/securityPolicy.node.js:285:10\n    at run (<repo>/test/securityPolicy.node.js:3794:7)\n    at Object.<anonymous> (<repo>/test/securityPolicy.node.js:3809:3)\n    at Module._compile (node:internal/modules/cjs/loader:1781:14)\n    at Object..js (node:internal/modules/cjs/loader:1913:10)\n    at Module.load (node:internal/modules/cjs/loader:1505:32)\n    at Function._load (node:internal/modules/cjs/loader:1309:12)\n    at wrapModuleLoad (node:internal/modules/cjs/loader:254:19)\nok mutable Action tag is rejected\nok wrong Action SHA is rejected\nok unpinned CycloneDX tool is rejected\nok missing contents: read permissions are rejected\nok job-level write permissions are rejected\nok routine native packaging in the check job is rejected\nok routine artifact upload in the check job is rejected\nok package jobs without manual-only guards are rejected\nok security workflow push trigger is rejected\nok SBOM workflow push or pull_request trigger is rejected\nok missing relevant path filters are rejected\nok omitting the Windows native build script from routine path filters is rejected\nok documentation-only CI path is rejected\nok missing npm audit is rejected\nok missing maintained Electron test is rejected\nok missing policy test is rejected\nok pinned Electronegativity or ElectroNG reintroduction is rejected\nok obsolete Electron-SAST input of social-main.js, repository root, main.js, or js/ is rejected\nok obsolete Electron-SAST SARIF, CSV, exclusion, and eng-disable forms are rejected\nok missing complete-history Gitleaks checkout is rejected\nok missing pinned Gitleaks install or complete-history scan is rejected\nok wrong Gitleaks archive URL is rejected\nok wrong Gitleaks archive SHA-256 is rejected\nok wrong Gitleaks archive size is rejected\nok wrong Gitleaks version is rejected\nok mutable Gitleaks release name is rejected\nok Gitleaks extraction other than gitleaks under RUNNER_TEMP is rejected\nok Gitleaks install cleanup or deletion is rejected\nok Gitleaks scan must immediately follow install with the exact root command\nok Gitleaks range or log opts are rejected\nok non-blocking scanner behavior is rejected\nok ignore, baseline, and suppression flags are rejected\nok Gitleaks report path, artifact, or summary is rejected\nok Gitleaks Action, token, or comment environment is rejected\nok altered scanner exit behavior is rejected\nok altered Gitleaks exit behavior is rejected\nok SBOM output other than one validated CycloneDX JSON artifact is rejected\nok SBOM workflow npm ci, audit, generation, and validation stay required\nok SBOM workflow must not package a native application binary\nok security workflow does not upload artifacts or package binaries\nok CycloneDX validator accepts a bitbook-desktop JSON document\nok CycloneDX validator rejects SPDX, empty, and wrong-root documents\nok routine social check keeps offline syntax and Node tests only\nnot ok strict nine-line reviewed Gitleaks ratchet bytes and content are enforced\nPolicyError: wallet Rust source inventory is missing or extra\n    at checkRustWalletSourceInventory (<repo>/scripts/security-policy.js:2161:11)\n    at Object.checkRepository (<repo>/scripts/security-policy.js:2590:3)\n    at <repo>/test/securityPolicy.node.js:987:10\n    at run (<repo>/test/securityPolicy.node.js:3794:7)\n    at Object.<anonymous> (<repo>/test/securityPolicy.node.js:3809:3)\n    at Module._compile (node:internal/modules/cjs/loader:1781:14)\n    at Object..js (node:internal/modules/cjs/loader:1913:10)\n    at Module.load (node:internal/modules/cjs/loader:1505:32)\n    at Function._load (node:internal/modules/cjs/loader:1309:12)\n    at wrapModuleLoad (node:internal/modules/cjs/loader:254:19)\nok exact current-tree Gitleaks dir scan presence, order, and flags are enforced\nok inherited metrics and feedback loaders are structurally neutralized and unused by maintained social source\nok wallet contract package command and maintained-source policy are exact and fail closed\nok wallet maintained-source checker permits only exact literal pure sibling, crypto, and buffer module loads\nok wallet maintained-source filters are required on every routine social and security trigger\nok routine CI executes the exact wallet contract command and rejects its removal\nok WAL-005 Pay model package, syntax, top-level, and routine CI commands are exact\nok WAL-005 Pay model paths trigger routine workflows and remain policy-maintained\nok WAL-005 Pay model source policy permits no imports, I/O, authority, nondeterminism, or timers\nok wallet broker boundary package scripts and syntax checks are exact\nok wallet broker and preload paths are required on every routine workflow trigger\nok routine CI executes the named wallet broker suite and rejects omission\nok wallet boundary source policy allows only reviewed built-ins and forbids listeners, shell, and generic IPC\nok WAL-004 Rust test-harness manifest pins the exact toolchain, dependencies, and native features\nok WAL-004 Rust first-party source policy forbids unsafe and unreviewed authority\nok WAL-004 exact Rust test build lint and native compile commands are reserved\nok WAL-004 routine Linux CI is single-platform, locked, package-free, and path-filtered\nok WAL-004 RustSec and cargo-deny gates use exact tool versions and locked inputs\nok WAL-004 manual SBOM contains separately validated npm and Rust CycloneDX JSON artifacts\nok WAL-004 policy and validator changes trigger every applicable routine workflow\nnot ok WAL-004 Rust source inventory is exported closed and enumerated by repository policy\nPolicyError: wallet Rust source inventory is missing or extra\n    at Object.checkRustWalletSourceInventory (<repo>/scripts/security-policy.js:2161:11)\n    at <repo>/test/securityPolicy.node.js:2029:10\n    at run (<repo>/test/securityPolicy.node.js:3794:7)\n    at Object.<anonymous> (<repo>/test/securityPolicy.node.js:3809:3)\n    at Module._compile (node:internal/modules/cjs/loader:1781:14)\n    at Object..js (node:internal/modules/cjs/loader:1913:10)\n    at Module.load (node:internal/modules/cjs/loader:1505:32)\n    at Function._load (node:internal/modules/cjs/loader:1309:12)\n    at wrapModuleLoad (node:internal/modules/cjs/loader:254:19)\n    at Function.executeUserEntryPoint [as runMain] (node:internal/modules/run_main:171:5)\nok WAL-004 vault and native source policy requires reviewed secret and path primitives\nok WAL-004 cargo-deny policy is exact fail-closed and has no bypass lists\nok WAL-004 Rust SBOM validator accepts only a complete broker CycloneDX graph\nok WAL-006 manifest requires six exact defaults-off pins and the minimum direct feature union\nok WAL-006 manifest pins the exact production support APIs for RNG and SQLite\nok WAL-006 prepare NFC dependency is one exact defaults-off Unicode normalization pin\nok WAL-006 feature policy distinguishes compiled upstream PCZT capability from BitBook authority\nok WAL-006 preserves the exact historical seven-path Phase-C ZEC production inventory\nok WAL-006 policy rejects live-network and authority-bearing Rust snippets without denying upstream transitives\nnot ok BBD-WAL-008 closes the hardware target and current fourteen-path WAL-008/WAL-009 ZEC policy inventory\nAssertionError [ERR_ASSERTION]: Expected values to be strictly deep-equal:\n+ actual - expected\n\n  [\n    'wallet-broker/src/zec.rs',\n    'wallet-broker/src/zec/address.rs',\n    'wallet-broker/src/zec/fixture.rs',\n    'wallet-broker/src/zec/hardware.rs',\n    'wallet-broker/src/zec/prepare.rs',\n    'wallet-broker/src/zec/scan.rs',\n-   'wallet-broker/src/zec/spend.rs',\n-   'wallet-broker/src/zec/spend/cleanup_lifecycle_tests.rs',\n-   'wallet-broker/src/zec/spend/effects.rs',\n-   'wallet-broker/src/zec/spend/external_binding_tests.rs',\n-   'wallet-broker/src/zec/spend/verification_context_tests.rs',\n    'wallet-broker/src/zec/store.rs',\n    'wallet-broker/src/zec/test_support.rs',\n-   'wallet-broker/src/zec/test_support/cleanup_lifecycle_tests.rs'\n  ]\n\n    at <repo>/test/securityPolicy.node.js:2707:10\n    at run (<repo>/test/securityPolicy.node.js:3794:7)\n    at Object.<anonymous> (<repo>/test/securityPolicy.node.js:3809:3)\n    at Module._compile (node:internal/modules/cjs/loader:1781:14)\n    at Object..js (node:internal/modules/cjs/loader:1913:10)\n    at Module.load (node:internal/modules/cjs/loader:1505:32)\n    at Function._load (node:internal/modules/cjs/loader:1309:12)\n    at wrapModuleLoad (node:internal/modules/cjs/loader:254:19)\n    at Function.executeUserEntryPoint [as runMain] (node:internal/modules/run_main:171:5)\n    at node:internal/main/run_main_module:36:49\nok WAL-009 package policy accepts only exact maplibre-gl 6.8.0 and rejects Leaflet, extras, and unpinned versions\nok WAL-009 manifest requires the exact reviewed Orchard circuit pin and rejects independent mutations\nok WAL-009 signing target requires zec_sign_verify in the hardware/signing/xmr neighborhood\nnot ok WAL-009 production spend.rs accepts the exact reviewed TransactionExtractor statement and rejects independent mutations\nPolicyError: wallet Rust source wallet-broker/src/zec/spend.rs contains forbidden WAL-006 Zcash authority\n    at Object.checkRustWalletSource (<repo>/scripts/security-policy.js:2329:13)\n    at assertWal009ExtractorSourcePolicy (<repo>/test/securityPolicy.node.js:3186:10)\n    at <repo>/test/securityPolicy.node.js:3253:3\n    at run (<repo>/test/securityPolicy.node.js:3794:7)\n    at Object.<anonymous> (<repo>/test/securityPolicy.node.js:3809:3)\n    at Module._compile (node:internal/modules/cjs/loader:1781:14)\n    at Object..js (node:internal/modules/cjs/loader:1913:10)\n    at Module.load (node:internal/modules/cjs/loader:1505:32)\n    at Function._load (node:internal/modules/cjs/loader:1309:12)\n    at wrapModuleLoad (node:internal/modules/cjs/loader:254:19)\nnot ok WAL-009 verification-fixture spend/verification_context_tests.rs accepts the exact reviewed TransactionExtractor statement and rejects independent mutations\nPolicyError: wallet Rust source wallet-broker/src/zec/spend/verification_context_tests.rs contains forbidden WAL-006 Zcash authority\n    at Object.checkRustWalletSource (<repo>/scripts/security-policy.js:2329:13)\n    at assertWal009ExtractorSourcePolicy (<repo>/test/securityPolicy.node.js:3186:10)\n    at <repo>/test/securityPolicy.node.js:3263:3\n    at run (<repo>/test/securityPolicy.node.js:3794:7)\n    at Object.<anonymous> (<repo>/test/securityPolicy.node.js:3809:3)\n    at Module._compile (node:internal/modules/cjs/loader:1781:14)\n    at Object..js (node:internal/modules/cjs/loader:1913:10)\n    at Module.load (node:internal/modules/cjs/loader:1505:32)\n    at Function._load (node:internal/modules/cjs/loader:1309:12)\n    at wrapModuleLoad (node:internal/modules/cjs/loader:254:19)\nok RATE-001 quote-worker package, syntax, top-level, and routine CI commands are exact\nok RATE-001 quote-worker paths trigger routine workflows and remain policy-maintained\nok RATE-001 source policy permits only reviewed built-ins and forbids wallet, Electron, and default-on providers\nok RATE-001 policy exports exact provider pins and rejects unreviewed hosts and paths\nok WAL-007 manifest freezes one MD5 interop dependency, empty local-gate feature, and seven targets\nok WAL-007 permits no Phase-A XMR source and freezes the only Phase-C Rust inventory\nok WAL-007 production inventory grants no Electron or Node expansion\nok WAL-007 closed checker scans every present runtime source and rejects authority mutations\n6 security policy test(s) failed\n",
      "timeout": false
    },
    {
      "argv": [
        "node",
        "scripts/security-policy.js"
      ],
      "exit": 1,
      "output": "wallet Rust source inventory is missing or extra\n",
      "timeout": false
    },
    {
      "argv": [
        "target/security-tools/gitleaks-v8.30.1/gitleaks",
        "dir",
        "--redact=0",
        "--no-banner",
        "--report-format",
        "json",
        "--report-path",
        "wallet-broker/target/wal018-green-01-gitleaks-dir.json",
        "."
      ],
      "exit": 1,
      "output": "9:46AM INF scanned ~2173384604 bytes (2.17 GB) in 22s\n9:46AM WRN leaks found: 352\n",
      "timeout": false
    }
  ],
  "session": {
    "id": "20260911_094455_113584",
    "model": "meituan/longcat-2.0:free",
    "provider": "nous"
  },
  "version": "Hermes Agent v0.18.2 (2026.7.7.2) \u00b7 upstream ad03f20d \u00b7 local 10b6d1a9 (+1 carried commit)\nInstall directory: <home>/.hermes/hermes-agent\nInstall method: git\nPython: 3.11.15\nOpenAI SDK: 2.24.0",
  "head": "acf53cf26c41992f198bdb6a4cde5fb04ccffb8b",
  "input_hashes": {
    "wallet-broker/src/accounts.rs": "8941884f837453bf71357f53341fae2c1334b8d10d017680053f9f8a44fb415d",
    "wallet-broker/src/account_ui.rs": "cbfaf77b02ea725422426f2b1276e738308e4a110654e56fa05f2008070b6e0e",
    "wallet-broker/src/session.rs": "9f88b3b0eca91d341a1c025bc8394a25e456350be58e99d9ac9417e31eef21af",
    "wallet-broker/Cargo.toml": "435907f00d7819bdf7dbe9ff9e367522f7f6eb8b6019dbde9e58212bbaea041b",
    "wallet-broker/Cargo.lock": "a5d53b729433a06f60edcef51e608629c7365326d06fef2bbac8f2b23f048c2a",
    "wallet-broker/tests/account_management.rs": "f1f9c70dc89e330fca6f5d292cfbabd626ef0c760a8800f522b48c070e57ddf0",
    "wallet-broker/tests/account_native_ui.rs": "67323f8889b789bcb9d01f9d26b4af8016da5bbf94d702bf8ae886ba5487ebb7",
    "package.json": "76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5",
    "package-lock.json": "0e32de43f3c1a31ae3d9e9c8b172d10cd37a03577dc7d1eaf4ac916967cfcdb8",
    "scripts/security-policy.js": "414a5645d8621b5d81cdeba7bb7461ec25647a5a1e5532295d6f75e7b8762532",
    "test/securityPolicy.node.js": "ff449a86e04570f8ff07bd280db6787c62f645f301d4235a65e825f3af7bbb9c",
    "docs/handoff/WAL018_QUIET.py": "fc58d8fc881f9c307b4fa688e69761fb1e6562a3319a41bbe09a18d22cd9fbaa",
    "docs/handoff/WAL018_RESTORE.py": "6e699e9d0caecee6df35ba37cff326d879fe8e38a5ca8e2d6967df990482633c",
    "docs/handoff/WAL018_CLEANUP.py": "1d5ad94fb001454544b0adf011e56ccd7af74a9ab89609af36a012c7015e0981",
    "docs/handoff/WAL016_RUNNER.py": "ebc5ae611b596709eda801ed3087d808a50cf79bd8cda55520eb02ccf74656f0"
  },
  "formatted": [],
  "falsification_restored": true,
  "inherited_policy_failures": [
    "not ok committed workflows satisfy the fail-closed checker",
    "not ok strict nine-line reviewed Gitleaks ratchet bytes and content are enforced",
    "not ok WAL-004 Rust source inventory is exported closed and enumerated by repository policy",
    "not ok BBD-WAL-008 closes the hardware target and current fourteen-path WAL-008/WAL-009 ZEC policy inventory",
    "not ok WAL-009 production spend.rs accepts the exact reviewed TransactionExtractor statement and rejects independent mutations",
    "not ok WAL-009 verification-fixture spend/verification_context_tests.rs accepts the exact reviewed TransactionExtractor statement and rejects independent mutations"
  ],
  "artifacts": {
    "wallet-broker/src/zec/test_support.rs": {
      "sha256": "de157d4a78c4c452e0e19122200997e6676c82099ddb25bfebe84781470f0791",
      "bytes": 183224
    },
    "wallet-broker/target/app-resources/wallet-broker/bitbook-wallet-broker": {
      "sha256": "65972fcab1661872232bb435c1ffe0e479f93b4f74df62c3eaafadb7c6b082a1",
      "bytes": 552201504
    },
    "wallet-broker/target/app-resources/wallet-broker/manifest.json": {
      "sha256": "bbe28eb0b2b1d973c7e390b4a01732b3588efd230fac3e2106b0ed52bb1ce34b",
      "bytes": 115
    }
  },
  "scans": [
    {
      "mode": "dir",
      "exit": 1,
      "public_checksum_false_positives": 352,
      "actual_credentials": 0,
      "classification": "docs/architecture/BBD-WAL-015-FINAL-SCAN-REVIEW.md"
    }
  ],
  "final_hashes": {
    "wallet-broker/src/accounts.rs": "8941884f837453bf71357f53341fae2c1334b8d10d017680053f9f8a44fb415d",
    "wallet-broker/src/account_ui.rs": "cbfaf77b02ea725422426f2b1276e738308e4a110654e56fa05f2008070b6e0e",
    "wallet-broker/src/session.rs": "9f88b3b0eca91d341a1c025bc8394a25e456350be58e99d9ac9417e31eef21af",
    "wallet-broker/Cargo.toml": "435907f00d7819bdf7dbe9ff9e367522f7f6eb8b6019dbde9e58212bbaea041b",
    "wallet-broker/Cargo.lock": "a5d53b729433a06f60edcef51e608629c7365326d06fef2bbac8f2b23f048c2a",
    "wallet-broker/tests/account_management.rs": "f1f9c70dc89e330fca6f5d292cfbabd626ef0c760a8800f522b48c070e57ddf0",
    "wallet-broker/tests/account_native_ui.rs": "67323f8889b789bcb9d01f9d26b4af8016da5bbf94d702bf8ae886ba5487ebb7",
    "package.json": "76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5",
    "package-lock.json": "0e32de43f3c1a31ae3d9e9c8b172d10cd37a03577dc7d1eaf4ac916967cfcdb8",
    "scripts/security-policy.js": "414a5645d8621b5d81cdeba7bb7461ec25647a5a1e5532295d6f75e7b8762532",
    "test/securityPolicy.node.js": "ff449a86e04570f8ff07bd280db6787c62f645f301d4235a65e825f3af7bbb9c",
    "docs/handoff/WAL018_QUIET.py": "fc58d8fc881f9c307b4fa688e69761fb1e6562a3319a41bbe09a18d22cd9fbaa",
    "docs/handoff/WAL018_RESTORE.py": "6e699e9d0caecee6df35ba37cff326d879fe8e38a5ca8e2d6967df990482633c",
    "docs/handoff/WAL018_CLEANUP.py": "1d5ad94fb001454544b0adf011e56ccd7af74a9ab89609af36a012c7015e0981",
    "docs/handoff/WAL016_RUNNER.py": "ebc5ae611b596709eda801ed3087d808a50cf79bd8cda55520eb02ccf74656f0"
  }
}
```
