# Codex Spark Handoff — BBD-WAL-007 Slice 4 Format Correction 03

Status: AUTHORIZED — EXACT TWO-PATH MECHANICAL EDIT

Source actor: Implementation Dev — Codex Spark, GPT-5.3-Codex-Spark High

Protected governance parent: the commit containing this handoff

Read `AGENTS.md`, `TESTING.md`, `CURRENT_TASK.md`, this handoff, and Green Resume 06
Stop Review 01. Edit only:

- `wallet-broker/src/xmr/rpc.rs`: 2,428 lines,
  `381ebe2d234d2f6f3c1b6ac9ab6dcec506fc815553d01e12053bc9e51b46f556`;
- `wallet-broker/src/xmr/test_support.rs`: 4,765 lines,
  `b0c5888d32e8aaca02593dfc1f76de17f38aea28ec70e1ec4b56ef01ccd5e3b8`.

Apply exactly the two Rust 1.98 formatter hunks preserved in the stop review transcript:

```rust
pub(crate) fn digest_response_for_test(input: DigestResponseInput<'_>) -> Result<String, XmrError> {
```

```rust
            for (index, chunk) in self
                .account_id
                .as_bytes()
                .as_chunks::<2>()
                .0
                .iter()
                .enumerate()
```

Replace only the corresponding existing layouts. Make no semantic or other whitespace
change. Freeze every other path, especially tests and the other six accepted sources.
Do not run rustfmt, Cargo, compiler, tests, Clippy, binaries, Node/npm, network, Git, or
GitHub; do not edit governance/evidence or invoke another actor. Report resulting line
counts and SHA-256 identities, then stop.
