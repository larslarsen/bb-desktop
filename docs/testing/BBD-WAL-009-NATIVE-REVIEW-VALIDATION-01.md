# WAL-009 native review validation 01

Integration note (2026-09-09): local repository and home prefixes are rendered as <repo> and <home>. Earlier hashes/counts describe execution-time files. This is an archival evidence record; acceptance limits and reported deviations remain governed by the corresponding reviewer handoffs. No test was repeated for this source checkpoint.


Actor: Hermes Jr Dev, execution and one evidence record only.
Governance parent: commit 2c8ea8f3187681f550299cc2a0c1cf242c619a2a (authorization aada9adb).
Run from bb-desktop root <repo>.

## Runtime identity

- hermes --version:
  ```
  Hermes Agent v0.18.2 (2026.7.7.2) · upstream fef0e16f · local 10b6d1a9 (+1 carried commit)
  Install directory: <home>/.hermes/hermes-agent
  Install method: git
  Python: 3.11.15
  OpenAI SDK: 2.24.0
  ```
- Provider/model: nous / meituan/longcat-2.0:free
- Session ID: outer terminal 29799 (from authorization invocation)
- Git HEAD: 2c8ea8f3187681f550299cc2a0c1cf242c619a2a
- git status --short --untracked-files=all:
  ```
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
  ?? docs/testing/BBD-WAL-009-NATIVE-REVIEW-EXPECTED-RED-01.md
  ?? docs/testing/BBD-WAL-009-NATIVE-REVIEW-PREREQUISITE-02.md
  ?? docs/testing/BBD-WAL-009-RETAINED-SPEND-VALIDATION-01.md
  ?? docs/testing/BBD-WAL-009-RETAINED-Spend-VALIDATION-RESUME-01.md
  ?? docs/testing/BBD-WAL-009-SIGNATURE-CONTEXT-EXPECTED-RED-01.md
  ?? docs/testing/BBD-WAL-009-SIGNATURE-CONTEXT-VALIDATION-01.md
  ?? wallet-broker/src/native_ui/zec_review_tests.rs
  ?? wallet-broker/src/zec/spend.rs
  ?? wallet-broker/src/zec/spend/external_binding_tests.rs
  ?? wallet-broker/src/zec/spend/verification_context_tests.rs
  ```
- git diff --cached --name-only: (empty — index is clean)
- Distinguish authorization: authorization checkpoint is aada9adb per CURRENT_TASK.md.
  The reviewer's launch/preflight source-review commit is 434d62f2; this execution
  runs from working-tree HEAD 2c8ea8f3.

## Preflight — inventory measurement

All 22 pending paths measured against the frozen inventory in the handoff. Every
entry matched (newline_count, sha256). Full JSON emitted from the measuring pass:

