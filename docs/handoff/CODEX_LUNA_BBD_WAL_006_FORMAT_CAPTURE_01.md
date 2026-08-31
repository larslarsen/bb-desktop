# Codex Luna Handoff — BBD-WAL-006 Formatter Diff Capture 01

You are **Jr Dev — Codex Luna**, using `gpt-5.6-luna`. This durable correction supersedes
only the stop/report boundary of the Phase-B handoff. All other restrictions remain.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

The first exact `cargo fmt --check` exited 1 before lock resolution, but its tool output
was truncated. Source actors may not run formatters and must not guess at rustfmt's
mechanical diff. Your sole task is to capture that same formatter result completely.

Require `HEAD == origin/master` at the protected parent, a clean index, the same exact
eight uncommitted paths/hashes from
`docs/testing/BBD-WAL-006-TEST-SOURCE-REVIEW-01.md`, and the unchanged 3,273-line
`wallet-broker/Cargo.lock` SHA-256
`1d182550f82b9dc443804d82e709b5d58021d4243bef1612c440aac59df1c2f5`.
Require the previously inspected ignored, real, disk-backed ext4 target and the existing
WAL-006 temp/Cargo directories. Stop on mismatch.

Run exactly once, capturing both streams beneath ignored target state:

```text
env TMPDIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-tmp CARGO_TARGET_DIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-cargo /home/lars/.cargo/bin/cargo +1.98.0 fmt --manifest-path wallet-broker/Cargo.toml --check > wallet-broker/target/wal006-format-check.stdout 2> wallet-broker/target/wal006-format-check.stderr
```

Expected exit is 1 with formatting diff only. Do not run `cargo fmt` without `--check`,
apply any change, resolve dependencies, run tests/Node, generate fixtures, stage, commit,
push, delete, or clean. Report both capture line counts/hashes, the complete formatter
path set, and confirmation that every accepted source hash and the lockfile remain exact.
Stop for XHigh to route a source-only correction.
