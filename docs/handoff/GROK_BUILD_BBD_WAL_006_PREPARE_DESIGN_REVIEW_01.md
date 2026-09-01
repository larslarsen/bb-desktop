# Grok Build Handoff — BBD-WAL-006 Prepare Design Review 01

You are **Senior Dev — Grok Build**. This is a bounded read-only design review; do not edit.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely before answering: `AGENTS.md`, `TESTING.md`, `tickets/BBD-WAL-006.md`,
`docs/architecture/BBD-WAL-006-UPSTREAM-REVIEW.md`, Scan Integration Review 01, the complete
current `wallet-broker/tests/zec_prepare.rs` and `zec_hygiene.rs`, all current ZEC production and
test-support source, the manifest/lockfile, and the exact pinned upstream source for
`zcash_client_backend 0.24.0`, `zcash_client_sqlite 0.22.0`, `pczt 0.9.3`,
`zcash_primitives 0.30.1`, `zcash_protocol 0.10.5`, and `zcash_keys 0.16.1`.

## Questions to resolve

1. Identify the exact stable upstream call graph and public types that can produce an unsigned v6
   PCZT with an Ironwood input/output bundle from the scanned `WalletDb`, using the official
   standard fee rule. Distinguish proposal creation, PCZT creation, and inspection. Name all
   feature gates and generic bounds.
2. Prove how source can constrain selection to confirmed spendable Ironwood only, ignore legacy
   pools when Ironwood is sufficient, and return the frozen `MIGRATION_REQUIRED`,
   `CAPABILITY_MISSING`, or `INSUFFICIENT_FUNDS` outcomes without substituting account total for
   Ironwood spendable value. Flag any upstream selector that could silently mix pools.
3. Determine whether unsigned PCZT creation requires seed/USK material or only viewing/proposal
   data. If spending material is required, identify the narrowest derivation/lifetime and how to
   prevent signing, proving, finalizing, extraction, or persistence. Do not propose a mock PCZT or
   locally invented transaction format.
4. Identify public, non-authority-bearing inspection APIs/fields that can independently verify
   network/branch, transaction version, destination, amount, memo commitment, exact fee, Ironwood
   input/output counts, empty transparent/Sapling/Orchard-output bundles, and absence of
   signatures/proofs/finalization/extractability. Do not invoke signer, prover, spend finalizer,
   transaction extractor, or raw transaction serialization.
5. Map the accepted adapter/session/store shape to bounded opaque in-memory handles: exact maximum,
   collision handling, no eviction, binding to account/session/request/intent, expiry recheck, and
   destruction on lock/timeout/cancel/replacement/error/panic/drop. Separate production behavior
   from hidden typed test seams.
6. Inventory the smallest production path set likely required from `zec.rs`, `zec/prepare.rs`,
   `zec/store.rs`, `zec/test_support.rs`, and any unavoidable existing module. Identify any
   required dependency/feature/schema/test change; such a change is not authorized and is a
   blocker unless already present.
7. Report any pinned API contradiction, private method, unstable feature, missing Ironwood
   support, test-contract ambiguity, or security boundary that prevents an honest implementation.

Return a concise design with exact source citations, call graph, value/pool/error mapping,
authority analysis, handle lifecycle, proposed source boundary, and explicit blockers. Do not run
Cargo, Rust, formatter, Clippy, tests, Node, policy, Git, network, fixture, wallet, node, device,
cleanup, or deletion commands. Do not create, edit, stage, commit, or push any file. Read-only
local source inspection only.
