# Codex Luna Handoff — BBD-WAL-004 RustSec Resume and Integration

You are **Jr Dev — Codex Luna**. This durable file is the complete prompt; ephemeral
chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff.

Read `AGENTS.md`, `TESTING.md`, roles, `tickets/BBD-WAL-004.md`, GREEN_6 handoff,
`docs/testing/BBD-WAL-004-GREEN-RUN-06.md`, `CURRENT_TASK.md`, and the complete current
worktree state.

Require `HEAD == origin/master` at the governance parent, clean index, exactly the same
15 production and six accepted test paths/hashes from GREEN_6, immutable lockfile SHA
`1d182550f82b9dc443804d82e709b5d58021d4243bef1612c440aac59df1c2f5`, no extra path,
and `git diff --check` green. No source/test/policy/workflow mutation is authorized.

Run exactly:

```text
/home/lars/.cargo/bin/cargo-audit audit --file wallet-broker/Cargo.lock
```

Outbound network is authorized solely for cargo-audit 0.22.2 to fetch/update the RustSec
advisory database and audit that exact lockfile. If the sandbox denies this exact
operation, request/run this same command with the execution environment's network
escalation and no broader prefix or command. Do not install, update project dependencies,
change a lockfile, use `/tmp`, or run another gate. Any advisory or non-network error is
a blocker; stop without staging.

On a clean audit, create only `docs/testing/BBD-WAL-004-GREEN.md` with the complete
GREEN_6 passed-command/count record, cargo-audit version/result, prior formatter
contingency/final hashes, one accepted compatibility test and five formatter-only test
paths, no-canary/scratch result, deferred cargo-deny/cyclonedx note, and every final
hash. Update only `docs/handoff/CURRENT_TASK.md` to `PRODUCTION GREEN INTEGRATED —
FALSIFICATION AND CI SECURITY/SBOM GATES PENDING` and link evidence.

Run `git diff --check`. Stage exactly the 15 production paths, six accepted Rust tests,
evidence, and `CURRENT_TASK.md`; inspect the complete staged diff and names. Commit once
as `feat: add encrypted wallet custody core` and push master. Require final
`HEAD == origin/master`, no non-ignored change, and report the audit, commit, full
manifest, evidence line/hash, all prior results/counts/tools, final hashes, and push. Do
not manually trigger GitHub. Reviewer owns falsification and workflow review.