```json
{
  "wallet-broker/src/zec/spend.rs": {"newline_count": "1046", "sha256": "8b70ecf7dea541d951184d89b1ed2d917ce5c3c6dceabb73151840c79517328a", "matches": true},
  "wallet-broker/src/zec/test_support.rs": {"newline_count": "4369", "sha256": "c34aa4dc95011c7585b44553e6e1680dd3c918ea78ffa98e07d2e271f7991159", "matches": true},
  "wallet-broker/src/zec/spend/verification_context_tests.rs": {"newline_count": "528", "sha256": "45f3bdf2f1d08924d53c298517c631427ab052c2d4955b31e98aa555386d380a", "matches": true},
  "wallet-broker/src/native_ui.rs": {"newline_count": "303", "sha256": "c6d5fc3a4dee46f4700f2a3ae5866a5003746eab5aa6dd6bc20e5ba5c6d726b4", "matches": true},
  "wallet-broker/src/native_ui/zec_review_tests.rs": {"newline_count": "233", "sha256": "2eec3b60eb1b9fbd32235832a867d1189928126ca79373fa9f4fa7fdf2ee87bf", "matches": true},
  "wallet-broker/src/native.rs": {"newline_count": "489", "sha256": "992138f18cbb0b969296109de3186aab923500c41ed2c0d41cf9c0696d3c3acc", "matches": true},
  "wallet-broker/Cargo.toml": {"newline_count": "122", "sha256": "73e5e585eb2fd1ca867d66962460cfa010443886a4b59d8a33556ef2a4ff3503", "matches": true},
  "wallet-broker/Cargo.lock": {"newline_count": "5395", "sha256": "b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71", "matches": true},
  "package.json": {"newline_count": "42", "sha256": "84b30b6860441a100588b5bdb92b37ddc45fb7610d48ee6f0e51c30ea0717780", "matches": true},
  "package-lock.json": {"newline_count": "396", "sha256": "5e1122f32b0db42eb4386d4d0f0c47a4160d23cb4ae790bbb3600b5d3a5bddfc", "matches": true},
  "wallet-broker/src/zec.rs": {"newline_count": "274", "sha256": "045cdc51f26ac8b9b1283cee5f995d60ceda38a25577aa7a937f5d07f177b90b", "matches": true},
  "wallet-broker/src/zec/prepare.rs": {"newline_count": "1238", "sha256": "44c0783fa3d75867599092d25912032b665838ed7a73c81a16b0eec2b26ffb07", "matches": true},
  "wallet-broker/src/zec/store.rs": {"newline_count": "2872", "sha256": "531d0a6171ecd9b5012602ccb870f16eab96888c809b3ffdf4e6303576f17c90", "matches": true},
  "docs/testing/BBD-WAL-009-LOCK-SYNC-01.md": {"newline_count": "120", "sha256": "ab25ffd2ac6ec7c26f7c21e42978697ec10da7ff395a9569afd9e78d1cd27ed2", "matches": true},
  "docs/testing/BBD-WAL-009-NATIVE-REVIEW-EXPECTED-RED-01.md": {"newline_count": "199", "sha256": "42825e83ba6e98471c18b0c63d246e9a8790c25591e395cb6a0934c8d7bee6aa", "matches": true},
  "docs/testing/BBD-WAL-009-NATIVE-REVIEW-PREREQUISITE-02.md": {"newline_count": "181", "sha256": "33d488dfcc88eb684d316bcffc547c051be412984d5633c54da76c1cef19dbe2", "matches": true},
  "docs/testing/BBD-WAL-009-SIGNATURE-CONTEXT-EXPECTED-RED-01.md": {"newline_count": "216", "sha256": "79b6e97f5a6a4ca4903d9db7b40233ecf0f2ca931116e0d232853f1b842d1ff2", "matches": true},
  "wallet-broker/tests/zec_sign_verify.rs": {"newline_count": "1118", "sha256": "7a481d3a954a92e04d324be047863a824ecf303bfd6232be70e9d13d3a295987", "matches": true},
  "wallet-broker/src/zec/spend/external_binding_tests.rs": {"newline_count": "114", "sha256": "bb2d6873dd7262ad6e30f117f1947b7d4cf7685ab04e32a5b700d7cb5b112b6d", "matches": true},
  "docs/testing/BBD-WAL-009-SIGNATURE-CONTEXT-VALIDATION-01.md": {"newline_count": "677", "sha256": "6a23bcb2943df54953d1acddf85a2ec9cfc41b92e84c12856a48b7d07be53c58", "matches": true},
  "docs/testing/BBD-WAL-009-RETAINED-SPEND-VALIDATION-01.md": {"newline_count": "358", "sha256": "d51cc09e82f60a02a1493338d0af249dd25f046e2b5ca75e46a0fe4019269ed1", "matches": true},
  "docs/testing/BBD-WAL-009-RETAINED-SPEND-VALIDATION-RESUME-01.md": {"newline_count": "560", "sha256": "62bb1ffd672c460f6fac6439abbc8d555683342e5410ad7098d2339bf509e5d0", "matches": true},
  "wallet-broker/tests/native_surface.rs": {"newline_count": "664", "sha256": "349f3a019a0e7a5c37aaae727192b0ddcecb02192a5fae6df9291eaf1357276d", "matches": true},
  "ALL_MATCH": true
}
```

native_ui.rs original (kept in memory, 303 lines):
SHA-256 c6d5fc3a4dee46f4700f2a3ae5866a5003746eab5aa6dd6bc20e5ba5c6d726b4

