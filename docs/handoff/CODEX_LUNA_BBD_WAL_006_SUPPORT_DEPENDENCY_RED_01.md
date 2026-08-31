# Codex Luna Handoff — BBD-WAL-006 Support-Dependency Expected Red 01

You are **Jr Dev — Codex Luna**. This durable file is the complete prompt; ephemeral
chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff.

Accepted uncommitted test source:

- `test/securityPolicy.node.js`
- 2,454 lines
- SHA-256 `f8340ae965a1f59e27594153cd3c9f3eca5ee1d8a78ed0a9a685faf6f1dc2647`
- 74 named tests

Read completely: `AGENTS.md`, `TESTING.md`, roles, `tickets/BBD-WAL-006.md`, the address
production stop review, support-dependency test handoff/review,
`docs/handoff/CURRENT_TASK.md`, the complete changed Node test, and the current manifest
and policy implementation.

## Preflight and execution

Require `HEAD == origin/master` at the protected governance parent, a clean index, and
exactly the one accepted modified test path with its line count/hash. Require that the
manifest still omits direct `rand_core` and `rusqlite` lines, the lockfile is unchanged,
and no `wallet-broker/src/zec*` path exists.

Run exactly once from repository root:

```text
node test/securityPolicy.node.js
```

Expected result is exit 1 with exactly 66 `ok`, eight `not ok`, and final line
`8 security policy test(s) failed`. The seven previously accepted failure groups must be
unchanged. The eighth must be exactly
`WAL-006 manifest pins the exact production support APIs for RNG and SQLite`, failing
because the real manifest contains neither required exact support line. It must fail at
the exact-line assertion before policy-export validation. No previous `ok` case may
regress. Syntax, exception-outside-test, fixture, Rust, module-resolution, or other-path
failure is a stop.

## Evidence and integration

If and only if exact, create only
`docs/testing/BBD-WAL-006-SUPPORT-DEPENDENCY-EXPECTED-RED-01.md` recording the protected
parent, exact command/status/counts/names/final line, the new failure assertion/cause,
test line/hash/count, unchanged manifest/lock hashes, absent ZEC source inventory, and
final state. Do not record secret/canary values.

Update only `docs/handoff/CURRENT_TASK.md` to `SUPPORT-DEPENDENCY EXPECTED RED RECORDED —
REVIEW REQUIRED`, link the evidence, and keep manifest/lock/policy/source frozen.

Stage exactly the accepted test, evidence, and current-task update. Inspect the staged
manifest/full diff, commit once as `test: require WAL-006 support dependencies`, and push
`master`. Require `HEAD == origin/master` and a clean tracked worktree/index. Report the
commit, staged paths, exact counts, evidence/test hashes and lines, unchanged
manifest/lock hashes, push, and final state.

Do not edit or run the manifest, lockfile, policy implementation, Rust source/test,
fixtures, npm, Cargo, Rust, other Node tests, builds, formatters, linters, scanners,
Electron, wallets, nodes, devices, network clients, cleanup, or any unlisted path.
