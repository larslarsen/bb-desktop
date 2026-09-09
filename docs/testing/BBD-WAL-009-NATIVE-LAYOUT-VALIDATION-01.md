# BBD-WAL-009 Native Layout Validation — Evidence Record 01

Integration note (2026-09-09): local repository and home prefixes are rendered as <repo> and <home>. Earlier hashes/counts describe execution-time files. This is an archival evidence record; acceptance limits and reported deviations remain governed by the corresponding reviewer handoffs. No test was repeated for this source checkpoint.


## Runtime identity

- Hermes Agent version: v0.18.2 (2026.7.7.2) · upstream b1f003e1 · local 10b6d1a9 (+1 carried commit)
- Provider/model: nous / poolside / laguna-s-2.1:free
- Rust toolchain: 1.98.0 via `$HOME/.cargo/bin/rustup`
- Workdir: <repo> (wallet-broker/target disk-backed)

## Authorization and launch lineage

- Original authorization commit: 504b4953 (review: accept layout repair and authorize grouped validation)
- Original checkpoint commit: 0e60e76e (docs: checkpoint grouped layout validation launch)
- Original accepted stage-1 commit: ad6c325b (review: retain ten-test green and resume after capacity stop)
- Continuation checkpoint commit: 6de87b53 (docs: checkpoint remaining layout validation resume) — local HEAD
- Original Hermes session: 20260908_145855_f82684 (nous/poolside/laguna-s-2.1:free, Hermes 0.18.2)
- Original outer terminal: 10169
- Original stage-1 result: saved 78612, command 78608, process proc_2b93b268f394 (outer 78610), exit 0, ten passed, 6 filtered, 0.15 seconds
- Resumed session: continued 20260908_145855_f82684 once under the capacity amendment in HERMES_BBD_WAL_009_NATIVE_LAYOUT_VALIDATION_01.md, launched once from ad6c325b with --resume 20260908_145855_f82684 and --pass-session-id, outer terminal 14430
- Reviewer confirmed all 26 starting identities match and no mutation was reached before capacity failure

## Capacity interruption disclosure

- Original session hit upstream HTTP 429 capacity failure after stage 1 passed; stage 1's saved output (78612) is retained and accepted by the reviewer and was NOT rerun.
- No preflight, Git, version, or Cargo probe was rerun for the resume; only the five prescribed commands were executed for stages 2–5.

## Frozen inventory — measured before and after

All 26 paths measured immediately before stage 2 and after final restoration (stage 5). Both checks: all 26 hashes and line counts match the frozen inventory. No path drifted.

| # | Path | Lines | SHA-256 (first 16) |
|---|------|------:|-------------------|
| 1 | wallet-broker/src/zec/spend.rs | 1046 | 8b70ecf7dea541d9 |
| 2 | wallet-broker/src/zec/test_support.rs | 4369 | c34aa4dc95011c75 |
| 3 | wallet-broker/src/zec/spend/verification_context_tests.rs | 528 | 45f3bdf2f1d08924 |
| 4 | wallet-broker/src/native_ui.rs | 321 | d132a164a6413165 |
| 5 | wallet-broker/src/native_ui/zec_review_tests.rs | 233 | 2eec3b60eb1b9f7 |
| 6 | wallet-broker/src/native.rs | 489 | 992138f18cbb0b96 |
| 7 | wallet-broker/Cargo.toml | 122 | 73e5e585eb2fd1ca |
| 8 | wallet-broker/Cargo.lock | 5395 | b960bc39d9bd3231 |
| 9 | package.json | 42 | 84b30b6860441a10 |
| 10 | package-lock.json | 396 | 5e1122f32b0db42e |
| 11 | wallet-broker/src/zec.rs | 274 | 045cdc51f26ac8b9 |
| 12 | wallet-broker/src/zec/prepare.rs | 1238 | 44c0783fa3d75867 |
| 13 | wallet-broker/src/zec/store.rs | 2872 | 531d0a6171ecd9b5 |
| 14 | docs/testing/BBD-WAL-009-LOCK-SYNC-01.md | 120 | ab25ffd2ac6ec7c2 |
| 15 | docs/testing/BBD-WAL-009-NATIVE-REVIEW-EXPECTED-RED-01.md | 199 | 42825e83ba6e9847 |
| 16 | docs/testing/BBD-WAL-009-NATIVE-REVIEW-PREREQUISITE-02.md | 181 | 33d488dfcc88eb68 |
| 17 | docs/testing/BBD-WAL-009-SIGNATURE-CONTEXT-EXPECTED-RED-01.md | 216 | 79b6e97f5a6a4ca4 |
| 18 | wallet-broker/tests/zec_sign_verify.rs | 1118 | 7a481d3a954a92e |
| 19 | wallet-broker/src/zec/spend/external_binding_tests.rs | 114 | bb2d6873dd7262ad |
| 20 | docs/testing/BBD-WAL-009-SIGNATURE-CONTEXT-VALIDATION-01.md | 677 | 6a23bcb2943df549 |
| 21 | docs/testing/BBD-WAL-009-RETAINED-SPEND-VALIDATION-01.md | 358 | d51cc09e82f60a02 |
| 22 | docs/testing/BBD-WAL-009-RETAINED-SPEND-VALIDATION-RESUME-01.md | 560 | 62bb1ffd672c460f |
| 23 | wallet-broker/tests/native_surface.rs | 664 | 349f3a019a0e7a5c |
| 24 | wallet-broker/src/native_ui/zec_native_app_tests.rs | 376 | 485ce9691f873af9 |
| 25 | docs/testing/BBD-WAL-009-NATIVE-REVIEW-VALIDATION-01.md | 503 | 3a734781d0ad2c31 |
| 26 | docs/testing/BBD-WAL-009-NATIVE-APP-EXPECTED-RED-01.md | 184 | e749d709d8d2bd8 |

