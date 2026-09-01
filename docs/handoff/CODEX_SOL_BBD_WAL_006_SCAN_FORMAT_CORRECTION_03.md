# Codex Sol Handoff — BBD-WAL-006 Scan Format Correction 03

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This is an exact mechanical
source correction.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely before acting: `AGENTS.md`, Scan Format Capture Review 02, and the complete exact
artifact `wallet-broker/target/wal006-scan-format-resume05.stdout`.

Edit only `wallet-broker/src/zec/scan.rs`, starting at 1,666 lines and SHA-256
`24255d50c550e3ae0504cdc4ec01f4fb4cdcc32892afb4a9f42f119785caff9a`. Using `apply_patch`, apply
exactly the artifact's four replacements in their displayed order:

1. join the two-line `WalletDb::from_connection` binding onto the formatter's one line;
2. join the two-line Ironwood checked-add comparison onto the formatter's one line;
3. join the four-line `receiver_sequence` signature onto the formatter's one line; and
4. wrap the marginal-fee conversion exactly as shown by the formatter.

Do not change a semantic token or another location. Every other source, test, fixture, manifest,
lockfile, document, policy, workflow, package, and repository path is frozen. In particular
`wallet-broker/tests/zec_scan.rs` remains 325 lines at
`87ed1c3e8db8219ad126efb4de7f681cf909cb404b88df6c2dfadec471a3701f` and the four supporting
source identities remain those in Scan Gate Format Review 03.
`wallet-broker/src/zec/prepare.rs` remains absent.

Do not run rustfmt, Cargo, Rust, Clippy, tests, Node, npm, policy, a compiler, linter, Git, network,
fixture, wallet, node, device, cleanup, or deletion command. Do not stage, commit, or push.

After editing, use only read-only file inspection, `wc -l`, and `sha256sum`. Return the changed
path's line count/hash, confirm the four exact replacements and no semantic change, and re-prove
the frozen test/supporting-source identities and absent ZEC prepare path. Stop on any ambiguity.
