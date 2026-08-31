# Codex Sol Handoff — BBD-WAL-006 Argon2 Production Correction 01

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This durable file is
the complete correction prompt; ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely before acting: `AGENTS.md`, `TESTING.md`, roles,
`tickets/BBD-WAL-006.md`, the second dependency-resolution review, the Argon2 test-source
review/red evidence/red review, and both authorized paths completely.

Require `HEAD == origin/master` at the protected parent, clean index,
`git diff --check`, and exactly eight uncommitted paths: manifest, production policy,
and six Rust tests. Require these starting hashes:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/Cargo.toml` | 79 | `da7639af39bbc578936f0d38fcc32317fbacb279e53040cad4b3543c67a54294` |
| `scripts/security-policy.js` | 2,231 | `627eaf04248ab744e8c4300f3cae3a34d114a69bad88dfa4de04492dea537d4d` |
| `test/securityPolicy.node.js` | 2,381 | `636eeae934dd3691a9a214275c40c4701feec07aa03ed868c4fa7cd3c5f8bd77` |
| `wallet-broker/Cargo.lock` | 3,273 | `1d182550f82b9dc443804d82e709b5d58021d4243bef1612c440aac59df1c2f5` |

The six Rust tests retain the format-correction hashes. All tests, Rust source, lockfile,
fixtures, and other paths are frozen.

Edit only:

- `wallet-broker/Cargo.toml`
- `scripts/security-policy.js`

Make exactly three version-literal changes, retaining every other byte:

1. manifest direct `argon2` from exact `=0.6.0` to exact `=0.5.3`;
2. production `WAL004_DIRECT_DEPENDENCIES.argon2.version` likewise; and
3. the exact Argon2 required manifest line in `checkWalletBrokerManifest` likewise.

Retain `default-features = false`, only `alloc`, optional state, dependency/order/test
targets, the accepted HKDF/SHA-2 correction, all Zcash pins/features, checker control
flow, exports, and errors. Add no alias, patch, override, git/path dependency, feature,
comment, or WAL-006 policy.

Use `apply_patch`. Do not run executable, resolution, network, fixture, Git, GitHub,
cleanup, wallet, node, or device commands. Do not touch an unlisted path. After editing,
only read-only `wc -l`, `sha256sum`, exact-literal inspection, and `git diff --check`
over the two paths are allowed. Report both line counts/hashes, exactly three version
changes, frozen hashes, and blockers. Luna owns the resolution/custody gate, evidence,
integration, and Git after reviewer acceptance.
