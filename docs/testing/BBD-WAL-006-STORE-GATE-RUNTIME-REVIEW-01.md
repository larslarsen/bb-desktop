# BBD-WAL-006 Store Gate Runtime Review 01

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Execution parent: `c1a6232e`

Result: **SAFE STOP ACCEPTED — CACHE-SCHEMA SOURCE CORRECTION REQUIRED**

Hermes v0.18.2 (`nous`, `meituan/longcat-2.0:free`) re-proved the protected five-path worktree,
eight non-source identities, clean index, diff checks, ext4 filesystem, and ignored target paths.
Rust 1.98.0 formatting passed without mutation. Locked/offline library Clippy passed without a
warning. The `zec_store` test ran and stopped the gate at 4 passed/4 failed; each failure returned
`STATE_CORRUPT`. `zec_address`, Node policy, evidence, staging, commit, and push did not follow.

The four failures share `preflight_store` after successful bootstrap:

- `initialization_and_reopen_bind_exact_account_network_and_schema`;
- `sqlite_schema_and_rows_contain_viewing_state_but_no_spend_secrets`;
- `schema_migration_is_atomic_across_write_sync_and_commit_failures`; and
- `failed_write_file_sync_and_directory_sync_never_report_durable_state`.

The other four store tests passed. The repository remains `HEAD == origin/master ==` the
execution parent with a clean index and the exact accepted five-path worktree.

## Root cause

Read-only inspection of a retained synthetic test cache and the locally pinned
`zcash_client_sqlite` 0.22.0 source proves an API/schema mismatch:

- production calls stable `BlockDb::for_path` plus `chain::init::init_cache_database`;
- that initializer creates exactly `compactblocks(height INTEGER PRIMARY KEY, data BLOB NOT
  NULL)` and no `schemer_migrations` table;
- `compactblocks_meta` and migration ID `68525b40-36e5-46aa-a765-720f8389b99d` belong to the
  separate unstable `FsBlockDb`/`init_blockmeta_db` API; but
- `validate_cache_schema` incorrectly requires that unused metadata schema, so every valid stable
  cache fails closed during preflight.

Store Production Source Review 02's acceptance of the single pinned cache migration was based on
that same API conflation and is superseded for the cache-schema point only. Its other semantic
acceptance remains controlling.

## Required correction

Sol may edit only `validate_cache_schema` in `wallet-broker/src/zec/store.rs`. It must validate
the exact stable cache actually initialized by production: a closed non-internal SQLite object
inventory containing only the `compactblocks` table, with exact `height`/`data` column names,
types, nullability, and primary-key flags. It must not accept the metadata cache, a migration
table, unknown objects, aliases, or extra columns. It must not change the initializer, enable the
unstable feature, relax wallet/extension validation, edit tests, or change dependencies.

A fresh reviewer source review is required before Hermes can restart the complete gate.
