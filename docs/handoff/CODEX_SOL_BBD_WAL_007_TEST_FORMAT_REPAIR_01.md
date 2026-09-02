# Codex Sol Handoff — BBD-WAL-007 Test Format Repair 01

Status: AUTHORIZED — FORMATTING-ONLY TEST-SOURCE REPAIR

Source actor: Principal Dev — Codex Sol (`gpt-5.6-sol`, High)

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Protected governance parent: the commit containing this handoff

Accepted pre-repair identity:
`../testing/BBD-WAL-007-TEST-SOURCE-REVIEW-02.md`

Hermes stop evidence is uncommitted at
`../testing/BBD-WAL-007-EXPECTED-RED-02.md`, SHA-256
`521d7087b7d632f7ff6771afc1852aa100c90584acd0ea9731c395443489375f`.

## Cause and objective

Hermes correctly stopped because the seven new XMR Rust test files do not pass the
required Rust 1.98.0 formatting check. Make only the deterministic rustfmt changes in
those seven files. Do not change test meaning, names, assertions, fixtures, constants,
imports except rustfmt ordering, manifest/policy text, or test counts.

The uncommitted `Cargo.lock` resolution and both expected-red evidence files belong to
Hermes and are read-only. In particular, do not revert, stage, or integrate them.

## Authorized paths

Edit only:

- `wallet-broker/tests/xmr_distribution.rs`
- `wallet-broker/tests/xmr_process.rs`
- `wallet-broker/tests/xmr_rpc.rs`
- `wallet-broker/tests/xmr_account.rs`
- `wallet-broker/tests/xmr_receiver.rs`
- `wallet-broker/tests/xmr_hygiene.rs`
- `wallet-broker/tests/xmr_local_gate.rs`

Every other path and repository is read-only.

## Authorized operation

After verifying the pre-repair hashes against the accepted review, run only this
source-formatting command from the repository root:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 rustfmt --edition 2021 wallet-broker/tests/xmr_distribution.rs wallet-broker/tests/xmr_process.rs wallet-broker/tests/xmr_rpc.rs wallet-broker/tests/xmr_account.rs wallet-broker/tests/xmr_receiver.rs wallet-broker/tests/xmr_hygiene.rs wallet-broker/tests/xmr_local_gate.rs
```

Then inspect the complete diff and remove any change that is not rustfmt-only. Do not
run Cargo, tests, builds, binaries, Node, npm, package managers, network operations,
Git mutation, or GitHub. Do not stage, commit, or push.

## Delivery

Stop after the formatting-only source drop. Report the seven paths with line counts and
SHA-256 hashes, confirm every diff hunk is rustfmt-only and test counts are unchanged,
and confirm no prohibited path or command was used. Reviewer acceptance is required
before Hermes may resume.
