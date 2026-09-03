# BBD-WAL-007 Slice-4 Source Review 04

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Governance parent: `121913f5`

Result: **REJECTED — UNPARSABLE DROP AND INCOMPLETE ARTIFACT OWNERSHIP**

Reviewed unstaged Grok 4.6 High Correction-03 drop:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/vault.rs` | 776 | `bb79b16f24a9abbcd266e856a37ee982bde8fcdc2da3858e75d34bdd34a4af86` |
| `wallet-broker/src/xmr.rs` | 7 | `8b382ef673e82001720727c161f365c0fa3c1401d1bdaa4d20ee2683c001559b` |
| `wallet-broker/src/xmr/account.rs` | 2,849 | `76840451ec7c87d7ed5f849f6b700b166b2a9c8cbead5163bea7f01fe25e8bc5` |
| `wallet-broker/src/xmr/store.rs` | 1,195 | `a4d89f8555c9f14626e5c9f989e636ed22eab27d713e62015a1f76f7bd97aaa5` |
| `wallet-broker/src/xmr/process.rs` | 1,752 | `b0ef7445fd8c3428f860b2656f83537cbe4ee5d97101329b2027af091eab3213` |
| `wallet-broker/src/xmr/rpc.rs` | 2,418 | `4f52baf1532a374dda748ec03a4a0a4311fd970b441f258f576c26030bb5ac14` |
| `wallet-broker/src/xmr/test_support.rs` | 3,924 | `c308efb25501e778f9b4a69664c88ec2e287478bedf64dc3923611ced8bc815e` |

`HEAD == origin/master == 121913f5`, the index is clean, these are the only seven
worktree paths, and `git diff --check` is clean. The frozen 586-line account test remains
byte-exact at SHA-256
`5e8afabaa8c820be84a010a88fef0e07617e67934c06b32ab445901b1ddad35b`.
The reviewer ran no formatter, compiler, test, build, binary, Node/npm,
policy/security, package-manager, staging, commit, or push command.

Correction 03 correctly isolates manager observation state, gates an unavailable
account, separates recovery cleanup, orders preflight before the node, uses strict
existing vault/state attachment, closes the public account authority, strengthens the
schema, adds zeroizing recording custody, and removes duplicate live validation/close
RPCs. Preserve that work. The drop is not eligible for execution.

## Blocking findings

1. **The source does not parse.** `pub(crate) struct AccountManager<P: AccountPort>` at
   `account.rs:886` is missing its opening `{` before the first field. This was found by
   direct source inspection; no compiler or formatter was run.

2. **Exclusive vault creation loses ownership on every late failure.**
   `write_exclusive_envelope` creates a staging file but has no exit guard. Write/fsync/
   metadata/link/open/unlink/sync failures can leave staging behind. More critically,
   after `hard_link` installs the active envelope, any later failure returns before
   `seal_vault` stores the returned `ArtifactIdentity`, so rollback has no active-vault
   ledger entry. A vault-directory sync failure again leaves an active envelope while
   manager state says the seal failed. Because this is create-new, use a direct
   `create_new` active handle or otherwise publish exact identity to the attempt ledger
   immediately after the no-replace namespace operation, before any fallible write/
   sync/validation step. Reconcile every early/late failure and surface cleanup
   uncertainty to the rollback latch.

3. **Wallet/keys ownership capture is optional and incomplete.** Create, generate, and
   restore call `capture_wallet_artifacts` but discard its `Result`. They can report
   success with missing, partial, wrong-mode, wrong-owner, or unrecorded files, and a
   failed RPC can leave files that rollback does not own. Combine the RPC and capture
   outcomes, retain any partial exact artifacts, require a complete revalidated pair on
   RPC success, and make capture uncertainty compound to `INTERNAL`. Captured regular
   files must prove exact owner and `0600` mode before success.

4. **State ownership can adopt a raced file and the cached connection survives
   cleanup.** `persist_state` calls `open_store_new`, then captures whatever pathname
   exists even when exclusive creation failed because another entry won the race. It
   can therefore quarantine a file it did not create. On a real post-open persistence
   failure, rollback leaves `self.store` holding the renamed file open. A retry then
   fails because the store is still attached. On successful create followed by open,
   `open_store_existing` returns the cached connection instead of strictly reopening
   the currently validated path. Publish the exact state identity to the attempt ledger
   immediately after successful exclusive file creation, never on `AlreadyExists`;
   drop an owned connection before cleanup; clear it after rollback; and strictly
   reopen/reattach the current state path on every open attempt.

5. **Unwind cleanup covers secret buffers but not the operation.** `SystemAccount`
   catches a manager panic, wipes passphrase/import/password buffers, and resumes the
   unwind without rolling back the active child or artifacts. If the caller catches the
   resumed panic and retains the account, a later `begin_attempt` discards that ownership
   ledger. Track whether an attempt is active and invoke the same exact teardown and
   artifact reconciliation before `resume_unwind`; latch unavailable if it cannot be
   proven. Secret wiping remains mandatory even if rollback itself fails.

6. **Vault/state primary identity is still not validated before child start.** Open
   compares kind, account, network, and height before the child, but defers sealed-versus-
   stored primary comparison and UTF-8/address validation until after wallet RPC.
   `StoredIdentity::validate` checks only byte length, so malformed primary bytes can
   reach child start. Decode and syntactically validate both primaries, require them to
   match before any child/recovery work, and retain the independent closed RPC network
   validation afterward.

7. **The process-created directory boundary still omits owner validation.** Grok
   correctly disclosed this residual. `process.rs::ensure_private_root` and
   `create_private_directory` validate type/mode but not effective owner, while account
   preflight does not cover existing runtime and ring directories. A wrong-owner `0700`
   runtime/ring component can therefore pass after the account gate. Validate exact
   owner and no-follow identity for every existing process-derived directory before
   config creation or child start. Never chmod/replace a hostile entry.

8. **Directory durability and identity are incomplete.** State `sync_directory` opens a
   no-follow directory but discards its device/inode instead of comparing it to a
   retained validated account-directory identity. Newly created vault/XMR/network/
   account directories are not durably linked by synchronizing each containing parent;
   syncing only the new leaf does not make the parent entry crash-durable. Retain the
   exact account directory capability for state sync, and sync each parent after a new
   private directory is created and validated.

The retained no-follow state-file handle plus `SQLITE_OPEN_NOFOLLOW` and before/after
device/inode validation is accepted as the safest available rusqlite 0.37 boundary
under repository-wide `forbid(unsafe_code)`. Correction 04 must not weaken it or invent
an unsafe from-fd path.

## Review decision

Correction 03 again makes material progress and its remaining defects are bounded.
Grok 4.6 High remains the sole source actor for a focused Correction 04; Sol is not
authorized. Hermes execution/integration, formatting, compilation, Slice 5, broader
acceptance, and the real offline local-Monero gate remain unauthorized.
