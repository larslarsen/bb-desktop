# Codex Sol Handoff — BBD-WAL-006 Address Production Correction 01

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This durable file is
the complete correction prompt; ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Uncommitted source drop: the six hashes recorded in
`docs/testing/BBD-WAL-006-ADDRESS-PRODUCTION-SOURCE-REVIEW-02.md`

Read completely: `AGENTS.md`, `TESTING.md`, roles, ticket, architecture, the original and
resume address-production handoffs, the current six-path drop, the accepted `zec_address` test
and frozen manifest, and Source Review 02. Preserve the existing uncommitted source drop.

## Sole task

Correct only the four blocking findings from Source Review 02 without executing the source.

1. Give `LocalNetwork` exact field equality consistent with its derived hash. Retain the outer
   `Network`'s regtest-discriminator equality needed by test-only UA decoding, and document that
   distinction adjacent to the implementation. Do not weaken the exact activation-height binding
   stored in SQLite.

2. Close validation of the three `ext_bitbook_` tables. Detect and reject every unreviewed table,
   index, trigger, or view named with or attached to the reserved extension namespace, accounting
   explicitly for SQLite-owned primary-key autoindexes. Validate the table definitions and
   constraints, not only column names. In the same `BEGIN IMMEDIATE` transaction that issues a
   receiver, revalidate the exact account, network discriminator, local birthday, NU6.3, and
   confirmation binding plus the extension schema before derivation or mutation.

3. Retain the fixed account paths in the production account owner and revalidate the complete
   root/network/account directory chain and both regular state files before every post-bootstrap
   database inspection or mutation. Preserve exact `0700`/`0600` checks and symlink rejection.
   Do not add a caller path, unsafe code, or another dependency.

4. Make the closed fixture serde model viable under the already-required warnings-denied Clippy
   gate. Prefer bounded structural validation of declared fields. Any dead-code allowance must be
   field/type scoped and carry a specific reason; module/crate-wide lint suppression is forbidden.

Preserve all sound behavior listed in Source Review 02. Do not change the accepted tests or make
the implementation detect test labels, fixture bytes, or expected receiver literals.

## Exact authorized paths

Only these existing uncommitted production paths are writable:

- `wallet-broker/src/zec.rs`
- `wallet-broker/src/zec/fixture.rs`
- `wallet-broker/src/zec/store.rs`

`wallet-broker/src/lib.rs`, `wallet-broker/src/zec/address.rs`, and
`wallet-broker/src/zec/test_support.rs` are frozen unless a concrete compile-independent type
contradiction in the three authorized corrections requires one; stop and report that contradiction
instead of widening the edit. Every test, fixture, manifest, lockfile, policy file, document,
workflow, package file, and other source path is frozen.

Use `apply_patch`. Read-only repository and cached-upstream inspection plus final `wc -l`,
`sha256sum`, and source-only `git diff --check` are permitted. Do not run Cargo, Rust, rustfmt,
Node, npm, tests, builds, linters, policy tools, scanners, Electron, wallets, nodes, devices,
network, or Git. Do not install, delete, clean, move, stage, commit, or push anything.

Stop after the correction and report exact changed paths, line counts, SHA-256 hashes, schema and
path-validation design, and any concern. Luna owns later execution/evidence/integration/Git only
after a new reviewer acceptance.
