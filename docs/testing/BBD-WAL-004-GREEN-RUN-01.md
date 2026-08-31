# BBD-WAL-004 Green Run 01

Integrator: Jr Dev — Codex Luna

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Governance baseline: `7020a6a21d2a5d9bb7619c055c096103ae14102b`

Result: **BLOCKED ON ONE EXISTING TEST — CORRECTION 3 REQUIRED**

Preflight matched exactly the accepted 15 production paths and Rust/Cargo 1.98.0.
The initial fmt check reported only formatting differences, so Luna ran the authorized
formatter contingency and the second fmt check passed. Formatter-only changes remain
unstaged in all six accepted Rust tests and production Rust modules.

`npm run build`, `npm test`, and `node scripts/security-policy.js` all passed. The locked
offline no-default Rust suite compiled and reached execution. `vault_crypto` 11/11,
`secret_hygiene` 10/10, `native_surface` 13/13, `vault_store` 20/20, and `vault_session`
13/13 passed. `vault_format` passed 10/11; only
`asset_network_and_account_id_are_closed` failed because parsed crossed asset/network
input returned `WRONG_NETWORK` instead of the required closed-schema `SCHEMA`.

The adjacent accepted constructor test proves the intended distinction: direct typed
`VaultMetadata::new` calls with crossed enums return `WRONG_NETWORK`, while untrusted
vault document parsing must return `SCHEMA`. No test change is needed. Clippy, native-
feature compile, RustSec audit, evidence, staging, commit, and push did not run.

Only the one-file production correction in
`CODEX_SOL_BBD_WAL_004_CORRECTION_3_PRODUCTION.md` is authorized. All formatter output
must remain unchanged while Sol edits `wallet-broker/src/vault.rs` only.
