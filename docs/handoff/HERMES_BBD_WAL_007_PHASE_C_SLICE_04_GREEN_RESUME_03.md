# Hermes Handoff — BBD-WAL-007 Phase-C Slice 4 Green Resume 03

You are **Jr Dev — Hermes**. This durable handoff is authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely before acting: `AGENTS.md`, `TESTING.md`, the development-role and
Hermes-routing policies, `tickets/BBD-WAL-007.md`, the original Slice-4 Green 01
handoff, Green Resume 02 Rejection 01, Green Correction 02 Source Review 01, and
`docs/handoff/CURRENT_TASK.md`.

## Non-negotiable execution protocol

Run commands sequentially, exactly as printed, never concurrently. Do not add a shell
wrapper, chain, redirection, pipeline, `tail`, masking clause, or repeat. On the first
unexpected exit, warning, diagnostic, count, output, or mutation: restore the temporary
falsification if present, perform only minimal read-only identity proof, and stop with
no evidence edit, staging, commit, or push. Never characterize a mismatch as harmless.

You may execute, format-check, record evidence, and integrate. You may not design or
repair source/tests, edit tests, invoke another actor, run the real local-Monero gate,
or begin Slice 5.

## Protected identities

Require `HEAD == origin/master ==` the protected parent, a clean index, and exactly
these seven worktree source paths with no other worktree path:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/vault.rs` | 776 | `bb79b16f24a9abbcd266e856a37ee982bde8fcdc2da3858e75d34bdd34a4af86` |
| `wallet-broker/src/xmr.rs` | 7 | `8b382ef673e82001720727c161f365c0fa3c1401d1bdaa4d20ee2683c001559b` |
| `wallet-broker/src/xmr/account.rs` | 3,034 | `318ca5ce58f0ced19d974155bdb66f3ecce915f7a600f99138b6f853d72348d8` |
| `wallet-broker/src/xmr/store.rs` | 1,329 | `19ac8891fb4deaf3cc323bb74647a5490c4684794171c0a262e9378ff51ecaea` |
| `wallet-broker/src/xmr/process.rs` | 1,748 | `8b373c6a984608f4689c7d8a210dd68a586d64c8bd470f05c2104641050944a0` |
| `wallet-broker/src/xmr/rpc.rs` | 2,413 | `95b6795969967d608efae322fce17fa81ac805830307170c7c6e69196f5cdf47` |
| `wallet-broker/src/xmr/test_support.rs` | 4,782 | `e422ed545d8c96127c240e64d899ca536f7bd9a454d5da03ea980a32013cb3b6` |

Also require every frozen identity from the original Green 01 handoff, especially
`xmr_account.rs` at 586 lines / `5e8afabaa8c820be84a010a88fef0e07617e67934c06b32ab445901b1ddad35b`
and `xmr_hygiene.rs` at 329 lines / `3653eba660be481d71e75185e024ae7e0a17f8754089c83b21edb397da84230f`.
Record Hermes version, `model.provider`, and `model.default` with three separate exact
commands. Require existing `wallet-broker/target` to be disk-backed. No `/tmp`,
download, network, personal Monero path, or product/Monero binary is authorized.

## Formatter and falsification

Run once verbatim:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo fmt --manifest-path wallet-broker/Cargo.toml --check
```

Require exit 0, no output diagnostic, and no mutation. Directly replace only this line:

```rust
        let teardown = if kind == AccountKind::Software || self.port.active_child_count() > 0 {
```

with:

```rust
        let teardown = if false && (kind == AccountKind::Software || self.port.active_child_count() > 0) {
```

Run once verbatim:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test xmr_account software_lock_closes_and_stops_child_while_watch_only_retention_is_post_open_only
```

Require exit 101, zero warnings/compile diagnostics, and only the selected intended
assertion failure: 0 passed, 1 failed, 0 ignored/measured, 15 filtered. Restore the
original line immediately and prove the accepted account identity above.

## Exact green sequence

Run each once, sequentially and verbatim, stopping on the first mismatch:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test xmr_account
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test xmr_hygiene
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test xmr_rpc
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test xmr_process
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test xmr_distribution
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test vault_crypto
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test vault_format
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test vault_store
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test vault_session
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test secret_hygiene
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test native_surface
/home/lars/.cargo/bin/rustup run 1.98.0 cargo clippy --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --lib --test xmr_account -- -D warnings
/home/lars/.cargo/bin/rustup run 1.98.0 cargo check --manifest-path wallet-broker/Cargo.toml --locked --offline --features native-ui --test native_surface
node test/securityPolicy.node.js
node scripts/security-policy.js
```

Require test totals respectively 16, 9, 15, 12, 12, 11, 11, 20, 13, 11, and 17,
all other test counts zero; Clippy and native check exit 0 without warning/diagnostic;
the first Node command exactly 86 `ok`, no `not ok`, and its expected final line; the
second exits 0 with its expected final line. No accepted source/test may mutate.

## Exact-success integration

Only on exact success, create `docs/testing/BBD-WAL-007-SLICE-04-GREEN-01.md`, update
`CURRENT_TASK.md` to `PHASE C SLICE 4 GREEN 01 COMPLETE — REVIEW REQUIRED`, and record
identity, disk-backed target, formatter, falsification/restoration, normalized results,
scope, and prohibited-action compliance without sensitive values.

Stage explicitly only the seven source paths, evidence, and `CURRENT_TASK.md`. Commit
exactly `feat: add BBD-WAL-007 Monero account custody`, push `master`, then perform only
read-only Git/identity confirmation. Stop for XHigh review. Sol, Grok, Slice 5,
broader/final acceptance, and the real local-Monero gate remain unauthorized.
