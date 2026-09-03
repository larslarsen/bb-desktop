# BBD-WAL-007 Slice-4 Source Review 05

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Governance parent: `4f51a9d6`

Result: **REJECTED — FINAL ATTEMPT-LIFETIME AND IDENTITY-SAFE CLEANUP CORRECTION REQUIRED**

Reviewed unstaged Grok 4.6 High Correction-04 drop:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/vault.rs` | 776 | `bb79b16f24a9abbcd266e856a37ee982bde8fcdc2da3858e75d34bdd34a4af86` |
| `wallet-broker/src/xmr.rs` | 7 | `8b382ef673e82001720727c161f365c0fa3c1401d1bdaa4d20ee2683c001559b` |
| `wallet-broker/src/xmr/account.rs` | 3,020 | `07a4e815082a6ca58c4d1d8b5c8f330f62391c89d4002c75a276750b71bc6455` |
| `wallet-broker/src/xmr/store.rs` | 1,320 | `412b1a7b92b07c0d39963e19565c467968e47ca2a02d74b36bf19369787c1b1b` |
| `wallet-broker/src/xmr/process.rs` | 1,808 | `b990de3e80db0a4d354ec6119fbc746b27a8989909e702b63270b6d5b43fd52a` |
| `wallet-broker/src/xmr/rpc.rs` | 2,418 | `4f52baf1532a374dda748ec03a4a0a4311fd970b441f258f576c26030bb5ac14` |
| `wallet-broker/src/xmr/test_support.rs` | 3,924 | `5695a67aac219f36e5cd4df156f0708843084c9befb8e396f641c7c3348f966e` |

`HEAD == origin/master == 4f51a9d6`, the index is clean, these are the only seven
worktree paths, and `git diff --check` is clean. The frozen 586-line account test remains
byte-exact at SHA-256
`5e8afabaa8c820be84a010a88fef0e07617e67934c06b32ab445901b1ddad35b`.
The reviewer ran no formatter, compiler, test, build, binary, Node/npm,
policy/security, package-manager, staging, commit, or push command while reviewing the
source drop.

Correction 04 is parseable by direct inspection and materially closes every broad
Correction-03 defect. It publishes normal vault/state identities before write or SQLite
work, makes wallet capture mandatory on the ordinary paths, reconciles unwind, binds
sealed and stored primaries before child start, strictly reopens state, retains the
state and directory capabilities, validates process-directory ownership, and syncs new
directory parents. Preserve all of that work. The retained state handle plus
`SQLITE_OPEN_NOFOLLOW` and before/after identity checks remains the accepted safe-Rust
SQLite boundary. The drop is not yet eligible for execution because several exceptional
edges still violate the same ownership contract.

## Blocking findings

1. **The attempt ledger is retired before returned success, and failed lock teardown
   does not latch the account unavailable.** `seal_and_persist` calls
   `port.commit_attempt()` while `attempt_active` remains true and before final secret
   cleanup and `success`; `success` then commits the same attempt a second time. An
   unwind in that interval sees an active operation with an empty ledger and cannot
   reconcile the just-created vault, state, wallet, and keys. The Correction-04 contract
   explicitly forbids resetting an active ledger before success or proven cleanup.
   Retire it only at the single successful operation boundary. Separately, `lock`
   returns a teardown error through `teardown?` without setting `unavailable`, even
   though exact child cleanup is then uncertain. A lock teardown failure must wipe
   secrets, clear retention, return compound `INTERNAL`, and latch later operations
   unavailable.

2. **Wallet artifact capture still stops after the first inspection failure.** In
   `capture_wallet_artifacts`, a wallet-path inspection error returns before the keys
   path is inspected. If the wallet path is uncertain but an exact keys inode exists,
   rollback never records that keys inode. Inspect both paths independently on every
   post-create/generate/restore outcome, retain every exact identity discovered, and
   return failure only after both inspections have run. Any uncertainty still compounds
   to `INTERNAL` and latches unavailable.

3. **Exclusive creation still has a post-create/pre-ledger failure window.** Both
   `exclusive_create_active_envelope` and `exclusive_create_state_file` perform fallible
   handle metadata/type work after `create_new` succeeds but before the system port can
   publish an identity or even record uncertainty. If that work fails, an exclusive
   active vault or state path remains while rollback has an empty ledger and leaves the
   account reusable. Split creation from identity derivation or return an explicit
   created-but-unidentified outcome: immediately after the namespace create succeeds,
   the attempt must own either an exact identity or an uncertainty latch. Clear the
   provisional uncertainty only after the exact device/inode is in the ledger. Never
   adopt a pathname after `AlreadyExists`.

4. **Quarantine error cleanup can unlink a substituted destination.** After the
   hard-link operation, the destination-open error branch removes any entry currently
   found at the random destination. The device/inode-mismatch branch likewise calls
   `remove_file(destination)` without first proving that the pathname still names the
   destination handle it just inspected. A same-user replacement can therefore cause
   cleanup to unlink an entry it did not create. Never unlink an unknown destination.
   When a destination handle identity is available, compare a fresh no-follow pathname
   identity immediately before unlinking, remove only on exact match, and synchronize
   the validated parent. Any open, metadata, comparison, unlink, or sync uncertainty
   must remain a compound cleanup failure; it must not be hidden by best-effort ignored
   results.

5. **Two concrete hygiene defects would undermine the warning/secret gate.** The
   wallet-password entropy buffer returns through `?` before zeroization if
   `fill_entropy` partially writes and fails. Wipe that buffer on both success and
   failure. `PathSqliteSurface::create_new` is an unused combined constructor that also
   bypasses immediate attempt-ledger publication, so remove it rather than preserve a
   second unsafe call path. The `i64` stored-sequence check compares an `i64` value to
   `i64::MAX`, an always-false upper-bound test; remove the useless comparison and any
   now-unused private constant without adding a lint suppression. Preserve the SQL
   signed-64 constraint for untrusted SQLite rows and the future public receiver limit.

## Review decision

These findings are bounded to the same account/store/test-support source and require no
architecture change. Grok 4.6 High remains the sole source actor for Correction 05;
Grok has continued to produce useful senior-level corrections, so Sol escalation is not
authorized. Hermes execution/integration, formatting, compilation, Slice 5, broader
acceptance, and the real offline local-Monero gate remain unauthorized.
