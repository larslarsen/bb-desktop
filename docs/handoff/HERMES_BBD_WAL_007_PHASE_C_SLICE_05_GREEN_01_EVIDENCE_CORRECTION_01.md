# Hermes Handoff — BBD-WAL-007 Slice 5 Green 01 Evidence Correction 01

You are **Jr Dev — Hermes**. This is a documentation-only correction.

Require `HEAD == origin/master == 64811deac78093e3e2a8e96d3039ed8b67d0b48c`, a
clean worktree/index, and the exact eleven-path commit scope. Read `AGENTS.md`, the
Green-01 evidence, Evidence Review 01, and `CURRENT_TASK.md` completely.

Edit only:

- `docs/testing/BBD-WAL-007-SLICE-05-GREEN-01.md`
- `docs/handoff/CURRENT_TASK.md`

Correct the evidence to record that after integration/push you ran the unauthorized
`node -e` package-script inspection, `npm run build`, and `npm run test`; that these
were a post-integration command-scope deviation; that they did not mutate the clean
repository; and that their results are not accepted or reusable as broader/final
evidence. Remove or correct every contradictory prohibition claim. Preserve the valid
pre-integration green results and identities.

Set `CURRENT_TASK.md` to `PHASE C SLICE 5 GREEN 01 EVIDENCE CORRECTED — REVIEW
REQUIRED`, link the evidence review, and state that source, execution, integration,
broader/final acceptance, and the real offline local-Monero gate are closed pending
reviewer acceptance.

Run no formatter, test, build, Clippy, check, Node/npm, policy/security, product,
Monero, or network command. Stage only the two documents. Commit exactly
`docs: correct WAL-007 slice 5 green evidence`, push `master`, then use read-only Git
commands only to report the new commit, exact two-path scope, and clean state.
