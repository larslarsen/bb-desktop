# Codex Sol Handoff — BBD-WAL-006 AEAD Production Correction 01

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This durable file is
authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read `AGENTS.md`, `TESTING.md`, roles, ticket, resolution review 03, AEAD test/red
reviews/evidence, and both authorized paths. Require protected `HEAD == origin/master`,
clean index, `git diff --check`, and exactly eight uncommitted paths: manifest, policy,
and six Rust tests. Require:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/Cargo.toml` | 79 | `34e953ca1418310856f4655fac4297b26551107e59e510968905709abfc09fdd` |
| `scripts/security-policy.js` | 2,231 | `5e4e4228266ff88c0fa80cda62795953d20c59d025156cd436429dc25ac8dbbf` |
| `test/securityPolicy.node.js` | 2,388 | `29cbe64a0a217a9a94eca23c52126b06174b79c547f512d8191282c0e00a4573` |
| `wallet-broker/Cargo.lock` | 3,273 | `1d182550f82b9dc443804d82e709b5d58021d4243bef1612c440aac59df1c2f5` |

The six Rust test hashes remain those in the format-correction review.

Edit only `wallet-broker/Cargo.toml` and `scripts/security-policy.js`. Make exactly three
version-literal changes, retaining every other byte:

1. manifest direct `chacha20poly1305` exact `=0.11.0` to exact `=0.10.1`;
2. production `WAL004_DIRECT_DEPENDENCIES.chacha20poly1305.version` likewise; and
3. the exact required manifest line in `checkWalletBrokerManifest` likewise.

Retain defaults off, only `alloc`, nonoptional state, order/targets, all accepted custody
pins, Zcash declarations, checker/export/error behavior. Add no alias, patch, override,
git/path dependency, feature, comment, or WAL-006 policy.

Use `apply_patch`; run no executable/resolution/network/fixture/Git/cleanup/device action
and touch no other path. After editing, only read-only `wc -l`, `sha256sum`, literal
inspection, and `git diff --check` over the two paths are allowed. Report exact
counts/hashes, three changes, frozen integrity, and blockers. Luna owns later execution,
evidence, integration, and Git.
