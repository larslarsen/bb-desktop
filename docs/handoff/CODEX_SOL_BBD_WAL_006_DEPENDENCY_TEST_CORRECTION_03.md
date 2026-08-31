# Codex Sol Handoff — BBD-WAL-006 Dependency Test Correction 03

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This durable file is
the complete correction prompt; ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely before acting: `AGENTS.md`, `TESTING.md`, roles,
`tickets/BBD-WAL-006.md`, all three dependency-resolution reviews, all prior dependency
test/source/red reviews/evidence, and the complete current Node test.

Require `HEAD == origin/master` at the protected parent, clean index,
`git diff --check`, and exactly eight uncommitted paths: manifest, production policy,
and six Rust tests. Frozen hashes:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `test/securityPolicy.node.js` | 2,381 | `636eeae934dd3691a9a214275c40c4701feec07aa03ed868c4fa7cd3c5f8bd77` |
| `wallet-broker/Cargo.toml` | 79 | `34e953ca1418310856f4655fac4297b26551107e59e510968905709abfc09fdd` |
| `scripts/security-policy.js` | 2,231 | `5e4e4228266ff88c0fa80cda62795953d20c59d025156cd436429dc25ac8dbbf` |
| `wallet-broker/Cargo.lock` | 3,273 | `1d182550f82b9dc443804d82e709b5d58021d4243bef1612c440aac59df1c2f5` |

The six Rust tests retain the format-correction hashes. Every path except the one below
is frozen.

Edit only `test/securityPolicy.node.js`:

1. Change test-side `WAL004_DIRECT_DEPENDENCIES.chacha20poly1305.version` from exact
   `=0.11.0` to exact `=0.10.1`, retaining defaults off, only `alloc`, and nonoptional.
2. Add two complete-line mutations to the existing unique/changed-bytes mutation array:
   corrected exact `=0.10.1` to superseded exact `=0.11.0`, and corrected exact
   `=0.10.1` to loose `0.10`.
3. Remove only the now-obsolete later generic mutation
   `['version = "=0.11.0"', 'version = "=0.10.0"']`; its exactness coverage is strictly
   strengthened by the new complete-line cases.

Preserve every other assertion, mutation, dependency, feature, Zcash expectation, regex,
and all 73 test cases. Do not edit manifest or production policy.

Use `apply_patch`. Do not run executable, resolution, network, fixture, Git, GitHub,
cleanup, wallet, node, or device commands; do not touch another path. After editing,
only read-only `wc -l`, `sha256sum`, test-count/literal inspection, and
`git diff --check` over the authorized path are allowed. Report exact count/hash,
unchanged test count, both mutations, obsolete-pair removal, frozen hashes, and blockers.
