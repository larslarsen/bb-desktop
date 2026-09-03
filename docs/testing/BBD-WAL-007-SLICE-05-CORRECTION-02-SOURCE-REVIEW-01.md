# BBD-WAL-007 Slice-5 Correction-02 Source Review 01

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Decision: **REJECTED — ONE-FILE CORRECTION 03 REQUIRED**

Reviewed unstaged drop:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/xmr.rs` | 8 | `dbcb6133b19f92bc0b0d99aa6ec82d7a55400f553b85c258d583a6584726c7ff` |
| `wallet-broker/src/xmr/model.rs` | 214 | `2a2d3ba1ce453aca65138df402bde3e7f1fee997d5d069024cb1beb8102152cb` |
| `wallet-broker/src/xmr/account.rs` | 3,374 | `14c41baa8e276e21e0405aab419454cb476e3e299fd09c4016df09e50b9bc5a6` |
| `wallet-broker/src/xmr/process.rs` | 1,968 | `ad9d77bbc73cc2e19075fb0b488ddc9961f8dfac521f80f06f431aa08843cd42` |
| `wallet-broker/src/xmr/rpc.rs` | 2,582 | `302a0d79869df8310973de86784ac138bb49400c174d71c2f15eee3dfd311c55` |
| `wallet-broker/src/xmr/store.rs` | 1,916 | `b3e66a34571a1801431956f526fef33b923eef645c13c099904dedbad922b018` |
| `wallet-broker/src/xmr/receiver.rs` | 870 | `cc3b001d680aa9d659f8cd43e7312349a6ee0e4fa965063cd66a78f0a108619c` |
| `wallet-broker/src/xmr/test_support.rs` | 6,027 | `c83fa81b0bfbec811e1b1a9c254c2f786df3b5ed3739f1be9bd7e2ac42ee62e8` |

The frozen receiver test remains byte-exact at 588 lines and SHA-256
`d8656c533d7b2d08eb9fa8831cd70da6e1da632ed9563fc2af569cab754d3622`.
`git diff --check` is clean. The reviewer ran read-only source inspection only; no
formatter, compiler, test, Clippy, build, product binary, Node/npm, policy, Git mutation,
or network command was run.

## Blocking finding

`SystemAccountPort::production_view` correctly maps a rejected local-node probe to
`NodeState::Unavailable`, but then—unless the wallet is locked—continues into
`prove_owned_session`, `refresh`, `get_height`, and `get_balance`. `NODE_UNAVAILABLE`
also represents an untrusted/bootstrap/wrong-network/malformed node, not only a stopped
node. Driving wallet RPC refresh after that verdict violates the ticket's fail-closed
local non-bootstrap-node boundary and can let the owned wallet consume state through a
daemon currently using a rejected bootstrap source. It can also label the wallet
`READY` when no accepted node height exists.

When the accepted node probe is unavailable, the view may still report the independent
locked state without wallet RPC; otherwise it must report wallet unavailable and omit
wallet height/balance acquisition. No wallet RPC may occur until the node gate has
returned an accepted syncing or ready observation. Preserve the exact state/error
classification and all other Correction-01/02 repairs.

The opaque `ReceiverRootLease` correction is accepted: callers cannot manufacture the
cleanup target, close transfers the existing guard, failure/abandonment drops that exact
guard, and success moves it into the reopened rig.

Principal Dev — Codex Sol at High may make the exact one-file Correction 03. Hermes
execution/integration remains unauthorized pending reviewer source acceptance.
