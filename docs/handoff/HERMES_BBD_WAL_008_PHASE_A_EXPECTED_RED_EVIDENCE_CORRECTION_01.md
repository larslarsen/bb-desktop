# Hermes Handoff — BBD-WAL-008 Expected-Red Evidence Correction 01

You are **Jr Dev — Hermes**. Repository: `/home/lars/OpenBazaar/bb-desktop`.

Protected governance parent: the commit containing this handoff

Read `AGENTS.md`, `docs/handoff/CURRENT_TASK.md`, and
`docs/testing/BBD-WAL-008-PHASE-A-EXPECTED-RED-01-EVIDENCE-REVIEW-01.md` completely.
Require a clean worktree and `HEAD == origin/master` at the protected parent.

In `docs/testing/BBD-WAL-008-PHASE-A-EXPECTED-RED-01.md`, replace only:

```text
7d89a7e98fde65c69392a91e633436ccf93b65a2f8e826e22b5e8ec27804da530
```

with:

```text
7d89a7e98fde65c69392a91e633436ccf93b65a2f8e826e22b5eec27804da530
```

Update `docs/handoff/CURRENT_TASK.md` to state that the correction is complete and
both files await reviewer acceptance. Run only read-only identity/scope inspection and
`git diff --check`; run no formatter, Cargo, test, build, product, network, or other
actor command. Stage exactly those two documentation paths, commit exactly
`docs: correct WAL-008 expected-red evidence`, push `master`, verify clean
`HEAD == origin/master`, and stop.
