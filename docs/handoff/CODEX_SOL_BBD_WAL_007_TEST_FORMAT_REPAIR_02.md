# Codex Sol Handoff — BBD-WAL-007 Test Format Repair 02

Status: AUTHORIZED — EDITION-2024 FORMATTING ONLY

Source actor: Principal Dev — Codex Sol (`gpt-5.6-sol`, High)

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Protected governance parent: the commit containing this handoff

Accepted pre-repair identity:
`../testing/BBD-WAL-007-TEST-SOURCE-REVIEW-03.md`

Latest Hermes stop evidence is uncommitted at
`../testing/BBD-WAL-007-EXPECTED-RED-02.md`, 68 lines, SHA-256
`d38dd8478d88efb2b080bb8930b451fef6d017ac7e4dd15c487266c556f5ba05`.

## Exact cause and objective

The first format-repair handoff incorrectly forced `--edition 2021`. The crate manifest
declares `edition = "2024"`, and there is no repository or crate rustfmt configuration
file. Hermes proved Cargo's edition-2024 formatting still differs in the seven XMR test
files. Reformat exactly those files using standalone Rust 1.98.0 rustfmt with edition
2024. Do not make a manual or semantic edit.

The manifest, native-surface test, Node policy, lockfile, both evidence files, and all
governance are read-only.

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

Verify the seven Review 03 pre-repair hashes, then run only this command from the
repository root:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 rustfmt --edition 2024 wallet-broker/tests/xmr_distribution.rs wallet-broker/tests/xmr_process.rs wallet-broker/tests/xmr_rpc.rs wallet-broker/tests/xmr_account.rs wallet-broker/tests/xmr_receiver.rs wallet-broker/tests/xmr_hygiene.rs wallet-broker/tests/xmr_local_gate.rs
```

Do not run Cargo, tests, builds, binaries, Node, npm, package managers, network, Git, or
GitHub. Do not stage, commit, or push.

## Delivery

Stop after the edition-2024 rustfmt source drop. Report the seven paths with line counts,
named test counts, and SHA-256 hashes. Confirm rustfmt was the sole writer, every test
name/count remains unchanged, and no prohibited path or command was used. Reviewer
acceptance is required before Hermes may resume.
