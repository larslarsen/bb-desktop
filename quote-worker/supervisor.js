'use strict';

const { spawn } = require('child_process');
const { Buffer } = require('buffer');
const { PROVIDERS } = require('./providers');
const { normalizeRateQuery, buildRateSnapshot } = require('./model');
const {
  encodeRateFrame,
  createRateFrameDecoder,
  validateRateResponse,
} = require('./framing');

function schema(message) {
  const error = new Error(message);
  error.code = 'SCHEMA';
  throw error;
}

function queryAsset(query) {
  return query.asset_ids[0] === 'zec-zcash' ? 'ZEC' : 'XMR';
}

function msToTimestamp(ms) {
  const date = new Date(ms);
  const pad = (n) => String(n).padStart(2, '0');
  return `${date.getUTCFullYear()}-${pad(date.getUTCMonth() + 1)}-${pad(date.getUTCDate())}T${pad(date.getUTCHours())}:${pad(date.getUTCMinutes())}:${pad(date.getUTCSeconds())}Z`;
}

function unavailableSnapshot(query, queriedAt) {
  const asset = queryAsset(query);
  return {
    v: 1,
    queried_at: queriedAt,
    quotes: [],
    display: {
      asset,
      quote_currency: 'USD',
      method: 'unavailable',
      source_ids: [],
      label: 'fiat estimate unavailable',
    },
  };
}

