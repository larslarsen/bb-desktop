# BBD-WAL-011 Transport Expected Red 01

Jr Dev — Hermes execution/evidence only. Source actor closed.

## Hermes runtime identity

- Version: `Hermes Agent v0.18.2 (2026.7.7.2) · upstream 990473a7 · local 10b6d1a9 (+1 carried commit)`
- Provider: `nous`
- Model: `poolside/laguna-s-2.1:free`
|- Resolved provider/model confirmed from narrow session metadata (`~/.hermes/config.yaml`
  model/provider routing); not from full status/config output.
|- Session: `20260909_135906_d3d5a1`

## Baseline

- Baseline per ticket BBD-WAL-011: `8020a6d47201f804f5503abd3307092ae5a50c43`
- Reviewer publication (HEAD): `ccb472807ce2571edac51b49812ea8eb0db0b781`
- Reviewer publication 23a62f44 (`Review live transport tests and authorize focused expected red`)
- Protected parent c0ce49a4 (`Record wallet transport test actor launch`)
- Branch `master` ✓, `origin/master` upstream ✓

## Five-input hash verification (before and after run)

| Path | SHA-256 | Status |
| --- | --- | --- |
| wallet-broker/supervisor.js | 2634fd116f476998db2cf4a4e948fcd864397d0d26d3fd98e4c2a5e41d2f0430 | unchanged ✓ |
| wallet-broker/protocol.js | 79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4 | unchanged ✓ |
| test/walletSupervisorTransport.node.js | 284546977bd0a1a11ddc92a9fa743c06884e5157029759dd5003001891510b8b | unchanged ✓ |
| test/fixtures/wallet-broker/transport-child.js | 17cdea3eaec6c01f839eb34c86e25d1fd93f8cad165cd8e9fb7f108bb9fa7fef | unchanged ✓ |
| test/walletSupervisor.node.js | db02281f9314e382255945d569de4f83be8bc0edbc3ff62e5485f11e3d4e4992 | unchanged ✓ |

All five hashes matched before execution and after execution. No source edits made.

## Authorized command (run exactly once, with 45-second timeout wrapper)

```text
timeout 45 node test/walletSupervisorTransport.node.js
```

## Result

- Exit code: `1`
- Failing groups: `6`

## Exact six failing groups

1. `transport: real framed hello, ack, bootstrap and a subsequent request return a distinct fixture result`
   - Cause: `Error: missing real framed hello/ack/bootstrap connection` at Timeout (line 109). The primary live success case reports `missing real framed hello/ack/bootstrap connection` as expected by the handoff.

2. `transport: concurrent requests resolve from reverse-order coalesced replies and broker errors drop diagnostic sentinels`
   - Cause: `Error: missing real framed hello/ack/bootstrap connection` at Timeout (line 109).

3. `transport: wrong session or uncorrelated response closes and rejects pending work`
   - Cause: `Error: missing real framed hello/ack/bootstrap connection` at Timeout (line 109).

4. `transport: partial-frame EOF and malformed frames close without a hanging promise`
   - Cause: `AssertionError [ERR_ASSERTION]: partial hello never published broker-down` at line 479. This is a harness gap flagged in the source review; the test expected broker-down state from a real framed connection that does not exist yet.

5. `transport: handshake and request deadlines fail closed`
   - Cause: `AssertionError [ERR_ASSERTION]: stdin write must be a Buffer` at line 306 in `decodeWriteList`/`bindFake` (line 318). Harness gap: the fake transport binding does not emit real framed bytes, so deadline assertions cannot fire.

6. `transport: request limit does not write; quit and exit settle pending work and release the child`
   - Cause: `AssertionError [ERR_ASSERTION]: stdin write must be a Buffer` at line 306 in `decodeWriteList`/`bindFake` (line 318). Same harness gap as group 5.

## Confirmed scope

- The primary live success case (group 1) reports `missing real framed hello/ack/bootstrap connection` — exactly the expected red per the handoff.
- Groups 1–3 all fail on the same missing real framed connection, not on missing module, syntax error, fixture startup, or spawn permission.
- Groups 4–6 fail on harness-level assertions within the test's own fake-transport binding, consistent with the source review's noted harness gaps (no real framed hello/ack/bootstrap connection to drive partial-frame, deadline, and request-limit paths). The source review retains an emergency-fallback cleanup limitation: the finally-block may leave the child directory when the child remains alive after a reap timeout. Per-test cleanup is observed but is not asserted as general leak-free teardown by the red phase alone.
- No Rust builds, npm, network, or other suites were executed.
- The suite reached assertion completion and exited with code 1 within the 45-second timeout wrapper.
- The suite owns its temporary fixture directories and child cleanup; no stray processes or fixture directories were observed post-run.

## Post-run state

- `git status`: same five modified/untracked paths as pre-run (package-lock.json, package.json, scripts/security-policy.js, test/securityPolicy.node.js, test/walletSupervisor.node.js modified; docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md, test/fixtures/wallet-broker/transport-child.js, test/walletSupervisorTransport.node.js untracked). All unrelated pending files preserved.
- Five input hashes rechecked after run: unchanged.
- No Git stage/commit/push performed.
- No CURRENT_TASK.md edit performed.
- No source changes made.
