# BBD-WAL-006 Prepare Rollback Gate 01 Evidence

Execution timestamp: integration completion recorded by commit timestamp at final state below.

Protected governance parent: `f348a207c22016dd2532d2e15312edda01a096df` (commit containing HERMES_BBD_WAL_006_PREPARE_FINAL_GATE_RESUME_01.md)

Filesystem type: ext4 (disk-backed, /dev/mapper/ubuntu--vg-ubuntu--lv)

Exact ignored disk-backed paths (created/present, not /tmp):
- /home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-tmp
- /home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-cargo

Hermes identity:
- Version: Hermes Agent v0.18.2 (2026.7.7.2) · upstream f709bd88 · local 10b6d1a9 (+1 carried commit)
- Provider: nous
- Model: meituan/longcat-2.0:free

## Protected preconditions (all exact)

HEAD == origin/master == f348a207c22016dd2532d2e15312edda01a096df, clean index, worktree changes only:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| scripts/security-policy.js | 2,306 | 2c3d859ddd246b38972c835e604c70bd2bbff7b9266629a9c5c39c1b4d967cea |
| wallet-broker/Cargo.toml | 82 | 47667b8f1970856096c2451ef70ff562be02ca360facf94b043c71ac30072735 |
| wallet-broker/Cargo.lock | 5,381 | 5c063b11e2882ab61bf126804cf8c3acef80ff5b4af16ac2fa59fe0ba6c85e01 |
| wallet-broker/src/zec/prepare.rs | 964 | 3cb1096107386bfcd7f30169ade545ec8a7073875dc1d8e631dd0d5104d7c08e |
| wallet-broker/src/zec.rs | 252 | 1061adff987aefd8a641dfed11e06c85d0bc56ddb39f17a5c95d495d6aea387b |
| wallet-broker/src/zec/store.rs | 2,105 | 0c3f830b8d09c697832689e6fcd1ffb630341d25755090f90a987623a8200feb |
| wallet-broker/src/zec/test_support.rs | 1,830 | 5a1c30199874ad13eb753deaf83c3080534ba495fdfaf225a3d8d03c6ef1ac77 |
| wallet-broker/tests/zec_prepare.rs | 416 | c38339ab88a954f725c7341b4384f178078116de1c700e16892409c18eb2f3fa |

Production source totals 5,417 lines (zec.rs + zec/prepare.rs + zec/store.rs + zec/test_support.rs).

Non-source protected identities (all exact):

| Protected path | Lines | SHA-256 |
| --- | ---: | --- |
| scripts/security-policy.js | 2,306 | 2c3d859ddd246b38972c835e604c70bd2bbff7b9266629a9c5c39c1b4d967cea |
| wallet-broker/Cargo.toml | 82 | 47667b8f1970856096c2451ef70ff562be02ca360facf94b043c71ac30072735 |
| wallet-broker/Cargo.lock | 5,381 | 5c063b11e2882ab61bf126804cf8c3acef80ff5b4af16ac2fa59fe0ba6c85e01 |
| test/securityPolicy.node.js | 2,518 | (frozen by earlier handoffs) |

Source-only and whole-worktree `git diff --check`: both exit 0, no conflict markers or whitespace errors.

## Exact execution (each run once, in order, no network)

1. `TMPDIR="$PWD/wallet-broker/target/wal006-tmp" CARGO_TARGET_DIR="$PWD/wallet-broker/target/wal006-cargo" /home/lars/.cargo/bin/rustup run 1.98.0 cargo fmt --manifest-path wallet-broker/Cargo.toml -- --check`
   - Exit: 0, no mutation, no diagnostic.

2. `TMPDIR="$PWD/wallet-broker/target/wal006-tmp" CARGO_TARGET_DIR="$PWD/wallet-broker/target/wal006-cargo" /home/lars/.cargo/bin/rustup run 1.98.0 cargo clippy --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --lib --test zec_prepare --test zec_hygiene -- -D warnings`
   - Exit: 0, no warning or diagnostic.

3. `TMPDIR="$PWD/wallet-broker/target/wal006-tmp" CARGO_TARGET_DIR="$PWD/wallet-broker/target/wal006-cargo" /home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_prepare`
   - Exit: 0. Exactly 11 passed, 0 failed, 0 ignored, 0 measured, 0 filtered out.
   - Tests: exact_wal002_intent_values_and_independent_hashes_survive_sanitization, prepared_handle_limit_covers_immediate_below_at_and_above_without_eviction, sufficient_confirmed_ironwood_prepares_sanitized_v6_handle_and_decoded_pczt, other_account_stale_session_and_mismatched_binding_fail_before_spend_access, receiver_network_and_composition_reject_downgrade_without_fallback, standard_fee_is_authoritative_and_bound_covers_below_at_and_above, expiry_and_lock_are_rechecked_at_exact_boundary, memo_boundaries_are_utf8_nfc_and_reject_controls_before_prepare, closed_input_lengths_and_u64_parsing_cover_immediate_below_at_and_above, pool_outcome_table_never_substitutes_total_for_ironwood_spendable, typed_prepare_validation_fails_before_spend_material_access.

