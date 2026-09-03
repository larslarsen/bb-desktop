# Hermes Handoff — BBD-WAL-007 Phase-C Slice 4 Green 01

You are **Jr Dev — Hermes**. This durable handoff is authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely before acting: `AGENTS.md`, `TESTING.md`,
`docs/engineering/DEVELOPMENT_ROLES.md`, `docs/engineering/HERMES_JR_DEV_ROUTING.md`,
`tickets/BBD-WAL-007.md`, Test-Source Review 04, Expected Red 02, Slice-3 Acceptance 01,
the Slice-3 Upstream RPC Decision, all six Slice-4 source reviews, all five Slice-4
correction handoffs, the complete accepted seven-path drop, and
`docs/handoff/CURRENT_TASK.md`.

## Sole task and first-mismatch stop

Integrate only the accepted Slice-4 source drop, perform the exact temporary lock
falsification, run only the commands below exactly once, record evidence, and
commit/push only on exact success. You are the execution, evidence, and Git actor. You
are not the reviewer and may not design or permanently edit tests, repair or format
source, change/wrap/repeat a command, accept a mismatch, begin Slice 5, run the real
local-Monero gate, or touch another repository.

Stop immediately on the first parent/index/path/hash mismatch, formatter failure or
mutation, unexpected falsification result, green command failure, unexpected test
count, warning/diagnostic, unapproved mutation, or command-scope deviation. Restore the
temporary falsification if it was applied, prove the accepted source hash, report the
exact result, and on a stop make no evidence edit, staging, commit, or push.

## Protected preconditions

Require:

- `HEAD == origin/master ==` the protected governance parent;
- a clean index;
- worktree changes consisting of exactly the seven paths and identities below;
- no other tracked or untracked path; and
- `git diff --check` clean before execution.

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/vault.rs` | 776 | `bb79b16f24a9abbcd266e856a37ee982bde8fcdc2da3858e75d34bdd34a4af86` |
| `wallet-broker/src/xmr.rs` | 7 | `8b382ef673e82001720727c161f365c0fa3c1401d1bdaa4d20ee2683c001559b` |
| `wallet-broker/src/xmr/account.rs` | 3,073 | `864ceeb41d74da04338b2c21f47e8be57f2f95215ff51fbf20b1bdfd4c95e61b` |
| `wallet-broker/src/xmr/store.rs` | 1,316 | `b94e26ef1d8dbcd12e275c1603806700ccb00a6efdb7c17500f3a177be11dfb8` |
| `wallet-broker/src/xmr/process.rs` | 1,808 | `b990de3e80db0a4d354ec6119fbc746b27a8989909e702b63270b6d5b43fd52a` |
| `wallet-broker/src/xmr/rpc.rs` | 2,418 | `4f52baf1532a374dda748ec03a4a0a4311fd970b441f258f576c26030bb5ac14` |
| `wallet-broker/src/xmr/test_support.rs` | 3,924 | `5695a67aac219f36e5cd4df156f0708843084c9befb8e396f641c7c3348f966e` |

Also require these committed/frozen identities:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/Cargo.toml` | 113 | `189f067739afd8bc1e775e4d58b3ce7d4cfa6ab53e7b7acc5b02737396c5bff5` |
| `wallet-broker/Cargo.lock` | 5,394 | `29a5e4a8b9fa40ca612f3b7c3b1f90d527544a519c06e50bc11aae7803f57420` |
| `wallet-broker/src/xmr/distribution.rs` | 914 | `4135db83eca151185d8222498d2435ef56e2d564ff17bd3fa88481260758aed5` |
| `wallet-broker/src/xmr/model.rs` | 151 | `23a85dc216839cb936161845b52cacdca68a24181e5d31afeaaba30e62c77ca9` |
| `wallet-broker/tests/xmr_account.rs` | 586 | `5e8afabaa8c820be84a010a88fef0e07617e67934c06b32ab445901b1ddad35b` |
| `wallet-broker/tests/xmr_hygiene.rs` | 329 | `3653eba660be481d71e75185e024ae7e0a17f8754089c83b21edb397da84230f` |
| `wallet-broker/tests/xmr_rpc.rs` | 694 | `a1face1660ca0daf66002671ba3d794058b4acd497de2f0a3e25f2ca57597d0b` |
| `wallet-broker/tests/xmr_process.rs` | 453 | `25166407a510fef113c5c1db948243ef60de884658f7a9545f49524af49da833` |
| `wallet-broker/tests/xmr_distribution.rs` | 297 | `481042e6dabe705fb7771c0cf692f0561298a9e0e8aa6d0cfd1d8f2ab9deb6f8` |
| `wallet-broker/tests/native_surface.rs` | 664 | `349f3a019a0e7a5c37aaae727192b0ddcecb02192a5fae6df9291eaf1357276d` |
| `wallet-broker/tests/vault_crypto.rs` | 394 | `a83bc4d1bf30201ad8c9bfa09556e8bae029b8eefebf2eeeab4db5fed03e561b` |
| `wallet-broker/tests/vault_format.rs` | 373 | `5c07a7a52a5be52d852e5c5d45bf62e2f86913324d8dcf642a455d6483b6f193` |
| `wallet-broker/tests/vault_store.rs` | 726 | `582dd24bb91b30db8ec3f38bca6103994b8896f3b3a351c63ae00a7187a838c5` |
| `wallet-broker/tests/vault_session.rs` | 319 | `67487db86d6788633e031418da71f6080409ac57a144ad362e311cb22519be6b` |
| `wallet-broker/tests/secret_hygiene.rs` | 281 | `dcebe361c7061b06b9ec6bb6fbea88ccb208069f7360ae14e2d7c6ca83b433a4` |
| `scripts/security-policy.js` | 2,689 | `6aba356098134e90731928a2a1083565d52a6d000c213b89549ba231997f5626` |
| `test/securityPolicy.node.js` | 3,189 | `dd22ab83645d5ffb9d695cd21f15175ffe109b6020142e1bff5ca8d60e0497` |

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

