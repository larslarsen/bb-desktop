# Codex Sol Handoff — BBD-WAL-006 Scan Runtime Correction Resume 01

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This durable handoff is
authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely before acting: `AGENTS.md`, `TESTING.md`, `tickets/BBD-WAL-006.md`, Scan Runtime
Source Stop Review 01, Scan Runtime Snapshot Review 01, Scan Runtime Design Review 01, Scan Gate
Runtime Review 01, the complete frozen `zec_scan` test, current `scan.rs`, and the pinned upstream
sources cited by those reviews.

## Authorized paths and starting identities

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/tests/zec_scan.rs` | 325 | `084aa1758cc10bdd1f9fc63f935e70328aca1f2f668ff8f67234cef7f5656434` |
| `wallet-broker/src/zec/scan.rs` | 1,400 | `17d411e4af9e64d1169d6326ae17f4ae13e1283236d690c80984971507dedab9` |

Author the exact test-source correction first, then production source, using `apply_patch`. Every
other source, test, fixture, manifest, lockfile, policy, document, workflow, package, and
repository path is frozen. In particular preserve these identities exactly:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/zec.rs` | 218 | `ae06d6a9b4448603860b55bf4eef42a1e970c414049a9304f42b436d2bf49cb4` |
| `wallet-broker/src/zec/fixture.rs` | 614 | `318820c6f125f2318ba0caf5ae44835b36b0f67856fc66b83401961f0c301b36` |
| `wallet-broker/src/zec/store.rs` | 1,816 | `d0b97b802209489804e5d550ef90138a87342e155c519a686b2d900189f6603f` |
| `wallet-broker/src/zec/test_support.rs` | 1,220 | `02609ae79bb8d35a5c7fb0058f0ab56fa5deb27c94cfa2ddb81347a2f38ac034` |

Require `wallet-broker/src/zec/prepare.rs` to remain absent.

## Test-source correction

At `zec_scan.rs:302`, replace only the first loop binding `(value, accepted)` with `(value, _)`.
Do not alter its values, body, assertions, second loop, test count, or any other test token.

## Production correction

Implement every invariant in Scan Runtime Snapshot Review 01. It supersedes the prior request to
use `transactionally_with_extension` for inspection; do not call that API in the inspection path.

1. After recovery, open one guarded `SQLITE_OPEN_READ_ONLY` wallet connection through the
   existing opener, load `rusqlite::vtab::array`, and compare `PRAGMA main.data_version` before
   and after every assembled observation. Do not open an outer transaction and do not retry a
   changed version. Evaluate binding validation, UFVK, receiver sequence, official wallet reads,
   and aggregate projection sequentially on this same connection. Refactor or inline the current
   connection-opening UFVK/sequence helpers so they do not escape the stable window.
2. Construct a scoped `WalletDb::from_connection(&mut connection, params, SystemClock, OsRng)`.
   Use official APIs for the sole account, view-only purpose/UFVK, max height/hash, current roots,
   and official summary. Drop the wrapper before querying the connection directly. The connection
   is read-only, so an accidental write fails; all upstream-owned read transactions must complete
   without nesting.
3. Change `official_tree_digest` to use `root_at_checkpoint_depth(None)` for Sapling, Orchard, and
   Ironwood. Remove its obsolete height parameter. Preserve the actual option shape and all
   fail-closed checks: one missing-root check for Sapling and Orchard, and outer optional-tree plus
   inner missing-root checks for Ironwood.
4. Execute one aggregate projection row using `v_received_outputs`,
   `v_received_output_spends`, `accounts`, and `transactions`. Bind the official account UUID and
   checked target height. Import and convert upstream `DEFAULT_TX_EXPIRY_DELTA` and
   `zip317::MARGINAL_FEE`; copy no numeric constant. Match the exact creating-transaction expiry
   and complete spending-transaction unexpired predicates from the snapshot review, including
   `>=` boundaries and the mined-below-target spend branch. Match spend rows by account, pool, and
   received-output ID. Separately count same-account unmined transparent outputs and unknown pool
   codes in that same statement and fail closed if present. Reject negative, overflowing,
   unknown, ambiguous, or unconvertible results.
5. Subtract the three shielded orphan sums only after proving each fits within its official
   pending buckets. Preserve official spendable/locked authority. Reconcile every adjusted pool,
   Ironwood pending, displayed component sum, and adjusted official account total with checked
   arithmetic. Derive pool classification from adjusted display components. Any mismatch is
   `STATE_CORRUPT`.
6. Preserve empty-account, balance-override, recovery, one-account, account-purpose, UFVK,
   receiver-sequence, tip/hash, and error-contract behavior. Every return after the before sample,
   including empty/override success, must compare the after sample first.

Do not mutate wallet-owned tables, delete/reclassify retained transactions, weaken recovery,
special-case fixture values, add a lint allowance, change a test assertion, add a dependency, or
change an error contract. Do not run rustfmt, Cargo, Rust, Clippy, tests, Node, npm, policy, a
compiler, a linter, Git, network, fixture, wallet, node, device, cleanup, or deletion command. Do
not stage, commit, or push.

After editing, use only read-only file inspection, `wc -l`, and `sha256sum`. Return both changed
path line counts/hashes, re-prove all four frozen identities and absent `prepare.rs`, enumerate
the connection/window, helper/signature/import/SQL/arithmetic changes, and identify any remaining
ambiguity. The reviewer will inspect and decide whether Hermes may restart the gate.