## Target cache check

- wallet-broker/target filesystem type: ext2/ext3 (disk-backed, not tmpfs).
- CARGO_TARGET_DIR: (empty)
- CARGO_BUILD_TARGET_DIR: (empty)
- No .cargo/config in bb-desktop root or wallet-broker. No conflicting target override.

## Stage 1 — original widget green

Command (launched once, background=true, notify_on_complete=true):
```
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --features native-ui --lib native_ui::zec_review_tests
```
Process ID: 3379109
Exit status: 0

Full saved output:
```
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
    Finished `test` profile [unoptimized + debuginfo] target(s) in 11.50s
     Running unittests src/lib.rs (wallet-broker/target/debug/deps/bitbook_wallet_broker-6e59b086531f1b58)

running 6 tests
test native_ui::zec_review_tests::cancel_click_denies_and_blocks_later_confirm ... ok
test native_ui::zec_review_tests::no_input_frames_and_outside_click_yield_no_confirmed_review ... ok
test native_ui::zec_review_tests::confirm_click_returns_exact_owned_review_and_cannot_rearm ... ok
test native_ui::zec_review_tests::escape_denies_and_blocks_later_confirm ... ok
test native_ui::zec_review_tests::viewport_close_denies_including_same_frame_confirm_release ... ok
test native_ui::zec_review_tests::fresh_dialog_is_independent_and_cancel_clears_undrained_confirm ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.14s
```

Result: 6 passed, 0 failed, 0 ignored, 0 measured, 6 filtered out.

## Stage 2 — confirm-falsification

### Mutation (from original, 303 lines)

Before mutation measurement:
```
c6d5fc3a4dee46f4700f2a3ae5866a5003746eab5aa6dd6bc20e5ba5c6d726b4  wallet-broker/src/native_ui.rs
303 wallet-broker/src/native_ui.rs
```

Replacement line 166, preserving twelve-space indentation:
```
-            self.state = ZecReviewState::Confirmed;
+            self.state = ZecReviewState::Denied;
```

After mutation measurement:
```
fc406fcf71021b1c08b8402e565ffcfc7e23b016ba366888324f5579bfdd6cca  wallet-broker/src/native_ui.rs
303 wallet-broker/src/native_ui.rs
```

### Execution

Command (launched once, background=true, notify_on_complete=true):
```
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --features native-ui --lib native_ui::zec_review_tests::confirm_click_returns_exact_owned_review_and_cannot_rearm -- --exact
```

Full saved output:
```
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

warning: variant `Confirmed` is never constructed
  --> src/native_ui.rs:115:5
   |
113 | enum ZecReviewState {
    |      -------------- variant in this enum
114 |     Pending,
115 |     Confirmed,
    |     ^^^^^^^^^^^

warning: `bitbook-wallet-broker` (lib test) generated 3 warnings
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.48s
     Running unittests src/lib.rs (wallet-broker/target/debug/deps/bitbook_wallet_broker-6e59b086531f1b58)

running 1 test
test native_ui::zec_review_tests::confirm_click_returns_exact_owned_review_and_cannot_rearm ... FAILED

failures:

---- native_ui::zec_review_tests::confirm_click_returns_exact_owned_review_and_cannot_rearm stdout ----

thread 'native_ui::zec_review_tests::confirm_click_returns_exact_owned_review_and_cannot_rearm' (3380090) panicked at src/native_ui/zec_review_tests.rs:153:5:
assertion failed: dialog.take_confirmed_review() == Some(original.clone())
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

failures:
    native_ui::zec_review_tests::confirm_click_returns_exact_owned_review_and_cannot_rearm

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 11 filtered out; finished in 0.04s
error: test failed, to rerun pass `--lib`
```

Result: exit 101, 1 executed, 0 passed, 1 failed, failing at zec_review_tests.rs:153
(`assertion failed: dialog.take_confirmed_review() == Some(original.clone())`) — not a
compile or fixture failure.

### Restoration (on every outcome)

Restored the complete original bytes immediately after this result.
Post-restoration measurement:
```
c6d5fc3a4dee46f4700f2a3ae5866a5003746eab5aa6dd6bc20e5ba5c6d726b4  wallet-broker/src/native_ui.rs
303 wallet-broker/src/native_ui.rs
```

