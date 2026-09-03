# Hermes Handoff — BBD-WAL-008 Slice-01 Evidence Correction 01

You are **Jr Dev — Hermes**. Repository: `/home/lars/OpenBazaar/bb-desktop`.

Protected governance parent: the commit containing this handoff

Read `AGENTS.md`, `docs/handoff/CURRENT_TASK.md`, the Slice-01 Green 01 evidence, and
its Evidence Review 01 completely. Require clean `HEAD == origin/master` at the
protected parent.

Edit only `docs/testing/BBD-WAL-008-SLICE-01-GREEN-01.md` and
`docs/handoff/CURRENT_TASK.md`. In the evidence:

- replace the abbreviated gate-command table entries with the complete command strings
  actually submitted, including each `cd /home/lars/OpenBazaar/bb-desktop &&` prefix;
- replace the false byte-for-byte/no-wrapper claim with the exact wrapper deviation and
  unchanged technical outcomes;
- state that `cd /home/lars/OpenBazaar/bb-desktop && node --version` was an unrequested
  non-mutating command used to obtain the recorded Node.js version; and
- retain the exact results, hashes, accepted falsification reference, and integration
  identity without claiming strict command compliance.

Update current task to say the correction is complete and awaits reviewer acceptance.
Run no formatter, compiler, Cargo, test, Node, npm, product, network, or other actor.
Use only read-only scope/identity inspection and `git diff --check`. Stage exactly the
two documentation paths, commit exactly `docs: correct WAL-008 slice one evidence`,
push `master`, verify clean `HEAD == origin/master`, and stop.
