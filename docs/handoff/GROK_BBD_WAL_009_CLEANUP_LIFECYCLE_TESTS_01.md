# WAL-009 real buffer cleanup lifecycle tests 01

Reviewer: Codex, XHigh. Source actor: Sr Dev — Grok Build, grok-4.6 High.
Baseline governance: ffb4e66e. Protected parent: the commit publishing this handoff.
Authorization: test source only in the five exact paths below. No execution or Git.

## Decision and retained acceptance

The owner selected XHigh for this lifecycle design. The six prior cleanup validation
outcomes are accepted and closed: four green runs, two intended falsification
failures, exact restoration and unchanged inventory. Retain those results and the
decoded-effects acceptance. No repeated expensive proof or pipeline baseline.

Replace the unsupported synthetic all-classes wipe assertion with actual owned-buffer
coverage. Existing exercise_* and panic_during_sign_for_test helpers do not enter the
real signing pipeline; they cannot establish operation-secret cleanup. Do not reuse
them as positive cleanup evidence or restore their fabricated counters.

This is coverage of the already-reviewed production correction and repair of an
invalid test oracle. No production behavior change is authorized. Initial new tests
are expected to pass; their red evidence will come from separately authorized
mechanism falsification, followed by exact restoration. This does not require a new
production edit merely to obtain a red result.

## Exactly five editable paths

1. wallet-broker/src/zec/test_support.rs: append only a blank line and
   #[cfg(test)] pub(crate) mod cleanup_lifecycle_tests; declaration, on conventional
   separate lines. Preserve its entire existing 4463-line byte prefix.
2. wallet-broker/src/zec/test_support/cleanup_lifecycle_tests.rs: new child module,
   exactly four #[test] functions plus bounded test helpers specified below.
3. wallet-broker/src/zec/spend.rs: append only a blank line and
   #[cfg(test)] mod cleanup_lifecycle_tests; declaration. Preserve its entire
   existing 1153-line byte prefix.
4. wallet-broker/src/zec/spend/cleanup_lifecycle_tests.rs: new child module,
   exactly two #[test] functions plus its metadata recorder/helpers.
5. wallet-broker/tests/zec_sign_verify.rs: replace only the old test
   every_sensitive_class_touched_is_positively_wiped_on_every_exit with the one
   replacement test specified below. Remove only its three now-unused exclusive
   helpers software_classes, assert_wiped, assert_every_touched_secret_wiped.
   Preserve imports, all other helpers and all other 14 tests byte-for-byte.

The old helper region, from fn software_classes through the blank line before the
first #[test], is 53 lines, SHA-256
17a0333f34930ccda9fb9f48d8255f65f35423464fc6db05a169976eaad9bc06.
The old test region, including its #[test] and trailing blank line before the next
#[test], is 67 lines, SHA-256
d6098b256b4cd68fab17a0d4b0b72b6308088c534d9c76f58cf4c52585ef7c17.
No other integration-test region may change. Do not ignore or weaken a remaining
test. Retain the accepted signer-failure regression unchanged.

No production function/body/type visibility/import, fault enum, callback, new
runtime hook, dependency, manifest/lock, fixture, UI, native surface, stored schema,
evidence or governance path may change. The only visibility addition is the cfg(test)
child module above so the spend tests can share its actual prepared-artifact helper.

## Test oracles and common helpers

Use actual SecretBytes/ObservedSecretBytes, AttemptOwner, ObservedSigningArtifact,
ExtractedTransaction and real prepare/sign adapter entry points. Do not reproduce
their Drop, classification or pipeline logic in test helpers. Do not construct
WipeEvent as an input, call observe/record_event/classify directly, set counters,
inspect production source text, manufacture verified effects, or use canaries as
operation secrets. Expected WipeEvent values used only as assertions are allowed.

A small observation assertion/snapshot helper must inspect all nine existing
TouchedSecretClass variants across all fourteen WipeExit variants. Require exact
touch/positive/failed counts, zero for every non-expected pair, and exact unclassified
count. Do not derive the expected class/exit/count list from actual observations.
Diagnostics contain only class, exit, counts or byte lengths; no secret/PCZT content.

Direct owner fixtures use nonzero, known byte patterns and check those contents are
nonzero while still owned, through a borrow returning a boolean. Do not read freed
memory, use unsafe pointers, disable panic hooks, leak/forget owners, clone a secret
for evidence, or claim ordinary upstream object drop zeroizes key/prover memory.

