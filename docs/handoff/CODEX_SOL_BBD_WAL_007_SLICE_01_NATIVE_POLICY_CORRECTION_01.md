# Codex Sol Handoff — BBD-WAL-007 Slice-1 Native Policy Correction 01

Status: AUTHORIZED — TWO-PATH POLICY CORRECTION ONLY

Source actor: Principal Dev — Codex Sol (`gpt-5.6-sol`, High)

Reviewer: Lead Engineer/Reviewer — Codex; XHigh review required before execution

Protected governance parent: the commit containing this handoff

Repository: `/home/lars/OpenBazaar/bb-desktop`

Read completely before acting: `AGENTS.md`, `TESTING.md`,
`tickets/BBD-WAL-007.md`, both Slice-1 source reviews, Slice-1 Node Stop Review 01,
the current provisional Green 01 evidence, and `docs/handoff/CURRENT_TASK.md`.

## Sole task and authorized paths

Correct the closed Rust-source policy collision caused by the exact approved native XMR
picker title. Edit only:

- `scripts/security-policy.js` — 2,678 lines, SHA-256
  `6dbf22fb3980e424d2bb108ca568612b8cb23f2c7307d45543871486c18eb3f6`;
- `test/securityPolicy.node.js` — 3,162 lines, 86 tests, SHA-256
  `f3464fe3f429c55f66cf1ac18e1a7be70d0d50263433b26068f2f20fa0dc3dad`.

Every other path is read-only, including all Rust source/tests, manifests, lockfiles,
evidence, governance, workflows, packages, Electron files, `bb-go`, and `go-ipfs`.

## Required correction

In `checkRustWalletSource`, retain the generic `monero` authority rejection everywhere.
For exactly `wallet-broker/src/native_ui.rs`, permit zero or one occurrence of exactly
this reviewed UI-only literal:

```text
"Select monero-wallet-rpc"
```

Remove only that exact occurrence from the text screened by the existing generic
forbidden-authority patterns. Reject more than one occurrence. Every alternate case,
spelling, surrounding value, Monero executable/process/RPC authority, and every Monero
token outside that exact literal and exact path must continue to fail. Do not weaken the
process, network, unsafe, FFI, temporary-directory, lossy-path, dependency, inventory,
WAL-006, or WAL-007 rules.

Within the existing WAL-004 vault/native policy test, add assertions proving:

1. the exact native-UI path with one exact reviewed picker title is accepted;
2. the same literal on another source path is rejected;
3. duplicate exact picker-title literals on native UI are rejected; and
4. alternate or additional `monero` text on native UI is rejected.

Keep the Node test count exactly 86. Preserve the five cumulative WAL-007 inventories,
the current top-level `xmr.rs` repository shape, and every other assertion. Do not edit
the accepted `native_ui.rs` to evade policy.

## Prohibited actions and delivery

Do not run tests, Node, Cargo, formatters, builds, binaries, npm, package managers,
security tools, network, Git, or GitHub. Do not stage, commit, push, edit evidence, or
begin another slice.

Stop after the two-path source drop. Report exact hashes/line counts, unchanged 86-test
count, the closed exception shape, the new negative assertions, and confirmation of
scope and prohibited commands. Reviewer XHigh acceptance is required before Hermes may
resume execution.
