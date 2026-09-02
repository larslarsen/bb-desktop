# Grok Build Handoff — BBD-WAL-007 Phase-C Slice 4 Correction 01

You are **Sr Dev — Grok Build using Grok 4.6 High**. This durable handoff is
authoritative. Do not delegate this source work to Sol.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff; the rejected source
drop remains unstaged above that parent.

Read completely before editing: `AGENTS.md`, `TESTING.md`, the development-role policy,
`tickets/BBD-WAL-007.md`, Test-Source Review 04, Expected Red 02, Slice-3 Acceptance
01, Slice-4 Source Review 01, the original Slice-4 handoff, the complete accepted XMR
and WAL-004 vault/store/process/RPC source, `wallet-broker/tests/xmr_account.rs`, and
`docs/handoff/CURRENT_TASK.md`.

## Sole task and corrected paths

Replace/correct the rejected Slice-4 account-custody/recovery drop. Edit only:

- `wallet-broker/src/vault.rs`;
- `wallet-broker/src/xmr.rs`;
- `wallet-broker/src/xmr/account.rs`;
- `wallet-broker/src/xmr/store.rs`;
- `wallet-broker/src/xmr/process.rs`;
- `wallet-broker/src/xmr/rpc.rs`; and
- `wallet-broker/src/xmr/test_support.rs`.

Require the four rejected-drop identities recorded in Source Review 01. Require these
three accepted starting identities:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/vault.rs` | 773 | `500cd2f91ec0a2e0052779ba6b2357053ce0bea1d644fb2c35066f768f363fe0` |
| `wallet-broker/src/xmr/process.rs` | 1,184 | `7533a86344c178ed25143d21bc3961a72ea7a1fcf85067f90b57134178928a1f` |
| `wallet-broker/src/xmr/rpc.rs` | 1,913 | `7593322a5aef2fc146698d2e07a541cd9fb796b92e1f8e3fd699bcfbb2b219f9` |

If any identity or scope differs, stop without editing. Preserve every test, manifest,
lockfile, source outside these seven paths, policy, workflow, documentation, fixture,
and other repository byte-exact.

Do not run Cargo, rustfmt, tests, Clippy, Node/npm, policy/security commands, builds,
binaries, package managers, network, Git, GitHub, staging, commit, push, or evidence.
Read-only source inspection and line/hash reporting are allowed. Leave the corrected
drop unstaged for XHigh review.

## Required architecture correction

The result must include a real, crate-usable Linux system account path that composes the
accepted distribution/process/RPC implementation, WAL-004 vault crypto/store, and a
path-backed XMR account-state store. `AccountRig` must be only a recording/fault port
over that same account/store state machine. If the real composition cannot be completed
inside these seven paths and existing dependencies, stop and report the exact missing
boundary rather than returning another simulated production core.

Extend the accepted RPC layer only with closed, phase-bound account operations:
`create_wallet`, mnemonic-only `query_key`, `get_address` plus exact-network validation,
`generate_from_keys`, `open_wallet`, `restore_deterministic_wallet`, and the already
accepted `close_wallet`/`stop_wallet`. Use enums/private constructors and typed results;
there must be no public or crate-wide raw method string, arbitrary parameter map, or
generic JSON entry point. Secret-bearing request/result/serialized data must be
non-`Debug`, redacted, bounded, and zeroized on every exit. Keep the Slice-3 node and
Digest/HTTP behavior byte-semantically intact.

Expose only the minimum closed process-control bridge required to call those account
operations on the exact authenticated broker-owned child/session and to perform the
accepted teardown. Do not permit account switching, raw RPC, arbitrary port/
credential access, adoption, or bypass of the four-child and exact-owned-child
lifecycle. Preserve every prior process invariant.

Add `XmrTestnet` to the WAL-004 vault metadata mapping and asset/network validation,
without widening any ZEC or mainnet combination. The system account path must actually
seal `XmrSecretV1` through `seal_vault`, durably store the envelope through
`VaultStore`, and recover only through `open_vault_bytes` after caller authentication.
Map errors to the ticket's stable XMR codes without exposing vault or upstream text.

Implement a path-backed SQLite account store beneath the derived network/account
directory. Validate all components without following hostile final entries; require
the broker-owned `0700` directories and `0600` wallet/keys/state files; reject partial,
wrong-owner/mode, symlink, non-regular, cross-account, cross-network, or replaced state.
Use `PRAGMA synchronous=FULL`, transactions, checked integers, exact schema/version/
column/constraint and identity validation, real file and directory synchronization,
and fail-closed reopen. Memory SQLite may exist only behind the recording test port;
its sync calls may not substantiate production durability.

On any vault/state/file failure, reconcile actual persisted artifacts: close/stop/reap
the exact child, wipe all secret copies, remove or quarantine generated wallet files,
and prevent a partial vault/state record from becoming an available account. A cleanup
failure compounds to `INTERNAL`. Do not merely clear in-memory success flags.

Replace `TypedWalletCall` and every secret-bearing ordinary `String`/`Vec<u8>` escape
with closed types and `SecretBytes`/`Zeroizing` custody as appropriate. Nothing holding
password, mnemonic, private view key, primary address, or serialized plaintext may
derive revealing `Debug` or survive its required scope. Test-only retained observer
copies must be explicitly redacted and zeroized on drop; wipe assertions must observe
real production-core wipe events, not a Boolean set by control flow.

For watch-only import, establish exact selected-network validity through the closed
authenticated wallet-RPC validation path before success. The frozen high-level
`rpc_calls()` observer may continue to report the three specified logical account
operations, but it must not conceal an unvalidated system path. The synthetic fixtures
remain deliberately non-spendable and must not become live wallet material.

Preserve the rest of the original handoff: exact secret frame, entropy, restore-height,
create/import/open/recovery order, identity checks, rollback, lock/retention,
capability negatives, stable errors, and all 16 frozen account tests. Do not weaken or
edit a test, add a conditional pass, begin Slice 5/viewing/receiver issuance, or add
Electron/UI/BBD-WAL-008/009 authority.

Return the exact changed paths, resulting line counts and SHA-256 values, a concise
production composition map, how each Source Review 01 blocker was closed, and any
residual concern. The reviewer will inspect source before Hermes may format, compile,
test, integrate, or use Git.
