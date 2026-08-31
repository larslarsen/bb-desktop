# Codex Sol Handoff — BBD-WAL-006 Support-Dependency Tests 01

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This durable file is
the complete test-source prompt; ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Test source baseline: `c660549724ba5dcd30cb9f3b68909d1383d96b48`

Protected governance parent: the commit containing this handoff. Its changes after the
test source baseline are reviewer-authored governance only. The attempted address-source
handoff changed no file.

Read completely: `AGENTS.md`, `TESTING.md`,
`docs/engineering/DEVELOPMENT_ROLES.md`, `tickets/BBD-WAL-006.md`,
`docs/testing/BBD-WAL-006-ADDRESS-PRODUCTION-SOURCE-REVIEW-01.md`, the accepted Phase-C
policy test/red records, `docs/handoff/CURRENT_TASK.md`, `wallet-broker/Cargo.toml`, and
the complete WAL-004/WAL-006 manifest-policy tests in `test/securityPolicy.node.js`.

## Sole task

Edit only `test/securityPolicy.node.js` to add a new independently named test group for
the exact two production support API dependencies required by the address source:

```text
rand_core = { version = "=0.6.4", default-features = false, features = ["std"] }
rusqlite = { version = "=0.37.0", default-features = false }
```

Define and assert an exact exported policy object named
`WAL006_SUPPORT_DEPENDENCIES`, with the same object shape used by
`WAL006_DIRECT_DEPENDENCIES`:

```text
rand_core: version =0.6.4, default_features false, features [std], optional false
rusqlite:  version =0.37.0, default_features false, features [],    optional false
```

The new test must read the real manifest and independently require each exact one-line
declaration before checking the policy export. This ordering gives an unambiguous red
while the manifest is still unchanged. Then require the policy export to deep-equal the
frozen object and call `checkWalletBrokerManifest` on the real manifest.

Add non-tautological manifest mutations proving rejection of at least:

- a loose `rand_core` version;
- rand_core default features enabled or its exact `std` feature removed/widened;
- a loose `rusqlite` version;
- rusqlite default features enabled; and
- direct rusqlite `load_extension`, `bundled-sqlcipher-vendored-openssl`, or any other
  feature beyond the empty direct feature set.

Mutation failures must match a support/dependency/manifest/RNG/SQLite policy error. Keep
the maintained-upstream Zcash object at exactly six crates; these two packages are
support API pins, not a rewrite of the reviewed Zcash compatibility set. Preserve every
existing expected path, dependency, feature, authority, workflow, and source assertion.

## Exact authorization

You may edit only:

- `test/securityPolicy.node.js`

Use `apply_patch`. Read-only `sed`/`rg` inspection is permitted. Do not edit the manifest,
lockfile, policy implementation, Rust source/test, fixture, documentation, evidence,
handoff, ticket, workflow, package file, or any other path. Do not create a stub.

Do not run Node, npm, Cargo, Rust, tests, formatters, linters, builds, policy checkers,
scanners, Electron, wallets, nodes, devices, Git, network, install, cleanup, commit, or
push. Report the exact changed path, line count, SHA-256, semantic diff, and any
contradiction. Luna owns expected-red execution/evidence/integration. Address production
remains frozen.
