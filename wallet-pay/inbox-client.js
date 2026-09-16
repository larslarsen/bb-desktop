'use strict';

const fs = require('fs');
const http = require('http');
const path = require('path');
const { decodeSignedObject } = require('../wallet-contract/canonical');

const DESCRIPTOR_CAP = 16 * 1024;
const HEADER_CAP = 16 * 1024;
const BODY_CAP = 4 * 1024 * 1024;
const PEEK_CAP = 256;
const CANONICAL_CAP = 64 * 1024;
const PEER_CAP = 256;
const NESTING_CAP = 8;
const KEY_ENCODED_CAP = 1024;
const REQUEST_DEADLINE_MS = 5000;
const HEX32 = /^[0-9a-f]{32}$/;
const HEX64 = /^[0-9a-f]{64}$/;
const ENDPOINT = /^http:\/\/127\.0\.0\.1:([1-9][0-9]{0,3}|[1-5][0-9]{4}|6[0-4][0-9]{3}|65[0-4][0-9]{2}|655[0-2][0-9]|6553[0-5])$/;
const RECEIVED_AT = /^([0-9]{4})-([0-9]{2})-([0-9]{2})T([0-9]{2}):([0-9]{2}):([0-9]{2})(?:\.([0-9]{1,9}))?Z$/;
const RECORD_KEYS = ['signed', 'digest', 'direction', 'received_at'];
const SIGNED_KEYS = ['version', 'kind', 'canonical', 'public_key', 'signature'];
const KIND_REQUEST = 'request';
const KIND_STATUS = 'status';
const REQUEST_FIELDS = [
  'request_id', 'digest', 'payee_peer_id', 'asset', 'network', 'amount_atomic',
  'amount_display', 'memo', 'created_at', 'expires_at', 'status',
];

function closedReply(state, peerId, requests) {
  return {
    v: 1,
    state,
    peer_id: typeof peerId === 'string' ? peerId : '',
    requests: Array.isArray(requests) ? requests : [],
  };
}

function emptyNonReady(state, peerId) {
  return closedReply(state, peerId || '', []);
}

function utf8Len(value) {
  return Buffer.byteLength(String(value), 'utf8');
}

function exactKeys(value, keys) {
  if (!value || typeof value !== 'object' || Array.isArray(value)) return false;
  const names = Object.keys(value);
  return names.length === keys.length && keys.every((key) => Object.prototype.hasOwnProperty.call(value, key));
}

function jsonNesting(text) {
  let depth = 0;
  let max = 0;
  let inString = false;
  let escape = false;
  for (let i = 0; i < text.length; i += 1) {
    const ch = text[i];
    if (inString) {
      if (escape) escape = false;
      else if (ch === '\\') escape = true;
      else if (ch === '"') inString = false;
      continue;
    }
    if (ch === '"') {
      inString = true;
      continue;
    }
    if (ch === '{' || ch === '[') {
      depth += 1;
      if (depth > max) max = depth;
    } else if (ch === '}' || ch === ']') depth -= 1;
  }
  return max;
}

function formatAtomic(atomic, scale) {
  const digits = String(atomic);
  if (digits.length <= scale) return `0.${digits.padStart(scale, '0')}`;
  return `${digits.slice(0, digits.length - scale)}.${digits.slice(digits.length - scale)}`;
}

function amountDisplay(asset, atomic) {
  if (asset === 'ZEC') return formatAtomic(atomic, 8);
  if (asset === 'XMR') return formatAtomic(atomic, 12);
  return null;
}