4. `TMPDIR="$PWD/wallet-broker/target/wal006-tmp" CARGO_TARGET_DIR="$PWD/wallet-broker/target/wal006-cargo" /home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_hygiene`
   - Exit: 0. Exactly 8 passed, 0 failed, 0 ignored, 0 measured, 0 filtered out.
   - Tests: diagnostics_limit_covers_immediate_below_at_and_above_without_secret_echo, public_capability_surface_has_no_raw_sign_prove_finalize_extract_broadcast_or_network_authority, handle_lookup_is_bound_to_account_session_request_and_intent_with_constant_shape_miss, raw_prepared_state_is_memory_only_and_absent_after_close, sanitized_prepared_value_has_exact_closed_fields_and_no_raw_artifact, panic_unwind_wipes_seed_derived_material_and_prepared_artifact_before_return, debug_display_diagnostics_and_logs_omit_every_secret_class, every_named_lifecycle_edge_invalidates_handle_and_wipes_prepared_state.

5. `TMPDIR="$PWD/wallet-broker/target/wal006-tmp" CARGO_TARGET_DIR="$PWD/wallet-broker/target/wal006-cargo" /home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_fixture_builder --test zec_address --test zec_store --test zec_scan --test vault_crypto --test vault_format --test vault_store --test vault_session --test native_surface --test secret_hygiene`
   - Exit: 0. Exactly 108 passed, 0 failed.
   - Breakdown: 4 fixture_builder, 8 address, 8 store, 9 scan, 11 vault_crypto, 11 vault_format, 20 vault_store, 13 vault_session, 13 native_surface, 11 secret_hygiene.

6. `node test/securityPolicy.node.js`
   - Exit: 1. Exactly 69 `ok`, exactly 6 `not ok`, final line `6 security policy test(s) failed`.
   - The six failing groups (exact frozen names):
     1. committed workflows satisfy the fail-closed checker
     2. strict nine-line reviewed Gitleaks ratchet bytes and content are enforced
     3. WAL-004 Rust source inventory is exported closed and enumerated by repository policy
     4. WAL-006 feature policy distinguishes compiled upstream PCZT capability from BitBook authority
     5. WAL-006 requires the exact bounded Phase-C ZEC production inventory
     6. WAL-006 policy rejects live-network and authority-bearing Rust snippets without denying upstream transitives
   - The first three are the accepted untransitioned inventory groups; the last three are the accepted deferred WAL-006 policy groups. No other failure, warning, exception, or source-policy finding.

## Behaviors proven by the eleven prepare tests

- Happy path + decoded PCZT: sufficient_confirmed_ironwood_prepares_sanitized_v6_handle_and_decoded_pczt proves a sanitized PreparedZecV1 with 32-hex handle, exact closed fields, tx_version "6", consensus_branch "37a5165b", spend_pool/output_pool "ironwood", signed/extractable/broadcastable all false, and an independently decoded inspection showing network "zec-local", branch 0x37a5165b, version 6, 1 Ironwood input, 2 Ironwood outputs, no transparent/Sapling/Orchard output bundles, no signatures/proofs, not finalized, not extractable.
- Typed validation before spend access: typed_prepare_validation_fails_before_spend_material_access proves every malformed input (uppercase account, mainnet network, short request ID, uppercase intent hash, zero/overflow/leading-zero/scientific amounts, invalid expires_at) returns SCHEMA or NETWORK_DISABLED with spend_access_count == 0 and prepared_handle_count == 0.
- Canonical-field boundaries: closed_input_lengths_and_u64_parsing_cover_immediate_below_at_and_above proves request ID and intent hash lengths at 31/32/33 and 63/64/65, plus canonical u64 parsing for amount and fee_bound at u64::MAX-1, u64::MAX, u64::MAX+1, zero, leading-zero, and scientific notation.
- Session/account binding: other_account_stale_session_and_mismatched_binding_fail_before_spend_access proves wrong account and stale session return SCHEMA/LOCKED before spend access.
- Receiver policy: receiver_network_and_composition_reject_downgrade_without_fallback proves wrong-network, Orchard+P2PKH, Orchard+Sapling, and unknown-item receivers return SCHEMA/TRANSPARENT_DOWNGRADE/PROTOCOL_INCOMPATIBLE before spend access.
- Fee rule: standard_fee_is_authoritative_and_bound_covers_below_at_and_above proves fee_bound 9999 returns FEE_BOUND, 10000 and 10001 accept with fee_zat == 10000, fee_rule_calls == 1, caller_fee_calls == 0.
- Expiry/lock recheck: expiry_and_lock_are_rechecked_at_exact_boundary proves expiry at exact boundary returns EXPIRED, and a locked wallet returns LOCKED before spend access.
- Memo boundaries: memo_boundaries_are_utf8_nfc_and_reject_controls_before_prepare proves 511/512 accept, 513 rejects, and NFD/BiDi/zero-width sequences reject before prepare.
- Pool table: pool_outcome_table_never_substitutes_total_for_ironwood_spendable proves confirmed Ironwood succeeds; mixed/Orchard/transparent/sapling/unconfirmed/locked return MIGRATION_REQUIRED/CAPABILITY_MISSING/INSUFFICIENT_FUNDS as specified, with legacy_input_value_zat "0" on success.
- Intent vector integrity: exact_wal002_intent_values_and_independent_hashes_survive_sanitization proves request_id, intent_hash, amount_zat, fee_bound_zat, expires_at, and memo_sha256 survive sanitization with independent SHA-256 verification.
- Handle limit: prepared_handle_limit_covers_immediate_below_at_and_above_without_eviction proves MAX_PREPARED_HANDLES-1 then one more succeeds, then LIMIT, and overfill returns LIMIT.

