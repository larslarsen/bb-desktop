# Codex Sol Handoff — BBD-WAL-006 Address Production Resume 01

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This durable file
resumes the stopped address source task; ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Implementation source baseline: `8af5db0b9f7238f22f62cf4148ddd095e9d948b2`

Protected governance parent: the commit containing this handoff.

Read completely: `AGENTS.md`, `TESTING.md`, roles, ticket, the original
`CODEX_SOL_BBD_WAL_006_ADDRESS_PRODUCTION_01.md`, its stop review, the complete support
dependency test/red/source/gate evidence and final review 03, `CURRENT_TASK.md`, the
complete `zec_address` test/frozen manifest, existing `lib.rs`/`vault.rs`, exact manifest
and lock, and relevant already-cached upstream source. Read-only `sed`/`rg` is permitted.

## Resumed sole task

Perform the original address-production handoff exactly, with its complete behavior,
security, facade, path, and reporting requirements. The dependency stop is resolved:

- use direct `rand_core 0.6.4` only for the accepted `RngCore` interface and
  `rand_core::OsRng` needed by official `WalletMigrator` initialization;
- use direct `rusqlite 0.37.0` with no direct features for broker-owned schema and
  atomic state transactions; and
- name every application-owned object with the reserved `ext_bitbook_` prefix. Do not
  modify or depend on private upstream schema details.

Initialize/migrate the official upstream wallet schema first through
`zcash_client_sqlite`, then transactionally create/validate the closed broker extension
schema. A partial official-only database is not a successful bootstrap: reopen must
detect missing/wrong broker schema and fail closed or safely complete only the exact
initial migration path. Receiver index and issuance sequence must commit atomically in
the broker extension transaction before success. The `AddressFault` ports must interrupt
the real operation boundaries and preserve the prior committed pair.

## Exact authorized paths

Only the original six paths are writable:

- `wallet-broker/src/lib.rs`
- `wallet-broker/src/zec.rs`
- `wallet-broker/src/zec/address.rs`
- `wallet-broker/src/zec/fixture.rs`
- `wallet-broker/src/zec/store.rs`
- `wallet-broker/src/zec/test_support.rs`

Do not edit the now-accepted manifest, lockfile, policy, tests, fixture, docs, ticket,
handoffs, evidence, workflows, packages, or any other path. Do not create future
`scan.rs`/`prepare.rs` or authority/stub modules. A need for `rand`, schemerz crates,
another direct dependency/feature, lock change, unsafe, network, process, or unlisted
path is a new stop.

Use `apply_patch`. Do not run Cargo, Rust, rustfmt, Node, npm, tests, builds, linters,
policy tools, scanners, Electron, wallets, nodes, devices, Git, network, install,
cleanup, commit, or push. Report every changed path with line count/SHA-256, the exact
upstream derivation/migration APIs, SQLite schema/transaction design, concurrency/wipe
design, and any concern. Luna owns later execution/evidence/integration/Git.
