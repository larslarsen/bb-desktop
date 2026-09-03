# Hermes Handoff — BBD-WAL-008 Phase-B Expected Red 01

You are **Jr Dev — Hermes**. This durable handoff is authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely: `AGENTS.md`, `TESTING.md`, both role/routing policies,
`docs/handoff/CURRENT_TASK.md`, `tickets/BBD-WAL-008.md`, all three Phase-A source
reviews, and the complete two-path test drop.

## Frozen source identities

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/Cargo.toml` | 117 | `7d89a7e98fde65c69392a91e633436ccf93b65a2f8e826e22b5eec27804da530` |
| `wallet-broker/tests/zec_hardware.rs` | 752 | `5759d612f70a5d21e2b9c7fb192449cf51633e3bff65f2ad7141feaf21812056` |

Before execution, record `hermes --version`, resolved provider/model, branch, exact
`HEAD` and `origin/master`, clean governance index, the two frozen identities, absence
of any other worktree path, and clean `git diff --check`. The only expected worktree
changes are the modified manifest and untracked test file above. Do not print secrets,
credentials, configuration, or environment.

## Exact sequential commands

Submit each fenced command as the terminal command string byte-for-byte, alone, once,
and sequentially. Append no wrapper, `echo`, redirection, pipeline, shell operator, or
other text. Use the tool-returned exit code.

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo fmt --manifest-path wallet-broker/Cargo.toml --check
```

Required: exit `0`, no mutation.

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_hardware
```

Required expected red: exit `101`; no test executes; diagnostics are limited to the
absent BBD-WAL-008 production/test-support contract—unresolved new
`zec::test_support` hardware items and/or the intentionally absent
`src/zec/hardware.rs` referenced by the production-inventory assertion. A dependency,
lockfile, syntax, type error in otherwise self-contained test source, existing-source
error, network attempt, or any other cause is not the expected red.

## Stop and integration rules

Stop immediately on any preflight mismatch, formatter failure/mutation, unexpected
exit, unrelated diagnostic, hang, leak, unlisted path, or `Cargo.lock` change. After a
stop, do not repair, rerun, edit evidence, stage, commit, push, or run another command.

Only after both exact outcomes may you create
`docs/testing/BBD-WAL-008-PHASE-A-EXPECTED-RED-01.md` and update
`docs/handoff/CURRENT_TASK.md`. Evidence records actual Hermes version/provider/model,
governance parent, frozen identities, exact commands and exit codes, normalized absent
contract diagnostics, unchanged lockfile, and prohibited-action confirmation. Do not
claim green behavior or production support.

Stage exactly these four paths:

- `wallet-broker/Cargo.toml`
- `wallet-broker/tests/zec_hardware.rs`
- `docs/testing/BBD-WAL-008-PHASE-A-EXPECTED-RED-01.md`
- `docs/handoff/CURRENT_TASK.md`

Commit exactly `test: add WAL-008 Zcash hardware contract`, push `master`, then use
only read-only Git status/HEAD/origin/commit-path proof. Stop for reviewer acceptance.

Do not edit or execute production source, existing tests, fixtures, `Cargo.lock`, Node,
Electron, policy, workflows, packages, WAL-007, or any other path. Do not run another
test, Clippy, build, audit, scanner, product binary, network operation, another actor,
or the Monero gate.
