# WAL-009 native review prerequisite check 02 — evidence record

Integration note (2026-09-09): local repository and home prefixes are rendered as <repo> and <home>. Earlier hashes/counts describe execution-time files. This is an archival evidence record; acceptance limits and reported deviations remain governed by the corresponding reviewer handoffs. No test was repeated for this source checkpoint.


Actor: Hermes Jr Dev.
Original governance parent: e5418536 ("docs: accept WAL-009 delimiter and authorize fresh prerequisite check").
This correction governance parent: 09cc3d5b ("docs: scope WAL-009 verification repairs at XHigh").
Correction session: 20260907_222229_2bc84d.
Original session: 20260907_220511_907287, outer session 25473.

## Runtime and session identity

- Original runtime: Hermes Agent v0.18.2 (2026.7.7.2) · upstream 2237be35 · local 10b6d1a9 (+1 carried commit). Provider `nous`, model `poolside/laguna-s-2.1:free`, as recorded in session 20260907_220511_907287. This is the original execution's actual model and billing provider, not routing-policy adoption state.
- HEAD at original execution: 1a36d1d0a222376bf14b00d449e024ca8a93b285.
- HEAD line: "docs: checkpoint Hermes native prerequisite check 02".
- Staged index: empty.
- Dirty/untracked set (14 paths): package.json, package-lock.json, wallet-broker/Cargo.lock, wallet-broker/Cargo.toml, wallet-broker/src/native.rs, wallet-broker/src/native_ui.rs, wallet-broker/src/zec.rs, wallet-broker/src/zec/prepare.rs, wallet-broker/src/zec/store.rs, wallet-broker/src/zec/test_support.rs, docs/testing/BBD-WAL-009-LOCK-SYNC-01.md, docs/testing/BBD-WAL-009-NATIVE-REVIEW-EXPECTED-RED-01.md, wallet-broker/src/native_ui/zec_review_tests.rs, wallet-broker/src/zec/spend.rs.
- Untracked directory expansion: the only untracked directory was `wallet-broker/src/native_ui/`, expanded read-only and found to contain only `zec_review_tests.rs`.
- Build target: existing `wallet-broker/target` on repository ext4 (disk-backed); `/tmp` is tmpfs and was not used for artifacts.

## Exact submitted command (one tool result)

```text
<home>/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --features native-ui --lib native_ui::zec_review_tests
```

Submitted exactly once as the entire terminal command. No wrappers, pipelines, shell operators, environment prefixes, or echo were added. No rerun, no concurrent acceptance tool.

Tool exit status: 101 (cargo test compilation failure).

## Captured diagnostics (from original transcript message 77521)

Paths normalized from local prefix `<repo>/wallet-broker/` to `src/`; contents otherwise preserved verbatim. Compiler-suggested repair text is quoted as-is and attributed to the compiler; no repair was applied.

error[E0432]: unresolved imports `super::ZecReviewControls`, `super::ZecReviewDialog`
 --> src/native_ui/zec_review_tests.rs:5:13
  |
5 | use super::{ZecReviewControls, ZecReviewDialog};
  |             ^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^ no `ZecReviewDialog` in `native_ui`
  |             |
  |             no `ZecReviewControls` in `native_ui`

error[E0277]: the trait bound `Authorized: TransparentAuthorizingContext` is not satisfied
   --> src/zec/spend.rs:517:19
    |
517 |     let sighash = signature_hash(transaction, &SignableInput::Shielded, &txid_parts);
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `zcash_transparent::sighash::TransparentAuthorizingContext` is not implemented for `zcash_transparent::bundle::Authorized`
    |
help: the following other types implement trait `zcash_transparent::sighash::TransparentAuthorizingContext`
   --> <home>/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zcash_transparent-0.10.0/src/builder.rs:340:1
    |
