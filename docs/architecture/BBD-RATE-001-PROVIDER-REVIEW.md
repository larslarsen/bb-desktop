# BBD-RATE-001 Provider Review

Date: 2026-09-01

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Decision: **ACCEPTED FOR RECORDED-FIXTURE PARSERS; RUNTIME DEFAULT OFF**

This review resolves BBD-WAL-001 questions Q11–Q13 only for BBD-RATE-001. It is an
engineering compatibility review, not legal advice or permission to redistribute a
provider's data.

## Current provider findings

The following source-pinned HTTPS shapes currently return both ZEC/USD and XMR/USD
without an API key:

| Provider ID | Pinned base and query shape | Stable mappings | Observed timestamp |
| --- | --- | --- | --- |
| `coinpaprika-v1` | `https://api.coinpaprika.com/v1/tickers/{id}?quotes=USD` | `zec-zcash`, `xmr-monero` | response `last_updated` |
| `coinmarketcap-keyless-v1` | `https://pro-api.coinmarketcap.com/public-api/v1/simple/price?ids={id}&convert=USD` | ZEC `1437`, XMR `328` | response `status.timestamp` |
| `kraken-spot-v1` | `https://api.kraken.com/0/public/Ticker?pair={pair}` | `ZECUSD`/`XZECZUSD`, `XMRUSD`/`XXMRZUSD` | local fetch time; ticker response has no observation time |

The reviewer confirmed the exact IDs/pairs and successful current response shapes on
2026-09-01. The production tests use committed recorded bodies only and never call these
services.

Current primary references:

- CoinPaprika REST introduction and ticker reference:
  <https://docs.coinpaprika.com/get-started/api-rest-introduction> and
  <https://docs.coinpaprika.com/api-reference/tickers/get-ticker-for-a-specific-coin>
- CoinPaprika API terms: <https://coinpaprika.com/api-terms-of-use/>
- CoinMarketCap keyless API: <https://coinmarketcap.com/api/documentation/pro-api-reference/keyless-public-api>
- CoinMarketCap API plans: <https://coinmarketcap.com/api/pricing/>
- Kraken public ticker and pair discovery:
  <https://docs.kraken.com/api-reference/market-data/get-ticker-information> and
  <https://docs.kraken.com/api-reference/market-data/get-tradable-asset-pairs>
- Kraken terms: <https://www.kraken.com/legal/global-terms>

CoinGecko is not a v1 parser because its current Demo API setup requires a user API key.
BitBook will not embed, collect, persist, or forward a provider credential.

## Runtime decision

All three provider pins are compiled as reviewed parser/request definitions but are
runtime-disabled by default. BBD-RATE-001 adds no automatic provider traffic. A later
owner-visible opt-in may enable a subset only after the UI presents applicable source
attribution and the distribution posture has been reviewed. Provider base URLs, paths,
asset mappings, and TLS rules are never renderer supplied.

The quote worker must remain useful with zero enabled providers: it returns a closed
`unavailable` snapshot and Pay remains fully functional. Provider removal or replacement
is a source review, not remote configuration.

## Aggregation decision

- Quote currency is exactly USD in v1.
- A query covers exactly one canonical asset ID: `zec-zcash` or `xmr-monero`.
- Median quorum is exactly three fresh provider quotes.
- The agreement bound is five percent relative spread:
  `(maximum - minimum) / median <= 0.05`, compared with integer decimal arithmetic.
- Three quotes inside the bound produce the exact middle price and method `median`.
- One quote produces `single_labeled_source` with that source named.
- Zero or two quotes produce `unavailable`; two quotes beyond the bound are labeled
  `quotes disagree`, otherwise `fiat estimate unavailable` for insufficient quorum.
- Missing, invalid, stale, rate-limited, redirected, timed-out, or disagreeing data never
  becomes zero and never changes coin amount, fee, receiver, Pay eligibility, signing, or
  broadcast behavior.

## Product default

Quote traffic is opt-in and default-off. BBD-RATE-001 supplies the worker, parsers,
supervisor, and closed protocol but no visible setting and no renderer network authority.
BBD-WAL-010 may later add the owner-visible opt-in and approximate display. Fiat-origin
payment requests remain out of scope.