## Stage 3 — close-precedence falsification

### Mutation (from restored original, 303 lines)

Before mutation measurement:
```
c6d5fc3a4dee46f4700f2a3ae5866a5003746eab5aa6dd6bc20e5ba5c6d726b4  wallet-broker/src/native_ui.rs
303 wallet-broker/src/native_ui.rs
```

Replacement block (lines 163-167), swapping close precedence over confirm:
```
-        if close_or_escape || cancel.clicked() {
-            self.cancel();
-        } else if confirm.clicked() && self.is_pending() {
-            self.state = ZecReviewState::Confirmed;
-        }
+        if confirm.clicked() && self.is_pending() {
+            self.state = ZecReviewState::Confirmed;
+        } else if close_or_escape || cancel.clicked() {
+            self.cancel();
+        }
```

After mutation measurement:
```
4fa5e01f20de8abe1d7a737fcdeaf162d46af3c34fcf79c7fc4989253a9f7e9a  wallet-broker/src/native_ui.rs
303 wallet-broker/src/native_ui.rs
```

### Execution

Command (launched once, background=true, notify_on_complete=true):
```
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --features native-ui --lib native_ui::zec_review_tests::viewport_close_denies_including_same_frame_confirm_release -- --exact
```

Full saved output:
```
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
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.09s
     Running unittests src/lib.rs (wallet-broker/target/debug/deps/bitbook_wallet_broker-6e59b086531f1b58)

running 1 test
test native_ui::zec_review_tests::viewport_close_denies_including_same_frame_confirm_release ... FAILED

failures:

---- native_ui::zec_review_tests::viewport_close_denies_including_same_frame_confirm_release stdout ----

thread 'native_ui::zec_review_tests::viewport_close_denies_including_same_frame_confirm_release' (3380611) panicked at src/native_ui/zec_review_tests.rs:190:5:
assertion failed: dialog.take_confirmed_review().is_none()
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

failures:
    native_ui::zec_review_tests::viewport_close_denies_including_same_frame_confirm_release

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 11 filtered out; finished in 0.08s
error: test failed, to rerun pass `--lib`
```

Result: exit 101, 1 executed, 0 passed, 1 failed, failing at zec_review_tests.rs:190
(`assertion failed: dialog.take_confirmed_review().is_none()`) — the close-only and
terminal-denial assertions passed first, then the simultaneous viewport close and
Confirm release assertion failed. Not a compile or fixture failure.

### Restoration (on every outcome)

Restored the complete original bytes immediately after this result.
Post-restoration measurement:
```
c6d5fc3a4dee46f4700f2a3ae5866a5003746eab5aa6dd6bc20e5ba5c6d726b4  wallet-broker/src/native_ui.rs
303 wallet-broker/src/native_ui.rs
```

The two mutations were never stacked. Tests and production were not edited, repaired,
or used to make a gate pass.

## Stage 4 — restored widget green

Command (launched once, background=true, notify_on_complete=true):
```
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --features native-ui --lib native_ui::zec_review_tests
```

Full saved output:
```
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
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.57s
     Running unittests src/lib.rs (wallet-broker/target/debug/deps/bitbook_wallet_broker-6e59b086531f1b58)

running 6 tests
test native_ui::zec_review_tests::no_input_frames_and_outside_click_yield_no_confirmed_review ... ok
test native_ui::zec_review_tests::confirm_click_returns_exact_owned_review_and_cannot_rearm ... ok
test native_ui::zec_review_tests::escape_denies_and_blocks_later_confirm ... ok
test native_ui::zec_review_tests::cancel_click_denies_and_blocks_later_confirm ... ok
test native_ui::zec_review_tests::viewport_close_denies_including_same_frame_confirm_release ... ok
test native_ui::zec_review_tests::fresh_dialog_is_independent_and_cancel_clears_undrained_confirm ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.12s
```

Result: 6 passed, 0 failed, 0 ignored, 0 measured, 6 filtered out.

## Stage 5 — existing native-surface regressions

