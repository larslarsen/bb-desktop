# Grok Build Handoff — BBD-WAL-007 Phase-C Slice 4 Correction 05

You are **Sr Dev — Grok Build using Grok 4.6 High**. This durable handoff is
authoritative. Do not delegate this source work to Sol.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff; the rejected
Correction-04 source drop remains unstaged above that parent.

Read completely before editing: `AGENTS.md`, `TESTING.md`, the development-role policy,
`tickets/BBD-WAL-007.md`, Test-Source Review 04, Expected Red 02, Slice-3 Acceptance 01,
the Slice-3 Upstream RPC Decision, all five Slice-4 source reviews, all earlier Slice-4
handoffs, the current seven source files, the frozen account test, and
`docs/handoff/CURRENT_TASK.md`.

## Sole task and exact starting identities

Correct the rejected Slice-4 account-custody/recovery drop. Edit only:

- `wallet-broker/src/vault.rs`;
- `wallet-broker/src/xmr.rs`;
- `wallet-broker/src/xmr/account.rs`;
- `wallet-broker/src/xmr/store.rs`;
- `wallet-broker/src/xmr/process.rs`;
- `wallet-broker/src/xmr/rpc.rs`; and
- `wallet-broker/src/xmr/test_support.rs`.

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/vault.rs` | 776 | `bb79b16f24a9abbcd266e856a37ee982bde8fcdc2da3858e75d34bdd34a4af86` |
| `wallet-broker/src/xmr.rs` | 7 | `8b382ef673e82001720727c161f365c0fa3c1401d1bdaa4d20ee2683c001559b` |
| `wallet-broker/src/xmr/account.rs` | 3,020 | `07a4e815082a6ca58c4d1d8b5c8f330f62391c89d4002c75a276750b71bc6455` |
| `wallet-broker/src/xmr/store.rs` | 1,320 | `412b1a7b92b07c0d39963e19565c467968e47ca2a02d74b36bf19369787c1b1b` |
| `wallet-broker/src/xmr/process.rs` | 1,808 | `b990de3e80db0a4d354ec6119fbc746b27a8989909e702b63270b6d5b43fd52a` |
| `wallet-broker/src/xmr/rpc.rs` | 2,418 | `4f52baf1532a374dda748ec03a4a0a4311fd970b441f258f576c26030bb5ac14` |
| `wallet-broker/src/xmr/test_support.rs` | 3,924 | `5695a67aac219f36e5cd4df156f0708843084c9befb8e396f641c7c3348f966e` |

If any identity/scope differs, stop without editing. Preserve the frozen 586-line
`wallet-broker/tests/xmr_account.rs` at SHA-256
`5e8afabaa8c820be84a010a88fef0e07617e67934c06b32ab445901b1ddad35b` and
every other test, manifest, lockfile, source outside these paths, policy, workflow,
documentation, and fixture byte-exact.

Do not run Cargo, rustfmt, tests, Clippy, Node/npm, policy/security commands, builds,
binaries, package managers, network, Git, GitHub, staging, commit, push, or evidence.
Use only direct source inspection plus `wc`/`sha256sum`. Leave the drop unstaged.

## 1. Retire an attempt exactly once, only at returned success

Remove the early `commit_attempt` from `seal_and_persist`. Vault, state, wallet, and
keys ownership must remain live through all final cleanup and until the single
successful manager boundary retires the ledger and clears `attempt_active`. An unwind
at any earlier instruction must still run the complete teardown and exact artifact
rollback. Preserve ordinary retry after fully proven cleanup and the unavailable latch
after uncertain cleanup.

On `lock`, run the same fail-closed lifecycle policy: always wipe the wallet password,
clear process retention, and require authenticated vault access again. If exact teardown
returns an error, set the manager unavailable and return `INTERNAL`; no later account
operation may run. Do not report a successful close/stop observation when cleanup was
not established. Apply the same latching principle to any production-reachable helper
that currently ignores an uncertain owned-child teardown result; test-only simulation
must not create an alternate production authority.

## 2. Inspect both wallet artifacts on every RPC outcome

Refactor `capture_wallet_artifacts` so wallet and keys inspections always both execute,
even when the first fails. Publish each exact identity that can be proven. Combine the
two outcomes only afterward. Preserve the RPC result when RPC and exact capture both
fail, but mark attempt uncertainty so rollback compounds to `INTERNAL`; on RPC success,
still require the complete revalidated owner-`0600` pair and unchanged private wallet
directory. Never assign ownership to an existing-wallet open.

## 3. Close the exclusive-create identity window

For both active vault and `state.sqlite`, distinguish namespace creation from identity
derivation. The instant `create_new` succeeds, record provisional uncertainty in the
current attempt before any fallible metadata, type, mode, owner, device/inode, write,
SQLite, file-sync, or directory-sync operation. Once the exact handle identity is
validated and published to the appropriate ledger entry, clear only that provisional
uncertainty. If identity derivation fails, rollback must fail compound and latch the
account unavailable rather than silently leaving a reusable account with an untracked
path. `AlreadyExists` and pre-create failures must never record ownership or adopt the
path.

Use the smallest crate-internal handle/identity API needed to enforce that ordering.
Remove the unused combined `PathSqliteSurface::create_new` constructor; it both creates
and binds before the account attempt can publish ownership and must not remain as a
second callable path. Retain the accepted state handle, account-directory handle,
`SQLITE_OPEN_NOFOLLOW`, exact before/after identity comparisons, FULL sync, strict
reopen, and exact schema proof. Do not introduce unsafe code, `/proc/self/fd`, a new
dependency, or a weaker path-only SQLite claim.

## 4. Make every quarantine destination unlink identity-bound

Keep the hard-link/no-replace design, but treat its destination as an exact temporary
artifact. Never call `remove_file(destination)` merely because some entry exists. If
destination open fails and no exact identity is available, do not guess: return cleanup
uncertainty and latch the account unavailable. If a destination handle was opened,
derive its device/inode, then immediately before any destination unlink use a fresh
no-follow metadata read and require exact device/inode equality with that handle.

On a source/destination mismatch, remove only the exact newly created destination link
whose identity is proven, then synchronize the validated parent. If its identity cannot
be proven or unlink/sync fails, retain compound `INTERNAL`. Never touch a substituted
source. On normal quarantine, revalidate the source identity immediately before source
unlink and synchronize the parent after the final namespace change. Do not hide any
material cleanup result behind `let _ =`.

## 5. Finish zeroization and warning hygiene

In wallet-password generation, explicitly zeroize the entropy array when `fill_entropy`
returns an error after a partial write as well as on success. Keep all later encoded
forms zeroizing and redacted.

Remove the useless Rust comparison of an `i64` stored issuance sequence to `i64::MAX`
and any now-unused private constant without a lint suppression. Preserve the SQL
constraint that rejects SQLite values outside the signed-64 domain and do not alter the
future public Slice-5 receiver limit. Remove the unused combined state constructor as
required above. Directly inspect all changed declarations and cfg branches for warning-
level dead code, unused bindings, and mechanical completeness, but do not run a
formatter or compiler.

Preserve every valid Correction-04 correction: crate-internal authority, exact pinned
RPC members and phases, one live address-validation call, strict preflight ordering,
primary binding before child start, mandatory complete wallet pair on success, unwind
reconciliation, exact per-attempt identities, strict state reopen, retained SQLite and
directory capabilities, exact schema verification, owner/type/mode validation,
parent-directory durability, zeroizing recording custody, and the disclosed same-user
path/SQLite residual. Do not begin Slice 5/viewing/receiver issuance, local-Monero
execution, Electron/UI wiring, or BBD-WAL-008/009.

Return exact changed paths, final line counts and SHA-256 values, a blocker-to-correction
map, and all residual concerns. XHigh source review precedes any Hermes formatter,
compiler, test, integration, Git, or evidence work.
