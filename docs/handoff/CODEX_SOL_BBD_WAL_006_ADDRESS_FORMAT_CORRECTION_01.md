# Codex Sol Handoff — BBD-WAL-006 Address Format Correction 01

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This durable file is
the complete correction prompt; ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely: `AGENTS.md`, `TESTING.md`, the address gate handoff, Source Review 03,
`BBD-WAL-006-ADDRESS-GATE-FORMAT-REVIEW-01.md`, and the four current source files.

Apply exactly the 14 retained rustfmt replacements recorded in Format Review 01, using
`apply_patch`. Make no semantic or additional layout change.

## Exact authorized paths

- `wallet-broker/src/zec/address.rs`
- `wallet-broker/src/zec/fixture.rs`
- `wallet-broker/src/zec/store.rs`
- `wallet-broker/src/zec/test_support.rs`

Every other source, test, fixture, manifest, lockfile, policy, document, workflow, package, and
repository path is frozen. Do not run Cargo, Rust, rustfmt, Node, npm, tests, builds, linters,
policy tools, scanners, network, or Git. Do not stage, commit, push, install, move, clean, or
delete anything.

After the patch, use only read-only `wc -l`, `sha256sum`, literal inspection, and source-only
`git diff --check`. Stop and report exact path lines/hashes and confirmation that every change is
one of the retained formatter replacements. Luna will resume the gate only after a new source-hash
review and reviewer authorization.
