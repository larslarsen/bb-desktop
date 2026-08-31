# Codex Sol Handoff — BBD-WAL-006 Dependency Test Correction 01

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This durable file is
the complete correction prompt; ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely before acting: `AGENTS.md`, `TESTING.md`,
`docs/engineering/DEVELOPMENT_ROLES.md`, `tickets/BBD-WAL-006.md`,
`docs/testing/BBD-WAL-006-TEST-SOURCE-REVIEW-01.md`,
`docs/testing/BBD-WAL-006-FORMAT-CORRECTION-REVIEW-01.md`,
`docs/testing/BBD-WAL-006-DEPENDENCY-RESOLUTION-REVIEW-01.md`, and the complete current
`test/securityPolicy.node.js`.

## Frozen state

Require `HEAD == origin/master` at the protected governance parent, a clean index, and
exactly the eight accepted uncommitted Phase-A paths. Before editing, require the Node
test to be 2,340 lines with SHA-256
`ef74c328719a374cbfacbba1f2b0a34e164c27541e58cd1ef0a876acccc348b2`. Every other
accepted path and the unchanged lockfile must match the format-correction review.

The 79-line `wallet-broker/Cargo.toml` with SHA-256
`420559e61a7c81338c97301f7d470a8be6560c0f71aa9f666d79bbcc1424361d`, the 2,231-line
`scripts/security-policy.js` with SHA-256
`affe15ea73706becb6156409afae80ce44076641915457a9e2b9399207ed558f`, production Rust,
all Rust tests, and `Cargo.lock` are frozen.

## Sole task and authorized path

Edit only:

- `test/securityPolicy.node.js`

In the test-side `WAL004_DIRECT_DEPENDENCIES` map, change only:

- `hkdf` from exact `=0.13.0` to exact `=0.12.4`; and
- `sha2` from exact `=0.11.0` to exact `=0.10.9`.

In the existing test named
`WAL-004 Rust test-harness manifest pins the exact toolchain, dependencies, and native features`,
preserve every current assertion and mutation. Add non-vacuous exact-line mutations
that independently prove:

1. corrected `hkdf = "=0.12.4"` rejects superseded stable `"=0.13.0"`;
2. corrected `hkdf = "=0.12.4"` rejects loose `"0.12"`;
3. corrected `sha2 = "=0.10.9"` rejects superseded stable `"=0.11.0"`; and
4. corrected `sha2 = "=0.10.9"` rejects loose `"0.10"`.

Each mutation must replace the complete unique manifest dependency line, assert the
replacement actually changed the bytes before invoking policy, and require rejection by
the existing manifest checker. Do not add a new test, change the current 73-test count,
weaken a regex, change any Zcash expectation, or edit the manifest/production policy.
The accepted outcome is deliberately red against the frozen old production dependency
map.

## Restrictions and report

Use `apply_patch`. Do not run Rust, Cargo, Node, npm, tests, formatters, linters, builds,
scanners, dependency resolution, network, fixtures, Git, GitHub, cleanup, wallets,
nodes, or devices. Do not edit, create, delete, move, stage, commit, or push any unlisted
path. Stop on a contradiction instead of changing production or weakening a test.

After editing, only read-only `wc -l`, `sha256sum`, test-count inspection, and
`git diff --check` over the one authorized path are allowed. Report the exact path, line
count, SHA-256, unchanged 73-test count, and the four new mutation pairs. Luna owns red
execution, evidence, integration, and Git after reviewer acceptance.
