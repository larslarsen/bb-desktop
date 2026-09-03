# Codex Spark Handoff — BBD-WAL-007 Slice 4 Format Correction 02

Status: AUTHORIZED — EXACT ONE-PATH MECHANICAL EDIT

Source actor: Implementation Dev — Codex Spark, GPT-5.3-Codex-Spark High

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read `AGENTS.md`, this handoff, Green Resume 03 Stop Review 01, and
`docs/handoff/CURRENT_TASK.md`. Verify `wallet-broker/src/xmr/test_support.rs` is
4,782 lines with SHA-256
`e422ed545d8c96127c240e64d899ca536f7bd9a454d5da03ea980a32013cb3b6`.

Edit only that file. Apply exactly the three transformations printed by Rust 1.98:

```rust
        let mut port =
            RecordingAccountPort::new("00112233445566778899aabbccddeeff", XmrNetwork::Stagenet);
```

```rust
const PUBLIC_DIAGNOSTIC_FIELDS: [&str; 5] = ["operation", "account_id", "asset", "network", "code"];
```

```rust
            .map(|canary| (canary.class, Zeroizing::new(canary.value.to_owned())))
```

Replace only the corresponding existing multiline forms. Make no semantic, identifier,
visibility, type, comment, or other whitespace change. Freeze every other path,
especially the other six accepted sources and all tests.

Do not run rustfmt, Cargo, compiler, tests, Clippy, binaries, Node/npm, network, Git, or
GitHub. Do not edit governance/evidence or invoke another actor. Report the resulting
line count and SHA-256, then stop for reviewer inspection.
