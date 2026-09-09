# WAL-009 cleanup observation completion correction 01

Reviewer: Codex, XHigh. Actor: Sr Dev — Grok Build, grok-4.6 High, source only.
Baseline governance: aa18db13. Protected parent: the commit publishing this handoff.
Authorization: only wallet-broker/src/zec/test_support.rs may change.

## Collected source review

Grok session 58a0aeab-a2f5-4298-8fdf-73e8fa2c43f1, outer 90521, was collected
once after owner done and completed with exit 0. Runtime: grok-4.6-build, High.
The three-path production authorization is closed. Its transcript contains 25
successful bounded source replacements and one failed old-string match followed
by correction. No tests, compiler, formatter, Cargo, Git or evidence edits ran.

Reviewer inspection and exact reversal of successful replacements reproduced all
three authorized starting hashes. The other 30 inventory rows remained unchanged.
The moved actual seed, observed raw-PCZT owner, unchanged extracted-transaction
wiping, removal of the pseudo-authority seed copy, actual triggered-fault tracking,
and removal of fabricated operational producers are retained. vault.rs and spend.rs
are frozen at the reviewed identities below. No runtime cleanup acceptance is made.

The adapter drop is rejected for the following related issues; fix them together:

1. At baseline lines 1039 and 1099, both attempt bindings are immutable, but
   AttemptOwner::finish takes &mut self and is called at 1072 and 1115. This is
   a compile blocker identified by source review; no compiler was run.
2. finish immediately drains events and marks the collector flushed. publish calls
   it at 1168 before canary work, account invalidation, verified insertion, and
   return construction. finish_failed_pipeline likewise finalizes before cleanup.
   A subsequent unwind cannot replace the earlier label with PanicUnwind because
   the flush guard returns early. The collector also silently discards later events.
3. After wrapping the real seed, viewing_key_binding()? and prepare.consume()?
   at 1041/1043 bypass outcome selection. In particular, consume can return LOCKED,
   CANCELLED or EXPIRED, but the attempt currently defaults to generic Error.

The explicit canary-label ignore in classify also hides an unexpected producer;
canaries must remain outside the operational collector, with unexpected labels
counted as unclassified.

## Exact correction semantics

Keep this a bounded correction of the existing production contract. Do not rewrite
the pipeline, alter public error/status behavior, add secret copies, or change any
cryptographic operation, counter, clock/status read, capability or account policy.

- Make the two attempt bindings mutable. Add private selected-outcome state to
  AttemptOwner, initially absent (default Error). finish(&mut self, exit) only
  records that outcome; it must not drain events or call classification.
- The non-cloneable AttemptOwner's Drop is the sole finalization point. If the
  thread is panicking, choose PanicUnwind regardless of a selected outcome;
  otherwise choose the selected outcome or Error. Drain the collected metadata
  exactly once there, after actual observed secret owners have dropped.
- Remove the premature flushed state/early-discard path. The collector accepts
  actual events for the attempt's whole lifetime. Keep its metadata-only storage
  and poison-tolerant mutex use. Collector clones must not escape the attempt.
  Preserve existing owner declaration/drop order and carry the owner through
  PipelineOutcome. Do not manually drop it before publication or failure cleanup.
- In publish, complete the existing rechecks, randomness, canary work, invalidation,
  insertion and publication counter before selecting the successful outcome.
  Construct the unchanged VerifiedZecV1 result before selecting that outcome, then
  return it without further fallible work. Preserve existing error precedence and
  effects ownership; partial moves of other PipelineOutcome fields must leave
  the attempt alive until scope exit. A later unwind must still override selection.
- In finish_failed_pipeline, preserve cleanup and record the requested failure
  outcome before returning; finalization still occurs only at owner drop.
- For both fallible calls after seed_owner creation in execute_pipeline
  (viewing_key_binding and prepare.consume), use bounded explicit error handling
  to record wipe_exit_for_error(&error, None) and return the same error. RAII must
  drop the real seed owner before AttemptOwner drains its event. Do not use the
  requested injection to label these earlier errors. Do not add extra lookup,
  signing, proof, status/clock reads, invalidation or counter changes.
- Keep the existing triggered_fault-based classification for actual authorization
  failures. Keep Error for an unselected unfinished attempt and randomness failure.
- Only the three actual labels zec-operation-seed, zec-authoritative-pczt and
  zec-extracted-transaction map to operational classes. Remove the canary ignore
  arm: every unexpected label increments unclassified. Canary destruction continues
  using IgnoreWipes and must not feed the operational collector.

These are corrections to already-authorized outcome/ownership semantics. The
understood regression red is retained; no fresh expected-red execution or new
test-source round is authorized for this correction.

## Frozen tests, later execution and limits

All 1215 integration-test lines, all seven library tests and every other inventory
row stay byte-identical. Preserve the historical synthetic all-classes wipe test
without ignore, deletion, weakened assertions or dummy events. That test has no
valid real-pipeline cleanup coverage and is not accepted evidence.

After source review, a separate Hermes handoff will authorize the retained focused
green, bounded false-event and positive-seed-notification falsification, restoration,
and the two affected successful pipeline tests. The exact focused command remains:

```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_sign_verify signer_failure_does_not_report_cleanup_for_unreached_secret_classes -- --exact
```

Expected green: exit 0, one passed, 14 filtered. Accepted red: exit 101, one failed,
14 filtered, 1.87s, four invented proof/transaction events after positive Seed
guards. Do not rerun that baseline. Execution is not authorized in this source task.
The focused test alone does not establish all lifecycle outcomes or typed-object
erasure. A real-ownership lifecycle test contract remains necessary before broader
cleanup acceptance, including early failures and unwind classification. The fixture's
already-zero seed cannot prove erasure by merely suppressing zeroization.

Retain decoded-effects acceptance and prior valid checks; no expensive library
proof reruns. Full typed-secret lifetime/erasure, native OS/owning-thread/capability
integration and final security remain open. Network, broadcast, mainnet, real
hardware, Monero and Electron send remain parked. Implementation and fourteen actor
evidence records remain uncommitted. No usable send flow is delivered by this task.

## Actor procedure and stop

Read AGENTS.md, TESTING.md, active CURRENT_TASK.md, ticket active prefix and this
handoff. Inspect only relevant local source sections; no historical handoff reload.
Verify all 33 rows below with one inline read-only script parsed from this table;
stop on mismatch. Do not create a helper file or reconstruct an inventory manually.
Edit only test_support.rs. Report its final SHA-256/newline count and verify all
other 32 rows unchanged. Explain the owner drop order, final outcome selection,
early error route, publication ordering and unknown-label handling in the final reply.

No formatter, compiler, test/Cargo/no-run probe, Git, evidence/document mutation,
network, dependency work, actor or subagent. Stop after the source drop. Use the
existing disk-backed wallet-broker/target for future separately authorized execution;
do not create large temporary artifacts.

Reviewer publication scope: this handoff, docs/handoff/CURRENT_TASK.md and
tickets/BBD-WAL-009.md only. Retain reviewer XHigh for the corrected source review.
Grok runs High. Launch once, record session/outer identity, collect after owner done,
never poll an actor.

## Frozen inventory

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| wallet-broker/src/zec/spend.rs | 1153 | b4e9b87c743394e638011076cab673945328b0fbd23b36de53a42db3c93028fe |
| wallet-broker/src/zec/test_support.rs | 4459 | d88966a1b2644cba525d905d7482359020097c5c27260d9a1889175db4282036 |
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
