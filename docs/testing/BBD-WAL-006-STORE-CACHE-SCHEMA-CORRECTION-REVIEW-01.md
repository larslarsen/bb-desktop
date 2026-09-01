# BBD-WAL-006 Store Cache-Schema Correction Review 01

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Governance parent: `3402ecdff88c7b46c9bcd527f0b3ee1e990d9e66`

Result: **SOURCE CORRECTION ACCEPTED — HERMES STORE GATE RESUME 04 AUTHORIZED**

Sol edited only `validate_cache_schema` in `wallet-broker/src/zec/store.rs`. The corrected
validator matches the pinned stable `BlockDb` initializer and remains closed and fail-closed:

- the deterministic non-internal `sqlite_schema` query is bounded at two rows, which proves the
  required singleton or rejects any larger inventory;
- identifier and SQL byte lengths are checked before decoding/allocating their strings;
- the only accepted object is the exact `compactblocks` table and normalized initializer SQL;
- exact ordered column metadata is `height INTEGER`, nullable, primary key, followed by
  `data BLOB NOT NULL`, not primary key; and
- the unstable metadata cache, migration table, aliases, changed SQL/columns, and every extra
  table/index/trigger/view are rejected as `STATE_CORRUPT`.

The initializer, dependencies/features, wallet schema, extension schema, tests, fixtures, and all
other source remain unchanged. `git diff --check` passes.

## Accepted worktree inventory

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/zec.rs` | 214 | `800111bf587265b49764de71c6d2beb054d1906932005bb3513b2d4fc132d90e` |
| `wallet-broker/src/zec/fixture.rs` | 280 | `01c4fa4b83b0b5f4501db966c27e07c2d3f7d88148daac1865bc35e67044c698` |
| `wallet-broker/src/zec/store.rs` | 1,700 | `779f847a328a8fe85ca7a951a67d6be12403ec3f73b9557c943c4e404742052f` |
| `wallet-broker/src/zec/test_support.rs` | 825 | `e37f486b7dceab7d35fc1d0385ca17d82d5082e77732edeaca727e8903592346` |
| `wallet-broker/tests/zec_store.rs` | 324 | `1c230a2a9cf51c841a0df6514393861387422e5d0b2a83e80af47022728e2225` |

Production source totals 3,019 lines. No formatter, compiler, Clippy, test, policy, Git, or network
command was executed by Sol. This is source acceptance, not runtime acceptance. Hermes must
restart all Store Gate 01 preconditions and commands; no prior pass may be reused.
