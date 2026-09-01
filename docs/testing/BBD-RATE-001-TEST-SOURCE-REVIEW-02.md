# BBD-RATE-001 Test-Source Review 02

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Governance HEAD at review: `dd4ac204`

Result: **CORRECTION 01 SUBSTANTIALLY ACCEPTED; TWO-SUITE ORACLE CORRECTION REQUIRED**

## Corrected uncommitted identities

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `test/rateWorker.node.js` | 1,108 | `f15a6021e9c169bdc272ed337d6a7fc704d5ae2c37ae32ce430b62dc56ba37c1` |
| `test/rateSupervisor.node.js` | 545 | `1e569289917eed1838dc6e5bcb43957cb5c72ed3fe0992ecef8db9e178826707` |
| `test/securityPolicy.node.js` | 2,844 | `1bc23cc41fd5c50b855637d80c342e0de3cc9ed8cd48219163f626b06e4391f6` |
| `test/fixtures/rates/provider-bodies-v1.json` | 263 | `7179969a17b92c02115ab6224266609c5a29af51c486734c6e321393da52a15b` |

Correction 01 fixed the request/response validator contradiction, amount/exponent oracle,
backoff clock, quote omission, decimal/timestamp boundaries, independent supervisor
failure cases, and exact provider-policy pins. The fixture and policy correction are
accepted and frozen. The reviewer executed no Node, npm, or test command.

Three connected worker/supervisor oracle defects still block expected red:

1. Supervisor failure cases inject canaries into the fake child's raw stderr, preserve
   those bytes in `child.stderrData`, and then `assertKilledUnavailable` requires that same
   raw capture not contain the canary. The test contradicts its own fixture. The boundary
   to prove is that raw child diagnostics never reach supervisor logs, snapshots, or a
   renderer-facing result.
2. The worker pin helper requires both 5,000 and 10,000 milliseconds on the HTTPS request.
   The contract has separate controls: a request/socket connect timeout and an injected
   overall timer. Calling `req.setTimeout` twice replaces rather than independently proves
   those controls. The overall timer is never fired. The same table also requires request
   abort/destroy after already-completed TLS/status/content-type failures, which is not a
   contract requirement.
3. The test titled as the child entry calls `createQuoteWorker` directly with fake streams.
   It never executes the `require.main === module` path and requires an undocumented
   `worker.exitCode`. The corrected backoff test also no longer proves the required
   one-request-per-provider in-flight limit.

Only `test/rateWorker.node.js` and `test/rateSupervisor.node.js` may change under
`docs/handoff/GROK_BUILD_BBD_RATE_001_TESTS_CORRECTION_02.md`.
