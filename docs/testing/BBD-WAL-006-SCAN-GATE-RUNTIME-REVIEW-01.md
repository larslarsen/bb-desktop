# BBD-WAL-006 Scan Gate Runtime Review 01

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Governance parent: `639ec316b722359ee67fabcec8faf9589cd05217`

Result: **RUNTIME STOP — READ-ONLY GROK DESIGN REVIEW AUTHORIZED**

Hermes restarted Scan Gate 01 from every protected precondition. Rust 1.98.0 formatter passed
with exit 0. Locked/offline/no-default library Clippy with `-D warnings` passed with exit 0 and no
diagnostic. `zec_scan` exited 101 with 7 passed and two failed:

1. `birthday_continuity_confirmation_and_unrelated_output_are_non_vacuous` reached the first
   post-scan balance inspection and received `STATE_CORRUPT` at test line 145.
2. `supported_one_block_reorg_rolls_back_exact_effects_and_applies_replacement` observed
   Ironwood pending value `150000000` instead of `120000000` at test line 242.

The test build also emitted `unused_variables` for the first loop's `accepted` binding at
`zec_scan.rs:302`. Hermes correctly stopped. No store/address test, Node policy command,
diagnostic command, evidence, edit, staging, commit, or push followed. The five accepted source
hashes remain exact and `git diff --check` passes.

## Confirmed upstream mechanics

Read-only inspection of pinned `shardtree 0.7.1` confirms that `root_at_checkpoint_id(height)`
returns `Ok(None)` when that exact checkpoint is absent; absence is not database corruption.
`root_at_checkpoint_depth(None)` instead computes the current root over all leaves and returns the
empty root for an empty tree. The wallet tip and all three trees are updated in the same official
wallet transaction, so the current-root API is the appropriate truth for post-scan inspection.

Read-only inspection of pinned `zcash_client_sqlite 0.22.0` confirms that
`truncate_to_chain_state` sets displaced transactions to unmined and intentionally retains their
transaction and note data. Its official `get_wallet_summary` includes unexpired unmined received
notes in pending balance. The observed `150000000` is therefore consistent with retained victim
`30000000` plus replacement `120000000`, but violates BitBook's frozen main-chain-only inspection
contract. `ExtensionTransaction` expressly denies writes to wallet-owned tables; direct deletion
or update is not authorized.

The reviewer does not yet authorize source or test edits. Grok must provide a read-only design for
current-tree digest extraction and a fail-closed main-chain-only balance projection that preserves
official wallet authority and performs no wallet-table mutation. It must also assess the exact
no-semantic-change correction for the unused test binding. Sol remains the prospective author for
the high-risk wallet source/test correction after reviewer semantics are fixed.
