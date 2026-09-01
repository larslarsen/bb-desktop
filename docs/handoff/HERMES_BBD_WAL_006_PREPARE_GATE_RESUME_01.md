# Hermes Handoff — BBD-WAL-006 Prepare Gate Resume 01

You are **Jr Dev — Hermes**. Resume only this execution/evidence/integration gate from command 1.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Accepted corrected source identities are exact in
`docs/testing/BBD-WAL-006-PREPARE-FORMAT-CORRECTION-REVIEW-01.md`. Frozen tests remain:

- `wallet-broker/tests/zec_prepare.rs`, 412 lines,
  `a616245fdcf8d2226277e34d785bb1792d3b8b54ebf268c3d1dc5f15566da942`;
- `wallet-broker/tests/zec_hygiene.rs`, 375 lines,
  `aad7c95a2ef661063661f2ec0f16a216d80328096b049d636b88fa0252ba1be6`.

Read completely: `AGENTS.md`, `TESTING.md`, the original Hermes prepare gate handoff, Prepare Gate
Format Review 01, Prepare Format Correction Review 01, all reviews/handoffs referenced by those
documents, the complete four changed sources, both frozen tests, and `CURRENT_TASK.md`.

## Preconditions

Record actual Hermes version/provider/model, protected `HEAD`, `origin/master`, status, exact diff
inventory including untracked `prepare.rs`, corrected source and frozen-test identities, lock
identity, and `git diff --check`. Verify both existing WAL-006 Cargo work directories are ext4, not
tmpfs. Stop on any mismatch or path beyond the four accepted sources.

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
   `5 security policy test(s) failed`; the bounded Phase-C ZEC production inventory test must pass.

The exact five expected Node failures remain those enumerated in the original prepare gate
handoff. Any formatter mutation, warning/failure, different count, lock mutation, network attempt,
additional Node failure, or inventory-policy failure is a stop.

## Evidence and integration

On the exact gate, use `apply_patch` to create only
`docs/testing/BBD-WAL-006-PREPARE-GATE-01.md` and update only the leading
state/actor/active-handoff block of `docs/handoff/CURRENT_TASK.md`. Include the original format
stop and accepted correction in the evidence, plus every item required by the original handoff.

Stage exactly the four accepted source paths, the new evidence file, and `CURRENT_TASK.md`. Commit
exactly `feat: add WAL-006 unsigned PCZT preparation`, push `master`, and prove a clean
worktree/index with `HEAD == origin/master`. Do not amend, rebase, merge, or force-push.

All other command, mutation, evidence, and negative-capability restrictions from the original
prepare gate remain in force. Stop and report exact evidence on any mismatch.
