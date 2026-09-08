# WAL-009 retained-spend repair 01

Actor: Grok Build, CLI grok-4.6, reasoning High.
Governance parent: the commit containing this handoff.
One combined source drop: tests first, then the two fixed production corrections.

Read AGENTS.md, TESTING.md, this handoff, only the first 55 lines of
docs/handoff/CURRENT_TASK.md, and
docs/testing/BBD-WAL-009-VALIDATION-ACCEPTANCE-AND-RETAINED-SPEND-REVIEW.md.
Inspect relevant functions in the authorized files and pinned APIs named below.
No historical handoff reload, repository-wide/dependency-tree scan, or other actor.

## Exact writable scope

| Path | Starting lines | Starting SHA-256 |
| --- | ---: | --- |
| wallet-broker/src/zec/spend.rs | 929 | bd51b3d0ee2f748fa31e483a2747c157ff20544aa62e3e00a7929c77060b7361 |
| wallet-broker/src/zec/test_support.rs | 4371 | fea8f65ed6637033506902688c8f547952cea848af51d05a49969cae81f48920 |
| wallet-broker/tests/zec_sign_verify.rs | 1115 | 80a3a342392f53553950fabae710f2e95082d357c281c6de23b54aedbc85eccd |
| wallet-broker/src/zec/spend/external_binding_tests.rs | new; must be absent | — |

Verify these baselines before editing. The existing context test module is frozen:
528 lines, SHA-256 45f3bdf2f1d08924d53c298517c631427ab052c2d4955b31e98aa555386d380a.
The accepted ShieldedVerificationAuth/Context and independently_verify bodies
remain unchanged. prepare.rs, store.rs, fixtures, native code, other tests,
dependencies/features, manifests, lockfiles, evidence, and governance are frozen.
Do not integrate any pending work.

## Tests first

The existing software and synthetic external success tests already failed in the
understood integration red (saved execution 77693). Preserve all their success,
actual pipeline-call, cryptographic, redaction, and no-publication/broadcast checks.
Before production edits:

- Strengthen the external success test to compare exported and contributed action
  indices with the actual index carried by the pinned SpendAuthSignature, requiring
  index < 2. Replace only the two invalid zero-index expectations. A small read-only
  ExternalContribution::signed_action_index_for_test accessor may expose that
  existing non-secret metadata from the private signed contribution.
- Add private tests in external_binding_tests.rs and its cfg(test) module declaration.
  The fixed private metadata-check interface is
  validate_external_binding(&TaggedContribution, &RetainedSpendBinding)
  -> Result<(), ZecError>. Test matched retained slots 0 AND 1 deterministically.
  For each slot, require SIGNATURE_INVALID for the other valid slot, out-of-range
  index 2, Orchard instead of Ironwood, a different randomized key, and an all-zero
  signature. Use explicit synthetic fixed arrays and pinned
  SpendAuthSignature::from_parts; never print raw signatures.
- Name these tests accepts_matching_retained_slot_zero_and_one and
  rejects_misbound_external_contribution. The positive cases prove metadata
  compatibility only; they do not assert cryptographic validity or mint any
  VerifiedEffects/capability. Existing end-to-end tests still execute the real
  pinned signer/prover/extractor/verifier. Do not stub those calls, use source-text
  assertions, loop until random fixtures choose a desired slot, or weaken tests.

This regression extension precedes production in the same task. The reviewer has
understood the existing red and fixed the semantics; no test execution is
authorized to Grok.

## Preserve the real spend through the pipeline

Introduce a PRIVATE RetainedSpendBinding with action_index: usize,
randomized_key: [u8; 32], nullifier: [u8; 32], and value_zat: u64.
No public constructor, serialization, Debug logging, mutable injection, or status
export. A private capture helper uses the original retained PCZT's unique unsigned
Ironwood action, requires a present positive value, and copies only those four
fields. Keep existing unsigned/prepared shape checks.

