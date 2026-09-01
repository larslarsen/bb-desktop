# Codex Sol Handoff — BBD-WAL-006 Prepare Format Correction 04

You are **Principal Dev — Codex Sol**. Own only this captured rustfmt correction. Do not execute
any command or use Git.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Read completely: `AGENTS.md`, `TESTING.md`, Prepare Gate Format Review 04, Prepare Clippy
Correction Review 01, `wallet-broker/src/zec/prepare.rs`, and `CURRENT_TASK.md`.

Use `apply_patch` to modify only `wallet-broker/src/zec/prepare.rs`, currently 963 lines and
SHA-256 `3c5a64d718ab108bc91186a7d709c858cb9cc643349563019b12f1578a0928ca`.

Replace exactly:

```rust
        2 if year.is_multiple_of(4)
            && (!year.is_multiple_of(100) || year.is_multiple_of(400)) => 29,
```

with exactly:

```rust
        2 if year.is_multiple_of(4) && (!year.is_multiple_of(100) || year.is_multiple_of(400)) => {
            29
        }
```

Do not make any other edit. Do not modify tests, another source, manifest, policy, lock, evidence,
or governance. Do not run formatter, compiler, tests, Node, network, or any other command. Do not
stage, commit, or push. Report the sole changed path and direct patch-accounting line delta;
reviewer will verify the resulting identity.
