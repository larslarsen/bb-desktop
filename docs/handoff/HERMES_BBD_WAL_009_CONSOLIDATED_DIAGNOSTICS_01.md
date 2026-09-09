# WAL-009 consolidated diagnostics 01

Actor: Jr Dev — Hermes. Reviewer: Codex; High suffices for this fixed task.
Source checkpoint: 502580fc12bff1789e2ced2a0692b1b35de587a3.
Protected parent: reviewer commit publishing Resume 01 below. One following reviewer
launch commit may change only CURRENT_TASK.md. No source actor is active.

## Purpose

Collect formatter, production-library lint and repository security-policy findings
in one bounded pass so the reviewer can assign corrections together. This is a
diagnostic inventory, not a green acceptance gate or proof rerun. Nonzero command
exits caused by reported format/lint/compile/policy findings are expected data:
record them and continue through all four commands. A tool invocation failure,
missing executable/toolchain/dependency, network attempt or source drift is a setup
stop: report it without repair, retry or executing the remaining commands.

Full typed-secret erasure remains a separate design/release blocker. No diagnostic
result resolves it. Native OS integration and complete all-target/final security
acceptance are not claimed by these library-only diagnostics.

## Scope and short preflight

Read AGENTS.md, TESTING.md, docs/engineering/HERMES_JR_DEV_ROUTING.md, this handoff,
CURRENT_TASK.md active prefix only, and tickets/BBD-WAL-009.md active prefix only.
Do not reload historical handoffs, old evidence or configuration. Every terminal
call sets explicit repo-root workdir. Do not prepend cd, git -C, wrappers, redirection,
echo, pipes, or multiple shell commands. Do not create temporary scripts/listings.

Run each of these once as a separate command:

```text
hermes --version
git status --short
git rev-parse HEAD
```

Require a clean initial worktree and the protected parent or CURRENT-only launch
commit. If HEAD needs disambiguation, a single read-only git show --stat --oneline
HEAD is allowed. No 39-file inventory, old-tree comparison, configuration/remote
query, directory discovery, source editing, formatter mutation, test execution,
proof generation, dependency update/download, extra scanner, actor, native UI,
network/device/Monero operation, Git staging/commit/push or CI polling is allowed.
Use the existing disk-backed wallet-broker/target; do not change target/cache paths.
The sole authored path is docs/testing/BBD-WAL-009-CONSOLIDATED-DIAGNOSTICS-01.md.
Require it absent before writing; do not update CURRENT or any other file.

## Exactly four diagnostics, in this order

Submit each command exactly once, separately and sequentially. No retries after
ordinary diagnostic failures. Compiler/build-cache writes under the existing target
are expected; repository source and dependency files must remain unchanged.

```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo fmt --manifest-path wallet-broker/Cargo.toml --all -- --check
```

```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo clippy --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --lib -- -D warnings
```

```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo clippy --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --features native-ui --lib -- -D warnings
```

```text
node scripts/security-policy.js
```

Preserve actual command/result tool IDs, exits and full output in the saved runtime.
Foreground is fine for quick commands. For background calls, wait only on that
process with timeout <=60 seconds and retrieve complete process.log output at
completion, paging if necessary. Do not treat a clipped completion tail as full.
The reviewer collects this actor only after the owner signals; no actor polling.

## Evidence and stop

After all diagnostics (or an actual setup stop), run git status --short once before
creating evidence. Require no source/manifest/lock/policy/governance changes. Do not
repair unexpected changes. Record the observed state and stop if it is not clean.

Create the one concise evidence record after execution. Include measured version,
actual starting HEAD copied from tool output, all four exact commands and observed
exit codes (mark unrun commands as unrun), command/result/log IDs, diagnostic file/
line/message summaries, and the clean-source status result. Do not invent hashes,
expand short commits from memory, infer runtime identity from another run, or claim
a future result. If session/provider/model are unavailable, say unavailable for
reviewer verification. A concise summary is sufficient; raw saved tool output is
the authority and must not be replaced by a claimed verbatim reconstruction.
Normalize local root/home prefixes to <repo>/<home> before the single record write.

Measure that evidence file's line count, byte count and SHA-256 as the final tool
operation, then stop. No extra todo updates, Git commands, evidence correction run
or new task after that measurement. Report command exits, actionable findings,
evidence identity and explicit absence of functional test/proof execution.

## Capacity stop and existing CI review — 2026-09-09

Outer 44655 was collected once on owner done and is closed. The CLI reported
HTTP 429 upstream model capacity after its three internal API retries, before any
agent tool call. Saved session 20260909_090226_2eeaa8 records model
poolside/laguna-s-2.1:free and tool_call_count=0; billing_provider is unset, so no
provider identity is inferred for this failed run. The outer exit code is 0 but
is not successful task execution. No preflight, diagnostic, evidence write or
source change occurred. The diagnostic evidence file is absent and the worktree
was independently verified clean. All four commands remain unrun. No automatic
retry, provider change or polling is authorized. The previous launch authorization
is closed; a fresh reviewer launch authorization can reuse the four-command task
when the owner next requests continuation.

