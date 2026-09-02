'use strict';

const { Buffer } = require('buffer');

const RATE_FRAME_LIMIT = 65536;
const ID_RE = /^[0-9a-f]{32}$/;
const REQUEST_KEYS = ['v', 'id', 'kind', 'method', 'params'];
const RESPONSE_KEYS = ['v', 'id', 'kind', 'method', 'result'];
const QUERY_KEYS = ['v', 'asset_ids', 'quote_currencies'];
const QUOTE_KEYS = [
  'v', 'asset', 'asset_id', 'quote_currency', 'price', 'provider_id',
  'provider_observed_at', 'fetched_at', 'fresh_until', 'expires_at', 'status',
];
const SNAPSHOT_KEYS = ['v', 'queried_at', 'quotes', 'display'];
const DISPLAY_FRESH_KEYS = ['asset', 'quote_currency', 'price', 'method', 'source_ids', 'label'];
const DISPLAY_UNAVAILABLE_KEYS = ['asset', 'quote_currency', 'method', 'source_ids', 'label'];
const TIMESTAMP_RE = /^([0-9]{4})-([0-9]{2})-([0-9]{2})T([0-9]{2}):([0-9]{2}):([0-9]{2})Z$/;
const DECIMAL_RE = /^(?:[1-9][0-9]{0,11}(?:\.[0-9]{1,18})?|0\.[0-9]{1,18})$/;

function frameError(code, message) {
  const error = new Error(message);
  error.code = code;
  return error;
}

function fail(code, message) {
  throw frameError(code, message);
}

function defineData(object, key, value) {
  if (key === '__proto__') fail('SCHEMA', 'forbidden key');
  Object.defineProperty(object, key, {
    value,
    enumerable: true,
    writable: true,
    configurable: true,
  });
}

class StrictJsonParser {
  constructor(text) {
    this.text = text;
    this.index = 0;
  }

  parse() {
    this.skipWhitespace();
    const value = this.parseValue(1);
    this.skipWhitespace();
    if (this.index !== this.text.length) fail('SCHEMA', 'trailing JSON bytes');
    return value;
  }

  skipWhitespace() {
    while (this.index < this.text.length && /[\x20\x09\x0a\x0d]/.test(this.text[this.index])) {
      this.index += 1;
    }
  }

  parseValue(depth) {
    if (depth > 8) fail('SCHEMA', 'JSON nesting exceeds depth 8');
    this.skipWhitespace();
    const char = this.text[this.index];
    if (char === '{') return this.parseObject(depth);
    if (char === '[') return this.parseArray(depth);
    if (char === '"') return this.parseString();
    if (char === '-' || (char >= '0' && char <= '9')) return this.parseNumber();
    for (const [token, value] of [['true', true], ['false', false], ['null', null]]) {
      if (this.text.startsWith(token, this.index)) {
        this.index += token.length;
        return value;
      }
    }
    fail('SCHEMA', 'invalid JSON value');
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
      if (this.text[this.index] !== '"') fail('SCHEMA', 'object key must be a string');
      const key = this.parseString();
      if (names.has(key)) fail('SCHEMA', `duplicate object key ${key}`);
      names.add(key);
      this.skipWhitespace();
      if (this.text[this.index] !== ':') fail('SCHEMA', 'object key is missing a colon');
      this.index += 1;
      defineData(value, key, this.parseValue(depth + 1));
      this.skipWhitespace();
      if (this.text[this.index] === '}') {
        this.index += 1;
        return value;
      }
      if (this.text[this.index] !== ',') fail('SCHEMA', 'object is missing a comma');
      this.index += 1;
      this.skipWhitespace();
    }
    fail('SCHEMA', 'unterminated object');
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
      if (this.text[this.index] !== ',') fail('SCHEMA', 'array is missing a comma');
      this.index += 1;
      this.skipWhitespace();
    }
    fail('SCHEMA', 'unterminated array');
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
      if (code <= 0x1f) fail('SCHEMA', 'unescaped control in JSON string');
      if (char === '\\') {
        this.index += 1;
        const esc = this.text[this.index];
        const map = { '"': '"', '\\': '\\', '/': '/', b: '\b', f: '\f', n: '\n', r: '\r', t: '\t' };
        if (esc === 'u') {
          const hex = this.text.slice(this.index + 1, this.index + 5);
          if (!/^[0-9a-fA-F]{4}$/.test(hex)) fail('SCHEMA', 'invalid unicode escape');
          out += String.fromCharCode(parseInt(hex, 16));
          this.index += 5;
        } else if (Object.prototype.hasOwnProperty.call(map, esc)) {
          out += map[esc];
          this.index += 1;
        } else {
          fail('SCHEMA', 'invalid escape');
        }
      } else {
        out += char;
        this.index += 1;
      }
    }
    fail('SCHEMA', 'unterminated JSON string');
    return out;
  }

  parseNumber() {
    const match = /^-?(?:0|[1-9][0-9]*)(?:\.[0-9]+)?(?:[eE][+-]?[0-9]+)?/.exec(this.text.slice(this.index));
    if (!match) fail('SCHEMA', 'invalid JSON number');
    this.index += match[0].length;
    const value = Number(match[0]);
    if (!Number.isFinite(value)) fail('SCHEMA', 'non-finite JSON number');
    return value;
  }
}

