# Codex Sol Handoff — BBD-WAL-004 Correction 3 Production

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This durable file is
the complete prompt; ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff.

Read `AGENTS.md`, `TESTING.md`, roles, `tickets/BBD-WAL-004.md`,
`docs/testing/BBD-WAL-004-GREEN-RUN-01.md`, the complete accepted
`asset_network_and_account_id_are_closed` and
`metadata_constructor_rejects_zero_epoch_and_crossed_networks` tests, and current
formatted `wallet-broker/src/vault.rs`.

Edit only `wallet-broker/src/vault.rs`, whose formatted pre-correction state is 755 lines
with SHA-256 `a95d3dc7de1ec0e2b3fd49e81d7cc46343310c7ac90f5ffd90554edac7692e41`.

In `parse_vault`, preserve parsing/profile/account/asset/network/epoch order, but convert
only `VaultMetadata::new`'s `WRONG_NETWORK` result for untrusted wire metadata into
`VaultError::schema()`. Preserve all other constructor errors and direct public
`VaultMetadata::new` behavior, including its intentional `WRONG_NETWORK` result for
typed crossed enums. Do not change `validate_asset_network`, error definitions, tests,
formatting elsewhere, cryptography, canonical bytes, or any other behavior/path.

Use `apply_patch`. Do not run Rust, Cargo, Node, tests, formatters, builds, scanners,
network, Git, or project commands. Do not stage, commit, push, delete, use `/tmp`, or use
root. After editing, only `wc -l wallet-broker/src/vault.rs` and
`sha256sum wallet-broker/src/vault.rs` are allowed. Report count/hash and no blocker.
