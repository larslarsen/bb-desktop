'use strict';

const { Buffer } = require('buffer');

const PINNED = Object.freeze({
  'coinbase-exchange-v1': Object.freeze({
    asset: 'ZEC', asset_id: 'zec-zcash', result_pair: null,
  }),
  'kraken-spot-v1': Object.freeze({
    asset: 'XMR', asset_id: 'xmr-monero', result_pair: 'XXMRZUSD',
  }),
});

const MAX_BODY = 65536;
const QUERY_KEYS = ['v', 'asset_ids', 'quote_currencies'];
const QUOTE_KEYS = [
  'v', 'asset', 'asset_id', 'quote_currency', 'price', 'provider_id',
  'provider_observed_at', 'fetched_at', 'fresh_until', 'expires_at', 'status',
];
const TIMESTAMP_RE = /^([0-9]{4})-([0-9]{2})-([0-9]{2})T([0-9]{2}):([0-9]{2}):([0-9]{2})Z$/;
const COINBASE_TIME_RE = /^([0-9]{4})-([0-9]{2})-([0-9]{2})T([0-9]{2}):([0-9]{2}):([0-9]{2})(\.[0-9]{1,9})?Z$/;
const WHOLE_DECIMAL = /^[1-9][0-9]{0,11}(?:\.[0-9]{1,18})?$/;
const FRAC_DECIMAL = /^0\.[0-9]{1,18}$/;
const ATOMIC_RE = /^(?:0|[1-9][0-9]{0,19})$/;
const U64_MAX = 18446744073709551615n;

function schema(message) {
  const error = new Error(message);
  error.code = 'SCHEMA';
  throw error;
}

function defineData(object, key, value) {
  if (key === '__proto__') schema('forbidden key');
  Object.defineProperty(object, key, {
    value,
    enumerable: true,
    writable: true,
    configurable: true,
  });
}

class BodyParser {
  constructor(text) {
    this.text = text;
    this.index = 0;
  }

  parse() {
    this.skipWhitespace();
    const value = this.parseValue(1);
    this.skipWhitespace();
    if (this.index !== this.text.length) schema('trailing JSON bytes');
    return value;
  }

  skipWhitespace() {
    while (this.index < this.text.length && /[\x20\x09\x0a\x0d]/.test(this.text[this.index])) {
      this.index += 1;
    }
  }

  parseValue(depth) {
    if (depth > 8) schema('JSON nesting exceeds depth 8');
    this.skipWhitespace();
    const char = this.text[this.index];
    if (char === '{') return this.parseObject(depth);
    if (char === '[') return this.parseArray(depth);
    if (char === '"') return this.parseString();
    if (char === '-' || (char >= '0' && char <= '9')) return this.parseNumeric();
    for (const [token, value] of [['true', true], ['false', false], ['null', null]]) {
      if (this.text.startsWith(token, this.index)) {
        this.index += token.length;
        return value;
      }
    }
    schema('invalid JSON value');
    return null;
  }

  parseObject(depth) {
    const value = {};
    const names = new Set();
    this.index += 1;
    this.skipWhitespace();
    if (this.text[this.index] === '}') {
      this.index += 1;
      return value;
    }
    while (this.index < this.text.length) {
      if (this.text[this.index] !== '"') schema('object key must be a string');
      const key = this.parseString();
      if (names.has(key)) schema(`duplicate object key ${key}`);
      names.add(key);
      this.skipWhitespace();
      if (this.text[this.index] !== ':') schema('object key is missing a colon');
      this.index += 1;
      defineData(value, key, this.parseValue(depth + 1));
      this.skipWhitespace();
      if (this.text[this.index] === '}') {
        this.index += 1;
        return value;
      }
      if (this.text[this.index] !== ',') schema('object is missing a comma');
      this.index += 1;
      this.skipWhitespace();
    }
    schema('unterminated object');
    return value;
  }

  parseArray(depth) {
    const value = [];
    this.index += 1;
    this.skipWhitespace();
    if (this.text[this.index] === ']') {
      this.index += 1;
      return value;
    }
    while (this.index < this.text.length) {
      value.push(this.parseValue(depth + 1));
      this.skipWhitespace();
      if (this.text[this.index] === ']') {
        this.index += 1;
        return value;
      }
      if (this.text[this.index] !== ',') schema('array is missing a comma');
      this.index += 1;
      this.skipWhitespace();
    }
    schema('unterminated array');
    return value;
  }

