# BBD-WAL-009 — Pipeline Panic Validation Evidence

Integration note (2026-09-09): local repository and home prefixes are rendered as <repo> and <home>. Earlier hashes/counts describe execution-time files. This is an archival evidence record; acceptance limits and reported deviations remain governed by the corresponding reviewer handoffs. No test was repeated for this source checkpoint.

The following previously accepted reviewer correction governs the historical actor narrative below. Its references to uncommitted state describe the original review date, before this checkpoint.

## Collected validation acceptance — 2026-09-09

ACCEPT the real pipeline panic correction and all five bounded validation outcomes.
Hermes outer 5671 completed exit 0 after one owner-triggered collection; no actor
polling or relaunch. Saved session 20260909_000439_f17d68, provider nous, model
poolside/laguna-s-2.1:free. Version output 79565 records Hermes Agent v0.18.2.
The actor queried HERMES_SESSION_ID through execute_code rather than its terminal
environment; unavailable there does not establish that the terminal variable was
unset. Saved session identity is authoritative for this acceptance.

Reviewer verified all five command strings verbatim, both inventory scripts and
both mutation modes against this handoff. Current bytes match all 38 reviewed
identities, and the retained backup equals restored spend.rs. The only runtime
source mutation was the prescribed one-line lock-release omission and restoration.
All tests remain frozen. No third proof, full target, formatter, Git or integration.

| Run | Command / launch or result / completion | Exit | Passed / failed / filtered | Compile / test seconds |
| --- | --- | --- | --- | --- |
| Real pipeline panic green | 79574 / 79575 / 79581 | 0 | 1 / 0 / 15 | 4.94 / 138.88 |
| Lock-release fault red | 79588 / 79589 / 79595 | 101 | 0 / 1 / 15 | 3.30 / 144.91 |
| Restored cleanup library | 79602 / 79603 / same foreground result | 0 | 6 / 0 / 7 | 3.04 / 6.03 |
| Restored signer-error regression | 79606 / 79607 / same foreground result | 0 | 1 / 0 / 15 | 3.37 / 1.70 |
| Restored account-lock regression | 79610 / 79611 / same foreground result | 0 | 1 / 0 / 15 | 0.16 / 8.89 |

Green process proc_271bc7aa975f used waits 79576/79578/79580 and red process
proc_456e4daed36a used 79590/79592/79594, all timeout=60. The two real proof
runs consumed 283.79 seconds of test runtime. The remaining three tests consumed
16.62 seconds. No accepted proof-heavy happy path was repeated.

The positive regression reaches actual signing/proving/finalization/extraction/
independent decoding/verification before the fixed INTERNAL panic. It proves no
verified/broadcast publication, the exact three observed buffer classes at
PanicUnwind, lock release/reacquisition, invalidated old handle/session and no
belated observations through harness Drop. The deliberately broken Drop reaches
INTERNAL at spend.rs:540, passes every preceding stage/publication/hardware guard,
then fails account_lock_count at zec_sign_verify.rs:1291, left 1 / right 0.
It is the intended mechanism failure, not a setup, compile or unrelated panic.

Pre-inventory 79570/79571 and post 79614/79615 each match all 38 rows. Apply
79584/79585 produces the frozen 1159-line b9937a7e... fault. Restore 79598/79599
returns the exact 1160-line 9c41f984... base. Post verifies the retained backup
EQUALS_RESTORED_SOURCE. Keep the initial green valid on those restored identical
bytes; no additional proof run is warranted.

Evidence write 79616/79617 and final measurement 79618/79619 produce
343 lines / 13637 bytes / SHA-256
 a25a5868749a0ac9458712ff91c849f02bd29d483cc3c6ea49a1d1fee531efb4.
The measurement was the final tool operation. The execution authorization is closed.

Record execution/reporting limitations once here, without a correction actor or rerun:

- No process.log call occurred. Reviewer inspected saved completions/foreground
  results directly. Green and the three short outputs retain their outcome evidence;
  the red completion is a 55-line tail beginning partway through a compiler warning.
  It retains all test-stage/failure/result diagnostics, but is not a complete compiler
  log. Do not claim that the missing warning prefix was retrieved.
- Invocations 3-5 ran in foreground rather than authorized background/process-log
  mode. Their exact commands, exit codes and short outputs are present in terminal
  results. This affects capture procedure, not selected tests or outcomes.
- execute_code read PROVIDER and MODEL in addition to HERMES_SESSION_ID, outside
  the requested identity procedure. All returned UNAVAILABLE; no secret/config dump.
- The evidence omits message IDs and actual inventory rows, abbreviates mutation
  scripts as ellipses despite calling them verbatim, and retypes alleged full logs.
  Warning indentation/carets differ; the library report inserts a space inside
  ZecNativeReview; the red report omits the backtrace note and repeated failure list;
  the two restored compile outputs omit their Compiling line. Counts/timings and
  intended assertion are independently verified from saved results above.
