# WAL-009 signature-context validation 01

Actor: Hermes Jr Dev. Governance parent: the commit containing this handoff.
Run from bb-desktop repository root. All Grok source tasks are closed.
This supersedes earlier execution permissions: at most six ordered test commands,
two exact temporary falsifications, and one uncommitted evidence record.

Read AGENTS.md, TESTING.md, docs/engineering/HERMES_JR_DEV_ROUTING.md, this handoff,
only the first 70 lines of docs/handoff/CURRENT_TASK.md, and
docs/testing/BBD-WAL-009-SIGNATURE-CONTEXT-FALSIFICATION-ORDER-01-REVIEW.md.
For mutation/error attribution only, bounded reads of spend.rs:508-537 and the
four test functions in verification_context_tests.rs are allowed. No historical
log/evidence reload, dependency lookup, filesystem discovery, or other actor.

## Preflight

Record one `hermes --version` result and this supplied current session ID's actual
provider/model. Adoption history is not runtime identity. Allowed read-only Git:
`git rev-parse HEAD`, `git status --short --untracked-files=all`, and
`git diff --cached --name-only`. The index must be empty. The reviewer may publish
only the launch checkpoint after the authorization commit; distinguish that
observed HEAD from the governance parent.

The seventeen paths below are the complete expected dirty/untracked set before
execution. The new evidence path must be absent. Emit one parsed JSON measurement
object from actual file bytes with SHA-256 and newline counts; compare every row.
Do not create the evidence table by copying expected values.

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| wallet-broker/src/zec/spend.rs | 929 | bd51b3d0ee2f748fa31e483a2747c157ff20544aa62e3e00a7929c77060b7361 |
| wallet-broker/src/zec/test_support.rs | 4371 | fea8f65ed6637033506902688c8f547952cea848af51d05a49969cae81f48920 |
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

Submit each command below as the entire terminal command, at most once, in order.
Use its actual tool output and exit_code. Do not add a shell operator, redirection,
pipe, environment assignment, wrapper, version probe, or status echo. Never run
tests concurrently. Waiting for the SAME running command is allowed; rerunning it
for clearer capture is not. If capture is incomplete, restore any active mutation,
record the limitation, and stop.

1. Baseline focused green:

```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --lib zec::spend::verification_context_tests
```

Require exit 0, exactly the four named context tests passed, zero failed/ignored.
Compilation failure, fixture failure, zero/missing tests, or any other result is
an unexpected stop: no mutation or further test follows. Record all diagnostics,
warnings, and actual counts; do not repair source.

2. Only after green, apply the transparent falsification to spend.rs. Require its
baseline SHA-256 bd51b3d0ee2f748fa31e483a2747c157ff20544aa62e3e00a7929c77060b7361
and exactly one occurrence of this old text:

```rust
Some(_) => Err(ZecError::intent_mismatch()),
```

Replace only that text with:

```rust
Some(_) => Ok(None),
```

Every other byte stays unchanged. Require 929 lines and mutated SHA-256
`e21eb580d998b483ad75bb13454b8f9a2208fc6b2a629fdcb7785a9bc3c944f7`.
Record the measured identity, then run:

```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --lib zec::spend::verification_context_tests::rejects_every_present_transparent_bundle -- --exact
```

Require exit 101, one executed/failed test, and the assertion panic
"constructor accepted a present transparent bundle". A compile error, unrelated
panic, skipped test, or unexpected pass does not falsify the intended control.

Immediately reverse only the replacement and verify the full baseline hash/929
lines, on both expected and unexpected outcomes. After an unexpected outcome,
stop after restoration and evidence. Never combine this mutation with the next.

3. Only after the required transparent failure and verified restoration, apply
the message falsification to the same baseline spend.rs. Replace exactly one
line (eight leading spaces and its newline):

```rust
        *sighash.as_ref()
```

with exactly these three lines:

```rust
        let mut message = *sighash.as_ref();
        message[0] ^= 0xff;
        message
```

Require 931 lines and mutated SHA-256
`b50c59b68cdd06bf318a396d699eaa9f64d0a296e22af60bf5079c6683f80fbc`.
Record the measured identity. Under this same single mutation, run these two
commands in order, continuing to the second only after the first required result:

```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --lib zec::spend::verification_context_tests::preserves_decoded_v6_ironwood_and_matches_pczt_sighash -- --exact
```

Require exit 101, one executed/failed test, specifically at the comparison of
context.shielded_sighash() with fixture.pczt_sighash in the preservation test.

```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --lib zec::spend::verification_context_tests::uses_real_spend_and_binding_signatures -- --exact
```

Require exit 101, one executed/failed test, specifically at the actual valid
action.rk().verify(&message, action.authorization()).is_ok() assertion before the
PCZT equality. A compile error, fixture failure, early oracle failure, parse
failure, skipped test, or unexpected pass is an unexpected stop.

Always reverse only the three-line replacement and verify the full baseline
hash/929 lines before leaving this stage, even on a failure. Never repair source
or change tests to obtain the desired failure. No Git checkout/restore/reset,
formatter, saved-source file, or speculative replacement is permitted.
If the current file unexpectedly matches neither its authorized mutated identity
nor the baseline, stop and disclose the restoration blocker without overwriting
unrecognized changes. Otherwise restoration is mandatory before evidence/stop.

4. Only after both required message failures and exact source restoration, run
the two reserved broader commands in order:

```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_prepare --test zec_sign_verify
```

Require exit 0 and nonzero executed tests in EACH named integration target, with
no failed tests. Record each target's actual counts; do not infer them from source.
Any failure stops before the final command.

```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --lib
```

Require exit 0, no failed tests, and all four context tests passing in the restored
source. Record complete library counts, including any existing ignored/filtered
tests; never call an ignored test passed. This final command supplies the restored
context green as well as the broader library regression. No extra focused rerun.

## Evidence and stop

After the last authorized result or unexpected stop, only restoration if needed,
the same seventeen-path measurement, read-only Git status, bounded saved-result
retrieval, and creation/verification of this file remain authorized:

docs/testing/BBD-WAL-009-SIGNATURE-CONTEXT-VALIDATION-01.md.

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
and exact gate tool-call IDs/results within that session. Parse their JSON and
render directly in memory. Do not enumerate the Hermes directory, search other
sessions, read prompts/configuration/credentials, or paste a manually reconstructed
output string into the renderer. No scratch or source-backup files are authorized.

Verify the evidence content against the saved data and report its final hash/line
count. All seventeen starting paths must have their full starting identities at
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
