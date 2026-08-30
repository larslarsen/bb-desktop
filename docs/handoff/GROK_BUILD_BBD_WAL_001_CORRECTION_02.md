# Grok Build Handoff — BBD-WAL-001 Correction 02

You are **Sr Dev — Grok Build**, using Grok 4.6 High. This is the complete durable
correction prompt; ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Read completely before acting: `AGENTS.md`, `TESTING.md`,
`docs/engineering/DEVELOPMENT_ROLES.md`, `tickets/BBD-WAL-001.md`,
`docs/handoff/CURRENT_TASK.md`, `docs/handoff/GROK_BUILD_BBD_WAL_001.md`,
`docs/handoff/GROK_BUILD_BBD_WAL_001_CORRECTION_01.md`, and the current
`docs/architecture/BBD-WAL-001-REVIEW.md`.

The current architecture source is 1,776 lines with SHA-256
`6389f4b920c594f97e3d2eb8048d308ab563c2cce23f345f1279982d57283aa4`. Correction 01 is
substantively accepted, but Codex Luna remains stopped because the owner identified the
omitted exchange-rate dependency and reviewer found two small canonical-validation
imprecisions. Correct the document in place; do not append a detached erratum.

You may edit only:

- `docs/architecture/BBD-WAL-001-REVIEW.md`

## Reviewer-supplied rate trace and current source facts

Use these facts without network access and add them to the dated source snapshot:

- Inherited desktop `js/utils/currency.js` polls its local daemon at
  `/ob/exchangerates/<coin>`; `js/utils/exchangeRateSyncer.js` repeats every five minutes.
- Inherited daemon `api/jsonapi.go` delegates that route to the legacy wallet's
  `ExchangeRates()` implementation.
- Vendored ZEC code tries `https://ticker.openbazaar.org/api` first, then old Bittrex,
  Bitfinex, Poloniex, and Kraken endpoints. This entire chain is deprecated.
- OB1 published <https://github.com/OpenBazaar/tickerproxy> under MIT. It gathered
  BitcoinAverage data and wrote cache output locally or to S3. Its latest listed release
  is from 2018; use only as historical design evidence, not code or infrastructure.
- CoinPaprika documents a mostly keyless free API, current ticker data, and a 20,000
  request/month free limit: <https://docs.coinpaprika.com/api-reference/rest-api/introduction>.
- CoinGecko documents `/simple/price`, stable asset IDs, timestamps, and a keyless public
  endpoint intended for low-volume/testing rather than production reliability:
  <https://docs.coingecko.com/reference/simple-price> and
  <https://docs.coingecko.com/docs/keyless-public-api>.
- Kraken documents public `Assets`, `AssetPairs`, and `Ticker` discovery; pair
  availability must be probed rather than assumed:
  <https://support.kraken.com/articles/360000920306-api-symbols-and-tickers>.

## Required rate architecture

1. Add exchange-rate providers to the context, trust diagram, threats, UX, offline tests,
   security/SBOM gates, ticket decomposition, open questions, and decision register.
2. Rates are optional, untrusted presentation data. `amount_atomic`, asset, network,
   receiver, and the prepared fee are authoritative. Wallet setup, balance, receive, Pay,
   signing, and broadcast must continue if every rate source is missing, stale,
   disagreeing, rate-limited, malformed, or offline.
3. Do not fetch rates in `bb-go`, the social renderer, or the wallet broker's trusted
   spend core. Specify a separate least-privileged desktop quote worker/child with no
   wallet IPC handle, wallet files, device access, social identity, broker session,
   account IDs, addresses, balances, peer IDs, memos, payment amounts, or request IDs.
   It receives only a fixed asset-ID/quote-currency query and emits a bounded normalized
   `RateSnapshotV1` to desktop presentation. A broker confirmation may display an
   explicitly approximate sanitized snapshot, but it must never use it to change the
   prepared atomic amount.
4. Define a provider interface and normalized fixed-decimal-string schema: stable asset
   IDs (not ambiguous tickers), base/quote, price, provider ID, provider observation time,
   local fetch time, freshness/expiry, and status. Never use IEEE-754 floating point for
   conversion. Bound body size, decimal digits, time skew, redirect behavior, TLS, parse
   depth, asset/quote allowlists, timeouts, cache lifetime, and backoff. Do not send an
   amount to a provider; fetch a unit price and convert locally.
5. Keep provider choice replaceable and source-pinned in configuration. No embedded API
   key, silent telemetry, OB1 endpoint, or mandatory BitBook-operated proxy. A later
   provider ticket may evaluate CoinPaprika, CoinGecko public, and direct Kraken market
   data against current terms and availability; this review must not promise that any
   provider or ZEC/XMR pair will remain available.
6. Recommend v1 behavior: coin-denominated signed payment requests only, with optional
   approximate local fiat display. If a later owner-approved ticket allows “request
   $10,” conversion occurs locally once before signing; the immutable request still
   contains exact `amount_atomic`, and signed non-authoritative quote provenance records
   currency, decimal price, sources, observation/expiry, rounding rule, and resulting
   atomic amount. Never dynamically reprice a signed request.
7. Specify aggregation/failure semantics. At minimum: stale or invalid quotes are absent,
   not zero; disagreement is visible and cannot block coin payment; provider data cannot
   change fees or Pay eligibility. Prefer a median only when enough fresh independent
   providers agree within a bounded spread; otherwise show one labeled source or
   “fiat estimate unavailable,” according to a later provider ticket. Do not use
   unauthenticated P2P peers as a price oracle.
8. Add a proposed, separately reviewable `BBD-RATE-001` before user-visible wallet UX,
   test-first with recorded fixtures only. Cover malformed/oversized bodies, duplicate
   assets, symbol collision, negative/zero/extreme/non-finite/scientific values, stale
   and future timestamps, provider disagreement, timeouts, redirects, cache/backoff,
   deterministic decimal rounding, and a query/log canary proving no private context is
   sent. Ordinary tests use no network and no API key.

## Required canonical-validation fixes

- The current Unicode text incorrectly implies bidi overrides are C0/C1 controls. Define
  and reject the relevant bidi formatting/isolate controls explicitly (including
  U+202A–U+202E and U+2066–U+2069), plus any other deliberately prohibited format
  controls; add golden vectors.
- `TimestampV1` regex shape alone accepts impossible calendar dates. Require strict
  calendar parsing, round-trip equality to the canonical UTC-second form, a bounded year
  range, and golden failures such as February 30, hour 24, and year 0000.

The first implementation slice remains offline, deterministic, credential-free, and
incapable of constructing or moving real funds. Rate implementation is not added to
BBD-WAL-002; only ensure the common contract does not make a rate mandatory.

Do not execute tests, builds, installs, formatters, scanners, Git, GitHub, wallet, node,
network, hardware, or package commands. Do not touch a real device or secret. Do not use
`/tmp`, root, deletion, cleanup, `rm`, globs, variables as targets, or unresolved paths.
Do not edit any other file.

In your terminal response report only the authored path, line count, SHA-256, principal
corrections, unresolved owner choices, and confirmation that no out-of-scope action ran.
Codex reviews the corrected source; Codex Luna remains stopped until exact acceptance.
