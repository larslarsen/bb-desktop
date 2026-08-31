# BBD-WAL-006 Address Production Source Review 01

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Protected governance parent: `de3d37a1`

Result: **NO SOURCE DROP — DEPENDENCY CORRECTION TEST REQUIRED**

Sol completed the required repository and cached-upstream inspection, changed no file,
and stopped at an explicit handoff condition. Reviewer inspection confirms the
contradiction in the accepted source contract:

- `zcash_client_sqlite::wallet::init::WalletMigrator::init_or_migrate` requires its
  wallet RNG to implement `rand_core::RngCore + Clone + 'static`;
- the broker has no direct crate naming that trait or a production RNG implementing it;
- the upstream concrete testing RNG is behind test support and cannot become the product
  implementation;
- `WalletDb::for_path(..., ())` can open a SQLite file but cannot initialize/migrate the
  wallet schema; and
- the external-migration API names `schemerz_rusqlite` types, while
  `ExtensionTransaction` intentionally denies DDL and therefore cannot bootstrap the
  broker extension schema.

The smallest accepted correction is two already-locked direct API dependencies:

```text
rand_core = { version = "=0.6.4", default-features = false, features = ["std"] }
rusqlite  = { version = "=0.37.0", default-features = false }
```

`rand_core::OsRng` satisfies the published migrator bound without using an upstream test
RNG. Direct `rusqlite` permits a broker-owned `ext_bitbook_*` schema and atomic coupled
receiver/sequence transaction after official upstream schema initialization. A direct
`schemerz`, `schemerz-rusqlite`, `rand`, or wider rusqlite feature is not authorized.

Both packages and rusqlite's current feature union already exist in the accepted lock
through `zcash_client_sqlite`; the eventual manifest change is still expected to mutate
the root package's direct-dependency list in `Cargo.lock`. Luna must prove the exact
resolution and no-new-package/build-script/source/license result before address source
is reauthorized.

Test first: Sol may add only a Node policy test for these exact support pins. Production
source, manifest, lockfile, policy implementation, execution, evidence, and Git remain
frozen until its source and expected red are accepted.