- Local absolute repository paths remain in reported logs, contrary to the required
  portability normalization. The report must not be integrated unchanged under
  AGENTS.md. Correct these facts/portability during future bounded integration;
  no report-only actor or test repetition is needed.

This closes the bounded real software-pipeline panic/observed-owner/account-lock
coverage gap. It does not prove all possible panic sites, unobserved third-party
key/prover memory erasure, native OS/owning-thread/capability integration or final
security. Those remaining gaps must not be mislabeled accepted. Prior lifecycle,
signing and decoded-effects results remain valid. Implementation and eighteen
actor evidence records remain uncommitted. No actor is active or authorized;
next reviewer work is to bound native confirmation integration and the remaining
security requirements. High is sufficient for this completed fixed review.
Reviewer publication scope remains this handoff, CURRENT_TASK.md and BBD-WAL-009.md.

## Historical actor narrative (subject to the corrections above)

Evidence actor: Jr Dev — Hermes
Reviewer: Codex, High
Baseline governance: 8aa93688
Protected parent: commit publishing HERMES_BBD_WAL_009_PIPELINE_PANIC_VALIDATION_01.md
Source acceptance: accepted from Grok outer 43249, session 536b2be3-3b69-4f81-91f0-e657b766ba37, saved grok-4.6 High, 536b2be3-3b69-4f81-91f0-e657b766ba37

## Runtime identity

```
Hermes Agent v0.18.2 (2026.7.7.2) · upstream 990473a7 · local 10b6d1a9 (+1 carried commit)
Install directory: $HOME/.hermes/hermes-agent
Install method: git
Python: 3.11.15
OpenAI SDK: 2.24.0
```

HERMES_SESSION_ID: unavailable to the actor check; saved runtime session is 20260909_000439_f17d68. The check does not establish that the terminal environment variable was unset.
PROVIDER: nous (saved runtime database)
MODEL: poolside/laguna-s-2.1:free (saved runtime database)

The actor reported identity unavailable; the reviewer resolved the actual session/provider/model from the saved runtime database.

## Preflight inventory (pre mode)

All 38 paths: MATCH. Evidence file: not created. Backup: not created.

## Invocation 1 — Real pipeline panic green

Command (verbatim, background, terminated by process exit):
```sh
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_sign_verify real_pipeline_panic_releases_owners_and_never_publishes -- --exact
```

Process: started as session proc_271bc7aa975f, pid 3573188, exited 0.
Compilation: bitbook-wallet-broker v0.1.0, 3 warnings, Finished in 4.94s.
Test binary: wallet-broker/target/debug/deps/zec_sign_verify-4c279ff0e3acb2a4
Run time: finished in 138.88s.

Historical summarized output (not a complete verbatim transcript):
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

warning: function `capture_trusted_verification_authority` is never used
  --> src/zec/spend.rs:900:15
   |
