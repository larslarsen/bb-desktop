# Codex Sol Handoff — BBD-WAL-008 Policy Production 01

You are **Principal Dev — Codex Sol** using `gpt-5.6-sol` at High. Repository:
`/home/lars/OpenBazaar/bb-desktop`.

Protected governance parent: the commit containing this handoff

Grok Build remains the default senior source actor, but the owner reports that its
weekly usage is exhausted. That is the documented fill-in condition for this bounded
Sol assignment.

Read completely: `AGENTS.md`, `TESTING.md`, both role/routing policies,
`docs/handoff/CURRENT_TASK.md`, `tickets/BBD-WAL-008.md`, Policy Test Source Review 01,
Policy Expected-Red Acceptance 01, the integrated expected-red evidence,
`test/securityPolicy.node.js`, `scripts/security-policy.js`, and
`wallet-broker/Cargo.toml`.

## Sole task

Edit only `scripts/security-policy.js` to satisfy the accepted WAL-008 policy contract.
Make the smallest direct implementation:

1. Preserve `WAL006_TEST_TARGETS` at exactly six entries,
   `WAL006_ALLOWED_RUST_SOURCE_PATHS` at exactly seven entries, and
   `checkWal006RustSourceInventory` as the historical exact checker.
2. Add exported `WAL008_TEST_TARGETS` with exactly `zec_hardware`.
3. Add exported `WAL008_ZEC_RUST_SOURCE_PATHS` with the exact lexically sorted current
   eight-path ZEC inventory: the historical seven paths plus
   `wallet-broker/src/zec/hardware.rs`.
4. Extend `checkWalletBrokerManifest`'s exact integration-test target sequence with
   only `zec_hardware:tests/zec_hardware.rs`, immediately after `zec_hygiene` and before
   `xmr_distribution`. It must reject removal, rename, duplication, displacement, or
   any other target change.
5. Add exported `checkWal008RustSourceInventory`. It accepts only the exact ordered
   eight-entry array and separately rejects a non-array, malformed/empty entry,
   duplicate, wrong order, missing hardware, or any extra/unlisted path with stable
   WAL-008/Zcash/inventory-oriented errors matching the accepted tests.
6. In `checkRepository`, apply the new WAL-008 inventory checker to the recursively
   collected current ZEC tree and iterate the new eight-path list for source scanning.
   The new `hardware.rs` must therefore traverse the existing
   `checkRustWalletSource` denials. Do not weaken, bypass, duplicate, or special-case
   those denials.
7. Export only the three new public contract items named above. Preserve all existing
   WAL-004, WAL-006, WAL-007, workflow, dependency, source, audit, and scanner policy.

Do not change `test/securityPolicy.node.js` or tailor errors by reading stack-line
numbers. Do not add skips, fallbacks, compatibility acceptance, unordered-set
acceptance, environment behavior, or runtime authority.

## Frozen boundaries

At the protected parent:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `scripts/security-policy.js` | 2,689 | `6aba356098134e90731928a2a1083565d52a6d000c213b89549ba231997f5626` |
| `test/securityPolicy.node.js` | 3,358 | `464a576803678a12165609a51df14a43630dbf52925ecb6cb22842e6fa226e07` |
| `wallet-broker/Cargo.toml` | 117 | `7d89a7e98fde65c69392a91e633436ccf93b65a2f8e826e22b5eec27804da530` |
| `wallet-broker/Cargo.lock` | 5,394 | `29a5e4a8b9fa40ca612f3b7c3b1f90d527544a519c06e50bc11aae7803f57420` |

The three accepted uncommitted Slice-02 Rust paths must remain byte-identical to Policy
Expected-Red Acceptance 01. Do not touch them.

Do not run a formatter, Node, Rust, test, build, lint, policy checker, product, network,
or Git command. Do not edit tests, documentation, evidence, manifests, lockfiles,
workflows, Rust, Monero, or any other path. Stop with the exact changed path, line
count, SHA-256, and a concise semantic report for reviewer inspection.
