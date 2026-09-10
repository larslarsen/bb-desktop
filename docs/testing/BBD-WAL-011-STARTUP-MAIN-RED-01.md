# WAL-011 main startup expected red 01

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
      "output": "4999e3b3c1f629f5426f508b0202fe61ec31d20a\n"
    },
    {
      "argv": [
        "git",
        "status",
        "--short"
      ],
      "exit": 0,
      "output": " M package-lock.json\n M package.json\n M scripts/security-policy.js\n M test/electronSecurity.node.js\n M test/securityPolicy.node.js\n?? docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md\n?? docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md\n?? test/walletBrokerBuild.node.js\n?? test/walletStartup.node.js\n?? test/walletStartupSmoke.node.js\n"
    },
    {
      "argv": [
        "node",
        "test/walletStartup.node.js"
      ],
      "exit": 1,
      "output": "not ok ready-only packaged and development paths resolve once with subscribe-before-start\nAssertionError [ERR_ASSERTION]: Expected values to be strictly equal:\n\n0 !== 1\n\n    at <repo>/test/walletStartup.node.js:400:12\n    at withMain (<repo>/test/walletStartup.node.js:377:26)\n    at <repo>/test/walletStartup.node.js:391:9\n    at run (<repo>/test/walletStartup.node.js:609:13)\n    at Object.<anonymous> (<repo>/test/walletStartup.node.js:624:3)\n    at Module._compile (node:internal/modules/cjs/loader:1781:14)\n    at Object..js (node:internal/modules/cjs/loader:1913:10)\n    at Module.load (node:internal/modules/cjs/loader:1505:32)\n    at Function.<anonymous> (node:internal/modules/cjs/loader:1309:12)\n    at wrapModuleLoad (node:internal/modules/cjs/loader:254:19)\nnot ok unbound snapshot get returns cached down clones then live status after handshake\nAssertionError [ERR_ASSERTION]: configured supervisor missing\n    at <repo>/test/walletStartup.node.js:430:12\n    at withMain (<repo>/test/walletStartup.node.js:377:26)\n    at <repo>/test/walletStartup.node.js:428:9\n    at run (<repo>/test/walletStartup.node.js:609:13)\nnot ok resolver or start failure leaves social usable; fallback still denies sender and payload\nAssertionError [ERR_ASSERTION]: Expected values to be strictly deep-equal:\n+ actual - expected\n\n  {\n    accounts: [],\n+   broker: 'ready',\n-   broker: 'down',\n    intent_preview: null,\n    v: 1\n  }\n\n    at <repo>/test/walletStartup.node.js:484:12\n    at async withMain (<repo>/test/walletStartup.node.js:377:20)\n    at async <repo>/test/walletStartup.node.js:479:3\n    at async run (<repo>/test/walletStartup.node.js:609:7)\nnot ok snapshot cache updates while the window is closed and clones stay isolated\nTypeError: Cannot read properties of undefined (reading 'subscriber')\n    at <repo>/test/walletStartup.node.js:528:16\n    at withMain (<repo>/test/walletStartup.node.js:377:26)\n    at <repo>/test/walletStartup.node.js:523:9\n    at run (<repo>/test/walletStartup.node.js:609:13)\nnot ok pre-ready quit and reentrant quit during resolver never spawn; activate stays idle-gated\nAssertionError [ERR_ASSERTION]: ready after quit created a window\n\n1 !== 0\n\n    at <repo>/test/walletStartup.node.js:552:12\n    at async withMain (<repo>/test/walletStartup.node.js:377:20)\n    at async <repo>/test/walletStartup.node.js:547:3\n    at async run (<repo>/test/walletStartup.node.js:609:7)\nnot ok configured supervisor shutdown is awaited on the replaced instance\nTypeError: Cannot read properties of undefined (reading 'kind')\n    at <repo>/test/walletStartup.node.js:581:53\n    at withMain (<repo>/test/walletStartup.node.js:377:26)\n    at <repo>/test/walletStartup.node.js:575:9\n    at run (<repo>/test/walletStartup.node.js:609:13)\n6 wallet startup test(s) failed\n"
    }
  ],
  "accepted": true,
  "version": "Hermes Agent v0.18.2 (2026.7.7.2) \u00b7 upstream 67764dc0 \u00b7 local 10b6d1a9 (+1 carried commit)\nInstall directory: <home>/.hermes/hermes-agent\nInstall method: git\nPython: 3.11.15\nOpenAI SDK: 2.24.0",
  "head": "4999e3b3c1f629f5426f508b0202fe61ec31d20a",
  "initial_status": "M package-lock.json\n M package.json\n M scripts/security-policy.js\n M test/electronSecurity.node.js\n M test/securityPolicy.node.js\n?? docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md\n?? docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md\n?? test/walletBrokerBuild.node.js\n?? test/walletStartup.node.js\n?? test/walletStartupSmoke.node.js",
  "session": {
    "id": "20260910_084936_e50477",
    "model": "poolside/laguna-s-2.1:free",
    "provider": "nous"
  },
  "input_hashes": {
    "social-main.js": "c7687b52ee45b17c3063cc2b39071be3d4e613a9d1e86558f2a7bbc1567db0a3",
    "test/walletStartup.node.js": "bf72c9f276938b8b2e538bbb7631303837d6f9da998f3a4c279d397a02f1bae3",
    "test/electronSecurity.node.js": "df81caae58607184a0c407f5aa845aa90a2f729c7fa8a9ac0ec9edbc366e8eea",
    "wallet-broker/supervisor.js": "c1410bfc5cd4c205ad014a42154c9d9d9714794b134a5acab7ed499934cad51a",
    "wallet-broker/launch-config.js": "68adf92e2d543e5d51353477b1944c35d4189c5590ac581d3289f1bd72898af8",
    "wallet-pay/model.js": "acf07238366f3e28253be8c9208fbb27e13e9c2687d374d1a6ae87e0d173fa5e",
    "package.json": "76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5",
    "package-lock.json": "0e32de43f3c1a31ae3d9e9c8b172d10cd37a03577dc7d1eaf4ac916967cfcdb8",
    "scripts/security-policy.js": "fec10759a73a3c675d8206da454abf4b0e8dca6b62da463f39a2db27b30dc078",
    "test/securityPolicy.node.js": "a58d85e5322f0cb01a78b636454f92547bd98e556013cc07712dc0cf64ea565c"
  },
  "counts": {
    "ok": 0,
    "not_ok": 6
  },
  "final_hashes": {
    "social-main.js": "c7687b52ee45b17c3063cc2b39071be3d4e613a9d1e86558f2a7bbc1567db0a3",
    "test/walletStartup.node.js": "bf72c9f276938b8b2e538bbb7631303837d6f9da998f3a4c279d397a02f1bae3",
    "test/electronSecurity.node.js": "df81caae58607184a0c407f5aa845aa90a2f729c7fa8a9ac0ec9edbc366e8eea",
    "wallet-broker/supervisor.js": "c1410bfc5cd4c205ad014a42154c9d9d9714794b134a5acab7ed499934cad51a",
    "wallet-broker/launch-config.js": "68adf92e2d543e5d51353477b1944c35d4189c5590ac581d3289f1bd72898af8",
    "wallet-pay/model.js": "acf07238366f3e28253be8c9208fbb27e13e9c2687d374d1a6ae87e0d173fa5e",
    "package.json": "76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5",
    "package-lock.json": "0e32de43f3c1a31ae3d9e9c8b172d10cd37a03577dc7d1eaf4ac916967cfcdb8",
    "scripts/security-policy.js": "fec10759a73a3c675d8206da454abf4b0e8dca6b62da463f39a2db27b30dc078",
    "test/securityPolicy.node.js": "a58d85e5322f0cb01a78b636454f92547bd98e556013cc07712dc0cf64ea565c"
  }
}
```
