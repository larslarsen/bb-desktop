# Grok Build Handoff — BBD-WAL-005 Test Source 01

State: ACTIVE

Parent: `14a6818740a1da0641f88a1f91dea3346554dddd`

Read `AGENTS.md`, `TESTING.md`, `tickets/BBD-WAL-005.md`, and the architecture sections
named by the ticket before editing.

Author the complete Phase A test source first. Edit only:

- `test/walletPay.node.js`
- `test/walletSupervisor.node.js`
- `test/electronSecurity.node.js`
- `test/securityPolicy.node.js`
- `test/fixtures/wallet-pay/snapshots-v1.json`

Do not edit production source or any unlisted path. Do not run tests, formatters, builds,
Git, GitHub, package-manager, network, browser, device, Rust, or cleanup commands.

The tests must reserve the exact closed module, sanitizer, Pay-view, payee-parameter,
preview, Electron-boundary, and policy behavior in the ticket. Prefer table-driven cases
and independent expected objects. Do not duplicate production algorithms in an oracle,
weaken an inherited assertion, or accept a self-reported eligibility boolean without
checking the underlying sanitized account fields.

Stop after reporting:

- exact changed paths;
- line counts and SHA-256 for each path;
- the high-value cases covered; and
- confirmation that no command or Git operation was run.

