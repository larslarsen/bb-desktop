# Hermes Handoff — BBD-WAL-009 Phase A2 Expected Red 01

You are **Jr Dev — Hermes**.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely: `AGENTS.md`, `TESTING.md`, both role/routing policies,
`docs/handoff/CURRENT_TASK.md`, `tickets/BBD-WAL-009.md`, Phase-A1 Test-Source Review
01, and the complete two-path test drop.

## Frozen source identities

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/Cargo.toml` | 121 | `71e0135dbc2a6086ee6658e173718d2dd8c608a1f6369f732a8979d942ef6450` |
| `wallet-broker/tests/zec_sign_verify.rs` | 1,105 | `670b6d0938bf061b774bc7126b4971105208f5385b395baed69fb967c00cb4b7` |

Before execution, record `hermes --version`, resolved provider/model, branch, exact
`HEAD` and `origin/master`, clean governance index, both frozen identities, absence of
any other worktree path, and clean diff/whitespace state. The only expected worktree
changes are the modified manifest and untracked test file above. Do not print secrets,
credentials, configuration, or environment.

## Exact sequential commands

Submit each fenced command as the terminal command string byte-for-byte, alone, once,
and sequentially. Append no wrapper, redirection, pipeline, shell operator, or other
text. Use the tool-returned exit code.

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo fmt --manifest-path wallet-broker/Cargo.toml --all -- --check
```

Required: exit `0`, no mutation.

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_sign_verify
```

Required expected red: exit `101`; no test executes; diagnostics are limited to the
absent BBD-WAL-009 typed sign/verify contract under `zec::test_support` and the
intentionally absent `src/zec/spend.rs` referenced by the production-inventory
assertion. A dependency or lockfile change, network attempt, formatter/syntax error,
error in an existing API used by the test, existing-source failure, or other cause is
not the expected red.

## Stop and integration rules

Stop immediately on any preflight mismatch, formatter failure or mutation, unexpected
exit, unrelated diagnostic, hang, leak, unlisted path, or `Cargo.lock` change. After a
stop, do not repair, rerun, edit evidence, stage, commit, push, or run another command.

Only after both exact outcomes may you create
`docs/testing/BBD-WAL-009-PHASE-A1-EXPECTED-RED-01.md` and update
`docs/handoff/CURRENT_TASK.md`. Evidence records the actual Hermes version,
provider/model, governance parent, frozen identities, exact commands and exit codes,
normalized absent-contract diagnostics, unchanged lockfile, and prohibited-action
confirmation. Do not claim green behavior or production support.

Stage exactly these four paths:

- `wallet-broker/Cargo.toml`
- `wallet-broker/tests/zec_sign_verify.rs`
- `docs/testing/BBD-WAL-009-PHASE-A1-EXPECTED-RED-01.md`
- `docs/handoff/CURRENT_TASK.md`

Commit exactly `test: add WAL-009 Zcash sign verify contract`, push `master`, then use
only read-only Git status/HEAD/origin/commit-path proof. Stop for reviewer acceptance.

Do not edit or execute production source, existing tests, fixtures, `Cargo.lock`, Node,
Electron, policy, workflows, packages, WAL-007, or any other path. Do not run another
test, Clippy, build, audit, scanner, product binary, network operation, another actor,
or the Monero gate.
