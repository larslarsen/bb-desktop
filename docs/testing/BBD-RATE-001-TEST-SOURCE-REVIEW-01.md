# BBD-RATE-001 Test-Source Review 01

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Governance HEAD at review: `ba0fb26a58c09ca7df02410c456589b9b4bf4c00`

Result: **TEST SOURCE REJECTED BEFORE EXECUTION — ONE BOUNDED CORRECTION REQUIRED**

## Reviewed uncommitted paths

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `test/rateWorker.node.js` | 906 | `00c65f399513f4bd9c9fb06f9e87db230c36e12f205390e587e752b04bd1fb0b` |
| `test/rateSupervisor.node.js` | 438 | `8783b32ae7e19da3275b915e575d435693160c960765359867dc493c88f92f5d` |
| `test/securityPolicy.node.js` | 2,823 | `113b4ddd942a9d1857055f3dc1e65081f060d431ce8af22e53ea4ab41aeb209c` |
| `test/fixtures/rates/provider-bodies-v1.json` | 195 | `e3e5cdf3d594dfdde38322dbcb64daf4baac12c496b9402d42685f6474f8a632` |

The inventory is 16 worker tests, 9 supervisor tests, and 4 new policy tests within
82 total policy tests. Only the four authorized paths differ from governance HEAD, and
`git diff --check` passes. The reviewer did not execute Node, npm, or any test.

## Accepted direction

The drop correctly uses Coinbase Exchange only for ZEC/USD and Kraken only for XMR/USD,
keeps both providers default-off, preserves decimal prices as strings, uses offline
injected boundaries, separates quote and wallet authority, and adds no production or
dependency change. No median, quorum, TradingView scrape, CoinPaprika, or CoinGecko path
was reintroduced.

## Blocking findings

1. The framing oracle is internally contradictory. It first accepts the canonical request,
   then routes a malformed response case back through `validateRateRequest` using that same
   canonical request and requires `SCHEMA`. A correct implementation cannot satisfy both.
2. `formatFiatEstimate` wrongly requires valid exponent-12 and exponent-8 calls to fail
   based only on the magnitude of `amount_atomic`. The API has no asset argument; every
   canonical u64 is valid with either allowed exponent.
3. The backoff oracle asks for recovery while the last one-hour backoff is still pending.
   Even after that is fixed, its eventual Coinbase body is more than ten minutes older
   than the advanced injected clock and therefore must be rejected by a correct parser.
4. Snapshot tests check only `display` for several hostile quotes. They do not prove stale,
   expired, malformed, or mismatched rows are omitted from `quotes`, so retained stale data
   can pass.
5. The HTTPS harness freezes timeout/header values only in the provider table. It does not
   prove the request applies the 5-second connect timeout, 10-second overall timeout,
   16-KiB header limit, HTTPS/443 pin, or an empty request body. The timeout case is an
   injected error, not an exercised timeout control.
6. The supervisor test named for malformed, oversized, unsolicited, duplicate, and
   diagnostic output emits only one unsolicited response followed by an oversized prefix.
   It asserts child termination only after calling `shutdown`, so protocol-failure
   termination is vacuous. Partial frames, duplicate responses, excess pending work, and
   malformed stdout are not independently proved.
7. Parser coverage omits the stated decimal whole/fraction boundaries and strict Gregorian
   timestamp failures, leaving shortcuts through permissive date parsing or incomplete
   decimal validation able to pass.
8. `RATE_PROVIDER_URLS` is a test-local constant checked only for an `https://` prefix.
   That is tautological and does not require the production policy to export or enforce the
   two exact provider pins; unreviewed HTTPS hosts and same-host wrong paths remain
   insufficiently falsified.

The correction is authorized only by
`docs/handoff/GROK_BUILD_BBD_RATE_001_TESTS_CORRECTION_01.md`. Hermes integration and
expected-red execution remain unauthorized until the corrected source is reviewed.
