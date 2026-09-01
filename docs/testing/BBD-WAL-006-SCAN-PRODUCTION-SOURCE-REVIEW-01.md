# BBD-WAL-006 Scan Production Source Review 01

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Governance parent: `3b0a199a`

Result: **CORRECTION REQUIRED — EXECUTION NOT AUTHORIZED**

Sol delivered the complete candidate-cache and inner-wallet-transaction vertical in exactly the
five authorized source paths. The frozen tests and manifest retain their accepted identities, no
dependency/schema/feature was changed, and `git diff --check` passes. Static review accepts the
fixture closure, full DELETE-mode candidate snapshot, official decode, one-transaction scan/reorg,
full-height recovery comparison, and deferred postcommit cache promotion design.

Three inspect/test-truth issues must be corrected before Hermes may format or compile.

## Findings

### 1. `tree_root` does not commit to any Merkle root

`scan.rs:1016-1022` hashes the tip height/hash and three `BlockMetadata` tree sizes. A replacement
tip necessarily changes that value even if the Merkle roots were never read, so the frozen
`assert_ne!(replaced.tree_root, old.tree_root)` cannot distinguish this shortcut.

The stated caveat that stable pinned APIs do not expose roots is incorrect. Public
`WalletCommitmentTrees::{with_sapling_tree_mut,with_orchard_tree_mut,with_ironwood_tree_mut}` is
implemented by `WalletDb` in sqlite 0.22.0, and each callback can call public
`ShardTree::root_at_checkpoint_id`. The sqlite backend explicitly overrides Ironwood. Inspection
must derive its bounded public fingerprint from the exact serialized checkpoint roots for all
three pools. Tip height/hash remain separate DTO fields and cannot substitute for root bytes.

### 2. Pool classification is a constant

`scan.rs:947-949` and `scan.rs:1025-1027` return the same taxonomy string regardless of official
wallet state. The handoff requires pool classification to be reconstructed from reopened official
balances/state. The value must distinguish empty, nonzero Sapling/transparent, Orchard
migration-required, and Ironwood pending/spendable states without reading fixture expected values.

### 3. Postcommit metric errors are silently converted to stale/default values

`scan.rs:423-436` uses `if let (Ok(...), Ok(...))` and `if let Some(...)`, swallowing wallet-query
and checked-subtraction failures after commit. The public hidden facade then returns plain counters
from the default/stale in-memory struct. This contradicts the source report that errors are not
turned into zero and lets the frozen count checks pass without an honest observation.

An already committed wallet scan still may not be returned as failure. Instead, compute fallible
metrics before commit when possible or represent postcommit test-only metrics as explicitly
unavailable. The hidden frozen-signature facade may `expect` an available metric, as its existing
recognized-note accessor does; it may not return a stale/default zero after an observation error.

## Accepted worktree inventory

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/zec.rs` | 218 | `ae06d6a9b4448603860b55bf4eef42a1e970c414049a9304f42b436d2bf49cb4` |
| `wallet-broker/src/zec/fixture.rs` | 600 | `708ebba85b215b873bacf580156dace9cd68e3d6ed6feb164719c1ff7c9776ee` |
| `wallet-broker/src/zec/scan.rs` | 1,301 | `3b9e7b67ec9543e0bc0652490292d6d7460a2b6974ad9a7e381fbaf1fa6ce319` |
| `wallet-broker/src/zec/store.rs` | 1,812 | `8cda1e3722f6c651c769d46411bc513af485522cb82b1759e1cdc2c0edd42d9e` |
| `wallet-broker/src/zec/test_support.rs` | 1,219 | `b4fb3231895413d1a5382b287084b4d32141885e6ca93ea3b9f81bc4ce92724b` |

Frozen `zec_scan`, `zec_store`, `zec_address`, and manifest hashes remain exactly those in the
resumed source handoff. Only the active Sol correction handoff may change source now.
