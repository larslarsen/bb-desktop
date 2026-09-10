# WAL-013 native account widgets and confirmation green

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
      "output": "bd7660786532dc0cadaba87a0bc51a582898b188\n"
    },
    {
      "argv": [
        "git",
        "status",
        "--short"
      ],
      "exit": 0,
      "output": " M package-lock.json\n M package.json\n M scripts/build-wallet-broker.js\n M scripts/security-policy.js\n M social-main.js\n M test/electronSecurity.node.js\n M test/securityPolicy.node.js\n M test/walletBrokerBuild.node.js\n M test/walletBrokerRuntime.node.js\n M test/walletStartup.node.js\n M test/walletStartupSmoke.node.js\n M test/walletSupervisor.node.js\n M test/walletSupervisorShutdown.node.js\n M wallet-broker/Cargo.toml\n M wallet-broker/src/lib.rs\n M wallet-broker/src/runtime.rs\n M wallet-broker/supervisor.js\n?? docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md\n?? docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md\n?? docs/testing/BBD-WAL-013-ACCOUNTS-GREEN-UI-RED-01.md\n?? docs/testing/BBD-WAL-013-ACCOUNTS-RED-01.md\n?? docs/testing/BBD-WAL-013-NATIVE-RUNTIME-GREEN-01.md\n?? docs/testing/BBD-WAL-013-NATIVE-UI-GREEN-01.md\n?? docs/testing/BBD-WAL-013-NATIVE-UI-GREEN-RESUME-01.md\n?? docs/testing/BBD-WAL-013-RUNTIME-JS-GREEN-01.md\n?? docs/testing/BBD-WAL-013-RUNTIME-JS-GREEN-FINISH-01.md\n?? docs/testing/BBD-WAL-013-RUNTIME-JS-RED-01.md\n?? test/fixtures/wallet-broker/x11-window.py\n?? test/walletAccountManagement.node.js\n?? test/walletAccountWindowSmoke.node.js\n?? wallet-broker/src/account_ui.rs\n?? wallet-broker/src/accounts.rs\n?? wallet-broker/tests/account_management.rs\n?? wallet-broker/tests/account_native_ui.rs\n"
    },
    {
      "argv": [
        "<home>/.cargo/bin/rustup",
        "run",
        "1.98.0",
        "rustfmt",
        "--edition",
        "2024",
        "--config",
        "skip_children=true",
        "wallet-broker/tests/account_native_ui.rs"
      ],
      "exit": 0,
      "output": ""
    },
    {
      "argv": [
        "<home>/.cargo/bin/rustup",
        "run",
        "1.98.0",
        "cargo",
        "test",
        "--manifest-path",
        "wallet-broker/Cargo.toml",
        "--locked",
        "--offline",
        "--no-default-features",
        "--features",
        "native-ui",
        "--test",
        "account_native_ui"
      ],
      "exit": 101,
      "output": "   Compiling bitbook-wallet-broker v0.1.0 (<repo>/wallet-broker)\n    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.78s\n     Running tests/account_native_ui.rs (wallet-broker/target/debug/deps/account_native_ui-6dcf24c6022d9a39)\n\nrunning 9 tests\ntest export_chooser_cancel_skips_port_and_chosen_path_reaches_port ... ok\ntest restore_requires_distinct_confirm_and_cannot_replay ... FAILED\ntest copy_cut_undo_and_platform_events_never_expose_secret ... ok\ntest unlock_wrong_then_correct_clears_password_and_lock_calls ... ok\ntest empty_and_populated_list_paints_notices_and_selection_controls ... ok\ntest window_control_reopen_quit_drop_and_closed_list_errors ... ok\ntest create_form_accepts_matching_unicode_and_rejects_invalid ... ok\ntest small_and_tall_layouts_keep_action_controls_usable ... ok\ntest shared_port_and_local_manager_create_unlock_lock_and_restart ... ok\n\nfailures:\n\n---- restore_requires_distinct_confirm_and_cannot_replay stdout ----\n\nthread 'restore_requires_distinct_confirm_and_cannot_replay' (4103686) panicked at tests/account_native_ui.rs:276:43:\nexact label must be painted\n\n\nfailures:\n    restore_requires_distinct_confirm_and_cannot_replay\n\ntest result: FAILED. 8 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 6.36s\n\nerror: test failed, to rerun pass `--test account_native_ui`\n"
    }
  ],
  "accepted": false,
  "version": "Hermes Agent v0.18.2 (2026.7.7.2) \u00b7 upstream 67764dc0 \u00b7 local 10b6d1a9 (+1 carried commit)\nInstall directory: <home>/.hermes/hermes-agent\nInstall method: git\nPython: 3.11.15\nOpenAI SDK: 2.24.0",
  "head": "bd7660786532dc0cadaba87a0bc51a582898b188",
  "initial_status": "M package-lock.json\n M package.json\n M scripts/build-wallet-broker.js\n M scripts/security-policy.js\n M social-main.js\n M test/electronSecurity.node.js\n M test/securityPolicy.node.js\n M test/walletBrokerBuild.node.js\n M test/walletBrokerRuntime.node.js\n M test/walletStartup.node.js\n M test/walletStartupSmoke.node.js\n M test/walletSupervisor.node.js\n M test/walletSupervisorShutdown.node.js\n M wallet-broker/Cargo.toml\n M wallet-broker/src/lib.rs\n M wallet-broker/src/runtime.rs\n M wallet-broker/supervisor.js\n?? docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md\n?? docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md\n?? docs/testing/BBD-WAL-013-ACCOUNTS-GREEN-UI-RED-01.md\n?? docs/testing/BBD-WAL-013-ACCOUNTS-RED-01.md\n?? docs/testing/BBD-WAL-013-NATIVE-RUNTIME-GREEN-01.md\n?? docs/testing/BBD-WAL-013-NATIVE-UI-GREEN-01.md\n?? docs/testing/BBD-WAL-013-NATIVE-UI-GREEN-RESUME-01.md\n?? docs/testing/BBD-WAL-013-RUNTIME-JS-GREEN-01.md\n?? docs/testing/BBD-WAL-013-RUNTIME-JS-GREEN-FINISH-01.md\n?? docs/testing/BBD-WAL-013-RUNTIME-JS-RED-01.md\n?? test/fixtures/wallet-broker/x11-window.py\n?? test/walletAccountManagement.node.js\n?? test/walletAccountWindowSmoke.node.js\n?? wallet-broker/src/account_ui.rs\n?? wallet-broker/src/accounts.rs\n?? wallet-broker/tests/account_management.rs\n?? wallet-broker/tests/account_native_ui.rs",
  "session": {
    "id": "20260910_112009_3f8342",
    "model": "poolside/laguna-s-2.1:free",
    "provider": "nous"
  },
  "input_hashes": {
    "social-main.js": "29fa41d88cb75d96b00bc2fe740d8701c9ddfbaf4ba2ae71be84b8755e192d30",
    "wallet-broker/supervisor.js": "1258444add1741dd6ba850c241871b8e68be71e8550dac14b0e8af6959b9f7a2",
    "scripts/build-wallet-broker.js": "4427ef3e713ec77c45daefe8d37b457ca8dc2c8529b30214dbf65628f43a1445",
    "wallet-broker/src/runtime.rs": "6788914524605f4669d20d65f7b6ef7282b0090a0696ea45f088ecc49dc1c962",
    "test/walletAccountManagement.node.js": "94a40ddbedea2298f707bb942218255ab6f55e7db9903c9401d18190f57eaaea",
    "test/walletStartup.node.js": "4a64be8628ff54e74f282703abfcaf286c9ae9cec96a15e1d13561a4fb0d501e",
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
    "wallet-broker/tests/account_native_ui.rs": "6c59850abcc28727af72afd89a7cc583ac93d656f23c9423210957a4d954906b",
    "wallet-broker/src/lib.rs": "e37713c4fd1a528a9260b8b11d740f1a317198f2eaf62a534729f4a566a10437",
    "wallet-broker/Cargo.toml": "c4e5700db87b30cd2da488fcc68c7305f8b59e2dd1b16f10da2fd6e59ab28cda",
    "wallet-broker/Cargo.lock": "b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71",
    "wallet-broker/src/vault.rs": "f23247b0e46c68dfc125804fa981d16f7022464e1e4d94a8c1e62771a045be49",
    "wallet-broker/src/store.rs": "611d837641069a98d05b9e68c14bf11a37a5076de58bf6516188870eeab19236",
    "wallet-broker/src/session.rs": "42e4f335bb4080ad530d93dcc04d824b4ab54835be7f6c7cd68feba3f20ee227",
    "wallet-broker/src/native.rs": "a0ccb0b11e3e636cc28f9576ec0cf6da2d8c2ef5344370e5a7a2a9a9b5cc32e5",
    "wallet-broker/src/native_ui.rs": "c6607af3f36278f8d7deeb2b4276017177bbb4c695017f18bf4d2bcb412b268f",
    "wallet-broker/src/account_ui.rs": "03044acaea68d6ddbc2dbe91039e7ffd425c110c29aaa5f88011578a653cd92f",
    "wallet-broker/src/native_ui/zec_native_app_tests.rs": "d1d405f8667c6c9d184104d2023ffcff209f6d013e3920dd5c5ccd7e5a2c40f0",
    "wallet-broker/src/native_ui/zec_review_tests.rs": "2eec3b60eb1b9fbd32235832a867d1189928126ca79373fa9f4fa7fdf2ee87bf",
    "test/walletBrokerRuntime.node.js": "989c5f64716d22e8b2903d6bf736fdadcae253d9ed11c6c648aec0fc2de14592",
    "test/walletStartupSmoke.node.js": "a1c75efee5fcfdbeb9bf56dfc64f0e747ab6aa527d30ce416c5027bc2aa2a1ef",
    "test/walletAccountWindowSmoke.node.js": "e4bc03f16d5c545c73298c02242011d70af1051a90c9dea29be1305aa9b381bd",
    "test/fixtures/wallet-broker/x11-window.py": "040d8214487216e6b91ec514c9473e95cd66811c26306792f5ca7b081696b5d4"
  },
  "format_diffs": {
    "wallet-broker/tests/account_native_ui.rs": ""
  },
  "formatted_input_hashes": {
    "social-main.js": "29fa41d88cb75d96b00bc2fe740d8701c9ddfbaf4ba2ae71be84b8755e192d30",
    "wallet-broker/supervisor.js": "1258444add1741dd6ba850c241871b8e68be71e8550dac14b0e8af6959b9f7a2",
    "scripts/build-wallet-broker.js": "4427ef3e713ec77c45daefe8d37b457ca8dc2c8529b30214dbf65628f43a1445",
    "wallet-broker/src/runtime.rs": "6788914524605f4669d20d65f7b6ef7282b0090a0696ea45f088ecc49dc1c962",
    "test/walletAccountManagement.node.js": "94a40ddbedea2298f707bb942218255ab6f55e7db9903c9401d18190f57eaaea",
    "test/walletStartup.node.js": "4a64be8628ff54e74f282703abfcaf286c9ae9cec96a15e1d13561a4fb0d501e",
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
    "wallet-broker/tests/account_native_ui.rs": "6c59850abcc28727af72afd89a7cc583ac93d656f23c9423210957a4d954906b",
    "wallet-broker/src/lib.rs": "e37713c4fd1a528a9260b8b11d740f1a317198f2eaf62a534729f4a566a10437",
    "wallet-broker/Cargo.toml": "c4e5700db87b30cd2da488fcc68c7305f8b59e2dd1b16f10da2fd6e59ab28cda",
    "wallet-broker/Cargo.lock": "b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71",
    "wallet-broker/src/vault.rs": "f23247b0e46c68dfc125804fa981d16f7022464e1e4d94a8c1e62771a045be49",
    "wallet-broker/src/store.rs": "611d837641069a98d05b9e68c14bf11a37a5076de58bf6516188870eeab19236",
    "wallet-broker/src/session.rs": "42e4f335bb4080ad530d93dcc04d824b4ab54835be7f6c7cd68feba3f20ee227",
    "wallet-broker/src/native.rs": "a0ccb0b11e3e636cc28f9576ec0cf6da2d8c2ef5344370e5a7a2a9a9b5cc32e5",
    "wallet-broker/src/native_ui.rs": "c6607af3f36278f8d7deeb2b4276017177bbb4c695017f18bf4d2bcb412b268f",
    "wallet-broker/src/account_ui.rs": "03044acaea68d6ddbc2dbe91039e7ffd425c110c29aaa5f88011578a653cd92f",
    "wallet-broker/src/native_ui/zec_native_app_tests.rs": "d1d405f8667c6c9d184104d2023ffcff209f6d013e3920dd5c5ccd7e5a2c40f0",
    "wallet-broker/src/native_ui/zec_review_tests.rs": "2eec3b60eb1b9fbd32235832a867d1189928126ca79373fa9f4fa7fdf2ee87bf",
    "test/walletBrokerRuntime.node.js": "989c5f64716d22e8b2903d6bf736fdadcae253d9ed11c6c648aec0fc2de14592",
    "test/walletStartupSmoke.node.js": "a1c75efee5fcfdbeb9bf56dfc64f0e747ab6aa527d30ce416c5027bc2aa2a1ef",
    "test/walletAccountWindowSmoke.node.js": "e4bc03f16d5c545c73298c02242011d70af1051a90c9dea29be1305aa9b381bd",
    "test/fixtures/wallet-broker/x11-window.py": "040d8214487216e6b91ec514c9473e95cd66811c26306792f5ca7b081696b5d4"
  },
  "stop": "native UI not green",
  "final_hashes": {
    "social-main.js": "29fa41d88cb75d96b00bc2fe740d8701c9ddfbaf4ba2ae71be84b8755e192d30",
    "wallet-broker/supervisor.js": "1258444add1741dd6ba850c241871b8e68be71e8550dac14b0e8af6959b9f7a2",
    "scripts/build-wallet-broker.js": "4427ef3e713ec77c45daefe8d37b457ca8dc2c8529b30214dbf65628f43a1445",
    "wallet-broker/src/runtime.rs": "6788914524605f4669d20d65f7b6ef7282b0090a0696ea45f088ecc49dc1c962",
    "test/walletAccountManagement.node.js": "94a40ddbedea2298f707bb942218255ab6f55e7db9903c9401d18190f57eaaea",
    "test/walletStartup.node.js": "4a64be8628ff54e74f282703abfcaf286c9ae9cec96a15e1d13561a4fb0d501e",
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
    "wallet-broker/tests/account_native_ui.rs": "6c59850abcc28727af72afd89a7cc583ac93d656f23c9423210957a4d954906b",
    "wallet-broker/src/lib.rs": "e37713c4fd1a528a9260b8b11d740f1a317198f2eaf62a534729f4a566a10437",
    "wallet-broker/Cargo.toml": "c4e5700db87b30cd2da488fcc68c7305f8b59e2dd1b16f10da2fd6e59ab28cda",
    "wallet-broker/Cargo.lock": "b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71",
    "wallet-broker/src/vault.rs": "f23247b0e46c68dfc125804fa981d16f7022464e1e4d94a8c1e62771a045be49",
    "wallet-broker/src/store.rs": "611d837641069a98d05b9e68c14bf11a37a5076de58bf6516188870eeab19236",
    "wallet-broker/src/session.rs": "42e4f335bb4080ad530d93dcc04d824b4ab54835be7f6c7cd68feba3f20ee227",
    "wallet-broker/src/native.rs": "a0ccb0b11e3e636cc28f9576ec0cf6da2d8c2ef5344370e5a7a2a9a9b5cc32e5",
    "wallet-broker/src/native_ui.rs": "c6607af3f36278f8d7deeb2b4276017177bbb4c695017f18bf4d2bcb412b268f",
    "wallet-broker/src/account_ui.rs": "03044acaea68d6ddbc2dbe91039e7ffd425c110c29aaa5f88011578a653cd92f",
    "wallet-broker/src/native_ui/zec_native_app_tests.rs": "d1d405f8667c6c9d184104d2023ffcff209f6d013e3920dd5c5ccd7e5a2c40f0",
    "wallet-broker/src/native_ui/zec_review_tests.rs": "2eec3b60eb1b9fbd32235832a867d1189928126ca79373fa9f4fa7fdf2ee87bf",
    "test/walletBrokerRuntime.node.js": "989c5f64716d22e8b2903d6bf736fdadcae253d9ed11c6c648aec0fc2de14592",
    "test/walletStartupSmoke.node.js": "a1c75efee5fcfdbeb9bf56dfc64f0e747ab6aa527d30ce416c5027bc2aa2a1ef",
    "test/walletAccountWindowSmoke.node.js": "e4bc03f16d5c545c73298c02242011d70af1051a90c9dea29be1305aa9b381bd",
    "test/fixtures/wallet-broker/x11-window.py": "040d8214487216e6b91ec514c9473e95cd66811c26306792f5ca7b081696b5d4"
  }
}
```
