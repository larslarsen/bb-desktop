'use strict';

const crypto = require('crypto');
const fs = require('fs');
const childProcess = require('child_process');
const { sanitizeWalletSnapshot } = require('../wallet-pay/model');
const {
  CONTROL_FRAME_LIMIT,
  computeSessionId,
  createBrokerFrameDecoder,
  createProtocolSession,
  encodeBrokerFrame,
  normalizeBrokerError,
  validateHello,
  validateHelloAck,
} = require('./protocol');

const PUBLIC_REQUEST_LIMIT = 32;
const DEADLINE_MS = 2000;
const KILL_ESCALATE_MS = 250;
const SHUTDOWN_MS = 1500;

const BROKER_METHODS = Object.freeze([
  'status.get',
  'account.list',
  'account.lock',
  'receiver.fresh',
  'intent.begin',
  'intent.cancel',
  'sync.subscribe',
]);
const ID = /^[0-9a-f]{32}$/;
const PIN = /^[0-9a-f]{64}$/;
const SAFE_ENV = Object.freeze(['LANG', 'PATH']);

class SupervisorError extends Error {
  constructor(code, message) {
    super(message || code);
    this.name = 'SupervisorError';
    this.code = code;
  }
}

function reject(code, message) {
  throw new SupervisorError(code, message);
}

function ownDataDescriptors(value) {
  if (value === null || typeof value !== 'object' || Array.isArray(value) ||
      Object.getPrototypeOf(value) !== Object.prototype) return null;
  const descriptors = Object.getOwnPropertyDescriptors(value);
  if (Object.values(descriptors).some(
    (descriptor) => !Object.prototype.hasOwnProperty.call(descriptor, 'value')
  )) return null;
  return descriptors;
}

function exactDataObject(value, keys) {
  const descriptors = ownDataDescriptors(value);
  if (!descriptors || Object.keys(descriptors).length !== keys.length ||
      keys.some((key) => !Object.prototype.hasOwnProperty.call(descriptors, key))) return null;
  return descriptors;
}

function isSafeTree(value, seen = new Set()) {
  if (value === null || ['string', 'boolean'].includes(typeof value)) return true;
  if (typeof value === 'number') return Number.isFinite(value);
  if (typeof value !== 'object' || seen.has(value)) return false;
  seen.add(value);
  const descriptors = Object.getOwnPropertyDescriptors(value);
  if (Object.values(descriptors).some(
    (descriptor) => !Object.prototype.hasOwnProperty.call(descriptor, 'value')
  )) return false;
  if (Array.isArray(value)) {
    if (Object.getPrototypeOf(value) !== Array.prototype) return false;
  } else if (Object.getPrototypeOf(value) !== Object.prototype) {
    return false;
  }
  return Object.values(descriptors).every((descriptor) => isSafeTree(descriptor.value, seen));
}

function cloneSafe(value) {
  if (value === null || typeof value !== 'object') return value;
  if (Array.isArray(value)) return value.map(cloneSafe);
  const result = {};
  for (const [key, descriptor] of Object.entries(Object.getOwnPropertyDescriptors(value))) {
    result[key] = cloneSafe(descriptor.value);
  }
  return result;
}

function validateParams(method, params) {
  let descriptors;
  if (['status.get', 'account.list', 'sync.subscribe'].includes(method)) {
    descriptors = exactDataObject(params, []);
  } else if (method === 'account.lock') {
    descriptors = exactDataObject(params, ['account_id']);
    if (!descriptors || typeof descriptors.account_id.value !== 'string' ||
        !ID.test(descriptors.account_id.value)) descriptors = null;
  } else if (method === 'receiver.fresh') {
    descriptors = exactDataObject(params, ['account_id', 'asset', 'network', 'request_id']);
    if (descriptors) {
      const accountId = descriptors.account_id.value;
      const asset = descriptors.asset.value;
      const network = descriptors.network.value;
      const requestId = descriptors.request_id.value;
      const networks = asset === 'ZEC'
        ? ['zec-mainnet', 'zec-testnet', 'zec-regtest']
        : asset === 'XMR' ? ['xmr-mainnet', 'xmr-stagenet', 'xmr-testnet'] : [];
      if (typeof accountId !== 'string' || !ID.test(accountId) ||
          typeof requestId !== 'string' || !ID.test(requestId) ||
          !networks.includes(network)) descriptors = null;
    }
  } else if (method === 'intent.begin') {
    descriptors = exactDataObject(params, ['payment_request']);
    if (!descriptors || !ownDataDescriptors(descriptors.payment_request.value) ||
        !isSafeTree(descriptors.payment_request.value)) descriptors = null;
  } else if (method === 'intent.cancel') {
    descriptors = exactDataObject(params, ['intent_id']);
    if (!descriptors || typeof descriptors.intent_id.value !== 'string' ||
        !ID.test(descriptors.intent_id.value)) descriptors = null;
  }
  if (!descriptors) reject('SCHEMA', 'invalid broker parameters');
  const cloned = cloneSafe(params);
  if (Buffer.byteLength(JSON.stringify(cloned), 'utf8') > 64 * 1024) reject('LIMIT', 'parameters too large');
  return cloned;
}

