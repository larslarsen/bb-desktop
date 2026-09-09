# WAL-009 real signing-pipeline panic regression 01

Reviewer: Codex, XHigh. Source actor: Sr Dev — Grok Build, grok-4.6 High.
Baseline governance: dfaf06c5. Protected parent: the commit publishing this handoff.
Authorization: one test-source path, two exact regions. No execution or Git.

## Decision and scope

Cleanup lifecycle validation is accepted. Existing actual-owner Drop tests cover
panic locally; the integration panic helper still ignores handle/route/clock,
acquires a lease, invalidates prepare state, destroys canaries and panics INTERNAL
without invoking the production signing pipeline. The existing integration test
only checks that some panic occurred and a lock was released. It cannot prove
cleanup after proof generation or prevent a false panic-path acceptance.

Add one non-vacuous regression against that existing helper API. The initial red
must be real pipeline activity absent (seed_accesses=0 expected=1), after the
specific INTERNAL payload passes. No new method, import or production stub is needed.
After that red is accepted, a separately authorized source correction will route
the helper through actual signing, proof, extraction and independent verification,
with a fixed panic immediately after successful verification and before dropping
the serializer buffer or returning effects for publication. Actual counters must
survive unwind; normal operation must retain existing behavior. The correction
must invalidate the prepare/session state and resume the original panic while
owners/lease unwind normally. This paragraph is a design constraint, not source
or execution authorization for this test-only task.

The completed cleanup, decoded-effects and signing results remain accepted. Do
not repeat proof-heavy baselines, repair reports, open a demo milestone or widen
to upstream typed-secret zeroization/native OS/network integration. Their remaining
gaps remain explicitly open. This one focused path will require a real proof on
future green, but the initial red uses the current cheap helper.

## Exact writable source regions

Only wallet-broker/tests/zec_sign_verify.rs may change. Baseline: 1258 lines,
SHA-256 46a0cc520421e43e4ca3215240be9a7f92f7ef5581525727964d2247e7f660a7.

1. Delete only the final helper-panic subsection of
   account_authorization_lock_is_scoped_and_released_on_every_exit. It starts at
   the indented let mut harness = software_harness("sign-verify-panic-lock-release");
   and ends after that subsection's final account_lock_count(ACCOUNT)==0 assertion,
   before the function-closing newline/brace. Exact region: 817 UTF-8 bytes,
   20 newline bytes, SHA-256
   30081711def0ae1e0c0dbc11d60cee6b8c0078949ae50a1863660009e90de766.
   Preserve the function's earlier account-concurrency/terminal-exit checks and
   closing brace. Do not rewrite or rename those checks.
2. Append one new #[test] function at EOF named
   real_pipeline_panic_releases_owners_and_never_publishes. Reuse the existing
   fixture/input/software_harness/prepared, cleanup snapshot and exact observation
   assertion helpers; do not edit them or add a new public API. Any tiny payload
   extraction logic belongs inside the new test. Exactly 16 integration tests result.

Preserve every byte outside those two regions, including imports and the other
14 tests. No test ignoring, counter-setting, input WipeEvent construction, canary
installation, source-text oracle, copied secret evidence, forced early panic,
unsafe memory access, global panic-hook mutation or leaked/forgotten owner.
No production function/body/type/visibility, dependency, manifest/lock, native UI,
fixture, existing evidence, governance, formatter or Git change.

## Required new test sequence and oracle

Use software_harness with a unique harmless label, then real prepared and
confirm_native with NOW. Reset sign/verify calls after preparation/confirmation;
attach fresh shared SignVerifyObservations. No synthetic secret canaries.

Inside catch_unwind(AssertUnwindSafe(...)), call only the existing
panic_during_sign_for_test with the real handle/confirmation, Software and NOW.
Require an Err payload; accept only &str or String exactly equal to INTERNAL.
Do not accept merely is_err or invoke another panic after a normal return.

After matching the payload, assert the following in this order:

