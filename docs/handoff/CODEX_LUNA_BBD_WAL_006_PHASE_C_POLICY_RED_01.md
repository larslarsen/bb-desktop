# Codex Luna Handoff — BBD-WAL-006 Phase-C Policy Expected Red 01

You are **Jr Dev — Codex Luna**. This durable file is the complete prompt; ephemeral
chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff.

Accepted uncommitted test source:

- `test/securityPolicy.node.js`
- 2,401 lines
- SHA-256 `19b7948bfa2c7f9b29426133bdda1630abfade5f1c438c7367e5c6dacd32688b`

Read completely: `AGENTS.md`, `TESTING.md`,
`docs/engineering/DEVELOPMENT_ROLES.md`, `tickets/BBD-WAL-006.md`,
`docs/testing/BBD-WAL-006-EXPECTED-RED-01.md`, both Phase-C policy-test source reviews,
the original/correction Sol handoffs, `docs/handoff/CURRENT_TASK.md`, the complete
changed `test/securityPolicy.node.js`, and the current WAL-006 implementation in
`scripts/security-policy.js`.

## Preflight

Require `HEAD == origin/master` at the protected governance parent and a clean index.
The worktree must contain exactly one modified path, the accepted test above, with exact
line count/hash and no other untracked or modified file. Confirm that no
`wallet-broker/src/zec*` production path exists.

Do not repair or edit the test, policy implementation, Rust source/test, fixture,
manifest, lockfile, or any production path.

## Exact expected-red execution

From the repository root, run exactly:

```text
node test/securityPolicy.node.js
```

Expected result is exit 1. The output must retain exactly 66 `ok` lines, seven `not ok`
lines, and the final line `7 security policy test(s) failed`. The accepted seven failing
groups are the prior three WAL-004/workflow integration checks plus these four WAL-006
groups:

1. manifest exact pins/minimum feature union;
2. compiled upstream PCZT capability versus BitBook authority;
3. the newly named exact bounded Phase-C ZEC production inventory; and
4. rejection of live-network/authority-bearing Rust source.

The newly named inventory failure must arise from the still-absent Phase-C policy export
and source inventory, not syntax, fixture, module resolution, an exception outside that
test, or a weakened assertion. No prior `ok` case may regress. If the count/name/cause
differs, stop without evidence or Git action and report the complete discrepancy.

## Evidence and integration

If and only if the red is exact, create only
`docs/testing/BBD-WAL-006-PHASE-C-POLICY-EXPECTED-RED-01.md`. Record the protected
parent, Node version, command/status, exact `ok`/`not ok` counts and names, final line,
the inventory failure cause, absence of ZEC production paths, accepted test line/hash,
and final tracked/index state. Do not include canary or secret values.

Update only `docs/handoff/CURRENT_TASK.md` to state `PHASE-C POLICY EXPECTED RED
RECORDED — REVIEW REQUIRED`, link the evidence, and preserve that production/policy
implementation remain frozen.

Inspect the exact diff and stage only:

- `test/securityPolicy.node.js`
- `docs/testing/BBD-WAL-006-PHASE-C-POLICY-EXPECTED-RED-01.md`
- `docs/handoff/CURRENT_TASK.md`

Commit once as `test: define WAL-006 phase C source inventory` and push `master`.
Require `HEAD == origin/master` and a clean tracked worktree/index afterward. Report the
commit, evidence line count/hash, test line count/hash, exact red counts, staged path
manifest, and push/final-state result.

Do not run npm, Cargo, Rust, other Node tests, formatters, linters, builds, scanners,
Electron, wallets, nodes, devices, network clients, falsifications, or cleanup. Do not
edit the ticket or any unlisted path. Production remains frozen after integration.
