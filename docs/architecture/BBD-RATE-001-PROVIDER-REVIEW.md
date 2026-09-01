# BBD-RATE-001 Provider Review

Date: 2026-09-01

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Decision: **ONE LABELED SOURCE PER ASSET; RUNTIME DEFAULT OFF**

This review resolves BBD-WAL-001 questions Q11–Q13 only for BBD-RATE-001. It is an
engineering compatibility review, not legal advice or permission to redistribute a
provider's data.

## v1 source

The first version supports one source-pinned HTTPS response shape per asset:

| Provider ID | Pinned request | Stable mappings | Observed timestamp |
| --- | --- | --- | --- |
| `coinbase-exchange-v1` | `https://api.exchange.coinbase.com/products/ZEC-USD/ticker` | ZEC/USD | response `time` |
| `kraken-spot-v1` | `https://api.kraken.com/0/public/Ticker?pair=XMRUSD` | `XMRUSD`/`XXMRZUSD` | local fetch time; the response has no observation time |

The reviewer confirmed both mapped USD products and their successful current response
shapes on 2026-09-01. Coinbase Exchange currently returns 404 for `XMR-USD`, so it is not
an XMR source. Tests use committed recorded bodies and never call either service.

Current primary references:

- Coinbase public Exchange market data and product ticker:
  <https://docs.cdp.coinbase.com/exchange/introduction/welcome> and
  <https://docs.cdp.coinbase.com/api-reference/exchange-api/rest-api/products/get-product-ticker>

- Kraken public ticker and pair discovery:
  <https://docs.kraken.com/api-reference/market-data/get-ticker-information> and
  <https://docs.kraken.com/api-reference/market-data/get-tradable-asset-pairs>
- Kraken terms: <https://www.kraken.com/legal/global-terms>

Both responses represent prices as decimal strings, so the worker does not need a general
arbitrary-precision JSON-number parser. It takes only Coinbase's exact `price` string for
ZEC or Kraken's exact last-trade string at `c[0]` for XMR.

## Rejected TradingView scrape

TradingView's current policy explicitly prohibits automated collection and screen
scraping. Its supported widgets are direct human-readable displays with required
attribution and a TradingView network connection; they are not a price API. BitBook will
not scrape a page, add an external TradingView document to the wallet renderer, or grant
its renderer CSP access for this ticket.

Primary references:

- <https://www.tradingview.com/policies/>
- <https://www.tradingview.com/support/solutions/43000674726-why-is-my-account-banned-due-to-suspicious-activity/>
- <https://www.tradingview.com/widget-docs/getting-started/>

## Runtime and display decision

- Quote currency is exactly USD.
- A query covers exactly one asset: ZEC or XMR.
- One fresh quote from the asset's pinned source is displayed as
  `single_labeled_source` and `approximate`.
- Missing, malformed, stale, redirected, timed-out, or unavailable data produces
  `unavailable` and `fiat estimate unavailable`, never zero.
- There is no v1 median, spread, quorum, or multi-provider aggregation.
- Quote traffic is opt-in and default-off. BBD-RATE-001 adds no automatic provider
  traffic and no visible setting.
- Pay, amount, fee, receiver, eligibility, signing, and broadcast never depend on the
  quote.

BBD-WAL-010 may later add an owner-visible opt-in and approximate display. A future
source-reviewed adapter may use DEX market data with a clearly named stablecoin basis;
it must not silently represent a wrapped asset or a stablecoin assumption as native
ZEC/USD or XMR/USD. Fiat-origin payment requests remain out of scope.
