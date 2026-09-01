# Hermes Handoff — BBD-WAL-005 Expected Red 01

State: STOPPED — TEST SOURCE SYNTAX ERROR

Reviewer governance parent before this handoff: `cf1768b7`

Accepted test-source identity:
`docs/testing/BBD-WAL-005-TEST-SOURCE-REVIEW-01.md`

## Authorized paths

Integrate the accepted bytes at exactly:

- `test/walletPay.node.js`
- `test/walletSupervisor.node.js`
- `test/electronSecurity.node.js`
- `test/securityPolicy.node.js`
- `test/fixtures/wallet-pay/snapshots-v1.json`

Create or edit only these records:

- `docs/testing/BBD-WAL-005-EXPECTED-RED-01.md`
- `docs/handoff/HERMES_BBD_WAL_005_EXPECTED_RED_01.md` (state line only)

No production, package, workflow, policy implementation, Rust, renderer, dependency,
lockfile, ticket, roadmap, current-task, or unlisted path may change.

## Required execution

1. Record `hermes --version` and the actual provider/model resolved for this run.
2. Verify the five exact hashes in the accepted review and confirm no unlisted source
   path is modified.
3. Run in order, continuing through all four expected-red commands:

```text
node test/walletPay.node.js
node test/walletSupervisor.node.js
node test/electronSecurity.node.js
node test/securityPolicy.node.js
```

Record exact exit codes and concise failure identities. Each must exit nonzero only
because the WAL-005 production module/wiring/policy is absent. A syntax error, fixture
parse error, inherited assertion regression unrelated to WAL-005, unexpected pass, hang,
resource leak, or changed hash is a stop.

4. Write `docs/testing/BBD-WAL-005-EXPECTED-RED-01.md` with the environment identity,
   hashes, command results, and path audit.
5. Change only this handoff's state line to `COMPLETE`.
6. Run `git diff --check`, stage only the seven authorized paths, commit exactly
   `test: reserve BBD-WAL-005 pay state`, and push `master` to `origin`.
7. Report the commit hash, push result, final `git status --short`, and exact command
   results. Do not authorize production.

If any stop condition occurs, do not commit or push. Record the stop only when its record
can be written without widening scope, then return control to the reviewer.