- seed_accesses exactly 1, with the fixed assertion message
  "real pipeline panic must access the operation seed". This is the initial red.
- spend_authority_derivations and authoritative_pczt_accesses exactly 1; signer,
  prover, finalizer, extractor, independent_decoder and verifier calls each 1.
  These counters must reflect real operations in later reviewed source, never
  manually constructed expected progress. They are stage evidence, not wipe proof.
- post_sign_status_reads and post_sign_clock_reads both zero: panic occurs before
  the publication/revalidation adapter. assert_no_verified_or_broadcast must pass;
  no hardware view/export/contribution/fallback counter may be nonzero.
- account_lock_count(ACCOUNT) is zero. Reacquire the same account through existing
  begin_authorization_for_test, assert held count 1, cancel that lease and assert 0.
- assert_exact_cleanup_observations must see exactly one touch, one positive wipe,
  zero failed wipes for each of Seed, AuthoritativePczt and ExtractedTransaction
  at PanicUnwind; all other class/exit cells and unclassified count are zero.
  Reuse the existing fixed table helper. This observes actual buffers only and
  makes no claim that third-party key/proof workspace memory has been zeroized.

Freeze the cleanup snapshot. Confirming the consumed old handle must return LOCKED.
Attempt fresh preparation with input_for(ACCOUNT, OTHER_REQUEST_ID, OTHER_INTENT_HASH)
and NOW without unlocking; require LOCKED, proving panic invalidated the session.
Use match/.err().expect as needed; do not add Debug to production authority types.
These rejection checks must not publish or change any cleanup snapshot cell.
Finally drop the harness and require the same snapshot, so deferred destruction
cannot create belated success/error observations. No successful reproving/retry is
needed to establish lock release or invalidation.

## Test-first execution plan (NOT authorized for Grok)

After reviewer source acceptance, Hermes will be authorized separately for exactly:

```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_sign_verify real_pipeline_panic_releases_owners_and_never_publishes -- --exact
```

Expected exit 101, one failed, 15 filtered. The INTERNAL payload guard passes and
then seed_accesses fails 0 versus 1 at the named assertion. Compile/setup failures
or a different assertion are not that red. Do not run the whole integration target.

Future implementation will be source-reviewed before executing this same focused
test for green. Falsification will temporarily suppress AccountAuthorizationLease
Drop's removal, requiring this test to fail at the post-panic lock count while the
specific panic/stage guards still pass, then restore exact source bytes. Existing
cleanup A-F falsifications remain valid; do not duplicate them. The future executor
handoff will freeze exact mutations, hashes and commands. No source mutation or
execution is authorized here; no broad/full-suite or security gate is yet authorized.

## Baseline, reporting and stop

Read AGENTS.md, TESTING.md, active CURRENT/ticket prefixes and this handoff. Inspect
only relevant local source for this bounded test. No historical reload, external
research/configuration discovery or other actors. Verify all frozen identities once
before edits, then once afterward with only this authorized test file permitted to
differ. Do not repeatedly remeasure unchanged files. Use only read-only measurement
commands; no compiler, formatter, Cargo/test, product, scanner, Git or network.

Report actual runtime model/session, the two regions changed, resulting path/line/
SHA-256, exact 16-test count and preserved other identities. State no execution
occurred. Stop after final read-only measurement and report; do not start Hermes.
If the contract is impossible with the existing API, stop with the concrete reason;
do not widen scope, invent progress counters or author production fixes.

Reviewer publication scope is CURRENT_TASK.md, tickets/BBD-WAL-009.md and this
handoff. No source/evidence integration is authorized. Launch once and collect on
owner done; never poll. High suffices for this bounded source task/collection.

## Frozen inventory (37 paths)

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
| wallet-broker/tests/zec_sign_verify.rs | 1258 | 46a0cc520421e43e4ca3215240be9a7f92f7ef5581525727964d2247e7f660a7 |
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
