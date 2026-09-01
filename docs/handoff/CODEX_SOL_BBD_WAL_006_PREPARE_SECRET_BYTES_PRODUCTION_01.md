# Codex Sol Handoff — BBD-WAL-006 Prepare Secret-Bytes Production 01

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This durable handoff is
authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely before acting: `AGENTS.md`, `TESTING.md`, `tickets/BBD-WAL-006.md`, Prepare
Production Source Stop Review 01, Prepare Secret-Bytes Design Review 01, Prepare Secret-Bytes
Expected-Red Review 01, the complete `wallet-broker/src/vault.rs`, complete
`wallet-broker/tests/secret_hygiene.rs`, and `docs/handoff/CURRENT_TASK.md`.

## Sole production task

Edit only `wallet-broker/src/vault.rs`, starting at exactly 759 lines and SHA-256
`89d8ada8ad7050910a92a9daa38f9d93a18b86c4b7d1bde7a7e9b4a8adf8b62b`.

Implement the accepted type split:

- remove `drop_observer` from ordinary `SecretBytes`; it must own only its current
  `SecretSlice<u8>`;
- preserve `SecretBytes::new`, `len`, `is_empty`, `expose`, `replace`, `wipe_with`, redacted
  Debug/Display, and unconditional zeroizing Drop semantics;
- add public `ObservedSecretBytes`, owning one `SecretBytes`, the static label, and the existing
  unconstrained `Box<dyn WipeObserver>`;
- change only the return type/constructor body of `SecretBytes::new_observed` so it constructs and
  returns `ObservedSecretBytes` while preserving the same arguments and fallible result;
- give `ObservedSecretBytes` redacted Debug and Display implementations; and
- implement its Drop by calling the inner `SecretBytes::wipe_with` with its label and observer.
  The observer must see the actual post-zeroize length/state exactly once. The subsequent inner
  `SecretBytes` Drop must operate on the empty replacement and emit no second observation.

Do not add `unsafe`, manual `Send`/`Sync`, a `WipeObserver` supertrait/trait-object bound, Clone,
Deref, exposure/serialization API on `ObservedSecretBytes`, thread, channel, global, alternate
secret container, dependency, feature, conditional compilation, or test-only production branch.
Do not weaken zeroization or change an existing public error/type contract beyond the reviewed
`new_observed` return type. Repository search has exactly one caller, an inferred drop-only local;
do not edit it.

Frozen test: `wallet-broker/tests/secret_hygiene.rs`, 281 lines,
`dcebe361c7061b06b9ec6bb6fbea88ccb208069f7360ae14e2d7c6ca83b433a4`. Do not edit any test,
ZEC source, Cargo/lock, policy, docs, fixture, workflow, or another path.

## Delivery boundary

Use `apply_patch` only. Read-only local inspection is permitted. Do not run Cargo, Rust, tests,
formatter, Clippy, Node, policy, dependency, Git, network, wallet/node/device, cleanup, or deletion.
Do not stage, commit, or push.

Return the sole changed path with line count/SHA-256, exact field/API/Drop changes, and any
ambiguity. Hermes will integrate the accepted source and run the separately authorized green and
regression gate.
