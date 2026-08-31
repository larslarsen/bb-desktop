# Codex Sol Handoff — BBD-WAL-006 Address Clippy Correction 01

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This durable file is
the complete correction prompt; ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely: `AGENTS.md`, `TESTING.md`, the address gate/resume, Source Review 03, the format
reviews, `BBD-WAL-006-ADDRESS-GATE-CLIPPY-REVIEW-01.md`, and current `address.rs`.

Using `apply_patch`, delete exactly the two `drop(spending);` statements diagnosed by Clippy in
`derive_ufvk_for` and `derive_only_for`. Add no lint suppression, comment, replacement statement,
or other change.

The sole writable path is `wallet-broker/src/zec/address.rs`. Every other source, test, fixture,
manifest, lockfile, policy, document, workflow, package, and repository path is frozen.

Do not run Cargo, Rust, rustfmt, Node, npm, tests, builds, linters, policy tools, scanners, network,
or Git. After the patch, use only read-only `wc -l`, `sha256sum`, literal inspection, and
source-only `git diff --check`. Report the exact line count/hash and confirm only the two retained
deletions. Luna will restart the gate only after reviewer hash acceptance.
