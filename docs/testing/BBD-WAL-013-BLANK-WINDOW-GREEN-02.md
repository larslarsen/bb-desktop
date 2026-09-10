# WAL-013 blank window fix: rendered native controls and host XWayland

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
      "output": "ee8690a91e3e29a101ed4d2db83cf8ffc7842413\n"
    },
    {
      "argv": [
        "git",
        "status",
        "--short"
      ],
      "exit": 0,
      "output": " M package-lock.json\n M package.json\n M scripts/security-policy.js\n M social-main.js\n M test/fixtures/wallet-broker/x11-window.py\n M test/securityPolicy.node.js\n M test/walletAccountManagement.node.js\n M test/walletAccountWindowSmoke.node.js\n M test/walletStartup.node.js\n M wallet-broker/supervisor.js\n?? docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md\n?? docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md\n?? docs/testing/BBD-WAL-013-BLANK-WINDOW-GREEN-01.md\n?? docs/testing/BBD-WAL-013-BLANK-WINDOW-RED-01.md\n?? docs/testing/BBD-WAL-013-HOST-SOFTWARE-DIAGNOSTIC-01.md\n?? docs/testing/BBD-WAL-013-HOST-WINDOW-DIAGNOSTIC-01.md\n?? docs/testing/BBD-WAL-013-HOST-WINDOW-DIAGNOSTIC-02.md\n"
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
        "test/walletAccountManagement.node.js"
      ],
      "exit": 1,
      "output": "ok ready menu registers Wallet/Manage accounts; bound click dispatches account.manage{}; failures stay closed\nok quit and repeated ready cannot open the account window\nnot ok supervisor spawn env adds fixed Linux renderer settings only with sanitized DISPLAY\nAssertionError [ERR_ASSERTION]: Expected values to be strictly deep-equal:\n+ actual - expected\n\n  {\n    DBUS_SESSION_BUS_ADDRESS: 'unix:path=/run/user/1000/bus',\n    DISPLAY: ':0',\n    LANG: 'C.UTF-8',\n-   LIBGL_ALWAYS_SOFTWARE: '1',\n    PATH: 'zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz',\n    XAUTHORITY: '/home/user/.Xauthority',\n    XDG_RUNTIME_DIR: '/run/user/1000',\n    __GLX_VENDOR_LIBRARY_NAME: 'mesa'\n  }\n\n    at <repo>/test/walletAccountManagement.node.js:822:12\n    at withFakeHarness (<repo>/test/walletAccountManagement.node.js:647:20)\n    at <repo>/test/walletAccountManagement.node.js:809:9\n    at run (<repo>/test/walletAccountManagement.node.js:1220:13)\nok main copies the six allowlisted process.env strings and omits WAYLAND_DISPLAY\nok dispatcher accepts account.manage{} only while bound and rejects secret-bearing methods\nok graceful EOF ends stdin once, waits 1000/1250/1500, and resolves only on close\nok graceful asynchronous and synchronous close are safe; missing or throwing end falls back immediately\n1 wallet account management test(s) failed\n"
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
        "target/security-tools/gitleaks-v8.30.1/gitleaks",
        "dir",
        "--redact=100",
        "--no-banner",
        "."
      ],
      "exit": 0,
      "output": "12:57PM INF scanned ~1952910921 bytes (1.95 GB) in 13.5s\n12:57PM INF no leaks found\n"
    }
  ],
  "accepted": true,
  "version": "Hermes Agent v0.18.2 (2026.7.7.2) \u00b7 upstream 67764dc0 \u00b7 local 10b6d1a9 (+1 carried commit)\nInstall directory: <home>/.hermes/hermes-agent\nInstall method: git\nPython: 3.11.15\nOpenAI SDK: 2.24.0",
  "head": "ee8690a91e3e29a101ed4d2db83cf8ffc7842413",
  "initial_status": "M package-lock.json\n M package.json\n M scripts/security-policy.js\n M social-main.js\n M test/fixtures/wallet-broker/x11-window.py\n M test/securityPolicy.node.js\n M test/walletAccountManagement.node.js\n M test/walletAccountWindowSmoke.node.js\n M test/walletStartup.node.js\n M wallet-broker/supervisor.js\n?? docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md\n?? docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md\n?? docs/testing/BBD-WAL-013-BLANK-WINDOW-GREEN-01.md\n?? docs/testing/BBD-WAL-013-BLANK-WINDOW-RED-01.md\n?? docs/testing/BBD-WAL-013-HOST-SOFTWARE-DIAGNOSTIC-01.md\n?? docs/testing/BBD-WAL-013-HOST-WINDOW-DIAGNOSTIC-01.md\n?? docs/testing/BBD-WAL-013-HOST-WINDOW-DIAGNOSTIC-02.md",
  "session": {
    "id": "20260910_125704_54c532",
    "model": "meituan/longcat-2.0:free",
    "provider": "nous"
  },
  "input_hashes": {
    "social-main.js": "d959e6adffc2579fa3f2f447f60a165f2b6bcdc279afde2f6ddaa91bc9d9aa4a",
    "wallet-broker/supervisor.js": "115e0e51425e7b25d8f6178b206f592165cb14dacde8cb544b970839833c4721",
    "scripts/build-wallet-broker.js": "4427ef3e713ec77c45daefe8d37b457ca8dc2c8529b30214dbf65628f43a1445",
    "wallet-broker/src/runtime.rs": "6788914524605f4669d20d65f7b6ef7282b0090a0696ea45f088ecc49dc1c962",
    "test/walletAccountManagement.node.js": "6a449335e1101b8dd6340d640514e8fc083a09235210c17dd582b57230de9475",
    "test/walletStartup.node.js": "448fb88268655b979c7c724ab21d13d09fee29a219628cd70f576daae4589de1",
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
    "test/walletAccountWindowSmoke.node.js": "3590096cafed5596378488e8ebca8c916e62352d4410e5748a2033ab0ec9fa3a",
    "test/fixtures/wallet-broker/x11-window.py": "d192f036781e5623b88a8ba3f31b9ee7c781f7bdffa46982aa431f8ada5330d3",
    "wallet-broker/target/wal011-runtime-package-red-01.json": "5608cd092ae81d83e658aa9231e2751378f5f15fbde3a60c2b9dc176d5a370bd"
  },
  "isolated_screenshots": {
    "wallet-broker/target/wal013-account-window-visible.png": "070924bb3e51e5cc75dcd93c5866057751475b9e3353c1e6d53fc0355727ae25",
    "wallet-broker/target/wal013-account-window-reopened.png": "070924bb3e51e5cc75dcd93c5866057751475b9e3353c1e6d53fc0355727ae25"
  },
  "host_xwayland_screenshots": {
    "wallet-broker/target/wal013-account-window-visible.png": "070924bb3e51e5cc75dcd93c5866057751475b9e3353c1e6d53fc0355727ae25",
    "wallet-broker/target/wal013-account-window-reopened.png": "070924bb3e51e5cc75dcd93c5866057751475b9e3353c1e6d53fc0355727ae25"
  },
  "software_constant_falsification_detected": true,
  "binary_sha256": "929751c8f956c8e764466279b9adb4ab8007c391206dd6478b18f1794853a11c",
  "policy_current_failures": [
    "not ok committed workflows satisfy the fail-closed checker",
    "not ok strict nine-line reviewed Gitleaks ratchet bytes and content are enforced",
    "not ok WAL-004 Rust source inventory is exported closed and enumerated by repository policy",
    "not ok BBD-WAL-008 closes the hardware target and current fourteen-path WAL-008/WAL-009 ZEC policy inventory",
    "not ok WAL-009 production spend.rs accepts the exact reviewed TransactionExtractor statement and rejects independent mutations",
    "not ok WAL-009 verification-fixture spend/verification_context_tests.rs accepts the exact reviewed TransactionExtractor statement and rejects independent mutations"
  ],
  "final_hashes": {
    "social-main.js": "d959e6adffc2579fa3f2f447f60a165f2b6bcdc279afde2f6ddaa91bc9d9aa4a",
    "wallet-broker/supervisor.js": "115e0e51425e7b25d8f6178b206f592165cb14dacde8cb544b970839833c4721",
    "scripts/build-wallet-broker.js": "4427ef3e713ec77c45daefe8d37b457ca8dc2c8529b30214dbf65628f43a1445",
    "wallet-broker/src/runtime.rs": "6788914524605f4669d20d65f7b6ef7282b0090a0696ea45f088ecc49dc1c962",
    "test/walletAccountManagement.node.js": "6a449335e1101b8dd6340d640514e8fc083a09235210c17dd582b57230de9475",
    "test/walletStartup.node.js": "448fb88268655b979c7c724ab21d13d09fee29a219628cd70f576daae4589de1",
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
    "test/walletAccountWindowSmoke.node.js": "3590096cafed5596378488e8ebca8c916e62352d4410e5748a2033ab0ec9fa3a",
    "test/fixtures/wallet-broker/x11-window.py": "d192f036781e5623b88a8ba3f31b9ee7c781f7bdffa46982aa431f8ada5330d3",
    "wallet-broker/target/wal011-runtime-package-red-01.json": "5608cd092ae81d83e658aa9231e2751378f5f15fbde3a60c2b9dc176d5a370bd"
  }
}
```
