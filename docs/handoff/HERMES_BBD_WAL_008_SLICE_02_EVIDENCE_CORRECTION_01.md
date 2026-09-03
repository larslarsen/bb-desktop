# Hermes Handoff — BBD-WAL-008 Slice-02 Evidence Correction 01

You are **Jr Dev — Hermes**. Repository: `/home/lars/OpenBazaar/bb-desktop`.

Protected governance parent: the commit containing this handoff

Read completely: `AGENTS.md`, both role/routing policies,
`docs/handoff/CURRENT_TASK.md`, `tickets/BBD-WAL-008.md`, Slice-02 Green 01 evidence,
and Slice-02 Green 01 Evidence Review 01.

This is documentation correction only. Run no formatter, test, build, lint, policy,
product, device, network, or actor command.

Preflight may verify only branch `master`, exact `HEAD == origin/master` at the
protected parent, clean index/worktree, and this frozen evidence identity:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `docs/testing/BBD-WAL-008-SLICE-02-GREEN-01.md` | 138 | `58c314e64a46777396948380336dfadfb12f61805ae003d38fdfba9f5afb49be` |

Edit only those two files. In the evidence, add an explicit transcript-deviation
section after integration/final-state reporting that records, in exact order:

1. post-integration `sha256sum wallet-broker/src/zec/store.rs`, returning
   `1ab107367c3f7af4581bba770dbb1943955b6ebe5ad0303d31c512fdab3e4b2a`;
2. the full literal focused persisted-narrowing test command, exit 0 and 1/0;
3. the same post-integration hash command and result a second time; and
4. the same full focused test command, exit 0 and 1/0, a second time.

State plainly that these four commands were unrequested, occurred after commit/push and
the required final repository proof, violated the stop/once-only contract, caused no
mutation, and are not part of the authorized acceptance sequence. Correct any blanket
claim that implies no commands ran after integration or that every execution command
ran only once. Preserve every original required-gate outcome and literal command.

Update only the leading active block of `CURRENT_TASK.md` to await reviewer acceptance
of Evidence Correction 01 and link this review plus the corrected evidence. Do not
rewrite historical content.

Stage exactly the two corrected records. Commit exactly
`docs: correct WAL-008 green evidence`, push `master`, then run only the minimum Git
final-state proof authorized by this handoff and stop. Do not amend or recreate
`369d811c`; do not edit source/test/evidence review/ticket/other documentation; do not
rerun any gate; do not use Grok or invoke another actor. On mismatch, stop without
mutation, commit, push, or post-stop command.
