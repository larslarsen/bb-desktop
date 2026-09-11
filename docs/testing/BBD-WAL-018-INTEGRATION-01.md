# WAL018 default-server correction integration

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
      "output": "acf53cf26c41992f198bdb6a4cde5fb04ccffb8b\n",
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
        "wallet-broker/src/account_ui.rs",
        "wallet-broker/tests/account_native_ui.rs",
        "tickets/BBD-WAL-018.md",
        "docs/handoff/CURRENT_TASK.md",
        "docs/architecture/BBD-WAL-018-REVIEW.md",
        "docs/handoff/WAL018_CLEANUP.py",
        "docs/handoff/WAL018_CLONE.py",
        "docs/handoff/WAL018_DIAG.json",
        "docs/handoff/WAL018_DIAG02.json",
        "docs/handoff/WAL018_DIAG03.json",
        "docs/handoff/WAL018_GREEN.json",
        "docs/handoff/WAL018_MOVE.py",
        "docs/handoff/WAL018_QUIET.py",
        "docs/handoff/WAL018_RESTORE.py",
        "docs/testing/BBD-WAL-018-DIAGNOSTIC-01.md",
        "docs/testing/BBD-WAL-018-DIAGNOSTIC-02.md",
        "docs/testing/BBD-WAL-018-DIAGNOSTIC-03.md",
        "docs/testing/BBD-WAL-018-GREEN-01.md",
        "docs/handoff/WAL018_INTEGRATE.json"
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
      "output": "docs/architecture/BBD-WAL-018-REVIEW.md\ndocs/handoff/CURRENT_TASK.md\ndocs/handoff/WAL018_CLEANUP.py\ndocs/handoff/WAL018_CLONE.py\ndocs/handoff/WAL018_DIAG.json\ndocs/handoff/WAL018_DIAG02.json\ndocs/handoff/WAL018_DIAG03.json\ndocs/handoff/WAL018_GREEN.json\ndocs/handoff/WAL018_INTEGRATE.json\ndocs/handoff/WAL018_MOVE.py\ndocs/handoff/WAL018_QUIET.py\ndocs/handoff/WAL018_RESTORE.py\ndocs/testing/BBD-WAL-018-DIAGNOSTIC-01.md\ndocs/testing/BBD-WAL-018-DIAGNOSTIC-02.md\ndocs/testing/BBD-WAL-018-DIAGNOSTIC-03.md\ndocs/testing/BBD-WAL-018-GREEN-01.md\ntickets/BBD-WAL-018.md\nwallet-broker/src/account_ui.rs\nwallet-broker/tests/account_native_ui.rs\n",
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
        "Use verified testnet sync server by default"
      ],
      "exit": 0,
      "output": "[master e33bb63df] Use verified testnet sync server by default\n 19 files changed, 1368 insertions(+), 2 deletions(-)\n create mode 100644 docs/architecture/BBD-WAL-018-REVIEW.md\n create mode 100644 docs/handoff/WAL018_CLEANUP.py\n create mode 100644 docs/handoff/WAL018_CLONE.py\n create mode 100644 docs/handoff/WAL018_DIAG.json\n create mode 100644 docs/handoff/WAL018_DIAG02.json\n create mode 100644 docs/handoff/WAL018_DIAG03.json\n create mode 100644 docs/handoff/WAL018_GREEN.json\n create mode 100644 docs/handoff/WAL018_INTEGRATE.json\n create mode 100644 docs/handoff/WAL018_MOVE.py\n create mode 100644 docs/handoff/WAL018_QUIET.py\n create mode 100644 docs/handoff/WAL018_RESTORE.py\n create mode 100644 docs/testing/BBD-WAL-018-DIAGNOSTIC-01.md\n create mode 100644 docs/testing/BBD-WAL-018-DIAGNOSTIC-02.md\n create mode 100644 docs/testing/BBD-WAL-018-DIAGNOSTIC-03.md\n create mode 100644 docs/testing/BBD-WAL-018-GREEN-01.md\n create mode 100644 tickets/BBD-WAL-018.md\n",
      "timeout": false
    },
    {
      "argv": [
        "git",
        "rev-parse",
        "HEAD"
      ],
      "exit": 0,
      "output": "e33bb63dfdb2e779b80bf3ddc541bd5090b4926a\n",
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
        "wallet-broker/target/wal018-integration-01-gitleaks-git.json",
        "."
      ],
      "exit": 1,
      "output": "9:48AM INF 5358 commits scanned.\n9:48AM INF scanned ~35502876 bytes (35.50 MB) in 2.27s\n9:48AM WRN leaks found: 82\n",
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
      "output": "To https://github.com/larslarsen/bb-desktop.git\n   acf53cf26..e33bb63df  master -> master\n",
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
      "output": "e33bb63dfdb2e779b80bf3ddc541bd5090b4926a\trefs/heads/master\n",
      "timeout": false
    }
  ],
  "session": {
    "id": "20260911_094809_d9cda9",
    "model": "meituan/longcat-2.0:free",
    "provider": "nous"
  },
  "version": "Hermes Agent v0.18.2 (2026.7.7.2) \u00b7 upstream ad03f20d \u00b7 local 10b6d1a9 (+1 carried commit)\nInstall directory: <home>/.hermes/hermes-agent\nInstall method: git\nPython: 3.11.15\nOpenAI SDK: 2.24.0",
  "head": "acf53cf26c41992f198bdb6a4cde5fb04ccffb8b",
  "input_hashes": {
    "wallet-broker/src/accounts.rs": "8941884f837453bf71357f53341fae2c1334b8d10d017680053f9f8a44fb415d",
    "wallet-broker/src/account_ui.rs": "cbfaf77b02ea725422426f2b1276e738308e4a110654e56fa05f2008070b6e0e",
    "wallet-broker/src/session.rs": "9f88b3b0eca91d341a1c025bc8394a25e456350be58e99d9ac9417e31eef21af",
    "wallet-broker/Cargo.toml": "435907f00d7819bdf7dbe9ff9e367522f7f6eb8b6019dbde9e58212bbaea041b",
    "wallet-broker/Cargo.lock": "a5d53b729433a06f60edcef51e608629c7365326d06fef2bbac8f2b23f048c2a",
    "wallet-broker/tests/account_management.rs": "f1f9c70dc89e330fca6f5d292cfbabd626ef0c760a8800f522b48c070e57ddf0",
    "wallet-broker/tests/account_native_ui.rs": "67323f8889b789bcb9d01f9d26b4af8016da5bbf94d702bf8ae886ba5487ebb7",
    "package.json": "76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5",
    "package-lock.json": "0e32de43f3c1a31ae3d9e9c8b172d10cd37a03577dc7d1eaf4ac916967cfcdb8",
    "scripts/security-policy.js": "414a5645d8621b5d81cdeba7bb7461ec25647a5a1e5532295d6f75e7b8762532",
    "test/securityPolicy.node.js": "ff449a86e04570f8ff07bd280db6787c62f645f301d4235a65e825f3af7bbb9c",
    "docs/handoff/WAL018_QUIET.py": "fc58d8fc881f9c307b4fa688e69761fb1e6562a3319a41bbe09a18d22cd9fbaa",
    "docs/handoff/WAL018_RESTORE.py": "6e699e9d0caecee6df35ba37cff326d879fe8e38a5ca8e2d6967df990482633c",
    "docs/handoff/WAL018_CLEANUP.py": "1d5ad94fb001454544b0adf011e56ccd7af74a9ab89609af36a012c7015e0981",
    "docs/handoff/WAL016_RUNNER.py": "ebc5ae611b596709eda801ed3087d808a50cf79bd8cda55520eb02ccf74656f0",
    "wallet-broker/src/zec/test_support.rs": "de157d4a78c4c452e0e19122200997e6676c82099ddb25bfebe84781470f0791",
    "tickets/BBD-WAL-018.md": "10da91196dcf73d073a5e42593f38aa871e9d2350b6703f7c63398ddf4333e9b",
    "docs/handoff/CURRENT_TASK.md": "d92317c6cc35ab587538244df8bad27a6110d9f8675753e6b17f51fe58bcec6f",
    "docs/architecture/BBD-WAL-018-REVIEW.md": "946a51211fc2483292c97f4f4169d839a08c5b350657c38fcb89954b75ae9da3",
    "docs/handoff/WAL018_CLONE.py": "b3794ccc65a8d64eb6d1773696e8fd7075696ffe2e41a364a53f24bd5efc7994",
    "docs/handoff/WAL018_DIAG.json": "a1842e90b5d5eb76627516462e8b88f931f055facb1fa4b67d0f2dabd062f133",
    "docs/handoff/WAL018_DIAG02.json": "25b7feb39c1bc3126a3a65e1e2c27e13a8974cbf77fee9f52905fc2033a336a9",
    "docs/handoff/WAL018_DIAG03.json": "a3d51f81ac3e5b9f982fab4b366fbe155d2c1799246fb35bf59ed9fb917279c8",
    "docs/handoff/WAL018_GREEN.json": "8ad01b7607ea0ff336be48144535e819f7ff88022ab8f5d6e008fb0cb7a07fea",
    "docs/handoff/WAL018_MOVE.py": "c9ab6ea090aa54aa74ec4a5e39708e6616566112666819747be57775b3509fdc",
    "docs/testing/BBD-WAL-018-DIAGNOSTIC-01.md": "a00533a4dec8f97c7189356a12752fe1af21fd42768086fa5b1bcfbcae355280",
    "docs/testing/BBD-WAL-018-DIAGNOSTIC-02.md": "40ef7a79e2558a0617102d9e7d3cb388e648139e483f5a4ac3c0f02344fd83aa",
    "docs/testing/BBD-WAL-018-DIAGNOSTIC-03.md": "55e2b7f935ef076b93921692a928c15e1ead8ecd1344bbf58381cb559538f028",
    "docs/testing/BBD-WAL-018-GREEN-01.md": "17edb16162895eed1bd4de63539cd57d2921ee9c8b0f1132de70d3e8e5294e6b"
  },
  "formatted": [],
  "integrated_paths": {
    "wallet-broker/src/account_ui.rs": {
      "sha256": "cbfaf77b02ea725422426f2b1276e738308e4a110654e56fa05f2008070b6e0e",
      "lines": 1281
    },
    "wallet-broker/tests/account_native_ui.rs": {
      "sha256": "67323f8889b789bcb9d01f9d26b4af8016da5bbf94d702bf8ae886ba5487ebb7",
      "lines": 2335
    },
    "tickets/BBD-WAL-018.md": {
      "sha256": "10da91196dcf73d073a5e42593f38aa871e9d2350b6703f7c63398ddf4333e9b",
      "lines": 105
    },
    "docs/handoff/CURRENT_TASK.md": {
      "sha256": "d92317c6cc35ab587538244df8bad27a6110d9f8675753e6b17f51fe58bcec6f",
      "lines": 95
    },
    "docs/architecture/BBD-WAL-018-REVIEW.md": {
      "sha256": "946a51211fc2483292c97f4f4169d839a08c5b350657c38fcb89954b75ae9da3",
      "lines": 42
    },
    "docs/handoff/WAL018_CLEANUP.py": {
      "sha256": "1d5ad94fb001454544b0adf011e56ccd7af74a9ab89609af36a012c7015e0981",
      "lines": 29
    },
    "docs/handoff/WAL018_CLONE.py": {
      "sha256": "b3794ccc65a8d64eb6d1773696e8fd7075696ffe2e41a364a53f24bd5efc7994",
      "lines": 20
    },
    "docs/handoff/WAL018_DIAG.json": {
      "sha256": "a1842e90b5d5eb76627516462e8b88f931f055facb1fa4b67d0f2dabd062f133",
      "lines": 63
    },
    "docs/handoff/WAL018_DIAG02.json": {
      "sha256": "25b7feb39c1bc3126a3a65e1e2c27e13a8974cbf77fee9f52905fc2033a336a9",
      "lines": 90
    },
    "docs/handoff/WAL018_DIAG03.json": {
      "sha256": "a3d51f81ac3e5b9f982fab4b366fbe155d2c1799246fb35bf59ed9fb917279c8",
      "lines": 56
    },
    "docs/handoff/WAL018_GREEN.json": {
      "sha256": "8ad01b7607ea0ff336be48144535e819f7ff88022ab8f5d6e008fb0cb7a07fea",
      "lines": 193
    },
    "docs/handoff/WAL018_MOVE.py": {
      "sha256": "c9ab6ea090aa54aa74ec4a5e39708e6616566112666819747be57775b3509fdc",
      "lines": 11
    },
    "docs/handoff/WAL018_QUIET.py": {
      "sha256": "fc58d8fc881f9c307b4fa688e69761fb1e6562a3319a41bbe09a18d22cd9fbaa",
      "lines": 17
    },
    "docs/handoff/WAL018_RESTORE.py": {
      "sha256": "6e699e9d0caecee6df35ba37cff326d879fe8e38a5ca8e2d6967df990482633c",
      "lines": 9
    },
    "docs/testing/BBD-WAL-018-DIAGNOSTIC-01.md": {
      "sha256": "a00533a4dec8f97c7189356a12752fe1af21fd42768086fa5b1bcfbcae355280",
      "lines": 108
    },
    "docs/testing/BBD-WAL-018-DIAGNOSTIC-02.md": {
      "sha256": "40ef7a79e2558a0617102d9e7d3cb388e648139e483f5a4ac3c0f02344fd83aa",
      "lines": 124
    },
    "docs/testing/BBD-WAL-018-DIAGNOSTIC-03.md": {
      "sha256": "55e2b7f935ef076b93921692a928c15e1ead8ecd1344bbf58381cb559538f028",
      "lines": 91
    },
    "docs/testing/BBD-WAL-018-GREEN-01.md": {
      "sha256": "17edb16162895eed1bd4de63539cd57d2921ee9c8b0f1132de70d3e8e5294e6b",
      "lines": 266
    },
    "docs/handoff/WAL018_INTEGRATE.json": {
      "sha256": "c05236c22739ffcd5d182accfbce7edbf699e07f5fab6929c6c8e0a132e20df0",
      "lines": 65
    }
  },
  "source_commit": "e33bb63dfdb2e779b80bf3ddc541bd5090b4926a",
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
    "wallet-broker/src/account_ui.rs": "cbfaf77b02ea725422426f2b1276e738308e4a110654e56fa05f2008070b6e0e",
    "wallet-broker/src/session.rs": "9f88b3b0eca91d341a1c025bc8394a25e456350be58e99d9ac9417e31eef21af",
    "wallet-broker/Cargo.toml": "435907f00d7819bdf7dbe9ff9e367522f7f6eb8b6019dbde9e58212bbaea041b",
    "wallet-broker/Cargo.lock": "a5d53b729433a06f60edcef51e608629c7365326d06fef2bbac8f2b23f048c2a",
    "wallet-broker/tests/account_management.rs": "f1f9c70dc89e330fca6f5d292cfbabd626ef0c760a8800f522b48c070e57ddf0",
    "wallet-broker/tests/account_native_ui.rs": "67323f8889b789bcb9d01f9d26b4af8016da5bbf94d702bf8ae886ba5487ebb7",
    "package.json": "76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5",
    "package-lock.json": "0e32de43f3c1a31ae3d9e9c8b172d10cd37a03577dc7d1eaf4ac916967cfcdb8",
    "scripts/security-policy.js": "414a5645d8621b5d81cdeba7bb7461ec25647a5a1e5532295d6f75e7b8762532",
    "test/securityPolicy.node.js": "ff449a86e04570f8ff07bd280db6787c62f645f301d4235a65e825f3af7bbb9c",
    "docs/handoff/WAL018_QUIET.py": "fc58d8fc881f9c307b4fa688e69761fb1e6562a3319a41bbe09a18d22cd9fbaa",
    "docs/handoff/WAL018_RESTORE.py": "6e699e9d0caecee6df35ba37cff326d879fe8e38a5ca8e2d6967df990482633c",
    "docs/handoff/WAL018_CLEANUP.py": "1d5ad94fb001454544b0adf011e56ccd7af74a9ab89609af36a012c7015e0981",
    "docs/handoff/WAL016_RUNNER.py": "ebc5ae611b596709eda801ed3087d808a50cf79bd8cda55520eb02ccf74656f0",
    "wallet-broker/src/zec/test_support.rs": "de157d4a78c4c452e0e19122200997e6676c82099ddb25bfebe84781470f0791",
    "tickets/BBD-WAL-018.md": "10da91196dcf73d073a5e42593f38aa871e9d2350b6703f7c63398ddf4333e9b",
    "docs/handoff/CURRENT_TASK.md": "d92317c6cc35ab587538244df8bad27a6110d9f8675753e6b17f51fe58bcec6f",
    "docs/architecture/BBD-WAL-018-REVIEW.md": "946a51211fc2483292c97f4f4169d839a08c5b350657c38fcb89954b75ae9da3",
    "docs/handoff/WAL018_CLONE.py": "b3794ccc65a8d64eb6d1773696e8fd7075696ffe2e41a364a53f24bd5efc7994",
    "docs/handoff/WAL018_DIAG.json": "a1842e90b5d5eb76627516462e8b88f931f055facb1fa4b67d0f2dabd062f133",
    "docs/handoff/WAL018_DIAG02.json": "25b7feb39c1bc3126a3a65e1e2c27e13a8974cbf77fee9f52905fc2033a336a9",
    "docs/handoff/WAL018_DIAG03.json": "a3d51f81ac3e5b9f982fab4b366fbe155d2c1799246fb35bf59ed9fb917279c8",
    "docs/handoff/WAL018_GREEN.json": "8ad01b7607ea0ff336be48144535e819f7ff88022ab8f5d6e008fb0cb7a07fea",
    "docs/handoff/WAL018_MOVE.py": "c9ab6ea090aa54aa74ec4a5e39708e6616566112666819747be57775b3509fdc",
    "docs/testing/BBD-WAL-018-DIAGNOSTIC-01.md": "a00533a4dec8f97c7189356a12752fe1af21fd42768086fa5b1bcfbcae355280",
    "docs/testing/BBD-WAL-018-DIAGNOSTIC-02.md": "40ef7a79e2558a0617102d9e7d3cb388e648139e483f5a4ac3c0f02344fd83aa",
    "docs/testing/BBD-WAL-018-DIAGNOSTIC-03.md": "55e2b7f935ef076b93921692a928c15e1ead8ecd1344bbf58381cb559538f028",
    "docs/testing/BBD-WAL-018-GREEN-01.md": "17edb16162895eed1bd4de63539cd57d2921ee9c8b0f1132de70d3e8e5294e6b"
  }
}
```
