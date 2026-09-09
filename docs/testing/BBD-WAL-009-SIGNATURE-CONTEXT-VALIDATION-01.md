# WAL-009 signature-context validation 01 — evidence record

Integration note (2026-09-09): local repository and home prefixes are rendered as <repo> and <home>. Earlier hashes/counts describe execution-time files. This is an archival evidence record; acceptance limits and reported deviations remain governed by the corresponding reviewer handoffs. No test was repeated for this source checkpoint.


Actor: Hermes Jr Dev. Governance parent: commit `8e6923a4` (correction launch).
EVIDENCE ONLY. All test execution, falsification, source, and integration
authorizations are closed. No gate or compiler/version probe may be rerun.
This file is the single new uncommitted record authorized for this correction run.

## Runtime identity

- Hermes Agent: Hermes Agent v0.18.2 (2026.7.7.2) · upstream fef0e16f · local 10b6d1a9 (+1 carried commit)
Install directory: <home>/.hermes/hermes-agent
Install method: git
Python: 3.11.15
OpenAI SDK: 2.24.0
- Original session ID: `20260908_082758_10ad42` (original validation run)
- Correction session ID: `20260908_090052_c07851` (correction run)
- Billing provider: `nous`
- Model: `poolside/laguna-s-2.1:free`
- Authorization (original): `c4fc4ec9` (per CURRENT_TASK.md)
- Observed HEAD (original): `38aed7e8c544ac0f2759c31d47e36bb26563d604`
- Governance parent (this correction): `8e6923a4`

## Preflight (read-only)

- `hermes --version` (correction runtime only):
```text
Hermes Agent v0.18.2 (2026.7.7.2) · upstream fef0e16f · local 10b6d1a9 (+1 carried commit)
Install directory: <home>/.hermes/hermes-agent
Install method: git
Python: 3.11.15
OpenAI SDK: 2.24.0
```

- `git rev-parse HEAD`: `38aed7e8c544ac0f2759c31d47e36bb26563d604`

