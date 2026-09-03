# Hermes Handoff — BBD-WAL-008 Phase-C Slice-02 Green Resume 03

You are **Jr Dev — Hermes**. Repository: `/home/lars/OpenBazaar/bb-desktop`.

Protected governance parent: the commit containing this handoff

Read completely: `AGENTS.md`, `TESTING.md`, both role/routing policies,
`docs/handoff/CURRENT_TASK.md`, `tickets/BBD-WAL-008.md`, Slice-02 Source Review 01,
Green Resume 02 Stop Review 01, Clippy Correction Source Review 01, the complete
`zec_hardware` test, and the three frozen source paths.

## Frozen source identities

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/zec/hardware.rs` | 924 | `9546188299a2b7225b65820f3022fbaa85c1b66acc5266c0c37cc858ae910760` |
| `wallet-broker/src/zec/store.rs` | 2,849 | `1ab107367c3f7af4581bba770dbb1943955b6ebe5ad0303d31c512fdab3e4b2a` |
| `wallet-broker/src/zec/test_support.rs` | 2,500 | `e3edc609dc6e3eaa80de52b6269caa0a84525e91e761483013b92affe5048f82` |

Preflight records Hermes identity/provider/model, branch, exact
`HEAD == origin/master`, clean index, only these three worktree paths, exact hashes,
unchanged lockfile, and clean `git diff --check`. Stop on any mismatch.

## Exact gate

Submit every fenced command byte-for-byte, alone, once, sequentially, with no wrapper,
redirection, pipeline, environment prefix, or appended shell text.

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo fmt --manifest-path wallet-broker/Cargo.toml --check
```

Require exit 0 and no mutation. Then replace only the final condition line in the
`store.rs` let chain:

```text
    && !expansion_authorized
```

with:

```text
    && false
```

Run exactly:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_hardware persisted_narrowing_reopens_without_expansion_and_requires_a_fresh_exact_restoration -- --exact
```

Require exit 101 with exactly that test failing because the stale wider decision was
accepted. Regardless of outcome, immediately restore `&& !expansion_authorized` and
confirm all three frozen hashes plus `git diff --check`. If failure or restoration is
not exact, stop without another execution command.

Run each command exactly once in this order:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_hardware
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_prepare --test zec_store --test zec_hygiene
/home/lars/.cargo/bin/rustup run 1.98.0 cargo clippy --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --lib --test zec_hardware --test zec_prepare --test zec_store --test zec_hygiene -- -D warnings
/home/lars/.cargo/bin/rustup run 1.98.0 cargo check --manifest-path wallet-broker/Cargo.toml --locked --offline --features native-ui --test native_surface
node test/walletContract.node.js
node test/securityPolicy.node.js
node scripts/security-policy.js
```

Require: focused 18/0; affected Rust 11/8/8 for
`zec_prepare`/`zec_store`/`zec_hygiene`; wallet contract 48; compile gates exit 0
without warnings/diagnostics; Node policy exactly 86 `ok` and no `not ok`; and security
script final line `BitBook desktop security policy checks passed.` No accepted source,
test, manifest, lockfile, or policy file may mutate. Stop at the first mismatch.

## Integration

Only after every exact outcome, create
`docs/testing/BBD-WAL-008-SLICE-02-GREEN-01.md` and update
`docs/handoff/CURRENT_TASK.md` to await reviewer acceptance. Record full literal
commands/exits/counts, identities, falsification/restoration, unchanged lockfile, and
scope. Stage exactly the three source paths plus those two records. Commit exactly
`feat: persist Zcash hardware decisions`, push `master`, prove a clean
`HEAD == origin/master`, and stop.

Do not repair source, edit tests, run broader full-suite/audit/scanner gates, use Grok,
invoke another actor, run product/device/network commands, touch Monero/WAL-007, or
start transport/signing/real-device work. On any stop, do not create evidence,
integrate, commit, push, rerun, or issue post-mismatch verification commands.
