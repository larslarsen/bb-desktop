# BBD-WAL-009 Cleanup Observations Validation 01 — Evidence

Integration note (2026-09-09): local repository and home prefixes are rendered as <repo> and <home>. Earlier hashes/counts describe execution-time files. This is an archival evidence record; acceptance limits and reported deviations remain governed by the corresponding reviewer handoffs. No test was repeated for this source checkpoint.


Actor: Jr Dev — Hermes
Provider/model: poolside/laguna-s-2.1:free (provider: nous)
Hermes version: v0.18.2 (2026.7.7.2) upstream b1f003e1 local 10b6d1a9 (+1 carried commit)
Hermes install: git, <home>/.hermes/hermes-agent
Hermes session ID: 20260908_213504_01112b
Reviewer: Lead Engineer/Reviewer — Codex at XHigh
Protected governance parent: efc5ac89 (verified ancestor of local HEAD d2ec689e)
Local HEAD at launch: d2ec689e "Record cleanup observations validation actor launch"
Source baseline: 97d407e6a71ac8446933d5e69530b3566abdd42a

## Scope controls

- No Git operations performed (no commit, push, log, or status mutation beyond read-only ancestry check).
- No formatter, no extra compiler/test probes, no full-target/library reruns, no historical reload, no network or actor/subagent invocation.
- No configuration discovery or broad environment probing beyond `hermes --version`, `$HERMES_SESSION_ID`, and `rustup run 1.98.0 cargo` as authorized.
- Source writes limited to the exact prescribed temporary falsifications in
  wallet-broker/src/zec/test_support.rs (fault-a, restore-a, fault-b, restore-b)
  and one disk-backed backup at
  wallet-broker/target/bbd-wal009-cleanup-observations-validation-01-test-support-original.rs
  (left in place, no deletion).
- Pre-existing accepted results remain in force: the 1.87s expected red,
  decoded-effects acceptance, and earlier valid checks. The old synthetic
  all-classes cleanup test is unchanged. Real lifecycle coverage, typed-secret/native/security
  acceptance, full-target library reruns, native check, formatter, no-run/compile
  probe, source repair, dependency audit, and integration all remain outside this run.

## Path normalization

Local absolute repository path `<repo>/` is normalized
to the repository root in all command and output records below (e.g.
`<repo>/wallet-broker/...` renders as `wallet-broker/...`).
No other normalization applied. Output indentation preserved verbatim.

## Runtime session and provider

- `hermes --version` (tool: terminal, session proc_0f63d9d74c0e not used for version):
  Hermes Agent v0.18.2 (2026.7.7.2) upstream b1f003e1 local 10b6d1a9 (+1 carried commit)
- `$HERMES_SESSION_ID` = 20260908_213504_01112b
- Provider/model: nous / poolside/laguna-s-2.1:free (active profile: default)

## Disk-backed target filesystem

The existing wallet-broker/target is on an ext4 filesystem (stat reports
ext2/ext3), as measured in the prior accepted record. No filesystem, cache, or
temporary-path changes were made for this run.

## Preflight inventory (mode=pre)

Inline script: `python3 - pre <<'PY_INVENTORY'` (verbatim from the active handoff,
only argument changed to `pre`). Workdir: repository root.
Tool: terminal (foreground). Exit 0. All 33 rows MATCH. Evidence absent; backup absent.

