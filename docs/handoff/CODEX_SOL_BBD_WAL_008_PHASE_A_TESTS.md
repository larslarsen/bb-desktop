# Codex Sol Handoff — BBD-WAL-008 Phase-A Test Source

You are **Principal Dev — Codex Sol** using `gpt-5.6-sol` at High. Grok Build's weekly
usage is exhausted, so this is the documented fill-in route.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely: `AGENTS.md`, `TESTING.md`, `docs/engineering/DEVELOPMENT_ROLES.md`,
`docs/handoff/CURRENT_TASK.md`, `tickets/BBD-WAL-008.md`, the BBD-WAL-006 ticket and
acceptance record, the Zcash hardware sections of
`docs/architecture/BBD-WAL-001-REVIEW.md`, and the existing Zcash source/tests needed to
understand the accepted adapter boundary.

Author only the Phase-A test source fixed by BBD-WAL-008. You may edit exactly:

- `wallet-broker/Cargo.toml`
- `wallet-broker/tests/zec_hardware.rs`

The manifest change may add only the explicit `zec_hardware` test target. Add no
dependency, feature, build script, profile, or other manifest change.

The test file must cover all thirteen required Phase-A groups, including a clearly
synthetic test-only positive Keystone profile, the empty production positive table,
exact branch/tx/PCZT-v2 matching, reviewed/live intersection, route-selection metadata,
Trezor/Ledger/disconnect negatives, persistence faults/drift, bounds, verified-field
intersection, redaction, and production-inventory prohibitions. Behavioral tests must
target a typed `zec::test_support` harness expected to be absent before production; do
not substitute local test implementations that make the suite green.

Do not edit production/test-support source, existing tests, fixtures, `Cargo.lock`,
Node/Electron, policy, docs, evidence, workflows, packages, or any other path. Do not
run a command, formatter, test, build, dependency resolver, Git, GitHub, network, or
another actor. Do not create stubs, ignored tests, conditional passes, real-device
claims, transport, PCZT signing/extraction, or a production capability pin.

Stop after editing. Report the two changed paths with exact line counts and SHA-256,
plus a concise mapping from every test to the ticket requirement it proves.
