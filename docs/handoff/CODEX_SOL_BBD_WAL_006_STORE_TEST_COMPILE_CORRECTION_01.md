# Codex Sol Handoff — BBD-WAL-006 Store Test Compile Correction 01

You are **Principal Dev — Codex Sol**. This durable handoff is authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely before acting: `AGENTS.md`, `TESTING.md`, Store Test Source Review 01, Store Gate
Test Compile Review 01, and `docs/handoff/CURRENT_TASK.md`.

## Sole task

Using `apply_patch`, edit only `wallet-broker/tests/zec_store.rs`. Require its starting identity to
be 334 lines/SHA-256
`492e4e6934f8cd9589de22cc338fd5e93131f3f3d3fcca5f79b44455b297e1ca`. Replace only these two
complete assertions:

```rust
    assert!(inspection.decoded_value_kinds().contains(&"text"));
    assert!(inspection.decoded_value_kinds().contains(&"blob"));
```

This preserves the exact two membership requirements while removing the invalid `&&str == str`
comparisons. Do not edit production source, another test, fixture, manifest, dependency, policy,
workflow, ticket, or documentation. Do not run a formatter, compiler, Clippy, test, policy, Git,
or network command. Do not stage, commit, or push.

Return the resulting line count and SHA-256, enumerate the two replacements, confirm the eight
test names and all other test source are unchanged, and report any ambiguity. The reviewer will
inspect and decide whether Hermes may restart the full gate.
