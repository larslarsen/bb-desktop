# WAL-009 decoded effects expected red 01

Integration note (2026-09-09): local repository and home prefixes are rendered as <repo> and <home>. Earlier hashes/counts describe execution-time files. This is an archival evidence record; acceptance limits and reported deviations remain governed by the corresponding reviewer handoffs. No test was repeated for this source checkpoint.


Actor: Jr Dev — Hermes, execution/evidence only.
Reviewer: Codex, XHigh.
Authorized handoff: docs/handoff/HERMES_BBD_WAL_009_DECODED_EFFECTS_EXPECTED_RED_01.md.
Source acceptance checkpoint: 3ea994fb. Grok session 7ddfcee6-c773-4d03-af90-51da48895a33.

## Runtime and launch

- Hermes version (recorded once, first run): `Hermes Agent v0.18.2 (2026.7.7.2) · upstream b1f003e1 · local 10b6d1a9 (+1 carried commit)`, Python 3.11.15, OpenAI SDK 2.24.0.
- Provider/model: nous / poolside/laguna-s-2.1:free.
- Runtime session ID: captured from terminal tool background launch; outer terminal 81835 (recorded in
  CURRENT_TASK.md). The cargo test process launched once via terminal background=true,
  notify_on_complete=true, workdir=<repo>.
- Launch process/session identifiers:
  - Terminal background process session_id: `proc_bd6c8d31578b`
  - Cargo test PID: 3459757 (rustup wrapper) → test binary PID 3460119
  - Exit code: 101

## Git state (recorded once, before execution)

- HEAD: `5405a5801f17f99a6901ec5147082df8c64097bd` (parent: `123f26c25feffbe68d6ff0a0442d6e0f220ece4e`)
- `git diff --cached --name-only`: empty (clean index)
- `git status --short --untracked-files=all`: 11 modified, 13 untracked

  ```text
   M package-lock.json
   M package.json
   M wallet-broker/Cargo.lock
   M wallet-broker/Cargo.toml
   M wallet-broker/src/native.rs
   M wallet-broker/src/native_ui.rs
   M wallet-broker/src/zec.rs
   M wallet-broker/src/zec/prepare.rs
   M wallet-broker/src/zec/store.rs
   M wallet-broker/src/zec/test_support.rs
   M wallet-broker/tests/zec_sign_verify.rs
  ?? docs/testing/BBD-WAL-009-LOCK-SYNC-01.md
  ?? docs/testing/BBD-WAL-009-NATIVE-APP-EXPECTED-RED-01.md
  ?? docs/testing/BBD-WAL-009-NATIVE-LAYOUT-VALIDATION-01.md
  ?? docs/testing/BBD-WAL-009-NATIVE-REVIEW-EXPECTED-RED-01.md
  ?? docs/testing/BBD-WAL-009-NATIVE-REVIEW-PREREQUISITE-02.md
  ?? docs/testing/BBD-WAL-009-NATIVE-REVIEW-VALIDATION-01.md
  ?? docs/testing/BBD-WAL-009-RETAINED-SPEND-VALIDATION-01.md
  ?? docs/testing/BBD-WAL-009-RETAINED-SPEND-VALIDATION-RESUME-01.md
  ?? docs/testing/BBD-WAL-009-SIGNATURE-CONTEXT-EXPECTED-RED-01.md
  ?? docs/testing/BBD-WAL-009-SIGNATURE-CONTEXT-VALIDATION-01.md
  ?? wallet-broker/src/native_ui/zec_native_app_tests.rs
  ?? wallet-broker/src/native_ui/zec_review_tests.rs
  ?? wallet-broker/src/zec/spend.rs
  ?? wallet-broker/src/zec/spend/external_binding_tests.rs
  ?? wallet-broker/src/zec/spend/verification_context_tests.rs
  ```

## Preflight inventory measurement (before execution)

All 27 frozen identities measured once via a single read-only script (line count + SHA-256).
All 27 matched the expected identities from HERMES_BBD_WAL_009_NATIVE_LAYOUT_VALIDATION_01.md
(updated: verification_context_tests.rs row replaced to 757 lines /
791e8cbb77d556ab05e35946e23cddbc3b5a27659c3ce7b5e2464a665210fda4; layout evidence row
added at docs/testing/BBD-WAL-009-NATIVE-LAYOUT-VALIDATION-01.md, 354 lines /
ffcfc14837ff02aba33a5a24fa6a58d610055b18b4efd150787c6b8acba7dc43; 27 total rows).

All 27 matched before execution.

## Authorized command (executed once)

```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --lib zec::spend::verification_context_tests::decoded_effects_reject_agreeing_metadata_that_disagrees_with_signed_bytes -- --exact
```

