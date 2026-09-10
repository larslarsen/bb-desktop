# WAL-015 reorg received-balance correction
Principal Sol High core actor authorized after actual RUNTIME02 reorg assertion RED:
pending150000000 versus120000000; original30M output retained upstream as unmined.
RUNTIME03 transport3/validation2/recovery1 passed and all pins agree, CLI exited0.
Source only: live.rs and minimal pub(crate) visibility for scan.rs orphan_projection,
OrphanProjection struct and orchard/ironwood fields. No other scan changes.
Use official summary plus existing read-only orphan projection, scoped to actual
WalletDb account, scanned tip+1 (checked), via validated live path after commit.
Require per-pool orphan <= official pending capacity; checked subtract from pool total
and pending, then checked combine Orchard/Ironwood. Keep all upstream rows, 120M test.
No execution/formatting/tests/Git/evidence/dependency/user-data changes. Freeze on drop.
