# BBD-WAL-019 phase — Sync green, falsification and local wallet refresh 01

Date: 2026-09-15. Actor: Hermes, free Nous Portal model (meituan/longcat-2.0:free),
manually relayed by owner. Reviewer: Codex.

## Model and session

Hermes Agent v0.18.2 (2026.7.7.2) · upstream 110baa09 · local 10b6d1a9 (+1 carried commit)
Provider: nous · Model: meituan/longcat-2.0:free
Session identifier: not recoverable from Hermes version output (provider session).

## Baseline

HEAD: `2b5ad193f01af31f2a2bdcc7b0d1080dd5a2f68e` (verified before/after).
Branch: master.
Filesystem: ext2/ext3 (disk-backed; not tmpfs/ramfs).
Capture directory: `wallet-broker/target/wal019-sync-green-01/`.

## Frozen input verification

All ten pins matched preflight and postflight. Unchanged during capture.

| Path | Lines | SHA-256 | Match |
|---|---|---|---|
| wallet-broker/src/account_ui.rs | 1332 | 09d5c5cbcb7867a06574ebb99426f4331e4d789bd4ec29c1c47122f3566c1f6a | yes |
| wallet-broker/tests/account_native_ui.rs | 2654 | e7ac08d26fad3eb0302c49b8983b62cb28a6aab5bad657b142c2e84d59a11446 | yes |
| wallet-broker/Cargo.toml | 126 | 435907f00d7819bdf7dbe9ff9e367522f7f6eb8b6019dbde9e58212bbaea041b | yes |
| wallet-broker/Cargo.lock | 5771 | a5d53b729433a06f60edcef51e608629c7365326d06fef2bbac8f2b23f048c2a | yes |
| wallet-broker/src/accounts.rs | 1006 | 8941884f837453bf71357f53341fae2c1334b8d10d017680053f9f8a44fb415d | yes |
| wallet-broker/src/zec/live.rs | 1238 | 17be2ab62cdda883940d301d9701ed14c507ff351c4356d46a523a045c3f149f | yes |
| wallet-broker/src/zec/live_transport.rs | 214 | a9f3b50e2694746ec6f443d7a35ded4c035baab4b6003946ac29d1d88e09b90e | yes |
| scripts/build-wallet-broker.js | 152 | 4427ef3e713ec77c45daefe8d37b457ca8dc2c8529b30214dbf65628f43a1445 | yes |
| wallet-broker/launch-config.js | 140 | 68adf92e2d543e5d51353477b1944c35d4189c5590ac581d3289f1bd72898af8 | yes |
| social-main.js | 299 | d959e6adffc2579fa3f2f447f60a165f2b6bcdc279afde2f6ddaa91bc9d9aa4a | yes |

## Stage results

### 00-hermes-version
Command: `timeout 30s hermes --version`
Exit: 0. Output: `Hermes Agent v0.18.2 (2026.7.7.2)`

### 01-rust-version
Command: `timeout 30s rustup run 1.98.0 rustc --version`
Exit: 0. Output: `rustc 1.98.0 (88d9e12ae 2026-08-18)`

### 02-format (rustfmt --check)
Command: `timeout 60s rustup run 1.98.0 rustfmt --check --edition 2024 wallet-broker/src/account_ui.rs wallet-broker/tests/account_native_ui.rs`
Exit: 0. No output (formatter clean).

### 03-focused-green
Command:
```
timeout --signal=TERM --kill-after=10s 600s rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --features native-ui --test account_native_ui wal019_ -- --nocapture
```
Exit: 0. Selected exactly 6 wal019 tests; 16 others filtered out.
`test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 16 filtered out; finished in 0.24s`

### 04-mutation (falsification prep)
Original SHA-256: `09d5c5cbcb7867a06574ebb99426f4331e4d789bd4ec29c1c47122f3566c1f6a`
Mutated SHA-256: `94b7a4d173e3b422f211a785241736e08d5247117f06db884b6cda79b04780f7`
Method: replaced `self.begin_sync(` with `self.open_sync_scene(` so List Sync navigates without start_sync dispatch.