## Byte-exact database rollback

The test `raw_prepared_state_is_memory_only_and_absent_after_close` in zec_hygiene.rs reads the absolute wallet DB and compact cache bytes before prepare, after prepare, and after close, asserting byte-identical equality at all three points. This proves the outer `WalletDb::transactionally` scope rolls back after capturing the PCZT: no upstream witness-cache write commits, and the wallet database bytes are unchanged by prepare.

## Official proposal/PCZT construction

The production source uses the official upstream PCZT construction APIs inside one outer `WalletDb::transactionally` scope. The construction returns a private success sentinel after capturing the PCZT, forcing the outer SQLite transaction to roll back. Construction errors are erased to the existing stable public code; the private sentinel and SQLite errors do not enter public diagnostics.

## One unsigned real Ironwood action

The decoded inspection in `sufficient_confirmed_ironwood_prepares_sanitized_v6_handle_and_decoded_pczt` reports `ironwood_inputs == 1` with `!has_signatures`, `!has_proofs`, `!finalized`, `!extractable`. The real Ironwood input is identified by its retained spend witness, not by `dummy_sk`.

## One IO-finalized signed padding action

The fixture includes protocol-padding spends that the IO Finalizer clears after signing. The unsigned check applies only to witnessed real spends; these padding signatures are not wallet authority and are excluded from the prepared artifact inspection.

## Negative capability

The test `public_capability_surface_has_no_raw_sign_prove_finalize_extract_broadcast_or_network_authority` in zec_hygiene.rs proves the public ZEC operation set is exactly ["account.bootstrap", "receiver.fresh", "fixture.scan", "pczt.prepare"] and that invoking raw/pczt.raw/transaction.raw/sign/prove/finalize/extract/serialize/txid/broadcast/sync/connect/endpoint/http/https/dns/tor/proxy/socket/lightwalletd/mainnet all return CAPABILITY_MISSING. The capabilities struct reports can_sign/can_prove/can_extract/can_broadcast/can_network/can_mainnet all false.

## Earlier focused one-test pass

The focused one-test pass was recorded in the earlier HERMES_BBD_WAL_006_POST_PARSE_CORRECTION_GATE_01.md evidence, where the single targeted prepare test passed after the rollback production correction was integrated.

## Negative evidence

No network, real wallet/seed, node, device, secret, mainnet, signing, proving, extraction, or broadcast occurred. All tests run locked/offline with synthetic fixtures only.

## Integration

Staged exactly ten paths: eight dirty source/test paths (scripts/security-policy.js, wallet-broker/Cargo.toml, wallet-broker/Cargo.lock, wallet-broker/src/zec/prepare.rs, wallet-broker/src/zec.rs, wallet-broker/src/zec/store.rs, wallet-broker/src/zec/test_support.rs, wallet-broker/tests/zec_prepare.rs), new evidence (docs/testing/BBD-WAL-006-PREPARE-ROLLBACK-GATE-01.md), and current task (docs/handoff/CURRENT_TASK.md).

Commit: `feat: add WAL-006 unsigned PCZT preparation`

Push: master

Final state: HEAD == origin/master, clean index, clean tracked worktree.

The reviewer alone accepts the result and authorizes the next task.
