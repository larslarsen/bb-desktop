# WAL-013 empty-window regression red

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
      "output": "46d61acd963c9d0b234415d5b5fa3e1d94b72970\n"
    },
    {
      "argv": [
        "git",
        "status",
        "--short"
      ],
      "exit": 0,
      "output": " M package-lock.json\n M package.json\n M scripts/security-policy.js\n M test/fixtures/wallet-broker/x11-window.py\n M test/securityPolicy.node.js\n M test/walletAccountManagement.node.js\n M test/walletAccountWindowSmoke.node.js\n M test/walletStartup.node.js\n?? docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md\n?? docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md\n"
    },
    {
      "argv": [
        "node",
        "test/walletAccountManagement.node.js"
      ],
      "exit": 1,
      "output": "ok ready menu registers Wallet/Manage accounts; bound click dispatches account.manage{}; failures stay closed\nok quit and repeated ready cannot open the account window\nnot ok supervisor spawn env is an own-data allowlist copy of the six X11 GUI strings\nAssertionError [ERR_ASSERTION]: spawn unexpected env WAYLAND_DISPLAY\n    at assertAllowlistedEnvCopy (<repo>/test/walletAccountManagement.node.js:200:12)\n    at <repo>/test/walletAccountManagement.node.js:810:5\n    at withFakeHarness (<repo>/test/walletAccountManagement.node.js:639:20)\n    at <repo>/test/walletAccountManagement.node.js:798:9\n    at run (<repo>/test/walletAccountManagement.node.js:1180:13)\nnot ok main copies the six allowlisted process.env strings and omits WAYLAND_DISPLAY\nAssertionError [ERR_ASSERTION]: main factory unexpected env WAYLAND_DISPLAY\n    at assertAllowlistedEnvCopy (<repo>/test/walletAccountManagement.node.js:200:12)\n    at <repo>/test/walletAccountManagement.node.js:946:7\n    at withMain (<repo>/test/walletAccountManagement.node.js:470:26)\n    at <repo>/test/walletAccountManagement.node.js:941:11\n    at run (<repo>/test/walletAccountManagement.node.js:1180:13)\nok dispatcher accepts account.manage{} only while bound and rejects secret-bearing methods\nok graceful EOF ends stdin once, waits 1000/1250/1500, and resolves only on close\nok graceful asynchronous and synchronous close are safe; missing or throwing end falls back immediately\n2 wallet account management test(s) failed\n"
    },
    {
      "argv": [
        "node",
        "test/walletStartup.node.js"
      ],
      "exit": 1,
      "output": "not ok ready-only packaged and development paths resolve once with subscribe-before-start\nAssertionError [ERR_ASSERTION]: unexpected supervisor env WAYLAND_DISPLAY\n    at assertConfiguredFactory (<repo>/test/walletStartup.node.js:345:14)\n    at <repo>/test/walletStartup.node.js:423:5\n    at withMain (<repo>/test/walletStartup.node.js:396:26)\n    at <repo>/test/walletStartup.node.js:410:9\n    at run (<repo>/test/walletStartup.node.js:628:13)\n    at Object.<anonymous> (<repo>/test/walletStartup.node.js:643:3)\n    at Module._compile (node:internal/modules/cjs/loader:1781:14)\n    at Object..js (node:internal/modules/cjs/loader:1913:10)\n    at Module.load (node:internal/modules/cjs/loader:1505:32)\n    at Function.<anonymous> (node:internal/modules/cjs/loader:1309:12)\nok unbound snapshot get returns cached down clones then live status after handshake\nok resolver or start failure leaves social usable; fallback still denies sender and payload\nok snapshot cache updates while the window is closed and clones stay isolated\nok pre-ready quit and reentrant quit during resolver never spawn; activate stays idle-gated\nok configured supervisor shutdown is awaited on the replaced instance\n1 wallet startup test(s) failed\n"
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
      "exit": 1,
      "output": "not ok actual supervisor opens, hides, reopens, and gracefully closes the native account window\nAssertionError [ERR_ASSERTION]: broker environment was not the fixed isolated Xvfb environment\n    at assertSpawn (<repo>/test/walletAccountWindowSmoke.node.js:572:10)\n    at Object.fn (<repo>/test/walletAccountWindowSmoke.node.js:702:5)\n    at run (<repo>/test/walletAccountWindowSmoke.node.js:805:19)\n    at Object.<anonymous> (<repo>/test/walletAccountWindowSmoke.node.js:821:3)\n    at Module._compile (node:internal/modules/cjs/loader:1781:14)\n    at Object..js (node:internal/modules/cjs/loader:1913:10)\n    at Module.load (node:internal/modules/cjs/loader:1505:32)\n    at Function._load (node:internal/modules/cjs/loader:1309:12)\n    at wrapModuleLoad (node:internal/modules/cjs/loader:254:19)\n    at Function.executeUserEntryPoint [as runMain] (node:internal/modules/run_main:171:5)\n1 native account window smoke test(s) failed\n"
    }
  ],
  "accepted": true,
  "version": "Hermes Agent v0.18.2 (2026.7.7.2) \u00b7 upstream 67764dc0 \u00b7 local 10b6d1a9 (+1 carried commit)\nInstall directory: <home>/.hermes/hermes-agent\nInstall method: git\nPython: 3.11.15\nOpenAI SDK: 2.24.0",
  "head": "46d61acd963c9d0b234415d5b5fa3e1d94b72970",
  "initial_status": "M package-lock.json\n M package.json\n M scripts/security-policy.js\n M test/fixtures/wallet-broker/x11-window.py\n M test/securityPolicy.node.js\n M test/walletAccountManagement.node.js\n M test/walletAccountWindowSmoke.node.js\n M test/walletStartup.node.js\n?? docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md\n?? docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md",
  "session": {
    "id": "20260910_123845_688725",
    "model": "meituan/longcat-2.0:free",
    "provider": "nous"
  },
  "input_hashes": {
    "social-main.js": "29fa41d88cb75d96b00bc2fe740d8701c9ddfbaf4ba2ae71be84b8755e192d30",
    "wallet-broker/supervisor.js": "1258444add1741dd6ba850c241871b8e68be71e8550dac14b0e8af6959b9f7a2",
    "scripts/build-wallet-broker.js": "4427ef3e713ec77c45daefe8d37b457ca8dc2c8529b30214dbf65628f43a1445",
    "wallet-broker/src/runtime.rs": "6788914524605f4669d20d65f7b6ef7282b0090a0696ea45f088ecc49dc1c962",
    "test/walletAccountManagement.node.js": "307480c4620c704af6f5a6f96afda0bf8941d5ee1a7583f6683dc30be0bb62bb",
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
    "test/walletAccountWindowSmoke.node.js": "5661f8945b7cebc331e4a2a1feb290d9ab0b2e77f4979a9adcaf2ac6cb3cdc94",
    "test/fixtures/wallet-broker/x11-window.py": "6a049e8dbcd10522d6a7c65f46792afac2c980aa33a413041e118a9d964aa25b",
    "wallet-broker/target/wal011-runtime-package-red-01.json": "5608cd092ae81d83e658aa9231e2751378f5f15fbde3a60c2b9dc176d5a370bd"
  },
  "final_hashes": {
    "social-main.js": "29fa41d88cb75d96b00bc2fe740d8701c9ddfbaf4ba2ae71be84b8755e192d30",
    "wallet-broker/supervisor.js": "1258444add1741dd6ba850c241871b8e68be71e8550dac14b0e8af6959b9f7a2",
    "scripts/build-wallet-broker.js": "4427ef3e713ec77c45daefe8d37b457ca8dc2c8529b30214dbf65628f43a1445",
    "wallet-broker/src/runtime.rs": "6788914524605f4669d20d65f7b6ef7282b0090a0696ea45f088ecc49dc1c962",
    "test/walletAccountManagement.node.js": "307480c4620c704af6f5a6f96afda0bf8941d5ee1a7583f6683dc30be0bb62bb",
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
    "test/walletAccountWindowSmoke.node.js": "5661f8945b7cebc331e4a2a1feb290d9ab0b2e77f4979a9adcaf2ac6cb3cdc94",
    "test/fixtures/wallet-broker/x11-window.py": "6a049e8dbcd10522d6a7c65f46792afac2c980aa33a413041e118a9d964aa25b",
    "wallet-broker/target/wal011-runtime-package-red-01.json": "5608cd092ae81d83e658aa9231e2751378f5f15fbde3a60c2b9dc176d5a370bd"
  }
}
```