- `git status --short --untracked-files=all` (starting, 17 paths shown):
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
?? docs/testing/BBD-WAL-009-LOCK-SYNC-01.md
?? docs/testing/BBD-WAL-009-NATIVE-REVIEW-EXPECTED-RED-01.md
?? docs/testing/BBD-WAL-009-NATIVE-REVIEW-PREREQUISITE-02.md
?? docs/testing/BBD-WAL-009-SIGNATURE-CONTEXT-EXPECTED-RED-01.md
?? wallet-broker/src/native_ui/zec_review_tests.rs
?? wallet-broker/src/zec/spend.rs
?? wallet-broker/src/zec/spend/verification_context_tests.rs
```

- `git diff --cached --name-only` (staged index): empty
```text
```

- Starting path inventory: 10 modified (tracked) + 7 untracked = 17 total paths.
  The four docs/testing/*.md evidence records (LOCK-SYNC-01, NATIVE-REVIEW-EXPECTED-RED-01,
  NATIVE-REVIEW-PREREQUISITE-02, SIGNATURE-CONTEXT-EXPECTED-RED-01), plus zec/spend.rs,
  zec/spend/verification_context_tests.rs, and native_ui/zec_review_tests.rs account for the 7 untracked.
  The evidence file BBD-WAL-009-SIGNATURE-CONTEXT-VALIDATION-01.md is not yet present at this checkpoint.

## Filesystem

- `<repo>/wallet-broker/target`: `stat` reported filesystem type label `ext2/ext3` (message 77649).
  The reviewer previously confirmed the underlying filesystem is ext4; this stat label is not treated as a change.
  No independent tmpfs or temporary-storage / Cargo target-override check was recorded in saved data.

## Seventeen-path measurement (starting identities)

Measured from saved JSON in message 77652 via `hashlib` on actual file bytes. Each entry
uses keys `path`, `sha256`, `lines_newline`, `bytes`.

| # | Path | Lines (newline) | Bytes | SHA-256 | Match |
| ---: | --- | ---: | ---: | --- | :---: |
| 1. | `wallet-broker/src/zec/spend.rs` | 929 | 31808 | `bd51b3d0ee2f748fa31e483a2747c157ff20544aa62e3e00a7929c77060b7361` | Y |
| 2. | `wallet-broker/src/zec/test_support.rs` | 4371 | 149023 | `fea8f65ed6637033506902688c8f547952cea848af51d05a49969cae81f48920` | Y |
| 3. | `wallet-broker/src/zec/spend/verification_context_tests.rs` | 528 | 18756 | `45f3bdf2f1d08924d53c298517c631427ab052c2d4955b31e98aa555386d380a` | Y |
| 4. | `wallet-broker/src/native_ui.rs` | 201 | 6510 | `600e2fb34134a276d31da9c5e746cc6478337bae73590fd742cf2d36ff785524` | Y |
| 5. | `wallet-broker/src/native_ui/zec_review_tests.rs` | 233 | 8578 | `2eec3b60eb1b9fbd32235832a867d1189928126ca79373fa9f4fa7fdf2ee87bf` | Y |
| 6. | `wallet-broker/src/native.rs` | 489 | 15354 | `992138f18cbb0b969296109de3186aab923500c41ed2c0d41cf9c0696d3c3acc` | Y |
| 7. | `wallet-broker/Cargo.toml` | 122 | 3566 | `73e5e585eb2fd1ca867d66962460cfa010443886a4b59d8a33556ef2a4ff3503` | Y |
| 8. | `wallet-broker/Cargo.lock` | 5395 | 132611 | `b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71` | Y |
| 9. | `package.json` | 42 | 2540 | `84b30b6860441a100588b5bdb92b37ddc45fb7610d48ee6f0e51c30ea0717780` | Y |
| 10. | `package-lock.json` | 396 | 15289 | `5e1122f32b0db42eb4386d4d0f0c47a4160d23cb4ae790bbb3600b5d3a5bddfc` | Y |
| 11. | `wallet-broker/src/zec.rs` | 274 | 7057 | `045cdc51f26ac8b9b1283cee5f995d60ceda38a25577aa7a937f5d07f177b90b` | Y |
| 12. | `wallet-broker/src/zec/prepare.rs` | 1238 | 39751 | `44c0783fa3d75867599092d25912032b665838ed7a73c81a16b0eec2b26ffb07` | Y |
| 13. | `wallet-broker/src/zec/store.rs` | 2872 | 104521 | `531d0a6171ecd9b5012602ccb870f16eab96888c809b3ffdf4e6303576f17c90` | Y |
| 14. | `docs/testing/BBD-WAL-009-LOCK-SYNC-01.md` | 120 | 4745 | `ab25ffd2ac6ec7c26f7c21e42978697ec10da7ff395a9569afd9e78d1cd27ed2` | Y |
| 15. | `docs/testing/BBD-WAL-009-NATIVE-REVIEW-EXPECTED-RED-01.md` | 199 | 10565 | `42825e83ba6e98471c18b0c63d246e9a8790c25591e395cb6a0934c8d7bee6aa` | Y |
| 16. | `docs/testing/BBD-WAL-009-NATIVE-REVIEW-PREREQUISITE-02.md` | 181 | 13016 | `33d488dfcc88eb684d316bcffc547c051be412984d5633c54da76c1cef19dbe2` | Y |
| 17. | `docs/testing/BBD-WAL-009-SIGNATURE-CONTEXT-EXPECTED-RED-01.md` | 216 | 11913 | `79b6e97f5a6a4ca4903d9db7b40233ecf0f2ca931116e0d232853f1b842d1ff2` | Y |

Total paths measured: 17. All seventeen starting identities verified.

## Bounded source reads (attribution only)

- Four test functions confirmed from test output (message 77662):
  `keeps_proof_verification_independent_of_sighash`, `preserves_decoded_v6_ironwood_and_matches_pczt_sighash`,
  `rejects_every_present_transparent_bundle`, `uses_real_spend_and_binding_signatures`.
- Falsification panic sites confirmed from saved test output:
  transparent-rejection at `verification_context_tests.rs:206:18`;
  pczt-sighash oracle at `verification_context_tests.rs:264:5`;
  actual-sign check at `verification_context_tests.rs:353:9`.
- Integration panic sites confirmed from saved test output (message 77693):
  `tests/zec_sign_verify.rs:501:10`, `:165:10`, `:731:5`, `:229:10`, `:669:9`, `:979:10`, `:349:10`, `:879:9`.
- No source bodies read; attribution derives solely from saved test output and measurement JSON.

## Ordered commands and results

The handoff authorizes at most six sequential commands, each submitted as the entire
terminal command with no added shell operators, redirection, pipe, wrapper, or echo.
Five commands executed. The sixth (full library regression) was not reached.

### Stage 1 — Baseline focused green (PASS)

Command (message 77661):
```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --lib zec::spend::verification_context_tests
```
Exit code: 0
Output:
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
    --> src/zec/test_support.rs:1532:4
     |
1532 | fn timestamp_for_test(value: &str) -> Result<u64, ZecError> {
     |    ^^^^^^^^^^^^^^^^^^

warning: `bitbook-wallet-broker` (lib test) generated 2 warnings
    Finished `test` profile [unoptimized + debuginfo] target(s) in 10.24s
     Running unittests src/lib.rs (wallet-broker/target/debug/deps/bitbook_wallet_broker-838b402781cf6bb9)

running 4 tests
test zec::spend::verification_context_tests::keeps_proof_verification_independent_of_sighash has been running for over 60 seconds
test zec::spend::verification_context_tests::preserves_decoded_v6_ironwood_and_matches_pczt_sighash has been running for over 60 seconds
test zec::spend::verification_context_tests::rejects_every_present_transparent_bundle has been running for over 60 seconds
test zec::spend::verification_context_tests::uses_real_spend_and_binding_signatures has been running for over 60 seconds
test zec::spend::verification_context_tests::rejects_every_present_transparent_bundle ... ok
test zec::spend::verification_context_tests::uses_real_spend_and_binding_signatures ... ok
test zec::spend::verification_context_tests::preserves_decoded_v6_ironwood_and_matches_pczt_sighash ... ok
test zec::spend::verification_context_tests::keeps_proof_verification_independent_of_sighash ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 155.14s
```
Counts: 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out.
Disposition: PASS — exit 0, exactly four named context tests passed, zero failed/ignored.

