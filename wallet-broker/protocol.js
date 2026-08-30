'use strict';

const crypto = require('crypto');

const CONTROL_FRAME_LIMIT = 64 * 1024;
const ABSOLUTE_FRAME_LIMIT = 1024 * 1024;
const HEX32 = /^[0-9a-f]{32}$/;
const HEX64 = /^[0-9a-f]{64}$/;
const PID = /^[1-9][0-9]*$/;
const METHOD = /^[a-z][a-z0-9]*(?:\.[a-z][a-z0-9]*)+$/;

class BrokerProtocolError extends Error {
  constructor(code, message) {
    super(message || code);
    this.name = 'BrokerProtocolError';
    this.code = code;
  }
}

function fail(code, message) {
  throw new BrokerProtocolError(code, message);
}

function isDataObject(value) {
  if (value === null || typeof value !== 'object' || Array.isArray(value)) return false;
  const prototype = Object.getPrototypeOf(value);
  if (prototype !== Object.prototype && prototype !== null) return false;
  return Object.values(Object.getOwnPropertyDescriptors(value)).every(
    (descriptor) => Object.prototype.hasOwnProperty.call(descriptor, 'value')
  );
}

function hasExactKeys(value, keys) {
  return isDataObject(value) &&
    Object.keys(value).length === keys.length &&
    keys.every((key) => Object.prototype.hasOwnProperty.call(value, key));
}

class StrictJsonParser {
  constructor(text) {
    this.text = text;
    this.index = 0;
  }

  parse() {
    this.space();
    const value = this.value();
    this.space();
    if (this.index !== this.text.length) fail('SCHEMA', 'trailing JSON');
    return value;
  }

  space() {
    while (/[\u0009\u000a\u000d\u0020]/.test(this.text[this.index] || '')) this.index += 1;
  }

  value() {
    const token = this.text[this.index];
    if (token === '{') return this.object();
    if (token === '[') return this.array();
    if (token === '"') return this.string();
    if (token === 't') return this.literal('true', true);
    if (token === 'f') return this.literal('false', false);
    if (token === 'n') return this.literal('null', null);
    return this.number();
  }

  literal(source, value) {
    if (this.text.slice(this.index, this.index + source.length) !== source) fail('SCHEMA', 'invalid JSON');
    this.index += source.length;
    return value;
  }

