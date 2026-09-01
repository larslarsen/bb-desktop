# BBD-WAL-006 Store Gate 01 Evidence

Execution timestamp: 2026-08-31 18:15:00 UTC (recorded at integration)

Protected governance parent: `2d2a52ef619aecab2fc1a29b6287b8c1aecfb8b5` (commit containing Store Gate Resume 05)

Filesystem type: ext4 (disk-backed, /dev/mapper/ubuntu--vg-ubuntu--lv)

Exact ignored disk-backed paths (created/present, not /tmp):
- /home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-tmp
- /home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-cargo

Hermes identity:
- Version: Hermes Agent v0.18.2 (2026.7.7.2) · upstream b20cc5f7 · local 10b6d1a9 (+1 carried commit)
- Provider: nous
- Model: meituan/longcat-2.0:free

## Protected preconditions (all exact)

HEAD == origin/master == 2d2a52ef619aecab2fc1a29b6287b8c1aecfb8b5, clean index, worktree changes only:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| wallet-broker/src/zec.rs | 214 | 800111bf587265b49764de71c6d2beb054d1906932005bb3513b2d4fc132d90e |
| wallet-broker/src/zec/fixture.rs | 280 | 01c4fa4b83b0b5f4501db966c27e07c2d3f7d88148daac1865bc35e67044c698 |
| wallet-broker/src/zec/store.rs | 1,700 | 779f847a328a8fe85ca7a951a67d6be12403ec3f73b9557c943c4e404742052f |
| wallet-broker/src/zec/test_support.rs | 825 | e37f486b7dceab7d35fc1d0385ca17d82d5082e77732edeaca727e8903592346 |
| wallet-broker/tests/zec_store.rs | 324 | 1c230a2a9cf51c841a0df6514393861387422e5d0b2a83e80af47022728e2225 |

Production source totals 3,019 lines.

Eight non-source protected identities (all exact):

| Protected path | Lines | SHA-256 |
| --- | ---: | --- |
| wallet-broker/tests/zec_address.rs | 277 | 2c5012e6884c8c2a81236266c6861b6e4e4fd6b124656dad2ab438add5848ee3 |
| wallet-broker/src/lib.rs | 11 | ad61f92cce12115df9da6f263240ec0c57b5746e79cdcf6459f03731dab45617 |
| wallet-broker/src/zec/address.rs | 204 | d9d54e301d188e3d4cb539095c5c01bcb07aaf011fcb1a50129aadc18f24cbfe |
| wallet-broker/tests/fixtures/zec/manifest.json | 238 | 0c5836405b29a2af618a9fa2784a973c981e807c3a8d46471aeeb90d8a6fe389 |
| wallet-broker/Cargo.toml | 81 | 6643435bdf59608b09e906c8b7010baf1c17bbaa785d8eb70e37039d4bb37632 |
| wallet-broker/Cargo.lock | 5,369 | ac454889b0796afea3f2a2ddfaf8c585bfff1080bec6d1428d0c227534ffb9dd |
| scripts/security-policy.js | 2,299 | 60e41a12462d77c6be875f1659e5ef8a86d2b8146bd25d37ec7777297847d767 |
| test/securityPolicy.node.js | 2,454 | f8340ae965a1f59e27594153cd3c9f3eca5ee1d8a78ed0a9a685faf6f1dc2647 |

Source-only and whole-worktree `git diff --check`: both exit 0, no conflict markers or whitespace errors.

## Exact execution (each run once, in order, no network)

1. `cargo +1.98.0 fmt --manifest-path wallet-broker/Cargo.toml --check`
   - Exit: 0, no mutation, no diagnostic.

2. `cargo +1.98.0 clippy --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --lib -- -D warnings`
   - Exit: 0, no warning or diagnostic.

