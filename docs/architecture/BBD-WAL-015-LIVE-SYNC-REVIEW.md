# WAL-015 live Zcash testnet sync — accepted design

Status: COMPLETE; accepted source edda35308913d59683a08be58052dc7c2d95dbaf.
Q2/Q3 resolved by delegated reviewer judgment. The owner responded “I don't
know or care about any of that.” This supersedes the earlier requirement to obtain
specific product selections. Continue autonomously under tickets/BBD-WAL-015.md.

The owner asked to continue after WAL-014 Receive. The next intended outcome is
native testnet synchronization with an actual scanned balance and visible progress.
Reviewed source baseline: e85dfdc8b6e06fdd09f84cee3c6ac37037448437.

## Product decisions

Reviewer chooses an editable suggested testnet endpoint and direct TLS connections.
The native sync screen explains that the selected server sees the connection IP.
Sync starts only on an explicit native action. No compulsory endpoint, silent
fallback, Tor dependency or further owner decision is required for this slice.

## Historical findings from the preimplementation baseline

The broker pins zcash_client_backend 0.24.0 and zcash_client_sqlite 0.22.0. Current
Cargo features do not enable a lightwalletd transport. Existing upstream cryptography,
compact-block scanning and wallet storage remain the implementation foundation.

- zec/scan.rs::execute first requires Network::Local and the validated fixture's
  exact local consensus schedule. Its later Testnet match arm does not make this a
  live testnet scanner. Removing the initial guard is not a valid implementation.
- The existing scanner retains the complete fixture cache, limits it to 4096 rows /
  64 MiB, derives checkpoint trees from fixture history, and implements the fixture's
  single-block replacement scenario. It is not a general live history/reorg loop.
- Receiving initializes the official SQLite schema and stores the account's UFVK in
  the broker extension. The offline scanner subsequently imports the upstream viewing
  account with a fixture birthday. Live sync needs its own valid birthday/checkpoint
  initialization; existing testnet extension heights are zero placeholders.
- AddressAccount::open_viewing_with_network calls scan::recover_account. Recovery
  requires the official wallet tip to match the retained compact cache. Independently
  advancing wallet.sqlite3 with upstream sync would violate that invariant on reopen.
  Preserve Receive across initial sync, cancellation, restart and reorg; explicitly
  design the testnet recovery/schema transition before touching this database.
- store.rs recognizes an exact broker extension schema. Adding a sync table without
  updating version recognition and migration would make existing reads fail closed.
  Do not weaken schema checks or reset existing account state to make sync work.
- scan::inspect can report a zero-valued fixture observation when the upstream account
  has not been imported. That is not a live balance. WAL-014 correctly avoids exposing
  it and displays “Balance unavailable — not synced”. Preserve that distinction.

The locally installed upstream sync.rs documents no progress notification and no
interrupt mechanism other than ending the process. Its flow also refreshes transparent
UTXOs when transparent-inputs is enabled, which this broker already enables through
PCZT. Therefore sync::run must not be wired directly into an egui callback or adopted
without reviewing cancellation, resource limits and address-query privacy.

