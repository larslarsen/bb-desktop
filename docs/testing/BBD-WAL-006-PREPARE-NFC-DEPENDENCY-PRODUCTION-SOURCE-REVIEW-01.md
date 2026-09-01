# BBD-WAL-006 Prepare NFC Dependency Production Source Review 01

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Protected governance parent: `4d215b21`

Result: **ACCEPTED — OFFLINE DEPENDENCY GATE AUTHORIZED**

The source drop modifies exactly two authorized paths:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/Cargo.toml` | 82 | `dcc4a9d7285aca962f1ea80d69ac3df9f276ffb735680b89c8e9c05ea15ffaf1` |
| `scripts/security-policy.js` | 2,306 | `1273868a1667aafc723d263bbb564ef3a9940a27d68e119deaee0308425e25dc` |

The manifest adds one exact declaration after `rusqlite`: `unicode-normalization 0.1.25`, defaults
off, `std` only. The policy defines and exports the exact separate
`WAL006_PREPARE_DEPENDENCIES` object and adds the same line at the matching position in the
whole-manifest ordered inventory.

Every prior dependency object, line, feature, duplicate/displacement check, global authority
rejection, test-target inventory, source policy, and workflow assertion is unchanged. No test,
lockfile, Rust source, fixture, or other path changed. Source inspection finds no weakening or
blocker.

Hermes may perform the one offline lock-resolution command, inspect the complete lock diff before
continuing, then run the exact feature, policy, and custody checks in the active handoff. Rust
prepare production remains frozen.

