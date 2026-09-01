# Hermes Handoff — BBD-WAL-006 Prepare NFC Dependency Evidence Correction 01

You are **Jr Dev — Hermes**. This is a documentation-only correction; do not rerun the gate.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Integrated gate commit: `9cdaa562550f4dc898b09411fb92d820fa64501f`

Read completely: `AGENTS.md`, Gate Evidence Review 01, the existing gate evidence, and
`docs/handoff/CURRENT_TASK.md`.

## Sole task

Use `apply_patch` to edit only:

- `docs/testing/BBD-WAL-006-PREPARE-NFC-DEPENDENCY-GATE-01.md`
- `docs/handoff/CURRENT_TASK.md`

In the evidence:

- change only the provider value from `meituan/longcat-2.0:free` to `nous`; retain model
  `meituan/longcat-2.0:free` and the exact Hermes version;
- add a concise resolved-metadata sentence stating that the inspected published
  `unicode-normalization 0.1.25` package has `build = false`, `rust-version = 1.36`, and license
  `MIT OR Apache-2.0`, within the existing allowlist; and
- do not change any command, result, count, hash, lock diff, source identity, or capability claim.

Update only the leading state/actor/active-handoff block of `CURRENT_TASK.md` to say the evidence
correction is complete and reviewer acceptance is required, and link the completed correction
handoff without removing prior records.

Verify the protected Git state and the two resulting doc paths. Stage exactly those two paths,
commit exactly `docs: correct WAL-006 prepare NFC gate evidence`, push `master`, and prove a clean
worktree/index with `HEAD == origin/master`.

Do not execute Node, npm, Cargo, Rust, formatter, Clippy, test, policy, audit, scanner, dependency
resolution, network, Electron, wallet/node/device, fixture, cleanup, or deletion. Do not edit
source/test, manifest, lock, policy implementation, fixture, ticket, workflow, package, deny
policy, or another file. Do not amend, rebase, merge, or force-push. Stop on any mismatch.

