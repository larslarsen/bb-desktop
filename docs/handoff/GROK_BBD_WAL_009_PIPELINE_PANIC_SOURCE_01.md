# WAL-009 pipeline panic red acceptance and source correction 01

Reviewer: Codex, High. Source actor: Sr Dev — Grok Build, grok-4.6 High.
Baseline governance: df2a21ab. Protected parent: the commit publishing this handoff.
Authorization: five regions in exactly two production-facing source files.
Tests are frozen. No execution, evidence or Git is authorized.

## Collected expected-red acceptance

ACCEPT the regression red. Hermes outer 34696 completed exit 0 after owner done;
session 20260908_232331_f34d0d, database provider nous / model
poolside/laguna-s-2.1:free; version output 79516 is Hermes v0.18.2.
The one test exited 101 with 0 passed / 1 failed / 15 filtered, compilation 1.60s,
test 1.62s. INTERNAL at test_support.rs:4280 passes the payload guard, then
zec_sign_verify.rs:1268 fails seed_accesses 0 versus 1 with the exact expected
message: real pipeline panic must access the operation seed.

Saved transcript: pre-inventory 79520/79521; test command/launch 79524/79525,
process proc_b9e0fccd2d63; wait/completion 79530/79531 (timeout 60); full log
79534/79535 (150 requested, all 51 lines returned); post-inventory 79538/79539;
evidence write 79542/79543; final measurement 79546/79547. Reviewer inspected the
complete output and matched log/completion, both exact inventory scripts and all
37 current identities. Evidence is 100 lines / 4291 bytes / SHA-256
acda820c0aa1b6be7cb8a8eea7a1310dc6b588502c1067d381ea4691795958c3.
No source mutation, repeated test, formatter, Git or network occurred.

Record deviations without a rerun or report-only correction actor:
- The Cargo command appended unrequested 2>&1. This only merged stderr into the
  retained output; the selected test/options and failure are intact. The report
  incorrectly calls the command verbatim and unredirected.
- The actor read historical CURRENT content, then guessed docs/tickets and
  docs/handoff ticket paths rather than tickets/BBD-WAL-009.md. Extra search/files,
  find/sort and ls/grep discovery followed. Two shell calls (79493/79505) failed
  because that tool does not exist; actual terminal discovery calls were read-only.
  The correct ticket was not read. Future prompts must name its exact root path.
- The evidence says post-inventory occurred after evidence creation, contradicting
  the saved order and absence assertion. It occurred before creation, as required.
- The report omits message IDs and both inventory outputs, retypes warning carets/
  indentation and removes two blank lines from the alleged full log. Raw saved log
  supplies the complete evidence. It also retains local absolute paths despite
  claiming normalization; do not integrate it unchanged under AGENTS.md.
- A todo call 79548 followed the final measurement. No subsequent source/execution
  change occurred. Correct report facts/portability in future bounded integration.

The observed red establishes a missing real-pipeline panic path, not a failing
cryptographic primitive. Preserve all accepted cleanup/signing/decoded-effects
results. The seventeen actor evidence records and implementation remain uncommitted.

## Exact writable source scope and frozen regions

1. wallet-broker/src/zec/spend.rs, baseline 1156 lines, SHA-256
   cb1f0889749b5f084f2421f41e9b3c77b9fe50a2241c0f39d29bf77ef793f81a.
   Only add one private PipelineFault variant and one panic branch at the specified
   finish_pipeline location. No other function, signature, call site or import edit.
2. wallet-broker/src/zec/test_support.rs, baseline 4466 lines, SHA-256
   d795d7fc1ee8c91b25412de52cee1fbd99d220c54515a3d6d0dd61e6efffe22a.
   Only wrap the authorize_software call in execute_pipeline, add the new exhaustive
   arm in wipe_exit_for_error, and replace panic_during_sign_for_test's body/unused
   parameter names. Prefer fully qualified std::panic paths; no import change.

Authorized baseline region identities:

