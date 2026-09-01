# Codex Sol Handoff — BBD-WAL-006 CC0 License Test 01

You are **Principal Dev — Codex Sol** using `gpt-5.6-sol` at High. Add the narrow test-first
expectation for the locally verified CC0-1.0 transitive-license correction. Do not edit policy
implementation or deny configuration.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely: `AGENTS.md`, `TESTING.md`, `tickets/BBD-WAL-006.md`, this handoff,
`test/securityPolicy.node.js`, `scripts/security-policy.js`, `deny.toml`, and the final-local-gate
handoff. The stopped cargo-deny 0.20.2 result is authoritative: `bounded-vec 0.9.0`,
`secp256k1 0.29.1`, and `secp256k1-sys 0.10.1` each declare `license = "CC0-1.0"` in their exact
locally cached pinned crate manifests; advisories, bans, and sources passed.

## Exact scope

Edit only `test/securityPolicy.node.js`, starting at 2,525 lines and SHA-256
`2c1bd92c778a975b1218dfd2445b6b1b605bb047032fd5289573cbbb6d0a0169`.

## Mandatory test change

1. Add `'CC0-1.0'` to the frozen `WAL004_ALLOWED_LICENSES` array immediately after `'BSL-1.0'`.
2. In `WAL-004 cargo-deny policy is exact fail-closed and has no bypass lists`, add an exact
   mutation that removes the future line `  "CC0-1.0",\n` and requires rejection. Preserve the
   existing empty exceptions/ignore/allow/deny/skip rules and every prior mutation.
3. Do not add a crate-name exception or waiver. CC0-1.0 is an exact generally allowed license;
   the three locked crates remain governed by the existing closed graph/source policy.

## Delivery boundary

Use `apply_patch` only. Read-only inspection is permitted. Do not run Node/tests/policy/npm,
Cargo/Rust, cargo-deny, Git, network, cleanup, or deletion. Do not edit implementation, deny.toml,
source, manifests, lockfiles, workflows, docs, or another repository. Do not stage/commit/push.

Return the path with line count/SHA-256 and summarize the added positive expectation and removal
falsification. Hermes owns expected-red execution; Sol may not execute it.
