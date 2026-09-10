# WAL-011 shutdown expected-red evidence 01

One driver invocation; actual results below. expected_red=true means the reviewed absent-API failure was observed, not suite green or real-child cleanup acceptance. No production change or integration is authorized.

```json
{
  "commands": [
    {
      "argv": [
        "hermes",
        "--version"
      ],
      "exit": 0,
      "output": "Hermes Agent v0.18.2 (2026.7.7.2) \u00b7 upstream 8e85b276 \u00b7 local 10b6d1a9 (+1 carried commit)\nInstall directory: <home>/.hermes/hermes-agent\nInstall method: git\nPython: 3.11.15\nOpenAI SDK: 2.24.0\n"
    },
    {
      "argv": [
        "git",
        "rev-parse",
        "HEAD"
      ],
      "exit": 0,
      "output": "82598823b72fe4a6247669ba1ea961646bc7a1e4\n"
    },
    {
      "argv": [
        "git",
        "status",
        "--short"
      ],
      "exit": 0,
      "output": " M package-lock.json\n M package.json\n M scripts/security-policy.js\n M test/securityPolicy.node.js\n?? docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md\n?? docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md\n?? test/fixtures/wallet-broker/shutdown-child.js\n?? test/walletSupervisorShutdown.node.js\n"
    },
    {
      "argv": [
        "node",
        "test/walletSupervisorShutdown.node.js"
      ],
      "exit": 1,
      "output": "not ok shutdown: no-start and failed-start resolve one Promise without spawn, kill, or timers\nAssertionError [ERR_ASSERTION]: absent shutdown API\n+ actual - expected\n\n+ 'undefined'\n- 'function'\n\n    at assertShutdownApi (<repo>/test/walletSupervisorShutdown.node.js:129:10)\n    at <repo>/test/walletSupervisorShutdown.node.js:591:5\n    at withFakeHarness (<repo>/test/walletSupervisorShutdown.node.js:322:20)\n    at <repo>/test/walletSupervisorShutdown.node.js:590:9\n    at run (<repo>/test/walletSupervisorShutdown.node.js:972:13)\n    at Object.<anonymous> (<repo>/test/walletSupervisorShutdown.node.js:982:30)\n    at Module._compile (node:internal/modules/cjs/loader:1781:14)\n    at Object..js (node:internal/modules/cjs/loader:1913:10)\n    at Module.load (node:internal/modules/cjs/loader:1505:32)\n    at Function._load (node:internal/modules/cjs/loader:1309:12)\nnot ok shutdown: bound cancel precedes SIGTERM and close, not exit, completes one Promise\nAssertionError [ERR_ASSERTION]: absent shutdown API\n+ actual - expected\n\n+ 'undefined'\n- 'function'\n\n    at assertShutdownApi (<repo>/test/walletSupervisorShutdown.node.js:129:10)\n    at <repo>/test/walletSupervisorShutdown.node.js:632:5\n    at withFakeHarness (<repo>/test/walletSupervisorShutdown.node.js:322:20)\n    at <repo>/test/walletSupervisorShutdown.node.js:631:9\n    at run (<repo>/test/walletSupervisorShutdown.node.js:972:13)\nnot ok shutdown: prior quit, protocol failure, and close-before-shutdown keep one termination\nAssertionError [ERR_ASSERTION]: absent shutdown API\n+ actual - expected\n\n+ 'undefined'\n- 'function'\n\n    at assertShutdownApi (<repo>/test/walletSupervisorShutdown.node.js:129:10)\n    at <repo>/test/walletSupervisorShutdown.node.js:701:5\n    at withFakeHarness (<repo>/test/walletSupervisorShutdown.node.js:322:20)\n    at <repo>/test/walletSupervisorShutdown.node.js:700:9\n    at run (<repo>/test/walletSupervisorShutdown.node.js:972:13)\nnot ok shutdown: stubborn child SIGKILL at 250 ms and TIMEOUT at 1500 ms\nAssertionError [ERR_ASSERTION]: absent shutdown API\n+ actual - expected\n\n+ 'undefined'\n- 'function'\n\n    at assertShutdownApi (<repo>/test/walletSupervisorShutdown.node.js:129:10)\n    at <repo>/test/walletSupervisorShutdown.node.js:793:5\n    at withFakeHarness (<repo>/test/walletSupervisorShutdown.node.js:322:20)\n    at <repo>/test/walletSupervisorShutdown.node.js:792:9\n    at run (<repo>/test/walletSupervisorShutdown.node.js:972:13)\nnot ok shutdown: real child normal termination observes close and closed streams\nAssertionError [ERR_ASSERTION]: absent shutdown API\n+ actual - expected\n\n+ 'undefined'\n- 'function'\n\n    at assertShutdownApi (<repo>/test/walletSupervisorShutdown.node.js:129:10)\n    at withRealChild (<repo>/test/walletSupervisorShutdown.node.js:519:5)\n    at <repo>/test/walletSupervisorShutdown.node.js:862:9\n    at run (<repo>/test/walletSupervisorShutdown.node.js:972:13)\nnot ok shutdown: real child ignoring SIGTERM terminates with SIGKILL after close\nAssertionError [ERR_ASSERTION]: absent shutdown API\n+ actual - expected\n\n+ 'undefined'\n- 'function'\n\n    at assertShutdownApi (<repo>/test/walletSupervisorShutdown.node.js:129:10)\n    at withRealChild (<repo>/test/walletSupervisorShutdown.node.js:519:5)\n    at <repo>/test/walletSupervisorShutdown.node.js:928:11\n    at run (<repo>/test/walletSupervisorShutdown.node.js:972:13)\n"
    }
  ],
  "expected_red": true,
  "version": "Hermes Agent v0.18.2 (2026.7.7.2) \u00b7 upstream 8e85b276 \u00b7 local 10b6d1a9 (+1 carried commit)\nInstall directory: <home>/.hermes/hermes-agent\nInstall method: git\nPython: 3.11.15\nOpenAI SDK: 2.24.0",
  "head": "82598823b72fe4a6247669ba1ea961646bc7a1e4",
  "initial_status": "M package-lock.json\n M package.json\n M scripts/security-policy.js\n M test/securityPolicy.node.js\n?? docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md\n?? docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md\n?? test/fixtures/wallet-broker/shutdown-child.js\n?? test/walletSupervisorShutdown.node.js",
  "session": {
    "id": "20260909_193149_7cb2b0",
    "model": "poolside/laguna-s-2.1:free",
    "provider": "nous"
  },
  "input_hashes": {
    "test/walletSupervisorShutdown.node.js": "69eb8bcbc2d169e00cb52a7bff73e1b5f39d7890d24d79383f63e77b3994b5c7",
    "test/fixtures/wallet-broker/shutdown-child.js": "0d8fbfa8179338a8fc05e6c41f4cbbf0ef652c4ccadd73db8876591a51f96dcb",
    "wallet-broker/supervisor.js": "1ac92493cb70cd4ea5e1884f4b96424c7fbc89126308299e7221f2b0ba1a8ff8",
    "wallet-broker/protocol.js": "79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4",
    "test/fixtures/wallet-broker/transport-child.js": "ba11ba8a37d4ee7a6df2e3499c40c972ca4a1cfd8ef0eed377afe1c7a1e3ba91",
    "test/walletSupervisorTransport.node.js": "e69fdcfa4aaeeca03b4abf8ac100888ff0b42fbb6001f53e461a35a5b7550700"
  },
  "final_hashes": {
    "test/walletSupervisorShutdown.node.js": "69eb8bcbc2d169e00cb52a7bff73e1b5f39d7890d24d79383f63e77b3994b5c7",
    "test/fixtures/wallet-broker/shutdown-child.js": "0d8fbfa8179338a8fc05e6c41f4cbbf0ef652c4ccadd73db8876591a51f96dcb",
    "wallet-broker/supervisor.js": "1ac92493cb70cd4ea5e1884f4b96424c7fbc89126308299e7221f2b0ba1a8ff8",
    "wallet-broker/protocol.js": "79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4",
    "test/fixtures/wallet-broker/transport-child.js": "ba11ba8a37d4ee7a6df2e3499c40c972ca4a1cfd8ef0eed377afe1c7a1e3ba91",
    "test/walletSupervisorTransport.node.js": "e69fdcfa4aaeeca03b4abf8ac100888ff0b42fbb6001f53e461a35a5b7550700"
  },
  "counts": {
    "ok": 0,
    "not_ok": 6
  }
}
```