  parseString() {
    this.index += 1;
    let out = '';
    while (this.index < this.text.length) {
      const code = this.text.charCodeAt(this.index);
      const char = this.text[this.index];
      if (char === '"') {
        this.index += 1;
        return out;
      }
      if (code <= 0x1f) schema('unescaped control in JSON string');
      if (char === '\\') {
        this.index += 1;
        const esc = this.text[this.index];
        const map = { '"': '"', '\\': '\\', '/': '/', b: '\b', f: '\f', n: '\n', r: '\r', t: '\t' };
        if (esc === 'u') {
          const hex = this.text.slice(this.index + 1, this.index + 5);
          if (!/^[0-9a-fA-F]{4}$/.test(hex)) schema('invalid unicode escape');
          out += String.fromCharCode(parseInt(hex, 16));
          this.index += 5;
        } else if (Object.prototype.hasOwnProperty.call(map, esc)) {
          out += map[esc];
          this.index += 1;
        } else {
          schema('invalid escape');
        }
      } else {
        out += char;
        this.index += 1;
      }
    }
    schema('unterminated JSON string');
    return out;
  }

  parseNumeric() {
    const match = /^-?(?:0|[1-9][0-9]*)(?:\.[0-9]+)?(?:[eE][+-]?[0-9]+)?/.exec(this.text.slice(this.index));
    if (!match) schema('invalid JSON number');
    this.index += match[0].length;
    const value = Number(match[0]);
    if (!Number.isFinite(value)) schema('non-finite JSON number');
    return value;
  }
}

function daysInMonth(year, month) {
  const leap = year % 4 === 0 && (year % 100 !== 0 || year % 400 === 0);
  return [0, 31, leap ? 29 : 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31][month];
}

function calendarParts(year, month, day, hour, minute, second) {
  if (year < 2020 || year > 2100 || month < 1 || month > 12 || hour > 23 || minute > 59 || second > 59) {
    return null;
  }
  if (day < 1 || day > daysInMonth(year, month)) return null;
  return { year, month, day, hour, minute, second };
}

function parseTimestamp(value) {
  if (typeof value !== 'string') return null;
  const match = TIMESTAMP_RE.exec(value);
  if (!match) return null;
  const parts = calendarParts(
    Number(match[1]), Number(match[2]), Number(match[3]),
    Number(match[4]), Number(match[5]), Number(match[6])
  );
  if (!parts) return null;
  const ms = Date.UTC(parts.year, parts.month - 1, parts.day, parts.hour, parts.minute, parts.second);
  return { text: value, ms };
}

function parseCoinbaseTime(value) {
  if (typeof value !== 'string') return null;
  const match = COINBASE_TIME_RE.exec(value);
  if (!match) return null;
  const parts = calendarParts(
    Number(match[1]), Number(match[2]), Number(match[3]),
    Number(match[4]), Number(match[5]), Number(match[6])
  );
  if (!parts) return null;
  const pad = (n) => String(n).padStart(2, '0');
  const text = `${parts.year}-${pad(parts.month)}-${pad(parts.day)}T${pad(parts.hour)}:${pad(parts.minute)}:${pad(parts.second)}Z`;
  const ms = Date.UTC(parts.year, parts.month - 1, parts.day, parts.hour, parts.minute, parts.second);
  return { text, ms };
}

function msToTimestamp(ms) {
  const date = new Date(ms);
  const pad = (n) => String(n).padStart(2, '0');
  return `${date.getUTCFullYear()}-${pad(date.getUTCMonth() + 1)}-${pad(date.getUTCDate())}T${pad(date.getUTCHours())}:${pad(date.getUTCMinutes())}:${pad(date.getUTCSeconds())}Z`;
}

function isDecimalShape(value) {
  if (typeof value !== 'string') return false;
  if (WHOLE_DECIMAL.test(value)) return true;
  if (!FRAC_DECIMAL.test(value)) return false;
  return /[1-9]/.test(value);
}

function canonicalDecimal(value) {
  if (!isDecimalShape(value)) return null;
  const parts = value.split('.');
  let whole = parts[0];
  let frac = parts[1] || '';
  frac = frac.replace(/0+$/, '');
  if (whole === '0' && frac === '') return null;
  return frac ? `${whole}.${frac}` : whole;
}

function inspectPlain(value) {
  if (value === null || typeof value !== 'object' || Array.isArray(value)) return null;
  if (Object.getPrototypeOf(value) !== Object.prototype) return null;
  if (Object.getOwnPropertySymbols(value).length) return null;
  const keys = Object.keys(value);
  const descriptors = Object.getOwnPropertyDescriptors(value);
  for (const key of keys) {
    if (descriptors[key].get || descriptors[key].set) return null;
  }
  return keys;
}

