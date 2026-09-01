# Hermes Handoff — BBD-WAL-006 Store Gate Resume 01

You are **Jr Dev — Hermes**. This durable handoff is authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely before acting: `AGENTS.md`, `TESTING.md`,
`docs/engineering/DEVELOPMENT_ROLES.md`, `docs/engineering/HERMES_JR_DEV_ROUTING.md`,
`tickets/BBD-WAL-006.md`, `docs/handoff/HERMES_BBD_WAL_006_STORE_GATE_01.md`, Store Source
Reviews 01 and 02, Store Gate Formatter Review 01, Store Format Correction Review 01, the
complete committed `zec_store` and `zec_address` tests, Address Gate Evidence 01, and
`docs/handoff/CURRENT_TASK.md`.

## Sole task and precedence

Restart Store Gate 01 from its protected preconditions and first formatter command. Do not reuse
the earlier formatter result or any earlier command output. This resume changes only the
governance parent and accepted four-source identity below. Every role boundary, protected
non-source identity, exact ignored path, exact command and order, expected count, stop rule,
evidence requirement, staging restriction, commit message, push, and final-state proof in Store
Gate 01 remains mandatory.

You are the execution, evidence, and Git actor. You are not the reviewer and may not design/edit a
test, repair/format source, change a command, accept a mismatch, or authorize further work. Stop
immediately on the first mismatch and make no evidence, source edit, staging, commit, or push.

## Replacement protected preconditions

Require `HEAD == origin/master ==` the protected governance parent, a clean index, and worktree
changes consisting only of:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/zec.rs` | 214 | `800111bf587265b49764de71c6d2beb054d1906932005bb3513b2d4fc132d90e` |
| `wallet-broker/src/zec/fixture.rs` | 280 | `01c4fa4b83b0b5f4501db966c27e07c2d3f7d88148daac1865bc35e67044c698` |
| `wallet-broker/src/zec/store.rs` | 1,687 | `534e118c4bb34bf9b27d8342bde4da7f3acca255cb440714790f4994c47a6ad4` |
| `wallet-broker/src/zec/test_support.rs` | 825 | `e37f486b7dceab7d35fc1d0385ca17d82d5082e77732edeaca727e8903592346` |

Total source is 3,006 lines. Re-prove every non-source protected identity from Store Gate 01 and
both source-only and whole-worktree `git diff --check` before execution.

Record the actual resolved identity before execution:

```text
hermes --version
hermes config get model.provider
hermes config get model.default
```

Inspect the wallet-broker filesystem type and use only the two ignored disk-backed paths named in
Store Gate 01. Do not use `/tmp`. Run every exact Store Gate 01 command once, from the repository
root, in the original order:

1. Rust 1.98.0 formatter check — require exit 0 without mutation.
2. locked/offline/no-default-features library Clippy with `-D warnings` — require exit 0 without
   warning or diagnostic.
3. locked/offline/no-default-features `zec_store` — require exactly 8 passed and 0 otherwise.
4. locked/offline/no-default-features `zec_address` — require exactly 8 passed and 0 otherwise.
5. Node security policy — require expected exit 1, exactly 69 `ok`, exactly 6 `not ok`, the exact
   six frozen failure groups from Store Gate 01, and final line
   `6 security policy test(s) failed`.

No other command, test, tool, source repair, cleanup, or network access is authorized.

## Exact-success integration

Only if all results are exact, create `docs/testing/BBD-WAL-006-STORE-GATE-01.md`, update
`docs/handoff/CURRENT_TASK.md` to `PHASE-C STORE GATE COMPLETE — REVIEW REQUIRED`, recheck exact
scope/hashes/diffs, and stage explicitly only the accepted four source paths, evidence, and
`CURRENT_TASK.md`. Inspect the staged list and diff. Commit exactly:

```text
feat: add WAL-006 viewing store boundary
```

Push `master` and prove `HEAD == origin/master`, clean index, and clean tracked worktree. Preserve
all evidence fields, redaction restrictions, prohibited-capability statements, and frozen scope
from Store Gate 01. The reviewer alone accepts the result and authorizes the next slice.
