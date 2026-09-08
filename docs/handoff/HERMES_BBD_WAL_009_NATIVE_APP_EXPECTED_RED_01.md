# WAL-009 native App/layout focused execution 01

Actor: Hermes Jr Dev. Governance parent: commit containing this handoff.
ONE Cargo command, one new evidence file, no mutation or integration.
Read AGENTS.md, TESTING.md, docs/engineering/HERMES_JR_DEV_ROUTING.md, CURRENT_TASK's active prefix,
this handoff, and the linked
[source review](../testing/BBD-WAL-009-NATIVE-APP-TEST-SOURCE-01-REVIEW.md).
Source reads are limited to native_ui.rs and its two private test modules.

All source/tests/package/locks and existing evidence are frozen at the inventory
below. Measure and compare all 25 identities before execution and once afterward;
native_surface.rs is tracked unchanged, while the other 24 are pending paths.
Emit actual full hashes/newline counts, not a copied expected-value assertion.
The only new write is docs/testing/BBD-WAL-009-NATIVE-APP-EXPECTED-RED-01.md;
it must initially be absent. No scratch files, source backups, or config changes.

Record hermes --version once, the supplied invocation session ID and actual
provider/model, plus git rev-parse HEAD, git status --short --untracked-files=all,
and git diff --cached --name-only once each. Index must be empty. Do not substitute
the adoption model or outer terminal ID for the actual runtime/session. Record
authorization and later launch checkpoint separately. Reuse the existing pinned
Rust route and disk-backed wallet-broker/target; no toolchain/version/dependency
probe, environment/configuration dump, directory discovery, or old-session search.
Any unexpected source/index/dirty-path mismatch stops before the test.

Run exactly once from bb-desktop root:

```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --features native-ui --lib native_ui::zec_native_app_tests
```

Set terminal background=true and notify_on_complete=true from the start. No
redirection, pipe, wrapper, extra flags, or second launch. Wait only on its
returned process with process.wait timeout 60 as needed; a still-running wait
timeout means keep waiting on that process. After exit retrieve process.log,
offset 0, limit 1000, paging if needed to capture all total_lines. The launch exit
is not test completion. No process.list/kill or unsupported get_log action.

Classify the outcome of EACH of the four named tests:

- long_review_at_native_size_keeps_controls_usable
- long_review_at_small_viewport_keeps_controls_usable
- native_app_confirm_closes_once_and_preserves_owned_review
- native_app_close_release_denies_and_closes_once

Anticipated behavioral red is a layout test failing viewport.contains_rect for
an off-screen control. The App lifecycle cases are expected to pass existing
logic; a failure there is a separate finding. An all-four pass is valid and needs
no invented failure. Compile/setup/paint-helper failure or zero tests is not
layout red. Record exact failure locations/messages and all counts, whatever the
outcome. Do not rerun, repair, mutate, or start another suite.

After the final inventory, write only the evidence record with runtime/lineage,
measured inventory, exact command/process/exit/counts, complete captured output,
and concise per-test classification. Render the log from saved tool JSON rather
than reconstructing warning/test lines. Normalize local absolute paths to portable
placeholders. Record deviations candidly. Do not put the evidence's own hash in
the file; measure it once after writing and report it in the final reply.

Then STOP. No additional Cargo/npm/compiler/formatter or Git command, no source
repair, prior-report correction, integration, OS window, network, broadcast,
mainnet, hardware, or Monero work. Do not rerun any previously accepted suite.
No report-only follow-up is implied. The reviewer collects after done; no polling.

## Frozen inventory

native_ui.rs includes only the accepted test-module declaration change. All other
existing production bytes and eight prior evidence records remain frozen.

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| wallet-broker/src/zec/spend.rs | 1046 | 8b70ecf7dea541d951184d89b1ed2d917ce5c3c6dceabb73151840c79517328a |
| wallet-broker/src/zec/test_support.rs | 4369 | c34aa4dc95011c7585b44553e6e1680dd3c918ea78ffa98e07d2e271f7991159 |
| wallet-broker/src/zec/spend/verification_context_tests.rs | 528 | 45f3bdf2f1d08924d53c298517c631427ab052c2d4955b31e98aa555386d380a |
| wallet-broker/src/native_ui.rs | 306 | b0d4da8770a400315889e5d69ad830e3e08d707c5bc62ca1f6e633798703349c |
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
