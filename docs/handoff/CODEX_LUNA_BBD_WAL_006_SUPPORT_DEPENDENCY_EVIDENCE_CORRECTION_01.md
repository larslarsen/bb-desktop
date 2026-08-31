# Codex Luna Handoff — BBD-WAL-006 Support-Dependency Evidence Correction 01

You are **Jr Dev — Codex Luna**. This is an evidence-only correction; ephemeral chat is
not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff.

Read completely: `AGENTS.md`, `TESTING.md`, the support-dependency gate evidence,
`docs/testing/BBD-WAL-006-SUPPORT-DEPENDENCY-GATE-REVIEW-02.md`, and `CURRENT_TASK.md`.

Edit only `docs/testing/BBD-WAL-006-SUPPORT-DEPENDENCY-GATE-01.md`. Replace the sentence
that currently claims no policy changed with this exact truthful scope:

```text
No fixture, test, ZEC Rust source, policy beyond the accepted support-dependency manifest checker, or unrelated path was changed.
```

Preserve formatter-consistent wrapping. Do not change any command, count, hash, feature,
lock-diff, provenance, or other claim. Update only `docs/handoff/CURRENT_TASK.md` to
`SUPPORT-DEPENDENCY EVIDENCE CORRECTED — REVIEW REQUIRED` and link this review/correction.

Run no command except read-only inspection/hash/line/Git-state operations needed for the
evidence integration. Do not rerun Node/Cargo or edit source, tests, manifest, lockfile,
ticket, policy, fixture, or any unlisted path.

Stage exactly the evidence and current-task files, commit once as
`docs: correct WAL-006 support dependency evidence`, push `master`, and require
`HEAD == origin/master` with clean tracked worktree/index. Report commit, both staged
paths, corrected evidence line count/SHA-256, push, and final state.
