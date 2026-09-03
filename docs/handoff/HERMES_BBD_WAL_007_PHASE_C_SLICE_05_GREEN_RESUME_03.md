# Hermes Handoff — BBD-WAL-007 Phase-C Slice 5 Green Resume 03

You are **Jr Dev — Hermes**. This durable handoff is authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely before acting: `AGENTS.md`, `TESTING.md`,
`docs/engineering/DEVELOPMENT_ROLES.md`, `docs/engineering/HERMES_JR_DEV_ROUTING.md`,
`tickets/BBD-WAL-007.md`, the original Slice-5 Green-01 handoff, both Slice-5 green
resume reviews, Compile-Correction-01 Source Review 01, Test-Oracle-Correction-01 Source
Review 01, all records named by the original handoff, the complete accepted nine-path
source/test drop, and `docs/handoff/CURRENT_TASK.md`.

## Sole task and first-mismatch stop

Run the fresh formatter, exact durable-replay falsification, focused Slice-5 and
affected-regression commands below exactly once and sequentially. Record evidence and
commit/push only after every exact success. Do not repair source/tests, change or repeat
a command, run the real local-Monero gate, begin broader/final acceptance, or touch
another repository.

For every fenced execution command below, the terminal tool's command argument must be
exactly the fenced text byte-for-byte. Use the terminal tool's native exit-code field.
Do not append/prepend `echo`, `2>&1`, `true`, `env`, a shell operator, redirection,
pipeline, wrapper, assignment, comment, or any other text. The command itself must not
be combined with another command.

Stop immediately on the first parent/index/path/hash mismatch, formatter failure or
mutation, unexpected falsification result, green failure, unexpected count,
warning/diagnostic, unapproved mutation, or command-scope deviation. Restore the
temporary falsification if applied, prove the accepted receiver hash, and on a stop
make no evidence edit, staging, commit, or push.

## Protected preconditions

Require `HEAD == origin/master ==` the protected governance parent, a clean index,
`git diff --check`, and exactly the following accepted worktree plus the frozen stop
draft, with no other tracked or untracked path:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/xmr.rs` | 8 | `78107f241bb4cb8f02ab4168cbc81a01fc90cc75c80328a2677f819d7c06adce` |
| `wallet-broker/src/xmr/model.rs` | 214 | `2a2d3ba1ce453aca65138df402bde3e7f1fee997d5d069024cb1beb8102152cb` |
| `wallet-broker/src/xmr/account.rs` | 3,375 | `5dcad3d450a2e5d8d780e7e490111c33ba06da6275d7d1ca84e5f76dde09cddb` |
| `wallet-broker/src/xmr/process.rs` | 1,964 | `66f0aae7fd0b507cbadc27628d0b1c26ee0033d90891c294721c11a00be9dd2d` |
| `wallet-broker/src/xmr/rpc.rs` | 2,576 | `1bbfdf3ec58f89728b2eb169e9d49c53512eb3b108e5c17f7b02bf2634fada33` |
| `wallet-broker/src/xmr/store.rs` | 1,904 | `3a7f4d5b8cc7b33e3596910ce0b9b10d2f760f24c3ccff98fd2941c410ee2df4` |
| `wallet-broker/src/xmr/receiver.rs` | 871 | `4a1fd4ddd8025ea3c64c443d2746f319e91e609526d00ef912548fbd20d833f0` |
| `wallet-broker/src/xmr/test_support.rs` | 6,019 | `a815ab198559e7942d1c91ce0466a52d3b751631dba6dd80d5044682ec90cf33` |
| `wallet-broker/tests/xmr_receiver.rs` | 588 | `39d438a767214f31fe07d68a844b217e41bcd73ead1a90ab666b596085b6583e` |
| `docs/testing/BBD-WAL-007-SLICE-05-GREEN-01.md` | 59 | `20f07f0b44dae0f006e192d91b717b541487a857d4d920047bbfd68818705637` |

Also require these committed/frozen identities:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/Cargo.toml` | 113 | `189f067739afd8bc1e775e4d58b3ce7d4cfa6ab53e7b7acc5b02737396c5bff5` |
| `wallet-broker/Cargo.lock` | 5,394 | `29a5e4a8b9fa40ca612f3b7c3b1f90d527544a519c06e50bc11aae7803f57420` |
| `wallet-broker/src/xmr/distribution.rs` | 914 | `163f8532bc7edfd80fc07966c0f8f32eebc0d12181fd273bc4e6c2870d86dea8` |
| `wallet-broker/tests/xmr_account.rs` | 586 | `5e8afabaa8c820be84a010a88fef0e07617e67934c06b32ab445901b1ddad35b` |
| `wallet-broker/tests/xmr_hygiene.rs` | 329 | `3653eba660be481d71e75185e024ae7e0a17f8754089c83b21edb397da84230f` |
| `wallet-broker/tests/xmr_rpc.rs` | 694 | `a1face1660ca0daf66002671ba3d794058b4acd497de2f0a3e25f2ca57597d0b` |
| `wallet-broker/tests/xmr_process.rs` | 453 | `25166407a510fef113c5c1db948243ef60de884658f7a9545f49524af49da833` |
| `wallet-broker/tests/xmr_distribution.rs` | 297 | `481042e6dabe705fb7771c0cf692f0561298a9e0e8aa6d0cfd1d8f2ab9deb6f8` |
| `wallet-broker/tests/native_surface.rs` | 664 | `349f3a019a0e7a5c37aaae727192b0ddcecb02192a5fae6df9291eaf1357276d` |
| `scripts/security-policy.js` | 2,689 | `6aba356098134e90731928a2a1083565d52a6d000c213b89549ba231997f5626` |
| `test/securityPolicy.node.js` | 3,189 | `dd22ab83645d5d5ffb9d695cd21f15175ffe109b6020142e1bff5ca8d60e0497` |

