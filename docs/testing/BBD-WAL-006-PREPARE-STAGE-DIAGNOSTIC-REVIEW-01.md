# BBD-WAL-006 Prepare Stage Diagnostic Review 01

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Protected parent: `411e9ffc`

Result: **DIAGNOSTIC ACCEPTED — ARCHITECTURE DECISION REQUIRED**

Jr Dev — Hermes v0.18.2 used provider `nous` and model `meituan/longcat-2.0:free`. The sole exact
locked/offline happy-path test exited 101 with zero passed and one failed. Exactly one closed marker
appeared: `BBD-WAL-006-DIAGNOSTIC:create-pczt`. The public result remained `INTERNAL`. There was no
compilation/setup failure, network attempt, file/lock mutation, or second command.

Principal Dev — Codex Sol then removed all four temporary markers verbatim. The restored
`wallet-broker/src/zec/store.rs` is exactly 2,048 lines with SHA-256
`f9f66f98f33b8457c955125b77453be018397ab120f78618d52ed817200fcf34`; the accepted seven-path
inventory and clean diff check are restored.

The failure is conclusively inside the official `create_pczt_from_proposal` call, before
redaction, serialization, secret ownership, or parse. Pinned source requires `WalletWrite +
WalletCommitmentTrees` and enters mutable SQLite transaction/tree plumbing, while the accepted
adapter supplies a physically read-only connection. That mismatch is the leading source-supported
cause, but the public error erasure does not expose the exact upstream inner variant.

A second independent defect is already source-proven: after pinned IO finalization, protocol
padding spends have signatures and no dummy key, so the current post-finalization `dummy_sk()`
input count and all-action signature check are invalid. Retained public spend witnesses can
distinguish real inputs, but correcting that alone cannot clear the current `INTERNAL`.

No further source, execution, or integration is authorized. Continuing requires an explicit
architecture choice: retain physical read-only access through an upstream/read facade; revise the
boundary to a rollback-enforced writable transaction with byte-for-byte persistence tests; or add
an in-memory SQLite snapshot capability, which the current manifest policy explicitly rejects.
