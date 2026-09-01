# Codex Sol Handoff — BBD-WAL-006 Final Policy Production 01

You are **Principal Dev — Codex Sol** using `gpt-5.6-sol` at High. Implement the already-committed
and already-red final WAL-006 policy contract. This is a one-source-path correction; do not alter
tests or wallet behavior.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely before acting: `AGENTS.md`, `TESTING.md`, `tickets/BBD-WAL-006.md`, this handoff,
`scripts/security-policy.js`, `test/securityPolicy.node.js`, all seven integrated ZEC Rust source
paths, and the CI/local six-failure evidence in
`docs/testing/BBD-WAL-006-PREPARE-ROLLBACK-GATE-01.md`.

## Exact scope

Edit only `scripts/security-policy.js`, starting at 2,306 lines and SHA-256
`2c3d859ddd246b38972c835e604c70bd2bbff7b9266629a9c5c39c1b4d967cea`.

The frozen test is `test/securityPolicy.node.js`, 2,525 lines and SHA-256
`2c1bd92c778a975b1218dfd2445b6b1b605bb047032fd5289573cbbb6d0a0169`.

## Exact expected-red closure

The current local and GitHub runs fail exactly these six tests and no others:

1. committed workflows satisfy the fail-closed checker;
2. strict nine-line reviewed Gitleaks ratchet bytes and content are enforced;
3. WAL-004 Rust source inventory is exported closed and enumerated by repository policy;
4. WAL-006 feature policy distinguishes compiled upstream PCZT capability from BitBook authority;
5. WAL-006 requires the exact bounded Phase-C ZEC production inventory;
6. WAL-006 policy rejects live-network and authority-bearing Rust snippets without denying
   upstream transitives.

## Mandatory implementation

1. Preserve the exported `WAL004_RUST_SOURCE_PATHS` value byte-for-byte. Make
   `checkRustWalletSourceInventory` accept only either that exact legacy top-level set or that set
   plus the now-required exact `wallet-broker/src/zec.rs` entry. It must remain order-independent
   and reject malformed, duplicate, missing, and every other extra path. The separate WAL-006
   recursive inventory check below must make removal of `zec.rs` fail in repository checking.
2. Add and export the exact frozen constants from the Node test:
   `WAL006_FORBIDDEN_FEATURES`, `WAL006_EXPECTED_COMPILED_PCZT_CAPABILITIES`, and
   `WAL006_ALLOWED_RUST_SOURCE_PATHS`.
3. Add and export `checkWal006ResolvedFeatures`. Require a closed, well-formed object; exact direct
   dependency contract; no forbidden enabled feature; exact compiled-PCZT capability inventory;
   and exact BitBook authority `receiver.fresh`, `fixture.scan`, `pczt.prepare`. Reject duplicates,
   unknown/missing values, raw PCZT, sign/prove/finalize/extract/broadcast, and network authority.
4. Add and export `checkWal006RustSourceInventory`. Require the exact seven sorted paths from the
   frozen constant, rejecting malformed, duplicate, missing, and extra entries.
5. Extend `checkRustWalletSource` contextually for WAL-006 source/test paths. Permit reviewed
   offline Zcash library use (including `zcash_client_backend` transitives) while rejecting direct
   network/listener/endpoint/lightwalletd/service-client authority, PCZT or transaction sign/prove/
   finalize/extract calls, broadcast calls, and `Network::MainNetwork` in product source.
   `wallet-broker/src/zec/test_support.rs` may retain its reviewed wrong-network MainNetwork test
   vector. Do not use broad `finalize` matching that rejects cryptographic hasher `.finalize()`.
6. In `checkRepository`, recursively enumerate the `wallet-broker/src/zec.rs` and
   `wallet-broker/src/zec/**/*.rs` inventory, enforce its exact closed set, require every path,
   apply contextual source screening to all seven, and apply the resolved-feature/authority
   contract using the reviewed constants. Keep all existing WAL-004 checks intact.
7. Do not special-case test names, return early for synthetic mutations, weaken an existing
   restriction, parse Cargo.lock heuristically, or add network/tool authority.

## Delivery boundary

Use `apply_patch` only. Read-only local inspection is permitted. Do not run Node, tests, npm,
Cargo, Rust, formatter, policy command, Git, network, cleanup, deletion, or device access. Do not
edit tests, wallet source, manifests, lockfiles, workflows, docs, evidence, or another repository.
Do not stage, commit, or push.

Return the exact changed path with line count/SHA-256, enumerate the new fail-closed checks, explain
how legitimate offline upstream calls remain allowed, and disclose ambiguity. Hermes remains the
sole execution/evidence/integration/Git actor.
