# Codex Luna Handoff — BBD-WAL-002 Correction 06 Expected Red

You are **Jr Dev — Codex Luna**. This is the complete durable handoff; ephemeral chat is
not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Read `AGENTS.md`, `TESTING.md`, `docs/engineering/DEVELOPMENT_ROLES.md`,
`tickets/BBD-WAL-002.md`, `docs/handoff/CURRENT_TASK.md`, Correction 06, Production
Correction 01, and the current wallet test/production source completely.

Reviewer XHigh accepts `test/walletContract.node.js` at 1,803 lines and SHA-256
`43830b1caec19904d23b400974c77c1edbebe32b4927b2f31ee4279611a46dbf`, containing 48
tests. Protected hashes remain the values in the prior correction-red handoff/evidence.

1. Verify `HEAD == origin/master`, record the full authorization baseline, inspect status,
   and verify wallet test, fixture, both security tests, and lockfile hashes.
2. Run exactly one foreground command: `node test/walletContract.node.js`.
3. Accept only exit 1 with all 45 previously accepted tests `ok` and exactly these three
   appended tests `not ok` for the intended Production Correction 01 gaps:

   - `recovery authority: durable signed_unverified restore requires crash recovery and fresh confirmation`
   - `capabilities: watch-only receive requires exact synthetic consensus compatibility`
   - `exceptions: untrusted dependency error codes normalize without leaking or retaining locks`

   No secret-canary value may appear in output.
4. If and only if exact expected red occurs, create with `apply_patch` only
   `docs/testing/BBD-WAL-002-CORRECTION-06-RED-EVIDENCE.md`. Record baseline/protected
   hashes, command/exit, 45-pass/3-fail counts, exact names and concise first causes,
   canary absence, and unchanged scope.
5. Run `git diff --check`; measure/hash the evidence; stage it alone; commit with
   `Record WAL-002 correction 06 expected red`; push `master`; verify the evidence-only
   commit and `HEAD == origin/master`.

Stop without an evidence commit if any protected hash differs, old test fails, new test
passes, cause is unintended, or a canary appears. Do not edit/stage/commit any test,
production, package, workflow, policy, fixture, governance, or other path. Do not run
security tests, npm, builds, installs, formatters, scanners, audit, falsification,
packaging, SBOM, Electron, network, wallet, daemon, hardware, or devices. No background
execution, root, `sudo`, `/tmp`, deletion, cleanup, `rm`, globs, unresolved paths, or
environment-variable targets.

Stop after push and report exact results, evidence line count/SHA-256, commit/full hash,
push/final baseline, protected hashes, and evidence-only scope. Reviewer XHigh must
accept this red before production changes resume.
