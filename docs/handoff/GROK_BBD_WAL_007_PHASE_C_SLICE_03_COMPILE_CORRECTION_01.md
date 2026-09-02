# Grok Handoff — BBD-WAL-007 Phase-C Slice 3 Compile Correction 01

Status: AUTHORIZED — ONE TEST-SUPPORT PATH ONLY

Source actor: Sr Dev — Grok Build (Grok 4.6 High)

Reviewer: Lead Engineer/Reviewer — Codex

Protected governance parent: the commit containing this handoff

Repository: `/home/lars/OpenBazaar/bb-desktop`

Read completely before acting: `AGENTS.md`, `TESTING.md`, Slice-3 Source Review 03,
Slice-3 Format Correction Source Review 01, Green Resume 01 Stop Review 01, the complete
accepted five-path drop, and `docs/handoff/CURRENT_TASK.md`.

## Baseline and exact path boundary

Require `HEAD == origin/master ==` the protected governance parent, a clean index,
`git diff --check`, and exactly these five worktree paths/identities with no other path:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/tests/xmr_rpc.rs` | 694 | `7b11f6fa6ecf29aa5556f1afc4bb7e3cd5e29018e9ff9eb1f40c5da05c3fafda` |
| `wallet-broker/src/xmr.rs` | 5 | `92af416c0ed6c5f2f8a6ff7011facb622da5e3f1ba8adaa2e96f600c09f31411` |
| `wallet-broker/src/xmr/model.rs` | 151 | `23a85dc216839cb936161845b52cacdca68a24181e5d31afeaaba30e62c77ca9` |
| `wallet-broker/src/xmr/rpc.rs` | 1,896 | `3f1f14972265fc79906c1f0f56f35b3ac55a2d68ffec7c0b91dbbea75a60c0b6` |
| `wallet-broker/src/xmr/test_support.rs` | 2,697 | `5c9bcd50558b2c9e114e3266f6300fbc380abc89178b7c4853bdb17d5892d2ed` |

Only `wallet-broker/src/xmr/test_support.rs` may change. Every other path is read-only,
including production RPC source, tests, crate roots, manifests, lockfiles, evidence,
and governance.

## Exact correction

In `valid_get_info_result`, replace only the single oversized `serde_json::json!`
construction beginning at the reported line with two or more smaller object
constructions whose entries are combined into the same final JSON object.

Preserve exactly:

- every existing member name and value expression;
- every existing JSON value type;
- the optional removal of `block_weight_limit` and `block_weight_median`;
- every existing `RpcFault` mutation and final serialization;
- every public API and all production/test behavior.

Do not add `#![recursion_limit]`, a custom macro, helper API, dependency, test,
assertion, allow attribute, or unrelated cleanup. Do not edit production RPC source or
weaken the falsification.

After the manual correction, run exactly once:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 rustfmt --edition 2024 wallet-broker/src/xmr/test_support.rs
```

Require exit 0 and no other path mutation. Do not run a formatter check, test, build,
binary, Node/npm, package-manager, security, network, Git, GitHub, evidence, or
governance command. Do not stage, commit, or push.

Stop after the formatter. Report the changed path, resulting line count and SHA-256,
the formatter exit, the exact construction strategy, and confirmation that no other
action or path was used. Reviewer inspection and a fresh Hermes falsification/green
handoff remain required.
