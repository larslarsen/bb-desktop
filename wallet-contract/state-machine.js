'use strict';

const crypto = require('crypto');
const { decodeSignedObject } = require('./canonical');
const { evaluateCapability } = require('./model');

const adapterLocks = new WeakMap();
const STABLE_ERROR_CODES = new Set([
  'SCHEMA',
  'LIMIT',
  'WRONG_NETWORK',
  'PROTOCOL_INCOMPATIBLE',
  'CAPABILITY_MISSING',
  'MIGRATION_REQUIRED',
  'WATCH_ONLY',
  'DEVICE_DISCONNECTED',
  'ACCOUNT_BUSY',
  'INTENT_MISMATCH',
  'CANCELLED',
  'EXPIRED',
  'REPLAY',
  'UNAVAILABLE',
]);

function clone(value) {
  return value === undefined ? undefined : JSON.parse(JSON.stringify(value));
}

function dependencyFailureCode(value, missingFallback = 'UNAVAILABLE') {
  if (!value || typeof value !== 'object') return missingFallback;
  const descriptor = Object.getOwnPropertyDescriptor(value, 'error_code');
  if (!descriptor) return missingFallback;
  if (!Object.prototype.hasOwnProperty.call(descriptor, 'value')) return 'UNAVAILABLE';
  return typeof descriptor.value === 'string' && STABLE_ERROR_CODES.has(descriptor.value)
    ? descriptor.value
    : 'UNAVAILABLE';
}

