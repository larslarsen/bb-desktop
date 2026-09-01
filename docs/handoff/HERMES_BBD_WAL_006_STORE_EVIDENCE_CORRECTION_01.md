# Hermes Handoff — BBD-WAL-006 Store Evidence Correction 01

You are **Jr Dev — Hermes**. This durable handoff is authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely before acting: `AGENTS.md`, `TESTING.md`, Store Gate Evidence 01, Store Gate
Policy-Count Review 01, Store Integration Review 01, Store Gate Resume 05, and
`docs/handoff/CURRENT_TASK.md`.

## Sole task

Correct only:

- `docs/testing/BBD-WAL-006-STORE-GATE-01.md`; and
- `docs/handoff/CURRENT_TASK.md`.

Do not edit or stage source, tests, fixtures, policies, dependencies, tickets, workflows, another
document, or another repository. Do not rerun Cargo, Rust, Node, policy, formatter, Clippy, or any
test. Do not clean/delete artifacts.

Require `HEAD == origin/master ==` the protected governance parent, a clean index/worktree, and
the integrated commit `b450cd78c9e2e74597a0724741d7d3cade0a55b2` as its first parent.

## Evidence corrections

Preserve every exact command result, protected identity, behavior claim not corrected below,
negative-evidence statement, and redaction. Make only these corrections:

1. Replace the impossible UTC timestamp with an honest record: exact command start time was not
   retained; integration completion is recorded by commit timestamp
   `2026-08-31T18:00:17-07:00` (`America/Los_Angeles`).
2. Show each of the four Cargo commands exactly with its original prefix:
   `env TMPDIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-tmp CARGO_TARGET_DIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-cargo`, followed by the exact
   Cargo command already listed. Do not rerun them.
3. Correct the durability test name to
   `failed_write_file_sync_and_directory_sync_never_report_durable_state`.
4. Keep initialization/reopen as the exact binding/schema claim. Attribute viewing-only to
   `sqlite_schema_and_rows_contain_viewing_state_but_no_spend_secrets`, which calls
   `open_viewing_context()` and proves `has_spending_authority() == false` in addition to secret
   exclusion.
5. Record exact integration commit and final synchronized state as
   `b450cd78c9e2e74597a0724741d7d3cade0a55b2`.

## Current-task restoration

Restore the complete 446-line `docs/handoff/CURRENT_TASK.md` audit history from commit
`2d2a52ef619aecab2fc1a29b6287b8c1aecfb8b5` (SHA-256
`2e0a0642ab249a965643ffe476594d805fe4bb200c2f90b3770e3c2337f84dde`) as the base, using
`apply_patch`; do not use a shell write/redirection. Then make only the current transition at the
top:

- state: `PHASE-C STORE GATE COMPLETE — REVIEW REQUIRED`;
- record integration commit `b450cd78c9e2e74597a0724741d7d3cade0a55b2`;
- link Store Gate Evidence 01, Store Gate Policy-Count Review 01, Store Integration Review 01, and
  this completed correction handoff; and
- preserve the reviewer, Hermes routing, source baseline, and every historical link from the
  restored parent.

Do not claim final reviewer acceptance.

## Integration

After exact file/scope/diff review, stage only the two corrected governance paths. Inspect the
staged list/diff. Commit exactly `docs: correct WAL-006 store gate evidence`, push `master`, and
prove `HEAD == origin/master`, clean index, and clean tracked worktree. Return both resulting file
line counts/SHA-256 values, the correction commit, push, and final state. The reviewer alone
accepts the result.
