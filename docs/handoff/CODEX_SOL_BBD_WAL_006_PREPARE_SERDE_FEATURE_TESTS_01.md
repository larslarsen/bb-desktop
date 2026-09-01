# Codex Sol Handoff — BBD-WAL-006 Prepare Serde Feature Tests 01

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. Author only the bounded policy
test-source correction.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely: `AGENTS.md`, `TESTING.md`, Prepare Gate Clippy Review 01, the complete
`test/securityPolicy.node.js`, the current manifest-policy implementation, and pinned
`zcash_client_sqlite 0.22.0` feature/`AccountUuid` definitions.

## Exact scope

Edit only `test/securityPolicy.node.js` with `apply_patch`.

Update the exact WAL-006 `zcash_client_sqlite` feature expectation from
`['orchard', 'test-dependencies', 'transparent-inputs']` to the lexically ordered
`['orchard', 'serde', 'test-dependencies', 'transparent-inputs']`. Update every exact manifest
mutation/source string in the same test so it expects precisely:

```text
features = ["orchard", "serde", "test-dependencies", "transparent-inputs"]
```

The forbidden-feature mutation must add only `zewif` after that new exact union, and the git-source
mutation must preserve the new exact union. Do not change a test name, count, assertion order,
failure matcher, forbidden feature/authority set, compiled-capability set, source inventory, or
other dependency expectation.

Do not edit the manifest, lockfile, policy implementation, Rust source/test, fixture, ticket,
workflow, documentation, or another path. Do not run Node, Cargo/Rust, formatter, Clippy, test,
policy, dependency, Git, network, wallet/node/device, cleanup, or deletion commands. Do not stage,
commit, or push.

Return the resulting line count/SHA-256, exact changed locations, and confirm the test count/names
remain unchanged. Hermes remains the sole expected-red execution/evidence/integration/Git actor.
