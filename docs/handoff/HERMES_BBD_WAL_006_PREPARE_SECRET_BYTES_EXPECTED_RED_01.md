# Hermes Handoff — BBD-WAL-006 Prepare Secret-Bytes Expected Red 01

You are **Jr Dev — Hermes**. Own only this execution/evidence/integration gate.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Accepted test source: `wallet-broker/tests/secret_hygiene.rs`, 281 lines,
`dcebe361c7061b06b9ec6bb6fbea88ccb208069f7360ae14e2d7c6ca83b433a4`.

Frozen production: `wallet-broker/src/vault.rs`, 759 lines,
`89d8ada8ad7050910a92a9daa38f9d93a18b86c4b7d1bde7a7e9b4a8adf8b62b`.

Read completely: `AGENTS.md`, `TESTING.md`, Prepare Production Source Stop Review 01, Prepare
Secret-Bytes Design Review 01, Prepare Secret-Bytes Test Source Review 01, the Sol test handoff,
the complete changed test and frozen production file, and `docs/handoff/CURRENT_TASK.md`.

## Preconditions

Record the actual Hermes version, provider, model, protected `HEAD`, `origin/master`, status, exact
diff inventory, line counts/SHA-256, and `git diff --check`. Verify that the existing
`wallet-broker/target/wal006-tmp` and `wallet-broker/target/wal006-cargo` directories are on ext4,
not tmpfs. Stop on any source path/hash or tracked/unstaged change beyond the accepted test.

## Sole acceptance command

Run from the repository root with those existing disk-backed directories:

```text
TMPDIR="$PWD/wallet-broker/target/wal006-tmp" \
CARGO_TARGET_DIR="$PWD/wallet-broker/target/wal006-cargo" \
rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml \
  --locked --offline --no-default-features --test secret_hygiene \
  secret_bytes_can_be_owned_by_synchronized_account_state -- --exact
```

Expected result: exit 101 during compilation, before any test runs. The failure must be E0277 at
the new `assert_send_sync::<SecretBytes>()` instantiation and must identify
`(dyn WipeObserver + 'static)` as not safely `Send` and not safely `Sync` through
`Box<dyn WipeObserver>` and `SecretBytes`. A dependency, syntax, linker, disk, network, unrelated
warning/error, different source location, or executed test is a stop, not evidence.

## Evidence and integration

On the exact expected red, use `apply_patch` to create only
`docs/testing/BBD-WAL-006-PREPARE-SECRET-BYTES-EXPECTED-RED-01.md` and update only the leading
state/actor/active-handoff block of `docs/handoff/CURRENT_TASK.md`. Record the command verbatim,
exit, zero executed tests, both auto-trait diagnostics, protected identities, disk location, and
negative capability record.

Stage exactly:

- `wallet-broker/tests/secret_hygiene.rs`
- `docs/testing/BBD-WAL-006-PREPARE-SECRET-BYTES-EXPECTED-RED-01.md`
- `docs/handoff/CURRENT_TASK.md`

Commit exactly `test: require thread-safe WAL-006 secret bytes`, push `master`, and prove a clean
worktree/index with `HEAD == origin/master`. Do not amend, rebase, merge, or force-push.

Do not modify test or production source after execution. Do not run another Cargo/Rust command,
formatter, Clippy, Node, npm, policy, audit, scanner, dependency resolution, network fetch,
Electron, wallet/node/device, fixture, cleanup, or deletion. Do not edit Cargo/lock, ZEC source or
tests, existing test assertions, policy, fixture, ticket, workflow, package, or another path. Stop
and report exact evidence on any mismatch.
