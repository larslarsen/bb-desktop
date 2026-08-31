# Codex Sol Handoff — BBD-WAL-006 Store Format Correction 01

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This durable handoff is
authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Governance parent: the commit containing this handoff

Read completely: `AGENTS.md`, `TESTING.md`, Store Gate 01, Store Source Review 02,
`BBD-WAL-006-STORE-GATE-FORMAT-REVIEW-01.md`, and the current four source paths.

## Sole task

Use `apply_patch` to make only the Rust 1.98.0 rustfmt-equivalent import grouping and line-wrapping
changes reported at these current locations:

- `wallet-broker/src/zec/fixture.rs`: line 52;
- `wallet-broker/src/zec/store.rs`: imports at lines 1 and 14; lines 116, 389, 502, 652, 725,
  767, 778, 786, 797, 896, 1093, 1135, 1163, and 1310; and
- `wallet-broker/src/zec/test_support.rs`: lines 377 and 586.

Inspect the nearby expressions and manually apply canonical rustfmt layout. Do not run rustfmt or
any command to generate replacements. Do not alter tokens, semantics, names, visibility, types,
constants, SQL, bounds, comments, or control flow. A location that is already canonical after
another listed wrap needs no separate edit.

`wallet-broker/src/zec.rs` is frozen at 214 lines/SHA-256
`800111bf587265b49764de71c6d2beb054d1906932005bb3513b2d4fc132d90e`. Tests, `lib.rs`,
`address.rs`, fixture bytes/manifest, dependencies/lock, policy, governance, and every unlisted
path are frozen.

## Restrictions and report

Do not run Cargo, rustfmt, compiler, test, linter, Node, policy, Git, network, cleanup, or any
other execution. Do not stage, commit, or push.

Report each edited expression, the final line count and SHA-256 for all three authorized paths,
the frozen `zec.rs` hash, and any ambiguity. Stop rather than make a semantic or unlisted edit.
Hermes will restart Store Gate 01 from its first command only after reviewer acceptance.
