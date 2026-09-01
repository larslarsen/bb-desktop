# Codex Sol Handoff — BBD-WAL-006 Scan Truth Correction 01

You are **Principal Dev — Codex Sol**. This is a bounded source correction.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely: `AGENTS.md`, `TESTING.md`, the ticket, Scan Production Source Review 01, Scan
Atomicity Review 01, the resumed source handoff, the complete current five-file source drop, the
complete frozen `zec_scan` test, and pinned `WalletCommitmentTrees`/SQLite implementations.

## Sole task

Correct only the three rejected truthfulness issues. You may edit only:

- `wallet-broker/src/zec/scan.rs`;
- `wallet-broker/src/zec/store.rs`; and
- `wallet-broker/src/zec/test_support.rs`.

Preserve `zec.rs` and `fixture.rs` byte-for-byte at the accepted source-drop hashes. Preserve all
tests, fixtures, Cargo files/lock, schema, dependencies, cache/transaction/recovery design,
documentation, and every path outside this list.

### Actual official tree state

- Import and use stable public `WalletCommitmentTrees` on the reopened official `WalletDb`.
- At the official inspected height, obtain Sapling, Orchard, and Ironwood roots with each
  `with_*_tree_mut` callback and `root_at_checkpoint_id`. Require the sqlite Ironwood backend and
  the expected checkpoints after a scan; map every tree/storage/absence problem to the closed
  error boundary.
- Build the public `tree_root` value as a domain-separated bounded commitment over the exact
  serialized root bytes with explicit pool tags. Do not include tip hash/tree-size-only data as a
  proxy for root bytes. The pre-import checkpoint view may use an explicit deterministic empty
  marker because no official wallet trees exist yet.

### Derived pool classification

- Derive the classification string from the official reopened balance components already used to
  build `ScanBalances`: transparent, Sapling, Orchard migration-required, Ironwood pending, and
  Ironwood spendable. Include a stable empty classification when all are zero.
- Do not return a constant taxonomy and do not read fixture expected values. Keep balance override
  behavior explicitly test-only and deterministic.

### Honest fallible metrics

- Remove the `if let Ok`/`if let Some` swallowing around postcommit note/unrelated/reorg metrics.
- Never return a failed scan after the intended official wallet commit merely because a test-only
  observation then failed. Represent every fallible postcommit metric as explicitly unavailable
  (for example, `Option<usize>`) or compute it safely before commit. Clear/reset metric availability
  so a new committed scan cannot expose a prior stale value.
- Preserve the frozen hidden facade's `usize` signatures by making unavailable observations fail
  loudly with a bounded `expect`, as `recognized_note_count` already does. Successful scan paths
  must expose real production-derived values; no default zero or fixture constant.

Do not alter the accepted complete-candidate protocol, official transaction call graph, three
confirmations, fault positions, recovery authority, cache promotion, allocation limits, or error
codes. If exact root serialization is unavailable through the pinned public inferred types, stop
without edits and report the precise method/type boundary.

## Delivery boundary

Use `apply_patch` only. Do not run formatter, compiler, Cargo, Clippy, tests, Node, policy,
dependency, Git, network, fixture-generation, wallet/node/device, or cleanup commands. Do not stage,
commit, or push.

Return exact line counts/SHA-256 for the three corrected files, protected hashes for `zec.rs`,
`fixture.rs`, frozen tests and manifest, the exact root APIs/serialization used, classification
mapping, and metric-unavailability behavior. Reviewer inspection precedes Hermes execution.