### 05-falsification-red
Command:
```
timeout --signal=TERM --kill-after=10s 600s rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --features native-ui --test account_native_ui wal019_list_server_disclosure_and_one_click_starts_at_both_sizes -- --exact --nocapture
```
Exit: 101. Single test selected; 21 filtered.
`test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 21 filtered out; finished in 0.06s`

The falsified mutant failed because the start-call list was empty:
```
assertion `left == right` failed
  left: []
  right: [("ffeeddccbbaa99887766554433221100", "https://zaino.testnet.unsafe.zec.rocks:443")]
```
This proves the test binds to the actual `begin_sync` dispatch — not to unrelated UI changes.

### 06-restoration (verification, not an executed test command)
Source restored to original 1332-line hash `09d5c5cbcb7867a06574ebb99426f4331e4d789bd4ec29c1c47122f3566c1f6a`. Postflight verified all ten pins unchanged.

### 07-restored-green
Command: same as Stage 03.
Exit: 0. `test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 16 filtered out`

### 08-native-ui (account_native_ui target, not the full native-ui/Rust suite)
Command:
```
timeout --signal=TERM --kill-after=10s 900s rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --features native-ui --test account_native_ui -- --nocapture
```
Exit: 0. `test result: ok. 22 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`
This is the account_native_ui integration test target (22 tests), not every native or Rust test.

### 09-local-build
Command:
```
timeout --signal=TERM --kill-after=10s 600s node scripts/build-wallet-broker.js
```
Exit: 0. Development wallet artifacts rebuilt under `wallet-broker/target/app-resources/wallet-broker/`.

### 10-local-artifacts
- `wallet-broker/target/app-resources/wallet-broker/bitbook-wallet-broker`: 552,768,544 bytes
  SHA-256: `6f0b2ce78a7d871bde75d1fa8f330bd27dc7a00f598853dcc60944b92d15f985`
- `wallet-broker/target/app-resources/wallet-broker/manifest.json`: 115 bytes
  SHA-256: `dfa323db0a754e094456fbaf1651e46c2a8f9a77281055b0512bd97782109bcd`
  Content: `{"v":1,"platform":"linux","arch":"x64","sha256":"6f0b2ce78a7d871bde75d1fa8f330bd27dc7a00f598853dcc60944b92d15f985"}`
- `wallet-broker/target/debug/bitbook-wallet-broker`: 552,768,544 bytes, same SHA-256.
- The developer resource was refreshed; the Electron app was NOT restarted and the daemon executable was NOT changed. These are debug development artifacts (~527.2 MiB).

## Outcome

GREEN, FALSIFICATION, RESTORED GREEN, ACCOUNT_NATIVE_UI TARGET, AND LOCAL WALLET REFRESH — CAPTURED.

The WAL-019 UI tests pass because Grok's source drop added the missing Server/Sync/Balance controls. Falsification (changing `begin_sync` to `open_sync_scene`) made the start-call test fail with an empty start list, proving the test actually binds to the dispatch. Full restoration returned all 6 tests to green. The entire account_native_ui target (22 tests) passes.

Local wallet was rebuilt/staged via `scripts/build-wallet-broker.js` (development binary). This does NOT update the daemon binary, restart the app, or complete any payments.

Raw captures (relative to repository root, disk-backed under wallet-broker/target):

