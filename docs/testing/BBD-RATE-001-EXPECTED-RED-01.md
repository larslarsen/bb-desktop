# BBD-RATE-001 Expected Red 01

State: COMPLETE

Integration actor: Jr Dev — Hermes

## Environment identity

Hermes Agent v0.18.2 (2026.7.7.2) · upstream f709bd88 · local 10b6d1a9
Provider: meituan/longcat-2.0:free
Node.js v22.23.1

## Accepted source identity (verified)

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `test/rateWorker.node.js` | 1,156 | `242de6e756e5d4db27d21fda16aedd2d7b7b9183d39a218ffe22c55a949736f8` |
| `test/rateSupervisor.node.js` | 549 | `8f3938cc46a86a893760d92a07da9a30b8d11f713514b89d2f8d77d42c866d80` |
| `test/securityPolicy.node.js` | 2,844 | `1bc23cc41fd5c50b855637d80c342e0de3cc9ed8cd48219163f626b06e4391f6` |
| `test/fixtures/rates/provider-bodies-v1.json` | 263 | `7179969a17b92c02115ab6224266609c5a29af51c486734c6e321393da52a15b` |

All four hashes match the accepted review. No unlisted source path is modified.

## Command execution

### node test/rateWorker.node.js

Exit: 1
Failure: `Cannot find module '../quote-worker/providers'` — expected, production module absent.

### node test/rateSupervisor.node.js

Exit: 1
Failure: `Cannot find module '../quote-worker/supervisor'` — expected, production module absent.

### node test/securityPolicy.node.js

Exit: 1
Inherited policy cases: 78 passed
RATE-001 failures (4, all expected):
1. `RATE-001 quote-worker package, syntax, top-level, and routine CI commands are exact` — expected `node test/rateWorker.node.js && node test/rateSupervisor.node.js`, got `undefined`
2. `RATE-001 quote-worker paths trigger routine workflows and remain policy-maintained` — `policy.SOCIAL_PATHS` does not include `RATE_SOURCE_FILTER`
3. `RATE-001 source policy permits only reviewed built-ins and forbids wallet, Electron, and default-on providers` — expected `function`, got `undefined`
4. `RATE-001 policy exports exact provider pins and rejects unreviewed hosts and paths` — expected provider URL array, got `undefined`

All four failures are due to the absent maintained source/script/workflow/policy contract. Every inherited policy case remains green.

## Path audit

Modified/added paths (only authorized):
- `test/rateWorker.node.js`
- `test/rateSupervisor.node.js`
- `test/securityPolicy.node.js`
- `test/fixtures/rates/provider-bodies-v1.json`

No production, package, workflow, policy implementation, wallet, Pay, broker, preload, renderer, Rust, dependency, lockfile, ticket, current-task, or unlisted path was changed.
