# Hermes Handoff — BBD-WAL-007 Phase-C Slice 5 Green 01

You are **Jr Dev — Hermes**. This durable handoff is authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely before acting: `AGENTS.md`, `TESTING.md`,
`docs/engineering/DEVELOPMENT_ROLES.md`, `docs/engineering/HERMES_JR_DEV_ROUTING.md`,
`tickets/BBD-WAL-007.md`, Test-Source Review 04, Expected Red 02, Slice-4 Acceptance
01, the original Slice-5 handoff, all Slice-5 correction handoffs and source reviews,
the complete accepted eight-path drop, the frozen receiver test, the Slice-5
Format-Correction-01 Source Review, and `docs/handoff/CURRENT_TASK.md`.

## Sole task and first-mismatch stop

Integrate only the accepted Slice-5 source drop, perform the exact temporary durable-
replay falsification, run only the commands below exactly once and sequentially, record
evidence, and commit/push only on exact success. You are the execution, evidence, and
Git actor. You are not the reviewer and may not design or permanently edit tests,
repair or format source, change/wrap/redirect/repeat a command, accept a mismatch, run
the real local-Monero gate, begin broader/final acceptance, or touch another repository.

Stop immediately on the first parent/index/path/hash mismatch, formatter failure or
mutation, unexpected falsification result, green command failure, unexpected test
count, warning/diagnostic, unapproved mutation, or command-scope deviation. Restore the
temporary falsification if it was applied, prove the accepted source hash, report the
exact result, and on a stop make no evidence edit, staging, commit, or push.

## Protected preconditions

Require:

- `HEAD == origin/master ==` the protected governance parent;
- a clean index;
- worktree changes consisting of exactly the eight source paths and identities below
  plus the frozen Hermes stop draft named after the table;
- no other tracked or untracked path; and
- tracked and untracked-addition whitespace checks clean before execution.

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/xmr.rs` | 8 | `78107f241bb4cb8f02ab4168cbc81a01fc90cc75c80328a2677f819d7c06adce` |
| `wallet-broker/src/xmr/model.rs` | 214 | `2a2d3ba1ce453aca65138df402bde3e7f1fee997d5d069024cb1beb8102152cb` |
| `wallet-broker/src/xmr/account.rs` | 3,375 | `5dcad3d450a2e5d8d780e7e490111c33ba06da6275d7d1ca84e5f76dde09cddb` |
| `wallet-broker/src/xmr/process.rs` | 1,964 | `66f0aae7fd0b507cbadc27628d0b1c26ee0033d90891c294721c11a00be9dd2d` |
| `wallet-broker/src/xmr/rpc.rs` | 2,576 | `1bbfdf3ec58f89728b2eb169e9d49c53512eb3b108e5c17f7b02bf2634fada33` |
| `wallet-broker/src/xmr/store.rs` | 1,904 | `08f7678c8fa5ce85d313c28e9b1ac79b42698ae6ead80c5e7e994f878d6069cd` |
| `wallet-broker/src/xmr/receiver.rs` | 871 | `4a1fd4ddd8025ea3c64c443d2746f319e91e609526d00ef912548fbd20d833f0` |
| `wallet-broker/src/xmr/test_support.rs` | 6,019 | `a815ab198559e7942d1c91ce0466a52d3b751631dba6dd80d5044682ec90cf33` |

Also require the frozen untracked Hermes stop draft at
`docs/testing/BBD-WAL-007-SLICE-05-GREEN-01.md`, 59 lines and SHA-256
`20f07f0b44dae0f006e192d91b717b541487a857d4d920047bbfd68818705637`.
Do not edit or delete it before exact success.

Also require these committed/frozen identities:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/Cargo.toml` | 113 | `189f067739afd8bc1e775e4d58b3ce7d4cfa6ab53e7b7acc5b02737396c5bff5` |
| `wallet-broker/Cargo.lock` | 5,394 | `29a5e4a8b9fa40ca612f3b7c3b1f90d527544a519c06e50bc11aae7803f57420` |
| `wallet-broker/src/xmr/distribution.rs` | 914 | `163f8532bc7edfd80fc07966c0f8f32eebc0d12181fd273bc4e6c2870d86dea8` |
| `wallet-broker/tests/xmr_receiver.rs` | 588 | `d8656c533d7b2d08eb9fa8831cd70da6e1da632ed9563fc2af569cab754d3622` |
| `wallet-broker/tests/xmr_account.rs` | 586 | `5e8afabaa8c820be84a010a88fef0e07617e67934c06b32ab445901b1ddad35b` |
| `wallet-broker/tests/xmr_hygiene.rs` | 329 | `3653eba660be481d71e75185e024ae7e0a17f8754089c83b21edb397da84230f` |
| `wallet-broker/tests/xmr_rpc.rs` | 694 | `a1face1660ca0daf66002671ba3d794058b4acd497de2f0a3e25f2ca57597d0b` |
| `wallet-broker/tests/xmr_process.rs` | 453 | `25166407a510fef113c5c1db948243ef60de884658f7a9545f49524af49da833` |
| `wallet-broker/tests/xmr_distribution.rs` | 297 | `481042e6dabe705fb7771c0cf692f0561298a9e0e8aa6d0cfd1d8f2ab9deb6f8` |
| `wallet-broker/tests/native_surface.rs` | 664 | `349f3a019a0e7a5c37aaae727192b0ddcecb02192a5fae6df9291eaf1357276d` |
| `scripts/security-policy.js` | 2,689 | `6aba356098134e90731928a2a1083565d52a6d000c213b89549ba231997f5626` |
| `test/securityPolicy.node.js` | 3,189 | `dd22ab83645d5d5ffb9d695cd21f15175ffe109b6020142e1bff5ca8d60e0497` |

