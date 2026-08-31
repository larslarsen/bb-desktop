# BBD-WAL-006 AEAD API Correction Expected Red 02

Timestamp: 2026-08-31T13:14:31-07:00 (America/Los_Angeles)

Protected governance parent: `eb3ec101`

The exact locked/offline command was run once:

```text
env TMPDIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-tmp CARGO_TARGET_DIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-cargo /home/lars/.cargo/bin/cargo +1.98.0 test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test vault_crypto
```

Result: exit `101`, exact expected production-only red. Compilation emitted 9 errors;
zero tests executed. Every error named only frozen production `wallet-broker/src/vault.rs`:

- `E0432` unresolved `AeadInOut` import at line 5;
- six `E0277` `TryFrom<&[u8]>`/`From<&[u8]>` failures at `XNonce::try_from` lines
  482 and 538 and `Tag::try_from` line 539; and
- two `E0599` missing `encrypt_inout_detached` line 484 and
  `decrypt_inout_detached` line 541.

The corrected test produced no separate API error. No canary or secret appeared. No
runtime, linker, wallet, node, device, network, fixture, or other source-path failure
occurred.

Pre/post hashes are unchanged:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/tests/vault_crypto.rs` | 394 | `a83bc4d1bf30201ad8c9bfa09556e8bae029b8eefebf2eeeab4db5fed03e561b` |
| `wallet-broker/src/vault.rs` | 760 | `519b544e34cba1dcba240e93f03b19d05fb5137477bfb2441dd3a57c91c79a41` |
| `wallet-broker/Cargo.toml` | 79 | `89f02a18de21114d96c9f7231a36d8b60434c757e996314cb7f33403ae51c6a3` |
| `scripts/security-policy.js` | 2,231 | `3551656d49bfdc717fdff627d137b477582f274565880aae51a20922ebd28e83` |
| `wallet-broker/Cargo.lock` | 5,367 | `bf9a36a70c96afe3c0b68355f082342c3e625e4764782228be9f4663b635cd83` |

The six ZEC test paths remain at their accepted format-correction hashes.