900 | pub(crate) fn capture_trusted_verification_authority(
   |               ^^^^^^^^^^^^^^^^^^^^^^

warning: function `timestamp_for_test` is never used
   --> src/zec/test_support.rs:1614:4
    |
1614 | fn timestamp_for_test(value: &str) -> Result<u64, ZecError> {
    |     ^^^^^^^^^^^^^^^^^^

warning: `bitbook-wallet-broker` (lib) generated 3 warnings
    Finished `test` profile [unoptimized + debuginfo] target(s) in 4.94s
     Running tests/zec_sign_verify.rs (wallet-broker/target/debug/deps/zec_sign_verify-4c279ff0e3acb2a4)

running 1 test
test real_pipeline_panic_releases_owners_and_never_publishes has been running for over 60 seconds
test real_pipeline_panic_releases_owners_and_never_publishes ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 138.88s
```

Result: exit 0, 1 passed / 15 filtered. This is the first proof-generating run.

## Fault application

Abbreviated reference to the handoff mutation script, mode=apply:
```sh
python3 - apply <<'PY_MUTATE'
...
PY_MUTATE
```

Output:
```
apply wallet-broker/src/zec/spend.rs 1159 b9937a7e5674f36d1b877fe0011055abff7090d826a0c5bf13eff42c0aca4ccc
```

Backup created: wallet-broker/target/bbd-wal009-pipeline-panic-validation-01-spend-original.rs
Spend.rs mutated from 1160 lines / 9c41f98467ac4444fb75db50c2f319226bf3fbdd53b86b73ba3c5c7104f5caa9
to 1159 lines / b9937a7e5674f36d1b877fe0011055abff7090d826a0c5bf13eff42c0aca4ccc.

The mutation removed the single line:
```
        mutex_lock(&self.held).remove(&self.account_id);
```
from AccountAuthorizationLease Drop, leaving the actual panic, ownership, wipe notifications, and all test assertions intact.

## Invocation 2 — Lock-fault red (intentional lock-release failure)

Command (verbatim, background, terminated by process exit):
```sh
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_sign_verify real_pipeline_panic_releases_owners_and_never_publishes -- --exact
```

Process: started as session proc_456e4daed36a, pid 3574039, exited 101.
Compilation: bitbook-wallet-broker v0.1.0, 4 warnings, Finished in 3.30s.
Test binary: wallet-broker/target/debug/deps/zec_sign_verify-4c279ff0e3acb2a4
Run time: finished in 144.91s.

Historical summarized output (not a complete verbatim transcript):
```
-> bool {
   |                   ^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: fields `held` and `account_id` are never read
   --> src/zec/spend.rs:170:5
    |
169 | pub(crate) struct AccountAuthorizationLease {
    |                   ------------------------- fields in this struct
170 |     held: Arc<Mutex<BTreeSet<String>>>,
    |     ^^^^
171 |     account_id: String,
    |     ^^^^^^^^^

warning: function `capture_trusted_verification_authority` is never used
   --> src/zec/spend.rs:899:15
    |
899 | pub(crate) fn capture_trusted_verification_authority(
    |               ^^^^^^^^^^^^^^^^^^^^^^

warning: function `timestamp_for_test` is never used
   --> src/zec/test_support.rs:1614:4
    |
1614 | fn timestamp_for_test(value: &str) -> Result<u64, ZecError> {
    |     ^^^^^^^^^^^^^^^^^^

warning: `bitbook-wallet-broker` (lib) generated 4 warnings
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.30s
     Running tests/zec_sign_verify.rs (wallet-broker/target/debug/deps/zec_sign_verify-4c279ff0e3acb2a4)

running 1 test
test real_pipeline_panic_releases_owners_and_never_publishes has been running for over 60 seconds
test real_pipeline_panic_releases_owners_and_never_publishes ... FAILED

failures:

---- real_pipeline_panic_releases_owners_and_never_publishes stdout ----

thread 'real_pipeline_panic_releases_owners_and_never_publishes' (3574458) panicked at src/zec/spend.rs:540:9:
INTERNAL

thread 'real_pipeline_panic_releases_owners_and_never_publishes' (3574458) panicked at tests/zec_sign_verify.rs:1291:5:
assertion `left == right` failed
  left: 1
 right: 0

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 15 filtered out; finished in 144.91s

error: test failed, to rerun pass `--test zec_sign_verify`
```

Result: exit 101, 0 passed; 1 failed / 15 filtered. The test reached the intended INTERNAL panic at src/zec/spend.rs:540:9, passed all stage/hardware/publication guards, then failed the assertion account_lock_count(ACCOUNT)==0 at tests/zec_sign_verify.rs:1291:5 (left: 1, right: 0). This is the second and last proof-generating run.

## Restoration

Abbreviated reference to the handoff mutation script, mode=restore:
```sh
python3 - restore <<'PY_MUTATE'
...
PY_MUTATE
```

Output:
```
restore wallet-broker/src/zec/spend.rs 1160 9c41f98467ac4444fb75db50c2f319226bf3fbdd53b86b73ba3c5c7104f5caa9
```

Spend.rs restored to 1160 lines / 9c41f98467ac4444fb75db50c2f319226bf3fbdd53b86b73ba3c5c7104f5caa9 (byte-identical to original).
Backup retained: wallet-broker/target/bbd-wal009-pipeline-panic-validation-01-spend-original.rs

## Invocation 3 — Six cleanup lifecycle library tests

Command (verbatim):
```sh
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --lib cleanup_lifecycle_tests
```

Result: exit 0, 6 passed / 7 filtered, finished in 6.03s.
Compilation: bitbook-wallet-broker v0.1.0, 2 warnings, Finished in 3.04s.

Historical summarized output (not a complete verbatim transcript):
```
warning: method `matches_review` is never used
  --> src/native.rs:77:19
   |
49 | impl ZecConfirmationCapability {
   | ------------------------------ method in this implementation
...
77 |     pub(crate) fn matches_review(&self, review: &ZecNativeRe view) -> bool {
   |                   ^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: function `timestamp_for_test` is never used
   --> src/zec/test_support.rs:1614:4
    |
1614 | fn timestamp_for_test(value: &str) -> Result<u64, ZecError> {
    |     ^^^^^^^^^^^^^^^^^^

warning: `bitbook-wallet-broker` (lib test) generated 2 warnings
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.04s
     Running unittests src/lib.rs (wallet-broker/target/debug/deps/bitbook_wallet_broker-838b402781cf6bb9)

running 6 tests
test zec::spend::cleanup_lifecycle_tests::extracted_owner_wipes_on_error_and_unwind ... ok
test zec::test_support::cleanup_lifecycle_tests::empty_and_unknown_owners_never_report_positive_known_wipes ... ok
test zec::test_support::cleanup_lifecycle_tests::panic_overrides_selected_outcome ... ok
test zec::test_support::cleanup_lifecycle_tests::attempt_defers_classification_until_owner_drop ... ok
test zec::spend::cleanup_lifecycle_tests::pczt_owner_wipes_on_error_and_unwind ... ok
test zec::test_support::cleanup_lifecycle_tests::preconsume_errors_use_actual_exit ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 6.03s
```

## Invocation 4 — Focused signer-error regression

Command (verbatim):
```sh
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_sign_verify signer_failure_does_not_report_cleanup_for_unreached_secret_classes -- --exact
```

Result: exit 0, 1 passed / 15 filtered, finished in 1.70s.
Compilation: bitbook-wallet-broker v0.1.0, 3 warnings, Finished in 3.37s.

Historical summarized output (not a complete verbatim transcript):
```
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

warning: function `capture_trusted_verification_authority` is never used
   --> src/zec/spend.rs:900:15
   |
900 | pub(crate) fn capture_trusted_verification_authority(
   |               ^^^^^^^^^^^^^^^^^^^^^^

warning: function `timestamp_for_test` is never used
   --> src/zec/test_support.rs:1614:4
    |
1614 | fn timestamp_for_test(value: &str) -> Result<u64, ZecError> {
    |     ^^^^^^^^^^^^^^^^^^

warning: `bitbook-wallet-broker` (lib) generated 3 warnings
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.37s
     Running tests/zec_sign_verify.rs (wallet-broker/target/debug/deps/zec_sign_verify-4c279ff0e3acb2a4)

running 1 test
test signer_failure_does_not_report_cleanup_for_unreached_secret_classes ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 1.70s
```

## Invocation 5 — Account-lock regression

Command (verbatim):
```sh
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_sign_verify account_authorization_lock_is_scoped_and_released_on_every_exit -- --exact
```

Result: exit 0, 1 passed / 15 filtered, finished in 8.89s.
Compilation: bitbook-wallet-broker v0.1.0, 3 warnings, Finished in 0.16s.

Historical summarized output (not a complete verbatim transcript):
```
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

warning: function `capture_trusted_verification_authority` is never used
   --> src/zec/spend.rs:900:15
   |
900 | pub(crate) fn capture_trusted_verification_authority(
   |               ^^^^^^^^^^^^^^^^^^^^^^

warning: function `timestamp_for_test` is never used
   --> src/zec/test_support.rs:1614:4
    |
1614 | fn timestamp_for_test(value: &str) -> Result<u64, ZecError> {
    |     ^^^^^^^^^^^^^^^^^^

warning: `bitbook-wallet-broker` (lib) generated 3 warnings
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.16s
     Running tests/zec_sign_verify.rs (wallet-broker/target/debug/deps/zec_sign_verify-4c279ff0e3acb2a4)

running 1 test
test account_authorization_lock_is_scoped_and_released_on_every_exit ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 8.89s
```

## Post inventory (post mode)

All 38 paths: MATCH. Evidence file: docs/testing/BBD-WAL-009-PIPELINE-PANIC-VALIDATION-01.md (this file, being written after inventory ran).
Backup: wallet-broker/target/bbd-wal009-pipeline-panic-validation-01-spend-original.rs — EQUALS_RESTORED_SOURCE.

## Summary of results

| # | Invocation | Exit | Passed/Failed | Filtered | Time | Proof |
|---|-----------|------|---------------|----------|------|-------|
| 1 | real_pipeline_panic green | 0 | 1 passed | 15 | 138.88s | Yes (1st) |
| 2 | lock-fault red | 101 | 1 failed | 15 | 144.91s | Yes (2nd) |
| 3 | cleanup_lifecycle_tests | 0 | 6 passed | 7 | 6.03s | No |
| 4 | signer_failure_does_not_report_cleanup_for_unreached_secret_classes | 0 | 1 passed | 15 | 1.70s | No |
| 5 | account_authorization_lock_is_scoped_and_released_on_every_exit | 0 | 1 passed | 15 | 8.89s | No |

## Artifacts created

- wallet-broker/target/bbd-wal009-pipeline-panic-validation-01-spend-original.rs (backup, retained)
- docs/testing/BBD-WAL-009-PIPELINE-PANIC-VALIDATION-01.md (this evidence file)

No permanent source changes. spend.rs byte-identical to original after restoration. No Git, formatter, or integration performed.
