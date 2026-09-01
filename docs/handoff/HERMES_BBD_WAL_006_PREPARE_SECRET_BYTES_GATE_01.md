# Hermes Handoff — BBD-WAL-006 Prepare Secret-Bytes Gate 01

You are **Jr Dev — Hermes**. Own only this execution/evidence/integration gate.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Accepted production: `wallet-broker/src/vault.rs`, 773 lines,
`500cd2f91ec0a2e0052779ba6b2357053ce0bea1d644fb2c35066f768f363fe0`.

Frozen test: `wallet-broker/tests/secret_hygiene.rs`, 281 lines,
`dcebe361c7061b06b9ec6bb6fbea88ccb208069f7360ae14e2d7c6ca83b433a4`.

Read completely: `AGENTS.md`, `TESTING.md`, Prepare Secret-Bytes Design Review 01, Expected-Red
Review 01, Production Source Review 01, both Sol handoffs, the complete changed source and frozen
test, and `docs/handoff/CURRENT_TASK.md`.

## Preconditions

Record actual Hermes version/provider/model, protected `HEAD`, `origin/master`, status, exact diff
inventory, line counts/SHA-256, and `git diff --check`. Verify the existing
`wallet-broker/target/wal006-tmp` and `wallet-broker/target/wal006-cargo` directories are on ext4,
not tmpfs. Stop on any changed path/hash beyond the accepted production source.

## Exact commands and expected results

Run in order and stop on the first mismatch.

1. `TMPDIR="$PWD/wallet-broker/target/wal006-tmp" CARGO_TARGET_DIR="$PWD/wallet-broker/target/wal006-cargo" rustup run 1.98.0 cargo fmt --manifest-path wallet-broker/Cargo.toml -- --check`
   — exit 0 and no diff.
2. `TMPDIR="$PWD/wallet-broker/target/wal006-tmp" CARGO_TARGET_DIR="$PWD/wallet-broker/target/wal006-cargo" rustup run 1.98.0 cargo clippy --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --lib --test secret_hygiene -- -D warnings`
   — exit 0 with no warning/diagnostic.
3. `TMPDIR="$PWD/wallet-broker/target/wal006-tmp" CARGO_TARGET_DIR="$PWD/wallet-broker/target/wal006-cargo" rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test secret_hygiene secret_bytes_can_be_owned_by_synchronized_account_state -- --exact`
   — exit 0, exactly 1 passed, 0 failed.
4. `TMPDIR="$PWD/wallet-broker/target/wal006-tmp" CARGO_TARGET_DIR="$PWD/wallet-broker/target/wal006-cargo" rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test secret_hygiene`
   — exit 0, exactly 11 passed, 0 failed.
5. `TMPDIR="$PWD/wallet-broker/target/wal006-tmp" CARGO_TARGET_DIR="$PWD/wallet-broker/target/wal006-cargo" rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test vault_crypto --test vault_format --test vault_session --test native_surface --test zec_address`
   — exit 0, exactly 56 passed in total, 0 failed.
6. `node test/securityPolicy.node.js`
   — exit 1, exactly 69 `ok`, exactly the six frozen Phase-C `not ok`, final line
   `6 security policy test(s) failed`; no new failure.

The expected-red compile failure is already integrated falsification evidence. Command 4 must also
prove the existing `Rc<RefCell<_>>` observed-drop regression still records one real post-zeroize
event. Any source mutation by format, extra warning/failure, different test count, lock mutation,
network attempt, or additional Node failure is a stop.

## Evidence and integration

On the exact gate, use `apply_patch` to create only
`docs/testing/BBD-WAL-006-PREPARE-SECRET-BYTES-GATE-01.md` and update only the leading
state/actor/active-handoff block of `docs/handoff/CURRENT_TASK.md`. Record commands verbatim,
exits/counts, source/test identities, disk location, the prior red-to-green transition, observer
regression, auto-trait result, and negative capability record.

Stage exactly:

- `wallet-broker/src/vault.rs`
- `docs/testing/BBD-WAL-006-PREPARE-SECRET-BYTES-GATE-01.md`
- `docs/handoff/CURRENT_TASK.md`

Commit exactly `fix: make WAL-006 secret bytes thread-safe`, push `master`, and prove a clean
worktree/index with `HEAD == origin/master`. Do not amend, rebase, merge, or force-push.

Do not modify source/test after execution. Do not run another Cargo/Rust/Node command, npm, audit,
scanner, dependency resolution, network fetch, Electron, wallet/node/device, fixture, cleanup, or
deletion. Do not edit Cargo/lock, ZEC source/tests, policy implementation, ticket, fixture,
workflow, package, or another path. Stop and report exact evidence on any mismatch.
