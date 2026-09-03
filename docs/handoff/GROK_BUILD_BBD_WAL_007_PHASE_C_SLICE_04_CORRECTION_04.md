# Grok Build Handoff — BBD-WAL-007 Phase-C Slice 4 Correction 04

You are **Sr Dev — Grok Build using Grok 4.6 High**. This durable handoff is
authoritative. Do not delegate this source work to Sol.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff; the rejected
Correction-03 source drop remains unstaged above that parent.

Read completely before editing: `AGENTS.md`, `TESTING.md`, the development-role policy,
`tickets/BBD-WAL-007.md`, the accepted account test-source/expected-red records, Slice-3
Acceptance 01 and Upstream RPC Decision, all four Slice-4 source reviews, all earlier
Slice-4 handoffs, the current seven source files, the frozen account test, and
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
| `wallet-broker/src/xmr/account.rs` | 2,849 | `76840451ec7c87d7ed5f849f6b700b166b2a9c8cbead5163bea7f01fe25e8bc5` |
| `wallet-broker/src/xmr/store.rs` | 1,195 | `a4d89f8555c9f14626e5c9f989e636ed22eab27d713e62015a1f76f7bd97aaa5` |
| `wallet-broker/src/xmr/process.rs` | 1,752 | `b0ef7445fd8c3428f860b2656f83537cbe4ee5d97101329b2027af091eab3213` |
| `wallet-broker/src/xmr/rpc.rs` | 2,418 | `4f52baf1532a374dda748ec03a4a0a4311fd970b441f258f576c26030bb5ac14` |
| `wallet-broker/src/xmr/test_support.rs` | 3,924 | `c308efb25501e778f9b4a69664c88ec2e287478bedf64dc3923611ced8bc815e` |

If any identity/scope differs, stop without editing. Preserve the frozen 586-line
`wallet-broker/tests/xmr_account.rs` at SHA-256
`5e8afabaa8c820be84a010a88fef0e07617e67934c06b32ab445901b1ddad35b` and
every other test, manifest, lockfile, source outside these paths, policy, workflow,
documentation, and fixture byte-exact.

Do not run Cargo, rustfmt, tests, Clippy, Node/npm, policy/security commands, builds,
binaries, package managers, network, Git, GitHub, staging, commit, push, or evidence.
Use only direct source inspection plus `wc`/`sha256sum`. Leave the drop unstaged.

## 1. Restore a parseable, closed account core

Add the missing opening brace to `AccountManager` and inspect every changed declaration
and impl boundary for the same mechanical completeness. Preserve the Correction-03
crate-internal visibility, exact pinned RPC members/phases, schema proof, honest identity
facts, Linux/non-Linux gates, and zeroizing recording changes. Do not broaden authority.

## 2. Publish ownership at exclusive creation, not after durable success

For create-new active vault storage, prefer a direct `OpenOptions::create_new` active
file: there is no prior active value to replace. Validate its no-follow handle and
publish device/inode ownership to the current attempt immediately after exclusive
creation, before write, file sync, or directory sync. Then write the exact WAL-004
envelope, sync the same file, sync the exact vault directory, and report success. Every
later failure is now recoverable through the published ledger. If staging remains, it
needs its own immediate identity/guard and exact cleanup on every exit; no orphan stage
is permitted. Never overwrite an active entry.

Apply the same principle to `state.sqlite`. The exact identity must enter the attempt
ledger from the handle returned by successful exclusive creation, before any SQLite/
PRAGMA/schema work. `AlreadyExists` or another pre-create error must never capture or
adopt the pathname. Change the crate-internal state constructor boundary as needed so
the system port receives the identity at the instant of creation. Preserve the retained
handle, `SQLITE_OPEN_NOFOLLOW`, and before/after identity checks.

Rollback must drop an owned SQLite connection before moving the state file and clear
the store afterward. Strict open always discards any cached prior connection and
reattaches the current validated state path, including after an earlier successful
create/open. An ordinary fully reconciled failure may be retried; cleanup uncertainty
latches unavailable.

## 3. Make wallet/keys capture mandatory and exact

Never discard `capture_wallet_artifacts`. After create/generate/restore returns, combine
the RPC result with artifact inspection. Retain every exact wallet or keys inode that
appeared, even on an RPC error, so partial output can be reconciled. On RPC success,
require both files, exact regular type, effective owner, `0600` mode, the unchanged
validated wallet directory, and a complete pair before continuing. Any capture or
identity uncertainty must make rollback return compound `INTERNAL` and latch the
account unavailable. An existing-wallet open owns neither file.

Rollback/quarantine may operate only on the published identities. If a hard-link
quarantine validation fails, remove only the newly created destination link when its
identity is known and sync the directory; never leave an extra link to a substituted
file or touch the substituted source.

## 4. Reconcile the operation on unwind

Track an explicit attempt-active state in the manager/port. In each `SystemAccount`
catch boundary, wipe all operation secrets and run the same complete child teardown and
owned-artifact rollback before `resume_unwind`. Secret wiping must still occur if
rollback fails. If reconciliation cannot be established, latch unavailable so a caller
that catches the resumed panic cannot discard or reuse the ledger. Do not reset an
active ledger until success or proven cleanup.

## 5. Complete the pre-child identity gate

Before child start, decode both the sealed and stored primary values, reject invalid
UTF-8/non-address syntax, and require them to match in addition to account/network/kind/
restore height. `StoredIdentity::validate` must independently reject malformed primary
bytes rather than only their length. Keep the post-start `get_address` plus single
closed `validate_address` network proof and exact comparison against the already-bound
primary.

## 6. Close directory owner and durability gaps

In the accepted process composition, validate the effective owner as well as no-follow
type and `0700` mode for the broker root and every existing namespace/network/account/
runtime/wallet/ring component before config creation or child start. Account preflight
or process helpers must cover existing runtime and ring paths. Existing wrong-owner or
hostile entries are rejected, never chmodded or replaced. Newly created entries must be
revalidated with the effective owner.

Retain a no-follow account-directory handle/device/inode in `PathSqliteSurface` and
compare it during state directory sync; do not discard the tuple. When creating any new
vault/XMR/network/account/wallet/runtime/ring directory, validate it and sync its exact
containing directory so the new directory entry is crash-durable. Preserve leaf
file/directory sync and strict reopen/identity verification.

The retained state-file handle plus pathname open with `SQLITE_OPEN_NOFOLLOW` and exact
before/after checks is accepted under rusqlite 0.37 and repository-wide
`forbid(unsafe_code)`. Do not add unsafe code, a `/proc/self/fd` journal workaround, a
dependency, or broader scope.

Preserve every valid Correction-03 correction. Do not begin Slice 5/viewing/receiver
issuance, local-Monero execution, Electron/UI wiring, or BBD-WAL-008/009.

Return exact changed paths, final line counts and SHA-256 values, a blocker-to-correction
map, and all residual concerns. XHigh source review precedes any Hermes formatter,
compiler, test, integration, Git, or evidence work.
