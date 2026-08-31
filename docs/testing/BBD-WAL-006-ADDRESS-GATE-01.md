# BBD-WAL-006 Address Gate Evidence 01

Timestamp: `2026-08-31T16:14:49-07:00` (America/Los_Angeles)

Execution parent: `78a488ff76b59291419e33b0e3fec0ed03425575`

Filesystem: `ext2/ext3`. The ignored disk-backed paths were:
`/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-tmp` and
`/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-cargo`.

## Preconditions and source inventory

Protected `HEAD == origin/master`, clean index, source-only scope, exact six-path
inventory, protected inputs, and source/whole-worktree `git diff --check` all passed.
The six accepted source paths were:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/lib.rs` | 11 | `ad61f92cce12115df9da6f263240ec0c57b5746e79cdcf6459f03731dab45617` |
| `wallet-broker/src/zec.rs` | 203 | `c86c030245e3caaec5182e4138f199a5bab08223c5c95ecb25b87745bbfa5e80` |
| `wallet-broker/src/zec/address.rs` | 204 | `d9d54e301d188e3d4cb539095c5c01bcb07aaf011fcb1a50129aadc18f24cbfe` |
| `wallet-broker/src/zec/fixture.rs` | 256 | `af26e693f39f85ecd428f4874f20bd9857812b48b05093c6ea8769b02f56b9b2` |
| `wallet-broker/src/zec/store.rs` | 791 | `3e786a1f236fd9528f7fd0b3dfd9725670969ab2ff75c80d9901ef180aca1314` |
| `wallet-broker/src/zec/test_support.rs` | 389 | `f7fa31df8f707ead35bba1cd3904c7a4d4b0610bcfe860d710e57a4c12d44ca5` |

Total source inventory: 1,854 lines. Protected inputs remained exact: accepted
`zec_address.rs` SHA `2c5012e6884c8c2a81236266c6861b6e4e4fd6b124656dad2ab438add5848ee3`,
manifest SHA `0c5836405b29a2af618a9fa2784a973c981e807c3a8d46471aeeb90d8a6fe389`,
Cargo.toml SHA `6643435bdf59608b09e906c8b7010baf1c17bbaa785d8eb70e37039d4bb37632`,
Cargo.lock SHA `ac454889b0796afea3f2a2ddfaf8c585bfff1080bec6d1428d0c227534ffb9dd`,
policy SHA `60e41a12462d77c6be875f1659e5ef8a86d2b8146bd25d37ec7777297847d767`,
and focused Node test SHA `f8340ae965a1f59e27594153cd3c9f3eca5ee1d8a78ed0a9a685faf6f1dc2647`.

## Prior safe stops and corrections

The first formatter run safely stopped with 14 hunks in four ZEC files:
`address.rs`, `fixture.rs`, `store.rs`, and `test_support.rs`. The reviewed
format correction was applied, and the fresh formatter run passed exit 0.

The next Clippy run safely stopped on two `drop_non_drop` diagnostics for explicit
`drop(spending)` calls at `address.rs:33` and `address.rs:182`. The correction
deleted those two calls; the narrow accepted claim is only that the observed
owned seed buffer is zeroed. No allocator, register, stack, copy, or upstream
derived-key memory erasure is claimed. Fresh Clippy passed with `-D warnings` and
no diagnostics.

The first address test run safely stopped at a parallel test-root race: 4 passed,
4 failed because four workers attempted an already-existing state ancestor at
`test_support.rs:76`. The accepted correction made state-root creation atomic;
the fresh run passed all 8 tests.

## Fresh gate results

The fresh formatter command exited 0 with no diagnostics. The locked/offline
Clippy command exited 0 with no warnings or diagnostics. The locked/offline
`zec_address` command exited 0:

```text
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

The final Node policy command exited 1 with exactly 69 `ok`, 6 `not ok`, and
final line `6 security policy test(s) failed`. The six exact failing groups were:

1. `committed workflows satisfy the fail-closed checker`
2. `strict nine-line reviewed Gitleaks ratchet bytes and content are enforced`
3. `WAL-004 Rust source inventory is exported closed and enumerated by repository policy`
4. `WAL-006 feature policy distinguishes compiled upstream PCZT capability from BitBook authority`
5. `WAL-006 requires the exact bounded Phase-C ZEC production inventory`
6. `WAL-006 policy rejects live-network and authority-bearing Rust snippets without denying upstream transitives`

The first two workflow checks and the WAL-004 inventory check are the accepted
inventory-transition consequence of adding the six ZEC source paths while the
WAL-004 policy export remains frozen. The last three are the accepted deferred
Phase-C policy reds. No other failure, warning, or source-policy finding occurred.

No network, real wallet or seed, node, device, secret, mainnet, signing, proving,
extraction, or broadcast was used. No receiver, UFVK, canary, user-data path, or
raw upstream error is recorded. Production source remains frozen after this gate.

## Integration state

This evidence and CURRENT_TASK update are integrated with the six accepted source
paths only. Final repository state is clean tracked worktree/index with
`HEAD == origin/master`; ignored target artifacts remain outside Git.
