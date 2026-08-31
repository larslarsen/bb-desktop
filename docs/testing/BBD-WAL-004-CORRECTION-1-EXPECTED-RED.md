# BBD-WAL-004 Correction 1 Expected-Red Evidence

Timestamp: 2026-08-30T19:44:14-0700 (PDT)
Governance baseline: `HEAD == origin/master == d70e2aeafc74824ef68d0f0aa6ade7af6ec1799e`

Accepted corrected test paths were verified unchanged:

- `wallet-broker/tests/vault_store.rs` — 483 lines — `5774432aef4173a2a1d64bf2dc2b2d9272b93df310f57aee28d7170b953082b9`
- `wallet-broker/tests/vault_session.rs` — 224 lines — `9161e738ae33771a347c782582c2875090295f58ef8c02bc233940e4d9368209`
- `wallet-broker/tests/native_surface.rs` — 376 lines — `2936ec15e13b7ecabad9c7340a269c741ea964e06c1df3649de9c7d7cbcb41ee`
- `wallet-broker/tests/secret_hygiene.rs` — 260 lines — `804c66c4cdec073990e4c4996acd993b6542183111939ca3a95f4797b03a50f0`
- `test/securityPolicy.node.js` — 2,038 lines — `dd2e5eef306037dffd846f0d9d239ca0493fd78e01c2ddee0f70816b8488cb84`

Rust/Cargo reported 1.98.0. `wallet-broker/Cargo.lock` is 3,273 lines with SHA-256
`1d182550f82b9dc443804d82e709b5d58021d4243bef1612c440aac59df1c2f5`.

## Rust commands

All four locked offline commands reached execution and exited 101 with only the
reviewed frozen-production defects:

- `cargo ... --test vault_store` — 18 passed, 1 failed:
  `linux_store_enforces_real_modes_regular_files_and_symlink_rejection`.
- `cargo ... --test vault_session` — 9 passed, 2 failed:
  `invalid_account_unlock_is_schema_and_wipes_supplied_material` and
  `late_native_authorization_at_existing_deadline_times_out_and_wipes`.
- `cargo ... --test native_surface` — 9 passed, 2 failed:
  `invalid_passphrase_lengths_wipe_before_unlock_or_restore_custody` and
  `invalid_unlock_and_export_accounts_fail_before_native_authority_moves`.
- `cargo ... --test secret_hygiene` — 8 passed, 2 failed:
  `diagnostic_fields_reject_malformed_accounts_and_secret_canaries` and
  `diagnostic_operations_and_codes_are_closed_to_exact_reviewed_values`.

## Node policy command

`node test/securityPolicy.node.js` exited 1 after all 64 tests ran: 57 `ok`, 5 `not ok`,
and one new WAL-004 SBOM-validator regression test passed. Failures were:

1. `checker constants match the ticketed Action and tool pins` — missing Rust SBOM
   validator path in the expected policy constants.
2. `WAL-004 policy and validator changes trigger every applicable routine workflow` —
   workflow omits `scripts/validate-rust-sbom.js`.
3. `WAL-004 Rust source inventory is exported closed and enumerated by repository policy` —
   Rust source inventory export is absent.
4. `WAL-004 vault and native source policy requires reviewed secret and path primitives` —
   required policy rejection is not enforced.
5. `WAL-004 cargo-deny policy is exact fail-closed and has no bypass lists` — cargo-deny
   policy export is absent.

No secret canary appeared in output. No unlisted path changed, no production path was
edited, and no prohibited command or action occurred. The 15 frozen production paths
remain unstaged and hash-identical.
