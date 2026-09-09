# WAL-009 actual buffer cleanup and truthful observations production 01

Reviewer: Codex, XHigh. Actor: Sr Dev — Grok Build, grok-4.6 High, source only.
Baseline governance: 8cfd27c7. Protected parent: the commit publishing this handoff.
Authorization: three bounded production/adapter paths; all tests remain frozen.

## Accepted red and scope decision

The [accepted expected red](HERMES_BBD_WAL_009_CLEANUP_OBSERVATIONS_EXPECTED_RED_01.md)
ran the real signer, stopped before proof/extraction, and failed at the final
assertion on four fabricated cleanup entries. Saved completion 79200: exit 101,
one failed, 14 filtered, 1.87s. Full log 79202: 46 lines. All earlier guards passed.
No repeated baseline, new source-test round or report-only correction is needed.

Repair the actual owned-byte observation path and remove fabricated operational
accounting globally in SignVerifyHarness. This is not full key/proof-workspace
erasure acceptance. Ordinary destruction of upstream key/PCZT/prover objects is
not a positive byte wipe. Do not invent a replacement drop counter unsupported by
an actual owning wrapper, unsafe overwrite, dependency patch or process boundary.
The remaining typed-object lifetime coverage stays explicitly open.

## Exactly three editable paths

- wallet-broker/src/vault.rs: two crate-private ownership/access helpers only.
- wallet-broker/src/zec/spend.rs: actual raw-PCZT owner, removal of copied-seed
  pseudo-authority, and tracking of the fault branch actually taken.
- wallet-broker/src/zec/test_support.rs: actual operation-seed owner, per-attempt
  event collection/outcome classification, and removal of fabricated accounting.

No other file, new module, test/helper fixture, dependency, manifest/lock, UI,
capability semantics, effect comparison, stored schema or public result may change.
Source identities are in the 33-row table below. Preserve all 1215 integration-test
lines and all seven library tests exactly. No formatting of unrelated code.

## Ownership contract

### 1. Move the actual seed into the existing observed owner

In vault.rs add only crate-private helpers with these semantics:
- SecretBytes::into_observed(self, label, Box<dyn WipeObserver>) -> ObservedSecretBytes
  moves the existing SecretBytes into the existing wrapper without exposing or
  copying its bytes or constructing another allocation as evidence.
- ObservedSecretBytes::as_secret(&self) -> &SecretBytes borrows that exact owned
  value for existing authorization APIs. It must not return an owned copy.

Preserve existing WipeObserver/WipeEvent shapes and all existing vault encryption,
wipe_with, zeroization and Drop behavior. No public API or trait change is needed.

In execute_pipeline, wrap the actual successful account.prepare.seed_copy() result
immediately, before subsequent fallible steps, with label zec-operation-seed and
an observer collecting into this attempt. Pass as_secret() to authorize_software.
Explicitly drop this owner after authorization returns and before publication or
error return. RAII must retain cleanup on earlier returns and panic unwind as well.
There must be no second seed copy made for observation. In spend.rs remove the
entire derived_authority seed clone and its zec-unified-spending-authority wipes;
the actual UnifiedSpendingKey/authorizing key retains its existing lexical lifetime.
Do not report that ordinary key drop as a measured wipe.

### 2. Own the consumed raw PCZT through every exit

Keep PreparedSigningArtifact and the public crate-private authorize_software /
authorize_external entry signatures unchanged. Move the incoming artifact's actual
raw SecretBytes into a private RAII owner at entry, before parsing/validation.
The owner also retains the public/inspection values needed by the unchanged checks
and a borrowed WipeObserver. Its Drop calls raw.wipe_with with label
zec-authoritative-pczt on the actual owned bytes, on normal/error/unwind paths.

A private observed-artifact struct owning raw/public/inspection and the borrowed
observer is the intended implementation. Inner software and finish_pipeline helpers
may borrow that owner instead of taking the plain artifact by value. Borrow the
observer field separately for ExtractedTransaction; the public/inspection fields
remain available for the existing comparisons. Do not clone raw, reserialize it
for evidence, expose it, or move it out of the guard. No self-referential or unsafe
lifetime technique is needed. Apply the same owner to external authorization.

Keep ExtractedTransaction's existing live-buffer zeroization, tail wiping and
pre-success drop unchanged. Its zec-extracted-transaction event must reach the same
attempt collector under its own class, not AuthoritativePczt. This covers the live
owned buffers only; it makes no new claim about earlier Vec allocations or upstream
parsed objects. All signing/proving/finalization/decode/verification semantics,
error precedence, counters and existing injected-fault timing remain unchanged.

### 3. Collect real events; classify once using the actual outcome

Replace SignVerifyWipeObserver's caller-chosen class and hard-coded Success with
an attempt-owned collection of actual WipeEvent metadata. A small shared collector
can implement WipeObserver and be cloned into the observed seed's boxed observer;
it contains no seed, PCZT, receiver, memo, key, pointer or transaction bytes.
Its lifetime owner is declared before secret owners, so those owners drop first.

Map only actual emitted owner labels:

| Actual owner label | Operational observation class |
| --- | --- |
| zec-operation-seed | Seed |
| zec-authoritative-pczt | AuthoritativePczt |
| zec-extracted-transaction | ExtractedTransaction |

For a recognized event, record a touch and classify the measured length/all_zero:
positive length and all_zero is a positive wipe; an empty/failed observation is
not a successful wipe. Do not fabricate lengths or derive touch events from stage
counts. Unknown labels cannot be reclassified or silently counted as a successful
known class; retain a bounded unclassified-event count in test observations if any
occur. Canary labels must never enter this operational collector.

