# Grok Handoff — BBD-WAL-007 Phase-C Slice 3 Warning Correction 01

Status: AUTHORIZED — ONE PRODUCTION PATH, NO NEW DISPATCH

Source actor: Sr Dev — Grok Build (Grok 4.6 High)

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Protected governance parent: the commit containing this handoff

Repository: `/home/lars/OpenBazaar/bb-desktop`

Read completely before acting: `AGENTS.md`, `TESTING.md`, Slice-3 Source Review 03,
Test Oracle Correction Source Review 01, Green Evidence 01 Rejection 01, the complete
committed five-path drop, and `docs/handoff/CURRENT_TASK.md`.

## Baseline and exact path boundary

Require `HEAD == origin/master ==` the protected governance parent, a clean index and
tracked/untracked worktree, and `git diff --check`.

Require this exact source baseline:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/tests/xmr_rpc.rs` | 694 | `a1face1660ca0daf66002671ba3d794058b4acd497de2f0a3e25f2ca57597d0b` |
| `wallet-broker/src/xmr.rs` | 5 | `92af416c0ed6c5f2f8a6ff7011facb622da5e3f1ba8adaa2e96f600c09f31411` |
| `wallet-broker/src/xmr/model.rs` | 151 | `23a85dc216839cb936161845b52cacdca68a24181e5d31afeaaba30e62c77ca9` |
| `wallet-broker/src/xmr/rpc.rs` | 1,896 | `2186f6fd56cfcce1edfbe00cd247ef2ab2177ba04dfd9b505416a1da33cbe4ed` |
| `wallet-broker/src/xmr/test_support.rs` | 2,705 | `2c445494b36fd645240cc9a25405782eff6d3e8187b4762aee6a0fd7ca640a40` |

Only `wallet-broker/src/xmr/rpc.rs` may change. Every other path is read-only, including
tests, support source, crate roots, manifests, lockfiles, evidence, and governance.

## Exact correction

Make only these two changes:

1. Remove the unused final `use core::fmt::Write as _;` line. Retain the existing
   `std::io::Write` import used by HTTP request construction.
2. Refactor only `request_dispatch_for_test` so it first performs a closed match from
   each of the 16 exact authority strings to the corresponding `RpcMethod` variant and
   returns false for every unknown string. It must then return true only for this exact
   implemented subset:

```text
GetVersion
CloseWallet
StopWallet
GetHeight
GetBalance
CreateAddress
GetInfo
HardForkInfo
```

The other eight closed variants — `CreateWallet`, `RestoreDeterministicWallet`,
`GenerateFromKeys`, `OpenWallet`, `QueryKey`, `Refresh`, `GetAddress`, and
`ValidateAddress` — must be constructed by the closed name match but return false from
the implemented-subset check. This is a test-only classifier; do not add any
`RpcRequest`, transport, public method-string API, dispatch path, or capability.

Do not add an `allow` attribute, modify tests or response parsing, remove any
`RpcMethod` variant, change method names, or make any unrelated edit.

After the manual correction, run exactly once:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 rustfmt --edition 2024 wallet-broker/src/xmr/rpc.rs
```

Require exit 0 and no other path mutation. Do not run a formatter check, test, build,
binary, Node/npm, package-manager, security, network, Git, GitHub, evidence, or
governance command. Do not stage, commit, or push.

Stop after the formatter. Report the changed path, resulting line count and SHA-256,
formatter exit, the exact closed-name and implemented-subset behavior, and confirmation
that no other action or path was used. Reviewer inspection and a wholly fresh Hermes
falsification/green handoff remain required.
