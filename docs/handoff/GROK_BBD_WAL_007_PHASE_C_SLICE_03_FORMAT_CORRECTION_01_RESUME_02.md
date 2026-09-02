# Grok Handoff — BBD-WAL-007 Phase-C Slice 3 Format Correction 01 Resume 02

Status: AUTHORIZED — PINNED FORMATTER ON THREE PATHS ONLY

Source actor: Sr Dev — Grok Build (Grok 4.6 High)

Reviewer: Lead Engineer/Reviewer — Codex

Protected governance parent: the commit containing this handoff

Repository: `/home/lars/OpenBazaar/bb-desktop`

Read completely before acting: `AGENTS.md`, `TESTING.md`, Slice-3 Source Review 03,
Slice-3 Green Stop Review 01, both Format Correction 01 stop/reroute reviews, the
complete three paths below, and `docs/handoff/CURRENT_TASK.md`.

## Exact path and byte boundary

Start from:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/tests/xmr_rpc.rs` | 679 | `09ddea33ab7cf784cc338caf6cf61fd26452d533fe6a117d029893d8139dcd98` |
| `wallet-broker/src/xmr/rpc.rs` | 1,789 | `0659a851a74f5fb5b62236d15feb5b74b82bd56e792b3eb31ad4a1c3bed74326` |
| `wallet-broker/src/xmr/test_support.rs` | 2,676 | `fdb5655e2531be8ef81f4f7254099c940cde02641df023aa4550ed710edad2c3` |

Freeze:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/xmr.rs` | 5 | `92af416c0ed6c5f2f8a6ff7011facb622da5e3f1ba8adaa2e96f600c09f31411` |
| `wallet-broker/src/xmr/model.rs` | 151 | `23a85dc216839cb936161845b52cacdca68a24181e5d31afeaaba30e62c77ca9` |
| `wallet-broker/src/xmr/process.rs` | 1,184 | `7533a86344c178ed25143d21bc3961a72ea7a1fcf85067f90b57134178928a1f` |

Every other path is read-only, including evidence, governance, manifests, lockfiles,
and all other source/tests.

## Sole authorized mutation command

Run exactly once from the repository root:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 rustfmt --edition 2024 wallet-broker/src/xmr/rpc.rs wallet-broker/src/xmr/test_support.rs wallet-broker/tests/xmr_rpc.rs
```

This command is the authorized source edit. It must exit 0 and may change only the three
named paths. Do not manually edit before or after it. Do not run `cargo fmt`, a formatter
check, tests, builds, binaries, Node/npm, package managers, security tools, network, Git,
or GitHub. Do not stage, commit, push, or edit evidence/governance.

Stop immediately after the command. Report its exit, the three resulting line counts
and SHA-256 hashes, and confirmation that no other action or path was used. Reviewer
inspection and Hermes's independent `cargo fmt --check` remain required.
