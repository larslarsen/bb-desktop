# Hermes Handoff — BBD-WAL-007 Phase-C Slice 3 Green 02 Evidence Correction 01

You are **Jr Dev — Hermes**. This durable handoff is authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely before acting: `AGENTS.md`, `TESTING.md`, Green Evidence 02, Green
Evidence 02 Review 01, Resume 07, and `docs/handoff/CURRENT_TASK.md`.

## Sole task

Require `HEAD == origin/master ==` the protected governance parent, a clean index and
tracked/untracked worktree, and `git diff --check` clean. Edit only:

- `docs/testing/BBD-WAL-007-SLICE-03-GREEN-02.md` — 53 lines, SHA-256
  `c831d510de4fe94c33464f4e038f75f084d02f89b031f9dd8306782064a1f697`;
- `docs/handoff/CURRENT_TASK.md` — change only its state and active-result text as
  specified below.

Make only these Green Evidence 02 corrections:

1. In the falsification result, record the complete count as `0 passed, 1 failed, 0
   ignored, 0 measured, 14 filtered out`.
2. Replace the sentence `All Rust commands emitted no warning or diagnostic. No
   accepted source/test mutated.` with the exact meaning that the formatter,
   falsification compile stage, and normalized green Rust commands emitted no warning
   or compile diagnostic; the falsification emitted only the expected runtime failure
   already recorded; and no accepted source/test remained mutated after restoration.
3. Replace the scope paragraph with the exact meaning that Resume 07 integrated only
   the accepted warning correction in `wallet-broker/src/xmr/rpc.rs`, Green Evidence
   02, and `CURRENT_TASK.md`; `6eb566d6` integrated the base Slice-3 RPC transport;
   `c4bda0e9` newly integrated the warning correction; and no other source/test was
   integrated.
4. In the prohibited-action confirmation, replace the broad
   `no command wrapper/redirection/pipeline/reuse` claim with the exact meaning that no
   formatter, falsification, or normalized-green command used a wrapper, redirection,
   or pipeline, and no Resume-06 result was reused.

Preserve every other identity, hash, result, count, scope, and prohibited-action
statement. Do not run or edit tests, source, policy, Cargo, formatters, Node, builds,
binaries, package managers, network, or any gate. Do not begin Slice 4 or the real
local-Monero gate.

Update `CURRENT_TASK.md` to
`PHASE C SLICE 3 GREEN 02 EVIDENCE CORRECTED — REVIEW REQUIRED`, link this completed
evidence correction and Green Evidence 02 Review 01, and state that no source actor or
integration actor is authorized pending reviewer acceptance. Preserve every other
line and history.

Stage explicitly only those two documentation paths, inspect the staged names/diff,
and commit exactly:

```text
docs: correct WAL-007 Slice-3 green evidence
```

Push `master`, prove `HEAD == origin/master` and a clean index and tracked/untracked
worktree using read-only Git commands, then stop. The reviewer alone accepts Slice 3
and authorizes any later slice or real local-Monero gate.
