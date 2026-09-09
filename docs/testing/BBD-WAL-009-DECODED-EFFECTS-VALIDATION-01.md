# WAL-009 decoded effects validation 01

Integration note (2026-09-09): local repository and home prefixes are rendered as <repo> and <home>. Earlier hashes/counts describe execution-time files. This is an archival evidence record; acceptance limits and reported deviations remain governed by the corresponding reviewer handoffs. No test was repeated for this source checkpoint.


Reviewer: Codex, XHigh. Actor: Jr Dev — Hermes, execution/evidence only.

## Runtime identity

| Field | Value |
|---|---|
| Hermes version | v0.18.2 (2026.7.7.2), upstream b1f003e1, local 10b6d1a9 |
| Provider | nous |
| Model | poolside/laguna-s-2.1:free |
| Hermes runtime session ID | HERMES_SESSION_ID=20260908_192320_041a4d |
| Process PID (stage 1) | 3472126 |
| Process session ID (stage 1) | proc_1267eab54939 |

## Pre-execution inventory measurement

All 29 frozen inventory rows were parsed directly from the handoff table in
HERMES_BBD_WAL_009_DECODED_EFFECTS_VALIDATION_01.md and measured (byte SHA-256 and
line count). All 29 matched their frozen identities. The measurement script used:
`python3 measure_inventory.py` (repo root). Results: 29/29 OK, 0 mismatches.

## Stage 1 — Regression green (expected exit 0, one passed, 6 filtered)

**Command (exact):**

```
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --lib zec::spend::verification_context_tests::decoded_effects_reject_agreeing_metadata_that_disagrees_with_signed_bytes -- --exact
```

**Actual outcome: exit 101 (unexpected; handoff expected exit 0).**

The command did not produce a test result. Compilation of
`bitbook-wallet-broker` (lib test) failed with a single error:

```
error[E0277]: `?` couldn't convert the error to `ZecError`
   --> src/zec/spend/effects.rs:279:48
    |
184 | ) -> Result<RecoveredEffects, ZecError> {
    |      ---------------------------------- expected `ZecError` because of this
...
278 |     let retained_input = Zatoshis::from_u64(authority.retained_spend().value_zat)
    |                          -------------------------------------------------------- this has type `Result<_, zcash_protocol::value::BalanceError>`
279 |         .map_err(|_| ZecError::intent_mismatch)?;
    |          --------------------------------------^ the trait `From<fn() -> ZecError {ZecError::intent_mismatch}>` is not implemented for `ZecError`
    |          |
    |          this can't be annotated with `?` because it has type `Result<_, fn() -> ZecError {ZecError::intent_mismatch}>`
    |
note: `ZecError` needs to implement `From<fn() -> ZecError {ZecError::intent_mismatch}>`
   --> src/zec.rs:158:1
    |
158 | pub struct ZecError {
    | ^^^^^^^^^^^^^^^^^^^^
    = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait

error: could not compile `bitbook-wallet-broker` (lib test) due to 1 previous error
```

**Root cause analysis (for evidence; no repair attempted):**

At `wallet-broker/src/zec/spend/effects.rs:279`, the expression
`ZecError::intent_mismatch` is referenced without parentheses.
`intent_mismatch` is a function defined at `wallet-broker/src/zec.rs:234`
as `pub(crate) fn intent_mismatch() -> Self`. Without the call parentheses `()`,
the compiler treats `ZecError::intent_mismatch` as a function item value of
type `fn() -> ZecError`, not a `ZecError` instance. The `.map_err(|_| ...)`
closure therefore yields `Result<_, fn() -> ZecError>` rather than
`Result<_, ZecError>`, and the `?` operator cannot convert the function
pointer into `ZecError`.

For comparison, the two adjacent lines in the same function use the correct
form with parentheses:
- effects.rs:272: `.map_err(|_| ZecError::intent_mismatch())?;`
- effects.rs:274: `.map_err(|_| ZecError::intent_mismatch())?;`

Line 279 is the sole occurrence omitting the call parentheses.

## Stop and no-further-stages

