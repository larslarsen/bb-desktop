# Codex Sol Handoff — BBD-WAL-006 CC0 License Production 01

You are **Principal Dev — Codex Sol** using `gpt-5.6-sol` at High. Implement the exact two-line
production policy correction proven by the accepted expected red. Do not alter tests or any other
policy rule.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely: `AGENTS.md`, `TESTING.md`, `tickets/BBD-WAL-006.md`, the CC0 test and expected-red
handoffs, this handoff, `test/securityPolicy.node.js`, `scripts/security-policy.js`, and
`deny.toml`.

The accepted expected red is 74 passed/one failed in
`WAL-004 cargo-deny policy is exact fail-closed and has no bypass lists`: the frozen test expects
`CC0-1.0`, while the exported policy constant and deny allow-list omit it. Cargo-deny 0.20.2
independently rejected the exact pinned CC0-1.0 crates `bounded-vec 0.9.0`, `secp256k1 0.29.1`, and
`secp256k1-sys 0.10.1`; advisories, bans, and sources passed.

## Exact scope

Edit only:

- `scripts/security-policy.js`, starting at 2,482 lines and SHA-256
  `d9b77bf5608e1aa79d565e5c3c9574c1e203286c584c30498dd173796434d7f5`;
- `deny.toml`, starting at 39 lines and SHA-256
  `3bd1161fbd31552f0c7887ac02d477be71e3c2542d15c1896bd5d037c562d5a8`.

## Mandatory correction

Add exactly one `CC0-1.0` entry immediately after `BSL-1.0` in each closed license list:

- `'CC0-1.0',` in `WAL004_ALLOWED_LICENSES`;
- `  "CC0-1.0",` in `[licenses].allow`.

Preserve order and every other byte/rule. Do not add crate exceptions, ignores, source bypasses,
license confidence changes, or any broader license. The expected-red test's exact removal mutation
must remain capable of rejecting omission.

## Delivery boundary

Use `apply_patch` only. Read-only inspection is permitted. Do not run Node/tests/policy/npm,
Cargo/Rust/cargo-deny, Git, network, cleanup, or deletion. Do not edit tests, source, manifest,
lock, workflow, docs, or another repository. Do not stage/commit/push.

Return the two paths with line counts/SHA-256 and confirm the exact entries/no exceptions. Hermes
owns execution/evidence/integration.
