# Codex Luna Handoff — BBD-WAL-004 Expected Red Resume 01

You are **Jr Dev — Codex Luna**, using `gpt-5.6-luna`. This is the complete durable
resume prompt; ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely before acting: `AGENTS.md`, `TESTING.md`,
`docs/engineering/DEVELOPMENT_ROLES.md`, `tickets/BBD-WAL-004.md`,
`docs/testing/BBD-WAL-004-TEST-SOURCE-REVIEW.md`, `docs/handoff/CURRENT_TASK.md`, the
original Sol/Luna handoffs, Corrections 1–3, and all accepted paths.

## Resume state

The owner installed official user-level Rust/Cargo 1.98.0. Your first preflight passed.
You added only `target/` to root `.gitignore`, then sandboxed Cargo failed DNS. The
reviewer's approved network retry reached crates.io and found the nonexistent secrecy
`alloc` feature. Cargo exited 101 before creating a lockfile. Sol Correction 3 is now
accepted and changes only the manifest secrecy declaration and its Node policy
expectation/regression.

Before acting, prove:

- `HEAD == origin/master` at the protected governance parent;
- `wallet-broker/Cargo.lock` is absent;
- root `.gitignore` is 12 lines, SHA-256
  `9e528c7294e2b5d37b9016991a20fcb111afd77dd318ba9073500e84d83e8ec5`, with exactly one
  uncommitted appended line `target/`;
- the only other uncommitted paths are the nine accepted paths in the test-source review,
  at their exact line counts and SHA-256 values;
- `git diff --check` passes; and
- `/home/lars/.cargo/bin/rustc +1.98.0 --version` and Cargo report 1.98.0.

Stop without editing/resolution/Git on any mismatch.

## Disk and command environment

`/home/lars/.cache/bb-desktop-rust-tmp` already exists. Every Cargo command must set:

```text
TMPDIR=/home/lars/.cache/bb-desktop-rust-tmp
CARGO_TARGET_DIR=/home/lars/OpenBazaar/bb-desktop/target/wal004-cargo
```

Never use `/tmp`, clean caches, delete build state, or change Cargo/Rustup homes.

## Resume resolution

With approved crates.io network access, rerun:

```text
/home/lars/.cargo/bin/cargo +1.98.0 generate-lockfile --manifest-path wallet-broker/Cargo.toml
```

Then run separately with the same environment:

```text
/home/lars/.cargo/bin/cargo +1.98.0 tree --manifest-path wallet-broker/Cargo.toml --locked --no-default-features
/home/lars/.cargo/bin/cargo +1.98.0 tree --manifest-path wallet-broker/Cargo.toml --locked --all-features
```

Only registry dependencies are permitted. Stop if another feature/version/MSRV conflict
appears, the manifest changes, a direct exact pin changes, a git or outside path dependency
appears, or any unlisted repository path is created. Record concise inventories; the
reviewer, not Luna, will judge the full graph/licenses/build scripts/advisories.

## Expected red and integration

Continue the original Luna handoff from its **Expected red** section without changing
commands or counts:

1. `node test/securityPolicy.node.js` must exit 1 with exactly 57 `ok` and the seven exact
   named `not ok` results in the review.
2. The exact named `vault_crypto` Cargo test must reach compilation and fail only because
   the future `bitbook_wallet_broker` library/API is absent. Any direct primitive API,
   manifest, dependency, linker, GUI, or runtime failure is rejection.
3. On exact red only, create `docs/testing/BBD-WAL-004-EXPECTED-RED.md`, update only
   `docs/handoff/CURRENT_TASK.md`, stage the original nine accepted paths plus `.gitignore`,
   `wallet-broker/Cargo.lock`, evidence, and current-task record, commit
   `test: define encrypted wallet custody`, push, and prove a clean worktree with
   `HEAD == origin/master`.

All evidence fields, forbidden actions, Git boundaries, and stop conditions in the
original Luna handoff remain authoritative. The approved crates.io resolution and Git
push are the only network operations. Do not run production, broader tests, fmt, clippy,
audit, deny, SBOM, npm, build, Electron, native UI, packaging, wallet, node, service,
hardware, or device actions. Do not use root, `sudo`, `/tmp`, cleanup, `rm`, recursive
deletion, globs, or unresolved destructive targets.

Report preflight, tool versions, lock/tree results, exact red outputs/counts, evidence,
commit/push/final state, or the exact stop condition. Stop after expected-red integration;
production remains unauthorized pending reviewer lock-graph and evidence review.