Launched via terminal tool with background=true, notify_on_complete=true,
workdir=<repo>. Session ID: proc_bd6c8d31578b.

## Complete saved process output

Rendered verbatim from the saved process tool JSON (session proc_bd6c8d31578b,
retrieved via process log with offset 0, limit 2000, total_lines 41).

```text
   Compiling bitbook-wallet-broker v0.1.0 (<repo>/wallet-broker)
warning: method `matches_review` is never used
  --> src/native.rs:77:19
   |
49 | impl ZecConfirmationCapability {
   | ------------------------------ method in this implementation
...
77 |     pub(crate) fn matches_review(&self, review: &ZecNativeReview) -> bool {
   |                   ^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: function `timestamp_for_test` is never used
  --> src/zec/test_support.rs:1528:4
    |
1528 | fn timestamp_for_test(value: &str) -> Result<u64, ZecError> {
     |     ^^^^^^^^^^^^^^^^^^

warning: `bitbook-wallet-broker` (lib test) generated 2 warnings
    Finished `test` profile [unoptimized + debuginfo] target(s) in 6.26s
     Running unittests src/lib.rs (wallet-broker/target/debug/deps/bitbook_wallet_broker-838b402781cf6bb9)

running 1 test
test zec::spend::verification_context_tests::decoded_effects_reject_agreeing_metadata_that_disagrees_with_signed_bytes has been running for over 60 seconds
test zec::spend::verification_context_tests::decoded_effects_reject_agreeing_metadata_that_disagrees_with_signed_bytes ... FAILED

failures:

---- zec::spend::verification_context_tests::decoded_effects_reject_agreeing_metadata_that_disagrees_with_signed_bytes stdout ----

thread 'zec::spend::verification_context_tests::decoded_effects_reject_agreeing_metadata_that_disagrees_with_signed_bytes' (3460120) panicked at src/zec/spend/verification_context_tests.rs:756:5:
receiver:ok,amount:ok,memo:ok,fee:ok,real Ironwood input count:ok,Ironwood output count:ok,spend pool:ok,has_transparent_bundle:ok,has_sapling_bundle:ok,has_orchard_output_bundle:ok
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    zec::spend::verification_context_tests::decoded_effects_reject_agreeing_metadata_that_disagrees_with_signed_bytes

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 6 filtered out; finished in 453.79s

error: test failed, to rerun pass `--lib`
```

## Result analysis

Exit code: 101 (as expected).

Compilation: succeeded with 2 pre-existing warnings (dead_code for `matches_review` in
src/native.rs:77 and `timestamp_for_test` in src/zec/test_support.rs:1528). No compile error.

Test execution: 0 passed; 1 failed; 0 ignored; 0 measured; 6 filtered out; finished in 453.79s.

Failure location: src/zec/spend/verification_context_tests.rs:756:5 (the final ten-case
assertion).

Failure diagnostic (verbatim from saved output):
```text
receiver:ok,amount:ok,memo:ok,fee:ok,real Ironwood input count:ok,Ironwood output count:ok,spend pool:ok,has_transparent_bundle:ok,has_sapling_bundle:ok,has_orchard_output_bundle:ok
```

Interpretation: The test ran the full regression — real signed fixture construction,
scoped outgoing-key recovery of outputs, incoming-decryption-based internal change
establishment, fee derivation from decoded value balance, memo normalization, and a
positive production-verifier result as an independent Oracle. Recovery and positive
verification succeeded (all ten cases marked `ok`), confirming the test reached its
final assertion. The production source then wrongly accepted all ten false metadata
claims, causing the final ten-case assertion to fail at line 756. This is the exact
expected-red outcome: exit 101, one test failing at its final ten-case assertion after
successful recovery and positive verification.

No compile, fixture, recovery, or crypto error. Not unexpected green. No source/test/old
evidence/governance edits, formatter, other tests, Cargo probes, Git mutation, network,
actor, or subagent were used.

## Post-execution inventory measurement (after execution)

The same 27 identities were measured once after execution. All 27 matched:

