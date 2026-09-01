# BBD-WAL-006 Scan Atomicity Review 01

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Governance parent: `60205e9d`

Senior protocol reviewer: Grok Build

Result: **CORRECTED DESIGN ACCEPTED — SCAN SOURCE MAY RESUME**

Grok's read-only review confirmed that the pinned upstream APIs can satisfy the frozen scan and
one-block-reorg contract without a new dependency, an unstable feature, signing/network authority,
or test-only scan logic. The originally proposed candidate-cache protocol was incomplete. The
corrected protocol below is normative for the resumed source handoff.

## Accepted API boundary

- Stable `BlockDb` is read-only but can decode opaque compact-block rows through public
  `BlockSource::with_blocks`; adapter-side protobuf decoding and a direct `prost` dependency are
  unnecessary.
- The accepted direct `rusqlite = 0.37.0` pin is the only stable bounded write path for a
  `BlockDb`-shaped candidate cache. Stable upstream exposes no cache insert API.
- `WalletDb::transactionally_with_extension` supplies one wallet transaction and one authorized
  `ext_*` transaction. Official account import, one-block rewind, replacement scan, and extension
  state must all use the inner `WalletDb<SqlTransaction, _>`; connection-level methods would
  commit separately.
- `WalletRead::get_max_height_hash` is the authoritative committed tip. Extension `scan_tip`,
  fixture labels, counters, and memory are not recovery authority.
- `scan_cached_blocks` may read the separate, closed-and-reopened candidate `BlockDb` while its
  wallet writes remain inside the extension transaction.

## Corrected durable protocol

1. Validate the complete closed manifest, paths, byte lengths, hashes, limits, scenario, and input
   ordering before creating a candidate or opening a wallet write transaction.
2. Under the account gate, recover any prior candidate before starting new work. Compare official
   wallet tip state with fully validated committed/candidate cache identities. Promote the sole
   exact matching candidate, discard a nonmatching orphan only when the committed cache matches,
   and fail closed when neither or conflicting same-tip datasets match. Before the first official
   account import, the bound checkpoint-99 state and empty committed cache are the only valid old
   state.
3. Create fixed-path `compact.sqlite3.candidate` with `create_new`, mode `0600`, and the accepted
   cache schema. Pin and verify `journal_mode=DELETE` and `synchronous=FULL`; refuse cache sidecars.
4. Build a complete intended cache snapshot, not a delta: bounded-copy the validated committed
   opaque rows, then insert/replace the validated new range. Commit, file-fsync, directory-fsync,
   close every connection, reopen read-only, validate the exact schema, and decode through
   `BlockSource`. Check raw hash-vector lengths before calling panic-capable hash helpers; prove
   bounded contiguous heights and previous hashes.
5. Use exactly one `transactionally_with_extension` call on the wallet. Import the UFVK as an
   official view-only account on first scan, using an `AccountBirthday` derived from the decoded
   checkpoint/first-block boundary. For a supported reorg, rewind exactly one block and apply its
   replacement in that same transaction. Update accepted `ext_*` state there. Use three untrusted
   confirmations for the frozen local schedule. Every typed injected scan fault is a logical abort
   before transaction commit.
6. Reconcile an unexpected wallet-commit error after closing/reopening: old official tip means the
   call failed unchanged; exact intended new official tip means the commit occurred and the call
   proceeds as success; any other state fails closed. SQLite atomicity permits old or new, not a
   partial wallet state.
7. After a known wallet commit, close cache and wallet connections, rename the complete candidate
   over `compact.sqlite3` in the same directory, and fsync that directory. A rename or directory
   sync problem after wallet commit is not returned as a failed scan. The call returns its committed
   result and leaves the two durable file states for mandatory next-operation recovery.

The candidate must use SQLite DELETE journaling. Renaming a WAL-mode main file without its sidecars
is corrupt. The candidate must also contain the full row span: replacing a cache with a height-107
delta would silently discard heights 100–106.

## Crash decision table

| Boundary | Durable authority | Required recovery/result |
| --- | --- | --- |
| Validation or candidate construction failure | Old wallet and committed cache | Fail unchanged; candidate is never authoritative |
| Durable candidate, before wallet commit | Old wallet and old committed cache | Discard orphan candidate; expose old state |
| Injected rewind/scan/commit fault | Old wallet and old committed cache | Transaction abort; fail unchanged |
| Wallet commit, before rename | New wallet, old cache, new candidate | Return success; next gated operation promotes candidate |
| Rename, before directory fsync | New wallet; filesystem may expose old+candidate or new target | Return success; recover by official tip and exact dataset identity |
| Directory fsync complete | New wallet and new committed cache | Success; replay is a no-op |
| Both files claim wallet tip but differ | Ambiguous/corrupt | Fail closed; never guess, empty, or rewrite |
| Neither file matches wallet tip | Corrupt | Fail closed; never silently initialize |

No cache-generation schema change is authorized. Exact byte/dataset comparison makes an identical
same-tip candidate redundant and a differing same-tip candidate ambiguous; failing the latter
closed preserves the accepted V1 extension schema.

## Rejected shortcuts

- No delta candidate, WAL-mode rename, `ATTACH`, unstable `FsBlockDb`, new `prost`, or cache write
  through an outer connection.
- No separate rewind and replacement commits.
- No inspect DTO sourced from extension `scan_tip`, fixture expected values, counters, or cached
  in-memory scan output. Tip, trees, notes, balances, and pool classification come from official
  reopened wallet state.
- No hard-coded recognized/unrelated/reorg counts. `ScanSummary` has no Ironwood count; counters
  must be derived from the real bounded production scan and durable wallet effects.
- No reported failure after authoritative wallet state has advanced.

The protected source/tests/fixture remain unchanged from Scan Source Stop Review 01. The resumed
Sol handoff is the only source authorization.
