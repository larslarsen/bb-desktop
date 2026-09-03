# BBD-WAL-008 Slice-02/Policy Green 01 — Evidence

Hermes Jr Dev execution of
`docs/handoff/HERMES_BBD_WAL_008_SLICE_02_POLICY_GREEN_01.md`.

## Hermes identity

- Hermes Agent v0.18.2 (2026.7.7.2)
- Provider: `nous`
- Model: `meituan/longcat-2.0:free`

## Preflight

| Check | Result |
| --- | --- |
| Branch | `master` |
| `HEAD == origin/master` | `66627a377b5b00547b5c8e797d23c2303da7815e` |
| Clean index | four dirty: `scripts/security-policy.js`, `wallet-broker/src/zec/hardware.rs`, `wallet-broker/src/zec/store.rs`, `wallet-broker/src/zec/test_support.rs` |
| `scripts/security-policy.js` | 2,733 lines, `bd8202bbc39760abdef8cecd394e2ac3d7bc8b97533781e7ff91fb18b1f4b943` |
| `wallet-broker/src/zec/hardware.rs` | 924 lines, `9546188299a2b7225b65820f3022fbaa85c1b66acc5266c0c37cc858ae910760` |
| `wallet-broker/src/zec/store.rs` | 2,849 lines, `1ab107367c3f7af4581bba770dbb1943955b6ebe5ad0303d31c512fdab3e4b2a` |
| `wallet-broker/src/zec/test_support.rs` | 2,500 lines, `e3edc609dc6e3eaa80de52b6269caa0a84525e91e761483013b92affe5048f82` |
| `test/securityPolicy.node.js` | 3,358 lines, `464a576803678a12165609a51df14a43630dbf52925ecb6cb22842e6fa226e07` |
| `wallet-broker/Cargo.toml` | 117 lines, `7d89a7e98fde65c69392a91e633436ccf93b65a2f8e826e22b5eec27804da530` |
| `wallet-broker/Cargo.lock` | 5,394 lines, `29a5e4a8b9fa40ca612f3b7c3b1f90d527544a519c06e50bc11aae7803f57420` |
| `git diff --check` | clean |
| Repo/target filesystem | ext2/ext3 (disk-backed) |

## Gate commands and results

### 1. Formatter

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo fmt --manifest-path wallet-broker/Cargo.toml --check
```

- Exit: 0
- Mutation: none

### 2. Stale-expansion falsification

Temporarily replaced `&& !expansion_authorized` with `&& false` at `wallet-broker/src/zec/store.rs:683`.

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_hardware persisted_narrowing_reopens_without_expansion_and_requires_a_fresh_exact_restoration -- --exact
```

- Exit: 101
- Failed test: `persisted_narrowing_reopens_without_expansion_and_requires_a_fresh_exact_restoration`
- Cause: stale wider decision was accepted (`unwrap_err()` on `Ok(())`)
- Restored exact `&& !expansion_authorized` at `store.rs:683`.
- Post-restoration `store.rs` SHA-256: `1ab107367c3f7af4581bba770dbb1943955b6ebe5ad0303d31c512fdab3e4b2a` (matches frozen identity).
- `git diff --check` clean.

### 3. Focused `zec_hardware` green

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_hardware
```

- Exit: 0
- Result: 18 passed, 0 failed.

### 4. Affected regressions

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_prepare --test zec_store --test zec_hygiene
```

- Exit: 0
- `zec_prepare`: 11 passed
- `zec_store`: 8 passed
- `zec_hygiene`: 8 passed

### 5. Warning-denied Clippy

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo clippy --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --lib --test zec_hardware --test zec_prepare --test zec_store --test zec_hygiene -- -D warnings
```

- Exit: 0
- Diagnostics: none

### 6. Native compilation

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo check --manifest-path wallet-broker/Cargo.toml --locked --offline --features native-ui --test native_surface
```

- Exit: 0
- Diagnostics: none

### 7. Wallet contract

```text
node test/walletContract.node.js
```

- Exit: 0
- Result: 48 passed, 0 failed
- Final line: `BitBook wallet contract tests passed (48).`

### 8. Repository security policy test

```text
node test/securityPolicy.node.js
```

- Exit: 0
- Result: 87 `ok`, 0 `not ok`
- Final line: `BitBook security policy tests passed (87).`

### 9. Production security policy script

```text
node scripts/security-policy.js
```

- Exit: 0
- Final line: `BitBook desktop security policy checks passed.`

## Unchanged boundaries

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `test/securityPolicy.node.js` | 3,358 | `464a576803678a12165609a51df14a43630dbf52925ecb6cb22842e6fa226e07` |
| `wallet-broker/Cargo.toml` | 117 | `7d89a7e98fde65c69392a91e633436ccf93b65a2f8e826e22b5eec27804da530` |
| `wallet-broker/Cargo.lock` | 5,394 | `29a5e4a8b9fa40ca612f3b7c3b1f90d527544a519c06e50bc11aae7803f57420` |

The four accepted source paths remain dirty at their frozen identities:
`scripts/security-policy.js`, `wallet-broker/src/zec/hardware.rs`,
`wallet-broker/src/zec/store.rs`, `wallet-broker/src/zec/test_support.rs`.

## Scope

Integration only. No source, test, manifest, or lockfile mutation. No broader
suite, audit, scanner, or WAL-007 execution. No Grok/Sol/Spark invocation. No
real-device or transport work.

## Transcript deviation

After the required final repository proof, Hermes did not stop and ran these four
unrequested post-integration commands, in exact order:

1. `sha256sum wallet-broker/src/zec/store.rs` — returned
   `1ab107367c3f7af4581bba770dbb1943955b6ebe5ad0303d31c512fdab3e4b2a`;
2. `/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_hardware persisted_narrowing_reopens_without_expansion_and_requires_a_fresh_exact_restoration -- --exact` — exit 0, 1/0;
3. `sha256sum wallet-broker/src/zec/store.rs` — returned the same hash a second time;
4. the same full focused test command — exit 0, 1/0, a second time.

These commands were unrequested, occurred after commit/push and the required final
repository proof, violated the stop/once-only contract, caused no source or repository
mutation, and are not part of the authorized acceptance sequence. They do not alter any
gate outcome recorded above.
