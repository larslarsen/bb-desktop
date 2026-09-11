# WAL017 public server conflict diagnosis and validated alternate batch

```json
{
  "accepted": true,
  "commands": [
    {
      "argv": [
        "hermes",
        "--version"
      ],
      "exit": 0,
      "output": "Hermes Agent v0.18.2 (2026.7.7.2) \u00b7 upstream ad03f20d \u00b7 local 10b6d1a9 (+1 carried commit)\nInstall directory: <home>/.hermes/hermes-agent\nInstall method: git\nPython: 3.11.15\nOpenAI SDK: 2.24.0\n",
      "timeout": false
    },
    {
      "argv": [
        "git",
        "rev-parse",
        "HEAD"
      ],
      "exit": 0,
      "output": "fa2ec32039eff0b27d0d81a78e91c20723d547e9\n",
      "timeout": false
    },
    {
      "argv": [
        "python3",
        "docs/handoff/WAL017_COMPARE.py"
      ],
      "exit": 0,
      "output": "{\n  \"verified\": true,\n  \"observations\": [\n    {\n      \"endpoint\": \"https://testnet.zec.rocks:443\",\n      \"persistent_connection\": false,\n      \"range\": [\n        4308220,\n        4308319\n      ],\n      \"prior_hash_matches\": true,\n      \"end_hash_matches\": false,\n      \"compact_tree_sizes\": [\n        400234,\n        248768,\n        183228\n      ],\n      \"endpoint_tree_sizes\": [\n        400234,\n        248768,\n        185128\n      ]\n    },\n    {\n      \"endpoint\": \"https://testnet.zec.rocks:443\",\n      \"persistent_connection\": true,\n      \"range\": [\n        4308220,\n        4308319\n      ],\n      \"prior_hash_matches\": true,\n      \"end_hash_matches\": false,\n      \"compact_tree_sizes\": [\n        400234,\n        248768,\n        183228\n      ],\n      \"endpoint_tree_sizes\": [\n        400234,\n        248768,\n        185128\n      ]\n    },\n    {\n      \"endpoint\": \"https://zaino.testnet.unsafe.zec.rocks:443\",\n      \"persistent_connection\": false,\n      \"range\": [\n        4308220,\n        4308319\n      ],\n      \"prior_hash_matches\": true,\n      \"end_hash_matches\": true,\n      \"compact_tree_sizes\": [\n        400234,\n        248768,\n        183228\n      ],\n      \"endpoint_tree_sizes\": [\n        400234,\n        248768,\n        183228\n      ]\n    }\n  ],\n  \"product_changes\": false,\n  \"user_wallet_modified\": false\n}\n",
      "timeout": false
    },
    {
      "argv": [
        "git",
        "diff",
        "--cached",
        "--name-only"
      ],
      "exit": 0,
      "output": "",
      "timeout": false
    },
    {
      "argv": [
        "git",
        "branch",
        "--show-current"
      ],
      "exit": 0,
      "output": "master\n",
      "timeout": false
    },
    {
      "argv": [
        "git",
        "remote",
        "get-url",
        "origin"
      ],
      "exit": 0,
      "output": "https://github.com/larslarsen/bb-desktop.git\n",
      "timeout": false
    },
    {
      "argv": [
        "git",
        "diff",
        "--check"
      ],
      "exit": 0,
      "output": "",
      "timeout": false
    },
    {
      "argv": [
        "git",
        "add",
        "--",
        "docs/handoff/CURRENT_TASK.md",
        "tickets/BBD-WAL-017.md",
        "docs/handoff/WAL017_DIAG.js",
        "docs/handoff/WAL017_DIAG_SAME_CONNECTION.js",
        "docs/handoff/WAL017_DIAG_ALTERNATES.js",
        "docs/handoff/WAL017_COMPARE.py",
        "docs/handoff/WAL017_INTEGRATE.json",
        "docs/testing/BBD-WAL-017-DIAGNOSTIC-01.md",
        "docs/testing/BBD-WAL-017-DIAGNOSTIC-02.md",
        "docs/testing/BBD-WAL-017-DIAGNOSTIC-zecpro.md",
        "docs/testing/BBD-WAL-017-DIAGNOSTIC-nighthawk.md",
        "docs/testing/BBD-WAL-017-DIAGNOSTIC-zaino.md"
      ],
      "exit": 0,
      "output": "",
      "timeout": false
    },
    {
      "argv": [
        "git",
        "diff",
        "--cached",
        "--name-only"
      ],
      "exit": 0,
      "output": "docs/handoff/CURRENT_TASK.md\ndocs/handoff/WAL017_COMPARE.py\ndocs/handoff/WAL017_DIAG.js\ndocs/handoff/WAL017_DIAG_ALTERNATES.js\ndocs/handoff/WAL017_DIAG_SAME_CONNECTION.js\ndocs/handoff/WAL017_INTEGRATE.json\ndocs/testing/BBD-WAL-017-DIAGNOSTIC-01.md\ndocs/testing/BBD-WAL-017-DIAGNOSTIC-02.md\ndocs/testing/BBD-WAL-017-DIAGNOSTIC-nighthawk.md\ndocs/testing/BBD-WAL-017-DIAGNOSTIC-zaino.md\ndocs/testing/BBD-WAL-017-DIAGNOSTIC-zecpro.md\ntickets/BBD-WAL-017.md\n",
      "timeout": false
    },
    {
      "argv": [
        "git",
        "diff",
        "--cached",
        "--check"
      ],
      "exit": 0,
      "output": "",
      "timeout": false
    },
    {
      "argv": [
        "git",
        "commit",
        "-m",
        "Record testnet sync server inconsistency and validated alternate endpoint"
      ],
      "exit": 0,
      "output": "[master aebe22db1] Record testnet sync server inconsistency and validated alternate endpoint\n 12 files changed, 623 insertions(+)\n create mode 100644 docs/handoff/WAL017_COMPARE.py\n create mode 100644 docs/handoff/WAL017_DIAG.js\n create mode 100644 docs/handoff/WAL017_DIAG_ALTERNATES.js\n create mode 100644 docs/handoff/WAL017_DIAG_SAME_CONNECTION.js\n create mode 100644 docs/handoff/WAL017_INTEGRATE.json\n create mode 100644 docs/testing/BBD-WAL-017-DIAGNOSTIC-01.md\n create mode 100644 docs/testing/BBD-WAL-017-DIAGNOSTIC-02.md\n create mode 100644 docs/testing/BBD-WAL-017-DIAGNOSTIC-nighthawk.md\n create mode 100644 docs/testing/BBD-WAL-017-DIAGNOSTIC-zaino.md\n create mode 100644 docs/testing/BBD-WAL-017-DIAGNOSTIC-zecpro.md\n create mode 100644 tickets/BBD-WAL-017.md\n",
      "timeout": false
    },
    {
      "argv": [
        "git",
        "rev-parse",
        "HEAD"
      ],
      "exit": 0,
      "output": "aebe22db1848f99ffa15d0a2b781d845fc6986cf\n",
      "timeout": false
    },
    {
      "argv": [
        "target/security-tools/gitleaks-v8.30.1/gitleaks",
        "git",
        "--redact=0",
        "--no-banner",
        "--report-format",
        "json",
        "--report-path",
        "wallet-broker/target/wal017-integration-01-gitleaks-git.json",
        "."
      ],
      "exit": 1,
      "output": "9:10AM INF 5356 commits scanned.\n9:10AM INF scanned ~35407413 bytes (35.41 MB) in 2.36s\n9:10AM WRN leaks found: 82\n",
      "timeout": false
    },
    {
      "argv": [
        "git",
        "push",
        "origin",
        "master"
      ],
      "exit": 0,
      "output": "To https://github.com/larslarsen/bb-desktop.git\n   fa2ec3203..aebe22db1  master -> master\n",
      "timeout": false
    },
    {
      "argv": [
        "git",
        "ls-remote",
        "origin",
        "refs/heads/master"
      ],
      "exit": 0,
      "output": "aebe22db1848f99ffa15d0a2b781d845fc6986cf\trefs/heads/master\n",
      "timeout": false
    }
  ],
  "session": {
    "id": "20260911_090946_035a5e",
    "model": "meituan/longcat-2.0:free",
    "provider": "nous"
  },
  "version": "Hermes Agent v0.18.2 (2026.7.7.2) \u00b7 upstream ad03f20d \u00b7 local 10b6d1a9 (+1 carried commit)\nInstall directory: <home>/.hermes/hermes-agent\nInstall method: git\nPython: 3.11.15\nOpenAI SDK: 2.24.0",
  "head": "fa2ec32039eff0b27d0d81a78e91c20723d547e9",
  "input_hashes": {
    "wallet-broker/src/accounts.rs": "8941884f837453bf71357f53341fae2c1334b8d10d017680053f9f8a44fb415d",
    "wallet-broker/src/account_ui.rs": "67bf08a6eb809796349f225cb551f342e20be98b2a7eb95570dada6f5fa2ff44",
    "wallet-broker/src/zec/test_support.rs": "de157d4a78c4c452e0e19122200997e6676c82099ddb25bfebe84781470f0791",
    "wallet-broker/src/session.rs": "9f88b3b0eca91d341a1c025bc8394a25e456350be58e99d9ac9417e31eef21af",
    "wallet-broker/Cargo.toml": "435907f00d7819bdf7dbe9ff9e367522f7f6eb8b6019dbde9e58212bbaea041b",
    "wallet-broker/Cargo.lock": "a5d53b729433a06f60edcef51e608629c7365326d06fef2bbac8f2b23f048c2a",
    "wallet-broker/tests/account_management.rs": "f1f9c70dc89e330fca6f5d292cfbabd626ef0c760a8800f522b48c070e57ddf0",
    "wallet-broker/tests/account_native_ui.rs": "a6719c17ca25e00f4a4fc217aadc97bcc49800b423b8a5b13e2e90598c393797",
    "package.json": "76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5",
    "package-lock.json": "0e32de43f3c1a31ae3d9e9c8b172d10cd37a03577dc7d1eaf4ac916967cfcdb8",
    "scripts/security-policy.js": "414a5645d8621b5d81cdeba7bb7461ec25647a5a1e5532295d6f75e7b8762532",
    "test/securityPolicy.node.js": "ff449a86e04570f8ff07bd280db6787c62f645f301d4235a65e825f3af7bbb9c",
    "docs/handoff/CURRENT_TASK.md": "7c140f96b1893dff1f1a212e8437d383f688950346b0cd8e03a29cb8056cc684",
    "tickets/BBD-WAL-017.md": "82bea83dbefa5b73a7d7cbb7d5d1dbc156f5b63dbb02914866d16fb1c34c475a",
    "docs/handoff/WAL017_DIAG.js": "7c31fefc2f928513a5be6749afa9bd5d5f99e1106445853e62990dc24aa5b47c",
    "docs/handoff/WAL017_DIAG_SAME_CONNECTION.js": "02c1997c96a278d4c67c496067d1236c95935cbd9e15d24186789269c5f882cc",
    "docs/handoff/WAL017_DIAG_ALTERNATES.js": "a0e747e46c1c90b4182dc6f79a06288bdb13204ec5b3aa4fd3bf9be3237899eb",
    "docs/handoff/WAL017_COMPARE.py": "8e25c84f692aa72c5f85e4a166548ac6374cc5a85552a652254275fc64b53c04",
    "docs/testing/BBD-WAL-017-DIAGNOSTIC-01.md": "838fc27ca90aea12b84b9473a642ce9da83dad2af1d6e5026b759ccbb08d6035",
    "docs/testing/BBD-WAL-017-DIAGNOSTIC-02.md": "42385e4cbc0d05a5857d96904682ad34c1af05ddb462f75b2a805af6207e380d",
    "docs/testing/BBD-WAL-017-DIAGNOSTIC-zecpro.md": "af90f73201b8d0571d5761a77a49a1a408bab13bbd37a239f458d74733b131ac",
    "docs/testing/BBD-WAL-017-DIAGNOSTIC-nighthawk.md": "a7a76e4f689f7bb4be11d75e4d45efedf59b1d5eecdbda15e88b3ba60ec7e540",
    "docs/testing/BBD-WAL-017-DIAGNOSTIC-zaino.md": "42d413e633751a94f8a40a982696a762de0632daf3fda4dfd3cb772a41f767b8"
  },
  "formatted": [],
  "integrated_paths": {
    "docs/handoff/CURRENT_TASK.md": {
      "sha256": "7c140f96b1893dff1f1a212e8437d383f688950346b0cd8e03a29cb8056cc684",
      "lines": 77
    },
    "tickets/BBD-WAL-017.md": {
      "sha256": "82bea83dbefa5b73a7d7cbb7d5d1dbc156f5b63dbb02914866d16fb1c34c475a",
      "lines": 68
    },
    "docs/handoff/WAL017_DIAG.js": {
      "sha256": "7c31fefc2f928513a5be6749afa9bd5d5f99e1106445853e62990dc24aa5b47c",
      "lines": 59
    },
    "docs/handoff/WAL017_DIAG_SAME_CONNECTION.js": {
      "sha256": "02c1997c96a278d4c67c496067d1236c95935cbd9e15d24186789269c5f882cc",
      "lines": 63
    },
    "docs/handoff/WAL017_DIAG_ALTERNATES.js": {
      "sha256": "a0e747e46c1c90b4182dc6f79a06288bdb13204ec5b3aa4fd3bf9be3237899eb",
      "lines": 62
    },
    "docs/handoff/WAL017_COMPARE.py": {
      "sha256": "8e25c84f692aa72c5f85e4a166548ac6374cc5a85552a652254275fc64b53c04",
      "lines": 67
    },
    "docs/handoff/WAL017_INTEGRATE.json": {
      "sha256": "cb390b83d468a5b683eae41cc60e6fd70d54ff9f544378882b12d0da1c17c36c",
      "lines": 59
    },
    "docs/testing/BBD-WAL-017-DIAGNOSTIC-01.md": {
      "sha256": "838fc27ca90aea12b84b9473a642ce9da83dad2af1d6e5026b759ccbb08d6035",
      "lines": 60
    },
    "docs/testing/BBD-WAL-017-DIAGNOSTIC-02.md": {
      "sha256": "42385e4cbc0d05a5857d96904682ad34c1af05ddb462f75b2a805af6207e380d",
      "lines": 60
    },
    "docs/testing/BBD-WAL-017-DIAGNOSTIC-zecpro.md": {
      "sha256": "af90f73201b8d0571d5761a77a49a1a408bab13bbd37a239f458d74733b131ac",
      "lines": 24
    },
    "docs/testing/BBD-WAL-017-DIAGNOSTIC-nighthawk.md": {
      "sha256": "a7a76e4f689f7bb4be11d75e4d45efedf59b1d5eecdbda15e88b3ba60ec7e540",
      "lines": 24
    },
    "docs/testing/BBD-WAL-017-DIAGNOSTIC-zaino.md": {
      "sha256": "42d413e633751a94f8a40a982696a762de0632daf3fda4dfd3cb772a41f767b8",
      "lines": 60
    }
  },
  "source_commit": "aebe22db1848f99ffa15d0a2b781d845fc6986cf",
  "scans": [
    {
      "mode": "git",
      "exit": 1,
      "public_checksum_false_positives": 82,
      "actual_credentials": 0,
      "classification": "docs/architecture/BBD-WAL-015-FINAL-SCAN-REVIEW.md"
    }
  ],
  "source_pushed": true,
  "final_hashes": {
    "wallet-broker/src/accounts.rs": "8941884f837453bf71357f53341fae2c1334b8d10d017680053f9f8a44fb415d",
    "wallet-broker/src/account_ui.rs": "67bf08a6eb809796349f225cb551f342e20be98b2a7eb95570dada6f5fa2ff44",
    "wallet-broker/src/zec/test_support.rs": "de157d4a78c4c452e0e19122200997e6676c82099ddb25bfebe84781470f0791",
    "wallet-broker/src/session.rs": "9f88b3b0eca91d341a1c025bc8394a25e456350be58e99d9ac9417e31eef21af",
    "wallet-broker/Cargo.toml": "435907f00d7819bdf7dbe9ff9e367522f7f6eb8b6019dbde9e58212bbaea041b",
    "wallet-broker/Cargo.lock": "a5d53b729433a06f60edcef51e608629c7365326d06fef2bbac8f2b23f048c2a",
    "wallet-broker/tests/account_management.rs": "f1f9c70dc89e330fca6f5d292cfbabd626ef0c760a8800f522b48c070e57ddf0",
    "wallet-broker/tests/account_native_ui.rs": "a6719c17ca25e00f4a4fc217aadc97bcc49800b423b8a5b13e2e90598c393797",
    "package.json": "76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5",
    "package-lock.json": "0e32de43f3c1a31ae3d9e9c8b172d10cd37a03577dc7d1eaf4ac916967cfcdb8",
    "scripts/security-policy.js": "414a5645d8621b5d81cdeba7bb7461ec25647a5a1e5532295d6f75e7b8762532",
    "test/securityPolicy.node.js": "ff449a86e04570f8ff07bd280db6787c62f645f301d4235a65e825f3af7bbb9c",
    "docs/handoff/CURRENT_TASK.md": "7c140f96b1893dff1f1a212e8437d383f688950346b0cd8e03a29cb8056cc684",
    "tickets/BBD-WAL-017.md": "82bea83dbefa5b73a7d7cbb7d5d1dbc156f5b63dbb02914866d16fb1c34c475a",
    "docs/handoff/WAL017_DIAG.js": "7c31fefc2f928513a5be6749afa9bd5d5f99e1106445853e62990dc24aa5b47c",
    "docs/handoff/WAL017_DIAG_SAME_CONNECTION.js": "02c1997c96a278d4c67c496067d1236c95935cbd9e15d24186789269c5f882cc",
    "docs/handoff/WAL017_DIAG_ALTERNATES.js": "a0e747e46c1c90b4182dc6f79a06288bdb13204ec5b3aa4fd3bf9be3237899eb",
    "docs/handoff/WAL017_COMPARE.py": "8e25c84f692aa72c5f85e4a166548ac6374cc5a85552a652254275fc64b53c04",
    "docs/testing/BBD-WAL-017-DIAGNOSTIC-01.md": "838fc27ca90aea12b84b9473a642ce9da83dad2af1d6e5026b759ccbb08d6035",
    "docs/testing/BBD-WAL-017-DIAGNOSTIC-02.md": "42385e4cbc0d05a5857d96904682ad34c1af05ddb462f75b2a805af6207e380d",
    "docs/testing/BBD-WAL-017-DIAGNOSTIC-zecpro.md": "af90f73201b8d0571d5761a77a49a1a408bab13bbd37a239f458d74733b131ac",
    "docs/testing/BBD-WAL-017-DIAGNOSTIC-nighthawk.md": "a7a76e4f689f7bb4be11d75e4d45efedf59b1d5eecdbda15e88b3ba60ec7e540",
    "docs/testing/BBD-WAL-017-DIAGNOSTIC-zaino.md": "42d413e633751a94f8a40a982696a762de0632daf3fda4dfd3cb772a41f767b8"
  }
}
```
