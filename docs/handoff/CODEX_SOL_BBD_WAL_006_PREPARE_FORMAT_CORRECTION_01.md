# Codex Sol Handoff — BBD-WAL-006 Prepare Format Correction 01

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. Own only this bounded
mechanical source correction.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely before acting: `AGENTS.md`, `TESTING.md`, Prepare Production Source Review 02,
Prepare Gate Format Review 01, Prepare Format Correction Routing Review 01, the Hermes prepare
gate handoff, and all four current changed source paths.

## Exact scope

You may edit only:

- `wallet-broker/src/zec/prepare.rs`;
- `wallet-broker/src/zec.rs`;
- `wallet-broker/src/zec/store.rs`; and
- `wallet-broker/src/zec/test_support.rs`.

Starting identities are the four accepted hashes in Prepare Production Source Review 02. The
formatter reported hunks in `prepare.rs` near 549, 654, 877, and 906; `store.rs` near 1831;
`test_support.rs` near 574, 610, 649, 663, 725, and 1530; and `zec.rs` near 17.

## Mandatory correction

Use `apply_patch` only to make the minimum whitespace/line-wrapping layout changes that Rust 1.98.0
rustfmt requires at the reported locations. Preserve the exact non-whitespace token stream,
behavior, visibility, types, constants, comments, strings, vector bytes, and source inventory.
Do not opportunistically reformat an unrelated region or make a semantic/Clippy correction.

Do not run a formatter, compiler, Cargo, Rust, Clippy, test, Node, policy, dependency, Git, network,
fixture-generation, wallet/node/device, cleanup, or deletion command. Do not stage, commit, or
push. Do not edit a test, manifest/lock, fixture, policy, workflow, documentation, or unlisted
path.

Return the exact four resulting line counts/SHA-256, enumerate every changed layout site, confirm
that the non-whitespace token stream is unchanged, and disclose any ambiguity. Hermes remains the
sole future execution, evidence, integration, and Git actor.
