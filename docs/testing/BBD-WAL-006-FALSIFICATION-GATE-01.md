# BBD-WAL-006 Falsification Gate 01

Ticket: BBD-WAL-006

State: FALSIFICATION GATE 01 COMPLETED

Execution date: 2026-09-01

Hermes identity: Hermes Agent v0.18.2 (2026.7.7.2) · upstream f709bd88 · local 10b6d1a9 (+1 carried commit); provider `nous`; model `meituan/longcat-2.0:free`

Repository: `/home/lars/OpenBazaar/bb-desktop`

Preconditions:
- HEAD == origin/master: `ad7c0f02d4e782d7445731164946ac2520b14931`
- Clean worktree/index and clean `git diff --check`
- ext4 filesystem confirmed on `/home/lars`

Baseline file identities (all verified byte-for-byte before and after):

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/zec/prepare.rs` | 964 | `3cb1096107386bfcd7f30169ade545ec8a7073875dc1d8e631dd0d5104d7c08e` |
| `wallet-broker/src/zec/scan.rs` | 1,661 | `54a890fd0b5a8ca8ce78dfb59c13424407341df25e47a5eacd2de8d6cc3317ad` |
| `wallet-broker/tests/zec_prepare.rs` | 416 | `c38339ab88a954f725c7341b4384f178078116de1c700e16892409c18eb2f3fa` |
| `wallet-broker/tests/zec_scan.rs` | 325 | `87ed1c3e8db8219ad126efb4de7f681cf909cb404b88df6c2dfadec471a3701f` |
| `wallet-broker/tests/zec_hygiene.rs` | 375 | `aad7c95a2ef661063661f2ec0f16a216d80328096b049d636b88fa0252ba1be6` |

## Falsification 1 — accept transparent/Sapling UA composition

Mutation: removed `has_transparent()` downgrade guard and `has_sapling()` composition guard in `wallet-broker/src/zec/prepare.rs`.

Command:
```
TMPDIR="$PWD/wallet-broker/target/wal006-tmp" CARGO_TARGET_DIR="$PWD/wallet-broker/target/wal006-cargo" /home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_prepare receiver_network_and_composition_reject_downgrade_without_fallback -- --exact
```

Result: exit 101. Failed test: `receiver_network_and_composition_reject_downgrade_without_fallback` (panicked at tests/zec_prepare.rs:246:14 — `Result::unwrap_err()` on `Ok` value). 0 passed, 1 failed.

Restoration: exact reverse patch applied; `prepare.rs` restored to `3cb1096107386bfcd7f30169ade545ec8a7073875dc1d8e631dd0d5104d7c08e`; worktree clean.

## Falsification 2 — bypass previous-hash continuity

Mutation: dropped `block.prev_hash != blocks[index - 1].hash` chain-continuity check and replaced `derive_chain_states(...)?.` propagation with a `PROTOCOL_INCOMPATIBLE` swallow path in `wallet-broker/src/zec/scan.rs`.

Command:
```
TMPDIR="$PWD/wallet-broker/target/wal006-tmp" CARGO_TARGET_DIR="$PWD/wallet-broker/target/wal006-cargo" /home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_scan malformed_discontinuous_wrong_branch_and_impossible_tree_fail_without_tip_advance -- --exact
```

Result: exit 101. Failed test: `malformed_discontinuous_wrong_branch_and_impossible_tree_fail_without_tip_advance` (panicked at tests/zec_scan.rs:208:75 — `Result::unwrap_err()` on `Ok(())`). 0 passed, 1 failed.

Restoration: exact reverse patches applied; `scan.rs` restored to `54a890fd0b5a8ca8ce78dfb59c13424407341df25e47a5eacd2de8d6cc3317ad`; worktree clean.

## Falsification 3 — mark Orchard value spendable

Mutation: changed `return Err(ZecError::migration_required())` to `return Ok(())` for the Orchard+Ironwood-spendable branch in `wallet-broker/src/zec/prepare.rs`.

Command:
```
TMPDIR="$PWD/wallet-broker/target/wal006-tmp" CARGO_TARGET_DIR="$PWD/wallet-broker/target/wal006-cargo" /home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_prepare pool_outcome_table_never_substitutes_total_for_ironwood_spendable -- --exact
```

Result: exit 101. Failed test: `pool_outcome_table_never_substitutes_total_for_ironwood_spendable` (panicked at tests/zec_prepare.rs:318:45 — `Result::unwrap_err()` on `Ok(PreparedZecV1)`). 0 passed, 1 failed.

Restoration: exact reverse patch applied; `prepare.rs` restored to `3cb1096107386bfcd7f30169ade545ec8a7073875dc1d8e631dd0d5104d7c08e`; worktree clean.

## Falsification 4 — report a non-v6 artifact

Mutation: changed `tx_version: "6".to_owned()` to `tx_version: "5".to_owned()` in `wallet-broker/src/zec/prepare.rs`.

Command:
```
TMPDIR="$PWD/wallet-broker/target/wal006-tmp" CARGO_TARGET_DIR="$PWD/wallet-broker/target/wal006-cargo" /home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_prepare sufficient_confirmed_ironwood_prepares_sanitized_v6_handle_and_decoded_pczt -- --exact
```

Result: exit 101. Failed test: `sufficient_confirmed_ironwood_prepares_sanitized_v6_handle_and_decoded_pczt` (assertion `left == right` failed, left: `"5"`, right: `"6"` at tests/zec_prepare.rs:96:5). 0 passed, 1 failed.

Restoration: exact reverse patch applied; `prepare.rs` restored to `3cb1096107386bfcd7f30169ade545ec8a7073875dc1d8e631dd0d5104d7c08e`; worktree clean.

## Falsification 5 — retain prepared handle across lock

Mutation: added early `return` when `edge == HandleInvalidation::Lock` at the top of `invalidate_inner` in `wallet-broker/src/zec/prepare.rs`.

Command:
```
TMPDIR="$PWD/wallet-broker/target/wal006-tmp" CARGO_TARGET_DIR="$PWD/wallet-broker/target/wal006-cargo" /home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_hygiene every_named_lifecycle_edge_invalidates_handle_and_wipes_prepared_state -- --exact
```

Result: exit 101. Failed test: `every_named_lifecycle_edge_invalidates_handle_and_wipes_prepared_state` (panicked at tests/zec_hygiene.rs:109:9 — `edge Lock`). 0 passed, 1 failed.

Restoration: exact reverse patch applied; `prepare.rs` restored to `3cb1096107386bfcd7f30169ade545ec8a7073875dc1d8e631dd0d5104d7c08e`; worktree clean.

## Final state

- No mutation bytes committed.
- Worktree/index clean; `git diff --check` clean.
- All five source identities match the accepted baseline.
- Hermes Agent v0.18.2, provider `nous`, model `meituan/longcat-2.0:free`.
