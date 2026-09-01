# Grok Build Handoff — BBD-WAL-006 Scan Atomicity Review 01

You are **Sr Dev — Grok Build**. This is a read-only protocol-design review.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Read completely: `AGENTS.md`, `TESTING.md`, `tickets/BBD-WAL-006.md`,
`docs/architecture/BBD-WAL-006-UPSTREAM-REVIEW.md`, Scan Source Stop Review 01, the complete
`zec_scan` test, current store/fixture/test-support source, and the pinned upstream 0.24.0/0.22.0
wallet, extension-transaction, block-source, scan, truncate, and cache implementations.

## Sole task

Determine whether the frozen scan/reorg contract can be implemented without relaxing externally
observable atomicity, adding signing/network authority, or using test-only scan logic. Do not edit
any repository file. Do not run tests, Cargo, Node, Git mutation, network, fixture generation, or
another application. Read-only source/search commands are allowed.

## Design candidate to audit

Evaluate a same-directory durable candidate-cache protocol that avoids direct protobuf decoding:

1. Validate the complete closed fixture/block input before mutation.
2. Construct a new mode-0600 `compact.sqlite3.candidate` with stable `BlockDb` schema and insert
   only validated opaque compact-block bytes through bounded direct `rusqlite` writes. Configure
   full sync, commit, fsync the file and account directory, then reopen/validate it read-only.
3. Under the account gate, compare the authoritative wallet tip hash/height with the committed
   cache and candidate. Resolve any prior interrupted promotion before exposing inspect/scan/open
   state: promote only the file whose chain identity matches the committed wallet tip, reject
   ambiguity/corruption, and never silently empty state.
4. For a new canonical range or one-block replacement, scan from the durable candidate
   `BlockDb`. Use one `WalletDb::transactionally_with_extension` closure for the official wallet
   rewind/scan plus extension tip/state. All injected failure seams fire before its commit.
5. After wallet commit, atomically rename the matching candidate over the committed cache and
   fsync the account directory. A post-wallet-commit promotion/sync problem must never be returned
   as a failure after advance; it must leave a recoverable state that the next gated operation
   deterministically resolves or fails closed before exposing inconsistent scan state.
6. Candidate/orphan handling must cover crashes before wallet commit, after wallet commit/before
   rename, after rename/before directory fsync, and after directory fsync. It may not depend on
   caller paths, network, clocks, cleanup success, or unpersisted memory.

Audit whether wallet tip/tree state and cache block identities are publicly inspectable enough to
make recovery unambiguous; whether `scan_cached_blocks` can operate against the candidate inside
the wallet transaction; whether SQLite/Unix rename and fsync ordering actually preserves at least
one recoverable copy in every crash window; whether replay and one-block replacement remain
idempotent; and whether unexpected I/O after wallet commit can be represented honestly without
reporting false durability.

Also compare, without assuming acceptance:

- an application `ext_*` journal coupled to the wallet transaction;
- making the compact cache explicitly non-authoritative/rebuildable;
- adding an exact direct protobuf dependency/custom bounded `BlockSource`; and
- any narrower stable upstream API path missed by Sol.

## Required response

Return a threat-model table for each crash/fault boundary, exact pinned APIs and visibility used,
whether the candidate protocol is sound, and the smallest safe contract/source/dependency change.
Call out any test expectation that cannot distinguish a shortcut. Recommend **accept**, **correct**,
or **reject**. If no design satisfies the ticket, say so explicitly; do not relax requirements by
implication.
