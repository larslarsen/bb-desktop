# WAL-009 Correction 02 — transaction buffer cleanup only

Actor: Sr Dev — Grok Build, High.
Repository: `/home/lars/OpenBazaar/bb-desktop`.
Governance parent: the commit containing this handoff.

This replaces Correction 01's broad authorization. Fix only finding 4 in
`docs/testing/BBD-WAL-009-PHASE-A3-GROK-CORRECTION-01-REVIEW.md`.
The owner is concerned about usage. Keep this a short, one-file task; do not attempt
to complete WAL-009 or revisit cryptography, UI, or the test harness.

Required reading, once: `AGENTS.md`, `TESTING.md`, this handoff, the leading active
section of `docs/handoff/CURRENT_TASK.md`, and the above review. Inspect
`wallet-broker/src/zec/spend.rs`, `vault.rs` secret/observer definitions, and the
existing `zec_sign_verify.rs` component-fault and wipe tests. These are the complete
task-specific reading requirements; do not recursively load historical handoffs,
acceptance records, entire dependency trees, or unrelated repository files.

## Exact source scope

Edit only `wallet-broker/src/zec/spend.rs`.
Starting identity: 837 lines; SHA-256
`dc2773cdf3bcf87f596b8acb3a89f9ea6d0c7479ca5a330ca4374b9aa065a5f9`.

The already accepted Phase-A1 tests require cleanup on success, faults, and unwind;
their expected red was accepted at `d7819e0f`. This is a bounded correction to that
unaccepted production implementation. Do not change tests or claim that the existing
dummy harness observations validate the repair.

## Required implementation

The buffer written by `Transaction::write` must have an RAII zeroizing owner before
serialization begins. Keep that owner through mutation, decoding, and verification.
Its destructor wipes its actual bytes before calling the existing WipeObserver with
label `zec-extracted-transaction`, original live length, and actual zero inspection.
The same owner must clean up on serialization failure, mutation failure, decode
failure, cryptographic/intent rejection, injected faults, normal success, and panic
unwind. An end-of-function wipe or a newly allocated dummy buffer is insufficient.

Use the already available zeroize crate and a small local private guard borrowing the
observer if needed; no new dependency, public API, or vault.rs edit is authorized.
Avoid raw buffer clones, ownership escapes, and manual loops that may be optimized
away. Wipe any tail before truncation in the malformed-state hook. Ensure partial
serialization remains owned on error. Cleanup must happen before success publication;
do not emit duplicate wipe observations. Remove the superseded manual wipe helper if
it has no remaining callers. If needed, use explicit drop to release the observer
borrow. Never invoke an observer before wiping the actual buffer.

Preserve all existing error codes, pipeline ordering, mutation semantics, and
cryptographic calls. Do not fix the undeclared orchard dependency, copied verification
metadata, synthetic cleanup counters, native UI, or any other rejection finding in
this pass. Those remain explicitly unresolved and prevent full acceptance.

## Stop

Read-only shell inspection and apply_patch edits only. Run no formatter, build, test,
compiler, Cargo/npm, dependency, audit, scanner, Git, network, product, device, wallet,
or other actor command. Do not edit any other file, including the unrelated modified
package.json/package-lock.json or the other six pending Rust files.

Report the resulting line count/SHA-256, a short description of how all return paths
reach the actual wipe, and any unresolved obstacle. Stop immediately afterward.
Reviewer inspection comes next. Hermes execution requires a separate handoff after
the remaining compilation and oracle blockers are resolved; none is authorized now.
