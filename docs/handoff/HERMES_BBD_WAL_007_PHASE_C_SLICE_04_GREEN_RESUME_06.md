# Hermes Handoff — BBD-WAL-007 Phase-C Slice 4 Green Resume 06

You are **Jr Dev — Hermes**. Protected governance parent is the commit containing this
handoff.

Read `AGENTS.md`, `TESTING.md`, Green Resume 05 Rejection 01, Clippy Correction 01
Source Review 01, the complete prior
`HERMES_BBD_WAL_007_PHASE_C_SLICE_04_GREEN_RESUME_03.md`, and `CURRENT_TASK.md`.

Resume 03 applies verbatim from the beginning, including every command, expected
result, stop condition, evidence requirement, commit message, and push rule, with these
identity/scope replacements:

| Source path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/vault.rs` | 776 | `bb79b16f24a9abbcd266e856a37ee982bde8fcdc2da3858e75d34bdd34a4af86` |
| `wallet-broker/src/xmr.rs` | 7 | `8b382ef673e82001720727c161f365c0fa3c1401d1bdaa4d20ee2683c001559b` |
| `wallet-broker/src/xmr/account.rs` | 3,034 | `67cc2261c138b83f3fa963bfe6ce646bea17c9258185d986a4c43daf0662c137` |
| `wallet-broker/src/xmr/distribution.rs` | 914 | `163f8532bc7edfd80fc07966c0f8f32eebc0d12181fd273bc4e6c2870d86dea8` |
| `wallet-broker/src/xmr/store.rs` | 1,327 | `248ca3f6eaeb98b66fbe2d041637c521f3b2371b8b9c231cbcdd3d3c57174607` |
| `wallet-broker/src/xmr/process.rs` | 1,763 | `98a18be4a0f26ae71b5818ba893910d3183a3ddea49263c9291185fbde09fc2f` |
| `wallet-broker/src/xmr/rpc.rs` | 2,428 | `381ebe2d234d2f6f3c1b6ac9ab6dcec506fc815553d01e12053bc9e51b46f556` |
| `wallet-broker/src/xmr/test_support.rs` | 4,765 | `b0c5888d32e8aaca02593dfc1f76de17f38aea28ec70e1ec4b56ef01ccd5e3b8` |

Require exactly these eight worktree paths, a clean index, all original frozen test
identities, and `HEAD == origin/master ==` the protected parent. Record separately and
before execution: `hermes --version`, `hermes config get model.provider`, `hermes
config get model.default`, and disk-backed `wallet-broker/target` filesystem proof.

Each terminal command must be one separate tool call that finishes before the next.
Never batch, wrap, chain, redirect, pipe, alter, or repeat. On the first mismatch,
restore temporary falsification if present, prove only its identity, and stop—no extra
command, evidence, staging, commit, or push.

On exact success, stage the eight sources above plus green evidence and `CURRENT_TASK.md`
(ten paths total), commit/push exactly as Resume 03 specifies, then stop after read-only
Git/identity proof. No repair, source/test edit, another actor, Slice 5, broader/final
acceptance, or real local-Monero gate is authorized.
