# Hermes Handoff — BBD-WAL-006 Prepare Serde Feature Expected Red 01

You are **Jr Dev — Hermes**. Own only this expected-red execution/evidence/integration gate.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Accepted changed test: `test/securityPolicy.node.js`, 2,525 lines, 75 named tests,
`2c1bd92c778a975b1218dfd2445b6b1b605bb047032fd5289573cbbb6d0a0169`.

The four accepted, uncommitted production paths must remain at the exact identities in Prepare
Format Correction Review 03. Read completely: `AGENTS.md`, `TESTING.md`, Prepare Gate Clippy
Review 01, Prepare Serde Feature Test Source Review 01, the Sol test handoff, and `CURRENT_TASK.md`.

## Preconditions

Record Hermes version/provider/model, protected `HEAD`, `origin/master`, status, exact five-path
diff inventory, accepted test/source identities, and `git diff --check`. Stop on any mismatch or
extra path. Do not modify source to make execution pass.

## Sole command

Run from repository root:

```text
node test/securityPolicy.node.js
```

Expected: exit 1, exactly 69 `ok`, exactly 6 `not ok`, final line
`6 security policy test(s) failed`. The additional failure beyond the five accepted Phase-C
partial-red names must be exactly:

```text
WAL-006 manifest requires six exact defaults-off pins and the minimum direct feature union
```

It must fail because the policy export/manifest still omit the required SQLite `serde` feature;
there may be no syntax/load error, different count/name, mutation, timeout, or other new failure.

## Evidence and integration

On the exact expected red, use `apply_patch` to create only
`docs/testing/BBD-WAL-006-PREPARE-SERDE-FEATURE-EXPECTED-RED-01.md` and update only the leading
state/actor/active-handoff block of `docs/handoff/CURRENT_TASK.md`. Record exact command/counts,
failure cause, identities, diff inventory, and negative capability.

Stage exactly:

- `test/securityPolicy.node.js`
- `docs/testing/BBD-WAL-006-PREPARE-SERDE-FEATURE-EXPECTED-RED-01.md`
- `docs/handoff/CURRENT_TASK.md`

Commit exactly `test: require WAL-006 prepare sqlite serde feature`, push `master`, and prove
`HEAD == origin/master`. The four production source paths must remain unstaged and uncommitted.
Do not amend, rebase, merge, or force-push.

Do not run Cargo/Rust, formatter, Clippy, another Node command, dependency resolution, network,
Electron, wallet/node/device, fixture, cleanup, or deletion. Do not edit manifest/lock, policy
implementation, production source, Rust tests, fixture, ticket, workflow, package, or another path.
Stop and report exact evidence on any mismatch.
