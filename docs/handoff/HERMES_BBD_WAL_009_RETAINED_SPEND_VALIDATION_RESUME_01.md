# WAL-009 retained-spend validation resume 01

Actor: Hermes Jr Dev. Governance parent: the commit containing this handoff.
Run from bb-desktop root. All Grok source tasks are closed.
Five ordered test commands, three exact temporary falsifications, one new evidence
record, and correction of the named prior evidence record. No integration/repair.

## Capacity interruption continuation

This amendment overrides the unstarted-stage wording below. Resume completed
Hermes invocation/session 20260908_114350_76a7e5 after its upstream HTTP 429
capacity error. Stage 1 already ran ONCE and PASSED: command 78030, background
proc_31fb6516c988 (78031), final completion 78047, exit 0, one passed/13 filtered,
475.66 seconds. There is no running test to restart. Do NOT rerun stage 1.

The reviewer accepts that saved completion result and its captured warning/build/
test output as the stage-1 result. Include it directly from saved JSON in evidence.
Retrieve the original full process log only if the existing handle remains
available; an unavailable old handle does not authorize a test rerun. If necessary,
read preflight and stage-1 records from that exact predecessor session even if
this resumed invocation receives a new session ID. Record both runtime identities.

All twenty-one file baselines still match; no mutation/evidence write was reached.
Retain the prior runtime/version and preflight data. Do not repeat Git, version,
filesystem, or toolchain probes. The reviewer separately confirmed that all known
cwd/ancestor Cargo config/config.toml paths are absent. Recheck file hashes before
the next mutation as already required, then continue ONLY stages 2-5 and the two
specified evidence records. Preserve mandatory restoration and single launches.

Disclose the capacity interruption and preflight deviations: original invocation
attempted bare rustup/rustc, then used a shell-local PATH export and rustc version
probe despite the prohibition. It also requested full config reads/directory
listings; no config content was returned. Do not repeat those actions. The absolute
HOME rustup Cargo command for stage 1 itself was submitted exactly as authorized.
Original execution authorization is cced19c8 and observed checkpoint is 650a4a98;
record this continuation amendment's commit separately.

Read AGENTS.md, TESTING.md, docs/engineering/HERMES_JR_DEV_ROUTING.md, this handoff,
the active CURRENT_TASK.md prefix, and the two bounded reviews:

- docs/testing/BBD-WAL-009-CLOCK-READ-ORACLE-SOURCE-REVIEW-01.md
- docs/testing/BBD-WAL-009-RETAINED-SPEND-VALIDATION-STOP-REVIEW.md

Source reads are limited to the mutation anchors below, the corrected
cancellation/expiry test, software happy test, and external_binding_tests.rs.
Do not reload historical handoffs, discover files, inspect other actors, or
rerun any command from the prior integration attempt.

## Preflight and writable boundaries

Record one hermes --version output, the supplied current session ID's actual
model/provider, and these three read-only Git results (once each):
`git rev-parse HEAD`, `git status --short --untracked-files=all`,
`git diff --cached --name-only`. The index must be empty. Authorization and the
reviewer's later launch checkpoint are different commits; record both accurately.
No Git log, toolchain/version probe, bare rustup/cargo attempt, or dependency action.
The HOME rustup path below is already resolved by prior executions; checking that
it remains executable is sufficient. Do not invoke rustc/cargo for preflight.

Measure all twenty-one paths below from actual bytes in one JSON object (full
SHA-256, newline count), compare every row, and match the exact dirty-path set.
The new resume evidence path must be absent. No expected-value-only evidence table.
Any mismatch, staged item, missing executable, or unexpected path stops before tests.

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
| wallet-broker/tests/zec_sign_verify.rs | 1118 | 7a481d3a954a92e04d324be047863a824ecf303bfd6232be70e9d13d3a295987 |
| wallet-broker/src/zec/spend/external_binding_tests.rs | 114 | bb2d6873dd7262ad6e30f117f1947b7d4cf7685ab04e32a5b700d7cb5b112b6d |
| docs/testing/BBD-WAL-009-SIGNATURE-CONTEXT-VALIDATION-01.md | 677 | 6a23bcb2943df54953d1acddf85a2ec9cfc41b92e84c12856a48b7d07be53c58 |
| docs/testing/BBD-WAL-009-RETAINED-SPEND-VALIDATION-01.md | 287 | 7bd3f2f915b7f4f41c00f942e98e56d0677a8776828a93a3e78351ba63b3a4d9 |

