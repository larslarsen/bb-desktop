# Codex Sol Handoff — BBD-WAL-006 Dependency Production Correction 01

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This durable file is
the complete correction prompt; ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely before acting: `AGENTS.md`, `TESTING.md`,
`docs/engineering/DEVELOPMENT_ROLES.md`, `tickets/BBD-WAL-006.md`,
`docs/testing/BBD-WAL-006-DEPENDENCY-RESOLUTION-REVIEW-01.md`,
`docs/testing/BBD-WAL-006-DEPENDENCY-TEST-SOURCE-REVIEW-01.md`,
`docs/testing/BBD-WAL-006-DEPENDENCY-CORRECTION-EXPECTED-RED-01.md`,
`docs/testing/BBD-WAL-006-DEPENDENCY-TEST-RED-REVIEW-01.md`, and both authorized paths
completely.

## Preflight and frozen state

Require `HEAD == origin/master` at the protected governance parent, a clean index, and
exactly seven uncommitted paths: the accepted 79-line manifest plus the six accepted
formatted Rust tests. Require `git diff --check` and these starting hashes:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/Cargo.toml` | 79 | `420559e61a7c81338c97301f7d470a8be6560c0f71aa9f666d79bbcc1424361d` |
| `scripts/security-policy.js` | 2,231 | `affe15ea73706becb6156409afae80ce44076641915457a9e2b9399207ed558f` |
| `wallet-broker/Cargo.lock` | 3,273 | `1d182550f82b9dc443804d82e709b5d58021d4243bef1612c440aac59df1c2f5` |
| `test/securityPolicy.node.js` | 2,374 | `f4f6defd5d55212c480a7a1a87d37edea85786b6125cad29467ae5e1f3480ee4` |

The six Rust test hashes must match the format-correction review. Test source, Rust
source, lockfile, fixture paths, workflow/package inputs, and every other path are frozen.

## Sole task and authorized paths

Edit only:

- `wallet-broker/Cargo.toml`
- `scripts/security-policy.js`

In the manifest, change only these exact direct dependency versions, retaining exact
requirements and `default-features = false`:

- `hkdf` from `=0.13.0` to `=0.12.4`;
- `sha2` from `=0.11.0` to `=0.10.9`.

In production policy, make only the four corresponding literal changes:

- the `WAL004_DIRECT_DEPENDENCIES.hkdf.version` value;
- the `WAL004_DIRECT_DEPENDENCIES.sha2.version` value;
- the exact `hkdf` line in `checkWalletBrokerManifest`'s required dependencies; and
- the exact `sha2` line in that required list.

Do not change any feature/default/optional property, dependency order, test target,
package field, Zcash pin/feature, checker control flow, error pattern, export, WAL-006
policy, or other byte. Do not add an alias, patch, git/path dependency, override, feature,
or comment.

## Restrictions and report

Use `apply_patch`. Do not run Rust, Cargo, Node, npm, tests, formatters, linters, builds,
scanners, dependency resolution, network, fixtures, Git, GitHub, cleanup, wallets,
nodes, or devices. Do not edit, create, delete, move, stage, commit, or push any unlisted
path. Stop on a contradiction instead of changing a Zcash version or weakening policy.

After editing, only read-only `wc -l`, `sha256sum`, exact-literal inspection, and
`git diff --check` over the two authorized paths are allowed. Report both exact line
counts/hashes, confirm exactly six corrected version occurrences across manifest and
policy, and confirm all frozen paths remain exact. Luna owns execution, lock resolution,
evidence, integration, and Git after reviewer acceptance.
