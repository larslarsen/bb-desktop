# Codex Sol Handoff — BBD-WAL-004 Test Source Correction 1

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This is the complete
durable correction prompt. Ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Reviewer governance baseline: `c770829e`

The initial test drop is present and uncommitted. Read completely before acting:
`AGENTS.md`, `TESTING.md`, `tickets/BBD-WAL-004.md`,
`docs/handoff/CODEX_SOL_BBD_WAL_004_TESTS.md`, `docs/handoff/CURRENT_TASK.md`, and all
nine initial authorized test-drop paths. The lead reviewer verified the initial path
set, line counts, and reported SHA-256 values, and ran the Node policy suite: all 58
pre-existing checks passed; the strengthened path assertion and six new WAL-004 checks
were red as expected.

Your sole task is correction 1. You may edit exactly:

- `wallet-broker/tests/vault_crypto.rs`
- `wallet-broker/tests/vault_format.rs`
- `wallet-broker/tests/vault_store.rs`

Do not edit the manifest, fixture, other Rust tests, Node tests, policy/workflow/source,
documentation, evidence, or any unlisted path. Use `apply_patch`. You may perform
read-only inspection and final `wc -l`/`sha256sum` reporting over the three authorized
paths only. Do not execute Rust, Cargo, Node, npm, tests, builds, formatters, scanners,
Git, GitHub, network, Electron, native windows, child processes, wallets, nodes,
hardware, or devices. Do not install anything or create any file. Do not use root,
`sudo`, `/tmp`, deletion, cleanup, `rm`, globs, or variable/substitution-resolved
destructive targets.

## Required corrections

1. Replace the test that merely compares `sealed(7)` to another call with a byte-for-byte
   assertion against the reviewer-fixed independent vector in the ticket. Keep the
   published primitive vectors. The exact ciphertext Base64 is
   `u97UrVKi33aeVs/8hWOSv2nFGDxmoitKxvjXfg6Dg99bvKJDu76lymjHUA`; copy the complete exact
   canonical envelope from the ticket, including its one final LF. Assert the entire
   output, not a prefix, length, recomputation through production, or second production
   call. Then open those fixed expected bytes and assert the plaintext.

2. Reserve the normal production open API with work observation in its real path:
   `open_vault_bytes(bytes, passphrase, work_observer, wipe_observer)`. Update every call
   in the authorized crypto/format tests. A valid fixed envelope must observe exactly one
   KDF call. Each well-formed but non-exact KDF/AEAD algorithm/version/cost mutation must
   pass through `open_vault_bytes` with a valid bounded passphrase, return `SCHEMA`, and
   observe zero KDF calls. Do not prove this only through `parse_vault`; that can pass
   while a future open path performs attacker-selected work first. Allocation-boundary
   assertions may remain on `parse_vault`.

3. Strengthen the fake-store fault matrix. For Create, Permission, Write, FileSync, and
   Replace failure, assert the active path still contains `OLD` byte-for-byte and the
   account lock is released. For DirectorySync failure, assert failure is reported and
   the active path is a complete `OLD` or `NEW` value—not absent, empty, or partial—and
   the lock is released. A recovery failure before replacement must preserve exact old
   active bytes plus complete new staging bytes. A directory-sync failure after an
   atomic replacement must preserve exact complete active bytes. Do not accept an
   arbitrary set containing empty values as proof of active-state preservation.

4. Add one real Linux boundary test using the future public `LinuxStorePort` with the
   existing `VaultStore` generic. It must work only beneath
   `target/wal004-scratch/os-<decimal process id>`, create/write through production store
   operations, and use `std::fs::symlink_metadata` plus Unix `PermissionsExt` to prove
   an actual `0700` data directory and actual `0600` regular active file. Create an
   explicit second-account symlink inside that exact root and prove `read_active` rejects
   it before reading. Cleanup may call `std::fs::remove_file` for the two fully constructed
   explicit file paths and `std::fs::remove_dir` for the one fully constructed explicit
   empty test root; no recursive removal, glob, environment/temp path, shell, external
   command, or cleanup abstraction is allowed. If a stale exact root exists, fail closed
   rather than deleting it. The test is Linux-only; no Windows/macOS claim or conditional
   skip is added.

The ticket now freezes the exact u32-big-endian framing and independently computed
intermediate values. Do not change them or add a test-side crypto oracle. The fixed
envelope is the independent oracle.

Preserve every other initial test and assertion. No production source or lockfile is
authorized. Stop and report the three changed paths with line counts/SHA-256, every test
name added/changed, exact future API changes, why the four corrected checks are
non-vacuous, and confirmation that no unlisted path changed and no prohibited command
ran. Lead Engineer/Reviewer — Codex XHigh will re-review before any install or execution.