Record separately before execution:

```text
hermes --version
hermes config get model.provider
hermes config get model.default
```

Inspect the filesystem type containing `wallet-broker/target`; it must be disk-backed.
Use only existing repository target/cache state. Do not use `/tmp`, download anything,
access the network, inspect a personal Monero path, or start a product/Monero binary.

## Formatter and exact falsification

Run once, exactly as written and with no wrapper, redirection, or pipeline:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo fmt --manifest-path wallet-broker/Cargo.toml --check
```

Require exit 0, no warning/diagnostic, and no source/test mutation. Then temporarily
replace exactly this line in `issue_fresh` in
`wallet-broker/src/xmr/receiver.rs`:

```rust
    if let Some(existing) = existing {
```

with exactly:

```rust
    if let Some(existing) = existing.filter(|_| false) {
```

Run once, exactly as written and with no wrapper, redirection, or pipeline:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test xmr_receiver exact_replay_returns_durable_binding_without_any_rpc_call
```

Require exit 101, no warning or compile diagnostic, and exactly the selected test
failing with 0 passed, 0 ignored, 0 measured, and 14 filtered out because suppressing
the durable replay return caused the replay attempt to continue instead of returning
the stored binding. Immediately restore the original line and prove
`wallet-broker/src/xmr/receiver.rs` is restored byte-for-byte to 871 lines and SHA-256
`4a1fd4ddd8025ea3c64c443d2746f319e91e609526d00ef912548fbd20d833f0`.
Never stage or commit the temporary mutation.

## Exact green sequence

After exact restoration, run each command exactly once, in order, exactly as written,
and with no wrapper, redirection, or pipeline:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test xmr_receiver
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test xmr_account
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test xmr_hygiene
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test xmr_rpc
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test xmr_process
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test xmr_distribution
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test native_surface
/home/lars/.cargo/bin/rustup run 1.98.0 cargo clippy --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --lib --test xmr_receiver -- -D warnings
/home/lars/.cargo/bin/rustup run 1.98.0 cargo check --manifest-path wallet-broker/Cargo.toml --locked --offline --features native-ui --test native_surface
node test/securityPolicy.node.js
node scripts/security-policy.js
```

Require all Rust commands to emit no warning or diagnostic. Require, respectively:

1. 15 passed, 0 failed/ignored/measured/filtered;
2. 16 passed, 0 failed/ignored/measured/filtered;
3. 9 passed, 0 failed/ignored/measured/filtered;
4. 15 passed, 0 failed/ignored/measured/filtered;
5. 12 passed, 0 failed/ignored/measured/filtered;
6. 12 passed, 0 failed/ignored/measured/filtered;
7. 17 passed, 0 failed/ignored/measured/filtered;
8. exit 0 with Clippy warnings denied;
9. exit 0 without warning or diagnostic;
10. exit 0, exactly 86 `ok`, no `not ok`, and final line
    `BitBook security policy tests passed (86).`; and
11. exit 0 with final line `BitBook desktop security policy checks passed.`

No accepted source/test file may mutate during these commands.

## Exact-success integration

Only on exact success, replace the frozen stop draft at
`docs/testing/BBD-WAL-007-SLICE-05-GREEN-01.md` with the complete green evidence.
Record Hermes version/provider/model, the disk-backed filesystem fact, protected
identities, formatter/no-mutation result, the warning-free temporary falsification and
exact restoration proof, every normalized green result and test count, scope, and
prohibited-action confirmation. Do not record a local Monero path, artifact/cache
path, environment value, port, credential, process ID, seed, primary address,
receiver, or raw sensitive output.

Update `docs/handoff/CURRENT_TASK.md` to
`PHASE C SLICE 5 GREEN 01 COMPLETE — REVIEW REQUIRED`, linking the evidence while
retaining the ticket, architecture decision, Slice-1–4 acceptances, all Slice-5 source
reviews/corrections, routing, and prior-ticket records.

Recheck identities, tracked and untracked-addition whitespace checks, and staged scope.
Stage explicitly only:

```text
wallet-broker/src/xmr.rs
wallet-broker/src/xmr/model.rs
wallet-broker/src/xmr/account.rs
wallet-broker/src/xmr/process.rs
wallet-broker/src/xmr/rpc.rs
wallet-broker/src/xmr/store.rs
wallet-broker/src/xmr/receiver.rs
wallet-broker/src/xmr/test_support.rs
docs/testing/BBD-WAL-007-SLICE-05-GREEN-01.md
docs/handoff/CURRENT_TASK.md
```

Commit exactly:

```text
feat: add BBD-WAL-007 Monero viewing and receivers
```

Push `master`. After commit/push, run no formatter, test, check, Clippy, Node/npm,
policy/security, build, or product command. Use only read-only Git/identity commands to
prove `HEAD == origin/master`, clean index and tracked/untracked worktree, the exact
commit message, and the ten-path commit scope. Then stop for reviewer acceptance.
Broader/final acceptance and the real offline local-Monero gate remain unauthorized.
