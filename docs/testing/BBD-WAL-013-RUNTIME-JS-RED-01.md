# WAL-013 runtime and menu expected red

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
      "output": "b328e6c29d449bd1efd4efbfaf0fc835e6b85940\n"
    },
    {
      "argv": [
        "git",
        "status",
        "--short"
      ],
      "exit": 0,
      "output": " M package-lock.json\n M package.json\n M scripts/security-policy.js\n M test/electronSecurity.node.js\n M test/securityPolicy.node.js\n M test/walletBrokerBuild.node.js\n M test/walletBrokerRuntime.node.js\n M test/walletStartup.node.js\n M test/walletStartupSmoke.node.js\n M test/walletSupervisorShutdown.node.js\n M wallet-broker/Cargo.toml\n M wallet-broker/src/lib.rs\n?? docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md\n?? docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md\n?? docs/testing/BBD-WAL-013-ACCOUNTS-GREEN-UI-RED-01.md\n?? docs/testing/BBD-WAL-013-ACCOUNTS-RED-01.md\n?? test/fixtures/wallet-broker/x11-window.py\n?? test/walletAccountManagement.node.js\n?? test/walletAccountWindowSmoke.node.js\n?? wallet-broker/src/accounts.rs\n?? wallet-broker/tests/account_management.rs\n?? wallet-broker/tests/account_native_ui.rs\n"
    },
    {
      "argv": [
        "node",
        "test/walletAccountManagement.node.js"
      ],
      "exit": 1,
      "output": "not ok ready menu registers Wallet/Manage accounts; bound click dispatches account.manage{}; failures stay closed\nAssertionError [ERR_ASSERTION]: Wallet menu is missing\n    at <repo>/test/walletAccountManagement.node.js:690:12\n    at withMain (<repo>/test/walletAccountManagement.node.js:471:26)\n    at <repo>/test/walletAccountManagement.node.js:686:9\n    at run (<repo>/test/walletAccountManagement.node.js:1146:13)\n    at Object.<anonymous> (<repo>/test/walletAccountManagement.node.js:1161:3)\n    at Module._compile (node:internal/modules/cjs/loader:1781:14)\n    at Object..js (node:internal/modules/cjs/loader:1913:10)\n    at Module.load (node:internal/modules/cjs/loader:1505:32)\n    at Function.<anonymous> (node:internal/modules/cjs/loader:1309:12)\n    at wrapModuleLoad (node:internal/modules/cjs/loader:254:19)\nnot ok quit and repeated ready cannot open the account window\nAssertionError [ERR_ASSERTION]: Manage accounts click handler is missing\n    at invokeClick (<repo>/test/walletAccountManagement.node.js:192:10)\n    at <repo>/test/walletAccountManagement.node.js:753:5\n    at async withMain (<repo>/test/walletAccountManagement.node.js:471:20)\n    at async <repo>/test/walletAccountManagement.node.js:743:3\n    at async run (<repo>/test/walletAccountManagement.node.js:1146:7)\nnot ok supervisor spawn env is an own-data allowlist copy of the seven GUI strings\nAssertionError [ERR_ASSERTION]: Expected values to be strictly deep-equal:\n+ actual - expected\n\n  {\n-   DBUS_SESSION_BUS_ADDRESS: 'unix:path=/run/user/1000/bus',\n-   DISPLAY: ':0',\n    LANG: 'C.UTF-8',\n    PATH: 'zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz',\n-   WAYLAND_DISPLAY: 'wayland-0',\n-   XAUTHORITY: '/home/user/.Xauthority',\n-   XDG_RUNTIME_DIR: '/run/user/1000'\n  }\n\n    at <repo>/test/walletAccountManagement.node.js:813:12\n    at withFakeHarness (<repo>/test/walletAccountManagement.node.js:640:20)\n    at <repo>/test/walletAccountManagement.node.js:800:9\n    at run (<repo>/test/walletAccountManagement.node.js:1146:13)\nnot ok main copies the seven allowlisted process.env strings and never passes process.env identity\nAssertionError [ERR_ASSERTION]: main did not supply supervisor env\n    at <repo>/test/walletAccountManagement.node.js:912:14\n    at withMain (<repo>/test/walletAccountManagement.node.js:471:26)\n    at <repo>/test/walletAccountManagement.node.js:909:11\n    at run (<repo>/test/walletAccountManagement.node.js:1146:13)\nnot ok dispatcher accepts account.manage{} only while bound and rejects secret-bearing methods\nAssertionError [ERR_ASSERTION]: account.manage missing from BROKER_METHODS\n    at <repo>/test/walletAccountManagement.node.js:926:10\n    at run (<repo>/test/walletAccountManagement.node.js:1146:13)\nnot ok graceful EOF ends stdin once, waits 1000/1250/1500, and resolves only on close\nAssertionError [ERR_ASSERTION]: stdin.end was not called\n    at <repo>/test/walletAccountManagement.node.js:993:12\n    at async withFakeHarness (<repo>/test/walletAccountManagement.node.js:640:14)\n    at async <repo>/test/walletAccountManagement.node.js:971:3\n    at async run (<repo>/test/walletAccountManagement.node.js:1146:7)\nnot ok graceful asynchronous and synchronous close are safe; missing or throwing end falls back immediately\nAssertionError [ERR_ASSERTION]: The expression evaluated to a falsy value:\n\n  assert.ok(cancelIndex >= 0 && endIndex >= 0 && cancelIndex < endIndex)\n\n    at <repo>/test/walletAccountManagement.node.js:1059:12\n    at async withFakeHarness (<repo>/test/walletAccountManagement.node.js:640:14)\n    at async <repo>/test/walletAccountManagement.node.js:1047:3\n    at async run (<repo>/test/walletAccountManagement.node.js:1146:7)\n7 wallet account management test(s) failed\n"
    },
    {
      "argv": [
        "node",
        "test/walletBrokerBuild.node.js"
      ],
      "exit": 1,
      "output": "ok module: exports synchronous stageWalletBroker\nok stage: copied bytes pin independently, modes match identity, resolver accepts layout\nok stage: repeat replacement preserves unrelated files and retains staging leftovers\nok preflight: invalid source, roots, and identities leave destination bytes unchanged\nok preflight: destination directory and pin links leave bytes unchanged\nnot ok cli: vm-mocked cargo command is exact and nonzero skips staging\nAssertionError [ERR_ASSERTION]: Expected values to be strictly deep-equal:\n+ actual - expected\n... Skipped lines\n\n  [\n    'run',\n    '1.98.0',\n    'cargo',\n    'build',\n...\n    '--no-default-features',\n-   '--features',\n-   'native-ui',\n    '--bin',\n    'bitbook-wallet-broker'\n  ]\n\n    at <repo>/test/walletBrokerBuild.node.js:563:12\n    at withOwnedTemp (<repo>/test/walletBrokerBuild.node.js:99:12)\n    at <repo>/test/walletBrokerBuild.node.js:546:3\n    at run (<repo>/test/walletBrokerBuild.node.js:606:7)\n    at Object.<anonymous> (<repo>/test/walletBrokerBuild.node.js:620:30)\n    at Module._compile (node:internal/modules/cjs/loader:1781:14)\n    at Object..js (node:internal/modules/cjs/loader:1913:10)\n    at Module.load (node:internal/modules/cjs/loader:1505:32)\n    at Function._load (node:internal/modules/cjs/loader:1309:12)\n    at wrapModuleLoad (node:internal/modules/cjs/loader:254:19)\n1 wallet broker build test(s) failed\n"
    }
  ],
  "accepted": true,
  "version": "Hermes Agent v0.18.2 (2026.7.7.2) \u00b7 upstream 67764dc0 \u00b7 local 10b6d1a9 (+1 carried commit)\nInstall directory: <home>/.hermes/hermes-agent\nInstall method: git\nPython: 3.11.15\nOpenAI SDK: 2.24.0",
  "head": "b328e6c29d449bd1efd4efbfaf0fc835e6b85940",
  "initial_status": "M package-lock.json\n M package.json\n M scripts/security-policy.js\n M test/electronSecurity.node.js\n M test/securityPolicy.node.js\n M test/walletBrokerBuild.node.js\n M test/walletBrokerRuntime.node.js\n M test/walletStartup.node.js\n M test/walletStartupSmoke.node.js\n M test/walletSupervisorShutdown.node.js\n M wallet-broker/Cargo.toml\n M wallet-broker/src/lib.rs\n?? docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md\n?? docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md\n?? docs/testing/BBD-WAL-013-ACCOUNTS-GREEN-UI-RED-01.md\n?? docs/testing/BBD-WAL-013-ACCOUNTS-RED-01.md\n?? test/fixtures/wallet-broker/x11-window.py\n?? test/walletAccountManagement.node.js\n?? test/walletAccountWindowSmoke.node.js\n?? wallet-broker/src/accounts.rs\n?? wallet-broker/tests/account_management.rs\n?? wallet-broker/tests/account_native_ui.rs",
  "session": {
    "id": "20260910_105327_6c4021",
    "model": "poolside/laguna-s-2.1:free",
    "provider": "nous"
  },
  "input_hashes": {
    "test/walletAccountManagement.node.js": "94a40ddbedea2298f707bb942218255ab6f55e7db9903c9401d18190f57eaaea",
    "test/walletBrokerBuild.node.js": "b1ac8ae3a1ca76c9580c92fec7cd1cbca0edb482246f072afcafb22a79e689de",
    "test/walletBrokerRuntime.node.js": "989c5f64716d22e8b2903d6bf736fdadcae253d9ed11c6c648aec0fc2de14592",
    "test/walletStartup.node.js": "4a64be8628ff54e74f282703abfcaf286c9ae9cec96a15e1d13561a4fb0d501e",
    "test/walletStartupSmoke.node.js": "a1c75efee5fcfdbeb9bf56dfc64f0e747ab6aa527d30ce416c5027bc2aa2a1ef",
    "test/walletSupervisorShutdown.node.js": "f1156d8bf81a03a40ec9eb235666275ce85a3394b12f7b567bbb9da20a7543ef",
    "test/electronSecurity.node.js": "d70d6337ffbe97dfeee1894895c757b804dc18ae819219e8742adda5358ab482",
    "social-main.js": "7047253ee27955b0838ec226cd1e49c4a7dca8c3cc2a54a776fb1211c8a820d0",
    "wallet-broker/supervisor.js": "c1410bfc5cd4c205ad014a42154c9d9d9714794b134a5acab7ed499934cad51a",
    "wallet-broker/src/runtime.rs": "968194aadd609f54ef28a11a090fcb1ca840cfc851da127456dd5b356437305b",
    "scripts/build-wallet-broker.js": "c96684b9d7242a020a93c0ea8a96d590c7a262482f44726ac12ce14ab618e382",
    "package.json": "76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5",
    "package-lock.json": "0e32de43f3c1a31ae3d9e9c8b172d10cd37a03577dc7d1eaf4ac916967cfcdb8",
    "scripts/security-policy.js": "fec10759a73a3c675d8206da454abf4b0e8dca6b62da463f39a2db27b30dc078",
    "test/securityPolicy.node.js": "a58d85e5322f0cb01a78b636454f92547bd98e556013cc07712dc0cf64ea565c"
  },
  "final_hashes": {
    "test/walletAccountManagement.node.js": "94a40ddbedea2298f707bb942218255ab6f55e7db9903c9401d18190f57eaaea",
    "test/walletBrokerBuild.node.js": "b1ac8ae3a1ca76c9580c92fec7cd1cbca0edb482246f072afcafb22a79e689de",
    "test/walletBrokerRuntime.node.js": "989c5f64716d22e8b2903d6bf736fdadcae253d9ed11c6c648aec0fc2de14592",
    "test/walletStartup.node.js": "4a64be8628ff54e74f282703abfcaf286c9ae9cec96a15e1d13561a4fb0d501e",
    "test/walletStartupSmoke.node.js": "a1c75efee5fcfdbeb9bf56dfc64f0e747ab6aa527d30ce416c5027bc2aa2a1ef",
    "test/walletSupervisorShutdown.node.js": "f1156d8bf81a03a40ec9eb235666275ce85a3394b12f7b567bbb9da20a7543ef",
    "test/electronSecurity.node.js": "d70d6337ffbe97dfeee1894895c757b804dc18ae819219e8742adda5358ab482",
    "social-main.js": "7047253ee27955b0838ec226cd1e49c4a7dca8c3cc2a54a776fb1211c8a820d0",
    "wallet-broker/supervisor.js": "c1410bfc5cd4c205ad014a42154c9d9d9714794b134a5acab7ed499934cad51a",
    "wallet-broker/src/runtime.rs": "968194aadd609f54ef28a11a090fcb1ca840cfc851da127456dd5b356437305b",
    "scripts/build-wallet-broker.js": "c96684b9d7242a020a93c0ea8a96d590c7a262482f44726ac12ce14ab618e382",
    "package.json": "76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5",
    "package-lock.json": "0e32de43f3c1a31ae3d9e9c8b172d10cd37a03577dc7d1eaf4ac916967cfcdb8",
    "scripts/security-policy.js": "fec10759a73a3c675d8206da454abf4b0e8dca6b62da463f39a2db27b30dc078",
    "test/securityPolicy.node.js": "a58d85e5322f0cb01a78b636454f92547bd98e556013cc07712dc0cf64ea565c"
  }
}
```
