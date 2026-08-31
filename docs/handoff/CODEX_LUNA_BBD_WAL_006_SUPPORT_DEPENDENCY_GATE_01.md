# Codex Luna Handoff — BBD-WAL-006 Support-Dependency Gate 01

You are **Jr Dev — Codex Luna**. This durable file is the complete prompt; ephemeral
chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff.

Accepted uncommitted source:

- `wallet-broker/Cargo.toml`: 81 lines, SHA-256
  `6643435bdf59608b09e906c8b7010baf1c17bbaa785d8eb70e37039d4bb37632`
- `scripts/security-policy.js`: 2,299 lines, SHA-256
  `60e41a12462d77c6be875f1659e5ef8a86d2b8146bd25d37ec7777297847d767`

Frozen inputs:

- `wallet-broker/Cargo.lock`: 5,367 lines, SHA-256
  `bf9a36a70c96afe3c0b68355f082342c3e625e4764782228be9f4663b635cd83`
- `test/securityPolicy.node.js`: 2,454 lines, SHA-256
  `f8340ae965a1f59e27594153cd3c9f3eca5ee1d8a78ed0a9a685faf6f1dc2647`

Read completely: `AGENTS.md`, `TESTING.md`, roles, ticket, support-dependency test/red
and both production-source reviews/handoffs, `CURRENT_TASK.md`, both changed source
files, the lock's root/rand_core/rusqlite entries, and the complete focused Node test.

## Preflight

Require `HEAD == origin/master` at the protected governance parent, clean index, and
exactly the two accepted modified source paths with exact hashes/lines. Require the
frozen test/lock hashes, no `wallet-broker/src/zec*` path, and no other tracked/untracked
path. Use only Rust/Cargo 1.98.0 via `/home/lars/.cargo/bin/rustup run 1.98.0`; no
network, native UI, wallet, node, device, Electron, or system temporary target/cache.

## Exact execution order

Run separately and preserve exact status/output:

```text
node test/securityPolicy.node.js
/home/lars/.cargo/bin/rustup run 1.98.0 cargo metadata --manifest-path wallet-broker/Cargo.toml --offline --format-version 1 --no-deps
/home/lars/.cargo/bin/rustup run 1.98.0 cargo tree --manifest-path wallet-broker/Cargo.toml --locked --offline -e features -i rand_core@0.6.4
/home/lars/.cargo/bin/rustup run 1.98.0 cargo tree --manifest-path wallet-broker/Cargo.toml --locked --offline -e features -i rusqlite@0.37.0
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test vault_crypto
```

The first Node command is an accepted partial-red gate: require exit 1, exactly 71 `ok`,
three `not ok`, and `3 security policy test(s) failed`. The only remaining groups may be
the compiled-PCZT-versus-BitBook-authority policy, exact bounded Phase-C ZEC source
inventory, and ZEC live-network/authority source screening. The workflows, Gitleaks,
WAL-004 manifest, six-Zcash-pin manifest, and two-support-pin manifest groups must be
green. Any different count/name/cause is a stop.

The offline metadata command may update only `wallet-broker/Cargo.lock`. Inspect the
entire lock diff before continuing. It must add exactly `rand_core` and `rusqlite` to the
`bitbook-wallet-broker` package's dependency array. No package block, version, checksum,
source, dependency edge outside that root array, or other lock byte may change. Both
packages must remain the existing crates.io `rand_core 0.6.4` checksum
`ec0be4795e2f6a28069bec0b5ff3e2ac9bafc99e6a9a7dc3547996c5c816922c`
and `rusqlite 0.37.0` checksum
`165ca6e57b20e1351573e3729b958bc62f0e48025386970b6e4d29e7a7e71f3f`.

The feature trees must show the direct `rand_core/std` path and the direct rusqlite
package without a direct feature; record the already-existing transitive rusqlite
feature union separately. They must show no new package, network, load-extension,
SQLCipher, vendored OpenSSL, or unexpected support authority. The locked/offline
`vault_crypto` target must pass all 11 tests with the accepted frozen vectors. Any
command failure, warning that changes the acceptance claim, or unexpected diff stops
without evidence/Git.

## Evidence and integration

If and only if exact, create only
`docs/testing/BBD-WAL-006-SUPPORT-DEPENDENCY-GATE-01.md` recording versions, every
command/status/count, the three intentional remaining policy reds, exact lock diff,
package checksums/sources/features, no-new-package/build-script/license conclusion,
11-test custody result, input/output hashes/lines, no ZEC source, and final state.

Update only `docs/handoff/CURRENT_TASK.md` to `SUPPORT-DEPENDENCY GATE INTEGRATED —
ADDRESS SOURCE REVIEW REQUIRED`, link evidence, and keep ZEC source frozen.

Inspect exact diff, then stage only:

- `wallet-broker/Cargo.toml`
- `wallet-broker/Cargo.lock`
- `scripts/security-policy.js`
- `docs/testing/BBD-WAL-006-SUPPORT-DEPENDENCY-GATE-01.md`
- `docs/handoff/CURRENT_TASK.md`

Commit once as `build: add WAL-006 support dependencies` and push `master`. Require
`HEAD == origin/master` and clean tracked worktree/index. Report commit, staged manifest,
all command results, lock/evidence hashes/lines, changed root dependencies, feature
summary, push, and final state.

Do not run npm, other Node tests, broader Cargo targets, fmt, clippy, audit, deny,
scanners, falsifications, builds, Electron, wallets, nodes, devices, network clients,
cleanup, or edit any unlisted path. Do not repair source or tests.
