# BBD-WAL-007 Slice-4 Green Resume 07 Evidence

## Identity and routing

- Hermes version: `Hermes Agent v0.18.2 (2026.7.7.2) · upstream 593aa74c · local 10b6d1a9 (+1 carried commit)`
- Provider: `nous`
- Model: `meituan/longcat-2.0:free`
- Repository: `/home/lars/OpenBazaar/bb-desktop`
- Protected governance parent: `c62ac2f68dfcdf500d95240e10251b04b78a1002`
- Disk-backed target filesystem: `ext2/ext3` (confirmed via `df -T`)

## Protected preconditions (all verified before execution)

`HEAD == origin/master == c62ac2f68dfcdf500d95240e10251b04b78a1002` — clean index — exact eight worktree paths with no other tracked/untracked path — `git diff --check` clean.

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/vault.rs` | 776 | `bb79b16f24a9abbcd266e856a37ee982bde8fcdc2da3858e75d34bdd34a4af86` |
| `wallet-broker/src/xmr.rs` | 7 | `8b382ef673e82001720727c161f365c0fa3c1401d1bdaa4d20ee2683c001559b` |
| `wallet-broker/src/xmr/account.rs` | 3,034 | `67cc2261c138b83f3fa963bfe6ce646bea17c9258185d986a4c43daf0662c137` |
| `wallet-broker/src/xmr/distribution.rs` | 914 | `163f8532bc7edfd80fc07966c0f8f32eebc0d12181fd273bc4e6c2870d86dea8` |
| `wallet-broker/src/xmr/store.rs` | 1,327 | `248ca3f6eaeb98b66fbe2d041637c521f3b2371b8b9c231cbcdd3d3c57174607` |
| `wallet-broker/src/xmr/process.rs` | 1,763 | `98a18be4a0f26ae71b5818ba893910d3183a3ddea49263c9291185fbde09fc2f` |
| `wallet-broker/src/xmr/rpc.rs` | 2,426 | `59a0f33f66cb65a007a96f7f4e073a987a3b8c0e123d7f59624e8d442bf6f56b` |
| `wallet-broker/src/xmr/test_support.rs` | 4,771 | `5ef016587b6eeffa146ee8a38baae42b57eaf988755eb85c2d96076c8ffa2502` |

### Frozen committed/frozen identities (verified)

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/tests/xmr_account.rs` | 586 | `5e8afabaa8c820be84a010a88fef0e07617e67934c06b32ab445901b1ddad35b` |
| `wallet-broker/tests/xmr_hygiene.rs` | 329 | `3653eba660be481d71e75185e024ae7e0a17f8754089c83b21edb397da84230f` |

## Formatter result

`/home/lars/.cargo/bin/rustup run 1.98.0 cargo fmt --manifest-path wallet-broker/Cargo.toml --check` — exit 0, no output diagnostic, no mutation.

## Temporary falsification and restoration

Applied exactly:
```rust
let teardown = if false && (kind == AccountKind::Software || self.port.active_child_count() > 0) {
```

Falsification test: exit 101, zero warnings/compile diagnostics, only the selected intended assertion failure (0 passed, 1 failed, 0 ignored/measured, 15 filtered).

Restored exactly to:
```rust
let teardown = if kind == AccountKind::Software || self.port.active_child_count() > 0 {
```

Restored identity verified: `wallet-broker/src/xmr/account.rs` at 3,034 lines / SHA-256 `67cc2261c138b83f3fa963bfe6ce646bea17c9258185d986a4c43daf0662c137`.

## Exact green sequence results

1. `cargo test --test xmr_account` — 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered
2. `cargo test --test xmr_hygiene` — 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered
3. `cargo test --test xmr_rpc` — 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered
4. `cargo test --test xmr_process` — 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered
5. `cargo test --test xmr_distribution` — 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered
6. `cargo test --test vault_crypto` — 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered
7. `cargo test --test vault_format` — 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered
8. `cargo test --test vault_store` — 20 passed; 0 failed; 0 ignored; 0 measured; 0 filtered
9. `cargo test --test vault_session` — 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered
10. `cargo test --test secret_hygiene` — 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered
11. `cargo test --test native_surface` — 17 passed; 0 failed; 0 ignored; 0 measured; 0 filtered
12. `cargo clippy --lib --test xmr_account -- -D warnings` — exit 0, no warning/diagnostic
13. `cargo check --features native-ui --test native_surface` — exit 0, no warning/diagnostic
14. `node test/securityPolicy.node.js` — exit 0, exactly 86 `ok`, no `not ok`, final line `BitBook security policy tests passed (86).`
15. `node scripts/security-policy.js` — exit 0, final line `BitBook desktop security policy checks passed.`

## Scope and prohibited-action compliance

- All Rust commands emitted no warning or diagnostic.
- No accepted source/test file mutated during the green sequence.
- Only the eight authorized worktree paths plus evidence and `CURRENT_TASK.md` are staged.
- No repair, source/test edit, another actor, Slice 5, broader/final acceptance, or real local-Monero gate was invoked.
- No `/tmp`, download, network, personal Monero path, or product/Monero binary was used.
