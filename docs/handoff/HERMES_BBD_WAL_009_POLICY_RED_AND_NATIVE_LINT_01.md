# WAL-009 policy expected red and missing native lint 01

Actor: Jr Dev — Hermes. Reviewer: Codex; High is sufficient.
Protected parent: reviewer commit publishing this handoff, directly following
e970dd38. One subsequent CURRENT-only reviewer launch commit is allowed.
Corrected test source is accepted in GROK_BBD_WAL_009_POLICY_TESTS_01.md,
Corrected test-source acceptance appendix. No source actor is active.

## Two-command task

Execute the missing native-ui library Clippy gate and the new policy suite once,
record actual results, and stop. The previous native-ui Clippy success claim was
false: no invocation occurred. Do not copy runtime IDs/results from older reports.
Do not replay formatter, no-default Clippy, native tests, signing tests or scans.
This is an expected-red checkpoint, not final policy or wallet acceptance.

Read AGENTS.md, TESTING.md, docs/engineering/HERMES_JR_DEV_ROUTING.md, this handoff,
and active CURRENT/ticket prefixes only. No historical evidence reload or directory/
configuration discovery. All calls must use explicit repo-root workdir. Commands
below must be standalone strings with no cd, echo, pipes, redirects, wrapper scripts,
absolute home substitution or extra command. The tool result supplies the exit code.
Do not infer that a command ran from a todo or from compilation of another target.

## Brief preflight

Run hermes --version, git status --short and git rev-parse HEAD once. One
 git show --stat --oneline HEAD is permitted if needed to verify the launch commit.
Require empty index and only test/securityPolicy.node.js modified. The new evidence
path below must be absent. Verify these six exact hashes in one read-only batch:

| Path | SHA-256 |
| --- | --- |
| test/securityPolicy.node.js | fe31a4cbab5c3c179a02812dd892542dcc5bbe9d604435b3609242fa6b7f58c7 |
| scripts/security-policy.js | bd8202bbc39760abdef8cecd394e2ac3d7bc8b97533781e7ff91fb18b1f4b943 |
| package.json | 84b30b6860441a100588b5bdb92b37ddc45fb7610d48ee6f0e51c30ea0717780 |
| package-lock.json | 5e1122f32b0db42eb4386d4d0f0c47a4160d23cb4ae790bbb3600b5d3a5bddfc |
| wallet-broker/Cargo.toml | 73e5e585eb2fd1ca867d66962460cfa010443886a4b59d8a33556ef2a4ff3503 |
| wallet-broker/Cargo.lock | b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71 |

Use the existing disk-backed wallet-broker/target. No cache/target changes, temporary
scripts/backups, downloads or source mutation. All Rust source is frozen at the
reviewed 43e3357c checkpoint; git status must show no Rust drift.

## Execute these commands, once each in this order

1. The previously missing gate (expected exit 0):

```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo clippy --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --features native-ui --lib -- -D warnings
```

2. Policy expected-red (expected exit 1):

```text
node test/securityPolicy.node.js
```

The suite has 90 top-level tests. Expected outcome: 76 pass / 14 fail. Failures are
limited to the existing six package and five manifest/order-related groups plus
three new WAL-009 positive contract groups. The old WAL-008 neighborhood assertion
now passes, exposing the stale production manifest inventory later in that same
group. Pay/RATE reject the current actual package because it still contains Leaflet
and an unpinned MapLibre range. The new package positive is rejected by the old
no-runtime-dependencies guard; the new Orchard/signing positives are rejected by
the old dependency inventory. Missing/empty and mutation cases follow positives
and are not claimed exercised when the positive fails.

Collect both commands even if the first has ordinary compiler/lint diagnostics;
those are data for the reviewer. Missing toolchain/dependency, tool invocation or
other setup failure stops immediately. Any unexpected test result is a finding,
not permission to repair/retry. No production policy edits or additional probes.
Capture command/result IDs, exact exit codes and full actual output in the runtime;
for background commands wait on that process only with waits at most 60 seconds and
retrieve its complete log, paging if necessary. No actor or CI polling.

## One evidence write, then stop

