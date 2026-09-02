# BBD-RATE-001 — Isolated Optional Fiat Quote Worker

Status: ACTIVE — PRODUCTION GREEN GATE RESUME 02 AUTHORIZED

Reviewer: Lead Engineer/Reviewer — Codex at High

Test and production source actor: Sr Dev — Grok Build (`Grok 4.6`, High)

Integration actor: Jr Dev — Hermes

Source baseline: `54cc0ccc17bc55eec5b23a1d414f3250f4c728cc`

Architecture: `docs/architecture/BBD-WAL-001-REVIEW.md` §§5.3–5.4 and 12;
provider decision: `docs/architecture/BBD-RATE-001-PROVIDER-REVIEW.md`

Cross-repository boundary: this ticket edits only `bb-desktop`. `../bb-go` remains
wallet-free and rate-free. `../go-ipfs` remains deprecated.

## Objective

Add a separate, least-privileged quote worker that parses one source-pinned recorded body
per asset—Coinbase Exchange for ZEC/USD and Kraken for XMR/USD—and formats an optional
approximate display without gaining wallet or payment authority.

Rates are untrusted presentation data. Every wallet and Pay path must behave identically
when the worker is disabled, absent, crashed, stale, rate-limited, or malformed. This
ticket adds no visible rate setting or Pay UI and enables no live provider by default.

## Closed public modules

Production may later create exactly these CommonJS modules:

```text
quote-worker/providers.js
quote-worker/model.js
quote-worker/framing.js
quote-worker/worker.js
quote-worker/supervisor.js
```

`providers.js` exports an immutable, closed `PROVIDERS` table containing only
`coinbase-exchange-v1` for ZEC and `kraken-spot-v1` for XMR, their HTTPS request pins,
exact mappings, parser IDs, and `enabled_by_default: false`.

`model.js` exports exactly:

```text
normalizeRateQuery(value)
parseProviderBody(provider_id, body, fetched_at)
buildRateSnapshot(query, quotes, queried_at)
formatFiatEstimate(amount_atomic, exponent, price, quote_currency)
```

`framing.js` exports only the bounded encoder/decoder and exact request/response
validators required by the worker pipe. `worker.js` exports `createQuoteWorker(options)`
and acts as the child entry point only under `require.main === module`.
`supervisor.js` exports exactly `createQuoteSupervisor(options)`.

No production dependency or lockfile change is allowed. Pure model/provider/framing code
may use only Node built-ins that do not perform I/O. HTTPS is permitted only in
`quote-worker/worker.js`; process spawning is permitted only in
`quote-worker/supervisor.js`.

## Query and normalized schemas

All objects must be fresh, plain, closed data. Accessors, symbols, exotic prototypes,
cycles, duplicate JSON keys, non-finite values, oversized values, and unknown control
fields fail closed without invoking user code.

```text
RateQueryV1 {
  v: 1,
  asset_ids: ["zec-zcash"] | ["xmr-monero"],
  quote_currencies: ["USD"]
}

RateQuoteV1 {
  v: 1,
  asset: "ZEC" | "XMR",
  asset_id: "zec-zcash" | "xmr-monero",
  quote_currency: "USD",
  price: DecimalString,
  provider_id: reviewed provider ID,
  provider_observed_at: strict TimestampV1,
  fetched_at: strict TimestampV1,
  fresh_until: strict TimestampV1,
  expires_at: strict TimestampV1,
  status: "fresh"
}

RateSnapshotV1 {
  v: 1,
  queried_at: strict TimestampV1,
  quotes: [fresh RateQuoteV1 only],
  display: {
    asset: "ZEC" | "XMR",
    quote_currency: "USD",
    price: DecimalString only for a fresh single source,
    method: "single_labeled_source" | "unavailable",
    source_ids: [] | ["coinbase-exchange-v1"] | ["kraken-spot-v1"],
    label: "approximate" | "fiat estimate unavailable"
  }
}
```

`DecimalString` is the architecture's strictly positive, non-exponent decimal with at
most 12 whole and 18 fractional digits. Canonical output removes leading whole zeroes and
trailing fractional zeroes, removing the decimal point when no fraction remains. Zero,
signs, exponent notation, grouping, whitespace, NaN, and Infinity are invalid prices.

The provider JSON parser must reject duplicate keys and depth greater than eight. A price
is accepted only as the exact JSON string at its pinned provider field; it must never pass
through `Number`, `parseFloat`, or binary floating-point arithmetic. Timestamp arithmetic
may use integer milliseconds after strict calendar validation.

Coinbase's reviewed parser requires response `time` as a strict UTC RFC 3339 calendar
value with zero through nine fractional digits, no offset, no leap second, and year
2020–2100. It normalizes that value down to the same whole second as `TimestampV1`; it
does not round. The result may be no more than ten minutes old or five minutes in the
future. Kraken uses `fetched_at` as `provider_observed_at` because the pinned public
ticker response has no observation timestamp. `fresh_until` and `expires_at` are exactly
five minutes after `fetched_at`. No quote is returned or cached after either bound.

## Exact provider parsing

Every response is bytes-first, at most 65,536 bytes, UTF-8 JSON with depth at most eight.
Unknown provider payload fields may be ignored only after the whole JSON envelope passes
the structural bounds and duplicate-key check.

- Coinbase requires the exact `ZEC-USD` ticker path, strict response `time`, and decimal
  string `price`. Missing, extra, numeric, or malformed price/time fields fail the result.
- Kraken requires an empty `error` array, exactly `XXMRZUSD`, and the decimal string at
  `c[0]`. Pair aliases, collisions, extra result pairs, and malformed ticker arrays fail
  the result.

