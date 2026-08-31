# BBD-WAL-006 Dependency Test Source Review 01

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Protected governance parent: `0b71e3d9`

Result: **TEST SOURCE ACCEPTED FOR FOCUSED EXPECTED RED**

Sol changed only `test/securityPolicy.node.js`. The accepted file is 2,374 lines with
SHA-256 `f4f6defd5d55212c480a7a1a87d37edea85786b6125cad29467ae5e1f3480ee4` and retains
all 73 test cases.

The test-side `WAL004_DIRECT_DEPENDENCIES` map now requires exact, defaults-off
`hkdf = 0.12.4` and `sha2 = 0.10.9`. The existing WAL-004 manifest-policy test retains
every prior assertion and mutation and adds four independent complete-line mutations:

- exact `hkdf 0.12.4` to superseded stable `0.13.0`;
- exact `hkdf 0.12.4` to loose `0.12`;
- exact `sha2 0.10.9` to superseded stable `0.11.0`; and
- exact `sha2 0.10.9` to loose `0.10`.

For each pair the test proves the source dependency line occurs exactly once, proves the
replacement changed the manifest bytes, and requires the existing checker to reject the
mutation. It adds no alternate implementation, skipped case, conditional pass, relaxed
pattern, Zcash expectation, new test case, or test-only production path.

Reviewer inspection confirms `git diff --check` passes. The manifest remains 79 lines at
SHA-256 `420559e61a7c81338c97301f7d470a8be6560c0f71aa9f666d79bbcc1424361d`, production
policy remains 2,231 lines at SHA-256
`affe15ea73706becb6156409afae80ce44076641915457a9e2b9399207ed558f`, and the lockfile
remains 3,273 lines at SHA-256
`1d182550f82b9dc443804d82e709b5d58021d4243bef1612c440aac59df1c2f5`.

Sol ran no Node, Rust, Cargo, npm, formatter, linter, build, scanner, dependency
resolution, network, fixture, Git, cleanup, wallet, node, or device command. Luna may run
only the focused existing WAL-004 manifest-policy case under the active handoff. It must
fail at the frozen old production dependency map before manifest validation. Manifest,
production policy, Rust source/tests, lockfile, fixtures, and all other test execution
remain unauthorized.
