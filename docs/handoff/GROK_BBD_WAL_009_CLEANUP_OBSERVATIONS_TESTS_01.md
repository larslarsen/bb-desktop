# WAL-009 cleanup observations regression 01

Reviewer: Codex, XHigh. Actor: Sr Dev — Grok Build, CLI grok-4.6 High, source only.
Baseline governance: 7e9bf8b2; protected parent is the commit publishing this handoff.
Authorization: one append-only test-source edit. No execution or integration.

## Fixed finding and test boundary

The [decoded-effects acceptance](../testing/BBD-WAL-009-DECODED-EFFECTS-VALIDATION-01-ACCEPTANCE.md)
closes that repair and retains all six completed validation outcomes. The remaining
cleanup finding is separate: a real signer failure occurs before proof/extraction,
yet test_support.rs synthesizes touch/wipe observations for those unreached classes
by wiping unrelated dummy buffers. The new regression must expose that contradiction
through the existing real pipeline. No proof construction is needed for this path.

Edit only wallet-broker/tests/zec_sign_verify.rs. Starting identity: 1118 lines,
SHA-256 7a481d3a954a92e04d324be047863a824ecf303bfd6232be70e9d13d3a295987.
Preserve every existing byte; append exactly one test named:
signer_failure_does_not_report_cleanup_for_unreached_secret_classes
Use the existing imports/helpers. Local arrays/collection in this test are allowed;
no shared helper, import rewrite, other test change or new production API.

Required sequence:
1. Build one existing software_harness, prepare one real frozen fixture payment,
   and obtain its existing native confirmation. Reset the call counters and attach
   a fresh SignVerifyObservations after that setup. Install no canaries.
2. Invoke sign_with_fault_for_test exactly once, with SignRoute::Software,
   FaultPoint::Signer and ManualClock::at(NOW). This calls the actual production
   authorization/signing path, then fails before the prover. Do not use
   exercise_terminal_exit_for_test, exercise_fault_exit_for_test, synthetic cleanup
   helpers, manual event injection, source-text inspection or a fake pipeline.
3. Require INTERNAL with the established public error message. Require exactly one
   seed access, spending-authority derivation, authoritative PCZT access and signer
   call. Require zero prover/finalizer/extractor/independent-decoder/verifier calls,
   no verified/broadcast output, and released account lock. These guards establish
   that the operation reached the real signer and stopped before later stages.
4. Require positive Seed touch and positive Seed wipe observations for SignerError,
   with zero failed Seed wipes. This is a guard against silencing all observations;
   production repair must wire it to the actual operation-owned seed bytes. The
   existing dummy seed observation lets this guard pass on the flawed baseline.
   It is not by itself proof that the baseline cleans up its actual keys.
5. For ProofWorkspace, ExtractedTransaction, SignerView and SignatureContribution,
   require zero touch_count, positive_wipe_count and failed_wipe_count across ALL
   fourteen existing WipeExit variants (not only SignerError). This software attempt
   never constructs any of those four classes. An inaccurate event cannot be hidden
   under Success, Error or another exit label. Keep the observations before harness
   teardown so the test examines this one completed attempt only.
6. Collect nonzero forbidden observations into one bounded diagnostic list and fail
   once at the end. Print only class/exit/counter/count, never secret bytes, receiver,
   memo or raw transaction. Expected current failure contains positive ProofWorkspace
   and ExtractedTransaction touch/wipe counts under SignerError. Setup/guard failure,
   compilation failure or earlier crypto failure is not the intended regression red.

This test proves honest non-observation of unreached classes plus one necessary
positive owned-byte cleanup guard. Full actual-secret ownership, panic and success
coverage remains open. Do not weaken existing tests or claim a broad cleanup fix.
After accepted red the reviewer will authorize production ownership/observer work;
suppressing counters, matching this test name or changing error labels is not a fix.

## Future execution and falsification (not authorized now)

After source review, Hermes will run exactly:

```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_sign_verify signer_failure_does_not_report_cleanup_for_unreached_secret_classes -- --exact
```

Expected red: exit 101, one failed, 14 filtered, final forbidden-observation
assertion after all real-signer and positive-seed guards passed. No full suite or
old expensive proof run is needed to establish this red. After repair, the same
test must turn green. Falsification will inject one spurious ProofWorkspace event
at this error boundary and require this assertion to fail while the positive
seed/real-signer guards still pass, followed by exact restoration. The reviewer
will define the concrete temporary patch against accepted production source.

## Actor procedure and stop

Read AGENTS.md, TESTING.md, active CURRENT_TASK.md, ticket active prefix, this
handoff, the authorized integration test, spend.rs software-authorization section,
and the relevant SignVerifyHarness/observer methods in test_support.rs. The above
APIs already exist. No dependency archaeology or historical-document reload.

Before editing, measure the 31 frozen paths below with byte SHA-256/newline counts.
Parse this table in one inline read-only script, no helper file and no manually
reconstructed inventory. Stop on any mismatch. Append the one test, then measure
its final hash/line count and confirm that removing the added suffix reproduces
the exact 1118-line baseline. All other paths must retain their frozen identities.
No formatter, compiler, Cargo/test/no-run probe, Git command, evidence/document
write, source repair, network, actor or subagent. Do not run the future commands.
Stop after the source drop and report changed path, append-only check, final
identity and any blocker. No automatic follow-up or report-only correction work.

Reviewer High is sufficient for bounded source collection; use XHigh again before
cleanup production architecture/acceptance. Reviewer publication scope: this handoff,
decoded-effects acceptance, CURRENT_TASK.md and tickets/BBD-WAL-009.md only.
Launch once, record session/outer identity, collect after owner done, never poll.

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
| wallet-broker/tests/zec_sign_verify.rs | 1118 | 7a481d3a954a92e04d324be047863a824ecf303bfd6232be70e9d13d3a295987 |
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
