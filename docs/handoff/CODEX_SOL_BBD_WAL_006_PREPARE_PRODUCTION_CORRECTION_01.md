# Codex Sol Handoff — BBD-WAL-006 Prepare Production Correction 01

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This bounded corrective
handoff is authoritative together with both prior prepare production handoffs and Prepare
Production Source Review 01.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely before acting: `AGENTS.md`, `TESTING.md`, `tickets/BBD-WAL-006.md`, both prior
prepare production handoffs, Prepare Design Review 01, Prepare Production Source Review 01, both
frozen prepare/hygiene tests, all four current changed source paths, the accepted scan read-only
`WalletDb::from_connection` pattern, and the pinned `zcash_keys 0.16.1` address source.

## Exact scope

Correct the uncommitted four-path source drop in place. You may create/edit only:

- `wallet-broker/src/zec/prepare.rs`;
- `wallet-broker/src/zec.rs`;
- `wallet-broker/src/zec/store.rs`; and
- `wallet-broker/src/zec/test_support.rs`.

Starting identities are the four hashes in Prepare Production Source Review 01. Do not edit a
test, fixture/manifest, address/scan/session/vault source, Cargo/lock, policy, workflow, docs, or
another repository.

## Mandatory correction

1. Map closed typed decimal overflow to `SCHEMA` before spend-material access. Preserve the
   canonical positive-u64 rules and inventory-state corruption mapping.
2. Replace the three string-suffix receiver helpers with genuine ZIP-316 UAs. Select pinned
   upstream test vectors whose decoded compositions are exactly Orchard+P2PKH,
   Orchard+Sapling, and Orchard+unknown; decode them under mainnet and re-encode the resulting
   `zcash_keys::address::Address` under the fixture `LocalNetwork`. The helpers are test-only, but
   malformed strings are forbidden.
3. In `prepare.rs`, validate the receiver under the bound account network with
   `zcash_keys::address::Address::decode`. Require `Address::Unified`; if
   `UnifiedAddress::has_transparent()` return `TRANSPARENT_DOWNGRADE`; otherwise accept only
   `has_orchard() == true`, `has_sapling() == false`, and `unknown().is_empty()`. A wrong-network
   decode returns `SCHEMA`. Do not edit `address.rs`.
4. Make viewing-only `unlock` return `WATCH_ONLY` before deriving a digest, allocating a session,
   or mutating existing state.
5. Establish one fail-closed post-access operation boundary. Every `Err` after the spend-access
   counter advances—including official inspection/parsing, checked arithmetic, inventory
   outcome, proposal/build, fee bound, and random handle failure—must first apply
   `HandleInvalidation::OperationError`. Explicitly wipe a newly built raw PCZT before that
   invalidation when it is not yet map-owned. Pre-access validation, binding, and full-capacity
   failures keep their current behavior.
6. Protect the complete post-access operation with unwind handling or an equivalent drop guard.
   A real panic must call `invalidate_inner(..., PanicUnwind)` and resume the same panic only after
   all current handles and derived material are wiped. The synthetic facade helper is not a
   substitute.
7. Remove the new whole-database `fs::read` snapshots. While holding the existing account gate,
   preserve path/schema/cache validation, open the wallet through
   `open_read_only_connection`, and construct the real upstream wallet with
   `WalletDb::from_connection(&mut connection, params, SystemClock, OsRng)`. This mirrors the
   accepted scan source and ensures any accidental write fails at SQLite. Keep `lock_inputs:
   None`; do not add a transaction, lock, row, schema, digest oracle, or alternate store.
8. Bound the new prepare wipe log. A ring retaining at least one complete maximum-handle
   invalidation plus its derived-material event is acceptable; discard the oldest observation on
   overflow. Preserve post-zeroization observation and exact labels.

Preserve the accepted real proposal/PCZT call graph and inspection. The hidden pool override may
only choose the frozen outcome table. Actual proposal fee remains the only returned fee and is
compared with the caller bound only after proposal construction. Do not weaken any frozen
assertion or replace real wallet/PCZT work with a mock, fixture label, input-side oracle, or
authority-bearing role.

## Delivery boundary

Use `apply_patch` only. Read-only local inspection is permitted. Do not run a formatter, Cargo,
Rust, Clippy, tests, Node, policy, dependency, Git, network, fixture-generation,
wallet/node/device, cleanup, or deletion command. Do not stage, commit, or push.

Return the exact four changed paths with line counts/SHA-256, explain each correction, enumerate
all post-access error/unwind cleanup paths, confirm the read-only upstream wallet construction,
and disclose any remaining ambiguity. Hermes remains the sole future execution, evidence,
integration, and Git actor.
