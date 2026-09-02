# Grok Handoff — BBD-WAL-007 Phase-C Slice 3 Compile Correction 04

Status: AUTHORIZED — TWO DERIVES IN ONE TEST-SUPPORT PATH

Source actor: Sr Dev — Grok Build (Grok 4.6 High)

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Protected governance parent: the commit containing this handoff

Repository: `/home/lars/OpenBazaar/bb-desktop`

Read completely before acting: `AGENTS.md`, `TESTING.md`, Slice-3 Source Review 03,
all three Slice-3 Compile Correction Source Reviews, Green Resume 04 Stop Review 01,
the complete accepted five-path drop, and `docs/handoff/CURRENT_TASK.md`.

## Baseline and exact path boundary

Require `HEAD == origin/master ==` the protected governance parent, a clean index,
`git diff --check`, and exactly these five worktree paths/identities with no other path:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/tests/xmr_rpc.rs` | 694 | `7b11f6fa6ecf29aa5556f1afc4bb7e3cd5e29018e9ff9eb1f40c5da05c3fafda` |
| `wallet-broker/src/xmr.rs` | 5 | `92af416c0ed6c5f2f8a6ff7011facb622da5e3f1ba8adaa2e96f600c09f31411` |
| `wallet-broker/src/xmr/model.rs` | 151 | `23a85dc216839cb936161845b52cacdca68a24181e5d31afeaaba30e62c77ca9` |
| `wallet-broker/src/xmr/rpc.rs` | 1,896 | `2186f6fd56cfcce1edfbe00cd247ef2ab2177ba04dfd9b505416a1da33cbe4ed` |
| `wallet-broker/src/xmr/test_support.rs` | 2,704 | `f3ff67c4958ab66f1167779639667611d9117f3c594aa4140c6e8f73fc9f3130` |

Only `wallet-broker/src/xmr/test_support.rs` may change. Every other path is read-only,
including production RPC source, tests, crate roots, manifests, lockfiles, evidence,
and governance.

## Exact correction

Make only these two derive changes:

1. Add `#[derive(Debug)]` immediately above `pub struct NodeProbeView`.
2. Change the existing `#[derive(Clone, Copy)]` immediately above `NodeStateView` to
   `#[derive(Clone, Copy, Debug)]`.

Do not add a manual `Debug` implementation or derive another trait. Do not modify any
field, visibility, test, assertion, production type, API beyond these trait
implementations, error behavior, dependency, compiler setting, allow attribute, or
unrelated code.

After the manual correction, run exactly once:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 rustfmt --edition 2024 wallet-broker/src/xmr/test_support.rs
```

Require exit 0 and no other path mutation. Do not run a formatter check, test, build,
binary, Node/npm, package-manager, security, network, Git, GitHub, evidence, or
governance command. Do not stage, commit, or push.

Stop after the formatter. Report the changed path, resulting line count and SHA-256,
formatter exit, the exact derives, and confirmation that no other action or path was
used. Reviewer inspection and a fresh Hermes falsification/green handoff remain
required.