Every deliberate panic uses a fixed harmless payload and a reached marker set
immediately before panic. Assert both outside catch_unwind(AssertUnwindSafe(...)):
an earlier failed assertion or unrelated panic must not count as the intended unwind.
Keep owners inside the unwinding closure so their real destructors run.

## Four test_support child-module tests

Use these exact names under zec::test_support::cleanup_lifecycle_tests.

### attempt_defers_classification_until_owner_drop

Exercise two cases: selected Success and unfinished early Result::Err(INTERNAL).
Declare AttemptOwner before actual observed seed owners. The first owned fixture
has 17 nonzero bytes; verify it, drop it to queue a real event, then call finish
only for the Success case. At that point every published observation must still
be zero: selecting an outcome cannot publish/drain the queued event.

Create a second actual observed seed owner of 31 nonzero bytes after selection
(or the corresponding point in the unfinished case). Both use the real
zec-operation-seed label and attempt collector. Assert observations remain empty
while it is alive, then let it drop before the attempt through normal scope return.
For the unfinished case return the real ZecError::internal() value without calling
finish; assert that error result outside the scope.

After scope exit require exactly two Seed touches/two positive wipes/zero failures
under Success or default Error respectively, all other pairs zero, unclassified 0.
Take another read-only snapshot and require no further events. These two independent
nonzero fixture buffers test the real owner/collector boundary, not two seed copies
in the application pipeline.

### panic_overrides_selected_outcome

Inside the guarded panic closure, declare an attempt, create/check/drop a 17-byte
nonzero observed seed to queue an event, and select Success. Create/check a second
31-byte nonzero seed owner and keep it alive. Assert observations still empty, set
the reached marker and panic. Both that live owner and the attempt must unwind.
Outside catch require exactly two Seed touches/two positive wipes/zero failures
under PanicUnwind, every other pair (including Success and Error) zero,
unclassified 0. This must reject eager finalization and lost late-owner events.

### preconsume_errors_use_actual_exit

For each case below use a fresh actual fixture-backed software SignVerifyHarness:
open FrozenFixture at tests/fixtures/zec, bootstrap/scan/unlock, prepare the ordinary
existing local coffee intent, and confirm natively at 2026-08-30T12:00:30Z.
Use the same fixture account/request/hash/amount/fee/expiry constants as the frozen
integration tests. Do not copy the signing implementation into this module.

After confirmation, arrange only the named real state/clock condition through the
existing private parent fields/APIs, reset call counters and attach fresh observations.
Call sign_with_fault_for_test once, Software route, requested FaultPoint::Signer.
That injection must never trigger because the actual error occurs earlier.

| Condition | Setup after confirmation | Actual public code | Expected wipe exit |
| --- | --- | --- | --- |
| Cancelled request | account.prepare.cancel_request(request_id) | CANCELLED | Cancellation |
| Exact expiry | call with clock 2026-08-30T12:15:00Z | EXPIRED | Expiry |
| Missing prepared handle with seed retained | account.prepare.invalidate(HandleInvalidation::Expiry), then call at original NOW | LOCKED | Lock |
| Invalid timestamp | call with clock invalid-timestamp | SCHEMA | Error |

The missing-handle setup uses Expiry specifically because it removes prepared
artifacts while retaining the unlocked seed; it tests consume's actual LOCKED return,
not user-lock handling before seed_copy. No direct private mutex/state edits.

Before cleanup assertions require the exact error code, seed_accesses = 1 and
spend_authority_derivations = 1 (existing adapter counters), every actual spend
stage including authoritative_pczt_accesses/signer/prover/finalizer/extractor/
decoder/verifier = 0, post-sign status/clock reads = 0, verified/publication/broadcast
counts = 0 and account lock count = 0. No claim that a counter itself proves wiping.

Require exactly one Seed touch/positive wipe under the actual expected exit, no
failed wipe, every other class/exit pair zero, unclassified 0, before harness drop.
After dropping the harness the same snapshot must remain unchanged. No fixture
proof generation is needed.

### empty_and_unknown_owners_never_report_positive_known_wipes

Within one real attempt, create an empty observed SecretBytes owner labelled
zec-operation-seed and a nonzero observed owner labelled cleanup-lifecycle-unknown.
Verify the latter has nonzero bytes; drop both owners and select Success.
Observations stay empty until the attempt drops. Afterwards require exactly one
Seed/Success touch, zero positive wipes, one failed wipe, and unclassified = 1;
every other pair zero. Do not manually deliver an event or special-case a canary.

### Shared real fixture setup

