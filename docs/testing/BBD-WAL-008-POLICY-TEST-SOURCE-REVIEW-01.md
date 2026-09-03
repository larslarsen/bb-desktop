# BBD-WAL-008 Policy Test Source Review 01

Reviewer: Lead Engineer/Reviewer — Codex at High

Protected governance parent: `1281c0f7`

Result: **ACCEPTED — FOCUSED POLICY EXPECTED RED AUTHORIZED**

Codex Sol High changed only `test/securityPolicy.node.js`, now 3,358 lines at SHA-256
`464a576803678a12165609a51df14a43630dbf52925ecb6cb22842e6fa226e07`.
The three previously accepted Slice-02 Rust paths remain byte-identical.

The 175-addition/six-deletion test delta (169-line net growth) makes the intended
ticket-separated transition:

- historical `WAL006_TEST_TARGETS` remains exactly six entries and
  `WAL006_ALLOWED_RUST_SOURCE_PATHS` remains exactly seven entries;
- `WAL008_TEST_TARGETS` requires only `zec_hardware` and its exact manifest path/order;
- `WAL008_ZEC_RUST_SOURCE_PATHS` requires the sorted current eight-path ZEC tree;
- missing, renamed, and duplicated manifest-target mutations are exercised;
- missing hardware, unlisted extra, duplicate, malformed, and wrong-order inventory
  mutations are exercised through the required new production checker; and
- the real nonempty hardware source plus transport, signing, broadcast, and mainnet
  mutations are sent through the source-policy boundary.

The historical WAL-006 inventory test still proves all seven paths are present, the
old checker accepts only that exact ordered set, and its four extra-path negatives are
retained. The new WAL-008 test alone owns the complete post-WAL-008 inventory, avoiding
a misleading rewrite of the earlier ticket's scope.

No production-policy, manifest, lockfile, Rust, workflow, documentation, or other
developer path changed. The reviewer inspected source only and ran no formatter, Node,
Rust, test, build, lint, policy checker, or product command. Hermes alone may now run
the exact focused Node policy file and integrate the test-only drop only on the frozen
80/7 intended red.
