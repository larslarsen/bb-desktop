# WAL-009 decoded transaction effects: XHigh decision and test-source task

Reviewer: Codex, XHigh, as requested by the owner on 2026-09-08.
Actor: Sr Dev — Grok Build, CLI grok-4.6, High; source only.
Baseline governance: 00b0ca29b93a18cd12edca497c0e609cfc8dfd36.
Current authorization: TEST SOURCE ONLY. No Hermes execution/integration yet.

## Finding and repair contract

spend.rs:455 clones artifact.inspection into both actual and expected; the
post-sign mutation hook changes only expected. independently_verify at 578 does
verify decoded proof and signatures, but lines 638-703 obtain payment details
and counts from the inspection or constants. Two agreeing metadata copies can
therefore describe a payment the valid decoded transaction does not make. Existing
one-sided metadata mutations cannot falsify this defect.

The production repair, after accepted behavioral red, must satisfy these fixed
semantics together. They are design requirements, not production authorization:

- The confirmed prepared review is the expected payment. Obtain the actual
  receiver bytes, amount, memo and output inventory from authenticated recovery
  of the decoded Ironwood actions. Do not compare two supplied inspections or
  use clear PCZT output labels/values as the decoded result.
- Use the account-bound full viewing key: software already verifies its derived
  UFVK against the stored account UFVK. The synthetic external route must receive
  equivalent trusted viewing authority from the broker account, never a device
  response. Use pinned Orchard APIs for outgoing recovery and internal-scope
  incoming decryption. Missing authority or unrecoverable outputs fail closed.
- Bind decoded effects to the authoritative pre-sign PCZT as well: retain its
  shielded signature hash before signing, and compare the independently computed
  decoded hash. Preserve the accepted real-spend index/nullifier/randomized-key/
  positive-value and zero-padding binding; do not infer real spend count from
  the number of signatures, witnesses, or opaque decoded nullifiers. These
  retained facts are verified pre-sign context, not plaintext recovered from a
  shielded transaction. Signature/proof verification remains necessary.
- Account for both actions once by index. Require one exact intended positive
  payment and one positive internal change output owned by the same account.
  Derive these counts from recovery and the retained spend binding, not literals.
  Internal decryption must establish change ownership; an absent user_address
  label is insufficient. Fail closed on missing, extra or ambiguous outputs.
  Self-payment to an external-scope address must not be mistaken for change.
- With transparent, Sapling and Orchard bundles absent, derive the fee from the
  decoded Ironwood signed value balance. Use checked, nonnegative, bounded amount
  arithmetic. Compare the exact fee and bound, and check retained input value
  equals recovered payment plus internal change plus fee. Reject other pools,
  wrong version/branch, disabled required spend/output flags, or the wrong shape.
- Hash the recovered full memo using the existing trailing-zero-stripped PCZT
  convention (MemoPlaintext::from_memo), preserving no-memo encoding. Do not
  substitute a text decoder or hash an expected memo.
- Network, account/session, request ID and intent hash are broker bindings, not
  encoded payment fields. Compare them with the frozen public/prepared binding
  and account authority; retain existing capability, cancellation and expiry
  checks. Never claim to decode a network or request ID from transaction bytes.
- Require complete byte consumption when decoding. Derive txid from that decoded
  transaction. Populate actual effect fields only after all checks; no verified
  handle on error. Preserve established schema/state/signature/mismatch errors.
  Do not expose viewing keys, recovered notes/memos or raw bytes in logs/public
  results. Existing cleanup acceptance is not broadened by this repair.

The locally installed pinned primary sources were inspected: orchard 0.15.5
src/bundle.rs:663-745 provides per-action incoming decryption and outgoing
recovery; src/keys.rs:399-480 supplies scope/address checks and scoped keys;
pczt 0.9.3 src/orchard.rs:268-293 defines memo normalization. Existing store.rs
uses OvkPolicy::Sender. No dependency update or custom crypto is needed. The
remote docs lookup failed; the pinned installed source governs this API review.

## Exact source authority

Edit only wallet-broker/src/zec/spend/verification_context_tests.rs.
Starting identity: 528 lines, SHA-256
45f3bdf2f1d08924d53c298517c631427ab052c2d4955b31e98aa555386d380a.

Keep the existing tests and their assertions intact. Extend only the existing
SignedFixture/signed_fixture test helper to retain prepared inspection and the
synthetic account Orchard FVK/local-network context needed by the new test.
No change to fixture construction, signing, proving, serialization or prior
assertion semantics. All added authority stays private to this cfg(test) module.
Append one test named:
decoded_effects_reject_agreeing_metadata_that_disagrees_with_signed_bytes

spend.rs is frozen at 1046 lines, SHA-256
8b70ecf7dea541d951184d89b1ed2d917ce5c3c6dceabb73151840c79517328a.
No production function/signature, module registration, test_support.rs, existing
integration test, manifest/lock, UI, fixture, evidence, documentation, Git or other
file may change. The current function is already callable from this child test
module; no stub, missing-API test or test-only production bypass is needed.

