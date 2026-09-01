# Codex Sol Handoff — BBD-WAL-006 Prepare Format Correction 02

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. Own only this exact final
mechanical correction.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely: `AGENTS.md`, Prepare Gate Format Review 02, Prepare Format Correction Review 01,
and the current changed source around the nine reported sites.

## Exact scope and transformations

Use `apply_patch` only. Edit only these three paths and make exactly the reported rustfmt layout
changes:

- `wallet-broker/src/zec/prepare.rs`: collapse the binding/session `||` condition near 549;
  collapse the iterator/enumerate/`any` closure near 874; wrap the nested `Ok((((...))))` return
  near 905.
- `wallet-broker/src/zec/store.rs`: wrap the `Payment::new(...)` arguments near 1831; collapse
  `WalletDb::from_connection(...)` near 1841.
- `wallet-broker/src/zec/test_support.rs`: wrap the `AddressAccount::bootstrap(...).map(...)` near
  574; wrap `AddressAccount::open_viewing_with_network(...).map(...)` near 610; wrap the
  inspection/map chain near 724; collapse the address decode/expect/encode chain near 1525.

Do not edit formatter-clean `wallet-broker/src/zec.rs`. Starting identities are those in Prepare
Format Correction Review 01. Preserve the exact non-whitespace token stream and every semantic
property. Do not make another cleanup, semantic, or Clippy change.

Do not run a formatter, compiler, Cargo, Rust, Clippy, test, Node, policy, dependency, Git, network,
fixture-generation, wallet/node/device, cleanup, or deletion command. Do not stage, commit, or
push. Do not edit a test, manifest/lock, fixture, policy, workflow, documentation, or unlisted
path.

Return the resulting line counts/SHA-256 for all four source paths, enumerate the nine layout
changes, confirm `zec.rs` remained byte-exact and the non-whitespace token stream is unchanged,
and disclose any ambiguity. Hermes remains the sole execution/evidence/integration/Git actor.
