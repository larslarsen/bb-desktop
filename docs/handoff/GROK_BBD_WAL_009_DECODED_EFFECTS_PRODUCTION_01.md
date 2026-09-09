# WAL-009 decoded effects: accepted red and production repair 01

Reviewer: Codex, XHigh. Actor: Sr Dev — Grok Build, grok-4.6 High.
Baseline governance: 5405a5801f17f99a6901ec5147082df8c64097bd.
Authorization: bounded production source and mechanical test-call adaptation.
No execution, evidence, integration, Git, or other actor authorization.

## Accepted behavioral red

Hermes outer 81835 collected once after owner done, exit 0. Actual Hermes session:
20260908_182241_3f2ee5, provider nous, model poolside/laguna-s-2.1:free.
Version output 78705: v0.18.2 (2026.7.7.2), upstream b1f003e1, local 10b6d1a9.
Authorization 123f26c2; launch checkpoint 5405a580.

Saved command 78730 exactly matches the handoff. Launch 78731 created
proc_bd6c8d31578b. Completion 78749 exited 101. Full process log 78751 retains
all 41 lines. Compilation succeeded in 6.26 seconds; the one regression failed
at verification_context_tests.rs:756:5 in 453.79 seconds, with 6 filtered.
All ten labels reported ok: receiver, amount, memo, fee, real input count,
output count, spend pool, and the three forbidden-pool presence flags.

ACCEPT: the real fixture, independent output recovery and positive verifier
control succeeded before all ten false metadata claims were wrongly accepted.
This is the intended behavioral red, not a compile or crypto failure. No rerun.
The other 27 existing identities remain correct, including the 757-line test.
New pending evidence: BBD-WAL-009-DECODED-EFFECTS-EXPECTED-RED-01.md, 196 lines,
SHA-256 a7def01abd54eae10e663e341ce8b253992d883863a1410c93278fa698d78431.

Execution/report exceptions are recorded here without a correction-only task:
Hermes initially measured the handoff file against the layout evidence identity
(78712), then had a missing-os import error (78722), then measured the correct
row (78726). Its claim of one uninterrupted 27-row preflight is false. Repeated
read-only Git/discovery and an unnecessary ps query occurred; no source or Git
mutation or second test invocation occurred. Postflight 78752 correctly measured
all 27. The evidence misidentifies the process ID as the Hermes runtime session,
uses the launch checkpoint as source acceptance, and miscounts untracked files.
Its raw-output block changes three warning-indentation lines from saved 78751;
that block is not byte-verbatim, but all test outcomes match. It leaves local
absolute paths for normalization during later integration. The decoder/recovery
oracle is independent; the positive production call is a control, not the oracle.

## Fixed implementation contract

Implement the architecture from the
[test-source handoff](GROK_BBD_WAL_009_DECODED_EFFECTS_TESTS_01.md), with the concrete
boundaries below. This closes the decoded-effects defect as one repair. Do not
expand into native UI, durable states, network, hardware transport or cleanup
redesign. Existing accepted crypto/context and retained-spend behavior stay intact.

### Trusted context before signing

Introduce a private, operation-local verification authority carrying the account
Network and Orchard FVK, retained real-spend binding, pre-sign shielded signature
hash, and resolved Ironwood anchor. It must not hold a second copy of the expected
receiver, amount, memo, fee or inspection as the source of actual effects.

Software: obtain the FVK only after the existing derived UFVK/stored UFVK equality
check. Keep the existing authority denial and call-count/error boundaries.
Synthetic external route: pass stored_ufvk and typed Network from the already
selected broker account in execute_external. Decode its UFVK under that network,
require an Orchard component, and supply that viewing authority to the common
pipeline. Do not accept a viewing key from a signer/device payload, derive a
fixture key in production, or add an alternative signing route. Malformed or
missing account viewing authority fails closed with LOCKED; upstream error text
must not escape.

