# Hermes Handoff — BBD-WAL-006 Integration Evidence Correction 01

You are **Jr Dev — Hermes** using only the free configured Nous route. Correct two stale
classification statements and finish the already-pushed integration record. Do not alter source,
tests, manifests, policy, lockfiles, or prior gate results.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely: `AGENTS.md`, `TESTING.md`, `tickets/BBD-WAL-006.md`, this handoff,
`wallet-broker/src/zec/store.rs` at integration commit
`be3b23ec295f144d532ae21df95c15223efeeefa`,
`docs/testing/BBD-WAL-006-PREPARE-ROLLBACK-GATE-01.md`, and `CURRENT_TASK.md`.

## Exact scope

Edit only:

- `docs/testing/BBD-WAL-006-PREPARE-ROLLBACK-GATE-01.md`, starting at 129 lines and SHA-256
  `7298f77aa612667d6684302f9928541ac3a04f85bc549417d8538ce5e24033ae`;
- the leading state/actor/active-handoff block of `docs/handoff/CURRENT_TASK.md`.

## Mandatory corrections

1. In `One unsigned real Ironwood action`, replace the stale witness discriminator with the
   accepted source truth: both actions may retain witnesses; the authorization-required real
   action is the one without `spend_auth_sig`; exactly one such action is required.
2. In `One IO-finalized signed padding action`, state that pinned upstream IO Finalizer signs the
   protocol-padding dummy, exactly one signed padding action is required, and that signature is
   excluded from `inspection.has_signatures` only after the exact two-action/one-unsigned/
   one-signed shape passes.
3. Correct the focused-pass sentence so it says the result was reported by the completed Hermes
   handoff, not recorded in an evidence file that does not exist.
4. Preserve every command, count, identity, byte-rollback statement, negative-capability result,
   integration commit, and final result verbatim.
5. Update the leading current-task block to state that preparation is integrated and its evidence
   corrected, with no authorized source or integration actor and this handoff listed completed.
   Keep integration commit `be3b23ec295f144d532ae21df95c15223efeeefa`; remove the duplicate
   consecutive stopped-final-gate entry.

Use `apply_patch` only for edits. Run only read-only precondition/diff checks. Do not run Rust,
Cargo, Node, tests, formatters, network, cleanup, or deletion.

After exact review, stage only the two changed documents, commit exactly
`docs: correct WAL-006 integration evidence`, push `master`, and prove clean worktree/index with
`HEAD == origin/master`. Do not amend, rebase, merge, or force-push.
