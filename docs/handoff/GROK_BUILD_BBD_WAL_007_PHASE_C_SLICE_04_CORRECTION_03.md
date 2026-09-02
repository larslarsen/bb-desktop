# Grok Build Handoff — BBD-WAL-007 Phase-C Slice 4 Correction 03

You are **Sr Dev — Grok Build using Grok 4.6 High**. This durable handoff is
authoritative. Do not delegate this source work to Sol.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff; the rejected
Correction-02 source drop remains unstaged above that parent.

Read completely before editing: `AGENTS.md`, `TESTING.md`, the development-role policy,
`tickets/BBD-WAL-007.md`, Test-Source Review 04, Expected Red 02, Slice-3 Acceptance 01,
the Slice-3 Upstream RPC Decision, all three Slice-4 source reviews, all prior Slice-4
handoffs, the complete accepted XMR and WAL-004 vault/store/process/RPC source,
`wallet-broker/tests/xmr_account.rs`, and `docs/handoff/CURRENT_TASK.md`.

## Sole task and exact source identities

Correct the rejected Slice-4 account-custody/recovery drop. Edit only:

- `wallet-broker/src/vault.rs`;
- `wallet-broker/src/xmr.rs`;
- `wallet-broker/src/xmr/account.rs`;
- `wallet-broker/src/xmr/store.rs`;
- `wallet-broker/src/xmr/process.rs`;
- `wallet-broker/src/xmr/rpc.rs`; and
- `wallet-broker/src/xmr/test_support.rs`.

