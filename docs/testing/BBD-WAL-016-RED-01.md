# WAL016 tests-first RED

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
      "output": "Hermes Agent v0.18.2 (2026.7.7.2) \u00b7 upstream 564aef29 \u00b7 local 10b6d1a9 (+1 carried commit)\nInstall directory: <home>/.hermes/hermes-agent\nInstall method: git\nPython: 3.11.15\nOpenAI SDK: 2.24.0\n",
      "timeout": false
    },
    {
      "argv": [
        "git",
        "rev-parse",
        "HEAD"
      ],
      "exit": 0,
      "output": "4cae91cb59635213ceecbae075c482ad8949aab2\n",
      "timeout": false
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
        "account_management",
        "wal016_"
      ],
      "exit": 101,
      "output": "   Compiling bitbook-wallet-broker v0.1.0 (<repo>/wallet-broker)\nerror[E0599]: no associated function or constant named `controlled_empty_testnet` found for struct `bitbook_wallet_broker::zec::test_support::RecordedLiveSource` in the current scope\n    --> tests/account_management.rs:2143:38\n     |\n2143 |     let source = RecordedLiveSource::controlled_empty_testnet();\n     |                                      ^^^^^^^^^^^^^^^^^^^^^^^^ associated function or constant not found in `bitbook_wallet_broker::zec::test_support::RecordedLiveSource`\n     |\nnote: if you're trying to build a new `bitbook_wallet_broker::zec::test_support::RecordedLiveSource` consider using one of the following associated functions:\n      bitbook_wallet_broker::zec::test_support::RecordedLiveSource::canonical\n      bitbook_wallet_broker::zec::test_support::RecordedLiveSource::one_block_reorg\n      bitbook_wallet_broker::zec::test_support::RecordedLiveSource::blocking_testnet_metadata\n      bitbook_wallet_broker::zec::test_support::RecordedLiveSource::fork_beyond_retained_checkpoint\n    --> src/zec/test_support.rs:4870:5\n     |\n4870 |     pub fn canonical(f: &FrozenFixture) -> Result<Self, ZecError> {\n     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n...\n4873 |     pub fn one_block_reorg(f: &FrozenFixture) -> Result<Self, ZecError> {\n     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n...\n4926 |     pub fn blocking_testnet_metadata() -> Self {\n     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n...\n4940 |     pub fn fork_beyond_retained_checkpoint(depth: usize) -> Self {\n     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\nFor more information about this error, try `rustc --explain E0599`.\nerror: could not compile `bitbook-wallet-broker` (test \"account_management\") due to 1 previous error\n",
      "timeout": false
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
        "account_native_ui",
        "wal016_"
      ],
      "exit": 101,
      "output": "   Compiling bitbook-wallet-broker v0.1.0 (<repo>/wallet-broker)\n    Finished `test` profile [unoptimized + debuginfo] target(s) in 4.32s\n     Running tests/account_native_ui.rs (wallet-broker/target/debug/deps/account_native_ui-e7381b3c5844547a)\n\nrunning 1 test\ntest wal016_idle_locked_sync_stays_visible_and_inline_unlock_reveals_the_same_completed_job ... FAILED\n\nfailures:\n\n---- wal016_idle_locked_sync_stays_visible_and_inline_unlock_reveals_the_same_completed_job stdout ----\n\nthread 'wal016_idle_locked_sync_stays_visible_and_inline_unlock_reveals_the_same_completed_job' (116800) panicked at tests/account_native_ui.rs:317:43:\nexact label must be painted\nnote: run with `RUST_BACKTRACE=1` environment variable to display a backtrace\n\n\nfailures:\n    wal016_idle_locked_sync_stays_visible_and_inline_unlock_reveals_the_same_completed_job\n\ntest result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 14 filtered out; finished in 0.06s\n\nerror: test failed, to rerun pass `--test account_native_ui`\n",
      "timeout": false
    }
  ],
  "session": {
    "id": "20260910_193513_ba76b0",
    "model": "meituan/longcat-2.0:free",
    "provider": "nous"
  },
  "version": "Hermes Agent v0.18.2 (2026.7.7.2) \u00b7 upstream 564aef29 \u00b7 local 10b6d1a9 (+1 carried commit)\nInstall directory: <home>/.hermes/hermes-agent\nInstall method: git\nPython: 3.11.15\nOpenAI SDK: 2.24.0",
  "head": "4cae91cb59635213ceecbae075c482ad8949aab2",
  "input_hashes": {
    "wallet-broker/src/accounts.rs": "f386f28f44c12bbdf1015713e4c736e81a86f8c0a27024f376b18a9eb8751d2f",
    "wallet-broker/src/account_ui.rs": "ae8aa3b7bd14390244c5e73475195c33a1ba69079ac9ba6b0d86d869941fda75",
    "wallet-broker/src/zec/test_support.rs": "7b4dad2fef0c505f610bdcfe7b87b9075bde0673c8713e34ed57b7714326ce57",
    "wallet-broker/src/session.rs": "9f88b3b0eca91d341a1c025bc8394a25e456350be58e99d9ac9417e31eef21af",
    "wallet-broker/Cargo.toml": "435907f00d7819bdf7dbe9ff9e367522f7f6eb8b6019dbde9e58212bbaea041b",
    "wallet-broker/Cargo.lock": "a5d53b729433a06f60edcef51e608629c7365326d06fef2bbac8f2b23f048c2a",
    "wallet-broker/tests/account_management.rs": "cc5313884b8737939abcd4a01485ca6e2b45abcb34f23611078cc21a20c465ad",
    "wallet-broker/tests/account_native_ui.rs": "f3524b8cbf1b8334510714cd601e653853dd4b6656873066fbb93d81ff282d5d",
    "package.json": "76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5",
    "package-lock.json": "0e32de43f3c1a31ae3d9e9c8b172d10cd37a03577dc7d1eaf4ac916967cfcdb8",
    "scripts/security-policy.js": "414a5645d8621b5d81cdeba7bb7461ec25647a5a1e5532295d6f75e7b8762532",
    "test/securityPolicy.node.js": "ff449a86e04570f8ff07bd280db6787c62f645f301d4235a65e825f3af7bbb9c"
  },
  "formatted": [],
  "final_hashes": {
    "wallet-broker/src/accounts.rs": "f386f28f44c12bbdf1015713e4c736e81a86f8c0a27024f376b18a9eb8751d2f",
    "wallet-broker/src/account_ui.rs": "ae8aa3b7bd14390244c5e73475195c33a1ba69079ac9ba6b0d86d869941fda75",
    "wallet-broker/src/zec/test_support.rs": "7b4dad2fef0c505f610bdcfe7b87b9075bde0673c8713e34ed57b7714326ce57",
    "wallet-broker/src/session.rs": "9f88b3b0eca91d341a1c025bc8394a25e456350be58e99d9ac9417e31eef21af",
    "wallet-broker/Cargo.toml": "435907f00d7819bdf7dbe9ff9e367522f7f6eb8b6019dbde9e58212bbaea041b",
    "wallet-broker/Cargo.lock": "a5d53b729433a06f60edcef51e608629c7365326d06fef2bbac8f2b23f048c2a",
    "wallet-broker/tests/account_management.rs": "cc5313884b8737939abcd4a01485ca6e2b45abcb34f23611078cc21a20c465ad",
    "wallet-broker/tests/account_native_ui.rs": "f3524b8cbf1b8334510714cd601e653853dd4b6656873066fbb93d81ff282d5d",
    "package.json": "76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5",
    "package-lock.json": "0e32de43f3c1a31ae3d9e9c8b172d10cd37a03577dc7d1eaf4ac916967cfcdb8",
    "scripts/security-policy.js": "414a5645d8621b5d81cdeba7bb7461ec25647a5a1e5532295d6f75e7b8762532",
    "test/securityPolicy.node.js": "ff449a86e04570f8ff07bd280db6787c62f645f301d4235a65e825f3af7bbb9c"
  }
}
```
