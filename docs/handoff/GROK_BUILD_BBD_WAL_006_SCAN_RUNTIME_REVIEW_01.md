# Grok Build Handoff — BBD-WAL-006 Scan Runtime Review 01

You are **Sr Dev — Grok Build**, using Grok 4.6 High. This is a read-only corrective design review.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely: `AGENTS.md`, `TESTING.md`, `tickets/BBD-WAL-006.md`, Scan Atomicity Review 01,
Scan Truth Correction Review 01, Scan Gate Runtime Review 01, the complete frozen `zec_scan` test,
the five current ZEC source files, and the pinned upstream implementations of:

- `shardtree 0.7.1` current/checkpoint root access;
- `zcash_client_sqlite 0.22.0` truncation and wallet summary balance selection;
- `ExtensionTransaction` authorization.

## Sole task

Return a static design verdict and exact recommended correction shape for all three observed
issues. Do not edit any file. Do not run a formatter, compiler, Clippy, test, Node, policy, Git,
network, fixture, wallet, node, device, cleanup, or deletion command.

The design must answer:

1. Whether replacing exact-height `root_at_checkpoint_id` calls with current-tree
   `root_at_checkpoint_depth(None)` calls for Sapling, Orchard, and Ironwood is correct after the
   wallet tip has been read from the same opened wallet, including empty-tree behavior and the
   nested Ironwood result shape.
2. How `inspect_with_params` can return main-chain-only per-pool and total balances after an
   official one-block truncation while retaining `WalletRead::get_wallet_summary` as the source
   of spendability/confirmation truth, performing read-only queries only, excluding the orphaned
   unmined received value exactly once, checking every subtraction/addition, and failing closed on
   an ambiguous or inconsistent state. Do not propose wallet-table writes, deletion, a lint
   suppression, fixture-specific constants, or test-only branching.
3. The exact minimal correction for the unused `accepted` binding at frozen test line 302 without
   changing the below/at/above boundary assertions or adding a lint allowance.
4. The exact source/test paths and helper-level changes Sol should receive, plus any new static
   invariant needed to prevent subtracting an unmined note that the official summary has already
   excluded.

Explicitly label the verdict `CORRECT`, `CORRECT WITH CHANGES`, or `INCORRECT`. Separate confirmed
upstream behavior from your inference. The reviewer will decide the semantics and publish a
source-authoring handoff; this review itself authorizes no edit.
