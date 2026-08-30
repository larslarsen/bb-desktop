'use strict';

const crypto = require('crypto');
const { Buffer } = require('buffer');

const SCHEMAS = Object.freeze({
  payment_request_v1: Object.freeze({
    domain: 'bitbook-payment-request-v1\n',
    fields: Object.freeze([
      'v', 'request_id', 'payer_peer_id', 'payee_peer_id', 'asset', 'network',
      'amount_atomic', 'receiver', 'receiver_kind', 'memo', 'nonce', 'created_at',
      'expires_at',
    ]),
  }),
  payment_status_event_v1: Object.freeze({
    domain: 'bitbook-payment-status-v1\n',
    fields: Object.freeze(['v', 'request_id', 'event_id', 'nonce', 'status', 'at', 'tx_ref']),
  }),
  review_image_v1: Object.freeze({
    domain: 'bitbook-intent-hash-v1\n',
    fields: Object.freeze([
      'v', 'intent_id', 'prepared_id', 'account_id', 'request_id',
      'payment_request_hash', 'payer_peer_id', 'payee_peer_id', 'asset', 'network',
      'amount_atomic', 'fee_atomic', 'fee_bound_atomic', 'receiver', 'receiver_kind',
      'change_policy', 'memo_hash', 'tx_version', 'zec_pools', 'expires_at',
      'prepared_at',
    ]),
  }),
});

const ZEC_NETWORKS = new Set(['zec-mainnet', 'zec-testnet', 'zec-regtest']);
const XMR_NETWORKS = new Set(['xmr-mainnet', 'xmr-stagenet', 'xmr-testnet']);
const HEX_32 = /^[0-9a-f]{32}$/;
const HEX_64 = /^[0-9a-f]{64}$/;
const POSITIVE_ATOMIC = /^[1-9][0-9]{0,19}$/;
const PRINTABLE_ASCII = /^[\x21-\x7e]+$/;
const TIMESTAMP = /^([0-9]{4})-([0-9]{2})-([0-9]{2})T([0-9]{2}):([0-9]{2}):([0-9]{2})Z$/;

class ContractError extends Error {
  constructor(code, message) {
    super(message);
    this.name = 'ContractError';
    this.code = code;
  }
}

function reject(code, message) {
  throw new ContractError(code, message);
}

class StrictJsonParser {
  constructor(text) {
    this.text = text;
    this.index = 0;
    this.numberTokens = [];
  }

  parse() {
    this.skipWhitespace();
    const value = this.parseValue();
    this.skipWhitespace();
    if (this.index !== this.text.length) reject('SCHEMA', 'trailing JSON bytes');
    return value;
  }

  skipWhitespace() {
    while (this.index < this.text.length && /[\x20\x09\x0a\x0d]/.test(this.text[this.index])) {
      this.index += 1;
    }
  }

  parseValue() {
    this.skipWhitespace();
    const char = this.text[this.index];
    if (char === '{') return this.parseObject();
    if (char === '[') return this.parseArray();
    if (char === '"') return this.parseString();
    if (char === '-' || (char >= '0' && char <= '9')) return this.parseNumber();
    for (const [token, value] of [['true', true], ['false', false], ['null', null]]) {
      if (this.text.startsWith(token, this.index)) {
        this.index += token.length;
        return value;
      }
    }
    reject('SCHEMA', 'invalid JSON value');
  }

  parseObject() {
    const value = {};
    const names = new Set();
    this.index += 1;
    this.skipWhitespace();
    if (this.text[this.index] === '}') {
      this.index += 1;
      return value;
    }
    while (this.index < this.text.length) {
      if (this.text[this.index] !== '"') reject('SCHEMA', 'object key must be a string');
      const key = this.parseString();
      if (names.has(key)) reject('SCHEMA', `duplicate object key ${key}`);
      names.add(key);
      this.skipWhitespace();
      if (this.text[this.index] !== ':') reject('SCHEMA', 'object key is missing a colon');
      this.index += 1;
      value[key] = this.parseValue();
      this.skipWhitespace();
      if (this.text[this.index] === '}') {
        this.index += 1;
        return value;
      }
      if (this.text[this.index] !== ',') reject('SCHEMA', 'object is missing a comma');
      this.index += 1;
      this.skipWhitespace();
    }
    reject('SCHEMA', 'unterminated object');
  }

