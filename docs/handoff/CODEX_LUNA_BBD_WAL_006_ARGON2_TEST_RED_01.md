# Codex Luna Handoff — BBD-WAL-006 Argon2 Expected Red 01

You are **Jr Dev — Codex Luna**, using `gpt-5.6-luna`. This durable file is the complete
integration prompt; ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely before acting: `AGENTS.md`, `TESTING.md`, roles, `tickets/BBD-WAL-006.md`,
the second dependency-resolution review, this Argon2 test-source review, and the complete
accepted Node test, manifest, production policy, and lockfile.

Require `HEAD == origin/master` at the protected governance parent, a clean index,
`git diff --check`, and exactly nine uncommitted paths: accepted Node test, manifest,
production policy, and six Rust tests. Require:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `test/securityPolicy.node.js` | 2,381 | `636eeae934dd3691a9a214275c40c4701feec07aa03ed868c4fa7cd3c5f8bd77` |
| `wallet-broker/Cargo.toml` | 79 | `da7639af39bbc578936f0d38fcc32317fbacb279e53040cad4b3543c67a54294` |
| `scripts/security-policy.js` | 2,231 | `627eaf04248ab744e8c4300f3cae3a34d114a69bad88dfa4de04492dea537d4d` |
| `wallet-broker/Cargo.lock` | 3,273 | `1d182550f82b9dc443804d82e709b5d58021d4243bef1612c440aac59df1c2f5` |

The six Rust test hashes must match the format-correction review. Stop on mismatch.

Run exactly once from repository root:

```text
node -e 'const name = "WAL-004 Rust test-harness manifest pins the exact toolchain, dependencies, and native features"; const matches = require("./test/securityPolicy.node.js").tests.filter((entry) => entry.name === name); if (matches.length !== 1) throw new Error(`expected one test, found ${matches.length}`); matches[0].fn();'
```

Expected exit is 1 from the first `assert.deepStrictEqual`. The sole map difference must
be frozen production `argon2 =0.6.0` versus accepted test `argon2 =0.5.3`; the corrected
HKDF/SHA-2 values and every other field must match. The test must be found exactly once
and must not reach manifest validation or mutation. Any other difference, syntax/load
error, unexpected pass, abort, signal, canary, or secret is unintended red; stop.

Do not run complete Node, Rust, Cargo, npm, formatter, linter, build, scanner, resolution,
network, fixture, wallet, node, device, or any unlisted command. Do not edit any source,
test, manifest, policy, lockfile, fixture, workflow, package, ticket, or handoff.

On exact red only, create
`docs/testing/BBD-WAL-006-ARGON2-CORRECTION-EXPECTED-RED-01.md` with timestamp/timezone,
protected parent, exact command/status, actual/expected Argon2 values, assertion
site/reason, no-canary result, and pre/post hashes. Update only `CURRENT_TASK.md` to
`ARGON2 PIN CORRECTION EXPECTED RED RECORDED — REVIEW REQUIRED` and link the evidence.

Run `git diff --check`. Stage only the accepted Node test, evidence, and `CURRENT_TASK.md`;
inspect staged names/diff. Commit once as `test: record WAL-006 argon2 correction red`
and push `master`. Leave exactly eight uncommitted paths: manifest, production policy,
and six Rust tests, all hash-exact. Require final `HEAD == origin/master` and clean index.
Report commit, evidence count/hash, exact exit/reason, hashes/status, and blockers. Stop
without production correction or resolution.
