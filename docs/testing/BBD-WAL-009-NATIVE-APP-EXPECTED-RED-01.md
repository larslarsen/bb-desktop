# WAL-009 native App/layout focused execution 01 — evidence

Integration note (2026-09-09): local repository and home prefixes are rendered as <repo> and <home>. Earlier hashes/counts describe execution-time files. This is an archival evidence record; acceptance limits and reported deviations remain governed by the corresponding reviewer handoffs. No test was repeated for this source checkpoint.


Actor: Hermes Jr Dev. Governance parent: commit containing the active handoff.
ONE Cargo command, one new evidence file. No mutation or integration.

## Runtime lineage

- Runtime: Hermes Agent v0.18.2 (2026.7.7.2) · upstream fef0e16f · local 10b6d1a9 (+1 carried commit)
- Python: 3.11.15 · OpenAI SDK: 2.24.0
- Provider: nous
- Model: poolside/laguna-s-2.1:free
- Runtime session ID: HERMES_SESSION_ID=20260908_141026_ac52d5 (actual runtime session, not the adoption session ID or outer terminal at launch)
- Authorization: 148ef213
- Launch checkpoint (from CURRENT_TASK.md): launched once from authorization 148ef213 via `hermes -z` with `--pass-session-id`, outer terminal 86077

## Baseline git state

- git rev-parse HEAD: 11270e18d557f44d8495911bd02f6ff6384fff06
- git status --short --untracked-files=all (11 modified, 11 untracked — all expected pending paths):
  - Modified: package-lock.json, package.json, wallet-broker/Cargo.lock, wallet-broker/Cargo.toml,
    wallet-broker/src/native.rs, wallet-broker/src/native_ui.rs, wallet-broker/src/zec.rs,
    wallet-broker/src/zec/prepare.rs, wallet-broker/src/zec/store.rs,
    wallet-broker/src/zec/test_support.rs, wallet-broker/tests/zec_sign_verify.rs
  - Untracked: docs/testing/BBD-WAL-009-LOCK-SYNC-01.md,
    docs/testing/BBD-WAL-009-NATIVE-REVIEW-EXPECTED-RED-01.md,
    docs/testing/BBD-WAL-009-NATIVE-REVIEW-PREREQUISITE-02.md,
    docs/testing/BBD-WAL-009-NATIVE-REVIEW-VALIDATION-01.md,
    docs/testing/BBD-WAL-009-RETAINED-SPEND-VALIDATION-01.md,
    docs/testing/BBD-WAL-009-RETAINED-SPEND-VALIDATION-RESUME-01.md,
    docs/testing/BBD-WAL-009-SIGNATURE-CONTEXT-EXPECTED-RED-01.md,
    docs/testing/BBD-WAL-009-SIGNATURE-CONTEXT-VALIDATION-01.md,
    wallet-broker/src/native_ui/zec_native_app_tests.rs,
    wallet-broker/src/native_ui/zec_review_tests.rs,
    wallet-broker/src/zec/spend.rs,
    wallet-broker/src/zec/spend/external_binding_tests.rs,
    wallet-broker/src/zec/spend/verification_context_tests.rs
- git diff --cached --name-only: (empty — index has no staged changes)

## Frozen inventory — before and after match (all 25 identities)

All 25 file SHA-256 hashes and line counts matched the accepted inventory both before
execution and after execution. No file was mutated by the test run.

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| wallet-broker/src/zec/spend.rs | 1046 | 8b70ecf7dea541d951184d89b1ed2d917ce5c3c6dceabb73151840c79517328a |
| wallet-broker/src/zec/test_support.rs | 4369 | c34aa4dc95011c7585b44553e6e1680dd3c918ea78ffa98e07d2e271f7991159 |
| wallet-broker/src/zec/spend/verification_context_tests.rs | 528 | 45f3bdf2f1d08924d53c298517c631427ab052c2d4955b31e98aa555386d380a |
| wallet-broker/src/native_ui.rs | 306 | b0d4da8770a400315889e5d69ad830e3e08d707c5bc62ca1f6e633798703349c |
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
| docs/testing/BBD-WAL-009-SIGNATURE-CONTEXT-VALIDATION-01.md | 677 | 6a23bcb2943df54953d1acddf85a2ec9cfc41b92e84c12856a48b7d07be53c58 |
| docs/testing/BBD-WAL-009-RETAINED-SPEND-VALIDATION-01.md | 358 | d51cc09e82f60a02a1493338d2af249dd25f046e2b5ca75e46a0fe4019269ed1 |
| docs/testing/BBD-WAL-009-RETAINED-SPEND-VALIDATION-RESUME-01.md | 560 | 62bb1ffd672c460f6fac6439abbc8d5553342e5410ad7098d2339bf509e5d0 |
| wallet-broker/tests/native_surface.rs | 664 | 349f3a019a0e7a5c37aaae727192b0ddcecb02192a5fae6df9291eaf1357276d |
| wallet-broker/src/native_ui/zec_native_app_tests.rs | 376 | 485ce9691f873af9304e1db36de74d49627279abf9c550ccce4c74bada22a5a4 |
| docs/testing/BBD-WAL-009-NATIVE-REVIEW-VALIDATION-01.md | 503 | 3a734781d0ad2c31989318373272f13ba89654ded2ab3a6b90f83ea322821de1 |