Require these exact starting identities:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/vault.rs` | 776 | `bb79b16f24a9abbcd266e856a37ee982bde8fcdc2da3858e75d34bdd34a4af86` |
| `wallet-broker/src/xmr.rs` | 7 | `9bdcef746110c164726a7baafeb7d7d123c24d12b96f9187ca0a21f965e36590` |
| `wallet-broker/src/xmr/account.rs` | 2,509 | `cb51f286f90d775381675454e6b0c34436bd90e122179514d0ce568c9f69f104` |
| `wallet-broker/src/xmr/store.rs` | 1,097 | `09ac04aa36c9282b35ea501dcc2f43c86cece4c72d8fa2e093c733338ee09f4b` |
| `wallet-broker/src/xmr/process.rs` | 1,750 | `8e2908efa115e870984d2bab2743ecc788c179d85564501ef4f9bed693f4f20e` |
| `wallet-broker/src/xmr/rpc.rs` | 2,418 | `e0f59a68266941287281d66d886e4d60147c0b968f7eda85fcf953dd88c2712e` |
| `wallet-broker/src/xmr/test_support.rs` | 3,826 | `23b8e84d1017421eb38ad74f99d9972cf47c6ae1a1050ba8f31eb49f8aa7d90c` |

If any identity or scope differs, stop without editing. Preserve every test, manifest,
lockfile, source outside these seven paths, policy, workflow, documentation, fixture,
and other repository byte-exact.

Do not run Cargo, rustfmt, tests, Clippy, Node/npm, policy/security commands, builds,
binaries, package managers, network, Git, GitHub, staging, commit, push, or evidence.
Read source and report line/hash results without Git. Leave the corrected drop unstaged
for XHigh review.

## 1. Make every operation an isolated fail-closed attempt

Replace manager-lifetime cleanup booleans with an exact per-attempt artifact ledger for
active vault, state DB, wallet, and keys. Start create/import/open with fresh observation
and rollback state. Retire successfully committed artifacts from rollback ownership
before returning. A later failure must never touch artifacts committed by an earlier
successful operation. Clear the current attempt after successful or completed failed
cleanup.

If cleanup cannot prove reconciliation, latch the account unavailable. Every later
create/import/open/lock-sensitive operation fails closed before path, node, child,
vault, SQLite, or RPC work. Do not make `unavailable` an observer-only flag.

Distinguish creation from missing-wallet recovery. Recovery owns only wallet/keys files
created or uncertainly created by its restore/generate call. It never owns or removes
the authenticated pre-existing vault or state DB. An existing-wallet open owns no
durable artifact. Adapt the recording port so its cleanup and fault model exercise
these same production-core distinctions.

## 2. Put a read-only storage gate before every effect

Before vault initialization or plaintext use, node probing, directory creation, child
start, SQLite, or wallet RPC, perform one read-only Linux preflight of the absolute root
and all existing derived vault/XMR/network/account/wallet components. Reject every
symlink, non-directory traversal, non-regular final entry, wrong owner/mode, partial
wallet set, cross-network/account substitution, and raced identity. Validate final
regular entries through no-follow handles and device/inode comparison.

Create-new requires active vault, state, wallet, and keys absence at that gate. Only
afterward may it create missing private directories. Open-existing requires the active
vault and state DB to exist and be valid and must not call any initialize/ensure helper
that can create them or their parent hierarchy. Authenticate metadata and load/validate
the exact durable identity before starting a child. Missing wallet files alone may
enter recovery and create only the missing wallet layout after that authentication
gate.

Split vault construction into create and strict-existing paths: strict open constructs
the existing store without `initialize`. Split state layout similarly: strict state
open validates existing account layout and never calls `ensure_layout`. Keep the
non-Linux system entry compile-safe and effect-free `UNAVAILABLE`.

## 3. Use exclusive active creation and exact no-replace cleanup

The active vault is a create-new artifact. Eliminate `exists` followed by an overwriting
replace. Use an exclusive/no-replace namespace operation for the final active envelope;
never overwrite an entry that wins a race. If the accepted WAL-004 store cannot provide
this without editing closed source, implement the smallest seven-path adapter/helper
that writes the exact existing envelope semantics safely, or stop and report the exact
missing boundary rather than weakening the rule.

Record exact device/inode identity for each newly created or uncertainly committed
artifact. On failure, first prove the path still names that artifact. Quarantine with an
atomic no-replace operation, not create-delete-then-overwriting-rename. A hard-link plus
validated unlink or an available no-replace rename is acceptable when it proves the
same inode. Never move/remove a pre-existing, substituted, or raced file. Revalidate
the result and sync the containing directory after the final rename/link/unlink. Any
uncertainty compounds to `INTERNAL` and latches the account unavailable.

## 4. Close the SQLite capability and schema boundary

Retain the no-follow validated state-file handle across SQLite open and revalidate the
same device/inode before and after using the safest capability available in the current
dependencies. Do not drop the capability and then trust a fresh path open. Open the
containing directory with Linux no-follow/directory flags for synchronization, validate
its owner/mode/type/device/inode, and sync that handle. Remove the duplicate cfg.

Verify exactly schema version 1, the exact two tables, exact ordered columns, declared
types, nullability and primary-key positions, every required integer/domain constraint,
and exactly the four independent receiver unique sets. Permit only the deterministic
SQLite implicit objects required by that schema; reject every extra index, unique set,
trigger, view, table, or constraint. Independently validate the complete loaded row.
Keep real commit, file sync, directory sync, strict reopen, repeated schema validation,
and exact identity comparison before durable success.

## 5. Make native and recording secrets scope-bound

Use zeroizing text for native watch primary/view-key conversion. Wrap every
`SystemAccount` secret-bearing operation in a guard or catch-cleanup-resume boundary so
the installed passphrase, input copies, and remembered wallet password are wiped
immediately on success, error, cancellation/drop, replacement, and unwind, even if the
caller catches the panic and retains the account. Do not retain the vault passphrase or
wallet password beyond that operation. Preserve live no-call-history behavior.

Narrow new authority to the intended broker-native `SystemAccount` and sanitized
result. Generic account ports/managers, the XMR secret codec and its password/mnemonic/
view-key expose/copy helpers, production state surfaces, password generation helpers,
`SystemAccountPort`, and the concrete system account RPC control are crate-internal
unless a frozen integration-test import requires a public wrapper. In particular, no
public path may bypass `SystemAccount` and operate on live secret material. Do not
disturb accepted public constants or frozen test-support imports.

In test support, keep `XmrSecretFixture`, the recording port's primary/mnemonic/view-key
state, all intermediate payload text, and returned mutated secret-frame bytes
zeroizing. Preserve redacted formatting, authenticated WAL-004 decrypt-before-decode,
and wipe observations from real nonempty instances. On state cleanup, reset/remove the
recording store only when that attempt owned it.

## 6. Emit each closed RPC/lifecycle action once

Keep the exact pinned request members and both account/process phase machines. Perform
the closed `validate_address` exactly once per retrieved primary; remove the second
system-layer validation. Track the wallet-open phase through explicit close. Owned
teardown skips `close_wallet` only when the same session is already known closed, but
always stops, waits or kills, reaps, closes sockets, removes runtime secrets, and wipes
credentials. Early errors cannot skip later cleanup, and cleanup uncertainty compounds
to `INTERNAL`.

Preserve all valid Correction-02 work: fixed empty `address_index`, `spendkey`,
`language`, and `seed_offset`; immediate one-shot mnemonic authority; honest RPC facts;
followed effective UID; Linux gates; metadata comparison; strict identity matching;
real path-store sync/reopen; redacted zeroizing RPC buffers; and no live observer call
history. Do not begin Slice 5/viewing/receiver issuance, local-Monero execution,
Electron/UI wiring, or BBD-WAL-008/009.

Return the exact changed paths, resulting line counts and SHA-256 values, a concise map
of each Source Review 03 blocker to its correction, and every residual concern. The
reviewer will inspect source before Hermes may format, compile, test, integrate, or use
Git.
