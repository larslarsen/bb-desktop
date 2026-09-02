# Hermes Handoff — BBD-WAL-007 Phase-C Slice 1 Green 01

You are **Jr Dev — Hermes**. This durable handoff is authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely before acting: `AGENTS.md`, `TESTING.md`,
`docs/engineering/DEVELOPMENT_ROLES.md`, `docs/engineering/HERMES_JR_DEV_ROUTING.md`,
`tickets/BBD-WAL-007.md`, `docs/testing/BBD-WAL-007-PHASE-B-ACCEPTANCE-01.md`, both
Slice-1 source reviews, and `docs/handoff/CURRENT_TASK.md`.

## Sole task and stop rule

Integrate only the accepted Slice-1 source/policy drop, run only the exact focused green
commands below, record evidence, then commit and push only on exact success. You are the
execution, evidence, and Git actor. You are not the reviewer and may not design or edit
tests, repair or format source, change a command, accept a mismatch, begin Slice 2, or
run the real local-Monero gate.

Stop immediately on the first precondition mismatch, command failure, unexpected test
count, warning/diagnostic, worktree mutation, or scope change. On a stop, report the
exact command/result and make no source/test/policy/evidence edit, staging, commit, or
push.

## Protected preconditions

Require:

- `HEAD == origin/master ==` the protected governance parent;
- a clean index;
- worktree changes consisting of exactly the nine paths and identities below;
- no other tracked or untracked path; and
- `git diff --check` clean before execution.

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/lib.rs` | 12 | `08dd09d23a8c18cdb9a50968ade153a2118b60132f2b7b66a36c6913596de925` |
| `wallet-broker/src/native.rs` | 320 | `228c007856097c8433bc6e1c7132921f69be0b6f10543c3d999fabd9b14487f8` |
| `wallet-broker/src/native_ui.rs` | 149 | `34fda529c4ac6035bb5147720f456a271145deb43878082fbdfe464d320a7bdf` |
| `wallet-broker/src/xmr.rs` | 3 | `b39ea4228dd25951701cda9595a2b521e3391a92a41904c9e61b68ea1368695b` |
| `wallet-broker/src/xmr/distribution.rs` | 910 | `f5e3b43c11f1a4a1b0389738b9729621fe80ea6f87bb5216640c47f213903ebb` |
| `wallet-broker/src/xmr/model.rs` | 93 | `acb9391d22df344d7d72cbe139a0b87366be30d8c8f464e82e7db6e97ebddc47` |
| `wallet-broker/src/xmr/test_support.rs` | 370 | `e2fb6496ddb731ad60753c169a31958f692be235bd0bf7b1a6c5e000872ad722` |
| `scripts/security-policy.js` | 2,678 | `6dbf22fb3980e424d2bb108ca568612b8cb23f2c7307d45543871486c18eb3f6` |
| `test/securityPolicy.node.js` | 3,162 | `f3464fe3f429c55f66cf1ac18e1a7be70d0d50263433b26068f2f20fa0dc3dad` |

Also require these frozen Phase-A identities:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/Cargo.toml` | 113 | `189f067739afd8bc1e775e4d58b3ce7d4cfa6ab53e7b7acc5b02737396c5bff5` |
| `wallet-broker/Cargo.lock` | 5,394 | `29a5e4a8b9fa40ca612f3b7c3b1f90d527544a519c06e50bc11aae7803f57420` |
| `wallet-broker/tests/native_surface.rs` | 664 | `349f3a019a0e7a5c37aaae727192b0ddcecb02192a5fae6df9291eaf1357276d` |
| `wallet-broker/tests/xmr_distribution.rs` | 297 | `481042e6dabe705fb7771c0cf692f0561298a9e0e8aa6d0cfd1d8f2ab9deb6f8` |

Record before execution:

```text
hermes --version
hermes config get model.provider
hermes config get model.default
```

Inspect the filesystem type containing `wallet-broker/target`; it must be disk-backed.
Use the repository's existing target/cache state only. Do not use `/tmp`, download
anything, access the network, inspect a personal Monero path, or start any binary.

## Exact commands and acceptance

Run once each, from the repository root, in this order:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo fmt --manifest-path wallet-broker/Cargo.toml --check
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test native_surface
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test xmr_distribution
/home/lars/.cargo/bin/rustup run 1.98.0 cargo check --manifest-path wallet-broker/Cargo.toml --locked --offline --features native-ui --test native_surface
node test/securityPolicy.node.js
node scripts/security-policy.js
```

Require, respectively:

1. exit 0 with no source or test mutation;
2. exactly 17 passed, 0 failed/ignored/measured/filtered out;
3. exactly 12 passed, 0 failed/ignored/measured/filtered out;
4. exit 0 without warning or diagnostic;
5. exit 0, exactly 86 `ok`, no `not ok`, and final line
   `BitBook security policy tests passed (86).`;
6. exit 0 and final line `BitBook desktop security policy checks passed.`

The already accepted Phase-B expected-red result is the test-first falsification record:
the same focused targets failed on the exact absent XMR production boundary before this
source existed. Do not perform a new mutation-based falsification in this gate.

## Exact-success integration

Only if every result is exact, create
`docs/testing/BBD-WAL-007-SLICE-01-GREEN-01.md`. Record the Hermes version/provider/model,
filesystem fact, protected identities, every exact command and normalized result, test
counts, no-mutation proof, source/policy identities, prohibited-action confirmation, and
the Phase-B falsification reference. Do not record a local absolute Monero path, test
artifact path, environment value, or raw sensitive output.

Update `docs/handoff/CURRENT_TASK.md` to
`PHASE C SLICE 1 GREEN COMPLETE — REVIEW REQUIRED`, linking the evidence and retaining
the ticket, architecture decision, routing, and prior-ticket records.

Recheck exact paths, hashes, `git diff --check`, and the staged diff. Stage explicitly
only:

- the nine accepted source/policy paths;
- `docs/testing/BBD-WAL-007-SLICE-01-GREEN-01.md`; and
- `docs/handoff/CURRENT_TASK.md`.

Commit exactly:

```text
feat: add BBD-WAL-007 Monero distribution boundary
```

Push `master`, then prove `HEAD == origin/master`, clean index, and clean tracked and
untracked worktree. Stop for reviewer acceptance. Slice 2, broader acceptance, the real
local-Monero gate, every other source/test/policy/document path, and every other
repository remain unauthorized.
