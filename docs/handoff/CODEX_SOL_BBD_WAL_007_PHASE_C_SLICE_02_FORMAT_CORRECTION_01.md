# Codex Sol Handoff — BBD-WAL-007 Phase-C Slice 2 Format Correction 01

Status: AUTHORIZED — FORMATTING ONLY

Source actor: Principal Dev — Codex Sol (`gpt-5.6-sol`, High)

Reviewer: Lead Engineer/Reviewer — Codex; XHigh review required before execution

Protected governance parent: the commit containing this handoff

Repository: `/home/lars/OpenBazaar/bb-desktop`

Read completely before acting: `AGENTS.md`, `TESTING.md`, Slice-2 Source Review 02,
Slice-2 Green Stop Review 01, the complete three paths below, and
`docs/handoff/CURRENT_TASK.md`.

## Exact path and byte boundary

Edit only these three paths, starting from the exact stopped identities:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/xmr/process.rs` | 1,191 | `b91a18f13568a8288b787c065ce72e165f81ed935c2fe2e508aa68a061ddaeee` |
| `wallet-broker/src/xmr/test_support.rs` | 1,173 | `aa737decda7cae13cd15c3f6b0de05ff15f88f96703fddece0f184bc696268d2` |
| `wallet-broker/tests/xmr_process.rs` | 452 | `0e4a3e7823e987da982fed572f1bd79e914ce730ca49aa3fb4c2260e6f7d962a` |

Every other path is read-only, including `xmr.rs`, `model.rs`, the untracked Hermes stop
record, evidence, governance, manifests, lockfiles, and all other tests/source.

Make only the mechanical layout changes Rust 1.98 rustfmt requires: line wrapping,
closure layout, import ordering, and function/assertion argument layout. Preserve every
token, identifier, literal, type, visibility, expression, statement, item, attribute,
test name, test count, comment, and behavior. Do not add/remove/reorder semantic items,
imports except rustfmt ordering, or config/test values. Do not repair, refactor, simplify,
rename, or optimize anything.

Do not run Cargo, rustfmt, tests, builds, binaries, Node/npm, package managers, security
tools, network, Git, or GitHub. Do not stage, commit, push, or edit evidence/governance.
Stop after the exact formatting-only three-path drop and report line counts/hashes plus
confirmation that no semantic token changed and no prohibited action ran.
