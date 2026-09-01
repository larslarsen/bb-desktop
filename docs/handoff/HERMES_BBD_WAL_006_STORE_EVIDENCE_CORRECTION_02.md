# Hermes Handoff — BBD-WAL-006 Store Evidence Correction 02

You are **Jr Dev — Hermes**. This durable handoff is authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely before acting: `AGENTS.md`, Store Gate Evidence 01, Store Gate Resume 05, Store
Evidence Correction Review 01, and `docs/handoff/CURRENT_TASK.md`.

## Sole task

Require `HEAD == origin/master ==` the protected governance parent and a clean index/worktree.
Using `apply_patch`, edit only:

- `docs/testing/BBD-WAL-006-STORE-GATE-01.md`; and
- `docs/handoff/CURRENT_TASK.md`.

In the evidence, replace only the four command-line occurrences of
`CARGO_TARGET_DIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-cargo cargo +1.98.0`
with
`CARGO_TARGET_DIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-cargo /home/lars/.cargo/bin/cargo +1.98.0`.

Do not change any other evidence byte. Do not rerun a formatter, compiler, Clippy, Rust test, Node
test, policy command, or other acceptance command. Do not edit/stage source, tests, fixtures,
policy, dependencies, tickets, workflows, another document, or another repository.

In `CURRENT_TASK.md`, set the state to `PHASE-C STORE GATE COMPLETE — REVIEW REQUIRED`, replace
the active-handoff link with a `Completed exact-command evidence correction` link to this handoff,
and preserve every other line/history.

After exact file/scope/diff review, stage only those two paths, inspect the staged list/diff,
commit exactly `docs: record exact WAL-006 store gate commands`, push `master`, and prove
`HEAD == origin/master`, clean index, and clean tracked worktree. Return the two line counts and
SHA-256 values, correction commit, push, and final state. The reviewer alone accepts the store
integration and authorizes the next slice.
