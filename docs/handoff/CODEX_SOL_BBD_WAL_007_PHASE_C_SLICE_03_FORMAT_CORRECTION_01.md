# Codex Sol Handoff — BBD-WAL-007 Phase-C Slice 3 Format Correction 01

Status: AUTHORIZED — FORMATTING ONLY

Source actor: Principal Dev — Codex Sol (`gpt-5.6-sol`, High)

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Protected governance parent: the commit containing this handoff

Repository: `/home/lars/OpenBazaar/bb-desktop`

Read completely before acting: `AGENTS.md`, `TESTING.md`, Slice-3 Source Review 03,
Slice-3 Green Stop Review 01, the complete three paths below, and
`docs/handoff/CURRENT_TASK.md`.

## Exact path and byte boundary

Edit only these three paths, starting from the exact stopped identities:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/tests/xmr_rpc.rs` | 691 | `67b745f4e951ad9acf473ca71153b99acd4ba5d3a387257e906de617e9052b49` |
| `wallet-broker/src/xmr/rpc.rs` | 1,789 | `0659a851a74f5fb5b62236d15feb5b74b82bd56e792b3eb31ad4a1c3bed74326` |
| `wallet-broker/src/xmr/test_support.rs` | 2,676 | `fdb5655e2531be8ef81f4f7254099c940cde02641df023aa4550ed710edad2c3` |

Freeze the other accepted paths:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/xmr.rs` | 5 | `92af416c0ed6c5f2f8a6ff7011facb622da5e3f1ba8adaa2e96f600c09f31411` |
| `wallet-broker/src/xmr/model.rs` | 151 | `23a85dc216839cb936161845b52cacdca68a24181e5d31afeaaba30e62c77ca9` |
| `wallet-broker/src/xmr/process.rs` | 1,184 | `7533a86344c178ed25143d21bc3961a72ea7a1fcf85067f90b57134178928a1f` |

Every other path is read-only, including evidence, governance, manifests, lockfiles,
and all other tests/source.

Make only the mechanical layout changes Rust 1.98 rustfmt requires: import grouping and
ordering, line wrapping, expression/condition layout, array/slice layout, and function/
assertion argument layout. The failed check reported multiple regions in each source
file and two regions in the test.

Preserve every semantic token, identifier, literal, type, visibility, expression,
statement, item, attribute, test name, test count, comment, and behavior. Imports may
move only according to rustfmt ordering. Do not repair, refactor, simplify, rename, or
optimize anything.

Do not run Cargo, rustfmt, tests, builds, binaries, Node/npm, package managers, security
tools, network, Git, or GitHub. Do not stage, commit, push, or edit evidence/governance.
Stop after the exact formatting-only three-path drop and report line counts/hashes,
changed layout regions, semantic-token preservation, and prohibited-action compliance.
