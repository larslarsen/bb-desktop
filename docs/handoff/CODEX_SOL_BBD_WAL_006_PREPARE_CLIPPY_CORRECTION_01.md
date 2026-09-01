# Codex Sol Handoff — BBD-WAL-006 Prepare Clippy Correction 01

You are **Principal Dev — Codex Sol**. Own only this source-style correction. Do not execute any
command or use Git.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Read completely: `AGENTS.md`, `TESTING.md`, Prepare Gate Clippy Review 02, Prepare Serde Feature
Production Source Review 01, `wallet-broker/src/zec/prepare.rs`, and `CURRENT_TASK.md`.

Use `apply_patch` to modify only `wallet-broker/src/zec/prepare.rs`, currently 963 lines and
SHA-256 `417178e0458a3a13e4f36331b8e17bb92148836631eefbdf1a0786501cd114e3`.

Make exactly these semantics-preserving corrections:

1. Collapse the nested `if let Some(binding) = binding` and its inner mismatch conditional into a
   Rust let-chain conditional. Preserve all five comparisons, their order, the `||` operators,
   and `return Err(ZecError::locked())` exactly.
2. Replace the leap-year guard
   `year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)` with
   `year.is_multiple_of(4) && (!year.is_multiple_of(100) || year.is_multiple_of(400))`.

Do not change formatting beyond what these two corrections require. Do not modify tests, another
source, manifest, policy, lock, evidence, or governance. Do not run formatter, compiler, tests,
Node, network, or any other command. Do not stage, commit, or push. Report the sole changed path,
new line count/SHA-256, and exact diff description for reviewer inspection.
