# WAL-015 live sync pinned Cargo audit

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
      "output": "Hermes Agent v0.18.2 (2026.7.7.2) \u00b7 upstream 564aef29 \u00b7 local 10b6d1a9 (+1 carried commit)\nInstall directory: <home>/.hermes/hermes-agent\nInstall method: git\nPython: 3.11.15\nOpenAI SDK: 2.24.0\n"
    },
    {
      "argv": [
        "git",
        "rev-parse",
        "HEAD"
      ],
      "exit": 0,
      "output": "e85dfdc8b6e06fdd09f84cee3c6ac37037448437\n"
    },
    {
      "argv": [
        "git",
        "status",
        "--short"
      ],
      "exit": 0,
      "output": " M docs/architecture/BBD-WAL-001-REVIEW.md\n M docs/handoff/CURRENT_TASK.md\n M package-lock.json\n M package.json\n M scripts/security-policy.js\n M test/securityPolicy.node.js\n M wallet-broker/Cargo.lock\n M wallet-broker/Cargo.toml\n M wallet-broker/src/zec.rs\n M wallet-broker/tests/account_management.rs\n M wallet-broker/tests/account_native_ui.rs\n?? docs/architecture/BBD-WAL-015-LIVE-SYNC-REVIEW.md\n?? docs/handoff/HERMES_BBD_WAL_015_AUDIT_01.md\n?? docs/handoff/HERMES_BBD_WAL_015_DEPS_01.md\n?? docs/handoff/HERMES_BBD_WAL_015_RED_01.md\n?? docs/handoff/SOL_BBD_WAL_015_PRODUCTION_01.md\n?? docs/handoff/SOL_BBD_WAL_015_TESTS_01.md\n?? docs/handoff/SOL_BBD_WAL_015_TRANSPORT_TESTS_01.md\n?? docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md\n?? docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md\n?? docs/testing/BBD-WAL-015-LIVE-DEPS-01.md\n?? docs/testing/BBD-WAL-015-LIVE-RED-01.md\n?? tickets/BBD-WAL-015.md\n?? wal015_audit_driver.py\n?? wallet-broker/src/zec/live.rs\n?? wallet-broker/tests/zec_live_sync.rs\n"
    },
    {
      "argv": [
        "<home>/.cargo/bin/cargo-audit",
        "--version"
      ],
      "exit": 0,
      "output": "cargo-audit 0.22.2\n"
    },
    {
      "argv": [
        "<home>/.cargo/bin/cargo-audit",
        "audit",
        "--file",
        "wallet-broker/Cargo.lock",
        "--json"
      ],
      "exit": 0,
      "output": "{\"database\":{\"advisory-count\":1243,\"last-commit\":\"b50980aad8b8f14f77e25a97b32dd94bf008b0af\",\"last-updated\":\"2026-09-09T12:49:52+02:00\"},\"lockfile\":{\"dependency-count\":546},\"settings\":{\"target_arch\":[],\"target_os\":[],\"severity\":null,\"ignore\":[],\"informational_warnings\":[\"unmaintained\",\"unsound\",\"notice\"]},\"vulnerabilities\":{\"found\":false,\"count\":0,\"list\":[]},\"warnings\":{\"unmaintained\":[{\"kind\":\"unmaintained\",\"package\":{\"name\":\"atomic-polyfill\",\"version\":\"1.0.3\",\"source\":\"registry+https://github.com/rust-lang/crates.io-index\",\"checksum\":\"8cf2bce30dfe09ef0bfaef228b9d414faaf7e563035494d7fe092dba54b300f4\",\"dependencies\":[{\"name\":\"critical-section\",\"version\":\"1.2.0\",\"source\":\"registry+https://github.com/rust-lang/crates.io-index\"}],\"replace\":null},\"advisory\":{\"id\":\"RUSTSEC-2023-0089\",\"package\":\"atomic-polyfill\",\"title\":\"atomic-polyfill is unmaintained\",\"description\":\"The author has archived the GitHub repository and mentions deprecation in\\nproject's\\n[README](https://github.com/embassy-rs/atomic-polyfill/blob/48e55c166684f37af0b00fbee5a0809b1a2bae8e/README.md).\\n\\n## Possible alternatives\\n\\n * [portable-atomic](https://crates.io/crates/portable-atomic)\",\"date\":\"2023-07-11\",\"aliases\":[],\"related\":[],\"collection\":\"crates\",\"categories\":[],\"keywords\":[],\"cvss\":null,\"informational\":\"unmaintained\",\"references\":[],\"source\":null,\"url\":\"https://github.com/embassy-rs/atomic-polyfill/commit/48e55c166684f37af0b00fbee5a0809b1a2bae8e\",\"withdrawn\":null,\"license\":\"CC0-1.0\",\"expect-deleted\":false},\"affected\":null,\"versions\":{\"patched\":[],\"unaffected\":[]}}]}}\n"
    }
  ],
  "accepted": true,
  "version": "Hermes Agent v0.18.2 (2026.7.7.2) \u00b7 upstream 564aef29 \u00b7 local 10b6d1a9 (+1 carried commit)\nInstall directory: <home>/.hermes/hermes-agent\nInstall method: git\nPython: 3.11.15\nOpenAI SDK: 2.24.0",
  "head": "e85dfdc8b6e06fdd09f84cee3c6ac37037448437",
  "initial_status": "M docs/architecture/BBD-WAL-001-REVIEW.md\n M docs/handoff/CURRENT_TASK.md\n M package-lock.json\n M package.json\n M scripts/security-policy.js\n M test/securityPolicy.node.js\n M wallet-broker/Cargo.lock\n M wallet-broker/Cargo.toml\n M wallet-broker/src/zec.rs\n M wallet-broker/tests/account_management.rs\n M wallet-broker/tests/account_native_ui.rs\n?? docs/architecture/BBD-WAL-015-LIVE-SYNC-REVIEW.md\n?? docs/handoff/HERMES_BBD_WAL_015_AUDIT_01.md\n?? docs/handoff/HERMES_BBD_WAL_015_DEPS_01.md\n?? docs/handoff/HERMES_BBD_WAL_015_RED_01.md\n?? docs/handoff/SOL_BBD_WAL_015_PRODUCTION_01.md\n?? docs/handoff/SOL_BBD_WAL_015_TESTS_01.md\n?? docs/handoff/SOL_BBD_WAL_015_TRANSPORT_TESTS_01.md\n?? docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md\n?? docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md\n?? docs/testing/BBD-WAL-015-LIVE-DEPS-01.md\n?? docs/testing/BBD-WAL-015-LIVE-RED-01.md\n?? tickets/BBD-WAL-015.md\n?? wal015_audit_driver.py\n?? wallet-broker/src/zec/live.rs\n?? wallet-broker/tests/zec_live_sync.rs",
  "session": {
    "id": "20260910_144852_8d00bf",
    "model": "meituan/longcat-2.0:free",
    "provider": "nous"
  },
  "input_hashes": {
    "wallet-broker/Cargo.toml": "508d571a3821deb2ae06400ff445be832a3ae1384d0cf4cb7d870f8e16d66b67",
    "wallet-broker/Cargo.lock": "b28a8e55de47130e53276669a4d5f3608755cb5390b2b229908090304f67ca29",
    "package.json": "76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5",
    "package-lock.json": "0e32de43f3c1a31ae3d9e9c8b172d10cd37a03577dc7d1eaf4ac916967cfcdb8",
    "scripts/security-policy.js": "fec10759a73a3c675d8206da454abf4b0e8dca6b62da463f39a2db27b30dc078",
    "test/securityPolicy.node.js": "a58d85e5322f0cb01a78b636454f92547bd98e556013cc07712dc0cf64ea565c"
  },
  "scanner_sha256": "4ac4b8a8d3893109351b2cf3b9a37c7483e63915f994ac841a405248f7a4e7fa",
  "final_hashes": {
    "wallet-broker/Cargo.toml": "508d571a3821deb2ae06400ff445be832a3ae1384d0cf4cb7d870f8e16d66b67",
    "wallet-broker/Cargo.lock": "b28a8e55de47130e53276669a4d5f3608755cb5390b2b229908090304f67ca29",
    "package.json": "76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5",
    "package-lock.json": "0e32de43f3c1a31ae3d9e9c8b172d10cd37a03577dc7d1eaf4ac916967cfcdb8",
    "scripts/security-policy.js": "fec10759a73a3c675d8206da454abf4b0e8dca6b62da463f39a2db27b30dc078",
    "test/securityPolicy.node.js": "a58d85e5322f0cb01a78b636454f92547bd98e556013cc07712dc0cf64ea565c"
  }
}
```