  parseArray() {
    const value = [];
    this.index += 1;
    this.skipWhitespace();
    if (this.text[this.index] === ']') {
      this.index += 1;
      return value;
    }
    while (this.index < this.text.length) {
      value.push(this.parseValue());
      this.skipWhitespace();
      if (this.text[this.index] === ']') {
        this.index += 1;
        return value;
      }
      if (this.text[this.index] !== ',') reject('SCHEMA', 'array is missing a comma');
      this.index += 1;
      this.skipWhitespace();
    }
    reject('SCHEMA', 'unterminated array');
  }

  parseString() {
    const start = this.index;
    this.index += 1;
    let escaped = false;
    while (this.index < this.text.length) {
      const code = this.text.charCodeAt(this.index);
      const char = this.text[this.index];
      if (!escaped && char === '"') {
        this.index += 1;
        try {
          return JSON.parse(this.text.slice(start, this.index));
        } catch (_) {
          reject('SCHEMA', 'invalid JSON string');
        }
      }
      if (!escaped && code <= 0x1f) reject('SCHEMA', 'unescaped control in JSON string');
      if (!escaped && char === '\\') {
        escaped = true;
      } else {
        escaped = false;
      }
      this.index += 1;
    }
    reject('SCHEMA', 'unterminated JSON string');
  }

  parseNumber() {
    const match = /^-?(?:0|[1-9][0-9]*)(?:\.[0-9]+)?(?:[eE][+-]?[0-9]+)?/.exec(
      this.text.slice(this.index)
    );
    if (!match) reject('SCHEMA', 'invalid JSON number');
    const token = match[0];
    this.index += token.length;
    this.numberTokens.push(token);
    const value = Number(token);
    if (!Number.isFinite(value)) reject('SCHEMA', 'non-finite JSON number');
    return value;
  }
}

function strictUtf8(raw) {
  if (!Buffer.isBuffer(raw) && !(raw instanceof Uint8Array)) {
    reject('SCHEMA', 'signed object must be bytes');
  }
  const bytes = Buffer.from(raw);
  if (bytes.length >= 3 && bytes[0] === 0xef && bytes[1] === 0xbb && bytes[2] === 0xbf) {
    reject('SCHEMA', 'BOM is forbidden');
  }
  const text = bytes.toString('utf8');
  if (!Buffer.from(text, 'utf8').equals(bytes)) reject('SCHEMA', 'malformed UTF-8');
  return text;
}

function parseSignedJson(raw) {
  const parser = new StrictJsonParser(strictUtf8(raw));
  const value = parser.parse();
  if (!value || typeof value !== 'object' || Array.isArray(value)) {
    reject('SCHEMA', 'signed object root must be an object');
  }
  if (parser.numberTokens.length !== 1 || parser.numberTokens[0] !== '1') {
    reject('SCHEMA', 'v must be the sole JSON number and exact integer 1');
  }
  return value;
}

function assertClosed(value, fields) {
  const keys = Object.keys(value);
  if (keys.length !== fields.length) reject('SCHEMA', 'signed object field inventory differs');
  const allowed = new Set(fields);
  for (const key of keys) {
    if (!allowed.has(key)) reject('SCHEMA', `unknown field ${key}`);
  }
  for (const field of fields) {
    if (!Object.prototype.hasOwnProperty.call(value, field)) reject('SCHEMA', `missing field ${field}`);
  }
  if (value.v !== 1 || !Number.isInteger(value.v)) reject('SCHEMA', 'v must be integer 1');
}