function createIntentMachine(options) {
  const account = clone(options.account);
  const request = clone(options.request);
  const adapter = options.adapter;
  const signer = options.signer;
  const now = options.now;
  const getRequestStatus = options.getRequestStatus;
  const restored = options.restoredState ? clone(options.restoredState) : null;
  const boundRequest = decodeSignedObject(
    'payment_request_v1',
    Buffer.from(JSON.stringify(request), 'utf8')
  );
  const boundMemoHash = crypto.createHash('sha256').update(Buffer.from(request.memo, 'utf8')).digest('hex');
  let state = restored ? restored.state : 'idle';
  let review = restored ? restored.review : null;
  let preparedArtifact = null;
  let signedArtifact = restored ? restored.signed_artifact : null;
  let intentHash = restored ? restored.intent_hash : null;
  let confirmationCount = restored ? restored.confirmation_count || 0 : 0;
  let errorCode = null;
  let ownsLock = false;
  let requiresRecovery = Boolean(
    restored && ['signed_unverified', 'verified'].includes(restored.state)
  );

  function result(extra = {}) {
    return Object.assign({ ok: true, state }, extra);
  }

  function releaseLock() {
    if (!ownsLock) return;
    const locks = adapterLocks.get(adapter);
    if (locks && locks.get(account.account_id) === machine) locks.delete(account.account_id);
    ownsLock = false;
  }

  function fail(code, nextState, extra = {}) {
    if (nextState) state = nextState;
    errorCode = code;
    if (['failed', 'cancelled', 'expired', 'unknown_needs_scan'].includes(state)) releaseLock();
    return Object.assign({ ok: false, state, error_code: code, funds_moved: false }, extra);
  }

  function acquireLock() {
    let locks = adapterLocks.get(adapter);
    if (!locks) {
      locks = new Map();
      adapterLocks.set(adapter, locks);
    }
    const owner = locks.get(account.account_id);
    if (owner && owner !== machine) return false;
    locks.set(account.account_id, machine);
    ownsLock = true;
    return true;
  }

  function barrier() {
    let status;
    let current;
    try {
      status = getRequestStatus();
      current = Date.parse(now());
    } catch (_) {
      return fail('UNAVAILABLE', 'failed');
    }
    if (status === 'cancelled') return fail('CANCELLED', 'cancelled');
    if (status === 'expired') return fail('EXPIRED', 'expired');
    if (status === 'paid') return fail('REPLAY', 'failed');
    const expiry = Date.parse(request.expires_at);
    if (!Number.isFinite(current) || !Number.isFinite(expiry) || current >= expiry) {
      return fail('EXPIRED', 'expired');
    }
    return null;
  }

  const machine = {
    begin() {
      if (state !== 'idle') return fail('SCHEMA', null);
      const blocked = barrier();
      if (blocked) return blocked;
      state = 'preparing';
      errorCode = null;
      return result();
    },

    prepare() {
      if (state !== 'preparing') return fail('SCHEMA', null);
      if (
        !['software', 'hardware_backed', 'watch_only'].includes(account.kind) ||
        !adapter || adapter.asset !== account.asset ||
        !signer || signer.kind !== account.kind
      ) {
        return fail('CAPABILITY_MISSING', 'failed');
      }
      const capability = evaluateCapability(account, request);
      if (!capability.can_spend) return fail(capability.error_code || 'CAPABILITY_MISSING', 'failed');
      if (!acquireLock()) return fail('ACCOUNT_BUSY', 'failed');
      let prepared;
      try {
        prepared = adapter.prepare(clone(request), clone(account));
      } catch (_) {
        return fail('UNAVAILABLE', 'failed');
      }
      if (!prepared || prepared.ok !== true || !prepared.review || !prepared.artifact) {
        return fail(dependencyFailureCode(prepared), 'failed');
      }
      let decodedReview;
      try {
        decodedReview = decodeSignedObject(
          'review_image_v1',
          Buffer.from(JSON.stringify(prepared.review), 'utf8')
        );
      } catch (error) {
        return fail(error.code || 'SCHEMA', 'failed');
      }
      const authoritative = {
        account_id: account.account_id,
        request_id: request.request_id,
        payment_request_hash: boundRequest.digest,
        payer_peer_id: request.payer_peer_id,
        payee_peer_id: request.payee_peer_id,
        asset: request.asset,
        network: request.network,
        amount_atomic: request.amount_atomic,
        receiver: request.receiver,
        receiver_kind: request.receiver_kind,
        expires_at: request.expires_at,
        memo_hash: boundMemoHash,
      };
      if (Object.keys(authoritative).some((key) => decodedReview.value[key] !== authoritative[key])) {
        return fail('INTENT_MISMATCH', 'failed');
      }
      review = clone(decodedReview.value);
      preparedArtifact = clone(prepared.artifact);
      state = 'prepared';
      errorCode = null;
      return result({ review: clone(review) });
    },

    showReview() {
      if (state !== 'prepared') return fail('SCHEMA', null);
      state = 'awaiting_confirm';
      return result({ review: clone(review) });
    },

    confirm() {
      if (state !== 'awaiting_confirm') return fail('SCHEMA', null);
      const blocked = barrier();
      if (blocked) return blocked;
      try {
        intentHash = decodeSignedObject(
          'review_image_v1',
          Buffer.from(JSON.stringify(review), 'utf8')
        ).digest;
      } catch (error) {
        return fail(error.code || 'SCHEMA', 'failed');
      }
      confirmationCount += 1;
      state = 'signing';
      errorCode = null;
      return result({ intent_hash: intentHash });
    },

    completeSign() {
      if (state !== 'signing') return fail('SCHEMA', null);
      let signed;
      try {
        signed = signer.sign(clone(preparedArtifact));
      } catch (_) {
        return fail('UNAVAILABLE', 'failed');
      }
      if (!signed || signed.ok !== true || !signed.artifact) {
        return fail(dependencyFailureCode(signed), 'failed');
      }
      signedArtifact = clone(signed.artifact);
      state = 'signed_unverified';
      errorCode = null;
      return result();
    },

    verifySigned() {
      if (state !== 'signed_unverified') return fail('SCHEMA', null);
      if (requiresRecovery) return fail('SCHEMA', null);
      const blocked = barrier();
      if (blocked) return blocked;
      let verified;
      try {
        verified = adapter.verify(clone(signedArtifact), intentHash);
      } catch (_) {
        return fail('UNAVAILABLE', 'failed');
      }
      if (!verified || verified.ok !== true) {
        return fail(dependencyFailureCode(verified, 'INTENT_MISMATCH'), 'failed');
      }
      state = 'verified';
      errorCode = null;
      return result();
    },

    broadcast() {
      if (state === 'cancelled') return fail('CANCELLED', null);
      if (state === 'expired') return fail('EXPIRED', null);
      if (state !== 'verified') return fail('SCHEMA', null);
      if (requiresRecovery) return fail('SCHEMA', null);
      const blocked = barrier();
      if (blocked) return blocked;
      state = 'broadcasting';
      let broadcasted;
      try {
        broadcasted = adapter.broadcast(clone(signedArtifact));
      } catch (_) {
        return fail('UNAVAILABLE', 'failed', { funds_moved: false });
      }
      if (!broadcasted || broadcasted.ok !== true) {
        return fail(
          dependencyFailureCode(broadcasted),
          'failed',
          { funds_moved: false }
        );
      }
      return fail('UNAVAILABLE', 'failed', { funds_moved: false });
    },

    cancel() {
      if (!['preparing', 'prepared', 'awaiting_confirm', 'signing', 'signed_unverified', 'verified', 'crash_recovery'].includes(state)) {
        return fail('SCHEMA', null);
      }
      state = 'cancelled';
      errorCode = 'CANCELLED';
      releaseLock();
      return result();
    },

    expire() {
      if (!['preparing', 'prepared', 'awaiting_confirm', 'signing', 'signed_unverified', 'verified', 'crash_recovery'].includes(state)) {
        return fail('SCHEMA', null);
      }
      state = 'expired';
      errorCode = 'EXPIRED';
      releaseLock();
      return result();
    },

    crash() {
      if (['preparing', 'prepared', 'awaiting_confirm', 'signing'].includes(state)) {
        return fail('UNAVAILABLE', 'failed');
      }
      if (state === 'signed_unverified' || state === 'verified') {
        state = 'crash_recovery';
        errorCode = null;
        return result();
      }
      if (state === 'crash_recovery') return result();
      if (state === 'broadcasting') return fail('UNAVAILABLE', 'unknown_needs_scan');
      return fail('SCHEMA', null);
    },

    confirmRecovery() {
      if (state !== 'crash_recovery') return fail('SCHEMA', null);
      const blocked = barrier();
      if (blocked) return blocked;
      if (!review || !intentHash || !signedArtifact) return fail('SCHEMA', 'failed');
      if (!ownsLock && !acquireLock()) return fail('ACCOUNT_BUSY', null);
      confirmationCount += 1;
      state = 'signed_unverified';
      errorCode = null;
      requiresRecovery = false;
      return result();
    },

    resume() {
      if (state === 'unknown_needs_scan') return fail('UNAVAILABLE', null);
      return fail('SCHEMA', null);
    },

    snapshot() {
      const snapshot = {
        state,
        confirmation_count: confirmationCount,
      };
      if (intentHash) snapshot.intent_hash = intentHash;
      if (errorCode) snapshot.error_code = errorCode;
      return snapshot;
    },
  };

  return machine;
}

module.exports = {
  createIntentMachine,
};