function parseReceivedAt(value) {
  if (typeof value !== 'string') return NaN;
  const match = RECEIVED_AT.exec(value);
  if (!match) return NaN;
  const year = Number(match[1]);
  const month = Number(match[2]);
  const day = Number(match[3]);
  const hour = Number(match[4]);
  const minute = Number(match[5]);
  const second = Number(match[6]);
  const fraction = match[7] || '';
  if (year < 2020 || year > 2100 || month < 1 || month > 12 || hour > 23 || minute > 59 || second > 59) {
    return NaN;
  }
  const leap = year % 4 === 0 && (year % 100 !== 0 || year % 400 === 0);
  const days = [31, leap ? 29 : 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
  if (day < 1 || day > days[month - 1]) return NaN;
  const millis = Number(`${fraction}000`.slice(0, 3));
  const epoch = Date.UTC(year, month - 1, day, hour, minute, second, millis);
  const iso = new Date(epoch).toISOString();
  if (!iso.startsWith(`${match[1]}-${match[2]}-${match[3]}T${match[4]}:${match[5]}:${match[6]}`)) {
    return NaN;
  }
  return epoch;
}

function canonicalBase64(value) {
  if (typeof value !== 'string' || value.length === 0 || value.length > KEY_ENCODED_CAP) return null;
  try {
    const buf = Buffer.from(value, 'base64');
    if (buf.length === 0 || buf.toString('base64') !== value) return null;
    return buf;
  } catch (_) {
    return null;
  }
}

function lstatSafe(target) {
  return fs.promises.lstat(target);
}

function modeOf(stat) {
  return stat.mode & 0o777;
}

function codedError(code) {
  const err = new Error(code);
  err.code = code;
  return err;
}

function unavailableError() {
  throw codedError('unavailable');
}

async function assertSelectedRoot(dirPath) {
  let stat;
  try {
    stat = await lstatSafe(dirPath);
  } catch (_) {
    unavailableError();
  }
  if (stat.isSymbolicLink() || !stat.isDirectory()) unavailableError();
}

async function assertPrivateDir(dirPath) {
  let stat;
  try {
    stat = await lstatSafe(dirPath);
  } catch (_) {
    unavailableError();
  }
  if (stat.isSymbolicLink() || !stat.isDirectory() || modeOf(stat) !== 0o700) unavailableError();
  if (typeof process.getuid === 'function' && stat.uid !== process.getuid()) unavailableError();
}

async function readDescriptorFile(filePath) {
  let listed;
  try {
    listed = await lstatSafe(filePath);
  } catch (_) {
    unavailableError();
  }
  if (listed.isSymbolicLink() || !listed.isFile()) unavailableError();
  const flags = fs.constants.O_RDONLY | fs.constants.O_NOFOLLOW | fs.constants.O_NONBLOCK;
  const handle = await fs.promises.open(filePath, flags);
  try {
    const stat = await handle.stat();
    if (!stat.isFile() || modeOf(stat) !== 0o600 || stat.size <= 0 || stat.size > DESCRIPTOR_CAP) {
      throw codedError('unavailable');
    }
    if (typeof process.getuid === 'function' && stat.uid !== process.getuid()) {
      throw codedError('unavailable');
    }
    const buf = Buffer.alloc(stat.size + 1);
    const { bytesRead } = await handle.read(buf, 0, buf.length, 0);
    if (bytesRead !== stat.size) throw codedError('unavailable');
    const slice = buf.subarray(0, bytesRead);
    if (!Buffer.from(slice.toString('utf8'), 'utf8').equals(slice)) throw codedError('invalid');
    return slice.toString('utf8');
  } finally {
    await handle.close();
  }
}

function parseDescriptor(text) {
  let parsed;
  try {
    parsed = JSON.parse(text);
  } catch (_) {
    return null;
  }
  if (!exactKeys(parsed, ['v', 'endpoint', 'peer_id', 'instance_id', 'token'])) return null;
  if (parsed.v !== 1) return null;
  if (typeof parsed.peer_id !== 'string' || parsed.peer_id.length === 0 || utf8Len(parsed.peer_id) > PEER_CAP) {
    return null;
  }
  if (typeof parsed.instance_id !== 'string' || !HEX32.test(parsed.instance_id)) return null;
  if (typeof parsed.token !== 'string' || !HEX64.test(parsed.token)) return null;
  if (typeof parsed.endpoint !== 'string' || !ENDPOINT.test(parsed.endpoint)) return null;
  const port = Number(parsed.endpoint.slice('http://127.0.0.1:'.length));
  if (!Number.isInteger(port) || port < 1 || port > 65535) return null;
  return {
    v: 1,
    endpoint: parsed.endpoint,
    peer_id: parsed.peer_id,
    instance_id: parsed.instance_id,
    token: parsed.token,
    port,
  };
}

function fetchRecords(descriptor, signal) {
  return new Promise((resolve, reject) => {
    let settled = false;
    let response = null;
    let onAbort = null;
    const req = http.request({
      hostname: '127.0.0.1',
      port: descriptor.port,
      path: '/v1/payment/records',
      method: 'GET',
      agent: false,
      timeout: REQUEST_DEADLINE_MS,
      maxHeaderSize: HEADER_CAP,
      headers: {
        Authorization: `Bearer ${descriptor.token}`,
        'X-BitBook-Instance': descriptor.instance_id,
        Accept: 'application/json',
        'Accept-Encoding': 'identity',
        Connection: 'close',
        Host: `127.0.0.1:${descriptor.port}`,
      },
    });
    const destroyOwned = () => {
      try { req.destroy(); } catch (_) { /* owned */ }
      if (response) {
        response.removeAllListeners('data');
        response.removeAllListeners('end');
        response.removeAllListeners('error');
        response.removeAllListeners('aborted');
        response.removeAllListeners('close');
        try { response.destroy(); } catch (_) { /* owned */ }
        response = null;
      }
    };
    const cleanup = () => {
      clearTimeout(timer);
      if (signal && onAbort) {
        signal.removeEventListener('abort', onAbort);
        onAbort = null;
      }
    };
    const finish = (fn) => {
      if (settled) return;
      settled = true;
      cleanup();
      fn();
    };
    const fail = (code) => {
      destroyOwned();
      finish(() => reject(codedError(code)));
    };
    const timer = setTimeout(() => fail('unavailable'), REQUEST_DEADLINE_MS);
    const consume = (limit, onBody) => {
      const chunks = [];
      let total = 0;
      let completed = false;
      const onData = (chunk) => {
        if (settled) return;
        if (total + chunk.length > limit) {
          fail(limit === BODY_CAP ? 'too_large' : 'unavailable');
          return;
        }
        total += chunk.length;
        chunks.push(chunk);
      };
      response.on('data', onData);
      response.on('end', () => {
        completed = true;
        if (settled) return;
        onBody(Buffer.concat(chunks, total));
      });
      response.on('error', () => fail('unavailable'));
      response.on('aborted', () => fail('unavailable'));
      response.on('close', () => {
        if (!settled && !completed) fail('unavailable');
      });
    };
    req.on('response', (res) => {
      response = res;
      if (settled) {
        destroyOwned();
        return;
      }
      const encoding = res.headers['content-encoding'];
      if (res.statusCode === 503) {
        consume(PEEK_CAP, (buf) => {
          let code = 'unavailable';
          try {
            const parsed = JSON.parse(buf.toString('utf8'));
            if (parsed && parsed.error === 'TOO_LARGE') code = 'too_large';
          } catch (_) { /* truncated or non-JSON */ }
          fail(code);
        });
        return;
      }
      if (res.statusCode !== 200) {
        fail('unavailable');
        return;
      }
      if (encoding && encoding !== 'identity') {
        fail('invalid');
        return;
      }
      consume(BODY_CAP, (buf) => {
        finish(() => resolve(buf));
      });
    });
    req.on('error', () => fail('unavailable'));
    req.on('timeout', () => fail('unavailable'));
    req.on('close', () => {
      if (!settled && !response) fail('unavailable');
    });
    if (signal) {
      onAbort = () => fail('aborted');
      if (signal.aborted) onAbort();
      else signal.addEventListener('abort', onAbort, { once: true });
    }
    req.end();
  });
}

function decodeSnapshot(buffer, descriptor, nowMs, localPeer) {
  if (!Buffer.from(buffer.toString('utf8'), 'utf8').equals(buffer)) throw codedError('invalid');
  let body;
  try {
    body = JSON.parse(buffer.toString('utf8'));
  } catch (_) {
    throw codedError('invalid');
  }
  if (!exactKeys(body, ['v', 'peer_id', 'instance_id', 'records'])) throw codedError('invalid');
  if (typeof body.peer_id !== 'string' || utf8Len(body.peer_id) > PEER_CAP) throw codedError('invalid');
  if (typeof body.instance_id !== 'string') throw codedError('invalid');
  if (!Array.isArray(body.records)) throw codedError('invalid');
  if (body.v !== 1) throw codedError('invalid');
  if (body.peer_id !== descriptor.peer_id) {
    const err = codedError('identity_changed');
    err.peer_id = body.peer_id;
    throw err;
  }
  if (body.instance_id !== descriptor.instance_id) throw codedError('invalid');
  const requests = new Map();
  const cancellations = [];
  const seenRequest = new Set();
  for (const record of body.records) {
    if (!exactKeys(record, RECORD_KEYS) || !exactKeys(record.signed, SIGNED_KEYS)) throw codedError('invalid');
    if (record.signed.version !== 1) throw codedError('invalid');
    if (record.signed.kind !== KIND_REQUEST && record.signed.kind !== KIND_STATUS) throw codedError('invalid');
    if (typeof record.digest !== 'string' || !HEX64.test(record.digest)) throw codedError('invalid');
    if (record.direction !== 'inbound' && record.direction !== 'outbound') throw codedError('invalid');
    if (!Number.isFinite(parseReceivedAt(record.received_at))) throw codedError('invalid');
    if (typeof record.signed.canonical !== 'string' || utf8Len(record.signed.canonical) > CANONICAL_CAP) {
      throw codedError('invalid');
    }
    if (jsonNesting(record.signed.canonical) > NESTING_CAP) throw codedError('invalid');
    const keyBytes = canonicalBase64(record.signed.public_key);
    const signature = canonicalBase64(record.signed.signature);
    if (!keyBytes || !signature) throw codedError('invalid');
    const kind = record.signed.kind === KIND_REQUEST ? 'payment_request_v1' : 'payment_status_event_v1';
    let decoded;
    try {
      decoded = decodeSignedObject(kind, Buffer.from(record.signed.canonical, 'utf8'));
    } catch (_) {
      throw codedError('invalid');
    }
    if (decoded.canonical !== record.signed.canonical || decoded.digest !== record.digest) {
      throw codedError('invalid');
    }
    if (record.signed.kind === KIND_STATUS) {
      if (decoded.value.status !== 'cancelled' || decoded.value.tx_ref !== '') throw codedError('invalid');
      cancellations.push({
        request_id: decoded.value.request_id,
        direction: record.direction,
        public_key: keyBytes,
      });
      continue;
    }
    if (seenRequest.has(decoded.value.request_id)) throw codedError('invalid');
    seenRequest.add(decoded.value.request_id);
    if (record.direction === 'outbound') {
      if (decoded.value.payee_peer_id !== localPeer) throw codedError('invalid');
    } else if (decoded.value.payer_peer_id !== localPeer) {
      throw codedError('invalid');
    }
    const display = amountDisplay(decoded.value.asset, decoded.value.amount_atomic);
    if (!display) throw codedError('invalid');
    requests.set(decoded.value.request_id, {
      request_id: decoded.value.request_id,
      digest: decoded.digest,
      payee_peer_id: decoded.value.payee_peer_id,
      asset: decoded.value.asset,
      network: decoded.value.network,
      amount_atomic: decoded.value.amount_atomic,
      amount_display: display,
      memo: decoded.value.memo,
      created_at: decoded.value.created_at,
      expires_at: decoded.value.expires_at,
      status: 'requested',
      public_key: keyBytes,
      direction: record.direction,
      display: record.direction === 'inbound',
      expires_ms: parseReceivedAt(decoded.value.expires_at) || Date.parse(decoded.value.expires_at),
    });
  }
  for (const cancel of cancellations) {
    const row = requests.get(cancel.request_id);
    if (!row) continue;
    if (row.direction !== cancel.direction || !row.public_key.equals(cancel.public_key)) {
      throw codedError('invalid');
    }
    row.status = 'cancelled';
  }
  const rows = [];
  for (const row of requests.values()) {
    if (!row.display) continue;
    if (row.status !== 'cancelled' && nowMs >= row.expires_ms) row.status = 'expired';
    const dto = {};
    for (const field of REQUEST_FIELDS) dto[field] = row[field];
    rows.push(dto);
  }
  rows.sort((a, b) => {
    const byTime = Date.parse(b.created_at) - Date.parse(a.created_at);
    if (byTime !== 0) return byTime;
    if (a.request_id < b.request_id) return -1;
    if (a.request_id > b.request_id) return 1;
    return 0;
  });
  return rows;
}

function mapFetchError(code) {
  if (code === 'too_large') return 'too_large';
  if (code === 'invalid') return 'invalid';
  if (code === 'identity_changed') return 'identity_changed';
  return 'unavailable';
}

function createPaymentInboxClient(options = {}) {
  const platform = options.platform || process.platform;
  const homedir = options.homedir || (() => require('os').homedir());
  const dialog = options.dialog;
  const parentWindow = options.parentWindow || (() => null);
  const now = options.now || (() => Date.now());
  const httpFetch = options.fetchRecords || fetchRecords;
  let selectedDir = null;
  let pinnedPeer = null;
  let generation = 0;
  let pendingGet = null;
  let pickerBusy = false;
  let disposed = false;
  const controllers = new Set();

  function defaultDir() {
    return path.join(homedir(), '.bitbook', 'modern');
  }

  function stale(gen) {
    return disposed || gen !== generation;
  }

  function abortAll() {
    for (const controller of controllers) controller.abort();
    controllers.clear();
  }

  async function loadDescriptor(root) {
    await assertSelectedRoot(root);
    const privateDir = path.join(root, 'local-client');
    await assertPrivateDir(privateDir);
    const text = await readDescriptorFile(path.join(privateDir, 'connection.json'));
    const descriptor = parseDescriptor(text);
    if (!descriptor) throw codedError('unavailable');
    return descriptor;
  }

  async function runGet(gen) {
    if (platform !== 'linux') return emptyNonReady('unsupported', '');
    if (stale(gen)) return emptyNonReady('unavailable', pinnedPeer || '');
    const root = selectedDir || defaultDir();
    let descriptor;
    try {
      descriptor = await loadDescriptor(root);
    } catch (error) {
      if (stale(gen)) return emptyNonReady('unavailable', pinnedPeer || '');
      return emptyNonReady(error && error.code === 'invalid' ? 'invalid' : 'unavailable', pinnedPeer || '');
    }
    if (stale(gen)) return emptyNonReady('unavailable', pinnedPeer || '');
    if (pinnedPeer && descriptor.peer_id !== pinnedPeer) {
      return emptyNonReady('identity_changed', pinnedPeer);
    }
    const controller = new AbortController();
    controllers.add(controller);
    let buffer;
    try {
      buffer = await httpFetch(descriptor, controller.signal);
    } catch (error) {
      controllers.delete(controller);
      if (stale(gen)) return emptyNonReady('unavailable', pinnedPeer || '');
      if (error && error.code === 'aborted') return emptyNonReady('unavailable', pinnedPeer || '');
      const mapped = mapFetchError(error && error.code);
      if (mapped === 'too_large') return emptyNonReady('too_large', pinnedPeer || descriptor.peer_id);
      return emptyNonReady(mapped, pinnedPeer || '');
    }
    controllers.delete(controller);
    if (stale(gen)) return emptyNonReady('unavailable', pinnedPeer || '');
    try {
      const rows = decodeSnapshot(buffer, descriptor, now(), descriptor.peer_id);
      if (stale(gen)) return emptyNonReady('unavailable', pinnedPeer || '');
      pinnedPeer = descriptor.peer_id;
      return closedReply('ready', pinnedPeer, rows);
    } catch (error) {
      if (stale(gen)) return emptyNonReady('unavailable', pinnedPeer || '');
      if (error && error.code === 'identity_changed') {
        return emptyNonReady('identity_changed', pinnedPeer || descriptor.peer_id);
      }
      return emptyNonReady(error && error.code === 'too_large' ? 'too_large' : 'invalid', pinnedPeer || '');
    }
  }

  function getInbox() {
    if (disposed) return Promise.resolve(emptyNonReady('unavailable', ''));
    if (pendingGet) return pendingGet;
    const gen = generation;
    const pending = runGet(gen).finally(() => {
      if (pendingGet === pending) pendingGet = null;
    });
    pendingGet = pending;
    return pending;
  }

  async function connectInbox() {
    if (disposed) return emptyNonReady('unavailable', pinnedPeer || '');
    if (platform !== 'linux') return emptyNonReady('unsupported', '');
    if (pickerBusy) return getInbox();
    if (!dialog || typeof dialog.showOpenDialog !== 'function') return getInbox();
    pickerBusy = true;
    let chosen = null;
    let canceled = false;
    let failed = false;
    try {
      let result;
      try {
        result = await dialog.showOpenDialog(parentWindow() || undefined, {
          title: 'Select BitBook data folder',
          properties: ['openDirectory'],
        });
      } catch (_) {
        failed = true;
        return emptyNonReady('unavailable', pinnedPeer || '');
      }
      if (disposed) return emptyNonReady('unavailable', pinnedPeer || '');
      if (!result || result.canceled || !Array.isArray(result.filePaths) || result.filePaths.length !== 1) {
        canceled = true;
      } else if (typeof result.filePaths[0] === 'string' && result.filePaths[0].length > 0) {
        chosen = result.filePaths[0];
      } else canceled = true;
    } finally {
      pickerBusy = false;
    }
    if (failed) return emptyNonReady('unavailable', pinnedPeer || '');
    if (disposed) return emptyNonReady('unavailable', pinnedPeer || '');
    if (canceled || !chosen) return getInbox();
    generation += 1;
    abortAll();
    pendingGet = null;
    selectedDir = chosen;
    pinnedPeer = null;
    return getInbox();
  }

  function dispose() {
    disposed = true;
    generation += 1;
    abortAll();
    pendingGet = null;
  }

  return {
    getInbox,
    connectInbox,
    dispose,
  };
}

module.exports = {
  createPaymentInboxClient,
  parseDescriptor,
  formatAtomic,
};