Local helpers may prepare/confirm the ordinary fixture for the early-error test.
Expose only one pub(crate) helper, prepared_artifact(label), to the spend child
tests. It creates a fresh ordinary prepared/confirmed fixture using those helpers
and returns the actual PreparedSigningArtifact moved from existing
account.prepare.consume(&confirmation.review, NOW). No fabricated raw bytes,
metadata reconstruction, signing/proof run, raw copy, new public non-test API,
or production helper is needed. The real returned raw PCZT must be nonempty and
contain nonzero bytes; callers check that before creating the observed owner.

## Two spend child-module tests

Use these exact names under zec::spend::cleanup_lifecycle_tests. A small shared
WipeObserver records actual metadata emitted by the production owner, using
poison-tolerant synchronized storage. It never calls another observer or invents events.

### pczt_owner_wipes_on_error_and_unwind

For both normal error-return and guarded panic-unwind cases, obtain a fresh actual
PreparedSigningArtifact from the helper above, capture only its original raw byte
length, and assert raw is nonempty/nonzero through a borrowed boolean.
Move it into the real ObservedSigningArtifact::from_prepared with the recorder.
Require no event while owned. Exit by Result::Err(ZecError::internal()) or the
marked panic, so the actual Drop runs. Assert the intended error/panic first.
Require exactly one recorded event: label zec-authoritative-pczt, the original
positive length, all_zero = true. No second Drop/event or manual wipe invocation.

### extracted_owner_wipes_on_error_and_unwind

For both normal error-return and guarded panic-unwind cases, instantiate the real
ExtractedTransaction::new with the recorder and use std::io::Write::write_all to
write 64 known nonzero bytes. Check the real as_slice has length 64 and all bytes
are that nonzero pattern, returning only a boolean. Require no event while owned.
Exit by the intended error or marked panic. Require exactly one recorded event:
zec-extracted-transaction, length 64, all_zero = true.

This is the real serializer buffer's live allocation and Drop, not a dummy buffer
reported as pipeline work. It makes no new claim about spare capacity, prior
reallocations, parsed transaction objects or third-party proof/key objects.

## Replacement integration test

Name: cleanup_lifecycle_wrong_seed_wipes_owned_buffers_before_pczt_access.

Use existing software_harness/prepared helpers, native confirmation at NOW, reset
call counters and attach fresh observations. Call the existing
sign_with_prerequisite_for_test with WrongSeed once. That helper replaces the
fixture seed with actual nonzero bytes and enters the real authorization path.
Require LOCKED, seed_accesses = 1, spend_authority_derivations = 1, and all actual
spend stages including authoritative_pczt_accesses and signer_calls = 0.
Require no verified result/publication/broadcast and released account lock.

Then require exactly one Seed/Lock touch and positive wipe, and exactly one
AuthoritativePczt/Lock touch and positive wipe, with zero failed wipes,
all other class/exit pairs zero and unclassified 0. The raw PCZT is already owned
by ObservedSigningArtifact even though the binding rejection precedes counted PCZT
access. This deliberately prevents treating zero stage counters as proof that no
buffer was owned. Assert before harness teardown; after drop, the snapshot is
unchanged. No synthetic lifecycle helper or canary installation.

A new bounded assertion/snapshot helper may be placed in the authorized replacement
region; do not alter the frozen helpers or other 14 tests. Keep total integration
test count at 15.

## Later execution and falsification contract

No command in this section is authorized for Grok to run. After source review,
Hermes will receive exact temporary mutations, fingerprints, restoration and
evidence instructions for one grouped run.

Initial and restored library green:
```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --lib cleanup_lifecycle_tests
```
Expected: six passed, seven filtered, exit 0. Total library tests become 13.

Replacement integration green:
```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_sign_verify cleanup_lifecycle_wrong_seed_wipes_owned_buffers_before_pczt_access -- --exact
```
Expected: one passed, 14 filtered, exit 0.

Red commands use the same library command prefix but replace cleanup_lifecycle_tests
with the full test name below and append -- --exact. Each must fail one test with
12 filtered and exit 101, for the named invariant, not compile/setup failure:

