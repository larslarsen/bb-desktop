# Codex Spark Handoff — BBD-WAL-008 Slice-02 Format Correction 01

Status: AUTHORIZED — PINNED FORMATTER ON TWO PATHS ONLY

Source actor: Implementation Dev — Codex Spark, GPT-5.3-Codex-Spark High

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely: `AGENTS.md`, `TESTING.md`, `docs/handoff/CURRENT_TASK.md`, Slice-02
Source Review 01, Green Stop Review 01, and both mutable paths.

Start from:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/zec/hardware.rs` | 925 | `39feb5c6ce943546f1b1d823f35cef405b81a2c3eb0cb8a6687152b93910784d` |
| `wallet-broker/src/zec/store.rs` | 2,865 | `852b32a8d8ff5ff3a243d5cdaa4e00dae17b82a2602b73626ba6b4aeb8565e4e` |

Freeze `wallet-broker/src/zec/test_support.rs` at 2,500 lines and SHA-256
`e3edc609dc6e3eaa80de52b6269caa0a84525e91e761483013b92affe5048f82`.
Every other path is read-only.

Run exactly once from the repository root:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 rustfmt --edition 2024 wallet-broker/src/zec/hardware.rs wallet-broker/src/zec/store.rs
```

This is the sole authorized mutation. It must exit 0. Do not manually edit, run
`cargo fmt`, formatter check, Cargo, compiler, test, Clippy, Git, network, product,
policy, or another actor. Stop immediately afterward and report the two resulting line
counts/hashes, the frozen `test_support.rs` identity, exact exit, and confirmation that
no other path or action was used.
