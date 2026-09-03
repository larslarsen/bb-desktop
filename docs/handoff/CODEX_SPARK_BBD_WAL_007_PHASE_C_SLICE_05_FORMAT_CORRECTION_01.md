# Codex Spark Handoff — BBD-WAL-007 Slice 5 Format Correction 01

Status: AUTHORIZED — PINNED FORMATTER ON SEVEN PATHS ONLY

Source actor: Implementation Dev — Codex Spark, GPT-5.3-Codex-Spark High

Reviewer: Lead Engineer/Reviewer — Codex

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely before acting: `AGENTS.md`, `TESTING.md`, Slice-5 Correction-04 Source
Review 01, Slice-5 Green Stop Review 01, the seven complete source paths below, and
`docs/handoff/CURRENT_TASK.md`.

## Exact path and byte boundary

Start from:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/xmr.rs` | 8 | `dbcb6133b19f92bc0b0d99aa6ec82d7a55400f553b85c258d583a6584726c7ff` |
| `wallet-broker/src/xmr/account.rs` | 3,374 | `8ab5650246afc1a657a91b7b013aa1c79995ee60ce4d78e0a34404db0adb05f6` |
| `wallet-broker/src/xmr/process.rs` | 1,968 | `ad9d77bbc73cc2e19075fb0b488ddc9961f8dfac521f80f06f431aa08843cd42` |
| `wallet-broker/src/xmr/receiver.rs` | 870 | `fb1ab7ff4210a09612de450b2ed5650f215b2d2a8ca20c868bc16b9e025ca23e` |
| `wallet-broker/src/xmr/rpc.rs` | 2,582 | `302a0d79869df8310973de86784ac138bb49400c174d71c2f15eee3dfd311c55` |
| `wallet-broker/src/xmr/store.rs` | 1,916 | `b3e66a34571a1801431956f526fef33b923eef645c13c099904dedbad922b018` |
| `wallet-broker/src/xmr/test_support.rs` | 6,027 | `c83fa81b0bfbec811e1b1a9c254c2f786df3b5ed3739f1be9bd7e2ac42ee62e8` |

Freeze these existing paths in particular:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/xmr/model.rs` | 214 | `2a2d3ba1ce453aca65138df402bde3e7f1fee997d5d069024cb1beb8102152cb` |
| `wallet-broker/tests/xmr_receiver.rs` | 588 | `d8656c533d7b2d08eb9fa8831cd70da6e1da632ed9563fc2af569cab754d3622` |
| `docs/testing/BBD-WAL-007-SLICE-05-GREEN-01.md` | 59 | `20f07f0b44dae0f006e192d91b717b541487a857d4d920047bbfd68818705637` |

The last path is Hermes's untracked stop draft. Do not edit, delete, stage, or interpret
it as accepted evidence. Every other source, test, manifest, lockfile, document,
evidence, policy, workflow, fixture, repository, and path is read-only.

## Sole authorized mutation command

Run exactly once from the repository root:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 rustfmt --edition 2024 wallet-broker/src/xmr.rs wallet-broker/src/xmr/account.rs wallet-broker/src/xmr/process.rs wallet-broker/src/xmr/receiver.rs wallet-broker/src/xmr/rpc.rs wallet-broker/src/xmr/store.rs wallet-broker/src/xmr/test_support.rs
```

This command is the authorized source edit. It must exit 0 and may change only the
seven named paths. Do not manually edit before or after it. Do not run `cargo fmt`, a
formatter check, Cargo, compiler, tests, Clippy, builds, product/Monero binaries,
Node/npm, package managers, security/policy tools, network, Git, or GitHub. Do not edit
governance/evidence or invoke another actor.

Stop immediately after the command. Report its exact exit, resulting line counts and
SHA-256 identities for all seven paths, confirmation that `model.rs`, the frozen
receiver test, and the untracked stop draft retain their exact identities, and
confirmation that no other action or path was used. Reviewer inspection and Hermes's
independent fresh `cargo fmt --check` remain required.
