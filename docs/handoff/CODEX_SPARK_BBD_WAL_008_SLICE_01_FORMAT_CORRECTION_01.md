# Codex Spark Handoff — BBD-WAL-008 Slice-01 Format Correction 01

Status: AUTHORIZED — PINNED FORMATTER ON TWO PATHS ONLY

Source actor: Implementation Dev — Codex Spark, GPT-5.3-Codex-Spark High

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely: `AGENTS.md`, `TESTING.md`, `docs/handoff/CURRENT_TASK.md`, Slice-01
Source Review 01, Green Stop Review 01, and both mutable paths.

Start from:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/zec/hardware.rs` | 868 | `590199f7ced6ca7389d8536e9a453ff082e1769a4f0b0ae9907d7d1d2c394aaf` |
| `wallet-broker/src/zec/test_support.rs` | 2,385 | `f8778937c22eeabcc5257c2e6458b20433b936c1b323cc4435ddde64f8e50697` |

Freeze `wallet-broker/src/zec.rs` at 253 lines and SHA-256
`6dd71f9f70d7b5b8aaddd2e3d4df2b9b2b232b45182ca1db4d32078146751fa2`.
Every other path is read-only.

Run exactly once from the repository root:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 rustfmt --edition 2024 wallet-broker/src/zec/hardware.rs wallet-broker/src/zec/test_support.rs
```

This is the sole authorized mutation. It must exit 0. Do not manually edit, run
`cargo fmt`, formatter check, Cargo, compiler, test, Clippy, Git, network, product,
policy, or another actor. Stop immediately afterward and report the two resulting
line counts/hashes, the frozen `zec.rs` identity, exact exit, and confirmation that no
other path or action was used.
