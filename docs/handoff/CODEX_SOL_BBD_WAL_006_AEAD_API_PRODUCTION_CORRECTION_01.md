# Codex Sol Handoff — BBD-WAL-006 AEAD API Production Correction 01

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This durable file is
authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read `AGENTS.md`, `TESTING.md`, roles, ticket, dependency-resolution review 04, AEAD API
test-source review 02, expected-red evidence 02, expected-red review 02, and complete
`wallet-broker/src/vault.rs`. Require protected `HEAD == origin/master`, clean index,
`git diff --check`, and exactly nine uncommitted paths: manifest, policy, lock, and six
ZEC tests. Require:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/vault.rs` | 760 | `519b544e34cba1dcba240e93f03b19d05fb5137477bfb2441dd3a57c91c79a41` |
| `wallet-broker/tests/vault_crypto.rs` | 394 | `a83bc4d1bf30201ad8c9bfa09556e8bae029b8eefebf2eeeab4db5fed03e561b` |
| `wallet-broker/Cargo.toml` | 79 | `89f02a18de21114d96c9f7231a36d8b60434c757e996314cb7f33403ae51c6a3` |
| `scripts/security-policy.js` | 2,231 | `3551656d49bfdc717fdff627d137b477582f274565880aae51a20922ebd28e83` |
| `wallet-broker/Cargo.lock` | 5,367 | `bf9a36a70c96afe3c0b68355f082342c3e625e4764782228be9f4663b635cd83` |

The six ZEC tests remain at the exact hashes in format-correction review 01.

Edit only `wallet-broker/src/vault.rs` with `apply_patch`. Make exactly these six logical
API substitutions:

1. Import `AeadInPlace` instead of `AeadInOut`, retaining `KeyInit`.
2. In `seal_vault`, replace the fallible `XNonce::try_from(nonce.as_slice())` plus
   `VaultError` mapping with `XNonce::from_slice(nonce.as_slice())`.
3. Call `encrypt_in_place_detached(cipher_nonce, &aad,
   plaintext_scratch.as_mut_slice())` instead of `encrypt_inout_detached` with a nonce
   double-reference and `InOutBuf` conversion.
4. In `open_vault_bytes`, replace the fallible
   `XNonce::try_from(envelope.nonce.as_slice())` plus `VaultError` mapping with
   `XNonce::from_slice(envelope.nonce.as_slice())`.
5. Replace the fallible `Tag::try_from(tag_bytes.as_slice())` plus `VaultError` mapping
   with `Tag::from_slice(tag_bytes.as_slice())`.
6. Call `decrypt_in_place_detached(cipher_nonce, &aad, plaintext.as_mut_slice(), tag)`
   instead of `decrypt_inout_detached` with double-references and `InOutBuf` conversion.

These slices have fixed lengths enforced by `[u8; NONCE_BYTES]`, the parsed envelope's
fixed nonce array, and `split_off` after the checked tag boundary. Preserve all
algorithms, constants, values, allocations, validation, ordering, errors, associated
data, envelope bytes, detached-tag behavior, key derivation, cleanup, and wipe paths.
Add no helper, conversion, branch, comment, dependency, feature, lint allowance, unsafe
code, or other change.

Run no executable, test, formatter, Cargo, Rust, Node, npm, resolution, network, fixture,
Git, cleanup, wallet, node, or device command. After editing, only read-only `wc -l`,
`sha256sum`, literal/diff inspection, and `git diff --check` over the authorized source
are allowed. Report exact line/hash/diff, frozen-path integrity, and blockers. Luna owns
later execution, evidence, integration, commit, and push.
