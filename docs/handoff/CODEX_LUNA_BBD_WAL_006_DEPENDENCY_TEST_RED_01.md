# Codex Luna Handoff — BBD-WAL-006 Dependency Test Expected Red 01

You are **Jr Dev — Codex Luna**, using `gpt-5.6-luna`. This durable file is the complete
integration prompt; ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely before acting: `AGENTS.md`, `TESTING.md`,
`docs/engineering/DEVELOPMENT_ROLES.md`, `tickets/BBD-WAL-006.md`,
`docs/testing/BBD-WAL-006-DEPENDENCY-RESOLUTION-REVIEW-01.md`,
`docs/testing/BBD-WAL-006-DEPENDENCY-TEST-SOURCE-REVIEW-01.md`, and the complete accepted
Node test, current manifest, production policy, and lockfile.

## Preflight

Require `HEAD == origin/master` at the protected governance parent and a clean index.
Require exactly the eight accepted uncommitted Phase-A paths, with the Node test at the
new review hash and every other path at its format-correction review hash. Require
`git diff --check` and these frozen paths:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `test/securityPolicy.node.js` | 2,374 | `f4f6defd5d55212c480a7a1a87d37edea85786b6125cad29467ae5e1f3480ee4` |
| `wallet-broker/Cargo.toml` | 79 | `420559e61a7c81338c97301f7d470a8be6560c0f71aa9f666d79bbcc1424361d` |
| `scripts/security-policy.js` | 2,231 | `affe15ea73706becb6156409afae80ce44076641915457a9e2b9399207ed558f` |
| `wallet-broker/Cargo.lock` | 3,273 | `1d182550f82b9dc443804d82e709b5d58021d4243bef1612c440aac59df1c2f5` |

The six Rust test hashes remain exactly those in the format-correction review. Stop on
an extra path, hash/count mismatch, staged change, or dirty protected source.

## Exact expected-red execution

From repository root, run exactly once:

```text
node -e 'const name = "WAL-004 Rust test-harness manifest pins the exact toolchain, dependencies, and native features"; const matches = require("./test/securityPolicy.node.js").tests.filter((entry) => entry.name === name); if (matches.length !== 1) throw new Error(`expected one test, found ${matches.length}`); matches[0].fn();'
```

Expected exit is nonzero from the first `assert.deepStrictEqual` in that test. The
diagnostic must show the frozen production `WAL004_DIRECT_DEPENDENCIES` map still has
`hkdf =0.13.0` and `sha2 =0.11.0`, while the test expects exact `=0.12.4` and `=0.10.9`.
The test must be found exactly once and must not reach manifest validation or any added
mutation. A syntax/load error, missing/duplicate test, manifest-policy error, unexpected
field difference, unexpected pass, abort, signal, canary, or secret is unintended red;
stop without integration.

Do not run the complete Node suite, Rust, Cargo, npm, formatter, linter, build, scanner,
dependency resolution, network, fixture generator, wallet, node, device, or any unlisted
command. Do not edit source, test, manifest, production policy, lockfile, fixture,
workflow, package, ticket, or handoff files.

## Evidence and Git

On exact red only, create
`docs/testing/BBD-WAL-006-DEPENDENCY-CORRECTION-EXPECTED-RED-01.md` recording timestamp
and timezone, protected parent, exact command/status, the two actual/expected dependency
pairs, precise assertion site/reason, no-canary result, and pre/post hashes. Update only
`docs/handoff/CURRENT_TASK.md` to
`DEPENDENCY PIN CORRECTION EXPECTED RED RECORDED — REVIEW REQUIRED` and link the evidence.

Run `git diff --check`. Stage only the accepted Node test, evidence, and `CURRENT_TASK.md`;
inspect staged names and diff. Commit once as
`test: record WAL-006 dependency correction red` and push `master`. Leave the manifest
and six Rust tests unstaged and hash-identical. Require final `HEAD == origin/master`, a
clean index, and exactly those seven remaining uncommitted paths. Report commit, evidence
line count/hash, exact exit/reason, final hashes/status, and no blocker. Do not continue
to manifest or production-policy correction.
