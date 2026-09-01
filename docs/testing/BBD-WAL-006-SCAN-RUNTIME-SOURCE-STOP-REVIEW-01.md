# BBD-WAL-006 Scan Runtime Source Stop Review 01

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Governance parent: `316d35bb56ff84d6f538029817cdea9665c9fc5c`

Result: **NO-EDIT STOP ACCEPTED — SNAPSHOT DESIGN CORRECTION REQUIRED**

Sol completed the required pinned-source review and stopped before either authorized edit. The
stop is correct. `WalletDb::transactionally_with_extension` starts a rusqlite transaction and
constructs an inner `WalletDb<SqlTransaction, _>`. The sole public `WalletRead::get_wallet_summary`
implementation then unconditionally calls `unchecked_transaction()` on its connection. Rusqlite
0.37 documents and tests that this nested transaction fails at runtime.

The prior requirement to evaluate the official summary inside `transactionally_with_extension`
is therefore impossible with the exact pinned API. Reimplementing the private summary query,
changing the dependency, or accepting a nested-transaction failure is not authorized. The
single-extension-snapshot clause in Scan Runtime Design Review 01 and Runtime Correction 01 is
superseded.

No source or test token changed during the stopped handoff. The protected identities remain:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/tests/zec_scan.rs` | 325 | `084aa1758cc10bdd1f9fc63f935e70328aca1f2f668ff8f67234cef7f5656434` |
| `wallet-broker/src/zec/scan.rs` | 1,400 | `17d411e4af9e64d1169d6326ae17f4ae13e1283236d690c80984971507dedab9` |
| `wallet-broker/src/zec.rs` | 218 | `ae06d6a9b4448603860b55bf4eef42a1e970c414049a9304f42b436d2bf49cb4` |
| `wallet-broker/src/zec/fixture.rs` | 614 | `318820c6f125f2318ba0caf5ae44835b36b0f67856fc66b83401961f0c301b36` |
| `wallet-broker/src/zec/store.rs` | 1,816 | `d0b97b802209489804e5d550ef90138a87342e155c519a686b2d900189f6603f` |
| `wallet-broker/src/zec/test_support.rs` | 1,220 | `02609ae79bb8d35a5c7fb0058f0ab56fa5deb27c94cfa2ddb81347a2f38ac034` |

`wallet-broker/src/zec/prepare.rs` remains absent. The current-root and main-chain balance
corrections remain necessary, but they require the corrected stable-observation design issued
with this review.
