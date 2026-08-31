# BBD-WAL-006 Argon2 Correction Expected Red 01

Timestamp: 2026-08-31T12:31:43-07:00 (America/Los_Angeles)

Protected governance parent: `78a3a777`

The focused command was run exactly once from the repository root:

```text
node -e 'const name = "WAL-004 Rust test-harness manifest pins the exact toolchain, dependencies, and native features"; const matches = require("./test/securityPolicy.node.js").tests.filter((entry) => entry.name === name); if (matches.length !== 1) throw new Error(`expected one test, found ${matches.length}`); matches[0].fn();'
```

Result: exit `1`, exact expected red. The named test was found exactly once and failed
at `test/securityPolicy.node.js:1657` on its first `assert.deepStrictEqual`, before
manifest validation or mutation. The frozen production dependency map contains
`argon2` version `=0.6.0`, while the accepted test expects `=0.5.3`. The corrected
`hkdf =0.12.4`, `sha2 =0.10.9`, and every other field match exactly.

No canary or secret appeared. No mutation was reached. No manifest, production policy,
Rust source/test, lockfile, fixture, wallet, node, device, network, or other command
was used.

Pre/post hashes are unchanged:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `test/securityPolicy.node.js` | 2,381 | `636eeae934dd3691a9a214275c40c4701feec07aa03ed868c4fa7cd3c5f8bd77` |
| `wallet-broker/Cargo.toml` | 79 | `da7639af39bbc578936f0d38fcc32317fbacb279e53040cad4b3543c67a54294` |
| `scripts/security-policy.js` | 2,231 | `627eaf04248ab744e8c4300f3cae3a34d114a69bad88dfa4de04492dea537d4d` |
| `wallet-broker/Cargo.lock` | 3,273 | `1d182550f82b9dc443804d82e709b5d58021d4243bef1612c440aac59df1c2f5` |

The six formatted Rust test paths remain at their accepted hashes from the format-
correction review.
