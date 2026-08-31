# Codex Sol Handoff — BBD-WAL-006 Support-Dependency Production 01

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This durable file is
the complete source prompt; ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Implementation source baseline: `1f24a22191c25a049ea690170b751be85eef4308`

Protected governance parent: the commit containing this handoff.

Read completely: `AGENTS.md`, `TESTING.md`, roles, `tickets/BBD-WAL-006.md`, the address
source stop review, support-dependency test/red evidence and reviews,
`docs/handoff/CURRENT_TASK.md`, the complete manifest, complete
`scripts/security-policy.js`, and the complete WAL-004/WAL-006 policy tests.

## Sole task and exact paths

Implement only the tested support-dependency manifest and narrow manifest-policy
correction. You may edit exactly:

- `wallet-broker/Cargo.toml`
- `scripts/security-policy.js`

Append these exact lines immediately after the existing `zcash_keys` dependency line,
without moving or changing any accepted dependency:

```text
rand_core = { version = "=0.6.4", default-features = false, features = ["std"] }
rusqlite = { version = "=0.37.0", default-features = false }
```

In the policy implementation:

- define/export `WAL006_DIRECT_DEPENDENCIES` exactly as the committed six-crate test
  object;
- define/export `WAL006_SUPPORT_DEPENDENCIES` exactly as the committed two-crate object;
- define/export `WAL006_TEST_TARGETS` exactly as the six committed ZEC test targets;
- extend `checkWalletBrokerManifest`'s exact ordered dependency inventory to the current
  12 custody/UI lines, six accepted Zcash lines, then the two new appended support lines;
- extend its exact integration-test inventory with the six accepted ZEC test targets;
- remove only the obsolete blanket `zcash` rejection from the manifest authority regex,
  because exact inventory/feature matching now owns those declarations; and
- use stable manifest/dependency/support/RNG/SQLite errors that satisfy every committed
  mutation without weakening the old WAL-004 checks.

Do not implement or export `WAL006_FORBIDDEN_FEATURES`,
`WAL006_EXPECTED_COMPILED_PCZT_CAPABILITIES`,
`WAL006_ALLOWED_RUST_SOURCE_PATHS`, `checkWal006ResolvedFeatures`, or
`checkWal006RustSourceInventory` in this slice. Do not broaden
`checkRustWalletSource` for ZEC yet. Those feature/source-authority policies remain
separate expected red until the corresponding production inventory exists.

## Boundaries

Do not edit `Cargo.lock`, Rust source/test, Node tests, fixtures, documentation, evidence,
handoffs, tickets, workflows, package files, deny policy, SBOM validator, or any unlisted
path. Do not add `rand`, `schemerz`, `schemerz-rusqlite`, another SQLite crate, another
feature, default feature, optional marker, patch, git source, or version range.

Use `apply_patch`. Read-only `sed`/`rg` inspection is permitted. Do not run Node, npm,
Cargo, Rust, tests, formatters, linters, builds, policy checkers, scanners, Electron,
wallets, nodes, devices, Git, network, dependency resolution, install, cleanup, commit,
or push.

Report both changed paths with line counts/SHA-256, the exact constants/checker changes,
and any contradiction. Luna—not Sol—will resolve the lock, run the exact dependency
gate, author evidence, and own all Git operations. ZEC Rust production remains frozen.
