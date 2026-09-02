# Codex Sol Handoff — BBD-WAL-007 Phase-C Slice 2 Process Resume 01

Status: AUTHORIZED — PRODUCTION SLICE 2 ONLY

Source actor: Principal Dev — Codex Sol (`gpt-5.6-sol`, High)

Reviewer: Lead Engineer/Reviewer — Codex; XHigh review required before execution

Protected governance parent: the commit containing this handoff

Accepted production baseline: `c139641ab59f50d931723e3a8a463d7de17aa1b7`

Accepted corrected test baseline: `1edf0c2f898481fe1d51d9959b68db52b3d28619`

Repository: `/home/lars/OpenBazaar/bb-desktop`

Read completely before acting: `AGENTS.md`, `TESTING.md`,
`tickets/BBD-WAL-007.md`, the corrected `wallet-broker/tests/xmr_process.rs`,
`docs/architecture/BBD-WAL-007-SLICE-02-OWNED-CHILD-DECISION.md`, Slice-2 Expected-Red
Acceptance 01, the original Slice-2 process handoff, and `docs/handoff/CURRENT_TASK.md`.

## Precedence and authorized paths

Resume the original Slice-2 process source task. Every objective, plan/config/entropy/
reservation/lifecycle/test-support requirement and prohibited action in
`CODEX_SOL_BBD_WAL_007_PHASE_C_SLICE_02_PROCESS.md` remains mandatory except its
process-group ambiguity. This handoff replaces that ambiguity with the exact-owned-child
contract below.

Edit only:

- `wallet-broker/src/xmr.rs` — 3 lines, SHA-256
  `b39ea4228dd25951701cda9595a2b521e3391a92a41904c9e61b68ea1368695b`;
- `wallet-broker/src/xmr/model.rs` — 93 lines, SHA-256
  `acb9391d22df344d7d72cbe139a0b87366be30d8c8f464e82e7db6e97ebddc47`;
- `wallet-broker/src/xmr/process.rs` — new;
- `wallet-broker/src/xmr/test_support.rs` — 368 lines, SHA-256
  `7819a2d7f20b49540346a16a53d94ea77e7acd21ceb17e48091e746eadc293b7`.

Every other path and repository is read-only. In particular the corrected test,
distribution, native/UI, policy, manifest, lockfile, evidence, and governance are frozen.

## Exact-owned-child replacement

The production system port owns the exact `std::process::Child` returned by the verified
spawn. On forced teardown it calls safe stable `Child::kill` on that retained handle,
then `wait`s/reaps that same child. It does not accept or signal a caller-provided PID,
look up a process by PID, signal a process group, use a helper/shell, add a dependency,
invoke FFI/libc/unsafe, require nightly, adopt another process, or claim descendant-tree
cleanup.

The centralized lifecycle operation exposed to the recording port is
`kill-exact-owned-child`, and `ProcessRig::killed_only_owned_child()` must report the
actual typed-port call with the same owned child identity. Preserve the two-second
boundary: delays at 1,999 and 2,000 ms do not force kill; 2,001 ms does. Hung,
stop-RPC-error, and unexpected child states use the exact forced-kill/reap order frozen
in the corrected test.

The official pinned wallet RPC is executed directly with detach/pidfile disabled. This
slice makes no claim about process-group signaling. All original requirements for a
verified executable capability, exact argv/config/environment, real OS entropy, retained
port reservation, private files, readiness, four-child cap, zeroization, cleanup,
failure isolation, and production-state-machine delegation remain unchanged.

## Prohibited actions and delivery

Do not run tests, Cargo, formatters, builds, binaries, Node, npm, package managers,
security tools, network, Git, or GitHub. Do not stage, commit, push, edit tests/evidence/
governance, or begin Slice 3.

Stop after the exact four-path source drop. Report paths, line counts/hashes, the
production-to-recording-port delegation shape, verified-executable consumption, entropy/
reservation design, exact-child teardown ownership, and confirmation that no prohibited
path or command was used. Reviewer XHigh acceptance is required before Hermes may run
the focused green gate.
