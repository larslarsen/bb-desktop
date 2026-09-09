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
