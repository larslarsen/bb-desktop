# BBD-WAL-006 Scan Runtime Source Review 01

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Governance parent: `9898b6a7`

Result: **CORRECT WITH ONE FAIL-CLOSED SOURCE CHANGE**

Sol's two-path drop matches the corrected stable-window design. The test edit is exactly the
authorized unused-binding replacement. Production uses one guarded read-only connection, loads
the required array module, samples `PRAGMA main.data_version` around every observation, scopes
official `WalletDb::from_connection` reads without nesting, queries one exact aggregate row,
performs checked pending-capacity subtraction and total reconciliation, and computes all three
current roots with the pinned current-root API.

One bounded source defect remains. The aggregate's official unexpired-spend exclusion correctly
uses an inner join to `transactions`, matching upstream. Its independent `malformed` detector
also uses an inner join, however, so a matching `v_received_output_spends` row whose referenced
spending transaction is absent disappears from the corruption check. SQLite `quick_check` does
not substitute for this relational check. This contradicts the required fail-closed handling of
missing/ambiguous transaction data.

Change only that independent malformed-spend subquery to left-join the spending transaction and
treat `spending_tx.id_tx IS NULL` as malformed before the existing height checks. Preserve the
official `NOT EXISTS` predicate's inner join exactly. No other source or test edit is authorized.

Accepted pre-correction identities:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/tests/zec_scan.rs` | 325 | `87ed1c3e8db8219ad126efb4de7f681cf909cb404b88df6c2dfadec471a3701f` |
| `wallet-broker/src/zec/scan.rs` | 1,665 | `10fda0c090d66159e5266fee5e2545d150b23d953f455dc82c916485ba49eee5` |

The four frozen supporting source identities remain exact and `wallet-broker/src/zec/prepare.rs`
remains absent. Hermes stays paused until the corrected source is returned and accepted.
