'use strict';

const { Buffer } = require('buffer');

const ABSOLUTE_LIMIT = 1024 * 1024;

function frameError(code, message) {
  const error = new Error(message);
  error.code = code;
  return error;
}

function strictObject(body) {
  const text = body.toString('utf8');
  if (!Buffer.from(text, 'utf8').equals(body)) throw frameError('SCHEMA', 'malformed frame UTF-8');
  let value;
  try {
    value = JSON.parse(text);
  } catch (_) {
    throw frameError('SCHEMA', 'invalid framed JSON');
  }
  if (!value || typeof value !== 'object' || Array.isArray(value)) {
    throw frameError('SCHEMA', 'frame must contain one JSON object');
  }
  return value;
}

function encodeFrame(value) {
  if (!value || typeof value !== 'object' || Array.isArray(value)) {
    throw frameError('SCHEMA', 'frame value must be an object');
  }
  const body = Buffer.from(JSON.stringify(value), 'utf8');
  if (body.length === 0) throw frameError('SCHEMA', 'empty frame body');
  if (body.length > ABSOLUTE_LIMIT) throw frameError('LIMIT', 'frame exceeds absolute limit');
  const prefix = Buffer.alloc(4);
  prefix.writeUInt32BE(body.length, 0);
  return Buffer.concat([prefix, body]);
}

function createFrameDecoder(options = {}) {
  const requested = Number.isInteger(options.limitBytes) && options.limitBytes > 0
    ? options.limitBytes
    : 64 * 1024;
  const limit = Math.min(requested, ABSOLUTE_LIMIT);
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
        if (length > limit || length > ABSOLUTE_LIMIT) {
          return closeAndThrow('LIMIT', 'frame length exceeds configured limit');
        }
        if (unread.length < length + 4) break;
        const body = unread.subarray(4, length + 4);
        unread = unread.subarray(length + 4);
        try {
          values.push(strictObject(body));
        } catch (error) {
          return closeAndThrow(error.code || 'SCHEMA', error.message);
        }
      }
      return values;
    },
  };
}

module.exports = {
  createFrameDecoder,
  encodeFrame,
};
