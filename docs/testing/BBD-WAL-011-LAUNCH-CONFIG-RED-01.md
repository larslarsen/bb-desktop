# WAL-011 launch configuration expected-red evidence 01

One driver invocation; actual results below. expected_red=true means the reviewed failure was observed, not a green suite. Production was not changed; all 58 fixture case bodies remain unexecuted on the absent-resolver baseline. No integration or validation replay is authorized.

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
      "output": "118e6120f5d17f5702d2636cc772bf3663947553\n"
    },
    {
      "argv": [
        "git",
        "status",
        "--short"
      ],
      "exit": 0,
      "output": " M package-lock.json\n M package.json\n M scripts/security-policy.js\n M test/securityPolicy.node.js\n?? docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md\n?? docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md\n?? test/walletBrokerLaunchConfig.node.js\n"
    },
    {
      "argv": [
        "node",
        "test/walletBrokerLaunchConfig.node.js"
      ],
      "exit": 1,
      "output": "not ok valid configuration: six platform identities return frozen supervisor options\nAssertionError [ERR_ASSERTION]: packaged broker launch resolver is missing\n    at loadResolver (<repo>/test/walletBrokerLaunchConfig.node.js:36:10)\n    at <repo>/test/walletBrokerLaunchConfig.node.js:223:19\n    at run (<repo>/test/walletBrokerLaunchConfig.node.js:843:11)\n    at Object.<anonymous> (<repo>/test/walletBrokerLaunchConfig.node.js:850:30)\n    at Module._compile (node:internal/modules/cjs/loader:1781:14)\n    at Object..js (node:internal/modules/cjs/loader:1913:10)\n    at Module.load (node:internal/modules/cjs/loader:1505:32)\n    at Function._load (node:internal/modules/cjs/loader:1309:12)\n    at wrapModuleLoad (node:internal/modules/cjs/loader:254:19)\n    at Function.executeUserEntryPoint [as runMain] (node:internal/modules/run_main:171:5)\nnot ok invalid options: reject missing extra accessor prototype and path identities\nAssertionError [ERR_ASSERTION]: packaged broker launch resolver is missing\n    at loadResolver (<repo>/test/walletBrokerLaunchConfig.node.js:36:10)\n    at <repo>/test/walletBrokerLaunchConfig.node.js:263:19\n    at run (<repo>/test/walletBrokerLaunchConfig.node.js:843:11)\n    at Object.<anonymous> (<repo>/test/walletBrokerLaunchConfig.node.js:850:30)\n    at Module._compile (node:internal/modules/cjs/loader:1781:14)\n    at Object..js (node:internal/modules/cjs/loader:1913:10)\n    at Module.load (node:internal/modules/cjs/loader:1505:32)\n    at Function._load (node:internal/modules/cjs/loader:1309:12)\n    at wrapModuleLoad (node:internal/modules/cjs/loader:254:19)\n    at Function.executeUserEntryPoint [as runMain] (node:internal/modules/run_main:171:5)\nnot ok invalid manifest: reject malformed extra mismatched and hostile digest rows\nAssertionError [ERR_ASSERTION]: packaged broker launch resolver is missing\n    at loadResolver (<repo>/test/walletBrokerLaunchConfig.node.js:36:10)\n    at <repo>/test/walletBrokerLaunchConfig.node.js:475:19\n    at run (<repo>/test/walletBrokerLaunchConfig.node.js:843:11)\n    at Object.<anonymous> (<repo>/test/walletBrokerLaunchConfig.node.js:850:30)\n    at Module._compile (node:internal/modules/cjs/loader:1781:14)\n    at Object..js (node:internal/modules/cjs/loader:1913:10)\n    at Module.load (node:internal/modules/cjs/loader:1505:32)\n    at Function._load (node:internal/modules/cjs/loader:1309:12)\n    at wrapModuleLoad (node:internal/modules/cjs/loader:254:19)\n    at Function.executeUserEntryPoint [as runMain] (node:internal/modules/run_main:171:5)\nnot ok manifest size: 4096 bytes succeed; 4097 and empty reject\nAssertionError [ERR_ASSERTION]: packaged broker launch resolver is missing\n    at loadResolver (<repo>/test/walletBrokerLaunchConfig.node.js:36:10)\n    at <repo>/test/walletBrokerLaunchConfig.node.js:575:19\n    at run (<repo>/test/walletBrokerLaunchConfig.node.js:843:11)\n    at Object.<anonymous> (<repo>/test/walletBrokerLaunchConfig.node.js:850:30)\n    at Module._compile (node:internal/modules/cjs/loader:1781:14)\n    at Object..js (node:internal/modules/cjs/loader:1913:10)\n    at Module.load (node:internal/modules/cjs/loader:1505:32)\n    at Function._load (node:internal/modules/cjs/loader:1309:12)\n    at wrapModuleLoad (node:internal/modules/cjs/loader:254:19)\n    at Function.executeUserEntryPoint [as runMain] (node:internal/modules/run_main:171:5)\nnot ok inventory: missing symlink and directory entries fail closed\nAssertionError [ERR_ASSERTION]: packaged broker launch resolver is missing\n    at loadResolver (<repo>/test/walletBrokerLaunchConfig.node.js:36:10)\n    at <repo>/test/walletBrokerLaunchConfig.node.js:647:19\n    at run (<repo>/test/walletBrokerLaunchConfig.node.js:843:11)\n    at Object.<anonymous> (<repo>/test/walletBrokerLaunchConfig.node.js:850:30)\n    at Module._compile (node:internal/modules/cjs/loader:1781:14)\n    at Object..js (node:internal/modules/cjs/loader:1913:10)\n    at Module.load (node:internal/modules/cjs/loader:1505:32)\n    at Function._load (node:internal/modules/cjs/loader:1309:12)\n    at wrapModuleLoad (node:internal/modules/cjs/loader:254:19)\n    at Function.executeUserEntryPoint [as runMain] (node:internal/modules/run_main:171:5)\n"
    }
  ],
  "expected_red": true,
  "version": "Hermes Agent v0.18.2 (2026.7.7.2) \u00b7 upstream 8e85b276 \u00b7 local 10b6d1a9 (+1 carried commit)\nInstall directory: <home>/.hermes/hermes-agent\nInstall method: git\nPython: 3.11.15\nOpenAI SDK: 2.24.0",
  "head": "118e6120f5d17f5702d2636cc772bf3663947553",
  "initial_status": "M package-lock.json\n M package.json\n M scripts/security-policy.js\n M test/securityPolicy.node.js\n?? docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md\n?? docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md\n?? test/walletBrokerLaunchConfig.node.js",
  "session": {
    "id": "20260909_221756_f0f9b6",
    "model": "poolside/laguna-s-2.1:free",
    "provider": "nous"
  },
  "input_hashes": {
    "test/walletBrokerLaunchConfig.node.js": "1b6a7d8c102360efdbd0920b6211e7b55999fbdfff18aff4eee23cd311d750f8",
    "social-main.js": "c7687b52ee45b17c3063cc2b39071be3d4e613a9d1e86558f2a7bbc1567db0a3",
    "wallet-broker/supervisor.js": "c1410bfc5cd4c205ad014a42154c9d9d9714794b134a5acab7ed499934cad51a",
    "wallet-broker/protocol.js": "79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4",
    "test/electronSecurity.node.js": "7f759f810762ccdfa5e29d69dc3bf2fa1b1502d2c416ca2b324442b2544ad18c"
  },
  "production_absent_before": true,
  "production_absent_after": true,
  "final_hashes": {
    "test/walletBrokerLaunchConfig.node.js": "1b6a7d8c102360efdbd0920b6211e7b55999fbdfff18aff4eee23cd311d750f8",
    "social-main.js": "c7687b52ee45b17c3063cc2b39071be3d4e613a9d1e86558f2a7bbc1567db0a3",
    "wallet-broker/supervisor.js": "c1410bfc5cd4c205ad014a42154c9d9d9714794b134a5acab7ed499934cad51a",
    "wallet-broker/protocol.js": "79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4",
    "test/electronSecurity.node.js": "7f759f810762ccdfa5e29d69dc3bf2fa1b1502d2c416ca2b324442b2544ad18c"
  },
  "counts": {
    "ok": 0,
    "not_ok": 5
  }
}
```
