# Hermes Handoff — BBD-WAL-007 Phase-C Slice 2 Green Resume 02

Status: AUTHORIZED — EXACT EXECUTION AND INTEGRATION ONLY

You are **Jr Dev — Hermes**. This durable handoff is authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely before acting: `AGENTS.md`, `TESTING.md`,
`docs/engineering/DEVELOPMENT_ROLES.md`, `docs/engineering/HERMES_JR_DEV_ROUTING.md`,
`tickets/BBD-WAL-007.md`, both Slice-2 architecture decisions, Slice-2 Expected-Red
Acceptance 01, Slice-2 Source Review 02, both Slice-2 green-stop reviews, Exact
Formatter Diff 01, Format-Correction Source Review 02, the complete accepted five-path
drop, and `docs/handoff/CURRENT_TASK.md`.

## Sole task and stop rule

Restart the Slice-2 gate from its first command. Verify the accepted drop, perform the
one exact temporary falsification, run only the focused green/regression commands,
replace the stopped evidence with complete green evidence, then commit and push only on
exact success. You are the execution, evidence, and Git actor. You may not design,
repair, or format source; change a command; accept a mismatch; begin Slice 3; run the
real local-Monero gate; or touch another repository.

Stop immediately on the first precondition mismatch, formatter failure/mutation,
unexpected falsification result, green command failure, unexpected test count,
warning/diagnostic, or unapproved worktree mutation. Restore the temporary
falsification if applied, prove the accepted source hash, leave the existing stop
record unchanged, report the exact result, and do not stage, commit, or push.

## Protected preconditions

Require `HEAD == origin/master ==` the protected governance parent, a clean index,
`git diff --check` clean, and exactly these six worktree paths/identities:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/xmr.rs` | 4 | `6c199bc807b16d80d19141dbe726d630dc6c2b89662253947ec280692af60fc6` |
| `wallet-broker/src/xmr/model.rs` | 143 | `704540f27a1d92c6e82281db01e8689b3f0dbc99c3f066311a806e0ecab437b7` |
| `wallet-broker/src/xmr/process.rs` | 1,184 | `7533a86344c178ed25143d21bc3961a72ea7a1fcf85067f90b57134178928a1f` |
| `wallet-broker/src/xmr/test_support.rs` | 1,151 | `5946ce53e5ddf0c1dbb64217019b90e0ba982b35c1b0a245ff45aa7079f39526` |
| `wallet-broker/tests/xmr_process.rs` | 453 | `25166407a510fef113c5c1db948243ef60de884658f7a9545f49524af49da833` |
| `docs/testing/BBD-WAL-007-SLICE-02-GREEN-01.md` | 51 | `c214f84921734bc522320b98e09d7eb1b55ba7eb5e6d242f4e473227f5903fe0` |

Also require these frozen identities:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/Cargo.toml` | 113 | `189f067739afd8bc1e775e4d58b3ce7d4cfa6ab53e7b7acc5b02737396c5bff5` |
| `wallet-broker/Cargo.lock` | 5,394 | `29a5e4a8b9fa40ca612f3b7c3b1f90d527544a519c06e50bc11aae7803f57420` |
| `wallet-broker/src/xmr/distribution.rs` | 914 | `4135db83eca151185d8222498d2435ef56e2d564ff17bd3fa88481260758aed5` |
| `wallet-broker/tests/xmr_distribution.rs` | 297 | `481042e6dabe705fb7771c0cf692f0561298a9e0e8aa6d0cfd1d8f2ab9deb6f8` |
| `wallet-broker/tests/native_surface.rs` | 664 | `349f3a019a0e7a5c37aaae727192b0ddcecb02192a5fae6df9291eaf1357276d` |
| `scripts/security-policy.js` | 2,689 | `6aba356098134e90731928a2a1083565d52a6d000c213b89549ba231997f5626` |
| `test/securityPolicy.node.js` | 3,189 | `dd22ab83645d5d5ffb9d695cd21f15175ffe109b6020142e1bff5ca8d60e0497` |

Record before execution as separate commands:

```text
hermes --version
hermes config get model.provider
hermes config get model.default
```

Inspect the filesystem type containing `wallet-broker/target`; it must be disk-backed.
Use only existing repository target/cache state. Do not use `/tmp`, download anything,
access the network, inspect a personal Monero path, or start a product/Monero binary.

## Exact formatter and falsification

Run once:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo fmt --manifest-path wallet-broker/Cargo.toml --check
```

Require exit 0 with no source/test mutation.

Temporarily add exactly this line immediately after
`entry("untrusted-daemon", "1"),` in `WalletRpcProcessPlan::build`:

```rust
entry("restricted-rpc", "1"),
```

Run exactly:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test xmr_process full_wallet_rpc_is_authenticated_ipv4_loopback_without_forbidden_options
```

Require exit 101 with exactly that selected test failing because the option name
`restricted-rpc` was observed. A compile, dependency, lock, network, timeout, or
unrelated failure is unacceptable. Immediately remove only the temporary line and
prove `wallet-broker/src/xmr/process.rs` is restored byte-for-byte to 1,184 lines and
SHA-256 `7533a86344c178ed25143d21bc3961a72ea7a1fcf85067f90b57134178928a1f`.
The temporary mutation must never be staged or committed.

## Exact green commands and acceptance

After exact restoration, run once each from the repository root, in order:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test xmr_process
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test xmr_distribution
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test native_surface
/home/lars/.cargo/bin/rustup run 1.98.0 cargo check --manifest-path wallet-broker/Cargo.toml --locked --offline --features native-ui --test native_surface
node test/securityPolicy.node.js
node scripts/security-policy.js
```

Require, respectively:

1. exactly 12 passed, 0 failed/ignored/measured/filtered out;
2. exactly 12 passed, 0 failed/ignored/measured/filtered out;
3. exactly 17 passed, 0 failed/ignored/measured/filtered out;
4. exit 0 without warning or diagnostic;
5. exit 0, exactly 86 `ok`, no `not ok`, final line
   `BitBook security policy tests passed (86).`;
6. exit 0, final line `BitBook desktop security policy checks passed.`

No accepted source/test file may mutate during these commands.

## Exact-success evidence and integration

Only if every result is exact, replace the existing stop contents of
`docs/testing/BBD-WAL-007-SLICE-02-GREEN-01.md` with complete green evidence. Record
the prior formatting stops/correction, Hermes version/provider/model, disk-backed
filesystem fact, protected identities, formatter/no-mutation result, exact temporary
falsification/restoration proof, every normalized green result/count, scope, and
prohibited-action confirmation. Do not record a local Monero path, artifact/cache path,
environment value, port, credential, process ID, or raw sensitive output.

Update `docs/handoff/CURRENT_TASK.md` to
`PHASE C SLICE 2 GREEN COMPLETE — REVIEW REQUIRED`, link the evidence, and retain the
ticket, architecture decisions, Slice-1 acceptance, routing, and prior-ticket records.

Recheck exact paths/hashes, `git diff --check`, and the staged diff. Stage explicitly
only the five accepted source/test paths, the replaced green evidence, and
`docs/handoff/CURRENT_TASK.md`. Commit exactly:

```text
feat: add BBD-WAL-007 Monero process lifecycle
```

Push `master`, prove `HEAD == origin/master`, clean index, and clean tracked/untracked
worktree, then stop for reviewer acceptance. Slice 3, broader acceptance, the real
local-Monero gate, and every other path/repository remain unauthorized.