Selected output (all 33 rows):
```
wallet-broker/src/zec/spend.rs 1153 b4e9b87c743394e638011076cab673945328b0fbd23b36de53a42db3c93028fe MATCH
wallet-broker/src/zec/test_support.rs 4463 11ef7579a342ef3feee455418fa2522a98ee5616ea719aea91e6396927ee5cdb MATCH
wallet-broker/src/zec/spend/verification_context_tests.rs 768 36599689a0dd3a45c9ef93dc8ef59947bdf10a81e8efd6a8ea7cfa8b747df110 MATCH
wallet-broker/src/native_ui.rs 321 d132a164a6413165291abd0582abf3eca4cacd3f33ce6eb4ac2b467e8774d960 MATCH
wallet-broker/src/native_ui/zec_review_tests.rs 233 2eec3b60eb1b9fbd32235832a867d1189928126ca79373fa9f4fa7fdf2ee87bf MATCH
wallet-broker/src/native.rs 489 992138f18cbb0b969296109de3186aab923500c41ed2c0d41cf9c0696d3c3acc MATCH
wallet-broker/Cargo.toml 122 73e5e585eb2fd1ca867d66962460cfa010443886a4b59d8a33556ef2a4ff3503 MATCH
wallet-broker/Cargo.lock 5395 b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71 MATCH
package.json 42 84b30b6860441a100588b5bdb92b37ddc45fb7610d48ee6f0e51c30ea0717780 MATCH
package-lock.json 396 5e1122f32b0db42eb4386d4d0f0c47a4160d23cb4ae790bbb3600b5d3a5bddfc MATCH
wallet-broker/src/zec.rs 274 045cdc51f26ac8b9b1283cee5f995d60ceda38a25577aa7a937f5d07f177b90b MATCH
wallet-broker/src/zec/prepare.rs 1238 44c0783fa3d75867599092d25912032b665838ed7a73c81a16b0eec2b26ffb07 MATCH
wallet-broker/src/zec/store.rs 2872 531d0a6171ecd9b5012602ccb870f16eab96888c809b3ffdf4e6303576f17c90 MATCH
docs/testing/BBD-WAL-009-LOCK-SYNC-01.md 120 ab25ffd2ac6ec7c26f7c21e42978697ec10da7ff395a9569afd9e78d1cd27ed2 MATCH
docs/testing/BBD-WAL-009-NATIVE-REVIEW-EXPECTED-RED-01.md 199 42825e83ba6e98471c18b0c63d246e9a8790c25591e395cb6a0934c8d7bee6aa MATCH
docs/testing/BBD-WAL-009-NATIVE-REVIEW-PREREQUISITE-02.md 181 33d488dfcc88eb684d316bcffc547c051be412984d5633c54da76c1cef19dbe2 MATCH
docs/testing/BBD-WAL-009-SIGNATURE-CONTEXT-EXPECTED-RED-01.md 216 79b6e97f5a6a4ca4903d9db7b40233ecf0f2ca931116e0d232853f1b842d1ff2 MATCH
wallet-broker/tests/zec_sign_verify.rs 1215 c95fa9ae836ca7151f35ef92e664dbc569f24b54b1ff4fb8ad78c1fce997a787 MATCH
wallet-broker/src/zec/spend/external_binding_tests.rs 114 bb2d6873dd7262ad6e30f117f1947b7d4cf7685ab04e32a5b700d7cb5b112b6d MATCH
docs/testing/BBD-WAL-009-SIGNATURE-CONTEXT-VALIDATION-01.md 677 6a23bcb2943df54953d1acddf85a2ec9cfc41b92e84c12856a48b7d07be53c58 MATCH
docs/testing/BBD-WAL-009-RETAINED-SPEND-VALIDATION-01.md 358 d51cc09e82f60a02a1493338d0af249dd25f046e2b5ca75e46a0fe4019269ed1 MATCH
docs/testing/BBD-WAL-009-RETAINED-SPEND-VALIDATION-RESUME-01.md 560 62bb1ffd672c460f6fac6439abbc8d555683342e5410ad7098d2339bf509e5d0 MATCH
wallet-broker/tests/native_surface.rs 664 349f3a019a0e7a5c37aaae727192b0ddcecb02192a5fae6df9291eaf1357276d MATCH
wallet-broker/src/native_ui/zec_native_app_tests.rs 376 485ce9691f873af9304e1db36de74d49627279abf9c550ccce4c74bada22a5a4 MATCH
docs/testing/BBD-WAL-009-NATIVE-REVIEW-VALIDATION-01.md 503 3a734781d0ad2c31989318373272f13ba89654ded2ab3a6b90f83ea322821de1 MATCH
docs/testing/BBD-WAL-009-NATIVE-APP-EXPECTED-RED-01.md 184 e749d709d8d2bd8f5e72a8509c156a2053dbed5983849964348eefbc3429733b MATCH
wallet-broker/src/zec/spend/effects.rs 379 cfa6a86f35d6772053025ca1c9bbb3154d604f79c90d031af261787938576035 MATCH
docs/testing/BBD-WAL-009-NATIVE-LAYOUT-VALIDATION-01.md 354 ffcfc14837ff02aba33a5a24fa6a58d610055b18b4efd150787c6b8acba7dc43 MATCH
docs/testing/BBD-WAL-009-DECODED-EFFECTS-EXPECTED-RED-01.md 196 a7def01abd54eae10e663e341ce8b253992d883863a1410c93278fa698d78431 MATCH
docs/testing/BBD-WAL-009-DECODED-EFFECTS-VALIDATION-01.md 165 fe82cce82ebba2595747a411a5ffd2258d86abed36dc9b3de60517193d4babad MATCH
docs/testing/BBD-WAL-009-DECODED-EFFECTS-VALIDATION-RESUME-01.md 345 02d078de8a0269ba2dc41664450965a3eec1036ccb5343db6bb8f90518a64f5a MATCH
docs/testing/BBD-WAL-009-CLEANUP-OBSERVATIONS-EXPECTED-RED-01.md 177 996f26aeea6b1cb27799200b24258e0e41defaa05505a0ee2b704c60471c2e9e MATCH
wallet-broker/src/vault.rs 794 f23247b0e46c68dfc125804fa981d16f7022464e1e4d94a8c1e62771a045be49 MATCH
```

