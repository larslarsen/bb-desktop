# Hermes Handoff — BBD-WAL-006 Prepare Final Gate 01

You are **Jr Dev — Hermes** using only the free configured Nous route. Own this one
execution/evidence/integration gate. Do not alter production or test source.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely: `AGENTS.md`, `TESTING.md`, `tickets/BBD-WAL-006.md`,
`docs/handoff/HERMES_BBD_WAL_006_PREPARE_ROLLBACK_GATE_01.md`,
`docs/handoff/HERMES_BBD_WAL_006_POST_PARSE_CORRECTION_GATE_01.md`, this handoff, all eight dirty
paths, `wallet-broker/tests/zec_hygiene.rs`, `test/securityPolicy.node.js`, and `CURRENT_TASK.md`.

## Preconditions

Record actual Hermes version/provider/model. Require `HEAD == origin/master`, exactly the eight
dirty paths below, clean `git diff --check`, and ext4 for both WAL-006 Cargo work directories.
Stop on mismatch.

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `scripts/security-policy.js` | 2,306 | `2c3d859ddd246b38972c835e604c70bd2bbff7b9266629a9c5c39c1b4d967cea` |
| `wallet-broker/Cargo.toml` | 82 | `47667b8f1970856096c2451ef70ff562be02ca360facf94b043c71ac30072735` |
| `wallet-broker/Cargo.lock` | 5,381 | `5c063b11e2882ab61bf126804cf8c3acef80ff5b4af16ac2fa59fe0ba6c85e01` |
| `wallet-broker/src/zec/prepare.rs` | 964 | `3cb1096107386bfcd7f30169ade545ec8a7073875dc1d8e631dd0d5104d7c08e` |
| `wallet-broker/src/zec.rs` | 252 | `1061adff987aefd8a641dfed11e06c85d0bc56ddb39f17a5c95d495d6aea387b` |
| `wallet-broker/src/zec/store.rs` | 2,105 | `5d05ce63a3da21d59ec3493624cd586a6d7de9e37bfaefba2ba91f697efa4ae1` |
| `wallet-broker/src/zec/test_support.rs` | 1,830 | `5a1c30199874ad13eb753deaf83c3080534ba495fdfaf225a3d8d03c6ef1ac77` |
| `wallet-broker/tests/zec_prepare.rs` | 416 | `c38339ab88a954f725c7341b4384f178078116de1c700e16892409c18eb2f3fa` |

## Exact commands

Run in order from the repository root and stop on the first mismatch.

1. `TMPDIR="$PWD/wallet-broker/target/wal006-tmp" CARGO_TARGET_DIR="$PWD/wallet-broker/target/wal006-cargo" /home/lars/.cargo/bin/rustup run 1.98.0 cargo fmt --manifest-path wallet-broker/Cargo.toml -- --check`
   — exit 0 and no diff.
2. `TMPDIR="$PWD/wallet-broker/target/wal006-tmp" CARGO_TARGET_DIR="$PWD/wallet-broker/target/wal006-cargo" /home/lars/.cargo/bin/rustup run 1.98.0 cargo clippy --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --lib --test zec_prepare --test zec_hygiene -- -D warnings`
   — exit 0 with no warning or diagnostic.
3. `TMPDIR="$PWD/wallet-broker/target/wal006-tmp" CARGO_TARGET_DIR="$PWD/wallet-broker/target/wal006-cargo" /home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_prepare`
   — exit 0, exactly 11 passed and 0 failed, including exact wallet-database byte immutability.
4. `TMPDIR="$PWD/wallet-broker/target/wal006-tmp" CARGO_TARGET_DIR="$PWD/wallet-broker/target/wal006-cargo" /home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_hygiene`
   — exit 0, exactly 8 passed and 0 failed.
5. `TMPDIR="$PWD/wallet-broker/target/wal006-tmp" CARGO_TARGET_DIR="$PWD/wallet-broker/target/wal006-cargo" /home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_fixture_builder --test zec_address --test zec_store --test zec_scan --test vault_crypto --test vault_format --test vault_store --test vault_session --test native_surface --test secret_hygiene`
   — exit 0, exactly 108 passed and 0 failed: 4 fixture, 8 address, 8 store, 9 scan,
   11 vault-crypto, 11 vault-format, 20 vault-store, 13 vault-session, 13 native-surface, and
   11 secret-hygiene.
6. `node test/securityPolicy.node.js`
   — exit 1, exactly 69 `ok`, exactly the six expected `not ok` below, and final line
   `6 security policy test(s) failed`. The WAL-006 manifest-feature policy test must be green.

The exact six accepted Node failures are:

- `committed workflows satisfy the fail-closed checker`;
- `strict nine-line reviewed Gitleaks ratchet bytes and content are enforced`;
- `WAL-004 Rust source inventory is exported closed and enumerated by repository policy`;
- `WAL-006 feature policy distinguishes compiled upstream PCZT capability from BitBook authority`;
- `WAL-006 requires the exact bounded Phase-C ZEC production inventory`;
- `WAL-006 policy rejects live-network and authority-bearing Rust snippets without denying upstream transitives`.

Any formatter mutation, warning/failure, different count, lock mutation, network attempt,
additional Node failure, or green/failed-count mismatch is a stop.

## Evidence and integration

Only on the exact complete gate, use `apply_patch` to create
`docs/testing/BBD-WAL-006-PREPARE-ROLLBACK-GATE-01.md` and update only the leading
state/actor/active-handoff block of `docs/handoff/CURRENT_TASK.md`. Record all commands and exact
results, source identities, the earlier focused one-test pass, byte-exact database rollback,
official proposal/PCZT construction, one unsigned real Ironwood action plus one IO-finalized signed
padding action, negative capability, and final Git state.

Stage exactly the eight dirty paths plus those two evidence/governance paths. Commit exactly
`feat: add WAL-006 unsigned PCZT preparation`, push `master`, and prove a clean worktree/index with
`HEAD == origin/master`.

Do not amend, rebase, merge, force-push, clean, delete, modify source/test/manifest/policy/lock after
execution, run another Rust/Node command, use npm/audit/scanners/network/Electron/wallet/node/device,
or edit another path. Stop and report the exact result on any mismatch.
