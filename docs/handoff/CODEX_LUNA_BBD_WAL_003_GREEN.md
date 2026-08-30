# Codex Luna Handoff — BBD-WAL-003 Local Green Integration

You are **Jr Dev — Codex Luna**, using `gpt-5.6-luna`. This is the complete durable
integration prompt; ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Governance baseline: `946739b6`

Reviewer-accepted uncommitted production paths:

- `wallet-broker/protocol.js` — 318 lines — `dac1689f2afab847301cb4ac0c9745a8d3a98de7f856764006a4b7f846f59a03`
- `wallet-broker/supervisor.js` — 393 lines — `0778470023150e1d198c99ef25fb403138ad0026822426939703f2264c99b8ab`
- `wallet-preload.js` — 68 lines — `3e6a18acf88dd5be4e6a88f326d6ace7a8071066480d9a70a2e8f89df035a1df`
- `social-main.js` — 229 lines — `42af3528f1afc10e9d6b2362489811a09695854159afba9f372443bfc69e672f`
- `package.json` — 36 lines — `2f1e2e6d221baf676dbdf0436d7c595f5976dad765234948da0d632250d8c47e`
- `.github/workflows/social.yml` — 140 lines — `4308b94dec1d0ed9575332a812f0f2b320af89b37d77461b61df1cabc3c324d3`
- `.github/workflows/security.yml` — 52 lines — `dd3edfcd4b40c6d41130f836a66c525480560584aa5dac72cc6a4a65ffe21e82`
- `scripts/security-policy.js` — 1,849 lines — `e9bdb4f927defee883e02eca2fb5a2ad6d263c518b12147e66ca92802c6a5e31`

Read the ticket, current task, production handoff and Correction 01, both expected-red
evidence files, and the accepted source/tests. Verify the exact eight line counts/hashes,
that they are the only dirty paths, and that `HEAD == origin/master == 946739b6...`.
Do not alter source or tests.

Run these targeted commands in order, stopping immediately on the first nonzero result or
wrong registered count:

```text
node test/walletBrokerProtocol.node.js
node test/walletSupervisor.node.js
node test/walletPreload.node.js
node test/electronSecurity.node.js
node test/securityPolicy.node.js
```

Expected: 11/11 protocol, 11/11 supervisor, 6/6 preload, 19/19 Electron, and 58/58
policy tests pass. No canary or inherited failure may appear.

Only if all targeted commands match, run these broader gates in order:

```text
npm test
npm run build
node scripts/security-policy.js
npm audit --audit-level=high
```

Every command must exit zero. Do not run Electron, packaging, cross-platform builds,
Gitleaks downloads, installs, formatters, or any other test/scanner command. On any
mismatch, stop and report the exact command, exit status, and raw failing test names or
audit finding without editing, committing, pushing, rerunning, or attempting a fix.

If and only if every command passes:

1. Create `docs/testing/BBD-WAL-003-LOCAL-GREEN.md` with timestamp, governance baseline,
   all eight hashes/line counts, exact command results/counts, no-canary statement, audit
   result, and confirmation that no production/test path changed during execution.
2. Update `docs/handoff/CURRENT_TASK.md` and `tickets/BBD-WAL-003.md` to state
   `LOCAL GREEN RECORDED — FALSIFICATION/CI REVIEW PENDING`, link the evidence, and
   preserve all prior history.
3. Reverify the eight source hashes, stage exactly those eight paths plus the new evidence,
   current task, and ticket, commit `feat: add fail-closed wallet broker boundary`, push,
   and prove a clean worktree with `HEAD == origin/master`.

Do not edit source, tests, fixtures, lockfile, architecture, prior evidence, other docs,
other workflows, or any other repository. Do not install, delete, clean, use root,
`sudo`, `rm`, `/tmp`, a real broker or wallet process, coin/node/network service,
hardware, or devices. Network is authorized only for the exact npm audit and Git push.