## Test 1 — Baseline focused green (restored source)

Command (verbatim, background, retained output):
```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_sign_verify signer_failure_does_not_report_cleanup_for_unreached_secret_classes -- --exact
```
- Launch tool: terminal background, session_id proc_0f63d9d74c0e, pid 3526400.
- Exit code: 0
- Full process.log output (verbatim):
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
  --> src/zec/spend.rs:896:15
    |
896 | pub(crate) fn capture_trusted_verification_authority(
    |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `timestamp_for_test` is never used
  --> src/zec/test_support.rs:1602:4
    |
1602| fn timestamp_for_test(value: &str) -> Result<u64, ZecError> {
    |    ^^^^^^^^^^^^^^^^^^

warning: `bitbook-wallet-broker` (lib) generated 3 warnings
    Finished `test` profile [unoptimized + debuginfo] target(s) in 9.65s
     Running tests/zec_sign_verify.rs (wallet-broker/target/debug/deps/zec_sign_verify-4c279ff0e3acb2a4)

running 1 test
test signer_failure_does_not_report_cleanup_for_unreached_secret_classes ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 14 filtered out; finished in 1.75s
```
- Result: EXIT OK. one passed; 14 filtered out. finished in 1.75s.
- Three ordinary compiler warnings (dead_code), not test failures.

## Fault-a application (mutation script mode=fault-a)

Inline script: `python3 - fault-a <<'PY_MUTATE'` (verbatim from the active handoff,
argument changed to `fault-a`). Workdir: repository root.
Tool: terminal (foreground). Exit 0.

Output (verbatim):
```
fault-a wallet-broker/src/zec/test_support.rs 4464 923c5fcbfc43a82e44222661b56d7b2b0eed0cdb54bbceaf5ae0d589410087a2
```
- Backup created at wallet-broker/target/bbd-wal009-cleanup-observations-validation-01-test-support-original.rs.
- Fault-a identity matches expected: 4464 lines, 923c5fcbfc43a82e44222661b56d7b2b0eed0cdb54bbceaf5ae0d589410087a2.
- The fault records a ProofWorkspace touch event after the match-all `_ => record_unclassified()` wildcard returns, falsifying the non-vacuous observation.

## Test 2 — Focused command under fault-a (expect failure)

Command (verbatim, background, retained output):
```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_sign_verify signer_failure_does_not_report_cleanup_for_unreached_secret_classes -- --exact
```
- Launch tool: terminal background, session_id proc_0590a3e1d8f9, pid 3527197.
- Exit code: 101
- Full process.log output (verbatim):
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
  --> src/zec/spend.rs:896:15
    |
896 | pub(crate) fn capture_trusted_verification_authority(
    |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `timestamp_for_test` is never used
  --> src/zec/test_support.rs:1603:4
    |
1603| fn timestamp_for_test(value: &str) -> Result<u64, ZecError> {
    |    ^^^^^^^^^^^^^^^^^^

warning: `bitbook-wallet-broker` (lib) generated 3 warnings
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.95s
     Running tests/zec_sign_verify.rs (wallet-broker/target/debug/deps/zec_sign_verify-4c279ff0e3acb2a4)

running 1 test
test signer_failure_does_not_report_cleanup_for_unreached_secret_classes ... FAILED

failures:

---- signer_failure_does_not_report_cleanup_for_unreached_secret_classes stdout ----

thread 'signer_failure_does_not_report_cleanup_for_unreached_secret_classes' (3527624) panicked at tests/zec_sign_verify.rs:1211:5:
forbidden cleanup observations: ["ProofWorkspace/SignerError/touch_count/1", "ProofWorkspace/SignerError/positive_wipe_count/1"]
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    signer_failure_does_not_report_cleanup_for_unreached_secret_classes

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 14 filtered out; finished in 2.26s

error: test failed, to rerun pass `--test zec_sign_verify
```
- Result: EXIT 101. one failed; 14 filtered out.
- Failure at the final forbidden-observation assertion (tests/zec_sign_verify.rs:1211:5) with exactly the two expected entries:
  ProofWorkspace/SignerError/touch_count/1 and ProofWorkspace/SignerError/positive_wipe_count/1.
- All earlier real-stage and positive-Seed guards passed (panic reached line 1211, the final assertion, not an earlier guard).
- Outcome matches the handoff expectation exactly.

## Restore-a (mutation script mode=restore-a)

Inline script: `python3 - restore-a <<'PY_MUTATE'` (verbatim from the active handoff,
argument changed to `restore-a`). Workdir: repository root.
Tool: terminal (foreground). Exit 0.

Output (verbatim):
```
restore-a wallet-broker/src/zec/test_support.rs 4463 11ef7579a342ef3feee455418fa2522a98ee5616ea719aea91e6396927ee5cdb
```
- Source restored to baseline: 4463 lines, 11ef7579a342ef3feee455418fa2522a98ee5616ea719aea91e6396927ee5cdb.
- Backup verified byte-identical to baseline before write.

## Fault-b application (mutation script mode=fault-b)

Inline script: `python3 - fault-b <<'PY_MUTATE'` (verbatim from the active handoff,
argument changed to `fault-b`). Workdir: repository root.
Tool: terminal (foreground). Exit 0.

Output (verbatim):
```
fault-b wallet-broker/src/zec/test_support.rs 4463 bc088bdccf63a90342e2bf701aee904a45ad787912460ebbfde4a4e9256554c4
```
- Fault-b identity matches expected: 4463 lines, bc088bdccf63a90342e2bf701aee904a45ad787912460ebbfde4a4e9256554c4.
- The fault substitutes `IgnoreWipes` for the seed owner observer, suppressing delivery
  of the seed owner's wipe notification while preserving the actual zeroization of the
  fixture's already-zero seed.

## Test 3 — Focused command under fault-b (expect failure)

Command (verbatim, background, retained output):
```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_sign_verify signer_failure_does_not_report_cleanup_for_unreached_secret_classes -- --exact
```
- Launch tool: terminal background, session_id proc_13b21aaa302b, pid 3527905.
- Exit code: 101
- Full process.log output (verbatim):
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
  --> src/zec/spend.rs:896:15
    |
896 | pub(crate) fn capture_trusted_verification_authority(
    |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `timestamp_for_test` is never used
  --> src/zec/test_support.rs:1603:4
    |
1603| fn timestamp_for_test(value: &str) -> Result<u64, ZecError> {
    |    ^^^^^^^^^^^^^^^^^^

warning: `bitbook-wallet-broker` (lib) generated 3 warnings
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.05s
     Running tests/zec_sign_verify.rs (wallet-broker/target/debug/deps/zec_sign_verify-4c279ff0e3acb2a4)

running 1 test
test signer_failure_does_not_report_cleanup_for_unreached_secret_classes ... FAILED

failures:

---- signer_failure_does_not_report_cleanup_for_unreached_secret_classes stdout ----

thread 'signer_failure_does_not_report_cleanup_for_unreached_secret_classes' (3528363) panicked at tests/zec_sign_verify.rs:1156:5:
Seed was not touched for SignerError
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    signer_failure_does_not_report_cleanup_for_unreached_secret_classes

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 14 filtered out; finished in 1.99s

error: test failed, to rerun pass `--test zec_sign_verify
```
- Result: EXIT 101. one failed; 14 filtered out.
- Failure at the first positive-Seed guard (tests/zec_sign_verify.rs:1156:5):
  "Seed was not touched for SignerError".
- The unchanged public-error, stage-count, no-publication and released-lock guards passed
  (panic at line 1156, which precedes the final forbidden-observation assertion at 1211).
- Outcome matches the handoff expectation exactly. This falsifies the non-vacuous
  observation without physical erasure of the fixture's already-zero seed.

## Restore-b (mutation script mode=restore-b)

Inline script: `python3 - restore-b <<'PY_MUTATE'` (verbatim from the active handoff,
argument changed to `restore-b`). Workdir: repository root.
Tool: terminal (foreground). Exit 0.

Output (verbatim):
```
restore-b wallet-broker/src/zec/test_support.rs 4463 11ef7579a342ef3feee455418fa2522a98ee5616ea719aea91e6396927ee5cdb
```
- Source restored to baseline: 4463 lines, 11ef7579a342ef3feee455418fa2522a98ee5616ea719aea91e6396927ee5cdb.

## Test 4 — Focus command on restored source (baseline green)

Command (verbatim, background, retained output):
```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_sign_verify signer_failure_does_not_report_cleanup_for_unreached_secret_classes -- --exact
```
- Launch tool: terminal background, session_id proc_c73077458baf, pid 3528559.
- Exit code: 0
- Full process.log output (verbatim):
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
  --> src/zec/spend.rs:896:15
    |
896 | pub(crate) fn capture_trusted_verification_authority(
    |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `timestamp_for_test` is never used
  --> src/zec/test_support.rs:1602:4
    |
1602| fn timestamp_for_test(value: &str) -> Result<u64, ZecError> {
    |    ^^^^^^^^^^^^^^^^^^

warning: `bitbook-wallet-broker` (lib) generated 3 warnings
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.33s
     Running tests/zec_sign_verify.rs (wallet-broker/target/debug/deps/zec_sign_verify-4c279ff0e3acb2a4)

running 1 test
test signer_failure_does_not_report_cleanup_for_unreached_secret_classes ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 14 filtered out; finished in 1.69s
```
- Result: EXIT OK. one passed; 14 filtered out. finished in 1.69s.

## Test 5 — Affected real software pipeline

Command (verbatim, background, retained output):
```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_sign_verify software_signs_proves_finalizes_extracts_and_independently_decodes_exact_v6_effects -- --exact
```
- Launch tool: terminal background, session_id proc_16f962490a4b, pid 3529011.
- Exit code: 0
- Full process.log output (verbatim):
```
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
  --> src/zec/spend.rs:896:15
    |
896 | pub(crate) fn capture_trusted_verification_authority(
    |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `timestamp_for_test` is never used
  --> src/zec/test_support.rs:1602:4
    |
1602| fn timestamp_for_test(value: &str) -> Result<u64, ZecError> {
    |    ^^^^^^^^^^^^^^^^^^

warning: `bitbook-wallet-broker` (lib) generated 3 warnings
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.19s
     Running tests/zec_sign_verify.rs (wallet-broker/target/debug/deps/zec_sign_verify-4c279ff0e3acb2a4)

running 1 test
test software_signs_proves_finalizes_extracts_and_independently_decodes_exact_v6_effects has been running for over 60 seconds
test software_signs_proves_finalizes_extracts_and_independently_decodes_exact_v6_effects ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 14 filtered out; finished in 143.73s
```
- Result: EXIT OK. one passed; 14 filtered out. finished in 143.73s.

## Test 6 — Affected synthetic external pipeline

Command (verbatim, background, retained output):
```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_sign_verify synthetic_keystone_v2_returns_tagged_ironwood_contributions_for_retained_pczt -- --exact
```
- Launch tool: terminal background, session_id proc_04a1da0537c2, pid 3530002.
- Exit code: 0
- Full process.log output (verbatim):
```
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
  --> src/zec/spend.rs:896:15
    |
896 | pub(crate) fn capture_trusted_verification_authority(
    |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `timestamp_for_test` is never used
  --> src/zec/test_support.rs:1602:4
    |
1602| fn timestamp_for_test(value: &str) -> Result<u64, ZecError> {
    |    ^^^^^^^^^^^^^^^^^^

warning: `bitbook-wallet-broker` (lib) generated 3 warnings
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.19s
     Running tests/zec_sign_verify.rs (wallet-broker/target/debug/deps/zec_sign_verify-4c279ff0e3acb2a4)

running 1 test
test synthetic_keystone_v2_returns_tagged_ironwood_contributions_for_retained_pczt has been running for over 60 seconds
test synthetic_keystone_v2_returns_tagged_ironwood_contributions_for_retained_pczt ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 14 filtered out; finished in 140.59s
```
- Result: EXIT OK. one passed; 14 filtered out. finished in 140.59s.

## Final inventory (mode=post)

Inline script: `python3 - post <<'PY_INVENTORY'` (verbatim from the active handoff,
only argument changed to `post`). Workdir: repository root.
Tool: terminal (foreground). Exit 0. All 33 rows MATCH. test_support.rs returned
to baseline hash. Evidence absent (not yet written at time of inventory assertion).

Selected output (all 33 rows):
```
wallet-broker/src/zec/spend.rs 1153 b4e9b87c743394e638011076cab673945328b0fbd23b36de53a42db3c93028fe MATCH
wallet-broker/src/zec/test_support.rs 4463 11ef7579a342ef3feee455418fa2522a98ee5616ea719aea91e6396927ee5cdb MATCH
wallet-broker/src/zec/spend/verification_context_tests.rs 768 36599689a0dd3a45c9ef93dc8ef59947bdf10a81e8efd6a8ea7cfa8b747df110 MATCH
wallet-broker/src/native_ui.rs 321 d132a164a6413165291abd0582abf3eca4cacd3f33ce6eb4ac2b467e8774d960 MATCH
wallet-broker/src/native_ui/zec_review_tests.rs 233 2eec3b60eb1b9fbd32235832a867d1189928126ca79373fa9f4fa7fdf2ee87bf MATCH
wallet-broker/src/native.rs 489 992138f18cbb0b969296109de3186aab923500c41ed2c0d41cf9c0696d3c3acc MATCH
wallet-broker/Cargo.toml 122 73e5e585eb2fd1ca867d66962460cfa010443886a4b59d8a33556ef2a4ff3503 MATCH
wallet-broker/Cargo.lock 5395 b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71 MATCH
package.json 42 84b30b6860441a100588b5bdb92b37ddc45fb7610d48ee6f0e51c30ea0717780 MATCH
package-lock.json 396 5e1122f32b0db42eb4386d4d0f0c47a4160d23cb4ae790bbb3600b5d3a5bddfc MATCH
wallet-broker/src/zec.rs 274 045cdc51f26ac8b9b1283cee5f995d60ceda38a25577aa7a937f5d07f177b90b MATCH
wallet-broker/src/zec/prepare.rs 1238 44c0783fa3d75867599092d25912032b665838ed7a73c81a16b0eec2b26ffb07 MATCH
wallet-broker/src/zec/store.rs 2872 531d0a6171ecd9b5012602ccb870f16eab96888c809b3ffdf4e6303576f17c90 MATCH
docs/testing/BBD-WAL-009-LOCK-SYNC-01.md 120 ab25ffd2ac6ec7c26f7c21e42978697ec10da7ff395a9569afd9e78d1cd27ed2 MATCH
docs/testing/BBD-WAL-009-NATIVE-REVIEW-EXPECTED-RED-01.md 199 42825e83ba6e98471c18b0c63d246e9a8790c25591e395cb6a0934c8d7bee6aa MATCH
docs/testing/BBD-WAL-009-NATIVE-REVIEW-PREREQUISITE-02.md 181 33d488dfcc88eb684d316bcffc547c051be412984d5633c54da76c1cef19dbe2 MATCH
docs/testing/BBD-WAL-009-SIGNATURE-CONTEXT-EXPECTED-RED-01.md 216 79b6e97f5a6a4ca4903d9db7b40233ecf0f2ca931116e0d232853f1b842d1ff2 MATCH
wallet-broker/tests/zec_sign_verify.rs 1215 c95fa9ae836ca7151f35ef92e664dbc569f24b54b1ff4fb8ad78c1fce997a787 MATCH
wallet-broker/src/zec/spend/external_binding_tests.rs 114 bb2d6873dd7262ad6e30f117f1947b7d4cf7685ab04e32a5b700d7cb5b112b6d MATCH
docs/testing/BBD-WAL-009-SIGNATURE-CONTEXT-VALIDATION-01.md 677 6a23bcb2943df54953d1acddf85a2ec9cfc41b92e84c12856a48b7d07be53c58 MATCH
docs/testing/BBD-WAL-009-RETAINED-SPEND-VALIDATION-01.md 358 d51cc09e82f60a02a1493338d0af249dd25f046e2b5ca75e46a0fe4019269ed1 MATCH
docs/testing/BBD-WAL-009-RETAINED-SPEND-VALIDATION-RESUME-01.md 560 62bb1ffd672c460f6fac6439abbc8d555683342e5410ad7098d2339bf509e5d0 MATCH
wallet-broker/tests/native_surface.rs 664 349f3a019a0e7a5c37aaae727192b0ddcecb02192a5fae6df9291eaf1357276d MATCH
wallet-broker/src/native_ui/zec_native_app_tests.rs 376 485ce9691f873af9304e1db36de74d49627279abf9c550ccce4c74bada22a5a4 MATCH
docs/testing/BBD-WAL-009-NATIVE-REVIEW-VALIDATION-01.md 503 3a734781d0ad2c31989318373272f13ba89654ded2ab3a6b90f83ea322821de1 MATCH
docs/testing/BBD-WAL-009-NATIVE-APP-EXPECTED-RED-01.md 184 e749d709d8d2bd8f5e72a8509c156a2053dbed5983849964348eefbc3429733b MATCH
wallet-broker/src/zec/spend/effects.rs 379 cfa6a86f35d6772053025ca1c9bbb3154d604f79c90d031af261787938576035 MATCH
docs/testing/BBD-WAL-009-NATIVE-LAYOUT-VALIDATION-01.md 354 ffcfc14837ff02aba33a5a24fa6a58d610055b18b4efd150787c6b8acba7dc43 MATCH
docs/testing/BBD-WAL-009-DECODED-EFFECTS-EXPECTED-RED-01.md 196 a7def01abd54eae10e663e341ce8b253992d883863a1410c93278fa698d78431 MATCH
docs/testing/BBD-WAL-009-DECODED-EFFECTS-VALIDATION-01.md 165 fe82cce82ebba2595747a411a5ffd2258d86abed36dc9b3de60517193d4babad MATCH
docs/testing/BBD-WAL-009-DECODED-EFFECTS-VALIDATION-RESUME-01.md 345 02d078de8a0269ba2dc41664450965a3eec1036ccb5343db6bb8f90518a64f5a MATCH
docs/testing/BBD-WAL-009-CLEANUP-OBSERVATIONS-EXPECTED-RED-01.md 177 996f26aeea6b1cb27799200b24258e0e41defaa05505a0ee2b704c60471c2e9e MATCH
wallet-broker/src/vault.rs 794 f23247b0e46c68dfc125804fa98d16f7022464e1e4d94a8c1e62771a045be49 MATCH
```
- Final state: test_support.rs at baseline 11ef7579a342ef3feee455418fa2522a98ee5616ea719aea91e6396927ee5cdb.
- Backup retained at wallet-broker/target/bbd-wal009-cleanup-observations-validation-01-test-support-original.rs (baseline bytes).
- No fault remains installed. No permanent source/test changes. No evidence file existed prior to this write.

## Result summary

| Stage | Exit | Passed | Failed | Filtered | Duration |
| --- | ---: | ---: | ---: | ---: | :--- |
| Test 1 baseline green | 0 | 1 | 0 | 14 | 1.75s |
| Test 2 fault-a | 101 | 0 | 1 | 14 | 2.26s |
| Test 3 fault-b | 101 | 0 | 1 | 14 | 1.99s |
| Test 4 restored green | 0 | 1 | 0 | 14 | 1.69s |
| Test 5 real software pipeline | 0 | 1 | 0 | 14 | 143.73s |
| Test 6 synthetic external pipeline | 0 | 1 | 0 | 14 | 140.59s |

## Mutation/restoration lineage

- fault-a: baseline(4463, 11ef7579...) -> 4464, 923c5fcbbfa82e44222661b56d7b2b0eed0cdb54bbceaf5ae0d589410087a2 (backup written).
- restore-a: 923c5fcb... -> 4463, 11ef7579... (backup verified byte-equal before write).
- fault-b: baseline(4463, 11ef7579...) -> 4463, bc088bddcc63a90342e2bf701aee904a45ad787912460ebbfde4a4e9256554c4 (backup intact).
- restore-b: bc088bdd... -> 4463, 11ef7579... (backup verified byte-equal before write).
- Every restoration reproduced the exact baseline bytes/hash. No fault remains installed at completion.

## Tool and process identifiers

- hermes --version: terminal (foreground), not a background process.
  Hermes Agent v0.18.2 (2026.7.7.2) upstream b1f003e1 local 10b6d1a9 (+1 carried commit).
- HERMES_SESSION_ID: 20260908_213504_01112b (read from environment).
- Preflight inventory: terminal (foreground), python3 heredoc.
- fault-a: terminal (foreground), python3 heredoc.
- Test 1 command: terminal background, session_id proc_0f63d9d74c0e, pid 3526400.
- Test 2 command: terminal background, session_id proc_0590a3e1d8f9, pid 3527197.
- restore-a: terminal (foreground), python3 heredoc.
- fault-b: terminal (foreground), python3 heredoc.
- Test 3 command: terminal background, session_id proc_13b21aaa302b, pid 3527905.
- restore-b: terminal (foreground), python3 heredoc.
- Test 4 command: terminal background, session_id proc_c73077458baf, pid 3528559.
- Test 5 command: terminal background, session_id proc_16f962490a4b, pid 3529011.
- Test 6 command: terminal background, session_id proc_04a1da0537c2, pid 3530002.
- Final inventory: terminal (foreground), python3 heredoc.
- Background waits used `process action=wait` (clamped to 60s per call; multiple waits
  issued where tests exceeded the single-wait window, per the runtime's 60s clamp).

## Deviations and disclosures

- No deviations from the authorized handoff. All six invocations used the exact
  command strings verbatim with no wrappers, redirection, or pipelines. No formatter,
  extra compiler/test probes, full-target reruns, Git, network, actor/subagent, or
  historical reload was performed.
- The three compiler warnings (dead_code / never-used) are ordinary and did not affect
  any test outcome; they are preserved verbatim in the process output above.
- Path normalization: `<repo>/` rendered as repository root
  in command and output records. No other normalization.
- `$HOME` in the command strings is the user's literal shell variable, unchanged.
- The old decoded-effects backup is unrelated and was not reused or changed.
- Provider/model attribution is recorded from the active profile; reviewer database
  verification should cross-check against session 20260908_213504_01112b.

## Outcome

All six test invocations produced exactly the authorized results. Both deliberate
faults were proven to cause the focused regression to fail at the expected guarded
assertion, and both were restored to the exact baseline byte/hash. The two affected
software/synthetic pipelines passed on the restored source. The final inventory
confirms all 33 frozen rows MATCH and test_support.rs is at baseline. No permanent
source/test changes; only the prescribed temporary falsifications, one retained
backup, and this evidence record were written.
