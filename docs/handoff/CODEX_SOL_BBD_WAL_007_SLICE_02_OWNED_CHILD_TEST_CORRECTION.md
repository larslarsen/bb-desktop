# Codex Sol Handoff — BBD-WAL-007 Slice-2 Owned-Child Test Correction

Status: AUTHORIZED — ONE TEST PATH ONLY

Source actor: Principal Dev — Codex Sol (`gpt-5.6-sol`, High)

Reviewer: Lead Engineer/Reviewer — Codex; XHigh review required before execution

Protected governance parent: the commit containing this handoff

Repository: `/home/lars/OpenBazaar/bb-desktop`

Read completely before acting: `AGENTS.md`, `TESTING.md`,
`tickets/BBD-WAL-007.md`, the complete `wallet-broker/tests/xmr_process.rs`,
`docs/architecture/BBD-WAL-007-SLICE-02-OWNED-CHILD-DECISION.md`, Slice-1 Acceptance 01,
the superseded Slice-2 process-source handoff, and `docs/handoff/CURRENT_TASK.md`.

## Sole task and authorized path

Edit only:

- `wallet-broker/tests/xmr_process.rs` — 374 lines, 12 tests, SHA-256
  `db0bb2272fb145a2317884dd98ed339cc248ed28b0250802f2462a0f88a781e0`.

Every other path and repository is read-only. No production source is authorized.

## Exact correction

In the existing hung/failed-child teardown test only:

- rename the test to state that the exact owned child is killed then reaped;
- replace the expected operation `kill-exact-process-group` with
  `kill-exact-owned-child`; and
- replace the assertion accessor `killed_only_owned_process_group()` with
  `killed_only_owned_child()`.

Preserve the three `ChildExit` cases, complete teardown order, child/open-handle zero
assertions, and every other byte unless rustfmt-consistent line wrapping of these exact
edits requires movement. Keep exactly 12 tests. Do not add PID input, process-group,
helper, FFI, unsafe, nightly, dependency, conditional, ignored, fake-success, or future
source behavior.

## Prohibited actions and delivery

Do not run tests, Cargo, formatters, builds, binaries, Node, npm, package managers,
security tools, network, Git, or GitHub. Do not stage, commit, push, edit production,
policy, evidence, governance, manifests, or locks.

Stop after the one-path test-source correction. Report its line count/hash, unchanged
12-test count, exact three semantic substitutions, and confirmation that no prohibited
path or command was used. Reviewer XHigh acceptance is required before Hermes may
integrate the corrected expected-red test contract.
