# WAL-011 runtime package validation 01

Component validation, actual package proof and preserved policy baseline below. release_green remains false while the recorded inherited policy blockers stand. No native broker pin, main startup, other-platform execution or release publication is claimed.

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
      "output": "fbaf8045676f1fdf922ec08763f0459c31163a3d\n"
    },
    {
      "argv": [
        "git",
        "status",
        "--short"
      ],
      "exit": 0,
      "output": " M package-lock.json\n M package.json\n M scripts/build-deb.sh\n M scripts/build-macos.sh\n M scripts/build-windows.ps1\n M scripts/security-policy.js\n M test/securityPolicy.node.js\n?? docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md\n?? docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md\n?? docs/testing/BBD-WAL-011-RUNTIME-PACKAGE-RED-01.md\n?? scripts/stage-wallet-runtime.js\n?? test/walletRuntimePackage.node.js\n"
    },
    {
      "argv": [
        "node",
        "test/walletRuntimePackage.node.js"
      ],
      "exit": 0,
      "output": "ok staging: exact five-file inventory preserves bytes and excludes native data\nok preflight: missing and hostile source entries leave destination unchanged\nok preflight: existing and hostile destinations are never overwritten\nok arguments: invalid roots fail before staging\nok cli: real checkout stages modules whose nested imports resolve\nok cli: missing and extra arguments fail without staging\nok packagers: shared staging precedes signing and archive with Linux disk storage\nBitBook wallet runtime package tests passed (7).\n",
      "counts": {
        "ok": 7,
        "not_ok": 0
      }
    },
    {
      "argv": [
        "node",
        "test/walletBrokerLaunchConfig.node.js"
      ],
      "exit": 0,
      "output": "ok valid configuration: six platform identities return frozen supervisor options\nok invalid options: reject missing extra accessor prototype and path identities\nok invalid manifest: reject malformed extra mismatched and hostile digest rows\nok manifest size: 4096 bytes succeed; 4097 and empty reject\nok inventory: missing symlink and directory entries fail closed\nok reflection errors: revoked proxy is sanitized\nok reflection errors: getPrototypeOf trap is sanitized\nok reflection errors: ownKeys trap is sanitized\nok reflection errors: getOwnPropertyDescriptor trap is sanitized\nBitBook wallet broker launch configuration tests passed (9).\n",
      "counts": {
        "ok": 9,
        "not_ok": 0
      }
    },
    {
      "argv": [
        "node",
        "test/electronSecurity.node.js"
      ],
      "exit": 0,
      "output": "ok app.enableSandbox is invoked before the window is created\nok BrowserWindow explicitly sets the fail-closed webPreferences\nok only the repository social/index.html is loaded\nok package.json keeps the maintained social-main.js entry point\nok renderer navigation is denied\nok renderer redirects are denied\nok webview attachment is denied\nok new-window creation is denied and shell.openExternal is unreachable\nok permission request handler denies every permission\nok permission check handler denies every permission\nok only the explicit local wallet preload and exact ipcMain bridge are introduced\nok CSP keeps self-only script/style and denies objects, frames, base, and forms\nok maintained source has no HTML injection, eval, or javascript: sinks\nok wallet preload retains exactly six frozen methods and no Electron confirmation action\nok wallet IPC registers only the exact renderer channel allowlist\nok wallet IPC rejects non-main frames, non-local origins, malformed shapes, and oversize input\nok wallet IPC valid calls map once to fixed supervisor methods with cloned parameters\nok wallet IPC delayed supervisor replies stay pending then clone fulfilled values for every channel\nok wallet IPC delayed supervisor rejections propagate original UNAVAILABLE and TIMEOUT errors\nok wallet IPC synchronous dispatch failure throws immediately\nok wallet snapshot subscription targets only the maintained main frame with sanitized cloned data\nok wallet Electron boundary exposes no confirmation, unlock, backup, sign, or broadcast surface\nok wallet reference contract is maintained source and retains an offline inert boundary\nok after ready, initial before-quit is prevented until deferred shutdown fulfills and resumes quit\nok before ready, fulfilled no-child shutdown allows one resumed quit without windows\nok shutdown-emitted nested before-quit stays prevented until the original shutdown fulfills\nok rejected shutdown keeps quit blocked and shows one fixed error box for TIMEOUT and UNAVAILABLE\nok synchronous shutdown throw is contained and keeps quit blocked with one fixed error box\nok rejected shutdown plus throwing error box stays contained without resumed quit\nok window-all-closed uses the host platform branch and the same before-quit gate\nBitBook electron security tests passed (30).\n",
      "counts": {
        "ok": 30,
        "not_ok": 0
      }
    },
    {
      "argv": [
        "bash",
        "-n",
        "scripts/build-deb.sh"
      ],
      "exit": 0,
      "output": ""
    },
    {
      "argv": [
        "bash",
        "-n",
        "scripts/build-macos.sh"
      ],
      "exit": 0,
      "output": ""
    },
    {
      "argv": [
        "node",
        "-e",
        "const assert=require('assert'); const name='staging: exact five-file inventory preserves bytes and excludes native data'; const tests=require('./test/walletRuntimePackage.node.js').tests.filter(t=>t.name===name); assert.strictEqual(tests.length,1); try{tests[0].fn(); console.log('ok '+name);}catch(e){console.error('not ok '+name+'\\n'+e.stack);process.exitCode=1;}"
      ],
      "exit": 1,
      "output": "not ok staging: exact five-file inventory preserves bytes and excludes native data\nAssertionError [ERR_ASSERTION]: Expected values to be strictly deep-equal:\n+ actual - expected\n... Skipped lines\n\n  [\n    'APP_SENTINEL',\n    'imgs',\n    'package.json',\n    'social',\n...\n    'wallet-pay',\n-   'wallet-preload.js'\n  ]\n\n    at assertStagedExact (<repo>/test/walletRuntimePackage.node.js:354:10)\n    at <repo>/test/walletRuntimePackage.node.js:430:5\n    at withOwnedTemp (<repo>/test/walletRuntimePackage.node.js:120:12)\n    at Object.fn (<repo>/test/walletRuntimePackage.node.js:410:3)\n    at [eval]:1:263\n    at runScriptInThisContext (node:internal/vm:209:10)\n    at node:internal/process/execution:446:12\n    at [eval]-wrapper:6:24\n    at runScriptInContext (node:internal/process/execution:444:60)\n    at evalFunction (node:internal/process/execution:279:30)\n",
      "counts": {
        "ok": 0,
        "not_ok": 1
      },
      "mutant_sha256": "df2b7bb82ea20d0c49574bbb92c2c31c86f4ce02caf70b6bc3c3c71cfc01ed4b",
      "restored_sha256": "97d04b4434e498bf4a2dd70d1ec69881221d8466664656e87fbffdd1c0694a70"
    },
    {
      "argv": [
        "node",
        "test/walletRuntimePackage.node.js"
      ],
      "exit": 0,
      "output": "ok staging: exact five-file inventory preserves bytes and excludes native data\nok preflight: missing and hostile source entries leave destination unchanged\nok preflight: existing and hostile destinations are never overwritten\nok arguments: invalid roots fail before staging\nok cli: real checkout stages modules whose nested imports resolve\nok cli: missing and extra arguments fail without staging\nok packagers: shared staging precedes signing and archive with Linux disk storage\nBitBook wallet runtime package tests passed (7).\n",
      "counts": {
        "ok": 7,
        "not_ok": 0
      }
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
      "output": "8:02AM INF 5261 commits scanned.\n8:02AM INF scanned ~32374531 bytes (32.37 MB) in 3.32s\n8:02AM INF no leaks found\n"
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
      "output": "8:02AM INF scanned ~1825638872 bytes (1.83 GB) in 13.4s\n8:02AM INF no leaks found\n"
    },
    {
      "argv": [
        "findmnt",
        "-T",
        "dist",
        "-n",
        "-o",
        "FSTYPE"
      ],
      "exit": 0,
      "output": "ext4\n"
    },
    {
      "argv": [
        "uname",
        "-m"
      ],
      "exit": 0,
      "output": "x86_64\n"
    },
    {
      "argv": [
        "bash",
        "scripts/build-deb.sh"
      ],
      "exit": 0,
      "output": "dpkg-deb: building package 'bitbook' in '<repo>/dist/bitbook_0.1.0_amd64.deb'.\nBuilt <repo>/dist/bitbook_0.1.0_amd64.deb\nStaging retained at <repo>/dist/bitbook-deb.umbyu6\n"
    },
    {
      "argv": [
        "dpkg-deb",
        "--contents",
        "dist/bitbook_0.1.0_amd64.deb"
      ],
      "exit": 0,
      "output": "drwxr-xr-x root/root         0 2026-09-10 08:02 ./\ndrwxr-xr-x root/root         0 2026-09-10 08:02 ./usr/\ndrwxr-xr-x root/root         0 2026-09-10 08:02 ./usr/bin/\ndrwxr-xr-x root/root         0 2026-09-10 08:02 ./usr/lib/\ndrwxr-xr-x root/root         0 2026-09-10 08:02 ./usr/lib/bitbook/\n-rw-r--r-- root/root      1096 1979-12-31 16:00 ./usr/lib/bitbook/LICENSE\n-rw-r--r-- root/root  20111206 1979-12-31 16:00 ./usr/lib/bitbook/LICENSES.chromium.html\n-rwxr-xr-x root/root 227163464 1979-12-31 16:00 ./usr/lib/bitbook/bitbook\n-rwsr-xr-x root/root     15232 1979-12-31 16:00 ./usr/lib/bitbook/chrome-sandbox\n-rw-r--r-- root/root    719546 1979-12-31 16:00 ./usr/lib/bitbook/chrome_100_percent.pak\n-rw-r--r-- root/root   1268994 1979-12-31 16:00 ./usr/lib/bitbook/chrome_200_percent.pak\n-rwxr-xr-x root/root   1926736 1979-12-31 16:00 ./usr/lib/bitbook/chrome_crashpad_handler\n-rw-r--r-- root/root  10876560 1979-12-31 16:00 ./usr/lib/bitbook/icudtl.dat\n-rwxr-xr-x root/root   2801768 1979-12-31 16:00 ./usr/lib/bitbook/libffmpeg.so\n-rwxr-xr-x root/root   4657848 1979-12-31 16:00 ./usr/lib/bitbook/libvk_swiftshader.so\n-rwxr-xr-x root/root   2483216 1979-12-31 16:00 ./usr/lib/bitbook/libvulkan.so.1\ndrwxr-xr-x root/root         0 2026-08-30 14:14 ./usr/lib/bitbook/locales/\n-rw-r--r-- root/root    115399 1979-12-31 16:00 ./usr/lib/bitbook/locales/af.pak\n-rw-r--r-- root/root    186161 1979-12-31 16:00 ./usr/lib/bitbook/locales/am.pak\n-rw-r--r-- root/root    185630 1979-12-31 16:00 ./usr/lib/bitbook/locales/ar.pak\n-rw-r--r-- root/root    211648 1979-12-31 16:00 ./usr/lib/bitbook/locales/bg.pak\n-rw-r--r-- root/root    269401 1979-12-31 16:00 ./usr/lib/bitbook/locales/bn.pak\n-rw-r--r-- root/root    129779 1979-12-31 16:00 ./usr/lib/bitbook/locales/ca.pak\n-rw-r--r-- root/root    130905 1979-12-31 16:00 ./usr/lib/bitbook/locales/cs.pak\n-rw-r--r-- root/root    119472 1979-12-31 16:00 ./usr/lib/bitbook/locales/da.pak\n-rw-r--r-- root/root    127397 1979-12-31 16:00 ./usr/lib/bitbook/locales/de.pak\n-rw-r--r-- root/root    229521 1979-12-31 16:00 ./usr/lib/bitbook/locales/el.pak\n-rw-r--r-- root/root    106855 1979-12-31 16:00 ./usr/lib/bitbook/locales/en-GB.pak\n-rw-r--r-- root/root    106959 1979-12-31 16:00 ./usr/lib/bitbook/locales/en-US.pak\n-rw-r--r-- root/root    128663 1979-12-31 16:00 ./usr/lib/bitbook/locales/es-419.pak\n-rw-r--r-- root/root    129345 1979-12-31 16:00 ./usr/lib/bitbook/locales/es.pak\n-rw-r--r-- root/root    116191 1979-12-31 16:00 ./usr/lib/bitbook/locales/et.pak\n-rw-r--r-- root/root    180526 1979-12-31 16:00 ./usr/lib/bitbook/locales/fa.pak\n-rw-r--r-- root/root    119427 1979-12-31 16:00 ./usr/lib/bitbook/locales/fi.pak\n-rw-r--r-- root/root    131693 1979-12-31 16:00 ./usr/lib/bitbook/locales/fil.pak\n-rw-r--r-- root/root    136115 1979-12-31 16:00 ./usr/lib/bitbook/locales/fr.pak\n-rw-r--r-- root/root    262681 1979-12-31 16:00 ./usr/lib/bitbook/locales/gu.pak\n-rw-r--r-- root/root    158977 1979-12-31 16:00 ./usr/lib/bitbook/locales/he.pak\n-rw-r--r-- root/root    278933 1979-12-31 16:00 ./usr/lib/bitbook/locales/hi.pak\n-rw-r--r-- root/root    125935 1979-12-31 16:00 ./usr/lib/bitbook/locales/hr.pak\n-rw-r--r-- root/root    137393 1979-12-31 16:00 ./usr/lib/bitbook/locales/hu.pak\n-rw-r--r-- root/root    114640 1979-12-31 16:00 ./usr/lib/bitbook/locales/id.pak\n-rw-r--r-- root/root    127462 1979-12-31 16:00 ./usr/lib/bitbook/locales/it.pak\n-rw-r--r-- root/root    147756 1979-12-31 16:00 ./usr/lib/bitbook/locales/ja.pak\n-rw-r--r-- root/root    295214 1979-12-31 16:00 ./usr/lib/bitbook/locales/kn.pak\n-rw-r--r-- root/root    126087 1979-12-31 16:00 ./usr/lib/bitbook/locales/ko.pak\n-rw-r--r-- root/root    134871 1979-12-31 16:00 ./usr/lib/bitbook/locales/lt.pak\n-rw-r--r-- root/root    135853 1979-12-31 16:00 ./usr/lib/bitbook/locales/lv.pak\n-rw-r--r-- root/root    312308 1979-12-31 16:00 ./usr/lib/bitbook/locales/ml.pak\n-rw-r--r-- root/root    261127 1979-12-31 16:00 ./usr/lib/bitbook/locales/mr.pak\n-rw-r--r-- root/root    117604 1979-12-31 16:00 ./usr/lib/bitbook/locales/ms.pak\n-rw-r--r-- root/root    115858 1979-12-31 16:00 ./usr/lib/bitbook/locales/nb.pak\n-rw-r--r-- root/root    121434 1979-12-31 16:00 ./usr/lib/bitbook/locales/nl.pak\n-rw-r--r-- root/root    131467 1979-12-31 16:00 ./usr/lib/bitbook/locales/pl.pak\n-rw-r--r-- root/root    126258 1979-12-31 16:00 ./usr/lib/bitbook/locales/pt-BR.pak\n-rw-r--r-- root/root    127689 1979-12-31 16:00 ./usr/lib/bitbook/locales/pt-PT.pak\n-rw-r--r-- root/root    132393 1979-12-31 16:00 ./usr/lib/bitbook/locales/ro.pak\n-rw-r--r-- root/root    207643 1979-12-31 16:00 ./usr/lib/bitbook/locales/ru.pak\n-rw-r--r-- root/root    133928 1979-12-31 16:00 ./usr/lib/bitbook/locales/sk.pak\n-rw-r--r-- root/root    126071 1979-12-31 16:00 ./usr/lib/bitbook/locales/sl.pak\n-rw-r--r-- root/root    196307 1979-12-31 16:00 ./usr/lib/bitbook/locales/sr.pak\n-rw-r--r-- root/root    116290 1979-12-31 16:00 ./usr/lib/bitbook/locales/sv.pak\n-rw-r--r-- root/root    124778 1979-12-31 16:00 ./usr/lib/bitbook/locales/sw.pak\n-rw-r--r-- root/root    314488 1979-12-31 16:00 ./usr/lib/bitbook/locales/ta.pak\n-rw-r--r-- root/root    289016 1979-12-31 16:00 ./usr/lib/bitbook/locales/te.pak\n-rw-r--r-- root/root    249549 1979-12-31 16:00 ./usr/lib/bitbook/locales/th.pak\n-rw-r--r-- root/root    125305 1979-12-31 16:00 ./usr/lib/bitbook/locales/tr.pak\n-rw-r--r-- root/root    209450 1979-12-31 16:00 ./usr/lib/bitbook/locales/uk.pak\n-rw-r--r-- root/root    185631 1979-12-31 16:00 ./usr/lib/bitbook/locales/ur.pak\n-rw-r--r-- root/root    149273 1979-12-31 16:00 ./usr/lib/bitbook/locales/vi.pak\n-rw-r--r-- root/root    103961 1979-12-31 16:00 ./usr/lib/bitbook/locales/zh-CN.pak\n-rw-r--r-- root/root    103133 1979-12-31 16:00 ./usr/lib/bitbook/locales/zh-TW.pak\ndrwxr-xr-x root/root         0 2026-09-10 08:02 ./usr/lib/bitbook/resources/\ndrwxr-xr-x root/root         0 2026-09-10 08:02 ./usr/lib/bitbook/resources/app/\ndrwxr-xr-x root/root         0 2026-09-10 08:02 ./usr/lib/bitbook/resources/app/imgs/\n-rw-r--r-- root/root    478786 2026-09-10 08:02 ./usr/lib/bitbook/resources/app/imgs/icon.png\n-rw-r--r-- root/root       219 2026-09-10 08:02 ./usr/lib/bitbook/resources/app/package.json\ndrwxr-xr-x root/root         0 2026-08-29 11:16 ./usr/lib/bitbook/resources/app/social/\n-rw-r--r-- root/root     26805 2026-08-29 11:14 ./usr/lib/bitbook/resources/app/social/app.js\n-rw-r--r-- root/root      1893 2026-08-29 11:09 ./usr/lib/bitbook/resources/app/social/core.js\n-rw-r--r-- root/root      7915 2026-08-29 22:55 ./usr/lib/bitbook/resources/app/social/index.html\n-rw-r--r-- root/root     14323 2026-08-29 12:38 ./usr/lib/bitbook/resources/app/social/styles.css\n-rw-r--r-- root/root      7564 2026-09-10 08:02 ./usr/lib/bitbook/resources/app/social-main.js\ndrwxr-xr-x root/root         0 2026-09-10 08:02 ./usr/lib/bitbook/resources/app/wallet-broker/\n-rw-r--r-- root/root      4518 2026-09-10 08:02 ./usr/lib/bitbook/resources/app/wallet-broker/launch-config.js\n-rw-r--r-- root/root     13482 2026-09-10 08:02 ./usr/lib/bitbook/resources/app/wallet-broker/protocol.js\n-rw-r--r-- root/root     20873 2026-09-10 08:02 ./usr/lib/bitbook/resources/app/wallet-broker/supervisor.js\ndrwxr-xr-x root/root         0 2026-09-10 08:02 ./usr/lib/bitbook/resources/app/wallet-pay/\n-rw-r--r-- root/root     23882 2026-09-10 08:02 ./usr/lib/bitbook/resources/app/wallet-pay/model.js\n-rw-r--r-- root/root      2556 2026-09-10 08:02 ./usr/lib/bitbook/resources/app/wallet-preload.js\n-rw-r--r-- root/root  12633730 1979-12-31 16:00 ./usr/lib/bitbook/resources.pak\n-rw-r--r-- root/root    368776 1979-12-31 16:00 ./usr/lib/bitbook/snapshot_blob.bin\n-rw-r--r-- root/root    742208 1979-12-31 16:00 ./usr/lib/bitbook/v8_context_snapshot.bin\n-rw-r--r-- root/root         6 1979-12-31 16:00 ./usr/lib/bitbook/version\n-rw-r--r-- root/root       107 1979-12-31 16:00 ./usr/lib/bitbook/vk_swiftshader_icd.json\ndrwxr-xr-x root/root         0 2026-09-10 08:02 ./usr/share/\ndrwxr-xr-x root/root         0 2026-09-10 08:02 ./usr/share/applications/\n-rw-r--r-- root/root       218 2026-09-10 08:02 ./usr/share/applications/bitbook.desktop\ndrwxr-xr-x root/root         0 2026-09-10 08:02 ./usr/share/doc/\ndrwxr-xr-x root/root         0 2026-09-10 08:02 ./usr/share/doc/bitbook/\n-rw-r--r-- root/root      1077 2026-09-10 08:02 ./usr/share/doc/bitbook/copyright\ndrwxr-xr-x root/root         0 2026-09-10 08:02 ./usr/share/icons/\ndrwxr-xr-x root/root         0 2026-09-10 08:02 ./usr/share/icons/hicolor/\ndrwxr-xr-x root/root         0 2026-09-10 08:02 ./usr/share/icons/hicolor/512x512/\ndrwxr-xr-x root/root         0 2026-09-10 08:02 ./usr/share/icons/hicolor/512x512/apps/\n-rw-r--r-- root/root    175081 2026-09-10 08:02 ./usr/share/icons/hicolor/512x512/apps/bitbook.png\nlrwxrwxrwx root/root         0 2026-09-10 08:02 ./usr/bin/bitbook -> ../lib/bitbook/bitbook\n"
    },
    {
      "argv": [
        "dpkg-deb",
        "--extract",
        "dist/bitbook_0.1.0_amd64.deb",
        "wallet-broker/target/wal011-runtime-package-extract-01"
      ],
      "exit": 0,
      "output": ""
    },
    {
      "argv": [
        "node",
        "-e",
        "const path=require('path');const root=process.argv[1];for(const rel of ['wallet-broker/supervisor.js','wallet-broker/protocol.js','wallet-broker/launch-config.js','wallet-pay/model.js'])require(path.resolve(root,rel));console.log('packaged nested imports resolved');",
        "wallet-broker/target/wal011-runtime-package-extract-01/usr/lib/bitbook/resources/app"
      ],
      "exit": 0,
      "output": "packaged nested imports resolved\n"
    }
  ],
  "component_green": true,
  "release_green": false,
  "version": "Hermes Agent v0.18.2 (2026.7.7.2) \u00b7 upstream 67764dc0 \u00b7 local 10b6d1a9 (+1 carried commit)\nInstall directory: <home>/.hermes/hermes-agent\nInstall method: git\nPython: 3.11.15\nOpenAI SDK: 2.24.0",
  "head": "fbaf8045676f1fdf922ec08763f0459c31163a3d",
  "initial_status": "M package-lock.json\n M package.json\n M scripts/build-deb.sh\n M scripts/build-macos.sh\n M scripts/build-windows.ps1\n M scripts/security-policy.js\n M test/securityPolicy.node.js\n?? docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md\n?? docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md\n?? docs/testing/BBD-WAL-011-RUNTIME-PACKAGE-RED-01.md\n?? scripts/stage-wallet-runtime.js\n?? test/walletRuntimePackage.node.js",
  "session": {
    "id": "20260910_080216_f2307a",
    "model": "poolside/laguna-s-2.1:free",
    "provider": "nous"
  },
  "input_hashes": {
    "scripts/build-deb.sh": "6ccaeed5fe31c487ed15d1d6424de1d9f2ef82f3b9ef79f90e4c4329e5142aa8",
    "scripts/build-macos.sh": "29e6ede69b2db49e780599af5a6a51cc4d263d3083683e6ea13c24321811f171",
    "scripts/build-windows.ps1": "8ce8faaff9e9a772540e63165122be9e30f78186f035570bbc63f6dd366d16c1",
    "wallet-preload.js": "3e6a18acf88dd5be4e6a88f326d6ace7a8071066480d9a70a2e8f89df035a1df",
    "wallet-broker/protocol.js": "79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4",
    "wallet-broker/supervisor.js": "c1410bfc5cd4c205ad014a42154c9d9d9714794b134a5acab7ed499934cad51a",
    "wallet-broker/launch-config.js": "68adf92e2d543e5d51353477b1944c35d4189c5590ac581d3289f1bd72898af8",
    "wallet-pay/model.js": "acf07238366f3e28253be8c9208fbb27e13e9c2687d374d1a6ae87e0d173fa5e",
    "social-main.js": "c7687b52ee45b17c3063cc2b39071be3d4e613a9d1e86558f2a7bbc1567db0a3",
    "packaging/runtime-package.json.in": "1660c63cf7d36abdfacf47333a69496c3a03121819100cb64a55e2c11a658d9e",
    "test/walletRuntimePackage.node.js": "5d8a8bf250a4e664fb15853cfb29deee62130efd72cdcf3acdac1129c2324f25",
    "package.json": "76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5",
    "package-lock.json": "0e32de43f3c1a31ae3d9e9c8b172d10cd37a03577dc7d1eaf4ac916967cfcdb8",
    "scripts/security-policy.js": "fec10759a73a3c675d8206da454abf4b0e8dca6b62da463f39a2db27b30dc078",
    "test/securityPolicy.node.js": "a58d85e5322f0cb01a78b636454f92547bd98e556013cc07712dc0cf64ea565c",
    "docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md": "1e548d364ac3a39e9bb1941a5c89b0ccb45143d9c8588e35e42f12434120799f",
    "docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md": "2a6abd0a41a93028ce63fce8b3f385436623a416dfcdc26565b0277e7875bc56",
    "scripts/stage-wallet-runtime.js": "97d04b4434e498bf4a2dd70d1ec69881221d8466664656e87fbffdd1c0694a70",
    "test/walletBrokerLaunchConfig.node.js": "90acd32c9863df0206364eb04f707d92a5979ee66c7e15b2ec7b8250a5fc2113",
    "test/electronSecurity.node.js": "7f759f810762ccdfa5e29d69dc3bf2fa1b1502d2c416ca2b324442b2544ad18c",
    "docs/testing/BBD-WAL-011-RUNTIME-PACKAGE-RED-01.md": "d6c1d5f642b23fa31c13ae933bb661359b123932c6d0541f53b66a9d6b6fa791",
    "node_modules/electron/package.json": "3280e83f3a5a5d0825f44708a12c7c695b03034dd0e07356790c655de67cbedf",
    ".gitleaksignore": "1e239ec10a1f2ccf59711258fe514f827727e984ca063a6a685ab325313b563b"
  },
  "policy_comparison": [
    {
      "argv": [
        "node",
        "test/securityPolicy.node.js"
      ],
      "exit": 1,
      "identical_to_baseline": true
    },
    {
      "argv": [
        "node",
        "scripts/security-policy.js"
      ],
      "exit": 1,
      "identical_to_baseline": true
    }
  ],
  "known_release_blockers": [
    "Six inherited security-policy test failures",
    "Repository Rust source inventory mismatch"
  ],
  "gitleaks_version": "8.30.1",
  "preserved_old_package": {
    "path": "dist/bitbook_0.1.0_amd64.pre-wal011-runtime-package-01.deb",
    "sha256": "67f074224711c249767fd86112cd2f84007d275dbd74283baa917c4f1bfff8f2"
  },
  "retained_staging": "<repo>/dist/bitbook-deb.umbyu6",
  "package": {
    "path": "dist/bitbook_0.1.0_amd64.deb",
    "sha256": "544127be43fa4474eaf6903f660131d64f840b64fd3f7cb8562ea8dfae924672",
    "bytes": 95946046
  },
  "sandbox_entry": "-rwsr-xr-x root/root     15232 1979-12-31 16:00 ./usr/lib/bitbook/chrome-sandbox",
  "packaged_hashes": {
    "wallet-preload.js": "3e6a18acf88dd5be4e6a88f326d6ace7a8071066480d9a70a2e8f89df035a1df",
    "wallet-broker/protocol.js": "79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4",
    "wallet-broker/supervisor.js": "c1410bfc5cd4c205ad014a42154c9d9d9714794b134a5acab7ed499934cad51a",
    "wallet-broker/launch-config.js": "68adf92e2d543e5d51353477b1944c35d4189c5590ac581d3289f1bd72898af8",
    "wallet-pay/model.js": "acf07238366f3e28253be8c9208fbb27e13e9c2687d374d1a6ae87e0d173fa5e"
  },
  "final_hashes": {
    "scripts/build-deb.sh": "6ccaeed5fe31c487ed15d1d6424de1d9f2ef82f3b9ef79f90e4c4329e5142aa8",
    "scripts/build-macos.sh": "29e6ede69b2db49e780599af5a6a51cc4d263d3083683e6ea13c24321811f171",
    "scripts/build-windows.ps1": "8ce8faaff9e9a772540e63165122be9e30f78186f035570bbc63f6dd366d16c1",
    "wallet-preload.js": "3e6a18acf88dd5be4e6a88f326d6ace7a8071066480d9a70a2e8f89df035a1df",
    "wallet-broker/protocol.js": "79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4",
    "wallet-broker/supervisor.js": "c1410bfc5cd4c205ad014a42154c9d9d9714794b134a5acab7ed499934cad51a",
    "wallet-broker/launch-config.js": "68adf92e2d543e5d51353477b1944c35d4189c5590ac581d3289f1bd72898af8",
    "wallet-pay/model.js": "acf07238366f3e28253be8c9208fbb27e13e9c2687d374d1a6ae87e0d173fa5e",
    "social-main.js": "c7687b52ee45b17c3063cc2b39071be3d4e613a9d1e86558f2a7bbc1567db0a3",
    "packaging/runtime-package.json.in": "1660c63cf7d36abdfacf47333a69496c3a03121819100cb64a55e2c11a658d9e",
    "test/walletRuntimePackage.node.js": "5d8a8bf250a4e664fb15853cfb29deee62130efd72cdcf3acdac1129c2324f25",
    "package.json": "76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5",
    "package-lock.json": "0e32de43f3c1a31ae3d9e9c8b172d10cd37a03577dc7d1eaf4ac916967cfcdb8",
    "scripts/security-policy.js": "fec10759a73a3c675d8206da454abf4b0e8dca6b62da463f39a2db27b30dc078",
    "test/securityPolicy.node.js": "a58d85e5322f0cb01a78b636454f92547bd98e556013cc07712dc0cf64ea565c",
    "docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md": "1e548d364ac3a39e9bb1941a5c89b0ccb45143d9c8588e35e42f12434120799f",
    "docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md": "2a6abd0a41a93028ce63fce8b3f385436623a416dfcdc26565b0277e7875bc56",
    "scripts/stage-wallet-runtime.js": "97d04b4434e498bf4a2dd70d1ec69881221d8466664656e87fbffdd1c0694a70",
    "test/walletBrokerLaunchConfig.node.js": "90acd32c9863df0206364eb04f707d92a5979ee66c7e15b2ec7b8250a5fc2113",
    "test/electronSecurity.node.js": "7f759f810762ccdfa5e29d69dc3bf2fa1b1502d2c416ca2b324442b2544ad18c",
    "docs/testing/BBD-WAL-011-RUNTIME-PACKAGE-RED-01.md": "d6c1d5f642b23fa31c13ae933bb661359b123932c6d0541f53b66a9d6b6fa791",
    "node_modules/electron/package.json": "3280e83f3a5a5d0825f44708a12c7c695b03034dd0e07356790c655de67cbedf",
    ".gitleaksignore": "1e239ec10a1f2ccf59711258fe514f827727e984ca063a6a685ab325313b563b"
  },
  "final_helper_sha256": "97d04b4434e498bf4a2dd70d1ec69881221d8466664656e87fbffdd1c0694a70"
}
```