The handoff authorizes exactly six stages, run in order, once each, and instructs:
"Stop at the first unexpected outcome, no repair or retry." and "Do not run later
stages after a failure." Stage 1 produced an unexpected exit 101 (compile error).
Per this instruction, Stages 2–6 (falsification, restored library green, software
integration, synthetic integration, native compile) were **not executed**.

Stage 2's falsification requires temporarily editing effects.rs and then restoring
it. Because Stage 1 did not pass, Stage 2 was not entered, and therefore no
temporary mutation or backup was staged. The backup file
`wallet-broker/target/bbd-wal009-effects-validation-original.rs` was **not created**.

## Post-stop inventory measurement

After the Stage 1 process exited, the 29-file inventory was measured again with the
same script. All 29 files still match their frozen identities (0 mismatches).
No source was modified during execution — the cargo test command compiles but does
not write to source files. The effects.rs file retains its accepted identity:

| File | Lines | SHA-256 |
|---|---|---|
| wallet-broker/src/zec/spend/effects.rs | 379 | bd62a867ba8c135517df6858aff9eb97aaa4e4c30c76e22c93a27b622412fd56 |

## Deviation disclosure

1. **Stage 1 failed at compile time (exit 101) instead of the expected exit 0.**
   The accepted four-file production drop contains a compile error in
   `effects.rs:279` — `ZecError::intent_mismatch` is used as a function reference
   rather than a call `ZecError::intent_mismatch()`. This prevents any test from
   executing. No repair was attempted per the handoff instruction to stop with no
   repair or retry.

2. **No backup file was created.** Because Stage 1 did not pass, Stage 2 was never
   entered, and the named backup
   `wallet-broker/target/bbd-wal009-effects-validation-original.rs` was not staged.

3. **Stages 2–6 were not executed.** Per the handoff: "Do not run later stages after
   a failure."

4. **No Git commands were run.** Per the active handoff: "No Git commands
   (including read-only)."

5. **No formatters, Clippy, or broader suites were run.** Per the handoff scope:
   "No formatter, Clippy, no-run probe, broader suite, live wallet, network,
   broadcast, mainnet, hardware transport, Monero, actor or subagent."

## Evidence file reference

- Pre-execution measurement script: `measure_inventory.py` (repo root)
- Full Stage 1 process output (saved directly from tool JSON): 23 lines

```
   Compiling bitbook-wallet-broker v0.1.0 (<repo>/wallet-broker)
error[E0277]: `?` couldn't convert the error to `ZecError`
   --> src/zec/spend/effects.rs:279:48
    |
184 | ) -> Result<RecoveredEffects, ZecError> {
    |      ---------------------------------- expected `ZecError` because of this
...
278 |     let retained_input = Zatoshis::from_u64(authority.retained_spend().value_zat)
    |                          -------------------------------------------------------- this has type `Result<_, zcash_protocol::value::BalanceError>`
279 |         .map_err(|_| ZecError::intent_mismatch)?;
    |          --------------------------------------^ the trait `From<fn() -> ZecError {ZecError::intent_mismatch}>` is not implemented for `ZecError`
    |          |
    |          this can't be annotated with `?` because it has type `Result<_, fn() -> ZecError {ZecError::intent_mismatch}>`
    |
note: `ZecError` needs to implement `From<fn() -> ZecError {ZecError::intent_mismatch}>`
   --> src/zec.rs:158:1
    |
158 | pub struct ZecError {
    | ^^^^^^^^^^^^^^^^^^^^
    = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait

For more information about this error, try `rustc --explain E0277`.
error: could not compile `bitbook-wallet-broker` (lib test) due to 1 previous error
```

## Summary

Stage 1 (regression green) was run exactly once in the background. The process
compiled the accepted effects.rs and failed at line 279 with `E0277` because
`ZecError::intent_mismatch` is referenced as a function value rather than called
with `()`. Exit code was 101 (expected 0). No test executed. Per handoff
instructions, all subsequent stages were not run. All 29 frozen inventory
identities were verified before and after execution with zero mismatches. No
source, test, backup, prior evidence, governance, or Git mutation occurred.
