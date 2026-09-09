# WAL-009 consolidated diagnostics 01

Actor: Jr Dev — Hermes. Reviewer: Codex; High suffices for this fixed task.
Source checkpoint: 502580fc12bff1789e2ced2a0692b1b35de587a3.
Protected parent: reviewer commit publishing this handoff. One following reviewer
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
