# BBD-WAL-006 Store Production Source Review 01

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Governance parent: `f27bd929`

Result: **REJECTED BEFORE EXECUTION — TWO-FILE CORRECTION AUTHORIZED**

Sol stayed within the four authorized paths. The protected `zec_store` test, `lib.rs`, and
`address.rs` hashes remain exact, and `git diff --check` passed. No command execution or Git
operation was performed on the production drop.

## Inspected source inventory

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/zec.rs` | 214 | `800111bf587265b49764de71c6d2beb054d1906932005bb3513b2d4fc132d90e` |
| `wallet-broker/src/zec/fixture.rs` | 279 | `5b1e91a730cddd82d0321383ec86f68dd781bf441a4b3e1db7e0514c5b9d5229` |
| `wallet-broker/src/zec/store.rs` | 1,617 | `96eb671465eaa7fdfe6291f35e1bfd1c9b4ce1226e2473fede814b4898506869` |
| `wallet-broker/src/zec/test_support.rs` | 830 | `51bb2d7f290c42c94fb9dc5361ec25af0632a6aa867befee19ae6746265c1e2c` |

The shared manifest limit, closed v0/v1 extension inventory, read-only reopen preflight, real
SQLite transactions, actual schema/row inspection, secret commitments, hostile-entry marker,
path owner/mode/type checks, and exact four-path scope are directionally accepted. They do not
authorize execution until the findings below are corrected.

## Blocking findings

1. `StateRoot::install_filesystem_fault` accepts an arbitrary `PathBuf`. The handoff permits only
   a closed wallet-entry facts seam whose path is internally derived from the state root, closed
   network, and validated account ID. A crate-private raw-path injection point still broadens the
   test capability beyond that contract.
2. `detect_network` propagates `LocalNetwork::new` failure as `SCHEMA`. Those heights came from an
   existing database, so invalid ordering is persisted-state corruption and must be mapped to
   `STATE_CORRUPT`. The binding check also accepts any nonempty UFVK rather than proving that the
   persisted viewing key decodes for the bound network.
3. V1 store validation proves only that `checkpoint_receiver_sequence` is nonnegative. It must
   also prove that the checkpoint sequence is not greater than the currently issued receiver
   sequence; otherwise internally contradictory persisted state passes preflight.
4. Wallet migration validation checks only 71 distinct 16-byte values, and cache validation reads
   one arbitrary row without proving there is exactly one. This does not establish the pinned
   upstream migration state. Require the wallet aggregate plus every published current leaf in
   `zcash_client_sqlite::wallet::init::migrations::CURRENT_LEAF_MIGRATIONS`; require exactly the
   single pinned cache migration ID and no additional row.
5. Migration/checkpoint success commits first and then calls fallible `sync_store_files`. A real
   post-commit sync failure would return an error after the schema/checkpoint had advanced, which
   contradicts the no-advance-on-durability-failure contract. SQLite must own journal/database
   synchronization at the commit boundary with full synchronous behavior. Test fault barriers
   must return before commit/rollback; do not present a pre-commit sync of the unchanged main DB
   as synchronization of uncommitted SQLite pages. Directory durability for initially created
   entries belongs to bootstrap creation, not a post-commit update that can fail after advance.
6. SQLite inspection clones text/blob cells before checking their size and has no aggregate cell/
   byte bound. Check each borrowed `ValueRef` length before allocation and enforce closed total
   decoded-cell and decoded-byte limits so corrupt state cannot force unbounded hidden inspection.

No test result is available, and none is authorized until Source Review 02 accepts the corrected
source. `zec.rs` and `fixture.rs` are frozen at the hashes above. Scan, preparation, handle
hygiene, policy transition, evidence, integration, and Git remain frozen.
