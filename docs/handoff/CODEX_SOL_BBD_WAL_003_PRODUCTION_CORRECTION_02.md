# Codex Sol Handoff — BBD-WAL-003 Production Correction 02

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This is the complete
durable correction prompt; ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Governance baseline: `ac357be787701f5688f61eb69a3e76f377363887`

Read the production handoff, both production corrections, green handoff, ticket, current
task, accepted tests, and the three authorized source paths. Luna stopped on the first
targeted command: protocol produced 10 pass / 1 fail at the handshake boundary. Static
adjudication identifies the failing invalid row as numeric `child_pid: 41002`:
`RegExp.prototype.test` coerces it to a string, so the current PID regex accepts it.

Modify only:

- `wallet-broker/protocol.js`
- `wallet-broker/supervisor.js`
- `social-main.js`

Add explicit primitive string checks before every security-sensitive regex validation in
these paths where a PID, nonce, session ID, frame ID, method, cancel target, account ID,
intent ID, request ID, pin, or snapshot account ID is required to be a string. Do not
invoke `toString`, accessors, proxies intentionally, or otherwise coerce hostile values.
Preserve the accepted positive schemas, no-payload normalization from Correction 01,
error codes, channel mapping, sanitizer output, and all other behavior. Do not alter a
regex, accept a wider syntax, refactor unrelated code, or edit any other production/test/
fixture/package/workflow/policy/documentation path.

Use `apply_patch`. Only read-only inspection and final `wc -l`/`sha256sum` for the three
authorized paths are allowed. Do not execute Node, npm, tests, builds, formatters,
scanners, Electron, Git, GitHub, network, child processes, wallets, nodes, hardware, or
devices. Do not install anything or use root, `sudo`, `/tmp`, deletion, cleanup, `rm`,
globs, or unresolved destructive targets.

Stop after authoring and report every guarded field/site, the three line counts/hashes,
preserved behavior, and confirmation that nothing ran. Reviewer XHigh must accept the
corrected source before Codex Luna resumes from the first targeted command.
