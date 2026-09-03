# BBD-WAL-007 Slice-5 Correction-04 Source Review 01

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Governance parent: `fe5ba6c7`

Result: **ACCEPTED FOR HERMES FOCUSED GREEN**

No formatter, compiler, test, Clippy, build, product binary, Node/npm,
policy/security, package-manager, staging, commit, push, or network command was run by
the reviewer. `HEAD == origin/master == fe5ba6c7`, the index is clean, the worktree
contains exactly the accepted eight-path Slice-5 source drop, the tracked diff passes
`git diff --check`, and the untracked `receiver.rs` addition independently passes the
equivalent no-index whitespace check. The frozen 588-line receiver test remains
byte-exact at SHA-256
`d8656c533d7b2d08eb9fa8831cd70da6e1da632ed9563fc2af569cab754d3622`.

## Accepted identities

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/xmr.rs` | 8 | `dbcb6133b19f92bc0b0d99aa6ec82d7a55400f553b85c258d583a6584726c7ff` |
| `wallet-broker/src/xmr/model.rs` | 214 | `2a2d3ba1ce453aca65138df402bde3e7f1fee997d5d069024cb1beb8102152cb` |
| `wallet-broker/src/xmr/account.rs` | 3,374 | `8ab5650246afc1a657a91b7b013aa1c79995ee60ce4d78e0a34404db0adb05f6` |
| `wallet-broker/src/xmr/process.rs` | 1,968 | `ad9d77bbc73cc2e19075fb0b488ddc9961f8dfac521f80f06f431aa08843cd42` |
| `wallet-broker/src/xmr/rpc.rs` | 2,582 | `302a0d79869df8310973de86784ac138bb49400c174d71c2f15eee3dfd311c55` |
| `wallet-broker/src/xmr/store.rs` | 1,916 | `b3e66a34571a1801431956f526fef33b923eef645c13c099904dedbad922b018` |
| `wallet-broker/src/xmr/receiver.rs` | 870 | `fb1ab7ff4210a09612de450b2ed5650f215b2d2a8ca20c868bc16b9e025ca23e` |
| `wallet-broker/src/xmr/test_support.rs` | 6,027 | `c83fa81b0bfbec811e1b1a9c254c2f786df3b5ed3739f1be9bd7e2ac42ee62e8` |

## Correction-04 scope and semantics

All seven pre-existing source identities are byte-exact to Correction 03. Moving the
single `port.prove_owned_identity()?` line in the accepted 870-line `receiver.rs` from
its current line 675 back to its former line 648 reconstructs the authorized starting
SHA-256 exactly:
`cc3b001d680aa9d659f8cd43e7312349a6ee0e4fa965063cd66a78f0a108619c`.
Correction 04 therefore consists exactly of the authorized one-line relocation.

`issue_fresh` now validates the caller account ID, request ID, and closed network and
matches them to the owned account before store access. It preserves the receiver
lifetime-unavailable latch, loads and validates the complete durable account identity,
and performs the exact durable request lookup. A matching binding returns immediately
from that authenticated store path without requiring wallet lock state, a live child,
or an owned wallet-RPC session.

Only after a durable lookup miss does the core call `prove_owned_identity`. The
production implementation retains the unavailable and locked checks and proves the
exact account/network session against the account-owned child. That gate remains before
`prepare_receiver`, local-node/eligibility decisions, `create_address`, address
classification/equality RPCs, exhaustion-sensitive issuance, and persistence. New
issuance therefore retains every accepted node, wallet, session, validation, durability,
consumed-gap, and fail-closed rule.

`SystemAccount::fresh_receiver`'s earlier `AccountManager::require_available` checks
only the account manager's lifetime reconciliation latch; it does not require a child,
an unlocked wallet, or an RPC session. Likewise, the receiver authority latch protects
uncertain durable state rather than ordinary child availability. Neither pre-return
check reintroduces the rejected live-child replay dependency.

This is source acceptance, not execution evidence. Formatter, compiler, focused test,
warning-denied Clippy, native, policy, integration, broader/final acceptance, and the
real offline local-Monero gate remain unproved. Hermes alone may run the exact linked
focused-green handoff and integrate only on exact success.
