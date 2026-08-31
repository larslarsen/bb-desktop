# Hermes Handoff — BBD-WAL-006 Store Gate 01

You are **Jr Dev — Hermes**. This durable handoff is authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely before acting: `AGENTS.md`, `TESTING.md`,
`docs/engineering/DEVELOPMENT_ROLES.md`, `docs/engineering/HERMES_JR_DEV_ROUTING.md`,
`tickets/BBD-WAL-006.md`, `docs/architecture/BBD-WAL-006-UPSTREAM-REVIEW.md`, both store
production handoffs, Store Source Reviews 01 and 02, the complete committed `zec_store` and
`zec_address` tests, Address Gate Evidence 01, and `CURRENT_TASK.md`.

## Sole task and role boundary

Verify the exact accepted four-path store source, run the five commands below once in order, and
only on exact success write bounded evidence and integrate the drop. You are the execution,
evidence, and Git actor. You are not the reviewer and may not design/edit a test, repair/format
source, change a command, accept a mismatch, or authorize further work.

Stop immediately on the first precondition, formatter, compiler/lint, test, policy, scope, hash,
or Git mismatch. On a stop, report retained output and make no evidence, source edit, staging,
commit, or push.

## Protected preconditions

Require `HEAD == origin/master ==` the protected governance parent, a clean index, and worktree
changes consisting only of:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/zec.rs` | 214 | `800111bf587265b49764de71c6d2beb054d1906932005bb3513b2d4fc132d90e` |
| `wallet-broker/src/zec/fixture.rs` | 279 | `5b1e91a730cddd82d0321383ec86f68dd781bf441a4b3e1db7e0514c5b9d5229` |
| `wallet-broker/src/zec/store.rs` | 1,720 | `9da2d00d7ed2fa4d942cf33cd8fbfe9bc28a02dd33174154bd6c2d54b4d81822` |
| `wallet-broker/src/zec/test_support.rs` | 824 | `0b4700eb776b01f9ab8cadfce44a916afe8c7eb01aa0be1f599c453986d097c4` |

Total source is 3,037 lines. Also require:

| Protected path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/tests/zec_store.rs` | 334 | `492e4e6934f8cd9589de22cc338fd5e93131f3f3d3fcca5f79b44455b297e1ca` |
| `wallet-broker/tests/zec_address.rs` | 277 | `2c5012e6884c8c2a81236266c6861b6e4e4fd6b124656dad2ab438add5848ee3` |
| `wallet-broker/src/lib.rs` | 11 | `ad61f92cce12115df9da6f263240ec0c57b5746e79cdcf6459f03731dab45617` |
| `wallet-broker/src/zec/address.rs` | 204 | `d9d54e301d188e3d4cb539095c5c01bcb07aaf011fcb1a50129aadc18f24cbfe` |
| `wallet-broker/tests/fixtures/zec/manifest.json` | 238 | `0c5836405b29a2af618a9fa2784a973c981e807c3a8d46471aeeb90d8a6fe389` |
| `wallet-broker/Cargo.toml` | 81 | `6643435bdf59608b09e906c8b7010baf1c17bbaa785d8eb70e37039d4bb37632` |
| `wallet-broker/Cargo.lock` | 5,369 | `ac454889b0796afea3f2a2ddfaf8c585bfff1080bec6d1428d0c227534ffb9dd` |
| `scripts/security-policy.js` | 2,299 | `60e41a12462d77c6be875f1659e5ef8a86d2b8146bd25d37ec7777297847d767` |
| `test/securityPolicy.node.js` | 2,454 | `f8340ae965a1f59e27594153cd3c9f3eca5ee1d8a78ed0a9a685faf6f1dc2647` |

Require source-only and whole-worktree `git diff --check` success. Record, before execution:

```text
hermes --version
hermes config get model.provider
hermes config get model.default
```

