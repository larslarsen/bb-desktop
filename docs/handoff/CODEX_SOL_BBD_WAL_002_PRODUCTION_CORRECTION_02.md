# Codex Sol Handoff — BBD-WAL-002 Production Correction 02

You are temporary **Sr Dev — Codex Sol**, using `gpt-5.6-sol` at High. This is the
complete durable handoff; ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Read `AGENTS.md`, `TESTING.md`, `docs/engineering/DEVELOPMENT_ROLES.md`,
`tickets/BBD-WAL-002.md`, `docs/architecture/BBD-WAL-001-REVIEW.md`,
`docs/handoff/CURRENT_TASK.md`, Test Correction 06, Production Correction 01, the
Correction 06 red handoff/evidence, and current accepted tests/production completely.

Correction 06 expected red is accepted at evidence commit
`12061f5c1ab24a90dda9f7e74c846b0174d0e039`. Modify only:

- `wallet-contract/model.js`
- `wallet-contract/state-machine.js`

Do not edit `fakes.js`, other wallet modules, tests/fixtures, package/lock files,
workflows, policy, evidence, tickets, handoffs, documentation, or any other path.

Implement exactly:

1. In the capability model, validate the exact synthetic ZEC/XMR consensus branch
   before returning a watch-only receive outcome. A branch mismatch returns
   `PROTOCOL_INCOMPATIBLE` with both booleans false. With the exact branch, preserve
   receive true/spend false/`WATCH_ONLY`. Do not make spend-only PCZT/transaction flags
   prerequisites for watch-only receiving.
2. Mark synthetic restored `signed_unverified` and restored `verified` states as requiring
   recovery authority. While a restored signed-unverified state has that gate, direct
   `verifySigned()` returns a closed failure, remains `signed_unverified`, and makes zero
   adapter verify/broadcast calls. `crash()` must still enter `crash_recovery`; only a
   successful fresh `confirmRecovery()` clears the gate. Normal in-process signed and
   verified paths remain unchanged.
3. Define one explicit stable error-code allowlist for state-machine dependency results.
   Preserve known codes. Normalize every arbitrary/non-string adapter prepare/verify/
   broadcast or signer result code to `UNAVAILABLE` (retaining the existing semantic
   fallback such as `INTENT_MISMATCH` when no verify result/code exists). Never reflect
   an untrusted string into a failure or snapshot. Exceptions remain normalized and
   locks remain released on terminal failure.

Preserve every Production Correction 01 property, all inert fake behavior, and the
protected hashes. The accepted wallet test is 1,803 lines at SHA-256
`43830b1caec19904d23b400974c77c1edbebe32b4927b2f31ee4279611a46dbf` and contains 48
tests.

Use `apply_patch`. Read-only inspection and `wc -l`/`sha256sum` over authorized/protected
paths are allowed. Do not execute tests, Node, npm, builds, formatters, scanners, Git,
GitHub, network, wallet, daemon, subprocess, hardware, USB/HID, or device commands. No
`/tmp`, root, `sudo`, deletion, cleanup, `rm`, globs, environment-variable targets, or
unresolved paths.

Stop after authoring. Report both path line counts/SHA-256, exact logic changes, protected
hashes, unchanged scope, and confirmation that nothing ran. Reviewer XHigh must inspect
before Luna may run green.