- wallet-broker/src/zec/spend.rs: 1046 lines / 8b70ecf7dea541d951184d89b1ed2d917ce5c3c6dceabb73151840c79517328a
- wallet-broker/src/zec/test_support.rs: 4369 lines / c34aa4dc95011c7585b44553e6e1680dd3c918ea78ffa98e07d2e271f7991159
- wallet-broker/src/zec/spend/verification_context_tests.rs: 757 lines / 791e8cbb77d556ab05e35946e23cddbc3b5a27659c3ce7b5e2464a665210fda4
- wallet-broker/src/native_ui.rs: 321 lines / d132a164a6413165291abd0582abf3eca4cacd3f33ce6eb4ac2b467e8774d960
- wallet-broker/src/native_ui/zec_review_tests.rs: 233 lines / 2eec3b60eb1b9fbd32235832a867d1189928126ca79373fa9f4fa7fdf2ee87bf
- wallet-broker/src/native.rs: 489 lines / 992138f18cbb0b969296109de3186aab923500c41ed2c0d41cf9c0696d3c3acc
- wallet-broker/Cargo.toml: 122 lines / 73e5e585eb2fd1ca867d66962460cfa010443886a4b59d8a33556ef2a4ff3503
- wallet-broker/Cargo.lock: 5395 lines / b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71
- package.json: 42 lines / 84b30b6860441a100588b5bdb92b37ddc45fb7610d48ee6f0e51c30ea0717780
- package-lock.json: 396 lines / 5e1122f32b0db42eb4386d4d0f0c47a4160d23cb4ae790bbb3600b5d3a5bddfc
- wallet-broker/src/zec.rs: 274 lines / 045cdc51f26ac8b9b1283cee5f995d60ceda38a25577aa7a937f5d07f177b90b
- wallet-broker/src/zec/prepare.rs: 1238 lines / 44c0783fa3d75867599092d25912032b665838ed7a73c81a16b0eec2b26ffb07
- wallet-broker/src/zec/store.rs: 2872 lines / 531d0a6171ecd9b5012602ccb870f16eab96888c809b3ffdf4e6303576f17c90
- docs/testing/BBD-WAL-009-LOCK-SYNC-01.md: 120 lines / ab25ffd2ac6ec7c26f7c21e42978697ec10da7ff395a9569afd9e78d1cd27ed2
- docs/testing/BBD-WAL-009-NATIVE-REVIEW-EXPECTED-RED-01.md: 199 lines / 42825e83ba6e98471c18b0c63d246e9a8790c25591e395cb6a0934c8d7bee6aa
- docs/testing/BBD-WAL-009-NATIVE-REVIEW-PREREQUISITE-02.md: 181 lines / 33d488dfcc88eb684d316bcffc547c051be412984d5633c54da76c1cef19dbe2
- docs/testing/BBD-WAL-009-SIGNATURE-CONTEXT-EXPECTED-RED-01.md: 216 lines / 79b6e97f5a6a4ca4903d9db7b40233ecf0f2ca931116e0d232853f1b842d1ff2
- wallet-broker/tests/zec_sign_verify.rs: 1118 lines / 7a481d3a954a92e04d324be047863a824ecf303bfd6232be70e9d13d3a295987
- wallet-broker/src/zec/spend/external_binding_tests.rs: 114 lines / bb2d6873dd7262ad6e30f117f1947b7d4cf7685ab04e32a5b700d7cb5b112b6d
- docs/testing/BBD-WAL-009-SIGNATURE-CONTEXT-VALIDATION-01.md: 677 lines / 6a23bcb2943df54953d1acddf85a2ec9cfc41b92e84c12856a48b7d07be53c58
- docs/testing/BBD-WAL-009-RETAINED-SPEND-VALIDATION-01.md: 358 lines / d51cc09e82f60a02a1493338d0af249dd25f046e2b5ca75e46a0fe4019269ed1
- docs/testing/BBD-WAL-009-RETAINED-SPEND-VALIDATION-RESUME-01.md: 560 lines / 62bb1ffd672c460f6fac6439abbc8d555683342e5410ad7098d2339bf509e5d0
- wallet-broker/tests/native_surface.rs: 664 lines / 349f3a019a0e7a5c37aaae727192b0ddcecb02192a5fae6df9291eaf1357276d
- wallet-broker/src/native_ui/zec_native_app_tests.rs: 376 lines / 485ce9691f873af9304e1db36de74d49627279abf9c550ccce4c74bada22a5a4
- docs/testing/BBD-WAL-009-NATIVE-REVIEW-VALIDATION-01.md: 503 lines / 3a734781d0ad2c31989318373272f13ba89654ded2ab3a6b90f83ea322821de1
- docs/testing/BBD-WAL-009-NATIVE-APP-EXPECTED-RED-01.md: 184 lines / e749d709d8d2bd8f5e72a8509c156a2053dbed5983849964348eefbc3429733b
- docs/testing/BBD-WAL-009-NATIVE-LAYOUT-VALIDATION-01.md: 354 lines / ffcfc14837ff02aba33a5a24fa6a58d610055b18b4efd150787c6b8acba7dc43

All 27 identities match before and after execution. No source or test files were
modified by execution; no files were created except this evidence record.

## Normalization disclosure

Local absolute paths under <repo> were not normalized in the
saved compiler/test output — they appear verbatim as emitted by cargo. No output was
reconstructed, truncated, or inferred. The output block above is rendered directly from
the saved process tool JSON (session proc_bd6c8d31578b, retrieved via process log with
offset 0 / limit 2000, total_lines 41).
