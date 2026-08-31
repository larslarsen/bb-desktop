# Codex Sol Handoff — BBD-WAL-004 Correction 6 Test Compatibility

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This durable file is
the complete prompt; ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff.

Read `AGENTS.md`, `TESTING.md`, roles, `tickets/BBD-WAL-004.md`,
`docs/testing/BBD-WAL-004-GREEN-RUN-05.md`, the complete current test, and the locally
cached pinned `aead`/`chacha20poly1305`/`inout` sources already named by Correction 4.

Edit only `wallet-broker/tests/vault_crypto.rs`, currently 391 lines with SHA-256
`57b4f432bf4a96d9023f74e1f25c43ebd091737c341ebf837fd0ea3994077655`.

Apply exactly four behavior-preserving corrections:

1. Import `AeadInOut` instead of deprecated `AeadInPlace`.
2. In the local `hex` oracle helper, replace fixed `chunks_exact(2)` with
   `as_chunks::<2>().0.iter()` after retaining its exact even-length assertion.
3. In `xchacha20poly1305_draft_vector_is_independent`, convert the existing 24-byte
   nonce slice with checked `XNonce::try_from(...).unwrap()` and borrow it.
4. Call `encrypt_inout_detached` with the same cipher, nonce, AAD, and mutable plaintext
   slice converted to `InOutBuf`.

Preserve every literal expected vector byte, independent input, assertion, test name,
case, panic behavior, plaintext buffer, tag comparison, and all other test semantics.
Do not edit production, another test, fixture, dependency, feature, policy, docs, or any
other path. These exact denied warnings are the expected red; do not add or weaken a
test.

Use `apply_patch`. Do not run Rust, Cargo, Node, tests, formatters, linters, builds,
scanners, network, Git, or project commands. Do not stage, commit, push, delete, use
`/tmp`, or use root. Stop on contradiction.

After editing, only `wc -l wallet-broker/tests/vault_crypto.rs` and
`sha256sum wallet-broker/tests/vault_crypto.rs` are allowed. Report exact count/hash and
no blocker. Luna owns execution, integration evidence, Git, and push after review.
