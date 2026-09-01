# BBD-WAL-006 Scan Gate 01 Evidence

Jr Dev — Hermes integration evidence. All gate commands ran from repository root with no network.

## Identity

- Hermes Agent: v0.18.2 (2026.7.7.2) upstream b20cc5f7, local 10b6d1a9 (+1 carried commit)
- Provider: nous
- Model: meituan/longcat-2.0:free
- Python: 3.11.15

## Protected state

- Governance parent: `eaf4fcba` (HEAD == origin/master)
- Index: clean before staging
- Worktree: five modified + one untracked accepted path only
- Filesystem: ext2/ext3 disk-backed at `/home/lars/OpenBazaar/bb-desktop/wallet-broker`
- TMPDIR: `/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-tmp`
- CARGO_TARGET_DIR: `/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-cargo`

## Preconditions — exact worktree inventory

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/zec.rs` | 218 | `ae06d6a9b4448603860b55bf4eef42a1e970c414049a9304f42b436d2bf49cb4` |
| `wallet-broker/src/zec/fixture.rs` | 614 | `318820c6f125f2318ba0caf5ae44835b36b0f67856fc66b83401961f0c301b36` |
| `wallet-broker/src/zec/scan.rs` | 1,661 | `54a890fd0b5a8ca8ce78dfb59c13424407341df25e47a5eacd2de8d6cc3317ad` |
| `wallet-broker/src/zec/store.rs` | 1,816 | `d0b97b802209489804e5d550ef90138a87342e155c519a686b2d900189f6603f` |
| `wallet-broker/src/zec/test_support.rs` | 1,220 | `02609ae79bb8d35a5c7fb0058f0ab56fa5deb27c94cfa2ddb81347a2f38ac034` |
| `wallet-broker/tests/zec_scan.rs` | 325 | `87ed1c3e8db8219ad126efb4de7f681cf909cb404b88df6c2dfadec471a3701f` |

Production source total: 5,529 lines.

## Other protected non-source identities

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/tests/zec_store.rs` | 324 | `1c230a2a9cf51c841a0df6514393861387422e5d0b2a83e80af47022728e2225` |
| `wallet-broker/tests/zec_address.rs` | 277 | `2c5012e6884c8c2a81236266c6861b6e4e4fd6b124656dad2ab438add5848ee3` |
| `wallet-broker/tests/zec_prepare.rs` | 412 | `a616245fdcf8d2226277e34d785bb1792d3b8b54ebf268c3d1dc5f15566da942` |
| `wallet-broker/tests/zec_hygiene.rs` | 375 | `aad7c95a2ef661063661f2ec0f16a216d80328096b049d636b88fa0252ba1be6` |
| `wallet-broker/tests/fixtures/zec/manifest.json` | 238 | `0c5836405b29a2af618a9fa2784a973c981e807c3a8d46471aeeb90d8a6fe389` |
| `wallet-broker/src/lib.rs` | 11 | `ad61f92cce12115df9da6f263240ec0c57b5746e79cdcf6459f03731dab45617` |
| `wallet-broker/src/zec/address.rs` | 204 | `d9d54e301d188e3d4cb539095c5c01bcb07aaf011fcb1a50129aadc18f24cbfe` |
| `wallet-broker/Cargo.toml` | 81 | `6643435bdf59608b09e906c8b7010baf1c17bbaa785d8eb70e37039d4bb37632` |
| `wallet-broker/Cargo.lock` | 5,369 | `ac454889b0796afea3f2a2ddfaf8c585bfff1080bec6d1428d0c227534ffb9dd` |
| `scripts/security-policy.js` | 2,299 | `60e41a12462d77c6be875f1659e5ef8a86d2b8146bd25d37ec7777297847d767` |
| `test/securityPolicy.node.js` | 2,454 | `f8340ae965a1f59e27594153cd3c9f3eca5ee1d8a78ed0a9a685faf6f1dc2647` |

Absent path: `wallet-broker/src/zec/prepare.rs` — confirmed absent.
Source-only and whole-worktree `git diff --check`: PASS.

## Exact commands and results

### 1. Formatter (Rust 1.98.0)

```
env TMPDIR=... CARGO_TARGET_DIR=... /home/lars/.cargo/bin/cargo +1.98.0 fmt --manifest-path wallet-broker/Cargo.toml --check
```

Exit: 0. No mutation.

### 2. Library Clippy (locked/offline/no-default, -D warnings)

```
env TMPDIR=... CARGO_TARGET_DIR=... /home/lars/.cargo/bin/cargo +1.98.0 clippy --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --lib -- -D warnings
```

Exit: 0. No warning or diagnostic.

### 3. `zec_scan`

```
env TMPDIR=... CARGO_TARGET_DIR=... /home/lars/.cargo/bin/cargo +1.98.0 test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_scan
```

Exit: 0. Result: 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out.

### 4. `zec_store`

```
env TMPDIR=... CARGO_TARGET_DIR=... /home/lars/.cargo/bin/cargo +1.98.0 test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_store
```

Exit: 0. Result: 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out.

### 5. `zec_address`

```
env TMPDIR=... CARGO_TARGET_DIR=... /home/lars/.cargo/bin/cargo +1.98.0 test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_address
```

Exit: 0. Result: 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out.

### 6. Node policy (expected partial red)

```
node test/securityPolicy.node.js
```

Exit: 1. Result: exactly 68 `ok`, exactly 6 `not ok`. Final line: `6 security policy test(s) failed`.

The six exact frozen failure names:

1. `committed workflows satisfy the fail-closed checker`
2. `strict nine-line reviewed Gitleaks ratchet bytes and content are enforced`
3. `WAL-004 Rust source inventory is exported closed and enumerated by repository policy`
4. `WAL-006 feature policy distinguishes compiled upstream PCZT capability from BitBook authority`
5. `WAL-006 requires the exact bounded Phase-C ZEC production inventory`
6. `WAL-006 policy rejects live-network and authority-bearing Rust snippets without denying upstream transitives`

No other failure, warning, exception, or source-policy finding.

## Behaviors proven by all 25 Rust tests

Scan (9): closed/ordered/hashed manifest bound before scan; hostile manifest families fail before any scan/advance; birthday continuity, confirmation, and unrelated output are non-vacuous; replay is idempotent and close/reopen preserves exact state; malformed/discontinuous/wrong-branch/impossible-tree fail without tip advance; one-block reorg rolls back exact effects and applies replacement; deep reorg and compound rollback never partially mutate; SQLite corruption fails closed without cache or tip commit; checked balance and compact block limits cover below/at/above.

Store (8): paths are closed/account/network-derived and Linux-private; initialization/reopen bind exact account/network/schema; schema and rows contain viewing state but no spend secrets; migration is atomic across write/sync/commit failures; symlink/nonregular/wrong-mode state are rejected without replacement; corrupt/wrong-schema/truncated SQLite fail closed without empty recreation; failed write/file-sync/directory-sync never report durable state; manifest-size limits cover below/at/above before allocation.

Address (8): fresh receiver decodes to exactly one Orchard-protocol receiver; issuance is monotonic, durable, and viewing-only after reopen; concurrent issuers serialize without duplicates; coupled write failure returns nothing and advances neither record; account/network/mainnet validation precede database/derivation; receiver limits cover below/at/above without wrap; seed is wiped on success/error/cancellation/replacement/unwind/drop; unsupported receiver composition never falls back.

## Negative record

No network. No real wallet/seed. No mainnet. No signing, proving, extraction, broadcast, node, or device activity. No raw fixture blocks, UFVK, receiver, note/nullifier data, SQLite rows, or user paths recorded.
