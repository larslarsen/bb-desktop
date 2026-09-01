# Codex Sol Handoff — BBD-WAL-006 Scan Format Correction 02

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This durable handoff is
authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely before acting: `AGENTS.md`, `TESTING.md`, Scan Clippy Correction Review 02, Scan
Gate Format Review 02, and `docs/handoff/CURRENT_TASK.md`.

## Sole task

In `wallet-broker/src/zec/scan.rs`, use `apply_patch` to replace exactly:

```rust
                && plan.fixture.manifest.expected.confirmation_height == local.confirmation_height() => {
        }
```

with:

```rust
                && plan.fixture.manifest.expected.confirmation_height
                    == local.confirmation_height() => {}
```

Make no other edit. Do not run rustfmt, Cargo, Rust, Clippy, tests, Node, npm, policy, a compiler,
a linter, Git, network, fixture, wallet, node, device, cleanup, or deletion command. Do not stage,
commit, or push. Do not change a token, predicate, comparison, branch, value, import, visibility,
or adjacent layout.

Require the starting `scan.rs` to be 1,400 lines with SHA-256
`adcc10fac7f6629b44d08be2c75d794a62726e3366398d0b3b02d44991206681`. These four source
paths are frozen:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/zec.rs` | 218 | `ae06d6a9b4448603860b55bf4eef42a1e970c414049a9304f42b436d2bf49cb4` |
| `wallet-broker/src/zec/fixture.rs` | 614 | `318820c6f125f2318ba0caf5ae44835b36b0f67856fc66b83401961f0c301b36` |
| `wallet-broker/src/zec/store.rs` | 1,816 | `d0b97b802209489804e5d550ef90138a87342e155c519a686b2d900189f6603f` |
| `wallet-broker/src/zec/test_support.rs` | 1,220 | `02609ae79bb8d35a5c7fb0058f0ab56fa5deb27c94cfa2ddb81347a2f38ac034` |

Require `wallet-broker/src/zec/prepare.rs` to remain absent. Every other source, test, fixture,
manifest, lockfile, policy, document, workflow, package, and repository path is frozen.

After the patch, use only read-only file inspection, `wc -l`, and `sha256sum`. Return the new
`scan.rs` line count and hash, re-prove the four frozen identities and absent `prepare.rs`, quote
the corrected fragment, and confirm no ambiguity or semantic change remains. The reviewer will
inspect and decide whether Hermes may restart the gate.
