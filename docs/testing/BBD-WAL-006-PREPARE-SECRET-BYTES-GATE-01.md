# BBD-WAL-006 Prepare Secret-Bytes Gate 01 — Evidence

Jr Dev — Hermes integration gate evidence for the WAL-006 prepare
secret-bytes thread-safety transition.

## Hermes identity

- Version: Hermes Agent v0.18.2 (2026.7.7.2) · upstream 21b2095d · local 10b6d1a9 (+1 carried commit)
- Provider: nous
- Model: meituan/longcat-2.0:free

## Protected parent

`128a88d4` — `docs: authorize WAL-006 secret bytes gate`. Verified
`HEAD == origin/master == 128a88d4` before any command.

## Source baseline verification

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/vault.rs` | 773 | `500cd2f91ec0a2e0052779ba6b2357053ce0bea1d644fb2c35066f768f363fe0` |
| `wallet-broker/tests/secret_hygiene.rs` | 281 | `dcebe361c7061b06b9ec6bb6fbea88ccb208069f7360ae14e2d7c6ca83b433a4` |

Single-path working-tree diff: only `wallet-broker/src/vault.rs`
(29 insertions, 15 deletions). `git diff --check` clean.

## Frozen lock (pre-command)

5,379 lines — `9a6166ef2b39b47aa41b7a77cc3054dd8aee481f5a198a1ad4e4882111f97f59`.

## Disk-backed target directories

`wallet-broker/target/wal006-tmp` and `wallet-broker/target/wal006-cargo`
exist on ext4 (`/dev/mapper/ubuntu--vg-ubuntu--lv`), not tmpfs.

## Command results

### 1 — Format check

```
TMPDIR="$PWD/wallet-broker/target/wal006-tmp" CARGO_TARGET_DIR="$PWD/wallet-broker/target/wal006-cargo" rustup run 1.98.0 cargo fmt --manifest-path wallet-broker/Cargo.toml -- --check
```

Exit 0. No diff.

### 2 — Clippy

```
TMPDIR="$PWD/wallet-broker/target/wal006-tmp" CARGO_TARGET_DIR="$PWD/wallet-broker/target/wal006-cargo" rustup run 1.98.0 cargo clippy --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --lib --test secret_hygiene -- -D warnings
```

Exit 0. No warning or diagnostic.

### 3 — Targeted green (exact)

```
TMPDIR="$PWD/wallet-broker/target/wal006-tmp" CARGO_TARGET_DIR="$PWD/wallet-broker/target/wal006-cargo" rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test secret_hygiene secret_bytes_can_be_owned_by_synchronized_account_state -- --exact
```

Exit 0. Exactly 1 passed, 0 failed.

### 4 — Secret-hygiene suite

```
TMPDIR="$PWD/wallet-broker/target/wal006-tmp" CARGO_TARGET_DIR="$PWD/wallet-broker/target/wal006-cargo" rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test secret_hygiene
```

Exit 0. Exactly 11 passed, 0 failed.

### 5 — Broader acceptance (56 total)

```
TMPDIR="$PWD/wallet-broker/target/wal006-tmp" CARGO_TARGET_DIR="$PWD/wallet-broker/target/wal006-cargo" rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test vault_crypto --test vault_format --test vault_session --test native_surface --test zec_address
```

Exit 0. Exactly 56 passed in total, 0 failed
(vault_crypto 11, vault_format 11, vault_session 13, native_surface 13,
zec_address 8).

### 6 — Node security policy

```
node test/securityPolicy.node.js
```

Exit 1. Exactly 69 `ok`, exactly 6 `not ok`, final line
`6 security policy test(s) failed`. No new failure.

The six frozen Phase-C failures:
- committed workflows satisfy the fail-closed checker
- strict nine-line reviewed Gitleaks ratchet bytes and content are enforced
- WAL-004 Rust source inventory is exported closed and enumerated by repository policy
- WAL-006 feature policy distinguishes compiled upstream PCZT capability from BitBook authority
- WAL-006 requires the exact bounded Phase-C ZEC production inventory
- WAL-006 policy rejects live-network and authority-bearing Rust snippets without denying upstream transitives

## Prior red-to-green transition

The expected-red compile failure (the old `SecretBytes` carrying
`drop_observer: Option<(&'static str, Box<dyn WipeObserver>)>`) was already
integrated as falsification evidence. The green transition splits the type
into plain `SecretBytes` (now `Send + Sync`) and `ObservedSecretBytes`
(observer-carrying, single-owner). The auto-trait test
`secret_bytes_can_be_owned_by_synchronized_account_state` asserts
`Send + Sync` for `SecretBytes` and now passes.

## Observer regression

Command 4 proves the existing `Rc<RefCell<_>>` observed-drop regression
still records one real post-zeroize event:
`observed_secret_drop_reports_post_wipe_state_not_predeclared_success`
passes, and `assert_all_zero` confirms the `drop-secret` event carries
`all_zero == true` with the correct length.

## Auto-trait result

`assert_send_sync::<SecretBytes>()` compiles and runs — `SecretBytes` is
`Send + Sync`. The `Box<dyn WipeObserver>` observer was moved out of
`SecretBytes` into `ObservedSecretBytes`, removing the `!Send`/`!Sync`
contamination from the boxed trait object stored inline.

## Negative capability

No network access. No Cargo/Rust/Node command beyond the six authorized.
No npm, audit, scanner, Electron, wallet/node/device, fixture,
cleanup, or deletion. No amend, rebase, merge, or force-push. No edit
to Cargo/lock, ZEC source/tests, policy implementation, ticket,
fixture, workflow, package, or another path.
