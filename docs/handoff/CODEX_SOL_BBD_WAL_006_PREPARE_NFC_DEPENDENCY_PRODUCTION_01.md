# Codex Sol Handoff — BBD-WAL-006 Prepare NFC Dependency Production 01

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This durable file is
authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Implementation source baseline: `29f3bc9abacfa2d32b72f3b9a7104115a9c38451`

Protected governance parent: the commit containing this handoff

Read completely before acting: `AGENTS.md`, `TESTING.md`, `tickets/BBD-WAL-006.md`, Prepare Design
Review 01, the accepted NFC dependency test source and expected-red review/evidence,
`docs/handoff/CURRENT_TASK.md`, `wallet-broker/Cargo.toml`, complete
`scripts/security-policy.js`, and the new policy test in `test/securityPolicy.node.js`.

## Sole task and exact paths

Implement only the tested Unicode-normalization dependency declaration and narrow manifest-policy
correction. You may edit exactly:

- `wallet-broker/Cargo.toml`
- `scripts/security-policy.js`

Append this exact line immediately after the existing `rusqlite` dependency without moving or
changing any accepted declaration:

```text
unicode-normalization = { version = "=0.1.25", default-features = false, features = ["std"] }
```

In the policy implementation:

- define `WAL006_PREPARE_DEPENDENCIES` exactly as the committed test object;
- export that object without renaming or merging it into either existing WAL-006 dependency set;
- append the exact manifest line to `checkWalletBrokerManifest`'s ordered dependency inventory
  immediately after `rusqlite`; and
- preserve the whole-manifest duplicate/displaced-declaration check and the existing global
  git/path/unreviewed-authority rejection so every committed mutation fails closed.

Do not change `WAL006_DIRECT_DEPENDENCIES`, `WAL006_SUPPORT_DEPENDENCIES`, any existing dependency
line or feature, test-target inventory, source inventory, authority regex, license allowlist,
workflow policy, or another checker. Do not add another normalization crate, alias, build script,
patch, git/path source, default feature, optional mode, or version range.

## Boundaries

Use `apply_patch`. Read-only `sed`/`rg` inspection is permitted. Do not edit `Cargo.lock`, test
source, Rust source/test, fixtures, documentation, evidence, handoffs, ticket, workflow, package
file, deny policy, or any other path.

Do not run Node, npm, Cargo, Rust, tests, formatters, linters, builds, policy checkers, scanners,
Electron, wallets, nodes, devices, Git, network, dependency resolution, install, cleanup, commit,
or push. Report both changed paths with line counts/SHA-256, exact semantic changes, and any
contradiction. Hermes will resolve the already cached published crate into the lock, run the
focused dependency gate, author evidence, and own Git. Rust prepare production remains frozen.

