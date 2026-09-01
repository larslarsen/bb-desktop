# Hermes Handoff — BBD-WAL-006 Final Policy Integration 01

You are **Jr Dev — Hermes** using only the free configured Nous route. Own this bounded
policy execution/evidence/integration gate. Do not alter source or tests.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely: `AGENTS.md`, `TESTING.md`, `tickets/BBD-WAL-006.md`, the policy source and
focused-gate handoffs, this handoff, `scripts/security-policy.js`, `test/securityPolicy.node.js`,
`test/electronSecurity.node.js`, `package.json`, and `CURRENT_TASK.md`.

## Preconditions

Record Hermes version/provider/model. Require `HEAD == origin/master`, exactly one dirty path,
clean `git diff --check`, and:

- `scripts/security-policy.js`: 2,482 lines,
  SHA-256 `d9b77bf5608e1aa79d565e5c3c9574c1e203286c584c30498dd173796434d7f5`;
- `test/securityPolicy.node.js`: 2,525 lines,
  SHA-256 `2c1bd92c778a975b1218dfd2445b6b1b605bb047032fd5289573cbbb6d0a0169`.

## Exact commands

Run once each, in order, from the repository root; stop on first mismatch.

1. `node test/securityPolicy.node.js`
   — exit 0, exactly 75 `ok`, zero `not ok`, final line
   `BitBook desktop security policy tests passed (75).`
2. `node scripts/security-policy.js`
   — exit 0, exact line `BitBook desktop security policy checks passed.`
3. `npm run test:security`
   — exit 0; Electron security exactly 19 passed, policy exactly 75 passed, no failure.

## Evidence and integration

Only on exact success, use `apply_patch` to create
`docs/testing/BBD-WAL-006-FINAL-POLICY-GATE-01.md` and update only the leading current-task block.
Record preconditions, all exact commands/results, closure of the six prior failures, exact closed
WAL-004/WAL-006 inventories, forbidden-feature/authority enforcement, legitimate offline upstream
allowance, source/test identities, and final Git state.

Stage exactly:

- `scripts/security-policy.js`;
- `docs/testing/BBD-WAL-006-FINAL-POLICY-GATE-01.md`;
- `docs/handoff/CURRENT_TASK.md`.

Commit exactly `fix: enforce WAL-006 security policy`, push `master`, and prove a clean
worktree/index with `HEAD == origin/master`.

Do not modify source/test/package/workflow/manifest/lock, run any other Node/npm/Rust/Cargo command,
use network/audit/scanners/Electron/wallet/node/device, amend/rebase/merge/force-push, clean, or
delete. Any mismatch stops without evidence or integration.
