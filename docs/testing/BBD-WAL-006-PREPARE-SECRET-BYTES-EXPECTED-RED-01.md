# BBD-WAL-006 Prepare Secret-Bytes Expected Red 01

Jr Dev — Hermes: expected-red gate execution evidence.

## Protected identities

- Hermes version: `0.18.2 (2026.7.7.2) · upstream b20cc5f7 · local 10b6d1a9 (+1 carried commit)`
- Provider: `nous`
- Model: `meituan/longcat-2.0:free`
- Protected HEAD: `d43a0ec749a6fcdc9999db2b9f3e4fae08aa064c`
- origin/master: `d43a0ec749a6fcdc9999db2b9f3e4fae08aa064c`
- Source baseline: `432e69c0443dd5233609d578b43d5a43d83d2c3d`
- Integration commit: `4be931150583876fabadf5a6ffb52021c791fdb3`
- Test source: `wallet-broker/tests/secret_hygiene.rs`, 281 lines, `dcebe361c7061b06b9ec6bb6fbea88ccb208069f7360ae14e2d7c6ca83b433a4`
- Frozen production: `wallet-broker/src/vault.rs`, 759 lines, `89d8ada8ad7050910a92a9daa38f9d93a18b6c4b7d1bde7a7e9b4a8adf8b62b`
- Diff inventory: exactly `wallet-broker/tests/secret_hygiene.rs`, 7 insertions, 0 deletions
- `git diff --check`: clean, no warnings

## Disk location

- `wallet-broker/target/wal006-tmp`: ext4 (`ext2/ext3`)
- `wallet-broker/target/wal006-cargo`: ext4 (`ext2/ext3`)

## Sole acceptance command

```text
TMPDIR="$PWD/wallet-broker/target/wal006-tmp" \
CARGO_TARGET_DIR="$PWD/wallet-broker/target/wal006-cargo" \
rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml \
  --locked --offline --no-default-features --test secret_hygiene \
  secret_bytes_can_be_owned_by_synchronized_account_state -- --exact
```

## Result

- Exit code: **101**
- Phase: compilation, before any test executed
- Executed tests: **0**

## Auto-trait diagnostics

Both E0277 errors at `tests/secret_hygiene.rs:50:24` (`assert_send_sync::<SecretBytes>()`):

1. `(dyn WipeObserver + 'static)` cannot be sent between threads safely — `Send` not implemented, required through `Unique<_>` → `Box<(dyn WipeObserver + 'static)>` → `(&'static str, Box<_>)` → `Option<_>` → `SecretBytes` (src/vault.rs:172).
2. `(dyn WipeObserver + 'static)` cannot be shared between threads safely — `Sync` not implemented, same chain through `SecretBytes`.

## Negative capability record

Current `SecretBytes` owns `Option<(&'static str, Box<dyn WipeObserver>)>`. Because `WipeObserver` has no `Send`/`Sync` supertrait (the accepted observed-drop regression uses an `Rc<RefCell<_>>` observer), `SecretBytes` cannot satisfy `Send + Sync`. The compile-time assertion fails exactly at the new instantiation, confirming the type contract blocks the concurrent-receiver thread boundary required by the accepted concurrent receiver test. No production, ZEC source/test, Cargo, lock, policy, fixture, or other path changed.
