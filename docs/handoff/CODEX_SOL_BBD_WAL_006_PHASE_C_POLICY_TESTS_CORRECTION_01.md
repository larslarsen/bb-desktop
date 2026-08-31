# Codex Sol Handoff — BBD-WAL-006 Phase-C Policy-Test Correction 01

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This durable file is
the complete correction prompt; ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff.

Accepted unchanged test drop before correction: `test/securityPolicy.node.js`, 2,399
lines, SHA-256
`add59ef67d0757cbb5b5397ea1f33ea9443ab8b2662b22cc8144e32a4174af9f`.

Read completely: `AGENTS.md`, `TESTING.md`,
`docs/testing/BBD-WAL-006-PHASE-C-POLICY-TEST-SOURCE-REVIEW-01.md`, the original Phase-C
policy-test handoff, and the complete changed WAL-006 test section.

## Sole correction

Edit only `test/securityPolicy.node.js`. Change the `actual` ZEC source inventory
construction so the paths remaining after this exact filter are lexically sorted before
the equality assertion and before `checkWal006RustSourceInventory(actual)`:

```text
/^wallet-broker\/src\/zec(?:[_.\/])/
```

Use the existing array `sort()` operation; do not change collection recursion, the
filter, the expected seven-path array, test name, equality assertion, policy call,
unlisted mutations, or any other byte except the minimum formatter-consistent layout
needed for the added sort.

Use `apply_patch`. Do not edit any other path. Do not run Node, npm, Cargo, Rust, tests,
formatters, linters, builds, policy checkers, scanners, Git, network, wallets, nodes,
devices, or cleanup. Report the corrected line count and SHA-256. Production remains
frozen, and Luna owns later expected-red execution and Git integration.
