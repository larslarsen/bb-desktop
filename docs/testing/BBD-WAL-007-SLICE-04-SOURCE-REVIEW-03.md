# BBD-WAL-007 Slice-4 Source Review 03

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Governance parent: `0b0e364f`

Result: **REJECTED — ATTEMPT ISOLATION, STRICT REOPEN, AND SECRET-CUSTODY CORRECTION REQUIRED**

Reviewed unstaged Grok 4.6 High Correction-02 drop:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/vault.rs` | 776 | `bb79b16f24a9abbcd266e856a37ee982bde8fcdc2da3858e75d34bdd34a4af86` |
| `wallet-broker/src/xmr.rs` | 7 | `9bdcef746110c164726a7baafeb7d7d123c24d12b96f9187ca0a21f965e36590` |
| `wallet-broker/src/xmr/account.rs` | 2,509 | `cb51f286f90d775381675454e6b0c34436bd90e122179514d0ce568c9f69f104` |
| `wallet-broker/src/xmr/store.rs` | 1,097 | `09ac04aa36c9282b35ea501dcc2f43c86cece4c72d8fa2e093c733338ee09f4b` |
| `wallet-broker/src/xmr/process.rs` | 1,750 | `8e2908efa115e870984d2bab2743ecc788c179d85564501ef4f9bed693f4f20e` |
| `wallet-broker/src/xmr/rpc.rs` | 2,418 | `e0f59a68266941287281d66d886e4d60147c0b968f7eda85fcf953dd88c2712e` |
| `wallet-broker/src/xmr/test_support.rs` | 3,826 | `23b8e84d1017421eb38ad74f99d9972cf47c6ae1a1050ba8f31eb49f8aa7d90c` |

`HEAD == origin/master == 0b0e364f`, the index is clean, these are the only seven
worktree paths, and `git diff --check` is clean. The frozen 586-line
`wallet-broker/tests/xmr_account.rs` remains byte-exact at SHA-256
`5e8afabaa8c820be84a010a88fef0e07617e67934c06b32ab445901b1ddad35b`.
The reviewer ran no formatter, compiler, test, build, binary, Node/npm,
policy/security, package-manager, staging, commit, or push command.

Correction 02 materially closes the pinned request-member, phase-bound mnemonic,
effective-UID, non-Linux gate, honest identity-observation, WAL-004 metadata,
path-backed sync/reopen, and fake decrypt/fabricated-wipe findings. It also narrows the
new pool methods and removes live secret-bearing call history. Those corrections must
be preserved. The drop is not safe to compile or integrate yet.

## Blocking findings

1. **Rollback ownership leaks across operations and compound failure does not make the
   account unavailable.** `generated_wallet`, `created_vault`, and `created_state` are
   manager-lifetime booleans. They are set during a successful create/import and never
   retired or reset at the beginning of a later attempt. A later open failure therefore
   treats valid prior artifacts as newly created and may quarantine them. The same stale
   state survives rollback. `unavailable` is set when cleanup fails but is only an
   observer field; create/import/open can still run afterward. Artifact ownership and
   observation state must be per-attempt, successful artifacts must be retired from the
   rollback set, and every later operation must fail closed after compound cleanup.

2. **Recovery cleanup destroys pre-existing durable state.** Missing-wallet recovery
   sets `generated_wallet`, but `remove_or_quarantine` always processes wallet, keys,
   and `state.sqlite`. A failed recovery RPC or identity comparison can therefore move
   the valid state DB that authenticated the recovery even though this attempt did not
   create it. Vault, state, wallet, and keys need separate exact artifact ownership;
   cleanup may touch only artifacts produced or uncertainly committed by that attempt.
   The recording port must model the same distinction instead of only changing wallet
   presence.

3. **Open-existing still has create side effects, and storage validation is ordered too
   late.** `open_vault` calls `ensure_vault_store`, whose `initialize` can create the
   vault directory. `open_store_existing` calls `ensure_layout`, which can create the
   XMR namespace, network, account, and wallet directories before a missing state DB is
   rejected. Create/import also probe the node before `create_wallet_files` performs
   derived path/artifact checks. Open decrypts the vault before validating all existing
   XMR components. A read-only preflight must validate root, every existing derived
   component, final vault/state/wallet/keys entries, and required state/vault presence
   before vault initialization, decryption, node probing, child start, SQLite, or RPC.
   Creation helpers may run only after that gate; recovery may create only its missing
   wallet layout after authenticated vault and state validation.

4. **Active-vault creation is not exclusive and cleanup is not identity-bound.**
   `seal_vault` checks `path_exists` and then calls WAL-004 `write_active`, whose atomic
   replacement can overwrite an entry created after the check. The quarantine helper
   reserves a random destination with `create_new`, deletes that reservation, and then
   uses overwriting `rename`, reopening a collision window. It also validates the
   source by pathname without proving that the moved inode is the artifact created by
   this attempt. Creation needs an exclusive no-replace active-vault result. Cleanup
   must retain and compare exact artifact identity, use an atomic no-replace link/rename
   strategy, never move a raced or pre-existing entry, and synchronize the containing
   directory after the final namespace operation.

5. **SQLite still has path-reopen and directory-follow gaps.** `open_sqlite` validates
   a no-follow file descriptor, drops it, and only then opens SQLite by pathname. The
   later pathname metadata comparison does not bind SQLite's connection to that
   descriptor. `sync_directory` uses following `File::open` after a separate
   validation. Retain and revalidate the opened capability across SQLite open using the
   safest available dependency/OS boundary, use no-follow directory handles for sync,
   and compare device/inode at each retained boundary. Remove the duplicate Linux cfg.

6. **The schema verifier is not exact.** It accepts any index whose target table is one
   of the two reviewed tables, permits additional unique sets because it uses subset
   comparison, and deliberately skips the declared `NOT NULL` check for integer primary
   keys. SQL substring checks also permit extra unreviewed constraints. Prove the exact
   two-table schema, exact columns/types/nullability/key positions and constraints,
   exactly the four receiver uniqueness sets, and only SQLite's expected implicit
   objects. Any addition or omission is `STATE_CORRUPT`.

7. **Operation-scoped secret cleanup is not unwind-safe.** Native watch import copies
   primary and view key into ordinary `String`s. The installed passphrase and those
   input copies are wiped only after a normal manager return; a caught unwind can leave
   the passphrase in the live port and ordinary allocations unwiped. Use zeroizing
   import text and an operation guard/catch-and-resume boundary that performs immediate
   passphrase, import, and wallet-password cleanup on success, error, cancellation,
   replacement, and unwind. The system port must never expose a path that bypasses that
   boundary.

8. **New internal account authority remains publicly callable.** The public
   `AccountPort`/`AccountManager` path can bypass `SystemAccount`'s broker-native secret
   boundary. `XmrSecretV1` exposes public password/mnemonic/view-key copy and expose
   methods, and the public wallet-password generator/observer returns the generated
   password. `SystemAccountPort`, store surfaces, and the concrete system RPC control
   are also broader than their required composition. Keep the intended sanitized
   `SystemAccount`/account result and frozen test-facing surface public, but make the
   generic ports, managers, codecs, production store implementation, secret helpers,
   and concrete account RPC control crate-internal unless a frozen imported type
   requires otherwise.

9. **The exact live RPC/teardown sequence still duplicates calls.** The concrete RPC
   control's `get_primary_address` already performs the closed `validate_address`, and
   `SystemAccountPort::get_primary_address` immediately validates the same address a
   second time. After creation/import explicitly closes the wallet, a later rollback or
   lock calls process teardown, which unconditionally sends another close even though
   its phase is `NoWallet`. Keep one address validation and have owned teardown skip
   close only when the tracked session is already closed, while still always performing
   stop, wait-or-kill, reap, socket/runtime-secret cleanup, and credential wiping.

10. **The recording adapter still retains unprotected secret copies.**
    `XmrSecretFixture::mutated_encoding` converts its zeroizing frame to an ordinary
    `Vec<u8>`. `from_secret` creates an ordinary payload `String` before wrapping it.
    `RecordingAccountPort` stores primary and mnemonic in ordinary `String`s and replaces
    them without zeroizing. Make returned frame bytes and every retained/intermediate
    password, primary, mnemonic, and view-key allocation zeroizing, with redacted
    formatting and guaranteed drop. Preserve authenticated `sealed_record` decryption
    and real nonempty wipe evidence.

## Review decision

The defects remain bounded to the same seven paths, and Correction 02 demonstrates
substantial progress. Grok 4.6 High remains the sole source actor for Correction 03;
there is no basis to invoke the Sol fill-in. Hermes execution/integration, formatting,
compilation, Slice 5, broader acceptance, and the real offline local-Monero gate remain
unauthorized.
