# WAL-009 native review validation 01

Actor: Hermes Jr Dev, execution and one evidence record only.
Governance parent: the commit containing this handoff. Run from bb-desktop root.
Grok source work is closed. Six ordered Cargo commands, two exact temporary
mutations with mandatory restoration, then evidence and stop. No integration.

Read AGENTS.md, TESTING.md, docs/engineering/HERMES_JR_DEV_ROUTING.md, this handoff,
CURRENT_TASK.md's active prefix, and
[source review](../testing/BBD-WAL-009-NATIVE-REVIEW-PRODUCTION-01-SOURCE-REVIEW.md).
Source reads are limited to native_ui.rs, its private zec_review_tests.rs, and
native_surface.rs. No historical sessions/handoffs, broader source discovery,
configuration dumps, or previous expensive integration commands.

## Scope and preflight

Only wallet-broker/src/native_ui.rs may be temporarily mutated, exactly below.
Only docs/testing/BBD-WAL-009-NATIVE-REVIEW-VALIDATION-01.md may be written as
evidence; it must initially be absent. All other files, including seven pending
evidence records and reviewer governance, stay frozen. No source backup or scratch
artifact. Keep the original native_ui.rs bytes in memory for restoration.

Record one hermes --version output and this invocation's actual provider/model
and supplied session ID. Record git rev-parse HEAD, git status --short
--untracked-files=all, and git diff --cached --name-only once each. The index must
be empty. Distinguish authorization from the reviewer's subsequent launch commit.
No other Git or version probes. The quoted HOME rustup route below is established;
do not invoke bare rustup, cargo, or rustc, or change PATH/configuration.

Measure all inventory paths below from actual bytes, emitting their full hashes
and newline counts in one JSON object, and compare every entry. The 22 pending
paths are exactly the inventory except tracked unchanged native_surface.rs. Any
unexpected dirty path, mismatch, or staged item stops before tests. Repeat the
same measured inventory once after final restoration/validation, before evidence.

Use the existing wallet-broker/target cache. Prior execution established ext4 and
no Cargo target override; verify only that target's filesystem and the presence
of CARGO_TARGET_DIR/CARGO_BUILD_TARGET_DIR or changed Cargo config target settings.
Narrow read-only checks are permitted; report only relevant target information.
Stop on a conflicting override. Do not dump environment/configuration, change
build settings, probe toolchain versions, or create build/scratch files in tmpfs.

## Exact execution and capture

Launch EACH numbered Cargo command once with the terminal tool's background=true
and notify_on_complete=true from the start. Use its exact string and repository
workdir. No pipes, redirection, wrappers, environment assignments, appended flags,
or retries. Wait only on that returned process using process.wait timeout 60 as
needed; no process discovery or kills. A normal wait timeout with that same
process still running means continue waiting, not failure or permission to relaunch.

After completion retrieve the same process's full log, process.log offset 0 and
limit 1000, paging only if necessary. Require showing == total_lines. Preserve
the actual command, process ID, exit status, and complete output in saved tool
JSON. A launch acknowledgement or wait tail is not full completion evidence.
Incomplete capture, lost/unknown process, compile/fixture failure, zero tests,
wrong assertion, or unexpected counts/result: restore any mutation, record the
stop, and stop. Do not rerun to fix capture. No post-stage-6 extra checks or tests.

1. Original widget green: require all SIX named native_ui::zec_review_tests tests
   pass, none failed or ignored. Filtered unrelated library tests are expected.

```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --features native-ui --lib native_ui::zec_review_tests
```

2. Confirm falsification: from original 303-line hash c6d5fc3a... only, replace
   the unique line below (preserve its twelve-space indentation):

```rust
            self.state = ZecReviewState::Confirmed;
```

with:

```rust
            self.state = ZecReviewState::Denied;
```

Measure 303 lines, SHA-256
fc406fcf71021b1c08b8402e565ffcfc7e23b016ba366888324f5579bfdd6cca before execution.
Require exactly ONE executed test, exit 101, failing its first confirmed-review
equality assertion (zec_review_tests.rs:153), not a compile or fixture failure.

```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --features native-ui --lib native_ui::zec_review_tests::confirm_click_returns_exact_owned_review_and_cannot_rearm -- --exact
```

Restore the complete original bytes and measure 303 lines/full c6d5fc3a... hash
immediately after this result, on EVERY outcome, before continuing or stopping.

3. Close precedence falsification: from the restored original only, replace this
   unique block:

```rust
        if close_or_escape || cancel.clicked() {
            self.cancel();
        } else if confirm.clicked() && self.is_pending() {
            self.state = ZecReviewState::Confirmed;
        }
```

with:

```rust
        if confirm.clicked() && self.is_pending() {
            self.state = ZecReviewState::Confirmed;
        } else if close_or_escape || cancel.clicked() {
            self.cancel();
        }
```