Retain the current unsigned real-spend selection and positive/zero value checks.
Before signing/applying a contribution, use the pinned typed PCZT Verifier view
to validate the retained values against their cryptographic fields: both actions'
verify_cv_net and output verify_note_commitment; the real spend's
verify_nullifier(Some(account_fvk)) and verify_rk(Some(account_fvk)). Use the
library's cross-address restriction verification. Parsing a Verifier view alone
does not perform these semantic checks. Missing/inconsistent retained fields
return INTENT_MISMATCH; do not synthesize absent fields or weaken the accepted
padding/index checks. This validation is part of establishing trusted context.

Capture the pre-sign hash from Signer::shielded_sighash BEFORE adding the real
signature/contribution. Capture the resolved anchor through the typed parsed
bundle, not an optional compact wire field: compact PCZT may omit that field.
After extraction compare decoded hash and anchor with this retained context.
Retain the index/nullifier/randomized-key binding against the decoded action.
The v6 bundle commitment alone must not be treated as a retained-anchor check.

### Real decoded effects

Keep ShieldedVerificationContext and its accepted proof, spend-authorization and
binding-signature checks. Decode the extracted buffer once with a Cursor and
require every byte consumed; trailing/truncated bytes are STATE_CORRUPT. The
existing live extracted-buffer guard and counters/fault hooks stay intact.

The verifier takes the trusted authority in addition to the prepared/expected
binding inputs. Remove the misleading actual inspection variable. A prepared
inspection may remain an immutable broker binding, with a separate expected copy
for existing test mutations; neither supplies actual payment effects. Check the
consumed artifact's public network/request/intent/receiver/amount/fee/pool/version
bindings against its inspection and account context. Preserve existing exact
review/capability and before/after-sign cancellation/expiry barriers.

Require V6, NU6.3, only the Ironwood bundle, exactly two actions, and required
spend/output flags. Derive forbidden-pool presence from decoded bundle accessors
and compare all declared pool/count flags with reality. Obtain fee from the
signed Ironwood value balance, reject negative/out-of-range values, and compare
the exact fee plus fee bound. Use checked bounded Zcash amount arithmetic.

For each decoded action, attempt outgoing recovery with scoped sender keys and
internal incoming decryption using pinned Orchard per-action APIs. Deduplicate
by action index; if multiple recovery paths succeed they must describe the same
note/address/value/memo. Require all actions accounted for. For this accepted
one-payment/one-change shape, require exactly one distinct positive external
payment and exactly one positive output decryptable with the same account's
internal incoming key. A payment to the account's external-scope address remains
a payment. Reject ambiguous two-internal-output shapes; do not infer change from
user_address or merely from being an output that is not the payment.

Compare recovered payment receiver bytes, its Orchard-only unified address under
the trusted network, amount and memo hash with the confirmed expectation. Apply
the existing MemoPlaintext trailing-zero-stripping convention to recovered full
memo bytes, preserving no-memo encoding; a borrowed slice of a zeroizing memo
buffer may implement the same convention without a new plaintext allocation.
Check retained input value = recovered payment + change + decoded fee using
checked arithmetic. Derive real-spend count from the validated retained binding
and positive/zero values; opaque on-chain nullifiers do not reveal note values.
Derive output counts from the successful recovery/classification, not constants.

Network/account/session/request/intent remain broker bindings, not purported
on-chain plaintext. Fill VerifiedEffects payment/fee/pool/version/txid fields
from decoded/recovered values only after all checks. Txid must come from the
independently decoded transaction. Keep existing result shape and no-broadcast
behavior. Existing metadata mutation hooks must still exercise the verifier and
preserve established stable errors: SCHEMA for invalid expected schema,
NETWORK_DISABLED for mainnet, STATE_CORRUPT for bad encoded state,
SIGNATURE_INVALID for invalid crypto, INTENT_MISMATCH for valid-crypto effect or
retained-binding mismatches. Do not bypass crypto to make the regression pass.

