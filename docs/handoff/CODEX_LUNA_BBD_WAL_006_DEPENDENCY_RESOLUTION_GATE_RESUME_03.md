# Codex Luna Handoff — BBD-WAL-006 Dependency Resolution Gate Resume 03

You are **Jr Dev — Codex Luna**, using `gpt-5.6-luna`. This durable resume supplements
the original dependency-resolution gate and Resume 02. These hashes and AEAD graph
requirements supersede them; every other exact preflight, disk/process rule, focused
policy command/result, Cargo command/order, inventory, evidence, Git boundary, stop
condition, and prohibition remains exact.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read the original gate, both prior resumes, all three resolution reviews, all dependency
test/source/red reviews/evidence, `CURRENT_TASK.md`, and every accepted path completely.

Require protected `HEAD == origin/master`, clean index, `git diff --check`, and exactly
eight uncommitted paths: manifest, production policy, and six Rust tests. Superseding
hashes:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/Cargo.toml` | 79 | `89f02a18de21114d96c9f7231a36d8b60434c757e996314cb7f33403ae51c6a3` |
| `scripts/security-policy.js` | 2,231 | `3551656d49bfdc717fdff627d137b477582f274565880aae51a20922ebd28e83` |
| `test/securityPolicy.node.js` | 2,388 | `29cbe64a0a217a9a94eca23c52126b06174b79c547f512d8191282c0e00a4573` |
| `wallet-broker/tests/vault_crypto.rs` | 394 | `26475b2ccddd692b036e5440fdfde66d105f943f6bde912d81391efe7984b76e` |
| `wallet-broker/Cargo.lock` | 3,273 | `1d182550f82b9dc443804d82e709b5d58021d4243bef1612c440aac59df1c2f5` |

The six Rust tests retain the format-correction hashes.

Run the original focused policy command exactly once. All corrected dependency-map
fields must match; it must exit 1 only when the frozen checker rejects the six WAL-006
manifest additions at `scripts/security-policy.js:1840`. Then run all original gate
Cargo commands in exact order: crates.io resolution/fetch, three locked/offline trees,
locked/offline metadata, and the complete locked/offline 11-test `vault_crypto` target.
Normal escalation applies only to an exact sandbox-blocked resolution/fetch command.

Retain every Resume-02 graph requirement. Additionally require:

- exact direct `chacha20poly1305 0.10.1`, checksum
  `10cd79432192d1c0f4e1a0fef9527696cc039165d729fb41b3f4f4f354c2dc35`;
- resolved `aead 0.5.2`, checksum
  `d122413f284cf2d62fb1b7db97e02edb8cda96d769b16e443a4f6195e35662b0`;
- resolved `chacha20 0.9.1`, checksum
  `c3613f74bd2eac03dad61bd53dbe620703d4371614fe0bc3b9f04dd36fe4e818`;
- resolved `cipher 0.4.4`, checksum
  `773f3b9af64447d2ce9850330c473515014aa235e6a783b02db81ff39e4a3dad`;
  and the stable `crypto-common 0.1` AEAD/cipher line; and
- absence of stable direct `chacha20poly1305 0.11.0`, stable `aead 0.6`, and stable
  `crypto-common 0.2.1/0.2.2`; Zcash's exact `crypto-common 0.2.0-rc.1` remains required.

All 11 custody tests must pass, including the exact independent Argon2id, HKDF-SHA256,
XChaCha20-Poly1305, and deterministic envelope vectors. On any further contradiction or
stop condition, halt without later command, repair, substitution, evidence, integration,
Git, or fixture work.

On full success, use the original gate's evidence path/current state/staging set/commit/
push/final-six-path/report contract, then stop before fixtures.
