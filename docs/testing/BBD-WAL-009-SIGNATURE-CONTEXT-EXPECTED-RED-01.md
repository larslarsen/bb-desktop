# BBD-WAL-009 signature-context expected red 01 — execution evidence

Integration note (2026-09-09): local repository and home prefixes are rendered as <repo> and <home>. Earlier hashes/counts describe execution-time files. This is an archival evidence record; acceptance limits and reported deviations remain governed by the corresponding reviewer handoffs. No test was repeated for this source checkpoint.


Actor: Hermes Jr Dev. Governance parent: commit bf39113b.
Source: Grok corrected four-test module, accepted by reviewer.
Result classified per handoff section 1.

## Runtime and session identity

- Hermes version: Hermes Agent v0.18.2 (2026.7.7.2) · upstream 2237be35 · local 10b6d1a9 (+1 carried commit)
Install directory: <home>/.hermes/hermes-agent
Install method: git
Python: 3.11.15
OpenAI SDK: 2.24.0
- Session ID: 20260907_232730_9fdc2a
- Provider: nous
- Model: poolside/laguna-s-2.1:free
- Governance parent: 67f22e0f82ccd19ed645b9d8206bd9b739a17e1e

## Preflight state (before command)

- Repository filesystem type: ext2/ext3
- Build-target filesystem type: ext2/ext3
- /tmp filesystem type: tmpfs (tmpfs, not used for build artifacts)
- Evidence file (docs/testing/BBD-WAL-009-SIGNATURE-CONTEXT-EXPECTED-RED-01.md) exists before: False
- Staged index (git diff --cached --name-only): empty

Pre-command git status --short --untracked-files=all:

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
?? docs/testing/BBD-WAL-009-LOCK-SYNC-01.md
?? docs/testing/BBD-WAL-009-NATIVE-REVIEW-EXPECTED-RED-01.md
?? docs/testing/BBD-WAL-009-NATIVE-REVIEW-PREREQUISITE-02.md
?? wallet-broker/src/native_ui/zec_review_tests.rs
?? wallet-broker/src/zec/spend.rs
?? wallet-broker/src/zec/spend/verification_context_tests.rs
```

## Exact submitted command

Submitted as the entire terminal command, exactly once, with no shell operators, pipeline, redirection, environment prefix, wrapper, echo, or version probe added:

```text
<home>/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --lib zec::spend::verification_context_tests
```

## Tool exit code

101

## Complete compiler output

Compiler path-prefix tokens normalized:
- `<repo>/` replaces the local repository root prefix `<repo>`
- `<cargo-registry>/` replaces the local Cargo registry prefix `<home>/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/`
- All other bytes of diagnostic text, including whitespace, are preserved exactly.

```
Compiling bitbook-wallet-broker v0.1.0 (<repo>/wallet-broker)
error[E0432]: unresolved import `super::ShieldedVerificationContext`
  --> src/zec/spend/verification_context_tests.rs:23:5
   |
23 | use super::ShieldedVerificationContext;
   |     ^^^^^^^---------------------------
   |            |
   |            no `ShieldedVerificationContext` in `zec::spend`

error[E0277]: the trait bound `Authorized: TransparentAuthorizingContext` is not satisfied
   --> src/zec/spend.rs:517:19
    |
517 |     let sighash = signature_hash(transaction, &SignableInput::Shielded, &txid_parts);
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `zcash_transparent::sighash::TransparentAuthorizingContext` is not implemented for `zcash_transparent::bundle::Authorized`
    |
help: the following other types implement trait `zcash_transparent::sighash::TransparentAuthorizingContext`
   --> <cargo-registry>/zcash_transparent-0.10.0/src/builder.rs:340:1
    |
