# WAL-013 JavaScript account menu and lifecycle green

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
      "output": "5bfa5fcff52a42fbc2689d1a496f308311b77e7c\n"
    },
    {
      "argv": [
        "git",
        "status",
        "--short"
      ],
      "exit": 0,
      "output": " M package-lock.json\n M package.json\n M scripts/build-wallet-broker.js\n M scripts/security-policy.js\n M social-main.js\n M test/electronSecurity.node.js\n M test/securityPolicy.node.js\n M test/walletBrokerBuild.node.js\n M test/walletBrokerRuntime.node.js\n M test/walletStartup.node.js\n M test/walletStartupSmoke.node.js\n M test/walletSupervisorShutdown.node.js\n M wallet-broker/Cargo.toml\n M wallet-broker/src/lib.rs\n M wallet-broker/src/runtime.rs\n M wallet-broker/supervisor.js\n?? docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md\n?? docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md\n?? docs/testing/BBD-WAL-013-ACCOUNTS-GREEN-UI-RED-01.md\n?? docs/testing/BBD-WAL-013-ACCOUNTS-RED-01.md\n?? docs/testing/BBD-WAL-013-RUNTIME-JS-RED-01.md\n?? test/fixtures/wallet-broker/x11-window.py\n?? test/walletAccountManagement.node.js\n?? test/walletAccountWindowSmoke.node.js\n?? wallet-broker/src/account_ui.rs\n?? wallet-broker/src/accounts.rs\n?? wallet-broker/tests/account_management.rs\n?? wallet-broker/tests/account_native_ui.rs\n"
    },
    {
      "argv": [
        "node",
        "test/walletAccountManagement.node.js"
      ],
      "exit": 0,
      "output": "ok ready menu registers Wallet/Manage accounts; bound click dispatches account.manage{}; failures stay closed\nok quit and repeated ready cannot open the account window\nok supervisor spawn env is an own-data allowlist copy of the seven GUI strings\nok main copies the seven allowlisted process.env strings and never passes process.env identity\nok dispatcher accepts account.manage{} only while bound and rejects secret-bearing methods\nok graceful EOF ends stdin once, waits 1000/1250/1500, and resolves only on close\nok graceful asynchronous and synchronous close are safe; missing or throwing end falls back immediately\nBitBook wallet account management tests passed (7).\n"
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
        "test/walletSupervisorShutdown.node.js"
      ],
      "exit": 0,
      "output": "ok shutdown: no-start and failed-start resolve one Promise without spawn, kill, or timers\nok shutdown: bound cancel precedes SIGTERM and close, not exit, completes one Promise\nok shutdown: prior quit, protocol failure, and close-before-shutdown keep one termination\nok shutdown: stubborn child SIGKILL at 250 ms and TIMEOUT at 1500 ms\nok shutdown: real child normal termination observes close and closed streams\nok shutdown: graceful stdin.end waits 1000ms then SIGTERM and SIGKILL; close still owns the promise\nok shutdown: asynchronous/synchronous close from end are safe and throwing end falls back\nok shutdown: real child ignoring SIGTERM terminates with SIGKILL after close\nBitBook wallet supervisor shutdown tests passed (8).\n"
    },
    {
      "argv": [
        "node",
        "test/walletSupervisorTransport.node.js"
      ],
      "exit": 0,
      "output": "ok transport: real framed hello, ack, bootstrap and a subsequent request return a distinct fixture result\nok transport: concurrent requests resolve from reverse-order coalesced replies and broker errors drop diagnostic sentinels\nok transport: wrong session or uncorrelated response closes and rejects pending work\nok transport: partial-frame EOF and malformed frames close without a hanging promise\nok transport: stdout data events reassemble split header/body frames before ack, bootstrap and a distinct result\nok transport: handshake and request deadlines fail closed\nok transport: request limit does not write; quit and exit settle pending work and release the child\nBitBook wallet supervisor transport tests passed (7).\n"
    },
    {
      "argv": [
        "node",
        "test/walletSupervisor.node.js"
      ],
      "exit": 1,
      "output": "ok launch: private data directory and regular readable pinned binary precede one inert spawn\nok launch: missing, non-file, symlink, unreadable, and hash mismatch never spawn\nok launch: missing, symlinked, non-directory, or non-0700 data directories never verify or spawn\nok handshake: real child-first fixture transcript binds both directions within two seconds\nok handshake: PID, session, diagnostics, timeout, and early exit failures never dispatch\nnot ok dispatch: exact supervisor methods and closed parameter schemas are enforced after binding\nAssertionError [ERR_ASSERTION]: Expected values to be strictly deep-equal:\n+ actual - expected\n\n  [\n    'status.get',\n    'account.list',\n    'account.lock',\n+   'account.manage',\n    'receiver.fresh',\n    'intent.begin',\n    'intent.cancel',\n    'sync.subscribe'\n  ]\n\n    at <repo>/test/walletSupervisor.node.js:288:10\n    at run (<repo>/test/walletSupervisor.node.js:513:17)\nok dispatch: pre-bind and oversize calls fail before broker send\nok dispatch: matching replies settle promises out of order with cloned sanitized results\nok lifecycle: exit publishes only sanitized down state and restart never buffers spend requests\nok quit: every in-flight intent is cancelled before child termination\nok quit: an unbound child terminates without any application frame\nok snapshot: supervisor exports the shared Pay sanitizer and removes every fixture canary\nok snapshot: sync publication traverses the shared sanitizer before every subscriber\n"
    }
  ],
  "accepted": false,
  "version": "Hermes Agent v0.18.2 (2026.7.7.2) \u00b7 upstream 67764dc0 \u00b7 local 10b6d1a9 (+1 carried commit)\nInstall directory: <home>/.hermes/hermes-agent\nInstall method: git\nPython: 3.11.15\nOpenAI SDK: 2.24.0",
  "head": "5bfa5fcff52a42fbc2689d1a496f308311b77e7c",
  "initial_status": "M package-lock.json\n M package.json\n M scripts/build-wallet-broker.js\n M scripts/security-policy.js\n M social-main.js\n M test/electronSecurity.node.js\n M test/securityPolicy.node.js\n M test/walletBrokerBuild.node.js\n M test/walletBrokerRuntime.node.js\n M test/walletStartup.node.js\n M test/walletStartupSmoke.node.js\n M test/walletSupervisorShutdown.node.js\n M wallet-broker/Cargo.toml\n M wallet-broker/src/lib.rs\n M wallet-broker/src/runtime.rs\n M wallet-broker/supervisor.js\n?? docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md\n?? docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md\n?? docs/testing/BBD-WAL-013-ACCOUNTS-GREEN-UI-RED-01.md\n?? docs/testing/BBD-WAL-013-ACCOUNTS-RED-01.md\n?? docs/testing/BBD-WAL-013-RUNTIME-JS-RED-01.md\n?? test/fixtures/wallet-broker/x11-window.py\n?? test/walletAccountManagement.node.js\n?? test/walletAccountWindowSmoke.node.js\n?? wallet-broker/src/account_ui.rs\n?? wallet-broker/src/accounts.rs\n?? wallet-broker/tests/account_management.rs\n?? wallet-broker/tests/account_native_ui.rs",
  "session": {
    "id": "20260910_110503_11c10a",
    "model": "poolside/laguna-s-2.1:free",
    "provider": "nous"
  },
  "input_hashes": {
    "social-main.js": "29fa41d88cb75d96b00bc2fe740d8701c9ddfbaf4ba2ae71be84b8755e192d30",
    "wallet-broker/supervisor.js": "1258444add1741dd6ba850c241871b8e68be71e8550dac14b0e8af6959b9f7a2",
    "scripts/build-wallet-broker.js": "4427ef3e713ec77c45daefe8d37b457ca8dc2c8529b30214dbf65628f43a1445",
    "wallet-broker/src/runtime.rs": "da1d1484a0e89bf824ba4548e5abf4319178c36a787dc3c6b9470345d834447d",
    "test/walletAccountManagement.node.js": "94a40ddbedea2298f707bb942218255ab6f55e7db9903c9401d18190f57eaaea",
    "test/walletStartup.node.js": "4a64be8628ff54e74f282703abfcaf286c9ae9cec96a15e1d13561a4fb0d501e",
    "test/walletBrokerBuild.node.js": "b1ac8ae3a1ca76c9580c92fec7cd1cbca0edb482246f072afcafb22a79e689de",
    "test/walletSupervisorShutdown.node.js": "f1156d8bf81a03a40ec9eb235666275ce85a3394b12f7b567bbb9da20a7543ef",
    "test/walletSupervisorTransport.node.js": "e69fdcfa4aaeeca03b4abf8ac100888ff0b42fbb6001f53e461a35a5b7550700",
    "test/walletSupervisor.node.js": "eaaa63777c8ded89ea6e9b2a07912bd3a680953bdb989a4ff0d6894e5df6ce5c",
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
    "test/securityPolicy.node.js": "a58d85e5322f0cb01a78b636454f92547bd98e556013cc07712dc0cf64ea565c"
  },
  "stop": "unexpected failure walletSupervisor",
  "final_hashes": {
    "social-main.js": "29fa41d88cb75d96b00bc2fe740d8701c9ddfbaf4ba2ae71be84b8755e192d30",
    "wallet-broker/supervisor.js": "1258444add1741dd6ba850c241871b8e68be71e8550dac14b0e8af6959b9f7a2",
    "scripts/build-wallet-broker.js": "4427ef3e713ec77c45daefe8d37b457ca8dc2c8529b30214dbf65628f43a1445",
    "wallet-broker/src/runtime.rs": "da1d1484a0e89bf824ba4548e5abf4319178c36a787dc3c6b9470345d834447d",
    "test/walletAccountManagement.node.js": "94a40ddbedea2298f707bb942218255ab6f55e7db9903c9401d18190f57eaaea",
    "test/walletStartup.node.js": "4a64be8628ff54e74f282703abfcaf286c9ae9cec96a15e1d13561a4fb0d501e",
    "test/walletBrokerBuild.node.js": "b1ac8ae3a1ca76c9580c92fec7cd1cbca0edb482246f072afcafb22a79e689de",
    "test/walletSupervisorShutdown.node.js": "f1156d8bf81a03a40ec9eb235666275ce85a3394b12f7b567bbb9da20a7543ef",
    "test/walletSupervisorTransport.node.js": "e69fdcfa4aaeeca03b4abf8ac100888ff0b42fbb6001f53e461a35a5b7550700",
    "test/walletSupervisor.node.js": "eaaa63777c8ded89ea6e9b2a07912bd3a680953bdb989a4ff0d6894e5df6ce5c",
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
    "test/securityPolicy.node.js": "a58d85e5322f0cb01a78b636454f92547bd98e556013cc07712dc0cf64ea565c"
  }
}
```
