# Codex Sol Handoff — BBD-WAL-004 Correction 5 Production

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This durable file is
the complete prompt; ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff.

Read `AGENTS.md`, `TESTING.md`, roles, `tickets/BBD-WAL-004.md`, source review 05,
`docs/testing/BBD-WAL-004-GREEN-RUN-04.md`, and both complete authorized source files.

Edit only:

- `wallet-broker/src/store.rs` — 532 lines, SHA-256
  `59948a11da60c398035e88ef1b17530d241911e982397223a549d00fc3d82499`;
- `wallet-broker/src/vault.rs` — 760 lines, SHA-256
  `8ce6bd4313e5972161e3258b92877ade9f4f7f54faba3e25a66798940abf0aea`.

Apply exactly the three captured Clippy corrections:

1. Collapse the nested `RestoreState::Authenticated(current)` and
   `candidate.epoch <= current` conditions into the suggested let-chain, retaining the
   same `StoreError::replay()` result and evaluation order.
2. Replace `passphrase.len() >= 1` with `!passphrase.is_empty()` and preserve the
   remaining upper-bound and UTF-8 conjunction unchanged.
3. After the existing exact 32-character/lowercase-hex validation, replace
   `value.as_bytes().chunks_exact(2)` with the suggested
   `value.as_bytes().as_chunks::<2>().0.iter()`. Preserve enumeration, nibble decoding,
   output, and every rejection behavior.

Do not change behavior, tests, formatting elsewhere, cryptography, account syntax,
restore policy, dependencies, features, policy, or another path. The exact denied
Clippy diagnostics are the regression proof; all 78 Rust behavioral tests and all Node
gates are already green.

Use `apply_patch`. Do not run Rust, Cargo, Node, tests, formatters, linters, builds,
scanners, network, Git, or project commands. Do not stage, commit, push, delete, use
`/tmp`, or use root. Stop on contradiction.

After editing, only `wc -l` and `sha256sum` over the two authorized paths are allowed.
Report exact counts/hashes and no blocker. Luna owns all execution, evidence, Git, and
push after reviewer source acceptance.
