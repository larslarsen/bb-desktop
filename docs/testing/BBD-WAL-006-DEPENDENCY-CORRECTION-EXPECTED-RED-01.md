# BBD-WAL-006 Dependency Correction Expected Red 01

Timestamp: 2026-08-31T12:10:42-07:00 (America/Los_Angeles)

Protected governance parent: `7c9f588d`

The focused command was run exactly once from the repository root:

```text
node -e 'const name = "WAL-004 Rust test-harness manifest pins the exact toolchain, dependencies, and native features"; const matches = require("./test/securityPolicy.node.js").tests.filter((entry) => entry.name === name); if (matches.length !== 1) throw new Error(`expected one test, found ${matches.length}`); matches[0].fn();'
```

Result: exit `1`, exact expected red. The named test was found exactly once and failed
at `test/securityPolicy.node.js:1657` on its first `assert.deepStrictEqual`, before
manifest validation or any mutation. The frozen production dependency map contains
`hkdf` version `=0.13.0`, while the accepted test expects `=0.12.4`; it contains
`sha2` version `=0.11.0`, while the accepted test expects `=0.10.9`.

No canary or secret appeared. No mutation was reached. No manifest, production policy,
Rust source/test, lockfile, fixture, wallet, node, device, network, or other command
was used.

Pre/post hashes are unchanged:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `test/securityPolicy.node.js` | 2,374 | `f4f6defd5d55212c480a7a1a87d37edea85786b6125cad29467ae5e1f3480ee4` |
| `wallet-broker/Cargo.toml` | 79 | `420559e61a7c81338c97301f7d470a8be6560c0f71aa9f666d79bbcc1424361d` |
| `scripts/security-policy.js` | 2,231 | `affe15ea73706becb6156409afae80ce44076641915457a9e2b9399207ed558f` |
| `wallet-broker/Cargo.lock` | 3,273 | `1d182550f82b9dc443804d82e709b5d58021d4243bef1612c440aac59df1c2f5` |

The six formatted Rust test paths remain at their accepted hashes from the format-
correction review.
