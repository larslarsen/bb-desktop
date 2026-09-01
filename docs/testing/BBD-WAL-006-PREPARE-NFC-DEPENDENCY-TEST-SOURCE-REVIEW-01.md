# BBD-WAL-006 Prepare NFC Dependency Test Source Review 01

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Protected governance parent: `dca7682a`

Result: **ACCEPTED — FOCUSED EXPECTED RED AUTHORIZED**

The only source change is `test/securityPolicy.node.js`, now 2,525 lines with SHA-256
`a70d3491c359b8d59a3b7cdb1307ee549b9c0cf8dad310570a47157d066d68ba` and 75 named tests.

The new independent test freezes `WAL006_PREPARE_DEPENDENCIES` to exactly
`unicode-normalization = 0.1.25`, defaults off, `std` only, non-optional. It first requires the
exact manifest declaration exactly once, before loading or comparing the future policy export, so
the current missing declaration is a specific non-tautological red.

After that prerequisite, it requires the exact future export, checks the complete real manifest,
and independently mutates loose versioning, enabled defaults, removed/widened features, optional
mode, git source, path source, and an aliased second implementation. The six maintained Zcash
dependencies, two existing support dependencies, manifest inventory, and prior tests remain
unchanged.

No manifest, lockfile, policy implementation, Rust source/test, fixture, workflow, or other path
changed. Source inspection finds no assertion weakening or blocker. Hermes may run only the
focused Node policy command and integrate the test-source drop with exact expected-red evidence.
Prepare dependency production and all Rust prepare production remain frozen.