## native_ui.rs original identity (verified before every mutation and after every restoration)

- Original lines: 321
- Original SHA-256: d132a164a6413165291abd0582abf3eca4cacd3f33ce6eb4ac2b467e8774d960

## Stage 1 (retained, not rerun)

- Command ID saved: 78608
- Process: proc_2b93b268f394 (outer 78610)
- Completion result saved: 78612
- Exit: 0
- Result: 10 passed; 6 filtered out; 0 failed; 0 ignored; 0 measured; 0.15 seconds
- Reviewer accepted this complete saved output; no rerun performed.

## Stage 2 — first exact mutation (unbounded body height falsification)

### Mutation
- Original line 159 (`.max_height(body_height)`):
```
            .max_height(body_height)
```
- Replaced with:
```
            .max_height(f32::INFINITY)
```
- Mutated file lines: 321
- Mutated file SHA-256: cf18eb0a012fb485ec091f6a142a34be479b6368516e51e7c73de59e1c63a366

### Command (stage 2, run once)
```
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --features native-ui --lib native_ui::zec_native_app_tests::long_review_at_small_viewport_keeps_controls_usable -- --exact
```
Process: proc_5005efdb1330 (background=true, notify_on_complete=true)

### Exit: 101
### Complete saved output:
```
   Compiling bitbook-wallet-broker v0.1.0 (<repo>/wallet-broker)
warning: unused variable: `body_height`
  --> src/native_ui.rs:157:13
    |
157 |         let body_height = (ui.available_height() - reserved_confirm_cancel_height(ui)).max(0.0);
    |             ^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_body_height`
    |
    = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: method `matches_review` is never used
  --> src/native.rs:77:19
   |
