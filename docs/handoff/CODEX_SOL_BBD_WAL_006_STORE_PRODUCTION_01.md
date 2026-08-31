# Codex Sol Handoff — BBD-WAL-006 Store Production 01

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This durable file is
the complete source prompt; ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Implementation source baseline: `432e69c0443dd5233609d578b43d5a43d83d2c3d`

Protected governance parent: the commit containing this handoff. Its changes after the
implementation source baseline are reviewer-authored review and routing records only.

Read completely before editing: `AGENTS.md`, `TESTING.md`,
`docs/engineering/DEVELOPMENT_ROLES.md`, `tickets/BBD-WAL-006.md`,
`docs/architecture/BBD-WAL-006-UPSTREAM-REVIEW.md`,
`docs/testing/BBD-WAL-006-ADDRESS-INTEGRATION-REVIEW-01.md`,
`docs/testing/BBD-WAL-006-ADDRESS-GATE-01.md`, `docs/handoff/CURRENT_TASK.md`, the
complete six accepted ZEC source files, and the complete committed
`wallet-broker/tests/zec_store.rs`. You may use read-only `sed`/`rg` inspection of the
repository and already-cached exact crate sources. Do not use network or execute a compiler,
formatter, test, build, policy tool, or Git command.

## Sole task

Author the smallest real production extension that satisfies the complete accepted
`zec_store` test without changing or bypassing it. Preserve the already accepted address
vertical and its public behavior. This slice owns account/network-bound SQLite viewing state,
closed schema recognition and migration, Linux path/type/mode/owner validation, atomic durable
state mutation, secret-exclusion inspection, and the fixture-manifest allocation limit.

This remains offline, synthetic, non-mainnet, viewing-only, unsigned, and unable to move funds.

## Exact authorized paths

You may edit only:

- `wallet-broker/src/zec.rs`
- `wallet-broker/src/zec/fixture.rs`
- `wallet-broker/src/zec/store.rs`
- `wallet-broker/src/zec/test_support.rs`

Do not edit `lib.rs`, `address.rs`, any test, fixture, manifest, `Cargo.toml`, `Cargo.lock`,
policy, ticket, documentation, evidence, handoff, workflow, package file, or other repository.
Do not create a file or future `scan.rs`/`prepare.rs` module. Stop if another path, dependency,
feature, version, or lockfile mutation is required.

The protected `zec_store` test is exactly 334 lines with SHA-256
`492e4e6934f8cd9589de22cc338fd5e93131f3f3d3fcca5f79b44455b297e1ca`.
The six accepted address source hashes are recorded in Address Integration Review 01 and must
remain unchanged except for the four authorized paths above. Do not weaken a prior address
invariant to satisfy this slice.

## Required public and hidden surface

Add public `StoreFault` in `zec.rs` with exactly the test-used variants:

- `MigrationWrite`, `MigrationSync`, `MigrationCommit`
- `Write`, `FileSync`, `DirectorySync`

Expose `MAX_FIXTURE_MANIFEST_BYTES` as the one shared public `usize` bound used by both
production fixture parsing and the hidden test allocation probe. Its value is exactly
`256 * 1024`. Remove or replace the private divergent `u64` bound; there must be one limit.

Extend only the existing `#[doc(hidden)] test_support` facade with the exact test-used surface:

- `SecretCanary`, `SecretClass`, `StoreEntryKind`, `TestAccount`, and `TestStateRoot`;
- all eight `SecretClass` variants and their exact lowercase/hyphenated `as_str()` values;
- all seven `StoreEntryKind` variants named by the test;
- path, store, SQLite, viewing-context, canary-commitment, migration, corruption, fault,
  checkpoint, hostile-entry, and allocation-observer methods invoked by `zec_store.rs`.

The public projections may reveal only the fields asserted by the test. Their `Debug` output
must not reveal paths, UFVKs, decoded row values, canary bytes, or other sensitive state.
Do not add a generic raw-path, raw-SQL, raw-row, arbitrary mutation, or arbitrary fault API.

## SQLite schema and binding

Keep the official upstream `WalletDb`/`BlockDb` initialization and the real upstream SQLite
tables. The accepted test must observe actual `accounts`, `addresses`, and `scan_queue` tables,
an actual `ufvk` column, and actual decoded text and blob values. Do not fabricate schema,
row counts, value kinds, or canary-search results in the facade.

Define a closed broker extension schema with exactly two recognized structural versions:

- version `0`, used only by the committed previous-schema test setup; and
- version `1`, created by current bootstrap and produced by the one supported migration.

Version recognition must follow exact table/column/index/constraint inventory, not only a
mutable version label. Unknown, missing, extra, or malformed broker extension structure is
`STATE_CORRUPT`. The current account binding contains the exact account ID, network, local
activation heights, UFVK/viewing material, receiver state, and the store state needed for the
test inspection. There is no spending-secret column.

Before invoking any API that can initialize or migrate an existing wallet, open it read-only
and perform fail-closed preflight: path chain, type/mode/owner, valid SQLite header/database,
recognized upstream schema, recognized broker version, and exact account/network binding.
Corrupt, truncated, unsupported-schema, wrong-account, and wrong-network state must return
`STATE_CORRUPT` with the wallet bytes unchanged. Reopen must never silently create, truncate,
replace, or initialize an empty database. Record no `initialize-empty` or `replace` operation.

Bootstrap may create a genuinely absent account only after validating every existing ancestor
and prospective entry. It creates version 1 directly. Reopen of version 1 is read-only with
respect to schema. Only recognized version 0 enters migration.