Command (launched once, background=true, notify_on_complete=true):
```
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --features native-ui --test native_surface
```

Full saved output:
```
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

warning: fields `confirm` and `cancel` are never read
  --> src/native_ui.rs:121:5
   |
120 | struct ZecReviewControls {
    |        ----------------- fields in this struct
121 |     confirm: egui::Rect,
    |     ^^^^^^^^
122 |     cancel: egui::Rect,
    |     ^^^^^^

warning: `bitbook-wallet-broker` (lib) generated 3 warnings
    Finished `test` profile [unoptimized + debuginfo] target(s) in 15.22s
     Running tests/native_surface.rs (wallet-broker/target/debug/deps/native_surface-c267a99ad63cf982)

running 17 tests
test cancel_or_window_close_wipes_passphrase_and_performs_no_partial_action ... ok
test cancelled_native_xmr_installation_selection_stops_after_the_chooser ... ok
test export_and_restore_are_also_rejected_from_every_nonnative_origin ... ok
test generic_unlock_backup_and_future_payment_confirmation_methods_are_absent ... ok
test generic_methods_cannot_reach_xmr_installation_selection ... ok
test invalid_unlock_and_export_accounts_fail_before_native_authority_moves ... ok
test export_exchanges_only_a_path_with_dialog_and_ciphertext_stays_in_core ... ok
test invalid_passphrase_lengths_wipe_before_unlock_or_restore_custody_or_commit ... ok
test invalid_utf8_restore_passphrase_wipes_before_custody_or_commit ... ok
test invalid_utf8_unlock_passphrase_wipes_before_custody ... ok
test native_xmr_installation_selection_chooses_validates_verifies_and_persists_in_order ... ok
test password_prompt_is_masked_noncopyable_and_bounded ... ok
test native_error_text_is_closed_and_secret_free ... ok
test restore_authenticates_before_metadata_and_requires_explicit_confirmation ... ok
test restore_cancel_never_commits_or_changes_active_state ... ok
test unlock_is_accepted_only_from_native_surface_origin ... ok
test xmr_installation_selection_is_rejected_before_every_nonnative_side_effect ... ok

test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Result: 17 passed, 0 failed, 0 ignored, 0 measured, 0 filtered out.

## Stage 6 — native-feature compilation

Command (launched once, background=true, notify_on_complete=true):
```
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo check --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --features native-ui
```

Full saved output:
```
   Checking bitbook-wallet-broker v0.1.0 (<repo>/wallet-broker)
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

warning: fields `confirm` and `cancel` are never read
  --> src/native_ui.rs:121:5
   |
120 | struct ZecReviewControls {
   |        ----------------- fields in this struct
121 |     confirm: egui::Rect,
   |     ^^^^^^^^
122 |     cancel: egui::Rect,
   |     ^^^^^^

warning: `bitbook-wallet-broker` (lib) generated 3 warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 6.28s
```

Result: exit 0. Warnings recorded (3 dead-code warnings). No warnings were fixed;
no no-run probe was added. This is not a warning-denied Clippy gate or OS window test.

## Final measured inventory (after restoration, before evidence)

Re-measured all 22 inventory paths against the frozen inventory. Every entry matched:
- native_ui.rs: 303 lines, SHA-256 c6d5fc3a4dee46f4700f2a3ae5866a5003746eab5aa6dd6bc20e5ba5c6d726b4
- ALL_MATCH: true

## Limitations

These are real egui widget-event regressions with synthetic inputs plus existing
controller tests and compilation. They do not launch a native window, verify
real-length field layout/platform lifecycle, or exercise the complete native
capability bridge. No OS/native integration acceptance is claimed. No product
launch, full retained-spend/signature suite rerun, networking, broadcast, mainnet,
hardware, Monero, new source/test authoring, dependency action, or integration.

## Deviations

- No deviations from the authorized execution protocol. Every command launched once
  in background with notify_on_complete=true; wait used only on the returned process.
  No pipes, redirection, wrappers, environment assignments, appended flags, or retries.
  No rerun, compiler repair, extra probes/tests, Git mutation, or integration.
  No post-stage-6 extra checks or tests. Only one evidence record is written.