function createQuoteSupervisor(options = {}) {
  const now = options.now || (() => Date.now());
  const setTimeoutFn = options.setTimeout || setTimeout;
  const clearTimeoutFn = options.clearTimeout || clearTimeout;
  const spawnFn = options.spawn || spawn;
  const log = typeof options.log === 'function' ? options.log : () => {};
  const execPath = options.execPath;
  const workerPath = options.workerPath;
  let enabled;
  if (options.enabledProviders === undefined) {
    enabled = [];
  } else if (!Array.isArray(options.enabledProviders)) {
    schema('enabledProviders');
  } else {
    enabled = options.enabledProviders.slice();
  }
  const seen = new Set();
  for (const id of enabled) {
    if (typeof id !== 'string' || !Object.prototype.hasOwnProperty.call(PROVIDERS, id) || seen.has(id)) {
      schema('enabled provider id');
    }
    seen.add(id);
  }

  const timers = new Set();
  const cache = Object.create(null);
  let child = null;
  let decoder = null;
  let pendingId = null;
  let pendingQuery = null;
  let pendingResult = null;
  let closed = false;
  let failing = false;
  let seq = 0;

  function nextId() {
    seq += 1;
    return seq.toString(16).padStart(32, '0');
  }

  function queriedAt() {
    return msToTimestamp(now());
  }

  function clearTimers() {
    for (const timer of timers) clearTimeoutFn(timer);
    timers.clear();
  }

  function detachChild() {
    if (!child) return;
    const stdio = [child.stdout, child.stderr];
    for (const stream of stdio) {
      if (!stream) continue;
      if (typeof stream.removeListener === 'function') {
        stream.removeListener('data', onStdout);
        stream.removeListener('end', onStdoutEnd);
        stream.removeListener('data', onStderr);
      }
    }
    if (typeof child.removeListener === 'function') {
      child.removeListener('error', onChildGone);
      child.removeListener('exit', onChildGone);
      child.removeListener('close', onChildGone);
    }
  }

  function failChild() {
    pendingId = null;
    pendingQuery = null;
    pendingResult = null;
    decoder = null;
    for (const key of Object.keys(cache)) delete cache[key];
    if (failing) return;
    failing = true;
    const current = child;
    detachChild();
    child = null;
    if (
      current &&
      !current.killed &&
      current.exitCode == null &&
      current.signalCode == null &&
      typeof current.kill === 'function'
    ) {
      try {
        current.kill();
      } catch (_) { /* ignore */ }
    }
    failing = false;
  }

  function onChildGone() {
    if (closed) return;
    failChild();
  }

  function onStderr() {}

  function acceptResponse(message) {
    if (!pendingId || !pendingQuery) {
      failChild();
      return;
    }
    let response;
    try {
      response = validateRateResponse(message);
    } catch (_) {
      failChild();
      return;
    }
    if (response.id !== pendingId) {
      failChild();
      return;
    }
    if (pendingResult) {
      failChild();
      return;
    }
    const rebuilt = buildRateSnapshot(pendingQuery, response.result.quotes, queriedAt());
    if (rebuilt.quotes.length === 1) {
      cache[queryAsset(pendingQuery)] = rebuilt.quotes[0];
    }
    pendingResult = rebuilt;
    pendingId = null;
    pendingQuery = null;
  }

  function onStdout(chunk) {
    if (!decoder || closed) return;
    try {
      const messages = decoder.push(Buffer.from(chunk));
      for (const message of messages) acceptResponse(message);
    } catch (_) {
      failChild();
    }
  }

  function onStdoutEnd() {
    if (decoder && decoder.incomplete) failChild();
    else if (pendingId && !pendingResult) failChild();
  }

  function ensureChild() {
    if (child && !child.killed) return;
    failing = false;
    const cleanEnv = { LANG: 'C.UTF-8' };
    const spawned = spawnFn(execPath, [workerPath].concat(enabled), {
      shell: false,
      stdio: ['pipe', 'pipe', 'pipe'],
      env: cleanEnv,
    });
    child = spawned;
    decoder = createRateFrameDecoder();
    pendingId = null;
    pendingQuery = null;
    pendingResult = null;
    if (spawned.stdout && typeof spawned.stdout.on === 'function') {
      spawned.stdout.on('data', onStdout);
      spawned.stdout.on('end', onStdoutEnd);
    }
    if (spawned.stderr && typeof spawned.stderr.on === 'function') {
      if (typeof spawned.stderr.resume === 'function') spawned.stderr.resume();
      spawned.stderr.on('data', onStderr);
    }
    if (typeof spawned.on === 'function') {
      spawned.on('error', onChildGone);
      spawned.on('exit', onChildGone);
      spawned.on('close', onChildGone);
    }
  }

  function cachedSnapshot(query, at) {
    const asset = queryAsset(query);
    const quote = cache[asset];
    if (!quote) return null;
    const rebuilt = buildRateSnapshot(query, [quote], at);
    if (rebuilt.quotes.length !== 1) {
      delete cache[asset];
      return null;
    }
    return rebuilt;
  }

  function query(rawQuery) {
    const q = normalizeRateQuery(rawQuery);
    const at = queriedAt();
    if (closed) return unavailableSnapshot(q, at);
    if (enabled.length === 0) return unavailableSnapshot(q, at);
    if (pendingId) {
      failChild();
      return unavailableSnapshot(q, at);
    }
    const cached = cachedSnapshot(q, at);
    if (cached) return cached;
    ensureChild();
    if (!child || child.killed) return unavailableSnapshot(q, at);
    const id = nextId();
    pendingId = id;
    pendingQuery = q;
    pendingResult = null;
    const request = {
      v: 1,
      id,
      kind: 'req',
      method: 'rate.query',
      params: q,
    };
    try {
      child.stdin.write(encodeRateFrame(request));
    } catch (_) {
      failChild();
      return unavailableSnapshot(q, at);
    }
    if (!child || child.killed) return unavailableSnapshot(q, at);
    if (pendingResult) {
      const result = pendingResult;
      pendingResult = null;
      return result;
    }
    return unavailableSnapshot(q, at);
  }

  function shutdown() {
    if (closed) return;
    closed = true;
    pendingId = null;
    pendingQuery = null;
    pendingResult = null;
    decoder = null;
    for (const key of Object.keys(cache)) delete cache[key];
    const current = child;
    detachChild();
    if (current && !current.killed && typeof current.kill === 'function') current.kill();
    child = null;
    clearTimers();
  }

  return { query, shutdown };
}

module.exports = { createQuoteSupervisor };
