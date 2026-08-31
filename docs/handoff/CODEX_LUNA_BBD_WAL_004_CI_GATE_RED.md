# Codex Luna Handoff — BBD-WAL-004 CI Gate Expected Red

You are **Jr Dev — Codex Luna**. This durable file is the complete integration prompt;
ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely before acting: `AGENTS.md`, `TESTING.md`,
`docs/handoff/CURRENT_TASK.md`, `tickets/BBD-WAL-004.md`,
`docs/testing/BBD-WAL-004-CI-GATE-REVIEW-01.md`,
`docs/testing/BBD-WAL-004-CI-GATE-TEST-SOURCE-REVIEW-01.md`, and the complete accepted
`test/securityPolicy.node.js`.

## Preflight and sole task

Require `HEAD == origin/master` at the governance parent, a clean index, and exactly one
unstaged path: `test/securityPolicy.node.js`. Verify it is 2,063 lines with SHA-256
`6b48023598984d91499466869533cf5c4b2d3b6a697cac567753f225dc044493` and that
`git diff --check` passes. Stop on any extra path, index entry, line/hash mismatch, or
unintended diff.

Your sole task is to prove that the accepted Gitleaks and Rust SBOM policy regressions
fail against the current production policy/workflow/ignore/ticket state. Do not repair
or edit production policy, workflow, `.gitleaksignore`, ticket, tests, package files,
wallet source, or any other implementation path.

## Exact command and expected result

Run exactly once:

```text
node --test --test-name-pattern='strict nine-line reviewed Gitleaks ratchet bytes and content are enforced|WAL-004 manual SBOM contains separately validated npm and Rust CycloneDX JSON artifacts' test/securityPolicy.node.js
```

Expected exit is 1 after discovery of all 69 policy cases: 0 pass, exactly 2 fail, and 67
skipped by name pattern. The only executed failures must be:

1. `strict nine-line reviewed Gitleaks ratchet bytes and content are enforced`, because
   the current production ratchet still has the old eight-entry rationale/content; and
2. `WAL-004 manual SBOM contains separately validated npm and Rust CycloneDX JSON
   artifacts`, because the current production command still omits `--all-features`.

An exception outside those assertions, different count, unexpected pass, unrelated
failure, signal, setup issue, or mutation of any file is unintended red: stop and report.
Do not run the whole Node suite, npm, Rust, Cargo, formatters, builds, scanners, SBOM
generation, GitHub workflows, installs, Electron, wallets, nodes, devices, network,
cleanup, deletion, or any unlisted command.

## Evidence and Git

If and only if the red result is exact, create only
`docs/testing/BBD-WAL-004-CI-GATE-EXPECTED-RED.md` with the governance parent, exact
command/exit, TAP totals, the two names/reasons, accepted test line/hash integrity, and
confirmation that no other path changed or secret/canary appeared. Update only
`docs/handoff/CURRENT_TASK.md` to `CI GATE EXPECTED RED RECORDED — PRODUCTION CORRECTION
REQUIRED`, link the evidence, and state that production remains unauthorized pending
reviewer handoff.

Run `git diff --check`. Stage only the accepted test path, the expected-red evidence,
and `CURRENT_TASK.md`; inspect the exact staged names/diff. Commit once as:

```text
test: record WAL-004 CI gate red
```

Push `master`. Require final `HEAD == origin/master`, clean worktree, and report commit,
evidence line count/hash, exact TAP totals/failure reasons, accepted test hash, and final
status. Do not amend, rewrite history, force push, delete, clean, or use `/tmp`, root,
`sudo`, globs, substitutions, variables, or unresolved destructive targets.
