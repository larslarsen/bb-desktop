# Codex Sol Handoff — BBD-WAL-006 Scan Runtime Fail-Closed Correction 01

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This durable handoff is
authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely before acting: `AGENTS.md`, Scan Runtime Source Review 01, the active runtime
snapshot review, and the complete current aggregate in `wallet-broker/src/zec/scan.rs`.

## Sole authorized edit

Edit only `wallet-broker/src/zec/scan.rs`, starting at 1,665 lines and SHA-256
`10fda0c090d66159e5266fee5e2545d150b23d953f455dc82c916485ba49eee5`.

Within only the independent `malformed` `EXISTS` subquery in `orphan_projection`:

1. Change the join from `JOIN transactions spending_tx` to
   `LEFT JOIN transactions spending_tx`.
2. Add `spending_tx.id_tx IS NULL` as the first disjunct in the parenthesized malformed spending
   transaction conditions.

Do not change the separate official `NOT EXISTS` unexpired-spend predicate; its inner join is
exact upstream behavior. Do not change another SQL token, helper, import, signature, source path,
test, fixture, manifest, lockfile, document, policy, workflow, or repository path.

Use `apply_patch` only. Do not run rustfmt, Cargo, Rust, Clippy, tests, Node, npm, policy, a
compiler, linter, Git, network, fixture, wallet, node, device, cleanup, or deletion command. Do
not stage, commit, or push.

After editing, use only read-only inspection, `wc -l`, and `sha256sum`. Return the new `scan.rs`
line count/hash, the exact changed hunk, and re-prove that `zec_scan.rs` remains 325 lines at
`87ed1c3e8db8219ad126efb4de7f681cf909cb404b88df6c2dfadec471a3701f` and the four supporting
source identities remain frozen. Report any ambiguity and stop rather than broadening the edit.
