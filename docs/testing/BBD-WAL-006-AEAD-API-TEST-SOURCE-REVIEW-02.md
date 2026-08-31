# BBD-WAL-006 AEAD API Test Source Review 02

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Protected governance parent: `5cd75079`

Result: **COMPLETE TEST COMPATIBILITY SOURCE ACCEPTED FOR EXPECTED RED**

Sol changed only `wallet-broker/tests/vault_crypto.rs`, retaining 394 lines and all 11
tests. Accepted SHA-256:
`a83bc4d1bf30201ad8c9bfa09556e8bae029b8eefebf2eeeab4db5fed03e561b`.

The exact edits are:

- import `AeadInPlace` instead of `AeadInOut`;
- construct the independent vector nonce with `XNonce::from_slice` instead of
  `XNonce::try_from`; and
- call `encrypt_in_place_detached` on the same mutable plaintext slice instead of
  `encrypt_inout_detached` through an `InOutBuf` conversion.

All test names, inputs, ciphertext, tag, assertions, failure cases, RFC vectors, and
deterministic envelope bytes remain exact. The test now uses only the published stable
`chacha20poly1305 0.10.1` API.

Reviewer verification confirms `HEAD == origin/master == 5cd75079`, a clean index,
the exact ten-path uncommitted state, frozen production/manifest/policy/lock/ZEC-test
hashes, and `git diff --check`. Sol ran no executable or Git command.

The previous red run and its evidence were rejected because the reviewer contract
incorrectly allowed only production's unresolved `AeadInOut` import. Stable AEAD 0.10
also rejects production's two `XNonce::try_from` calls, `Tag::try_from`, and two old
in/out method calls. Luna may rerun only the exact locked/offline `vault_crypto` target.
The accepted red is zero executed tests and compiler errors only from those six frozen
production constructs; the corrected test must produce no API error.
