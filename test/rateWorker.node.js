'use strict';

const assert = require('assert');
const { spawnSync } = require('child_process');
const fs = require('fs');
const path = require('path');

const repoRoot = path.resolve(__dirname, '..');
const fixturePath = path.join(__dirname, 'fixtures', 'rates', 'provider-bodies-v1.json');
const fixtureBytes = fs.readFileSync(fixturePath);
const fixture = JSON.parse(fixtureBytes.toString('utf8'));

assert.strictEqual(fixture.name, 'bbd-rate-001-provider-bodies-v1');
assert.strictEqual(typeof fixture.prices.zec, 'string');
assert.strictEqual(typeof fixture.prices.xmr, 'string');
assert.notStrictEqual(String(Number(fixture.prices.zec)), fixture.prices.zec);
assert.notStrictEqual(String(Number(fixture.prices.xmr)), fixture.prices.xmr);
assert.ok(fixture.bodies.coinbase_zec_valid.raw_json.includes(`"${fixture.prices.zec}"`));
assert.ok(fixture.bodies.kraken_xmr_valid.raw_json.includes(`"${fixture.prices.xmr}"`));
assert.ok(!/"price"\s*:\s*42\.123456789012345678/.test(fixture.bodies.coinbase_zec_valid.raw_json));
assert.ok(!/"c"\s*:\s*\[\s*158\.987654321098765432/.test(fixture.bodies.kraken_xmr_valid.raw_json));

const { PROVIDERS } = require('../quote-worker/providers');
const {
  normalizeRateQuery,
  parseProviderBody,
  buildRateSnapshot,
  formatFiatEstimate,
} = require('../quote-worker/model');
const {
  RATE_FRAME_LIMIT,
  encodeRateFrame,
  createRateFrameDecoder,
  validateRateRequest,
  validateRateResponse,
} = require('../quote-worker/framing');
const { createQuoteWorker } = require('../quote-worker/worker');

const tests = [];
function test(name, fn) { tests.push({ name, fn }); }

const FETCHED_AT = fixture.fetched_at;
const FRESH_UNTIL = fixture.fresh_until;
const EXPIRES_AT = fixture.expires_at;
const FRAME_ID = 'aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa';
const COINBASE_ID = 'coinbase-exchange-v1';
const KRAKEN_ID = 'kraken-spot-v1';
const CANARIES = fixture.canaries;
const QUOTE_KEYS = Object.freeze([
  'v', 'asset', 'asset_id', 'quote_currency', 'price', 'provider_id',
  'provider_observed_at', 'fetched_at', 'fresh_until', 'expires_at', 'status',
]);
const SNAPSHOT_KEYS = Object.freeze(['v', 'queried_at', 'quotes', 'display']);
const DISPLAY_FRESH_KEYS = Object.freeze([
  'asset', 'quote_currency', 'price', 'method', 'source_ids', 'label',
]);
const DISPLAY_UNAVAILABLE_KEYS = Object.freeze([
  'asset', 'quote_currency', 'method', 'source_ids', 'label',
]);
const QUERY_KEYS = Object.freeze(['v', 'asset_ids', 'quote_currencies']);
const FORBIDDEN_HOSTS = Object.freeze([
  'ticker.openbazaar.org', 'bittrex', 'bitfinex', 'poloniex', 'coingecko',
  'coinpaprika', 'tradingview', 'api.coinbase.com', 'pro.coinbase.com',
]);
const FORBIDDEN_IMPORTS = Object.freeze([
  'electron', 'fs', 'node:fs', 'child_process', 'node:child_process',
  'net', 'http', 'dgram', 'cluster', 'worker_threads', 'os', 'dns',
  '../wallet-contract', '../wallet-pay/model', '../wallet-broker/protocol',
  '../wallet-broker/supervisor', '../wallet-preload', '../social-main',
  '../social/core',
]);
const SAFE_HEADERS = Object.freeze({
  accept: 'application/json',
  'accept-encoding': 'identity',
});

function clone(value) {
  return JSON.parse(JSON.stringify(value));
}

function assertClosedKeys(value, expected) {
  assert.deepStrictEqual(Object.keys(value).sort(), [...expected].sort());
  assert.strictEqual(Object.getPrototypeOf(value), Object.prototype);
}

function assertCode(fn, code) {
  let error;
  try { fn(); } catch (caught) { error = caught; }
  assert.ok(error, `expected ${code}`);
  assert.strictEqual(error.code, code);
  const message = String(error.message || error);
  for (const canary of Object.values(CANARIES)) {
    assert.ok(!message.includes(canary), `error leaked canary ${canary}`);
  }
}

function bodyBytes(entry) {
  if (entry.hex_bytes) return Buffer.from(entry.hex_bytes, 'hex');
  if (entry.byte_length) {
    const prefix = Buffer.from(entry.prefix, 'utf8');
    const suffix = Buffer.from(entry.suffix, 'utf8');
    const fill = Buffer.alloc(entry.byte_length - prefix.length - suffix.length, entry.fill.charCodeAt(0));
    return Buffer.concat([prefix, fill, suffix]);
  }
  return Buffer.from(entry.raw_json, 'utf8');
}

function independentFrame(value) {
  const body = Buffer.from(JSON.stringify(value), 'utf8');
  const prefix = Buffer.alloc(4);
  prefix.writeUInt32BE(body.length, 0);
  return Buffer.concat([prefix, body]);
}

function decodeFrame(bytes) {
  const buffer = Buffer.from(bytes);
  const length = buffer.readUInt32BE(0);
  assert.strictEqual(buffer.length, length + 4);
  return JSON.parse(buffer.subarray(4).toString('utf8'));
}

function zecQuery() { return clone(fixture.queries.zec); }
function xmrQuery() { return clone(fixture.queries.xmr); }

function timestampFromMs(ms) {
  return new Date(ms).toISOString().replace(/\.\d{3}Z$/, 'Z');
}

function expectedZecQuote() {
  return {
    v: 1,
    asset: 'ZEC',
    asset_id: 'zec-zcash',
    quote_currency: 'USD',
    price: fixture.prices.zec,
    provider_id: COINBASE_ID,
    provider_observed_at: FETCHED_AT,
    fetched_at: FETCHED_AT,
    fresh_until: FRESH_UNTIL,
    expires_at: EXPIRES_AT,
    status: 'fresh',
  };
}

function expectedXmrQuote() {
  return {
    v: 1,
    asset: 'XMR',
    asset_id: 'xmr-monero',
    quote_currency: 'USD',
    price: fixture.prices.xmr,
    provider_id: KRAKEN_ID,
    provider_observed_at: FETCHED_AT,
    fetched_at: FETCHED_AT,
    fresh_until: FRESH_UNTIL,
    expires_at: EXPIRES_AT,
    status: 'fresh',
  };
}

function unavailableDisplay(asset) {
  return {
    asset,
    quote_currency: 'USD',
    method: 'unavailable',
    source_ids: [],
    label: 'fiat estimate unavailable',
  };
}

function freshDisplay(asset, price, sourceId) {
  return {
    asset,
    quote_currency: 'USD',
    price,
    method: 'single_labeled_source',
    source_ids: [sourceId],
    label: 'approximate',
  };
}

function assertUnavailableSnapshot(snapshot, asset) {
  assert.deepStrictEqual(snapshot.quotes, []);
  assert.deepStrictEqual(snapshot.display, unavailableDisplay(asset));
  assertClosedKeys(snapshot.display, DISPLAY_UNAVAILABLE_KEYS);
  assert.ok(!Object.prototype.hasOwnProperty.call(snapshot.display, 'price'));
}

function decimalDigits(price) {
  assert.strictEqual(typeof price, 'string');
  const match = /^(0|[1-9][0-9]*)(?:\.([0-9]+))?$/.exec(price);
  assert.ok(match, `price ${price} is not a decimal`);
  return { whole: match[1], frac: match[2] || '' };
}

function formatOracle(amountAtomic, exponent, price) {
  const amount = BigInt(amountAtomic);
  const parsed = decimalDigits(price);
  const unscaled = BigInt(parsed.whole + parsed.frac);
  const scale = parsed.frac.length;
  const product = amount * unscaled;
  const down = exponent + scale - 2;
  let rounded;
  if (down >= 0) {
    const divisor = 10n ** BigInt(down);
    const quotient = product / divisor;
    const remainder = product % divisor;
    if (remainder * 2n < divisor) rounded = quotient;
    else if (remainder * 2n > divisor) rounded = quotient + 1n;
    else rounded = quotient % 2n === 0n ? quotient : quotient + 1n;
  } else {
    rounded = product * (10n ** BigInt(-down));
  }
  const abs = rounded < 0n ? -rounded : rounded;
  const whole = abs / 100n;
  const frac = abs % 100n;
  const wholeText = whole.toString();
  assert.ok(wholeText.length <= 32, 'oracle overflowed 32 whole digits');
  return `${wholeText}.${frac.toString().padStart(2, '0')}`;
}

function serializeCanaryScan(value) {
  if (Buffer.isBuffer(value) || value instanceof Uint8Array) return Buffer.from(value).toString('utf8');
  return JSON.stringify(value);
}

function assertNoCanary(value, label) {
  const text = serializeCanaryScan(value);
  for (const [name, canary] of Object.entries(CANARIES)) {
    assert.ok(!text.includes(canary), `${label} leaked ${name}`);
  }
}

function headerMap(record) {
  const headers = {};
  for (const [key, value] of Object.entries(Object.assign({}, record.options.headers, record.headers))) {
    headers[String(key).toLowerCase()] = value;
  }
  return headers;
}

function createHttpsFake(resolver) {
  const requests = [];
  return {
    requests,
    request(urlOrOptions, optionsOrCb, maybeCb) {
      let options;
      let cb;
      if (typeof urlOrOptions === 'string' || (urlOrOptions && typeof urlOrOptions === 'object' && urlOrOptions.href)) {
        const url = new URL(urlOrOptions);
        options = Object.assign({
          protocol: url.protocol,
          hostname: url.hostname,
          host: url.hostname,
          path: `${url.pathname}${url.search}`,
          port: url.port ? Number(url.port) : 443,
        }, typeof optionsOrCb === 'object' && optionsOrCb ? optionsOrCb : {});
        cb = typeof optionsOrCb === 'function' ? optionsOrCb : maybeCb;
      } else {
        options = Object.assign({}, urlOrOptions);
        cb = optionsOrCb;
      }
      const record = {
        options,
        headers: Object.assign({}, options.headers),
        writes: [],
        timeouts: [],
        events: [],
        aborted: false,
        destroyed: false,
        ended: false,
      };
      const listeners = { error: [], timeout: [], response: [], abort: [] };
      function complete(outcome) {
        if (outcome.error) {
          (listeners.error || []).forEach((fn) => fn(outcome.error));
          return;
        }
        const dataListeners = [];
        const endListeners = [];
        const errorListeners = [];
        const incoming = {
          statusCode: outcome.statusCode,
          headers: outcome.headers || { 'content-type': 'application/json' },
          on(event, fn) {
            if (event === 'data') dataListeners.push(fn);
            if (event === 'end') endListeners.push(fn);
            if (event === 'error') errorListeners.push(fn);
            return incoming;
          },
          resume() { return incoming; },
          destroy() { record.destroyed = true; record.aborted = true; },
        };
        if (typeof cb === 'function') cb(incoming);
        (listeners.response || []).forEach((fn) => fn(incoming));
        if (outcome.responseError) {
          errorListeners.forEach((fn) => fn(outcome.responseError));
          return;
        }
        const chunk = Buffer.isBuffer(outcome.body) ? outcome.body : Buffer.from(outcome.body || '', 'utf8');
        dataListeners.forEach((fn) => fn(chunk));
        endListeners.forEach((fn) => fn());
      }
      const req = {
        on(event, fn) {
          listeners[event] = listeners[event] || [];
          listeners[event].push(fn);
          record.events.push(['on', event]);
          return req;
        },
        once(event, fn) {
          return req.on(event, fn);
        },
        setHeader(name, value) {
          record.headers[name] = value;
        },
        setTimeout(ms, fn) {
          record.timeouts.push(ms);
          record.events.push(['setTimeout', ms]);
          if (fn) req.on('timeout', fn);
          return req;
        },
        write(chunk) {
          record.writes.push(Buffer.from(chunk == null ? '' : chunk));
          record.events.push(['write']);
          return true;
        },
        abort() {
          record.aborted = true;
          record.events.push(['abort']);
          (listeners.abort || []).forEach((fn) => fn());
        },
        destroy() {
          record.destroyed = true;
          record.aborted = true;
          record.events.push(['destroy']);
        },
        fireTimeout() {
          record.events.push(['fireTimeout']);
          (listeners.timeout || []).forEach((fn) => fn());
        },
        end(chunk) {
          if (chunk != null) record.writes.push(Buffer.from(chunk));
          record.ended = true;
          record.events.push(['end']);
          const result = resolver(record) || {};
          if (result.hold) {
            record.release = (outcome) => complete(outcome || {});
            return;
          }
          complete(result);
        },
      };
      requests.push(record);
      record.req = req;
      return req;
    },
  };
}

function createTimerHarness() {
  const timers = new Map();
  let nextId = 1;
  return {
    timers,
    setTimeout(fn, ms) {
      const id = nextId;
      nextId += 1;
      timers.set(id, { fn, ms });
      return id;
    },
    clearTimeout(id) {
      timers.delete(id);
    },
    fire(id) {
      const timer = timers.get(id);
      assert.ok(timer, 'missing timer');
      timers.delete(id);
      timer.fn();
    },
  };
}

function connectTimeouts(record) {
  return record.timeouts.concat(record.options.timeout == null ? [] : [record.options.timeout]);
}

function fireTimerMs(timers, ms) {
  const found = [...timers.timers.entries()].find(([, timer]) => timer.ms === ms);
  assert.ok(found, `missing ${ms}ms timer`);
  timers.fire(found[0]);
}

function assertPinnedAttempt(record, pin) {
  const options = record.options;
  const hostname = options.hostname || options.host;
  const port = options.port == null || options.port === '' ? 443 : Number(options.port);
  const protocol = options.protocol || 'https:';
  assert.strictEqual(protocol, 'https:');
  assert.strictEqual(port, 443);
  assert.strictEqual(options.method || 'GET', 'GET');
  assert.strictEqual(hostname, pin.hostname);
  assert.strictEqual(options.path, pin.path);
  assert.strictEqual(options.minVersion, 'TLSv1.2');
  assert.strictEqual(options.rejectUnauthorized, true);
  assert.ok(options.maxRedirects === 0 || options.followRedirect === false);
  assert.strictEqual(options.maxHeaderSize, 16384);
  assert.ok(connectTimeouts(record).includes(5000), 'missing 5-second connect timeout');
  assert.deepStrictEqual(record.writes, []);
  const headers = headerMap(record);
  assert.strictEqual(headers.accept, SAFE_HEADERS.accept);
  assert.strictEqual(headers['accept-encoding'], SAFE_HEADERS['accept-encoding']);
  for (const forbidden of ['cookie', 'referer', 'authorization', 'x-api-key', 'api-key']) {
    assert.ok(!Object.prototype.hasOwnProperty.call(headers, forbidden), `request sent ${forbidden}`);
  }
  const observed = Buffer.from(JSON.stringify({
    protocol, port, hostname, path: options.path, method: options.method || 'GET', headers, writes: record.writes,
  }));
  assert.ok(observed.includes(Buffer.from(pin.hostname)));
  assert.ok(observed.includes(Buffer.from(pin.path)));
  assertNoCanary(observed, 'https request');
  assertNoCanary(record.writes, 'https body');
}

function assertAbortedUnavailable(record, snapshot, asset) {
  assert.ok(record.aborted || record.destroyed, 'request was not aborted');
  assertUnavailableSnapshot(snapshot, asset);
}

test('fixture: recorded prices stay JSON strings that IEEE-754 cannot preserve', () => {
  assert.strictEqual(fixture.bodies.coinbase_zec_valid.price, fixture.prices.zec);
  assert.strictEqual(fixture.bodies.kraken_xmr_valid.price, fixture.prices.xmr);
  assert.notStrictEqual(Number(fixture.prices.zec), fixture.prices.zec);
  assert.notStrictEqual(Number(fixture.prices.xmr), fixture.prices.xmr);
  const numericCoinbase = JSON.parse(fixture.bodies.coinbase_numeric_price.raw_json);
  assert.strictEqual(typeof numericCoinbase.price, 'number');
  assert.notStrictEqual(String(numericCoinbase.price), fixture.prices.zec);
});

test('modules export exactly the closed quote-worker surface', () => {
  assert.deepStrictEqual(Object.keys(require('../quote-worker/providers')).sort(), ['PROVIDERS']);
  assert.deepStrictEqual(Object.keys(require('../quote-worker/model')).sort(), [
    'buildRateSnapshot', 'formatFiatEstimate', 'normalizeRateQuery', 'parseProviderBody',
  ]);
  assert.deepStrictEqual(Object.keys(require('../quote-worker/framing')).sort(), [
    'RATE_FRAME_LIMIT', 'createRateFrameDecoder', 'encodeRateFrame',
    'validateRateRequest', 'validateRateResponse',
  ]);
  assert.deepStrictEqual(Object.keys(require('../quote-worker/worker')).sort(), ['createQuoteWorker']);
  assert.strictEqual(typeof createQuoteWorker, 'function');
  assert.strictEqual(RATE_FRAME_LIMIT, 65536);
});

test('providers: closed Coinbase-ZEC and Kraken-XMR pins are frozen and default-off', () => {
  assert.ok(Object.isFrozen(PROVIDERS));
  assert.deepStrictEqual(Object.keys(PROVIDERS).sort(), [COINBASE_ID, KRAKEN_ID].sort());
  const coinbase = PROVIDERS[COINBASE_ID];
  const kraken = PROVIDERS[KRAKEN_ID];
  assert.ok(Object.isFrozen(coinbase));
  assert.ok(Object.isFrozen(kraken));
  assert.strictEqual(coinbase.provider_id, COINBASE_ID);
  assert.strictEqual(coinbase.parser_id, 'coinbase-exchange-ticker-v1');
  assert.strictEqual(coinbase.asset, 'ZEC');
  assert.strictEqual(coinbase.asset_id, 'zec-zcash');
  assert.strictEqual(coinbase.quote_currency, 'USD');
  assert.strictEqual(coinbase.method, 'GET');
  assert.strictEqual(coinbase.url, fixture.https.coinbase.url);
  assert.strictEqual(coinbase.hostname, fixture.https.coinbase.hostname);
  assert.strictEqual(coinbase.path, fixture.https.coinbase.path);
  assert.strictEqual(coinbase.enabled_by_default, false);
  assert.strictEqual(kraken.provider_id, KRAKEN_ID);
  assert.strictEqual(kraken.parser_id, 'kraken-spot-ticker-v1');
  assert.strictEqual(kraken.asset, 'XMR');
  assert.strictEqual(kraken.asset_id, 'xmr-monero');
  assert.strictEqual(kraken.quote_currency, 'USD');
  assert.strictEqual(kraken.method, 'GET');
  assert.strictEqual(kraken.url, fixture.https.kraken.url);
  assert.strictEqual(kraken.hostname, fixture.https.kraken.hostname);
  assert.strictEqual(kraken.path, fixture.https.kraken.path);
  assert.strictEqual(kraken.result_pair, 'XXMRZUSD');
  assert.strictEqual(kraken.request_pair, 'XMRUSD');
  assert.strictEqual(kraken.enabled_by_default, false);
  for (const provider of [coinbase, kraken]) {
    assert.strictEqual(provider.tls_min_version, 'TLSv1.2');
    assert.strictEqual(provider.accept, 'application/json');
    assert.strictEqual(provider.accept_encoding, 'identity');
    assert.strictEqual(provider.max_redirects, 0);
    assert.strictEqual(provider.connect_timeout_ms, 5000);
    assert.strictEqual(provider.overall_timeout_ms, 10000);
    assert.strictEqual(provider.max_header_bytes, 16384);
    assert.strictEqual(provider.max_body_bytes, 65536);
    assert.strictEqual(provider.port, 443);
  }
  const serialized = JSON.stringify(PROVIDERS);
  for (const host of FORBIDDEN_HOSTS) {
    assert.ok(!serialized.toLowerCase().includes(host), `providers table contains ${host}`);
  }
  assert.throws(() => { PROVIDERS.extra = true; });
  assert.throws(() => { coinbase.enabled_by_default = true; });
});

test('query: only the two singleton USD queries normalize to fresh closed objects', () => {
  const zec = normalizeRateQuery(zecQuery());
  const xmr = normalizeRateQuery(xmrQuery());
  assert.deepStrictEqual(zec, fixture.queries.zec);
  assert.deepStrictEqual(xmr, fixture.queries.xmr);
  assertClosedKeys(zec, QUERY_KEYS);
  assertClosedKeys(xmr, QUERY_KEYS);
  assert.notStrictEqual(zec, zecQuery());
  for (const value of [
    null, undefined, 1, 'query', [], Object.create({ v: 1 }),
    Object.assign(zecQuery(), { extra: true }),
    Object.assign(zecQuery(), { v: 2 }),
    Object.assign(zecQuery(), { asset_ids: [] }),
    Object.assign(zecQuery(), { asset_ids: ['zec-zcash', 'xmr-monero'] }),
    Object.assign(zecQuery(), { asset_ids: ['btc-bitcoin'] }),
    Object.assign(zecQuery(), { quote_currencies: ['EUR'] }),
    Object.assign(zecQuery(), { quote_currencies: ['USD', 'EUR'] }),
    Object.assign(zecQuery(), { quote_currencies: [] }),
    Object.assign(zecQuery(), { account_id: CANARIES.account_id }),
    Object.assign(zecQuery(), { amount_atomic: CANARIES.amount_atomic }),
    Object.assign(zecQuery(), { address: CANARIES.address }),
  ]) {
    assertCode(() => normalizeRateQuery(value), 'SCHEMA');
  }
  let getterCalls = 0;
  const accessor = zecQuery();
  Object.defineProperty(accessor, 'asset_ids', {
    enumerable: true,
    get() { getterCalls += 1; return ['zec-zcash']; },
  });
  assertCode(() => normalizeRateQuery(accessor), 'SCHEMA');
  assert.strictEqual(getterCalls, 0);
});

test('parser: recorded Coinbase ZEC and Kraken XMR bodies become exact closed quotes', () => {
  const zec = parseProviderBody(COINBASE_ID, bodyBytes(fixture.bodies.coinbase_zec_valid), FETCHED_AT);
  const xmr = parseProviderBody(KRAKEN_ID, bodyBytes(fixture.bodies.kraken_xmr_valid), FETCHED_AT);
  assert.deepStrictEqual(zec, expectedZecQuote());
  assert.deepStrictEqual(xmr, expectedXmrQuote());
  assertClosedKeys(zec, QUOTE_KEYS);
  assertClosedKeys(xmr, QUOTE_KEYS);
  assert.strictEqual(zec.price, fixture.prices.zec);
  assert.strictEqual(xmr.price, fixture.prices.xmr);
  assert.notStrictEqual(String(Number(zec.price)), zec.price);
  assert.notStrictEqual(String(Number(xmr.price)), xmr.price);
  const trailing = parseProviderBody(
    COINBASE_ID,
    bodyBytes(fixture.bodies.coinbase_zec_trailing_zeros),
    FETCHED_AT
  );
  assert.strictEqual(trailing.price, fixture.prices.zec_canonical_expected);
  const depthOk = parseProviderBody(COINBASE_ID, bodyBytes(fixture.bodies.depth_eight_ok), FETCHED_AT);
  assert.strictEqual(depthOk.price, '42.1');
  const twelve = parseProviderBody(COINBASE_ID, bodyBytes(fixture.bodies.coinbase_decimal_12_whole), FETCHED_AT);
  assert.strictEqual(twelve.price, '999999999999.1');
  const eighteen = parseProviderBody(COINBASE_ID, bodyBytes(fixture.bodies.coinbase_decimal_18_frac), FETCHED_AT);
  assert.strictEqual(eighteen.price, '1.123456789012345678');
});

test('parser: decimal digit bounds and strict Coinbase timestamps fail closed', () => {
  const malformed = [
    'coinbase_numeric_price', 'coinbase_missing_price', 'coinbase_missing_time',
    'coinbase_time_offset', 'coinbase_time_leap_second', 'coinbase_time_too_old',
    'coinbase_time_too_future', 'coinbase_zero_price', 'coinbase_exponent_price',
    'coinbase_decimal_13_whole', 'coinbase_decimal_19_frac', 'coinbase_decimal_all_zero_frac',
    'coinbase_decimal_sign', 'coinbase_decimal_plus_sign', 'coinbase_decimal_whitespace',
    'coinbase_decimal_leading_zero', 'coinbase_time_feb30', 'coinbase_time_2019',
    'coinbase_time_2101', 'coinbase_time_10_frac', 'coinbase_time_lowercase_z',
    'kraken_error_array', 'kraken_wrong_pair_xxbtzusd', 'kraken_alias_xmrusd',
    'kraken_extra_pair', 'kraken_numeric_last', 'kraken_malformed_ticker_array',
    'duplicate_key_coinbase', 'duplicate_key_kraken', 'depth_nine_fail',
    'trailing_json', 'empty_object', 'malformed_utf8',
  ];
  for (const name of malformed) {
    const result = parseProviderBody(fixture.bodies[name].provider_id, bodyBytes(fixture.bodies[name]), FETCHED_AT);
    assert.strictEqual(result, null, `${name} produced a quote`);
  }
  assert.strictEqual(parseProviderBody(COINBASE_ID, bodyBytes(fixture.bodies.oversize), FETCHED_AT), null);
  assert.strictEqual(parseProviderBody(COINBASE_ID, bodyBytes(fixture.bodies.exact_limit), FETCHED_AT).price, '1');
  const zeroFrac = parseProviderBody(COINBASE_ID, bodyBytes(fixture.bodies.coinbase_time_zero_frac), FETCHED_AT);
  assert.strictEqual(zeroFrac.provider_observed_at, FETCHED_AT);
  const nineFrac = parseProviderBody(COINBASE_ID, bodyBytes(fixture.bodies.coinbase_time_nine_frac), FETCHED_AT);
  assert.strictEqual(nineFrac.provider_observed_at, FETCHED_AT);
  const validBody = bodyBytes(fixture.bodies.coinbase_time_zero_frac);
  for (const fetchedAt of [
    'not-a-time', '2026-02-30T12:00:00Z', '2026-09-01T12:00:00.1Z',
    '2026-09-01T12:00:00+00:00', '2026-09-01T12:00:00z', '2019-12-31T12:00:00Z',
  ]) {
    assert.strictEqual(parseProviderBody(COINBASE_ID, validBody, fetchedAt), null, `fetched_at ${fetchedAt}`);
  }
  assert.strictEqual(parseProviderBody('unknown-v1', validBody, FETCHED_AT), null);
  assert.strictEqual(parseProviderBody(KRAKEN_ID, bodyBytes(fixture.bodies.coinbase_zec_valid), FETCHED_AT), null);
  assert.strictEqual(parseProviderBody(COINBASE_ID, bodyBytes(fixture.bodies.kraken_xmr_valid), FETCHED_AT), null);
});

test('parser: Coinbase observation truncates RFC 3339 fractions and never rounds', () => {
  const body = Buffer.from('{"price":"42.1","time":"2026-09-01T12:00:00.999999999Z"}', 'utf8');
  const quote = parseProviderBody(COINBASE_ID, body, FETCHED_AT);
  assert.strictEqual(quote.provider_observed_at, '2026-09-01T12:00:00Z');
  const boundaryOld = parseProviderBody(
    COINBASE_ID,
    Buffer.from('{"price":"42.1","time":"2026-09-01T11:50:00Z"}', 'utf8'),
    FETCHED_AT
  );
  assert.strictEqual(boundaryOld.provider_observed_at, '2026-09-01T11:50:00Z');
  const pastOld = parseProviderBody(
    COINBASE_ID,
    Buffer.from('{"price":"42.1","time":"2026-09-01T11:49:59Z"}', 'utf8'),
    FETCHED_AT
  );
  assert.strictEqual(pastOld, null);
  const boundaryFuture = parseProviderBody(
    COINBASE_ID,
    Buffer.from('{"price":"42.1","time":"2026-09-01T12:05:00Z"}', 'utf8'),
    FETCHED_AT
  );
  assert.strictEqual(boundaryFuture.provider_observed_at, '2026-09-01T12:05:00Z');
  const pastFuture = parseProviderBody(
    COINBASE_ID,
    Buffer.from('{"price":"42.1","time":"2026-09-01T12:05:01Z"}', 'utf8'),
    FETCHED_AT
  );
  assert.strictEqual(pastFuture, null);
});

test('snapshot: hostile, stale, expired, and non-closed quotes are omitted', () => {
  const zecQuote = expectedZecQuote();
  const xmrQuote = expectedXmrQuote();
  const zecFresh = buildRateSnapshot(normalizeRateQuery(zecQuery()), [zecQuote], FETCHED_AT);
  assert.deepStrictEqual(zecFresh, {
    v: 1,
    queried_at: FETCHED_AT,
    quotes: [zecQuote],
    display: freshDisplay('ZEC', fixture.prices.zec, COINBASE_ID),
  });
  assertClosedKeys(zecFresh, SNAPSHOT_KEYS);
  assertClosedKeys(zecFresh.display, DISPLAY_FRESH_KEYS);
  const xmrFresh = buildRateSnapshot(normalizeRateQuery(xmrQuery()), [xmrQuote], FETCHED_AT);
  assert.deepStrictEqual(xmrFresh.quotes, [xmrQuote]);
  assert.deepStrictEqual(xmrFresh.display, freshDisplay('XMR', fixture.prices.xmr, KRAKEN_ID));
  const atBound = buildRateSnapshot(normalizeRateQuery(zecQuery()), [zecQuote], FRESH_UNTIL);
  assert.deepStrictEqual(atBound.quotes, [zecQuote]);
  assert.deepStrictEqual(atBound.display.method, 'single_labeled_source');
  const pastBound = buildRateSnapshot(normalizeRateQuery(zecQuery()), [zecQuote], '2026-09-01T12:05:01Z');
  assertUnavailableSnapshot(pastBound, 'ZEC');
  const hostiles = [
    [],
    [xmrQuote],
    [zecQuote, clone(zecQuote)],
    [zecQuote, xmrQuote],
    [Object.assign(clone(zecQuote), { provider_id: KRAKEN_ID })],
    [Object.assign(clone(zecQuote), { asset: 'XMR', asset_id: 'xmr-monero' })],
    [Object.assign(clone(zecQuote), { quote_currency: 'EUR' })],
    [Object.assign(clone(zecQuote), { status: 'stale' })],
    [Object.assign(clone(zecQuote), { expires_at: '2026-09-01T11:59:59Z', fresh_until: '2026-09-01T11:59:59Z' })],
    [Object.assign(clone(zecQuote), { provider_observed_at: 'not-a-time' })],
    [Object.assign(clone(zecQuote), { extra: true })],
    [Object.create(zecQuote)],
    [Object.assign(clone(zecQuote), { price: '0' })],
  ];
  for (const quotes of hostiles) {
    assertUnavailableSnapshot(buildRateSnapshot(normalizeRateQuery(zecQuery()), quotes, FETCHED_AT), 'ZEC');
  }
  let getterCalls = 0;
  const accessor = clone(zecQuote);
  Object.defineProperty(accessor, 'price', {
    enumerable: true,
    get() { getterCalls += 1; return fixture.prices.zec; },
  });
  assertUnavailableSnapshot(
    buildRateSnapshot(normalizeRateQuery(zecQuery()), [accessor], FETCHED_AT),
    'ZEC'
  );
  assert.strictEqual(getterCalls, 0);
  const withSymbol = clone(zecQuote);
  withSymbol[Symbol('secret')] = CANARIES.secret;
  assertUnavailableSnapshot(
    buildRateSnapshot(normalizeRateQuery(zecQuery()), [withSymbol], FETCHED_AT),
    'ZEC'
  );
  assert.notStrictEqual(zecFresh.quotes[0], zecQuote);
  zecFresh.quotes[0].price = '0';
  assert.strictEqual(zecQuote.price, fixture.prices.zec);
});

test('formatFiatEstimate: integer decimal conversion uses round-half-even and two fractional digits', () => {
  assert.strictEqual(formatFiatEstimate('100000000', 8, '42.50', 'USD'), '42.50');
  assert.strictEqual(formatFiatEstimate('100000000', 8, fixture.prices.round_even, 'USD'), fixture.prices.round_even_expected);
  assert.strictEqual(formatFiatEstimate('100000000', 8, fixture.prices.round_odd, 'USD'), fixture.prices.round_odd_expected);
  assert.strictEqual(formatFiatEstimate('100000000', 8, '1.225000000000000001', 'USD'), '1.23');
  assert.strictEqual(formatFiatEstimate('100000000', 8, '1.224999999999999999', 'USD'), '1.22');
  assert.strictEqual(formatFiatEstimate('0', 8, '42.50', 'USD'), '0.00');
  assert.strictEqual(formatFiatEstimate('1', 8, '42.50', 'USD'), '0.00');
  assert.strictEqual(formatFiatEstimate('1000000000000', 12, '158.99', 'USD'), '158.99');
  assert.strictEqual(formatFiatEstimate('1000000000000', 12, fixture.prices.round_even, 'USD'), fixture.prices.round_even_expected);
  assert.strictEqual(formatFiatEstimate('100000000', 12, '1.00', 'USD'), '0.00');
  assert.strictEqual(formatFiatEstimate('1000000000000', 8, '1.00', 'USD'), '10000.00');
  assert.strictEqual(
    formatFiatEstimate('100000000', 8, fixture.prices.zec, 'USD'),
    formatOracle('100000000', 8, fixture.prices.zec)
  );
  assert.strictEqual(
    formatFiatEstimate('1000000000000', 12, fixture.prices.xmr, 'USD'),
    formatOracle('1000000000000', 12, fixture.prices.xmr)
  );
  assert.strictEqual(
    formatFiatEstimate('18446744073709551615', 8, '1.00', 'USD'),
    formatOracle('18446744073709551615', 8, '1.00')
  );
  assert.strictEqual(
    formatFiatEstimate('18446744073709551615', 12, '1.00', 'USD'),
    formatOracle('18446744073709551615', 12, '1.00')
  );
  const amount = '100000000';
  const price = fixture.prices.zec;
  formatFiatEstimate(amount, 8, price, 'USD');
  assert.strictEqual(amount, '100000000');
  assert.strictEqual(price, fixture.prices.zec);
  for (const [atomic, exponent, unitPrice, quote] of [
    ['01', 8, '1.00', 'USD'],
    ['-1', 8, '1.00', 'USD'],
    ['18446744073709551616', 8, '1.00', 'USD'],
    ['100000000', 7, '1.00', 'USD'],
    ['100000000', 9, '1.00', 'USD'],
    ['100000000', 8, '1.00', 'EUR'],
    ['100000000', 8, '0', 'USD'],
    ['100000000', 8, '-1.00', 'USD'],
    ['100000000', 8, '1e2', 'USD'],
    ['100000000', 8, '1.2345678901234567890', 'USD'],
    ['100000000', 8, Number(fixture.prices.zec), 'USD'],
  ]) {
    assertCode(() => formatFiatEstimate(atomic, exponent, unitPrice, quote), 'SCHEMA');
  }
});

test('framing: independently encoded rate frames match production bytes and stay at 64 KiB', () => {
  const request = {
    v: 1,
    id: FRAME_ID,
    kind: 'req',
    method: 'rate.query',
    params: fixture.queries.zec,
  };
  const snapshot = buildRateSnapshot(normalizeRateQuery(zecQuery()), [expectedZecQuote()], FETCHED_AT);
  const response = {
    v: 1,
    id: FRAME_ID,
    kind: 'res',
    method: 'rate.snapshot',
    result: snapshot,
  };
  assert.deepStrictEqual(validateRateRequest(clone(request)), request);
  assert.deepStrictEqual(validateRateResponse(clone(response)), response);
  const expectedRequest = independentFrame(request);
  const expectedResponse = independentFrame(response);
  assert.deepStrictEqual(encodeRateFrame(request), expectedRequest);
  assert.deepStrictEqual(encodeRateFrame(response), expectedResponse);
  assert.ok(expectedRequest.length > 4);
  assert.strictEqual(expectedRequest.readUInt32BE(0), expectedRequest.length - 4);
  const decoder = createRateFrameDecoder();
  assert.deepStrictEqual(decoder.push(expectedRequest.subarray(0, 2)), []);
  assert.deepStrictEqual(decoder.push(expectedRequest.subarray(2)), [request]);
  assert.deepStrictEqual(decoder.push(Buffer.concat([expectedResponse, expectedRequest])), [response, request]);
  assert.strictEqual(RATE_FRAME_LIMIT, 65536);
  const over = createRateFrameDecoder();
  const prefix = Buffer.alloc(4);
  prefix.writeUInt32BE(RATE_FRAME_LIMIT + 1, 0);
  assertCode(() => over.push(prefix), 'LIMIT');
  assert.strictEqual(over.closed, true);
  const requestMutations = [
    Object.assign(clone(request), { extra: true }),
    Object.assign(clone(request), { method: 'intent.begin' }),
    Object.assign(clone(request), { method: 'rate.fetch' }),
    Object.assign(clone(request), { kind: 'evt' }),
    Object.assign(clone(request), { id: 'AA'.repeat(16) }),
    Object.assign(clone(request), { id: 'a'.repeat(31) }),
    Object.assign(clone(request), { params: Object.assign(zecQuery(), { account_id: CANARIES.account_id }) }),
  ];
  for (const extra of requestMutations) {
    assertCode(() => validateRateRequest(extra), 'SCHEMA');
  }
  const responseMutations = [
    Object.assign(clone(response), { extra: true }),
    Object.assign(clone(response), { method: 'rate.query' }),
    Object.assign(clone(response), { kind: 'req' }),
    Object.assign(clone(response), { id: 'AA'.repeat(16) }),
    Object.assign(clone(response), { result: Object.assign(clone(snapshot), { extra: true }) }),
  ];
  for (const extra of responseMutations) {
    assertCode(() => validateRateResponse(extra), 'SCHEMA');
  }
  const duplicateKey = createRateFrameDecoder();
  const dupBody = Buffer.from('{"v":1,"v":1}', 'utf8');
  const dupPrefix = Buffer.alloc(4);
  dupPrefix.writeUInt32BE(dupBody.length, 0);
  assertCode(() => duplicateKey.push(Buffer.concat([dupPrefix, dupBody])), 'SCHEMA');
});

test('worker: default-off query never contacts a provider and returns a closed unavailable snapshot', () => {
  const https = createHttpsFake(() => { throw new Error('network reached'); });
  const timers = createTimerHarness();
  const logs = [];
  const worker = createQuoteWorker({
    now: () => Date.UTC(2026, 8, 1, 12, 0, 0),
    https,
    setTimeout: timers.setTimeout,
    clearTimeout: timers.clearTimeout,
    log: (line) => logs.push(String(line)),
  });
  const snapshot = worker.query(zecQuery());
  assertUnavailableSnapshot(snapshot, 'ZEC');
  assert.strictEqual(https.requests.length, 0);
  worker.shutdown();
  assert.strictEqual(timers.timers.size, 0);
  assertNoCanary(snapshot, 'default snapshot');
  assertNoCanary(logs, 'default logs');
});

test('worker: enabled Coinbase and Kraken fetches use exact pinned requests and closed snapshots', () => {
  const bodies = {
    [fixture.https.coinbase.path]: bodyBytes(fixture.bodies.coinbase_zec_valid),
    [fixture.https.kraken.path]: bodyBytes(fixture.bodies.kraken_xmr_valid),
  };
  const https = createHttpsFake((record) => {
    const requestPath = record.options.path;
    assert.ok(Object.prototype.hasOwnProperty.call(bodies, requestPath), `unpinned path ${requestPath}`);
    return { statusCode: 200, body: bodies[requestPath], headers: { 'content-type': 'application/json' } };
  });
  const timers = createTimerHarness();
  const logs = [];
  const worker = createQuoteWorker({
    now: () => Date.UTC(2026, 8, 1, 12, 0, 0),
    https,
    setTimeout: timers.setTimeout,
    clearTimeout: timers.clearTimeout,
    enabledProviders: [COINBASE_ID, KRAKEN_ID],
    log: (line) => logs.push(String(line)),
  });
  const zecSnapshot = worker.query(zecQuery());
  const xmrSnapshot = worker.query(xmrQuery());
  assert.deepStrictEqual(zecSnapshot.display, freshDisplay('ZEC', fixture.prices.zec, COINBASE_ID));
  assert.deepStrictEqual(xmrSnapshot.display, freshDisplay('XMR', fixture.prices.xmr, KRAKEN_ID));
  assert.strictEqual(zecSnapshot.quotes[0].price, fixture.prices.zec);
  assert.strictEqual(xmrSnapshot.quotes[0].price, fixture.prices.xmr);
  assert.strictEqual(https.requests.length, 2);
  assertPinnedAttempt(https.requests[0], fixture.https.coinbase);
  assertPinnedAttempt(https.requests[1], fixture.https.kraken);
  for (const record of https.requests) {
    assert.notStrictEqual(record.options.path, fixture.bodies.coinbase_wrong_product_eth.request_path);
    assert.notStrictEqual(record.options.path, fixture.bodies.coinbase_wrong_product_xmr.request_path);
    assert.notStrictEqual(record.options.path, '/0/public/Ticker?pair=XBTUSD');
  }
  worker.shutdown();
  assert.strictEqual(timers.timers.size, 0);
  assertNoCanary(zecSnapshot, 'zec snapshot');
  assertNoCanary(xmrSnapshot, 'xmr snapshot');
  assertNoCanary(logs, 'worker logs');
  assertNoCanary(https.requests, 'https records');
});

test('worker: TLS, non-200, redirect, and wrong content type fail closed without retry', () => {
  const cases = [
    {
      name: 'tls',
      result: { error: Object.assign(new Error('TLS'), { code: 'UNABLE_TO_VERIFY_LEAF_SIGNATURE' }) },
    },
    {
      name: 'status',
      result: { statusCode: 500, body: '{}' },
    },
    {
      name: 'redirect',
      result: { statusCode: 302, headers: { location: 'https://evil.example/x' }, body: '{}' },
    },
    {
      name: 'content-type',
      result: {
        statusCode: 200,
        headers: { 'content-type': 'text/html' },
        body: fixture.bodies.coinbase_zec_valid.raw_json,
      },
    },
  ];
  for (const item of cases) {
    const https = createHttpsFake(() => item.result);
    const timers = createTimerHarness();
    const worker = createQuoteWorker({
      now: () => Date.UTC(2026, 8, 1, 12, 0, 0),
      https,
      setTimeout: timers.setTimeout,
      clearTimeout: timers.clearTimeout,
      enabledProviders: [COINBASE_ID],
    });
    const snapshot = worker.query(zecQuery());
    assert.strictEqual(https.requests.length, 1, item.name);
    assertPinnedAttempt(https.requests[0], fixture.https.coinbase);
    assertUnavailableSnapshot(snapshot, 'ZEC');
    assert.strictEqual(https.requests.length, 1, `${item.name} followed a redirect or retried`);
    const cached = worker.query(zecQuery());
    assert.strictEqual(https.requests.length, 1, `${item.name} cached a failed quote`);
    assertUnavailableSnapshot(cached, 'ZEC');
    worker.shutdown();
    assert.strictEqual(timers.timers.size, 0);
  }
});

test('worker: header and body limits abort an active response and fail closed', () => {
  const cases = [
    {
      name: 'header-limit',
      result: {
        statusCode: 200,
        headers: { 'content-type': 'application/json', 'x-pad': 'a'.repeat(16384) },
        body: fixture.bodies.coinbase_zec_valid.raw_json,
      },
    },
    {
      name: 'body-limit',
      result: { statusCode: 200, body: bodyBytes(fixture.bodies.oversize) },
    },
  ];
  for (const item of cases) {
    const https = createHttpsFake(() => item.result);
    const timers = createTimerHarness();
    const worker = createQuoteWorker({
      now: () => Date.UTC(2026, 8, 1, 12, 0, 0),
      https,
      setTimeout: timers.setTimeout,
      clearTimeout: timers.clearTimeout,
      enabledProviders: [COINBASE_ID],
    });
    const snapshot = worker.query(zecQuery());
    assert.strictEqual(https.requests.length, 1, item.name);
    assertPinnedAttempt(https.requests[0], fixture.https.coinbase);
    assertAbortedUnavailable(https.requests[0], snapshot, 'ZEC');
    assert.strictEqual(https.requests.length, 1, `${item.name} retried after a limit breach`);
    worker.shutdown();
    assert.strictEqual(timers.timers.size, 0);
  }
});

test('worker: a distinct 10-second overall timer aborts an in-flight request and starts 30-second backoff', () => {
  let fetchCount = 0;
  const https = createHttpsFake(() => {
    fetchCount += 1;
    return { hold: true };
  });
  const timers = createTimerHarness();
  const worker = createQuoteWorker({
    now: () => Date.UTC(2026, 8, 1, 12, 0, 0),
    https,
    setTimeout: timers.setTimeout,
    clearTimeout: timers.clearTimeout,
    enabledProviders: [COINBASE_ID],
  });
  const snapshot = worker.query(zecQuery());
  assert.strictEqual(fetchCount, 1);
  assertPinnedAttempt(https.requests[0], fixture.https.coinbase);
  assert.ok(connectTimeouts(https.requests[0]).includes(5000), 'missing 5-second connect timeout');
  assert.ok([...timers.timers.values()].some((timer) => timer.ms === 10000), 'missing 10-second overall timer');
  fireTimerMs(timers, 10000);
  assert.ok(https.requests[0].aborted || https.requests[0].destroyed, 'overall timer did not abort the request');
  assertUnavailableSnapshot(snapshot, 'ZEC');
  assert.ok([...timers.timers.values()].some((timer) => timer.ms === 30000), 'missing 30-second backoff');
  const blocked = worker.query(zecQuery());
  assert.strictEqual(fetchCount, 1, 'fetched before 30-second backoff expired');
  assertUnavailableSnapshot(blocked, 'ZEC');
  worker.shutdown();
  assert.strictEqual(timers.timers.size, 0);
});

test('worker: only one HTTPS request per provider may be in flight', () => {
  let fetchCount = 0;
  const https = createHttpsFake(() => {
    fetchCount += 1;
    return { hold: true };
  });
  const timers = createTimerHarness();
  const worker = createQuoteWorker({
    now: () => Date.UTC(2026, 8, 1, 12, 0, 0),
    https,
    setTimeout: timers.setTimeout,
    clearTimeout: timers.clearTimeout,
    enabledProviders: [COINBASE_ID],
  });
  const first = worker.query(zecQuery());
  assert.strictEqual(https.requests.length, 1);
  assertUnavailableSnapshot(first, 'ZEC');
  const second = worker.query(zecQuery());
  assert.strictEqual(https.requests.length, 1);
  assert.strictEqual(fetchCount, 1);
  assertUnavailableSnapshot(second, 'ZEC');
  https.requests[0].release({ error: Object.assign(new Error('timeout'), { code: 'ETIMEDOUT' }) });
  assert.ok([...timers.timers.values()].some((timer) => timer.ms === 30000), 'missing backoff after in-flight failure');
  const blocked = worker.query(zecQuery());
  assert.strictEqual(fetchCount, 1);
  assertUnavailableSnapshot(blocked, 'ZEC');
  worker.shutdown();
  assert.strictEqual(timers.timers.size, 0);
});

test('worker: backoff is causal, capped at one hour, then Kraken cache lasts five minutes', () => {
  let now = Date.UTC(2026, 8, 1, 12, 0, 0);
  let fetchCount = 0;
  const https = createHttpsFake(() => {
    fetchCount += 1;
    if (fetchCount < 9) return { error: Object.assign(new Error('timeout'), { code: 'ETIMEDOUT' }) };
    return {
      statusCode: 200,
      body: bodyBytes(fixture.bodies.kraken_xmr_valid),
      headers: { 'content-type': 'application/json' },
    };
  });
  const timers = createTimerHarness();
  const worker = createQuoteWorker({
    now: () => now,
    https,
    setTimeout: timers.setTimeout,
    clearTimeout: timers.clearTimeout,
    enabledProviders: [KRAKEN_ID],
  });
  const expectedDelays = [30000, 60000, 120000, 240000, 480000, 960000, 1920000, 3600000];
  let previousFetches = 0;
  for (const delay of expectedDelays) {
    const failed = worker.query(xmrQuery());
    assertUnavailableSnapshot(failed, 'XMR');
    assert.strictEqual(fetchCount, previousFetches + 1);
    previousFetches = fetchCount;
    assert.strictEqual([...timers.timers.values()][0].ms, delay);
    const blocked = worker.query(xmrQuery());
    assert.strictEqual(fetchCount, previousFetches, `fetched before ${delay}ms delay expired`);
    assertUnavailableSnapshot(blocked, 'XMR');
    now += delay;
    timers.fire([...timers.timers.keys()][0]);
  }
  const recovered = worker.query(xmrQuery());
  assert.deepStrictEqual(recovered.quotes.length, 1);
  assert.deepStrictEqual(recovered.display, freshDisplay('XMR', fixture.prices.xmr, KRAKEN_ID));
  assert.strictEqual(recovered.quotes[0].fetched_at, timestampFromMs(now));
  assert.strictEqual(recovered.quotes[0].provider_observed_at, timestampFromMs(now));
  const cachedFetches = fetchCount;
  const cached = worker.query(xmrQuery());
  assert.strictEqual(cached.quotes[0].price, fixture.prices.xmr);
  assert.strictEqual(fetchCount, cachedFetches);
  now += 5 * 60 * 1000;
  const atBound = worker.query(xmrQuery());
  assert.strictEqual(fetchCount, cachedFetches);
  assert.strictEqual(atBound.quotes[0].price, fixture.prices.xmr);
  now += 1000;
  const expired = worker.query(xmrQuery());
  assert.ok(fetchCount > cachedFetches);
  assert.ok(expired);
  worker.shutdown();
  assert.strictEqual(timers.timers.size, 0);
});

test('worker: complete private-context canaries never appear in query, logs, or HTTPS', () => {
  const https = createHttpsFake(() => ({
    statusCode: 200,
    body: bodyBytes(fixture.bodies.coinbase_zec_valid),
  }));
  const timers = createTimerHarness();
  const logs = [];
  const worker = createQuoteWorker({
    now: () => Date.UTC(2026, 8, 1, 12, 0, 0),
    https,
    setTimeout: timers.setTimeout,
    clearTimeout: timers.clearTimeout,
    enabledProviders: [COINBASE_ID],
    env: Object.assign({ HTTP_PROXY: CANARIES.proxy, API_KEY: CANARIES.api_key, COOKIE: CANARIES.cookie }, CANARIES),
    log: (line) => logs.push(String(line)),
  });
  const snapshot = worker.query(zecQuery());
  assert.deepStrictEqual(snapshot.display.source_ids, [COINBASE_ID]);
  assertNoCanary(snapshot, 'canary snapshot');
  assertNoCanary(logs, 'canary logs');
  assertNoCanary(https.requests, 'canary https');
  assertCode(() => normalizeRateQuery(Object.assign(zecQuery(), CANARIES)), 'SCHEMA');
  worker.shutdown();
  assert.strictEqual(timers.timers.size, 0);
});

test('worker child entry: zero provider arguments answers one framed query and exits without leftover process', () => {
  const workerPath = path.join(repoRoot, 'quote-worker', 'worker.js');
  const request = {
    v: 1,
    id: FRAME_ID,
    kind: 'req',
    method: 'rate.query',
    params: fixture.queries.zec,
  };
  const result = spawnSync(process.execPath, [workerPath], {
    input: independentFrame(request),
    timeout: 2000,
    maxBuffer: 65540,
    encoding: 'buffer',
    shell: false,
    env: { LANG: 'C.UTF-8' },
    windowsHide: true,
  });
  assert.strictEqual(result.error, undefined);
  assert.strictEqual(result.signal, null);
  assert.strictEqual(result.status, 0);
  assert.deepStrictEqual(result.stderr, Buffer.alloc(0));
  const response = decodeFrame(result.stdout);
  assert.deepStrictEqual(validateRateResponse(clone(response)), response);
  assert.strictEqual(response.id, FRAME_ID);
  assert.strictEqual(response.kind, 'res');
  assert.strictEqual(response.method, 'rate.snapshot');
  assertUnavailableSnapshot(response.result, 'ZEC');
  assert.deepStrictEqual(encodeRateFrame(response), result.stdout);
});

test('worker source does not import Electron, wallet, filesystem, or daemon modules', () => {
  const rels = [
    'quote-worker/providers.js',
    'quote-worker/model.js',
    'quote-worker/framing.js',
    'quote-worker/worker.js',
  ];
  for (const rel of rels) {
    const source = fs.readFileSync(path.join(repoRoot, rel), 'utf8');
    assert.ok(source.trim(), `${rel} is empty`);
    for (const specifier of FORBIDDEN_IMPORTS) {
      const pattern = new RegExp(`require\\((['"])${specifier.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')}\\1\\)|from\\s+(['"])${specifier.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')}\\2`);
      assert.ok(!pattern.test(source), `${rel} imports ${specifier}`);
    }
    if (rel !== 'quote-worker/worker.js') {
      assert.ok(!/require\((['"])https\1\)|require\((['"])node:https\2\)/.test(source), `${rel} performs HTTPS`);
      assert.ok(!/require\((['"])child_process\1\)/.test(source), `${rel} spawns`);
    }
  }
  const model = fs.readFileSync(path.join(repoRoot, 'quote-worker/model.js'), 'utf8');
  assert.ok(!/parseFloat\(/.test(model), 'model uses parseFloat');
  assert.ok(!/Number\(price/.test(model), 'model coerces price through Number');
  assert.ok(!/JSON\.parse\(/.test(model), 'model uses JSON.parse which cannot reject duplicate keys');
});

function run() {
  let failed = 0;
  for (const { name, fn } of tests) {
    try {
      fn();
      process.stdout.write(`ok ${name}\n`);
    } catch (error) {
      failed += 1;
      process.stderr.write(`not ok ${name}\n${error && error.stack ? error.stack : error}\n`);
    }
  }
  if (failed) process.exit(1);
  process.stdout.write(`BitBook rate worker tests passed (${tests.length}).\n`);
}

if (require.main === module) run();
module.exports = { tests };
