# Codex Sol Handoff — BBD-WAL-006 Scan Production 01

You are **Principal Dev — Codex Sol**. This durable handoff is authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely before acting: `AGENTS.md`, `TESTING.md`, `tickets/BBD-WAL-006.md`,
`docs/architecture/BBD-WAL-006-UPSTREAM-REVIEW.md`, Store Integration Review 02, the complete
current `zec_scan`, `zec_store`, and `zec_address` tests, the complete frozen fixture manifest and
all current ZEC production source, and `docs/handoff/CURRENT_TASK.md`. Inspect the pinned upstream
0.24.0/0.22.0 scan/cache/wallet APIs before editing.

## Sole task and source boundary

Author only the compact-block scan vertical required by the frozen nine-test `zec_scan` contract.
You may create/edit only:

- `wallet-broker/src/zec/scan.rs` (currently absent);
- `wallet-broker/src/zec.rs`;
- `wallet-broker/src/zec/fixture.rs`;
- `wallet-broker/src/zec/store.rs`; and
- `wallet-broker/src/zec/test_support.rs`.

Do not edit tests, fixture bytes/manifest, `lib.rs`, `address.rs`, Cargo files/lock, policy,
workflow, ticket, documentation, Electron/Node source, or another repository. Do not add a
dependency or feature. Do not create `prepare.rs` or any sign/prove/finalize/extract/broadcast/
network/mainnet capability.

Starting protected identities:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/zec.rs` | 214 | `800111bf587265b49764de71c6d2beb054d1906932005bb3513b2d4fc132d90e` |
| `wallet-broker/src/zec/fixture.rs` | 280 | `01c4fa4b83b0b5f4501db966c27e07c2d3f7d88148daac1865bc35e67044c698` |
| `wallet-broker/src/zec/store.rs` | 1,700 | `779f847a328a8fe85ca7a951a67d6be12403ec3f73b9557c943c4e404742052f` |
| `wallet-broker/src/zec/test_support.rs` | 825 | `e37f486b7dceab7d35fc1d0385ca17d82d5082e77732edeaca727e8903592346` |
| `wallet-broker/tests/zec_scan.rs` | 325 | `084aa1758cc10bdd1f9fc63f935e70328aca1f2f668ff8f67234cef7f5656434` |
| `wallet-broker/tests/fixtures/zec/manifest.json` | 238 | `0c5836405b29a2af618a9fa2784a973c981e807c3a8d46471aeeb90d8a6fe389` |

Also preserve exact `zec_store` and `zec_address` tests and all accepted store/address behavior.

## Required scan semantics

- Validate the closed manifest and every referenced relative path, unique entry, length, SHA-256,
  network/schedule/generator version, scenario link, and canonical ordering before any scanner or
  database/cache advance. Duplicate JSON keys and unknown fields fail closed. Production scanning
  consumes validated bounded block bytes; it does not generate fixtures or trust manifest labels
  as scan results.
- Use the pinned official upstream compact-block decoding, wallet/cache, and viewing-key scan APIs
  for recognized notes, tree/chain state, and pool semantics. Do not hard-code fixture hashes,
  heights, values, receiver, note recognition, balance outcomes, or test labels into production.
- Start at checkpoint 99/birthday 100 for the bound local schedule; require contiguous height,
  previous hash, correct network/branch, valid protobuf, and possible tree state. Replay must be
  idempotent.
- Persist/reopen exact tip height/hash, tree state, receiver sequence, balances, and pool
  classification. Represent checked u64 zatoshis at the adapter boundary as canonical decimal
  strings; never substitute total for spendable.
- Classify the recognized pre-NU6.3 Orchard value as migration-required and the post-NU6.3
  Ironwood value as pending until the fixture confirmation boundary, then spendable-for-prepare.
  Transparent and Sapling balances remain separate and zero for this fixture. Ignore unrelated
  outputs without inventing value.
- Support exactly a one-block reorg: atomically remove the prior tip effects and apply the validated
  replacement. Reject deeper rollback. Rollback-write/sync, replacement-apply, wallet corruption,
  cache corruption, and commit-sync faults must leave the externally inspectable pre-call state
  unchanged. Never report failure after durable state has advanced.
- `MAX_COMPACT_BLOCK_BYTES` must reject above-limit input before allocation/decoding; all manifest,
  block, row, count, value, and aggregate arithmetic is bounded and checked before allocation or
  conversion.
- Expose only the closed `ScanError` codes and sanitized scan/balance/state DTOs required by the
  frozen test. No raw upstream error, SQL value, note plaintext, UFVK, receiver secret, user path,
  block oracle, or unbounded byte vector crosses the adapter boundary.
- Test-only counters, typed fault ports, corrupt-state seams, sized-allocation observation, and
  fixture adapters live behind `test_support`; they must exercise the real production path and may
  not implement alternate scan logic.

Use no `unsafe`, process, environment, socket, HTTP, DNS, endpoint, clock authority, background
worker, generic callback, raw SQL/PCZT escape hatch, signing material, or broad public visibility.
Keep all write ordering and rollback behavior explicit. If the pinned upstream APIs cannot prove
the required one-block atomicity or Ironwood classification without a semantic shortcut, stop and
report the exact API/design conflict instead of faking the test.

## Delivery boundary

Use `apply_patch` only. Do not execute a formatter, Cargo/Rust, Clippy, test, Node, policy,
dependency, Git, network, fixture-generation, wallet/node/device, or cleanup command. Do not stage,
commit, or push.

Return every changed path with line count/SHA-256, the exact upstream APIs used, persistence and
rollback design, allocation bounds, test seams, and any ambiguity. The reviewer will inspect the
complete source drop before Hermes receives any execution/integration authority.
