# BBD-RATE-001 Production-Source Review 01

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Governance HEAD at review: `755c406c`

Result: **PRODUCTION SOURCE REJECTED BEFORE EXECUTION — FOUR-MODULE CORRECTION REQUIRED**

## Reviewed uncommitted identities

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `quote-worker/providers.js` | 50 | `e473fb6d32f6dcaa19f8f5825ef47c0a63ca068767f0b75960a2c65d9102470e` |
| `quote-worker/model.js` | 474 | `0b49ba1199fc823f264d9358b405be4ffc03f1f8b7409b723c9b342bb7aeb925` |
| `quote-worker/framing.js` | 403 | `453603fd9c33585b822c2912df9a8163bda0951ef58dee4ea43b3f45cf20cbaf` |
| `quote-worker/worker.js` | 326 | `585f0dc3944c1e1e79395ae0560a906b2365281efb1ff2295ceab90ee715d64e` |
| `quote-worker/supervisor.js` | 210 | `03c551a0869d5d6be7a5086adbe02ee285a8817b9724fae8deb59985356baa8a` |
| `package.json` | 38 | `f8b13d53e80c8f91c87a473e3c873999a337078f8eae90779814ac368a10197a` |
| `scripts/security-policy.js` | 2,667 | `f66f6df408d434082b14b8e8a5e1bb61722a7f5bc09c97c7a5e224793b301e7e` |
| `.github/workflows/social.yml` | 153 | `5968dc31bbc72bfc010417381a3b6f83df1f1fa6abf9f71275b007b8254dc9b2` |
| `.github/workflows/security.yml` | 61 | `9b890179bcb5b8ade9503a43ec97c18ed3bca0ab4e2d7e1f0ebcec495225be4e` |

Only the nine authorized paths differ from governance HEAD, `git diff --check` passes,
and all four committed test/fixture hashes remain frozen. The reviewer executed no Node,
npm, test, workflow, provider, or network command.

## Accepted and frozen from this drop

`providers.js`, `package.json`, `scripts/security-policy.js`, and both workflows match the
closed Coinbase-ZEC/Kraken-XMR, default-off, no-dependency, exact script/build/CI/path,
source-policy, and renderer-isolation contract. Those five paths are frozen at the hashes
above during correction.

## Blocking source findings

1. `worker.js` buffers stdin and processes frames only on EOF. A real supervisor writes a
   request while keeping the anonymous pipe open, so the child never answers. The existing
   subprocess test sends EOF and therefore does not expose the deadlock.
2. `supervisor.js` clears `pendingId` immediately when a response is not delivered inside
   the synchronous fake `stdin.write`. A real child response arrives later; `onStdout`
   then classifies it as unsolicited and kills the child. There is no asynchronous result
   cache, so the production path cannot ever return a live quote.
3. The supervisor does not subscribe to child `error`, `exit`, or `close`. A crashed child
   can remain as an apparently usable handle, with pending state and later writes handled
   inconsistently.
4. Framing validates quote rows and display fields separately but not their relationship.
   A compromised child can return empty quotes with a fresh display, a display price/source
   different from its quote, or provider timestamps whose five-minute bounds are forged.
   The supervisor forwards the validated result without rebuilding it from normalized rows.
5. Ordinary request/response failures can be delivered more than once without a settled
   guard, causing repeated backoff/timers. Non-200 and wrong-content-type responses are not
   drained or destroyed, leaving avoidable response/socket lifetime risk.
6. Both strict JSON parsers assign object keys through ordinary bracket assignment. A
   provider or frame key named `__proto__` therefore invokes the legacy prototype setter
   instead of creating an inert parsed data property, violating the closed-data boundary.

The correction is limited to `model.js`, `framing.js`, `worker.js`, and `supervisor.js`
under `docs/handoff/GROK_BUILD_BBD_RATE_001_PRODUCTION_CORRECTION_01.md`. Tests remain
frozen; Hermes execution remains unauthorized.
