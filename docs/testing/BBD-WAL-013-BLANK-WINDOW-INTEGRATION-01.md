# WAL-013 blank-window correction integration and committed secret scan

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
      "output": "Hermes Agent v0.18.2 (2026.7.7.2) \u00b7 upstream 67764dc0 \u00b7 local 10b6d1a9 (+1 carried commit)\nInstall directory: <home>/.hermes/hermes-agent\nInstall method: git\nPython: 3.11.15\nOpenAI SDK: 2.24.0\n"
    },
    {
      "argv": [
        "git",
        "rev-parse",
        "HEAD"
      ],
      "exit": 0,
      "output": "90d259f9e6d563a97e142b78c8691a0526ca3bd2\n"
    },
    {
      "argv": [
        "git",
        "status",
        "--short"
      ],
      "exit": 0,
      "output": " M package-lock.json\n M package.json\n M scripts/security-policy.js\n M social-main.js\n M test/fixtures/wallet-broker/x11-window.py\n M test/securityPolicy.node.js\n M test/walletAccountManagement.node.js\n M test/walletAccountWindowSmoke.node.js\n M test/walletStartup.node.js\n M wallet-broker/supervisor.js\n?? docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md\n?? docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md\n?? docs/testing/BBD-WAL-013-BLANK-WINDOW-GREEN-01.md\n?? docs/testing/BBD-WAL-013-BLANK-WINDOW-GREEN-02.md\n?? docs/testing/BBD-WAL-013-BLANK-WINDOW-RED-01.md\n?? docs/testing/BBD-WAL-013-HOST-SOFTWARE-DIAGNOSTIC-01.md\n?? docs/testing/BBD-WAL-013-HOST-WINDOW-DIAGNOSTIC-01.md\n?? docs/testing/BBD-WAL-013-HOST-WINDOW-DIAGNOSTIC-02.md\n"
    },
    {
      "argv": [
        "git",
        "branch",
        "--show-current"
      ],
      "exit": 0,
      "output": "master\n"
    },
    {
      "argv": [
        "git",
        "diff",
        "--cached",
        "--name-only"
      ],
      "exit": 0,
      "output": ""
    },
    {
      "argv": [
        "git",
        "remote",
        "get-url",
        "origin"
      ],
      "exit": 0,
      "output": "https://github.com/larslarsen/bb-desktop.git\n"
    },
    {
      "argv": [
        "git",
        "diff",
        "--check"
      ],
      "exit": 0,
      "output": ""
    },
    {
      "argv": [
        "git",
        "add",
        "--",
        "social-main.js",
        "wallet-broker/supervisor.js",
        "test/walletAccountManagement.node.js",
        "test/walletStartup.node.js",
        "test/walletAccountWindowSmoke.node.js",
        "test/fixtures/wallet-broker/x11-window.py",
        "docs/testing/BBD-WAL-013-BLANK-WINDOW-RED-01.md",
        "docs/testing/BBD-WAL-013-BLANK-WINDOW-GREEN-01.md",
        "docs/testing/BBD-WAL-013-BLANK-WINDOW-GREEN-02.md",
        "docs/testing/BBD-WAL-013-HOST-WINDOW-DIAGNOSTIC-01.md",
        "docs/testing/BBD-WAL-013-HOST-WINDOW-DIAGNOSTIC-02.md",
        "docs/testing/BBD-WAL-013-HOST-SOFTWARE-DIAGNOSTIC-01.md"
      ],
      "exit": 0,
      "output": ""
    },
    {
      "argv": [
        "git",
        "diff",
        "--cached",
        "--name-only"
      ],
      "exit": 0,
      "output": "docs/testing/BBD-WAL-013-BLANK-WINDOW-GREEN-01.md\ndocs/testing/BBD-WAL-013-BLANK-WINDOW-GREEN-02.md\ndocs/testing/BBD-WAL-013-BLANK-WINDOW-RED-01.md\ndocs/testing/BBD-WAL-013-HOST-SOFTWARE-DIAGNOSTIC-01.md\ndocs/testing/BBD-WAL-013-HOST-WINDOW-DIAGNOSTIC-01.md\ndocs/testing/BBD-WAL-013-HOST-WINDOW-DIAGNOSTIC-02.md\nsocial-main.js\ntest/fixtures/wallet-broker/x11-window.py\ntest/walletAccountManagement.node.js\ntest/walletAccountWindowSmoke.node.js\ntest/walletStartup.node.js\nwallet-broker/supervisor.js\n"
    },
    {
      "argv": [
        "git",
        "diff",
        "--cached",
        "--check"
      ],
      "exit": 0,
      "output": ""
    },
    {
      "argv": [
        "git",
        "commit",
        "-m",
        "Fix native wallet windows on Wayland and NVIDIA desktops"
      ],
      "exit": 0,
      "output": "[master 25113d5db] Fix native wallet windows on Wayland and NVIDIA desktops\n 12 files changed, 1473 insertions(+), 27 deletions(-)\n create mode 100644 docs/testing/BBD-WAL-013-BLANK-WINDOW-GREEN-01.md\n create mode 100644 docs/testing/BBD-WAL-013-BLANK-WINDOW-GREEN-02.md\n create mode 100644 docs/testing/BBD-WAL-013-BLANK-WINDOW-RED-01.md\n create mode 100644 docs/testing/BBD-WAL-013-HOST-SOFTWARE-DIAGNOSTIC-01.md\n create mode 100644 docs/testing/BBD-WAL-013-HOST-WINDOW-DIAGNOSTIC-01.md\n create mode 100644 docs/testing/BBD-WAL-013-HOST-WINDOW-DIAGNOSTIC-02.md\n"
    },
    {
      "argv": [
        "git",
        "rev-parse",
        "HEAD"
      ],
      "exit": 0,
      "output": "25113d5dbf9bc72c058c9f22e9398afde1576f24\n"
    },
    {
      "argv": [
        "git",
        "diff-tree",
        "--no-commit-id",
        "--name-only",
        "-r",
        "HEAD"
      ],
      "exit": 0,
      "output": "docs/testing/BBD-WAL-013-BLANK-WINDOW-GREEN-01.md\ndocs/testing/BBD-WAL-013-BLANK-WINDOW-GREEN-02.md\ndocs/testing/BBD-WAL-013-BLANK-WINDOW-RED-01.md\ndocs/testing/BBD-WAL-013-HOST-SOFTWARE-DIAGNOSTIC-01.md\ndocs/testing/BBD-WAL-013-HOST-WINDOW-DIAGNOSTIC-01.md\ndocs/testing/BBD-WAL-013-HOST-WINDOW-DIAGNOSTIC-02.md\nsocial-main.js\ntest/fixtures/wallet-broker/x11-window.py\ntest/walletAccountManagement.node.js\ntest/walletAccountWindowSmoke.node.js\ntest/walletStartup.node.js\nwallet-broker/supervisor.js\n"
    },
    {
      "argv": [
        "target/security-tools/gitleaks-v8.30.1/gitleaks",
        "git",
        "--redact=100",
        "--no-banner",
        "."
      ],
      "exit": 0,
      "output": "1:00PM INF 5335 commits scanned.\n1:00PM INF scanned ~33817236 bytes (33.82 MB) in 2.17s\n1:00PM INF no leaks found\n"
    },
    {
      "argv": [
        "git",
        "push",
        "origin",
        "master"
      ],
      "exit": 0,
      "output": "To https://github.com/larslarsen/bb-desktop.git\n   c786041a9..25113d5db  master -> master\n"
    },
    {
      "argv": [
        "git",
        "ls-remote",
        "origin",
        "refs/heads/master"
      ],
      "exit": 0,
      "output": "25113d5dbf9bc72c058c9f22e9398afde1576f24\trefs/heads/master\n"
    }
  ],
  "accepted": true,
  "version": "Hermes Agent v0.18.2 (2026.7.7.2) \u00b7 upstream 67764dc0 \u00b7 local 10b6d1a9 (+1 carried commit)\nInstall directory: <home>/.hermes/hermes-agent\nInstall method: git\nPython: 3.11.15\nOpenAI SDK: 2.24.0",
  "head": "90d259f9e6d563a97e142b78c8691a0526ca3bd2",
  "initial_status": "M package-lock.json\n M package.json\n M scripts/security-policy.js\n M social-main.js\n M test/fixtures/wallet-broker/x11-window.py\n M test/securityPolicy.node.js\n M test/walletAccountManagement.node.js\n M test/walletAccountWindowSmoke.node.js\n M test/walletStartup.node.js\n M wallet-broker/supervisor.js\n?? docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md\n?? docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md\n?? docs/testing/BBD-WAL-013-BLANK-WINDOW-GREEN-01.md\n?? docs/testing/BBD-WAL-013-BLANK-WINDOW-GREEN-02.md\n?? docs/testing/BBD-WAL-013-BLANK-WINDOW-RED-01.md\n?? docs/testing/BBD-WAL-013-HOST-SOFTWARE-DIAGNOSTIC-01.md\n?? docs/testing/BBD-WAL-013-HOST-WINDOW-DIAGNOSTIC-01.md\n?? docs/testing/BBD-WAL-013-HOST-WINDOW-DIAGNOSTIC-02.md",
  "session": {
    "id": "20260910_125948_6dd871",
    "model": "meituan/longcat-2.0:free",
    "provider": "nous"
  },
  "input_hashes": {
    "social-main.js": "d959e6adffc2579fa3f2f447f60a165f2b6bcdc279afde2f6ddaa91bc9d9aa4a",
    "wallet-broker/supervisor.js": "115e0e51425e7b25d8f6178b206f592165cb14dacde8cb544b970839833c4721",
    "scripts/build-wallet-broker.js": "4427ef3e713ec77c45daefe8d37b457ca8dc2c8529b30214dbf65628f43a1445",
    "wallet-broker/src/runtime.rs": "6788914524605f4669d20d65f7b6ef7282b0090a0696ea45f088ecc49dc1c962",
    "test/walletAccountManagement.node.js": "6a449335e1101b8dd6340d640514e8fc083a09235210c17dd582b57230de9475",
    "test/walletStartup.node.js": "448fb88268655b979c7c724ab21d13d09fee29a219628cd70f576daae4589de1",
    "test/walletBrokerBuild.node.js": "b1ac8ae3a1ca76c9580c92fec7cd1cbca0edb482246f072afcafb22a79e689de",
    "test/walletSupervisorShutdown.node.js": "f1156d8bf81a03a40ec9eb235666275ce85a3394b12f7b567bbb9da20a7543ef",
    "test/walletSupervisorTransport.node.js": "e69fdcfa4aaeeca03b4abf8ac100888ff0b42fbb6001f53e461a35a5b7550700",
    "test/walletSupervisor.node.js": "7fcb775d2db7ae8f28b6d9f7660c44a056e59891f3e552db80a248d843c76aa4",
    "test/electronSecurity.node.js": "d70d6337ffbe97dfeee1894895c757b804dc18ae819219e8742adda5358ab482",
    "test/walletPreload.node.js": "60b151344a01776e1d7f38238f69534426883503f9d627466bac7acbf4dc4f9e",
    "test/walletBrokerLaunchConfig.node.js": "90acd32c9863df0206364eb04f707d92a5979ee66c7e15b2ec7b8250a5fc2113",
    "wallet-preload.js": "3e6a18acf88dd5be4e6a88f326d6ace7a8071066480d9a70a2e8f89df035a1df",
    "wallet-broker/protocol.js": "79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4",
    "wallet-pay/model.js": "acf07238366f3e28253be8c9208fbb27e13e9c2687d374d1a6ae87e0d173fa5e",
    "wallet-broker/launch-config.js": "68adf92e2d543e5d51353477b1944c35d4189c5590ac581d3289f1bd72898af8",
    "package.json": "76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5",
    "package-lock.json": "0e32de43f3c1a31ae3d9e9c8b172d10cd37a03577dc7d1eaf4ac916967cfcdb8",
    "scripts/security-policy.js": "fec10759a73a3c675d8206da454abf4b0e8dca6b62da463f39a2db27b30dc078",
    "test/securityPolicy.node.js": "a58d85e5322f0cb01a78b636454f92547bd98e556013cc07712dc0cf64ea565c",
    "wallet-broker/src/accounts.rs": "f79eb3286cdccae2058e099c2d08b8e5a7167477a687e36609f91df4dc5b51f1",
    "wallet-broker/tests/account_management.rs": "7f876b1a9b7c4e65f8a6225d29867b2ae7dc4902f10cbbb829b4012675dff0d8",
    "wallet-broker/tests/account_native_ui.rs": "95584435ad547752481bfb2ebefbb83d734a6a0edbffb63a4ef8ec0681d166d5",
    "wallet-broker/src/lib.rs": "e37713c4fd1a528a9260b8b11d740f1a317198f2eaf62a534729f4a566a10437",
    "wallet-broker/Cargo.toml": "73e5e585eb2fd1ca867d66962460cfa010443886a4b59d8a33556ef2a4ff3503",
    "wallet-broker/Cargo.lock": "b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71",
    "wallet-broker/src/vault.rs": "f23247b0e46c68dfc125804fa981d16f7022464e1e4d94a8c1e62771a045be49",
    "wallet-broker/src/store.rs": "611d837641069a98d05b9e68c14bf11a37a5076de58bf6516188870eeab19236",
    "wallet-broker/src/session.rs": "42e4f335bb4080ad530d93dcc04d824b4ab54835be7f6c7cd68feba3f20ee227",
    "wallet-broker/src/native.rs": "a0ccb0b11e3e636cc28f9576ec0cf6da2d8c2ef5344370e5a7a2a9a9b5cc32e5",
    "wallet-broker/src/native_ui.rs": "c6607af3f36278f8d7deeb2b4276017177bbb4c695017f18bf4d2bcb412b268f",
    "wallet-broker/src/account_ui.rs": "f8a47778d00be2cd7a448aec33117172880bde6e70e220b9f783dcca1ad8c1c7",
    "wallet-broker/src/native_ui/zec_native_app_tests.rs": "d1d405f8667c6c9d184104d2023ffcff209f6d013e3920dd5c5ccd7e5a2c40f0",
    "wallet-broker/src/native_ui/zec_review_tests.rs": "2eec3b60eb1b9fbd32235832a867d1189928126ca79373fa9f4fa7fdf2ee87bf",
    "test/walletBrokerRuntime.node.js": "989c5f64716d22e8b2903d6bf736fdadcae253d9ed11c6c648aec0fc2de14592",
    "test/walletStartupSmoke.node.js": "a1c75efee5fcfdbeb9bf56dfc64f0e747ab6aa527d30ce416c5027bc2aa2a1ef",
    "test/walletAccountWindowSmoke.node.js": "3590096cafed5596378488e8ebca8c916e62352d4410e5748a2033ab0ec9fa3a",
    "test/fixtures/wallet-broker/x11-window.py": "d192f036781e5623b88a8ba3f31b9ee7c781f7bdffa46982aa431f8ada5330d3",
    "wallet-broker/target/wal011-runtime-package-red-01.json": "5608cd092ae81d83e658aa9231e2751378f5f15fbde3a60c2b9dc176d5a370bd",
    "docs/testing/BBD-WAL-013-BLANK-WINDOW-RED-01.md": "cc484ec6e0db26767685d5a3e27ced16809a971b024d0192c8ac4c67fe0f55c4",
    "docs/testing/BBD-WAL-013-BLANK-WINDOW-GREEN-01.md": "4348ede94df977d023cbcff96265dc9072c49366c75e9fb9c7b024818e34e2b3",
    "docs/testing/BBD-WAL-013-BLANK-WINDOW-GREEN-02.md": "2ff17248b8f57e00325b75ae0ef5d7d0bd89a9fe4ba80a3acf0828285647934b",
    "docs/testing/BBD-WAL-013-HOST-WINDOW-DIAGNOSTIC-01.md": "d288a0d090cd80c6c448dc52c4ac58cb4a7b7d33312bc979150b8e37b6b017c7",
    "docs/testing/BBD-WAL-013-HOST-WINDOW-DIAGNOSTIC-02.md": "3f48415d5b4ed1e4326e6ccbeb1776022b628b9b8d471f42725c9972c14a1bbc",
    "docs/testing/BBD-WAL-013-HOST-SOFTWARE-DIAGNOSTIC-01.md": "c7b45b958df6a01ed158feb2cc6366dbda2055a53aca077ec6be3811ee87e00c",
    "wallet-broker/target/wal013-blank-window-green-02.json": "eb63c8e3fbd69b72796c88fb6cf257e863f8819925ff7002a92303b8cd10ead0"
  },
  "source_commit": "25113d5dbf9bc72c058c9f22e9398afde1576f24",
  "source_pushed": true,
  "final_hashes": {
    "social-main.js": "d959e6adffc2579fa3f2f447f60a165f2b6bcdc279afde2f6ddaa91bc9d9aa4a",
    "wallet-broker/supervisor.js": "115e0e51425e7b25d8f6178b206f592165cb14dacde8cb544b970839833c4721",
    "scripts/build-wallet-broker.js": "4427ef3e713ec77c45daefe8d37b457ca8dc2c8529b30214dbf65628f43a1445",
    "wallet-broker/src/runtime.rs": "6788914524605f4669d20d65f7b6ef7282b0090a0696ea45f088ecc49dc1c962",
    "test/walletAccountManagement.node.js": "6a449335e1101b8dd6340d640514e8fc083a09235210c17dd582b57230de9475",
    "test/walletStartup.node.js": "448fb88268655b979c7c724ab21d13d09fee29a219628cd70f576daae4589de1",
    "test/walletBrokerBuild.node.js": "b1ac8ae3a1ca76c9580c92fec7cd1cbca0edb482246f072afcafb22a79e689de",
    "test/walletSupervisorShutdown.node.js": "f1156d8bf81a03a40ec9eb235666275ce85a3394b12f7b567bbb9da20a7543ef",
    "test/walletSupervisorTransport.node.js": "e69fdcfa4aaeeca03b4abf8ac100888ff0b42fbb6001f53e461a35a5b7550700",
    "test/walletSupervisor.node.js": "7fcb775d2db7ae8f28b6d9f7660c44a056e59891f3e552db80a248d843c76aa4",
    "test/electronSecurity.node.js": "d70d6337ffbe97dfeee1894895c757b804dc18ae819219e8742adda5358ab482",
    "test/walletPreload.node.js": "60b151344a01776e1d7f38238f69534426883503f9d627466bac7acbf4dc4f9e",
    "test/walletBrokerLaunchConfig.node.js": "90acd32c9863df0206364eb04f707d92a5979ee66c7e15b2ec7b8250a5fc2113",
    "wallet-preload.js": "3e6a18acf88dd5be4e6a88f326d6ace7a8071066480d9a70a2e8f89df035a1df",
    "wallet-broker/protocol.js": "79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4",
    "wallet-pay/model.js": "acf07238366f3e28253be8c9208fbb27e13e9c2687d374d1a6ae87e0d173fa5e",
    "wallet-broker/launch-config.js": "68adf92e2d543e5d51353477b1944c35d4189c5590ac581d3289f1bd72898af8",
    "package.json": "76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5",
    "package-lock.json": "0e32de43f3c1a31ae3d9e9c8b172d10cd37a03577dc7d1eaf4ac916967cfcdb8",
    "scripts/security-policy.js": "fec10759a73a3c675d8206da454abf4b0e8dca6b62da463f39a2db27b30dc078",
    "test/securityPolicy.node.js": "a58d85e5322f0cb01a78b636454f92547bd98e556013cc07712dc0cf64ea565c",
    "wallet-broker/src/accounts.rs": "f79eb3286cdccae2058e099c2d08b8e5a7167477a687e36609f91df4dc5b51f1",
    "wallet-broker/tests/account_management.rs": "7f876b1a9b7c4e65f8a6225d29867b2ae7dc4902f10cbbb829b4012675dff0d8",
    "wallet-broker/tests/account_native_ui.rs": "95584435ad547752481bfb2ebefbb83d734a6a0edbffb63a4ef8ec0681d166d5",
    "wallet-broker/src/lib.rs": "e37713c4fd1a528a9260b8b11d740f1a317198f2eaf62a534729f4a566a10437",
    "wallet-broker/Cargo.toml": "73e5e585eb2fd1ca867d66962460cfa010443886a4b59d8a33556ef2a4ff3503",
    "wallet-broker/Cargo.lock": "b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71",
    "wallet-broker/src/vault.rs": "f23247b0e46c68dfc125804fa981d16f7022464e1e4d94a8c1e62771a045be49",
    "wallet-broker/src/store.rs": "611d837641069a98d05b9e68c14bf11a37a5076de58bf6516188870eeab19236",
    "wallet-broker/src/session.rs": "42e4f335bb4080ad530d93dcc04d824b4ab54835be7f6c7cd68feba3f20ee227",
    "wallet-broker/src/native.rs": "a0ccb0b11e3e636cc28f9576ec0cf6da2d8c2ef5344370e5a7a2a9a9b5cc32e5",
    "wallet-broker/src/native_ui.rs": "c6607af3f36278f8d7deeb2b4276017177bbb4c695017f18bf4d2bcb412b268f",
    "wallet-broker/src/account_ui.rs": "f8a47778d00be2cd7a448aec33117172880bde6e70e220b9f783dcca1ad8c1c7",
    "wallet-broker/src/native_ui/zec_native_app_tests.rs": "d1d405f8667c6c9d184104d2023ffcff209f6d013e3920dd5c5ccd7e5a2c40f0",
    "wallet-broker/src/native_ui/zec_review_tests.rs": "2eec3b60eb1b9fbd32235832a867d1189928126ca79373fa9f4fa7fdf2ee87bf",
    "test/walletBrokerRuntime.node.js": "989c5f64716d22e8b2903d6bf736fdadcae253d9ed11c6c648aec0fc2de14592",
    "test/walletStartupSmoke.node.js": "a1c75efee5fcfdbeb9bf56dfc64f0e747ab6aa527d30ce416c5027bc2aa2a1ef",
    "test/walletAccountWindowSmoke.node.js": "3590096cafed5596378488e8ebca8c916e62352d4410e5748a2033ab0ec9fa3a",
    "test/fixtures/wallet-broker/x11-window.py": "d192f036781e5623b88a8ba3f31b9ee7c781f7bdffa46982aa431f8ada5330d3",
    "wallet-broker/target/wal011-runtime-package-red-01.json": "5608cd092ae81d83e658aa9231e2751378f5f15fbde3a60c2b9dc176d5a370bd",
    "docs/testing/BBD-WAL-013-BLANK-WINDOW-RED-01.md": "cc484ec6e0db26767685d5a3e27ced16809a971b024d0192c8ac4c67fe0f55c4",
    "docs/testing/BBD-WAL-013-BLANK-WINDOW-GREEN-01.md": "4348ede94df977d023cbcff96265dc9072c49366c75e9fb9c7b024818e34e2b3",
    "docs/testing/BBD-WAL-013-BLANK-WINDOW-GREEN-02.md": "2ff17248b8f57e00325b75ae0ef5d7d0bd89a9fe4ba80a3acf0828285647934b",
    "docs/testing/BBD-WAL-013-HOST-WINDOW-DIAGNOSTIC-01.md": "d288a0d090cd80c6c448dc52c4ac58cb4a7b7d33312bc979150b8e37b6b017c7",
    "docs/testing/BBD-WAL-013-HOST-WINDOW-DIAGNOSTIC-02.md": "3f48415d5b4ed1e4326e6ccbeb1776022b628b9b8d471f42725c9972c14a1bbc",
    "docs/testing/BBD-WAL-013-HOST-SOFTWARE-DIAGNOSTIC-01.md": "c7b45b958df6a01ed158feb2cc6366dbda2055a53aca077ec6be3811ee87e00c",
    "wallet-broker/target/wal013-blank-window-green-02.json": "eb63c8e3fbd69b72796c88fb6cf257e863f8819925ff7002a92303b8cd10ead0"
  }
}
```