`install_previous_schema_for_test` must create a structurally real recognized version 0 from
the same database; it is not a receipt or an inspection override. `reopen_and_migrate` must run
the real `0 -> 1` change in one immediate SQLite transaction. The three migration fault ports
must interrupt distinct real write/pre-commit boundaries and force rollback. After each fault,
the main database bytes must equal their exact pre-call bytes and inspection must still report
version `0`. Never catch a migration error and continue with a new database.

## Paths and Linux filesystem boundary

Continue deriving only:

```text
zec-local/{32-lowercase-hex-account}/wallet.sqlite3
zec-local/{32-lowercase-hex-account}/compact.sqlite3
```

No caller path is accepted. Validate every component with `symlink_metadata` before use and
again at the SQLite operation boundary. Directories are regular directories, non-symlinks,
mode `0700`, and owned by the state-root owner. Files are regular files, non-symlinks, mode
`0600`, and owned by the state-root owner. Compare Unix owner IDs through safe standard-library
metadata. Any missing entry on reopen or any wrong type, symlink, owner, or access mode is
`STATE_CORRUPT`; do not chmod, chown, replace, or follow it.

For hostile fixtures that an unprivileged safe Rust process cannot construct—especially block
device, character device, and wrong-owner metadata—you may add one closed, typed, test-only
filesystem-facts fault seam inside `StateRoot`. Production resolution must always use actual
filesystem metadata. The seam must feed the same production validator, be impossible to select
through product input, and contain no arbitrary path or raw metadata API. Use real entries for
the variants that safe standard-library APIs can construct. Do not use `unsafe`, subprocesses,
privilege changes, or a new dependency.

`entry_marker()` must freshly derive its result on every call from the entry's current
symlink/regular/special identity, safe metadata facts, and any actual bounded marker bytes. It
must not return a stored invariant token that would pass after replacement. For a typed metadata
seam, incorporate the exact injected facts into the freshly computed marker. `operations()` is
observational only and cannot substitute for actual entry preservation.

## Viewing-only state and secret exclusion

`inspect_store()` must read actual SQLite state and return exact account, network, schema
version, scan tip, and receiver sequence. Closing and reopening the exact account/network must
produce equal inspection. `open_viewing_context()` must wrap the same reopened viewing state and
must never expose or reconstruct spending authority.

The database may persist the UFVK, birthday, diversifier/receiver state, scanned note/nullifier
viewing state, and tree state. It must never persist seed, mnemonic, Unified Spending Key,
derived spending material, vault plaintext, passphrase, raw/prepared PCZT, or authorization
session bytes or columns.

`install_nonpersistent_canaries_for_test` is a test-only in-memory inspection scope. For each
provided canary, compute the real SHA-256 of its bytes and return only exact class name, byte
length, and lowercase digest. The receipt must prove its class set is closed and retain no
canary byte copy. It must not write a canary, add a persistence slot, or fabricate an SQLite
scan. `inspect_sqlite_for_test` must independently enumerate the actual schema and decoded
SQLite cell values through bounded reads, exposing only names/count/kinds and the closed
`contains_decoded_row_bytes` predicate. A raw database-byte scan remains independent in the
test.

## Durable mutations and limits

`persist_checkpoint_for_test(103)` must traverse the real store mutation path and couple the
checkpoint/tip update with receiver sequence state in one immediate transaction. Each of
`Write`, `FileSync`, and `DirectorySync` must interrupt a distinct typed boundary before a
successful commit can be reported. On every injected error the transaction rolls back, the
method returns `STATE_CORRUPT` or `INTERNAL`, and both inspected fields equal their pre-call
values. A fault port may cause a real operation to fail; it may not update and then repair an
in-memory projection or fabricate a success/failure receipt. If the exact unchanged-state
contract cannot be implemented honestly with the accepted APIs and paths, stop and report.

`read_manifest_sized_for_test(length)` must exercise the same immediate bound check used before
production manifest allocation. At `limit - 1` and `limit`, record and perform exactly the
requested allocation. At `limit + 1`, return `LIMIT` before allocation and leave the observer
empty. Do not allocate first, truncate, reserve a larger capacity, or maintain a second limit.

All integer conversion and update paths are checked. All shared state remains poison/failure
safe. No error/log/debug string contains a raw upstream error, path, UFVK, SQLite value, canary,
secret, receiver, or user data.

## Prohibitions and stop conditions

Do not use `unsafe`, spawn a process, open a network connection, contact a node, use mainnet,
sign, prove, finalize, extract, broadcast, add scanning or preparation behavior, or claim a
durability/secret-erasure property not exercised here. Do not duplicate the store in memory,
special-case test labels, infer a hostile type from a label, hard-code inspection success, or
turn an assertion into a tautology.

Stop and report instead of substituting if the exact accepted test needs a new dependency,
feature, lockfile change, unsafe code, privilege, unlisted path, raw public test hook, or cannot
exercise real SQLite/path/transaction behavior. A safe, closed typed filesystem fact seam as
bounded above is the sole exception for OS entry facts unavailable to an unprivileged process.

## Source-actor restrictions and report

Use `apply_patch`. Do not run Cargo, Rust, rustfmt, Node, npm, tests, linters, builds, policy
checkers, scanners, Electron, wallets, nodes, devices, network, Git, or GitHub. Do not install,
delete, clean, move, stage, commit, or push anything.

When complete, stop and report every changed path with line count and SHA-256; the exact schema
versions and inventory; read-only preflight ordering; migration/transaction/fault behavior;
Linux metadata validation and any closed test seam; SQLite inspection/canary design; allocation
bound behavior; and every concern. Luna—not Sol—will inspect the drop, run formatting/compile/
tests, write evidence, and own all Git operations.
