# BBD-WAL-006 Prepare NFC Dependency Integration Review 01

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Integrated dependency commit: `9cdaa562550f4dc898b09411fb92d820fa64501f`

Corrected evidence commit: `7d5e8a4f256b6703aeefb66de7fe8bb01ebe093e`

Result: **ACCEPTED — PREPARE PRODUCTION SOURCE AUTHORIZED**

The final integration is exact, `HEAD == origin/master`, and the tracked worktree/index are
clean. The provider correction now records Hermes Agent v0.18.2 with provider `nous` and model
`meituan/longcat-2.0:free`; it does not alter a command or result.

## Accepted final identities

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/Cargo.toml` | 82 | `dcc4a9d7285aca962f1ea80d69ac3df9f276ffb735680b89c8e9c05ea15ffaf1` |
| `wallet-broker/Cargo.lock` | 5,379 | `9a6166ef2b39b47aa41b7a77cc3054dd8aee481f5a198a1ad4e4882111f97f59` |
| `scripts/security-policy.js` | 2,306 | `1273868a1667aafc723d263bbb564ef3a9940a27d68e119deaee0308425e25dc` |
| `test/securityPolicy.node.js` | 2,525 | `a70d3491c359b8d59a3b7cdb1307ee549b9c0cf8dad310570a47157d066d68ba` |
| `docs/testing/BBD-WAL-006-PREPARE-NFC-DEPENDENCY-GATE-01.md` | 88 | `2635920d0f277182c6bd9ed352ac8a66988bee5bd9deee4019c53dd084e380e6` |

The resolved graph adds only the broker's exact direct dependency on
`unicode-normalization 0.1.25` and its single package block with accepted checksum
`5fd4f6878c9cb28d874b009da9e8d183b5abc80117c40bbd187a1fde336be6e8` and already locked
`tinyvec`. Defaults are disabled and only `std` is enabled. Published metadata records no build
script, Rust version 1.36, and license `MIT OR Apache-2.0`, within the existing allowlist.

## Accepted gate

- Offline Cargo check: exit 0.
- Locked offline feature-tree inspection: exit 0 and the exact defaults-off `std` feature.
- Node security policy: exit 1 with 69 passing and exactly the six already frozen Phase-C
  expected failures; the NFC dependency assertion passes.
- Locked offline custody regression: exit 0 with 11/0 tests.

The complete lock diff changed no prior package, version, checksum, source, feature, or dependency
edge. No network or unlisted execution occurred. This closes the NFC prerequisite from Prepare
Design Review 01 and authorizes only the bounded Rust prepare production source handoff. Frozen
prepare/hygiene tests, fixture bytes, Cargo inputs, dependency policy, Electron/Node source, and
other repositories remain protected.
