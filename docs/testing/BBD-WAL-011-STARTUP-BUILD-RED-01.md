# WAL-011 build command expected red 01

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
      "output": "51dd39d97a3b6dfb2467b4ee9c99c75b3718f2b1\n"
    },
    {
      "argv": [
        "git",
        "status",
        "--short"
      ],
      "exit": 0,
      "output": " M package-lock.json\n M package.json\n M scripts/security-policy.js\n M test/electronSecurity.node.js\n M test/securityPolicy.node.js\n?? docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md\n?? docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md\n?? docs/testing/BBD-WAL-011-STARTUP-MAIN-RED-01.md\n?? test/walletBrokerBuild.node.js\n?? test/walletStartup.node.js\n?? test/walletStartupSmoke.node.js\n"
    },
    {
      "argv": [
        "node",
        "test/walletBrokerBuild.node.js"
      ],
      "exit": 1,
      "output": "not ok module: exports synchronous stageWalletBroker\nAssertionError [ERR_ASSERTION]: wallet broker build helper is missing\n    at loadStager (<repo>/test/walletBrokerBuild.node.js:36:10)\n    at <repo>/test/walletBrokerBuild.node.js:364:17\n    at run (<repo>/test/walletBrokerBuild.node.js:604:7)\n    at Object.<anonymous> (<repo>/test/walletBrokerBuild.node.js:618:30)\n    at Module._compile (node:internal/modules/cjs/loader:1781:14)\n    at Object..js (node:internal/modules/cjs/loader:1913:10)\n    at Module.load (node:internal/modules/cjs/loader:1505:32)\n    at Function._load (node:internal/modules/cjs/loader:1309:12)\n    at wrapModuleLoad (node:internal/modules/cjs/loader:254:19)\n    at Function.executeUserEntryPoint [as runMain] (node:internal/modules/run_main:171:5)\nnot ok stage: copied bytes pin independently, modes match identity, resolver accepts layout\nAssertionError [ERR_ASSERTION]: wallet broker build helper is missing\n    at loadStager (<repo>/test/walletBrokerBuild.node.js:36:10)\n    at <repo>/test/walletBrokerBuild.node.js:369:17\n    at run (<repo>/test/walletBrokerBuild.node.js:604:7)\n    at Object.<anonymous> (<repo>/test/walletBrokerBuild.node.js:618:30)\n    at Module._compile (node:internal/modules/cjs/loader:1781:14)\n    at Object..js (node:internal/modules/cjs/loader:1913:10)\n    at Module.load (node:internal/modules/cjs/loader:1505:32)\n    at Function._load (node:internal/modules/cjs/loader:1309:12)\n    at wrapModuleLoad (node:internal/modules/cjs/loader:254:19)\n    at Function.executeUserEntryPoint [as runMain] (node:internal/modules/run_main:171:5)\nnot ok stage: repeat replacement preserves unrelated files and retains staging leftovers\nAssertionError [ERR_ASSERTION]: wallet broker build helper is missing\n    at loadStager (<repo>/test/walletBrokerBuild.node.js:36:10)\n    at <repo>/test/walletBrokerBuild.node.js:392:17\n    at run (<repo>/test/walletBrokerBuild.node.js:604:7)\n    at Object.<anonymous> (<repo>/test/walletBrokerBuild.node.js:618:30)\n    at Module._compile (node:internal/modules/cjs/loader:1781:14)\n    at Object..js (node:internal/modules/cjs/loader:1913:10)\n    at Module.load (node:internal/modules/cjs/loader:1505:32)\n    at Function._load (node:internal/modules/cjs/loader:1309:12)\n    at wrapModuleLoad (node:internal/modules/cjs/loader:254:19)\n    at Function.executeUserEntryPoint [as runMain] (node:internal/modules/run_main:171:5)\nnot ok preflight: invalid source, roots, and identities leave destination bytes unchanged\nAssertionError [ERR_ASSERTION]: wallet broker build helper is missing\n    at loadStager (<repo>/test/walletBrokerBuild.node.js:36:10)\n    at <repo>/test/walletBrokerBuild.node.js:417:17\n    at run (<repo>/test/walletBrokerBuild.node.js:604:7)\n    at Object.<anonymous> (<repo>/test/walletBrokerBuild.node.js:618:30)\n    at Module._compile (node:internal/modules/cjs/loader:1781:14)\n    at Object..js (node:internal/modules/cjs/loader:1913:10)\n    at Module.load (node:internal/modules/cjs/loader:1505:32)\n    at Function._load (node:internal/modules/cjs/loader:1309:12)\n    at wrapModuleLoad (node:internal/modules/cjs/loader:254:19)\n    at Function.executeUserEntryPoint [as runMain] (node:internal/modules/run_main:171:5)\nnot ok preflight: destination directory and pin links leave bytes unchanged\nAssertionError [ERR_ASSERTION]: wallet broker build helper is missing\n    at loadStager (<repo>/test/walletBrokerBuild.node.js:36:10)\n    at <repo>/test/walletBrokerBuild.node.js:459:17\n    at run (<repo>/test/walletBrokerBuild.node.js:604:7)\n    at Object.<anonymous> (<repo>/test/walletBrokerBuild.node.js:618:30)\n    at Module._compile (node:internal/modules/cjs/loader:1781:14)\n    at Object..js (node:internal/modules/cjs/loader:1913:10)\n    at Module.load (node:internal/modules/cjs/loader:1505:32)\n    at Function._load (node:internal/modules/cjs/loader:1309:12)\n    at wrapModuleLoad (node:internal/modules/cjs/loader:254:19)\n    at Function.executeUserEntryPoint [as runMain] (node:internal/modules/run_main:171:5)\nnot ok cli: vm-mocked cargo command is exact and nonzero skips staging\nAssertionError [ERR_ASSERTION]: wallet broker build helper is missing\n    at runCli (<repo>/test/walletBrokerBuild.node.js:276:10)\n    at <repo>/test/walletBrokerBuild.node.js:555:21\n    at withOwnedTemp (<repo>/test/walletBrokerBuild.node.js:99:12)\n    at <repo>/test/walletBrokerBuild.node.js:544:3\n    at run (<repo>/test/walletBrokerBuild.node.js:604:7)\n    at Object.<anonymous> (<repo>/test/walletBrokerBuild.node.js:618:30)\n    at Module._compile (node:internal/modules/cjs/loader:1781:14)\n    at Object..js (node:internal/modules/cjs/loader:1913:10)\n    at Module.load (node:internal/modules/cjs/loader:1505:32)\n    at Function._load (node:internal/modules/cjs/loader:1309:12)\n6 wallet broker build test(s) failed\n"
    }
  ],
  "accepted": true,
  "version": "Hermes Agent v0.18.2 (2026.7.7.2) \u00b7 upstream 67764dc0 \u00b7 local 10b6d1a9 (+1 carried commit)\nInstall directory: <home>/.hermes/hermes-agent\nInstall method: git\nPython: 3.11.15\nOpenAI SDK: 2.24.0",
  "head": "51dd39d97a3b6dfb2467b4ee9c99c75b3718f2b1",
  "initial_status": "M package-lock.json\n M package.json\n M scripts/security-policy.js\n M test/electronSecurity.node.js\n M test/securityPolicy.node.js\n?? docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md\n?? docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md\n?? docs/testing/BBD-WAL-011-STARTUP-MAIN-RED-01.md\n?? test/walletBrokerBuild.node.js\n?? test/walletStartup.node.js\n?? test/walletStartupSmoke.node.js",
  "session": {
    "id": "20260910_085135_5a8edd",
    "model": "poolside/laguna-s-2.1:free",
    "provider": "nous"
  },
  "input_hashes": {
    "test/walletBrokerBuild.node.js": "614e3ad8dde731c344d23353076672e0ba046a3e9d4919e303b5601350b97035",
    "wallet-broker/launch-config.js": "68adf92e2d543e5d51353477b1944c35d4189c5590ac581d3289f1bd72898af8",
    "wallet-pay/model.js": "acf07238366f3e28253be8c9208fbb27e13e9c2687d374d1a6ae87e0d173fa5e",
    "test/walletStartupSmoke.node.js": "35d87dc7c3f8f9f919277e5c16cc66e22fdc7228718d62ef455652b32dc3f09f",
    "wallet-broker/Cargo.toml": "73e5e585eb2fd1ca867d66962460cfa010443886a4b59d8a33556ef2a4ff3503",
    "wallet-broker/Cargo.lock": "b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71"
  },
  "counts": {
    "ok": 0,
    "not_ok": 6
  },
  "final_hashes": {
    "test/walletBrokerBuild.node.js": "614e3ad8dde731c344d23353076672e0ba046a3e9d4919e303b5601350b97035",
    "wallet-broker/launch-config.js": "68adf92e2d543e5d51353477b1944c35d4189c5590ac581d3289f1bd72898af8",
    "wallet-pay/model.js": "acf07238366f3e28253be8c9208fbb27e13e9c2687d374d1a6ae87e0d173fa5e",
    "test/walletStartupSmoke.node.js": "35d87dc7c3f8f9f919277e5c16cc66e22fdc7228718d62ef455652b32dc3f09f",
    "wallet-broker/Cargo.toml": "73e5e585eb2fd1ca867d66962460cfa010443886a4b59d8a33556ef2a4ff3503",
    "wallet-broker/Cargo.lock": "b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71"
  }
}
```
