# Hermes Handoff — BBD-WAL-005 Expected Red Resume 01

State: COMPLETE

Correction governance parent before this handoff: `cc3077dd`

Accepted corrected test-source identity:
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
- `docs/handoff/HERMES_BBD_WAL_005_EXPECTED_RED_RESUME_01.md` (state line only)

No production, package, workflow, policy implementation, Rust, renderer, dependency,
lockfile, ticket, roadmap, current-task, stopped handoff, or unlisted path may change.

## Required execution

1. Record `hermes --version`, actual resolved provider/model, Node version, and actual
   `git rev-parse HEAD`. Replace the stopped record's incorrect governance-HEAD value.
2. Verify all five current hashes in the corrected accepted review and confirm no
   unlisted source path is modified.
3. Run in order, continuing through all four expected-red commands:

```text
node test/walletPay.node.js
node test/walletSupervisor.node.js
node test/electronSecurity.node.js
node test/securityPolicy.node.js
```

Record exact exit codes, passing-test counts before the expected failures when available,
and concise failure identities. Each must exit nonzero only because the WAL-005 production
module/wiring/policy is absent. A syntax/fixture error, inherited assertion regression
unrelated to WAL-005, unexpected pass, hang, resource leak, or changed hash is a stop.

4. Replace `docs/testing/BBD-WAL-005-EXPECTED-RED-01.md` with the complete resumed record.
5. Change only this resume handoff's state line to `COMPLETE`.
6. Run `git diff --check`, stage only the seven authorized paths, commit exactly
   `test: reserve BBD-WAL-005 pay state`, and push `master` to `origin`.
7. Report the commit hash, push result, final `git status --short`, and exact command
   results. Do not authorize production.

If any stop condition occurs, do not commit or push. Record the stop only when its record
can be written without widening scope, then return control to the reviewer.