function parseFrameJson(body) {
  if (!Buffer.isBuffer(body) && !(body instanceof Uint8Array)) fail('SCHEMA', 'frame must be bytes');
  const bytes = Buffer.from(body);
  if (bytes.length >= 3 && bytes[0] === 0xef && bytes[1] === 0xbb && bytes[2] === 0xbf) {
    fail('SCHEMA', 'BOM is forbidden');
  }
  const text = bytes.toString('utf8');
  if (!Buffer.from(text, 'utf8').equals(bytes)) fail('SCHEMA', 'malformed frame UTF-8');
  const value = new StrictJsonParser(text).parse();
  if (!value || typeof value !== 'object' || Array.isArray(value)) {
    fail('SCHEMA', 'frame must contain one JSON object');
  }
  return value;
}

function encodeRateFrame(value) {
  if (!value || typeof value !== 'object' || Array.isArray(value)) {
    fail('SCHEMA', 'frame value must be an object');
  }
  const body = Buffer.from(JSON.stringify(value), 'utf8');
  if (body.length === 0) fail('SCHEMA', 'empty frame body');
  if (body.length > RATE_FRAME_LIMIT) fail('LIMIT', 'frame exceeds limit');
  const prefix = Buffer.alloc(4);
  prefix.writeUInt32BE(body.length, 0);
  return Buffer.concat([prefix, body]);
}

function createRateFrameDecoder() {
  let unread = Buffer.alloc(0);
  let closed = false;

  function closeAndThrow(code, message) {
    closed = true;
    unread = Buffer.alloc(0);
    throw frameError(code, message);
  }

  return {
    get closed() {
      return closed;
    },
    get incomplete() {
      return unread.length > 0;
    },
    push(chunk) {
      if (closed) throw frameError('SCHEMA', 'decoder is permanently closed');
      if (!Buffer.isBuffer(chunk) && !(chunk instanceof Uint8Array)) {
        return closeAndThrow('SCHEMA', 'decoder input must be bytes');
      }
      unread = Buffer.concat([unread, Buffer.from(chunk)]);
      const values = [];
      while (unread.length >= 4) {
        const length = unread.readUInt32BE(0);
        if (length === 0) return closeAndThrow('SCHEMA', 'zero-length frame is invalid');
        if (length > RATE_FRAME_LIMIT) return closeAndThrow('LIMIT', 'frame length exceeds configured limit');
        if (unread.length < length + 4) break;
        const body = unread.subarray(4, length + 4);
        unread = unread.subarray(length + 4);
        try {
          values.push(parseFrameJson(body));
        } catch (error) {
          return closeAndThrow(error.code || 'SCHEMA', error.message);
        }
      }
      return values;
    },
  };
}

function ownKeys(value) {
  if (value === null || typeof value !== 'object' || Array.isArray(value)) fail('SCHEMA', 'expected object');
  if (Object.getPrototypeOf(value) !== Object.prototype) fail('SCHEMA', 'exotic prototype');
  if (Object.getOwnPropertySymbols(value).length) fail('SCHEMA', 'symbol keys');
  const keys = Object.keys(value);
  const descriptors = Object.getOwnPropertyDescriptors(value);
  for (const key of keys) {
    if (descriptors[key].get || descriptors[key].set) fail('SCHEMA', 'accessor');
  }
  return keys;
}

function assertExactKeys(value, expected) {
  const keys = ownKeys(value);
  if (keys.length !== expected.length) fail('SCHEMA', 'field inventory differs');
  const allowed = new Set(expected);
  for (const key of keys) {
    if (!allowed.has(key)) fail('SCHEMA', `unknown field ${key}`);
  }
  for (const field of expected) {
    if (!Object.prototype.hasOwnProperty.call(value, field)) fail('SCHEMA', `missing field ${field}`);
  }
}

