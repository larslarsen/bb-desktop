'use strict';

const crypto = require('crypto');
const fs = require('fs');
const childProcess = require('child_process');
const {
  computeSessionId,
  createProtocolSession,
  validateHello,
  validateHelloAck,
} = require('./protocol');

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

function readData(value, key) {
  if (value === null || typeof value !== 'object') return undefined;
  const descriptor = Object.getOwnPropertyDescriptor(value, key);
  return descriptor && Object.prototype.hasOwnProperty.call(descriptor, 'value')
    ? descriptor.value : undefined;
}

function isPlainObject(value) {
  return value !== null && typeof value === 'object' && !Array.isArray(value) &&
    Object.getPrototypeOf(value) === Object.prototype;
}

function safeCapabilities(value) {
  const descriptors = isPlainObject(value) ? Object.getOwnPropertyDescriptors(value) : null;
  const result = {};
  if (!descriptors) return result;
  for (const [key, descriptor] of Object.entries(descriptors)) {
    if (Object.prototype.hasOwnProperty.call(descriptor, 'value') &&
        /^[a-z][a-z0-9_]*$/.test(key) && typeof descriptor.value === 'boolean') result[key] = descriptor.value;
  }
  return result;
}

function sanitizeAccount(value) {
  if (!isPlainObject(value)) return null;
  const accountId = readData(value, 'account_id');
  if (typeof accountId !== 'string' || !ID.test(accountId)) return null;
  const result = { account_id: accountId };
  for (const key of ['label', 'asset', 'network', 'kind', 'privacy', 'balance_atomic']) {
    const field = readData(value, key);
    if (typeof field === 'string') result[key] = field;
  }
  result.capabilities = safeCapabilities(readData(value, 'capabilities'));
  const sync = readData(value, 'sync');
  if (isPlainObject(sync)) {
    result.sync = {};
    if (typeof readData(sync, 'state') === 'string') result.sync.state = readData(sync, 'state');
    if (typeof readData(sync, 'progress') === 'number' && Number.isFinite(readData(sync, 'progress'))) {
      result.sync.progress = readData(sync, 'progress');
    }
  }
  const device = readData(value, 'device');
  if (isPlainObject(device)) {
    result.device = {};
    if (typeof readData(device, 'present') === 'boolean') result.device.present = readData(device, 'present');
    if (typeof readData(device, 'label') === 'string') result.device.label = readData(device, 'label');
    const verified = readData(device, 'verified_fields');
    if (Array.isArray(verified) && verified.every((item) => typeof item === 'string')) {
      result.device.verified_fields = verified.slice();
    }
  }
  return result;
}

function sanitizeSnapshot(value) {
  const brokerValues = ['down', 'locked', 'ready', 'syncing', 'degraded'];
  const broker = brokerValues.includes(readData(value, 'broker')) ? readData(value, 'broker') : 'down';
  const sourceAccounts = readData(value, 'accounts');
  const accounts = [];
  if (Array.isArray(sourceAccounts)) {
    for (const source of sourceAccounts.slice(0, 256)) {
      const account = sanitizeAccount(source);
      if (account) accounts.push(account);
    }
  }
  return { v: 1, broker, accounts };
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
  let child = null;
  let timer = null;
  let protocolSession = null;
  let sessionId = null;
  let nextId = 1;
  let snapshot = sanitizeSnapshot({ v: 1, broker: 'down', accounts: [] });
  let failed = false;

  function publish(value) {
    snapshot = sanitizeSnapshot(value);
    for (const callback of [...subscribers]) {
      try { callback(sanitizeSnapshot(snapshot)); } catch (_) { /* subscriber isolation */ }
    }
    return sanitizeSnapshot(snapshot);
  }

  function terminate() {
    if (!child) return;
    if (typeof child.terminate === 'function') child.terminate();
    else if (typeof child.kill === 'function') child.kill('SIGTERM');
  }

  function close() {
    failed = true;
    protocolSession = null;
    sessionId = null;
    if (timer) system.clearTimeout(timer);
    timer = null;
    terminate();
    return publish({ v: 1, broker: 'down', accounts: [] });
  }

  function send(method, params) {
    const id = (nextId++).toString(16).padStart(32, '0');
    const envelope = {
      v: 1, id, seq: nextId - 1, kind: 'req', method, params,
      session: sessionId, expires_ms: system.now() + 2000,
    };
    protocolSession.accept('parent', envelope);
    child.stdin.write(envelope);
    return { ok: true };
  }

  const dispatch = createBrokerDispatcher({
    bound: () => Boolean(protocolSession && protocolSession.bound.child && !failed),
    send,
  });

  function dispatchFromMain(method, params) {
    if (params === undefined && (method === 'status.get' || method === 'account.list')) {
      return dispatch(method, {});
    }
    return dispatch(method, params);
  }

  const supervisor = {
    get bound() {
      return Boolean(protocolSession && protocolSession.bound.parent && protocolSession.bound.child && !failed);
    },
    get sessionId() { return sessionId; },

    start() {
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
        if (typeof system.spawn === 'function') {
          child = system.spawn(options.brokerPath, [], {
            cwd: options.dataDir,
            env: cleanEnv,
            shell: false,
            stdio: ['pipe', 'pipe', 'pipe'],
          });
        } else {
          child = childProcess.spawn(options.brokerPath, [], {
            cwd: options.dataDir,
            env: cleanEnv,
            shell: false,
            stdio: ['pipe', 'pipe', 'pipe'],
          });
        }
        protocolSession = null;
        sessionId = null;
        nextId = 1;
        failed = false;
        timer = system.setTimeout(() => close(), 2000);
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
      if (!child || failed) return { ok: false, snapshot: sanitizeSnapshot(snapshot) };
      try {
        if (!sessionId) {
          const hello = validateHello(value).value;
          if (hello.child_pid !== String(child.pid)) reject('UNAUTH', 'child PID mismatch');
          const ack = {
            protocol: 'bitbook-wallet-broker', version: 1,
            parent_nonce: options.nonce(), parent_pid: String(options.parentPid),
          };
          validateHelloAck(ack);
          sessionId = computeSessionId({
            parent_pid: ack.parent_pid, child_pid: hello.child_pid,
            parent_nonce: ack.parent_nonce, child_nonce: hello.child_nonce,
          });
          protocolSession = createProtocolSession({ sessionId, now: system.now });
          child.stdin.write(ack);
          return { ok: true };
        }
        protocolSession.accept('child', value);
        if (timer) system.clearTimeout(timer);
        timer = null;
        return { ok: true };
      } catch (_) {
        return { ok: false, snapshot: close() };
      }
    },

    dispatch: dispatchFromMain,
    trackIntent(intentId) {
      if (typeof intentId !== 'string' || !ID.test(intentId)) reject('SCHEMA', 'invalid intent id');
      intents.add(intentId);
    },
    pendingRequests() { return []; },
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
    quit() {
      if (supervisor.bound) {
        for (const intentId of intents) dispatch('intent.cancel', { intent_id: intentId });
      }
      terminate();
      intents.clear();
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
