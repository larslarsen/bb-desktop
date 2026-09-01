# Codex Sol Handoff — BBD-WAL-006 Prepare NFC Dependency Tests 01

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This durable file is
authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Test source baseline: `ec8cec39`

Protected governance parent: the commit containing this handoff

Read completely before acting: `AGENTS.md`, `TESTING.md`, `tickets/BBD-WAL-006.md`,
`docs/architecture/BBD-WAL-006-UPSTREAM-REVIEW.md`, Prepare Design Review 01,
`docs/handoff/CURRENT_TASK.md`, `wallet-broker/Cargo.toml`, and the complete WAL-004/WAL-006
manifest-policy tests in `test/securityPolicy.node.js`.

## Sole task

Edit only `test/securityPolicy.node.js` to add an independently named test group for the one exact
Unicode-normalization dependency required by the already frozen WAL-002 memo contract:

```text
unicode-normalization = { version = "=0.1.25", default-features = false, features = ["std"] }
```

Define and assert an exact future exported policy object named
`WAL006_PREPARE_DEPENDENCIES`, using the existing dependency-object shape:

```text
unicode-normalization: version =0.1.25, default_features false, features [std], optional false
```

The test must:

- read the real manifest and independently require the exact declaration exactly once before
  consulting the policy export, so the current missing-manifest state has an unambiguous red;
- assert the future policy export deep-equals the frozen object;
- pass the real manifest to `checkWalletBrokerManifest`; and
- prove rejection of a loose version, enabled default features, removed `std`, widened feature
  sets, optional dependency, git/path source, or a second Unicode-normalization implementation.

Mutation failures must match a stable prepare/Unicode/normalization/dependency/manifest policy
error. Preserve the six-crate maintained Zcash object and the two existing WAL-006 support APIs
unchanged. This package is a prepare-validation dependency, not part of either existing set.
Preserve every existing dependency, test-target, feature, source-authority, workflow, and license
assertion. Do not weaken the exact whole-manifest inventory.

## Exact authorization

You may edit only:

- `test/securityPolicy.node.js`

Use `apply_patch`. Read-only `sed`/`rg` inspection is permitted. Do not edit the manifest,
lockfile, policy implementation, Rust source/test, fixture, documentation, evidence, handoff,
ticket, workflow, package file, or any other path. Do not create a stub.

Do not run Node, npm, Cargo, Rust, tests, formatters, linters, builds, policy checkers, scanners,
Electron, wallets, nodes, devices, Git, network, install, cleanup, commit, or push. Report the exact
changed path, line count, SHA-256, semantic diff, and any contradiction. Hermes owns expected-red
execution, evidence, integration, and Git. Prepare production remains frozen.