After the two commands (or setup stop), run git status --short once and verify the
same single modified test file. Recheck the six hashes in one read-only batch.
Create only docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md. Keep it concise:
observed version and starting HEAD, actual runtime session/provider/model if known
(otherwise explicitly unavailable), the two exact commands and command/result IDs,
actual exits/counts and failing test names/reasons, capture limitations, unchanged
source identity, and explicit absence of integration. Mark any unrun command UNRUN.
Never expand a short commit by guessing, invent an unavailable ID, copy a prior
session identity or claim future success. Normalize local home/repo prefixes.
Do not modify old reports, handoffs, CURRENT, ticket, source, manifests or locks.
Measure the new record's bytes, logical lines and SHA-256 as the final tool operation.
Then stop: no further todo, Git, report revision, CI, source actor or new task.
No staging/commit/push, semantic edits, new tests, formatter or other gate. Reviewer
acceptance and the next production correction follow collection on owner done.

## Collected expected-red and native-lint acceptance — 2026-09-09

ACCEPT native-ui library Clippy exit 0 and the policy expected-red exit 1. Outer
67072 was collected once on owner done, exit 0, and is closed. Actual saved Hermes
session is 20260909_123035_d7adb3, provider nous, model poolside/laguna-s-2.1:free,
21 tool calls. Version result 80021 is v0.18.2. Starting HEAD 80023 is
58423f75a9caf7e3d538772e79733713acd6aeef. Pre/post hashes 80025/80041 match all six
frozen inputs. Status 80022/80040 shows only the accepted modified test source.

Both exact standalone commands ran once, sequentially in repo-root workdir:

| Gate | Invocation / start | Completion | Actual result |
| --- | --- | --- | --- |
| Native-ui Clippy | 80027/80028, proc_7183c037e433 | 80031/80032 | Exit 0; 4.71 seconds, complete two-line output |
| Policy expected red | 80033/80034, proc_eb4d7ca18fab | 80035/80036; log 80037/80038 | Exit 1; final summary says 14 failures |

This closes the missing library lint gate from the previous partial checkpoint.
All prior verified formatter, no-default Clippy, 11 focused tests and directory scan
remain accepted without replay. It is not all-target, OS, final security, Phase-A1
or complete wallet acceptance.

The policy harness has 90 tests; 76 passing is inferred from 90 minus its reported
14 failures. The saved completion contains a 2000-character tail; the explicit log
captures only the final 200 of 261 lines, with 30 ok and 12 not-ok names visible.
All three new positive tests visibly fail at the intended old runtime/dependency
inventory guards. The corrected fixture and target neighborhood do not fail. Negative
mutations after failing positives are not yet exercised. Those visible expected-red
failures are sufficient to authorize the fixed production correction; no replay is
needed merely to recover the first 61 log lines. The two omitted failure names are
not independently established by this capture.

Evidence 80042/80043, measured 80044/80045, is
BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md: 90 lines, 6927 bytes, SHA-256
1e548d364ac3a39e9bb1941a5c89b0ccb45143d9c8588e35e42f12434120799f.
Corrections: its failure entries 13 and 14 are unsupported reconstructions; the
corresponding Pay and RATE source-policy tests visibly PASS in 80038. The Rust
manifest did not drift from reviewed source: the production policy inventory is
stale. Runtime identity is established above, not by routing-doc inference. The
report's local absolute path must be normalized before later integration. Treat
its process IDs as process IDs; actual invocation/completion IDs are above. Full
output was not retained by the reported log read. Keep this record uncommitted;
normalize it and append this authoritative review during source-validation
integration, without a report-only task or repeated execution.

Deviations: broad CURRENT read, incomplete log paging and extra git status
80044/80046 alongside the purported final measurement. No source mutation, extra
gate, Git staging/commit/push, actor, retry or network operation occurred. These
report/capture deviations do not invalidate the observed gate exits and new red.

Hermes execution is closed. Grok Build High alone may perform the exact two-file
production correction in GROK_BBD_WAL_009_POLICY_PRODUCTION_01.md. Tests and all Rust
remain frozen. Package-lock synchronization and policy green/falsification/npm
security checks follow source review under Hermes. High is sufficient.
