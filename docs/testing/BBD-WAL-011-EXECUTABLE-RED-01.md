# WAL-011 executable expected red — corrected evidence

Regenerated during integration from original session messages 80253, 80262 and 80263. The original draft omitted or changed stack frames and denied observed deviations. This record supersedes that draft without rerunning the test.

Reviewer accepted the actual exit-1/nine-failure missing-executable result. Actual command included 2>&1. The actor also performed forbidden session-environment discovery, extra Git/history/source reads and a binary existence check, and used execute_code instead of the prescribed metadata command. See the collected review in the executable-red handoff. No credential values are reproduced. The missing-binary branch did not exercise child cleanup; no independent resource validation is claimed.

```json
{
  "session": {
    "id": "20260909_164019_e64925",
    "model": "poolside/laguna-s-2.1:free",
    "provider": "nous"
  },
  "version_and_baseline_tool_result": {
    "output": "9ce6e6699d2a63af1ed34fdc7e6ebe05952915b6\n M package-lock.json\n M package.json\n M scripts/security-policy.js\n M test/securityPolicy.node.js\n?? docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md\n?? test/walletBrokerRuntime.node.js\na49c3e0c49ff664997f78222d979e20603d9a2e9985735c6aa07c5c02308f39e  test/walletBrokerRuntime.node.js\n1ac92493cb70cd4ea5e1884f4b96424c7fbc89126308299e7221f2b0ba1a8ff8  wallet-broker/supervisor.js\n79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4  wallet-broker/protocol.js\nHermes Agent v0.18.2 (2026.7.7.2) \u00b7 upstream 990473a7 \u00b7 local 10b6d1a9 (+1 carried commit)\nInstall directory: <home>/.hermes/hermes-agent\nInstall method: git\nPython: 3.11.15\nOpenAI SDK: 2.24.0",
    "exit_code": 0,
    "error": null
  },
  "actual_command": "node test/walletBrokerRuntime.node.js 2>&1",
  "result": {
    "output": "not ok runtime: compiled native executable is present\nError: native wallet broker executable is missing\n    at requireBrokerPath (<repo>/test/walletBrokerRuntime.node.js:52:11)\n    at <repo>/test/walletBrokerRuntime.node.js:532:3\n    at run (<repo>/test/walletBrokerRuntime.node.js:935:13)\n    at Object.<anonymous> (<repo>/test/walletBrokerRuntime.node.js:946:30)\n    at Module._compile (node:internal/modules/cjs/loader:1781:14)\n    at Object..js (node:internal/modules/cjs/loader:1913:10)\n    at Module.load (node:internal/modules/cjs/loader:1505:32)\n    at Function._load (node:internal/modules/cjs/loader:1309:12)\n    at wrapModuleLoad (node:internal/modules/cjs/loader:254:19)\n    at Function.executeUserEntryPoint [as runMain] (node:internal/modules/run_main:171:5)\nnot ok supervisor: real compiled child binds, degraded snapshot, status.get, account.list UNAVAILABLE, quit reaps\nError: native wallet broker executable is missing\n    at requireBrokerPath (<repo>/test/walletBrokerRuntime.node.js:52:11)\n    at withSupervisor (<repo>/test/walletBrokerRuntime.node.js:454:22)\n    at <repo>/test/walletBrokerRuntime.node.js:536:9\n    at run (<repo>/test/walletBrokerRuntime.node.js:935:13)\n    at Object.<anonymous> (<repo>/test/walletBrokerRuntime.node.js:946:30)\n    at Module._compile (node:internal/modules/cjs/loader:1781:14)\n    at Object..js (node:internal/modules/cjs/loader:1913:10)\n    at Module.load (node:internal/modules/cjs/loader:1505:32)\n    at Function._load (node:internal/modules/cjs/loader:1309:12)\n    at wrapModuleLoad (node:internal/modules/cjs/loader:254:19)\nnot ok direct: hello uses exact v1 keys, actual PID, and a fresh nonce on each start\nError: native wallet broker executable is missing\n    at requireBrokerPath (<repo>/test/walletBrokerRuntime.node.js:52:11)\n    at withDirectChild (<repo>/test/walletBrokerRuntime.node.js:497:22)\n    at <repo>/test/walletBrokerRuntime.node.js:610:23\n    at run (<repo>/test/walletBrokerRuntime.node.js:935:13)\nnot ok direct: split ack and coalesced status requests return exact IDs, degraded results, and independent child sequences\nError: native wallet broker executable is missing\n    at requireBrokerPath (<repo>/test/walletBrokerRuntime.node.js:52:11)\n    at withDirectChild (<repo>/test/walletBrokerRuntime.node.js:497:22)\n    at <repo>/test/walletBrokerRuntime.node.js:632:9\n    at run (<repo>/test/walletBrokerRuntime.node.js:935:13)\nnot ok direct: malformed, duplicate-key, and oversize frames terminate promptly\nError: native wallet broker executable is missing\n    at requireBrokerPath (<repo>/test/walletBrokerRuntime.node.js:52:11)\n    at withDirectChild (<repo>/test/walletBrokerRuntime.node.js:497:22)\n    at <repo>/test/walletBrokerRuntime.node.js:664:9\n    at run (<repo>/test/walletBrokerRuntime.node.js:935:13)\nnot ok direct: wrong session, sequence, kind, and duplicate ids terminate promptly\nError: native wallet broker executable is missing\n    at requireBrokerPath (<repo>/test/walletBrokerRuntime.node.js:52:11)\n    at withDirectChild (<repo>/test/walletBrokerRuntime.node.js:497:22)\n    at <repo>/test/walletBrokerRuntime.node.js:727:9\n    at run (<repo>/test/walletBrokerRuntime.node.js:935:13)\nnot ok direct: absent and partial ack hit the two-second deadline\nError: native wallet broker executable is missing\n    at requireBrokerPath (<repo>/test/walletBrokerRuntime.node.js:52:11)\n    at withDirectChild (<repo>/test/walletBrokerRuntime.node.js:497:22)\n    at <repo>/test/walletBrokerRuntime.node.js:780:9\n    at run (<repo>/test/walletBrokerRuntime.node.js:935:13)\nnot ok direct: partial-frame EOF after a valid session exits\nError: native wallet broker executable is missing\n    at requireBrokerPath (<repo>/test/walletBrokerRuntime.node.js:52:11)\n    at withDirectChild (<repo>/test/walletBrokerRuntime.node.js:497:22)\n    at <repo>/test/walletBrokerRuntime.node.js:829:9\n    at run (<repo>/test/walletBrokerRuntime.node.js:935:13)\nnot ok direct: SCHEMA, TIMEOUT, and UNAVAILABLE stay fixed, canaries stay off the wire, and cwd stays empty\nError: native wallet broker executable is missing\n    at requireBrokerPath (<repo>/test/walletBrokerRuntime.node.js:52:11)\n    at withDirectChild (<repo>/test/walletBrokerRuntime.node.js:497:22)\n    at <repo>/test/walletBrokerRuntime.node.js:859:9\n    at run (<repo>/test/walletBrokerRuntime.node.js:935:13)",
    "exit_code": 1,
    "error": null
  },
  "counts": {
    "ok": 0,
    "not_ok": 9
  },
  "frozen_input_hashes": {
    "test/walletBrokerRuntime.node.js": "a49c3e0c49ff664997f78222d979e20603d9a2e9985735c6aa07c5c02308f39e",
    "wallet-broker/supervisor.js": "1ac92493cb70cd4ea5e1884f4b96424c7fbc89126308299e7221f2b0ba1a8ff8",
    "wallet-broker/protocol.js": "79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4"
  }
}
```