In authorize_software_for and authorize_external, capture that binding after
validate_prepared and BEFORE signing/applying contributions. Use its index for
signing and carry the immutable binding through finish_pipeline to inspect_final_pczt.
Do not infer the real spend from post-signature absence/presence or witness presence.

In inspect_final_pczt, retain all existing output, receiver, memo, fee/value-sum,
nonempty proof, and shape checks. Require exactly two actions and a valid captured
index. The indexed spend must have the captured randomized key, nullifier, and
positive value; the other spend must have zero value. Require both actual spend
signatures to be present. Witnesses on both actions are valid; they cannot count
as two real spends. Select ClearEffects.spend_auth_sig from the captured real
index, not the last encountered signature. Preserve existing error categories.

Do not erase a predicate to make the fixture pass, return copied success flags,
alter proofs/signatures, or weaken the actual decoded transaction verification.
Keep all fault injection points, counters, post-sign barriers, RAII lifetimes,
publication, and cleanup sequencing otherwise unchanged. Full independent
recovered-effect verification and actual-secret cleanup remain separate blockers.

## Carry the actual external action index

Add the actual action_index to SignerViewArtifact alongside its existing randomized
key, deriving both from the original retained PCZT before redaction. In
synthetic_sign_view_for, require that the unique unsigned redacted-view action
and its key still match those retained fields before signing it. Preserve the
actual signature's pinned pool/action-index tags.

authorize_external must derive expected identity from its OWN retained raw PCZT.
Remove its expected_pool/expected_index/expected_rk parameters and the corresponding
caller arguments, which currently include zero and contribution-derived data.
Use validate_external_binding to require typed Ironwood pool, exact retained index,
exact retained randomized key, and a nonzero signature. Then call the existing
pinned apply_orchard_spend_auth_signature; successful metadata checks alone never
permit finalization/publication. No unsafe casts, custom signature codec, or bypass.

Propagate real action indices through reviewed_view, SyntheticKeystoneV2 single/
batch contribution construction, and reviewed_signer_batch_for_test action metadata.
validate_external must require outer action-index/key fields to match the inner
signed contribution and preserve all route/count/intent/batch/replay/test-only
checks. Range checks cannot replace the authorizer's retained-PCZT equality.
Do not relabel a signature, infer its action index from its batch position, or
trust a caller-supplied expected index/key.

The existing apply_external_batch_and_verify remains closed/fail-closed; do not
implement batch success in this slice. Remove its incorrect batch-position-as-
action-index assumption only if needed for coherent metadata validation, using
inner signed metadata equality while retaining its existing unconditional rejection.
All malformed/replayed/misbound negative tests and account/capability checks remain.

## Read-only API anchors and later validation

Relevant local pinned definitions: pczt 0.9.3 roles/signer/mod.rs (SpendAuthSignature,
sign/apply APIs), orchard.rs getters/serialize_from; Orchard 0.15.5 pczt/prover.rs
(witness on every action), builder.rs (shuffled actions). The relevant application
anchors are prepare.rs::signer_view and store.rs's prepared PCZT inspection.
No dependency lookup beyond those named definitions.

After source review Hermes alone will receive the affected green commands:

```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --lib zec::spend::external_binding_tests
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_prepare --test zec_sign_verify
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --lib
```

Later falsifications will restore the old witness-based real-spend rejection and
force the external metadata check to slot zero. The software success regression
and slot-one test must respectively fail. Exact mutation hashes/order will be
frozen after source review. Existing signature-context green/falsifications are
accepted; the full library run rechecks that frozen context alongside new work.

Only directly related edits/formatting. No formatter, compiler, tests, Cargo/npm,
Git, evidence, integration, native launch, network, fixture execution, or subagent.
Stop on a concrete obstacle that requires broader scope; do not invent a new
architecture or repair unrelated failures. Report final four-path hashes/line
counts, test names, binding/index flow, and unchanged context-test identity.
Then stop. The reviewer collects once after done/explicit collection; no polling.
