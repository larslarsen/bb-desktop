# Codex Luna Handoff — BBD-WAL-003 Expected Red Resume 01

You are **Jr Dev — Codex Luna**, using `gpt-5.6-luna`. This is the complete durable resume
prompt; ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

The first three missing-module red commands matched. The Electron command emitted all 19
registered results and the exact inventory is reviewer-adjudicated as **13 pass / 6 fail**.
The earlier completion report's “9 pass” statement was a counting mistake; the raw list
contains 13 `ok` lines. The six failures are exactly the two converted preload/IPC
expectations plus the four new integration behaviors that cannot pass before production.
No inherited sandbox, navigation, permission, CSP, injection, or wallet-contract behavior
failed. The Electron red is accepted; do not rerun it.

Read the original red handoff and `docs/handoff/CURRENT_TASK.md`. Verify the same six
accepted hashes and that no path changed since the stopped run. Then run only:

```text
node test/securityPolicy.node.js
```

Expected: exit 1 with exactly 54 passing and 4 failing tests; all four new boundary tests
must fail only because package, workflow, CI, and source-policy production enforcement is
absent. Stop on any other count, cause, canary output, or unexpected pass.

If and only if it matches, complete steps 5–6 of
`docs/handoff/CODEX_LUNA_BBD_WAL_003_RED.md`: write the exact combined five-command
evidence, update only current task to `EXPECTED RED RECORDED — PRODUCTION NOT AUTHORIZED`,
stage exactly the six accepted test paths plus the evidence and current-task paths, commit
`test: define wallet broker boundary`, push, and prove clean `HEAD == origin/master`.

All original restrictions remain. No source edit, production, broader test, npm, build,
scanner, Electron rerun, other repository, or other action is authorized.
