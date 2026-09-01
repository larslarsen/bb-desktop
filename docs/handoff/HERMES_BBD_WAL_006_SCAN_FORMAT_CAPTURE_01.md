# Hermes Handoff — BBD-WAL-006 Scan Format Capture 01

You are **Jr Dev — Hermes**. This is a formatter-output capture only.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely: `AGENTS.md`, `TESTING.md`, Hermes routing, Scan Gate 01, Scan Gate Formatter
Review 01, and `CURRENT_TASK.md`.

## Preconditions

Require `HEAD == origin/master ==` the protected governance parent, clean index, source-only and
whole-worktree `git diff --check`, absent `wallet-broker/src/zec/prepare.rs`, and exactly the same
five source paths/line counts/SHA-256 values protected by Scan Gate 01. Require the two existing
ignored WAL-006 target directories to remain disk-backed. Any mismatch stops with no capture run.

## Sole command

Run the following formatter check exactly once from repository root, redirecting complete output
to the two named ignored disk-backed files:

```text
env TMPDIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-tmp CARGO_TARGET_DIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-cargo /home/lars/.cargo/bin/cargo +1.98.0 fmt --manifest-path wallet-broker/Cargo.toml --check > wallet-broker/target/wal006-scan-format.stdout 2> wallet-broker/target/wal006-scan-format.stderr
```

Expected result is exit 1 with rustfmt-only diff hunks and no source mutation. Afterward, read and
return both complete capture files, the exit status, and their line counts/SHA-256 values. Re-prove
the five source hashes, clean index, and exact worktree scope. A different exit/cause or any source
mutation is a stop.

Do not run Clippy, tests, Node, policy, another formatter invocation, Cargo diagnostic, Git
mutation, network, fixture, wallet, node, device, cleanup, or deletion. Do not edit source, tests,
evidence, current task, or any tracked file. Do not stage, commit, or push. The reviewer will issue
the exact mechanical source correction separately.
