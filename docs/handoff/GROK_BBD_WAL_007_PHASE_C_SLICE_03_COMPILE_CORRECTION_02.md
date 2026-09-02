# Grok Handoff — BBD-WAL-007 Phase-C Slice 3 Compile Correction 02

Status: AUTHORIZED — TWO EXACT COMPILE REPAIRS

Source actor: Sr Dev — Grok Build (Grok 4.6 High)

Reviewer: Lead Engineer/Reviewer — Codex

Protected governance parent: the commit containing this handoff

Repository: `/home/lars/OpenBazaar/bb-desktop`

Read completely before acting: `AGENTS.md`, `TESTING.md`, Slice-3 Source Review 03,
Slice-3 Compile Correction Source Review 01, Green Resume 02 Stop Review 01, the complete
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
| `wallet-broker/src/xmr/test_support.rs` | 2,702 | `fbee13d0a646966359dd408bc8e9b6ab672c47ffbb81ea1c8fa5ac2bbaac7e80` |

Only `wallet-broker/src/xmr/rpc.rs` and `wallet-broker/src/xmr/test_support.rs` may
change. Every other path is read-only, including tests, crate roots, manifests,
lockfiles, evidence, and governance.

## Exact corrections

Make only these two repairs:

1. In `build_authorization`, change the existing cnonce expression
   `hex_bytes(&source)` to `hex_bytes(&*source)`. Do not change the `Zeroizing<[u8; 16]>`
   owner, entropy fill, encoding, or any other digest/authentication code.
2. In `apply_json_fault`, bind the existing `match fault` result to a local replacement
   value, then assign that completed value to `*body`. Preserve every match arm and its
   exact byte construction. `None` must still return without assignment.

Do not modify tests, assertions, APIs, error behavior, cryptography, production policy,
dependencies, compiler settings, allow attributes, or unrelated code.

After both manual corrections, run exactly once:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 rustfmt --edition 2024 wallet-broker/src/xmr/rpc.rs wallet-broker/src/xmr/test_support.rs
```

Require exit 0 and no other path mutation. Do not run a formatter check, test, build,
binary, Node/npm, package-manager, security, network, Git, GitHub, evidence, or
governance command. Do not stage, commit, or push.

Stop after the formatter. Report both changed paths, resulting line counts and SHA-256
identities, formatter exit, the exact two edits, and confirmation that no other action
or path was used. Reviewer inspection and a fresh Hermes falsification/green handoff
remain required.
