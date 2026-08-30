# Codex Luna Handoff — BBD-WAL-003 Correction 04 Policy Red

You are **Jr Dev — Codex Luna**, using `gpt-5.6-luna`. This is the complete durable
integration prompt; ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Governance baseline: `cb820311`

Reviewer-accepted uncommitted test correction:

- `test/securityPolicy.node.js` — 1,574 lines —
  `1414a32cb114b1467c9d39bbcbf02228aa185857b0ccc304cd4356be9a02507b`

Read the ticket, current task, Correction 04 Sol handoff, prior expected-red evidence, and
the corrected test. Verify the exact line count/hash and confirm it is the only dirty
path. Do not alter it. Then run only:

```text
node test/securityPolicy.node.js
```

Expected: exit 1 after all 58 registered tests run, with exactly 53 `ok` and 5 `not ok`.
The failures must be exactly:

1. `wallet contract package command and maintained-source policy are exact and fail closed`
2. `wallet broker boundary package scripts and syntax checks are exact`
3. `wallet broker and preload paths are required on every routine workflow trigger`
4. `routine CI executes the named wallet broker suite and rejects omission`
5. `wallet boundary source policy allows only reviewed built-ins and forbids listeners, shell, and generic IPC`

They must fail only because the current package/workflow/policy production wiring is
absent. No canary, inherited failure, unexpected pass, changed count, or other cause is
acceptable. On any mismatch, stop immediately and report raw result names without
editing, committing, pushing, rerunning, or attempting a fix.

If and only if the result matches:

1. Create `docs/testing/BBD-WAL-003-CORRECTION-04-EXPECTED-RED.md` with timestamp,
   governance baseline, corrected test hash/line count, exact command/exit status,
   53-pass/5-fail inventory, five failure names and causes, no-canary statement, and
   confirmation that no production path ran or changed.
2. Update `docs/handoff/CURRENT_TASK.md` and `tickets/BBD-WAL-003.md` to state
   `CORRECTION 04 EXPECTED RED RECORDED — PRODUCTION NOT AUTHORIZED`, linking the new
   evidence and preserving all prior evidence/history.
3. Stage exactly `test/securityPolicy.node.js`, the new evidence, current task, and the
   ticket. Commit `test: correct wallet broker policy contract`, push, and prove a clean
   worktree with `HEAD == origin/master`.

Do not run any other Node/npm/test/build/format/scanner/Electron command. Do not edit
production, package, workflow, policy implementation, other tests, fixture, prior
evidence, or any other repository. Do not install, delete, clean, use root, `sudo`, `rm`,
`/tmp`, network beyond the authorized Git push, child processes other than the one Node
test process, wallets, nodes, hardware, or devices.
