# Codex Luna Handoff — BBD-WAL-004 Correction 2 Expected Red

You are **Jr Dev — Codex Luna**. This durable file is the complete integration prompt;
ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff.

Read completely before acting: `AGENTS.md`, `TESTING.md`, roles, `tickets/BBD-WAL-004.md`,
`docs/testing/BBD-WAL-004-PRODUCTION-SOURCE-REVIEW-02.md`,
`docs/testing/BBD-WAL-004-CORRECTION-2-TEST-SOURCE-REVIEW.md`, prior WAL-004 red evidence,
`docs/handoff/CURRENT_TASK.md`, the four accepted test paths, and current production.

## Sole task and preflight

Prove the Correction 2 regressions fail for exactly source-review-02 reasons. Do not
repair or edit source/test/policy/workflow/manifest/lock/deny/validator/package files.

Require `HEAD == origin/master` at the governance parent, clean index, and exactly the
four accepted modified test paths plus 15 frozen production paths. Match the four test
hashes in the test-source review and every production hash in source review 02/Correction
2 handoff before and after. Stop on an extra path or mismatch.

Use the installed toolchain only through the absolute rustup proxy. All Cargo commands
are locked, offline, no-default-features, and use the ignored repository target; never
use `/tmp` or another cache/target.

## Exact execution

Run separately, in order, recording output and exit status:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 rustc --version
/home/lars/.cargo/bin/rustup run 1.98.0 cargo --version
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test vault_session
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test native_surface
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test vault_store
node test/securityPolicy.node.js
```

All three Rust binaries must compile and reach execution. Every prior accepted case must
be green. Exactly the five new Rust tests and the extended inventory case must fail for
their reviewed reasons. A compile error, warning-as-error, setup/offline failure,
unrelated test failure, abort/signal, scratch-cleanup failure, or secret canary disclosure
is unintended red: stop and report without integration. The Node suite must execute all
65 cases and report exact `ok`/`not ok` totals.

Do not run all-features, native UI, fmt, clippy, audit, deny, SBOM generation, npm,
Electron, wallets, nodes, devices, network, installers, cleanup, deletion, or any
unlisted command.

## Evidence and Git

If and only if red is exact, create only
`docs/testing/BBD-WAL-004-CORRECTION-2-EXPECTED-RED.md` with versions, commands/statuses,
named failures/reasons, prior green counts, Node totals, no-canary result, and pre/post
hash integrity. Update only `docs/handoff/CURRENT_TASK.md` to `CORRECTION 2 EXPECTED RED
RECORDED — PRODUCTION CORRECTION REQUIRED` and link evidence.

Run `git diff --check`. Stage only the four accepted test paths, evidence, and
`CURRENT_TASK.md`; inspect staged names/diff. Commit once as
`test: record wallet custody correction two red` and push `master`. Leave all 15
production paths unstaged and hash-identical. Require final `HEAD == origin/master` and
report commit, evidence count/hash, exact pass/fail totals, path hashes, and status.
