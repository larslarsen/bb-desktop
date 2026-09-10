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
      "output": "9a641f021fb93ba75cb952f29164cba181099e48\n"
    },
    {
      "argv": [
        "git",
        "status",
        "--short"
      ],
      "exit": 0,
      "output": " M package-lock.json\n M package.json\n M scripts/security-policy.js\n M test/securityPolicy.node.js\n M wallet-broker/src/account_ui.rs\n M wallet-broker/src/accounts.rs\n M wallet-broker/src/session.rs\n M wallet-broker/src/zec.rs\n M wallet-broker/tests/account_management.rs\n M wallet-broker/tests/account_native_ui.rs\n?? docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md\n?? docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md\n?? docs/testing/BBD-WAL-014-RECEIVE-GREEN-01.md\n?? docs/testing/BBD-WAL-014-RECEIVE-GREEN-02.md\n?? docs/testing/BBD-WAL-014-RECEIVE-RED-01.md\n"
    },
    {
      "argv": [
        "node",
        "-e",
        "const {spawnSync}=require('child_process'); const r=spawnSync('rustup',['--version'],{encoding:'utf8'}); console.log(JSON.stringify({status:r.status,error:r.error&&r.error.code,stdout:r.stdout,stderr:r.stderr}));"
      ],
      "exit": 0,
      "output": "{\"status\":null,\"error\":\"ENOENT\"}\n"
    },
    {
      "argv": [
        "node",
        "scripts/build-wallet-broker.js"
      ],
      "exit": 0,
      "output": "    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.21s\n"
    },
    {
      "argv": [
        "node",
        "test/walletAccountManagement.node.js"
      ],
      "exit": 0,
      "output": "ok ready menu registers Wallet/Manage accounts; bound click dispatches account.manage{}; failures stay closed\nok quit and repeated ready cannot open the account window\nok supervisor spawn env adds fixed Linux renderer settings only with sanitized DISPLAY\nok main copies the six allowlisted process.env strings and omits WAYLAND_DISPLAY\nok dispatcher accepts account.manage{} only while bound and rejects secret-bearing methods\nok graceful EOF ends stdin once, waits 1000/1250/1500, and resolves only on close\nok graceful asynchronous and synchronous close are safe; missing or throwing end falls back immediately\nBitBook wallet account management tests passed (7).\n"
    },
    {
      "argv": [
        "node",
        "test/walletStartup.node.js"
      ],
      "exit": 0,
      "output": "ok ready-only packaged and development paths resolve once with subscribe-before-start\nok unbound snapshot get returns cached down clones then live status after handshake\nok resolver or start failure leaves social usable; fallback still denies sender and payload\nok snapshot cache updates while the window is closed and clones stay isolated\nok pre-ready quit and reentrant quit during resolver never spawn; activate stays idle-gated\nok configured supervisor shutdown is awaited on the replaced instance\nBitBook wallet startup tests passed (6).\n"
    },
    {
      "argv": [
        "node",
        "test/walletSupervisor.node.js"
      ],
      "exit": 0,
      "output": "ok launch: private data directory and regular readable pinned binary precede one inert spawn\nok launch: missing, non-file, symlink, unreadable, and hash mismatch never spawn\nok launch: missing, symlinked, non-directory, or non-0700 data directories never verify or spawn\nok handshake: real child-first fixture transcript binds both directions within two seconds\nok handshake: PID, session, diagnostics, timeout, and early exit failures never dispatch\nok dispatch: exact supervisor methods and closed parameter schemas are enforced after binding\nok dispatch: pre-bind and oversize calls fail before broker send\nok dispatch: matching replies settle promises out of order with cloned sanitized results\nok lifecycle: exit publishes only sanitized down state and restart never buffers spend requests\nok quit: every in-flight intent is cancelled before child termination\nok quit: an unbound child terminates without any application frame\nok snapshot: supervisor exports the shared Pay sanitizer and removes every fixture canary\nok snapshot: sync publication traverses the shared sanitizer before every subscriber\nBitBook wallet supervisor tests passed (13).\n"
    },
    {
      "argv": [
        "node",
        "test/walletStartupSmoke.node.js"
      ],
      "exit": 0,
      "output": "ok ready starts the pinned development broker, degraded snapshot, empty account.list, and awaited quit\nBitBook wallet startup smoke tests passed (1).\n"
    },
    {
      "argv": [
        "xvfb-run",
        "-a",
        "-s",
        "-screen 0 1024x768x24",
        "node",
        "test/walletAccountWindowSmoke.node.js"
      ],
      "exit": 0,
      "output": "ok actual supervisor opens, hides, reopens, and gracefully closes the native account window\nBitBook native account window smoke tests passed (1).\n"
    },
    {
      "argv": [
        "node",
        "test/walletAccountWindowSmoke.node.js"
      ],
      "exit": 0,
      "output": "ok actual supervisor opens, hides, reopens, and gracefully closes the native account window\nBitBook native account window smoke tests passed (1).\n"
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
        "dir",
        "--redact=100",
        "--no-banner",
        "."
      ],
      "exit": 0,
      "output": "1:52PM INF scanned ~1953427638 bytes (1.95 GB) in 14.9s\n1:52PM INF no leaks found\n"
    },
    {
      "argv": [
        "git",
        "diff",
        "--check"
      ],
      "exit": 0,
      "output": ""
    }
  ],
  "accepted": true,
  "version": "Hermes Agent v0.18.2 (2026.7.7.2) \u00b7 upstream 67764dc0 \u00b7 local 10b6d1a9 (+1 carried commit)\nInstall directory: <home>/.hermes/hermes-agent\nInstall method: git\nPython: 3.11.15\nOpenAI SDK: 2.24.0",
  "head": "9a641f021fb93ba75cb952f29164cba181099e48",
  "initial_status": "M package-lock.json\n M package.json\n M scripts/security-policy.js\n M test/securityPolicy.node.js\n M wallet-broker/src/account_ui.rs\n M wallet-broker/src/accounts.rs\n M wallet-broker/src/session.rs\n M wallet-broker/src/zec.rs\n M wallet-broker/tests/account_management.rs\n M wallet-broker/tests/account_native_ui.rs\n?? docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md\n?? docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md\n?? docs/testing/BBD-WAL-014-RECEIVE-GREEN-01.md\n?? docs/testing/BBD-WAL-014-RECEIVE-GREEN-02.md\n?? docs/testing/BBD-WAL-014-RECEIVE-RED-01.md",
  "session": {
    "id": "20260910_135125_b40cfc",
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
    "docs/testing/BBD-WAL-014-RECEIVE-GREEN-01.md": "c091d139092d68b8c3c28f74605dced9fca1b86256cff6f77e7dc8bbd9112dc9",
    "wallet-broker/target/wal014-receive-green-02.json": "1375570d23b150eed60ca5177f4313eea9056ac30f26862fc6885d37e5b1aedb",
    "docs/testing/BBD-WAL-014-RECEIVE-GREEN-02.md": "a6dd88997f3839f3d8c39f99f7b044f05a9107b9aed9f7409c0de729a58f4402"
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
  "reused_production_clippy_and_falsifications": true,
  "build_runner_rustup_diagnostic": {
    "status": null,
    "error": "ENOENT"
  },
  "build_path_correction": "prepend trusted <home>/.cargo/bin for build command only",
  "binary_sha256": "5ed35fe09b1172b2639faa6170621fca60fe99825636398b78fe1bd382509ae4",
  "xvfb_screenshots": {
    "wallet-broker/target/wal013-account-window-visible.png": "c05a2423a15eee8593a40a0e7d48fdaf18c6d6ec9c51c23b42a0da6c7ab014d0",
    "wallet-broker/target/wal013-account-window-reopened.png": "c05a2423a15eee8593a40a0e7d48fdaf18c6d6ec9c51c23b42a0da6c7ab014d0"
  },
  "host_screenshots": {
    "wallet-broker/target/wal013-account-window-visible.png": "c05a2423a15eee8593a40a0e7d48fdaf18c6d6ec9c51c23b42a0da6c7ab014d0",
    "wallet-broker/target/wal013-account-window-reopened.png": "c05a2423a15eee8593a40a0e7d48fdaf18c6d6ec9c51c23b42a0da6c7ab014d0"
  },
  "policy_current_failures": [
    "not ok committed workflows satisfy the fail-closed checker",
    "not ok strict nine-line reviewed Gitleaks ratchet bytes and content are enforced",
    "not ok WAL-004 Rust source inventory is exported closed and enumerated by repository policy",
    "not ok BBD-WAL-008 closes the hardware target and current fourteen-path WAL-008/WAL-009 ZEC policy inventory",
    "not ok WAL-009 production spend.rs accepts the exact reviewed TransactionExtractor statement and rejects independent mutations",
    "not ok WAL-009 verification-fixture spend/verification_context_tests.rs accepts the exact reviewed TransactionExtractor statement and rejects independent mutations"
  ],
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
    "docs/testing/BBD-WAL-014-RECEIVE-GREEN-01.md": "c091d139092d68b8c3c28f74605dced9fca1b86256cff6f77e7dc8bbd9112dc9",
    "wallet-broker/target/wal014-receive-green-02.json": "1375570d23b150eed60ca5177f4313eea9056ac30f26862fc6885d37e5b1aedb",
    "docs/testing/BBD-WAL-014-RECEIVE-GREEN-02.md": "a6dd88997f3839f3d8c39f99f7b044f05a9107b9aed9f7409c0de729a58f4402"
  }
}
```
