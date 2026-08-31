# Codex Sol Handoff — BBD-WAL-004 Correction 4 Production

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This durable file is
the complete prompt; ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff.

Read `AGENTS.md`, `TESTING.md`, roles, `tickets/BBD-WAL-004.md`,
`docs/testing/BBD-WAL-004-GREEN-RUN-02.md`, the complete current versions of the two
authorized source files, and these locally cached exact dependency sources:

- `eframe-0.36.1/src/epi.rs` and its crate-level `App::ui` example;
- `aead-0.6.1/src/lib.rs` for `AeadInOut`;
- `chacha20poly1305-0.11.0/src/lib.rs` for its `AeadInOut` implementation; and
- `inout-0.2.2/src/inout_buf.rs` for mutable-slice conversion.

Edit only:

- `wallet-broker/src/native_ui.rs` — 135 lines, SHA-256
  `6887b3468a10a946c9f8b8f05aa260538065e883c59fc3b39900ce860d75fad0`;
- `wallet-broker/src/vault.rs` — 761 lines, SHA-256
  `03ebaf98327094842c60a40de6cdb16670e559a4e00f599d13cad97db8097525`.

## Exact correction

1. Implement the pinned `eframe::App::ui(&mut self, ui: &mut egui::Ui,
   _frame: &mut eframe::Frame)` callback instead of the removed `update` callback. Pass
   the provided root `Ui` to `CentralPanel::show`. Preserve every displayed string,
   password control property, button, state transition, and zeroization behavior.
2. Import and use `AeadInOut` instead of deprecated `AeadInPlace`. Replace the detached
   encrypt/decrypt calls with `encrypt_inout_detached` and `decrypt_inout_detached`,
   converting the existing mutable byte slices into the required `InOutBuf`.
3. Replace deprecated nonce/tag `from_slice` constructors with checked `TryFrom<&[u8]>`
   conversions and map impossible length failures to the existing closed
   `VaultError::locked()` result. Keep nonce and tag values borrowed for the crypto
   calls.
4. Preserve the exact v1 Argon2id/HKDF/XChaCha20-Poly1305 composition, AAD, nonce,
   postfix-tag wire representation, ciphertext bytes, independent vector, error
   normalization, wipe ordering, and all public behavior. Do not add allocation,
   cloning, unsafe code, dependency, feature, test, or policy changes.

This correction is already demonstrated by the exact all-features compile/lint failure;
no new test source is required or authorized. All 78 behavioral Rust tests and the Node
suite passed before this gate.

Use `apply_patch`. Do not run Rust, Cargo, Node, tests, formatters, linters, builds,
scanners, network, Git, or project commands. Do not stage, commit, push, delete, use
`/tmp`, or use root. Do not edit the already-formatted tests or any other path. Stop on
contradiction.

After editing, only `wc -l` and `sha256sum` over the two authorized paths are allowed.
Report exact counts/hashes and no blocker. Luna owns all execution, evidence, Git, and
push after reviewer source acceptance.
