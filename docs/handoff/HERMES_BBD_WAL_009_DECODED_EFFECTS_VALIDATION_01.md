# WAL-009 decoded effects production source acceptance and validation 01

Reviewer: Codex, XHigh. Actor: Jr Dev — Hermes, execution/evidence only.
Grok authorization: 302047ec; launch checkpoint: cf0cd8a6.
Grok session 378e7dc3-8f4e-4afe-989f-468a73a4a9a5, outer 87196, collected once
following owner done, exit 0; runtime grok-4.6-build High.

## Source acceptance

ACCEPT the exact four-file drop for grouped validation. This does not yet accept
runtime correctness, full cleanup/native integration or broader WAL-009 delivery.

The three existing files reverse through the saved source replacements to their
exact authorized baselines. The new effects.rs reverses to the exact saved write
(d80e722a62ab8c741a08ed82036b86a039843d3a57e73c58759dea439828e957).
The actor used source/API reads, source writes and three measurement/discovery
terminal commands. No test/compiler/formatter/Git/network/evidence execution.
All 25 unaffected source/evidence identities match. Final identities are frozen
in the 29-row table below, including all eleven prior actor evidence records.

The source now obtains software viewing authority after stored/derived UFVK
equality and forwards the selected account's UFVK/Network on the synthetic route.
Typed pre-sign validation checks value commitments, output note commitments, real
nullifier/rk ownership and cross-address rules before retaining spend/hash/anchor
context. Complete decode, decoded hash/anchor/action binding and existing proof/
spend/binding signatures precede effect acceptance. Scoped per-action recovery
establishes payment and internal change; checked Zatoshis arithmetic establishes
fee/conservation. Actual payment, memo and output counts now come from recovery.

The private authority contains no financial inspection copy. Regression changes
only capture/pass the authority; the independent oracle, positive control and all
ten false-metadata assertions remain intact. test_support.rs changes only four
lines retrieving/forwarding external-route UFVK/Network. No source finding blocks
this validation. The pinned compact-field resolver and complete prepared fixture
anchor path were inspected; this is not acceptance of deferred-anchor workflows.
Known full actual-secret cleanup and OS/capability gaps remain open.

## Authority and preflight

Hermes may create docs/testing/BBD-WAL-009-DECODED-EFFECTS-VALIDATION-01.md and the
small backup wallet-broker/target/bbd-wal009-effects-validation-original.rs.
Only stage 2 may temporarily edit wallet-broker/src/zec/spend/effects.rs, exactly
as defined below, with full restoration on every outcome. No other source/test,
prior evidence, governance, dependency or Git mutation. No integration or source
repair. Stop if the new evidence or backup path already exists.

Use existing wallet-broker/target on the measured disk-backed filesystem, not a
new temporary target/cache. Record hermes --version once and the actual provider,
model and Hermes runtime session ID; distinguish that ID from process session IDs.
Read only active instructions, relevant frozen source, and this handoff. No Git
commands (including read-only), broad directory/transcript discovery, repeated
version checks or process-list probes. Launch checkpoint commits do not alter
source identity and do not require a Git-history investigation.

Measure the 29 frozen identities before execution and after the final executed
stage using one read-only script each time. PARSE the rows of this exact handoff;
do not manually reconstruct the path list or substitute a similarly named file.
For every row calculate byte SHA-256 and newline count and compare to this table.
Stop on mismatch; do not fix an inventory script and silently claim the original
measurement passed. Report the actual measurements and deviations accurately.

## Six stages, once each, in order

Each command is run in the repository root, exactly as written, once in the
terminal background with retained output. Wait only on that process for at most
60 seconds per wait. On completion retrieve the full process.log, paging until
reported lines are complete. A capped wait tail is not the full log. Preserve
exact launch/command/process/completion identifiers, exit, counts and timings.

1. Regression green: exit 0, one passed, 6 filtered.

```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --lib zec::spend::verification_context_tests::decoded_effects_reject_agreeing_metadata_that_disagrees_with_signed_bytes -- --exact
```

2. Receiver-comparison falsification. First retain complete effects.rs bytes in
the named backup and verify their original 379-line identity from the table.
Replace this unique exact block:

```rust
    if expected.destination_receiver_bytes != recovered.payment_receiver_bytes
        || expected.destination != recovered.payment_receiver
        || expected_amount != recovered.payment_amount
```

with:

```rust
    if expected_amount != recovered.payment_amount
```

No other mutation. Expected temporary identity: 377 lines, SHA-256
3a15bd53e98d514d9e71e22c2e4d707598fe018ba4dce3f35c43f44ef4ec5972.
Run the exact stage-1 command once. Expected exit 101 and the regression's final
assertion reports receiver:ok only; the other nine cases must still reject.
This deliberately disables both receiver comparisons while retaining recovery,
positive verification, crypto, account/hash/anchor and other effect checks.

IMMEDIATELY restore the complete original effects.rs bytes and measure its full
379-line original hash on every outcome, including compile/tool/unexpected-test
failure. Do not continue or publish with a fault left in source. No stacked fault.
If the intended falsification does not occur, restore then stop; no workaround.

