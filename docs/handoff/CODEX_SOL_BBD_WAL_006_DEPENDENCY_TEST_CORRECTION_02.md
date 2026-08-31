# Codex Sol Handoff — BBD-WAL-006 Dependency Test Correction 02

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This durable file is
the complete correction prompt; ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely before acting: `AGENTS.md`, `TESTING.md`,
`docs/engineering/DEVELOPMENT_ROLES.md`, `tickets/BBD-WAL-006.md`, both dependency
resolution reviews, every first-correction test/source/red review and evidence file, and
the complete current `test/securityPolicy.node.js`.

## Frozen state

Require `HEAD == origin/master` at the protected governance parent, a clean index, and
exactly eight uncommitted paths: the accepted manifest, accepted production policy, and
six accepted formatted Rust tests. Require `git diff --check` and these hashes:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `test/securityPolicy.node.js` | 2,374 | `f4f6defd5d55212c480a7a1a87d37edea85786b6125cad29467ae5e1f3480ee4` |
| `wallet-broker/Cargo.toml` | 79 | `da7639af39bbc578936f0d38fcc32317fbacb279e53040cad4b3543c67a54294` |
| `scripts/security-policy.js` | 2,231 | `627eaf04248ab744e8c4300f3cae3a34d114a69bad88dfa4de04492dea537d4d` |
| `wallet-broker/Cargo.lock` | 3,273 | `1d182550f82b9dc443804d82e709b5d58021d4243bef1612c440aac59df1c2f5` |

The six Rust tests must retain the format-correction review hashes. Manifest, production
policy, Rust source/tests, lockfile, fixtures, and every other path are frozen.

## Sole task and authorized path

Edit only:

- `test/securityPolicy.node.js`

In the test-side `WAL004_DIRECT_DEPENDENCIES` map, change only `argon2` from exact
`=0.6.0` to exact `=0.5.3`, retaining defaults off, `features: ['alloc']`, and
`optional: false`.

In the existing WAL-004 manifest-policy test's complete-line mutation array, add two
non-vacuous Argon2 mutations:

1. corrected exact `argon2 = "=0.5.3"` to superseded stable `"=0.6.0"`; and
2. corrected exact `argon2 = "=0.5.3"` to loose `"0.5"`.

Use the complete manifest dependency line, the existing uniqueness assertion, explicit
changed-bytes assertion, and existing rejection checker. Remove only the now-obsolete
generic pair `['version = "=0.6.0"', 'version = "0.6"']` from the later mutation array;
its exact-pin and loose-requirement coverage is strictly replaced by the stronger two
complete-line mutations. Preserve every other assertion, mutation, dependency, Zcash
expectation, regex, and all 73 test cases. Do not edit manifest or production policy.

## Restrictions and report

Use `apply_patch`. Do not run Rust, Cargo, Node, npm, tests, formatters, linters, builds,
scanners, dependency resolution, network, fixtures, Git, GitHub, cleanup, wallets,
nodes, or devices. Do not edit, create, delete, move, stage, commit, or push any unlisted
path. Stop on a contradiction rather than weakening a test.

After editing, only read-only `wc -l`, `sha256sum`, test-count/literal inspection, and
`git diff --check` over the one authorized path are allowed. Report exact line count,
SHA-256, unchanged 73-test count, both new mutation pairs, removal of only the obsolete
generic pair, frozen-path integrity, and blockers. Luna owns red execution, evidence,
integration, and Git after reviewer acceptance.
