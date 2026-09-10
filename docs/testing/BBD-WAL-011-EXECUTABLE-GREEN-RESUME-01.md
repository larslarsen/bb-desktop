# WAL-011 executable validation Resume 01

Generated from the actual driver/gate result. Earlier green-01 is a rejected zero-stage stop. This record does not claim integration or app/release acceptance.

```json
{
  "stages": [
    {
      "name": "format",
      "argv": [
        "<home>/.cargo/bin/rustup",
        "run",
        "1.98.0",
        "rustfmt",
        "--edition",
        "2024",
        "wallet-broker/src/main.rs"
      ],
      "exit": 0,
      "output": ""
    },
    {
      "name": "build",
      "argv": [
        "<home>/.cargo/bin/rustup",
        "run",
        "1.98.0",
        "cargo",
        "build",
        "--manifest-path",
        "wallet-broker/Cargo.toml",
        "--locked",
        "--offline",
        "--no-default-features",
        "--bin",
        "bitbook-wallet-broker"
      ],
      "exit": 0,
      "output": "   Compiling bitbook-wallet-broker v0.1.0 (<repo>/wallet-broker)\n    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.79s\n"
    },
    {
      "name": "clippy",
      "argv": [
        "<home>/.cargo/bin/rustup",
        "run",
        "1.98.0",
        "cargo",
        "clippy",
        "--manifest-path",
        "wallet-broker/Cargo.toml",
        "--locked",
        "--offline",
        "--no-default-features",
        "--bin",
        "bitbook-wallet-broker",
        "--",
        "-D",
        "warnings"
      ],
      "exit": 0,
      "output": "    Checking bitbook-wallet-broker v0.1.0 (<repo>/wallet-broker)\n    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.58s\n"
    },
    {
      "name": "mutant-build",
      "argv": [
        "<home>/.cargo/bin/rustup",
        "run",
        "1.98.0",
        "cargo",
        "build",
        "--manifest-path",
        "wallet-broker/Cargo.toml",
        "--locked",
        "--offline",
        "--no-default-features",
        "--bin",
        "bitbook-wallet-broker"
      ],
      "exit": 0,
      "output": "   Compiling bitbook-wallet-broker v0.1.0 (<repo>/wallet-broker)\n    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.45s\n"
    },
    {
      "name": "session-falsification",
      "argv": [
        "node",
        "-e",
        "const tests = require('./test/walletBrokerRuntime.node.js').tests;\nconst selected = tests.filter(t => t.name === 'direct: wrong session, sequence, kind, and duplicate ids terminate promptly');\nif (selected.length !== 1) throw new Error('missing unique session test');\nselected[0].fn().then(() => { console.error('UNEXPECTED MUTANT PASS'); process.exitCode=0; }, error => { console.error(error.stack || error); process.exitCode=1; });"
      ],
      "exit": 1,
      "output": "Error: wrong session did not terminate promptly\n    at Timeout.<anonymous> (<repo>/test/walletBrokerRuntime.node.js:217:43)\n    at listOnTimeout (node:internal/timers:585:17)\n    at process.processTimers (node:internal/timers:521:7)\n"
    },
    {
      "name": "restored-build",
      "argv": [
        "<home>/.cargo/bin/rustup",
        "run",
        "1.98.0",
        "cargo",
        "build",
        "--manifest-path",
        "wallet-broker/Cargo.toml",
        "--locked",
        "--offline",
        "--no-default-features",
        "--bin",
        "bitbook-wallet-broker"
      ],
      "exit": 0,
      "output": "   Compiling bitbook-wallet-broker v0.1.0 (<repo>/wallet-broker)\n    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.31s\n"
    },
    {
      "name": "runtime-green",
      "argv": [
        "node",
        "test/walletBrokerRuntime.node.js"
      ],
      "exit": 0,
      "output": "ok runtime: compiled native executable is present\nok supervisor: real compiled child binds, degraded snapshot, status.get, account.list UNAVAILABLE, quit reaps\nok direct: hello uses exact v1 keys, actual PID, and a fresh nonce on each start\nok direct: split ack and coalesced status requests return exact IDs, degraded results, and independent child sequences\nok direct: malformed, duplicate-key, and oversize frames terminate promptly\nok direct: wrong session, sequence, kind, and duplicate ids terminate promptly\nok direct: absent and partial ack hit the two-second deadline\nok direct: partial-frame EOF after a valid session exits\nok direct: SCHEMA, TIMEOUT, and UNAVAILABLE stay fixed, canaries stay off the wire, and cwd stays empty\nBitBook wallet broker runtime tests passed (9).\n"
    }
  ],
  "success": true,
  "input_hashes": {
    "wallet-broker/src/main.rs": "19b1651d88d85e5d968597eb26eeb62f76a11af56eaa311d60a55d3ef282736d",
    "wallet-broker/src/runtime.rs": "c579bf54af9e17abc2eb49b42c9a944cd6ec2b6be1e5d900963564f27ecd86e9",
    "test/walletBrokerRuntime.node.js": "a49c3e0c49ff664997f78222d979e20603d9a2e9985735c6aa07c5c02308f39e",
    "wallet-broker/supervisor.js": "1ac92493cb70cd4ea5e1884f4b96424c7fbc89126308299e7221f2b0ba1a8ff8",
    "wallet-broker/protocol.js": "79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4",
    "wallet-broker/Cargo.toml": "73e5e585eb2fd1ca867d66962460cfa010443886a4b59d8a33556ef2a4ff3503",
    "wallet-broker/Cargo.lock": "b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71",
    "wallet-broker/src/lib.rs": "08dd09d23a8c18cdb9a50968ade153a2118b60132f2b7b66a36c6913596de925"
  },
  "formatted_hashes": {
    "wallet-broker/src/main.rs": "19b1651d88d85e5d968597eb26eeb62f76a11af56eaa311d60a55d3ef282736d",
    "wallet-broker/src/runtime.rs": "968194aadd609f54ef28a11a090fcb1ca840cfc851da127456dd5b356437305b",
    "test/walletBrokerRuntime.node.js": "a49c3e0c49ff664997f78222d979e20603d9a2e9985735c6aa07c5c02308f39e",
    "wallet-broker/supervisor.js": "1ac92493cb70cd4ea5e1884f4b96424c7fbc89126308299e7221f2b0ba1a8ff8",
    "wallet-broker/protocol.js": "79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4",
    "wallet-broker/Cargo.toml": "73e5e585eb2fd1ca867d66962460cfa010443886a4b59d8a33556ef2a4ff3503",
    "wallet-broker/Cargo.lock": "b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71",
    "wallet-broker/src/lib.rs": "08dd09d23a8c18cdb9a50968ade153a2118b60132f2b7b66a36c6913596de925"
  },
  "restored_runtime_sha256": "968194aadd609f54ef28a11a090fcb1ca840cfc851da127456dd5b356437305b",
  "final_hashes": {
    "wallet-broker/src/main.rs": "19b1651d88d85e5d968597eb26eeb62f76a11af56eaa311d60a55d3ef282736d",
    "wallet-broker/src/runtime.rs": "968194aadd609f54ef28a11a090fcb1ca840cfc851da127456dd5b356437305b",
    "test/walletBrokerRuntime.node.js": "a49c3e0c49ff664997f78222d979e20603d9a2e9985735c6aa07c5c02308f39e",
    "wallet-broker/supervisor.js": "1ac92493cb70cd4ea5e1884f4b96424c7fbc89126308299e7221f2b0ba1a8ff8",
    "wallet-broker/protocol.js": "79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4",
    "wallet-broker/Cargo.toml": "73e5e585eb2fd1ca867d66962460cfa010443886a4b59d8a33556ef2a4ff3503",
    "wallet-broker/Cargo.lock": "b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71",
    "wallet-broker/src/lib.rs": "08dd09d23a8c18cdb9a50968ade153a2118b60132f2b7b66a36c6913596de925"
  },
  "metadata": {
    "preflight": [
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
        "output": "85b2820db0d6aebc9743cb555e2463f51345625a\n"
      },
      {
        "argv": [
          "git",
          "status",
          "--short"
        ],
        "exit": 0,
        "output": " M package-lock.json\n M package.json\n M scripts/security-policy.js\n M test/securityPolicy.node.js\n?? docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md\n?? docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md\n?? docs/testing/BBD-WAL-011-EXECUTABLE-RED-01.md\n?? test/walletBrokerRuntime.node.js\n?? wallet-broker/src/main.rs\n?? wallet-broker/src/runtime.rs\n"
      },
      {
        "argv": [
          "git",
          "status",
          "--short"
        ],
        "exit": 0,
        "output": " M package-lock.json\n M package.json\n M scripts/security-policy.js\n M test/securityPolicy.node.js\n?? docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md\n?? docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md\n?? docs/testing/BBD-WAL-011-EXECUTABLE-RED-01.md\n?? test/walletBrokerRuntime.node.js\n?? wallet-broker/src/main.rs\n?? wallet-broker/src/runtime.rs\n"
      }
    ],
    "version": "Hermes Agent v0.18.2 (2026.7.7.2) \u00b7 upstream 8e85b276 \u00b7 local 10b6d1a9 (+1 carried commit)\nInstall directory: <home>/.hermes/hermes-agent\nInstall method: git\nPython: 3.11.15\nOpenAI SDK: 2.24.0",
    "head": "85b2820db0d6aebc9743cb555e2463f51345625a",
    "initial_status": " M package-lock.json\n M package.json\n M scripts/security-policy.js\n M test/securityPolicy.node.js\n?? docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md\n?? docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md\n?? docs/testing/BBD-WAL-011-EXECUTABLE-RED-01.md\n?? test/walletBrokerRuntime.node.js\n?? wallet-broker/src/main.rs\n?? wallet-broker/src/runtime.rs\n",
    "session": {
      "id": "20260909_173905_d15995",
      "model": "poolside/laguna-s-2.1:free",
      "provider": "nous"
    },
    "final_status": " M package-lock.json\n M package.json\n M scripts/security-policy.js\n M test/securityPolicy.node.js\n?? docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md\n?? docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md\n?? docs/testing/BBD-WAL-011-EXECUTABLE-RED-01.md\n?? test/walletBrokerRuntime.node.js\n?? wallet-broker/src/main.rs\n?? wallet-broker/src/runtime.rs\n"
  },
  "driver_exit": 0
}
```