The provider/model may differ from adoption state, but the exact resolved values must be retained
in the report/evidence. This identity inspection does not authorize a Hermes configuration edit.

Inspect the filesystem type for the wallet broker. Use only these ignored disk-backed paths,
creating them if absent; do not use `/tmp`:

```text
/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-tmp
/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-cargo
```

## Exact execution

Run once, in order, from the repository root with no network:

```text
env TMPDIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-tmp CARGO_TARGET_DIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-cargo /home/lars/.cargo/bin/cargo +1.98.0 fmt --manifest-path wallet-broker/Cargo.toml --check
```

Require exit 0 without mutation.

```text
env TMPDIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-tmp CARGO_TARGET_DIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-cargo /home/lars/.cargo/bin/cargo +1.98.0 clippy --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --lib -- -D warnings
```

Require exit 0 with no warning or diagnostic.

```text
env TMPDIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-tmp CARGO_TARGET_DIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-cargo /home/lars/.cargo/bin/cargo +1.98.0 test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_store
```

Require exactly 8 passed, 0 failed, 0 ignored, 0 measured, and 0 filtered out.

```text
env TMPDIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-tmp CARGO_TARGET_DIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-cargo /home/lars/.cargo/bin/cargo +1.98.0 test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_address
```

Require exactly 8 passed, 0 failed, 0 ignored, 0 measured, and 0 filtered out.

Then run once:

```text
node test/securityPolicy.node.js
```

This remains an expected partial red because the broader Phase-C policy transition is frozen.
Require exit 1, exactly 69 `ok`, 6 `not ok`, and final line
`6 security policy test(s) failed`. The six failing groups must be exactly:

1. `committed workflows satisfy the fail-closed checker`
2. `strict nine-line reviewed Gitleaks ratchet bytes and content are enforced`
3. `WAL-004 Rust source inventory is exported closed and enumerated by repository policy`
4. `WAL-006 feature policy distinguishes compiled upstream PCZT capability from BitBook authority`
5. `WAL-006 requires the exact bounded Phase-C ZEC production inventory`
6. `WAL-006 policy rejects live-network and authority-bearing Rust snippets without denying upstream transitives`

The first three remain only the accepted untransitioned inventory groups; the last three remain
the accepted deferred WAL-006 policy groups. No other failure, warning, exception, or source-
policy finding is accepted.

## Evidence and integration

Only after every result is exact, create `docs/testing/BBD-WAL-006-STORE-GATE-01.md` and update
`docs/handoff/CURRENT_TASK.md` to `PHASE-C STORE GATE COMPLETE — REVIEW REQUIRED`. Record:

- timestamp/timezone, protected parent, filesystem type, exact ignored paths, Hermes version,
  provider, and model;
- all protected scope/hash/line/diff checks;
- each exact command, exit, warning/test counts, and the exact six policy-red names/explanation;
- v0/v1 migration, corrupt-state read-only preflight, hostile-entry rejection, store durability,
  secret-exclusion, viewing-only, and allocation-bound behaviors proven by the eight tests;
- no network, real wallet/seed, node, device, secret, mainnet, signing, proving, extraction, or
  broadcast; and
- exact source/evidence inventory, integration commit, push, and final repository state.

Do not include a seed, receiver, UFVK, canary, decoded SQLite value, user-data path, or raw
upstream error.

Recheck hashes, exact scope, and `git diff --check`. Stage explicitly only the four accepted
source paths, the new evidence, and `CURRENT_TASK.md`; inspect the staged list/diff. Commit as
`feat: add WAL-006 viewing store boundary`, push `master`, and prove `HEAD == origin/master`,
clean index, and clean tracked worktree. Do not stage ignored target artifacts.

Do not run any unlisted test/tool or edit source, tests, fixture, manifest, dependencies, lock,
policy, workflow, ticket, package, other documentation, or another repository. Do not clean or
delete artifacts. The reviewer alone accepts the result and authorizes the next slice.
