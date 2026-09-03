# Codex Sol Handoff — BBD-WAL-007 Phase-C Slice 5 Correction 04

You are **Principal Dev — Codex Sol at High**, continuing the documented fill-in because
Grok's usage balance remains exhausted.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely before editing: `AGENTS.md`, `TESTING.md`, `tickets/BBD-WAL-007.md`,
the frozen receiver tests, the Slice-5 correction handoffs/reviews, and the complete
current `wallet-broker/src/xmr/receiver.rs` and production `ReceiverPort` implementation
in `wallet-broker/src/xmr/account.rs`.

Edit only `wallet-broker/src/xmr/receiver.rs`, starting at 870 lines and SHA-256
`cc3b001d680aa9d659f8cd43e7312349a6ee0e4fa965063cd66a78f0a108619c`.
Every other source path, test, manifest, lockfile, document, policy, workflow, fixture,
and repository must remain byte-exact.

Do not run Cargo, rustfmt, tests, Clippy, Node/npm, policy/security commands, builds,
wallet/Monero executables, package managers, network, Git, or GitHub. Do not stage,
commit, push, or maintain evidence. Leave source unstaged.

In `issue_fresh`, preserve validation, the lifetime authority latch, the durable stored-
identity load/comparison, and the exact durable lookup before returning a replay. Do not
require a live/unlocked wallet child for that replay. Apply `prove_owned_identity`—the
production live child/lock/session gate—only after a durable lookup miss and before
`prepare_receiver`, eligibility checks, address creation, or any other new-issuance RPC.
New issuance must retain all current session, node, wallet, watch-only, exhaustion,
validation, persistence, and fail-closed behavior. Do not change the frozen tests or
weaken corruption handling.

Stop on any need for another path, test edit, dependency change, or architecture change.
Report the resulting line count/SHA-256, exact replay/new-issuance order, and residual
concern.
