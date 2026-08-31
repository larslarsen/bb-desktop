# Codex Sol Handoff — BBD-WAL-006 Phase-C Policy-Test Transition

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This durable file is
the complete source prompt; ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Test source baseline: `accac4407041f14079211a1e9eeb7047d862922a`

Protected governance parent: the commit containing this handoff. Its changes after the
test source baseline are reviewer-authored acceptance, architecture, and routing records
only; they change no test, fixture, manifest, lockfile, policy implementation, or
production byte.

Read completely before editing: `AGENTS.md`, `TESTING.md`,
`docs/engineering/DEVELOPMENT_ROLES.md`, `tickets/BBD-WAL-006.md`,
`docs/architecture/BBD-WAL-006-UPSTREAM-REVIEW.md`,
`docs/testing/BBD-WAL-006-EXPECTED-RED-01.md`,
`docs/testing/BBD-WAL-006-FIXTURE-EXPECTED-RED-INTEGRATION-REVIEW-01.md`,
`docs/handoff/CURRENT_TASK.md`, and the complete current
`test/securityPolicy.node.js` WAL-006 section.

## Sole task

Edit exactly `test/securityPolicy.node.js` to transition the WAL-006 Rust production
inventory assertion from completed test-only Phase A to the bounded Phase-C
architecture. Change `WAL006_ALLOWED_RUST_SOURCE_PATHS` from the empty array to this
exact lexically ordered array:

```text
wallet-broker/src/zec.rs
wallet-broker/src/zec/address.rs
wallet-broker/src/zec/fixture.rs
wallet-broker/src/zec/prepare.rs
wallet-broker/src/zec/scan.rs
wallet-broker/src/zec/store.rs
wallet-broker/src/zec/test_support.rs
```

Rename the related test so it states that the exact bounded Phase-C ZEC production
inventory is required. Change its repository assertion from `actual == []` to
`actual == WAL006_ALLOWED_RUST_SOURCE_PATHS`. Preserve the recursive source collection,
the policy-export equality check, and the call to
`checkWal006RustSourceInventory(actual)`.

Strengthen the unlisted-path mutation table to cover at least:

- `wallet-broker/src/zec_network.rs`
- `wallet-broker/src/zec/network.rs`
- `wallet-broker/src/zec/raw.rs`
- `wallet-broker/src/zec/sign.rs`

For each mutation, pass the complete accepted list plus the unlisted path to the policy
checker so the test proves rejection of an extra path, not merely rejection of an
incomplete list. Preserve the existing live-network and authority-bearing source
mutations. Do not change a manifest dependency, feature, test target, fixture, Rust
test, expected value, or any other accepted assertion.

## Exact authorization

You may edit only:

- `test/securityPolicy.node.js`

Do not edit `scripts/security-policy.js`, Rust source, Rust tests, fixtures,
`wallet-broker/Cargo.toml`, `Cargo.lock`, documentation, tickets, handoffs, evidence,
workflows, package files, or any other path. Do not create a production stub, directory,
or placeholder.

Use `apply_patch`. Do not run Node, npm, Cargo, Rust, tests, formatters, linters, builds,
policy checkers, scanners, Electron, wallets, nodes, devices, network, Git, or GitHub.
Do not install, delete, clean, move, stage, commit, or push anything.

Report the exact changed path, line count, SHA-256, a concise semantic diff, and any
contradiction. Luna—not Sol—will inspect the drop, capture expected red, author evidence,
and own Git operations. Production remains frozen.
