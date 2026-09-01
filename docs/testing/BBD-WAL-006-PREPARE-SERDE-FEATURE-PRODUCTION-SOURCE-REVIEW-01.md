# BBD-WAL-006 Prepare Serde Feature Production Source Review 01

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Governance parent: `c35a813c`

Result: **STATICALLY ACCEPTED — OFFLINE LOCK CAPTURE AUTHORIZED**

Principal Dev — Codex Sol changed exactly the three authorized paths with no execution or Git.
The manifest adds only the pinned SQLite crate's `serde` feature; the policy constant and exact
manifest literal match the committed test; parsed PCZT real inputs use the public no-dummy-key
marker; and signature absence now inspects every Ironwood action. The official read-only wallet and
PCZT builder remain unchanged. `git diff --check` is clean.

## Accepted correction identities

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/Cargo.toml` | 82 | `47667b8f1970856096c2451ef70ff562be02ca360facf94b043c71ac30072735` |
| `scripts/security-policy.js` | 2,306 | `2c3d859ddd246b38972c835e604c70bd2bbff7b9266629a9c5c39c1b4d967cea` |
| `wallet-broker/src/zec/store.rs` | 2,048 | `f9f66f98f33b8457c955125b77453be018397ab120f78618d52ed817200fcf34` |

Other accepted production identities remain:

- `wallet-broker/src/zec/prepare.rs`: 963 lines,
  `417178e0458a3a13e4f36331b8e17bb92148836631eefbdf1a0786501cd114e3`;
- `wallet-broker/src/zec.rs`: 252 lines,
  `1061adff987aefd8a641dfed11e06c85d0bc56ddb39f17a5c95d495d6aea387b`;
- `wallet-broker/src/zec/test_support.rs`: 1,830 lines,
  `5a1c30199874ad13eb753deaf83c3080534ba495fdfaf225a3d8d03c6ef1ac77`.

The current lock remains 5,379 lines,
`9a6166ef2b39b47aa41b7a77cc3054dd8aee481f5a198a1ad4e4882111f97f59`. The pinned crate metadata
shows the feature should add only `serde` to `zcash_client_sqlite`'s lock dependency edges and
`serde_core` to `uuid`'s edges. Both packages are already pinned in the graph. Hermes must capture
that exact offline delta before any formatter/compiler/test resumes.