The reviewer made a read-only inspection of existing CI for source commit
502580fc12bff1789e2ced2a0692b1b35de587a3, without starting a workflow or local test:
[Social client run 34327240542](https://github.com/larslarsen/bb-desktop/actions/runs/34327240542),
check job 102387210886. It completed with failure. Build and social tests passed;
all 20 Electron security tests passed. The repository policy suite reported 11
failures and exited 1. All following wallet/Rust/formatter/Clippy/native steps were
skipped, so CI does not replace the unrun diagnostics.

The job log groups the failures as follows:

- Six package-policy tests reject added runtime dependencies: repository workflow,
  Gitleaks-ratchet, wallet contract, WAL-005 Pay, broker package, and RATE-001 package.
  The checkpoint contains leaflet ^1.9.4 and maplibre-gl ^6.8.0 in package.json.
  These were frozen checkpoint inputs; establish their purpose/provenance before
  any removal or policy exception. Do not blanket-allow runtime dependencies.
- Four manifest-policy tests reject the dependency list: WAL-004, WAL-006 direct
  pins, WAL-006 RNG/SQLite support and WAL-006 NFC normalization. Source inspection
  finds the reviewed orchard =0.15.5 defaults-off circuit dependency in Cargo.toml
  but absent from checkWalletBrokerManifest's exact expectedDependencies list.
- The WAL-008 target-order assertion still expects xmr_distribution immediately
  after zec_hardware; current Cargo.toml contains zec_sign_verify there instead.

This preserves useful failure evidence for the later consolidated correction
contract; no policy/source change is authorized now. Passing the secret scan and
accepting a partial source checkpoint did not imply CI or final wallet acceptance.
The exact four diagnostics remain available after the capacity stop. High remains
sufficient for their execution review; no reasoning-setting change is needed.

Reviewer publication scope for this stop: this handoff, CURRENT_TASK.md and
tickets/BBD-WAL-009.md only. No actor is running.

## Resume 01 — owner requested continuation, 2026-09-09

The owner has requested continuation after the capacity stop. Authorize one fresh
Hermes launch of the unchanged four-command task above. The previous attempt ran
zero tools, so no diagnostic is repeated. Worktree is clean and the sole evidence
path remains absent. Preserve the existing CI findings; they do not substitute
for these diagnostics. This resume supersedes the no-actor/no-retry state of the
historical capacity stop only for this one launch; no provider/model change,
automatic relaunch or polling is authorized. All original command, write-scope,
capture and final-stop rules remain unchanged. Protected parent is the commit
publishing this resume, with one CURRENT-only reviewer launch commit allowed.
High is sufficient. No functional signing tests or proofs are authorized.

## Collected diagnostics acceptance — 2026-09-09

ACCEPT the four observed diagnostic outcomes; this is not green acceptance.
Outer 8844 was collected once on owner done, exit 0, and is closed. Saved runtime
20260909_092601_7ccfbe records provider nous, model poolside/laguna-s-2.1:free.
Version result 79785 is Hermes Agent v0.18.2. Initial status 79786 and final source
status 79799 are clean. Starting/final HEAD results 79787/79801 agree with
cd4d3f04f376f3b560b55a141b0df827c0c07ef4. Reviewer confirmed no tracked source change;
the sole untracked path is the diagnostic evidence record.

All four exact command strings ran once, sequentially, in repo-root workdir:

| Diagnostic | Command / result IDs | Exit | Observed result |
| --- | --- | --- | --- |
| Formatter check | 79790 / 79791 | 1 | Formatting differences; eight paths visible |
| No-default library Clippy | 79792 / 79793 | 101 | 15 reported lint errors |
| Native-ui library Clippy | 79794 / 79795 | 101 | Same 15 plus unused native control geometry |
| Security-policy CLI | 79796 / 79797 | 1 | Runtime dependencies rejected by package policy |

Clippy outputs are retained (196 and 206 lines). Formatter capture is incomplete:
49911 characters / 1140 lines retained, with an explicit marker that 16460 characters
were omitted from 66387 total. The report confuses retained and omitted sizes and
must not be described as full output. No rerun is required to establish the format
failure; the later actual formatter/final check will address formatting. Visible
paths are native_ui/zec_native_app_tests.rs, zec/prepare.rs, zec/spend.rs,
zec/spend/cleanup_lifecycle_tests.rs, zec/spend/effects.rs,
zec/spend/verification_context_tests.rs, zec/test_support.rs and
wallet-broker/tests/zec_sign_verify.rs (src/ prefixes apply to the first seven).

The 16 distinct lint findings are exhausted by three unused items, native control
geometry, two collapsible conditions, four deliberate long argument lists, explicit
drop of a non-Drop unified key, three redundant closures, one needless return and
one len-without-is_empty helper. The bounded source contract is
GROK_BBD_WAL_009_RUST_LINT_CLEANUP_01.md. The key lifetime remains explicitly bounded;
this cleanup must not imply erasure of upstream copies. Existing policy/CI findings
remain open. The owner has been asked whether the mapping dependencies are intended;
no dependency removal or policy exception is authorized while that is unresolved.

Evidence write 79802/79803 and final measurement 79804/79805 created
BBD-WAL-009-CONSOLIDATED-DIAGNOSTICS-01.md: 7270 bytes, SHA-256
380278a26835d868139ad455041a7d25170dc03171485829f594f91a6d47db74.
It has 148 newline bytes / 149 logical splitlines (no final newline). The measurement
was the final tool operation. No functional test, proof, source edit or Git mutation
occurred. A read-only git log -3 and extra final HEAD query were outside the narrow
preflight/status list. The report omits IDs, retains a local install path, contains
contradictory provider prose, and claims full output it does not contain. Actual
runtime identity and capture limits above supersede that prose. Keep the artifact
uncommitted; normalize/correct it during the later code-validation integration,
without a separate report-only actor or repeated diagnostic pass.

Hermes execution is closed. Grok alone is authorized for the fixed five-path Rust
lint source cleanup. Formatter mutation, further execution and source integration
require the next reviewer handoff. High is sufficient for this task and review.
Reviewer publication scope: this handoff, CURRENT_TASK.md, tickets/BBD-WAL-009.md,
and GROK_BBD_WAL_009_RUST_LINT_CLEANUP_01.md only.
