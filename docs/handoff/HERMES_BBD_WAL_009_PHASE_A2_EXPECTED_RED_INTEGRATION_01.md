# Hermes Handoff — BBD-WAL-009 Phase A2 Expected-Red Integration 01

You are **Jr Dev — Hermes**. This is a documentation-and-integration-only continuation
of Expected Red Resume 01. Do not rerun either command or any other gate.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely: `AGENTS.md`, `TESTING.md`, both role/routing policies,
`docs/handoff/CURRENT_TASK.md`, `tickets/BBD-WAL-009.md`, the Phase-A1 test-source and
format-correction reviews, the Expected-Red Resume 01 handoff, and the Expected-Red
Resume 01 Stop Review 01.

Verify with read-only checks that `HEAD` and `origin/master` equal the protected
governance parent, the index is clean, the worktree contains only the following exact
source identities, and `wallet-broker/Cargo.lock` remains unchanged:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/Cargo.toml` | 121 | `71e0135dbc2a6086ee6658e173718d2dd8c608a1f6369f732a8979d942ef6450` |
| `wallet-broker/tests/zec_sign_verify.rs` | 1,115 | `80a3a342392f53553950fabae710f2e95082d357c281c6de23b54aedbc85eccd` |

Create `docs/testing/BBD-WAL-009-PHASE-A1-EXPECTED-RED-01.md` and update only the
leading active state in `docs/handoff/CURRENT_TASK.md`. Record, without execution:

- Hermes `v0.18.2 (2026.7.7.2)` and provider/model `meituan/longcat-2.0:free`;
- execution parent `efd1210dbbf6fbf942b5b00ce2e56def7027703a`;
- the exact formatter command, exit `0`, and no mutation;
- the exact focused command, exit `101`, and zero tests executed;
- the three normalized absent-contract diagnostic groups accepted by the stop review;
- both frozen source identities and unchanged `Cargo.lock`;
- Hermes's conservative stop and confirmation that no command followed it; and
- no claim of green, production, hardware, network, or broadcast support.

Stage exactly these four paths:

- `wallet-broker/Cargo.toml`
- `wallet-broker/tests/zec_sign_verify.rs`
- `docs/testing/BBD-WAL-009-PHASE-A1-EXPECTED-RED-01.md`
- `docs/handoff/CURRENT_TASK.md`

Commit exactly `test: add WAL-009 Zcash sign verify contract`, push `master`, then use
only read-only Git status/HEAD/origin/commit-path proof. Stop for reviewer acceptance.

Do not run a formatter, test, build, compiler, Clippy, audit, scanner, dependency or
product command. Do not edit source, tests, fixtures, `Cargo.lock`, policy, workflows,
packages, Electron/Node, WAL-007, or any other path. Do not invoke network beyond the
authorized Git push, a wallet/node process, hardware/device action, or another actor.
