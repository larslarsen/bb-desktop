# Codex Sol Handoff — BBD-WAL-006 Store Production Correction 01

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This durable correction
handoff is authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Governance parent: the commit containing this handoff

Read completely: `AGENTS.md`, `TESTING.md`,
`CODEX_SOL_BBD_WAL_006_STORE_PRODUCTION_01.md`,
`BBD-WAL-006-STORE-PRODUCTION-SOURCE-REVIEW-01.md`, the complete accepted `zec_store` test,
and the current four-file drop.

## Sole task

Correct the six blocking findings in Source Review 01. Preserve the accepted design and test
surface. Edit only:

- `wallet-broker/src/zec/store.rs`
- `wallet-broker/src/zec/test_support.rs`

`wallet-broker/src/zec.rs` is frozen at 214 lines/SHA-256
`800111bf587265b49764de71c6d2beb054d1906932005bb3513b2d4fc132d90e`.
`wallet-broker/src/zec/fixture.rs` is frozen at 279 lines/SHA-256
`5b1e91a730cddd82d0321383ec86f68dd781bf441a4b3e1db7e0514c5b9d5229`.
All tests, `lib.rs`, `address.rs`, dependencies, fixtures, policy, and governance are frozen.

## Required correction

1. Replace the raw-path filesystem-fault installer with a closed method that accepts only the
   validated account ID and closed target/network needed here, and internally derives exactly
   the local wallet database path beneath this `StateRoot`. The internal stored fault may retain
   that derived path. `test_support` must never pass a raw path into the fault seam.
2. Map an invalid local schedule read from SQLite to `STATE_CORRUPT`. During binding validation,
   prove the stored UFVK is valid for the exact bound network using the existing production
   address decoder/derivation path; map any failure to `STATE_CORRUPT` and expose no receiver.
3. Pass the actual issued receiver sequence into v1 store-state validation and reject a negative
   or greater checkpoint sequence. The checkpoint may lag current issuance; it may not lead it.
4. Retain the wallet count/distinct/length aggregate and additionally prove every published
   `zcash_client_sqlite::wallet::init::migrations::CURRENT_LEAF_MIGRATIONS` ID is present exactly
   once. For the cache, prove the entire migration table contains exactly one distinct 16-byte
   value and that value is the pinned blockmeta ID. Do not add `uuid` or any dependency.
5. Remove the fallible post-commit synchronization path from migration/checkpoint success.
   Configure/verify full SQLite synchronous behavior before the transaction, let SQLite commit be
   the journal/database durability boundary, and keep every typed write/sync/directory/commit
   fault return before commit so rollback preserves exact bytes/state. Do not claim that opening
   and syncing the pre-commit main database flushes uncommitted SQLite pages. Sync newly created
   files/account directory after bootstrap initialization before returning the account; a sync
   failure there returns failure and never reports success. Do not weaken the exact-byte migration
   fault contract.
6. In inspection, check borrowed text/blob lengths before `to_vec`, cap the aggregate decoded
   cell count and aggregate decoded byte count with checked arithmetic, and return `LIMIT` before
   an over-limit allocation. Keep the public projection redacted and unchanged.

Use the pinned upstream published `CURRENT_LEAF_MIGRATIONS` constant directly; do not copy a
private migration list, invent IDs, run a generator, or hard-code a test result. Keep `unsafe`,
processes, network, raw SQL/path APIs, arbitrary metadata, and new dependencies absent.

## Restrictions and report

Use `apply_patch`. Do not run Cargo, rustfmt, tests, linters, Node, policy, Git, network, cleanup,
or any other execution. Stop if either authorized file is insufficient.

Report both final line counts and SHA-256 hashes; the closed fault API; stored-state validation;
upstream migration proof; SQLite durability ordering; inspection bounds; protected hashes; and
any concern. Hermes remains the only execution/integration actor after reviewer acceptance.
