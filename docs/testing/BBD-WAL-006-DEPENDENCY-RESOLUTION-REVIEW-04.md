# BBD-WAL-006 Dependency Resolution Review 04

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Protected governance parent: `c47394ce`

Result: **GRAPH RESOLVED — AEAD API TEST-FIRST COMPATIBILITY CORRECTION REQUIRED**

Luna's preflight and focused policy progression passed. `cargo generate-lockfile`, the
crates.io fetch, both feature trees, duplicate tree, and offline metadata all exited 0.
The resolved lockfile is 5,367 lines with SHA-256
`bf9a36a70c96afe3c0b68355f082342c3e625e4764782228be9f4663b635cd83`.

The exact locked/offline 11-test custody command reached compilation and stopped on one
API incompatibility before executing a test:

```text
error[E0432]: unresolved import `chacha20poly1305::aead::AeadInOut`
 --> wallet-broker/src/vault.rs:5:30
```

Published `chacha20poly1305 0.10.1` re-exports `AeadInPlace`; that trait provides
`encrypt_in_place_detached(nonce, aad, &mut [u8])` and
`decrypt_in_place_detached(nonce, aad, &mut [u8], tag)`. The accepted production and
independent vector test currently use the 0.11 names `AeadInOut`,
`encrypt_inout_detached`, `decrypt_inout_detached`, and an `InOutBuf` conversion. No
algorithm, nonce/tag size, associated data, buffer mutation, detached-tag, error, or wipe
semantics need change.

The compatibility correction is test-first and source-bounded:

1. Sol changes only `wallet-broker/tests/vault_crypto.rs` to import `AeadInPlace` and call
   `encrypt_in_place_detached` on the same mutable plaintext slice. No test vector,
   expected byte, assertion, input, or test name may change.
2. Luna runs the exact custody target; it must still fail only at frozen production
   `AeadInOut` before executing tests, proving the test source compiles far enough to
   retain pressure on production.
3. After reviewer red acceptance, Sol may change only `wallet-broker/src/vault.rs`: the
   trait import and the encrypt/decrypt method names/arguments, with all surrounding
   custody/error/wipe behavior byte-exact.
4. Luna reruns the resolved-graph inventory and all 11 custody tests. Every independent
   vector and deterministic envelope must pass exactly before integration.

No dependency, policy, lockfile, Zcash test, fixture, or other production source change
is authorized by this review.
