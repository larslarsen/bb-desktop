# Hermes Handoff — BBD-WAL-007 Phase-C Slice 3 Green Resume 04

You are **Jr Dev — Hermes**. This durable handoff is authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely before acting: `AGENTS.md`, `TESTING.md`, the development-role and
Hermes-routing policies, `tickets/BBD-WAL-007.md`, Slice-3 Upstream RPC Decision,
Slice-3 Source Review 03, all three Slice-3 Compile Correction Source Reviews, all
three prior Green Resume stop reviews, the complete accepted five-path drop, Green
Resume 03 handoff, and `docs/handoff/CURRENT_TASK.md`.

## Sole task and first-mismatch stop

Integrate only the accepted Slice-3 drop, perform the exact temporary falsification,
run only the commands below, record evidence, and commit/push only on exact success.
Do not design, repair, or format source; change a command; begin Slice 4; run the real
local-Monero gate; or touch another repository.

Stop immediately on any parent/index/path/hash mismatch, formatter failure or mutation,
unexpected falsification, command failure/count/warning/diagnostic, or unapproved
mutation. Restore the falsification if applied, prove the accepted hash, and on a stop
make no evidence edit, staging, commit, or push.

## Preconditions

Require `HEAD == origin/master ==` the protected governance parent, clean index,
`git diff --check`, and exactly these five worktree paths/identities with no other path:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/tests/xmr_rpc.rs` | 694 | `7b11f6fa6ecf29aa5556f1afc4bb7e3cd5e29018e9ff9eb1f40c5da05c3fafda` |
| `wallet-broker/src/xmr.rs` | 5 | `92af416c0ed6c5f2f8a6ff7011facb622da5e3f1ba8adaa2e96f600c09f31411` |
| `wallet-broker/src/xmr/model.rs` | 151 | `23a85dc216839cb936161845b52cacdca68a24181e5d31afeaaba30e62c77ca9` |
| `wallet-broker/src/xmr/rpc.rs` | 1,896 | `2186f6fd56cfcce1edfbe00cd247ef2ab2177ba04dfd9b505416a1da33cbe4ed` |
| `wallet-broker/src/xmr/test_support.rs` | 2,704 | `f3ff67c4958ab66f1167779639667611d9117f3c594aa4140c6e8f73fc9f3130` |

Also require these frozen identities:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/Cargo.toml` | 113 | `189f067739afd8bc1e775e4d58b3ce7d4cfa6ab53e7b7acc5b02737396c5bff5` |
| `wallet-broker/Cargo.lock` | 5,394 | `29a5e4a8b9fa40ca612f3b7c3b1f90d527544a519c06e50bc11aae7803f57420` |
| `wallet-broker/src/xmr/distribution.rs` | 914 | `4135db83eca151185d8222498d2435ef56e2d564ff17bd3fa88481260758aed5` |
| `wallet-broker/src/xmr/process.rs` | 1,184 | `7533a86344c178ed25143d21bc3961a72ea7a1fcf85067f90b57134178928a1f` |
| `wallet-broker/tests/xmr_distribution.rs` | 297 | `481042e6dabe705fb7771c0cf692f0561298a9e0e8aa6d0cfd1d8f2ab9deb6f8` |
| `wallet-broker/tests/xmr_process.rs` | 453 | `25166407a510fef113c5c1db948243ef60de884658f7a9545f49524af49da833` |
| `wallet-broker/tests/native_surface.rs` | 664 | `349f3a019a0e7a5c37aaae727192b0ddcecb02192a5fae6df9291eaf1357276d` |
| `scripts/security-policy.js` | 2,689 | `6aba356098134e90731928a2a1083565d52a6d000c213b89549ba231997f5626` |
| `test/securityPolicy.node.js` | 3,189 | `dd22ab83645d5dffb9d695cd21f15175ffe109b6020142e1bff5ca8d60e0497` |

Record separately before execution:

```text
hermes --version
hermes config get model.provider
hermes config get model.default
```

Require `wallet-broker/target` to be disk-backed. Use only existing repository cache/
target state. No `/tmp`, download, network, personal Monero path, or product/Monero
binary.

## Formatter and falsification

Run once:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo fmt --manifest-path wallet-broker/Cargo.toml --check
```

Require exit 0 and no mutation. Then temporarily remove exactly this line from
`evaluate_node_policy` in `wallet-broker/src/xmr/rpc.rs`:

```rust
        || !info.bootstrap_daemon_address.is_empty()
```

Run exactly:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test xmr_rpc node_syncing_is_distinct_from_bootstrap_remote_and_unavailable
```

Require exit 101 with exactly that selected test failing because the production core
accepted the injected current bootstrap address. Immediately restore only the exact
line and prove `wallet-broker/src/xmr/rpc.rs` is 1,896 lines with SHA-256
`2186f6fd56cfcce1edfbe00cd247ef2ab2177ba04dfd9b505416a1da33cbe4ed`.
Never stage or commit the temporary mutation.

## Exact green sequence

Run once each, in order:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test xmr_rpc
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test xmr_process
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test xmr_distribution
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test native_surface
/home/lars/.cargo/bin/rustup run 1.98.0 cargo check --manifest-path wallet-broker/Cargo.toml --locked --offline --features native-ui --test native_surface
node test/securityPolicy.node.js
node scripts/security-policy.js
```

Require: 15/12/12/17 tests passed respectively, all with zero failed/ignored/measured/
filtered; check exit 0 without warning/diagnostic; Node policy exit 0 with exactly 86
`ok`, no `not ok`, and final `BitBook security policy tests passed (86).`; security
script exit 0 with final `BitBook desktop security policy checks passed.` No accepted
source/test may mutate.

## Exact-success integration

Only on exact success, create `docs/testing/BBD-WAL-007-SLICE-03-GREEN-01.md` recording
Hermes identity, disk-backed filesystem, protected identities, formatter/no-mutation,
falsification/restoration, normalized command results/counts, scope, and prohibited-
action confirmation. Record no sensitive path, cache/artifact path, environment value,
port, credential, process ID, or raw output.

Update `docs/handoff/CURRENT_TASK.md` to
`PHASE C SLICE 3 GREEN COMPLETE — REVIEW REQUIRED`, linking the evidence and retaining
all ticket/decision/review/routing/prior-ticket records.

Recheck identities, `git diff --check`, and staged scope. Stage explicitly only the five
accepted paths, the green evidence, and `CURRENT_TASK.md`. Commit exactly:

```text
feat: add BBD-WAL-007 Monero RPC transport
```

Push `master`, prove `HEAD == origin/master`, clean index and worktree, then stop for
reviewer acceptance. Slice 4, broader acceptance, and the real local-Monero gate remain
unauthorized.
