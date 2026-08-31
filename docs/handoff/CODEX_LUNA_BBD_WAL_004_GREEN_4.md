# Codex Luna Handoff — BBD-WAL-004 Formatter Contingency and Final Green

You are **Jr Dev — Codex Luna**. This durable file is the complete prompt; ephemeral
chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff.

Read `AGENTS.md`, `TESTING.md`, roles, `tickets/BBD-WAL-004.md`,
`docs/testing/BBD-WAL-004-GREEN-RUN-03.md`, source review 05, the prior GREEN_3 handoff,
`CURRENT_TASK.md`, and all source/tests/policy/workflows.

## Frozen pre-format state and sole mutation

Require `HEAD == origin/master` at the governance parent, clean index, and the same
exact 15 production plus six formatter-only test paths accepted by GREEN_3. Require
`wallet-broker/src/vault.rs` at 770 lines and SHA-256
`6014c99d1bdef16ff2b554c2a9e778ac711f05ba82b6fe08e3f49fe6531732b5`; require every
other path and the lockfile at its GREEN_3 handoff hash.

Run this sole authorized mutation:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo fmt --manifest-path wallet-broker/Cargo.toml
```

Inspect the result and require exactly the three already captured layout changes in
`wallet-broker/src/vault.rs`: the seal nonce conversion and the detached encrypt/decrypt
calls collapse to canonical single-line layout. No other production/test path or
semantic token may change. Record the new vault hash. Stop on any other formatter
effect.

## Resume complete green gate

Then run separately in this exact order:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo fmt --manifest-path wallet-broker/Cargo.toml --check
npm run build
npm test
node scripts/security-policy.js
npm audit --audit-level=low
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features
/home/lars/.cargo/bin/rustup run 1.98.0 cargo clippy --manifest-path wallet-broker/Cargo.toml --locked --offline --all-targets --all-features -- -D warnings
/home/lars/.cargo/bin/rustup run 1.98.0 cargo check --manifest-path wallet-broker/Cargo.toml --locked --offline --features native-ui --test native_surface
/home/lars/.cargo/bin/cargo-audit audit --file wallet-broker/Cargo.lock
```

Use the ignored disk-backed repository target and no `/tmp`. Network is authorized only
for the exact npm audit and final push. Success requires all npm suites, all 65 direct
policy cases, zero npm vulnerabilities, all 78 Rust tests and the independent vector,
Clippy/all-features, native compile, and RustSec green; no canary/scratch residue,
unrelated change, window, package build, wallet, node, device, or network client. Stop
without staging on any failure. Do not install cargo-deny/cyclonedx or manually trigger
GitHub; their pinned workflows remain post-push gates.

## Evidence and integration

On exact green create only `docs/testing/BBD-WAL-004-GREEN.md` with preflight, the exact
formatter contingency and resulting vault hash, every command/status/count/tool, the six
formatter-only test paths, no-canary/scratch result, npm/RustSec results, deferred tool
note, and all final hashes. Update only `docs/handoff/CURRENT_TASK.md` to `PRODUCTION
GREEN INTEGRATED — FALSIFICATION AND CI SECURITY/SBOM GATES PENDING` and link evidence.

Run `git diff --check`. Stage exactly the 15 production paths, six formatter-only tests,
evidence, and `CURRENT_TASK.md`; inspect names and the complete staged diff. Commit once
as `feat: add encrypted wallet custody core` and push master. Require
`HEAD == origin/master`, no non-ignored changes, and report commit, full manifest,
evidence line/hash, every result/count/tool, final hashes, and push. The reviewer owns
the separate post-commit falsification handoff and GitHub gate review.
