# BBD-WAL-007 Slice-5 Correction-01 Source Review 01

Reviewer: Lead Engineer/Reviewer — Codex at High

Decision: **REJECTED — FOCUSED CORRECTION 02 REQUIRED**

Reviewed unstaged drop:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/xmr.rs` | 8 | `dbcb6133b19f92bc0b0d99aa6ec82d7a55400f553b85c258d583a6584726c7ff` |
| `wallet-broker/src/xmr/model.rs` | 214 | `2a2d3ba1ce453aca65138df402bde3e7f1fee997d5d069024cb1beb8102152cb` |
| `wallet-broker/src/xmr/account.rs` | 3,334 | `700b153a1387936c53ae7540d9f8f94f395b4aa849e3be951ff8bd60030982f6` |
| `wallet-broker/src/xmr/process.rs` | 1,968 | `ad9d77bbc73cc2e19075fb0b488ddc9961f8dfac521f80f06f431aa08843cd42` |
| `wallet-broker/src/xmr/rpc.rs` | 2,582 | `302a0d79869df8310973de86784ac138bb49400c174d71c2f15eee3dfd311c55` |
| `wallet-broker/src/xmr/store.rs` | 1,916 | `b3e66a34571a1801431956f526fef33b923eef645c13c099904dedbad922b018` |
| `wallet-broker/src/xmr/receiver.rs` | 870 | `cc3b001d680aa9d659f8cd43e7312349a6ee0e4fa965063cd66a78f0a108619c` |
| `wallet-broker/src/xmr/test_support.rs` | 6,034 | `1339a5c0fc5b710d0b679315f5ac0fa96f8aa9713fcfff0f7242874455fd9ad0` |

The frozen `wallet-broker/tests/xmr_receiver.rs` remains byte-exact at 588 lines and
SHA-256 `d8656c533d7b2d08eb9fa8831cd70da6e1da632ed9563fc2af569cab754d3622`.
The reviewer ran read-only inspection plus `git diff --check`; no formatter, compiler,
test, Clippy, build, product binary, Node/npm, policy, Git mutation, or network command
was run.

## Blocking findings

1. The production view still cannot represent the required independent unavailable and
   locked states. `SystemAccountPort::production_view` returns `LOCKED` before probing
   the node, requires an owned child before any view can be built, propagates
   `NODE_UNAVAILABLE`, and always supplies `wallet_available=true` and
   `wallet_locked=false`. Consequently the live callable surface can emit only the
   syncing/ready node and refreshing/ready wallet subset. The frozen independence test
   passes caller-scripted states through `ViewRig`; it does not repair this production
   disconnect. `SystemAccount::view` must distinguish ordinary node and wallet
   unavailability from integrity/protocol failures and construct the sanitized snapshot
   without inventing authority. Locked viewing must not call wallet RPC, and no state
   branch may weaken identity, store, local-node, hard-fork, or balance validation.
2. Failed `ReceiverRig::open` can delete a caller-selected directory. It accepts a raw
   `PathBuf` and calls `cleanup_receiver_root` on error; that function proves only the
   target parent, a public filename prefix, and current nonsymlink directory metadata.
   It has no provenance that this invocation created or received cleanup ownership for
   that exact leaf, and validation remains separate from recursive removal. Replace the
   raw close/open path transfer with an opaque owned lease/guard: `close` transfers the
   exact generated leaf's cleanup ownership, `open` consumes that ownership, failure
   drops only that same lease, success moves it into the reopened rig, and an abandoned
   closed lease also cleans up. Do not expose an arbitrary-path cleanup entry point or
   follow symlinks.

Correction 01 otherwise materially fixes the disconnected production port, real store
fault staging, complete durable binding proof, lifetime fail-closed latch, FULL-drift
handling, model-module boundary, and target-backed collision resistance. Preserve those
repairs and every Slice-1–4 invariant. Principal Dev — Codex Sol at High may make only
the focused Correction 02. Hermes execution/integration and all tests remain
unauthorized pending reviewer source acceptance.