A malformed result produces no `RateQuoteV1`; it never produces a zero or retained stale
row.

## Deterministic selection and display conversion

The one fresh quote must match the query asset/USD and its pinned provider exactly:
`coinbase-exchange-v1` for ZEC and `kraken-spot-v1` for XMR. It produces its price,
method `single_labeled_source`, that source ID, and label `approximate`. Zero, duplicate,
mismatched, or multiple quotes produce no price, method `unavailable`, an empty source
list, and label `fiat estimate unavailable`. There is no v1 median, spread, quorum, or
disagreement calculation.

`formatFiatEstimate` accepts a canonical nonnegative u64 `amount_atomic`, exponent 8 for
ZEC or 12 for XMR, a valid price, and exactly USD. It returns a plain decimal string with
exactly two fractional digits using integer/decimal arithmetic and round-half-even.
It sends nothing to a provider, carries no authority, and never mutates input. Overflow
beyond 32 whole display digits fails closed.

## Worker transport and process isolation

The worker pipe is distinct from the wallet broker pipe. Frames are four-byte
big-endian-length-prefixed JSON, at most 65,536 bytes, with one exact request and response:

```text
{ v: 1, id: 32 lowercase hex, kind: "req", method: "rate.query", params: RateQueryV1 }
{ v: 1, id: same, kind: "res", method: "rate.snapshot", result: RateSnapshotV1 }
```

Unknown methods/fields, duplicate IDs, unsolicited responses, malformed frames, excess
pending requests, stdout diagnostics, and partial/oversized frames fail closed. Stderr is
diagnostic-only, bounded, and never reaches the renderer.

The supervisor may spawn only the reviewed worker path with `shell: false`, anonymous
`pipe` stdio, an exact clean environment, and only reviewed provider IDs as non-secret
startup arguments. Zero enabled providers is the default and must not spawn or contact a
provider. It returns a closed unavailable snapshot. It never shares a wallet handle,
broker session, wallet data directory, social identity, account, address, balance, peer,
memo, amount, request ID, cookie, proxy environment, or API key.

When explicitly enabled later, `worker.js` may issue only exact pinned HTTPS GETs with
certificate validation, TLS 1.2 minimum, `Accept: application/json`, identity encoding,
no cookies/referer/key, 16 KiB maximum response headers, 5-second connect timeout,
10-second overall timeout, no redirect, and 64 KiB maximum body. HTTP other than 200,
unexpected content type, TLS error, timeout, or limit breach drops that provider.

Per-provider failures back off 30 seconds, doubling to a one-hour cap. Only one request
per provider may be in flight. Cached quotes last at most five minutes and are never
served stale. Clock, timers, spawn, and HTTPS are injectable for deterministic tests;
production defaults remain closed.

## Forbidden authority and leakage

The complete query, child arguments, environment, logs, and HTTPS requests may contain
only the reviewed provider ID, canonical asset ID, USD, fixed path/headers, frame ID, and
bounded timing/error categories. Tests use canaries to reject every account ID, address,
balance, amount, fee, peer ID, memo, request ID, receiver, wallet/broker method, identity,
secret, API key, cookie, proxy, OS/user path, and arbitrary URL.

The worker and supervisor must not import Electron, wallet contract/pay/broker modules,
Rust/native modules, filesystem, device, social, renderer, or daemon code. The renderer,
`bb-go`, wallet broker, and Pay model gain no provider network path. CSP gains no provider
host. Existing Pay/prepare/confirm tests remain unchanged and green with rates absent.

## Test-first phases

### Phase A — test source only (authorized now)

Under `docs/handoff/GROK_BUILD_BBD_RATE_001_TESTS_01.md`, Grok Build may create or edit
only:

- `test/rateWorker.node.js`
- `test/rateSupervisor.node.js`
- `test/securityPolicy.node.js`
- `test/fixtures/rates/provider-bodies-v1.json`

The two new suites must use only Node built-ins, deterministic clocks/transports, and the
committed fixture. They must leave no handles, child processes, timers, listeners, files,
or network activity. Existing wallet, Pay, Electron, preload, production, package,
workflow, policy implementation, documentation, evidence, dependency, and lockfile bytes
are frozen.

After reviewer source inspection, Hermes will run:

```text
node test/rateWorker.node.js
node test/rateSupervisor.node.js
node test/securityPolicy.node.js
```

The first two must fail only because the new quote-worker modules are absent. The policy
suite must fail only on the absent maintained source/script/workflow/policy contract;
all pre-existing assertions must remain green.

### Phase B — production source only (future, not authorized)

After accepted expected red, a separate handoff may authorize only the five quote-worker
modules, `package.json`, `scripts/security-policy.js`, and the two routine workflow files.
Tests, lockfiles, wallet/Pay/broker/preload/renderer/daemon/Rust code, and all other paths
will remain frozen.

## Acceptance and falsification

The future green gate must include both rate suites, Electron security, repository policy,
the existing wallet Pay and wallet contract suites, `npm test`, `npm run build`,
`node scripts/security-policy.js`, npm audit, both Gitleaks scans, and `git diff --check`.
No live provider call, package build, Rust gate, device, daemon, or cross-repository command
is required.

Before acceptance Hermes must isolate, detect, and restore at least:

1. IEEE-754 coercion of a precision canary price;
2. one private-context field admitted to a rate query/log/request;
3. redirect acceptance or an unpinned provider URL;
4. a wrong Coinbase product or wrong/extra Kraken result pair accepted as the requested
   asset; and
5. a provider enabled or spawned by default without opt-in.

Every mutation must make the focused test fail for the intended reason. No falsification
mutation is committed.
