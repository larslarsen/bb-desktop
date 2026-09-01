# Codex Sol Handoff — BBD-WAL-006 Scan Format Correction 01

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This durable handoff is
authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely before acting: `AGENTS.md`, `TESTING.md`, Scan Truth Correction Review 01, Scan
Gate 01, Scan Gate Formatter Review 01, Scan Format Capture Review 01, and
`docs/handoff/CURRENT_TASK.md`.

## Sole task

Read the complete retained formatter artifact at
`wallet-broker/target/wal006-scan-format.stdout`. Require exactly 577 lines and SHA-256
`261480b0560df8ddbfb1a71d33c068fffdca2b31fd4c9d58801cbe061ee444a3`. Its 34 `Diff in`
sections are the exhaustive patch specification: five target `fixture.rs`, 22 target `scan.rs`,
and seven target `test_support.rs`.

Using `apply_patch`, replace the `-` side of every captured hunk with its exact `+` side. Apply all
34 hunks and no other change. Do not infer, improve, reformat, or clean up adjacent source. Do not
run rustfmt, Cargo, Rust, Clippy, tests, Node, npm, policy, a compiler, a linter, Git, network,
fixture, wallet, node, device, cleanup, or deletion command. Do not stage, commit, or push.

## Starting source identities

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/zec.rs` | 218 | `ae06d6a9b4448603860b55bf4eef42a1e970c414049a9304f42b436d2bf49cb4` |
| `wallet-broker/src/zec/fixture.rs` | 600 | `708ebba85b215b873bacf580156dace9cd68e3d6ed6feb164719c1ff7c9776ee` |
| `wallet-broker/src/zec/scan.rs` | 1,368 | `6f7ef21d8bd951e071ed6b4454ffad0a27ad334cdd4b4c671d1a11e042406e9e` |
| `wallet-broker/src/zec/store.rs` | 1,814 | `3b475dadffa6c1adc4c500c020c82d4e0571805c51d049f475ad4ee08ffbf894` |
| `wallet-broker/src/zec/test_support.rs` | 1,231 | `10f453de6e41de698c60255881715b9211a14a8642ffb59ce307eeddadb3ca6c` |

Require `wallet-broker/src/zec/prepare.rs` to remain absent. `zec.rs`, `store.rs`, every test,
fixture, manifest, lockfile, policy, document, workflow, package, and all other repository paths
are frozen.

After applying the patch, use only read-only file inspection, `wc -l`, and `sha256sum`. Return the
three resulting line counts and hashes, re-prove the two frozen source identities, enumerate the
34 applied capture sections by path and original line label, and confirm no ambiguity or semantic
change remains. The reviewer will inspect the source and decide whether Hermes may restart the
gate.
