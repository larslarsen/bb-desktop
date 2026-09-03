# Grok Build Handoff — BBD-WAL-007 Phase-C Slice 5 Correction 01

You are **Sr Dev — Grok Build using Grok 4.6 High**. Continue the current unstaged
Slice-5 drop. Do not delegate to Sol.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely before editing: `AGENTS.md`, `TESTING.md`, `tickets/BBD-WAL-007.md`,
Slice-4 Acceptance 01, the original Slice-5 handoff, Slice-5 Source Review 01, the
frozen receiver tests, and the complete current source in every authorized path.

## Exact scope

Edit only these eight paths:

- `wallet-broker/src/xmr.rs`;
- `wallet-broker/src/xmr/model.rs`;
- `wallet-broker/src/xmr/account.rs`;
- `wallet-broker/src/xmr/process.rs`;
- `wallet-broker/src/xmr/rpc.rs`;
- `wallet-broker/src/xmr/store.rs`;
- `wallet-broker/src/xmr/receiver.rs`; and
- `wallet-broker/src/xmr/test_support.rs`.

The seven changed-path starting identities are exactly the table in Source Review 01.
`wallet-broker/src/xmr/model.rs` starts at 151 lines and SHA-256
`23a85dc216839cb936161845b52cacdca68a24181e5d31afeaaba30e62c77ca9`.
Every test, manifest, lockfile, other production source, documentation, policy,
workflow, fixture, and repository must remain byte-exact.

Do not run Cargo, rustfmt, tests, Clippy, Node/npm, policy/security commands, a build,
wallet/Monero executable, package manager, network action, Git command, or GitHub
action. Do not stage, commit, push, or maintain evidence. Leave the corrected source
unstaged for reviewer inspection.

## Required correction

Restore `xmr.rs` to an ordinary `pub mod model;` declaration and place `NodeState`,
`WalletState`, `DeviceState`, and the four stable receiver error constructors in
`model.rs`.

Eliminate the disconnected `SystemReceiverPort` design. Production view and fresh
receiver operations must use the same already-started, authenticated, identity-proved
wallet child, process pool, state store, and unavailable latch owned by `SystemAccount` /
`SystemAccountPort`. Extend the existing typed process/pool/account boundaries narrowly;
do not create a second RPC control, child, account directory, or state store. A locked
software account has no RPC authority. A successfully retained watch-only account may
view/receive; cold restart still requires the accepted authenticated open flow first.

Add actual production operations that:

- validate identity/network before side effects and prove the owned session;
- obtain node state/height/hard-fork only through the accepted local-node probe;
- obtain wallet refresh/height/balances only through the authenticated owned child;
- derive node and wallet states independently and validate exact u64 balance invariants;
- return only the frozen sanitized view; and
- perform replay-before-RPC and new issuance through the same account-owned store and
  child.

Keep `ReceiverManager`'s useful sequencing if desired, but its production port must
borrow or route through the live account authority. Ensure there is a real callable
production surface from `SystemAccount`, not only generic internals used by test rigs.

Move row-write, sequence-write, commit, file-sync, directory-sync, reopen, and loaded-
binding substitution fault injection to a thin `StoreSurface` wrapper or equivalent
that drives `AccountStore::persist_receiver`. The frozen tests may group some named
faults, but none may return the expected error before entering the production stage it
claims to test. Observer proof must be emitted by completed production stages, not set
unconditionally by the manager.

On any transaction, commit, sync, reopen, schema, or post-write identity/binding proof
failure, latch the account/receiver authority unavailable for the lifetime of that
owned instance. Never replay or extend uncertain state. Compare the complete loaded
binding—request ID, account index, subaddress index, subaddress, and sequence—to the
candidate before return. Preserve the consumed address/index gap without returning or
reusing it after a pre-binding failure.

Receiver operations must require, never reset, `PRAGMA synchronous = FULL`. Preserve
Slice-4 initialization behavior where configuration is legitimate, but do not let a
receiver read heal durability drift before checking it.

Replace `std::env::temp_dir()` with a collision-safe test-owned directory under
`env!("CARGO_MANIFEST_DIR")/target`. Validate and remove only the exact leaf. Add drop
cleanup that makes repeated test-binary runs safe and transfers cleanup ownership
correctly across `ReceiverRig::close` / `open`. Do not remove a broader target or parent
directory and do not follow symlinks.

Preserve every valid part of the first drop and every Slice-1–4 invariant. Do not weaken
tests, add conditional passes, expose secrets/paths/receivers in diagnostics, widen RPC
authority, add mainnet/remote-node behavior, or begin the real local-Monero gate.

Stop on any need for a ninth path, dependency change, test edit, or architecture that
cannot share the accepted account-owned process/store authority. Report exact changed
paths, resulting line counts/SHA-256 values, how production view/receiver reach the
owned session, how each persistence fault reaches its named production stage, cleanup
ownership, and residual concerns.
