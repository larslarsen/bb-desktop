# Grok Build Handoff — BBD-RATE-001 Production 01

State: ACTIVE

Actor: Sr Dev — Grok Build (`Grok 4.6`, High)

Source baseline: `7f1520e0b8460026ecb5129f558b4b1582b381ec`

Read completely before editing:

- `AGENTS.md`
- `TESTING.md`
- `tickets/BBD-RATE-001.md`
- `docs/architecture/BBD-RATE-001-PROVIDER-REVIEW.md`
- `docs/testing/BBD-RATE-001-TEST-SOURCE-REVIEW-03.md`
- `docs/testing/BBD-RATE-001-EXPECTED-RED-01.md`
- all four committed RATE-001 test/fixture paths

## Authorized paths

Create or edit only:

- `quote-worker/providers.js`
- `quote-worker/model.js`
- `quote-worker/framing.js`
- `quote-worker/worker.js`
- `quote-worker/supervisor.js`
- `package.json`
- `scripts/security-policy.js`
- `.github/workflows/social.yml`
- `.github/workflows/security.yml`

All tests, fixtures, evidence, documentation, tickets, handoffs, lockfiles, dependencies,
wallet, Pay, broker, preload, renderer, Rust, daemon, other repository, Git, and unlisted
paths are frozen. Do not execute commands, tests, formatters, package managers, network
access, providers, or Git.

## Required implementation

Implement exactly the committed contract; do not weaken or special-case tests.

- Export only the closed CommonJS surfaces named by the ticket and tests. Keep the provider
  table deeply immutable and default-off. The only pins are Coinbase Exchange `ZEC-USD`
  and Kraken `XMRUSD`/`XXMRZUSD`.
- Use a bytes-first duplicate-key/depth/UTF-8/size-bounded JSON parser. Preserve prices as
  decimal strings throughout. Do not use `Number` or `parseFloat` for unit price or atomic
  conversion. Validate Gregorian timestamps strictly and perform half-even display
  rounding with integer/decimal arithmetic.
- Omit every invalid, stale, expired, multiple, mismatched, or non-closed quote. Never
  synthesize zero. Return only a labeled single fresh source or unavailable.
- Keep the quote child pipe separate from every wallet/broker path. Implement exact bounded
  frames, exact methods/keys, pending-ID checks, child failure containment, clean spawn
  environment, and deterministic injected seams used by the committed tests.
- Provider HTTPS exists only in `worker.js`. Apply the exact request pins, TLS/certificate,
  header/body, connect/overall timeout, no-redirect, one-in-flight, cache, and exponential
  backoff behavior. Real startup with zero provider arguments must remain offline and
  answer framed queries as unavailable.
- The synchronous injected fakes may complete during `query`; a real asynchronous provider
  or child response may populate only the later cache. Never block the event loop waiting
  for network or child output.
- Add exactly the rate test script, top-level test chaining, five syntax checks, routine CI
  command, and both workflow path filters required by the policy tests. Add no dependency
  and do not touch `package-lock.json`.
- Extend `scripts/security-policy.js` with the exact exported rate constants and fail-closed
  package/workflow/source checks required by the committed mutations. Permit only each
  module's reviewed built-ins/sibling imports and exact provider pins. Provider hosts must
  never enter renderer CSP.
- Do not add visible UI, a setting, IPC/preload surface, wallet/Pay integration, telemetry,
  API key, proxy inheritance, TradingView, CoinPaprika, CoinGecko, DEX data, aggregation,
  stablecoin assumptions, or live traffic by default.

## Stop and report

After the nine-path production drop, stop. Report:

- every changed path with SHA-256 and line count;
- the exact exported surfaces and provider defaults;
- a short test-contract mapping for model, framing, worker, supervisor, package/workflows,
  and policy;
- confirmation that all committed tests/fixtures retain their accepted hashes; and
- confirmation that no command, test, network/provider access, Git operation, dependency,
  lockfile, documentation, wallet, or unlisted edit occurred.

Do not author evidence, integrate, commit, push, falsify, or begin UI/Pay work. Hermes is
not authorized until reviewer source inspection accepts the drop.