## Exact command

Run once from bb-desktop root:

```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --features native-ui --lib native_ui::zec_native_app_tests
```

Portable path form (no local absolute paths exposed in command semantics):
`$HOME/.cargo/bin/rustup` resolves to the user's cargo home; the command runs from
`$HOME/OpenBazaar/bb-desktop`.

- rustup toolchain: 1.98.0
- cargo: 1.98.0 (797e8a9bc 2026-08-05)
- terminal background: true, notify_on_complete: true
- process session ID: proc_d0674bb9920f
- exit code: 101
- completion reason: exited

## Complete captured output

```
   Compiling bitbook-wallet-broker v0.1.0 (<repo>/wallet-broker)
warning: method `matches_review` is never used
  --> src/native.rs:77:19
   |
49 | impl ZecConfirmationCapability {
   | ------------------------------ method in this implementation
...
77 |     pub(crate) fn matches_review(&self, review: &ZecNativeReview) -> bool {
   |               ^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: function `timestamp_for_test` is never used
    --> src/zec/test_support.rs:1528:4
1528| fn timestamp_for_test(value: &str) -> Result<u64, ZecError> {
   |     ^^^^^^^^^^^^^^^^^^

warning: `bitbook-wallet-broker` (lib test) generated 2 warnings
    Finished `test` profile [unoptimized + debuginfo] target(s) in 6.46s
     Running unittests src/lib.rs (wallet-broker/target/debug/deps/bitbook_wallet_broker-6e59b086531f1b58)

running 4 tests
test native_ui::zec_native_app_tests::native_app_close_release_denies_and_closes_once ... ok
test native_ui::zec_native_app_tests::long_review_at_small_viewport_keeps_controls_usable ... FAILED
test native_ui::zec_native_app_tests::native_app_confirm_closes_once_and_preserves_owned_review ... ok
test native_ui::zec_native_app_tests::long_review_at_native_size_keeps_controls_usable ... ok

failures:

---- native_ui::zec_native_app_tests::long_review_at_small_viewport_keeps_controls_usable stdout ----

thread 'native_ui::zec_native_app_tests::long_review_at_small_viewport_keeps_controls_usable' (3394115) panicked at src/native_ui/zec_native_app_tests.rs:152:5:
assertion failed: viewport.contains_rect(rect)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

failures:
    native_ui::zec_native_app_tests::long_review_at_small_viewport_keeps_controls_usable

test result: FAILED. 3 passed; 1 failed; 0 ignored; 0 measured; 12 filtered out; finished in 0.10s

error: test failed, to rerun pass `--lib`
```

## Counts

- Tests run: 4
- Passed: 3
- Failed: 1
- Ignored: 0
- Measured: 0
- Filtered out: 12

## Per-test classification

1. `long_review_at_native_size_keeps_controls_usable` — **PASS**. The long-review layout at
   native size (520x720) kept both Confirm and Cancel controls within the viewport. Asserted
   `viewport.contains_rect(rect)` at `zec_native_app_tests.rs:152` via
   `assert_rendered_button` / `assert_usable_controls`. No failure.

2. `long_review_at_small_viewport_keeps_controls_usable` — **FAIL (layout red)**. The
   long-review layout at the constrained small viewport (360x480) failed to keep an
   off-screen control within the viewport. Panic at `zec_native_app_tests.rs:152:5`:
   `assertion failed: viewport.contains_rect(rect)`. This is the anticipated behavioral red:
   an off-screen control violates viewport containment under the constrained layout. The
   failure is a genuine layout containment violation, not a compile/setup/paint-helper
   failure or zero-tests case.

3. `native_app_confirm_closes_once_and_preserves_owned_review` — **PASS**. The App
   lifecycle case for confirm-then-close ran existing logic and passed all assertions:
   zero close commands until confirmation, exactly one Close after confirmation, no Close
   on programmatic close request after consumption, preserved undrained approval through
   later frames, and fresh-instance isolation. This is the separate App lifecycle result,
   reported as passing per the source review's expectation that App callback lifecycle
   cases pass existing logic.

4. `native_app_close_release_denies_and_closes_once` — **PASS**. The App lifecycle case
   for close-release denial ran existing logic and passed all assertions: zero close
   commands on initial render, denied close on pointer release during pending confirmation
   (close_commands == 1 only when release_closes == true), no re-close on subsequent close
   request, and fresh-instance isolation. This is the separate App lifecycle result,
   reported as passing.

## Summary

- Anticipated behavioral red (layout): confirmed — exactly one of four tests failed,
  `long_review_at_small_viewport_keeps_controls_usable`, due to
  `viewport.contains_rect(rect)` failing at `zec_native_app_tests.rs:152` for an
  off-screen control in the constrained 360x480 layout.
- App lifecycle cases (2 of 4): both passed, reported separately as distinct outcomes.
- The result is 3 passed; 1 failed; not all-four green.
- Two cargo warnings (dead code) are pre-existing and unrelated to test behavior.
- No source repair, no rerun, no mutation, no integration, no Git operations performed.
- All 25 frozen identities remain unchanged before and after execution.