| Temporary mechanism fault, one at a time | Exact library test selector | Required failure |
| --- | --- | --- |
| Eager classify from AttemptOwner::finish | zec::test_support::cleanup_lifecycle_tests::attempt_defers_classification_until_owner_drop | queued real event becomes visible before attempt Drop |
| Ignore panic override in AttemptOwner::Drop | zec::test_support::cleanup_lifecycle_tests::panic_overrides_selected_outcome | marked real unwind produces the wrong outcome |
| Omit actual-error selection on consume failure | zec::test_support::cleanup_lifecycle_tests::preconsume_errors_use_actual_exit | actual early error has the wrong observation exit, stage/error guards still pass |
| Skip SecretBytes::wipe_with zeroization before observer | zec::test_support::cleanup_lifecycle_tests::attempt_defers_classification_until_owner_drop | nonzero fixture produces failed rather than positive wipe |
| Suppress ObservedSigningArtifact::Drop wipe notification | zec::spend::cleanup_lifecycle_tests::pczt_owner_wipes_on_error_and_unwind | expected actual-owner event is missing |
| Skip ExtractedTransaction::Drop zeroization before observer | zec::spend::cleanup_lifecycle_tests::extracted_owner_wipes_on_error_and_unwind | live nonzero bytes are not all zero at observation |

Restore each exact source before the next fault; finish with the six-test library
green and the replacement integration green. Do not rerun prior proof-heavy library
or full integration targets, already accepted happy paths or the old invalid test.
No new dependency/build-input/renderer/security boundary change occurs here; final
security scans remain a separate outstanding gate, not a new scan round for tests.

## Actor procedure and stop

Read AGENTS.md, TESTING.md, active CURRENT_TASK.md/ticket prefixes and this handoff.
Verify all 34 inventory rows below once using an inline read-only script parsed
from the table, and verify the two new paths are absent. Stop on mismatch.
Read only relevant local owner/adapter/fixture APIs. Do not reload historical
handoffs, search dependency sources, or reconstruct financial/cryptographic fixtures.

Author only the bounded tests/declarations above. Verify both original parent
prefixes unchanged, integration text outside its two allowed regions unchanged,
all other 31 inventory rows unchanged, and test counts exactly six new library
tests plus one replacement integration test. Report all five changed path hashes/
newline counts, original-prefix checks, unchanged-region proof and limitations.
No formatter, compiler, Cargo/test/no-run probe, Git, evidence/document change,
network, actor or subagent. Stop after source delivery.

Implementation and fifteen actor evidence records remain uncommitted. This stage
covers owning-buffer destruction/outcome bookkeeping and the named real early
operation paths. It does not establish a full proof-to-publication panic integration,
third-party typed-secret erasure or native OS/owning-thread/capability integration.
Those and final security remain open; network/broadcast/mainnet/hardware/Monero/
Electron send remain parked. Do not claim a usable send flow or full wallet acceptance.

Reviewer publication scope: this handoff, docs/handoff/CURRENT_TASK.md and
tickets/BBD-WAL-009.md only. Keep reviewer XHigh for test-source review and the
falsification contract. Grok runs High. Launch once, collect after owner done,
never poll an actor.

