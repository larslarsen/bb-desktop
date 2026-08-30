# Codex Sol Handoff — BBD-WAL-003 Production Correction 01

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This is the complete
durable correction prompt; ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Governance baseline: `e089d8d4`

Read the production handoff, ticket, current task, accepted tests, and all eight current
production paths. The first source drop is reviewer-rejected before execution for one
real/mock integration mismatch:

- `social-main.js` intentionally calls `walletSupervisor.dispatch('status.get')` and
  `walletSupervisor.dispatch('account.list')` with no parameter, matching the accepted
  Electron test's fixed no-payload channels.
- `createWalletSupervisor()` currently exposes the strict internal dispatcher directly;
  that dispatcher requires `{}` for those broker protocol methods. Once a real session is
  bound, both renderer calls would fail `SCHEMA`, even though the Electron mock passes.

Modify only `wallet-broker/supervisor.js`. Preserve `createBrokerDispatcher`'s strict
closed broker schema. At the public supervisor boundary, normalize `undefined` to a fresh
empty object only for `status.get` and `account.list` before calling the strict internal
dispatcher. Do not normalize `sync.subscribe`, an unknown method, `null`, any nonempty
value, or any parameter-bearing method. Ensure the broker envelope still contains
`params: {}`. Preserve every other behavior and all seven other production paths byte for
byte.

Use `apply_patch`. Only read-only inspection and final `wc -l`/`sha256sum` for
`wallet-broker/supervisor.js` are allowed. Do not execute Node, npm, tests, builds,
formatters, scanners, Electron, Git, GitHub, network, child processes, wallets, nodes,
hardware, or devices. Do not install anything or use root, `sudo`, `/tmp`, deletion,
cleanup, `rm`, globs, or unresolved destructive targets.

Stop after authoring and report the exact change, line count, SHA-256, preserved strict
cases, and confirmation that nothing ran. Reviewer XHigh must accept the corrected source
before Codex Luna executes it.
