# BBD-WAL-009 Phase-A1 Expected-Red Acceptance 01

Reviewer: Lead Engineer/Reviewer — Codex at High

Reviewed integration commit: `d7819e0f3ab3874dfad6e87e304f391678d03d83`

Result: **PHASE-A1 TEST CONTRACT AND EXPECTED RED ACCEPTED**

Hermes integrated exactly the manifest target, the formatted 14-test contract, its
expected-red evidence, and current-task update. `HEAD` and `origin/master` match the
reviewed commit and the repository is clean. The accepted source identities are:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/Cargo.toml` | 121 | `71e0135dbc2a6086ee6658e173718d2dd8c608a1f6369f732a8979d942ef6450` |
| `wallet-broker/tests/zec_sign_verify.rs` | 1,115 | `80a3a342392f53553950fabae710f2e95082d357c281c6de23b54aedbc85eccd` |

The Rust 1.98 formatter passed with exit `0`. The focused target exited `101` before
any test executed, solely because the typed BBD-WAL-009 sign/verify contract and
`zec/spend.rs` do not exist yet. The receiver-byte oracle method is an intentional new
member of that contract, as resolved in the stop review. `Cargo.lock` was unchanged.

The evidence correctly distinguishes the execution parent `efd1210d` from the later
documentation/integration governance parent `7191541e`. Hermes did not rerun either
gate during integration. This acceptance proves only a valid leading red contract; it
does not prove signing, verification, production hardware, network, broadcast,
mainnet, or live testnet behavior.