Do not add Debug/serialization/public exposure of keys, notes or memo buffers.
Keep viewing authority and recovered plaintext scoped to this operation, use
existing zeroizing byte wrappers for new owned secret buffers, and do not invent
wipe observations for unrelated canaries. Full actual-secret cleanup remains
unaccepted and is not claimed by this repair.

## Exact write paths

1. wallet-broker/src/zec/spend.rs — common pipeline/context plumbing, complete
   decoding and verifier repair; private module declaration if needed.
   Baseline: 1046 lines,
   8b70ecf7dea541d951184d89b1ed2d917ce5c3c6dceabb73151840c79517328a.
2. wallet-broker/src/zec/spend/effects.rs — optional new private helper module
   for the authority/recovery/comparison code above; currently absent. No public
   API or new operation. Prefer this file if it keeps spend.rs reviewable.
3. wallet-broker/src/zec/test_support.rs — only execute_external's retrieval and
   forwarding of its selected account's stored UFVK and Network to the changed
   authorize_external call. No other harness/oracle/counter/cleanup change.
   Baseline: 4369 lines,
   c34aa4dc95011c7585b44553e6e1680dd3c918ea78ffa98e07d2e271f7991159.
4. wallet-broker/src/zec/spend/verification_context_tests.rs — mechanical fixture/
   call adaptation for the new authority parameter only. Baseline: 757 lines,
   791e8cbb77d556ab05e35946e23cddbc3b5a27659c3ce7b5e2464a665210fda4.

For the test adapter retain the same single signed fixture, independent output
oracle, positive control, all ten mutations and final error assertion. Capture
its authority before signing from the same real PCZT, synthetic account FVK and
already independently captured PCZT sighash. A production retained-context helper
may be used for fixture plumbing, but the independent recovery oracle must stay
independent. Never put the original payment inspection in the new authority so
that a negative case passes solely by comparing against another metadata copy.
Both prepared/expected metadata inputs must continue to agree in every negative
case, as in the accepted red. No changed assertion, outcome, fixture amount,
ignored test, production test bypass or extra proof construction is authorized.
Existing unrelated tests remain byte-identical apart from shared helper plumbing.

All other source/tests, prepare/store/native/UI, manifest/lock, fixture, existing
evidence and governance are frozen. Use only pinned installed dependency APIs;
no dependency/feature changes or custom cryptography. The reviewed APIs include
orchard 0.15.5 bundle.rs, keys.rs and pczt/verify.rs and pczt 0.9.3 Verifier views.
Stop with a concrete blocker if this contract cannot be expressed, without
widening paths or inventing a weaker boundary.

## Later execution contract and stop

Grok runs no test, compiler, formatter, Cargo/version probe, Git, network, product,
actor or subagent. Verify allowed starting identities, edit the bounded source,
then report exact changed paths/full hashes/line counts and concise semantics.
No implementation evidence or integration. Stop for XHigh source review.

After source acceptance, Hermes will run the exact regression green:

```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --lib zec::spend::verification_context_tests::decoded_effects_reject_agreeing_metadata_that_disagrees_with_signed_bytes -- --exact
```

Then an exact, separately frozen temporary suppression of recovered receiver
comparison must make that regression fail while its positive control and crypto
remain valid; restore complete source. The bounded broader green is:

```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --lib
```

The next Hermes handoff will freeze the affected software/synthetic integration
checks and remaining validation using reviewed source. They remain required for
acceptance, but are not execution authority now. No native UI repeats or report-
only correction cycles. Full security/cleanup/native acceptance remains open.

Implementation and eleven actor evidence records remain uncommitted. Reviewer
publication scope: this handoff, CURRENT_TASK.md, tickets/BBD-WAL-009.md only.
Keep reviewer XHigh. Launch Grok once, record the session/outer ID, collect after
owner done/explicit collection, and never poll the actor.