3. `cargo +1.98.0 test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_store`
   - Exit: 0. Exactly 8 passed, 0 failed, 0 ignored, 0 measured, 0 filtered out.
   - Tests: symlink_nonregular_and_wrong_mode_state_are_rejected_without_replacement, store_limits_cover_immediate_below_at_and_above_before_allocation, initialization_and_reopen_bind_exact_account_network_and_schema, sqlite_paths_are_closed_account_network_derived_and_linux_private, sqlite_schema_and_rows_contain_viewing_state_but_no_spend_secrets, failed_write_file_sync_and_directory_sync_never_report_durable_state, schema_migration_is_atomic_across_write_sync_and_commit_failures, corrupt_wrong_schema_and_truncated_sqlite_fail_closed_without_empty_recreation.

4. `cargo +1.98.0 test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_address`
   - Exit: 0. Exactly 8 passed, 0 failed, 0 ignored, 0 measured, 0 filtered out.
   - Tests: seed_is_wiped_on_success_error_cancellation_replacement_unwind_and_drop, account_network_and_mainnet_validation_precede_database_or_derivation, fresh_receiver_decodes_to_exactly_one_orchard_protocol_receiver, two_concurrent_issuers_serialize_one_account_without_duplicates, unsupported_receiver_composition_never_falls_back, receiver_issuance_is_monotonic_durable_and_viewing_only_after_reopen, coupled_receiver_state_write_failure_returns_nothing_and_advances_neither_record, receiver_limits_cover_immediate_below_at_and_above_without_wrap.

5. `node test/securityPolicy.node.js`
   - Exit: 1. Exactly 68 `ok`, exactly 6 `not ok`, final line `6 security policy test(s) failed`.
   - The six failing groups (exact frozen names):
     1. committed workflows satisfy the fail-closed checker
     2. strict nine-line reviewed Gitleaks ratchet bytes and content are enforced
     3. WAL-004 Rust source inventory is exported closed and enumerated by repository policy
     4. WAL-006 feature policy distinguishes compiled upstream PCZT capability from BitBook authority
     5. WAL-006 requires the exact bounded Phase-C ZEC production inventory
     6. WAL-006 policy rejects live-network and authority-bearing Rust snippets without denying upstream transitives
   - The first three are the accepted untransitioned inventory groups; the last three are the accepted deferred WAL-006 policy groups. No other failure, warning, exception, or source-policy finding.

## Behaviors proven by the eight store tests

- v0/v1 migration: schema_migration_is_atomic_across_write_sync_and_commit_failures proves migration is atomic across write/sync/commit failures.
- Corrupt-state read-only preflight: corrupt_wrong_schema_and_truncated_sqlite_fail_closed_without_empty_recreation proves corrupt/wrong-schema/truncated SQLite fails closed without empty recreation.
- Hostile-entry rejection: symlink_nonregular_and_wrong_mode_state_are_rejected_without_replacement proves symlink/nonregular/wrong-mode state are rejected without replacement.
- Store durability: failed_write_file_file_sync_and_directory_sync_never_report_durable_state proves failed write/file-sync/directory-sync never report durable state.
- Secret-exclusion: sqlite_schema_and_rows_contain_viewing_state_but_no_spend_secrets proves SQLite schema/rows contain viewing state but no spend secrets.
- Viewing-only: initialization_and_reopen_bind_exact_account_network_and_schema proves initialization/reopen bind exact account/network/schema without spend authority.
- Allocation-bound: store_limits_cover_immediate_below_at_and_above_before_allocation proves store limits cover immediate-below/at/above before allocation.
- Closed paths: sqlite_paths_are_closed_account_network_derived_and_linux_private proves SQLite paths are closed/account/network-derived/Linux-private.

## Negative evidence

No network, real wallet/seed, node, device, secret, mainnet, signing, proving, extraction, or broadcast occurred. The eight store tests run locked/offline with synthetic fixtures only.

## Integration

Staged exactly seven paths: four source files (zec.rs, fixture.rs, store.rs, test_support.rs), corrected store test (tests/zec_store.rs), new evidence (docs/testing/BBD-WAL-006-STORE-GATE-01.md), and current task (docs/handoff/CURRENT_TASK.md).

Commit: `feat: add WAL-006 viewing store boundary`

Push: master

Final state: HEAD == origin/master, clean index, clean tracked worktree.

The reviewer alone accepts the result and authorizes the next slice.
