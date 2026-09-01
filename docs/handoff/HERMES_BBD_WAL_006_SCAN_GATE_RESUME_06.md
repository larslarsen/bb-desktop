# Hermes Handoff — BBD-WAL-006 Scan Gate Resume 06

You are **Jr Dev — Hermes**. This durable handoff is authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely before acting: `AGENTS.md`, `TESTING.md`,
`docs/engineering/DEVELOPMENT_ROLES.md`, `docs/engineering/HERMES_JR_DEV_ROUTING.md`,
`tickets/BBD-WAL-006.md`, Scan Gate 01, Scan Gate Resume 05, Scan Format Correction Review 03,
Scan Runtime Fail-Closed Correction Review 01, the complete current `zec_scan`, `zec_store`, and
`zec_address` tests, and `CURRENT_TASK.md`.

## Sole task and precedence

Restart Scan Gate 01 from every precondition and the formatter. Reuse no prior result. This
resume replaces the governance parent, accepted worktree inventory, and exact-success staging
inventory below. Every other protected identity, absent-path requirement, disk path,
command/order, expected count, immediate stop rule, evidence requirement, commit message, push,
and final-state proof in Scan Gate 01 and Resume 05 remains mandatory.

At the first mismatch, retain and report that result immediately. Run no diagnostic or follow-up
command. Make no evidence/current-task/source/test edit, staging, commit, or push on a stop.

Require `HEAD == origin/master ==` the protected governance parent, a clean index, and worktree
changes consisting only of:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/zec.rs` | 218 | `ae06d6a9b4448603860b55bf4eef42a1e970c414049a9304f42b436d2bf49cb4` |
| `wallet-broker/src/zec/fixture.rs` | 614 | `318820c6f125f2318ba0caf5ae44835b36b0f67856fc66b83401961f0c301b36` |
| `wallet-broker/src/zec/scan.rs` | 1,661 | `54a890fd0b5a8ca8ce78dfb59c13424407341df25e47a5eacd2de8d6cc3317ad` |
| `wallet-broker/src/zec/store.rs` | 1,816 | `d0b97b802209489804e5d550ef90138a87342e155c519a686b2d900189f6603f` |
| `wallet-broker/src/zec/test_support.rs` | 1,220 | `02609ae79bb8d35a5c7fb0058f0ab56fa5deb27c94cfa2ddb81347a2f38ac034` |
| `wallet-broker/tests/zec_scan.rs` | 325 | `87ed1c3e8db8219ad126efb4de7f681cf909cb404b88df6c2dfadec471a3701f` |

Production source totals 5,529 lines. Re-prove the other 11 protected non-source identities from
Scan Gate 01, absent `wallet-broker/src/zec/prepare.rs`, source-only and whole-worktree
`git diff --check`, actual Hermes identity, disk-backed filesystem, and the two exact ignored
paths. Never use `/tmp`.

Run every exact Scan Gate 01 command once in order: Rust 1.98.0 formatter; strict
locked/offline/no-default library Clippy; `zec_scan` with exact 9/0; `zec_store` with exact 8/0;
`zec_address` with exact 8/0; and Node policy with expected exit 1, exact 68 `ok`, exact 6
`not ok`, the six exact frozen failure names, and exact final summary. Stop at the first mismatch.
Do not edit or format source/tests.

Only on complete exact success, create `docs/testing/BBD-WAL-006-SCAN-GATE-01.md`, set
`CURRENT_TASK.md` to `PHASE-C SCAN GATE COMPLETE — REVIEW REQUIRED`, and record every item
required by Scan Gate 01 plus the corrected runtime/test inventory. Stage exactly eight paths:
the five production source paths, corrected `zec_scan.rs`, evidence, and current task. Inspect the
staged names/diff, commit exactly `feat: add WAL-006 compact block scanning`, push `master`, and
prove `HEAD == origin/master`, clean index, and clean tracked worktree. Never stage ignored
targets. The reviewer alone accepts the result and authorizes the next slice.
