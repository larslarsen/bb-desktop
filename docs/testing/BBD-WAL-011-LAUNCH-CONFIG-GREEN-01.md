# WAL-011 launch configuration green evidence 01

Actual command outputs and restoration hashes. green=true requires nine resolver groups, thirteen supervisor groups, two detected falsifications, exact restoration and final nine-group green. No packaging or main-startup integration is claimed.

```json
{
  "commands": [
    {
      "argv": [
        "hermes",
        "--version"
      ],
      "exit": 0,
      "output": "Hermes Agent v0.18.2 (2026.7.7.2) \u00b7 upstream cfdbbb6e \u00b7 local 10b6d1a9 (+1 carried commit)\nInstall directory: <home>/.hermes/hermes-agent\nInstall method: git\nPython: 3.11.15\nOpenAI SDK: 2.24.0\n"
    },
    {
      "argv": [
        "git",
        "rev-parse",
        "HEAD"
      ],
      "exit": 0,
      "output": "34e622d2d6257b4ce93fac004696867858138487\n"
    },
    {
      "argv": [
        "git",
        "status",
        "--short"
      ],
      "exit": 0,
      "output": " M package-lock.json\n M package.json\n M scripts/security-policy.js\n M test/securityPolicy.node.js\n?? docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md\n?? docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md\n?? docs/testing/BBD-WAL-011-LAUNCH-CONFIG-RED-01.md\n?? docs/testing/BBD-WAL-011-LAUNCH-CONFIG-REFLECTION-RED-01.md\n?? test/walletBrokerLaunchConfig.node.js\n?? wallet-broker/launch-config.js\n"
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
        "test/walletSupervisor.node.js"
      ],
      "exit": 0,
      "output": "ok launch: private data directory and regular readable pinned binary precede one inert spawn\nok launch: missing, non-file, symlink, unreadable, and hash mismatch never spawn\nok launch: missing, symlinked, non-directory, or non-0700 data directories never verify or spawn\nok handshake: real child-first fixture transcript binds both directions within two seconds\nok handshake: PID, session, diagnostics, timeout, and early exit failures never dispatch\nok dispatch: exact supervisor methods and closed parameter schemas are enforced after binding\nok dispatch: pre-bind and oversize calls fail before broker send\nok dispatch: matching replies settle promises out of order with cloned sanitized results\nok lifecycle: exit publishes only sanitized down state and restart never buffers spend requests\nok quit: every in-flight intent is cancelled before child termination\nok quit: an unbound child terminates without any application frame\nok snapshot: supervisor exports the shared Pay sanitizer and removes every fixture canary\nok snapshot: sync publication traverses the shared sanitizer before every subscriber\nBitBook wallet supervisor tests passed (13).\n",
      "counts": {
        "ok": 13,
        "not_ok": 0
      }
    },
    {
      "argv": [
        "node",
        "-e",
        "const assert=require('assert'); const names=[\"invalid manifest: reject malformed extra mismatched and hostile digest rows\"]; const tests=require('./test/walletBrokerLaunchConfig.node.js').tests.filter(t=>names.includes(t.name)); assert.deepStrictEqual(tests.map(t=>t.name),names); let failed=0; for(const {name,fn} of tests){try{fn(); console.log('ok '+name);}catch(e){failed++; console.error('not ok '+name+'\\n'+e.stack);}} process.exitCode=failed?1:0;"
      ],
      "exit": 1,
      "output": "not ok invalid manifest: reject malformed extra mismatched and hostile digest rows\nAssertionError [ERR_ASSERTION]: platform mismatch: expected UNAVAILABLE\n    at assertUnavailable (<repo>/test/walletBrokerLaunchConfig.node.js:168:10)\n    at <repo>/test/walletBrokerLaunchConfig.node.js:559:7\n    at withOwnedTemp (<repo>/test/walletBrokerLaunchConfig.node.js:83:12)\n    at <repo>/test/walletBrokerLaunchConfig.node.js:476:3\n    at [eval]:1:330\n    at runScriptInThisContext (node:internal/vm:209:10)\n    at node:internal/process/execution:446:12\n    at [eval]-wrapper:6:24\n    at runScriptInContext (node:internal/process/execution:444:60)\n    at evalFunction (node:internal/process/execution:279:30)\n",
      "counts": {
        "ok": 0,
        "not_ok": 1
      },
      "mutant_sha256": "d570de28bb67a3e87a7b15072961e59d2ac345d1c495f8793f770ea24d1c1bf3",
      "restored_sha256": "68adf92e2d543e5d51353477b1944c35d4189c5590ac581d3289f1bd72898af8"
    },
    {
      "argv": [
        "node",
        "-e",
        "const assert=require('assert'); const names=[\"reflection errors: revoked proxy is sanitized\", \"reflection errors: getPrototypeOf trap is sanitized\", \"reflection errors: ownKeys trap is sanitized\", \"reflection errors: getOwnPropertyDescriptor trap is sanitized\"]; const tests=require('./test/walletBrokerLaunchConfig.node.js').tests.filter(t=>names.includes(t.name)); assert.deepStrictEqual(tests.map(t=>t.name),names); let failed=0; for(const {name,fn} of tests){try{fn(); console.log('ok '+name);}catch(e){failed++; console.error('not ok '+name+'\\n'+e.stack);}} process.exitCode=failed?1:0;"
      ],
      "exit": 1,
      "output": "not ok reflection errors: revoked proxy is sanitized\nAssertionError [ERR_ASSERTION]: revoked proxy: code\n+ actual - expected\n\n+ undefined\n- 'UNAVAILABLE'\n\n    at assertUnavailable (<repo>/test/walletBrokerLaunchConfig.node.js:171:10)\n    at <repo>/test/walletBrokerLaunchConfig.node.js:852:3\n    at [eval]:1:468\n    at runScriptInThisContext (node:internal/vm:209:10)\n    at node:internal/process/execution:446:12\n    at [eval]-wrapper:6:24\n    at runScriptInContext (node:internal/process/execution:444:60)\n    at evalFunction (node:internal/process/execution:279:30)\n    at evalTypeScript (node:internal/process/execution:291:3)\n    at node:internal/main/eval_string:74:3\nnot ok reflection errors: getPrototypeOf trap is sanitized\nAssertionError [ERR_ASSERTION]: getPrototypeOf trap: code\n+ actual - expected\n\n+ undefined\n- 'UNAVAILABLE'\n\n    at assertUnavailable (<repo>/test/walletBrokerLaunchConfig.node.js:171:10)\n    at <repo>/test/walletBrokerLaunchConfig.node.js:877:3\n    at [eval]:1:468\n    at runScriptInThisContext (node:internal/vm:209:10)\n    at node:internal/process/execution:446:12\n    at [eval]-wrapper:6:24\n    at runScriptInContext (node:internal/process/execution:444:60)\n    at evalFunction (node:internal/process/execution:279:30)\n    at evalTypeScript (node:internal/process/execution:291:3)\n    at node:internal/main/eval_string:74:3\nnot ok reflection errors: ownKeys trap is sanitized\nAssertionError [ERR_ASSERTION]: ownKeys trap: code\n+ actual - expected\n\n+ undefined\n- 'UNAVAILABLE'\n\n    at assertUnavailable (<repo>/test/walletBrokerLaunchConfig.node.js:171:10)\n    at <repo>/test/walletBrokerLaunchConfig.node.js:903:3\n    at [eval]:1:468\n    at runScriptInThisContext (node:internal/vm:209:10)\n    at node:internal/process/execution:446:12\n    at [eval]-wrapper:6:24\n    at runScriptInContext (node:internal/process/execution:444:60)\n    at evalFunction (node:internal/process/execution:279:30)\n    at evalTypeScript (node:internal/process/execution:291:3)\n    at node:internal/main/eval_string:74:3\nnot ok reflection errors: getOwnPropertyDescriptor trap is sanitized\nAssertionError [ERR_ASSERTION]: getOwnPropertyDescriptor trap: code\n+ actual - expected\n\n+ undefined\n- 'UNAVAILABLE'\n\n    at assertUnavailable (<repo>/test/walletBrokerLaunchConfig.node.js:171:10)\n    at <repo>/test/walletBrokerLaunchConfig.node.js:929:3\n    at [eval]:1:468\n    at runScriptInThisContext (node:internal/vm:209:10)\n    at node:internal/process/execution:446:12\n    at [eval]-wrapper:6:24\n    at runScriptInContext (node:internal/process/execution:444:60)\n    at evalFunction (node:internal/process/execution:279:30)\n    at evalTypeScript (node:internal/process/execution:291:3)\n    at node:internal/main/eval_string:74:3\n",
      "counts": {
        "ok": 0,
        "not_ok": 4
      },
      "mutant_sha256": "5b905a911303e0cb2e3d162a423a2ad7c7082313cbd976e94541ccc958a12d48",
      "restored_sha256": "68adf92e2d543e5d51353477b1944c35d4189c5590ac581d3289f1bd72898af8"
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
    }
  ],
  "green": true,
  "version": "Hermes Agent v0.18.2 (2026.7.7.2) \u00b7 upstream cfdbbb6e \u00b7 local 10b6d1a9 (+1 carried commit)\nInstall directory: <home>/.hermes/hermes-agent\nInstall method: git\nPython: 3.11.15\nOpenAI SDK: 2.24.0",
  "head": "34e622d2d6257b4ce93fac004696867858138487",
  "initial_status": "M package-lock.json\n M package.json\n M scripts/security-policy.js\n M test/securityPolicy.node.js\n?? docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md\n?? docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md\n?? docs/testing/BBD-WAL-011-LAUNCH-CONFIG-RED-01.md\n?? docs/testing/BBD-WAL-011-LAUNCH-CONFIG-REFLECTION-RED-01.md\n?? test/walletBrokerLaunchConfig.node.js\n?? wallet-broker/launch-config.js",
  "session": {
    "id": "20260909_230128_986ca6",
    "model": "poolside/laguna-s-2.1:free",
    "provider": "nous"
  },
  "input_hashes": {
    "wallet-broker/launch-config.js": "68adf92e2d543e5d51353477b1944c35d4189c5590ac581d3289f1bd72898af8",
    "test/walletBrokerLaunchConfig.node.js": "90acd32c9863df0206364eb04f707d92a5979ee66c7e15b2ec7b8250a5fc2113",
    "wallet-broker/supervisor.js": "c1410bfc5cd4c205ad014a42154c9d9d9714794b134a5acab7ed499934cad51a",
    "test/walletSupervisor.node.js": "eaaa63777c8ded89ea6e9b2a07912bd3a680953bdb989a4ff0d6894e5df6ce5c",
    "wallet-broker/protocol.js": "79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4",
    "social-main.js": "c7687b52ee45b17c3063cc2b39071be3d4e613a9d1e86558f2a7bbc1567db0a3",
    "test/electronSecurity.node.js": "7f759f810762ccdfa5e29d69dc3bf2fa1b1502d2c416ca2b324442b2544ad18c",
    "wallet-pay/model.js": "acf07238366f3e28253be8c9208fbb27e13e9c2687d374d1a6ae87e0d173fa5e",
    "test/fixtures/wallet-pay/snapshots-v1.json": "bd51c24e003eb63f84fd99ca9573e765f28c3b7bc3ccbd924b14ba34afe05252",
    "docs/testing/BBD-WAL-011-LAUNCH-CONFIG-RED-01.md": "1c98fde59cd882a406b6e73082f5fd5a1d8b7328cadc2a71e3d46941a3ae5485",
    "docs/testing/BBD-WAL-011-LAUNCH-CONFIG-REFLECTION-RED-01.md": "b19697561f63f4690304f0c57a8eef89e86204ba9d57024758bd509f32063523",
    "package.json": "76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5",
    "package-lock.json": "0e32de43f3c1a31ae3d9e9c8b172d10cd37a03577dc7d1eaf4ac916967cfcdb8",
    "scripts/security-policy.js": "fec10759a73a3c675d8206da454abf4b0e8dca6b62da463f39a2db27b30dc078",
    "test/securityPolicy.node.js": "a58d85e5322f0cb01a78b636454f92547bd98e556013cc07712dc0cf64ea565c",
    "docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md": "1e548d364ac3a39e9bb1941a5c89b0ccb45143d9c8588e35e42f12434120799f",
    "docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md": "2a6abd0a41a93028ce63fce8b3f385436623a416dfcdc26565b0277e7875bc56"
  },
  "final_hashes": {
    "wallet-broker/launch-config.js": "68adf92e2d543e5d51353477b1944c35d4189c5590ac581d3289f1bd72898af8",
    "test/walletBrokerLaunchConfig.node.js": "90acd32c9863df0206364eb04f707d92a5979ee66c7e15b2ec7b8250a5fc2113",
    "wallet-broker/supervisor.js": "c1410bfc5cd4c205ad014a42154c9d9d9714794b134a5acab7ed499934cad51a",
    "test/walletSupervisor.node.js": "eaaa63777c8ded89ea6e9b2a07912bd3a680953bdb989a4ff0d6894e5df6ce5c",
    "wallet-broker/protocol.js": "79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4",
    "social-main.js": "c7687b52ee45b17c3063cc2b39071be3d4e613a9d1e86558f2a7bbc1567db0a3",
    "test/electronSecurity.node.js": "7f759f810762ccdfa5e29d69dc3bf2fa1b1502d2c416ca2b324442b2544ad18c",
    "wallet-pay/model.js": "acf07238366f3e28253be8c9208fbb27e13e9c2687d374d1a6ae87e0d173fa5e",
    "test/fixtures/wallet-pay/snapshots-v1.json": "bd51c24e003eb63f84fd99ca9573e765f28c3b7bc3ccbd924b14ba34afe05252",
    "docs/testing/BBD-WAL-011-LAUNCH-CONFIG-RED-01.md": "1c98fde59cd882a406b6e73082f5fd5a1d8b7328cadc2a71e3d46941a3ae5485",
    "docs/testing/BBD-WAL-011-LAUNCH-CONFIG-REFLECTION-RED-01.md": "b19697561f63f4690304f0c57a8eef89e86204ba9d57024758bd509f32063523",
    "package.json": "76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5",
    "package-lock.json": "0e32de43f3c1a31ae3d9e9c8b172d10cd37a03577dc7d1eaf4ac916967cfcdb8",
    "scripts/security-policy.js": "fec10759a73a3c675d8206da454abf4b0e8dca6b62da463f39a2db27b30dc078",
    "test/securityPolicy.node.js": "a58d85e5322f0cb01a78b636454f92547bd98e556013cc07712dc0cf64ea565c",
    "docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md": "1e548d364ac3a39e9bb1941a5c89b0ccb45143d9c8588e35e42f12434120799f",
    "docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md": "2a6abd0a41a93028ce63fce8b3f385436623a416dfcdc26565b0277e7875bc56"
  },
  "final_source_sha256": "68adf92e2d543e5d51353477b1944c35d4189c5590ac581d3289f1bd72898af8"
}
```