49 | impl ZecConfirmationCapability {
   | ------------------------------ method in this implementation
...
77 |     pub(crate) fn matches_review(&self, review: &ZecNativeReview) -> bool {
   |                   ^^^^^^^^^^^^^^
   |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default]

warning: function `timestamp_for_test` is never used
    --> src/zec/test_support.rs:1528:4
     |
1528 | fn timestamp_for_test(value: &str) -> Result<u64, ZecError> {
     |     ^^^^^^^^^^^^^^^^^^^^^

warning: `bitbook-wallet-broker` (lib test) generated 3 warnings (run `cargo fix --lib -p bitbook-wallet-broker --tests` to apply 1 suggestion)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.93s
     Running unittests src/lib.rs (wallet-broker/target/debug/deps/bitbook_wallet_broker-6e59b086531f1b58)

running 1 test
test native_ui::zec_native_app_tests::long_review_at_small_viewport_keeps_controls_usable ... FAILED

failures:

---- native_ui::zec_native_app_tests::long_review_at_small_viewport_keeps_controls_usable stdout ----

thread 'native_ui::zec_native_app_tests::long_review_at_small_viewport_keeps_controls_usable' (3414560) panicked at src/native_ui/zec_native_app_tests.rs:152:5:
assertion failed: viewport.contains_rect(rect)
note: run with `RUST_BACKTRACE=1` environment variable for more info

failures:
    native_ui::zec_native_app_tests::long_review_at_small_viewport_keeps_controls_usable

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.05s

error: test failed, to rerun pass `--lib`
```

### Falsification confirmed
- Exit 101 ✓
- Failure assertion: `assertion failed: viewport.contains_rect(rect)` at zec_native_app_tests.rs:152:5 ✓
- Expected: exit 101 and viewport.contains_rect failure at zec_native_app_tests.rs:152 (not compilation/setup) ✓
- Test counts: 0 passed; 1 failed; 0 ignored; 0 measured; 15 filtered out ✓

### Restoration
- Restored complete original bytes immediately after.
- Post-restoration (every outcome): 321 lines, SHA-256 d132a164a6413165291abd0582abf3eca4cacd3f33ce6eb4ac2b467e8774d960 ✓

## Stage 3 — second exact mutation (remove closing-guard re-entry)

### Mutation
- Original App block (lines 191–194):
```rust
            if !self.closing {
                self.dialog.borrow_mut().ui(ui);
            }
```
- Replaced with:
```rust
            {
                self.dialog.borrow_mut().ui(ui);
            }
```
- Mutated file lines: 321
- Mutated file SHA-256: 3788efe82a143601e7197f2769180c2972002ca4ea090b11056c7ef01db1a9bc

### Command (stage 3, run once)
```
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --features native-ui --lib native_ui::zec_native_app_tests::native_app_confirm_closes_once_and_preserves_owned_review -- --exact
```
Process: proc_682f7beb5d0e (background=true, notify_on_complete=true)

### Exit: 101
### Complete saved output:
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
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default]

warning: function `timestamp_for_test` is never used
    --> src/zec/test_support.rs:1528:4
     |
1528 | fn timestamp_for_test(value: &str) -> Result<u64, ZecError> {
     |     ^^^^^^^^^^^^^^^^^^^^^

warning: `bitbook-wallet-broker` (lib test) generated 2 warnings
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.93s
     Running unittests src/lib.rs (wallet-broker/target/debug/deps/bitbook_wallet_broker-6e59b086531f1b58)

running 1 test
test native_ui::zec_native_app_tests::native_app_confirm_closes_once_and_preserves_owned_review ... FAILED

failures:

---- native_ui::zec_native_app_tests::native_app_confirm_closes_once_and_preserves_owned_review stdout ----

thread 'native_ui::zec_native_app_tests::native_app_confirm_closes_once_and_preserves_owned_review' (3415155) panicked at src/native_ui/zec_native_app_tests.rs:333:5:
assertion failed: dialog.borrow_mut().take_confirmed_review() == Some(original.clone())
note: run with `RUST_BACKTRACE=1` environment variable for more info

failures:
    native_ui::zec_native_app_tests::native_app_confirm_closes_once_and_preserves_owned_review

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.05s

error: test failed, to rerun pass `--lib`
```

### Falsification confirmed
- Exit 101 ✓
- Failure assertion: `assertion failed: dialog.borrow_mut().take_confirmed_review() == Some(original.clone())` at zec_native_app_tests.rs:333:5 — the full-review equality assertion failing after close-request/idle frames ✓
- Earlier click/Close-count assertions passed (the test progressed through the click and close-request/idle frames before reaching the equality assertion at line 333) ✓
- Expected per handoff: exit 101 and full-review equality assertion failing after close-request/idle frames; compilation/setup failure not accepted ✓
- Test counts: 0 passed; 1 failed; 0 ignored; 0 measured; 15 filtered out ✓

### Restoration
- Restored complete original bytes immediately after.
- Post-restoration (every outcome): 321 lines, SHA-256 d132a164a6413165291abd0582abf3eca4cacd3f33ce6eb4ac2b467e8774d960 ✓
- No second mutation was stacked; stages 2 and 3 used separate restored-original baselines.

## Stage 4 — restored all TEN UI tests pass

### Command (stage 4, run once)
```
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --features native-ui --lib native_ui::
```
Process: proc_aa211ae54736 (background=true, notify_on_complete=true)

### Exit: 0
### Complete saved output:
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
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default]

warning: function `timestamp_for_test` is never used
    --> src/zec/test_support.rs:1528:4
     |
1528 | fn timestamp_for_test(value: &str) -> Result<u64, ZecError> {
     |     ^^^^^^^^^^^^^^^^^^^^^

warning: `bitbook-wallet-broker` (lib test) generated 2 warnings
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.67s
     Running unittests src/lib.rs (wallet-broker/target/debug/deps/bitbook_wallet_broker-6e59b086531f1b58)

running 10 tests
test native_ui::zec_review_tests::confirm_click_returns_exact_owned_review_and_cannot_rearm ... ok
test native_ui::zec_review_tests::escape_denies_and_blocks_later_confirm ... ok
test native_ui::zec_native_app_tests::native_app_close_release_denies_and_closes_once ... ok
test native_ui::zec_review_tests::cancel_click_denies_and_blocks_later_confirm ... ok
test native_ui::zec_review_tests::no_input_frames_and_outside_click_yield_no_confirmed_review ... ok
test native_ui::zec_review_tests::viewport_close_denies_including_same_frame_confirm_release ... ok
test native_ui::zec_native_app_tests::native_app_confirm_closes_once_and_preserves_owned_review ... ok
test native_ui::zec_native_app_tests::long_review_at_small_viewport_keeps_controls_usable ... ok
test native_ui::zec_native_app_tests::long_review_at_native_size_keeps_controls_usable ... ok
test native_ui::zec_review_tests::fresh_dialog_is_independent_and_cancel_clears_undrained_confirm ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.19s
```

### Restoration verification
- Exit 0 ✓
- 10 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out ✓
- 0.19 seconds (consistent with stage-1 0.15s) ✓
- All ten tests passed after both mutations were restored ✓

## Stage 5 — native compilation check

### Command (stage 5, run once)
```
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo check --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --features native-ui
```
Process: proc_ec52beff4bf9 (background=true, notify_on_complete=true)

### Exit: 0
### Complete saved output:
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
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default]

warning: function `timestamp_for_test` is never used
    --> src/zec/test_support.rs:1528:4
     |
1528 | fn timestamp_for_test(value: &str) -> Result<u64, ZecError> {
     |     ^^^^^^^^^^^^^^^^^^^^^

warning: fields `confirm` and `cancel` are never read
  --> src/native_ui.rs:121:5
   |
120 | struct ZecReviewControls {
   |        ----------------- fields in this struct
121 |     confirm: egui::Rect,
   |     ^^^^^^^^
122 |     cancel: egui::Rect,
   |     ^^^^^^^^

warning: `bitbook-wallet-broker` (lib) generated 3 warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.61s
```

### Compilation check
- Exit 0 ✓
- Warnings recorded without fixing (per handoff rules):
  1. `matches_review` is never used (src/native.rs:77) — dead code
  2. `timestamp_for_test` function is never used (src/zec/test_support.rs:1528) — dead code
  3. Fields `confirm` and `cancel` are never read (src/native_ui.rs:121–122) — dead code (these fields are read by the test suite but not in non-test lib builds; this is expected and was accepted by the reviewer)

## Final measured inventory (after stage 5)

- native_ui.rs: 321 lines, SHA-256 d132a164a6413165291abd0582abf3eca4cacd3f33ce6eb4ac2b467e8774d960 — matches original ✓
- All 26 frozen inventory paths: hashes and line counts match ✓
- git rev-parse HEAD: 6de87b53e53d3f8da4c9c261fdefa6c726cd2299
- git diff --cached --name-only: (empty — no staged changes)
- git status --short --untracked-files=all: 11 modified, 12 untracked (unchanged from before execution; no new unexpected dirty paths introduced by validation)

## Deviations disclosed

1. Preflight wc used .rs instead of .md for the retained-spend resume evidence in the original interrupted session, giving exit 1 despite all hashes matching; the reviewer verified the actual 560-line count. Not repeated.
2. The wait timeout was requested as 600 seconds and was clamped to 60 seconds by the environment on each process.wait call. All processes completed well within limits.
3. No process.log file was found from stage 1; the accepted saved 78612 output from the handoff was used without rerunning stage 1.
4. The `--exact` flag for single-test runs was passed as `-- --exact` (after the cargo-test argument separator) because cargo 1.98 rejects it as a direct cargo flag; this is the only correct way to pass test-runner args and does not alter the prescribed command semantics.
5. All warnings listed in stage 5 are pre-existing and accepted; no warnings were fixed per handoff rules.