function forbiddenCodepoint(codepoint) {
  if (codepoint <= 0x1f || (codepoint >= 0x7f && codepoint <= 0x9f)) return true;
  if (codepoint >= 0xd800 && codepoint <= 0xdfff) return true;
  if (codepoint >= 0xfdd0 && codepoint <= 0xfdef) return true;
  if ((codepoint & 0xffff) === 0xfffe || (codepoint & 0xffff) === 0xffff) return true;
  if (codepoint >= 0x202a && codepoint <= 0x202e) return true;
  if (codepoint >= 0x2066 && codepoint <= 0x206f) return true;
  if (codepoint >= 0x200b && codepoint <= 0x200f) return true;
  if (codepoint === 0x061c || codepoint === 0x2060 || codepoint === 0xfeff) return true;
  if (codepoint >= 0xfff9 && codepoint <= 0xfffb) return true;
  return codepoint >= 0xe0001 && codepoint <= 0xe007f;
}

function assertUnicode(value) {
  if (typeof value !== 'string') reject('SCHEMA', 'signed field must be a string');
  for (const char of value) {
    if (forbiddenCodepoint(char.codePointAt(0))) reject('SCHEMA', 'forbidden signed-string codepoint');
  }
}

function assertAllStrings(value) {
  for (const member of Object.values(value)) {
    if (typeof member === 'string') {
      assertUnicode(member);
    } else if (Array.isArray(member)) {
      for (const item of member) assertUnicode(item);
    }
  }
}

function assertAscii(value, allowEmpty = false) {
  if (typeof value !== 'string' || (!allowEmpty && value.length === 0)) reject('SCHEMA', 'invalid ASCII field');
  if (value !== '' && !PRINTABLE_ASCII.test(value)) reject('SCHEMA', 'invalid ASCII field');
}

