# Codex Spark Handoff — BBD-WAL-009 Phase-A1 Format Correction 01

Status: AUTHORIZED — EXACT ONE-PATH MECHANICAL FORMAT

Source actor: Implementation Dev — Codex Spark, High

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read `AGENTS.md`, this handoff, the Phase-A2 Expected-Red 01 Stop Review 01, the
Phase-A1 Test-Source Review 01, and `docs/handoff/CURRENT_TASK.md`. Verify these exact
starting identities:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/Cargo.toml` | 121 | `71e0135dbc2a6086ee6658e173718d2dd8c608a1f6369f732a8979d942ef6450` |
| `wallet-broker/tests/zec_sign_verify.rs` | 1,105 | `670b6d0938bf061b774bc7126b4971105208f5385b395baed69fb967c00cb4b7` |

Run this exact command once:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 rustfmt --edition 2024 wallet-broker/tests/zec_sign_verify.rs
```

It is the sole authorized mutation. Edit no other path and make no semantic,
identifier, visibility, type, literal, assertion, or comment change. Do not run Cargo,
a formatter check, tests, compiler, Clippy, audit, scanner, dependency/product command,
Git, network, wallet/node process, hardware/device action, or another actor. Report the
command exit code and resulting exact line count and SHA-256 for the test file, confirm
the manifest identity is unchanged, then stop for reviewer inspection.
