# Codex Sol Handoff — BBD-WAL-007 Phase-C Slice 2 Format Correction 03

Status: AUTHORIZED — APPLY THE EXACT RECORDED FORMATTER DIFF ONLY

Source actor: Principal Dev — Codex Sol (`gpt-5.6-sol`, High)

Reviewer: Lead Engineer/Reviewer — Codex; XHigh review required before execution

Protected governance parent: the commit containing this handoff

Repository: `/home/lars/OpenBazaar/bb-desktop`

Read completely before acting: `AGENTS.md`, `TESTING.md`, Slice-2 Source Review 02,
both green-stop reviews, Format-Correction Source Review 01, Format-Correction 02 Stop
Review 01, Exact Formatter Diff 01, the complete three paths below, and
`docs/handoff/CURRENT_TASK.md`.

## Exact source boundary

Edit only these three paths from these exact identities:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/xmr/process.rs` | 1,189 | `6e47fa9a6d07f4028331b8e9f3b859c54c2507ab78fb669856fb495d22714712` |
| `wallet-broker/src/xmr/test_support.rs` | 1,157 | `8e4720f77e60f35b8b40783e5957b2a48c0e5a1ab675bfb04fd5c1b5c11727ca` |
| `wallet-broker/tests/xmr_process.rs` | 455 | `395496959636b78f9896bec3b47e58c89b41fa70f1156c279de0a73931d617f7` |

Apply every minus-to-plus transformation, and only those transformations, in
`docs/testing/BBD-WAL-007-SLICE-02-FORMATTER-DIFF-01.md`. All recorded context and
minus lines must match before editing. Stop without editing on any mismatch. Do not
infer, omit, add, or revise a formatter change.

Every other path is read-only, including `xmr.rs`, `model.rs`, the untracked Hermes stop
record, evidence/governance, manifests, lockfiles, and all other tests/source. Preserve
every semantic token, import target, identifier, literal, type, visibility, expression,
statement, item, attribute, test name/count, comment, and behavior.

Do not run Cargo, rustfmt, tests, builds, binaries, Node/npm, package managers, security
tools, network, Git, or GitHub. Do not stage, commit, push, or edit evidence/governance.
Stop after the exact recorded formatting drop and report new line counts/hashes,
confirmation that every recorded hunk was applied exactly, that no semantic token
changed, and that no prohibited action ran.
