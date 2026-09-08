# WAL-009 grouped native layout validation 01

Actor: Hermes Jr Dev. Governance parent: this handoff's commit.
Five ordered Cargo commands, two exact temporary mutations, one evidence record.
Read AGENTS.md, TESTING.md, docs/engineering/HERMES_JR_DEV_ROUTING.md, CURRENT_TASK's
active prefix, this handoff, and the linked
[source review](../testing/BBD-WAL-009-NATIVE-LAYOUT-REPAIR-01-SOURCE-REVIEW.md).
Read only native_ui.rs and its two test modules as needed. No historical sessions.

## Boundaries

Only native_ui.rs may be temporarily mutated, exactly below. Keep its original
bytes in memory and restore them on EVERY outcome. Its accepted identity is
321 lines, d132a164a6413165291abd0582abf3eca4cacd3f33ce6eb4ac2b467e8774d960.
Only docs/testing/BBD-WAL-009-NATIVE-LAYOUT-VALIDATION-01.md is a new persistent
write; it must initially be absent. No source backup, scratch file, other evidence
correction, source repair, test edits, Git mutation, or integration.

Measure all 26 inventory paths before execution and once after final restoration;
compare actual full hashes/counts. There are 25 pending paths plus tracked unchanged
native_surface.rs. Record one hermes --version, actual runtime session/provider/model,
git rev-parse HEAD, git status --short --untracked-files=all, and git diff --cached
--name-only. Require empty index and no unexpected dirty paths. No repeated Git,
version/Cargo probe, environment dump, or file discovery. Use the established
quoted HOME rustup route and existing disk-backed wallet-broker/target unchanged.

## Five commands, each once

Every numbered command uses the exact string and repository workdir with terminal
background=true and notify_on_complete=true FROM THE START. Do not prepend cd,
append redirection/flags, or add wrappers/pipes. Wait only on that returned process;
ordinary running timeouts mean keep waiting. After completion use process.log
offset 0/limit 1000, paging if needed for all total_lines, and retain actual exit
status plus full log. Never use launch acknowledgement or a partial tail as final
evidence. No process discovery/kill or rerun. Unexpected failure or incomplete
capture: restore source, record the stop, and stop without repair.

1. All TEN native UI tests must pass, none failed/ignored:
```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --features native-ui --lib native_ui::
```

2. From original only, replace the unique line:
```rust
            .max_height(body_height)
```
with:
```rust
            .max_height(f32::INFINITY)
```
Measure 321 lines and hash cf18eb0a012fb485ec091f6a142a34be479b6368516e51e7c73de59e1c63a366.
Run exactly one test, requiring exit 101 and the viewport.contains_rect failure
at zec_native_app_tests.rs:152 (not compilation/setup):
```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --features native-ui --lib native_ui::zec_native_app_tests::long_review_at_small_viewport_keeps_controls_usable -- --exact
```
Immediately restore complete original bytes and measure the full original hash
on every outcome, before continuing or stopping.

3. From restored original only, replace this unique App block:
```rust
            if !self.closing {
                self.dialog.borrow_mut().ui(ui);
            }
```
with:
```rust
            {
                self.dialog.borrow_mut().ui(ui);
            }
```
Measure 321 lines and hash 3788efe82a143601e7197f2769180c2972002ca4ea090b11056c7ef01db1a9bc.
Require exactly one test, exit 101, and the full-review equality assertion failing
after close-request/idle frames in the test below. Earlier click/Close-count
assertions must have passed; compilation/setup failure is not accepted falsification.
```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --features native-ui --lib native_ui::zec_native_app_tests::native_app_confirm_closes_once_and_preserves_owned_review -- --exact
```
Immediately restore complete original bytes and measure the full original hash
on every outcome, before continuing or stopping. Never stack mutations.

4. Restored all TEN UI tests must pass:
```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --features native-ui --lib native_ui::
```

5. Native compilation must exit 0; record warnings without fixing them:
```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo check --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --features native-ui
```

## Evidence and stop

After final measured inventory, write the one evidence record: actual runtime,
authorization/launch lineage, measured identities, exact commands/processes/exits,
both mutation/restoration measurements, all test counts and failure assertions,
and complete saved output. Render output blocks directly from saved tool JSON;
do not reconstruct warning/test lines. Normalize local absolute paths, disclose
deviations, and do not embed the evidence's own hash. Measure that hash once after
writing and report it in the short final reply. Then STOP, with no extra command.

No old test-suite repetition, formatter, Clippy, no-run/version probe, source
repair, prior-report correction, Git integration, product launch, OS/capability
claim, network, broadcast, mainnet, hardware, or Monero. Prior accepted gates stay
valid. Owner wants fewer cycles; neither report imperfections nor unused warnings
authorize extra work. Reviewer collects after done; no actor polling.

## Frozen inventory

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| wallet-broker/src/zec/spend.rs | 1046 | 8b70ecf7dea541d951184d89b1ed2d917ce5c3c6dceabb73151840c79517328a |
| wallet-broker/src/zec/test_support.rs | 4369 | c34aa4dc95011c7585b44553e6e1680dd3c918ea78ffa98e07d2e271f7991159 |
| wallet-broker/src/zec/spend/verification_context_tests.rs | 528 | 45f3bdf2f1d08924d53c298517c631427ab052c2d4955b31e98aa555386d380a |
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
| wallet-broker/tests/zec_sign_verify.rs | 1118 | 7a481d3a954a92e04d324be047863a824ecf303bfd6232be70e9d13d3a295987 |
| wallet-broker/src/zec/spend/external_binding_tests.rs | 114 | bb2d6873dd7262ad6e30f117f1947b7d4cf7685ab04e32a5b700d7cb5b112b6d |
| docs/testing/BBD-WAL-009-SIGNATURE-CONTEXT-VALIDATION-01.md | 677 | 6a23bcb2943df54953d1acddf85a2ec9cfc41b92e84c12856a48b7d07be53c58 |
| docs/testing/BBD-WAL-009-RETAINED-SPEND-VALIDATION-01.md | 358 | d51cc09e82f60a02a1493338d0af249dd25f046e2b5ca75e46a0fe4019269ed1 |
| docs/testing/BBD-WAL-009-RETAINED-SPEND-VALIDATION-RESUME-01.md | 560 | 62bb1ffd672c460f6fac6439abbc8d555683342e5410ad7098d2339bf509e5d0 |
| wallet-broker/tests/native_surface.rs | 664 | 349f3a019a0e7a5c37aaae727192b0ddcecb02192a5fae6df9291eaf1357276d |
| wallet-broker/src/native_ui/zec_native_app_tests.rs | 376 | 485ce9691f873af9304e1db36de74d49627279abf9c550ccce4c74bada22a5a4 |
| docs/testing/BBD-WAL-009-NATIVE-REVIEW-VALIDATION-01.md | 503 | 3a734781d0ad2c31989318373272f13ba89654ded2ab3a6b90f83ea322821de1 |
| docs/testing/BBD-WAL-009-NATIVE-APP-EXPECTED-RED-01.md | 184 | e749d709d8d2bd8f5e72a8509c156a2053dbed5983849964348eefbc3429733b |
