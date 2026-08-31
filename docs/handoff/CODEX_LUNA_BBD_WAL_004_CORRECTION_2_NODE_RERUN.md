# Codex Luna Handoff — BBD-WAL-004 Correction 2 Node Rerun

You are **Jr Dev — Codex Luna**. This durable file is the complete prompt; ephemeral
chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff.

Read `AGENTS.md`, `TESTING.md`, roles, `tickets/BBD-WAL-004.md`,
`docs/testing/BBD-WAL-004-CORRECTION-2-INTEGRATION-REVIEW-01.md`,
`docs/testing/BBD-WAL-004-CORRECTION-2-NODE-FIXTURE-REVIEW.md`, the existing Correction 2
red evidence, `CURRENT_TASK.md`, and the complete changed Node test.

Verify `HEAD == origin/master`, clean index, the Node test at 2,053 lines/SHA
`cf167b1bd27b28e7c59db438af5a06304fd16506fb6056904e8dbe5215222ee2`, and all 15 frozen
production hashes. Run only:

```text
node test/securityPolicy.node.js
```

All 65 cases must execute. Exact acceptable red is 62 `ok` and three `not ok`:
`committed workflows satisfy the fail-closed checker`, `strict eight-line inherited
Gitleaks ratchet bytes and content are enforced`, and `WAL-004 Rust source inventory is
exported closed and enumerated by repository policy`. Each must fail only because the
same exact seven source paths are rejected when enumeration order differs. The generic
Rust first-party source policy test must now pass. Any other result, canary, exception,
setup issue, or changed path is a blocker.

Do not rerun Rust or any other Node/npm/test/build/scanner/policy command. Do not edit
production. If exact, update only
`docs/testing/BBD-WAL-004-CORRECTION-2-EXPECTED-RED.md` with a dated Node-rerun section
superseding its original Node totals, and update only `docs/handoff/CURRENT_TASK.md` to
`CORRECTION 2 EXPECTED RED ACCEPTED — PRODUCTION CORRECTION REQUIRED`.

Run `git diff --check`. Stage only `test/securityPolicy.node.js`, the updated evidence,
and `CURRENT_TASK.md`; inspect staged names/diff. Commit once as
`test: correct wallet custody node red` and push master. Leave all production unstaged
and hash-identical. Report command/status/totals, no-canary result, evidence line/hash,
commit/push, and final status.
