# Codex Luna Handoff — BBD-WAL-004 Final Green Integration 5

You are **Jr Dev — Codex Luna**. This durable file is the complete prompt; ephemeral
chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff.

Read completely: `AGENTS.md`, `TESTING.md`, roles, `tickets/BBD-WAL-004.md`, every
WAL-004 review/red/green record, prior green handoffs, `CURRENT_TASK.md`, and all
source/tests/policy/deny/validator/workflows.

## Preflight

Require `HEAD == origin/master` at the governance parent and clean index. Require exactly
the accepted 15 production paths plus six formatter-only Rust test paths and no extra
path. Match every SHA-256 in GREEN_3 except these accepted later values:

```text
611d837641069a98d05b9e68c14bf11a37a5076de58bf6516188870eeab19236  wallet-broker/src/store.rs
519b544e34cba1dcba240e93f03b19d05fb5137477bfb2441dd3a57c91c79a41  wallet-broker/src/vault.rs
```

The immutable lockfile remains
`1d182550f82b9dc443804d82e709b5d58021d4243bef1612c440aac59df1c2f5`.
Inspect the six test diffs and require rustfmt-only layout with no semantic change.

## Exact complete gate

Use `/home/lars/.cargo/bin/rustup run 1.98.0`, locked/offline Cargo where shown, the
ignored disk-backed repository target, and no `/tmp`. Run separately in order:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 rustc --version
/home/lars/.cargo/bin/rustup run 1.98.0 cargo --version
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

Do not mutate formatting. Network is authorized only for the exact npm audit and final
push. Success requires all npm suites, all 65 direct policy cases, zero npm
vulnerabilities, all 78 Rust tests and the independent vector, Clippy/all-features,
native compile, and RustSec green; no canary/scratch residue, unrelated change, window,
package build, wallet, node, device, or network client. Stop without staging on any
failure. Do not install cargo-deny/cyclonedx or manually trigger GitHub.

## Evidence and integration

On exact green create only `docs/testing/BBD-WAL-004-GREEN.md` with preflight, prior
formatter contingency/final hash, every command/status/count/tool, six formatter-only
test paths, no-canary/scratch result, npm/RustSec results, deferred local-tool note, and
all final hashes. Update only `docs/handoff/CURRENT_TASK.md` to `PRODUCTION GREEN
INTEGRATED — FALSIFICATION AND CI SECURITY/SBOM GATES PENDING` and link evidence.

Run `git diff --check`. Stage exactly the 15 production paths, six formatter-only tests,
evidence, and `CURRENT_TASK.md`; inspect the complete staged diff/names. Commit once as
`feat: add encrypted wallet custody core` and push master. Require final
`HEAD == origin/master`, no non-ignored change, and report commit, full manifest,
evidence line/hash, every result/count/tool, final hashes, and push. Reviewer owns the
separate post-commit falsification handoff and GitHub gate review.