function createBrokerDispatcher(options = {}) {
  if (typeof options.bound !== 'function' || typeof options.send !== 'function') reject('SCHEMA', 'invalid dispatcher');
  return function dispatch(method, params) {
    if (!options.bound()) reject('UNAUTH', 'broker session is unbound');
    if (!BROKER_METHODS.includes(method)) reject('SCHEMA', 'unknown broker method');
    return options.send(method, validateParams(method, params));
  };
}

const sanitizeSnapshot = sanitizeWalletSnapshot;

function eventSnapshot(value) {
  const envelope = exactDataObject(value, [
    'v', 'id', 'seq', 'kind', 'method', 'params', 'session',
  ]);
  if (!envelope || envelope.kind.value !== 'evt' ||
      envelope.method.value !== 'sync.subscribe') return undefined;
  const params = exactDataObject(envelope.params.value, ['snapshot']);
  return params ? params.snapshot.value : undefined;
}

function defaultSystem() {
  return {
    mkdir(directory, options) { return fs.mkdirSync(directory, options); },
    lstat(file) { return fs.lstatSync(file); },
    access(file, mode) { return fs.accessSync(file, mode); },
    sha256(file) {
      return crypto.createHash('sha256').update(fs.readFileSync(file)).digest('hex');
    },
    setTimeout(callback, milliseconds) { return setTimeout(callback, milliseconds); },
    clearTimeout(timer) { clearTimeout(timer); },
    now() { return Date.now(); },
  };
}

function cleanEnvironment(source) {
  const result = {};
  for (const name of SAFE_ENV) {
    const descriptor = source && Object.getOwnPropertyDescriptor(source, name);
    if (descriptor && Object.prototype.hasOwnProperty.call(descriptor, 'value') && typeof descriptor.value === 'string') {
      result[name] = descriptor.value;
    }
  }
  return result;
}

