# Hermes Handoff — BBD-WAL-006 Store Gate Resume 02

You are **Jr Dev — Hermes**. This durable handoff is authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely before acting: `AGENTS.md`, `TESTING.md`,
`docs/engineering/DEVELOPMENT_ROLES.md`, `docs/engineering/HERMES_JR_DEV_ROUTING.md`,
`tickets/BBD-WAL-006.md`, `docs/handoff/HERMES_BBD_WAL_006_STORE_GATE_01.md`, Store Production
Source Review 02, both Store Gate Formatter Reviews, both Store Format Correction Reviews, the
complete committed `zec_store` and `zec_address` tests, Address Gate Evidence 01, and
`docs/handoff/CURRENT_TASK.md`.

## Sole task and precedence

Restart Store Gate 01 from all protected preconditions and its first formatter command. Do not
reuse any earlier precondition, formatter, identity, or command result. This resume replaces only
the protected governance parent and four-source identity below. Every other protected identity,
ignored path, exact command/order, expected count, stop rule, evidence requirement, staging
restriction, commit message, push, and final-state proof in Store Gate 01 remains mandatory.

Require `HEAD == origin/master ==` the protected governance parent, a clean index, and worktree
changes consisting only of:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/zec.rs` | 214 | `800111bf587265b49764de71c6d2beb054d1906932005bb3513b2d4fc132d90e` |
| `wallet-broker/src/zec/fixture.rs` | 280 | `01c4fa4b83b0b5f4501db966c27e07c2d3f7d88148daac1865bc35e67044c698` |
| `wallet-broker/src/zec/store.rs` | 1,686 | `f12f634b90d8a517038866d8632a94bb12bbbcae109e35fd2721bbb3b9401662` |
| `wallet-broker/src/zec/test_support.rs` | 825 | `e37f486b7dceab7d35fc1d0385ca17d82d5082e77732edeaca727e8903592346` |

Total source is 3,005 lines. Re-prove the nine protected non-source identities and both source-
only and whole-worktree `git diff --check`. Record actual Hermes version/provider/model, inspect
the wallet-broker filesystem type, and use only the exact ignored disk-backed paths in Store Gate
01. Do not use `/tmp`.

Run every exact Store Gate 01 command once and in its original order: Rust 1.98.0 formatter;
locked/offline/no-default library Clippy with `-D warnings`; `zec_store` with exact 8/0 counts;
`zec_address` with exact 8/0 counts; and Node policy with expected exit 1, exact 69 `ok`, exact 6
`not ok`, exact frozen failure names, and exact final summary. Stop immediately at the first
mismatch. Do not edit source/tests or run another command.

Only on complete exact success, perform the Store Gate 01 evidence and integration steps without
change: create `docs/testing/BBD-WAL-006-STORE-GATE-01.md`, set `CURRENT_TASK.md` to
`PHASE-C STORE GATE COMPLETE — REVIEW REQUIRED`, stage only the four source paths/evidence/current
task, commit `feat: add WAL-006 viewing store boundary`, push `master`, and prove the clean exact
final state. The reviewer alone accepts the result and authorizes later work.
