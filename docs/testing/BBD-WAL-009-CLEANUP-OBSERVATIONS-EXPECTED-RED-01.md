# WAL-009 cleanup observations expected red 01

Integration note (2026-09-09): local repository and home prefixes are rendered as <repo> and <home>. Earlier hashes/counts describe execution-time files. This is an archival evidence record; acceptance limits and reported deviations remain governed by the corresponding reviewer handoffs. No test was repeated for this source checkpoint.


Reviewer: Codex, High.
Actor: Jr Dev Hermes, one test execution and one evidence record only.
Authorized by: CURRENT_TASK.md (BBD-WAL-009), baseline c9d701ec; source accepted for expected-red execution.
Runtime identity — launched once from e612c5cc via `hermes -z --pass-session-id`; outer terminal 29552.

## Runtime identity

- hermes --version: `Hermes Agent v0.18.2 (2026.7.7.2) · upstream b1f003e1 · local 10b6d1a9 (+1 carried commit)`
- HERMES_SESSION_ID: `20260908_203457_d46097`
- Provider/model: unavailable (not surfaced by this Hermes build; reviewer may verify the completed-session database).

## Source lineage

- Source under test: wallet-broker/tests/zec_sign_verify.rs — 1215 lines, SHA-256 c95fa9ae836ca7151f35ef92e664dbc569f24b54b1ff4fb8ad78c1fce997a787.
- Original 1118-line prefix confirmed: SHA-256 7a481d3a954a92e04d324be047863a824ecf303bfd6232be70e9d13d3a295987.
- New suffix: exactly 97 lines, one test: signer_failure_does_not_report_cleanup_for_unreached_secret_classes.
- All other 30 frozen identities match (see preflight and postflight inventory below).

## Preflight inventory (31-row frozen-identity check, run once before execution)

Command (exact inline read-only, workdir = repository root):

```
"$HOME/.cargo/bin/rustup" run 1.98.0 python3 - <<'PY_INVENTORY'
from pathlib import Path
import hashlib
import re
handoff = Path('docs/handoff/HERMES_BBD_WAL_009_CLEANUP_OBSERVATIONS_EXPECTED_RED_01.md')
rows = re.findall(r'^\| ([^|]+) \| (\d+) \| ([0-9a-f]{64}) \|$', handoff.read_text(), re.M)
assert len(rows) == 31, len(rows)
failed = False
for name, expected_lines, expected_hash in rows:
    data = Path(name).read_bytes()
    actual_lines = data.count(b'\n')
    actual_hash = hashlib.sha256(data).hexdigest()
    matches = actual_lines == int(expected_lines) and actual_hash == expected_hash
    print(name, actual_lines, actual_hash, 'MATCH' if matches else 'MISMATCH')
    failed = failed or not matches
raise SystemExit(1 if failed else 0)
PY_INVENTORY
```

Result: exit 0 — all 31 rows MATCH (lines + SHA-256).

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

## Execution

Exact command (run once, in terminal background, complete process.log retained):

```
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_sign_verify signer_failure_does_not_report_cleanup_for_unreached_secret_classes -- --exact
```

- Process/session: background proc_b7d883627e69, PID 3500044.
- Exit code: 101.
- Compilation: finished in 2.63s (3 dead_code warnings, unrelated, pre-existing).
- Test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 14 filtered out; finished in 1.87s.
- Panic location: wallet-broker/tests/zec_sign_verify.rs:1211:5.

## Final assertion output (exact)

```
thread 'signer_failure_does_not_report_cleanup_for_unreached_secret_classes' (3500200) panicked at tests/zec_sign_verify.rs:1211:5:
forbidden cleanup observations: ["ProofWorkspace/SignerError/touch_count/1", "ProofWorkspace/SignerError/positive_wipe_count/1", "ExtractedTransaction/SignerError/touch_count/1", "ExtractedTransaction/SignerError/positive_wipe_count/1"]
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

### Expected forbidden entries — confirmed present

```
ProofWorkspace/SignerError/touch_count/1
ProofWorkspace/SignerError/positive_wipe_count/1
ExtractedTransaction/SignerError/touch_count/1
ExtractedTransaction/SignerError/positive_wipe_count/1
```

The real-signer and positive-Seed guards passed (the panic is the final assertion, not an earlier guard or crypto failure). The four observations are spurious because prover/extractor never ran. No compile, setup, earlier guard, or crypto failure occurred.

## Complete saved process.log (46 lines, no local absolute-path normalization applied)

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
  --> src/zec/spend.rs:868:15
   |
868 | pub(crate) fn capture_trusted_verification_authority(
   |               ^^^^^^^^^^^^^^^^^^^^^^^

warning: function `timestamp_for_test` is never used
  --> src/zec/test_support.rs:1532:4
   |
1532 | fn timestamp_for_test(value: &str) -> Result<u64, ZecError> {
   |     ^^^^^^^^^^^^^^^^^^

warning: `bitbook-wallet-broker` (lib) generated 3 warnings
   Compiling bitbook-wallet-broker v0.1.0 (<repo>/wallet-broker)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.63s
     Running tests/zec_sign_verify.rs (wallet-broker/target/debug/deps/zec_sign_verify-4c279ff0e3acb2a4)

running 1 test
test signer_failure_does_not_report_cleanup_for_unreached_secret_classes ... FAILED

failures:

---- signer_failure_does_not_report_cleanup_for_unreached_secret_classes stdout ----

thread 'signer_failure_does_not_report_cleanup_for_unreached_secret_classes' (3500200) panicked at tests/zec_sign_verify.rs:1211:5:
forbidden cleanup observations: ["ProofWorkspace/SignerError/touch_count/1", "ProofWorkspace/SignerError/positive_wipe_count/1", "ExtractedTransaction/SignerError/touch_count/1", "ExtractedTransaction/SignerError/positive_wipe_count/1"]
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    signer_failure_does_not_report_cleanup_for_unreached_secret_classes

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 14 filtered out; finished in 1.87s

error: test failed, to rerun pass `--test zec_sign_verify`
```

## Post-execution inventory (31-row frozen-identity check, run once after completion)

Command: identical to preflight.

Result: exit 0 — all 31 rows MATCH (lines + SHA-256 identical to preflight). No source or test mutation occurred.

## Deviations

None. No local absolute-path normalization was applied to the saved process.log output because the contract requires preservation of output bytes and indentation; only the evidence record's narrative normalizes repo-relative paths. Provider/model metadata was unavailable from this Hermes build and is left unavailable for reviewer lookup rather than invented.

## Final stop

No source edit, fault injection, backup, helper/log file beyond this evidence record, Git, broader test, formatter/compiler probe, retry, config/environment discovery, actor launch, or report-only correction performed. Stopping after evidence-file SHA-256 and newline count below.
