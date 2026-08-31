# Codex Luna Handoff — BBD-WAL-004 Correction 1 Expected Red

You are **Jr Dev — Codex Luna**. This durable file is the complete integration prompt;
ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff.

Read completely before acting: `AGENTS.md`, `TESTING.md`,
`docs/engineering/DEVELOPMENT_ROLES.md`, `tickets/BBD-WAL-004.md`,
`docs/testing/BBD-WAL-004-PRODUCTION-SOURCE-REVIEW-01.md`,
`docs/testing/BBD-WAL-004-CORRECTION-1-TEST-SOURCE-REVIEW.md`,
`docs/handoff/CURRENT_TASK.md`, the five accepted test paths, and the current 15-path
uncommitted production drop.

## Sole task

Independently prove the accepted Correction 1 regressions fail for the reviewed reasons
against frozen rejected production drop 01. This is an expected-red run, not a source
repair. Do not edit production, policy implementation, workflows, manifests, lockfile,
fixtures, test source, ticket, roadmap, or this handoff.

Official user-level Rust and Cargo must both report exactly `1.98.0`. All Cargo work is
offline, locked, no-default-features, and disk-backed under the repository's ignored
`wallet-broker/target`; do not use `/tmp` or create another target/cache.

## Immutable preflight

Require `HEAD == origin/master` at the protected governance parent and verify a clean
index. The worktree must contain exactly the five accepted modified test paths plus the
15 frozen production paths, and no other change. Match the five test line counts and
SHA-256 values in `BBD-WAL-004-CORRECTION-1-TEST-SOURCE-REVIEW.md`.

Match these frozen production SHA-256 values before and after execution:

| Path | SHA-256 |
| --- | --- |
| `wallet-broker/Cargo.toml` | `d1d338ff0cb63eb6c7f992b2573b6ebc4ee5d7d459301961eb0b4aaa8d2ebd7c` |
| `wallet-broker/src/lib.rs` | `09e1ba98383fedda2b0db5e36ff716f0a8ce30ef37072901dc8b1e31be06dbdc` |
| `wallet-broker/src/vault.rs` | `a03c4f2950ac2bde535b5b163dae99f798e34e00197d0988f1293389e4d3b09c` |
| `wallet-broker/src/store.rs` | `d8ac68267a27887c8ca5568acdf04a8b30685b25a4ed967e02d57fb3b35a14d1` |
| `wallet-broker/src/session.rs` | `ee2f1bbb516757f4db173ec4be8edfbb029dbc469704182614229912c15c4560` |
| `wallet-broker/src/native.rs` | `a49c44a0e5818b64a9f2ae077e72da78c01c3d50c47bf3d5a812d8d9ddf37f7f` |
| `wallet-broker/src/native_ui.rs` | `d13b86f33bad2cad2727ece60dc627e24cb2dc34ff65fa60b62c2a1882831085` |
| `wallet-broker/src/hygiene.rs` | `f69ea62246d3c7a45530e314785182171a9fc674c6aa23d5eb6f8bb204a66e06` |
| `deny.toml` | `3bd1161fbd31552f0c7887ac02d477be71e3c2542d15c1896bd5d037c562d5a8` |
| `scripts/security-policy.js` | `1b9dd0e704652353da1a45355e16353981c80ec846e59af57796fbd7aecbe738` |
| `scripts/validate-rust-sbom.js` | `6b90e1a5dcc423a6a891cbf3d5964536e00772fb6549e64d82bce3fcec84b4a0` |
| `package.json` | `9b8b03edc602554e98266d7a79168eacbffe4243a686b52d92bc1dd8a52e3893` |
| `.github/workflows/social.yml` | `2da766941b0670ac398b8537b77010337e6f352f0d182283d6a9cf86805275c6` |
| `.github/workflows/security.yml` | `54b185e83a3a5db6848834a7769e1871057bb53b3d259e6ead47309b64ccd62a` |
| `.github/workflows/sbom.yml` | `8407f00fc0ed9ad7bd88c726d64e5cd02a61922653991f9cf4b7cf8bea528824` |

Stop on any mismatch or extra path.

## Exact execution

Run and capture exit status/output for these commands separately, in this order:

```text
rustc --version
cargo --version
cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test vault_store
cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test vault_session
cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test native_surface
cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test secret_hygiene
node test/securityPolicy.node.js
```

The four Rust test binaries must reach execution. Existing accepted tests must remain
green. The newly accepted test names must fail on the corresponding reviewed production
gap; a compile error, missing dependency, offline resolution failure, unrelated failure,
abort, signal, or canary disclosure is not acceptable red. The Node suite must execute
all cases; record every `ok`/`not ok`, and distinguish any new case that already passes
from the cases that prove the frozen defects. No command is required to fail if its
new case is already satisfied; exact observed results govern.

Do not run all-features/native UI, fmt, clippy, audit, deny, SBOM generation, npm, Electron,
wallets, nodes, devices, network, installers, cleanup, deletion, or unlisted commands.

## Evidence and Git

Create only `docs/testing/BBD-WAL-004-CORRECTION-1-EXPECTED-RED.md`. Record tool versions,
every command and exit status, exact new failing test names/reasons, counts of prior
green tests, Node `ok`/`not ok` totals, no-canary result, pre/post integrity, and any
new regression that unexpectedly passes. Do not copy secret canary values into evidence.

Update only `docs/handoff/CURRENT_TASK.md` to state `CORRECTION 1 EXPECTED RED RECORDED —
PRODUCTION CORRECTION REQUIRED` and link the evidence. Run `git diff --check`. Stage only
the five accepted test paths, the new evidence, and `CURRENT_TASK.md`; inspect the staged
manifest and diff. Commit once as `test: record wallet custody correction red` and push
`master`. The 15 frozen production paths must remain untracked/unstaged or modified/
unstaged exactly as they began. Final `HEAD == origin/master`; report commit, evidence
line count/hash, test path hashes, frozen production hashes, and final status.
