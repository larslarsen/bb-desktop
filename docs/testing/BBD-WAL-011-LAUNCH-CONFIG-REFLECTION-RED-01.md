# WAL-011 launch configuration reflection expected-red evidence 01

One driver invocation; actual results below. expected_red=true means the reviewed failure was observed, not a green suite. Production was unchanged; only four reflection regression groups were executed. No integration or validation replay is authorized.

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
      "output": "1e69bba5ab67c99c8c28e540e4c121872ca9191e\n"
    },
    {
      "argv": [
        "git",
        "status",
        "--short"
      ],
      "exit": 0,
      "output": " M package-lock.json\n M package.json\n M scripts/security-policy.js\n M test/securityPolicy.node.js\n?? docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md\n?? docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md\n?? docs/testing/BBD-WAL-011-LAUNCH-CONFIG-RED-01.md\n?? test/walletBrokerLaunchConfig.node.js\n?? wallet-broker/launch-config.js\n"
    },
    {
      "argv": [
        "node",
        "-e",
        "const assert=require('assert'); const names=['revoked proxy','getPrototypeOf trap','ownKeys trap','getOwnPropertyDescriptor trap'].map(x=>'reflection errors: '+x+' is sanitized'); const tests=require('./test/walletBrokerLaunchConfig.node.js').tests.filter(t=>names.includes(t.name)); assert.deepStrictEqual(tests.map(t=>t.name),names); let failed=0; for(const {name,fn} of tests){try{fn(); console.log('ok '+name);}catch(e){failed++; console.error('not ok '+name+'\\n'+e.stack);}} process.exitCode=failed?1:0;"
      ],
      "exit": 1,
      "output": "not ok reflection errors: revoked proxy is sanitized\nAssertionError [ERR_ASSERTION]: revoked proxy: code\n+ actual - expected\n\n+ undefined\n- 'UNAVAILABLE'\n\n    at assertUnavailable (<repo>/test/walletBrokerLaunchConfig.node.js:171:10)\n    at <repo>/test/walletBrokerLaunchConfig.node.js:852:3\n    at [eval]:1:385\n    at runScriptInThisContext (node:internal/vm:209:10)\n    at node:internal/process/execution:446:12\n    at [eval]-wrapper:6:24\n    at runScriptInContext (node:internal/process/execution:444:60)\n    at evalFunction (node:internal/process/execution:279:30)\n    at evalTypeScript (node:internal/process/execution:291:3)\n    at node:internal/main/eval_string:74:3\nnot ok reflection errors: getPrototypeOf trap is sanitized\nAssertionError [ERR_ASSERTION]: getPrototypeOf trap: code\n+ actual - expected\n\n+ undefined\n- 'UNAVAILABLE'\n\n    at assertUnavailable (<repo>/test/walletBrokerLaunchConfig.node.js:171:10)\n    at <repo>/test/walletBrokerLaunchConfig.node.js:877:3\n    at [eval]:1:385\n    at runScriptInThisContext (node:internal/vm:209:10)\n    at node:internal/process/execution:446:12\n    at [eval]-wrapper:6:24\n    at runScriptInContext (node:internal/process/execution:444:60)\n    at evalFunction (node:internal/process/execution:279:30)\n    at evalTypeScript (node:internal/process/execution:291:3)\n    at node:internal/main/eval_string:74:3\nnot ok reflection errors: ownKeys trap is sanitized\nAssertionError [ERR_ASSERTION]: ownKeys trap: code\n+ actual - expected\n\n+ undefined\n- 'UNAVAILABLE'\n\n    at assertUnavailable (<repo>/test/walletBrokerLaunchConfig.node.js:171:10)\n    at <repo>/test/walletBrokerLaunchConfig.node.js:903:3\n    at [eval]:1:385\n    at runScriptInThisContext (node:internal/vm:209:10)\n    at node:internal/process/execution:446:12\n    at [eval]-wrapper:6:24\n    at runScriptInContext (node:internal/process/execution:444:60)\n    at evalFunction (node:internal/process/execution:279:30)\n    at evalTypeScript (node:internal/process/execution:291:3)\n    at node:internal/main/eval_string:74:3\nnot ok reflection errors: getOwnPropertyDescriptor trap is sanitized\nAssertionError [ERR_ASSERTION]: getOwnPropertyDescriptor trap: code\n+ actual - expected\n\n+ undefined\n- 'UNAVAILABLE'\n\n    at assertUnavailable (<repo>/test/walletBrokerLaunchConfig.node.js:171:10)\n    at <repo>/test/walletBrokerLaunchConfig.node.js:929:3\n    at [eval]:1:385\n    at runScriptInThisContext (node:internal/vm:209:10)\n    at node:internal/process/execution:446:12\n    at [eval]-wrapper:6:24\n    at runScriptInContext (node:internal/process/execution:444:60)\n    at evalFunction (node:internal/process/execution:279:30)\n    at evalTypeScript (node:internal/process/execution:291:3)\n    at node:internal/main/eval_string:74:3\n"
    }
  ],
  "expected_red": true,
  "version": "Hermes Agent v0.18.2 (2026.7.7.2) \u00b7 upstream 8e85b276 \u00b7 local 10b6d1a9 (+1 carried commit)\nInstall directory: <home>/.hermes/hermes-agent\nInstall method: git\nPython: 3.11.15\nOpenAI SDK: 2.24.0",
  "head": "1e69bba5ab67c99c8c28e540e4c121872ca9191e",
  "initial_status": "M package-lock.json\n M package.json\n M scripts/security-policy.js\n M test/securityPolicy.node.js\n?? docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md\n?? docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md\n?? docs/testing/BBD-WAL-011-LAUNCH-CONFIG-RED-01.md\n?? test/walletBrokerLaunchConfig.node.js\n?? wallet-broker/launch-config.js",
  "session": {
    "id": "20260909_225544_2c118d",
    "model": "poolside/laguna-s-2.1:free",
    "provider": "nous"
  },
  "input_hashes": {
    "wallet-broker/launch-config.js": "4a639b1644ff0ea618fc53fd525cbd5c0fde84fbea5905f3f3eb56a5520dc01a",
    "test/walletBrokerLaunchConfig.node.js": "90acd32c9863df0206364eb04f707d92a5979ee66c7e15b2ec7b8250a5fc2113",
    "social-main.js": "c7687b52ee45b17c3063cc2b39071be3d4e613a9d1e86558f2a7bbc1567db0a3",
    "wallet-broker/supervisor.js": "c1410bfc5cd4c205ad014a42154c9d9d9714794b134a5acab7ed499934cad51a",
    "wallet-broker/protocol.js": "79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4",
    "test/electronSecurity.node.js": "7f759f810762ccdfa5e29d69dc3bf2fa1b1502d2c416ca2b324442b2544ad18c"
  },
  "final_hashes": {
    "wallet-broker/launch-config.js": "4a639b1644ff0ea618fc53fd525cbd5c0fde84fbea5905f3f3eb56a5520dc01a",
    "test/walletBrokerLaunchConfig.node.js": "90acd32c9863df0206364eb04f707d92a5979ee66c7e15b2ec7b8250a5fc2113",
    "social-main.js": "c7687b52ee45b17c3063cc2b39071be3d4e613a9d1e86558f2a7bbc1567db0a3",
    "wallet-broker/supervisor.js": "c1410bfc5cd4c205ad014a42154c9d9d9714794b134a5acab7ed499934cad51a",
    "wallet-broker/protocol.js": "79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4",
    "test/electronSecurity.node.js": "7f759f810762ccdfa5e29d69dc3bf2fa1b1502d2c416ca2b324442b2544ad18c"
  },
  "counts": {
    "ok": 0,
    "not_ok": 4
  }
}
```