Use maintained upstream scan_cached_blocks, WalletDb, ChainState/TreeState conversion,
account import, wallet summaries and rewind operations. Broker code should coordinate
those APIs, not implement note decryption, tree hashing, monetary arithmetic or consensus
rules. Upstream reference: [librustzcash synchronization source](https://github.com/zcash/librustzcash/blob/main/zcash_client_backend/src/sync.rs).
Implementation must use the pinned local 0.24.0 source rather than assume main is identical.

## Required implementation contract

The reviewer converted these requirements into tickets/BBD-WAL-015.md after the owner
delegated the product choices. That ticket fixes the separate viewing-cache design;
this earlier inspection explains why directly advancing the fixture store was rejected.

1. Native account-bound Sync action, visible progress, cancellation and a last-scanned
   height. Never connect merely because the app starts, a window repaints or Receive is
   opened. Transport and scanning run outside the UI/account-manager mutex. Closing,
   locking, expiry and broker EOF cancel owned work without leaving a detached worker.
2. Only viewing material crosses into the worker. Authenticate its account binding
   against the unlocked account before starting; reject results from an old job/account.
   No seed, passphrase, UFVK, account-derived address or transaction broadcast RPC is
   sent to a compact-block server. Any additional request family requires review.
3. Validate testnet identity, bounded heights, protocol compatibility and all response
   sizes. Validate checkpoint height/hash, contiguous block heights and previous hashes.
   A server's chain label is not independent verification of consensus. Connection
   failures and malformed responses must not become a fresh zero balance.
4. Choose and persist a scan birthday that cannot silently omit earlier funds. Existing
   accounts/restores have no accepted live birthday; do not default them to today's tip.
   Any user-selected starting height must explain the history it excludes.
5. A bounded batch commits upstream scanned data and its broker checkpoint together.
   Cancellation and crash recovery preserve the last consistent state and receiver
   sequence. Define rewind depth and behavior when a reorg exceeds retained history.
6. Display balances from a real upstream summary only after the required range is
   scanned. Keep pending and spendable values distinct, use exact integer amounts, and
   label partial/stale/offline observations. Transparent and unscanned pools must not
   be represented as a complete total. Mainnet and payment paths stay disabled.
7. Tests first: recorded compact blocks through the actual worker/scanner/storage
   boundary; nonzero incoming funds; repeat/restart/no double counting; malformed or
   wrong-network input; partial sync; cancellation; reorg; wrong UFVK; native events and
   stale job results. Falsify continuity/account binding or commit atomicity and show
   the meaningful test fails. Ordinary tests remain offline and credential-free.
8. Review exact transport dependency pins/features and the resolved lockfile. Hermes
   owns fetching/resolution, execution, security scans, native rebuild and integration.
   Keep fixed native Mesa/X11 rendering and the unrelated npm/policy edits untouched.

## Endpoint and direct dependency review

Suggested editable endpoint: `https://testnet.zec.rocks:443`. The operator links its
[Hosh endpoint list](https://hosh.zec.rocks/zec) from [zec.rocks](https://zec.rocks/);
the list identifies this endpoint as Testnet and online at review time. Availability
is not a promise or independent chain validation. The app must validate each run and
offer Custom, without silently switching services. No account data was queried.

Preserve all current Zcash pins. Enable only backend `lightwalletd-tonic`, not `sync`
or `tor`. Direct candidate additions for production authoring:

- tonic =0.14.6, defaults false, channel/tls-ring/tls-webpki-roots;
- tokio =1.52.3, defaults false, rt/net/time/macros;
- prost =0.14.4, defaults false, std (already present transitively).

The pinned backend accepts tonic/tonic-prost 0.14 and prost 0.14. Tokio and prost
versions are locally available. [Tonic's published manifest](https://docs.rs/crate/tonic/latest/source/Cargo.toml)
confirms channel enables client transport and the selected TLS features choose ring
and WebPKI roots. Do not enable the broader transport/server/router defaults in the
application. Hermes must fetch missing crates, resolve the lockfile and audit the
result; this feature review is not a claim that the new graph has passed checks.

The source actor is writing tests. WAL-014 remains the runnable build until the new
implementation has passed review and execution. No user account has been read.

## Production draft review and corrective authorization

The first live.rs/test-support draft was rejected as incomplete: it lacked real
reorg handling, complete schema and official-account binding validation, committed
progress, and pool-specific balances. Test helpers restored database bytes after
success instead of injecting transaction failures, fabricated scanner/search
observations, short-circuited malformed responses, and returned cached inspection
markers. These are not accepted evidence. The core source split must replace them
with real upstream transactions and thin observation hooks. Root remains reviewer;
Sol owns source and Hermes alone executes.

Shared source boundary preserves raw LightdInfo/BlockId/TreeState through the
transport and recorded source, then validates them in the same core. Existing
resolved incrementalmerkletree0.8.2 legacy-api supports upstream fixture tree
serialization; no custom cryptographic conversion is authorized. LOCK_EDGE01
actual Hermes session20260910_150301_cd22a3 exited0, changed only that root edge,
and preserved every transitive package/version/checksum/dependency list. Input
pins and final lock hash were independently checked. AUDIT01 remains applicable.

Reviewer corrected one contradictory test requirement: a fixture retaining only
eight block checkpoints cannot demonstrate one hundred distinct retained-checkpoint
queries. Require an actual bounded search across the retained fixture checkpoints
and no queries below its birthday checkpoint, not fabricated call counts. The
cancellation Result remains Err(CANCELLED); persisted inspection/latest progress
indicates Stale after a partial committed scan.

Three local TLS/gRPC tests were reviewed, including runtime-generated CA trust,
exact protobuf request bodies and RPC allowlisting, decoded message size limits,
entire-stream deadlines and explicit cancellation. Follow-up adds probe-worker
RAII and attempts all owned server cleanup after reap. Their probe API must remain
absent until Hermes records its expected missing-API red; no execution claim yet.

Manager/UI corrections require completed jobs to be reaped, retry after failure,
visible phase/status, and private snapshots cleared after lock or polling errors.
These are implementation corrections within the authorized Sync lifecycle.

COMPILE01 actual Hermes session20260910_151731_f2b8b1 completed the first five
production format operations, then stopped at two missing semicolons in the new
test-support hostile-entry match. Cargo did not run. Reviewer checked all final
pins and actual tool execution; no source drift beyond authorized formatting.
Primary is authorized to fix those parse errors and make the deep fork's raw
metadata tip hash consistent with its raw TreeState.

Two additional preflight regressions are authorized in their own test source: the
pinned upstream CompactBlock helpers prefer a nonempty parsed header over explicit
hash fields, and CompactTx::txid requires exactly32 bytes. The real batch validator
must enforce those preconditions before passing untrusted messages upstream. Their
missing test wrapper is kept absent until expected RED is recorded.

COMPILE02 actual Hermes session20260910_152009_c124f2 completed formatting and
ran Cargo. It stopped on11 ordinary source compiler errors: six core BlockHash
as_ref calls, one equivalent fixture call, two missing Merkle generic bounds and
two Linux metadata method/import mismatches. All relevant final pins were checked.
This is not the transport expected RED. Source actors fixed only those reported
errors plus the newly unused import and froze again for a combined compiler pass.

A real hot-journal regression is authorized separately. It must crash an owned
SQLite writer after actual dirty-page spill, then reopen through prepare_live_path
and verify rollback/resume. The present preparation ordering rejects the journal
before later recovery; do not claim crash recovery until that regression executes.

## Implemented outcome
The accepted implementation follows the fixed separate viewing-cache design and
uses live WebPKI TLS transport with bounded upstream scanning. Reorg balances reuse
the existing upstream orphan projection so replaced notes are not falsely counted
as pending. Real SQLite hot-journal recovery precedes schema/account checks under
private filesystem validation. UI and worker lifecycle tests cover actual native
authority, lock/cancel/join behavior and stale-result rejection. See completedticket
and integration evidence for exact verification and remaining releaseblockers.