340 | impl TransparentAuthorizingContext for Coinbase {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `zcash_transparent::builder::Coinbase`
...
594 | impl TransparentAuthorizingContext for Unauthorized {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `zcash_transparent::builder::Unauthorized`
    |
   ::: <cargo-registry>/zcash_transparent-0.10.0/src/bundle.rs:41:1
    |
 41 | impl TransparentAuthorizingContext for EffectsOnly {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `zcash_transparent::bundle::EffectsOnly`
    |
   ::: <cargo-registry>/zcash_transparent-0.10.0/src/pczt/tx_extractor.rs:112:1
    |
112 | impl TransparentAuthorizingContext for Unbound {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `zcash_transparent::pczt::tx_extractor::Unbound`
note: required by a bound in `signature_hash`
   --> <cargo-registry>/zcash_primitives-0.30.1/src/transaction/sighash.rs:38:9
    |
 37 | pub fn signature_hash<
    |        -------------- required by a bound in this function
 38 |     TA: ::transparent::sighash::TransparentAuthorizingContext,
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `signature_hash`
    = note: the full name for the type has been written to '<repo>/wallet-broker/target/debug/deps/bitbook_wallet_broker-838b402781cf6bb9.long-type-15818760262692786830.txt'
    = note: consider using `--verbose` to print the full type name to the console

error[E0599]: no method named `cloned` found for reference `&std::option::Option<Vec<u8>>` in the current scope
   --> src/zec/spend.rs:766:10
    |
763 |       let proof = pczt
    |  _________________-
764 | |         .ironwood()
765 | |         .zkproof()
766 | |         .cloned()
    | |         -^^^^^^ `&std::option::Option<Vec<u8>>` is not an iterator
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
     |    ^^^^^^^^^^^^^^^^^^^ -----------------

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
error: could not compile `bitbook-wallet-broker` (lib test) due to 5 previous errors; 1 warning emitted
```

## Declared versus actual executed tests

- Declared tests (per accepted module): 4
- Actual executed tests: 0
- Classification: MIXED / PREREQUISITE-BLOCKED

Classification rationale: compilation failed before test runtime. The missing private
ShieldedVerificationContext contract (E0432) is present alongside existing
spend.rs authorizing-context/Option/lifetime errors (E0277, E0599, E0515) and a
test_support.rs fault-type error (E0308), plus the existing unused-mut warning.
This is not isolated behavioral red; zero tests executed.

Error summary:
- error[E0432]: unresolved import `super::ShieldedVerificationContext` (verification_context_tests.rs:23)
- error[E0277]: the trait bound `Authorized: TransparentAuthorizingContext` is not satisfied (spend.rs:517)
- error[E0599]: no method named `cloned` found for `&Option<Vec<u8>>` (spend.rs:766)
- error[E0308]: mismatched types — expected `FaultPoint`, found `PipelineFault` (test_support.rs:1326)
- error[E0515]: cannot return value referencing function parameter `value` (spend.rs:669)
- warning: variable does not need to be mutable (test_support.rs:1131)
- Final summary: 5 previous errors; 1 warning emitted

## All sixteen frozen-path measurements (before/after)

| Path | Lines (before) | SHA-256 (before) | Lines (after) | SHA-256 (after) | Match |
| --- | ---: | --- | ---: | --- | --- |
| wallet-broker/src/zec/test_support.rs | 4364 | `6822e458cc8475155e6c2d3bbdef592e1af2a339100f29b3e4f7359a027886ce` | 4364 | `6822e458cc8475155e6c2d3bbdef592e1af2a339100f29b3e4f7359a027886ce` | yes |
| wallet-broker/src/native_ui.rs | 201 | `600e2fb34134a276d31da9c5e746cc6478337bae73590fd742cf2d36ff785524` | 201 | `600e2fb34134a276d31da9c5e746cc6478337bae73590fd742cf2d36ff785524` | yes |
| wallet-broker/src/native_ui/zec_review_tests.rs | 233 | `2eec3b60eb1b9fbd32235832a867d1189928126ca79373fa9f4fa7fdf2ee87bf` | 233 | `2eec3b60eb1b9fbd32235832a867d1189928126ca79373fa9f4fa7fdf2ee87bf` | yes |
| wallet-broker/src/native.rs | 489 | `992138f18cbb0b969296109de3186aab923500c41ed2c0d41cf9c0696d3c3acc` | 489 | `992138f18cbb0b969296109de3186aab923500c41ed2c0d41cf9c0696d3c3acc` | yes |
| wallet-broker/Cargo.toml | 122 | `73e5e585eb2fd1ca867d66962460cfa010443886a4b59d8a33556ef2a4ff3503` | 122 | `73e5e585eb2fd1ca867d66962460cfa010443886a4b59d8a33556ef2a4ff3503` | yes |
| wallet-broker/Cargo.lock | 5395 | `b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71` | 5395 | `b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71` | yes |
| package.json | 42 | `84b30b6860441a100588b5bdb92b37ddc45fb7610d48ee6f0e51c30ea0717780` | 42 | `84b30b6860441a100588b5bdb92b37ddc45fb7610d48ee6f0e51c30ea0717780` | yes |
| package-lock.json | 396 | `5e1122f32b0db42eb4386d4d0f0c47a4160d23cb4ae790bbb3600b5d3a5bddfc` | 396 | `5e1122f32b0db42eb4386d4d0f0c47a4160d23cb4ae790bbb3600b5d3a5bddfc` | yes |
| wallet-broker/src/zec.rs | 274 | `045cdc51f26ac8b9b1283cee5f995d60ceda38a25577aa7a937f5d07f177b90b` | 274 | `045cdc51f26ac8b9b1283cee5f995d60ceda38a25577aa7a937f5d07f177b90b` | yes |
| wallet-broker/src/zec/prepare.rs | 1238 | `44c0783fa3d75867599092d25912032b665838ed7a73c81a16b0eec2b26ffb07` | 1238 | `44c0783fa3d75867599092d25912032b665838ed7a73c81a16b0eec2b26ffb07` | yes |
| wallet-broker/src/zec/store.rs | 2872 | `531d0a6171ecd9b5012602ccb870f16eab96888c809b3ffdf4e6303576f17c90` | 2872 | `531d0a6171ecd9b5012602ccb870f16eab96888c809b3ffdf4e6303576f17c90` | yes |
| wallet-broker/src/zec/spend.rs | 883 | `c646ea835eceac69f899dea111b19ae7fb3354ecc682b73b08b598138b0d5b0d` | 883 | `c646ea835eceac69f899dea111b19ae7fb3354ecc682b73b08b598138b0d5b0d` | yes |
| docs/testing/BBD-WAL-009-LOCK-SYNC-01.md | 120 | `ab25ffd2ac6ec7c26f7c21e42978697ec10da7ff395a9569afd9e78d1cd27ed2` | 120 | `ab25ffd2ac6ec7c26f7c21e42978697ec10da7ff395a9569afd9e78d1cd27ed2` | yes |
| docs/testing/BBD-WAL-009-NATIVE-REVIEW-EXPECTED-RED-01.md | 199 | `42825e83ba6e98471c18b0c63d246e9a8790c25591e395cb6a0934c8d7bee6aa` | 199 | `42825e83ba6e98471c18b0c63d246e9a8790c25591e395cb6a0934c8d7bee6aa` | yes |
| wallet-broker/src/zec/spend/verification_context_tests.rs | 528 | `78e7fe50c3bd213f8d0067957bf1771bd42137d46b9e0e8e7adbcb50d8330dec` | 528 | `78e7fe50c3bd213f8d0067957bf1771bd42137d46b9e0e8e7adbcb50d8330dec` | yes |
| docs/testing/BBD-WAL-009-NATIVE-REVIEW-PREREQUISITE-02.md | 181 | `33d488dfcc88eb684d316bcffc547c051be412984d5633c54da76c1cef19dbe2` | 181 | `33d488dfcc88eb684d316bcffc547c051be412984d5633c54da76c1cef19dbe2` | yes |

All sixteen paths unchanged by/after execution: yes

## Post-execution state

No source, test, or evidence files were modified by this execution. No Git, integration,
or repair action was authorized. The index remains empty; the sixteen frozen paths
remain in their pre-execution dirty/untracked state. The new evidence file is the only
artifact added and remains uncommitted.
