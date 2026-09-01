# Hermes Handoff — BBD-WAL-006 Store Gate Resume 04

You are **Jr Dev — Hermes**. This durable handoff is authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely before acting: `AGENTS.md`, `TESTING.md`,
`docs/engineering/DEVELOPMENT_ROLES.md`, `docs/engineering/HERMES_JR_DEV_ROUTING.md`,
`tickets/BBD-WAL-006.md`, `docs/handoff/HERMES_BBD_WAL_006_STORE_GATE_01.md`, Store Gate Runtime
Review 01, Store Cache-Schema Correction Review 01, Store Test Compile Correction Review 01, the
complete current `zec_store` and committed `zec_address` tests, and
`docs/handoff/CURRENT_TASK.md`.

## Sole task and precedence

Restart every Store Gate 01 protected precondition and command from the beginning. Reuse no
earlier formatter, Clippy, test, policy, or identity result. This resume replaces the governance
parent, four-source identity, `zec_store` test identity/worktree scope, and exact-success staging
inventory below. Every other identity, ignored path, exact command/order, expected result, stop
rule, evidence restriction, commit message, push, and final-state proof in Store Gate 01 remains
mandatory.

Require `HEAD == origin/master ==` the protected governance parent, a clean index, and worktree
changes consisting only of:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/zec.rs` | 214 | `800111bf587265b49764de71c6d2beb054d1906932005bb3513b2d4fc132d90e` |
| `wallet-broker/src/zec/fixture.rs` | 280 | `01c4fa4b83b0b5f4501db966c27e07c2d3f7d88148daac1865bc35e67044c698` |
| `wallet-broker/src/zec/store.rs` | 1,700 | `779f847a328a8fe85ca7a951a67d6be12403ec3f73b9557c943c4e404742052f` |
| `wallet-broker/src/zec/test_support.rs` | 825 | `e37f486b7dceab7d35fc1d0385ca17d82d5082e77732edeaca727e8903592346` |
| `wallet-broker/tests/zec_store.rs` | 324 | `1c230a2a9cf51c841a0df6514393861387422e5d0b2a83e80af47022728e2225` |

The production-source total is 3,019 lines. Re-prove the other eight protected non-source
identities from Store Gate 01, including `zec_address`, and both source-only and whole-worktree
`git diff --check`. Record actual Hermes version/provider/model, inspect the filesystem, and use
only the exact ignored disk-backed paths from Store Gate 01. Do not use `/tmp`.

Run once in the exact original order: Rust 1.98.0 formatter; locked/offline/no-default library
Clippy with `-D warnings`; `zec_store` requiring exact 8/0 counts; `zec_address` requiring exact
8/0 counts; and Node policy requiring expected exit 1, exact 69 `ok`, exact 6 `not ok`, the exact
six frozen names, and exact final summary. Stop immediately at the first mismatch. Do not edit
source/tests or run another command.

Only after complete exact success, perform the Store Gate 01 evidence and integration steps. Stage
exactly seven paths: four source files, corrected store test, new evidence, and
`docs/handoff/CURRENT_TASK.md`. Inspect the staged list/diff, commit exactly
`feat: add WAL-006 viewing store boundary`, push `master`, and prove the clean synchronized final
state. The reviewer alone accepts the integration or authorizes later work.
