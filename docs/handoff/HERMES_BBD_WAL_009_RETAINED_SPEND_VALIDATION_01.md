# WAL-009 retained-spend validation 01

Actor: Hermes Jr Dev. Governance parent: the commit containing this handoff.
Run from bb-desktop root. Grok repair 02 is closed and accepted for execution.
This authorizes five ordered Cargo test commands, two exact temporary
falsifications, and one uncommitted evidence record. No integration or repair.

Read AGENTS.md, TESTING.md, docs/engineering/HERMES_JR_DEV_ROUTING.md, this handoff,
the active leading CURRENT_TASK.md section, and
docs/testing/BBD-WAL-009-RETAINED-SPEND-SOURCE-REVIEW-02.md.
Bounded source reads for mutation/error attribution: spend.rs::validate_external_binding,
capture_retained_spend, inspect_ironwood_spend_values, inspect_final_pczt;
external_binding_tests.rs and failing functions in tests/zec_sign_verify.rs.
No historical evidence/session reload, broad discovery, or other actor.

## Preflight

Record one `hermes --version` result and this supplied current session ID's actual
provider/model. Adoption history is not runtime identity. Allowed read-only Git:
`git rev-parse HEAD`, `git status --short --untracked-files=all`, and
`git diff --cached --name-only`. The index must be empty. The reviewer may publish
only the launch checkpoint after the authorization commit; distinguish that
observed HEAD from the governance parent.

The twenty paths below are the complete expected dirty/untracked set before
execution. The new evidence path must be absent. Emit one parsed JSON measurement
object from actual file bytes with SHA-256 and newline counts; compare every row.
Do not create the evidence table by copying expected values.

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| wallet-broker/src/zec/spend.rs | 1046 | 8b70ecf7dea541d951184d89b1ed2d917ce5c3c6dceabb73151840c79517328a |
| wallet-broker/src/zec/test_support.rs | 4369 | c34aa4dc95011c7585b44553e6e1680dd3c918ea78ffa98e07d2e271f7991159 |
| wallet-broker/src/zec/spend/verification_context_tests.rs | 528 | 45f3bdf2f1d08924d53c298517c631427ab052c2d4955b31e98aa555386d380a |
| wallet-broker/src/native_ui.rs | 201 | 600e2fb34134a276d31da9c5e746cc6478337bae73590fd742cf2d36ff785524 |
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
| wallet-broker/tests/zec_sign_verify.rs | 1117 | 2b75901787126ec318bfe481fedd6d388bfe4afa4184cb03ca052447e803518d |
| wallet-broker/src/zec/spend/external_binding_tests.rs | 114 | bb2d6873dd7262ad6e30f117f1947b7d4cf7685ab04e32a5b700d7cb5b112b6d |
| docs/testing/BBD-WAL-009-SIGNATURE-CONTEXT-VALIDATION-01.md | 677 | 6a23bcb2943df54953d1acddf85a2ec9cfc41b92e84c12856a48b7d07be53c58 |

Verify wallet-broker/target remains disk-backed repository ext4 and is the
effective existing Cargo target. Filesystem checks of repository, target, and
temporary storage, plus narrow inspection for a Cargo target override, are
allowed. Do not print the environment or configuration wholesale. Temporary
storage is tmpfs; no build, evidence-scratch, or backup files belong there.

Resolve the quoted HOME rustup path used below to the existing executable; do not
change HOME, PATH, toolchain, target, Cargo config, dependencies, or lockfiles.
This portable spelling replaces the earlier machine-specific absolute spelling,
with the same pinned executable/toolchain and Cargo arguments. Any unexpected
file, identity, staged entry, target override, missing executable, or preflight
failure stops before Cargo. Record the actual limitation without repairing it.

## Ordered commands and falsifications

Submit each numbered Cargo command exactly as its entire terminal command, once,
in this order. No shell operators, redirects, pipes, wrappers, environment
assignments, status echoes, version probes, or concurrent tests. Waiting on the
SAME running command is permitted; rerunning to improve capture is not.
An unexpected result stops the sequence after mandatory restoration and evidence.
A compile error, zero/skipped expected tests, fixture error, unrelated panic, or
incomplete capture does not satisfy a required pass/falsification.
Record all diagnostics and warnings. Do not format or repair anything.

1. Focused metadata green:

```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --lib zec::spend::external_binding_tests
```

Require exit 0: exactly accepts_matching_retained_slot_zero_and_one and
rejects_misbound_external_contribution pass, zero failed/ignored.
These synthetic fixed-array tests validate metadata only, not cryptography.

2. Affected integration green, before either mutation:

```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_prepare --test zec_sign_verify
```

Require exit 0, zec_prepare 11 passed and zec_sign_verify 14 passed, zero
failed/ignored in each. Record actual results; any failure stops before mutations.
The software/external happy paths must execute the real pinned pipeline.
Test names about exact effects or cleanup do not establish acceptance of the
known separate metadata-copying/actual-secret-cleanup blockers.

3. Only after both greens, falsify retained external index binding. Require
spend.rs baseline 1046 lines and SHA-256
8b70ecf7dea541d951184d89b1ed2d917ce5c3c6dceabb73151840c79517328a.
Replace exactly one complete line (eight leading spaces; preserve newline):

