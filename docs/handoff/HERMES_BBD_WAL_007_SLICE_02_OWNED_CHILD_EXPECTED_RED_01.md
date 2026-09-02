# Hermes Handoff — BBD-WAL-007 Slice-2 Owned-Child Expected Red 01

You are **Jr Dev — Hermes**. This durable handoff is authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely before acting: `AGENTS.md`, `TESTING.md`,
`docs/engineering/HERMES_JR_DEV_ROUTING.md`, `tickets/BBD-WAL-007.md`,
`docs/architecture/BBD-WAL-007-SLICE-02-OWNED-CHILD-DECISION.md`, Slice-1 Acceptance
01, Slice-2 Owned-Child Test Source Review 01, the complete corrected test, and
`docs/handoff/CURRENT_TASK.md`.

## Sole task and stop rule

Integrate only the accepted one-file test correction, prove formatting without mutation,
and record its expected red against the still-absent process production boundary. You
are the execution, evidence, and Git actor. You may not design/edit tests, repair or
format source, add production, change a command, accept an unrelated failure, or run any
other gate.

Stop immediately on the first precondition mismatch, formatting failure/mutation, or
unexpected compiler result. After a stop, do not edit evidence, stage, commit, or push.

## Protected preconditions

Require:

- `HEAD == origin/master ==` the protected governance parent;
- a clean index;
- exactly one modified worktree path:
  `wallet-broker/tests/xmr_process.rs`, 374 lines, 12 tests, SHA-256
  `12cb52a5efca6a5ebfa53b1e856fc816c5ae7e8e01849b9034bd11d5a74d6f06`;
- every production source path equals accepted commit `c139641a`; and
- the manifest and lockfile retain the frozen Slice-1 hashes.

Require `git diff --check` clean. Record the actual Hermes version, provider, and model
from separate commands before execution:

```text
hermes --version
hermes config get model.provider
hermes config get model.default
```

Use the existing disk-backed repository target/cache only. Do not use `/tmp`, access the
network, inspect a Monero installation, or start any product/Monero binary.

## Exact commands and acceptance

Run once each, from the repository root, in this order:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo fmt --manifest-path wallet-broker/Cargo.toml --check
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test xmr_process
```

Require:

1. formatter exit 0 with no source or test mutation; and
2. Cargo exit 101 during compile, with diagnostics limited to the exact absent Slice-2
   production API: `bitbook_wallet_broker::xmr::process` and the corresponding absent
   process test-support types (`ChildExit`, `ProcessFault`, `ProcessRig`,
   `TeardownCause`, and `XmrNetwork`).

There must be no dependency, lock, syntax, formatting, toolchain, network, timeout,
personal-installation, linker, unrelated source, or runtime-test failure. The target must
run zero tests because its production imports cannot compile. Do not run another XMR
target or try to make it green.

## Exact-success integration

Only on the exact expected red, create
`docs/testing/BBD-WAL-007-SLICE-02-OWNED-CHILD-EXPECTED-RED-01.md`. Record the separate
Hermes identity fields, protected identities, formatter result/no-mutation proof, exact
normalized compiler failure, zero executed tests, architecture-decision reference,
scope, and prohibited-action confirmation. Do not record local artifact paths,
environment values, or raw sensitive output.

Update `docs/handoff/CURRENT_TASK.md` to
`PHASE C SLICE 2 OWNED-CHILD EXPECTED RED COMPLETE — REVIEW REQUIRED`, link the evidence,
and retain the ticket, decision, Slice-1 acceptance, routing, and prior-ticket records.

Stage explicitly only the corrected test, evidence, and `CURRENT_TASK.md`. Inspect the
staged names and diff. Commit exactly:

```text
test: correct WAL-007 owned-child teardown contract
```

Push `master`, then prove `HEAD == origin/master`, clean index, and clean tracked and
untracked worktree. Stop for reviewer acceptance. Production source, process execution,
Slice 3, broader acceptance, real Monero, and every other repository remain unauthorized.
