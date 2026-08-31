# Codex Sol Handoff — BBD-WAL-004 Correction 2 Node Fixture

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This durable file is
the complete prompt; ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff.

Read `AGENTS.md`, `TESTING.md`, roles, `tickets/BBD-WAL-004.md`,
`docs/testing/BBD-WAL-004-CORRECTION-2-INTEGRATION-REVIEW-01.md`, and the complete test
`WAL-004 Rust first-party source policy forbids unsafe and unreviewed authority` in
`test/securityPolicy.node.js`.

Edit only `test/securityPolicy.node.js`. In that existing generic policy test, change
only the positive-control path argument paired with the minimal synthetic zeroize source
from `wallet-broker/src/vault.rs` to `wallet-broker/src/synthetic.rs`. Preserve the source
string, every negative mutation, all Correction 1/2 tests, names, order, and assertions.
Do not edit production policy or weaken vault-specific primitive checks. Do not touch any
other path.

Use `apply_patch`. Do not run Node, Rust, Cargo, tests, formatters, builds, scanners,
network, Git, or any project command. Do not stage, commit, push, delete, use `/tmp`, or
use root. After the edit, only `wc -l test/securityPolicy.node.js` and
`sha256sum test/securityPolicy.node.js` are allowed. Report count/hash and no blocker.