Measure 303 lines, SHA-256
4fa5e01f20de8abe1d7a737fcdeaf162d46af3c34fcf79c7fc4989253a9f7e9a before execution.
Require exactly ONE executed test, exit 101, failing the is_none assertion after
the simultaneous viewport close and Confirm release (zec_review_tests.rs:190).
The earlier close-only/terminal-denial assertions must have passed first.

```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --features native-ui --lib native_ui::zec_review_tests::viewport_close_denies_including_same_frame_confirm_release -- --exact
```

Restore the complete original bytes and measure 303 lines/full c6d5fc3a... hash
immediately after this result, on EVERY outcome, before continuing or stopping.
Never stack the two mutations, edit tests, or repair production to make a gate pass.

4. Restored widget green: all SIX tests pass, none failed or ignored.

```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --features native-ui --lib native_ui::zec_review_tests
```

5. Existing native-surface regressions: all SEVENTEEN tests pass, none failed,
   ignored, or filtered. Its synthetic XMR-selection controller assertions do not
   launch Monero or authorize the parked real-node/device work.

```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --features native-ui --test native_surface
```

6. Native-feature compilation: exit 0. Record warnings; this is not a warning-denied
   Clippy gate or OS window test. Do not fix warnings or add a no-run probe.

```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo check --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --features native-ui
```

## Evidence and final stop

After the final measured inventory, write the one evidence record. Include runtime
identity, authorization/checkpoint lineage, preflight and actual inventories,
each exact command/process/exit/count, both measured mutations and restorations,
and COMPLETE saved outputs. Render log blocks from the actual saved tool JSON;
do not reconstruct compiler/test output from recollection or substitute a wait
tail. Only normalize local absolute repository/tool-cache paths to portable
placeholders; preserve all other lines. Disclose every deviation and limitation.
Do not search old sessions or rewrite any earlier evidence. Do not put this new
record's own hash inside itself; measure it once after writing and report it in
the final reply. No repeated self-hash correction loop.

These are real egui widget-event regressions with synthetic inputs plus existing
controller tests and compilation. They do not launch a native window, verify
real-length field layout/platform lifecycle, or exercise the complete native
capability bridge. Do not claim OS/native integration acceptance. No product
launch, full retained-spend/signature suite rerun, networking, broadcast, mainnet,
hardware, Monero, new source/test authoring, dependency action, or integration.

Then stop. No formatter, additional Cargo/test command, Git mutation, commit,
push, CURRENT_TASK/ticket change, or other evidence write. The reviewer collects
once after done/explicit collection. A short final reply gives stage outcomes,
restoration status, evidence identity, and blockers; no further work.

## Frozen inventory

All hashes/counts are full-file identities. native_ui.rs is the sole temporary
mutation target; native_surface.rs is tracked unchanged, and every other row is
an existing pending source/package/lock/evidence path.

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| wallet-broker/src/zec/spend.rs | 1046 | 8b70ecf7dea541d951184d89b1ed2d917ce5c3c6dceabb73151840c79517328a |
| wallet-broker/src/zec/test_support.rs | 4369 | c34aa4dc95011c7585b44553e6e1680dd3c918ea78ffa98e07d2e271f7991159 |
| wallet-broker/src/zec/spend/verification_context_tests.rs | 528 | 45f3bdf2f1d08924d53c298517c631427ab052c2d4955b31e98aa555386d380a |
| wallet-broker/src/native_ui.rs | 303 | c6d5fc3a4dee46f4700f2a3ae5866a5003746eab5aa6dd6bc20e5ba5c6d726b4 |
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
| wallet-broker/tests/zec_sign_verify.rs | 1118 | 7a481d3a954a92e04d324be047863a824ecf303bfd6232be70e9d13d3a295987 |
| wallet-broker/src/zec/spend/external_binding_tests.rs | 114 | bb2d6873dd7262ad6e30f117f1947b7d4cf7685ab04e32a5b700d7cb5b112b6d |
| docs/testing/BBD-WAL-009-SIGNATURE-CONTEXT-VALIDATION-01.md | 677 | 6a23bcb2943df54953d1acddf85a2ec9cfc41b92e84c12856a48b7d07be53c58 |
| docs/testing/BBD-WAL-009-RETAINED-SPEND-VALIDATION-01.md | 358 | d51cc09e82f60a02a1493338d0af249dd25f046e2b5ca75e46a0fe4019269ed1 |
| docs/testing/BBD-WAL-009-RETAINED-SPEND-VALIDATION-RESUME-01.md | 560 | 62bb1ffd672c460f6fac6439abbc8d555683342e5410ad7098d2339bf509e5d0 |
| wallet-broker/tests/native_surface.rs | 664 | 349f3a019a0e7a5c37aaae727192b0ddcecb02192a5fae6df9291eaf1357276d |
