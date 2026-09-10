# WAL-011 shutdown focused validation 01

One driver invocation. Actual metadata, complete stage output and restoration identities follow. This validates supervisor lifecycle with Node fixtures, not app startup, native descendants or release readiness. No integration is authorized.

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
      "output": "4ec2cf351c9ddee96d7ba0f4104a955ca48feb82\n"
    },
    {
      "argv": [
        "git",
        "status",
        "--short"
      ],
      "exit": 0,
      "output": " M package-lock.json\n M package.json\n M scripts/security-policy.js\n M test/securityPolicy.node.js\n M wallet-broker/supervisor.js\n?? docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md\n?? docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md\n?? docs/testing/BBD-WAL-011-SHUTDOWN-RED-01.md\n?? test/fixtures/wallet-broker/shutdown-child.js\n?? test/walletSupervisorShutdown.node.js\n"
    }
  ],
  "stages": [
    {
      "argv": [
        "node",
        "test/walletSupervisorShutdown.node.js"
      ],
      "exit": 0,
      "output": "ok shutdown: no-start and failed-start resolve one Promise without spawn, kill, or timers\nok shutdown: bound cancel precedes SIGTERM and close, not exit, completes one Promise\nok shutdown: prior quit, protocol failure, and close-before-shutdown keep one termination\nok shutdown: stubborn child SIGKILL at 250 ms and TIMEOUT at 1500 ms\nok shutdown: real child normal termination observes close and closed streams\nok shutdown: real child ignoring SIGTERM terminates with SIGKILL after close\nBitBook wallet supervisor shutdown tests passed (6).\n",
      "name": "shutdown-green",
      "counts": {
        "ok": 6,
        "not_ok": 0
      }
    },
    {
      "argv": [
        "node",
        "test/walletSupervisorTransport.node.js"
      ],
      "exit": 0,
      "output": "ok transport: real framed hello, ack, bootstrap and a subsequent request return a distinct fixture result\nok transport: concurrent requests resolve from reverse-order coalesced replies and broker errors drop diagnostic sentinels\nok transport: wrong session or uncorrelated response closes and rejects pending work\nok transport: partial-frame EOF and malformed frames close without a hanging promise\nok transport: stdout data events reassemble split header/body frames before ack, bootstrap and a distinct result\nok transport: handshake and request deadlines fail closed\nok transport: request limit does not write; quit and exit settle pending work and release the child\nBitBook wallet supervisor transport tests passed (7).\n",
      "name": "transport-green",
      "counts": {
        "ok": 7,
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
      "name": "supervisor-green",
      "counts": {
        "ok": 13,
        "not_ok": 0
      }
    },
    {
      "argv": [
        "node",
        "-e",
        "const {tests}=require('./test/walletSupervisorShutdown.node.js');const name='shutdown: bound cancel precedes SIGTERM and close, not exit, completes one Promise';const matches=tests.filter(t=>t.name===name);if(matches.length!==1)throw new Error('falsification selection mismatch');Promise.resolve().then(()=>matches[0].fn()).then(()=>{console.error('FALSIFICATION_UNEXPECTED_PASS');process.exitCode=2;},e=>{console.error(JSON.stringify({code:e.code,actual:e.actual,expected:e.expected,message:e.message,stack:e.stack}));process.exitCode=1;});"
      ],
      "exit": 1,
      "output": "{\"code\":\"ERR_ASSERTION\",\"actual\":\"resolved\",\"expected\":\"pending\",\"message\":\"Expected values to be strictly equal:\\n+ actual - expected\\n\\n+ 'resolved'\\n- 'pending'\\n\",\"stack\":\"AssertionError [ERR_ASSERTION]: Expected values to be strictly equal:\\n+ actual - expected\\n\\n+ 'resolved'\\n- 'pending'\\n\\n    at <repo>/test/walletSupervisorShutdown.node.js:670:12\\n    at async withFakeHarness (<repo>/test/walletSupervisorShutdown.node.js:322:14)\\n    at async Object.fn (<repo>/test/walletSupervisorShutdown.node.js:631:3)\"}\n",
      "name": "premature-completion-falsification",
      "counts": {
        "ok": 0,
        "not_ok": 0
      }
    },
    {
      "argv": [
        "node",
        "test/walletSupervisorShutdown.node.js"
      ],
      "exit": 0,
      "output": "ok shutdown: no-start and failed-start resolve one Promise without spawn, kill, or timers\nok shutdown: bound cancel precedes SIGTERM and close, not exit, completes one Promise\nok shutdown: prior quit, protocol failure, and close-before-shutdown keep one termination\nok shutdown: stubborn child SIGKILL at 250 ms and TIMEOUT at 1500 ms\nok shutdown: real child normal termination observes close and closed streams\nok shutdown: real child ignoring SIGTERM terminates with SIGKILL after close\nBitBook wallet supervisor shutdown tests passed (6).\n",
      "name": "shutdown-restored-green",
      "counts": {
        "ok": 6,
        "not_ok": 0
      }
    }
  ],
  "success": true,
  "version": "Hermes Agent v0.18.2 (2026.7.7.2) \u00b7 upstream 8e85b276 \u00b7 local 10b6d1a9 (+1 carried commit)\nInstall directory: <home>/.hermes/hermes-agent\nInstall method: git\nPython: 3.11.15\nOpenAI SDK: 2.24.0",
  "head": "4ec2cf351c9ddee96d7ba0f4104a955ca48feb82",
  "initial_status": "M package-lock.json\n M package.json\n M scripts/security-policy.js\n M test/securityPolicy.node.js\n M wallet-broker/supervisor.js\n?? docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md\n?? docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md\n?? docs/testing/BBD-WAL-011-SHUTDOWN-RED-01.md\n?? test/fixtures/wallet-broker/shutdown-child.js\n?? test/walletSupervisorShutdown.node.js",
  "session": {
    "id": "20260909_195724_724f82",
    "model": "poolside/laguna-s-2.1:free",
    "provider": "nous"
  },
  "input_hashes": {
    "wallet-broker/supervisor.js": "c1410bfc5cd4c205ad014a42154c9d9d9714794b134a5acab7ed499934cad51a",
    "wallet-broker/protocol.js": "79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4",
    "test/walletSupervisorShutdown.node.js": "69eb8bcbc2d169e00cb52a7bff73e1b5f39d7890d24d79383f63e77b3994b5c7",
    "test/fixtures/wallet-broker/shutdown-child.js": "0d8fbfa8179338a8fc05e6c41f4cbbf0ef652c4ccadd73db8876591a51f96dcb",
    "test/fixtures/wallet-broker/transport-child.js": "ba11ba8a37d4ee7a6df2e3499c40c972ca4a1cfd8ef0eed377afe1c7a1e3ba91",
    "test/walletSupervisorTransport.node.js": "e69fdcfa4aaeeca03b4abf8ac100888ff0b42fbb6001f53e461a35a5b7550700",
    "test/walletSupervisor.node.js": "eaaa63777c8ded89ea6e9b2a07912bd3a680953bdb989a4ff0d6894e5df6ce5c",
    "docs/testing/BBD-WAL-011-SHUTDOWN-RED-01.md": "32e6d2b69e05e95def6eaac0ccce6d4ff606b535fc362376934716c028b443ae"
  },
  "mutant_sha256": "25fe467451f58ed2cc12b45206d6471c1c1eb7c7aad3f016d459e4b8f3657a12",
  "restored_sha256": "c1410bfc5cd4c205ad014a42154c9d9d9714794b134a5acab7ed499934cad51a",
  "final_hashes": {
    "wallet-broker/supervisor.js": "c1410bfc5cd4c205ad014a42154c9d9d9714794b134a5acab7ed499934cad51a",
    "wallet-broker/protocol.js": "79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4",
    "test/walletSupervisorShutdown.node.js": "69eb8bcbc2d169e00cb52a7bff73e1b5f39d7890d24d79383f63e77b3994b5c7",
    "test/fixtures/wallet-broker/shutdown-child.js": "0d8fbfa8179338a8fc05e6c41f4cbbf0ef652c4ccadd73db8876591a51f96dcb",
    "test/fixtures/wallet-broker/transport-child.js": "ba11ba8a37d4ee7a6df2e3499c40c972ca4a1cfd8ef0eed377afe1c7a1e3ba91",
    "test/walletSupervisorTransport.node.js": "e69fdcfa4aaeeca03b4abf8ac100888ff0b42fbb6001f53e461a35a5b7550700",
    "test/walletSupervisor.node.js": "eaaa63777c8ded89ea6e9b2a07912bd3a680953bdb989a4ff0d6894e5df6ce5c",
    "docs/testing/BBD-WAL-011-SHUTDOWN-RED-01.md": "32e6d2b69e05e95def6eaac0ccce6d4ff606b535fc362376934716c028b443ae"
  }
}
```