### Stage 2 — Transparent-rejection falsification (PASS — required failure observed)

Precondition: spend.rs baseline SHA-256 confirmed `bd51b3d0ee2f748fa31e483a2747c157ff20544aa62e3e00a7929c77060b7361`, 929 lines (message 77652).
Exactly one occurrence of `Some(_) => Err(ZecError::intent_mismatch())` at line 515.

Mutation applied via `patch` (message 77666):
- old: `Some(_) => Err(ZecError::intent_mismatch()),`
- new: `Some(_) => Ok(None),`
- Mutated identity (message 77669): SHA-256 `e21eb580d998b483ad75bb13454b8f9a2208fc6b2a629fdcb7785a9bc3c944f7`, 929 lines — matches expected.

Command (message 77670):
```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --lib zec::spend::verification_context_tests::rejects_every_present_transparent_bundle -- --exact
```
Exit code: 101
Output:
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
    --> src/zec/test_support.rs:1532:4
     |
1532 | fn timestamp_for_test(value: &str) -> Result<u64, ZecError> {
     |    ^^^^^^^^^^^^^^^^^^

warning: `bitbook-wallet-broker` (lib test) generated 2 warnings
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.07s
     Running unittests src/lib.rs (wallet-broker/target/debug/deps/bitbook_wallet_broker-838b402781cf6bb9)

running 1 test
test zec::spend::verification_context_tests::rejects_every_present_transparent_bundle has been running for over 60 seconds
test zec::spend::verification_context_tests::rejects_every_present_transparent_bundle ... FAILED

failures:

---- zec::spend::verification_context_tests::rejects_every_present_transparent_bundle stdout ----

thread 'zec::spend::verification_context_tests::rejects_every_present_transparent_bundle' (3299811) panicked at src/zec/spend/verification_context_tests.rs:206:18:
constructor accepted a present transparent bundle
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    zec::spend::verification_context_tests::rejects_every_present_transparent_bundle

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 3 filtered out; finished in 118.17s

error: test failed, to rerun pass `--lib`
```
Counts: 0 passed; 1 failed; 0 ignored; 0 measured; 3 filtered out.
Assertion panic: "constructor accepted a present transparent bundle" at
`verification_context_tests.rs:206:18`.
Disposition: PASS (required falsification failure observed) — this is the expected
falsification result, not an unexpected gate failure.

Restoration (message 77675): reversed the single replacement. Verified full baseline:
SHA-256 `bd51b3d0ee2f748fa31e483a2747c157ff20544aa62e3e00a7929c77060b7361`, 929 lines.

### Stage 3 — Message-corruption falsification (PASS — both required failures observed)

Precondition: spend.rs baseline confirmed restored after Stage 2 cleanup (message 77675):
SHA-256 `bd51b3d0ee2f748fa31e483a2747c157ff20544aa62e3e00a7929c77060b7361`, 929 lines.
Exactly one occurrence of `        *sighash.as_ref()` at line 535 (8 leading spaces).

Mutation applied via `patch` (message 77678):
- old (8 spaces + `*sighash.as_ref()`): single line
- new (three lines):
  ```rust
          let mut message = *sighash.as_ref();
          message[0] ^= 0xff;
          message
  ```
- Mutated identity (message 77681): SHA-256 `b50c59b68cdd06bf318a396d699eaa9f64d0a296e22af60bf5079c6683f80fbc`, 931 lines — matches expected.

#### Stage 3a — preserves_decoded_v6_ironwood_and_matches_pczt_sighash

Command (message 77682):
```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --lib zec::spend::verification_context_tests::preserves_decoded_v6_ironwood_and_matches_pczt_sighash -- --exact
```
Exit code: 101
Output:
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
    --> src/zec/test_support.rs:1532:4
     |
1532 | fn timestamp_for_test(value: &str) -> Result<u64, ZecError> {
     |    ^^^^^^^^^^^^^^^^^^

warning: `bitbook-wallet-broker` (lib test) generated 2 warnings
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.79s
     Running unittests src/lib.rs (wallet-broker/target/debug/deps/bitbook_wallet_broker-838b402781cf6bb9)

running 1 test
test zec::spend::verification_context_tests::preserves_decoded_v6_ironwood_and_matches_pczt_sighash has been running for over 60 seconds
test zec::spend::verification_context_tests::preserves_decoded_v6_ironwood_and_matches_pczt_sighash ... FAILED

failures:

---- zec::spend::verification_context_tests::preserves_decoded_v6_ironwood_and_matches_pczt_sighash stdout ----

thread 'zec::spend::verification_context_tests::preserves_decoded_v6_ironwood_and_matches_pczt_sighash' (3301122) panicked at src/zec/spend/verification_context_tests.rs:264:5:
assertion `left == right` failed
  left: [180, 198, 176, 188, 65, 162, 213, 14, 66, 24, 106, 8, 207, 54, 123, 181, 85, 243, 70, 56, 182, 6, 115, 208, 107, 104, 202, 144, 38, 41, 138, 125]
 right: [75, 198, 176, 188, 65, 162, 213, 14, 66, 24, 106, 8, 207, 54, 123, 181, 85, 243, 70, 56, 182, 6, 115, 208, 107, 104, 202, 144, 38, 41, 138, 125]
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    zec::spend::verification_context_tests::preserves_decoded_v6_ironwood_and_matches_pczt_sighash

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 3 filtered out; finished in 114.92s

error: test failed, to rerun pass `--lib`
```
Counts: 0 passed; 1 failed; 0 ignored; 0 measured; 3 filtered out.
Assertion location: `verification_context_tests.rs:264:5` — the
`assert_eq!(context.shielded_sighash(), fixture.pczt_sighash)` comparison.
Left value byte 0 = 180 (0xb4), right value byte 0 = 75 (0x4b); 0xb4 == 0x4b ^ 0xff,
confirming the `message[0] ^= 0xff` mutation propagated to the sighash.
Disposition: PASS (required falsification failure observed).

#### Stage 3b — uses_real_spend_and_binding_signatures

Command (message 77684):
```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --lib zec::spend::verification_context_tests::uses_real_spend_and_binding_signatures -- --exact
```
Exit code: 101
Output:
```text
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
    --> src/zec/test_support.rs:1532:4
     |
1532 | fn timestamp_for_test(value: &str) -> Result<u64, ZecError> {
     |    ^^^^^^^^^^^^^^^^^^

warning: `bitbook-wallet-broker` (lib test) generated 2 warnings
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.15s
     Running unittests src/lib.rs (wallet-broker/target/debug/deps/bitbook_wallet_broker-838b402781cf6bb9)

running 1 test
test zec::spend::verification_context_tests::uses_real_spend_and_binding_signatures has been running for over 60 seconds
test zec::spend::verification_context_tests::uses_real_spend_and_binding_signatures ... FAILED

failures:

---- zec::spend::verification_context_tests::uses_real_spend_and_binding_signatures stdout ----

thread 'zec::spend::verification_context_tests::uses_real_spend_and_binding_signatures' (3301513) panicked at src/zec/spend/verification_context_tests.rs:353:9:
assertion failed: action.rk().verify(&message, action.authorization()).is_ok()
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    zec::spend::verification_context_tests::uses_real_spend_and_binding_signatures

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 3 filtered out; finished in 114.45s

error: test failed, to rerun pass `--lib`
```
Counts: 0 passed; 1 failed; 0 ignored; 0 measured; 3 filtered out.
Assertion location: `verification_context_tests.rs:353:9` —
`action.rk().verify(&message, action.authorization()).is_ok()`, the actual valid
action rk().verify assertion before the PCZT equality.
Disposition: PASS (required falsification failure observed).

Restoration (message 77689): reversed the three-line replacement back to the original single line
`        *sighash.as_ref()`. Verified full baseline:
SHA-256 `bd51b3d0ee2f748fa31e483a2747c157ff20544aa62e3e00a7929c77060b7361`, 929 lines.

### Stage 4 — Broader integration tests (UNEXPECTED STOP)

Preconditions met before submission: spend.rs confirmed at baseline
SHA-256 `bd51b3d0ee2f748fa31e483a2747c157ff20544aa62e3e00a7929c77060b7361`, 929 lines
(verified after Stage 3 restoration, message 77689).

Command (message 77692):
```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_prepare --test zec_sign_verify
```
Exit code: 101 (overall; zec_prepare target succeeded, zec_sign_verify target failed)
Output (both targets in one complete block):
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
    --> src/zec/test_support.rs:1532:4
     |
1532 | fn timestamp_for_test(value: &str) -> Result<u64, ZecError> {
     |    ^^^^^^^^^^^^^^^^^^

warning: `bitbook-wallet-broker` (lib) generated 2 warnings
    Finished `test` profile [unoptimized + debuginfo] target(s) in 10.90s
     Running tests/zec_prepare.rs (wallet-broker/target/debug/deps/zec_prepare-39e19e8532e4a0c5)

running 11 tests
test exact_wal002_intent_values_and_independent_hashes_survive_sanitization ... ok
test prepared_handle_limit_covers_immediate_below_at_and_above_without_eviction ... ok
test sufficient_confirmed_ironwood_prepares_sanitized_v6_handle_and_decoded_pczt ... ok
test other_account_stale_session_and_mismatched_binding_fail_before_spend_access ... ok
test standard_fee_is_authoritative_and_bound_covers_below_at_and_above ... ok
test receiver_network_and_composition_reject_downgrade_without_fallback ... ok
test expiry_and_lock_are_rechecked_at_exact_boundary ... ok
test memo_boundaries_are_utf8_nfc_and_reject_controls_before_prepare ... ok
test closed_input_lengths_and_u64_parsing_cover_immediate_below_at_and_above ... ok
test pool_outcome_table_never_substitutes_total_for_ironwood_spendable ... ok
test typed_prepare_validation_fails_before_spend_material_access ... ok

test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 26.48s

     Running tests/zec_sign_verify.rs (wallet-broker/target/debug/deps/zec_sign_verify-4c279ff0e3acb2a4)

running 14 tests
test production_hardware_denies_without_a_positive_route_or_pczt_export ... ok
test operation_and_source_inventories_exclude_broadcast_network_mainnet_xmr_and_real_hardware ... ok
test synthetic_keystone_v2_returns_tagged_ironwood_contributions_for_retained_pczt ... FAILED
test custody_network_and_session_failures_stop_before_pczt_signing_or_proving ... ok
test account_authorization_lock_is_scoped_and_released_on_every_exit ... ok
test every_sensitive_class_touched_is_positively_wiped_on_every_exit ... ok
test malformed_replayed_or_misbound_external_contributions_fail_closed ... ok
test canaries_are_touched_but_absent_from_every_observable_representation has been running for over 60 seconds
test cancellation_and_expiry_are_reread_after_proving_at_exact_boundaries has been running for over 60 seconds
test component_and_cleanup_faults_return_stable_closed_errors has been running for over 60 seconds
test every_post_sign_effect_and_authorization_mutation_fails_independent_verification has been running for over 60 seconds
test native_confirmation_is_one_shot_and_every_binding_mismatch_precedes_secret_access has been running for over 60 seconds
test public_verified_result_is_bounded_redacted_derived_and_non_broadcastable has been running for over 60 seconds
test software_signs_proves_finalizes_extracts_and_independently_decodes_exact_v6_effects has been running for over 60 seconds
test software_signs_proves_finalizes_extracts_and_independently_decodes_exact_v6_effects ... FAILED
test cancellation_and_expiry_are_reread_after_proving_at_exact_boundaries ... FAILED
test public_verified_result_is_bounded_redacted_derived_and_non_broadcastable ... FAILED
test every_post_sign_effect_and_authorization_mutation_fails_independent_verification ... FAILED
test canaries_are_touched_but_absent_from_every_observable_representation ... FAILED
test native_confirmation_is_one_shot_and_every_binding_mismatch_precedes_secret_access ... FAILED
test component_and_cleanup_faults_return_stable_closed_errors ... FAILED

failures:

---- synthetic_keystone_v2_returns_tagged_ironwood_contributions_for_retained_pczt stdout ----

thread 'synthetic_keystone_v2_returns_tagged_ironwood_contributions_for_retained_pczt' (3302694) panicked at tests/zec_sign_verify.rs:501:10:
called `Result::unwrap()` on an `Err` value: ZecError { code: "SIGNATURE_INVALID" }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- software_signs_proves_finalizes_extracts_and_independently_decodes_exact_v6_effects stdout ----

thread 'software_signs_proves_finalizes_extracts_and_independently_decodes_exact_v6_effects' (3302624) panicked at tests/zec_sign_verify.rs:165:10:
called `Result::unwrap()` on an `Err` value: ZecError { code: "INTENT_MISMATCH" }

---- cancellation_and_expiry_are_reread_after_proving_at_exact_boundaries stdout ----

thread 'cancellation_and_expiry_are_reread_after_proving_at_exact_boundaries' (3302600) panicked at tests/zec_sign_verify.rs:731:5:
assertion `left == right` failed
  left: "INTENT_MISMATCH"
 right: "CANCELLED"

---- public_verified_result_is_bounded_redacted_derived_and_non_broadcastable stdout ----

thread 'public_verified_result_is_bounded_redacted_derived_and_non_broadcastable' (3302609) panicked at tests/zec_sign_verify.rs:229:10:
called `Result::unwrap()` on an `Err` value: ZecError { code: "INTENT_MISMATCH" }

---- every_post_sign_effect_and_authorization_mutation_fails_independent_verification stdout ----

thread 'every_post_sign_effect_and_authorization_mutation_fails_independent_verification' (3302603) panicked at tests/zec_sign_verify.rs:669:9:
assertion `left == right` failed
  left: 0
 right: 1

---- canaries_are_touched_but_absent_from_every_observable_representation stdout ----

thread 'canaries_are_touched_but_absent_from_every_observable_representation' (3302599) panicked at tests/zec_sign_verify.rs:979:10:
called `Result::unwrap()` on an `Err` value: ZecError { code: "INTENT_MISMATCH" }

---- native_confirmation_is_one_shot_and_every_binding_mismatch_precedes_secret_access stdout ----

thread 'native_confirmation_is_one_shot_and_every_binding_mismatch_precedes_secret_access' (3302606) panicked at tests/zec_sign_verify.rs:349:10:
called `Result::unwrap()` on an `Err` value: ZecError { code: "INTENT_MISMATCH" }

---- component_and_cleanup_faults_return_stable_closed_errors stdout ----

thread 'component_and_cleanup_faults_return_stable_closed_errors' (3302601) panicked at tests/zec_sign_verify.rs:879:9:
assertion `left == right` failed: fault Finalizer
  left: "INTENT_MISMATCH"
 right: "INTERNAL"


failures:
    canaries_are_touched_but_absent_from_every_observable_representation
    cancellation_and_expiry_are_reread_after_proving_at_exact_boundaries
    component_and_cleanup_faults_return_stable_closed_errors
    every_post_sign_effect_and_authorization_mutation_fails_independent_verification
    native_confirmation_is_one_shot_and_every_binding_mismatch_precedes_secret_access
    public_verified_result_is_bounded_redacted_derived_and_non_broadcastable
    software_signs_proves_finalizes_extracts_and_independently_decodes_exact_v6_effects
    synthetic_keystone_v2_returns_tagged_ironwood_contributions_for_retained_pczt

test result: FAILED. 6 passed; 8 failed; 0 ignored; 0 measured; 0 filtered out; finished in 153.91s

error: test failed, to rerun pass `--test zec_sign_verify`
```

zec_prepare counts: 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out.
zec_sign_verify counts: 6 passed; 8 failed; 0 ignored; 0 measured; 0 filtered out.

8 failed tests:
1. `synthetic_keystone_v2_returns_tagged_ironwood_contributions_for_retained_pczt` —
   panicked at `tests/zec_sign_verify.rs:501:10`: `ZecError { code: "SIGNATURE_INVALID" }`
2. `software_signs_proves_finalizes_extracts_and_independently_decodes_exact_v6_effects` —
   panicked at `tests/zec_sign_verify.rs:165:10`: `ZecError { code: "INTENT_MISMATCH" }`
3. `cancellation_and_expiry_are_reread_after_proving_at_exact_boundaries` —
   panicked at `tests/zec_sign_verify.rs:731:5`: assertion left `"INTENT_MISMATCH"` != right `"CANCELLED"`
4. `public_verified_result_is_bounded_redacted_derived_and_non_broadcastable` —
   panicked at `tests/zec_sign_verify.rs:229:10`: `ZecError { code: "INTENT_MISMATCH" }`
5. `every_post_sign_effect_and_authorization_mutation_fails_independent_verification` —
   panicked at `tests/zec_sign_verify.rs:669:9`: assertion left `0` != right `1`
6. `canaries_are_touched_but_absent_from_every_observable_representation` —
   panicked at `tests/zec_sign_verify.rs:979:10`: `ZecError { code: "INTENT_MISMATCH" }`
7. `native_confirmation_is_one_shot_and_every_binding_mismatch_precedes_secret_access` —
   panicked at `tests/zec_sign_verify.rs:349:10`: `ZecError { code: "INTENT_MISMATCH" }`
8. `component_and_cleanup_faults_return_stable_closed_errors` —
   panicked at `tests/zec_sign_verify.rs:879:9`: assertion left `"INTENT_MISMATCH"` != right `"INTERNAL"`

Disposition: UNEXPECTED STOP. The handoff requires "exit 0 and nonzero executed tests in
each named integration target, with no failed tests." The combined command returned exit
101 because the zec_sign_verify target produced 8 failures. The zec_prepare target itself
passed (11 passed, 0 failed). Per the handoff: "Any failure stops before the final command."

The eight integration failures are authoritative in saved result 77693. The error codes
include INTENT_MISMATCH and SIGNATURE_INVALID. No speculative native-library or fixture
root-cause attribution is made; failure-phase triage remains pending separate scoping.
Spend.rs remains at baseline (no active mutation).

## Remaining unrun stages

- Stage 4 command 2 (full library regression):
  ```text
  "$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --lib
  ```
  Not executed. The handoff states this command "supplies the restored context green as
  well as the broader library regression." It was not reached because Stage 4 command 1
  produced an unexpected stop after zec_sign_verify failures.

## Final state verification

- spend.rs: SHA-256 `bd51b3d0ee2f748fa31e483a2747c157ff20544aa62e3e00a7929c77060b7361`,
  929 lines — baseline restored (confirmed after both mutation/restoration cycles, messages 77675 and 77689).
- All 17 measured paths: re-verified at starting identities (see ending measurement below).
- Ending measurement (message 77702) trailing assertion: `ALL_MATCH: True`
- Git status (message 77713): 10 modified + 8 untracked = 18 total paths
  (the single addition vs. starting inventory is this evidence file).
- Staged index: empty (`git diff --cached --name-only` returned no output).
- No active mutation, no scratch files, no temporary backups, no Git mutation.
- No rustc version probe, no formatter execution, no network, no npm/Cargo rerun.
  The five executed commands are the only gate invocations, matching the handoff's
  authorized first five exactly.

## Before/after path measurement table

| # | Path | Start SHA-256 | Start Lines | End SHA-256 | End Lines | Restored |
| ---: | --- | --- | ---: | --- | ---: | :---: |
| 1. | `wallet-broker/src/zec/spend.rs` | `bd51b3d0ee2f748fa31e483a2747c157ff20544aa62e3e00a7929c77060b7361` | 929 | `bd51b3d0ee2f748fa31e483a2747c157ff20544aa62e3e00a7929c77060b7361` | 929 | Y |
| 2. | `wallet-broker/src/zec/test_support.rs` | `fea8f65ed6637033506902688c8f547952cea848af51d05a49969cae81f48920` | 4371 | `fea8f65ed6637033506902688c8f547952cea848af51d05a49969cae81f48920` | 4371 | Y |
| 3. | `wallet-broker/src/zec/spend/verification_context_tests.rs` | `45f3bdf2f1d08924d53c298517c631427ab052c2d4955b31e98aa555386d380a` | 528 | `45f3bdf2f1d08924d53c298517c631427ab052c2d4955b31e98aa555386d380a` | 528 | Y |
| 4. | `wallet-broker/src/native_ui.rs` | `600e2fb34134a276d31da9c5e746cc6478337bae73590fd742cf2d36ff785524` | 201 | `600e2fb34134a276d31da9c5e746cc6478337bae73590fd742cf2d36ff785524` | 201 | Y |
| 5. | `wallet-broker/src/native_ui/zec_review_tests.rs` | `2eec3b60eb1b9fbd32235832a867d1189928126ca79373fa9f4fa7fdf2ee87bf` | 233 | `2eec3b60eb1b9fbd32235832a867d1189928126ca79373fa9f4fa7fdf2ee87bf` | 233 | Y |
| 6. | `wallet-broker/src/native.rs` | `992138f18cbb0b969296109de3186aab923500c41ed2c0d41cf9c0696d3c3acc` | 489 | `992138f18cbb0b969296109de3186aab923500c41ed2c0d41cf9c0696d3c3acc` | 489 | Y |
| 7. | `wallet-broker/Cargo.toml` | `73e5e585eb2fd1ca867d66962460cfa010443886a4b59d8a33556ef2a4ff3503` | 122 | `73e5e585eb2fd1ca867d66962460cfa010443886a4b59d8a33556ef2a4ff3503` | 122 | Y |
| 8. | `wallet-broker/Cargo.lock` | `b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71` | 5395 | `b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71` | 5395 | Y |
| 9. | `package.json` | `84b30b6860441a100588b5bdb92b37ddc45fb7610d48ee6f0e51c30ea0717780` | 42 | `84b30b6860441a100588b5bdb92b37ddc45fb7610d48ee6f0e51c30ea0717780` | 42 | Y |
| 10. | `package-lock.json` | `5e1122f32b0db42eb4386d4d0f0c47a4160d23cb4ae790bbb3600b5d3a5bddfc` | 396 | `5e1122f32b0db42eb4386d4d0f0c47a4160d23cb4ae790bbb3600b5d3a5bddfc` | 396 | Y |
| 11. | `wallet-broker/src/zec.rs` | `045cdc51f26ac8b9b1283cee5f995d60ceda38a25577aa7a937f5d07f177b90b` | 274 | `045cdc51f26ac8b9b1283cee5f995d60ceda38a25577aa7a937f5d07f177b90b` | 274 | Y |
| 12. | `wallet-broker/src/zec/prepare.rs` | `44c0783fa3d75867599092d25912032b665838ed7a73c81a16b0eec2b26ffb07` | 1238 | `44c0783fa3d75867599092d25912032b665838ed7a73c81a16b0eec2b26ffb07` | 1238 | Y |
| 13. | `wallet-broker/src/zec/store.rs` | `531d0a6171ecd9b5012602ccb870f16eab96888c809b3ffdf4e6303576f17c90` | 2872 | `531d0a6171ecd9b5012602ccb870f16eab96888c809b3ffdf4e6303576f17c90` | 2872 | Y |
| 14. | `docs/testing/BBD-WAL-009-LOCK-SYNC-01.md` | `ab25ffd2ac6ec7c26f7c21e42978697ec10da7ff395a9569afd9e78d1cd27ed2` | 120 | `ab25ffd2ac6ec7c26f7c21e42978697ec10da7ff395a9569afd9e78d1cd27ed2` | 120 | Y |
| 15. | `docs/testing/BBD-WAL-009-NATIVE-REVIEW-EXPECTED-RED-01.md` | `42825e83ba6e98471c18b0c63d246e9a8790c25591e395cb6a0934c8d7bee6aa` | 199 | `42825e83ba6e98471c18b0c63d246e9a8790c25591e395cb6a0934c8d7bee6aa` | 199 | Y |
| 16. | `docs/testing/BBD-WAL-009-NATIVE-REVIEW-PREREQUISITE-02.md` | `33d488dfcc88eb684d316bcffc547c051be412984d5633c54da76c1cef19dbe2` | 181 | `33d488dfcc88eb684d316bcffc547c051be412984d5633c54da76c1cef19dbe2` | 181 | Y |
| 17. | `docs/testing/BBD-WAL-009-SIGNATURE-CONTEXT-EXPECTED-RED-01.md` | `79b6e97f5a6a4ca4903d9db7b40233ecf0f2ca931116e0d232853f1b842d1ff2` | 216 | `79b6e97f5a6a4ca4903d9db7b40233ecf0f2ca931116e0d232853f1b842d1ff2` | 216 | Y |

Start count: 17 paths. End count: 17 paths.
Starting git status: 17 paths (10 modified + 7 untracked, including four docs/testing evidence records).
Final git status: 18 paths (10 modified + 8 untracked, including five docs/testing evidence records
  after adding this evidence file).
End measurement trailing assertion: `ALL_MATCH: True`.

## Mutated and restored identities

| Stage | Action | Lines (newline) | SHA-256 |
| --- | --- | ---: | --- |
| 2 | Mutated (message 77669) | 929 | `e21eb580d998b483ad75bb13454b8f9a2208fc6b2a629fdcb7785a9bc3c944f7` |
| 2 | Restored (message 77675) | 929 | `bd51b3d0ee2f748fa31e483a2747c157ff20544aa62e3e00a7929c77060b7361` |
| 3 | Mutated (message 77681) | 931 | `b50c59b68cdd06bf318a396d699eaa9f64d0a296e22af60bf5079c6683f80fbc` |
| 3 | Restored (message 77689) | 929 | `bd51b3d0ee2f748fa31e483a2747c157ff20544aa62e3e00a7929c77060b7361` |

## Reviewer errata (applied per BBD-WAL-009-SIGNATURE-CONTEXT-VALIDATION-01-REVIEW.md)

The following corrections are applied to this reconstruction:

1. **Starting/final path counts corrected**: starting git status shows 17 paths (10 modified + 7
   untracked, including four docs/testing evidence records). Final git status shows 18 paths (10
   modified + 8 untracked, including five docs/testing evidence records after adding this file).
   The original rejected artifact conflated these and stated 'five' docs/testing records at start.

2. **Filesystem stat corrected**: `stat` reported ext2/ext3 family label (message 77649). The
   reviewer previously confirmed the underlying filesystem is ext4. The original artifact's
   phrasing 'reviewer confirms this is not a different filesystem type' is replaced with the
   factual stat label and prior reviewer confirmation. No independent tmpfs or Cargo
   target-override check was recorded in saved data.

3. **No source body reads**: no source files were read for this reconstruction; attribution of
   test function names, panic sites, and mutation locations derives solely from saved test
   output (messages 77662, 77671, 77683, 77685, 77693) and measurement JSON (messages 77652, 77702).

4. **Path normalization corrected**: repository-relative `wallet-broker/target/debug/deps/`
   paths in test output are preserved verbatim, not relabeled as cargo-registry paths.
   Only the literal local prefix `<repo>/` is normalized to
   `<repo>/`, and `<home>/.hermes/hermes-agent` to `<home>/.hermes/hermes-agent`.

5. **Backtrace note corrected**: falsification notes read `display a backtrace` (matching saved
   output), not `execute a backtrace` as in the rejected artifact.

6. **Thread names and IDs preserved**: all integration thread names and IDs are rendered
   verbatim from saved output (message 77693), with no invented prefixes and no removed IDs.

7. **Final cargo error included**: the complete integration output retains the final
   `error: test failed, to rerun pass --test zec_sign_verify` line directing a zec_sign_verify rerun.

8. **Speculative root-cause removed**: the INTENT_MISMATCH and SIGNATURE_INVALID errors are
   reported as authoritative in saved result 77693 without speculative native-library or
   fixture root-cause attribution. Failure-phase triage remains pending separate scoping.

9. **Full before/after table included**: all seventeen paths shown with both starting and ending
   SHA-256 and newline counts, with restoration confirmation column.

10. **Correction runtime recorded separately**: original runtime from session
    `20260908_082758_10ad42` / message 77645; correction runtime from session
    `20260908_090052_c07851`.

11. **Procedural deviations documented**: the original run also performed an unrequested rustc
    version probe and Git diff inspection after the integration stop, opened the Hermes
    database without URI read-only mode, enumerated table schemas, and queried metadata for
    five recent sessions instead of only the supplied session. These deviations are recorded
    as errata and do not affect the five verified gate executions or the two source restorations.

12. **Redundant final checks noted**: later automatic verification prompts led to redundant
    source hash/search and Git-diff checks in the original run. Full preflight compliance is
    not claimed for the original run; the correction itself ran only `hermes --version`,
    `git rev-parse HEAD`, `git status --short --untracked-files=all`, and
    `git diff --cached --name-only`.

## Validation summary

Five commands executed from saved data:
- Focused context green: 4 passed, 0 failed (exit 0)
- Transparent-rejection falsification: 0 passed, 1 failed at line 206 (exit 101)
- Message-corruption falsification (PCZT oracle): 0 passed, 1 failed at line 264 (exit 101)
- Message-corruption falsification (signature check): 0 passed, 1 failed at line 353 (exit 101)
- Broader integration: zec_prepare 11 passed/0 failed; zec_sign_verify 6 passed/8 failed (exit 101)

Both mutations were exactly restored (messages 77675, 77689). All seventeen starting
identities match the ending measurement (ALL_MATCH: True).

This is partial validation: focused context and falsifications verified, prepare passed,
sign/verify blocked by 8 failures, restored full-library green outstanding. The full
library/restored focused green was not run after the 8-failure integration stop.

## Final measurement (this file)

Per the correction handoff, the evidence's own changing hash is not embedded inside itself.
The file's identity is computed externally after write. This file is the sole new record
added to the 17-path starting set (17 -> 18 total paths in final git status).