function inspectArray(value) {
  if (!Array.isArray(value)) return false;
  if (Object.getPrototypeOf(value) !== Array.prototype) return false;
  if (Object.getOwnPropertySymbols(value).length) return false;
  const descriptors = Object.getOwnPropertyDescriptors(value);
  for (let i = 0; i < value.length; i += 1) {
    const desc = descriptors[i];
    if (!desc || desc.get || desc.set) return false;
  }
  return true;
}

function normalizeRateQuery(value) {
  const keys = inspectPlain(value);
  if (!keys) schema('query must be a closed plain object');
  if (keys.length !== QUERY_KEYS.length) schema('query field inventory differs');
  for (const key of keys) {
    if (!QUERY_KEYS.includes(key)) schema(`unknown query field ${key}`);
  }
  for (const key of QUERY_KEYS) {
    if (!Object.prototype.hasOwnProperty.call(value, key)) schema(`missing query field ${key}`);
  }
  if (value.v !== 1 || !Number.isInteger(value.v)) schema('v must be integer 1');
  if (!inspectArray(value.asset_ids) || value.asset_ids.length !== 1) schema('asset_ids');
  if (!inspectArray(value.quote_currencies) || value.quote_currencies.length !== 1) schema('quote_currencies');
  const assetId = value.asset_ids[0];
  const quote = value.quote_currencies[0];
  if (quote !== 'USD') schema('quote currency must be USD');
  if (assetId !== 'zec-zcash' && assetId !== 'xmr-monero') schema('asset id');
  return { v: 1, asset_ids: [assetId], quote_currencies: ['USD'] };
}

function copyQuote(quote) {
  const keys = inspectPlain(quote);
  if (!keys) return null;
  if (keys.length !== QUOTE_KEYS.length) return null;
  for (const key of keys) {
    if (!QUOTE_KEYS.includes(key)) return null;
  }
  for (const key of QUOTE_KEYS) {
    if (!Object.prototype.hasOwnProperty.call(quote, key)) return null;
  }
  if (quote.v !== 1 || !Number.isInteger(quote.v)) return null;
  if (quote.status !== 'fresh') return null;
  if (quote.quote_currency !== 'USD') return null;
  const price = canonicalDecimal(quote.price);
  if (price === null || price !== quote.price) return null;
  const observed = parseTimestamp(quote.provider_observed_at);
  const fetched = parseTimestamp(quote.fetched_at);
  const freshUntil = parseTimestamp(quote.fresh_until);
  const expires = parseTimestamp(quote.expires_at);
  if (!observed || !fetched || !freshUntil || !expires) return null;
  const boundMs = fetched.ms + 5 * 60 * 1000;
  if (freshUntil.ms !== boundMs || expires.ms !== boundMs) return null;
  if (quote.provider_id === 'kraken-spot-v1' && observed.text !== fetched.text) return null;
  if (quote.provider_id === 'coinbase-exchange-v1') {
    if (observed.ms < fetched.ms - 10 * 60 * 1000 || observed.ms > fetched.ms + 5 * 60 * 1000) return null;
  }
  const pinned = PINNED[quote.provider_id];
  if (!pinned) return null;
  if (quote.asset !== pinned.asset || quote.asset_id !== pinned.asset_id) return null;
  return {
    v: 1,
    asset: quote.asset,
    asset_id: quote.asset_id,
    quote_currency: 'USD',
    price,
    provider_id: quote.provider_id,
    provider_observed_at: observed.text,
    fetched_at: fetched.text,
    fresh_until: freshUntil.text,
    expires_at: expires.text,
    status: 'fresh',
    _freshUntilMs: freshUntil.ms,
    _expiresMs: expires.ms,
  };
}

