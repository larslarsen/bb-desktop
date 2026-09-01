# Hermes Handoff — BBD-RATE-001 Expected Red 01

State: COMPLETE

Reviewer governance parent before this handoff: `c8eefb55`

Accepted test-source identity:
`docs/testing/BBD-RATE-001-TEST-SOURCE-REVIEW-03.md`

## Authorized paths

Integrate the accepted bytes at exactly:

- `test/rateWorker.node.js`
- `test/rateSupervisor.node.js`
- `test/securityPolicy.node.js`
- `test/fixtures/rates/provider-bodies-v1.json`

Create or edit only these records:

- `docs/testing/BBD-RATE-001-EXPECTED-RED-01.md`
- `docs/handoff/HERMES_BBD_RATE_001_EXPECTED_RED_01.md` (state line only)

No production, package, workflow, policy implementation, wallet, Pay, broker, preload,
renderer, Rust, dependency, lockfile, ticket, current-task, or unlisted path may change.

## Required execution

1. Record `hermes --version` and the actual provider/model resolved for this run.
2. Verify the four exact hashes in the accepted review and confirm no unlisted source
   path is modified.
3. Run in order, continuing through all three expected-red commands:

```text
node test/rateWorker.node.js
node test/rateSupervisor.node.js
node test/securityPolicy.node.js
```

Expected results:

- worker exits nonzero only because `quote-worker/providers.js` and the remaining reviewed
  quote-worker modules do not exist yet;
- supervisor exits nonzero only because `quote-worker/supervisor.js` and its reviewed
  sibling modules do not exist yet; and
- policy exits nonzero only in the four new RATE-001 cases because the package, workflow,
  checker exports, maintained quote-worker paths, and exact provider-pin policy are absent.
  Every inherited policy case must remain green.

A syntax/fixture error, unrelated inherited failure, unexpected pass, hang, live provider
access, resource leak, changed hash, or different source path is a stop.

4. Write `docs/testing/BBD-RATE-001-EXPECTED-RED-01.md` with environment identity, hashes,
   exact exit codes/failure identities, the inherited policy pass count, and path audit.
5. Change only this handoff's state line to `COMPLETE`.
6. Run `git diff --check`, stage only the six authorized source/record paths, commit exactly
   `test: reserve BBD-RATE-001 quote worker`, and push `master` to `origin`.
7. Report commit hash, push result, final `git status --short`, and exact command outcomes.
   Do not authorize or begin production.

If a stop condition occurs, do not commit or push. Record the stop only when that record
can be written without widening scope, then return control to the reviewer.
