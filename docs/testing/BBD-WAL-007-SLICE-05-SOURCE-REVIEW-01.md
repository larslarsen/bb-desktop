# BBD-WAL-007 Slice-5 Source Review 01

Reviewer: Lead Engineer/Reviewer — Codex at High

Decision: **REJECTED — BOUNDED CORRECTION REQUIRED**

Reviewed unstaged drop:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/xmr.rs` | 75 | `4019cdbbd87630bd447c2e4e187ee4c81be376cf89b680be8cd6ce4e62f81d09` |
| `wallet-broker/src/xmr/account.rs` | 3,034 | `dc67b69426b63f5b6e2652cf74c0c08af20054564bde83f1b4f9be46686b2617` |
| `wallet-broker/src/xmr/process.rs` | 1,776 | `c60640d5edde7bc8e0bd9f7bd7ca92038792b91acbcb36d9e162c7d9c0320cd9` |
| `wallet-broker/src/xmr/rpc.rs` | 2,535 | `1a24cadfb0adb31289d532fab5f640f65e84154969c4d132e69fbd612791ad38` |
| `wallet-broker/src/xmr/store.rs` | 1,861 | `6b2600a98423f7ab322d74b1bd2f611a4421bdacba11aedf095218d77eb30e4d` |
| `wallet-broker/src/xmr/receiver.rs` | 924 | `de991795b2171191e2782992d5e56c68ff78c88df42151b9225d0b301351ffe7` |
| `wallet-broker/src/xmr/test_support.rs` | 5,740 | `decc80264a42e1adf1ccdd99bd134556a91718ec4994cb8f1ce4e0500de4c2a0` |

The frozen `xmr_receiver.rs` remains byte-exact at 588 lines and SHA-256
`d8656c533d7b2d08eb9fa8831cd70da6e1da632ed9563fc2af569cab754d3622`.
No formatter, compiler, test, Clippy, Node, policy, build, product binary, Git mutation,
or network command was run by the reviewer.

## Blocking findings

1. The production receiver path is disconnected from account custody. Both
   `SystemReceiverPort::open` and `create` construct `SystemWalletRpcControl::new()`.
   That control has no authenticated session and is not the control owned by the live
   `SystemAccountPort` process pool. Its first receiver RPC therefore cannot operate on
   the accepted owned child. `create` also attempts to create a new account directory
   and state store instead of using the Slice-4 account's established identity/store.
2. Production viewing is absent. `sanitize_view` accepts caller-supplied states,
   heights, balances, and branch, while `ViewRig` manufactures/parses those values.
   No production operation proves the owned account session, probes the accepted local
   node, refreshes/reads wallet height and balance, validates hard-fork state, derives
   independent states, and only then constructs the sanitized view.
3. The four persistence fault cases do not exercise production persistence. The
   recording port returns `STATE_CORRUPT` before `AccountStore::persist_receiver` for
   row, sequence, commit, and file-sync faults. The manager then sets both commit/sync
   observer flags merely because `persist_binding` returned success. This encodes the
   expected test answers in the rig and leaves rollback, sync, and consumed-gap behavior
   unproved.
4. Durability failure is not fail-closed. `persist_receiver` commits before its explicit
   file/directory sync and has no corruption/unavailability latch. A sync/reopen/proof
   failure can return an error while leaving a replayable row; later operations may
   return or build on state whose required durability proof failed.
5. The post-write verification compares only account index, subaddress index, and
   sequence. It does not compare the loaded request ID and subaddress to the exact
   candidate, so substituted durable binding content can pass before the original
   receiver is returned.
6. `AccountStore::load_identity` still calls `configure_full_synchronous`, silently
   repairing a non-FULL session before receiver lookup. Receiver operations must detect
   drift as `STATE_CORRUPT`, not heal it as a side effect.
7. `xmr.rs` embeds `model.rs` with `include!` solely to add receiver states and errors
   while keeping the model file byte-exact. This circumvents the intended source-module
   boundary. The correction explicitly opens `model.rs`; restore the ordinary
   `pub mod model;` declaration and put model-owned definitions in that file.
8. Receiver test roots use deterministic names under `std::env::temp_dir()`, have no
   drop cleanup, and collide on a later test-process run after the counter resets. This
   also bypasses the repository's disk-backed-target discipline. Use a collision-safe,
   test-owned leaf under the repository target and remove only the exact validated leaf,
   while preserving the close/reopen test until its reopened owner drops.

The overall module split, closed RPC request additions, redacted receiver value, exact
schema constraints, replay-before-RPC ordering, and mutex-serialized core are useful and
may be preserved. Grok 4.6 High alone may make Correction 01 under the linked handoff.
Hermes execution/integration, Sol, Spark, test edits, broader/final acceptance, and the
real offline local-Monero gate remain unauthorized.
