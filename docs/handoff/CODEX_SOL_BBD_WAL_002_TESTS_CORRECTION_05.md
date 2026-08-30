# Codex Sol Handoff — BBD-WAL-002 Test Source Correction 05

You are temporary **Sr Dev — Codex Sol**, using `gpt-5.6-sol` at High. This is a narrow
follow-up to Correction 04. Ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Read `AGENTS.md`, `TESTING.md`, `docs/engineering/DEVELOPMENT_ROLES.md`,
`tickets/BBD-WAL-002.md`, `docs/handoff/CURRENT_TASK.md`,
`docs/handoff/CODEX_SOL_BBD_WAL_002_TESTS_CORRECTION_04.md`, and the current
`test/walletContract.node.js` completely.

Correction 04 is bounded and mostly sound but is not yet accepted. Modify only:

- `test/walletContract.node.js`

Preserve all existing coverage and correct exactly these two non-vacuity gaps:

1. In `recovery restart: repeated recovery crash is inert and durable verified restore
   cannot broadcast`, require the second `crash()` while already in `crash_recovery` to
   succeed as an inert durable operation (`ok: true`) and prove the complete snapshot is
   byte-for-byte/deeply unchanged. This must catch an implementation that returns
   `SCHEMA`, injects an error code, or merely leaves the state string unchanged.
2. In `exceptions: injected status, prepare, signer, verify, and broadcast throws return
   closed failures`, after the thrown-broadcast failure, construct another lifecycle with
   the same adapter and account ID and prove it can begin and prepare, then cancel it.
   This behaviorally proves the failed broadcaster released the process-local account
   lock. The overridden broadcast function need not be invoked by that cleanup machine.

Do not change production, fixtures, security tests, dependencies, package/lock files,
workflows, policy, evidence, documentation, or any other path. Do not weaken or rename
the seven Correction 04 tests.

Use `apply_patch`. Read-only inspection and `wc -l`/`sha256sum` for the one authorized
path are allowed. Do not execute tests, Node, npm, builds, formatters, scanners, Git,
GitHub, network, wallet, daemon, subprocess, hardware, USB/HID, or device commands. Do
not use `/tmp`, root, `sudo`, deletion, cleanup, `rm`, globs, environment-variable
targets, or unresolved paths.

Stop after authoring and report the new line count/SHA-256, exact assertions added, all
preserved test counts/names, unchanged protected paths, and that nothing ran. Reviewer
Codex at XHigh must inspect the result before Luna executes expected red.