function assertTimestamp(value) {
  if (typeof value !== 'string') fail('SCHEMA', 'timestamp must be a string');
  const match = TIMESTAMP_RE.exec(value);
  if (!match) fail('SCHEMA', 'timestamp has a non-canonical shape');
  const year = Number(match[1]);
  const month = Number(match[2]);
  const day = Number(match[3]);
  const hour = Number(match[4]);
  const minute = Number(match[5]);
  const second = Number(match[6]);
  if (year < 2020 || year > 2100 || month < 1 || month > 12 || hour > 23 || minute > 59 || second > 59) {
    fail('SCHEMA', 'timestamp is outside its calendar bounds');
  }
  const days = [0, 31, year % 4 === 0 && (year % 100 !== 0 || year % 400 === 0) ? 29 : 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
  if (day < 1 || day > days[month]) fail('SCHEMA', 'timestamp is not a real calendar date');
}

function assertDecimal(value) {
  if (typeof value !== 'string' || !DECIMAL_RE.test(value)) fail('SCHEMA', 'invalid decimal');
  if (/^0\.0+$/.test(value)) fail('SCHEMA', 'zero decimal');
  if (value.startsWith('0.') && !/[1-9]/.test(value)) fail('SCHEMA', 'zero decimal');
}

function canonicalDecimal(value) {
  assertDecimal(value);
  const parts = value.split('.');
  const whole = parts[0];
  let frac = parts[1] || '';
  frac = frac.replace(/0+$/, '');
  if (whole === '0' && frac === '') fail('SCHEMA', 'zero decimal');
  return frac ? `${whole}.${frac}` : whole;
}

function timestampMs(value) {
  assertTimestamp(value);
  const match = TIMESTAMP_RE.exec(value);
  return Date.UTC(
    Number(match[1]), Number(match[2]) - 1, Number(match[3]),
    Number(match[4]), Number(match[5]), Number(match[6])
  );
}

function copyQuery(params) {
  assertExactKeys(params, QUERY_KEYS);
  if (params.v !== 1 || !Number.isInteger(params.v)) fail('SCHEMA', 'v must be integer 1');
  if (!Array.isArray(params.asset_ids) || params.asset_ids.length !== 1) fail('SCHEMA', 'asset_ids');
  if (!Array.isArray(params.quote_currencies) || params.quote_currencies.length !== 1) fail('SCHEMA', 'quote_currencies');
  const assetId = params.asset_ids[0];
  const quote = params.quote_currencies[0];
  if (quote !== 'USD') fail('SCHEMA', 'quote currency');
  if (assetId !== 'zec-zcash' && assetId !== 'xmr-monero') fail('SCHEMA', 'asset id');
  return { v: 1, asset_ids: [assetId], quote_currencies: ['USD'] };
}

function copyQuote(quote) {
  assertExactKeys(quote, QUOTE_KEYS);
  if (quote.v !== 1 || !Number.isInteger(quote.v)) fail('SCHEMA', 'v');
  if (quote.status !== 'fresh') fail('SCHEMA', 'status');
  if ((quote.asset === 'ZEC' && quote.asset_id === 'zec-zcash' && quote.provider_id === 'coinbase-exchange-v1') ||
      (quote.asset === 'XMR' && quote.asset_id === 'xmr-monero' && quote.provider_id === 'kraken-spot-v1')) {
    /* pinned pair */
  } else {
    fail('SCHEMA', 'quote mapping');
  }
  if (quote.quote_currency !== 'USD') fail('SCHEMA', 'quote currency');
  if (canonicalDecimal(quote.price) !== quote.price) fail('SCHEMA', 'noncanonical price');
  assertTimestamp(quote.provider_observed_at);
  assertTimestamp(quote.fetched_at);
  assertTimestamp(quote.fresh_until);
  assertTimestamp(quote.expires_at);
  const fetchedMs = timestampMs(quote.fetched_at);
  const boundMs = fetchedMs + 5 * 60 * 1000;
  if (timestampMs(quote.fresh_until) !== boundMs || timestampMs(quote.expires_at) !== boundMs) {
    fail('SCHEMA', 'freshness bounds');
  }
  if (quote.provider_id === 'kraken-spot-v1' && quote.provider_observed_at !== quote.fetched_at) {
    fail('SCHEMA', 'kraken observation');
  }
  if (quote.provider_id === 'coinbase-exchange-v1') {
    const observedMs = timestampMs(quote.provider_observed_at);
    if (observedMs < fetchedMs - 10 * 60 * 1000 || observedMs > fetchedMs + 5 * 60 * 1000) {
      fail('SCHEMA', 'coinbase observation window');
    }
  }
  return {
    v: 1,
    asset: quote.asset,
    asset_id: quote.asset_id,
    quote_currency: 'USD',
    price: quote.price,
    provider_id: quote.provider_id,
    provider_observed_at: quote.provider_observed_at,
    fetched_at: quote.fetched_at,
    fresh_until: quote.fresh_until,
    expires_at: quote.expires_at,
    status: 'fresh',
  };
}

function copyDisplay(display) {
  const keys = ownKeys(display);
  if (display.method === 'single_labeled_source') {
    assertExactKeys(display, DISPLAY_FRESH_KEYS);
    assertDecimal(display.price);
    if (display.label !== 'approximate') fail('SCHEMA', 'label');
    if (!Array.isArray(display.source_ids) || display.source_ids.length !== 1) fail('SCHEMA', 'source_ids');
  } else if (display.method === 'unavailable') {
    assertExactKeys(display, DISPLAY_UNAVAILABLE_KEYS);
    if (display.label !== 'fiat estimate unavailable') fail('SCHEMA', 'label');
    if (!Array.isArray(display.source_ids) || display.source_ids.length !== 0) fail('SCHEMA', 'source_ids');
  } else {
    fail('SCHEMA', 'display method');
  }
  if (display.asset !== 'ZEC' && display.asset !== 'XMR') fail('SCHEMA', 'display asset');
  if (display.quote_currency !== 'USD') fail('SCHEMA', 'display currency');
  const copied = {
    asset: display.asset,
    quote_currency: 'USD',
    method: display.method,
    source_ids: display.source_ids.slice(),
    label: display.label,
  };
  if (display.method === 'single_labeled_source') copied.price = display.price;
  return copied;
}

function copySnapshot(result) {
  assertExactKeys(result, SNAPSHOT_KEYS);
  if (result.v !== 1 || !Number.isInteger(result.v)) fail('SCHEMA', 'v');
  assertTimestamp(result.queried_at);
  if (!Array.isArray(result.quotes)) fail('SCHEMA', 'quotes');
  const quotes = result.quotes.map(copyQuote);
  const display = copyDisplay(result.display);
  if (display.method === 'unavailable') {
    if (quotes.length !== 0 || display.source_ids.length !== 0) fail('SCHEMA', 'unavailable display requires zero quotes');
  } else {
    if (quotes.length !== 1) fail('SCHEMA', 'fresh display requires one quote');
    const quote = quotes[0];
    if (quote.asset !== display.asset || quote.quote_currency !== display.quote_currency) {
      fail('SCHEMA', 'display asset mismatch');
    }
    if (quote.price !== display.price) fail('SCHEMA', 'display price mismatch');
    if (display.source_ids.length !== 1 || display.source_ids[0] !== quote.provider_id) {
      fail('SCHEMA', 'display source mismatch');
    }
  }
  return {
    v: 1,
    queried_at: result.queried_at,
    quotes,
    display,
  };
}

function validateRateRequest(value) {
  assertExactKeys(value, REQUEST_KEYS);
  if (value.v !== 1 || !Number.isInteger(value.v)) fail('SCHEMA', 'v');
  if (typeof value.id !== 'string' || !ID_RE.test(value.id)) fail('SCHEMA', 'id');
  if (value.kind !== 'req') fail('SCHEMA', 'kind');
  if (value.method !== 'rate.query') fail('SCHEMA', 'method');
  return {
    v: 1,
    id: value.id,
    kind: 'req',
    method: 'rate.query',
    params: copyQuery(value.params),
  };
}

function validateRateResponse(value) {
  assertExactKeys(value, RESPONSE_KEYS);
  if (value.v !== 1 || !Number.isInteger(value.v)) fail('SCHEMA', 'v');
  if (typeof value.id !== 'string' || !ID_RE.test(value.id)) fail('SCHEMA', 'id');
  if (value.kind !== 'res') fail('SCHEMA', 'kind');
  if (value.method !== 'rate.snapshot') fail('SCHEMA', 'method');
  return {
    v: 1,
    id: value.id,
    kind: 'res',
    method: 'rate.snapshot',
    result: copySnapshot(value.result),
  };
}

module.exports = {
  RATE_FRAME_LIMIT,
  encodeRateFrame,
  createRateFrameDecoder,
  validateRateRequest,
  validateRateResponse,
};
