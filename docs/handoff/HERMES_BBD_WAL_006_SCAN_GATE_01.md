# Hermes Handoff — BBD-WAL-006 Scan Gate 01

You are **Jr Dev — Hermes**. This durable handoff is authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely before acting: `AGENTS.md`, `TESTING.md`,
`docs/engineering/DEVELOPMENT_ROLES.md`, `docs/engineering/HERMES_JR_DEV_ROUTING.md`, the ticket,
the complete frozen `zec_scan`, `zec_store`, and `zec_address` tests, Scan Atomicity Review 01,
Scan Production Source Review 01, Scan Truth Correction Review 01, and `CURRENT_TASK.md`.

## Sole task and stop rule

Verify the exact accepted five-path source, run the six commands below once in order, and only on
exact success write evidence and integrate the drop. You own execution/evidence/Git, not source or
test design. Do not repair, format, or otherwise edit source/tests.

At the first precondition, formatter, compiler/lint, test, policy, scope, hash, or Git mismatch,
stop immediately. Retain and report the exact result. Make no evidence/current-task/source edit,
staging, commit, or push on a stop. Run no diagnostic or follow-up command after the mismatch.

## Protected preconditions

Require `HEAD == origin/master ==` the protected governance parent, a clean index, and worktree
changes consisting only of:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/zec.rs` | 218 | `ae06d6a9b4448603860b55bf4eef42a1e970c414049a9304f42b436d2bf49cb4` |
| `wallet-broker/src/zec/fixture.rs` | 600 | `708ebba85b215b873bacf580156dace9cd68e3d6ed6feb164719c1ff7c9776ee` |
| `wallet-broker/src/zec/scan.rs` | 1,368 | `6f7ef21d8bd951e071ed6b4454ffad0a27ad334cdd4b4c671d1a11e042406e9e` |
| `wallet-broker/src/zec/store.rs` | 1,814 | `3b475dadffa6c1adc4c500c020c82d4e0571805c51d049f475ad4ee08ffbf894` |
| `wallet-broker/src/zec/test_support.rs` | 1,231 | `10f453de6e41de698c60255881715b9211a14a8642ffb59ce307eeddadb3ca6c` |

Total source: 5,231 lines. `wallet-broker/src/zec/prepare.rs` must remain absent. Also require:

| Protected path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/tests/zec_scan.rs` | 325 | `084aa1758cc10bdd1f9fc63f935e70328aca1f2f668ff8f67234cef7f5656434` |
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

Require source-only and whole-worktree `git diff --check` success. Before execution record:

```text
hermes --version
hermes config get model.provider
hermes config get model.default
```

The actual values, not the adoption defaults, go in evidence. Inspect the wallet-broker filesystem
type and require disk-backed storage. Use only these existing ignored paths, creating them if
absent; never use `/tmp`:

```text
/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-tmp
/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-cargo
```

## Exact execution

Run once, in this order, from the repository root with no network:

```text
env TMPDIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-tmp CARGO_TARGET_DIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-cargo /home/lars/.cargo/bin/cargo +1.98.0 fmt --manifest-path wallet-broker/Cargo.toml --check
```

Require exit 0 without mutation.

```text
env TMPDIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-tmp CARGO_TARGET_DIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-cargo /home/lars/.cargo/bin/cargo +1.98.0 clippy --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --lib -- -D warnings
```

Require exit 0 without warning or diagnostic.

```text
env TMPDIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-tmp CARGO_TARGET_DIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-cargo /home/lars/.cargo/bin/cargo +1.98.0 test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_scan
```

Require exactly 9 passed, 0 failed, 0 ignored, 0 measured, and 0 filtered out.

```text
env TMPDIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-tmp CARGO_TARGET_DIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-cargo /home/lars/.cargo/bin/cargo +1.98.0 test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_store
```

Require exactly 8 passed, 0 failed, 0 ignored, 0 measured, and 0 filtered out.

```text
env TMPDIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-tmp CARGO_TARGET_DIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-cargo /home/lars/.cargo/bin/cargo +1.98.0 test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_address
```

Require exactly 8 passed, 0 failed, 0 ignored, 0 measured, and 0 filtered out.

```text
node test/securityPolicy.node.js
```

This is an expected partial red while broader Phase-C policy remains frozen. Require exit 1,
exactly 68 `ok`, exactly 6 `not ok`, final line `6 security policy test(s) failed`, and exactly:

1. `committed workflows satisfy the fail-closed checker`
2. `strict nine-line reviewed Gitleaks ratchet bytes and content are enforced`
3. `WAL-004 Rust source inventory is exported closed and enumerated by repository policy`
4. `WAL-006 feature policy distinguishes compiled upstream PCZT capability from BitBook authority`
5. `WAL-006 requires the exact bounded Phase-C ZEC production inventory`
6. `WAL-006 policy rejects live-network and authority-bearing Rust snippets without denying upstream transitives`

No other failure, warning, exception, or source-policy finding is accepted.

## Exact-success evidence and integration only

Only after all six results are exact, create `docs/testing/BBD-WAL-006-SCAN-GATE-01.md` and update
`CURRENT_TASK.md` to `PHASE-C SCAN GATE COMPLETE — REVIEW REQUIRED`. Record timestamp/timezone,
parent, filesystem/paths, actual Hermes identity, all preconditions, exact commands/exits/counts,
the expected policy-red names, and behaviors proven by all 25 Rust tests. Record no network,
real wallet/seed, mainnet, signing, proving, extraction, broadcast, node, or device activity. Do
not include raw fixture blocks, UFVK, receiver, note/nullifier data, SQLite rows, or user paths.

Recheck hashes, exact scope, and diff checks. Stage explicitly only the five accepted source files,
the new evidence, and `CURRENT_TASK.md`; inspect staged names/diff. Commit exactly
`feat: add WAL-006 compact block scanning`, push `master`, and prove `HEAD == origin/master`, clean
index, and clean tracked worktree. Never stage ignored targets.

Do not run any unlisted tool or edit source, tests, fixtures, dependencies, lock, policy, ticket,
workflow, package, other documentation, or another repository. Do not clean/delete artifacts. The
reviewer alone accepts the result and authorizes the next slice.
