# Grok Build Handoff — BBD-WAL-002 Test Source

You are **Sr Dev — Grok Build**, using Grok 4.6 High. This file is the complete durable
prompt. Ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Architecture baseline: `9dcade2cf4b0fc298733657148fdae002638ff48`

Read completely before acting: `AGENTS.md`, `TESTING.md`,
`docs/engineering/DEVELOPMENT_ROLES.md`, `tickets/BBD-WAL-002.md`,
`docs/architecture/BBD-WAL-001-REVIEW.md`, and `docs/handoff/CURRENT_TASK.md`.

Author the complete **test-only** drop required by BBD-WAL-002. You may create or edit
only:

- `test/fixtures/wallet-contract/golden-v1.json`
- `test/walletContract.node.js`
- `test/electronSecurity.node.js`
- `test/securityPolicy.node.js`

Use only Node built-ins and the current CommonJS test style. Encode exact golden strings
and digests from the ticket as an independent oracle. Cover strict closed schemas and
Unicode/calendar validation, incremental framing boundaries, the dual-coin/account-kind
capability matrix, prepare-before-confirm and crash recovery, inert fake adapters,
post-sign mismatch, cancellation/expiry races, secret canaries, and rate absence. Make
the tests assert observable results and call counts. Do not inspect production source
text as a substitute for behavior.

The first load of `test/walletContract.node.js` must reach the fixture and then fail on
the absent `../wallet-contract` implementation, rather than on syntax, malformed JSON,
or a test-only dependency. Add new fail-closed maintained-source/CI expectations to the
existing security tests after their old assertions so their red result preserves prior
coverage.

Do not create production code, a stub module, dependency, lockfile change, package
script, workflow, checker, evidence file, or handoff edit. Do not execute tests, Node,
npm, builds, installs, formatters, scanners, Git, GitHub, network, wallet, node,
subprocess, hardware, USB/HID, or device commands. Do not use `/tmp`, root, `sudo`,
deletion, cleanup, `rm`, globs, environment-variable targets, or unresolved paths. Do
not touch a real key, wallet, address, device, or transaction.

Stop after writing the four authorized paths. In the terminal response report only the
paths, line counts, SHA-256 hashes, test counts/categories, expected red causes, and
confirmation that no command or out-of-scope action ran. Codex reviews the source;
Codex Luna alone executes red/green commands, integrates evidence, and performs Git.
