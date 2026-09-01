# Hermes Handoff — BBD-WAL-006 Prepare Rollback Expected Red 01

You are **Jr Dev — Hermes** using only the free configured Nous route. Own one focused
expected-red execution. Do not modify files or perform Git mutations.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read `AGENTS.md`, `TESTING.md`, `tickets/BBD-WAL-006.md`,
`docs/testing/BBD-WAL-006-PREPARE-STAGE-DIAGNOSTIC-REVIEW-01.md`, this handoff,
`wallet-broker/tests/zec_prepare.rs`, `wallet-broker/src/zec/store.rs`, and
`docs/handoff/CURRENT_TASK.md`.

Verify the free Hermes version/provider/model, `HEAD == origin/master`, exactly eight dirty source
or test paths, a clean `git diff --check`, and ext4 for both Cargo work directories. The eight paths
are:

- `scripts/security-policy.js`
- `wallet-broker/Cargo.lock`
- `wallet-broker/Cargo.toml`
- `wallet-broker/src/zec.rs`
- `wallet-broker/src/zec/prepare.rs`
- `wallet-broker/src/zec/store.rs`
- `wallet-broker/src/zec/test_support.rs`
- `wallet-broker/tests/zec_prepare.rs`

Run exactly once from the repository root:

```text
TMPDIR="$PWD/wallet-broker/target/wal006-tmp" CARGO_TARGET_DIR="$PWD/wallet-broker/target/wal006-cargo" rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_prepare sufficient_confirmed_ironwood_prepares_sanitized_v6_handle_and_decoded_pczt -- --exact --nocapture
```

Expected: exit 101 with zero passed and one failed because current production returns public
`INTERNAL` from PCZT construction. A byte-equality assertion failure after successful construction
is also an acceptable expected red. Compilation/setup failure, warning, network attempt, another
test result, or file/lock mutation is a stop.

Do not run a second command, formatter, Clippy, another test, Node, npm, audit, scanner, dependency
resolution, wallet/node/device, or network client. Do not create evidence, edit `CURRENT_TASK.md`,
stage, commit, push, clean, or delete. Report the exact result.
