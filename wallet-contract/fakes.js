'use strict';

const { decodeSignedObject } = require('./canonical');

function clone(value) {
  return value === undefined ? undefined : JSON.parse(JSON.stringify(value));
}

function failure(errorCode) {
  return { ok: false, error_code: errorCode, funds_moved: false };
}

function createFakeAdapter(asset, options = {}) {
  const review = clone(options.review);
  const calls = { prepare: 0, verify: 0, broadcast: 0 };
  return {
    asset,
    calls,

    prepare() {
      calls.prepare += 1;
      return {
        ok: true,
        review: clone(review),
        artifact: {
          kind: 'synthetic-prepared-artifact',
          asset,
          review: clone(review),
        },
      };
    },

    verify(artifact, intentHash) {
      calls.verify += 1;
      if (!artifact || artifact.kind !== 'synthetic-signed-artifact' || !artifact.review) {
        return failure('INTENT_MISMATCH');
      }
      try {
        const raw = Buffer.from(JSON.stringify(artifact.review), 'utf8');
        const decoded = decodeSignedObject('review_image_v1', raw);
        if (decoded.digest !== intentHash) return failure('INTENT_MISMATCH');
      } catch (_) {
        return failure('INTENT_MISMATCH');
      }
      return { ok: true };
    },

    broadcast() {
      calls.broadcast += 1;
      const code = options.broadcastErrorCode === 'CAPABILITY_MISSING'
        ? 'CAPABILITY_MISSING'
        : 'UNAVAILABLE';
      return failure(code);
    },
  };
}

function createFakeSigner(kind, options = {}) {
  const calls = { sign: 0 };
  return {
    kind,
    calls,

    sign(prepared) {
      calls.sign += 1;
      if (kind === 'watch_only') return failure('WATCH_ONLY');
      if (options.disconnected) return failure('DEVICE_DISCONNECTED');
      if (options.failCode) return failure(options.failCode);
      if (!prepared || prepared.kind !== 'synthetic-prepared-artifact' || !prepared.review) {
        return failure('SCHEMA');
      }
      const review = Object.assign(clone(prepared.review), clone(options.mutation) || {});
      return {
        ok: true,
        artifact: {
          kind: 'synthetic-signed-artifact',
          review,
        },
      };
    },
  };
}

function sanitizeLog(event) {
  const sanitized = {};
  if (!event || (typeof event !== 'object' && typeof event !== 'function')) return sanitized;
  const validators = {
    account_id: (value) => /^account-test-[1-9][0-9]*$/.test(value),
    intent_id: (value) => /^intent-test-[1-9][0-9]*$/.test(value),
    request_id: (value) => /^[0-9a-f]{32}$/.test(value),
    state: (value) => new Set([
      'idle', 'preparing', 'prepared', 'awaiting_confirm', 'signing',
      'signed_unverified', 'verified', 'broadcasting', 'crash_recovery',
      'unknown_needs_scan', 'cancelled', 'expired', 'failed',
    ]).has(value),
    error_code: (value) => new Set([
      'SCHEMA', 'LIMIT', 'WRONG_NETWORK', 'PROTOCOL_INCOMPATIBLE',
      'CAPABILITY_MISSING', 'MIGRATION_REQUIRED', 'WATCH_ONLY',
      'DEVICE_DISCONNECTED', 'ACCOUNT_BUSY', 'INTENT_MISMATCH',
      'CANCELLED', 'EXPIRED', 'REPLAY', 'UNAVAILABLE',
    ]).has(value),
  };
  for (const [key, validate] of Object.entries(validators)) {
    const descriptor = Object.getOwnPropertyDescriptor(event, key);
    if (
      descriptor &&
      Object.prototype.hasOwnProperty.call(descriptor, 'value') &&
      typeof descriptor.value === 'string' &&
      !/[\u0000-\u001f\u007f-\u009f]/.test(descriptor.value) &&
      validate(descriptor.value)
    ) {
      sanitized[key] = descriptor.value;
    }
  }
  return sanitized;
}

module.exports = {
  createFakeAdapter,
  createFakeSigner,
  sanitizeLog,
};
