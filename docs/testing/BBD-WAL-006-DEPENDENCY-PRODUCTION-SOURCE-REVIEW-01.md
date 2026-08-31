# BBD-WAL-006 Dependency Production Source Review 01

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Protected governance parent: `0765f348`

Result: **BOUNDED MANIFEST/POLICY CORRECTION ACCEPTED FOR RESOLUTION GATE**

Sol changed exactly two authorized paths and six version literals:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/Cargo.toml` | 79 | `da7639af39bbc578936f0d38fcc32317fbacb279e53040cad4b3543c67a54294` |
| `scripts/security-policy.js` | 2,231 | `627eaf04248ab744e8c4300f3cae3a34d114a69bad88dfa4de04492dea537d4d` |

The manifest now pins exact, defaults-off `hkdf =0.12.4` and `sha2 =0.10.9`. Production
policy matches those versions in its exported WAL-004 direct-dependency map and exact
manifest required-line list. No feature/default/optional property, ordering, package
field, test target, Zcash declaration, checker control flow, error pattern, export, or
other byte changed.

Reviewer inspection confirms `git diff --check` passes. The committed Node test remains
2,374 lines with SHA-256
`f4f6defd5d55212c480a7a1a87d37edea85786b6125cad29467ae5e1f3480ee4`; the lockfile
remains 3,273 lines with SHA-256
`1d182550f82b9dc443804d82e709b5d58021d4243bef1612c440aac59df1c2f5`; and all six
uncommitted Rust tests retain their format-correction hashes.

Sol ran no Node, Rust, Cargo, npm, formatter, linter, build, scanner, resolution, network,
fixture, Git, cleanup, wallet, node, or device command. Luna may now resolve and inventory
the graph, prove the corrected production map advances to the already accepted WAL-006
manifest red, and run the complete existing 11-test `vault_crypto` target. Any custody
vector/envelope change, source mutation, wrong graph, non-crates.io source, or unexpected
failure rejects the correction. Fixture generation remains unauthorized until reviewer
acceptance of that gate.