3. Restored broader library green: exit 0, seven tests passed, none filtered.
This includes the regression and existing signature/context and contribution
binding tests; it also proves the shared fixture adapter has not broken them.

```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --lib
```

4. Software pipeline integration: exit 0, one passed, 13 filtered.

```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_sign_verify software_signs_proves_finalizes_extracts_and_independently_decodes_exact_v6_effects -- --exact
```

5. Synthetic contribution pipeline integration: exit 0, one passed, 13 filtered.

```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_sign_verify synthetic_keystone_v2_returns_tagged_ironwood_contributions_for_retained_pczt -- --exact
```

6. Native-feature compile: exit 0. Record warnings without fixing or suppressing.
This is a compile check, not an OS-window or warning-denied acceptance claim.

```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo check --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --features native-ui
```

Proof generation/verification can take several minutes. Elapsed time does not
authorize retries, a new process, extra probes or a relaxed test. Stop at the first
unexpected result after required restoration. Do not run later stages after a
failure. No formatter, Clippy, no-run probe, broader suite, live wallet, network,
broadcast, mainnet, hardware transport, Monero, actor or subagent. The prior
453.79-second expected red remains accepted and must not be rerun as baseline.

## Evidence and final stop

After the last executed stage and required restoration, measure the 29 identities
once and write the one named evidence record: runtime, lineage, actual full path/
line/hash measurements, exact commands/process IDs/exits, both fault/restoration
identities, all observed counts/timings/assertions and full saved process output.
Use the saved tool JSON directly for output blocks. Normalize local absolute
paths to portable repo-relative placeholders and disclose only that normalization;
do not rewrite warning indentation or invent output. If complete logs cannot be
retrieved, record what is missing and stop without rerunning anything.

Disclose mistakes instead of claiming clean compliance. Do not label a process
ID as the Hermes runtime session or a production positive control as the independent
oracle. No extra Git status, repeated preflight or report-only correction cycle.
Measure the new record's full SHA-256/line count once after writing, then stop.
No self-hash in the record, no source/evidence integration, no broader acceptance.
Reviewer will independently accept or reject the actual outcomes.

Implementation and eleven prior actor evidence records remain uncommitted.
Keep reviewer XHigh for effects acceptance. Reviewer publication scope: this
handoff, docs/handoff/CURRENT_TASK.md, tickets/BBD-WAL-009.md only. Launch once,
record the outer identity, collect after owner done, and never poll the actor.

## Frozen inventory

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
| wallet-broker/tests/zec_sign_verify.rs | 1118 | 7a481d3a954a92e04d324be047863a824ecf303bfd6232be70e9d13d3a295987 |
| wallet-broker/src/zec/spend/external_binding_tests.rs | 114 | bb2d6873dd7262ad6e30f117f1947b7d4cf7685ab04e32a5b700d7cb5b112b6d |
| docs/testing/BBD-WAL-009-SIGNATURE-CONTEXT-VALIDATION-01.md | 677 | 6a23bcb2943df54953d1acddf85a2ec9cfc41b92e84c12856a48b7d07be53c58 |
| docs/testing/BBD-WAL-009-RETAINED-SPEND-VALIDATION-01.md | 358 | d51cc09e82f60a02a1493338d0af249dd25f046e2b5ca75e46a0fe4019269ed1 |
| docs/testing/BBD-WAL-009-RETAINED-SPEND-VALIDATION-RESUME-01.md | 560 | 62bb1ffd672c460f6fac6439abbc8d555683342e5410ad7098d2339bf509e5d0 |
| wallet-broker/tests/native_surface.rs | 664 | 349f3a019a0e7a5c37aaae727192b0ddcecb02192a5fae6df9291eaf1357276d |
| wallet-broker/src/native_ui/zec_native_app_tests.rs | 376 | 485ce9691f873af9304e1db36de74d49627279abf9c550ccce4c74bada22a5a4 |
| docs/testing/BBD-WAL-009-NATIVE-REVIEW-VALIDATION-01.md | 503 | 3a734781d0ad2c31989318373272f13ba89654ded2ab3a6b90f83ea322821de1 |
| docs/testing/BBD-WAL-009-NATIVE-APP-EXPECTED-RED-01.md | 184 | e749d709d8d2bd8f5e72a8509c156a2053dbed5983849964348eefbc3429733b |
| wallet-broker/src/zec/spend/effects.rs | 379 | bd62a867ba8c135517df6858aff9eb97aaa4e4c30c76e22c93a27b622412fd56 |
| docs/testing/BBD-WAL-009-NATIVE-LAYOUT-VALIDATION-01.md | 354 | ffcfc14837ff02aba33a5a24fa6a58d610055b18b4efd150787c6b8acba7dc43 |
| docs/testing/BBD-WAL-009-DECODED-EFFECTS-EXPECTED-RED-01.md | 196 | a7def01abd54eae10e663e341ce8b253992d883863a1410c93278fa698d78431 |