## Required regression, one proof construction

1. Build exactly one signed_fixture in the new test, using the real existing
   compact fixture and pinned signer/prover/extractor. Keep serialized bytes
   unchanged for every case and decode completely for every production call.
   No global fixture cache or additional proof generation per matrix row.
2. Independently recover the decoded outputs using the synthetic sender FVK's
   scoped outgoing keys and decrypt the change using its internal incoming key.
   Use per-action APIs; account for each index once, including if multiple keys
   recover the same action. Establish exact external receiver bytes, amount
   100000000, stripped memo coffee and fee 10000 from the decoded data. Establish
   one distinct positive internal change output and two accounted-for actions.
   Compare with the fixture inspection only after computing this oracle. Do not
   use a production verifier's reported effects or success flags as the oracle.
3. Call the existing super::independently_verify with the original inspection
   passed as BOTH actual and expected, fee bound 12000 and no expected txid.
   Require success and assert its receiver, amount, fee, memo, counts and txid
   equal the independently recovered/decoded observations. This proves that
   later negative results cannot be explained by rejecting every transaction.
4. For each case below, clone the original inspection and alter that logical
   claim only. Pass the altered inspection as BOTH actual and expected to the
   same real production verifier over the same unchanged valid signed bytes.
   Confirm before each call that the changed field differs from the independent
   oracle/original and both metadata arguments agree. Do not call
   apply_expected_mutation or sign_mutate_and_verify_for_test.

   Cases (10 total):
   - receiver: different valid Orchard-only unified address and its corresponding
     receiver bytes, encoded for the same local network; assert it is neither
     recovered output's recipient (bounded search of synthetic diversifiers is
     acceptable, not an invalid string or invalid address encoding);
   - amount: 100000001;
   - memo: SHA-256 of a different literal text, independently computed;
   - fee: 10001, still below the unchanged bound 12000;
   - real Ironwood input count: 0;
   - Ironwood output count: 1;
   - spend pool: orchard;
   - has_transparent_bundle: true;
   - has_sapling_bundle: true;
   - has_orchard_output_bundle: true.

5. Collect all ten observed outcomes before the final assertion (no early
   unwrap_err/assert that hides the other cases). The final assertion must
   require INTENT_MISMATCH for every labeled case and display only case labels
   and stable codes if it fails. Avoid Debug bounds on VerifiedEffects and avoid
   dumping inspection/keys/bytes in panic output. The current source is expected
   to report success for these claims; a successful test is not expected here.

This is a real verifier-boundary regression for the metadata/bytes mismatch.
It is not yet an end-to-end native capability or broker-publication test. Do not
add unrelated test coverage, rebuild alternate valid transactions, mutate proof
bytes, change production, or repair the flaw during this source-only task.

## Execution plan, reserved for Hermes after source acceptance

Exact targeted red and later restored green command:

```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --lib zec::spend::verification_context_tests::decoded_effects_reject_agreeing_metadata_that_disagrees_with_signed_bytes -- --exact
```

Expected red: exit 101, one executed test fails at the final ten-case outcome
assertion after successful recovery and positive verification. Compile errors,
missing keys/recovery, fixture failures or broken crypto are not accepted red.
Future falsification: suppress the repaired decoded-effects comparison, preserve
crypto and the positive control, then require this same regression to fail;
restore exact source. The precise mutation will be frozen after source review.

After the production repair, the bounded broader green command is:

```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --lib
```

Affected software/synthetic integration and full security gates remain required
for eventual WAL-009 acceptance; their exact remaining commands will be frozen
with the production handoff. No old expensive suites or UI tests are run now.
There are no dependency/build/privilege changes in this test-source task.

## Baseline and stop

Reviewer compared all 26 rows of the frozen inventory in
[the closed native layout handoff](HERMES_BBD_WAL_009_NATIVE_LAYOUT_VALIDATION_01.md):
all match. New layout evidence is also frozen: 354 lines, SHA-256
ffcfc14837ff02aba33a5a24fa6a58d610055b18b4efd150787c6b8acba7dc43.
All implementation and ten actor evidence records remain uncommitted.

Grok verifies the target hash before editing, inspects only relevant source and
pinned local dependency APIs, writes only the authorized test file, then reports
its final full hash/line count and concise changes. No tests/compiler/formatter,
Cargo/version probe, Git, network, actor/subagent, evidence or integration.
If the fixed test cannot be expressed against these actual APIs, stop and report
the concrete blocker without widening scope. Do not add speculative work.

Reviewer publication scope: this handoff, docs/handoff/CURRENT_TASK.md and
tickets/BBD-WAL-009.md only. Launch Grok once, record its session/outer ID, and
collect only after done/explicit collection. Never poll the actor.