| Region | Bytes | Newlines | SHA-256 |
| --- | --- | --- | --- |
| PipelineFault enum, pub(crate) through closing brace/newline | 115 | 8 | e3876dc0057719231b3bc6e3d2150d4e0933ee54aa69fc11d474b0f2aa6d8c91 |
| finish hook anchor, drop(encoded_transaction) through following Cleanup opening brace | 77 | 1 | 88fcba8c625a88cef825b508dcb3d36b32467cf67b9e1d91d183b7a43449087f |
| execute_pipeline authorize_software let statement, before drop(observer) | 265 | 10 | c1878820eed1ac1ff0a9de21bb0af524f3b7506b9948a53b1b62ecee7d3885fa |
| wipe_exit_for_error through blank line before PendingAuthorization | 730 | 18 | 9bc410e144f7903019414b13191e6be70bb53478d62a93e6b256407cc4e9da90 |
| panic_during_sign_for_test through blank line before exercise_terminal_exit_for_test | 612 | 19 | baca12f58ad0aa1fa054ca34ecc94371cf8d3b0e90ee8e355799611f71c329d0 |

Preserve every other byte, particularly all tests, AccountAuthorizationLease Drop,
AttemptOwner/collector/classification, actual secret wrapper Drops, normal publication,
existing stage counters/faults and external-signing path. No formatter or broad cleanup.
No dependency/manifest/lock, native UI, schema, public protocol/method/capability,
fixture, other source, evidence, governance, export/network/hardware or Git change.

## Required semantics

### Real fault location

Add PipelineFault::PanicAfterVerification to the existing crate-private enum. In
finish_pipeline, immediately after independently_verify has returned Ok effects
and BEFORE drop(encoded_transaction), check for this new fault and panic with
exactly the fixed harmless string INTERNAL. Both authoritative raw-PCZT and actual
serializer bytes must still be owned. Do not move/drop/copy them to a fake owner.
Do not advance or fabricate any stage counter, call record_triggered_fault for
this panic, return an INTERNAL Result, or put the panic before successful verification.
The existing normal cleanup/fault handling and return Ok(effects) remain intact.
The new variant is reachable only through the existing private fault parameter;
no new public fault/API route is added.

### Preserve evidence and invalidate during unwind

Wrap only the existing spend::authorize_software invocation inside execute_pipeline
with std::panic::catch_unwind(std::panic::AssertUnwindSafe(...)). Keep its actual
arguments, lifetime/ownership, and location. In the Ok branch, unwrap the inner
Result and continue the existing drop(observer), drop(seed_owner), merge and
success/error handling unchanged.

On Err(payload), merge the actual local pipeline_calls exactly once into the
harness, invalidate that account's prepare state with HandleInvalidation::PanicUnwind,
and immediately std::panic::resume_unwind(payload). Use a fresh account lookup
after merging so immutable account borrows do not conflict with self mutation.
Do not catch and convert the panic to Result or select/emit a synthetic wipe exit.
Do not explicitly drop attempt/seed/lease before resume_unwind; their ordinary
unwind Drop must run. The raw PCZT and serializer owners have already unwound
inside authorize_software; their events remain queued in the existing collector.
AttemptOwner must classify all three real events as PanicUnwind when resumed
unwind leaves execute_pipeline. Never call classify/record_event directly.

This catch applies to actual software-authorization panics, including unrelated
ones; preserve their original payload and invalidate authorization. Do not catch
assertions in the integration test, swallow a panic, call a global panic hook,
change the successful/Result-error path, or broaden to external signing here.

Add only an exhaustive PipelineFault::PanicAfterVerification arm to the existing
wipe_exit_for_error match. It must delegate to wipe_exit_for_error(error, None)
so an ordinary returned error is classified by its actual code, never mislabeled
as unwind because a panic fault was requested. The panic hook does not record
that variant as a returned fault, so this is a defensive exhaustive mapping.

### Route the existing test adapter through real execution

Keep panic_during_sign_for_test's public signature and ! return type. Rename its
ignored handle/route/clock parameters to usable names and invoke self.execute with
those exact arguments, confirmation, Some(PipelineFault::PanicAfterVerification),
None mutation and None barrier. Remove the old standalone lease acquisition,
manual prepare invalidation and canary destruction from this helper. No direct
stage-counter assignment, dummy proof/result, additional test-only owner or direct
cleanup observer action is allowed here.