function parseProviderBody(providerId, body, fetchedAt) {
  try {
    const provider = PINNED[providerId];
    if (!provider) return null;
    const fetched = parseTimestamp(fetchedAt);
    if (!fetched) return null;
    if (!Buffer.isBuffer(body) && !(body instanceof Uint8Array)) return null;
    const bytes = Buffer.from(body);
    if (bytes.length > MAX_BODY) return null;
    if (bytes.length >= 3 && bytes[0] === 0xef && bytes[1] === 0xbb && bytes[2] === 0xbf) return null;
    const text = bytes.toString('utf8');
    if (!Buffer.from(text, 'utf8').equals(bytes)) return null;
    const parsed = new BodyParser(text).parse();
    if (!parsed || typeof parsed !== 'object' || Array.isArray(parsed)) return null;
    let priceRaw;
    let observed;
    if (providerId === 'coinbase-exchange-v1') {
      if (typeof parsed.price !== 'string' || typeof parsed.time !== 'string') return null;
      priceRaw = parsed.price;
      observed = parseCoinbaseTime(parsed.time);
      if (!observed) return null;
      if (observed.ms < fetched.ms - 10 * 60 * 1000) return null;
      if (observed.ms > fetched.ms + 5 * 60 * 1000) return null;
    } else if (providerId === 'kraken-spot-v1') {
      if (!inspectArray(parsed.error) || parsed.error.length !== 0) return null;
      const result = parsed.result;
      const resultKeys = inspectPlain(result);
      if (!resultKeys || resultKeys.length !== 1 || resultKeys[0] !== provider.result_pair) return null;
      const ticker = result[provider.result_pair];
      if (!inspectPlain(ticker)) return null;
      if (!inspectArray(ticker.c) || typeof ticker.c[0] !== 'string') return null;
      priceRaw = ticker.c[0];
      observed = fetched;
    } else {
      return null;
    }
    const price = canonicalDecimal(priceRaw);
    if (price === null) return null;
    const freshUntilMs = fetched.ms + 5 * 60 * 1000;
    const freshUntil = msToTimestamp(freshUntilMs);
    return {
      v: 1,
      asset: provider.asset,
      asset_id: provider.asset_id,
      quote_currency: 'USD',
      price,
      provider_id: providerId,
      provider_observed_at: observed.text,
      fetched_at: fetched.text,
      fresh_until: freshUntil,
      expires_at: freshUntil,
      status: 'fresh',
    };
  } catch (error) {
    return null;
  }
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

function queryAsset(query) {
  return query.asset_ids[0] === 'zec-zcash' ? 'ZEC' : 'XMR';
}

function pinnedIdForAsset(asset) {
  return asset === 'ZEC' ? 'coinbase-exchange-v1' : 'kraken-spot-v1';
}

function buildRateSnapshot(query, quotes, queriedAt) {
  const normalized = normalizeRateQuery(query);
  const queried = parseTimestamp(queriedAt);
  if (!queried) schema('queried_at');
  const asset = queryAsset(normalized);
  const expectedProvider = pinnedIdForAsset(asset);
  const expectedAssetId = normalized.asset_ids[0];
  if (!inspectArray(quotes) || quotes.length !== 1) {
    return { v: 1, queried_at: queried.text, quotes: [], display: unavailableDisplay(asset) };
  }
  const accepted = [];
  for (const row of quotes) {
    const copied = copyQuote(row);
    if (!copied) continue;
    if (copied.asset !== asset || copied.asset_id !== expectedAssetId) continue;
    if (copied.provider_id !== expectedProvider) continue;
    if (copied.quote_currency !== 'USD') continue;
    if (queried.ms > copied._freshUntilMs || queried.ms > copied._expiresMs) continue;
    delete copied._freshUntilMs;
    delete copied._expiresMs;
    accepted.push(copied);
  }
  if (accepted.length !== 1) {
    return { v: 1, queried_at: queried.text, quotes: [], display: unavailableDisplay(asset) };
  }
  const quote = accepted[0];
  return {
    v: 1,
    queried_at: queried.text,
    quotes: [quote],
    display: {
      asset,
      quote_currency: 'USD',
      price: quote.price,
      method: 'single_labeled_source',
      source_ids: [quote.provider_id],
      label: 'approximate',
    },
  };
}

function formatFiatEstimate(amountAtomic, exponent, price, quoteCurrency) {
  if (typeof amountAtomic !== 'string' || !ATOMIC_RE.test(amountAtomic)) schema('amount_atomic');
  const amount = BigInt(amountAtomic);
  if (amount > U64_MAX) schema('amount_atomic overflow');
  if (exponent !== 8 && exponent !== 12) schema('exponent');
  if (quoteCurrency !== 'USD') schema('quote_currency');
  const canonical = canonicalDecimal(price);
  if (canonical === null) schema('price');
  const parts = canonical.split('.');
  const whole = parts[0];
  const frac = parts[1] || '';
  const unscaled = BigInt(whole + frac);
  const scale = frac.length;
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
  if (rounded < 0n) schema('negative display');
  const displayWhole = rounded / 100n;
  const displayFrac = rounded % 100n;
  const wholeText = displayWhole.toString();
  if (wholeText.length > 32) schema('display overflow');
  return `${wholeText}.${displayFrac.toString().padStart(2, '0')}`;
}

module.exports = {
  normalizeRateQuery,
  parseProviderBody,
  buildRateSnapshot,
  formatFiatEstimate,
};
