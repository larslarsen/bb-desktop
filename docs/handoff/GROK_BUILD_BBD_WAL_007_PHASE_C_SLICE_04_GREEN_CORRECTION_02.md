# Grok Build Handoff — BBD-WAL-007 Phase-C Slice 4 Green Correction 02

Status: AUTHORIZED — FIVE-PATH SOURCE CORRECTION

Source actor: Sr Dev — Grok Build using Grok 4.6 High

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Protected governance parent: the commit containing this handoff

Repository: `/home/lars/OpenBazaar/bb-desktop`

Read completely before acting: `AGENTS.md`, `TESTING.md`, the development-role policy,
`tickets/BBD-WAL-007.md`, Test-Source Review 04, the complete frozen
`wallet-broker/tests/xmr_account.rs` and `wallet-broker/tests/xmr_hygiene.rs`, Slice-4
Source Review 06, Compile Correction Source Review 01, Green Resume 02 Rejection 01,
all five editable source files, and `docs/handoff/CURRENT_TASK.md`.

## Exact boundary

Edit only:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/xmr/account.rs` | 3,039 | `c3ae5b07174a9d1732ab3ec7ee2628f8a7f2c394d9af875026fa57d594d311ee` |
| `wallet-broker/src/xmr/store.rs` | 1,380 | `21ef2db4eaf32389809a86bcc3c0c8164ac57763ac7567c35c6f2007abb86749` |
| `wallet-broker/src/xmr/process.rs` | 1,803 | `aec5e5cc8bf93be3ee86888aa1ea5209ceed9a7ce229c3ab2fd9e0935d85688c` |
| `wallet-broker/src/xmr/rpc.rs` | 2,413 | `7f5019c9f4fb668a8f68bdf06f8ad8f20433890cef299b458f00f515b3c89965` |
| `wallet-broker/src/xmr/test_support.rs` | 3,918 | `b359256394de4dcb2cb0788aa558c381c8f6e1a5733aa52a462b41b7b7018bb4` |

Freeze byte-for-byte:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/vault.rs` | 776 | `bb79b16f24a9abbcd266e856a37ee982bde8fcdc2da3858e75d34bdd34a4af86` |
| `wallet-broker/src/xmr.rs` | 7 | `8b382ef673e82001720727c161f365c0fa3c1401d1bdaa4d20ee2683c001559b` |
| `wallet-broker/tests/xmr_account.rs` | 586 | `5e8afabaa8c820be84a010a88fef0e07617e67934c06b32ab445901b1ddad35b` |
| `wallet-broker/tests/xmr_hygiene.rs` | 329 | `3653eba660be481d71e75185e024ae7e0a17f8754089c83b21edb397da84230f` |

Every other path is read-only, including all tests, governance/evidence, manifests,
lockfiles, configuration, and generated/cache state.

## Required correction

### 1. Warning-free structure without suppression

Eliminate all 17 warnings recorded in Green Resume 02 Rejection 01. Do not add an
`allow` attribute, dummy read, fake branch, unreachable call, underscore rename used
only to silence lint, or public API widening.

- Make `RpcSecret` exactly `pub(crate)` so it matches the existing crate-visible
  `RpcRequest` boundary while staying outside the public crate API.
- Remove genuinely unused forwarding methods and paired platform/test implementations,
  including the unused account `stop_wallet` trait path, unused process RPC-stop path,
  unused broker-exit-all/pool observer wrappers, and no-longer-referenced store query/
  column/surface verification helpers. Preserve every method reached by frozen tests or
  live account teardown; in particular, do not weaken `teardown_owned`, process-manager
  cleanup, or the ProcessRig-owned account isolation paths.
- Use meaningful existing `StoredIdentity` accessors from the SQL binding if those
  accessors remain; remove only APIs with no present semantic consumer.
- Remove the unused stored `RecordingAccountPort::kind` field without changing the
  account kind owned by `AccountManager` or sealed identity.

### 2. Preserve the complete `u64` restore height

The production store must round-trip every `u64`, including `u64::MAX - 100`, without
clamp, lossy cast, signed reinterpretation exposed as an SQL number, or test-only
bypass. Change the closed `account_identity.restore_height` representation to a
canonical `BLOB NOT NULL CHECK (length(restore_height) = 8)`. Bind exactly the eight
big-endian bytes from `u64::to_be_bytes`; load only an exact eight-byte blob and decode
with `u64::from_be_bytes`. Update `ColumnSpec` and strict schema-text verification to
require the BLOB type and exact-length constraint. Malformed type/length remains
`STATE_CORRUPT`. Do not alter receiver sequence/index signed bounds or schema version.

### 3. Implement the frozen negative-authority/hygiene support

Implement the missing crate-visible test-support surface required by the frozen
329-line `xmr_hygiene.rs`: `AuthorityRig`, `HygieneExit`, `HygieneRig`,
`ObservableCanary`, `ObservableSecretClass`, the commitment/receipt and immediate-
failure views they return, and every exact method consumed by the nine frozen tests.

This support must exercise and reflect the accepted production boundaries, not return
unconditional pass booleans or copy expected assertions into disconnected constants:

- authority exposes only the exact seven phase-bound typed operations and routes typed
  wallet/node calls through the existing `RpcMethod`/`RpcRequest` allowlist;
- unlisted/raw/spend/daemon-switch names fail `SCHEMA` before bytes, state transition,
  return data, or side effect;
- mnemonic query is phase-bound, mnemonic-only, and once-only during fresh software
  creation;
- mainnet rejection uses the real account/network validation before any recorded node,
  filesystem, database, vault, socket, child, or returned-value effect;
- canary receipts use actual SHA-256 commitments over each distinct installed value and
  retain only class, byte length, and lowercase digest as observable receipt data;
- success, error, cancellation, replacement, panic-unwind, and drop paths keep secret
  material in zeroizing owners and record actual wipe observations for the ten frozen
  labels without putting canary plaintext into Debug, Display, errors, error chains,
  logs, diagnostics, panic, or teardown output;
- every named teardown cause leaves no owned child/handle/runtime secret/credential,
  proves the owned process group reaped, and never touches a non-owned process; and
- XMR failure preserves immutable ZEC/social/Electron/quote snapshots and zero call
  counts.

Public diagnostics remain exactly `operation`, `account_id`, `asset`, `network`, and
`code`; every other requested field is `SCHEMA`. All new Debug implementations are
closed/redacted. Do not expose a generic RPC, raw bytes, arbitrary diagnostic field,
secret/path getter, mainnet route, or mutable cross-subsystem handle.

Make the additions deterministic, offline, credential-free, and bounded. Reuse the
existing account/RPC/process secret and teardown machinery where applicable; a test rig
may record observables, but it may not forge a production success or make a frozen test
vacuous.

## Prohibited actions and stop

Do not edit tests. Do not run rustfmt, Cargo, compiler, tests, Clippy, builds, binaries,
Node/npm, package-manager, policy/security, network, Git, or GitHub. Do not stage,
commit, push, edit governance/evidence, or invoke Sol/another actor.

Stop without editing on any parent/index/path/hash/source-boundary mismatch. After the
five-path source drop, report exact changed paths, line counts, SHA-256 identities,
warning-removal inventory, full-width height representation, hygiene/authority
semantics, and prohibited-action compliance. Stop for XHigh inspection; Hermes remains
unauthorized.
