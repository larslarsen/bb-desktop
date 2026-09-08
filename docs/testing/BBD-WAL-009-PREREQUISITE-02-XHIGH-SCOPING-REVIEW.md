# WAL-009 prerequisite 02 XHigh scoping review

Reviewer: Codex at XHigh, resumed by the owner after the reasoning checkpoint.
Baseline: `a5650948316bb06858ef29b50e2e1948ded114da` plus the preserved pending
source identities in prerequisite check 02. This is static source/API review;
no compiler, formatter, test, or acceptance command was run by the reviewer.

Decision: open only the one-file Hermes evidence correction. Record the source
boundaries below for subsequent test-first handoffs. No source repair, test-source
edit, execution, or integration is authorized by this review.

## Signature-hash authorization context

`spend.rs:517` passes TransactionData<Authorized> to signature_hash. In the pinned
zcash_primitives 0.30.1, this uses transparent::bundle::Authorized, which does not
implement TransparentAuthorizingContext. A transaction with no transparent bundle
still must satisfy the function's static type bound. A cast, fake previous-output
context, skipped verification, or dependency change is not an approved solution.

The bounded design is a private verification authorization marker with these
associated types:

- TransparentAuth from `<pczt::EffectsOnly as Authorization>::TransparentAuth`;
- SaplingAuth from `<zcash_primitives::transaction::Authorized as Authorization>::SaplingAuth`;
- OrchardAuth from `<zcash_primitives::transaction::Authorized as Authorization>::OrchardAuth`.

This is a Rust type adapter, not a new cryptographic algorithm or capability.
pczt 0.9.3 publicly exposes EffectsOnly under the already-used signer feature;
the marker can use the associated type through existing direct dependencies.
No direct transparent/Sapling dependency needs to be added.

The later implementation should consume the independently decoded Transaction,
retain its library-derived txid locally, and move its data through the pinned
TransactionData::try_map_bundles API. Its transparent closure must return
INTENT_MISMATCH for every Some(bundle), including an empty bundle, and yield None
only for an actually absent bundle. Sapling and Orchard authorization payloads
must move unchanged. The pinned API applies the Orchard closure independently
to the Orchard and Ironwood fields; neither field may be swapped or discarded.
All version/branch/header fields remain library-owned and preserved. Unsupported
pool/version/branch shapes must fail closed under the ticket's precedence.

Compute TxIdDigester and call the existing signature_hash with SignableInput::Shielded
on that same typed data. Verify the actual decoded Ironwood spend signatures,
binding signature, and proof. Preserve existing error returns and post-sign
checks. Do not clone or reserialize the transaction to satisfy the type system,
manufacture an EffectsOnly bundle, substitute a signer-reported hash, or return
success merely because the type conversion succeeds.

Pinned references inspected:

- zcash_primitives 0.30.1, transaction/mod.rs: Authorization, Authorized,
  Transaction::into_data, and TransactionData::try_map_bundles;
- transaction/sighash.rs and sighash_v6.rs: generic authorizing-context bound and
  v6 digest construction;
- transaction/txid.rs and sighash_v5.rs: txid and transparent digest construction;
- pczt 0.9.3, lib.rs: public EffectsOnly associated types and feature gating;
- pczt roles/signer/mod.rs: Signer::shielded_sighash.

The pinned source also implies that a v6 transaction with no transparent bundle
has equal txid and shielded-sighash bytes. This review deliberately retains the
typed signature_hash route: no production shortcut based on that equivalence is
authorized. No custom hash or transaction/signature codec is introduced.

## Tests required before accepting that repair

The next signature-context handoff must bound test source before production source.
The existing native-widget missing-import error is not its behavioral red oracle.
Use a private test boundary over real pinned fixture transactions, not the existing
harness's self-reported verification or wipe flags. Required proofs include:

1. A real v6 Ironwood fixture's decoded verification data preserves all transaction
   fields/bundle domains and computes the same shielded message as the pinned PCZT
   Signer::shielded_sighash route. The oracle must be obtained independently of the
   helper under test; comparing the helper with itself is insufficient.
2. A present transparent bundle is rejected before signature/proof work; none of
   its inputs/outputs may be silently dropped to obtain a shielded-only view.
3. The valid fixture's real spend and binding signatures verify against that message,
   while an altered message or real authorization bytes fail. Preserve the separate
   real proof-failure regression. Exercise actual verifier calls.
4. Falsification must show that suppressing transparent rejection and corrupting the
   computed message each breaks the relevant test, with exact source restoration.

The production helper/interface and exact execution commands are to be frozen in
that later handoff, after the evidence correction. These requirements do not open
a broad rewrite or waive the ticket's independent recovered-effects requirement.

## Other compile prerequisites

- `spend.rs:669`: borrow the recipient Option before mapping to a slice, retaining
  the exact receiver comparison and error. Do not return a reference to a copied
  closure-owned address or replace the comparison with a reported boolean.
- `spend.rs:766`: zkproof is a borrowed Option<Vec<u8>>. Use its actual Option API
  to preserve the intended single owned proof value; do not follow the compiler's
  iterator suggestion into a different result shape. Proof/signature retention in
  ClearEffects remains subject to the unaccepted secret-lifetime work; a compile
  fix cannot be reported as positive wipe evidence or introduce additional copies.
- `test_support.rs:1326`: map all six PipelineFault variants explicitly to the
  corresponding WipeExit (or matching FaultPoint). No catch-all Success/Error
  fallback; this type repair does not validate the fabricated cleanup observations.
- `test_support.rs:1131`: remove only the unused mut from the PipelineOutcome
  parameter; do not alter ownership or publication semantics.
- `native_ui.rs:117`: any API-only prerequisite adaptation to Context::run_ui must
  use egui 0.36.1's Ui callback and dispose output correctly. It still cannot receive
  a real native click from one fresh default-input frame. Do not conflate that
  compilation fix with the separately fixed modal/event/capability contract.

The later source work must keep these mechanical fixes distinct from the
signature-context test/production slice and the native modal implementation.
Exact paths, hashes, and allowed edits must be named before launching Grok.

## Acceptance and next actor

The six widget tests and delimiter repair remain source-accepted; prerequisite
check 02 remains a valid mixed compile stop with zero tests run. Full independent
transaction-effect recovery and observation of real secret cleanup remain blockers.
This review neither accepts the current VerifiedEffects construction from prepared
metadata nor the canary-only cleanup counters.

Hermes alone may perform
[prerequisite evidence correction 01](../handoff/HERMES_BBD_WAL_009_PREREQUISITE_02_EVIDENCE_CORRECTION_01.md).
It must recover the existing transcript mechanically and write only the named
evidence record. No test rerun, source change, Git integration, or other actor is
authorized. Collect once only after the owner's completion report; do not poll.

Reviewer publication scope: this review, that handoff, docs/handoff/CURRENT_TASK.md,
and tickets/BBD-WAL-009.md. Implementation evidence and source are excluded.