## Frozen inventory

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| wallet-broker/src/zec/spend.rs | 1153 | b4e9b87c743394e638011076cab673945328b0fbd23b36de53a42db3c93028fe |
| wallet-broker/src/zec/test_support.rs | 4463 | 11ef7579a342ef3feee455418fa2522a98ee5616ea719aea91e6396927ee5cdb |
| wallet-broker/src/zec/spend/verification_context_tests.rs | 768 | 36599689a0dd3a45c9ef93dc8ef59947bdf10a81e8efd6a8ea7cfa8b747df110 |
| wallet-broker/src/native_ui.rs | 321 | d132a164a6413165291abd0582abf3eca4cacd3f33ce6eb4ac2b467e8774d960 |
| wallet-broker/src/native_ui/zec_review_tests.rs | 233 | 2eec3b60eb1b9fbd32235832a867d1189928126ca79373fa9f4fa7fdf2ee87bf |
| wallet-broker/src/native.rs | 489 | 992138f18cbb0b969296109de3186aab923500c41ed2c0d41cf9c0696d3c3acc |
| wallet-broker/Cargo.toml | 122 | 73e5e585eb2fd1ca867d66962460cfa010443886a4b59d8a33556ef2a4ff3503 |
| wallet-broker/Cargo.lock | 5395 | b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71 |
| package.json | 42 | 84b30b6860441a100588b5bdb92b37ddc45fb7610d48ee6f0e51c30ea0717780 |
| package-lock.json | 396 | 5e1122f32b0db42eb4386d4d0f0c47a4160d23cb4ae790bbb3600b5d3a5bddfc |
| wallet-broker/src/zec.rs | 274 | 045cdc51f26ac8b9b1283cee5f995d60ceda38a25577aa7a937f5d07f177b90b |
| wallet-broker/src/zec/prepare.rs | 1238 | 44c0783fa3d75867599092d25912032b665838ed7a73c81a16b0eec2b26ffb07 |
| wallet-broker/src/zec/store.rs | 2872 | 531d0a6171ecd9b5012602ccb870f16eab96888c809b3ffdf4e6303576f17c90 |
| docs/testing/BBD-WAL-009-LOCK-SYNC-01.md | 120 | ab25ffd2ac6ec7c26f7c21e42978697ec10da7ff395a9569afd9e78d1cd27ed2 |
| docs/testing/BBD-WAL-009-NATIVE-REVIEW-EXPECTED-RED-01.md | 199 | 42825e83ba6e98471c18b0c63d246e9a8790c25591e395cb6a0934c8d7bee6aa |
| docs/testing/BBD-WAL-009-NATIVE-REVIEW-PREREQUISITE-02.md | 181 | 33d488dfcc88eb684d316bcffc547c051be412984d5633c54da76c1cef19dbe2 |
| docs/testing/BBD-WAL-009-SIGNATURE-CONTEXT-EXPECTED-RED-01.md | 216 | 79b6e97f5a6a4ca4903d9db7b40233ecf0f2ca931116e0d232853f1b842d1ff2 |
| wallet-broker/tests/zec_sign_verify.rs | 1215 | c95fa9ae836ca7151f35ef92e664dbc569f24b54b1ff4fb8ad78c1fce997a787 |
| wallet-broker/src/zec/spend/external_binding_tests.rs | 114 | bb2d6873dd7262ad6e30f117f1947b7d4cf7685ab04e32a5b700d7cb5b112b6d |
| docs/testing/BBD-WAL-009-SIGNATURE-CONTEXT-VALIDATION-01.md | 677 | 6a23bcb2943df54953d1acddf85a2ec9cfc41b92e84c12856a48b7d07be53c58 |
| docs/testing/BBD-WAL-009-RETAINED-SPEND-VALIDATION-01.md | 358 | d51cc09e82f60a02a1493338d0af249dd25f046e2b5ca75e46a0fe4019269ed1 |
| docs/testing/BBD-WAL-009-RETAINED-SPEND-VALIDATION-RESUME-01.md | 560 | 62bb1ffd672c460f6fac6439abbc8d555683342e5410ad7098d2339bf509e5d0 |
| wallet-broker/tests/native_surface.rs | 664 | 349f3a019a0e7a5c37aaae727192b0ddcecb02192a5fae6df9291eaf1357276d |
| wallet-broker/src/native_ui/zec_native_app_tests.rs | 376 | 485ce9691f873af9304e1db36de74d49627279abf9c550ccce4c74bada22a5a4 |
| docs/testing/BBD-WAL-009-NATIVE-REVIEW-VALIDATION-01.md | 503 | 3a734781d0ad2c31989318373272f13ba89654ded2ab3a6b90f83ea322821de1 |
| docs/testing/BBD-WAL-009-NATIVE-APP-EXPECTED-RED-01.md | 184 | e749d709d8d2bd8f5e72a8509c156a2053dbed5983849964348eefbc3429733b |
| wallet-broker/src/zec/spend/effects.rs | 379 | cfa6a86f35d6772053025ca1c9bbb3154d604f79c90d031af261787938576035 |
| docs/testing/BBD-WAL-009-NATIVE-LAYOUT-VALIDATION-01.md | 354 | ffcfc14837ff02aba33a5a24fa6a58d610055b18b4efd150787c6b8acba7dc43 |
| docs/testing/BBD-WAL-009-DECODED-EFFECTS-EXPECTED-RED-01.md | 196 | a7def01abd54eae10e663e341ce8b253992d883863a1410c93278fa698d78431 |
| docs/testing/BBD-WAL-009-DECODED-EFFECTS-VALIDATION-01.md | 165 | fe82cce82ebba2595747a411a5ffd2258d86abed36dc9b3de60517193d4babad |
| docs/testing/BBD-WAL-009-DECODED-EFFECTS-VALIDATION-RESUME-01.md | 345 | 02d078de8a0269ba2dc41664450965a3eec1036ccb5343db6bb8f90518a64f5a |
| docs/testing/BBD-WAL-009-CLEANUP-OBSERVATIONS-EXPECTED-RED-01.md | 177 | 996f26aeea6b1cb27799200b24258e0e41defaa05505a0ee2b704c60471c2e9e |
| wallet-broker/src/vault.rs | 794 | f23247b0e46c68dfc125804fa981d16f7022464e1e4d94a8c1e62771a045be49 |
| docs/testing/BBD-WAL-009-CLEANUP-OBSERVATIONS-VALIDATION-01.md | 564 | 5bfed1076b4668f2df03cc2766983989323a8c0332ada7b4f43db34f2c8868a1 |
