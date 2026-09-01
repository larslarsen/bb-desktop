# BBD-WAL-006 Scan Runtime Snapshot Review 01

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Governance parent: the commit containing this review

Result: **CORRECTED STABLE OBSERVATION WINDOW — BOUNDED SOL CORRECTION AUTHORIZED**

This review supersedes only the impossible `transactionally_with_extension` inspection clause in
Scan Runtime Design Review 01 and Runtime Correction 01. Their current-root, exact orphan
selection, checked-balance, fail-closed, and test-binding requirements remain authoritative.

## Pinned API basis

- Rusqlite 0.37 `Connection::unchecked_transaction` rejects nested transactions.
- `zcash_client_sqlite` 0.22 `WalletRead::get_wallet_summary` owns such a transaction.
- `WalletDb::from_connection` is public for any connection implementing `Borrow<Connection>`;
  the caller must first load rusqlite's array module.
- The pinned SQLite implementation states that `PRAGMA data_version`, compared twice on the same
  connection, detects intervening commits made through other connections and omits changes made
  by the sampled connection itself.

The omission is safe here because the sampled connection is opened with
`SQLITE_OPEN_READ_ONLY`. All official wallet operations and the application projection use that
same read-only connection sequentially. The account's existing `gate` mutex excludes mutations
through its in-process handles, and a commit by any other SQLite connection during inspection
changes the sampled data version. A changed version, query/transaction error, or malformed value
fails `STATE_CORRUPT`; inspection does not retry or mix observations.

## Correct stable-window algorithm

After mandatory recovery and path validation:

1. Open exactly one wallet `Connection` through the existing guarded read-only opener. Load
   `rusqlite::vtab::array` on it. Do not begin an outer transaction.
2. Read `PRAGMA main.data_version` as the before value on that connection.
3. On that same connection, validate the complete V1 extension/account/network binding and read
   the stored UFVK and receiver sequence. No helper may open a second connection for these values.
4. Temporarily wrap `&mut Connection` with `WalletDb::from_connection`. Through official
   `WalletRead`/tree APIs, read the sole account and its purpose/UFVK, wallet tip/hash, current
   Sapling/Orchard/Ironwood roots, and official `get_wallet_summary`. Drop the wrapper before the
   application projection. Each upstream-owned read transaction must finish before the next
   operation; never nest it in an application transaction.
5. On the original connection, execute one aggregate SELECT for the same official account UUID,
   target height, and orphan values. Then read `PRAGMA main.data_version` again.
6. Accept the assembled inspection only if before equals after. That equality supplies a stable
   database-state window across all sequential official and application reads. The second sample
   is the inspection's linearization point; a later commit is later state, not a mixed result.

The empty-account and hidden balance-override branches must also complete their extension,
official-account, tip/root where applicable, and data-version checks before returning. They may
skip only the official summary/orphan adjustment that their existing semantics do not use.

## Exact main-chain projection

The projection is read-only and returns one aggregate row. It binds the official account UUID
from `get_account_ids`, checked `target_height = wallet_tip + 1`, and values converted from the
upstream `DEFAULT_TX_EXPIRY_DELTA` and `zip317::MARGINAL_FEE` constants. It does not copy either
numeric constant.

For `v_received_outputs` pool 2, 3, or 4 joined to `accounts` and its creating `transactions` row,
an orphan is selected only when all of these are true:

- the account UUID is the official account UUID;
- the creating transaction has `mined_height IS NULL`;
- it is unexpired by `expiry_height = 0`, `expiry_height >= target_height`, or unknown expiry with
  checked/bounded SQL equivalence to
  `min_observed_height + DEFAULT_TX_EXPIRY_DELTA >= target_height`;
- the individual value is strictly greater than `zip317::MARGINAL_FEE`; and
- no matching row in `v_received_output_spends`, by account, pool, and received-output ID, joins
  to a spending transaction satisfying the complete pinned unexpired predicate: mined below
  target, expiry zero, expiry at/above target, or the same unknown-expiry branch.

The same aggregate statement must separately expose any same-account unmined transparent output
and any unknown pool code; either condition fails closed. Negative values/counts, SQLite integer
overflow, conversion failure, or more/fewer than the expected aggregate row also fail closed.

Each shielded orphan sum must fit within that pool's checked official
`change_pending_confirmation + value_pending_spendability`. Subtract it only from the displayed
pool total. Spendable and locked official values remain unchanged. Ironwood pending is the
checked adjusted Ironwood total minus official Ironwood spendable. The displayed account total is
both the checked sum of adjusted display components and checked official account total minus the
three orphan sums; disagreement is `STATE_CORRUPT`.

## Other retained corrections

`official_tree_digest` uses `root_at_checkpoint_depth(None)` for current Sapling, Orchard, and
Ironwood roots. Sapling and Orchard retain their missing-root checks. Ironwood retains both its
outer optional-tree and inner missing-root checks. Its obsolete height argument is removed.

The sole test-source correction remains replacing the unused first maximum-balance loop binding
`accepted` with `_`. No case, assertion, test count, or other test token changes. Sol may edit only
the two paths named in the active handoff and may not execute tests or Git.