Record Hermes version/provider/model before execution. Confirm the filesystem containing
`wallet-broker/target` is disk-backed. Use existing offline cache state only; do not use
`/tmp`, download, access a personal Monero path, or start product/Monero binaries.

## Formatter and exact falsification

Run exactly:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo fmt --manifest-path wallet-broker/Cargo.toml --check
```

Require exit 0, no output/diagnostic, and no source/test mutation. Temporarily replace
only `if let Some(existing) = existing {` in `issue_fresh` with
`if let Some(existing) = existing.filter(|_| false) {`, then run exactly:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test xmr_receiver exact_replay_returns_durable_binding_without_any_rpc_call
```

Require exit 101, no warning/compile diagnostic, and exactly 0 passed, 1 failed, 0
ignored/measured, and 14 filtered out because replay continued rather than returning
the durable binding. Immediately restore the original line and prove `receiver.rs` is
871 lines at SHA-256
`4a1fd4ddd8025ea3c64c443d2746f319e91e609526d00ef912548fbd20d833f0`.

## Exact green sequence

Run each exact line separately and once, in this order:

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

Require test counts respectively 15, 16, 9, 15, 12, 12, and 17 passed with every
other count zero; Clippy exit 0 with warnings denied; native check exit 0 without
warning/diagnostic; the Node policy test exit 0 with exactly 86 `ok`, no `not ok`, and
final line `BitBook security policy tests passed (86).`; and the policy script exit 0
with final line `BitBook desktop security policy checks passed.`. No accepted file may
mutate.

## Exact-success integration

Only after exact success, replace the frozen stop draft with complete green evidence
recording identity, filesystem, formatter/no-mutation, falsification/restoration, every
normalized command result/count, scope, and prohibitions without sensitive raw values.
Update `CURRENT_TASK.md` to `PHASE C SLICE 5 GREEN 01 COMPLETE — REVIEW REQUIRED`.

Stage explicitly only these eleven paths:

```text
wallet-broker/src/xmr.rs
wallet-broker/src/xmr/model.rs
wallet-broker/src/xmr/account.rs
wallet-broker/src/xmr/process.rs
wallet-broker/src/xmr/rpc.rs
wallet-broker/src/xmr/store.rs
wallet-broker/src/xmr/receiver.rs
wallet-broker/src/xmr/test_support.rs
wallet-broker/tests/xmr_receiver.rs
docs/testing/BBD-WAL-007-SLICE-05-GREEN-01.md
docs/handoff/CURRENT_TASK.md
```

Commit exactly `feat: add BBD-WAL-007 Monero viewing and receivers` and push `master`.
After push run no execution command; use only read-only Git/identity checks to prove
`HEAD == origin/master`, clean worktree/index, exact message, and eleven-path scope.
Then stop for reviewer acceptance. Broader/final acceptance and the real offline
local-Monero gate remain unauthorized.
