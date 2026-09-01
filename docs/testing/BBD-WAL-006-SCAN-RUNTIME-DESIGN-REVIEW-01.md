# BBD-WAL-006 Scan Runtime Design Review 01

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Governance parent: `e68d5db3`

Result: **CORRECT WITH REVIEWER CHANGES — BOUNDED SOL CORRECTION AUTHORIZED**

Grok's first read-only invocation stalled after loading the pinned sources and was interrupted
without repository mutation. A narrower read-only invocation returned `CORRECT WITH CHANGES`.
The reviewer accepts its high-level conclusions: use current tree roots, preserve official wallet
summary as confirmation/spendability truth, remove retained orphan value through a read-only
main-chain projection, use checked arithmetic, perform no wallet-table writes, and make the
unused-binding test correction without suppression.

Two Grok details are rejected and superseded here. First, shardtree 0.7.1
`root_at_checkpoint_depth(None)` returns `Result<Option<H>, _>`, not `Result<H, _>`; an empty tree
produces `Some(empty_root)`. Sapling and Orchard therefore retain one fail-closed missing-root
check. Ironwood retains its outer optional-tree check and inner missing-root check. An absent
Ironwood tree is not silently replaced by an empty root.

Second, the proposed SQL sketch was not equivalent to pinned wallet-summary selection. The
authoritative query must use the pinned `v_received_outputs` and `v_received_output_spends` views,
the same account UUID and target height, the exact `>=` expiry boundary, the unknown-expiry
`DEFAULT_TX_EXPIRY_DELTA` branch, the exact unexpired-spend exclusion, and the ZIP-317 marginal-fee
boundary. This proves every subtracted shielded value is already present in official summary
`Balance::total()`. Any unmined transparent received output is fail-closed because this compact
scanner does not establish an equivalent transparent-summary projection.

## Fixed projection invariants

- Read wallet tip, current roots, official account summary, and orphan projection from one
  `transactionally_with_extension` snapshot; the extension handle performs SELECT only.
- `target_height` is checked `wallet_tip + 1`, matching the summary target.
- An orphan row is same-account, shielded pool 2/3/4, transaction unmined, exactly unexpired,
  economically relevant (`value > zip317::MARGINAL_FEE`), and not spent by any transaction the
  pinned unexpired-spend predicate recognizes.
- For each shielded pool, orphan value must be no greater than checked
  `change_pending_confirmation + value_pending_spendability`. Spendable and locked values are
  unchanged. Main-chain pool total is checked `summary_pool.total - orphan_value`.
- Ironwood pending is checked `main_chain_ironwood_total - summary_ironwood_spendable`.
  Transparent total is unchanged only when the same snapshot proves zero unmined transparent
  received outputs.
- The displayed total is the checked sum of adjusted per-pool totals and unchanged transparent
  total, and must equal checked `summary_account.total - sum(shielded_orphans)`.
- Any negative database value, conversion failure, unknown pool, overflow, underflow, predicate
  ambiguity, or invariant mismatch returns `STATE_CORRUPT`.

The existing two failing runtime tests are the test-first falsification for the production
correction. The test-source change is limited to replacing the unused `accepted` binding in the
first maximum-balance loop with `_`; its cases and assertions remain unchanged. Sol may author
only the two paths in the active handoff and must not execute tests or Git.
