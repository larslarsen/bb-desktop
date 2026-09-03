# Hermes Handoff — BBD-WAL-008 Slice-02 and Policy Green 01

You are **Jr Dev — Hermes**. Repository: `/home/lars/OpenBazaar/bb-desktop`.

Protected governance parent: the commit containing this handoff

Read completely: `AGENTS.md`, `TESTING.md`, both role/routing policies,
`docs/handoff/CURRENT_TASK.md`, `tickets/BBD-WAL-008.md`, Slice-02 Source Review 01,
Clippy Correction Source Review 01, Resume-03 Stop Review 01, Policy Test Source Review
01, Policy Expected-Red Acceptance 01, Policy Production Source Review 01, the complete
`zec_hardware` and policy tests, and all four frozen source paths.

Record actual Hermes version, provider, and model. Preflight must prove branch `master`,
exact `HEAD == origin/master` at the protected parent, clean index, exactly the four
dirty source paths below, all frozen identities, unchanged manifest/lockfile/test,
clean `git diff --check`, and disk-backed ext4 repository/Cargo work directories.

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `scripts/security-policy.js` | 2,733 | `bd8202bbc39760abdef8cecd394e2ac3d7bc8b97533781e7ff91fb18b1f4b943` |
| `wallet-broker/src/zec/hardware.rs` | 924 | `9546188299a2b7225b65820f3022fbaa85c1b66acc5266c0c37cc858ae910760` |
| `wallet-broker/src/zec/store.rs` | 2,849 | `1ab107367c3f7af4581bba770dbb1943955b6ebe5ad0303d31c512fdab3e4b2a` |
| `wallet-broker/src/zec/test_support.rs` | 2,500 | `e3edc609dc6e3eaa80de52b6269caa0a84525e91e761483013b92affe5048f82` |

Frozen unchanged boundaries:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `test/securityPolicy.node.js` | 3,358 | `464a576803678a12165609a51df14a43630dbf52925ecb6cb22842e6fa226e07` |
| `wallet-broker/Cargo.toml` | 117 | `7d89a7e98fde65c69392a91e633436ccf93b65a2f8e826e22b5eec27804da530` |
| `wallet-broker/Cargo.lock` | 5,394 | `29a5e4a8b9fa40ca612f3b7c3b1f90d527544a519c06e50bc11aae7803f57420` |

## Exact gate

Submit every fenced command byte-for-byte, alone, once, sequentially, with no `cd`,
wrapper, redirection, pipeline, environment prefix, or appended shell text.

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo fmt --manifest-path wallet-broker/Cargo.toml --check
```

Require exit 0 and no mutation. Then replace only the final condition line in the
`store.rs` narrowing let-chain from `&& !expansion_authorized` to `&& false`. Run:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_hardware persisted_narrowing_reopens_without_expansion_and_requires_a_fresh_exact_restoration -- --exact
```

Require exit 101 with that test alone failing because the stale wider decision was
accepted. Regardless of outcome, immediately restore `&& !expansion_authorized` and
confirm all four frozen source hashes plus `git diff --check`. If failure or restoration
is not exact, stop without another execution command.

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

Require: focused 18/0; affected `zec_prepare`/`zec_store`/`zec_hygiene` 11/8/8;
warning-denied Clippy and native check exit 0 without diagnostics; wallet contract
48/0; policy exactly 87 `ok`, zero `not ok`, final line
`BitBook security policy tests passed (87).`; and script exact final line
`BitBook desktop security policy checks passed.` No accepted source, test, manifest,
or lockfile may mutate. Stop at the first mismatch.

## Integration

Only after every exact result, create `docs/testing/BBD-WAL-008-SLICE-02-GREEN-01.md`
and update `docs/handoff/CURRENT_TASK.md` to await reviewer acceptance. Record all full
literal commands, exits/counts, falsification/restoration, identities, unchanged
boundaries, and scope. Stage exactly the four source paths plus those two records.
Commit exactly `feat: persist Zcash hardware decisions`, push `master`, prove clean
index/worktree and `HEAD == origin/master`, then stop.

Do not repair source, edit tests, run broader suites/audits/scanners, use Grok, invoke
another actor, touch Monero/WAL-007, or start transport/signing/real-device work. On any
stop, do not create evidence, integrate, commit, push, rerun, or issue post-mismatch
verification commands.
