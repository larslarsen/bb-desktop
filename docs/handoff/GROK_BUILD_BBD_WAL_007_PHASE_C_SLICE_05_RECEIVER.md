# Grok Build Handoff — BBD-WAL-007 Phase-C Slice 5 Viewing/Fresh Receiver

You are **Sr Dev — Grok Build using Grok 4.6 High**. This durable handoff is
authoritative. Do not delegate this source work to Sol.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely before editing: `AGENTS.md`, `TESTING.md`, the development-role policy,
`tickets/BBD-WAL-007.md`, Test-Source Review 04, Slice-4 Acceptance 01, the complete
accepted XMR test/source inventory, and `docs/handoff/CURRENT_TASK.md`.

## Sole task and paths

Implement only Phase-C Slice 5: sanitized XMR account viewing and durable idempotent
fresh-receiver issuance. Edit only:

- `wallet-broker/src/xmr.rs`;
- `wallet-broker/src/xmr/account.rs` only for the narrow production account-state/view
  bridge required by this slice;
- `wallet-broker/src/xmr/process.rs` only if the accepted typed wallet control boundary
  must expose a receiver/view operation;
- `wallet-broker/src/xmr/rpc.rs` only for closed typed balance, height, hard-fork,
  address-creation, address-validation, and exact-address reads already in the approved
  RPC allowlist;
- `wallet-broker/src/xmr/store.rs` only for receiver lookup/commit/reopen semantics;
- create `wallet-broker/src/xmr/receiver.rs`; and
- `wallet-broker/src/xmr/test_support.rs` only for thin recording/fault-injection rigs
  that drive the production receiver/view core.

Starting identities:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/xmr.rs` | 7 | `8b382ef673e82001720727c161f365c0fa3c1401d1bdaa4d20ee2683c001559b` |
| `wallet-broker/src/xmr/account.rs` | 3,034 | `67cc2261c138b83f3fa963bfe6ce646bea17c9258185d986a4c43daf0662c137` |
| `wallet-broker/src/xmr/process.rs` | 1,763 | `98a18be4a0f26ae71b5818ba893910d3183a3ddea49263c9291185fbde09fc2f` |
| `wallet-broker/src/xmr/rpc.rs` | 2,426 | `59a0f33f66cb65a007a96f7f4e073a987a3b8c0e123d7f59624e8d442bf6f56b` |
| `wallet-broker/src/xmr/store.rs` | 1,327 | `248ca3f6eaeb98b66fbe2d041637c521f3b2371b8b9c231cbcdd3d3c57174607` |
| `wallet-broker/src/xmr/test_support.rs` | 4,771 | `5ef016587b6eeffa146ee8a38baae42b57eaf988755eb85c2d96076c8ffa2502` |
| `wallet-broker/tests/xmr_receiver.rs` | 588 | `d8656c533d7b2d08eb9fa8831cd70da6e1da632ed9563fc2af569cab754d3622` |

Require `receiver.rs` to be absent before editing. Every test, manifest, lockfile,
existing source outside the seven authorized paths, policy, workflow, documentation,
fixture, and other repository must remain byte-exact.

Do not run Cargo, rustfmt, tests, Clippy, Node/npm, a policy/security command, a build,
wallet/Monero executable, package manager, network action, Git command, or GitHub
action. Do not stage, commit, push, or maintain evidence. Read-only source inspection
and line/hash reporting are allowed. Leave the completed drop unstaged for reviewer
inspection.

## Required production core

Implement the ticket's viewing and receiver contract as production logic, never as
answers encoded in `ViewRig` or `ReceiverRig`. Test-support ports may record operations
and inject named failures, but validation, RPC ordering, state derivation, durable
lookup/commit, concurrency serialization, and fail-closed behavior must execute through
the same receiver/view core available to production.

Represent node and wallet state independently. Node height comes only from the accepted
local-node probe. Wallet height and exact balance fields come only from the authenticated
wallet child. Preserve `NODE_UNAVAILABLE`/`NODE_SYNCING`/`READY` independently of
`UNAVAILABLE`/`LOCKED`/`WALLET_REFRESHING`/`READY`; device state is always
`NOT_APPLICABLE`. Parse total and unlocked atomic units as canonical decimal u64 values,
reject missing/stale/negative/floating/overflow/leading-zero/inconsistent data, and
never substitute total for unlocked.

Return only the ticket's sanitized account-view fields. Set XMR viewing and private
receive capabilities exactly as frozen, keep all ZEC/hardware/prepare/sign/broadcast
capabilities false/null, and validate the hard-fork result before exposing its decimal
version as `consensus_branch`. Never expose primary address, receiver history, any
secret, RPC credential/endpoint, path, PID, raw response, SQLite row, request ID, or
upstream diagnostic.

Validate account ID, request ID, and the closed non-mainnet network before any lookup or
RPC side effect. An exact durable `(account_id, network, request_id)` replay returns the
stored binding without RPC. A new request requires a ready eligible account, then calls
typed `create_address` for account zero/empty label, validates positive u32 index,
network and non-primary subaddress classification, confirms exact equality through
`get_address`, and durably commits the binding and next positive signed-64 issuance
sequence before return. Never return the primary address.

Serialize concurrent issuance per account. Distinct request IDs must receive distinct,
increasing indices and sequences. Preserve all returned bindings across reopen. A
post-RPC persistence failure may consume and permanently skip an address/index, but the
unbound address must never be returned or reused. Reject index/sequence exhaustion
without wrap.

Extend the existing schema-version-1 store without widening it: independently unique
request ID, `(account_index, subaddress_index)`, subaddress, and issuance sequence;
account index exactly zero; positive subaddress index; positive signed-64-bounded
sequence; `FULL` sync and `0600` file. Schema drift, duplicate binding, rollback,
corruption, failed sync, or wrong identity is `STATE_CORRUPT`, never reconstruction from
wallet output.

Lock, syncing, watch-only initialization failure, wrong network, RPC mismatch,
persistence failure, and exhaustion return only the frozen stable code with no primary
or stale-address fallback. Preserve the phase-bound typed RPC allowlist and all account
custody, process ownership, cleanup, secret wiping, and error-redaction invariants from
Slices 1–4.

## Frozen observer and stop conditions

Make all 15 accepted tests in `wallet-broker/tests/xmr_receiver.rs` meaningful through
production behavior. Implement exactly the constants/types/methods it imports; do not
weaken an assertion, add a conditional pass, encode answers in a rig, or add raw/generic,
transfer, signing, submission, mainnet, or alternate-endpoint authority.

Stop without further edits on an architectural conflict, need for an eighth path or
dependency change, inability to preserve the accepted custody/process/RPC/store
boundaries, or any requirement that weakens durability, idempotence, concurrency,
validation, redaction, or address non-reuse. Do not begin the real offline local-Monero
gate, Electron/UI wiring, BBD-WAL-008/009, broader integration, or final acceptance.

Return the exact changed-path list, resulting line counts and SHA-256 values, a concise
mapping from production modules to the 15 receiver-test groups, and any residual
concern. The reviewer will inspect source before Hermes may format, compile, test,
integrate, or use Git.
