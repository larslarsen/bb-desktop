# Hermes Handoff — BBD-WAL-007 Slice-2 Owned-Child Red Evidence Correction 01

You are **Jr Dev — Hermes**. This durable handoff is authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely before acting: `AGENTS.md`,
`docs/architecture/BBD-WAL-007-SLICE-02-OWNED-CHILD-DECISION.md`,
`docs/testing/BBD-WAL-007-SLICE-02-OWNED-CHILD-EXPECTED-RED-REVIEW-01.md`, the complete
expected-red evidence, and `docs/handoff/CURRENT_TASK.md`.

## Sole task

Make one evidence-only semantic correction. Edit only:

- `docs/testing/BBD-WAL-007-SLICE-02-OWNED-CHILD-EXPECTED-RED-01.md` — 68 lines,
  SHA-256 `41ab2f2592ec49ce1be509ce319f255a871d1c81f46a255432ff1289780a8dcf`;
- `docs/handoff/CURRENT_TASK.md` — change only its state and active result text as
  specified below.

Require `HEAD == origin/master ==` the protected governance parent, clean index and
worktree, and `git diff --check` clean before editing.

Replace the first two sentences under `## Architecture-decision reference` with this
exact meaning:

```text
Slice 2 requires production code that can kill and reap the exact broker-owned child.
The XHigh decision documents why the original group-signal assertion was impossible
under safe stable Rust and replaces it with exact-owned-child semantics.
```

Preserve the following expected-red/non-vacuity sentence and every command, diagnostic,
identity, hash, count, scope, and prohibited-action statement. Do not run or edit tests,
source, policy, Cargo, formatters, Node, builds, binaries, package managers, network,
or any gate.

Update `CURRENT_TASK.md` to
`PHASE C SLICE 2 OWNED-CHILD EXPECTED RED EVIDENCE CORRECTED — REVIEW REQUIRED`, link
the corrected evidence, and state that no production source is authorized pending
reviewer acceptance.

Stage explicitly only those two documentation paths, inspect the staged names/diff,
and commit exactly:

```text
docs: correct WAL-007 owned-child red evidence
```

Push `master`, prove `HEAD == origin/master` and a clean tracked/untracked worktree, then
stop. Do not authorize or begin process source.
