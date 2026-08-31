# BBD-WAL-006 AEAD Correction Expected Red 01

Timestamp: 2026-08-31T12:45:32-07:00 (America/Los_Angeles)

Protected governance parent: `5a76925c`

The focused command was run exactly once from the repository root:

```text
node -e 'const name = "WAL-004 Rust test-harness manifest pins the exact toolchain, dependencies, and native features"; const matches = require("./test/securityPolicy.node.js").tests.filter((entry) => entry.name === name); if (matches.length !== 1) throw new Error(`expected one test, found ${matches.length}`); matches[0].fn();'
```

Result: exit `1`, exact expected red. The named test was found exactly once and failed
at `test/securityPolicy.node.js:1657` on its first `assert.deepStrictEqual`, before
manifest validation or mutation. The frozen production dependency map contains
`chacha20poly1305` version `=0.11.0`, while the accepted test expects `=0.10.1`.
Argon2, HKDF, SHA-2, and every other field match exactly.

No canary or secret appeared. No mutation was reached. No manifest, production policy,
Rust source/test, lockfile, fixture, wallet, node, device, network, or other command
was used.

Pre/post hashes are unchanged:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `test/securityPolicy.node.js` | 2,388 | `29cbe64a0a217a9a94eca23c52126b06174b79c547f512d8191282c0e00a4573` |
| `wallet-broker/Cargo.toml` | 79 | `34e953ca1418310856f4655fac4297b26551107e59e510968905709abfc09fdd` |
| `scripts/security-policy.js` | 2,231 | `5e4e4228266ff88c0fa80cda62795953d20c59d025156cd436429dc25ac8dbbf` |
| `wallet-broker/Cargo.lock` | 3,273 | `1d182550f82b9dc443804d82e709b5d58021d4243bef1612c440aac59df1c2f5` |

The six formatted Rust test paths remain at their accepted hashes from the format-
correction review.