  string() {
    const start = this.index;
    this.index += 1;
    while (this.index < this.text.length) {
      const code = this.text.charCodeAt(this.index);
      if (code === 0x22) {
        this.index += 1;
        try { return JSON.parse(this.text.slice(start, this.index)); } catch (_) { fail('SCHEMA', 'invalid string'); }
      }
      if (code < 0x20) fail('SCHEMA', 'invalid string');
      if (code === 0x5c) {
        this.index += 1;
        if (this.index >= this.text.length) fail('SCHEMA', 'invalid escape');
        if (this.text[this.index] === 'u') {
          if (!/^[0-9a-fA-F]{4}$/.test(this.text.slice(this.index + 1, this.index + 5))) fail('SCHEMA', 'invalid escape');
          this.index += 5;
          continue;
        }
        if (!/["\\/bfnrt]/.test(this.text[this.index])) fail('SCHEMA', 'invalid escape');
      }
      this.index += 1;
    }
    fail('SCHEMA', 'unterminated string');
  }

  number() {
    const rest = this.text.slice(this.index);
    const match = rest.match(/^-?(?:0|[1-9][0-9]*)(?:\.[0-9]+)?(?:[eE][+-]?[0-9]+)?/);
    if (!match) fail('SCHEMA', 'invalid JSON value');
    this.index += match[0].length;
    const value = Number(match[0]);
    if (!Number.isFinite(value)) fail('SCHEMA', 'non-finite number');
    return value;
  }

  array() {
    const result = [];
    this.index += 1;
    this.space();
    if (this.text[this.index] === ']') { this.index += 1; return result; }
    while (true) {
      result.push(this.value());
      this.space();
      if (this.text[this.index] === ']') { this.index += 1; return result; }
      if (this.text[this.index] !== ',') fail('SCHEMA', 'invalid array');
      this.index += 1;
      this.space();
    }
  }

  object() {
    const result = {};
    const names = new Set();
    this.index += 1;
    this.space();
    if (this.text[this.index] === '}') { this.index += 1; return result; }
    while (true) {
      if (this.text[this.index] !== '"') fail('SCHEMA', 'invalid object name');
      const name = this.string();
      if (names.has(name)) fail('SCHEMA', 'duplicate object name');
      names.add(name);
      this.space();
      if (this.text[this.index] !== ':') fail('SCHEMA', 'invalid object');
      this.index += 1;
      this.space();
      Object.defineProperty(result, name, {
        value: this.value(), enumerable: true, configurable: true, writable: true,
      });
      this.space();
      if (this.text[this.index] === '}') { this.index += 1; return result; }
      if (this.text[this.index] !== ',') fail('SCHEMA', 'invalid object');
      this.index += 1;
      this.space();
    }
  }
}

function decodeJson(body) {
  const text = body.toString('utf8');
  if (!Buffer.from(text, 'utf8').equals(body)) fail('SCHEMA', 'invalid UTF-8');
  const value = new StrictJsonParser(text).parse();
  if (!isDataObject(value)) fail('SCHEMA', 'frame must contain an object');
  return value;
}

function encodeBrokerFrame(value) {
  const body = Buffer.from(JSON.stringify(value), 'utf8');
  const prefix = Buffer.alloc(4);
  prefix.writeUInt32BE(body.length, 0);
  return Buffer.concat([prefix, body]);
}

function createBrokerFrameDecoder(options = {}) {
  const limitBytes = options.limitBytes;
  const stream = options.stream;
  if (!Number.isInteger(limitBytes) || limitBytes <= 0 || limitBytes > ABSOLUTE_FRAME_LIMIT || stream !== 'protocol') {
    fail('SCHEMA', 'invalid decoder options');
  }
  let bytes = Buffer.alloc(0);
  const decoder = {
    closed: false,
    push(chunk) {
      if (decoder.closed) fail('SCHEMA', 'decoder closed');
      if (!Buffer.isBuffer(chunk)) {
        decoder.closed = true;
        fail('SCHEMA', 'protocol bytes required');
      }
      bytes = Buffer.concat([bytes, chunk]);
      const values = [];
      try {
        while (bytes.length >= 4) {
          const length = bytes.readUInt32BE(0);
          if (length === 0) fail('SCHEMA', 'empty frame');
          if (length > limitBytes || length > ABSOLUTE_FRAME_LIMIT) fail('LIMIT', 'frame too large');
          if (bytes.length < length + 4) break;
          const body = bytes.subarray(4, length + 4);
          bytes = bytes.subarray(length + 4);
          values.push(decodeJson(body));
        }
        return values;
      } catch (error) {
        decoder.closed = true;
        bytes = Buffer.alloc(0);
        throw error;
      }
    },
  };
  return decoder;
}

function validateHello(value) {
  const keys = ['protocol', 'min', 'max', 'child_nonce', 'child_pid'];
  if (!hasExactKeys(value, keys) || value.protocol !== 'bitbook-wallet-broker' ||
      !Number.isInteger(value.min) || !Number.isInteger(value.max) || value.min < 1 ||
      value.max < value.min || value.min > 1 || value.max < 1 ||
      typeof value.child_nonce !== 'string' || !HEX32.test(value.child_nonce) ||
      typeof value.child_pid !== 'string' || !PID.test(value.child_pid)) fail('SCHEMA', 'invalid hello');
  return { value, negotiated_version: 1 };
}

function validateHelloAck(value) {
  const keys = ['protocol', 'version', 'parent_nonce', 'parent_pid'];
  if (!hasExactKeys(value, keys) || value.protocol !== 'bitbook-wallet-broker' ||
      value.version !== 1 || typeof value.parent_nonce !== 'string' ||
      !HEX32.test(value.parent_nonce) || typeof value.parent_pid !== 'string' ||
      !PID.test(value.parent_pid)) {
    fail('SCHEMA', 'invalid hello ack');
  }
  return value;
}

function computeSessionId(transcript) {
  if (!isDataObject(transcript)) fail('SCHEMA', 'invalid session transcript');
  const descriptors = Object.getOwnPropertyDescriptors(transcript);
  const parentPid = descriptors.parent_pid && descriptors.parent_pid.value;
  const childPid = descriptors.child_pid && descriptors.child_pid.value;
  const parentNonce = descriptors.parent_nonce && descriptors.parent_nonce.value;
  const childNonce = descriptors.child_nonce && descriptors.child_nonce.value;
  if (typeof parentPid !== 'string' || !PID.test(parentPid) ||
      typeof childPid !== 'string' || !PID.test(childPid) ||
      typeof parentNonce !== 'string' || !HEX32.test(parentNonce) ||
      typeof childNonce !== 'string' || !HEX32.test(childNonce)) {
    fail('SCHEMA', 'invalid session transcript');
  }
  const preimage = `bitbook-wallet-session-v1\n${parentPid}\n${childPid}\n${parentNonce}\n${childNonce}`;
  return crypto.createHash('sha256').update(Buffer.from(preimage, 'utf8')).digest('hex');
}

function envelopeKeys(kind) {
  if (kind === 'req') return ['v', 'id', 'seq', 'kind', 'method', 'params', 'session', 'expires_ms'];
  if (kind === 'res') return ['v', 'id', 'seq', 'kind', 'result', 'session'];
  if (kind === 'evt') return ['v', 'id', 'seq', 'kind', 'method', 'params', 'session'];
  if (kind === 'cancel') return ['v', 'id', 'seq', 'kind', 'cancel_of', 'session'];
  if (kind === 'error') return ['v', 'id', 'seq', 'kind', 'error', 'session'];
  return null;
}

function validateEnvelope(direction, value, sessionId, expectedSeq, now) {
  if (!isDataObject(value) || !envelopeKeys(value.kind) || value.v !== 1 ||
      typeof value.id !== 'string' || !HEX32.test(value.id) ||
      !Number.isInteger(value.seq) || value.seq !== expectedSeq) {
    fail('SCHEMA', 'invalid envelope');
  }
  if (typeof value.session !== 'string' || value.session !== sessionId ||
      !HEX64.test(value.session)) fail('UNAUTH', 'session mismatch');
  if (!hasExactKeys(value, envelopeKeys(value.kind))) fail('SCHEMA', 'invalid envelope fields');
  if (direction === 'parent' && !['req', 'cancel'].includes(value.kind)) fail('SCHEMA', 'invalid direction');
  if (direction === 'child' && !['res', 'error', 'evt'].includes(value.kind)) fail('SCHEMA', 'invalid direction');
  if ((value.kind === 'req' || value.kind === 'evt') &&
      (typeof value.method !== 'string' || !METHOD.test(value.method) ||
       !isDataObject(value.params))) fail('SCHEMA', 'invalid method');
  if (value.kind === 'req' && (!Number.isInteger(value.expires_ms) || value.expires_ms <= now())) {
    fail('TIMEOUT', 'request expired');
  }
  if (value.kind === 'cancel' &&
      (typeof value.cancel_of !== 'string' || !HEX32.test(value.cancel_of))) {
    fail('SCHEMA', 'invalid cancellation');
  }
  if (value.kind === 'res' && (value.result === undefined || typeof value.result === 'function')) fail('SCHEMA', 'invalid result');
  if (value.kind === 'error' && !isDataObject(value.error)) fail('SCHEMA', 'invalid error');
}

function createProtocolSession(options = {}) {
  if (typeof options.sessionId !== 'string' || !HEX64.test(options.sessionId) ||
      typeof options.now !== 'function') fail('SCHEMA', 'invalid session options');
  const next = { parent: 1, child: 1 };
  const bound = { parent: false, child: false };
  const used = new Set();
  const pending = new Set();
  return {
    bound,
    accept(direction, value) {
      if (!['parent', 'child'].includes(direction)) fail('SCHEMA', 'invalid direction');
      validateEnvelope(direction, value, options.sessionId, next[direction], options.now);
      if (value.kind === 'req') {
        if (used.has(value.id)) fail('SCHEMA', 'duplicate id');
        used.add(value.id);
        pending.add(value.id);
      } else if (value.kind === 'cancel') {
        if (used.has(value.id) || !pending.has(value.cancel_of)) fail('SCHEMA', 'invalid cancellation');
        used.add(value.id);
        pending.delete(value.cancel_of);
      } else if (value.kind === 'res' || value.kind === 'error') {
        if (!pending.has(value.id)) fail('SCHEMA', 'uncorrelated response');
        pending.delete(value.id);
      } else if (value.kind === 'evt') {
        if (used.has(value.id)) fail('SCHEMA', 'duplicate id');
        used.add(value.id);
      }
      next[direction] += 1;
      bound[direction] = true;
      return value;
    },
  };
}

const SAFE_ERRORS = Object.freeze({
  SCHEMA: ['Invalid request', false], UNAUTH: ['Unauthorized', false], UNAVAILABLE: ['Unavailable', true],
  LOCKED: ['Wallet locked', false], SYNCING: ['Wallet syncing', true], NODE_UNAVAILABLE: ['Node unavailable', true],
  DEVICE_DISCONNECTED: ['Device disconnected', true], CAPABILITY_MISSING: ['Capability unavailable', false],
  PROTOCOL_INCOMPATIBLE: ['Protocol incompatible', false], INTENT_MISMATCH: ['Intent mismatch', false],
  EXPIRED: ['Request expired', false], CANCELLED: ['Cancelled', false], REPLAY: ['Replay rejected', false],
  WRONG_NETWORK: ['Wrong network', false], AMOUNT_INVALID: ['Invalid amount', false],
  TRANSPARENT_DOWNGRADE: ['Privacy downgrade rejected', false], ACCOUNT_BUSY: ['Account busy', true],
  WATCH_ONLY: ['Watch-only account', false], MIGRATION_REQUIRED: ['Migration required', false],
  LIMIT: ['Limit exceeded', false], STATE_CORRUPT: ['Wallet state unavailable', false],
  TIMEOUT: ['Timed out', true], INTERNAL: ['Unavailable', false],
});

function normalizeBrokerError(value) {
  const safe = value && typeof value.code === 'string' && SAFE_ERRORS[value.code]
    ? SAFE_ERRORS[value.code] : SAFE_ERRORS.INTERNAL;
  const code = value && typeof value.code === 'string' && SAFE_ERRORS[value.code] ? value.code : 'INTERNAL';
  return { code, message: safe[0], retryable: safe[1] };
}

module.exports = {
  ABSOLUTE_FRAME_LIMIT,
  CONTROL_FRAME_LIMIT,
  computeSessionId,
  createBrokerFrameDecoder,
  createProtocolSession,
  encodeBrokerFrame,
  normalizeBrokerError,
  validateHello,
  validateHelloAck,
};