The intended INTERNAL panic must come from finish_pipeline and propagate through
the actual execute_pipeline ownership scope. Since the helper returns !, if
self.execute unexpectedly returns either Ok or Err, panic only with the distinct
fixed diagnostic "WAL-009 pipeline panic injection was not reached". Never format
that result/error or fall back to INTERNAL: the frozen test must reject a normal
return or earlier prerequisite failure as the intended pipeline panic.

## Frozen tests and subsequent validation plan

Do not edit the accepted 1338-line/16-test zec_sign_verify target or either cleanup
module. The current red is accepted; do not rerun it before this source task.
After source acceptance, Hermes will receive an exact separate validation handoff:

- the new real_pipeline_panic_releases_owners_and_never_publishes test must pass
  through one real software proof and observe all expected stages/cleanup;
- falsify AccountAuthorizationLease Drop removal for the same test, requiring the
  post-panic lock assertion to fail after the real panic/stage guards pass, then
  restore exact bytes; preserve the valid initial green on restored identical source;
- run the six cheap cleanup lifecycle unit tests, the existing focused signer-error
  regression and cheap account-lock regression to check affected normal error/owner
  behavior; do not rerun accepted proof-heavy happy paths or full targets.

This future plan is not execution authorization. Exact selectors/commands/mutation
hashes and restoration guards will be frozen after source review. No full-wallet,
third-party typed-secret erasure, native OS integration or final security acceptance
is implied by this bounded change.

## Read/measure/report and stop

Read exactly AGENTS.md and TESTING.md, docs/handoff/CURRENT_TASK.md active prefix,
tickets/BBD-WAL-009.md active prefix (root tickets/, not docs/tickets/), and this
handoff. Read relevant source regions as needed. No historical reload or guessed
file discovery. Verify all 38 frozen identities once before source edits and once
afterward; only the two named source paths may differ. Measure the five allowed
regions before changing them. No repeated inventory measurements or test execution.

Report actual session/model, exact changed regions, resulting line/SHA-256 for both
files, 36 unchanged identities and frozen test counts. Stop after final read-only
measurement/report. No Cargo, compiler, formatter, test, scanner, product, network,
other actor, evidence, Git, integration or source edits outside these regions.
If an API/borrow issue cannot be solved within the stated regions, report it and
stop without widening paths or silently changing the semantics.

Reviewer publication scope: this handoff, CURRENT_TASK.md and tickets/BBD-WAL-009.md.
High remains sufficient. Launch once, collect on owner done, never poll.

## Frozen inventory (38 paths)

| Path | Lines | SHA-256 |
| --- | --- | --- |
| wallet-broker/src/zec/spend.rs | 1156 | cb1f0889749b5f084f2421f41e9b3c77b9fe50a2241c0f39d29bf77ef793f81a |
| wallet-broker/src/zec/test_support.rs | 4466 | d795d7fc1ee8c91b25412de52cee1fbd99d220c54515a3d6d0dd61e6efffe22a |
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
| wallet-broker/tests/zec_sign_verify.rs | 1338 | 99c0d053c247b21af1ebc090e6b284bfdc0c0e4f2b559e3720939a591411196b |
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
| wallet-broker/src/zec/test_support/cleanup_lifecycle_tests.rs | 452 | b280e7dfa0eb3f65360b33d6f2f5e7debb3fe53c1d4ea4ea8ba1fcfd54500c06 |
| wallet-broker/src/zec/spend/cleanup_lifecycle_tests.rs | 174 | ca7a373f8d009e9fc66e65ecfb9d63b4b1483fb924ab9f66dcba511a3f904ccc |
| docs/testing/BBD-WAL-009-CLEANUP-LIFECYCLE-VALIDATION-01.md | 400 | 46415af46bd4313598c9c30caea3c07b06cb2fdb5d0d6c2ce479919a7c58a869 |
| docs/testing/BBD-WAL-009-PIPELINE-PANIC-EXPECTED-RED-01.md | 100 | acda820c0aa1b6be7cb8a8eea7a1310dc6b588502c1067d381ea4691795958c3 |
