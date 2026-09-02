# Codex Sol Handoff — BBD-WAL-007 Phase-C Slice 2 Correction 01

Status: AUTHORIZED — REVIEW CORRECTION ONLY

Source actor: Principal Dev — Codex Sol (`gpt-5.6-sol`, High)

Reviewer: Lead Engineer/Reviewer — Codex; XHigh review required before execution

Protected governance parent: the commit containing this handoff

Repository: `/home/lars/OpenBazaar/bb-desktop`

Read completely before acting: `AGENTS.md`, `TESTING.md`, `tickets/BBD-WAL-007.md`,
Slice-2 Source Review 01, Slice-2 Port Preflight Decision, Slice-2 Exact Owned-Child
Decision, the original and resume Slice-2 handoffs, the complete current four-path
drop, the complete corrected `xmr_process` test, and `docs/handoff/CURRENT_TASK.md`.

## Exact path boundary

Edit only:

- `wallet-broker/tests/xmr_process.rs` — 374 lines — SHA-256
  `12cb52a5efca6a5ebfa53b1e856fc816c5ae7e8e01849b9034bd11d5a74d6f06`;
- `wallet-broker/src/xmr/process.rs` — 980 lines — SHA-256
  `d5069097b835d5a69f19da22ac5b5ec0af85c8202844db5fc449d575ccc64673`;
- `wallet-broker/src/xmr/test_support.rs` — 1,045 lines — SHA-256
  `e93ea3f4275c7eb816251efb6cc713c8a95c00c596af5d0c6d8290d30972ba9b`.

Freeze `wallet-broker/src/xmr.rs` at
`6c199bc807b16d80d19141dbe726d630dc6c2b89662253947ec280692af60fc6` and
`wallet-broker/src/xmr/model.rs` at
`704540f27a1d92c6e82281db01e8689b3f0dbc99c3f066311a806e0ecab437b7`.
Every other path and repository is read-only.

## Required correction

Retain the valid core and correct every blocking finding in Source Review 01:

1. Move per-account admission and the four-child cap into a production process-pool/
   coordinator boundary. The public production path used by later slices must go
   through it. Make `ProcessRig::pool`, `start_account`, failure, removal, and count
   delegate to that same production coordinator; test support may inject ports but may
   not own a duplicate `BTreeMap`/registry algorithm.
2. Separate `start` from ongoing `poll_health`. Expose the real production health path
   through the coordinator/wrapper, check the exact owned child's liveness and selected
   executable identity, and route every post-child startup/poll/broker-exit failure
   through the same close/stop/bounded-wait/exact-kill/reap/wipe/close/remove sequence.
   Do not restart a running process in order to poll it.
3. Gate Linux-only imports and system implementations so the crate still compiles on
   other targets and the production XMR constructor returns `UNAVAILABLE` before
   filesystem, socket, or process effects. The generic recording manager must remain
   testable without the host platform changing its result.
4. Correct the frozen test's `trusted-daemon` substring bug so it checks exact argv/
   config option names while still rejecting IPv6 values. Remove the helper's special
   omission: the inspected text/inventory must include the actual required
   `untrusted-daemon=1`. Replace hard-coded provenance answers with observations derived
   from the manager/port calls and exact derived paths. Keep all 12 existing test names
   and assertions unless a stronger assertion is necessary; do not weaken coverage.
5. Give every transient secret buffer/string/config value unwind-safe drop zeroization,
   including partial entropy on error and the combined RPC-login temporary. Do not add a
   dependency or expose `Debug`/cloneable production secret views.
6. Reject config-unsafe private roots containing newline, carriage return, NUL, or other
   control characters before directory, entropy, listener, config, or child effects.
7. Implement the Port Preflight Decision exactly: retain the real listener until the
   typed spawn boundary, release it immediately before spawn, do no intervening
   application work, check the exact owned child before and after authenticated
   exact-version readiness, and never claim atomic socket transfer.

Keep the exact-owned-child decision: stable safe `Child::kill`, then wait/reap the same
retained handle; no PID lookup, process-group claim, helper, dependency, FFI/libc,
`unsafe`, or nightly. Do not begin the RPC, account, store, receiver, hygiene, or local-
gate implementation.

## Prohibited actions and delivery

Do not run tests, Cargo, formatters, builds, binaries, Node, npm, package managers,
security tools, network, Git, or GitHub. Do not stage, commit, push, edit evidence or
governance, or touch another path.

Stop after the exact three-path correction. Report line counts/hashes and map each of
the seven findings to the corrected production/test path. Reviewer XHigh acceptance is
required before Hermes may execute or integrate the focused green gate.
