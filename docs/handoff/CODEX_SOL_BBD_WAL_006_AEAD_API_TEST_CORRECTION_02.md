# Codex Sol Handoff — BBD-WAL-006 AEAD API Test Correction 02

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This durable file is
authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read `AGENTS.md`, `TESTING.md`, roles, ticket, resolution reviews 04/05, fetched stable
AEAD API, and complete current test. Require protected parent/remote, clean index,
`git diff --check`, and exactly ten paths. Require current test 394 lines at SHA-256
`9bc3fe14718cd5ecdec5da3e8416f46356b88e28bf2f56db9ee9407cd0f71b6d`; production,
manifest, policy, lock, and ZEC test hashes remain those in the prior API-red handoff.

Edit only `wallet-broker/tests/vault_crypto.rs` with `apply_patch`:

- replace `let nonce = XNonce::try_from(nonce.as_slice()).unwrap();` with
  `let nonce = XNonce::from_slice(nonce.as_slice());`; and
- pass `nonce` rather than `&nonce` to the already-corrected
  `encrypt_in_place_detached` call.

Preserve the prior `AeadInPlace` correction and every test/vector/assertion/other byte.
Run no executable, test, formatter, resolution, network, fixture, Git, cleanup, wallet,
node, or device command. After editing only read-only count/hash/diff checks over that
test are allowed. Report exact line/hash, two edits, unchanged 11 tests/vectors, frozen
integrity, and blockers.
