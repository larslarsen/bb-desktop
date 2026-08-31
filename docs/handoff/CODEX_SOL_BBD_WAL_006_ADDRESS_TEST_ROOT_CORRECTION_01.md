# Codex Sol Handoff — BBD-WAL-006 Address Test-Root Correction 01

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This durable file is
the complete correction prompt; ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely: `AGENTS.md`, `TESTING.md`, Address Gate 01 and Resumes 01/02, all gate stop and
correction reviews, Source Review 03, the accepted `zec_address` test, and current
`wallet-broker/src/zec/test_support.rs`.

Correct only the parallel `create_test_directory` race described in
`BBD-WAL-006-ADDRESS-GATE-TEST-REVIEW-01.md`. Use `std::os::unix::fs::DirBuilderExt` so a newly
created ancestor receives mode `0700` atomically. If creation loses to another test with
`AlreadyExists`, continue only after fresh `symlink_metadata` proves a real nonsymlink directory.
Preserve failure for other errors/types. Do not chmod an existing repository/build ancestor and do
not change unique state-root allocation or production path validation.

The sole writable path is `wallet-broker/src/zec/test_support.rs`. Use `apply_patch`. Every other
source, test, fixture, manifest, lockfile, policy, document, workflow, package, and repository path
is frozen.

Do not run Cargo, Rust, rustfmt, Node, npm, tests, builds, linters, policy tools, scanners, network,
or Git. After editing, use only read-only `wc -l`, `sha256sum`, literal inspection, and source-only
`git diff --check`. Report the exact line count/hash and concurrency design. Luna will restart the
full gate only after reviewer source acceptance.
