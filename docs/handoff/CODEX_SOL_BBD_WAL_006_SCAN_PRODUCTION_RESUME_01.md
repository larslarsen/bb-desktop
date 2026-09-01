# Codex Sol Handoff — BBD-WAL-006 Scan Production Resume 01

You are **Principal Dev — Codex Sol**. This durable corrective handoff is authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely before acting: `AGENTS.md`, `TESTING.md`, `tickets/BBD-WAL-006.md`, the original
Scan Production 01 handoff, Scan Source Stop Review 01, Scan Atomicity Review 01, the upstream
review, all frozen `zec_scan`, `zec_store`, and `zec_address` tests, the fixture manifest, all
current ZEC production source, and `docs/handoff/CURRENT_TASK.md`. Reconfirm the exact pinned APIs
you use before editing.

## Sole task and unchanged source boundary

Resume the compact-block scan production source. The original Scan Production 01 semantics and
delivery rules remain in force except where this handoff corrects the stopped atomicity design.
You may create/edit only:

- `wallet-broker/src/zec/scan.rs` (currently absent);
- `wallet-broker/src/zec.rs`;
- `wallet-broker/src/zec/fixture.rs`;
- `wallet-broker/src/zec/store.rs`; and
- `wallet-broker/src/zec/test_support.rs`.

Do not edit tests, fixture bytes/manifest, Cargo files/lock, policy, workflow, ticket,
documentation, Electron/Node source, or another repository. Do not add a dependency, feature,
extension table/column, or schema version. Preserve all accepted store/address behavior.

Starting protected identities remain:

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

## Normative atomicity correction

### Validate and recover before mutation

- Validate the complete closed manifest/input, bounds, paths, unique entries, byte lengths,
  SHA-256 values, network/schedule/version, scenario membership, and canonical ordering before
  candidate creation or a wallet write transaction.
- Every gated open/inspect/scan path recovers first. Official
  `WalletRead::get_max_height_hash` is authoritative after official account import; extension
  `scan_tip`, fixture labels, counters, and memory are never recovery/inspect authority.
- Before first import, require the stored bound checkpoint 99 and empty committed cache. A leftover
  nonempty candidate with no official account is a precommit orphan and is discarded or causes a
  closed failure if safe removal cannot be proven.
- Validate each existing cache file's exact schema and bounded full identity: contiguous height
  span, row count, decoded hash chain/tip, and exact dataset bytes/digest. If committed cache alone
  matches the wallet, discard the candidate. If candidate alone matches, promote it. If both are
  byte-identical, retain the committed target and remove the redundant candidate. If both claim the
  tip but differ, or neither matches, fail `STATE_CORRUPT`; never guess or initialize empty state.

### Build a complete durable candidate

- Use only fixed `compact.sqlite3.candidate` beside `compact.sqlite3`. Create it with `create_new`,
  mode `0600`, and no symlink/non-regular target. The account directory remains mode `0700`.
- Use the stable `BlockDb` schema and bounded direct `rusqlite` opaque-row writes. Pin and verify
  `PRAGMA journal_mode=DELETE` and `PRAGMA synchronous=FULL`; refuse `-wal`, `-shm`, and unexpected
  journal sidecars.
- The candidate is a complete intended snapshot. Bounded-copy every validated committed row, then
  insert/replace the validated new/replacement range. A height-only delta may never replace the
  committed cache.
- Commit SQLite, fsync the candidate file and account directory, close all connections, reopen via
  `BlockDb::for_path`, validate the exact schema, and decode through public
  `BlockSource::with_blocks`. Check public hash/previous-hash vector lengths are exactly 32 before
  calling panic-capable helpers. Prove bounded heights, continuity, and intended range before
  opening a wallet write transaction.

### Commit one official wallet transaction

- Use one `WalletDb::transactionally_with_extension` call. Call official import, rewind,
  `scan_cached_blocks`, tree/note writes, and extension updates only through the inner
  `WalletDb<SqlTransaction, _>`/`ExtensionTransaction`. Never invoke connection-level helpers that
  start a second transaction.
- On first scan, import the stored UFVK as a view-only official account inside that transaction.
  Construct the birthday/checkpoint state from the officially decoded first-block previous hash,
  not a fixture label or hard-coded hash.
- For the frozen schedule, use exactly three untrusted confirmations so the height-104 Ironwood
  note becomes spendable at height 106. Derive balances and Orchard/Ironwood classification from
  official wallet state.
- Replay of an exact committed chain is a no-op. A supported reorg rewinds exactly the old tip and
  scans the replacement in this one transaction. Reject a deeper reorg before the transaction.
- Every `ScanFault` seam, including `CommitSync`, is a logical precommit abort. Cache corruption is
  injected against the candidate, never by damaging the committed cache. The seams wrap the real
  production path.

### Reconcile commit and promote

- If the wallet transaction reports an unexpected commit error, close/reopen and compare the
  official wallet tip to the exact old and intended states. Old means return the sanitized failure
  unchanged. Exact intended new means commit occurred: continue as success. Any other state fails
  closed. Do not report failure when the intended durable wallet state is visible.
- After a known commit, close all wallet/cache/candidate connections. Atomically rename the
  complete candidate over `compact.sqlite3` in the same directory, then fsync the account
  directory.
- A rename or directory-fsync problem after the wallet commit must not be returned as a failed
  scan. Return the committed sanitized result and leave the old target plus candidate, or new
  target, for mandatory next-operation recovery. Recovery may not depend on clocks, network,
  caller paths, cleanup success, or unpersisted memory.

## Production truth and test seams

Inspect/reopen must read official wallet tip, tree state, notes, balances, and pool classification
from disk. `ScanSummary` does not expose Ironwood or unrelated-output counts; derive observable
counts from the bounded production decode and official durable wallet deltas. Do not hard-code
2/3/1 counts, fixture values, hashes, heights, roots, or expected DTOs. `scan_calls` remains zero
for hostile manifests because no production scanner/cache mutation may be reached.

Keep the original error, allocation, visibility, no-secret, and no-signing/network constraints.
Do not add `prost`, `sync`, `unstable`, a cache-generation journal, or a custom protobuf decoder.
If any exact pinned call cannot remain on the inner transaction, stop without edits and report the
signature/borrow conflict.

## Delivery boundary

Use `apply_patch` only. Do not execute a formatter, Cargo/Rust, Clippy, test, Node, policy,
dependency, Git, network, fixture-generation, wallet/node/device, or cleanup command. Do not stage,
commit, or push.

Return every changed path with line count/SHA-256, exact pinned APIs used, cache identity and crash
recovery algorithm, transaction call graph, allocation bounds, classification derivation, test
seams, and every remaining ambiguity. The reviewer will inspect the complete source drop before
Hermes receives execution/integration authority.
