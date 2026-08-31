# Codex Luna Handoff — BBD-WAL-006 Address Gate 01

You are **Jr Dev — Codex Luna**, using `gpt-5.6-luna`. This durable file is authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely: `AGENTS.md`, `TESTING.md`, roles, ticket, architecture, the original/resume/
correction address handoffs, Source Reviews 02 and 03, the complete accepted `zec_address` test,
fixture manifest, current six-path source drop, support-dependency final gate review, and
`CURRENT_TASK.md`.

## Protected preconditions

Require `HEAD == origin/master ==` the protected governance parent, a clean index, and worktree
changes consisting only of the six accepted production paths below at these exact bytes:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/lib.rs` | 11 | `ad61f92cce12115df9da6f263240ec0c57b5746e79cdcf6459f03731dab45617` |
| `wallet-broker/src/zec.rs` | 203 | `c86c030245e3caaec5182e4138f199a5bab08223c5c95ecb25b87745bbfa5e80` |
| `wallet-broker/src/zec/address.rs` | 206 | `16ebba57e1503bc8fecbc8727c676a19ff944633e254137de31744901a97fdce` |
| `wallet-broker/src/zec/fixture.rs` | 257 | `6c3a5368617dc0039c6d1da970a489f9e4fb4f4235bc39b32866fd085a33a715` |
| `wallet-broker/src/zec/store.rs` | 802 | `946fd7531bd34bfc2ac411d35d582c0333375c0f712de8847fb729a8bf6d8fc6` |
| `wallet-broker/src/zec/test_support.rs` | 378 | `9be9d676b5764ace0814786fdb7cc7fcb782bb365d21937a7d472f0efd69a3cc` |

Also require these protected inputs:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/tests/zec_address.rs` | 277 | `2c5012e6884c8c2a81236266c6861b6e4e4fd6b124656dad2ab438add5848ee3` |
| `wallet-broker/tests/fixtures/zec/manifest.json` | 238 | `0c5836405b29a2af618a9fa2784a973c981e807c3a8d46471aeeb90d8a6fe389` |
| `wallet-broker/Cargo.toml` | 81 | `6643435bdf59608b09e906c8b7010baf1c17bbaa785d8eb70e37039d4bb37632` |
| `wallet-broker/Cargo.lock` | 5,369 | `ac454889b0796afea3f2a2ddfaf8c585bfff1080bec6d1428d0c227534ffb9dd` |
| `scripts/security-policy.js` | 2,299 | `60e41a12462d77c6be875f1659e5ef8a86d2b8146bd25d37ec7777297847d767` |
| `test/securityPolicy.node.js` | 2,454 | `f8340ae965a1f59e27594153cd3c9f3eca5ee1d8a78ed0a9a685faf6f1dc2647` |

Require source-only and whole-worktree `git diff --check` success. Stop on any mismatch; do not
repair, format, regenerate, resolve dependencies, or edit source/test/policy.

## Exact execution

Inspect the filesystem type for `/home/lars/OpenBazaar/bb-desktop/wallet-broker`. Use only the
ignored disk-backed paths below, create them if absent, and do not use `/tmp`:

```text
/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-tmp
/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-cargo
```

Run each command once, in order, from the repository root with no network:

```text
env TMPDIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-tmp CARGO_TARGET_DIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-cargo /home/lars/.cargo/bin/cargo +1.98.0 fmt --manifest-path wallet-broker/Cargo.toml --check
```

It must exit 0 without mutation.

```text
env TMPDIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-tmp CARGO_TARGET_DIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-cargo /home/lars/.cargo/bin/cargo +1.98.0 clippy --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --lib -- -D warnings
```

It must exit 0 with no warning or diagnostic.

```text
env TMPDIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-tmp CARGO_TARGET_DIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-cargo /home/lars/.cargo/bin/cargo +1.98.0 test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_address
```

It must exit 0 with exactly 8 passed, 0 failed, 0 ignored, 0 measured, and 0 filtered out.

Then run once:

```text
node test/securityPolicy.node.js
```

This remains an expected partial red because broader Phase-C source/policy is frozen. Require exit
1 with exactly 69 `ok`, 5 `not ok`, and final `5 security policy test(s) failed`. The five failing
groups must be exactly:

1. `committed workflows satisfy the fail-closed checker`
2. `strict nine-line reviewed Gitleaks ratchet bytes and content are enforced`
3. `WAL-006 feature policy distinguishes compiled upstream PCZT capability from BitBook authority`
4. `WAL-006 requires the exact bounded Phase-C ZEC production inventory`
5. `WAL-006 policy rejects live-network and authority-bearing Rust snippets without denying upstream transitives`

The first two must fail only because the still-WAL-004 production inventory rejects the new
top-level `zec.rs`; the last three remain the accepted deferred Phase-C policy groups. No other
failure, exception, source-policy finding, or count is accepted.

## Evidence and integration

Only after all four results match, create
`docs/testing/BBD-WAL-006-ADDRESS-GATE-01.md` and update `docs/handoff/CURRENT_TASK.md` to
`PHASE-C ADDRESS GATE COMPLETE — REVIEW REQUIRED`. Record timestamp/timezone, protected parent,
filesystem type and exact ignored paths, commands/exits/counts, exact five expected-red groups,
source/protected hashes and lines, no-warning result, no network, no secret/wallet/node/device,
and the final integration state. Do not copy a seed, address, UFVK, canary, path containing user
data, or raw error payload into evidence.

Recheck all protected hashes, `git diff --check`, and exact scope. Stage explicitly only the six
accepted source paths, the new evidence, and `CURRENT_TASK.md`; inspect the staged list/diff.
Commit as `feat: add WAL-006 viewing address foundation`, push `master`, and prove
`HEAD == origin/master`, clean index, and clean tracked worktree. Do not stage ignored target
artifacts.

Do not run any unlisted command or test; do not edit production, tests, fixture, manifest,
lockfile, policy, workflow, ticket, package, or another repository. Do not clean or delete prior
artifacts. Stop and report the first mismatch without evidence, integration, commit, push, or
repair. XHigh owns result acceptance and the next source authorization.
