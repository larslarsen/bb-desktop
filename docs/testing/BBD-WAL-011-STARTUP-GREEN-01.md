# WAL-011 application startup and native composition green 01

Actual bounded execution; release blockers are not waived.

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
      "output": "fceb659c020acaf57bbf112cd2b281b01c6f92f8\n"
    },
    {
      "argv": [
        "git",
        "status",
        "--short"
      ],
      "exit": 0,
      "output": " M package-lock.json\n M package.json\n M scripts/security-policy.js\n M social-main.js\n M test/electronSecurity.node.js\n M test/securityPolicy.node.js\n?? docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md\n?? docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md\n?? docs/testing/BBD-WAL-011-STARTUP-BUILD-RED-01.md\n?? docs/testing/BBD-WAL-011-STARTUP-MAIN-RED-01.md\n?? scripts/build-wallet-broker.js\n?? test/walletBrokerBuild.node.js\n?? test/walletStartup.node.js\n?? test/walletStartupSmoke.node.js\n"
    },
    {
      "argv": [
        "df",
        "-T",
        "wallet-broker/target"
      ],
      "exit": 0,
      "output": "Filesystem                        Type 1K-blocks      Used Available Use% Mounted on\n/dev/mapper/ubuntu--vg-ubuntu--lv ext4 957134040 826579136  81861452  91% /\n"
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
        "test/walletBrokerBuild.node.js"
      ],
      "exit": 0,
      "output": "ok module: exports synchronous stageWalletBroker\nok stage: copied bytes pin independently, modes match identity, resolver accepts layout\nok stage: repeat replacement preserves unrelated files and retains staging leftovers\nok preflight: invalid source, roots, and identities leave destination bytes unchanged\nok preflight: destination directory and pin links leave bytes unchanged\nok cli: vm-mocked cargo command is exact and nonzero skips staging\nBitBook wallet broker build tests passed (6).\n"
    },
    {
      "argv": [
        "node",
        "test/electronSecurity.node.js"
      ],
      "exit": 0,
      "output": "ok app.enableSandbox is invoked before the window is created\nok BrowserWindow explicitly sets the fail-closed webPreferences\nok only the repository social/index.html is loaded\nok package.json keeps the maintained social-main.js entry point\nok renderer navigation is denied\nok renderer redirects are denied\nok webview attachment is denied\nok new-window creation is denied and shell.openExternal is unreachable\nok permission request handler denies every permission\nok permission check handler denies every permission\nok only the explicit local wallet preload and exact ipcMain bridge are introduced\nok CSP keeps self-only script/style and denies objects, frames, base, and forms\nok maintained source has no HTML injection, eval, or javascript: sinks\nok wallet preload retains exactly six frozen methods and no Electron confirmation action\nok wallet IPC registers only the exact renderer channel allowlist\nok wallet IPC rejects non-main frames, non-local origins, malformed shapes, and oversize input\nok wallet IPC valid calls map once to fixed supervisor methods with cloned parameters\nok wallet IPC delayed supervisor replies stay pending then clone fulfilled values for every channel\nok wallet IPC delayed supervisor rejections propagate original UNAVAILABLE and TIMEOUT errors\nok wallet IPC synchronous dispatch failure throws immediately\nok wallet snapshot subscription targets only the maintained main frame with sanitized cloned data\nok wallet Electron boundary exposes no confirmation, unlock, backup, sign, or broadcast surface\nok wallet reference contract is maintained source and retains an offline inert boundary\nok after ready, initial before-quit is prevented until deferred shutdown fulfills and resumes quit\nok before ready, fulfilled no-child shutdown allows one resumed quit without windows\nok shutdown-emitted nested before-quit stays prevented until the original shutdown fulfills\nok rejected shutdown keeps quit blocked and shows one fixed error box for TIMEOUT and UNAVAILABLE\nok synchronous shutdown throw is contained and keeps quit blocked with one fixed error box\nok rejected shutdown plus throwing error box stays contained without resumed quit\nok window-all-closed uses the host platform branch and the same before-quit gate\nBitBook electron security tests passed (30).\n"
    },
    {
      "argv": [
        "node",
        "test/walletPreload.node.js"
      ],
      "exit": 0,
      "output": "ok preload: one frozen bitbookWallet object exposes exactly six frozen own functions\nok preload: each callable API uses only its fixed channel and page supplies no method string\nok preload: arguments and results are structured clones immune to caller mutation\nok preload: invalid callback types register no listener\nok preload: subscription strips event objects, clones values, and unsubscribe is bounded\nok preload: hostile callbacks cannot retain Electron events or widen the bridge\nBitBook wallet preload tests passed (6).\n"
    },
    {
      "argv": [
        "node",
        "test/walletBrokerLaunchConfig.node.js"
      ],
      "exit": 0,
      "output": "ok valid configuration: six platform identities return frozen supervisor options\nok invalid options: reject missing extra accessor prototype and path identities\nok invalid manifest: reject malformed extra mismatched and hostile digest rows\nok manifest size: 4096 bytes succeed; 4097 and empty reject\nok inventory: missing symlink and directory entries fail closed\nok reflection errors: revoked proxy is sanitized\nok reflection errors: getPrototypeOf trap is sanitized\nok reflection errors: ownKeys trap is sanitized\nok reflection errors: getOwnPropertyDescriptor trap is sanitized\nBitBook wallet broker launch configuration tests passed (9).\n"
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
        "rustup",
        "run",
        "1.98.0",
        "rustc",
        "-vV"
      ],
      "exit": 0,
      "output": "rustc 1.98.0 (88d9e12ae 2026-08-18)\nbinary: rustc\ncommit-hash: 88d9e12ae178fab0fb5cc050a94da85685d449ea\ncommit-date: 2026-08-18\nhost: x86_64-unknown-linux-gnu\nrelease: 1.98.0\nLLVM version: 22.1.8\n"
    },
    {
      "argv": [
        "node",
        "scripts/build-wallet-broker.js"
      ],
      "exit": 0,
      "output": "    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.58s\n"
    },
    {
      "argv": [
        "node",
        "test/walletStartupSmoke.node.js"
      ],
      "exit": 0,
      "output": "ok ready starts the pinned development broker, degraded snapshot, UNAVAILABLE list, and awaited quit\nBitBook wallet startup smoke tests passed (1).\n"
    },
    {
      "argv": [
        "node",
        "test/walletStartupSmoke.node.js"
      ],
      "exit": 1,
      "output": "not ok ready starts the pinned development broker, degraded snapshot, UNAVAILABLE list, and awaited quit\nAssertionError [ERR_ASSERTION]: ready did not spawn one broker\n\n0 !== 1\n\n    at <repo>/test/walletStartupSmoke.node.js:373:12\n    at run (<repo>/test/walletStartupSmoke.node.js:525:13)\n    at Object.<anonymous> (<repo>/test/walletStartupSmoke.node.js:541:3)\n    at Module._compile (node:internal/modules/cjs/loader:1781:14)\n    at Object..js (node:internal/modules/cjs/loader:1913:10)\n    at Module.load (node:internal/modules/cjs/loader:1505:32)\n    at Function.<anonymous> (node:internal/modules/cjs/loader:1309:12)\n    at wrapModuleLoad (node:internal/modules/cjs/loader:254:19)\n    at Function.executeUserEntryPoint [as runMain] (node:internal/modules/run_main:171:5)\n    at node:internal/main/run_main_module:36:49\n1 wallet startup smoke test(s) failed\n"
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
        "test/walletStartupSmoke.node.js"
      ],
      "exit": 0,
      "output": "ok ready starts the pinned development broker, degraded snapshot, UNAVAILABLE list, and awaited quit\nBitBook wallet startup smoke tests passed (1).\n"
    },
    {
      "argv": [
        "node",
        "-e",
        "const {tests}=require('./test/walletBrokerBuild.node.js'); const entry=tests.find(t=>t.name==='stage: copied bytes pin independently, modes match identity, resolver accepts layout'); if(!entry) throw new Error('selected test absent'); entry.fn();"
      ],
      "exit": 1,
      "output": "<repo>/test/walletBrokerBuild.node.js:119\n  if (originalError) throw originalError;\n                     ^\n\nAssertionError [ERR_ASSERTION]: Expected values to be strictly deep-equal:\n+ actual - expected\n\n  {\n    arch: 'x64',\n    platform: 'linux',\n+   sha256: '0000000000000000000000000000000000000000000000000000000000000000',\n-   sha256: '3eb810748d1050a04322295e55164dae8ed1b2a917b7359f7879cf08ce6d5cf3',\n    v: 1\n  }\n\n    at assertStaged (<repo>/test/walletBrokerBuild.node.js:215:10)\n    at <repo>/test/walletBrokerBuild.node.js:382:22\n    at withOwnedTemp (<repo>/test/walletBrokerBuild.node.js:99:12)\n    at Object.fn (<repo>/test/walletBrokerBuild.node.js:370:3)\n    at [eval]:1:242\n    at runScriptInThisContext (node:internal/vm:209:10)\n    at node:internal/process/execution:446:12\n    at [eval]-wrapper:6:24\n    at runScriptInContext (node:internal/process/execution:444:60)\n    at evalFunction (node:internal/process/execution:279:30) {\n  generatedMessage: true,\n  code: 'ERR_ASSERTION',\n  actual: {\n    v: 1,\n    platform: 'linux',\n    arch: 'x64',\n    sha256: '0000000000000000000000000000000000000000000000000000000000000000'\n  },\n  expected: {\n    v: 1,\n    platform: 'linux',\n    arch: 'x64',\n    sha256: '3eb810748d1050a04322295e55164dae8ed1b2a917b7359f7879cf08ce6d5cf3'\n  },\n  operator: 'deepStrictEqual',\n  diff: 'simple'\n}\n\nNode.js v22.23.1\n"
    },
    {
      "argv": [
        "node",
        "test/walletBrokerBuild.node.js"
      ],
      "exit": 0,
      "output": "ok module: exports synchronous stageWalletBroker\nok stage: copied bytes pin independently, modes match identity, resolver accepts layout\nok stage: repeat replacement preserves unrelated files and retains staging leftovers\nok preflight: invalid source, roots, and identities leave destination bytes unchanged\nok preflight: destination directory and pin links leave bytes unchanged\nok cli: vm-mocked cargo command is exact and nonzero skips staging\nBitBook wallet broker build tests passed (6).\n"
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
        "npm",
        "audit",
        "--audit-level=low"
      ],
      "exit": 0,
      "output": "found 0 vulnerabilities\n"
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
      "output": "8:59AM INF 5274 commits scanned.\n8:59AM INF scanned ~32547863 bytes (32.55 MB) in 2.42s\n8:59AM INF no leaks found\n"
    }
  ],
  "accepted": false,
  "version": "Hermes Agent v0.18.2 (2026.7.7.2) \u00b7 upstream 67764dc0 \u00b7 local 10b6d1a9 (+1 carried commit)\nInstall directory: <home>/.hermes/hermes-agent\nInstall method: git\nPython: 3.11.15\nOpenAI SDK: 2.24.0",
  "head": "fceb659c020acaf57bbf112cd2b281b01c6f92f8",
  "initial_status": "M package-lock.json\n M package.json\n M scripts/security-policy.js\n M social-main.js\n M test/electronSecurity.node.js\n M test/securityPolicy.node.js\n?? docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md\n?? docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md\n?? docs/testing/BBD-WAL-011-STARTUP-BUILD-RED-01.md\n?? docs/testing/BBD-WAL-011-STARTUP-MAIN-RED-01.md\n?? scripts/build-wallet-broker.js\n?? test/walletBrokerBuild.node.js\n?? test/walletStartup.node.js\n?? test/walletStartupSmoke.node.js",
  "session": {
    "id": "20260910_085930_0dff59",
    "model": "poolside/laguna-s-2.1:free",
    "provider": "nous"
  },
  "input_hashes": {
    "social-main.js": "7047253ee27955b0838ec226cd1e49c4a7dca8c3cc2a54a776fb1211c8a820d0",
    "scripts/build-wallet-broker.js": "c96684b9d7242a020a93c0ea8a96d590c7a262482f44726ac12ce14ab618e382",
    "test/walletStartup.node.js": "bf72c9f276938b8b2e538bbb7631303837d6f9da998f3a4c279d397a02f1bae3",
    "test/walletBrokerBuild.node.js": "614e3ad8dde731c344d23353076672e0ba046a3e9d4919e303b5601350b97035",
    "test/walletStartupSmoke.node.js": "35d87dc7c3f8f9f919277e5c16cc66e22fdc7228718d62ef455652b32dc3f09f",
    "test/electronSecurity.node.js": "df81caae58607184a0c407f5aa845aa90a2f729c7fa8a9ac0ec9edbc366e8eea",
    "wallet-broker/supervisor.js": "c1410bfc5cd4c205ad014a42154c9d9d9714794b134a5acab7ed499934cad51a",
    "wallet-broker/launch-config.js": "68adf92e2d543e5d51353477b1944c35d4189c5590ac581d3289f1bd72898af8",
    "wallet-broker/protocol.js": "79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4",
    "wallet-pay/model.js": "acf07238366f3e28253be8c9208fbb27e13e9c2687d374d1a6ae87e0d173fa5e",
    "wallet-preload.js": "3e6a18acf88dd5be4e6a88f326d6ace7a8071066480d9a70a2e8f89df035a1df",
    "test/walletPreload.node.js": "60b151344a01776e1d7f38238f69534426883503f9d627466bac7acbf4dc4f9e",
    "test/walletSupervisor.node.js": "eaaa63777c8ded89ea6e9b2a07912bd3a680953bdb989a4ff0d6894e5df6ce5c",
    "test/walletBrokerLaunchConfig.node.js": "90acd32c9863df0206364eb04f707d92a5979ee66c7e15b2ec7b8250a5fc2113",
    "test/fixtures/wallet-pay/snapshots-v1.json": "bd51c24e003eb63f84fd99ca9573e765f28c3b7bc3ccbd924b14ba34afe05252",
    "package.json": "76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5",
    "package-lock.json": "0e32de43f3c1a31ae3d9e9c8b172d10cd37a03577dc7d1eaf4ac916967cfcdb8",
    "scripts/security-policy.js": "fec10759a73a3c675d8206da454abf4b0e8dca6b62da463f39a2db27b30dc078",
    "test/securityPolicy.node.js": "a58d85e5322f0cb01a78b636454f92547bd98e556013cc07712dc0cf64ea565c",
    "docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md": "1e548d364ac3a39e9bb1941a5c89b0ccb45143d9c8588e35e42f12434120799f",
    "docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md": "2a6abd0a41a93028ce63fce8b3f385436623a416dfcdc26565b0277e7875bc56",
    "docs/testing/BBD-WAL-011-STARTUP-MAIN-RED-01.md": "89bc8c6a6619ff848d7f632f3f36c474b5d2f5b8f1e3104a8f9fcc53073bf5f7",
    "docs/testing/BBD-WAL-011-STARTUP-BUILD-RED-01.md": "efd7134cd477ddaa853b9f2e0a7bbf976d495ae3e0120cfec1a1bc4b3d879177",
    "wallet-broker/Cargo.lock": "b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71",
    "wallet-broker/Cargo.toml": "73e5e585eb2fd1ca867d66962460cfa010443886a4b59d8a33556ef2a4ff3503",
    "wallet-broker/src/hygiene.rs": "7676aaad8ed78fb01fdb3cf2a763fd057693f5fe6f2721b385c3c8dd6d39bdbf",
    "wallet-broker/src/lib.rs": "08dd09d23a8c18cdb9a50968ade153a2118b60132f2b7b66a36c6913596de925",
    "wallet-broker/src/main.rs": "19b1651d88d85e5d968597eb26eeb62f76a11af56eaa311d60a55d3ef282736d",
    "wallet-broker/src/native.rs": "a0ccb0b11e3e636cc28f9576ec0cf6da2d8c2ef5344370e5a7a2a9a9b5cc32e5",
    "wallet-broker/src/native_ui.rs": "c6607af3f36278f8d7deeb2b4276017177bbb4c695017f18bf4d2bcb412b268f",
    "wallet-broker/src/native_ui/zec_native_app_tests.rs": "d1d405f8667c6c9d184104d2023ffcff209f6d013e3920dd5c5ccd7e5a2c40f0",
    "wallet-broker/src/native_ui/zec_review_tests.rs": "2eec3b60eb1b9fbd32235832a867d1189928126ca79373fa9f4fa7fdf2ee87bf",
    "wallet-broker/src/runtime.rs": "968194aadd609f54ef28a11a090fcb1ca840cfc851da127456dd5b356437305b",
    "wallet-broker/src/session.rs": "42e4f335bb4080ad530d93dcc04d824b4ab54835be7f6c7cd68feba3f20ee227",
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
    "wallet-broker/src/zec.rs": "045cdc51f26ac8b9b1283cee5f995d60ceda38a25577aa7a937f5d07f177b90b",
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
    "wallet-broker/src/zec/test_support/cleanup_lifecycle_tests.rs": "5b01727fdc2d864e0ec31693a58ad7f25041891f5cab1bc091ec0b8743323cca"
  },
  "build_path_prefix": "<home>/.cargo/bin",
  "artifact": {
    "binary": "wallet-broker/target/app-resources/wallet-broker/bitbook-wallet-broker",
    "sha256": "5f112d4f5997615bfc43f585ac79a9e6754364273ea3ca348fa0654797eb0f26",
    "bytes": 8437624,
    "mode": "0o755",
    "manifest": {
      "v": 1,
      "platform": "linux",
      "arch": "x64",
      "sha256": "5f112d4f5997615bfc43f585ac79a9e6754364273ea3ca348fa0654797eb0f26"
    },
    "manifest_sha256": "642fd89955a4fbe9e7af8f9569bae63b46b8a572905ab5c5ac9df0c9f2ce4c25"
  },
  "main_falsification_restored": true,
  "build_falsification_restored": true,
  "release_green": false,
  "policy_baseline_unchanged": true,
  "stop": "[Errno 32] Broken pipe",
  "final_hashes": {
    "social-main.js": "7047253ee27955b0838ec226cd1e49c4a7dca8c3cc2a54a776fb1211c8a820d0",
    "scripts/build-wallet-broker.js": "c96684b9d7242a020a93c0ea8a96d590c7a262482f44726ac12ce14ab618e382",
    "test/walletStartup.node.js": "bf72c9f276938b8b2e538bbb7631303837d6f9da998f3a4c279d397a02f1bae3",
    "test/walletBrokerBuild.node.js": "614e3ad8dde731c344d23353076672e0ba046a3e9d4919e303b5601350b97035",
    "test/walletStartupSmoke.node.js": "35d87dc7c3f8f9f919277e5c16cc66e22fdc7228718d62ef455652b32dc3f09f",
    "test/electronSecurity.node.js": "df81caae58607184a0c407f5aa845aa90a2f729c7fa8a9ac0ec9edbc366e8eea",
    "wallet-broker/supervisor.js": "c1410bfc5cd4c205ad014a42154c9d9d9714794b134a5acab7ed499934cad51a",
    "wallet-broker/launch-config.js": "68adf92e2d543e5d51353477b1944c35d4189c5590ac581d3289f1bd72898af8",
    "wallet-broker/protocol.js": "79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4",
    "wallet-pay/model.js": "acf07238366f3e28253be8c9208fbb27e13e9c2687d374d1a6ae87e0d173fa5e",
    "wallet-preload.js": "3e6a18acf88dd5be4e6a88f326d6ace7a8071066480d9a70a2e8f89df035a1df",
    "test/walletPreload.node.js": "60b151344a01776e1d7f38238f69534426883503f9d627466bac7acbf4dc4f9e",
    "test/walletSupervisor.node.js": "eaaa63777c8ded89ea6e9b2a07912bd3a680953bdb989a4ff0d6894e5df6ce5c",
    "test/walletBrokerLaunchConfig.node.js": "90acd32c9863df0206364eb04f707d92a5979ee66c7e15b2ec7b8250a5fc2113",
    "test/fixtures/wallet-pay/snapshots-v1.json": "bd51c24e003eb63f84fd99ca9573e765f28c3b7bc3ccbd924b14ba34afe05252",
    "package.json": "76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5",
    "package-lock.json": "0e32de43f3c1a31ae3d9e9c8b172d10cd37a03577dc7d1eaf4ac916967cfcdb8",
    "scripts/security-policy.js": "fec10759a73a3c675d8206da454abf4b0e8dca6b62da463f39a2db27b30dc078",
    "test/securityPolicy.node.js": "a58d85e5322f0cb01a78b636454f92547bd98e556013cc07712dc0cf64ea565c",
    "docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md": "1e548d364ac3a39e9bb1941a5c89b0ccb45143d9c8588e35e42f12434120799f",
    "docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md": "2a6abd0a41a93028ce63fce8b3f385436623a416dfcdc26565b0277e7875bc56",
    "docs/testing/BBD-WAL-011-STARTUP-MAIN-RED-01.md": "89bc8c6a6619ff848d7f632f3f36c474b5d2f5b8f1e3104a8f9fcc53073bf5f7",
    "docs/testing/BBD-WAL-011-STARTUP-BUILD-RED-01.md": "efd7134cd477ddaa853b9f2e0a7bbf976d495ae3e0120cfec1a1bc4b3d879177",
    "wallet-broker/Cargo.lock": "b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71",
    "wallet-broker/Cargo.toml": "73e5e585eb2fd1ca867d66962460cfa010443886a4b59d8a33556ef2a4ff3503",
    "wallet-broker/src/hygiene.rs": "7676aaad8ed78fb01fdb3cf2a763fd057693f5fe6f2721b385c3c8dd6d39bdbf",
    "wallet-broker/src/lib.rs": "08dd09d23a8c18cdb9a50968ade153a2118b60132f2b7b66a36c6913596de925",
    "wallet-broker/src/main.rs": "19b1651d88d85e5d968597eb26eeb62f76a11af56eaa311d60a55d3ef282736d",
    "wallet-broker/src/native.rs": "a0ccb0b11e3e636cc28f9576ec0cf6da2d8c2ef5344370e5a7a2a9a9b5cc32e5",
    "wallet-broker/src/native_ui.rs": "c6607af3f36278f8d7deeb2b4276017177bbb4c695017f18bf4d2bcb412b268f",
    "wallet-broker/src/native_ui/zec_native_app_tests.rs": "d1d405f8667c6c9d184104d2023ffcff209f6d013e3920dd5c5ccd7e5a2c40f0",
    "wallet-broker/src/native_ui/zec_review_tests.rs": "2eec3b60eb1b9fbd32235832a867d1189928126ca79373fa9f4fa7fdf2ee87bf",
    "wallet-broker/src/runtime.rs": "968194aadd609f54ef28a11a090fcb1ca840cfc851da127456dd5b356437305b",
    "wallet-broker/src/session.rs": "42e4f335bb4080ad530d93dcc04d824b4ab54835be7f6c7cd68feba3f20ee227",
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
    "wallet-broker/src/zec.rs": "045cdc51f26ac8b9b1283cee5f995d60ceda38a25577aa7a937f5d07f177b90b",
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
    "wallet-broker/src/zec/test_support/cleanup_lifecycle_tests.rs": "5b01727fdc2d864e0ec31693a58ad7f25041891f5cab1bc091ec0b8743323cca"
  }
}
```
