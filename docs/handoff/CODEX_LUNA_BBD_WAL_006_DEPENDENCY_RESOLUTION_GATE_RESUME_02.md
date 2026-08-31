# Codex Luna Handoff — BBD-WAL-006 Dependency Resolution Gate Resume 02

You are **Jr Dev — Codex Luna**, using `gpt-5.6-luna`. This durable resume supplements
`CODEX_LUNA_BBD_WAL_006_DEPENDENCY_RESOLUTION_GATE_01.md`. The hashes and graph
requirements below supersede that handoff; every other preflight, disk/process boundary,
exact command and order, expected policy progression, inventory, evidence, Git boundary,
stop condition, and prohibition remains exact.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read the original gate, both resolution reviews, all first correction and Argon2
test/source/red reviews/evidence, `CURRENT_TASK.md`, and every accepted/frozen path
completely.

Require `HEAD == origin/master` at the protected parent, clean index,
`git diff --check`, and exactly eight uncommitted paths: accepted manifest, accepted
production policy, and six Rust tests. Superseding hashes are:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/Cargo.toml` | 79 | `34e953ca1418310856f4655fac4297b26551107e59e510968905709abfc09fdd` |
| `scripts/security-policy.js` | 2,231 | `5e4e4228266ff88c0fa80cda62795953d20c59d025156cd436429dc25ac8dbbf` |
| `test/securityPolicy.node.js` | 2,381 | `636eeae934dd3691a9a214275c40c4701feec07aa03ed868c4fa7cd3c5f8bd77` |
| `wallet-broker/tests/vault_crypto.rs` | 394 | `26475b2ccddd692b036e5440fdfde66d105f943f6bde912d81391efe7984b76e` |
| `wallet-broker/Cargo.lock` | 3,273 | `1d182550f82b9dc443804d82e709b5d58021d4243bef1612c440aac59df1c2f5` |

The six Rust test hashes remain those in the format-correction review.

Run the original focused policy command once. Dependency-map equality must now pass for
Argon2, HKDF, SHA-2, and every other field; the command must exit 1 only because the
still-WAL-004-only checker rejects the six WAL-006 dependency additions at
`scripts/security-policy.js:1840`. Stop on any other result.

Then run every original gate Cargo command separately, in its exact order, including
network-enabled `generate-lockfile` and `fetch`, the three locked/offline tree commands,
locked/offline metadata, and the complete locked/offline 11-test `vault_crypto` target.
Normal crates.io network escalation is authorized only for an exact sandbox-blocked
resolution/fetch command.

In addition to the original requirements, require exact direct `argon2 0.5.3` with
checksum `3c3610892ee6e0cbce8ae2700349fcf8f98adb0dbfbee85aec3c9179d29cc072`, resolved
`blake2 0.10.6` with checksum
`46502ad458c9a52b69d4d4d32775c788b7a1b85e8bc9d482d92250fc0e3f8efe`, and its stable
`digest 0.10` line. Stable `argon2 0.6.0`, `blake2 0.11.0`, `digest 0.11.2/0.11.3`,
`hmac 0.13.0`, and `sha2 0.11.0` must be absent. Zcash's exact
`digest 0.11.0-pre.9`, `hmac 0.13.0-pre.4`, and `sha2 0.11.0-pre.4` remain required.

All 11 custody tests must pass, including exact independent Argon2id RFC 9106,
HKDF-SHA256, XChaCha20-Poly1305, and deterministic encrypted-envelope vectors. On any
new graph contradiction or other stop condition, do not fetch further, test, repair,
substitute, generate fixtures, write evidence, integrate, or use Git; report and stop.

On exact success, use the original evidence path, current-task state, staging set, commit
message, push, final six-path state, and report requirements. Stop before fixture work.
