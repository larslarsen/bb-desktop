# Codex Sol Handoff — BBD-WAL-006 Format Correction 01

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This durable file is
the complete source-only correction prompt; ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read `AGENTS.md`, `TESTING.md`, `docs/handoff/CURRENT_TASK.md`,
`docs/testing/BBD-WAL-006-TEST-SOURCE-REVIEW-01.md`, the original source handoff, all six
accepted Rust test files, and the complete ignored formatter capture at
`wallet-broker/target/wal006-format-check.stdout` before editing. The capture is 1,184
lines with SHA-256 `11a606adcbe0d509a6287a1bbc1e5c0029c5aadfe47eb75b8257ad45cdea88ca`;
its paired stderr is empty with SHA-256
`e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`.

Require a clean index, `HEAD == origin/master` at the protected parent, the exact eight
uncommitted Phase-A hashes in the test-source review, and unchanged 3,273-line
`wallet-broker/Cargo.lock` SHA-256
`1d182550f82b9dc443804d82e709b5d58021d4243bef1612c440aac59df1c2f5`.
Stop without editing on mismatch.

Your sole task is to apply the capture's exact rustfmt-proposed textual transformations,
using `apply_patch`, to exactly:

- `wallet-broker/tests/zec_address.rs`
- `wallet-broker/tests/zec_fixture_builder.rs`
- `wallet-broker/tests/zec_hygiene.rs`
- `wallet-broker/tests/zec_prepare.rs`
- `wallet-broker/tests/zec_scan.rs`
- `wallet-broker/tests/zec_store.rs`

Do not make a semantic correction, accept a formatter change outside those files, or
edit `wallet-broker/Cargo.toml`, `test/securityPolicy.node.js`, the ignored capture,
fixture output, lockfile, production, policy, evidence, documentation, or another path.
Preserve every test name/count, literal, assertion, type/API reservation, and prior
review correction. If the capture cannot be applied exactly or implies a semantic change,
stop and report the hunk.

You may use read-only inspection and report `wc -l`/`sha256sum` over the six corrected
paths. Do not run Rust, Cargo, rustfmt, Node, npm, tests, builds, scanners, Git, network,
dependency resolution, fixture generation, deletion, cleanup, wallets, nodes, hardware,
or devices. Stop after reporting the six new line counts/hashes and confirmation that
the other two accepted paths and lockfile remain byte-exact. XHigh must inspect the new
hashes before a separate Luna resume exists.
