# Hermes Handoff — BBD-WAL-006 Prepare NFC Dependency Expected Red 01

You are **Jr Dev — Hermes**. Own only this execution/evidence/integration gate.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Accepted source drop:

- `test/securityPolicy.node.js`: 2,525 lines,
  `a70d3491c359b8d59a3b7cdb1307ee549b9c0cf8dad310570a47157d066d68ba`
- 75 named tests

Read completely: `AGENTS.md`, `TESTING.md`, Prepare Design Review 01, Prepare NFC Dependency Test
Source Review 01, the Sol handoff, `docs/handoff/CURRENT_TASK.md`, and the complete changed test.

## Preconditions

Record the actual Hermes Agent version, provider, model, protected `HEAD`, `origin/master`, status,
diff inventory, line count, SHA-256, and `git diff --check`. Stop on any source path or hash other
than the one accepted above. Do not modify source to make execution pass.

## Sole command

Run from repository root:

```text
node test/securityPolicy.node.js
```

Expected result: exit 1, exactly 68 `ok`, exactly 7 `not ok`, and final line
`7 security policy test(s) failed`. The seven failures must be exactly the six accepted frozen
Phase-C partial-red names plus:

```text
WAL-006 prepare NFC dependency is one exact defaults-off Unicode normalization pin
```

The new failure must be the missing exact real-manifest declaration, before any missing policy
export. Any syntax/load error, different count/name, timeout, mutation, or additional failure is a
stop, not evidence.

## Evidence and integration

On the exact expected red, use `apply_patch` to create only
`docs/testing/BBD-WAL-006-PREPARE-NFC-DEPENDENCY-EXPECTED-RED-01.md` and update only the leading
state/actor/active-handoff block of `docs/handoff/CURRENT_TASK.md`. Record exact command/output
counts, the specific missing-manifest assertion, protected identities, changed inventory, and the
negative capability record.

Stage exactly:

- `test/securityPolicy.node.js`
- `docs/testing/BBD-WAL-006-PREPARE-NFC-DEPENDENCY-EXPECTED-RED-01.md`
- `docs/handoff/CURRENT_TASK.md`

Commit exactly `test: require WAL-006 prepare NFC dependency`, push `master`, and prove
`HEAD == origin/master` with a clean worktree/index. Do not amend, rebase, merge, force-push, or
touch any other path.

Do not run Cargo/Rust, npm, formatter, Clippy, another Node test, policy implementation, scanner,
dependency resolution, network fetch, Electron, wallet/node/device, fixture, cleanup, or deletion.
Do not edit the manifest, lockfile, policy implementation, production source, frozen Rust tests,
fixture, ticket, workflow, or package files. Stop and report exact evidence on any mismatch.

