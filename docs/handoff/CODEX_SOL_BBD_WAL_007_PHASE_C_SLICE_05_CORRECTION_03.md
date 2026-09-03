# Codex Sol Handoff — BBD-WAL-007 Phase-C Slice 5 Correction 03

You are **Principal Dev — Codex Sol at High**, continuing the documented fill-in because
Grok's usage balance remains exhausted.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely before editing: `AGENTS.md`, `TESTING.md`, `tickets/BBD-WAL-007.md`,
the frozen receiver tests, both prior Slice-5 correction handoffs and source reviews,
and the complete current `wallet-broker/src/xmr/account.rs`.

Edit only `wallet-broker/src/xmr/account.rs`, starting at 3,374 lines and SHA-256
`14c41baa8e276e21e0405aab419454cb476e3e299fd09c4016df09e50b9bc5a6`.
Every other source path, test, manifest, lockfile, document, policy, workflow, fixture,
and repository must remain byte-exact.

Do not run Cargo, rustfmt, tests, Clippy, Node/npm, policy/security commands, builds,
wallet/Monero executables, package managers, network, Git, or GitHub. Do not stage,
commit, push, or maintain evidence. Leave source unstaged.

In `SystemAccountPort::production_view`, do not perform any wallet RPC after the local
node probe returns `NODE_UNAVAILABLE`. Preserve a locked wallet as `LOCKED` without
wallet RPC; for an otherwise unlocked wallet, construct the sanitized snapshot as wallet
`UNAVAILABLE`, with no wallet height or acquired balance, while preserving node
`NODE_UNAVAILABLE`. Wallet RPC is permitted only after the node probe has returned an
accepted syncing or ready observation. Do not collapse authentication, protocol,
identity, schema, durability, hard-fork, or other integrity failures into an ordinary
state. Preserve every other Correction-01/02 repair and Slice-1–4 invariant.

Stop on any need for another path, test edit, dependency change, or architecture change.
Report the resulting line count/SHA-256, the exact branch behavior, and residual concern.
