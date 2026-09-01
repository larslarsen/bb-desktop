# Hermes Handoff — BBD-WAL-006 Prepare Gate Resume 05

You are **Jr Dev — Hermes** using only the free configured Nous route. Own only this
execution/evidence/integration gate from command 1.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely: `AGENTS.md`, `TESTING.md`, `tickets/BBD-WAL-006.md`, the original Prepare Gate
01 handoff, Prepare Gate Resume 04, Prepare Gate Clippy Review 02, Prepare Clippy Correction Review
01, Prepare Serde Lock Capture Review 01, this handoff, all seven changed paths, both frozen tests,
and `CURRENT_TASK.md`.

## Preconditions

Record actual Hermes version/provider/model, protected `HEAD`, `origin/master`, status, exact
seven-path diff inventory including untracked `prepare.rs`, every identity below,
`git diff --check`, and ext4 location for both WAL-006 Cargo work directories. Stop on any
mismatch or extra path.

Accepted identities:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `scripts/security-policy.js` | 2,306 | `2c3d859ddd246b38972c835e604c70bd2bbff7b9266629a9c5c39c1b4d967cea` |
| `wallet-broker/Cargo.toml` | 82 | `47667b8f1970856096c2451ef70ff562be02ca360facf94b043c71ac30072735` |
| `wallet-broker/Cargo.lock` | 5,381 | `5c063b11e2882ab61bf126804cf8c3acef80ff5b4af16ac2fa59fe0ba6c85e01` |
| `wallet-broker/src/zec/prepare.rs` | 963 | `3c5a64d718ab108bc91186a7d709c858cb9cc643349563019b12f1578a0928ca` |
| `wallet-broker/src/zec.rs` | 252 | `1061adff987aefd8a641dfed11e06c85d0bc56ddb39f17a5c95d495d6aea387b` |
| `wallet-broker/src/zec/store.rs` | 2,048 | `f9f66f98f33b8457c955125b77453be018397ab120f78618d52ed817200fcf34` |
| `wallet-broker/src/zec/test_support.rs` | 1,830 | `5a1c30199874ad13eb753deaf83c3080534ba495fdfaf225a3d8d03c6ef1ac77` |
| `wallet-broker/tests/zec_prepare.rs` | 412 | `a616245fdcf8d2226277e34d785bb1792d3b8b54ebf268c3d1dc5f15566da942` |
| `wallet-broker/tests/zec_hygiene.rs` | 375 | `aad7c95a2ef661063661f2ec0f16a216d80328096b049d636b88fa0252ba1be6` |
| `test/securityPolicy.node.js` | 2,525 | `2c1bd92c778a975b1218dfd2445b6b1b605bb047032fd5289573cbbb6d0a0169` |

## Exact commands and expected results

Run in order and stop on the first mismatch.

1. `TMPDIR="$PWD/wallet-broker/target/wal006-tmp" CARGO_TARGET_DIR="$PWD/wallet-broker/target/wal006-cargo" rustup run 1.98.0 cargo fmt --manifest-path wallet-broker/Cargo.toml -- --check`
   — exit 0 and no diff.
2. `TMPDIR="$PWD/wallet-broker/target/wal006-tmp" CARGO_TARGET_DIR="$PWD/wallet-broker/target/wal006-cargo" rustup run 1.98.0 cargo clippy --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --lib --test zec_prepare --test zec_hygiene -- -D warnings`
   — exit 0 with no warning or diagnostic.
3. `TMPDIR="$PWD/wallet-broker/target/wal006-tmp" CARGO_TARGET_DIR="$PWD/wallet-broker/target/wal006-cargo" rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_prepare`
   — exit 0, exactly 11 passed, 0 failed.
4. `TMPDIR="$PWD/wallet-broker/target/wal006-tmp" CARGO_TARGET_DIR="$PWD/wallet-broker/target/wal006-cargo" rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_hygiene`
   — exit 0, exactly 8 passed, 0 failed.
5. `TMPDIR="$PWD/wallet-broker/target/wal006-tmp" CARGO_TARGET_DIR="$PWD/wallet-broker/target/wal006-cargo" rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_fixture_builder --test zec_address --test zec_store --test zec_scan --test vault_crypto --test vault_format --test vault_store --test vault_session --test native_surface --test secret_hygiene`
   — exit 0, exactly 108 passed in total and 0 failed: 4 fixture, 8 address, 8 store,
   9 scan, 11 vault-crypto, 11 vault-format, 20 vault-store, 13 vault-session, 13 native-surface,
   and 11 secret-hygiene.
6. `node test/securityPolicy.node.js`
   — exit 1, exactly 69 `ok`, exactly the six expected `not ok` from Resume 04, and final line
   `6 security policy test(s) failed`. The manifest-feature policy test must be green.

Any formatter mutation, warning/failure, different count, lock mutation, network attempt,
additional Node failure, or manifest-feature policy failure is a stop. Do not continue after a
stop.

## Evidence and integration

Only on the exact complete gate, use `apply_patch` to create
`docs/testing/BBD-WAL-006-PREPARE-GATE-01.md` and update only the leading
state/actor/active-handoff block of `docs/handoff/CURRENT_TASK.md`. Record every command verbatim,
exit/count, identity, disk location, all prior gate stops/corrections, real proposal/PCZT path
coverage, lifecycle/unwind coverage, read-only SQLite boundary, corrected Node transition, and
negative capability.

Stage exactly:

- `scripts/security-policy.js`
- `wallet-broker/Cargo.toml`
- `wallet-broker/Cargo.lock`
- `wallet-broker/src/zec/prepare.rs`
- `wallet-broker/src/zec.rs`
- `wallet-broker/src/zec/store.rs`
- `wallet-broker/src/zec/test_support.rs`
- `docs/testing/BBD-WAL-006-PREPARE-GATE-01.md`
- `docs/handoff/CURRENT_TASK.md`

Commit exactly `feat: add WAL-006 unsigned PCZT preparation`, push `master`, and prove a clean
worktree/index with `HEAD == origin/master`. Do not amend, rebase, merge, or force-push.

Do not modify source, test, manifest, policy, or lock after execution. Do not run another
Cargo/Rust/Node command, npm, audit, scanner, dependency resolution, network fetch, Electron,
wallet/node/device, fixture, falsification, cleanup, or deletion. Do not edit the ticket,
workflow, package, reviewer documents, or another path. Stop and report exact evidence on any
mismatch.
