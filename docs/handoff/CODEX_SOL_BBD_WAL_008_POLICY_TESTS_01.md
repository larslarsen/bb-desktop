# Codex Sol Handoff — BBD-WAL-008 Policy Tests 01

You are **Principal Dev — Codex Sol** using `gpt-5.6-sol` at High. Repository:
`/home/lars/OpenBazaar/bb-desktop`.

Protected governance parent: the commit containing this handoff

Grok Build is the default senior source actor, but the owner reports that its weekly
usage is exhausted. That is the documented fill-in condition for this bounded Sol
assignment.

Read completely: `AGENTS.md`, `TESTING.md`, both role/routing policies,
`docs/handoff/CURRENT_TASK.md`, `tickets/BBD-WAL-008.md`, Resume-03 Stop Review 01,
`test/securityPolicy.node.js`, `scripts/security-policy.js`, and
`wallet-broker/Cargo.toml`.

## Sole task

Edit only `test/securityPolicy.node.js`. Author the test contract for the omitted
BBD-WAL-008 policy transition. Do not edit production policy yet.

Preserve the historical `WAL006_TEST_TARGETS` six-entry value and
`WAL006_ALLOWED_RUST_SOURCE_PATHS` seven-entry value exactly. Add independently named
BBD-WAL-008 expectations for:

- exactly one new integration target, `zec_hardware`, at
  `tests/zec_hardware.rs` in the existing manifest order;
- the exact sorted eight-path current ZEC production inventory: the historical seven
  WAL-006 paths plus `wallet-broker/src/zec/hardware.rs` in lexical order;
- a production export named `WAL008_TEST_TARGETS`;
- a production export named `WAL008_ZEC_RUST_SOURCE_PATHS`; and
- a production function named `checkWal008RustSourceInventory`.

Add one clearly named BBD-WAL-008 test group that:

1. proves the current manifest contains exactly the new target block and that removing,
   renaming, or duplicating it is rejected by `checkWalletBrokerManifest`;
2. recursively collects the current ZEC Rust tree, sorts it, and requires exactly the
   eight reviewed paths;
3. requires the two new exports and checker above;
4. requires the new checker to accept the exact eight paths and reject missing
   `hardware.rs`, an unlisted extra path, a duplicate, a malformed entry, and wrong
   order; and
5. directly sends the current nonempty `hardware.rs` source through
   `checkRustWalletSource`, then proves representative transport, signing, broadcast,
   and mainnet mutations are rejected.

Adjust the existing WAL-006 inventory test only enough to keep proving its historical
seven-path contract without falsely claiming that it is the complete post-WAL-008
repository inventory. It must still prove the seven paths are present and that the
WAL-006 checker accepts only that exact ordered historical set and rejects its existing
four unlisted additions. The new WAL-008 group alone owns the complete current ZEC
inventory.

The intended future expected red is the whole policy test exiting 1 only because
`scripts/security-policy.js` lacks the new WAL-008 exports/checker, omits
`zec_hardware` from its closed manifest target list, and still applies the WAL-006
inventory to the complete current ZEC tree. Existing successful groups must not regress.

## Frozen boundaries

At the protected parent:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/Cargo.toml` | 117 | `7d89a7e98fde65c69392a91e633436ccf93b65a2f8e826e22b5eec27804da530` |
| `wallet-broker/Cargo.lock` | 5,394 | `29a5e4a8b9fa40ca612f3b7c3b1f90d527544a519c06e50bc11aae7803f57420` |
| `test/securityPolicy.node.js` | 3,189 | `dd22ab83645d5d5ffb9d695cd21f15175ffe109b6020142e1bff5ca8d60e0497` |
| `scripts/security-policy.js` | 2,689 | `6aba356098134e90731928a2a1083565d52a6d000c213b89549ba231997f5626` |

The three accepted uncommitted Slice-02 source paths must remain byte-identical to the
hashes in Resume-03 Stop Review 01. Do not touch them.

Do not run a formatter, Node, Rust, test, build, lint, policy checker, product, network,
or Git command. Do not edit documentation, evidence, manifests, lockfiles, workflows,
production policy, Rust, Monero, or any other path. Stop with the exact changed path,
line count, SHA-256, and a concise semantic summary for reviewer inspection.