Flush each actual event exactly once under the operation's final outcome. Default
to Error for an unfinished attempt, and PanicUnwind if its owner drops while the
thread is panicking. Carry the attempt owner in PipelineOutcome after pipeline
success; only mark Success after the existing publication rechecks and fallible
work succeed. Post-sign cancellation/expiry/lock must classify the collected events
under that actual outcome. Preserve release of account/capability ownership and
all existing post-sign status/clock reads; no extra proof/sign call or publication.
Ordinary earlier errors use their actual code, not an anticipated requested fault.

Add one private Option<PipelineFault> field to PipelineCalls, initially None. Set
it only inside each existing fault branch immediately before its injected return.
This records which injection really triggered; do not infer it from an enabled
fault option or incremented stage counter. Use that value to select SignerError /
ProverError / FinalizerError / ExtractorError / VerifierError / CleanupError after
a matching failure. Earlier errors must retain their ordinary outcome. This field
is private bookkeeping, not a public status field or new control over the pipeline.

Use existing poison-tolerant mutex helpers for the collector. Do not add global
logs, repeated writes, an actor, a worker or callbacks that format secret data.
Both software and synthetic external paths must use this mechanism.

### 4. Remove all fake operational producers

Delete the unrelated vec![0x5a; 32] allocation and its zec-sign-verify-secret wipe.
Remove the inference that every software/hardware class was touched on success,
failure, panic or lifecycle-helper calls. Do not fix only the SignerError branch
or filter out the four classes named by the regression. Actual owners are the
only producers of operational wipe observations.

Existing installed canary owners may still be destroyed and their canary-only
counters retained for representation checks. Rename/separate the old blanket
observe/wipe helpers to make this distinction clear. Their caller-specified class
lists and exercise_* methods must not create operational touch/wipe/failure events.
Preserve account invalidation, lock release and other behavior in those helpers.
Do not manufacture key/capability/prover/view/contribution events to retain a
historical all-classes-green claim. No actual owner means no such observation.

## Tests and acceptance limits

The new signer-failure regression stays byte-identical and must turn green with
real Seed wipe evidence and zero events for unreached classes. Both existing
successful pipeline tests must retain their original cryptographic behavior.
The vault's existing zeroization semantics are reused, not reimplemented.

The historical every_sensitive_class_touched_is_positively_wiped_on_every_exit
test calls synthetic exercise_* helpers; those helpers never ran the pipeline and
its all-classes positive result was never accepted. Removing fabricated events
exposes that test's invalid assumption. Preserve it unchanged in this source drop:
no deletion, ignore, weakened assertion or dummy event to force a green result.
It must receive a separate real-ownership test contract before broader cleanup
acceptance. This known unsupported test is not a reason to rerun the expensive
full target or to preserve falsified evidence. No full-suite success is claimed.

After source review Hermes will run the exact focused green, bounded falsification,
restored green, and the two affected software/synthetic pipeline tests. No test
execution is authorized now. The focused command is:

```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_sign_verify signer_failure_does_not_report_cleanup_for_unreached_secret_classes -- --exact
```

Expected green: exit 0, one passed, 14 filtered. The planned falsification adds one
spurious ProofWorkspace event after genuine seed accounting; the final forbidden
assertion must fail while positive guards still pass. A second fast falsification
may suppress the real observed-seed Drop notification and require the positive
Seed guard to fail, without using the fixture's already-zero seed as erasure proof.
Reviewer will prescribe exact mutations, hashes and full restoration against the
accepted source. The prior 1.87s expected red is retained and must not be rerun.

Full third-party key/proof-object lifetime observation, actual-memory limitations,
native OS/owning-thread/capability integration and final security gates stay open.
The accepted decoded-effects repair is retained; do not reopen its matrix or the
old expensive library proof checks without an observed reason.

## Actor procedure and stop

Read AGENTS.md, TESTING.md, active CURRENT_TASK.md, ticket active prefix, this
handoff and only the named source/test sections needed for this repair. All needed
ownership/observer APIs are local. No dependency archaeology or historical reload.
Verify all 33 rows before editing with one inline read-only inventory script parsed
from this table. No helper file or manually reconstructed list. Stop on mismatch.
Edit only the three paths above; measure their final byte hashes/newline counts
and verify the other 30 rows unchanged. Report the actual owner/drop order, event
label/outcome route, removal of fake producers, changed paths/identities and limits.
No formatter, compiler, test/Cargo/no-run probe, Git, evidence/document mutation,
network, dependency work, actor or subagent. Stop after the source drop.

Implementation and fourteen actor evidence records remain uncommitted. Reviewer
publication scope: this handoff, CURRENT_TASK.md and tickets/BBD-WAL-009.md only.
Keep reviewer XHigh for the production source review. Grok runs High. Launch once,
record session/outer identity, collect after owner done, never poll an actor.

## Frozen inventory

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| wallet-broker/src/zec/spend.rs | 1125 | 0f47d4f8a7e611997d3276aa090a8585b962b27e820408feea6afd47f14a0b9a |
| wallet-broker/src/zec/test_support.rs | 4373 | f310bdd3ebc95baa5e4f1cf3fd710fb23aaf5fdcfad869eb9bd5105e721be145 |
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
| wallet-broker/src/vault.rs | 776 | bb79b16f24a9abbcd266e856a37ee982bde8fcdc2da3858e75d34bdd34a4af86 |