```rust
        || contribution.signature.action_index() != binding.action_index
```

with:

```rust
        || contribution.signature.action_index() != 0
```

All other bytes stay unchanged. Measure mutated identity: 1046 lines, SHA-256
46063dd2cb180924959085d6a474b3461a4b8882de379dd9338d9110db1ac621.
Then run:

```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --lib zec::spend::external_binding_tests::accepts_matching_retained_slot_zero_and_one -- --exact
```

Require exit 101, one executed/failed test, specifically the slot-one is_ok()
assertion at external_binding_tests.rs:58. Slot zero must have passed its earlier
assertion. Immediately reverse only that replacement and verify full original
SHA-256/1046 lines, even on an unexpected outcome. Stop after restoration on any
unexpected result. Never run the next mutation on an already mutated file.

4. Only after the required failure and restoration, restore the bad witness-based
classification as a falsification. Require the same baseline. Replace exactly one
occurrence of these two complete lines, including their newlines:

```rust
    let mut external_outputs = 0;
    let mut change_outputs = 0;
```

with these twelve lines:

```rust
    if pczt
        .ironwood()
        .actions()
        .iter()
        .filter(|action| action.spend().witness().is_some())
        .count()
        != 1
    {
        return Err(ZecError::intent_mismatch());
    }
    let mut external_outputs = 0;
    let mut change_outputs = 0;
```

All other bytes stay unchanged. Measure mutated identity: 1056 lines, SHA-256
878e3dd26f3f6eb3a897f84e6639186c5a94d6065529b1cf2e4de44822f301c7.
Then run:

```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_sign_verify software_signs_proves_finalizes_extracts_and_independently_decodes_exact_v6_effects -- --exact
```

Require exit 101, one executed/failed test, specifically sign_and_verify's success
unwrap at zec_sign_verify.rs:165 receiving INTENT_MISMATCH. The prior same-source
integration green plus this isolated injected guard is the classification control.
Immediately reverse only this replacement and measure original SHA-256/1046 lines,
even on an unexpected outcome; stop after restoration on any unexpected result.

For both mutations, require exact unique old text and full starting/mutated hash.
Never use Git restore/checkout/reset, source backups, formatter, or speculative
replacements. If the file matches neither baseline nor the authorized mutation,
disclose the restoration blocker without overwriting unrecognized changes.
Restoration is mandatory before leaving a reached mutation stage.

5. Only after both required failures and exact restoration, final library green:

```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --lib
```

Require exit 0, zero failed, both new binding tests and all four frozen signature
context tests passed. Record complete library counts and any ignored/filtered
cases; do not count ignored tests as passed. This includes the restored metadata
green; no extra focused rerun. Native-feature execution remains outside this task.

## Evidence and stop

After the last authorized result or unexpected stop, only restoration if needed,
the same twenty-path measurement, read-only Git status, bounded saved-result
retrieval, and creation/verification of this file remain authorized:

docs/testing/BBD-WAL-009-RETAINED-SPEND-VALIDATION-01.md.

Generate it programmatically from actual saved tool data. Include runtime/version/
current session ID, governance parent and observed HEAD, preflight/filesystem,
every exact submitted command, tool exit code, complete output, actual counts and
assertion location, stage disposition, every unrun stage, both measured mutated
identities/restorations if reached, all full before/after hashes/line counts, and
any deviation. A partial stop is not full validation success.

Normalize local prefixes in captured output to <repo>/, <cargo-registry>/, and
<home>/ while preserving every other byte, whitespace, diagnostic, and summary.
Use those tokens in prose too; never include literal local installation paths.
Record submitted command strings exactly (their HOME spelling is already portable).
No raw PCZT, transaction, key, seed, receiver, memo, proof, or signature artifact
may be written or logged. Do not manually retype diagnostics or measured tables.

Use actual saved terminal JSON directly. If needed, open the known Hermes
state.db via Path.home()/".hermes"/"state.db" in SQLite URI mode=ro with uri=True.
Query ONLY the supplied current session. Retrieve its model/billing_provider and
resolve only the needed preflight, version, measurement, mutation/restoration,
and exact gate tool-call IDs/results within that session. Read-only schema PRAGMA
for sessions/messages is allowed if needed; never select recent or other sessions.
Parse their JSON and render directly in memory. Do not enumerate the Hermes directory, search other
sessions, read prompts/configuration/credentials, or paste a manually reconstructed
output string into the renderer. No scratch or source-backup files are authorized.

Verify the evidence content against the saved data and report its final hash/line
count. All twenty starting paths must have their full starting identities at
the end. The evidence is the only new repository record. Existing build/test-state
artifacts under wallet-broker/target are permitted; no other new artifact path is
authorized. Do not modify earlier evidence or governance or integrate anything.

No source/test design, permanent repair, dependency operation, network, native
launch, other test, formatter, lint, audit, scanner, or other actor. No Git mutation,
staging, commit, or push. Full recovered-effect, actual-secret cleanup, native,
security, mainnet, Monero, and broadcast gates remain separately pending.
Report the reached/remaining stages, actual counts, runtime/session identity,
restoration status, and evidence identity, then stop. The reviewer collects once
only after the owner reports done; no actor polling or duplicate launch.
