# BBD-WAL-006 Scan Runtime Fail-Closed Correction Review 01

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Protected governance parent: `a65566bc`

Result: **SOURCE ACCEPTED — HERMES SCAN GATE RESUME 05 AUTHORIZED**

Read-only inspection confirms that Sol applied exactly the bounded correction from Scan Runtime
Source Review 01. Only the malformed-spend detector changed: it now left-joins the referenced
spending transaction and treats a missing transaction row as malformed. The separate official
unexpired-spend exclusion retains its inner join and exact pinned semantics.

The complete corrected runtime drop is accepted for execution. It uses one guarded read-only
connection, an unnested official `WalletDb::from_connection` scope, a before/after SQLite data
version envelope, current tree roots, exact orphan predicates and upstream constants, and checked
pending/pool/account reconciliation. The test-source edit remains the exact unused-binding
correction with all 9 test cases and assertions unchanged.

## Accepted execution inventory

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/zec.rs` | 218 | `ae06d6a9b4448603860b55bf4eef42a1e970c414049a9304f42b436d2bf49cb4` |
| `wallet-broker/src/zec/fixture.rs` | 614 | `318820c6f125f2318ba0caf5ae44835b36b0f67856fc66b83401961f0c301b36` |
| `wallet-broker/src/zec/scan.rs` | 1,666 | `24255d50c550e3ae0504cdc4ec01f4fb4cdcc32892afb4a9f42f119785caff9a` |
| `wallet-broker/src/zec/store.rs` | 1,816 | `d0b97b802209489804e5d550ef90138a87342e155c519a686b2d900189f6603f` |
| `wallet-broker/src/zec/test_support.rs` | 1,220 | `02609ae79bb8d35a5c7fb0058f0ab56fa5deb27c94cfa2ddb81347a2f38ac034` |
| `wallet-broker/tests/zec_scan.rs` | 325 | `87ed1c3e8db8219ad126efb4de7f681cf909cb404b88df6c2dfadec471a3701f` |

Production source totals 5,534 lines. `wallet-broker/src/zec/prepare.rs` remains absent and
`git diff --check` passes. This is static source acceptance, not execution acceptance. Hermes must
restart the complete gate and stop at its first mismatch.
