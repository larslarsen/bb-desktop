# Hermes Handoff — BBD-WAL-006 Prepare Gate 01

You are **Jr Dev — Hermes**. Own only this execution/evidence/integration gate.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Accepted source and frozen-test identities are exact in
`docs/testing/BBD-WAL-006-PREPARE-PRODUCTION-SOURCE-REVIEW-02.md`.

Read completely: `AGENTS.md`, `TESTING.md`, `tickets/BBD-WAL-006.md`, Prepare Design Review 01,
Prepare Production Source Reviews 01 and 02, both prepare production handoffs, the complete four
changed source paths, both frozen tests, and `docs/handoff/CURRENT_TASK.md`.

## Preconditions

Record actual Hermes version/provider/model, protected `HEAD`, `origin/master`, status, exact diff
inventory including the untracked `prepare.rs`, all accepted line counts/SHA-256, frozen-test and
lock identities, and `git diff --check`. Verify the existing `wallet-broker/target/wal006-tmp` and
`wallet-broker/target/wal006-cargo` directories are on ext4, not tmpfs. Stop on any changed path or
hash beyond the four accepted production paths.

## Exact commands and expected results

Run in order and stop on the first mismatch.

1. `TMPDIR="$PWD/wallet-broker/target/wal006-tmp" CARGO_TARGET_DIR="$PWD/wallet-broker/target/wal006-cargo" rustup run 1.98.0 cargo fmt --manifest-path wallet-broker/Cargo.toml -- --check`
   — exit 0 and no diff.
2. `TMPDIR="$PWD/wallet-broker/target/wal006-tmp" CARGO_TARGET_DIR="$PWD/wallet-broker/target/wal006-cargo" rustup run 1.98.0 cargo clippy --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --lib --test zec_prepare --test zec_hygiene -- -D warnings`
   — exit 0 with no warning/diagnostic.
3. `TMPDIR="$PWD/wallet-broker/target/wal006-tmp" CARGO_TARGET_DIR="$PWD/wallet-broker/target/wal006-cargo" rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_prepare`
   — exit 0, exactly 11 passed, 0 failed.
4. `TMPDIR="$PWD/wallet-broker/target/wal006-tmp" CARGO_TARGET_DIR="$PWD/wallet-broker/target/wal006-cargo" rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_hygiene`
   — exit 0, exactly 8 passed, 0 failed.
5. `TMPDIR="$PWD/wallet-broker/target/wal006-tmp" CARGO_TARGET_DIR="$PWD/wallet-broker/target/wal006-cargo" rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_fixture_builder --test zec_address --test zec_store --test zec_scan --test vault_crypto --test vault_format --test vault_store --test vault_session --test native_surface --test secret_hygiene`
   — exit 0, exactly 108 passed in total and 0 failed: 4 fixture, 8 address, 8 store,
   9 scan, 11 vault-crypto, 11 vault-format, 20 vault-store, 13 vault-session, 13 native-surface,
   and 11 secret-hygiene.
6. `node test/securityPolicy.node.js`
   — exit 1, exactly 70 `ok`, exactly the five listed expected `not ok`, final line
   `5 security policy test(s) failed`; the exact bounded Phase-C ZEC production inventory test must
   turn green and no new failure is permitted.

The exact five expected Node failures are:

- `committed workflows satisfy the fail-closed checker`;
- `strict nine-line reviewed Gitleaks ratchet bytes and content are enforced`;
- `WAL-004 Rust source inventory is exported closed and enumerated by repository policy`;
- `WAL-006 feature policy distinguishes compiled upstream PCZT capability from BitBook authority`;
- `WAL-006 policy rejects live-network and authority-bearing Rust snippets without denying upstream transitives`.

Any formatter mutation, warning/failure, different test count, lock mutation, network attempt,
additional Node failure, or failure of the production-inventory Node test is a stop.

## Evidence and integration

On the exact gate, use `apply_patch` to create only
`docs/testing/BBD-WAL-006-PREPARE-GATE-01.md` and update only the leading
state/actor/active-handoff block of `docs/handoff/CURRENT_TASK.md`. Record commands verbatim,
exits/counts, identities, disk location, real proposal/PCZT path coverage, lifecycle/unwind
coverage, read-only SQLite boundary, Node transition, and negative capability.

Stage exactly:

- `wallet-broker/src/zec/prepare.rs`
- `wallet-broker/src/zec.rs`
- `wallet-broker/src/zec/store.rs`
- `wallet-broker/src/zec/test_support.rs`
- `docs/testing/BBD-WAL-006-PREPARE-GATE-01.md`
- `docs/handoff/CURRENT_TASK.md`

Commit exactly `feat: add WAL-006 unsigned PCZT preparation`, push `master`, and prove a clean
worktree/index with `HEAD == origin/master`. Do not amend, rebase, merge, or force-push.

Do not modify source/test after execution. Do not run another Cargo/Rust/Node command, npm, audit,
scanner, dependency resolution, network fetch, Electron, wallet/node/device, fixture, cleanup, or
deletion. Do not edit Cargo/lock, tests, policy implementation, ticket, fixture, workflow, package,
reviewer documents, or another path. Stop and report exact evidence on any mismatch.
