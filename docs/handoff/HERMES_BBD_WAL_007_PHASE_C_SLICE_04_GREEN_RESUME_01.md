# Hermes Handoff — BBD-WAL-007 Phase-C Slice 4 Green Resume 01

You are **Jr Dev — Hermes**. This durable handoff is authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely before acting: `AGENTS.md`, `TESTING.md`, the development-role and
Hermes-routing policies, `tickets/BBD-WAL-007.md`, the Slice-3 decisions and
acceptance, all Slice-4 source reviews and correction handoffs, Slice-4 Green Stop
Review 01, Slice-4 Formatter Diff 01, Slice-4 Format Correction Source Review 01, the
complete accepted seven-path drop, the prior Slice-4 Green 01 handoff, and
`docs/handoff/CURRENT_TASK.md`.

## Sole task and first-mismatch stop

Integrate only the accepted Slice-4 source drop, perform the exact temporary lock
falsification, run only the commands below exactly once, record evidence, and
commit/push only on exact success. You are the execution, evidence, and Git actor. Do
not design, repair, or format source; change/wrap/repeat a command; begin Slice 5; run
the real local-Monero gate; or touch another repository.

Stop immediately on the first parent/index/path/hash mismatch, formatter failure or
mutation, unexpected falsification result, green command failure, unexpected count,
warning/diagnostic, unapproved mutation, or command-scope deviation. Restore the
temporary falsification if applied, prove the accepted source hash, report the exact
result, and on a stop make no evidence edit, staging, commit, or push.

## Protected preconditions

Require `HEAD == origin/master ==` the protected governance parent, a clean index,
`git diff --check`, and exactly these seven worktree paths/identities with no other
tracked or untracked path:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/vault.rs` | 776 | `bb79b16f24a9abbcd266e856a37ee982bde8fcdc2da3858e75d34bdd34a4af86` |
| `wallet-broker/src/xmr.rs` | 7 | `8b382ef673e82001720727c161f365c0fa3c1401d1bdaa4d20ee2683c001559b` |
| `wallet-broker/src/xmr/account.rs` | 3,039 | `a014af7f26f257511b534d8dd96d74c0d87c2c15eb09e21b8f9f0ed217db7499` |
| `wallet-broker/src/xmr/store.rs` | 1,380 | `21ef2db4eaf32389809a86bcc3c0c8164ac57763ac7567c35c6f2007abb86749` |
| `wallet-broker/src/xmr/process.rs` | 1,803 | `aec5e5cc8bf93be3ee86888aa1ea5209ceed9a7ce229c3ab2fd9e0935d85688c` |
| `wallet-broker/src/xmr/rpc.rs` | 2,413 | `7f5019c9f4fb668a8f68bdf06f8ad8f20433890cef299b458f00f515b3c89965` |
| `wallet-broker/src/xmr/test_support.rs` | 3,918 | `20bcf14c992f88733082034de0c7ea5f91ec0f1f77764a576dbef17d8847ec53` |

Also require every committed/frozen identity in the prior Slice-4 Green 01 handoff,
including the 586-line account test at SHA-256
`5e8afabaa8c820be84a010a88fef0e07617e67934c06b32ab445901b1ddad35b`.

Record separately before execution:

```text
hermes --version
hermes config get model.provider
hermes config get model.default
```

Require `wallet-broker/target` to be disk-backed. Use only existing repository
target/cache state. Do not use `/tmp`, download anything, access the network, inspect
a personal Monero path, or start a product/Monero binary.

## Formatter and exact falsification

Run once, exactly as written and with no wrapper, redirection, or pipeline:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo fmt --manifest-path wallet-broker/Cargo.toml --check
```

Require exit 0, no output diagnostic, and no source/test mutation. Then temporarily
replace exactly this line in `AccountManager::lock` in
`wallet-broker/src/xmr/account.rs`:

```rust
        let teardown = if kind == AccountKind::Software || self.port.active_child_count() > 0 {
```

with exactly:

```rust
        let teardown = if false && (kind == AccountKind::Software || self.port.active_child_count() > 0) {
```

Run once, exactly as written and with no wrapper, redirection, or pipeline:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test xmr_account software_lock_closes_and_stops_child_while_watch_only_retention_is_post_open_only
```

Require exit 101, no warning or compile diagnostic, and exactly the selected test
failing with 0 passed, 0 ignored, 0 measured, and 15 filtered out because software
lock did not perform teardown. Restore the original line immediately and prove
`wallet-broker/src/xmr/account.rs` is restored byte-for-byte to 3,039 lines and
SHA-256 `a014af7f26f257511b534d8dd96d74c0d87c2c15eb09e21b8f9f0ed217db7499`.
Never stage or commit the temporary mutation.

## Exact green sequence

After exact restoration, run each command exactly once, in order, with no wrapper,
redirection, or pipeline:

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

Require respectively: 16, 9, 15, 12, 12, 11, 11, 20, 13, 11, and 17 tests passed,
each with zero failed/ignored/measured/filtered; focused Clippy exit 0 with warnings
denied; native check exit 0 without warning/diagnostic; Node policy exit 0 with exactly
86 `ok`, no `not ok`, and final `BitBook security policy tests passed (86).`; and the
security script exit 0 with final `BitBook desktop security policy checks passed.` No
accepted source/test file may mutate.

## Exact-success integration

Only on exact success, create `docs/testing/BBD-WAL-007-SLICE-04-GREEN-01.md`. Record
Hermes version/provider/model, the disk-backed filesystem fact, protected identities,
formatter/no-mutation, falsification/restoration proof, every normalized green result
and count, scope, and prohibited-action confirmation. Record no local Monero path,
cache/artifact path, environment value, port, credential, process ID, seed, primary
address, receiver, or raw sensitive output.

Update `docs/handoff/CURRENT_TASK.md` to
`PHASE C SLICE 4 GREEN 01 COMPLETE — REVIEW REQUIRED`, linking the evidence and
retaining all ticket, architecture, prior acceptance, Slice-4 review/correction, role,
routing, and prior-ticket records.

Recheck identities, `git diff --check`, and staged scope. Stage explicitly only the
seven accepted source paths, the green evidence, and `CURRENT_TASK.md`. Commit exactly:

```text
feat: add BBD-WAL-007 Monero account custody
```

Push `master`. After commit/push, run no formatter, test, check, Clippy, Node/npm,
policy/security, build, or product command. Use only read-only Git/identity commands
to prove `HEAD == origin/master`, a clean index/worktree, the exact commit message,
and nine-path commit scope. Then stop for reviewer acceptance. Slice 5,
broader/final acceptance, and the real offline local-Monero gate remain unauthorized.