Only spend.rs and prepare.rs may be temporarily changed, exactly as specified.
The test file is frozen. The named prior retained-spend evidence may be corrected
only in the final evidence phase; all other twenty paths stay at these identities.
The new record is docs/testing/BBD-WAL-009-RETAINED-SPEND-VALIDATION-RESUME-01.md.
No other persistent write, source backup, or scratch file is authorized.

Confirm the existing wallet-broker/target is the effective disk-backed ext4 target.
Narrow filesystem checks of repository/target/temporary storage are allowed. Check
only relevant target override variables (CARGO_TARGET_DIR/CARGO_BUILD_TARGET_DIR)
and build.target-dir in Cargo config files at the known cwd/ancestor and Cargo-home
locations. Do not dump configuration or environment; report only presence/absence
and the resolved target. An override or unsupported configuration stops. No find,
whole-directory listing, ps/top/strace, or configuration edits. Temporary storage
is tmpfs; do not use it for builds, backups, or evidence scratch.

## Single-execution capture protocol

For EACH numbered Cargo command, call terminal with the exact command string,
repository workdir, background=true and notify_on_complete=true FROM THE START.
Do not launch any numbered command in the foreground. Do not add shell operators,
redirection, pipes, wrappers, environment assignments, or status echoes.
The background launch acknowledgement is not the command's final exit code.
Wait on ONLY that returned process ID, with process.wait timeout 60 as needed.
No process.list, kill, second launch, or command rerun is allowed. A slow proving
test is not a reason to restart it; keep waiting on that same process.

After exit, retrieve that SAME process's complete log using process.log offset 0,
limit 1000. Require showing == total_lines (page by offset only if needed), and
retain the full output and the actual completion exit code from saved tool JSON.
The tail returned by wait is not a complete log. No acceptance stage proceeds
until its expected result and full capture are established. On timeout, unknown
process, incomplete capture, compile/fixture failure, zero/missing tests, wrong
panic, or unexpected result: restore any mutation, record the limitation, stop.
Never rerun to repair capture. Normal progress can be reported while waiting.

## Ordered tests and isolated falsifications

Previous metadata green (2 passed) and partial integration results (prepare 11,
other sign/verify 13 passed) are retained with the disclosed procedural deviations.
Do not rerun either full integration target. Every following command runs once;
the repeated focused command at stages 1 and 2 intentionally uses different source.

1. Corrected cancellation/expiry green, original production:

```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_sign_verify cancellation_and_expiry_are_reread_after_proving_at_exact_boundaries -- --exact
```

Require exit 0, exactly one passed, zero failed/ignored, 13 filtered. All four
scenarios (cancellation, before/at/after expiry) remain in that unchanged test.

2. Only after green, falsify the actual expiry guard. Require prepare.rs's full
baseline hash 44c0783fa3d75867599092d25912032b665838ed7a73c81a16b0eec2b26ffb07,
1238 lines. Replace exactly one complete line (eight leading spaces, newline):

```rust
        if parse_timestamp(now)? >= parse_timestamp(&expected.public.expires_at)? {
```

with:

```rust
        if parse_timestamp(now)? > parse_timestamp(&expected.public.expires_at)? {
```

Measure 1238 lines, mutated SHA-256
375071bacf80be0d750bc4436b7da3f1b1955bf6a3eb71b9b488428af5225744.
Every other byte stays unchanged. Then run the same focused test under this mutation:

```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_sign_verify cancellation_and_expiry_are_reread_after_proving_at_exact_boundaries -- --exact
```

Require exit 101, one executed/failed test, specifically unwrap_err on an Ok
verified result at tests/zec_sign_verify.rs:763 in the exact-expiry case. The
cancellation and just-before-expiry cases must have reached/passed their preceding
assertions. A clock-count failure or unrelated error does not satisfy this control.
Immediately reverse ONLY the replacement and verify the complete prepare.rs
baseline hash/1238 lines, on every expected or unexpected outcome. Stop after
restoration if unexpected. Never carry this mutation into the next stage.