Require exit 101, no warning or compile diagnostic, and exactly one selected test
failing with 0 passed, 0 ignored, 0 measured, and 15 filtered out because software lock
did not perform the teardown and the closed call/child assertions detected it. Restore
the original line immediately and prove `wallet-broker/src/xmr/account.rs` is restored
byte-for-byte to 3,073 lines and SHA-256
`864ceeb41d74da04338b2c21f47e8be57f2f95215ff51fbf20b1bdfd4c95e61b`.
Never stage or commit the temporary mutation.

## Exact green sequence

After exact restoration, run each command exactly once, in order, exactly as written,
and with no wrapper, redirection, or pipeline:

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

Require all Rust commands to emit no warning or diagnostic. Require, respectively:

1. 16 passed, 0 failed/ignored/measured/filtered;
2. 9 passed, 0 failed/ignored/measured/filtered;
3. 15 passed, 0 failed/ignored/measured/filtered;
4. 12 passed, 0 failed/ignored/measured/filtered;
5. 12 passed, 0 failed/ignored/measured/filtered;
6. 11 passed, 0 failed/ignored/measured/filtered;
7. 11 passed, 0 failed/ignored/measured/filtered;
8. 20 passed, 0 failed/ignored/measured/filtered;
9. 13 passed, 0 failed/ignored/measured/filtered;
10. 11 passed, 0 failed/ignored/measured/filtered;
11. 17 passed, 0 failed/ignored/measured/filtered;
12. exit 0 with Clippy warnings denied;
13. exit 0 without warning or diagnostic;
14. exit 0, exactly 86 `ok`, no `not ok`, and final line
    `BitBook security policy tests passed (86).`; and
15. exit 0 with final line `BitBook desktop security policy checks passed.`

No accepted source/test file may mutate during these commands.

## Exact-success integration

Only on exact success, create `docs/testing/BBD-WAL-007-SLICE-04-GREEN-01.md`. Record
Hermes version/provider/model, the disk-backed filesystem fact, protected identities,
formatter/no-mutation result, the warning-free temporary falsification and restoration
proof, every normalized green result and test count, scope, and prohibited-action
confirmation. Do not record a local Monero path, artifact/cache path, environment value,
port, credential, process ID, seed, primary address, receiver, or raw sensitive output.

Update `docs/handoff/CURRENT_TASK.md` to
`PHASE C SLICE 4 GREEN 01 COMPLETE — REVIEW REQUIRED`, linking the evidence while
retaining the ticket, architecture decisions, Slice-1/2/3 acceptances, all Slice-4
source reviews/corrections, routing, and prior-ticket records.

Recheck identities, `git diff --check`, and staged scope. Stage explicitly only:

```text
wallet-broker/src/vault.rs
wallet-broker/src/xmr.rs
wallet-broker/src/xmr/account.rs
wallet-broker/src/xmr/store.rs
wallet-broker/src/xmr/process.rs
wallet-broker/src/xmr/rpc.rs
wallet-broker/src/xmr/test_support.rs
docs/testing/BBD-WAL-007-SLICE-04-GREEN-01.md
docs/handoff/CURRENT_TASK.md
```

Commit exactly:

```text
feat: add BBD-WAL-007 Monero account custody
```

Push `master`. After commit/push, run no formatter, test, check, Clippy, Node/npm,
policy/security, build, or product command. Use only read-only Git/identity commands to
prove `HEAD == origin/master`, clean index and tracked/untracked worktree, the exact
commit message, and the nine-path commit scope. Then stop for reviewer acceptance.
Slice 5, broader/final acceptance, and the real offline local-Monero gate remain
unauthorized.
