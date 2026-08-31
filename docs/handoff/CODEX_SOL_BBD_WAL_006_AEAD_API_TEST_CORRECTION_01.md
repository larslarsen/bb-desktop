# Codex Sol Handoff — BBD-WAL-006 AEAD API Test Correction 01

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This durable file is
authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read `AGENTS.md`, `TESTING.md`, roles, ticket, resolution review 04, the complete fetched
`aead 0.5.2`/`chacha20poly1305 0.10.1` API source, and the complete authorized test.

Require protected `HEAD == origin/master`, clean index, `git diff --check`, and exactly
nine uncommitted paths: manifest, production policy, resolved lockfile, and six Rust ZEC
tests. Require:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/tests/vault_crypto.rs` | 394 | `26475b2ccddd692b036e5440fdfde66d105f943f6bde912d81391efe7984b76e` |
| `wallet-broker/src/vault.rs` | 760 | `519b544e34cba1dcba240e93f03b19d05fb5137477bfb2441dd3a57c91c79a41` |
| `wallet-broker/Cargo.toml` | 79 | `89f02a18de21114d96c9f7231a36d8b60434c757e996314cb7f33403ae51c6a3` |
| `scripts/security-policy.js` | 2,231 | `3551656d49bfdc717fdff627d137b477582f274565880aae51a20922ebd28e83` |
| `test/securityPolicy.node.js` | 2,388 | `29cbe64a0a217a9a94eca23c52126b06174b79c547f512d8191282c0e00a4573` |
| `wallet-broker/Cargo.lock` | 5,367 | `bf9a36a70c96afe3c0b68355f082342c3e625e4764782228be9f4663b635cd83` |

The six ZEC Rust tests retain the format-correction hashes. Every path except the one
below is frozen.

Edit only `wallet-broker/tests/vault_crypto.rs` with `apply_patch`:

1. change only the trait import `AeadInOut` to `AeadInPlace`; and
2. in `xchacha20poly1305_draft_vector_is_independent`, change only
   `.encrypt_inout_detached(&nonce, &aad, plaintext.as_mut_slice().into())` to
   `.encrypt_in_place_detached(&nonce, &aad, plaintext.as_mut_slice())`.

Preserve every test name, input, vector byte, expected tag/ciphertext, assertion, count,
and all other bytes. Do not change production, manifest, policy, lockfile, ZEC test, or
dependency source.

Run no executable, test, formatter, resolution, network, fixture, Git, cleanup, wallet,
node, or device command. After editing, only read-only `wc -l`, `sha256sum`, exact diff,
and `git diff --check` over the one test are allowed. Report exact count/hash, the two
changes, unchanged 11-test count/vectors, frozen integrity, and blockers.
