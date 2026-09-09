# BBD-WAL-009 pipeline panic expected red 01

Integration note (2026-09-09): local repository and home prefixes are rendered as <repo> and <home>. Earlier hashes/counts describe execution-time files. This is an archival evidence record; acceptance limits and reported deviations remain governed by the corresponding reviewer handoffs. No test was repeated for this source checkpoint.


Actor: Jr Dev — Hermes (execution/evidence only).
Reviewer: Codex, High. Authorization parent: ea5017ae.
Handoff: docs/handoff/HERMES_BBD_WAL_009_PIPELINE_PANIC_EXPECTED_RED_01.md.

## Runtime identity

- hermes --version: Hermes Agent v0.18.2 (2026.7.7.2) · upstream 990473a7 · local 10b6d1a9 (+1 carried commit)
- Provider/model: unavailable (not recorded by runtime; reviewer verifies).
- HERMES_SESSION_ID: 20260908_232331_f34d0d

## Inventory (37-file baseline, both pre and post)

All 37 paths MATCH expected line counts and SHA-256 digests (no source/test
mutation). The inventory script from the active handoff was run once before
testing and once after evidence creation. The evidence output file below was
not present during either inventory run (assertion not-Path exists passed).

## Exact command (one invocation, no wrapper/redirection/pipeline/option override)

"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_sign_verify real_pipeline_panic_releases_owners_and_never_publishes -- --exact

Launch workdir (explicit, no cd): $REPO (repository root = <repo>)
Background session id: proc_b9e0fccd2d63
PID: 3564312
Exit code: 101
Completion reason: exited
Compilation: finished in 1.60s (unoptimized + debuginfo)
Test execution: finished in 1.62s

## Complete process.log

```
warning: method `matches_review` is never used
  --> src/native.rs:77:19
   |
49 | impl ZecConfirmationCapability {
   | ------------------------------ method in this implementation
...
77 |     pub(crate) fn matches_review(&self, review: &ZecNativeReview) -> bool {
   |                   ^^^^^^^^^^^^^^^
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
1602 | fn timestamp_for_test(value: &str) -> Result<u64, ZecError> {
     |      ^^^^^^^^^^^^^^^

warning: `bitbook-wallet-broker` (lib) generated 3 warnings
   Compiling bitbook-wallet-broker v0.1.0 (<repo>/wallet-broker)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.60s
     Running tests/zec_sign_verify.rs (wallet-broker/target/debug/deps/zec_sign_verify-4c279ff0e3acb2a4)

running 1 test
test real_pipeline_panic_releases_owners_and_never_publishes ... FAILED

failures:

---- real_pipeline_panic_releases_owners_and_never_publishes stdout ----
thread 'real_pipeline_panic_releases_owners_and_never_publishes' (3564470) panicked at src/zec/test_support.rs:4280:9:
INTERNAL
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

thread 'real_pipeline_panic_releases_owners_and_never_publishes' (3564470) panicked at tests/zec_sign_verify.rs:1268:5:
assertion `left == right` failed: real pipeline panic must access the operation seed
  left: 0
 right: 1

failures:
    real_pipeline_panic_releases_owners_and_never_publishes

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 15 filtered out; finished in 1.62s

error: test failed, to rerun pass `--test zec_sign_verify`
```

## Outcome verification

- Exit code 101: matches handoff expectation.
- Test result line: "0 passed; 1 failed; 0 ignored; 0 measured; 15 filtered out" — one
  failed, 15 filtered: matches handoff expectation.
- INTERNAL panic payload at src/zec/test_support.rs:4280:9: passes (the named panic
  text "INTERNAL" is emitted at the helper-only panic point).
- seed_accesses assertion at tests/zec_sign_verify.rs:1268:5 then fails with message
  "real pipeline panic must access the operation seed", left 0, right 1: passes
  (absence of real stage activity is the regression, not a compilation failure,
  setup error, unrelated panic, or manually fabricated stage count).

No repair or retry was performed. No source/test edits, formatter, Git, network,
or broader tests were run. All 37 baseline identities verified unchanged by the
post-test inventory.
