# BBD-WAL-006 Dependency Correction Gate 01

Timestamp: 2026-08-31T13:25:48-07:00 (America/Los_Angeles)

Protected governance parent: `63af10cc`

The required policy progression exited `1` for the expected reason: corrected
dependency-map equality passed, then the frozen checker rejected the six WAL-006
manifest additions at `scripts/security-policy.js:1840`.

The formatter check, three locked/offline feature trees, duplicate tree, metadata, and
locked/offline `vault_crypto` target all exited `0`. The custody target compiled and ran
exactly 11 tests: 11 passed, 0 failed, 0 ignored, 0 measured, 0 filtered. The tests
included independent Argon2id RFC 9106, HKDF-SHA256, XChaCha20-Poly1305, and
deterministic encrypted-envelope vectors. No warning, canary, or secret appeared.

Resolved lock inventory: 5,367 lines, SHA-256
`bf9a36a70c96afe3c0b68355f082342c3e625e4764782228be9f4663b635cd83`; 513 package
stanzas versus 327 previously (textual diff +2,193/−99; package stanzas +192/−6).
All package sources are crates.io registry sources. Required direct pins/checksums are:

- `argon2 0.5.3` — `3c3610892ee6e0cbce8ae2700349fcf8f98adb0dbfbee85aec3c9179d29cc072`;
- `chacha20poly1305 0.10.1` — `10cd79432192d1c0f4e1a0fef9527696cc039165d729fb41b3f4f4f354c2dc35`;
- `hkdf 0.12.4` — `7b5f8eb2ad728638ea2c7d47a21db23b7b58a72ed6a38256b8a1849f15fbbdf7`;
- `sha2 0.10.9` — `a7507d819769d01a365ab707794a4084392c824f54a7a6a7862f8c3d0892b283`;
- `zcash_client_backend 0.24.0`, `zcash_client_sqlite 0.22.0`, `pczt 0.9.3`,
  `zcash_primitives 0.30.1`, `zcash_protocol 0.10.5`, and `zcash_keys 0.16.1`
  matched the fixed architecture-review versions and checksums.

The graph contains `aead 0.5.2`, `chacha20 0.9.1`, `cipher 0.4.4`, stable
`crypto-common 0.1.7`, Zcash `crypto-common 0.2.0-rc.1`, stable `digest 0.10.7`,
`hmac 0.12.1`, `blake2 0.10.6`, and Zcash `digest 0.11.0-pre.9`,
`hmac 0.13.0-pre.4`, `sha2 0.11.0-pre.4`. Rejected stable AEAD 0.11/aead 0.6,
crypto-common 0.2.1/0.2.2, and stable hmac/sha2 0.13/0.11 are absent. The feature
trees show the reviewed Orchard/PCZT/SQLite/local-consensus union; upstream transitive
PCZT signer/prover/finalizer/extractor capabilities remain compiled but are not exposed
by BitBook. Build-script and proc-macro packages are transitive registry packages only;
no git/path/patched source was selected.

The required disk boundary was the real, ignored ext4 `wallet-broker/target` with the
existing `wal006-tmp` and `wal006-cargo` paths. No fixture, ZEC adapter test, full suite,
live endpoint, wallet, node, device, or other repository was accessed.

Pre/post hashes:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/vault.rs` | 759 | `89d8ada8ad7050910a92a9daa38f9d93a18b86c4b7d1bde7a7e9b4a8adf8b62b` |
| `wallet-broker/tests/vault_crypto.rs` | 394 | `a83bc4d1bf30201ad8c9bfa09556e8bae029b8eefebf2eeeab4db5fed03e561b` |
| `wallet-broker/Cargo.toml` | 79 | `89f02a18de21114d96c9f7231a36d8b60434c757e996314cb7f33403ae51c6a3` |
| `scripts/security-policy.js` | 2,231 | `3551656d49bfdc717fdff627d137b477582f274565880aae51a20922ebd28e83` |
| `test/securityPolicy.node.js` | 2,388 | `29cbe64a0a217a9a94eca23c52126b06174b79c547f512d8191282c0e00a4573` |
| `wallet-broker/Cargo.lock` | 5,367 | `bf9a36a70c96afe3c0b68355f082342c3e625e4764782228be9f4663b635cd83` |

The six ZEC tests remain at their accepted format-correction hashes.