3. After the focused green and required expiry falsification/restoration above,
falsify retained external index binding. Require
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
unwrap at zec_sign_verify.rs:165 receiving INTENT_MISMATCH. The prior software
happy-test pass on this production source plus the isolated injected guard is
the classification control; the prior whole integration run was not green.
Immediately reverse only this replacement and measure original SHA-256/1046 lines,
even on an unexpected outcome; stop after restoration on any unexpected result.

For all three mutations, require exact unique old text and full starting/mutated hash.
Never use Git restore/checkout/reset, source backups, formatter, or speculative
replacements. If the file matches neither baseline nor the authorized mutation,
disclose the restoration blocker without overwriting unrecognized changes.
Restoration is mandatory before leaving a reached mutation stage.

5. Only after all three required failures and exact restoration, final library green:

```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --lib
```

Require exit 0, zero failed, both new binding tests and all four frozen signature
context tests passed. Record complete library counts and any ignored/filtered
cases; do not count ignored tests as passed. This includes the restored metadata
green; no extra focused rerun. Native-feature execution remains outside this task.

## Evidence correction, new record, and stop

After the last reached stage, only mandatory restoration, a full twenty-one-path
measurement, read-only Git status, bounded saved-data retrieval, and the two named
evidence writes are allowed. All twenty frozen paths must match. No Git mutation,
staging, commit, push, source/test repair, extra gate, formatter/lint/audit, network,
native launch, mainnet/broadcast/Monero, or other actor.

Generate evidence programmatically in memory from actual saved JSON. Do not retype
outputs, manually reconstruct diagnostics, or put source/evidence backups in scratch.
Use SQLite Path.home()/".hermes"/"state.db" via as_uri()+"?mode=ro", uri=True.
Current session queries may retrieve its model/billing_provider and the needed
version, preflight, measurements, mutation/restoration, gate and full-log tool
calls/results. Do not query other sessions except the exact correction inputs below.
Read-only schema PRAGMA for sessions/messages is allowed only if needed. Never
select recent sessions, prompts, credentials, or whole configuration.

Correct docs/testing/BBD-WAL-009-RETAINED-SPEND-VALIDATION-01.md, starting from its
287-line hash above. Preserve the original record as historical content and add a
clear leading correction notice that its incomplete execution/compliance claims
are superseded by a new correction section. Add the full normalized 78-line final
log from prior session 20260908_102051_01693b, message 77962, paired with exit 101
from 77956 and exact original command from 77885. Record authorization b513dbd2,
observed checkpoint 897aab48, and all material errata from the stop review.

Prior-session access is restricted to session identity and message IDs
77811, 77812, 77832, 77837, 77838, 77841, 77842, 77843, 77844, 77845, 77846,
77847, 77848, 77883, 77884, 77885, 77886, 77955, 77956, 77961, 77962, 77990.
These establish the original pass, three integration launches (including the piped
run and its kill), complete third-run failure, version, and measurements. Do not
start, resume, or inspect live state of those completed processes. The stop review
supplies other procedural findings; no further session-wide discovery is needed.
The correction must not relabel the 2000-character tail as complete, the checkpoint
as authorization, or claim identical full diagnostics for timed-out/killed runs.
Keep the old test baseline (1117 lines) attributed to that historical execution;
the current 1118-line test correction belongs to this resume's source table.

The new docs/testing/BBD-WAL-009-RETAINED-SPEND-VALIDATION-RESUME-01.md must record:
actual current runtime/version/session, authorization and observed HEAD, actual
preflight and full before/after measurements, each reached exact command and
completion exit code/full output, counts/expected panic location, measured mutation
and restoration identities, every unrun stage, all deviations, and the prior
correction's final hash/line count. Link the corrected prior report for retained
results; disclose reuse of its verified partial passes and rejected procedure.
A partial stop is not full validation acceptance. Report final identities of both
records; all source/other evidence identities must match the preflight.

Normalize only local prefixes to <repo>/, <cargo-registry>/, <home>/, preserving
all other output bytes and relative target paths. Commands retain literal quoted
HOME spelling. No raw PCZT/transaction/key/seed/receiver/memo/proof/signature dumps.
No further execution after the final result/evidence step. Report reached stages,
counts, restoration and record identities, then stop. Reviewer collects once
only after done/explicit collection. No actor polling or duplicate launch.
