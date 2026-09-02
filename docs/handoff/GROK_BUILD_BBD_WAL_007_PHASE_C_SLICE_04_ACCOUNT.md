# Grok Build Handoff — BBD-WAL-007 Phase-C Slice 4 Account Custody/Recovery

You are **Sr Dev — Grok Build using Grok 4.6 High**. This durable handoff is
authoritative. Do not delegate this source work to Sol.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely before editing: `AGENTS.md`, `TESTING.md`, the development-role policy,
`tickets/BBD-WAL-007.md`, Test-Source Review 04, Expected Red 02, Slice-3 Acceptance
01, the complete accepted XMR test/source inventory, the existing vault/store
primitives, and `docs/handoff/CURRENT_TASK.md`.

## Sole task and paths

Implement only Phase-C Slice 4: XMR account custody, durable account identity/state,
software creation, watch-only import, authenticated recovery/open, and lock behavior.
Edit only:

- `wallet-broker/src/xmr.rs`;
- create `wallet-broker/src/xmr/account.rs`;
- create `wallet-broker/src/xmr/store.rs`; and
- `wallet-broker/src/xmr/test_support.rs` only for a thin recording/fault-injection
  adapter that drives the production account/store core used by `xmr_account`.

Starting identities:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/xmr.rs` | 5 | `92af416c0ed6c5f2f8a6ff7011facb622da5e3f1ba8adaa2e96f600c09f31411` |
| `wallet-broker/src/xmr/test_support.rs` | 2,705 | `2c445494b36fd645240cc9a25405782eff6d3e8187b4762aee6a0fd7ca640a40` |
| `wallet-broker/tests/xmr_account.rs` | 586 | `5e8afabaa8c820be84a010a88fef0e07617e67934c06b32ab445901b1ddad35b` |
| `wallet-broker/src/xmr/model.rs` | 151 | `23a85dc216839cb936161845b52cacdca68a24181e5d31afeaaba30e62c77ca9` |
| `wallet-broker/src/xmr/distribution.rs` | 914 | `4135db83eca151185d8222498d2435ef56e2d564ff17bd3fa88481260758aed5` |
| `wallet-broker/src/xmr/process.rs` | 1,184 | `7533a86344c178ed25143d21bc3961a72ea7a1fcf85067f90b57134178928a1f` |
| `wallet-broker/src/xmr/rpc.rs` | 1,913 | `7593322a5aef2fc146698d2e07a541cd9fb796b92e1f8e3fd699bcfbb2b219f9` |

Require `account.rs` and `store.rs` to be absent before editing. Every test, manifest,
lockfile, existing source outside the four authorized paths, policy, workflow,
documentation, fixture, and other repository must remain byte-exact.

Do not run Cargo, rustfmt, tests, Clippy, Node/npm, a policy/security command, a build,
wallet/Monero executable, package manager, network action, Git command, or GitHub
action. Do not stage, commit, push, or maintain evidence. Read-only source inspection
and line/hash reporting are allowed. Leave the completed drop unstaged for XHigh review.

## Required production core

Implement the ticket's account contract as production logic, not behavior duplicated
inside `AccountRig`. Test-support ports may record operations and inject the named
faults, but encoding/decoding, validation, call order, rollback, persistence ordering,
capability decisions, and state transitions must execute through the same account/store
core available to production.

The closed `XmrSecretV1` frame must use magic `BBXMR001`, kinds 1/2, big-endian integer
lengths, exact 64-byte lowercase-hex wallet password, exact 95-byte primary address,
bounded 25-word software mnemonic or exact 64-byte lowercase-hex private view key, no
unknown/trailing bytes, and a 2,048-byte total ceiling. Secret-bearing types must have
no revealing `Debug`/display path and must wipe on success, error, cancellation,
unwind, and drop. Generate wallet passwords from 32 fresh OS-random bytes.

Validate the 32-lowercase-hex account ID and stagenet/testnet before any path, node,
wallet, or vault side effect; mainnet fails `NETWORK_DISABLED`. Derive all
network-bound private paths and the wallet filename from validated state only. Enforce
directory `0700`, wallet/keys/state DB `0600`, no symlink/non-regular/wrong-owner/
wrong-mode/partial/cross-account/cross-network substitution, and never replace a
hostile entry.

Software creation must use the accepted local `height_without_bootstrap` with a
saturating 100-block margin; call typed `create_wallet`, mnemonic-only `query_key`,
`get_address`, and `close_wallet` in the frozen order; verify the primary address; seal
the vault secret before durable account-state success; and return no primary or secret.
Watch-only import must require restore height no greater than the accepted local
height, use only `generate_from_keys`, `get_address`, and `close_wallet`, verify RPC
watch-only kind and the same primary, and never gain prepare/sign/broadcast authority.

Existing files open only through `open_wallet`. Missing files recover only after
authenticated-vault access through `restore_deterministic_wallet` for software or
`generate_from_keys` for watch-only. After open/recovery, verify primary address,
network, kind, restore height, and wallet-file identity against sealed/durable state.
Any mismatch is fail-closed with no returned account or retained handle/child.

Vault or durable-state failure must close/stop the owned child, wipe secrets, remove or
quarantine generated wallet files, and return no account. Cleanup failure compounds to
`INTERNAL` and leaves the account unavailable. Software lock calls `close_wallet` then
`stop_wallet`, reaps the child, and wipes its password. A watch-only process may be
retained only after successful import/open; cold restart again requires authenticated
vault access.

`store.rs` must provide the ticket's closed schema-version-1, network/account/primary
identity binding, `0600` SQLite file, `FULL` synchronous durability, checked positive
signed-64 issuance sequence, and independently unique receiver keys/values for the
next slice. It must reject schema drift, duplicate/rollback/corrupt identity, partial
state, and failed file/directory sync as `STATE_CORRUPT`; do not implement receiver
issuance or viewing behavior in this slice.

Keep every capability false for prepare/sign/broadcast and every device-specific/ZEC
capability. Public results and errors must exclude primary address, seed/view key,
wallet/RPC password, selected or wallet path, endpoint/port/PID, request/receiver data,
raw RPC/JSON, SQLite rows, and upstream text. Preserve the ticket's stable error codes.

## Frozen observer and stop conditions

Make all 16 accepted tests in `wallet-broker/tests/xmr_account.rs` meaningful through
production behavior. Implement exactly the constants/types/methods it imports; do not
weaken an assertion, add a conditional pass, encode answers in the rig, or add broader
public/raw RPC authority. `XmrSecretFixture` must delegate the production secret codec
and validators. `AccountRig` must drive the production account/store state machine
through explicit recording ports.

Stop without further edits on an architectural conflict, need for any fifth path or
dependency change, inability to use the existing vault/store/process/RPC boundaries,
or any requirement that would weaken custody, durability, validation, rollback, or
secret wiping. Do not begin Slice 5, viewing/receiver issuance, real local-Monero work,
Electron/UI wiring, or BBD-WAL-008/009 authority.

Return the exact changed-path list, resulting line counts and SHA-256 values, a concise
mapping from production modules to the account-test groups, and any residual concern.
The reviewer will inspect source before Hermes may format, compile, test, integrate, or
use Git.