340 | impl TransparentAuthorizingContext for Coinbase {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `zcash_transparent::builder::Coinbase`
...
594 | impl TransparentAuthorizingContext for Unauthorized {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `zcash_transparent::builder::Unauthorized`
    |
   ::: <home>/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zcash_transparent-0.10.0/src/bundle.rs:41:1
    |
 41 | impl TransparentAuthorizingContext for EffectsOnly {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `zcash_transparent::bundle::EffectsOnly`
    |
   ::: <home>/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zcash_transparent-0.10.0/src/pczt/tx_extractor.rs:112:1
    |
112 | impl TransparentAuthorizingContext for Unbound {
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `zcash_transparent::pczt::tx_extractor::Unbound`
note: required by a bound in `signature_hash`
   --> <home>/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zcash_primitives-0.30.1/src/transaction/sighash.rs:38:9
    |
 37 | pub fn signature_hash<
    |        -------------- required by a bound in this function
 38 |     TA: ::transparent::sighash::TransparentAuthorizingContext,
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `signature_hash`
    |         note: the full name for the type has been written to '<repo>/wallet-broker/target/debug/deps/bitbook_wallet_broker-6e59b086531f1b58.long-type-6584072649249278528.txt'
    = note: consider using `--verbose` to print the full type name to the console

error[E0599]: no method named `cloned` found for reference `&std::option::Option<std::vec::Vec<u8>>` in the current scope
   --> src/zec/spend.rs:766:10
    |
763 |       let proof = pczt
    |  _________________-
764 | |         .ironwood()
765 | |         .zkproof()
766 | |         .cloned()
    | |         -^^^^^^ `&std::option::Option<std::vec::Vec<u8>>` is not an iterator
    | |_________|
    |
    |
help: call `.into_iter()` first
    |
766 |         .into_iter().cloned()
    |          ++++++++++++

error[E0308]: mismatched types
    --> src/zec/test_support.rs:1326:36
     |
1326 |         return wipe_exit_for_fault(fault);
     |                ------------------- ^^^^^ expected `FaultPoint`, found `PipelineFault`
     |                |
     |                arguments to this function are incorrect
     |
note: function defined here
    --> src/zec/test_support.rs:1492:4
     |
1492 | fn wipe_exit_for_fault(fault: FaultPoint) -> WipeExit {
     |    ^^^^^^^^^^^^^^^^^ ^ -----------------

error[E0599]: no method named `run` found for struct `eframe::egui::Context` in the current scope
   --> src/native_ui.rs:117:21
    |
117 |         let _ = ctx.run(egui::RawInput::default(), |ctx| {
    |                 ----^^^
    |
help: there is a method `run_ui` with a similar name
    |
117 |         let _ = ctx.run_ui(egui::RawInput::default(), |ctx| {
    |                        +++

error[E0515]: cannot return value referencing function parameter `value`
   --> src/zec/spend.rs:669:56
    |
669 |             if action.output().recipient().map(|value| value.as_slice())
    |                                                        -----^^^^^^^^^^^
    |                                                        |
    |                                                        returns a value referencing data owned by the current function
    |                                                        `value` is borrowed here

warning: variable does not need to be mutable
    --> src/zec/test_support.rs:1131:9
     |
1131 |         mut outcome: PipelineOutcome,
     |         ----^^^^^^^
     |         |
     |         help: remove this `mut`
     |
     = note: `#[warn(unused_mut)]` (part of `#[warn(unused)]`) on by default

Some errors have detailed explanations: E0277, E0308, E0432, E0515, E0599.
For more information about an error, try `rustc --explain E0277`.
warning: `bitbook-wallet-broker` (lib test) generated 1 warning
error: could not compile `bitbook-wallet-broker` (lib test) due to 6 previous errors; 1 warning emitted

## Declared tests versus actual executed count

- Declared in module src/native_ui/zec_review_tests.rs (233 lines): 6 test items per the handoff.
- Actual executed: 0 — compilation failed before any test binary ran; zero tests executed.

## Classification

Per the handoff decision tree: this is NOT the intended missing-private-contract red, and it is NOT a dependency-resolution failure. It is a MIXED / PREREQUISITE-BLOCKED outcome — the missing ZecReviewDialog/ZecReviewControls contract diagnostic (E0432, item above) appears alongside unrelated source-compile diagnostics: eframe `Context::run`/`run_ui` incompatibility (E0599), zcash_transparent `Authorized: TransparentAuthorizingContext` trait-bound error (E0277), `Option::cloned`/`into_iter` API mismatch (E0599), `FaultPoint`/`PipelineFault` type mismatch (E0308), return-borrow lifetime error (E0515), and an unused-mut warning. The presence of unrelated diagnostics disqualifies this from the intended absent-contract red. The source was not repaired; the repaired two-byte delimiter suffix in test_support.rs was compiled as-is, untouched.

## Before/after hashes (all 14 paths, full values)

Before and after are identical — all 14 paths preserved their bytes. Hashes recovered verbatim from original transcript messages 77518 (before) and 77523 (after), parsed from the terminal tool JSON content field. No hash was retyped or inferred.

| Path | Before | After | Match |
| --- | --- | --- | --- |
| wallet-broker/src/zec/test_support.rs | 461fdd070318cc5f31a29af2cdceaab1d0b63dc23473641b61ea9aafc145cd1b | 461fdd070318cc5f31a29af2cdceaab1d0b63dc23473641b61ea9aafc145cd1b | yes |
| wallet-broker/src/native_ui.rs | 600e2fb34134a276d31da9c5e746cc6478337bae73590fd742cf2d36ff785524 | 600e2fb34134a276d31da9c5e746cc6478337bae73590fd742cf2d36ff785524 | yes |
| wallet-broker/src/native_ui/zec_review_tests.rs | 2eec3b60eb1b9fbd32235832a867d1189928126ca79373fa9f4fa7fdf2ee87bf | 2eec3b60eb1b9fbd32235832a867d1189928126ca79373fa9f4fa7fdf2ee87bf | yes |
| wallet-broker/src/native.rs | 992138f18cbb0b969296109de3186aab923500c41ed2c0d41cf9c0696d3c3acc | 992138f18cbb0b969296109de3186aab923500c41ed2c0d41cf9c0696d3c3acc | yes |
| wallet-broker/Cargo.toml | 73e5e585eb2fd1ca867d66962460cfa010443886a4b59d8a33556ef2a4ff3503 | 73e5e585eb2fd1ca867d66962460cfa010443886a4b59d8a33556ef2a4ff3503 | yes |
| wallet-broker/Cargo.lock | b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71 | b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71 | yes |
| package.json | 84b30b6860441a100588b5bdb92b37ddc45fb7610d48ee6f0e51c30ea0717780 | 84b30b6860441a100588b5bdb92b37ddc45fb7610d48ee6f0e51c30ea0717780 | yes |
| package-lock.json | 5e1122f32b0db42eb4386d4d0f0c47a4160d23cb4ae790bbb3600b5d3a5bddfc | 5e1122f32b0db42eb4386d4d0f0c47a4160d23cb4ae790bbb3600b5d3a5bddfc | yes |
| wallet-broker/src/zec.rs | 045cdc51f26ac8b9b1283cee5f995d60ceda38a25577aa7a937f5d07f177b90b | 045cdc51f26ac8b9b1283cee5f995d60ceda38a25577aa7a937f5d07f177b90b | yes |
| wallet-broker/src/zec/prepare.rs | 44c0783fa3d75867599092d25912032b665838ed7a73c81a16b0eec2b26ffb07 | 44c0783fa3d75867599092d25912032b665838ed7a73c81a16b0eec2b26ffb07 | yes |
| wallet-broker/src/zec/store.rs | 531d0a6171ecd9b5012602ccb870f16eab96888c809b3ffdf4e6303576f17c90 | 531d0a6171ecd9b5012602ccb870f16eab96888c809b3ffdf4e6303576f17c90 | yes |
| wallet-broker/src/zec/spend.rs | ae665e4742925d88e1f7cf6a66172e89a268f613a5fd0efc0b3cc3581ac4cdf4 | ae665e4742925d88e1f7cf6a66172e89a268f613a5fd0efc0b3cc3581ac4cdf4 | yes |
| docs/testing/BBD-WAL-009-LOCK-SYNC-01.md | ab25ffd2ac6ec7c26f7c21e42978697ec10da7ff395a9569afd9e78d1cd27ed2 | ab25ffd2ac6ec7c26f7c21e42978697ec10da7ff395a9569afd9e78d1cd27ed2 | yes |
| docs/testing/BBD-WAL-009-NATIVE-REVIEW-EXPECTED-RED-01.md | 42825e83ba6e98471c18b0c63d246e9a8790c25591e395cb6a0934c8d7bee6aa | 42825e83ba6e98471c18b0c63d246e9a8790c25591e395cb6a0934c8d7bee6aa | yes |

## Deviations

The gate itself is exact and single: the one command above was submitted exactly once with no wrappers, pipelines, shell operators, environment prefixes, or echo, and was not rerun. No formatter, cargo check, lint, other test, native launch, dependency operation, source mutation, or Git operation occurred.

The single deviation is an unnecessary initial read-only `git log` performed during preflight for repository-state confirmation; it was not a gate rerun and had no effect on the working tree.

## Correction attribution

This correction was performed by Hermes Jr Dev, session 20260907_222229_2bc84d (model poolside/laguna-s-2.1:free, provider nous). All original execution data — runtime identity, HEAD, command, exit 101, six errors/one warning, six declared tests/zero executed, mixed/prerequisite-blocked classification, and all fourteen before/after hashes — was recovered verbatim from the original session transcript messages 77518, 77520, 77521, and 77523 in session 20260907_220511_907287. No compiler, Cargo, rustup, formatter, test, lint, audit, scanner, source edit, network call, or other actor was invoked for this correction. No other record was modified.

Status: CORRECTION COMPLETE — evidence file rewritten in place, original evidence and source preserved, bytes unchanged, nothing staged, no further gates or actors authorized. Stop.
