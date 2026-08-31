# Codex Luna Handoff — BBD-WAL-006 AEAD Expected Red 01

You are **Jr Dev — Codex Luna**, using `gpt-5.6-luna`. This durable file is authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read `AGENTS.md`, `TESTING.md`, roles, ticket, resolution review 03, this test-source
review, and the complete accepted Node test/manifest/policy/lockfile. Require protected
`HEAD == origin/master`, clean index, `git diff --check`, and exactly nine uncommitted
paths: Node test, manifest, policy, and six Rust tests. Require:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `test/securityPolicy.node.js` | 2,388 | `29cbe64a0a217a9a94eca23c52126b06174b79c547f512d8191282c0e00a4573` |
| `wallet-broker/Cargo.toml` | 79 | `34e953ca1418310856f4655fac4297b26551107e59e510968905709abfc09fdd` |
| `scripts/security-policy.js` | 2,231 | `5e4e4228266ff88c0fa80cda62795953d20c59d025156cd436429dc25ac8dbbf` |
| `wallet-broker/Cargo.lock` | 3,273 | `1d182550f82b9dc443804d82e709b5d58021d4243bef1612c440aac59df1c2f5` |

The six Rust test hashes remain those in the format-correction review.

Run exactly once:

```text
node -e 'const name = "WAL-004 Rust test-harness manifest pins the exact toolchain, dependencies, and native features"; const matches = require("./test/securityPolicy.node.js").tests.filter((entry) => entry.name === name); if (matches.length !== 1) throw new Error(`expected one test, found ${matches.length}`); matches[0].fn();'
```

It must exit 1 at the first dependency-map `assert.deepStrictEqual`; the sole difference
must be production `chacha20poly1305 =0.11.0` versus test `=0.10.1`. Argon2, HKDF,
SHA-2, and every other field must match. It must not reach manifest validation/mutation.
Any other result is unintended red; stop.

Run no other executable/Node/Rust/Cargo/npm/build/scanner/resolution/network/fixture/
wallet/node/device command and edit no source/test/policy/manifest/lock/fixture path.

On exact red create only
`docs/testing/BBD-WAL-006-AEAD-CORRECTION-EXPECTED-RED-01.md` with timestamp/timezone,
parent, exact command/status, actual/expected AEAD values, assertion site/reason,
no-canary result, and pre/post hashes. Update only `CURRENT_TASK.md` to
`AEAD PIN CORRECTION EXPECTED RED RECORDED — REVIEW REQUIRED` and link evidence.

Run `git diff --check`; stage only Node test, evidence, and `CURRENT_TASK.md`; inspect.
Commit `test: record WAL-006 AEAD correction red` and push. Leave exactly eight
hash-exact uncommitted paths: manifest, policy, and six Rust tests. Require final
`HEAD == origin/master`, clean index, and report commit/evidence/hash/result/status.
Stop before production correction or resolution.
