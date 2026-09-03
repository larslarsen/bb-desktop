# Grok Build Handoff — BBD-WAL-007 Slice 4 Authority Correction 01

Status: AUTHORIZED — EXACT ONE-PATH SOURCE CORRECTION

Source actor: Sr Dev — Grok Build using Grok 4.6 High

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read `AGENTS.md`, `TESTING.md`, `tickets/BBD-WAL-007.md`, Green Resume 04 Rejection
01, the complete frozen `wallet-broker/tests/xmr_hygiene.rs`, and `CURRENT_TASK.md`.

Edit only `wallet-broker/src/xmr/test_support.rs`, starting at 4,774 lines and SHA-256
`055af3ba8b55cb68bd87c56cb23d6050aca6b24dba47cff4372f37cd634de17b`.
Freeze every other path, including all tests and the other six accepted sources.

Correct only `AuthorityRig::invoke_for_test` so it is callable through `&self`, as the
frozen negative-authority test requires. This method is a validation probe, not an
operation executor: return `Ok(())` only when `operation` exactly equals one of the
seven `phase_operations`; otherwise return `SCHEMA`. It must never record an operation,
increment side effects, touch typed transports, or return data. Keep actual effectful
behavior in the typed phase methods (`call`, creation flow, and `open`). Do not add
interior mutability, suppression, a generic/raw route, or a test-specific branch.

Do not run rustfmt, Cargo, compiler, tests, Clippy, binaries, Node/npm, network, Git, or
GitHub. Do not edit tests/governance/evidence or invoke another actor. Report the exact
changed path, resulting line count and SHA-256, semantic summary, and prohibited-action
compliance, then stop for reviewer inspection.
