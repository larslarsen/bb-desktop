# Grok Build Handoff — BBD-RATE-001 Tests Correction 01

State: COMPLETE — SOURCE DELIVERED; REVIEW 02 REQUIRES NARROW CORRECTION

Actor: Sr Dev — Grok Build (`Grok 4.6`, High)

Governance baseline: `ba0fb26a58c09ca7df02410c456589b9b4bf4c00`

Read completely before editing:

- `AGENTS.md`
- `TESTING.md`
- `tickets/BBD-RATE-001.md`
- `docs/testing/BBD-RATE-001-TEST-SOURCE-REVIEW-01.md`
- `docs/architecture/BBD-RATE-001-PROVIDER-REVIEW.md`
- `docs/handoff/GROK_BUILD_BBD_RATE_001_TESTS_01.md`

## Authorized paths

Edit only the existing uncommitted four-path test drop:

- `test/rateWorker.node.js`
- `test/rateSupervisor.node.js`
- `test/securityPolicy.node.js`
- `test/fixtures/rates/provider-bodies-v1.json`

All production, package, workflow, policy implementation, wallet, Pay, broker, preload,
renderer, Rust, documentation, dependency, lockfile, evidence, Git, and unlisted paths are
frozen. Do not execute commands, tests, formatters, network access, or Git.

## Required bounded correction

Preserve the simplified product decision: Coinbase Exchange ZEC/USD only, Kraken XMR/USD
only, one labeled source, USD only, and runtime default-off. Correct the existing tests in
one pass without adding providers, aggregation, UI, or production source.

1. Split malformed request and response validation. Every request mutation must be passed
   to `validateRateRequest`; every response mutation must be passed to
   `validateRateResponse`. The canonical request and response must each remain accepted.
2. Treat exponent 8 and exponent 12 as independently valid for every canonical u64 amount.
   Specifically, `100000000` at exponent 12 must produce `0.00`, and `1000000000000` at
   exponent 8 must produce `10000.00`; reject only disallowed exponents or malformed values.
3. Make the backoff timeline causal. Prove no new fetch occurs before each delay expires,
   prove `30s, 60s, 120s, 240s, ...` capped at one hour, expire the final pending delay
   before recovery, and make the successful Coinbase observation time agree with the
   advanced clock (or use the timestamp-free Kraken response). Then prove a fresh cache is
   used through five minutes and omitted after expiry.
4. For every stale, expired, malformed, duplicate, multiple, wrong-asset, wrong-provider,
   wrong-currency, bad-time, or non-closed quote, assert both an unavailable display and an
   empty `quotes` array. Add exact-at-bound and one-tick-past-bound freshness cases. Reject
   accessors, symbols, exotic prototypes, and extra fields without invoking user code.
5. Add independent decimal boundaries: valid 12 whole digits and 18 fractional digits;
   invalid 13 whole digits, 19 fractional digits, zero/all-zero fractions, sign, exponent,
   whitespace, and leading whole zeroes. Add strict Coinbase time cases for an impossible
   Gregorian date, years 2019 and 2101, ten fractional digits, lowercase/offset zone, and
   the accepted 0- and 9-fraction boundaries. Invalid `fetched_at` must fail closed.
6. Make the HTTPS fake record request-body writes, request options, timeout controls, and
   lifecycle events. Prove each real attempt uses the pinned HTTPS host/path, port 443,
   GET, TLS 1.2 minimum, certificate validation, exact safe headers, no body, 16-KiB maximum
   headers, 5-second connect timeout, 10-second overall timeout, and no redirects. Exercise
   the timeout control itself, not only a prebuilt `ETIMEDOUT` error, and prove timeout,
   TLS, bad status/content type, header limit, and body limit abort and return unavailable.
7. Replace the overclaimed supervisor case with independent cases for malformed JSON,
   oversized frame, unsolicited/wrong ID, duplicate response ID, stdout diagnostics,
   partial frame at EOF, and one-pending-request overflow. Each protocol violation must
   kill or quarantine the child before test cleanup, clear pending state, return no stale
   quote, and leak no stderr/canary. Reject unknown or duplicate enabled provider IDs
   without spawning.
8. Exercise the actual child entry boundary offline once: start `quote-worker/worker.js`
   with zero provider arguments, send one exact framed query, observe one exact unavailable
   framed response with the same ID, close input, and prove clean exit with no network.
   Keep this deterministic and ensure cleanup on assertion failure.
9. Make provider policy non-tautological. Require production policy to export the exact
   two `RATE_PROVIDER_URLS`, require maintained source to contain only those reviewed pins,
   and mutation-test HTTP downgrade, arbitrary host, same-host wrong product/path, alternate
   Coinbase host, wrong Kraken pair, legacy ticker, TradingView, CoinGecko, and CoinPaprika.
   Keep renderer CSP free of every provider host and preserve the exact import allowlists.
10. Rename or remove any assertion whose title claims behavior its body does not exercise.
    Prefer replacing weak assertions over growing parallel helpers; keep the suite offline,
    deterministic, dependency-free, and no more complex than necessary.

## Stop and report

After the corrected four-path drop, stop. Report only:

- changed paths;
- SHA-256 and line count for each path;
- corrected worker/supervisor/policy test counts;
- a short mapping from findings 1–10 to the corrected test names; and
- confirmation that no command, test, network access, Git operation, production edit, or
  documentation edit ran.

Do not begin production source. Hermes remains unauthorized until reviewer acceptance.