function createWalletSupervisor(options = {}) {
  const system = Object.assign(defaultSystem(), options.system || {});
  const subscribers = new Set();
  const intents = new Set();
  const pendingPublic = new Map();
  let child = null;
  let decoder = null;
  let handshakeTimer = null;
  let killTimer = null;
  let protocolSession = null;
  let sessionId = null;
  let bootstrapId = null;
  let nextId = 1;
  let parentSeq = 1;
  let snapshot = sanitizeSnapshot({ v: 1, broker: 'down', accounts: [] });
  let failed = false;
  let bootstrapped = false;
  let downPublished = false;
  let terminating = false;
  let childExited = false;
  let childClosed = false;
  let shutdownPromise = null;
  let shutdownResolve = null;
  let shutdownReject = null;
  let shutdownTimer = null;
  let shutdownSettled = false;

  function publish(value) {
    snapshot = sanitizeSnapshot(value);
    for (const callback of [...subscribers]) {
      try { callback(sanitizeSnapshot(snapshot)); } catch (_) { /* subscriber isolation */ }
    }
    return sanitizeSnapshot(snapshot);
  }

  function parentPid() {
    return String(options.parentPid == null ? process.pid : options.parentPid);
  }

  function parentNonce() {
    if (typeof options.nonce === 'function') return options.nonce();
    return crypto.randomBytes(16).toString('hex');
  }

  function makeError(code, wire) {
    const normalized = normalizeBrokerError(wire && typeof wire === 'object' ? wire : { code });
    const error = new SupervisorError(normalized.code, normalized.message);
    error.retryable = normalized.retryable;
    return error;
  }

  function publicResult(method, result) {
    if (method === 'status.get' || method === 'sync.subscribe') return sanitizeSnapshot(result);
    return cloneSafe(result);
  }

  function settle(entry, error, result) {
    if (!entry || entry.settled) return;
    entry.settled = true;
    if (entry.timer) {
      system.clearTimeout(entry.timer);
      entry.timer = null;
    }
    if (error) entry.reject(error);
    else entry.resolve(result);
  }

  function childIsGone() {
    return childExited || (child != null && (child.exitCode != null || child.signalCode != null));
  }

  function signalChild(signal) {
    if (!child) return;
    try {
      if (typeof child.kill === 'function') child.kill(signal);
      else if (signal === 'SIGTERM' && typeof child.terminate === 'function') child.terminate();
    } catch (_) { /* already gone */ }
  }

  function detachDataListeners() {
    if (!child) return;
    if (child.stdout) {
      child.stdout.removeListener('data', onStdoutData);
      child.stdout.removeListener('end', onStdoutEnd);
    }
    if (child.stderr) child.stderr.removeListener('data', onStderrData);
  }

  function completeShutdown(error) {
    if (shutdownSettled) return;
    shutdownSettled = true;
    if (shutdownTimer) {
      system.clearTimeout(shutdownTimer);
      shutdownTimer = null;
    }
    if (error) shutdownReject(error);
    else shutdownResolve(undefined);
  }

  function terminate() {
    if (!child || terminating || childIsGone()) return;
    terminating = true;
    signalChild('SIGTERM');
    if (childClosed) return;
    killTimer = system.setTimeout(() => {
      killTimer = null;
      if (!childIsGone()) signalChild('SIGKILL');
    }, KILL_ESCALATE_MS);
  }

  function close() {
    failed = true;
    bootstrapped = false;
    if (handshakeTimer) {
      system.clearTimeout(handshakeTimer);
      handshakeTimer = null;
    }
    const entries = [...pendingPublic.values()];
    pendingPublic.clear();
    bootstrapId = null;
    protocolSession = null;
    sessionId = null;
    decoder = null;
    detachDataListeners();
    for (const entry of entries) settle(entry, makeError('UNAVAILABLE'));
    let published = sanitizeSnapshot(snapshot);
    if (!downPublished) {
      downPublished = true;
      published = publish({ v: 1, broker: 'down', accounts: [] });
    }
    terminate();
    return published;
  }

  function quit() {
    if (bootstrapped && !failed) {
      for (const intentId of [...intents]) {
        try {
          const result = dispatch('intent.cancel', { intent_id: intentId });
          if (result && typeof result.then === 'function') result.catch(() => {});
        } catch (_) { /* still terminate */ }
      }
    }
    intents.clear();
    close();
  }

  function writeRaw(value) {
    const frame = encodeBrokerFrame(value);
    if (frame.readUInt32BE(0) > CONTROL_FRAME_LIMIT) reject('LIMIT', 'frame too large');
    if (!child || !child.stdin || failed) reject('UNAVAILABLE', 'broker unavailable');
    try {
      child.stdin.write(frame);
    } catch (_) {
      close();
      reject('UNAVAILABLE', 'broker unavailable');
    }
    if (failed) reject('UNAVAILABLE', 'broker unavailable');
  }

  function writeRequest(method, params, trackPublic) {
    if (!protocolSession || failed || !child || !child.stdin) reject('UNAVAILABLE', 'broker unavailable');
    if (trackPublic && pendingPublic.size >= PUBLIC_REQUEST_LIMIT) reject('LIMIT', 'limit exceeded');
    const id = nextId.toString(16).padStart(32, '0');
    const envelope = {
      v: 1, id, seq: parentSeq, kind: 'req', method, params,
      session: sessionId, expires_ms: system.now() + DEADLINE_MS,
    };
    const frame = encodeBrokerFrame(envelope);
    if (frame.readUInt32BE(0) > CONTROL_FRAME_LIMIT) reject('LIMIT', 'frame too large');
    protocolSession.accept('parent', envelope);
    nextId += 1;
    parentSeq += 1;
    try {
      child.stdin.write(frame);
    } catch (_) {
      close();
      reject('UNAVAILABLE', 'broker unavailable');
    }
    if (failed) reject('UNAVAILABLE', 'broker unavailable');
    if (!trackPublic) {
      bootstrapId = id;
      return undefined;
    }
    const entry = { method, settled: false, timer: null, resolve: null, reject: null };
    const promise = new Promise((resolve, rejectFn) => {
      entry.resolve = resolve;
      entry.reject = rejectFn;
    });
    entry.timer = system.setTimeout(() => {
      if (entry.settled) return;
      pendingPublic.delete(id);
      settle(entry, makeError('TIMEOUT'));
      close();
    }, DEADLINE_MS);
    pendingPublic.set(id, entry);
    return promise;
  }

  const dispatch = createBrokerDispatcher({
    bound: () => Boolean(bootstrapped && !failed),
    send: (method, params) => writeRequest(method, params, true),
  });

  function dispatchFromMain(method, params) {
    if (params === undefined && (method === 'status.get' || method === 'account.list')) {
      return dispatch(method, {});
    }
    return dispatch(method, params);
  }

  function finishBootstrap(value) {
    bootstrapId = null;
    if (value.kind === 'error') {
      normalizeBrokerError(value.error);
      return { ok: false, snapshot: close() };
    }
    if (handshakeTimer) {
      system.clearTimeout(handshakeTimer);
      handshakeTimer = null;
    }
    bootstrapped = true;
    publish(value.result);
    return { ok: true };
  }

  function acceptHello(value) {
    const hello = validateHello(value).value;
    if (hello.child_pid !== String(child.pid)) reject('UNAUTH', 'child PID mismatch');
    const ack = {
      protocol: 'bitbook-wallet-broker',
      version: 1,
      parent_nonce: parentNonce(),
      parent_pid: parentPid(),
    };
    validateHelloAck(ack);
    sessionId = computeSessionId({
      parent_pid: ack.parent_pid,
      child_pid: hello.child_pid,
      parent_nonce: ack.parent_nonce,
      child_nonce: hello.child_nonce,
    });
    protocolSession = createProtocolSession({ sessionId, now: system.now });
    writeRaw(ack);
    writeRequest('status.get', {}, false);
    return { ok: true };
  }

  function acceptChild(value) {
    protocolSession.accept('child', value);
    if (value.kind === 'evt') {
      if (bootstrapped) {
        const receivedSnapshot = eventSnapshot(value);
        if (receivedSnapshot !== undefined) publish(receivedSnapshot);
      }
      return { ok: true };
    }
    if (value.kind !== 'res' && value.kind !== 'error') return { ok: true };
    if (!bootstrapped && bootstrapId && value.id === bootstrapId) return finishBootstrap(value);
    const entry = pendingPublic.get(value.id);
    if (!entry) return { ok: false, snapshot: close() };
    pendingPublic.delete(value.id);
    if (value.kind === 'error') {
      settle(entry, makeError(value.error && value.error.code, value.error));
      return { ok: true };
    }
    settle(entry, null, publicResult(entry.method, value.result));
    return { ok: true };
  }

  function handleIncoming(value) {
    if (!child || failed) return { ok: false, snapshot: sanitizeSnapshot(snapshot) };
    try {
      if (!sessionId) return acceptHello(value);
      return acceptChild(value);
    } catch (_) {
      return { ok: false, snapshot: close() };
    }
  }

  function onStdoutData(chunk) {
    if (failed || !decoder) return;
    try {
      if (!Buffer.isBuffer(chunk)) {
        close();
        return;
      }
      const values = decoder.push(chunk);
      for (let index = 0; index < values.length; index += 1) {
        if (failed) return;
        handleIncoming(values[index]);
      }
    } catch (_) {
      close();
    }
  }

  function onStdoutEnd() {
    close();
  }

  function onStderrData() {
    // Drain diagnostics without retaining or forwarding them.
  }

  function onTransportError() {
    close();
  }

  function onChildExit() {
    childExited = true;
    if (killTimer) {
      system.clearTimeout(killTimer);
      killTimer = null;
    }
    close();
  }

  function onChildClose() {
    childClosed = true;
    childExited = true;
    if (killTimer) {
      system.clearTimeout(killTimer);
      killTimer = null;
    }
    if (shutdownTimer) {
      system.clearTimeout(shutdownTimer);
      shutdownTimer = null;
    }
    close();
    if (shutdownPromise && !shutdownSettled) {
      Promise.resolve().then(() => completeShutdown());
    }
  }

  function attachTransport(target) {
    decoder = createBrokerFrameDecoder({ limitBytes: CONTROL_FRAME_LIMIT, stream: 'protocol' });
    target.stdout.on('data', onStdoutData);
    target.stdout.on('end', onStdoutEnd);
    target.stdout.on('error', onTransportError);
    target.stdin.on('error', onTransportError);
    target.stderr.on('data', onStderrData);
    target.stderr.on('error', onTransportError);
    target.on('error', onTransportError);
    target.on('exit', onChildExit);
    target.on('close', onChildClose);
  }

  const supervisor = {
    get bound() {
      return Boolean(bootstrapped && protocolSession && !failed);
    },
    get sessionId() { return sessionId; },

    start() {
      if (child || failed) return { ok: false, snapshot: sanitizeSnapshot(snapshot) };
      if (!options.brokerPath || typeof options.expectedSha256 !== 'string' ||
          !options.expectedSha256 || !options.dataDir || !PIN.test(options.expectedSha256)) {
        return { ok: false, snapshot: publish({ v: 1, broker: 'down', accounts: [] }) };
      }
      try {
        system.mkdir(options.dataDir, { recursive: true, mode: 0o700 });
        const dataStat = system.lstat(options.dataDir);
        if (!dataStat || dataStat.isSymbolicLink() || !dataStat.isDirectory() || (dataStat.mode & 0o777) !== 0o700) {
          reject('UNAVAILABLE', 'invalid broker data directory');
        }
        const brokerStat = system.lstat(options.brokerPath);
        if (!brokerStat || brokerStat.isSymbolicLink() || !brokerStat.isFile()) reject('UNAVAILABLE', 'invalid broker binary');
        system.access(options.brokerPath, fs.constants.R_OK);
        if (system.sha256(options.brokerPath) !== options.expectedSha256) reject('UNAUTH', 'broker hash mismatch');
        const cleanEnv = cleanEnvironment(options.env || {});
        const spawn = typeof system.spawn === 'function' ? system.spawn : childProcess.spawn;
        child = spawn(options.brokerPath, [], {
          cwd: options.dataDir,
          env: cleanEnv,
          shell: false,
          stdio: ['pipe', 'pipe', 'pipe'],
        });
        decoder = null;
        protocolSession = null;
        sessionId = null;
        bootstrapId = null;
        nextId = 1;
        parentSeq = 1;
        failed = false;
        bootstrapped = false;
        downPublished = false;
        terminating = false;
        childExited = false;
        childClosed = false;
        pendingPublic.clear();
        attachTransport(child);
        handshakeTimer = system.setTimeout(() => close(), DEADLINE_MS);
        return { ok: true, snapshot: sanitizeSnapshot(snapshot) };
      } catch (_) {
        return { ok: false, snapshot: close() };
      }
    },

    receiveDiagnostic(value) {
      if (Buffer.isBuffer(value) || typeof value === 'string') return undefined;
      return { ok: false, snapshot: close() };
    },

    receiveProtocol(value) {
      return handleIncoming(value);
    },

    dispatch: dispatchFromMain,
    trackIntent(intentId) {
      if (typeof intentId !== 'string' || !ID.test(intentId)) reject('SCHEMA', 'invalid intent id');
      intents.add(intentId);
    },
    pendingRequests() { return Array.from(pendingPublic.keys()); },
    restartDelays(count) {
      return Array.from({ length: Math.max(0, count) }, (_, index) => Math.min(250 * (2 ** index), 5000));
    },
    unexpectedExit() { return close(); },
    subscribeSnapshot(callback) {
      if (typeof callback !== 'function') throw new TypeError('snapshot callback must be a function');
      subscribers.add(callback);
      let active = true;
      return () => {
        if (!active) return false;
        active = false;
        subscribers.delete(callback);
        return true;
      };
    },
    quit,
    shutdown() {
      if (shutdownPromise) return shutdownPromise;
      shutdownPromise = new Promise((resolve, reject) => {
        shutdownResolve = resolve;
        shutdownReject = reject;
      });
      if (!child || childClosed) {
        completeShutdown();
      } else {
        shutdownTimer = system.setTimeout(() => {
          shutdownTimer = null;
          completeShutdown(makeError('TIMEOUT'));
        }, SHUTDOWN_MS);
      }
      quit();
      return shutdownPromise;
    },
  };
  return supervisor;
}

module.exports = {
  BROKER_METHODS,
  createBrokerDispatcher,
  createWalletSupervisor,
  sanitizeSnapshot,
};
