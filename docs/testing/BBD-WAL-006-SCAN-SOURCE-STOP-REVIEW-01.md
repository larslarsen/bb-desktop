# BBD-WAL-006 Scan Source Stop Review 01

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Governance parent: `04dda2c052a1430e3cfe257f7347d38b2903b889`

Result: **NO-EDIT STOP ACCEPTED — ATOMICITY DESIGN REVIEW REQUIRED**

Sol completed the required local/upstream API inspection and made no edit. The protected source,
tests, fixture, index, and worktree remain exact; `wallet-broker/src/zec/scan.rs` remains absent.

## Confirmed pinned-API boundary

- `WalletDb::transactionally_with_extension` provides one wallet transaction plus an
  `ExtensionTransaction`, sufficient for atomic wallet writes and `ext_*` state.
- The extension authorizer permits writes only to `ext_*` objects and denies wallet-owned tables,
  `ATTACH`/`DETACH`, transaction control, DDL, and other actions.
- Stable `BlockDb` exposes `for_path`; its connection is private and `from_connection` is
  crate-private. Stable `BlockSource` is read-only.
- Therefore a replacement block in the separate `compactblocks` database cannot be written in the
  same upstream wallet transaction as rewind, replacement scan, and extension-tip update.
- Committing either database first without a recovery protocol creates a crash window. Keeping the
  canonical height-107 cache after the wallet accepts the replacement creates durable reopen
  inconsistency.
- Direct protobuf decoding is not currently available to the adapter without a new direct `prost`
  dependency; the handoff did not authorize one.

Official `scan_cached_blocks`, transaction-backed `WalletWrite::put_blocks`,
`truncate_to_chain_state`, and wallet summaries otherwise support real scanning and distinct
Orchard/Ironwood classification. The blocker is cache/wallet crash consistency, not recognition.

## Protected identities

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/zec.rs` | 214 | `800111bf587265b49764de71c6d2beb054d1906932005bb3513b2d4fc132d90e` |
| `wallet-broker/src/zec/fixture.rs` | 280 | `01c4fa4b83b0b5f4501db966c27e07c2d3f7d88148daac1865bc35e67044c698` |
| `wallet-broker/src/zec/store.rs` | 1,700 | `779f847a328a8fe85ca7a951a67d6be12403ec3f73b9557c943c4e404742052f` |
| `wallet-broker/src/zec/test_support.rs` | 825 | `e37f486b7dceab7d35fc1d0385ca17d82d5082e77732edeaca727e8903592346` |
| `wallet-broker/tests/zec_scan.rs` | 325 | `084aa1758cc10bdd1f9fc63f935e70328aca1f2f668ff8f67234cef7f5656434` |
| `wallet-broker/tests/zec_store.rs` | 324 | `1c230a2a9cf51c841a0df6514393861387422e5d0b2a83e80af47022728e2225` |
| `wallet-broker/tests/zec_address.rs` | 277 | `2c5012e6884c8c2a81236266c6861b6e4e4fd6b124656dad2ab438add5848ee3` |
| `wallet-broker/tests/fixtures/zec/manifest.json` | 238 | `0c5836405b29a2af618a9fa2784a973c981e807c3a8d46471aeeb90d8a6fe389` |

No source implementation is authorized until a reviewer accepts a crash-consistency design.
Grok Build may perform only the bounded protocol review in the active handoff.
