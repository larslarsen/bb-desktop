# WAL-009 native review prerequisite check 02

Actor: Hermes Jr Dev. Governance parent: the commit containing this handoff.
Run from the bb-desktop repository root. This supersedes every earlier execution
permission. Grok's delimiter source task and the evidence correction are closed.

Read AGENTS.md, TESTING.md, docs/engineering/HERMES_JR_DEV_ROUTING.md,
CURRENT_TASK.md's leading section, this handoff, and
docs/testing/BBD-WAL-009-HARNESS-DELIMITER-05-REVIEW.md only. Do not reload
historical handoffs, evidence, or Rust implementations beyond hash/line checks.

## Preflight

Record actual Hermes version/provider/model, actor session ID, HEAD, and empty
staged index. For runtime identity, `hermes --version` is allowed; do not print
configuration, environment, or credentials. Do not run Cargo/rustup version probes.

Verify these exact identities:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| wallet-broker/src/zec/test_support.rs | 4359 | `461fdd070318cc5f31a29af2cdceaab1d0b63dc23473641b61ea9aafc145cd1b` |
| wallet-broker/src/native_ui.rs | 201 | `600e2fb34134a276d31da9c5e746cc6478337bae73590fd742cf2d36ff785524` |
| wallet-broker/src/native_ui/zec_review_tests.rs | 233 | `2eec3b60eb1b9fbd32235832a867d1189928126ca79373fa9f4fa7fdf2ee87bf` |
| wallet-broker/src/native.rs | 489 | `992138f18cbb0b969296109de3186aab923500c41ed2c0d41cf9c0696d3c3acc` |
| wallet-broker/Cargo.toml | 122 | `73e5e585eb2fd1ca867d66962460cfa010443886a4b59d8a33556ef2a4ff3503` |
| wallet-broker/Cargo.lock | — | `b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71` |

The expected dirty/untracked set is the six paths above plus package.json,
package-lock.json, wallet-broker/src/zec.rs, wallet-broker/src/zec/prepare.rs,
wallet-broker/src/zec/store.rs, wallet-broker/src/zec/spend.rs,
docs/testing/BBD-WAL-009-LOCK-SYNC-01.md, and
docs/testing/BBD-WAL-009-NATIVE-REVIEW-EXPECTED-RED-01.md. Hash all fourteen
paths before and after execution; preserve their bytes. An untracked directory in
Git output must be expanded read-only to confirm its file inventory.

Verify the effective build target is the existing disk-backed wallet-broker/target
(repository ext4, local /tmp tmpfs). Do not alter environment/configuration or put
substantial artifacts in /tmp. Unexpected source/identity/index/target state stops
before Cargo; evidence reporting below remains allowed.

## One command, one tool result

Submit the following string as the entire terminal command exactly once:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --features native-ui --lib native_ui::zec_review_tests
```

The terminal tool already returns output and exit status. Do not add `2>&1`, echo,
a pipeline, a shell operator, environment prefix, or wrapper. Do not rerun to obtain
an exit status, clearer output, or confirmation. If the tool cannot capture the
result, report that limitation and stop. A tool wait on the SAME running command
is allowed; it must not execute a second command. Never run two acceptance tools
concurrently. No formatter, cargo check, other test, lint, scanner, native launch,
dependency operation, source mutation, network, or falsification is authorized.

Classify the complete returned diagnostics:

- Missing private ZecReviewDialog/ZecReviewControls contract only, with exit 101
  and zero tests executed: intended absent-contract red, pending reviewer acceptance.
- Context::run incompatibility or any other unrelated diagnostic, even alongside
  missing-dialog diagnostics: MIXED / PREREQUISITE-BLOCKED, not behavioral red.
- Success or executed tests: unexpected outcome; report it without continuing.

The previous unclosed delimiter was repaired in source only. Do not assume the
crate now compiles, hide remaining errors, or repair anything after this check.

## Record and stop

After any result, only read-only postflight hashes/status, reading the already
captured command output, and creation of
docs/testing/BBD-WAL-009-NATIVE-REVIEW-PREREQUISITE-02.md are allowed. Preserve
the original expected-red evidence. The new record must contain runtime/session
identity, governance parent, exact submitted command and tool exit, each distinct
diagnostic (full relevant spans, normalized repository paths), six declared tests
versus actual executed count, classification, and all fourteen before/after hashes.
Copy hashes and command from captured tool data; do not retype or infer them.
Disclose deviations rather than claiming compliance contrary to the transcript.

Leave evidence uncommitted. Do not update any other record, stage, commit, push,
launch another actor, repair source, or execute another gate. Stop for review.
The reviewer will not poll; collection waits for the owner's completion report.
