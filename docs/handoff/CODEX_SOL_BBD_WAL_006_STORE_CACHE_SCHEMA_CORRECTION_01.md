# Codex Sol Handoff — BBD-WAL-006 Store Cache Schema Correction 01

You are **Principal Dev — Codex Sol**. This durable handoff is authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely before acting: `AGENTS.md`, `TESTING.md`, Store Production Source Review 02,
Store Gate Runtime Review 01, the pinned upstream files
`zcash_client_sqlite-0.22.0/src/chain/init.rs` and `src/chain/migrations/blockmeta.rs`, and
`docs/handoff/CURRENT_TASK.md`.

## Sole task

Using `apply_patch`, edit only `validate_cache_schema` in
`wallet-broker/src/zec/store.rs`. Require the starting file to be 1,686 lines/SHA-256
`f12f634b90d8a517038866d8632a94bb12bbbcae109e35fd2721bbb3b9401662`.

Replace the incorrect unstable metadata-cache invariant with the exact stable cache invariant
for the already selected `BlockDb::for_path` plus `init_cache_database` API:

- query the closed `sqlite_schema` inventory for all non-internal objects in deterministic order;
- require exactly one object: type `table`, name/table name `compactblocks`, and the normalized
  SQL created by the pinned stable initializer;
- validate exact columns as `height INTEGER` with `NOT NULL == false` and primary key true, then
  `data BLOB` with `NOT NULL == true` and primary key false;
- reject `compactblocks_meta`, `schemer_migrations`, extra tables/indexes/triggers/views, extra or
  reordered columns, and any SQL/schema mismatch; and
- retain fail-closed error mapping and bounded reads.

Do not change `initialize_official_cache`, enable `unstable`, add a dependency/feature, modify
wallet or extension validation, edit a test/fixture/manifest/policy/workflow/doc, or make unrelated
cleanup. No formatter, compiler, Clippy, test, policy, Git, or network command is authorized. Do
not stage, commit, or push.

The other source paths and corrected store test are frozen:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/zec.rs` | 214 | `800111bf587265b49764de71c6d2beb054d1906932005bb3513b2d4fc132d90e` |
| `wallet-broker/src/zec/fixture.rs` | 280 | `01c4fa4b83b0b5f4501db966c27e07c2d3f7d88148daac1865bc35e67044c698` |
| `wallet-broker/src/zec/test_support.rs` | 825 | `e37f486b7dceab7d35fc1d0385ca17d82d5082e77732edeaca727e8903592346` |
| `wallet-broker/tests/zec_store.rs` | 324 | `1c230a2a9cf51c841a0df6514393861387422e5d0b2a83e80af47022728e2225` |

Return the resulting `store.rs` line count and SHA-256, explain the exact closed inventory and
column checks, confirm every frozen identity, and report any ambiguity. The reviewer will inspect
the source and decide whether Hermes may restart.
