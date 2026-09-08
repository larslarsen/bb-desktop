# WAL-009 native review initial red 01

Actor: Hermes Jr Dev. Governance parent: the commit containing this handoff.
Run from the bb-desktop repository root. Grok's source task is closed.

Read AGENTS.md, TESTING.md, HERMES_JR_DEV_ROUTING.md, CURRENT_TASK.md's leading
section, this handoff, and BBD-WAL-009-NATIVE-REVIEW-TEST-SOURCE-04-REVIEW.md.
Do not reload historical handoffs or cryptographic implementations.

## Preflight and frozen inputs

Record actual Hermes version/provider/model, HEAD, and an empty staged index.
Verify all three source identities in the source review. Also verify:

- wallet-broker/Cargo.toml: SHA-256
  `73e5e585eb2fd1ca867d66962460cfa010443886a4b59d8a33556ef2a4ff3503`.
- wallet-broker/Cargo.lock: SHA-256
  `b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71`.

The existing dirty/untracked set is package.json, package-lock.json,
wallet-broker/Cargo.toml, wallet-broker/Cargo.lock, wallet-broker/src/native.rs,
wallet-broker/src/native_ui.rs, wallet-broker/src/native_ui/zec_review_tests.rs,
wallet-broker/src/zec.rs, wallet-broker/src/zec/prepare.rs,
wallet-broker/src/zec/store.rs, wallet-broker/src/zec/test_support.rs,
wallet-broker/src/zec/spend.rs, and docs/testing/BBD-WAL-009-LOCK-SYNC-01.md.
Hash these before execution and preserve them byte for byte. Do not stage them.

The reviewer inspected filesystem types: the repository is on ext4; local /tmp
is tmpfs. Use the existing disk-backed wallet-broker/target build location;
do not place substantial artifacts in /tmp or alter Cargo configuration. If the
effective target directory is elsewhere, inspect its filesystem before execution
and stop if it is RAM-backed. Preflight mismatch stops without a Cargo command.

## Single authorized execution

Submit this exact terminal command once, with no wrapper, pipeline, redirection,
environment prefix, or other appended command:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --features native-ui --lib native_ui::zec_review_tests
```

This run deliberately classifies the current compile prerequisites. No formatter,
native_surface test, cargo check, broader test, lint, scan, GUI, source mutation,
dependency change, network access, or falsification is authorized.

Classify the actual result precisely:

- Intended absent-contract red: exit 101, no tests execute, and diagnostics limited
  to missing private ZecReviewDialog/ZecReviewControls and their direct consequences.
- Mixed/prerequisite-blocked: the known old production Context::run incompatibility,
  any unrelated source/API/dependency error, or another failure. List each distinct
  diagnostic separately; this is not accepted behavioral red, even if missing-dialog
  diagnostics also appear.
- Unexpected success or any executed tests: report the actual result as a mismatch.

Do not rerun, repair, restore, or execute another gate after any result.

## Evidence and stop

After the one command ends, read-only postflight hashes/status and creation of only
docs/testing/BBD-WAL-009-NATIVE-REVIEW-EXPECTED-RED-01.md are explicitly allowed
for every outcome, including a preflight stop or prerequisite failure. This narrow
reporting allowance is not permission to run another acceptance command.

Record runtime identity and actor session ID, governance HEAD, source hashes and
line counts, six declared tests versus actual number executed, exact command and
exit code, the complete relevant compiler diagnostics, result classification, and
before/after protected-file identity comparison. Preserve the execution output in
the actor transcript and identify that transcript without copying secrets or local
absolute paths into the committed record. Do not claim passing UI behavior or an OS
window launch. Report any unauthorized action candidly.

Leave evidence uncommitted. Do not update CURRENT_TASK, stage, commit, push, start
another actor, or authorize production work. Stop for reviewer evidence review.
