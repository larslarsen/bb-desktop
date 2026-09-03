# BBD-WAL-007 Slice-5 Correction-03 Source Review 01

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Decision: **REJECTED — ONE-FILE CORRECTION 04 REQUIRED**

Correction 03 changed only `wallet-broker/src/xmr/account.rs`, now 3,374 lines at
SHA-256 `8ab5650246afc1a657a91b7b013aa1c79995ee60ce4d78e0a34404db0adb05f6`.
All other Correction-02 identities remain exact, the frozen 588-line receiver test
remains SHA-256 `d8656c533d7b2d08eb9fa8831cd70da6e1da632ed9563fc2af569cab754d3622`,
and `git diff --check` is clean. The reviewer ran read-only source inspection only.

The Correction-03 node gate is accepted. A rejected node observation now causes no
wallet-RPC call, preserves `LOCKED` when applicable, otherwise reports wallet
`UNAVAILABLE`, and cannot refresh through a rejected bootstrap/untrusted node.

## Blocking finding

`issue_fresh` calls `port.prove_owned_identity()` before its durable receiver lookup.
For the production `SystemAccountPort`, that method rejects a locked wallet and requires
a live owned child. Thus an exact persisted request cannot replay while locked or while
the child is unavailable, even though replay needs only the already-authenticated
account store and performs no RPC. This contradicts the ticket's explicit order: an
exact durable `(account_id, network, request_id)` returns the stored receiver; otherwise
only a ready eligible account may create a new address.

Keep authority-latch, durable identity, network, schema, and binding proof before replay.
Move the live child/lock/session proof after the exact durable miss and before any new-
issuance preparation or RPC. Existing replay must never require wallet-RPC authority;
new issuance must retain every current eligibility and owned-session gate.

Principal Dev — Codex Sol at High may make the exact one-file Correction 04. Hermes
execution/integration remains unauthorized pending reviewer source acceptance.