function timestampEpoch(value) {
  if (typeof value !== 'string') reject('SCHEMA', 'timestamp must be a string');
  const match = TIMESTAMP.exec(value);
  if (!match) reject('SCHEMA', 'timestamp has a non-canonical shape');
  const [year, month, day, hour, minute, second] = match.slice(1).map(Number);
  if (year < 2020 || year > 2100 || month < 1 || month > 12 || hour > 23 || minute > 59 || second > 59) {
    reject('SCHEMA', 'timestamp is outside its calendar bounds');
  }
  const leap = year % 4 === 0 && (year % 100 !== 0 || year % 400 === 0);
  const days = [31, leap ? 29 : 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
  if (day < 1 || day > days[month - 1]) reject('SCHEMA', 'timestamp is not a Gregorian date');
  const epoch = Date.UTC(year, month - 1, day, hour, minute, second);
  if (new Date(epoch).toISOString() !== value.replace('Z', '.000Z')) {
    reject('SCHEMA', 'timestamp does not round-trip');
  }
  return epoch;
}

function assertAssetRelation(value) {
  if (value.asset === 'ZEC') {
    if (!ZEC_NETWORKS.has(value.network) || value.receiver_kind !== 'zec-ua-orchard-protocol') {
      reject('SCHEMA', 'ZEC network or receiver relation is invalid');
    }
  } else if (value.asset === 'XMR') {
    if (!XMR_NETWORKS.has(value.network) || value.receiver_kind !== 'xmr-subaddress') {
      reject('SCHEMA', 'XMR network or receiver relation is invalid');
    }
  } else {
    reject('SCHEMA', 'unknown asset');
  }
  assertAscii(value.receiver);
}

function validateRequest(value) {
  if (!HEX_32.test(value.request_id) || !HEX_32.test(value.nonce)) reject('SCHEMA', 'invalid request identifier');
  assertAscii(value.payer_peer_id);
  assertAscii(value.payee_peer_id);
  assertAssetRelation(value);
  if (typeof value.amount_atomic !== 'string' || !POSITIVE_ATOMIC.test(value.amount_atomic)) {
    reject('SCHEMA', 'invalid atomic amount');
  }
  if (typeof value.memo !== 'string' || value.memo !== value.memo.normalize('NFC')) reject('SCHEMA', 'memo is not NFC');
  if (Buffer.byteLength(value.memo, 'utf8') > 512) reject('SCHEMA', 'memo exceeds 512 UTF-8 bytes');
  const created = timestampEpoch(value.created_at);
  const expires = timestampEpoch(value.expires_at);
  if (expires <= created) reject('SCHEMA', 'request expiry must follow creation');
}

function validateStatus(value) {
  if (!HEX_32.test(value.request_id) || !HEX_32.test(value.event_id) || !HEX_32.test(value.nonce)) {
    reject('SCHEMA', 'invalid status identifier');
  }
  if (!['cancelled', 'paid', 'expired'].includes(value.status)) reject('SCHEMA', 'invalid status');
  timestampEpoch(value.at);
  assertAscii(value.tx_ref, true);
  if ((value.status === 'paid') !== (value.tx_ref.length > 0)) reject('SCHEMA', 'status tx-ref relation is invalid');
}

function validateReview(value) {
  for (const field of ['intent_id', 'prepared_id', 'account_id', 'payer_peer_id', 'payee_peer_id']) {
    assertAscii(value[field]);
  }
  if (!HEX_32.test(value.request_id) || !HEX_64.test(value.payment_request_hash) || !HEX_64.test(value.memo_hash)) {
    reject('SCHEMA', 'invalid review identifier or hash');
  }
  assertAssetRelation(value);
  for (const field of ['amount_atomic', 'fee_atomic', 'fee_bound_atomic']) {
    if (typeof value[field] !== 'string' || !POSITIVE_ATOMIC.test(value[field])) reject('SCHEMA', 'invalid review amount');
  }
  if (BigInt(value.fee_atomic) > BigInt(value.fee_bound_atomic)) reject('SCHEMA', 'fee exceeds bound');
  if (!Array.isArray(value.zec_pools) || value.zec_pools.some((pool) => typeof pool !== 'string')) {
    reject('SCHEMA', 'invalid ZEC pool list');
  }
  if (value.asset === 'ZEC') {
    if (value.change_policy !== 'shielded_internal' || value.tx_version !== '6') {
      reject('SCHEMA', 'invalid ZEC review relation');
    }
    if (value.zec_pools.length === 1 && value.zec_pools[0] === 'orchard') {
      reject('MIGRATION_REQUIRED', 'Orchard funds require migration');
    }
    if (value.zec_pools.length !== 1 || value.zec_pools[0] !== 'ironwood') {
      reject('SCHEMA', 'invalid ZEC pool list');
    }
  } else if (value.change_policy !== 'xmr_change' || value.tx_version !== '0' || value.zec_pools.length !== 0) {
    reject('SCHEMA', 'invalid XMR review relation');
  }
  const prepared = timestampEpoch(value.prepared_at);
  const expires = timestampEpoch(value.expires_at);
  if (expires <= prepared) reject('SCHEMA', 'review expiry must follow preparation');
}

function canonicalize(value) {
  if (typeof value === 'string' || typeof value === 'number') return JSON.stringify(value);
  if (Array.isArray(value)) return `[${value.map(canonicalize).join(',')}]`;
  const entries = Object.keys(value).sort().map((key) => `${JSON.stringify(key)}:${canonicalize(value[key])}`);
  return `{${entries.join(',')}}`;
}

function decodeSignedObject(kind, raw) {
  const schema = SCHEMAS[kind];
  if (!schema) reject('SCHEMA', 'unknown signed object kind');
  const value = parseSignedJson(raw);
  assertClosed(value, schema.fields);
  assertAllStrings(value);
  if (kind === 'payment_request_v1') validateRequest(value);
  if (kind === 'payment_status_event_v1') validateStatus(value);
  if (kind === 'review_image_v1') validateReview(value);
  const canonical = canonicalize(value);
  const canonicalBytes = Buffer.from(canonical, 'utf8');
  const digest = crypto
    .createHash('sha256')
    .update(Buffer.from(schema.domain, 'utf8'))
    .update(canonicalBytes)
    .digest('hex');
  return { value, canonical, canonical_bytes: canonicalBytes, digest };
}

module.exports = {
  ContractError,
  decodeSignedObject,
};