| Artifact | SHA-256 |
|---|---|
| [preflight.json](../../wallet-broker/target/wal019-sync-green-01/preflight.json) | `57cab1d3f3b7991a73dac5b2a60c8185bc1dae8047fe3f42c264c234bef48d02` |
| [postflight.json](../../wallet-broker/target/wal019-sync-green-01/postflight.json) | `5d74f4f9fcb7436dac5d0994f16084549506713516fcd950d3b085b62dd3efc3` |
| [source-before-falsification.rs](../../wallet-broker/target/wal019-sync-green-01/source-before-falsification.rs) | `09d5c5cbcb7867a06574ebb99426f4331e4d789bd4ec29c1c47122f3566c1f6a` |
| [00-hermes-version.log](../../wallet-broker/target/wal019-sync-green-01/00-hermes-version.log) | `7171f1418b425b98c7040b6e838fab15ef17a048bda0b419b5638867ba5768bf` |
| [00-hermes-version.json](../../wallet-broker/target/wal019-sync-green-01/00-hermes-version.json) | `10d8775ff332a7fdf8fb8372f0f5e2094c17c160b8f39d4b00e3212e7d6545b7` |
| [01-rust-version.log](../../wallet-broker/target/wal019-sync-green-01/01-rust-version.log) | `785f1364c0d5bf077f7bfb885fd4bde99899dfd80ce0fb6226a4527d9b27724c` |
| [01-rust-version.json](../../wallet-broker/target/wal019-sync-green-01/01-rust-version.json) | `66667f40150ea4563c7d5c37862887c66a86140cc443eab81502bbc19731fc82` |
| [02-format.log](../../wallet-broker/target/wal019-sync-green-01/02-format.log) | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` (empty) |
| [02-format.json](../../wallet-broker/target/wal019-sync-green-01/02-format.json) | `8211ce8a3936c25fbb0e60c8d9039dd4dba0b73ec14d0dcf78ca007871a7b543` |
| [03-focused-green.log](../../wallet-broker/target/wal019-sync-green-01/03-focused-green.log) | `5fde09e0e5722e2f0ce1db3ec03bf57aaa7cb7bbb4abc6a7776b5779d09c5cd8` |
| [03-focused-green.json](../../wallet-broker/target/wal019-sync-green-01/03-focused-green.json) | `9e71fe1b0ee1bb189c77f1f335b30e2634ef900888978fe74fb00dd8ed3d204a` |
| [04-mutation.json](../../wallet-broker/target/wal019-sync-green-01/04-mutation.json) | `8a881ff9409fbc218308c6259d1518ad0694e915357cc617ab82fb3a11da6f8d` |
| [05-falsification-red.log](../../wallet-broker/target/wal019-sync-green-01/05-falsification-red.log) | `5285e7731197683a9deb2770114712267565b3797e7709f6948b83ce2e44ad39` |
| [05-falsification-red.json](../../wallet-broker/target/wal019-sync-green-01/05-falsification-red.json) | `8e72ba87cb6dcfbd057b43b511936a84fb6932ea82c790cec1b1eb255a95576b` |
| [06-restoration.json](../../wallet-broker/target/wal019-sync-green-01/06-restoration.json) | `b068b2fbf4458390145986e1a43baac553255658857345519e034089e3156047` |
| [07-restored-green.log](../../wallet-broker/target/wal019-sync-green-01/07-restored-green.log) | `9a2c6cce81e3b5374a23ab8e73d639b3d0e05b9731d475a1c56da38fa8d90e8b` |
| [07-restored-green.json](../../wallet-broker/target/wal019-sync-green-01/07-restored-green.json) | `bd2775009fae2ff98ba0930af0da26df988c03c8927dcdd8522dad975d2506fe` |
| [08-native-ui.log](../../wallet-broker/target/wal019-sync-green-01/08-native-ui.log) | `fee57a5120bf3a24be6eb6edf922179634628fbbcc01145dd5da26ec6e28a628` |
| [08-native-ui.json](../../wallet-broker/target/wal019-sync-green-01/08-native-ui.json) | `2c2f8fa5f35814da9e31f48f0c348874e697dd963b85d61b1f341b089eefd761` |
| [09-local-build.log](../../wallet-broker/target/wal019-sync-green-01/09-local-build.log) | `9d892c5778d0c975f64a671e66ea5a753502353da3addd1dfdf3825225e406a2` |
| [09-local-build.json](../../wallet-broker/target/wal019-sync-green-01/09-local-build.json) | `337bf7ac05f98db836716e9cb88838033625fa706a229be5537b66aa02b4c98c` |
| [10-local-artifacts.json](../../wallet-broker/target/wal019-sync-green-01/10-local-artifacts.json) | `ece2e053502accb02fd3988e64e7f9a511adc2bd921979c7254a2c29e67525ed` |

No repository record edits are authorized by this handoff beyond the corrections
above. Report returned to owner for Codex review.
