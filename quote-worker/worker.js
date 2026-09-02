'use strict';

const httpsNative = require('https');
const { Buffer } = require('buffer');
const { PROVIDERS } = require('./providers');
const {
  normalizeRateQuery,
  parseProviderBody,
  buildRateSnapshot,
} = require('./model');
const {
  encodeRateFrame,
  createRateFrameDecoder,
  validateRateRequest,
} = require('./framing');

const MAX_BODY = 65536;
const MAX_HEADER = 16384;
const CONNECT_MS = 5000;
const OVERALL_MS = 10000;
const BACKOFF_START = 30000;
const BACKOFF_CAP = 3600000;
const CACHE_MS = 5 * 60 * 1000;

function queryAsset(query) {
  return query.asset_ids[0] === 'zec-zcash' ? 'ZEC' : 'XMR';
}

function providerIdForQuery(query) {
  return queryAsset(query) === 'ZEC' ? 'coinbase-exchange-v1' : 'kraken-spot-v1';
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

function headerBytes(headers) {
  let total = 0;
  for (const [key, value] of Object.entries(headers || {})) {
    total += String(key).length + String(value).length;
  }
  return total;
}

function contentTypeOk(value) {
  if (typeof value !== 'string') return false;
  const base = value.split(';')[0].trim().toLowerCase();
  return base === 'application/json';
}

function createQuoteWorker(options = {}) {
  const https = options.https || httpsNative;
  const now = options.now || (() => Date.now());
  const setTimeoutFn = options.setTimeout || setTimeout;
  const clearTimeoutFn = options.clearTimeout || clearTimeout;
  const log = typeof options.log === 'function' ? options.log : () => {};
  let enabled = [];
  if (Array.isArray(options.enabledProviders)) {
    enabled = options.enabledProviders.slice();
  } else if (Array.isArray(options.argv) && options.argv.length > 2) {
    enabled = options.argv.slice(2);
  }
  enabled = enabled.filter((id) => Object.prototype.hasOwnProperty.call(PROVIDERS, id));

  const cache = Object.create(null);
  const backoffUntil = Object.create(null);
  const backoffDelay = Object.create(null);
  const inflight = Object.create(null);
  const timers = new Set();

  function trackTimer(id) {
    timers.add(id);
    return id;
  }

  function clearTimer(id) {
    if (id == null) return;
    clearTimeoutFn(id);
    timers.delete(id);
  }

  function queriedAt() {
    return msToTimestamp(now());
  }

  function snapshotFromQuotes(query, quotes) {
    return buildRateSnapshot(query, quotes, queriedAt());
  }

  function failProvider(providerId) {
    const delay = backoffDelay[providerId] || BACKOFF_START;
    backoffUntil[providerId] = now() + delay;
    backoffDelay[providerId] = Math.min(delay * 2, BACKOFF_CAP);
    const wait = delay;
    const timer = trackTimer(setTimeoutFn(() => {
      timers.delete(timer);
    }, wait));
    return wait;
  }

  function abortRecord(state) {
    if (!state || !state.req) return;
    try {
      if (typeof state.req.abort === 'function') state.req.abort();
    } catch (_) { /* ignore */ }
    try {
      if (typeof state.req.destroy === 'function') state.req.destroy();
    } catch (_) { /* ignore */ }
  }

  function discardResponse(res) {
    if (!res) return;
    try {
      if (typeof res.resume === 'function') res.resume();
    } catch (_) { /* ignore */ }
    try {
      if (typeof res.destroy === 'function') res.destroy();
    } catch (_) { /* ignore */ }
  }

  function settle(providerId, quote) {
    const state = inflight[providerId];
    if (!state || state.settled) return;
    state.settled = true;
    state.aborted = true;
    clearTimer(state.overall);
    delete inflight[providerId];
    if (quote) {
      cache[providerId] = quote;
      backoffDelay[providerId] = BACKOFF_START;
      delete backoffUntil[providerId];
    } else {
      failProvider(providerId);
    }
  }

  function cachedQuote(providerId) {
    const quote = cache[providerId];
    if (!quote) return null;
    const t = now();
    const fetched = Date.parse(quote.fetched_at);
    if (t > fetched + CACHE_MS) {
      delete cache[providerId];
      return null;
    }
    return quote;
  }

  function fetchProvider(providerId) {
    const provider = PROVIDERS[providerId];
    const state = { chunks: [], aborted: false, settled: false };
    inflight[providerId] = state;
    const requestOptions = {
      protocol: 'https:',
      hostname: provider.hostname,
      host: provider.hostname,
      port: provider.port,
      path: provider.path,
      method: 'GET',
      headers: {
        Accept: provider.accept,
        'Accept-Encoding': provider.accept_encoding,
      },
      minVersion: provider.tls_min_version,
      rejectUnauthorized: true,
      maxRedirects: 0,
      followRedirect: false,
      maxHeaderSize: provider.max_header_bytes,
      timeout: provider.connect_timeout_ms,
    };
    let req;
    try {
      req = https.request(requestOptions, (res) => {
        if (state.settled || state.aborted) {
          discardResponse(res);
          return;
        }
        if (headerBytes(res.headers) > MAX_HEADER) {
          abortRecord(state);
          discardResponse(res);
          settle(providerId, null);
          return;
        }
        if (res.statusCode !== 200 || !contentTypeOk(res.headers['content-type'] || res.headers['Content-Type'])) {
          discardResponse(res);
          settle(providerId, null);
          return;
        }
        res.on('data', (chunk) => {
          if (state.settled || state.aborted) return;
          state.chunks.push(Buffer.from(chunk));
          const size = state.chunks.reduce((n, part) => n + part.length, 0);
          if (size > MAX_BODY) {
            abortRecord(state);
            discardResponse(res);
            settle(providerId, null);
          }
        });
        res.on('end', () => {
          if (state.settled || state.aborted) return;
          const body = Buffer.concat(state.chunks);
          if (body.length > MAX_BODY) {
            abortRecord(state);
            settle(providerId, null);
            return;
          }
          const quote = parseProviderBody(providerId, body, msToTimestamp(now()));
          settle(providerId, quote || null);
        });
        res.on('error', () => {
          if (state.settled || state.aborted) return;
          settle(providerId, null);
        });
      });
    } catch (_) {
      settle(providerId, null);
      return;
    }
    state.req = req;
    req.on('error', () => {
      if (state.settled || state.aborted) return;
      settle(providerId, null);
    });
    req.setTimeout(CONNECT_MS, () => {
      if (state.settled || state.aborted) return;
      abortRecord(state);
      settle(providerId, null);
    });
    state.overall = trackTimer(setTimeoutFn(() => {
      timers.delete(state.overall);
      if (state.settled || !inflight[providerId]) return;
      abortRecord(state);
      settle(providerId, null);
    }, OVERALL_MS));
    req.end();
  }

  function query(rawQuery) {
    const q = normalizeRateQuery(rawQuery);
    const providerId = providerIdForQuery(q);
    const at = queriedAt();
    if (!enabled.includes(providerId)) {
      return unavailableSnapshot(q, at);
    }
    const fresh = cachedQuote(providerId);
    if (fresh) return snapshotFromQuotes(q, [fresh]);
    if (inflight[providerId]) {
      return unavailableSnapshot(q, at);
    }
    if (backoffUntil[providerId] && now() < backoffUntil[providerId]) {
      return unavailableSnapshot(q, at);
    }
    fetchProvider(providerId);
    const after = cachedQuote(providerId);
    if (after) return snapshotFromQuotes(q, [after]);
    return unavailableSnapshot(q, at);
  }

  function shutdown() {
    for (const id of Object.keys(inflight)) {
      abortRecord(inflight[id]);
      delete inflight[id];
    }
    for (const timer of timers) clearTimeoutFn(timer);
    timers.clear();
  }

  return { query, shutdown };
}

function runChild() {
  const worker = createQuoteWorker({
    argv: process.argv,
    https: httpsNative,
    now: () => Date.now(),
    setTimeout,
    clearTimeout,
  });
  const decoder = createRateFrameDecoder();
  function fail() {
    worker.shutdown();
    process.exit(1);
  }
  process.stdin.on('data', (chunk) => {
    try {
      const messages = decoder.push(Buffer.from(chunk));
      for (const message of messages) {
        const request = validateRateRequest(message);
        const snapshot = worker.query(request.params);
        process.stdout.write(encodeRateFrame({
          v: 1,
          id: request.id,
          kind: 'res',
          method: 'rate.snapshot',
          result: snapshot,
        }));
      }
    } catch (_) {
      fail();
    }
  });
  process.stdin.on('end', () => {
    worker.shutdown();
    if (decoder.incomplete || decoder.closed) {
      process.exit(1);
    }
    process.exit(0);
  });
}

if (require.main === module) {
  runChild();
}

module.exports = { createQuoteWorker };
