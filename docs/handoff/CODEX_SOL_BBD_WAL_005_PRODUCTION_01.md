# Codex Sol Handoff — BBD-WAL-005 Production 01

State: COMPLETE — REVIEWER ACCEPTED

Parent: `d00ba98e2d3951cbadbb913e84dbbac11f4a443e`

Read `AGENTS.md`, `TESTING.md`, `tickets/BBD-WAL-005.md`, the accepted expected-red review,
the five frozen test/fixture paths, and the current production files needed for the drop.

Implement the exact closed Pay model and shared sanitizer reserved by the tests. Create or
edit only:

- `wallet-pay/model.js`
- `wallet-broker/supervisor.js`
- `social-main.js`
- `package.json`
- `scripts/security-policy.js`
- `.github/workflows/social.yml`
- `.github/workflows/security.yml`

Required integration details:

- `wallet-pay/model.js` has exactly the three ticket exports, no imports, I/O, authority,
  nondeterminism, or timer surface;
- supervisor and Electron main use that shared sanitizer, with no duplicate local wallet
  snapshot allowlist left behind;
- only a valid bound `sync.subscribe` event carrying exact `{ snapshot }` publishes a
  sanitized snapshot; malformed or other events do not create an unsanitized path;
- the existing broker method list, preload API, and Electron channel list do not grow;
- package and routine workflows add only the exact syntax/test/path entries reserved by
  the policy tests; and
- security policy exports and enforces the exact wallet-Pay paths, commands, zero-import
  model rule, and reviewed supervisor import without weakening an inherited check.

The accepted test and fixture bytes are frozen. Do not edit them. Do not edit preload,
renderer DOM/CSS, wallet contract, Rust, dependencies, lockfile, evidence, documentation,
or any unlisted path. Do not run tests, builds, formatters, package-manager, Git mutation,
GitHub, network, browser, device, Rust, or cleanup commands. Read-only inspection,
`git diff --check`, line counts, and SHA-256 are permitted.

Stop after reporting exact paths, line counts, hashes, implementation summary, and
confirmation that no test/build/Git operation ran.
