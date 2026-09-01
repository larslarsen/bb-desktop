# Codex Sol Handoff — BBD-WAL-006 Prepare Production Resume 01

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This durable resume handoff is
authoritative together with Prepare Production Handoff 01.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Implementation baseline: `2bfeb3bcffebaf99b7cf3fae727f7151fbe5accb`

Read completely before acting: `AGENTS.md`, `TESTING.md`, `tickets/BBD-WAL-006.md`, Prepare
Production Handoff 01, Prepare Production Source Stop Review 01, Prepare Secret-Bytes Design and
Integration Reviews 01, both frozen prepare/hygiene tests, the complete current ZEC and vault
source, pinned upstream source named by the original handoff, fixture manifest, and
`docs/handoff/CURRENT_TASK.md`.

## Resolved blocker and exact scope

The prior source attempt stopped with no edits. The accepted custody correction now makes ordinary
`SecretBytes` automatically `Send + Sync` while preserving its zeroizing lifecycle. Therefore all
semantics, construction APIs, validation, PCZT inspection, state ownership, invalidation, negative
capability, and delivery constraints in
`docs/handoff/CODEX_SOL_BBD_WAL_006_PREPARE_PRODUCTION_01.md` are reauthorized unchanged.

You may create/edit only:

- `wallet-broker/src/zec/prepare.rs` (currently absent);
- `wallet-broker/src/zec.rs`;
- `wallet-broker/src/zec/store.rs`; and
- `wallet-broker/src/zec/test_support.rs`.

Starting identities remain:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/zec.rs` | 218 | `ae06d6a9b4448603860b55bf4eef42a1e970c414049a9304f42b436d2bf49cb4` |
| `wallet-broker/src/zec/store.rs` | 1,816 | `d0b97b802209489804e5d550ef90138a87342e155c519a686b2d900189f6603f` |
| `wallet-broker/src/zec/test_support.rs` | 1,220 | `02609ae79bb8d35a5c7fb0058f0ab56fa5deb27c94cfa2ddb81347a2f38ac034` |
| `wallet-broker/tests/zec_prepare.rs` | 412 | `a616245fdcf8d2226277e34d785bb1792d3b8b54ebf268c3d1dc5f15566da942` |
| `wallet-broker/tests/zec_hygiene.rs` | 375 | `aad7c95a2ef661063661f2ec0f16a216d80328096b049d636b88fa0252ba1be6` |

Frozen corrected custody identities:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/vault.rs` | 773 | `500cd2f91ec0a2e0052779ba6b2357053ce0bea1d644fb2c35066f768f363fe0` |
| `wallet-broker/tests/secret_hygiene.rs` | 281 | `dcebe361c7061b06b9ec6bb6fbea88ccb208069f7360ae14e2d7c6ca83b433a4` |

Own derived material and prepared PCZT artifacts directly as `SecretBytes` inside the narrow
synchronized prepare state beside `TestAccount`. Keep the concrete thread-safe
`RecordingWipes` observer separately and pass it only to explicit `wipe_with` calls. Do not use
`ObservedSecretBytes` for persistent prepare state, and do not add a thread, channel, global,
thread-local, unsafe/manual auto-trait, raw secret vector, or alternate lifecycle. The accepted
`Arc<TestAccount>` receiver concurrency must continue to compile from automatic field auto-traits.

Do not edit `vault.rs`, any test, Cargo/lock, fixture, other ZEC source, policy, docs, or another
path. If another contradiction appears, stop before weakening either the original handoff or this
resume.

## Delivery boundary

Use `apply_patch` only. Read-only local inspection is permitted. Do not run a formatter, Cargo,
Rust, Clippy, tests, Node, policy, dependency, Git, network, fixture-generation, wallet/node/device,
cleanup, or deletion. Do not stage, commit, or push.

Return all changed paths with line counts/SHA-256 and every design/API fact required by the
original handoff. Hermes remains the sole future execution, evidence, integration, and Git actor.
