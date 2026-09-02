# Hermes Handoff — BBD-WAL-007 Phase-C Slice 3 Green 01

You are **Jr Dev — Hermes**. This durable handoff is authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely before acting: `AGENTS.md`, `TESTING.md`,
`docs/engineering/DEVELOPMENT_ROLES.md`, `docs/engineering/HERMES_JR_DEV_ROUTING.md`,
`tickets/BBD-WAL-007.md`, Slice-3 Upstream RPC Decision, all three Slice-3 source
reviews, both Slice-3 correction handoffs, the complete accepted five-path drop, and
`docs/handoff/CURRENT_TASK.md`.

## Sole task and stop rule

Integrate only the accepted Slice-3 test/production drop, perform the one exact
temporary falsification, run only the focused green/regression commands below, record
evidence, then commit and push only on exact success. You are the execution, evidence,
and Git actor. You are not the reviewer and may not design or permanently edit tests,
repair or format source, change a command, accept a mismatch, begin Slice 4, run the
real local-Monero gate, or touch another repository.

Stop immediately on the first precondition mismatch, formatter failure/mutation,
unexpected falsification result, green command failure, unexpected test count, warning/
diagnostic, unapproved worktree mutation, or scope change. On a stop, restore the exact
temporary falsification if it was applied, prove the accepted source hash, report the
exact result, and make no evidence edit, staging, commit, or push.

## Protected preconditions

Require:

- `HEAD == origin/master ==` the protected governance parent;
- a clean index;
- worktree changes consisting of exactly the five paths and identities below;
- no other tracked or untracked path; and
- `git diff --check` clean before execution.

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/tests/xmr_rpc.rs` | 691 | `67b745f4e951ad9acf473ca71153b99acd4ba5d3a387257e906de617e9052b49` |
| `wallet-broker/src/xmr.rs` | 5 | `92af416c0ed6c5f2f8a6ff7011facb622da5e3f1ba8adaa2e96f600c09f31411` |
| `wallet-broker/src/xmr/model.rs` | 151 | `23a85dc216839cb936161845b52cacdca68a24181e5d31afeaaba30e62c77ca9` |
| `wallet-broker/src/xmr/rpc.rs` | 1,789 | `0659a851a74f5fb5b62236d15feb5b74b82bd56e792b3eb31ad4a1c3bed74326` |
| `wallet-broker/src/xmr/test_support.rs` | 2,676 | `fdb5655e2531be8ef81f4f7254099c940cde02641df023aa4550ed710edad2c3` |

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

Record before execution, as separate commands:

```text
hermes --version
hermes config get model.provider
hermes config get model.default
```

Inspect the filesystem type containing `wallet-broker/target`; it must be disk-backed.
Use only existing repository target/cache state. Do not use `/tmp`, download anything,
access the network, inspect a personal Monero path, or start a product/Monero binary.

## Exact formatter and falsification

First run once:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo fmt --manifest-path wallet-broker/Cargo.toml --check
```

Require exit 0 with no source/test mutation.

Then temporarily remove exactly this one condition line from `evaluate_node_policy` in
`wallet-broker/src/xmr/rpc.rs`:

```rust
        || !info.bootstrap_daemon_address.is_empty()
```

Run exactly:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test xmr_rpc node_syncing_is_distinct_from_bootstrap_remote_and_unavailable
```

Require exit 101 with exactly that one selected test failed because the production core
accepted the injected nonempty current bootstrap address; no compile, dependency, lock,
network, timeout, or unrelated failure is acceptable. Immediately restore only that
exact line and prove `wallet-broker/src/xmr/rpc.rs` is restored byte-for-byte to 1,789
lines and SHA-256
`0659a851a74f5fb5b62236d15feb5b74b82bd56e792b3eb31ad4a1c3bed74326`.
The temporary mutation must never be staged or committed.

## Exact green commands and acceptance

After exact restoration, run once each from the repository root, in order:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test xmr_rpc
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test xmr_process
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test xmr_distribution
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test native_surface
/home/lars/.cargo/bin/rustup run 1.98.0 cargo check --manifest-path wallet-broker/Cargo.toml --locked --offline --features native-ui --test native_surface
node test/securityPolicy.node.js
node scripts/security-policy.js
```

Require, respectively:

1. exactly 15 passed, 0 failed/ignored/measured/filtered out;
2. exactly 12 passed, 0 failed/ignored/measured/filtered out;
3. exactly 12 passed, 0 failed/ignored/measured/filtered out;
4. exactly 17 passed, 0 failed/ignored/measured/filtered out;
5. exit 0 without warning or diagnostic;
6. exit 0, exactly 86 `ok`, no `not ok`, and final line
   `BitBook security policy tests passed (86).`;
7. exit 0 and final line `BitBook desktop security policy checks passed.`

No accepted source/test file may mutate during the green commands.

## Exact-success integration

Only if every result is exact, create
`docs/testing/BBD-WAL-007-SLICE-03-GREEN-01.md`. Record Hermes version/provider/model,
the disk-backed filesystem fact, protected identities, formatter/no-mutation result,
the exact temporary falsification and restoration proof, every normalized green result
and test count, scope, and prohibited-action confirmation. Do not record a local Monero
path, artifact/cache path, environment value, port, credential, process ID, or raw
sensitive output.

Update `docs/handoff/CURRENT_TASK.md` to
`PHASE C SLICE 3 GREEN COMPLETE — REVIEW REQUIRED`, linking the evidence while retaining
the ticket, architecture decisions, prior source reviews/corrections, Slice-1/2
acceptances, routing, and prior-ticket records.

Recheck exact paths/hashes, `git diff --check`, and the staged diff. Stage explicitly
only:

- the five accepted test/production paths;
- `docs/testing/BBD-WAL-007-SLICE-03-GREEN-01.md`; and
- `docs/handoff/CURRENT_TASK.md`.

Commit exactly:

```text
feat: add BBD-WAL-007 Monero RPC transport
```

Push `master`, then prove `HEAD == origin/master`, clean index, and clean tracked and
untracked worktree. Stop for reviewer acceptance. Slice 4, broader acceptance, the real
local-Monero gate, and every other path/repository remain unauthorized.
