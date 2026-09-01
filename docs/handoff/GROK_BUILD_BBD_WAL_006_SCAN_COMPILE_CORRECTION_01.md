# Grok Build Handoff — BBD-WAL-006 Scan Compile Correction 01

You are **Sr Dev — Grok Build**, using Grok 4.6 High. This durable handoff is authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely before acting: `AGENTS.md`, `TESTING.md`, Scan Truth Correction Review 01, Scan
Format Correction Review 01, Scan Gate Clippy Review 01, and `docs/handoff/CURRENT_TASK.md`.

## Sole task

Apply exactly these two bounded compile corrections with `apply_patch`:

1. In `wallet-broker/src/zec/scan.rs`, add `network: Network` immediately after `account_id` in
   the private `execute_with_params` signature. Pass the existing `network` argument immediately
   after `account_id` at both calls from public `execute`. Make no other edit; the existing
   `stored_ufvk(root, paths, account_id, network)` call remains unchanged.
2. In `wallet-broker/src/zec/fixture.rs`, replace only
   `let mut previous_height = None;` with
   `let mut previous_height: Option<u32> = None;`.

Do not run rustfmt, Cargo, Rust, Clippy, tests, Node, npm, policy, a compiler, a linter, Git,
network, fixture, wallet, node, device, cleanup, or deletion command. Do not stage, commit, or
push. Do not change formatting except where `apply_patch` necessarily inserts the three
`network` argument/parameter lines. Do not address an unobserved warning or infer another fix.

## Starting source identities

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/zec.rs` | 218 | `ae06d6a9b4448603860b55bf4eef42a1e970c414049a9304f42b436d2bf49cb4` |
| `wallet-broker/src/zec/fixture.rs` | 614 | `59709f36f4de70bcef3e0c4d89a73746bbca1491ad4cbde92ed96e868403174a` |
| `wallet-broker/src/zec/scan.rs` | 1,396 | `e09647e0be673e76a421f60f0c70913ce2f021d02971727eb5f4423f3796e3ff` |
| `wallet-broker/src/zec/store.rs` | 1,814 | `3b475dadffa6c1adc4c500c020c82d4e0571805c51d049f475ad4ee08ffbf894` |
| `wallet-broker/src/zec/test_support.rs` | 1,220 | `02609ae79bb8d35a5c7fb0058f0ab56fa5deb27c94cfa2ddb81347a2f38ac034` |

Require `wallet-broker/src/zec/prepare.rs` to remain absent. `zec.rs`, `store.rs`,
`test_support.rs`, every test, fixture, manifest, lockfile, policy, document, workflow, package,
and all other repository paths are frozen.

After editing, use only read-only file inspection, `wc -l`, and `sha256sum`. Return both changed
path line counts and hashes, re-prove the three frozen source identities and absent `prepare.rs`,
enumerate the exact replacements, and confirm no ambiguity or broader change remains. The
reviewer will inspect and decide whether Hermes may restart the gate.
