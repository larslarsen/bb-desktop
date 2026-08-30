# Codex Luna Handoff — BBD-WAL-002 Correction Expected Red

You are **Jr Dev — Codex Luna**. This file is the complete durable handoff; ephemeral
chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Read completely before acting: `AGENTS.md`, `TESTING.md`,
`docs/engineering/DEVELOPMENT_ROLES.md`, `tickets/BBD-WAL-002.md`,
`docs/architecture/BBD-WAL-001-REVIEW.md`, `docs/handoff/CURRENT_TASK.md`,
`docs/handoff/CODEX_SOL_BBD_WAL_002_TESTS_CORRECTION_04.md`,
`docs/handoff/CODEX_SOL_BBD_WAL_002_TESTS_CORRECTION_05.md`, and the current wallet
test and production source.

Reviewer XHigh accepts the correction test source for expected-red execution at:

- `test/walletContract.node.js`: 1,697 lines, SHA-256
  `3e51281d16da7eec4a178eeb799ec23e2854206a096ed741cba920fc35825ee9`

The protected source remains:

- fixture SHA-256 `08905d2b082fa6370211ebd494130d62e3696db0f8314636f3685ba5887f929f`;
- Electron security test SHA-256
  `3c18292790642ec3430b3ca1717657603da1a4e41c46b6c7812184dfb978939a`;
- security-policy test SHA-256
  `79b9286be5a679ba40baa5171417ab13bd19f0a2d8a56029398db8b083713204`;
  and
- `package-lock.json` SHA-256
  `7d347693664abe2639dfd1cd04f5f7e8757adb2dd71098b0a23b49aedade5413`.

## Authorized execution

1. Verify `HEAD`, `origin/master`, worktree status, and every hash above. `HEAD` must
   equal `origin/master`, contain this authorization handoff, and be recorded in full.
2. Run exactly this one foreground command from the repository root:

   ```text
   node test/walletContract.node.js
   ```

3. Accept only the intended correction red: all 38 previously accepted wallet tests are
   `ok`; the seven appended tests are `not ok`; the command exits nonzero; and the first
   failure in each appended test is attributable to the current unaccepted production
   gaps named below, not syntax, fixtures, imports, test setup, dependencies, or an
   inherited regression.

Expected failing test names:

- `binding: prepared reviews are recomputed and bound to the selected request and account`
- `capabilities: account, signer, adapter, and exact synthetic protocol pins cannot be substituted`
- `recovery locking: crash_recovery retains ownership and restored confirmation acquires it`
- `recovery terminal: cancellation and expiry release crash_recovery account locks`
- `recovery restart: repeated recovery crash is inert and durable verified restore cannot broadcast`
- `exceptions: injected status, prepare, signer, verify, and broadcast throws return closed failures`
- `secrets: sanitization validates allowlisted values without invoking accessors`

4. If and only if that exact expected red is observed, create
   `docs/testing/BBD-WAL-002-CORRECTION-RED-EVIDENCE.md` using `apply_patch`. Record the
   baseline and protected hashes, exact command/exit status, 38-pass/7-fail counts, all
   seven failure names with concise observed first causes, confirmation that no secret
   canary value appeared in command output, and unchanged worktree scope.
5. Run `git diff --check`, verify the evidence line count/SHA-256, stage that evidence
   path only, commit it with message `Record WAL-002 correction expected red`, and push
   `master`. Verify the evidence-only commit and `HEAD == origin/master`.

## Stop conditions and exclusions

If any old test fails, any new test unexpectedly passes, a hash differs, output exposes a
secret canary, or the failure cause is not the intended production gap, stop without an
evidence commit and report the blocker.

Do not edit, stage, commit, or push tests, fixtures, production/wiring, package/lock
files, workflows, policy, tickets, handoffs, or any path other than the named evidence
file. Do not run security tests, npm, builds, installs, formatters, scanners, audit,
falsifications, packaging, SBOM, Electron, network, wallet, daemon, hardware, or device
commands. Do not use root, `sudo`, `/tmp`, deletion, cleanup, `rm`, globs, unresolved
paths, or background execution.

Stop after the evidence-only push. Report command output counts/exit code, evidence path
with line count/SHA-256, commit/full hash and push result, protected hashes, final status,
and confirmation that only the evidence file was committed. Reviewer XHigh must accept
the red evidence before any production correction.
