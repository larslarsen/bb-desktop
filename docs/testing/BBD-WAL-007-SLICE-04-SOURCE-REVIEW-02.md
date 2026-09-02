# BBD-WAL-007 Slice-4 Source Review 02

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Governance parent: `8b55a052`

Result: **REJECTED — LIVE CUSTODY, DURABILITY, AND PINNED-WIRE CORRECTION REQUIRED**

Reviewed unstaged Grok 4.6 High Correction-01 drop:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/vault.rs` | 776 | `bb79b16f24a9abbcd266e856a37ee982bde8fcdc2da3858e75d34bdd34a4af86` |
| `wallet-broker/src/xmr.rs` | 7 | `9bdcef746110c164726a7baafeb7d7d123c24d12b96f9187ca0a21f965e36590` |
| `wallet-broker/src/xmr/account.rs` | 2,254 | `b7f235791bfed485c92ee7dde7906d87bdf2df28f9c7faf207776609a2be3c64` |
| `wallet-broker/src/xmr/store.rs` | 702 | `11a279af1770e607cd42ad8e1680956db3db01f165f9ec04e1105754d59d860c` |
| `wallet-broker/src/xmr/process.rs` | 1,662 | `a1d6b38d1056685b1833a4612a74f29f08e5fd0ac957fe6f885126156e4a6608` |
| `wallet-broker/src/xmr/rpc.rs` | 2,324 | `7b07e261330a0b1c5b905c42b64e297f1a628420fdb073ea3312048d74b38304` |
| `wallet-broker/src/xmr/test_support.rs` | 3,772 | `004413c04179a94ba7cb84ad2219d51301830faa9ec3fde6432fef47f35abb42` |

`HEAD == origin/master == 8b55a052`, the index is clean, these are the only seven
worktree paths, and `git diff --check` is clean. The reviewer ran no formatter,
compiler, test, build, binary, Node/npm, policy/security, package-manager, staging,
commit, or push command. The pinned request shapes were checked against the exact
v0.18.5.1 source already fixed by the Slice-3 Upstream RPC Decision.

Correction 01 materially improves the rejected drop: it adds a real system composition,
closed request variants, a process bridge, WAL-004 encryption/storage, XMR testnet vault
metadata, and a path-backed SQLite surface. It is not safe to compile or integrate yet.

## Blocking findings

1. **The live existing-wallet open path deterministically rejects every nonzero restore
   height.** `SystemAccountPort::get_primary_address` reports `restore_height: 0`, while
   `verify_open_identity` requires that value to equal the sealed height. More broadly,
   the system observation fabricates network, kind, filename, and watch-only fields from
   requested state instead of returning independent RPC facts. The accepted RPC set
   cannot report a wallet restore height or kind after `open_wallet`. The correct live
   proof is: sealed metadata and the durable identity agree on account/network/kind/
   restore height; the exact derived filename and exact network-bound owned child are
   used; and an independently parsed, network-validated RPC primary equals the sealed
   and durable primary. Recovery additionally binds kind and height through the exact
   closed restore/generate request. Test faults must mutate those real evidence sources,
   not justify fabricated production observations.

2. **The new account requests are not compatible with the pinned v0.18.5.1 wire
   schema.** `get_address` omits its serialized `address_index` vector;
   `generate_from_keys` omits the serialized `spendkey` and `language` members; and
   `restore_deterministic_wallet` omits `seed_offset`. The safe fixed values are an
   empty address-index vector, an empty spend key, an empty watch-import language, and
   an empty seed offset. The high-level frozen account observer must continue to omit
   `spendkey` and the other internal fixed-safe members. The process/RPC bridge also
   permits mnemonic query repeatedly and outside fresh-software creation, contrary to
   the phase-bound authority contract.

3. **The live effective-user check is wrong and the new modules break the platform
   boundary.** `current_uid` calls `symlink_metadata("/proc/self")`, which reads the
   `/proc/self` symlink owner rather than the followed process-directory owner. On the
   review host those values are UID 65534 and UID 1000 respectively, so the live port
   rejects the broker's own files. `account.rs` and `store.rs` also import Unix APIs
   unconditionally and perform Linux work unconditionally. Non-Linux builds must
   compile and return `UNAVAILABLE` before filesystem/process effects.

4. **Hostile paths can be repaired or followed before rejection.**
   `ensure_private_directory` chmods an already-existing wrong-mode directory instead
   of rejecting it. The wallet inspection checks only the final wallet directory/files
   and can follow hostile intermediate components. Vault initialization happens before
   a strict root/component preflight. `Connection::open(path)` follows a final path
   after a separate metadata check, leaving a no-follow time-of-check/time-of-use gap.
   Existing wrong-owner/mode, symlink, non-directory, non-regular, and substituted
   components must never be replaced, chmodded, or followed.

5. **Creation and open use the same create-capable SQLite path.** A missing state DB on
   open is created rather than rejected, while an existing empty/version-zero or corrupt
   DB can be treated as an initialization candidate. A failed `PRAGMA user_version`
   query is converted to zero with `unwrap_or(0)`. Creation does not require state and
   vault absence, so it can overwrite an existing vault before a duplicate state insert
   fails. Production needs distinct create-new and open-existing flows with exclusive
   creation, no-follow opened-file validation, and no reconstruction on open.

6. **Durability success and rollback are in-memory claims, not reconciled artifacts.**
   The path store never exercises its `reopen` method after sync and never revalidates
   the exact durable identity before success. File/directory sync failure occurs after
   the SQLite identity commit, yet rollback leaves `state.sqlite` in place and merely
   clears `state_committed`. A vault directory-sync failure can leave the active
   envelope installed while `vault_committed` remains false, so rollback skips it.
   Recovery-created files are not marked generated and remain after an open mismatch.
   Every uncertain vault/state/wallet result must inspect and remove or uniquely
   quarantine the actual new artifact, synchronize its parent directory, and keep the
   account unavailable on cleanup failure.

7. **The quarantine implementation is unsafe and non-durable.** Both `account_id` and
   `account_id.keys` map to `account_id.quarantine`, so wallet and keys collide. Rename
   can overwrite an existing destination, its result is not revalidated, and neither
   rename nor fallback deletion synchronizes the parent. The same helper is used for
   vault cleanup. Cleanup must use noncolliding exclusive destinations or validated
   removal, never replace a hostile/existing entry, and fsync the containing directory.

8. **Vault authentication is not bound back to the requested identity.** The system
   path decrypts any valid WAL-004 envelope found at the filename without comparing the
   parsed authenticated `VaultMetadata` to the expected account ID, XMR asset, selected
   network, and epoch. It also maps every store read failure to `UNAUTH`. Parse and
   compare metadata before plaintext use, preserve stable distinctions without leaking
   details, and preflight that a create cannot replace an existing active envelope.

9. **Secrets outlive their authorized scope.** `SystemAccountPort` retains the native
   vault passphrase for its whole lifetime and retains every password, mnemonic/view
   key, and primary copy in `calls`. `last_password` also survives a successful open
   until lock even though RPC no longer needs it. `SystemAccount::import_watch_only`
   accepts borrowed strings that it cannot wipe. The public process pool exposes
   secret-bearing account RPC methods, including reusable mnemonic query. Production
   must use operation-scoped `SecretBytes`/`Zeroizing` custody, never record live secret
   calls, wipe caller/import temporaries on every exit, and expose only the minimum
   closed broker-native account boundary.

10. **Teardown can duplicate RPC and can report a non-compound failure.** Account lock
    calls live `close_wallet` and `stop_wallet`, then `stop_account`, whose accepted
    teardown calls close and stop again. Early `?` exits on either first RPC skip reap
    and password wiping. Open failure ignores all cleanup results. A single exact-owned
    lifecycle must perform close/stop/wait-or-kill/reap once, always wipe secrets, and
    return compound `INTERNAL` when required cleanup cannot be established.

11. **The claimed exact SQLite schema check is partial.** It compares column names and
    searches for only selected SQL substrings. It does not prove column types,
    nullability/primary-key facts, all restore/sequence/subaddress bounds, all four
    independent unique constraints, or absence of extra schema objects. The loaded row
    is not independently revalidated as the complete closed identity. This does not
    meet the correction's exact schema/constraint/reopen requirement.

12. **The test adapter is not an honest oracle.** `sealed_record` passes encrypted
    WAL-004 envelope bytes directly to `XmrSecretV1::decode`, so the frozen sealed/open
    tests cannot succeed through the claimed vault path. `wipe_wallet_password`
    fabricates a successful 64-byte wipe event when no password exists.
    `XmrSecretFixture` and other retained fixture strings lack zeroizing drop custody.
    The adapter must decrypt/authenticate through the same vault primitive, report only
    real wipe events, and make every retained test-only secret copy redacted and
    zeroizing.

## Review decision

The defects are bounded to the same seven source paths and do not establish that Grok
is incapable of the correction. Grok 4.6 High remains the sole source actor under
Correction 02; Sol is not authorized. Grok's Correction-01 run also used read-only Git
baseline/status inspection despite the explicit no-Git source-actor rule. It caused no
repository mutation, but Correction 02 records the deviation and repeats the ban.

Hermes execution/integration, formatting, compilation, Slice 5, broader acceptance, and
the real offline local-Monero gate remain unauthorized.
