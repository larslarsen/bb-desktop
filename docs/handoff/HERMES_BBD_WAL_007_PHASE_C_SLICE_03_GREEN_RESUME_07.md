# Hermes Handoff — BBD-WAL-007 Phase-C Slice 3 Green Resume 07

You are **Jr Dev — Hermes**. This durable handoff is authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely before acting: `AGENTS.md`, `TESTING.md`, the development-role and
Hermes-routing policies, `tickets/BBD-WAL-007.md`, Slice-3 Upstream RPC Decision,
Slice-3 Source Review 03, Test Oracle Correction Source Review 01, Green Evidence 01
Rejection 01, Warning Correction Source Review 01, the complete committed Slice-3
source, the one-path accepted correction, and `docs/handoff/CURRENT_TASK.md`.

## Sole task and first-mismatch stop

Integrate only the accepted warning correction, perform the exact temporary
falsification, run only the commands below exactly once and without a shell pipeline,
record new evidence, and commit/push only on exact success. Do not reuse any Resume-06
result; design, repair, or format source; change/wrap/repeat a command; begin Slice 4;
run the real local-Monero gate; or touch another repository.

Stop immediately on any parent/index/path/hash mismatch, formatter failure or mutation,
unexpected falsification, command failure/count/warning/diagnostic, unapproved mutation,
or command-scope deviation. Restore the falsification if applied, prove the accepted
hash, and on a stop make no evidence edit, staging, commit, or push.

## Preconditions

Require `HEAD == origin/master ==` the protected governance parent, a clean index,
`git diff --check`, and exactly one worktree path:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/xmr/rpc.rs` | 1,913 | `7593322a5aef2fc146698d2e07a541cd9fb796b92e1f8e3fd699bcfbb2b219f9` |

Require these committed/frozen identities:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/tests/xmr_rpc.rs` | 694 | `a1face1660ca0daf66002671ba3d794058b4acd497de2f0a3e25f2ca57597d0b` |
| `wallet-broker/src/xmr.rs` | 5 | `92af416c0ed6c5f2f8a6ff7011facb622da5e3f1ba8adaa2e96f600c09f31411` |
| `wallet-broker/src/xmr/model.rs` | 151 | `23a85dc216839cb936161845b52cacdca68a24181e5d31afeaaba30e62c77ca9` |
| `wallet-broker/src/xmr/test_support.rs` | 2,705 | `2c445494b36fd645240cc9a25405782eff6d3e8187b4762aee6a0fd7ca640a40` |
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

Run once, exactly as written and with no wrapper, redirection, or pipeline:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo fmt --manifest-path wallet-broker/Cargo.toml --check
```

Require exit 0, no output diagnostic, and no mutation. Then temporarily remove exactly
this line from `evaluate_node_policy` in `wallet-broker/src/xmr/rpc.rs`:

```rust
        || !info.bootstrap_daemon_address.is_empty()
```

Run once, exactly as written and with no wrapper, redirection, or pipeline:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test xmr_rpc node_syncing_is_distinct_from_bootstrap_remote_and_unavailable
```

Require exit 101, no warning or compile diagnostic, and exactly one selected test
failing with 0 passed, 0 ignored, 0 measured, and 14 filtered because the production
core accepted the injected current bootstrap address. Immediately restore only the
exact line and prove `wallet-broker/src/xmr/rpc.rs` is 1,913 lines with SHA-256
`7593322a5aef2fc146698d2e07a541cd9fb796b92e1f8e3fd699bcfbb2b219f9`.
Never stage or commit the temporary mutation.

## Exact green sequence

Run each command exactly once, in order, exactly as written, and with no wrapper,
redirection, or pipeline:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test xmr_rpc
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test xmr_process
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test xmr_distribution
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test native_surface
/home/lars/.cargo/bin/rustup run 1.98.0 cargo check --manifest-path wallet-broker/Cargo.toml --locked --offline --features native-ui --test native_surface
node test/securityPolicy.node.js
node scripts/security-policy.js
```

Require all Rust commands to emit no warning or diagnostic. Require: 15/12/12/17 tests
passed respectively, all with zero failed/ignored/measured/filtered; check exit 0; Node
policy exit 0 with exactly 86 `ok`, no `not ok`, and final
`BitBook security policy tests passed (86).`; security script exit 0 with final
`BitBook desktop security policy checks passed.` No accepted source/test may mutate.

## Exact-success integration

Only on exact success, create `docs/testing/BBD-WAL-007-SLICE-03-GREEN-02.md` recording
Hermes identity, disk-backed filesystem, protected identities, formatter/no-mutation,
warning-free falsification/restoration, warning-free normalized command results/counts,
scope, Green Evidence 01 rejection, and prohibited-action confirmation. Record no
sensitive path, cache/artifact path, environment value, port, credential, process ID,
or raw output.

Update `docs/handoff/CURRENT_TASK.md` to
`PHASE C SLICE 3 GREEN 02 COMPLETE — REVIEW REQUIRED`, linking Green Evidence 02 and
retaining the rejected Green Evidence 01, all ticket/decision/review/routing, and
prior-ticket records.

Recheck identities, `git diff --check`, and staged scope. Stage explicitly only:

```text
wallet-broker/src/xmr/rpc.rs
docs/testing/BBD-WAL-007-SLICE-03-GREEN-02.md
docs/handoff/CURRENT_TASK.md
```

Commit exactly:

```text
fix: close BBD-WAL-007 RPC warning gate
```

Push `master`. After commit/push, run no formatter, test, check, Node/npm, policy,
security, build, or product command. Use only read-only Git/identity commands to prove
`HEAD == origin/master`, clean index and tracked/untracked worktree, the exact commit
message, and the three-path commit scope. Then stop for reviewer acceptance. Slice 4,
broader acceptance, and the real local-Monero gate remain unauthorized.
