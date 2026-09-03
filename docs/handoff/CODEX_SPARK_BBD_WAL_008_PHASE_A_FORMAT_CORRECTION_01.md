# Codex Spark Handoff — BBD-WAL-008 Phase-A Format Correction 01

Status: AUTHORIZED — PINNED FORMATTER ON ONE PATH ONLY

Source actor: Implementation Dev — Codex Spark, GPT-5.3-Codex-Spark High

Reviewer: Lead Engineer/Reviewer — Codex

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely before acting: `AGENTS.md`, `TESTING.md`,
`docs/handoff/CURRENT_TASK.md`, `tickets/BBD-WAL-008.md`, Phase-A Test-Source Review
03, Phase-B Format Stop Review 01, and the complete test path below.

## Exact path and byte boundary

Start from:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/tests/zec_hardware.rs` | 752 | `5759d612f70a5d21e2b9c7fb192449cf51633e3bff65f2ad7141feaf21812056` |

Freeze the manifest in particular:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/Cargo.toml` | 117 | `7d89a7e98fde65c69392a91e633436ccf93b65a2f8e826e22b5eec27804da530` |

Every other source, test, manifest, lockfile, document, evidence, policy, workflow,
fixture, repository, and path is read-only.

## Sole authorized mutation command

Run exactly once from the repository root:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 rustfmt --edition 2024 wallet-broker/tests/zec_hardware.rs
```

This command is the authorized source edit. It must exit 0 and may change only the
named test path. Do not manually edit before or after it. Do not run `cargo fmt`, a
formatter check, Cargo, compiler, tests, Clippy, builds, product binaries, Node/npm,
package managers, security/policy tools, network, Git, or GitHub. Do not edit
governance/evidence or invoke another actor.

Stop immediately after the command. Report its exact exit, the resulting line count
and SHA-256 identity of the test path, confirmation that the manifest retains its
exact identity, and confirmation that no other action or path was used. Reviewer
inspection and Hermes's independent fresh `cargo fmt --check` remain required.
