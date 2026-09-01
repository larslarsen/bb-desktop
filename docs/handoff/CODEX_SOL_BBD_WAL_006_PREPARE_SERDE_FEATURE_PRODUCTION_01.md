# Codex Sol Handoff — BBD-WAL-006 Prepare Serde Feature Production 01

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. Own only this bounded
production compile correction.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely: `AGENTS.md`, `TESTING.md`, Prepare Gate Clippy Review 01, Prepare Serde Feature
Expected-Red Review 01, the accepted test/evidence, pinned `zcash_client_backend 0.24.0`
`create_pczt_from_proposal` bound, pinned `zcash_client_sqlite 0.22.0` feature/`AccountUuid`
definition, pinned `pczt 0.9.3` Orchard getters/redactor behavior, and current changed sources.

## Exact scope

Use `apply_patch` only. Edit only:

- `wallet-broker/Cargo.toml`;
- `scripts/security-policy.js`;
- `wallet-broker/src/zec/store.rs`.

## Mandatory correction

1. Change the exact defaults-off `zcash_client_sqlite = "=0.22.0"` feature union to lexically
   ordered `['orchard', 'serde', 'test-dependencies', 'transparent-inputs']` in TOML syntax. Add no
   dependency or other feature.
2. In `scripts/security-policy.js`, update only
   `WAL006_DIRECT_DEPENDENCIES.zcash_client_sqlite.features` and the exact manifest dependency
   literal to the same union. Do not implement or change the still-partial-red inventory,
   resolved-capability, live-network, workflow, Gitleaks, or WAL-004 policy.
3. In the accepted real-PCZT inspection, replace real input counting with the public padded-spend
   discriminator: count actions whose `action.spend().dummy_sk().is_none()`. A real spend has no
   dummy signing key; a padding spend has one before finalization. Do not inspect private value,
   serialized internals, fixture expectations, or proposal input as a substitute for parsed PCZT
   inspection.
4. Replace the second private-value filter with an all-action signature check:
   `actions.iter().any(|action| action.spend().spend_auth_sig().is_some())`. This is stricter and
   rejects a signature on either real or padding action.

Preserve the read-only `WalletDb::from_connection` and official
`create_pczt_from_proposal` call. Do not build a wrapper wallet, custom serde implementation,
alternate PCZT, source-side feature bypass, authority, or mock.

Do not edit `Cargo.lock`; Hermes will resolve it offline. Do not edit any other Rust source/test,
Node test, fixture, ticket, workflow, docs, package, or another path. Do not run formatter,
compiler, Cargo/Rust, Clippy, test, Node, policy, dependency resolution, Git, network,
wallet/node/device, cleanup, or deletion. Do not stage, commit, or push.

Return the three resulting line counts/SHA-256, exact changed sites, confirm the accepted four-path
production inventory and all frozen tests otherwise remain unchanged, and disclose ambiguity.
Hermes remains the sole lock-resolution/execution/evidence/integration/Git actor.
