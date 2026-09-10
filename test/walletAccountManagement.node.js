'use strict';

const assert = require('assert');
const Module = require('module');
const path = require('path');
const { EventEmitter } = require('events');
const {
  BROKER_METHODS,
  createBrokerDispatcher,
  createWalletSupervisor,
} = require('../wallet-broker/supervisor');

const repoRoot = path.resolve(__dirname, '..');
const mainPath = path.join(repoRoot, 'social-main.js');
const expectedPage = path.join(repoRoot, 'social', 'index.html');
const expectedPreload = path.join(repoRoot, 'wallet-preload.js');
const USER_DATA = path.resolve('/bb-wal013-user-data');
const PIN = 'ab'.repeat(32);
const BROKER_PATH = path.resolve('/bb-wal013-broker', 'bitbook-wallet-broker');
const DATA_DIR = path.join(USER_DATA, 'wallet-broker');
const CANARY = 'WAL013_ACCOUNT_CANARY_SECRET';
const CANARY_PATH = '/var/lib/bitbook/private/wal013';
const ACCOUNT_DIALOG_TITLE = 'Accounts unavailable';
const ACCOUNT_DIALOG_MESSAGE = 'The account window could not be opened. Please restart BitBook.';
const ALLOWED_ENV = Object.freeze([
  'LANG',
  'PATH',
  'DISPLAY',
  'WAYLAND_DISPLAY',
  'XDG_RUNTIME_DIR',
  'XAUTHORITY',
  'DBUS_SESSION_BUS_ADDRESS',
]);
const PRELOAD_METHODS = Object.freeze([
  'beginIntent',
  'cancelIntent',
  'getPayeeRequest',
  'getSnapshot',
  'listAccounts',
  'subscribeSnapshot',
]);
const IPC_CHANNELS = Object.freeze([
  'wallet:accounts:list',
  'wallet:intent:begin',
  'wallet:intent:cancel',
  'wallet:payee-request:get',
  'wallet:snapshot:get',
]);
const SECRET_METHODS = Object.freeze([
  'account.unlock',
  'account.createSoftware',
  'account.exportBackup',
  'account.restore',
  'account.prepareRestore',
  'account.confirmRestore',
]);
const TRANSCRIPT = require('./fixtures/wallet-broker/transcript-v1.json');
const BOOTSTRAP_ID = '00000000000000000000000000000001';
const BOOTSTRAP_SNAPSHOT = { v: 1, broker: 'degraded', accounts: [] };
const INTENT_ID = 'aabbccddeeff00112233445566778899';
const FAKE_PIN = 'a'.repeat(64);
const FAKE_BROKER = '/app/resources/bitbook-wallet-broker';
const FAKE_DATA = '/user-data/wallet-broker';
const KILL_MS = 250;
const GRACE_MS = 1000;
const SIGKILL_AT_MS = 1250;
const SHUTDOWN_MS = 1500;
const SETTLEMENT_MS = 1000;

const tests = [];
function test(name, fn) { tests.push({ name, fn }); }

function createDeferred() {
  let resolveFn;
  let rejectFn;
  const deferred = {
    settled: false,
    promise: null,
    resolve(value) {
      if (deferred.settled) return;
      deferred.settled = true;
      resolveFn(value);
    },
    reject(reason) {
      if (deferred.settled) return;
      deferred.settled = true;
      rejectFn(reason);
    },
  };
  deferred.promise = new Promise((resolve, reject) => {
    resolveFn = resolve;
    rejectFn = reject;
  });
  deferred.promise.then(() => {}, () => {});
  return deferred;
}

function waitTurn() {
  return new Promise((resolve) => { setImmediate(resolve); });
}

function withTimeout(promise, message) {
  let timer = null;
  const timeout = new Promise((_, reject) => {
    timer = setTimeout(() => reject(new Error(message)), SETTLEMENT_MS);
  });
  return Promise.race([Promise.resolve(promise), timeout]).finally(() => {
    if (timer !== null) clearTimeout(timer);
  });
}

function watchUnhandled() {
  const seen = [];
  function onUnhandled(reason) { seen.push(reason); }
  process.on('unhandledRejection', onUnhandled);
  return {
    seen,
    restore() { process.removeListener('unhandledRejection', onUnhandled); },
  };
}

function watch(promise) {
  const watched = { state: 'pending', value: undefined, error: undefined };
  watched.promise = Promise.resolve(promise).then(
    (value) => {
      watched.state = 'resolved';
      watched.value = value;
      return value;
    },
    (error) => {
      watched.state = 'rejected';
      watched.error = error;
      throw error;
    }
  );
  watched.promise.catch(() => {});
  return watched;
}

function flush() {
  return Promise.resolve().then(() => Promise.resolve());
}

function decodeFrames(chunks) {
  const buffers = chunks.map((chunk) => {
    if (!Buffer.isBuffer(chunk)) throw new Error('expected framed Buffer writes');
    return chunk;
  });
  const bytes = Buffer.concat(buffers);
  const values = [];
  let offset = 0;
  while (offset + 4 <= bytes.length) {
    const length = bytes.readUInt32BE(offset);
    if (offset + 4 + length > bytes.length) break;
    values.push(JSON.parse(bytes.subarray(offset + 4, offset + 4 + length).toString('utf8')));
    offset += 4 + length;
  }
  return { values, unread: bytes.length - offset };
}

function decodeWriteList(writes) {
  return writes.map((write) => {
    assert.ok(Buffer.isBuffer(write), 'stdin write must be a Buffer');
    const decoded = decodeFrames([write]);
    assert.strictEqual(decoded.values.length, 1);
    assert.strictEqual(decoded.unread, 0);
    return decoded.values[0];
  });
}

function menuItems(menu) {
  if (!menu) return [];
  if (Array.isArray(menu)) return menu;
  if (Array.isArray(menu.items)) return menu.items;
  if (Array.isArray(menu.template)) return menu.template;
  return [];
}

function findMenuItem(menu, label) {
  const stack = menuItems(menu).slice();
  while (stack.length) {
    const item = stack.shift();
    if (!item || typeof item !== 'object') continue;
    if (item.label === label) return item;
    const nested = menuItems(item.submenu);
    if (nested.length) stack.push(...nested);
  }
  return null;
}

function invokeClick(item) {
  assert.ok(item && typeof item.click === 'function', 'Manage accounts click handler is missing');
  return item.click();
}

function assertAllowlistedEnvCopy(env, source, label) {
  assert.ok(env && typeof env === 'object' && !Array.isArray(env), `${label} env missing`);
  if (source) assert.notStrictEqual(env, source, `${label} passed process.env identity`);
  const descriptors = Object.getOwnPropertyDescriptors(env);
  for (const key of Object.keys(descriptors)) {
    assert.ok(ALLOWED_ENV.includes(key), `${label} unexpected env ${key}`);
    const descriptor = descriptors[key];
    assert.ok(
      Object.prototype.hasOwnProperty.call(descriptor, 'value'),
      `${label} env accessor ${key}`
    );
    assert.strictEqual(typeof descriptor.value, 'string', `${label} nonstring env ${key}`);
    assert.ok(!descriptor.value.includes('\0'), `${label} embedded NUL in ${key}`);
    assert.ok(
      Buffer.byteLength(descriptor.value, 'utf8') <= 4096,
      `${label} oversized env ${key}`
    );
  }
  const serialized = JSON.stringify(env);
  assert.ok(!serialized.includes(CANARY), `${label} leaked canary`);
  assert.ok(!serialized.includes(CANARY_PATH), `${label} leaked path`);
}

function assertFixedAccountDialog(box, label) {
  assert.ok(box, `${label} dialog missing`);
  assert.strictEqual(box.title, ACCOUNT_DIALOG_TITLE, `${label} dialog title`);
  assert.strictEqual(box.content, ACCOUNT_DIALOG_MESSAGE, `${label} dialog message`);
  const payload = JSON.stringify(box);
  assert.ok(!payload.includes(CANARY), `${label} dialog leaked canary`);
  assert.ok(!payload.includes(CANARY_PATH), `${label} dialog leaked path`);
  assert.ok(!payload.includes('Error:'), `${label} dialog leaked raw error`);
}

function assertSixMethodPreload() {
  let exposed;
  const electron = {
    contextBridge: {
      exposeInMainWorld(name, value) {
        assert.strictEqual(name, 'bitbookWallet');
        exposed = value;
      },
    },
    ipcRenderer: {
      invoke() { return Promise.resolve({ ok: true }); },
      on() {},
      removeListener() {},
    },
  };
  const originalLoad = Module._load;
  Module._load = function loadPreload(request, parent, isMain) {
    if (request === 'electron') return electron;
    return originalLoad.call(this, request, parent, isMain);
  };
  try {
    delete require.cache[expectedPreload];
    require(expectedPreload);
  } finally {
    Module._load = originalLoad;
    delete require.cache[expectedPreload];
  }
  assert.deepStrictEqual(Object.keys(exposed).sort(), PRELOAD_METHODS.slice().sort());
  assert.strictEqual(Object.isFrozen(exposed), true);
  for (const method of PRELOAD_METHODS) {
    assert.strictEqual(typeof exposed[method], 'function');
    assert.strictEqual(Object.isFrozen(exposed[method]), true);
  }
  for (const forbidden of ['manageAccounts', 'unlock', 'exportBackup', 'createSoftware', 'restore']) {
    assert.strictEqual(exposed[forbidden], undefined);
  }
}

function launchFromResolver() {
  return Object.freeze({
    brokerPath: BROKER_PATH,
    expectedSha256: PIN,
    dataDir: DATA_DIR,
  });
}

function createMainHarness(options = {}) {
  const state = {
    enableSandboxCalls: 0,
    windows: [],
    appHandlers: Object.create(null),
    ipcHandlers: Object.create(null),
    menuSet: [],
    menuTemplates: [],
    permissionRequestHandler: null,
    permissionCheckHandler: null,
    errorBoxes: [],
    rendererMessages: [],
    resolverCalls: [],
    factoryCalls: [],
    startCalls: [],
    dispatchCalls: [],
    subscribers: [],
    shutdownCalls: [],
    supervisors: [],
    quitCalls: 0,
    beforeQuitEvents: [],
    getPathCalls: [],
  };

  class WebContents {
    constructor() {
      this.handlers = Object.create(null);
      this.mainFrame = { url: `file://${expectedPage}` };
      this.getURL = () => `file://${expectedPage}`;
      this.send = (channel, payload) => {
        state.rendererMessages.push([this.mainFrame, channel, payload]);
      };
    }

    on(event, handler) {
      if (!this.handlers[event]) this.handlers[event] = [];
      this.handlers[event].push(handler);
    }

    setWindowOpenHandler(handler) { this.windowOpenHandler = handler; }
  }

  class BrowserWindow {
    constructor(windowOptions) {
      this.options = windowOptions;
      this.webContents = new WebContents();
      this.loadedFiles = [];
      this._closedHandlers = [];
      state.windows.push(this);
    }

    loadFile(filePath) { this.loadedFiles.push(filePath); }

    on(event, handler) {
      if (event === 'closed') this._closedHandlers.push(handler);
    }
  }

  function emitRegisteredBeforeQuit() {
    const handlers = state.appHandlers['before-quit'] || [];
    const event = {
      defaultPrevented: false,
      preventDefault() { this.defaultPrevented = true; },
    };
    const record = { defaultPrevented: false };
    state.beforeQuitEvents.push(record);
    for (const handler of handlers) handler(event);
    record.defaultPrevented = event.defaultPrevented;
    return event;
  }

  const app = {
    enableSandbox() { state.enableSandboxCalls += 1; },
    getPath(name) {
      state.getPathCalls.push(name);
      if (name !== 'userData') throw new Error(`unexpected app.getPath(${String(name)})`);
      return options.userDataPath || USER_DATA;
    },
    isPackaged: false,
    on(event, handler) {
      if (!state.appHandlers[event]) state.appHandlers[event] = [];
      state.appHandlers[event].push(handler);
    },
    quit() {
      state.quitCalls += 1;
      emitRegisteredBeforeQuit();
    },
  };

  const electron = {
    app,
    BrowserWindow,
    Menu: {
      buildFromTemplate(template) {
        assert.ok(Array.isArray(template), 'Menu.buildFromTemplate requires an array');
        state.menuTemplates.push(template);
        return { items: template };
      },
      setApplicationMenu(menu) { state.menuSet.push(menu); },
    },
    ipcMain: {
      handle(channel, handler) {
        assert.ok(!state.ipcHandlers[channel], `duplicate ipc ${channel}`);
        state.ipcHandlers[channel] = handler;
      },
    },
    session: {
      defaultSession: {
        setPermissionRequestHandler(handler) { state.permissionRequestHandler = handler; },
        setPermissionCheckHandler(handler) { state.permissionCheckHandler = handler; },
      },
    },
    dialog: {
      showErrorBox(title, content) { state.errorBoxes.push({ title, content }); },
    },
  };

  function makeSupervisor(kind, factoryOptions) {
    const record = {
      kind,
      factoryOptions,
      boundFlag: options.bound === true,
    };
    const supervisor = {
      get bound() { return record.boundFlag; },
      set bound(value) { record.boundFlag = Boolean(value); },
      start() {
        state.startCalls.push({ kind, factoryOptions });
        return { ok: true, snapshot: { v: 1, broker: 'down', accounts: [] } };
      },
      subscribeSnapshot(callback) {
        state.subscribers.push({ kind, callback });
        return () => true;
      },
      dispatch(method, params) {
        state.dispatchCalls.push([method, params]);
        if (typeof options.dispatch === 'function') return options.dispatch(method, params);
        if (options.dispatchError) throw options.dispatchError;
        return options.dispatchResult === undefined ? {} : options.dispatchResult;
      },
      shutdown() {
        state.shutdownCalls.push({ kind });
        if (typeof options.shutdown === 'function') return options.shutdown(kind);
        return Promise.resolve();
      },
    };
    record.supervisor = supervisor;
    state.supervisors.push(record);
    return supervisor;
  }

  return {
    electron,
    state,
    emitApp(event, ...args) {
      for (const handler of state.appHandlers[event] || []) handler(...args);
    },
    emitBeforeQuit: emitRegisteredBeforeQuit,
    resolveWalletBrokerLaunch(value) {
      state.resolverCalls.push(value);
      return options.launch || launchFromResolver();
    },
    createWalletSupervisor(factoryOptions) {
      state.factoryCalls.push(factoryOptions);
      const kind = state.factoryCalls.length === 1 ? 'initial' : 'configured';
      return makeSupervisor(kind, factoryOptions);
    },
    installedMenu() {
      if (state.menuTemplates.length) return state.menuTemplates[state.menuTemplates.length - 1];
      return state.menuSet[state.menuSet.length - 1];
    },
  };
}

async function withMain(options, fn) {
  const originalLoad = Module._load;
  const previousMain = require.cache[mainPath];
  const unhandled = watchUnhandled();
  const harness = createMainHarness(options);
  Module._load = function loadWithMocks(request, parent, isMain) {
    if (request === 'electron') return harness.electron;
    if (/(?:^|\/)wallet-broker\/launch-config$/.test(request)) {
      return { resolveWalletBrokerLaunch: harness.resolveWalletBrokerLaunch.bind(harness) };
    }
    if (/(?:^|\/)wallet-broker\/supervisor$/.test(request)) {
      return {
        createWalletSupervisor: harness.createWalletSupervisor.bind(harness),
        sanitizeSnapshot: (value) => value,
      };
    }
    return originalLoad.call(this, request, parent, isMain);
  };
  try {
    delete require.cache[mainPath];
    require(mainPath);
    if (options.emitReady !== false) harness.emitApp('ready');
    const result = await fn(harness);
    assert.strictEqual(unhandled.seen.length, 0, 'unhandled rejection during account management fixture');
    return result;
  } finally {
    Module._load = originalLoad;
    if (previousMain) require.cache[mainPath] = previousMain;
    else delete require.cache[mainPath];
    unhandled.restore();
  }
}

function createFakeClock() {
  let now = 0;
  const timers = [];
  const clock = {
    now() { return now; },
    setTimeout(fn, ms) {
      const timer = { fn, ms, due: now + ms, cleared: false, fired: false };
      timers.push(timer);
      return timer;
    },
    clearTimeout(timer) {
      if (timer) timer.cleared = true;
    },
    advance(ms) {
      now += ms;
      clock.flushDue();
    },
    flushDue() {
      let progress = true;
      while (progress) {
        progress = false;
        const due = timers
          .filter((timer) => timer.cleared !== true && timer.fired !== true && timer.due <= now)
          .sort((left, right) => left.due - right.due);
        for (const timer of due) {
          if (timer.cleared === true || timer.fired === true) continue;
          timer.fired = true;
          timer.fn();
          progress = true;
        }
      }
    },
    timers,
  };
  return clock;
}

function killSignals(ctx) {
  return ctx.calls.filter((call) => call[0] === 'kill').map((call) => call[1]);
}

function activeTimers(ctx, ms) {
  return ctx.timers.filter((timer) => timer.ms === ms && timer.cleared !== true && timer.fired !== true);
}

function createFakeHarness(config = {}) {
  const calls = [];
  const protocolWrites = [];
  const clock = createFakeClock();
  const child = new EventEmitter();
  child.pid = 41002;
  child.exitCode = null;
  child.signalCode = null;
  child.killed = false;
  child.stdin = new EventEmitter();
  child.stdin.write = (value) => {
    protocolWrites.push(value);
    calls.push(['protocol', value]);
    return true;
  };
  if (config.stdinEnd !== false) {
    child.stdin.end = function endStdin() {
      calls.push(['end']);
      if (typeof config.onEnd === 'function') return config.onEnd(child, calls);
      return undefined;
    };
  }
  child.stdout = new EventEmitter();
  child.stderr = new EventEmitter();
  child.kill = (signal) => {
    calls.push(['kill', signal || 'SIGTERM']);
    child.killed = true;
    return true;
  };
  const system = {
    mkdir(target, options) { calls.push(['mkdir', target, options]); },
    lstat(target) {
      calls.push(['lstat', target]);
      if (target === FAKE_DATA) {
        return { isDirectory: () => true, isFile: () => false, isSymbolicLink: () => false, mode: 0o700 };
      }
      return { isDirectory: () => false, isFile: () => true, isSymbolicLink: () => false, mode: 0o755 };
    },
    access(target, mode) { calls.push(['access', target, mode]); },
    sha256() { return FAKE_PIN; },
    spawn(target, argv, options) {
      calls.push(['spawn', target, argv, options]);
      return child;
    },
    setTimeout(fn, ms) {
      const timer = clock.setTimeout(fn, ms);
      calls.push(['timer', ms, timer]);
      return timer;
    },
    clearTimeout(timer) {
      calls.push(['clearTimeout', timer]);
      clock.clearTimeout(timer);
    },
    now() { return clock.now(); },
  };
  const supervisor = createWalletSupervisor({
    brokerPath: FAKE_BROKER,
    expectedSha256: FAKE_PIN,
    dataDir: FAKE_DATA,
    env: config.env || { PATH: '/usr/bin', LANG: 'C.UTF-8', SECRET_TOKEN: CANARY },
    parentPid: 41001,
    nonce: () => TRANSCRIPT.parent_nonce,
    system,
  });
  const trackedPromises = [];
  function observe(promise) {
    const watched = watch(promise);
    trackedPromises.push(watched);
    return watched;
  }
  async function teardown() {
    child.emit('close', 0, null);
    supervisor.quit();
    await flush();
    let teardownError = null;
    const pending = trackedPromises.filter((watched) => watched.state === 'pending');
    if (pending.length) {
      teardownError = new Error(`${pending.length} tracked fake promise(s) remained pending after teardown`);
    }
    const leftoverTimers = clock.timers.filter(
      (timer) => timer.cleared !== true && timer.fired !== true
    );
    if (leftoverTimers.length) {
      const timerError = new Error(
        `${leftoverTimers.length} unfired/uncleared fake timer(s) remained after teardown`
      );
      teardownError = teardownError
        ? new Error(`${teardownError.message}\n${timerError.message}`)
        : timerError;
    }
    if (teardownError) throw teardownError;
  }
  return {
    calls,
    child,
    protocolWrites,
    clock,
    supervisor,
    timers: clock.timers,
    advance(ms) { clock.advance(ms); },
    observe,
    teardown,
  };
}

async function withFakeHarness(config, fn) {
  const options = typeof config === 'function' ? {} : config;
  const body = typeof config === 'function' ? config : fn;
  const ctx = createFakeHarness(options);
  let originalError = null;
  let cleanupError = null;
  let result;
  try {
    result = await body(ctx);
  } catch (error) {
    originalError = error;
  } finally {
    try {
      await ctx.teardown();
    } catch (error) {
      cleanupError = error;
    }
  }
  if (originalError && cleanupError) {
    originalError.message = `${originalError.message}\ncleanup also failed: ${cleanupError.stack || cleanupError.message}`;
    throw originalError;
  }
  if (originalError) throw originalError;
  if (cleanupError) throw cleanupError;
  return result;
}

function bindFake(ctx, result) {
  const started = ctx.supervisor.start();
  assert.strictEqual(started.ok, true);
  assert.strictEqual(ctx.supervisor.receiveProtocol(TRANSCRIPT.hello).ok, true);
  const frames = decodeWriteList(ctx.protocolWrites);
  assert.deepStrictEqual(frames[0], TRANSCRIPT.hello_ack);
  const received = ctx.supervisor.receiveProtocol({
    v: 1,
    id: BOOTSTRAP_ID,
    seq: 1,
    kind: 'res',
    result: result || BOOTSTRAP_SNAPSHOT,
    session: TRANSCRIPT.session_id,
  });
  assert.strictEqual(received.ok, true);
  assert.strictEqual(ctx.supervisor.bound, true);
}

function assertNoCanary(value, label) {
  const payload = typeof value === 'string' ? value : JSON.stringify(value);
  assert.ok(!payload.includes(CANARY), `${label} leaked canary`);
  assert.ok(!payload.includes(CANARY_PATH), `${label} leaked path`);
}

test('ready menu registers Wallet/Manage accounts; bound click dispatches account.manage{}; failures stay closed', async () => {
  assertSixMethodPreload();

  await withMain({ bound: true }, async (harness) => {
    assert.deepStrictEqual(Object.keys(harness.state.ipcHandlers).sort(), IPC_CHANNELS.slice().sort());
    const wallet = findMenuItem(harness.installedMenu(), 'Wallet');
    const manage = findMenuItem(harness.installedMenu(), 'Manage accounts');
    assert.ok(wallet, 'Wallet menu is missing');
    assert.ok(manage, 'Manage accounts item is missing');
    const before = harness.state.dispatchCalls.length;
    const beforeDialogs = harness.state.errorBoxes.length;
    invokeClick(manage);
    await waitTurn();
    assert.deepStrictEqual(
      harness.state.dispatchCalls.slice(before),
      [['account.manage', {}]]
    );
    assert.strictEqual(harness.state.errorBoxes.length, beforeDialogs);
    assert.deepStrictEqual(Object.keys(harness.state.ipcHandlers).sort(), IPC_CHANNELS.slice().sort());
    assertSixMethodPreload();
  });

  await withMain({ bound: false }, async (harness) => {
    const manage = findMenuItem(harness.installedMenu(), 'Manage accounts');
    invokeClick(manage);
    await waitTurn();
    assert.deepStrictEqual(harness.state.dispatchCalls, []);
    assert.strictEqual(harness.state.errorBoxes.length, 1);
    assertFixedAccountDialog(harness.state.errorBoxes[0], 'unbound');
  });

  const syncError = new Error(`${CANARY} ${CANARY_PATH}`);
  syncError.code = 'UNAVAILABLE';
  await withMain({ bound: true, dispatchError: syncError }, async (harness) => {
    const manage = findMenuItem(harness.installedMenu(), 'Manage accounts');
    invokeClick(manage);
    await waitTurn();
    assert.deepStrictEqual(harness.state.dispatchCalls, [['account.manage', {}]]);
    assert.strictEqual(harness.state.errorBoxes.length, 1);
    assertFixedAccountDialog(harness.state.errorBoxes[0], 'sync rejection');
  });

  const asyncError = new Error(`${CANARY} stack ${CANARY_PATH}`);
  asyncError.code = 'UNAVAILABLE';
  await withMain({
    bound: true,
    dispatch() { return Promise.reject(asyncError); },
  }, async (harness) => {
    const manage = findMenuItem(harness.installedMenu(), 'Manage accounts');
    invokeClick(manage);
    await withTimeout(waitTurn(), 'async manage rejection did not settle');
    await waitTurn();
    assert.deepStrictEqual(harness.state.dispatchCalls, [['account.manage', {}]]);
    assert.strictEqual(harness.state.errorBoxes.length, 1);
    assertFixedAccountDialog(harness.state.errorBoxes[0], 'async rejection');
  });
});

test('quit and repeated ready cannot open the account window', async () => {
  const deferred = createDeferred();
  await withMain({
    bound: true,
    shutdown() { return deferred.promise; },
  }, async (harness) => {
    const manage = findMenuItem(harness.installedMenu(), 'Manage accounts');
    const prevented = harness.emitBeforeQuit();
    assert.strictEqual(prevented.defaultPrevented, true);
    await waitTurn();
    const before = harness.state.dispatchCalls.length;
    const dialogs = harness.state.errorBoxes.length;
    invokeClick(manage);
    await waitTurn();
    assert.strictEqual(harness.state.dispatchCalls.length, before, 'manage dispatched after quit began');
    assert.strictEqual(harness.state.errorBoxes.length, dialogs, 'manage dialog after quit began');

    harness.emitApp('ready');
    assert.strictEqual(harness.state.startCalls.length, 1, 'repeated ready restarted the broker');
    invokeClick(findMenuItem(harness.installedMenu(), 'Manage accounts'));
    await waitTurn();
    assert.strictEqual(harness.state.dispatchCalls.length, before, 'repeated ready reopened accounts');
    deferred.resolve();
    await withTimeout(deferred.promise, 'pending quit shutdown did not fulfill');
  });
});

test('supervisor spawn env is an own-data allowlist copy of the seven GUI strings', async () => {
  const exactLimit = 'z'.repeat(4096);
  const env = {
    LANG: 'C.UTF-8',
    PATH: exactLimit,
    DISPLAY: ':0',
    WAYLAND_DISPLAY: 'wayland-0',
    XDG_RUNTIME_DIR: '/run/user/1000',
    XAUTHORITY: '/home/user/.Xauthority',
    DBUS_SESSION_BUS_ADDRESS: 'unix:path=/run/user/1000/bus',
    HOME: CANARY_PATH,
    NODE_OPTIONS: `--require ${CANARY_PATH}`,
    DYLD_INSERT_LIBRARIES: CANARY,
    LD_PRELOAD: CANARY,
    SECRET_TOKEN: CANARY,
    NUMBER: 1,
    FLAG: true,
  };
  Object.defineProperty(env, 'ACCESSOR', {
    enumerable: true,
    get() { assert.fail('env accessor invoked'); },
  });
  const allowedExact = {
    LANG: 'C.UTF-8',
    PATH: exactLimit,
    DISPLAY: ':0',
    WAYLAND_DISPLAY: 'wayland-0',
    XDG_RUNTIME_DIR: '/run/user/1000',
    XAUTHORITY: '/home/user/.Xauthority',
    DBUS_SESSION_BUS_ADDRESS: 'unix:path=/run/user/1000/bus',
  };

  await withFakeHarness({ env, stdinEnd: false }, async (ctx) => {
    const started = ctx.supervisor.start();
    assert.strictEqual(started.ok, true);
    const spawn = ctx.calls.find((call) => call[0] === 'spawn');
    assert.ok(spawn, 'supervisor never spawned');
    assert.strictEqual(spawn[1], FAKE_BROKER);
    assert.deepStrictEqual(spawn[2], []);
    assert.strictEqual(spawn[3].cwd, FAKE_DATA);
    assert.strictEqual(spawn[3].shell, false);
    assert.deepStrictEqual(spawn[3].stdio, ['pipe', 'pipe', 'pipe']);
    assert.notStrictEqual(spawn[3].env, env);
    assert.notStrictEqual(spawn[3].env, process.env);
    assertAllowlistedEnvCopy(spawn[3].env, env, 'spawn');
    assert.deepStrictEqual(spawn[3].env, allowedExact);
    assertNoCanary(spawn, 'spawn request');
  });

  const omitted = {
    LANG: `C\0${CANARY}`,
    PATH: '/usr/bin',
    DISPLAY: 'x'.repeat(4097),
    WAYLAND_DISPLAY: 1,
    XDG_RUNTIME_DIR: { path: CANARY_PATH },
    XAUTHORITY: '/tmp/xauth',
    DBUS_SESSION_BUS_ADDRESS: 'unix:path=/tmp/bus',
  };
  Object.defineProperty(omitted, 'XAUTHORITY', {
    enumerable: true,
    get() { assert.fail('XAUTHORITY accessor invoked'); },
  });
  await withFakeHarness({ env: omitted, stdinEnd: false }, async (ctx) => {
    const started = ctx.supervisor.start();
    assert.strictEqual(started.ok, true);
    const spawn = ctx.calls.find((call) => call[0] === 'spawn');
    assert.deepStrictEqual(spawn[3].env, {
      PATH: '/usr/bin',
      DBUS_SESSION_BUS_ADDRESS: 'unix:path=/tmp/bus',
    });
    assert.ok(!Object.prototype.hasOwnProperty.call(spawn[3].env, 'LANG'));
    assert.ok(!Object.prototype.hasOwnProperty.call(spawn[3].env, 'DISPLAY'));
    assert.ok(!Object.prototype.hasOwnProperty.call(spawn[3].env, 'WAYLAND_DISPLAY'));
    assert.ok(!Object.prototype.hasOwnProperty.call(spawn[3].env, 'XDG_RUNTIME_DIR'));
    assert.ok(!Object.prototype.hasOwnProperty.call(spawn[3].env, 'XAUTHORITY'));
    assertNoCanary(spawn, 'omitted spawn env');
  });

  const unicodeAtLimit = '\u00e9'.repeat(2048);
  const displayCases = [
    { label: 'nonstring', value: 13, expected: undefined },
    { label: 'embedded NUL', value: `:0\0${CANARY}`, expected: undefined },
    { label: '4097 ASCII bytes', value: 'x'.repeat(4097), expected: undefined },
    { label: '4096 ASCII bytes', value: 'x'.repeat(4096), expected: 'x'.repeat(4096) },
    { label: '4096 Unicode bytes', value: unicodeAtLimit, expected: unicodeAtLimit },
    { label: '4097 Unicode bytes', value: `${unicodeAtLimit}x`, expected: undefined },
  ];
  for (const entry of displayCases) {
    const source = { PATH: '/usr/bin', DISPLAY: entry.value };
    await withFakeHarness({ env: source, stdinEnd: false }, async (ctx) => {
      assert.strictEqual(ctx.supervisor.start().ok, true, `${entry.label} start`);
      const spawn = ctx.calls.find((call) => call[0] === 'spawn');
      const expected = { PATH: '/usr/bin' };
      if (entry.expected !== undefined) expected.DISPLAY = entry.expected;
      assert.deepStrictEqual(spawn[3].env, expected, entry.label);
    });
  }

  let displayGetterCalls = 0;
  const accessorSource = { PATH: '/usr/bin' };
  Object.defineProperty(accessorSource, 'DISPLAY', {
    enumerable: true,
    get() {
      displayGetterCalls += 1;
      return CANARY;
    },
  });
  await withFakeHarness({ env: accessorSource, stdinEnd: false }, async (ctx) => {
    assert.strictEqual(ctx.supervisor.start().ok, true, 'accessor start');
    const spawn = ctx.calls.find((call) => call[0] === 'spawn');
    assert.deepStrictEqual(spawn[3].env, { PATH: '/usr/bin' });
    assert.strictEqual(displayGetterCalls, 0, 'allowlisted DISPLAY getter was invoked');
  });
});

test('main copies the seven allowlisted process.env strings and never passes process.env identity', async () => {
  const injected = {
    LANG: 'C.WAL013',
    PATH: '/usr/bin/wal013',
    DISPLAY: ':13',
    WAYLAND_DISPLAY: 'wayland-13',
    XDG_RUNTIME_DIR: '/run/user/13',
    XAUTHORITY: '/tmp/wal013-xauth',
    DBUS_SESSION_BUS_ADDRESS: 'unix:path=/run/user/13/bus',
  };
  const extras = {
    HOME: CANARY_PATH,
    NODE_OPTIONS: `--require ${CANARY}`,
    SECRET_TOKEN: CANARY,
    DYLD_INSERT_LIBRARIES: CANARY,
    LD_PRELOAD: CANARY,
  };
  const previous = {};
  for (const key of [...ALLOWED_ENV, ...Object.keys(extras)]) {
    previous[key] = Object.prototype.hasOwnProperty.call(process.env, key)
      ? process.env[key]
      : undefined;
  }
  try {
    for (const [key, value] of Object.entries(injected)) process.env[key] = value;
    for (const [key, value] of Object.entries(extras)) process.env[key] = value;
    await withMain({}, async (harness) => {
      assert.ok(harness.state.factoryCalls.length >= 2, 'configured supervisor was not created');
      const factoryOptions = harness.state.factoryCalls[harness.state.factoryCalls.length - 1];
      assert.ok(factoryOptions.env, 'main did not supply supervisor env');
      assert.notStrictEqual(factoryOptions.env, process.env);
      assertAllowlistedEnvCopy(factoryOptions.env, process.env, 'main factory');
      assert.deepStrictEqual(factoryOptions.env, injected);
    });
  } finally {
    for (const [key, value] of Object.entries(previous)) {
      if (value === undefined) delete process.env[key];
      else process.env[key] = value;
    }
  }
});

test('dispatcher accepts account.manage{} only while bound and rejects secret-bearing methods', () => {
  assert.ok(BROKER_METHODS.includes('account.manage'), 'account.manage missing from BROKER_METHODS');
  assert.ok(BROKER_METHODS.includes('account.list'));
  assert.ok(BROKER_METHODS.includes('account.lock'));
  for (const method of SECRET_METHODS) {
    assert.ok(!BROKER_METHODS.includes(method), `${method} was published on the wire`);
  }

  const calls = [];
  const dispatch = createBrokerDispatcher({
    bound: () => true,
    send: (method, params) => {
      calls.push([method, params]);
      return { ok: true };
    },
  });
  assert.strictEqual(dispatch('account.manage', {}).ok, true);
  assert.deepStrictEqual(calls, [['account.manage', {}]]);

  const unbound = createBrokerDispatcher({
    bound: () => false,
    send: () => { assert.fail('unbound manage sent'); },
  });
  assert.throws(() => unbound('account.manage', {}), (error) => error.code === 'UNAUTH');

  for (const params of [
    { extra: true },
    { secret: CANARY },
    { account_id: '00112233445566778899aabbccddeeff' },
    [],
    null,
  ]) {
    assert.throws(() => dispatch('account.manage', params), (error) => error.code === 'SCHEMA');
  }
  for (const method of SECRET_METHODS) {
    assert.throws(() => dispatch(method, {}), (error) => error.code === 'SCHEMA');
    assert.throws(
      () => dispatch(method, { secret: CANARY, mnemonic: CANARY }),
      (error) => error.code === 'SCHEMA'
    );
  }
  assert.strictEqual(calls.length, 1);
  assertNoCanary(calls, 'dispatcher');
});

test('graceful EOF ends stdin once, waits 1000/1250/1500, and resolves only on close', async () => {
  await withFakeHarness(async (ctx) => {
    bindFake(ctx);
    ctx.supervisor.trackIntent(INTENT_ID);
    const pending = ctx.supervisor.dispatch('account.list', {});
    const pendingWatch = ctx.observe(pending);
    const stdoutData = ctx.child.stdout.listenerCount('data');
    assert.ok(stdoutData >= 1);

    const first = ctx.supervisor.shutdown();
    const shutdownWatch = ctx.observe(first);
    assert.strictEqual(first, ctx.supervisor.shutdown());
    await flush();

    const frames = decodeWriteList(ctx.protocolWrites);
    assert.ok(frames.some((frame) => frame.method === 'intent.cancel'));
    const cancelIndex = ctx.calls.findIndex((call) => (
      call[0] === 'protocol' && decodeFrames([call[1]]).values.some(
        (frame) => frame.method === 'intent.cancel'
      )
    ));
    const endIndex = ctx.calls.findIndex((call) => call[0] === 'end');
    assert.ok(cancelIndex >= 0, 'intent.cancel was not written');
    assert.ok(endIndex >= 0, 'stdin.end was not called');
    assert.ok(cancelIndex < endIndex, 'stdin.end preceded intent.cancel');
    assert.strictEqual(pendingWatch.state, 'rejected');
    assert.strictEqual(pendingWatch.error.code, 'UNAVAILABLE');
    assert.deepStrictEqual(ctx.supervisor.pendingRequests(), []);
    assert.strictEqual(ctx.supervisor.bound, false);
    assert.strictEqual(ctx.child.stdout.listenerCount('data'), 0, 'input observers were not detached');
    assert.deepStrictEqual(ctx.calls.filter((call) => call[0] === 'end').length, 1);
    assert.deepStrictEqual(killSignals(ctx), []);
    assert.strictEqual(shutdownWatch.state, 'pending');

    ctx.supervisor.shutdown();
    ctx.supervisor.quit();
    assert.deepStrictEqual(ctx.calls.filter((call) => call[0] === 'end').length, 1);
    assert.deepStrictEqual(killSignals(ctx), []);

    ctx.advance(GRACE_MS - 1);
    await flush();
    assert.deepStrictEqual(killSignals(ctx), []);
    assert.strictEqual(shutdownWatch.state, 'pending');
    assert.strictEqual(activeTimers(ctx, GRACE_MS).length, 1);

    ctx.advance(1);
    await flush();
    assert.deepStrictEqual(killSignals(ctx), ['SIGTERM']);
    assert.strictEqual(shutdownWatch.state, 'pending');

    ctx.advance(KILL_MS - 1);
    await flush();
    assert.deepStrictEqual(killSignals(ctx), ['SIGTERM']);

    ctx.advance(1);
    await flush();
    assert.deepStrictEqual(killSignals(ctx), ['SIGTERM', 'SIGKILL']);
    assert.strictEqual(SIGKILL_AT_MS, GRACE_MS + KILL_MS);
    ctx.child.emit('exit', null, 'SIGKILL');
    await flush();
    assert.strictEqual(shutdownWatch.state, 'pending', 'shutdown resolved on exit instead of close');

    ctx.advance(SHUTDOWN_MS - SIGKILL_AT_MS - 1);
    await flush();
    assert.strictEqual(shutdownWatch.state, 'pending');
    ctx.advance(1);
    await flush();
    assert.strictEqual(shutdownWatch.state, 'rejected');
    assert.strictEqual(shutdownWatch.error.code, 'TIMEOUT');
    assert.strictEqual(first, ctx.supervisor.shutdown());
    assert.strictEqual(activeTimers(ctx, GRACE_MS).length, 0);
    assert.strictEqual(activeTimers(ctx, KILL_MS).length, 0);
    assert.strictEqual(activeTimers(ctx, SHUTDOWN_MS).length, 0);
  });
});

test('graceful asynchronous and synchronous close are safe; missing or throwing end falls back immediately', async () => {
  await withFakeHarness({}, async (ctx) => {
    bindFake(ctx);
    ctx.supervisor.trackIntent(INTENT_ID);
    const first = ctx.supervisor.shutdown();
    const shutdownWatch = ctx.observe(first);
    await flush();
    const cancelIndex = ctx.calls.findIndex((call) => (
      call[0] === 'protocol' && decodeFrames([call[1]]).values.some(
        (frame) => frame.method === 'intent.cancel'
      )
    ));
    const endIndex = ctx.calls.findIndex((call) => call[0] === 'end');
    assert.ok(cancelIndex >= 0 && endIndex >= 0 && cancelIndex < endIndex);
    assert.deepStrictEqual(killSignals(ctx), []);
    assert.strictEqual(shutdownWatch.state, 'pending');

    ctx.child.exitCode = 0;
    ctx.child.emit('exit', 0, null);
    await flush();
    assert.strictEqual(shutdownWatch.state, 'pending', 'shutdown resolved on asynchronous exit');
    assert.deepStrictEqual(killSignals(ctx), []);

    ctx.child.emit('close', 0, null);
    await flush();
    assert.strictEqual(shutdownWatch.state, 'resolved');
    assert.strictEqual(shutdownWatch.value, undefined);
    assert.strictEqual(first, ctx.supervisor.shutdown());
    assert.strictEqual(
      ctx.timers.filter((timer) => timer.cleared !== true && timer.fired !== true).length,
      0,
      'graceful close left an active timer'
    );
    ctx.advance(SHUTDOWN_MS * 2);
    await flush();
    assert.deepStrictEqual(killSignals(ctx), [], 'graceful close produced a late signal');
  });

  await withFakeHarness({
    onEnd(child) {
      child.exitCode = 0;
      child.emit('close', 0, null);
    },
  }, async (ctx) => {
    bindFake(ctx);
    const first = ctx.supervisor.shutdown();
    const shutdownWatch = ctx.observe(first);
    await flush();
    assert.deepStrictEqual(ctx.calls.filter((call) => call[0] === 'end').length, 1);
    assert.deepStrictEqual(killSignals(ctx), []);
    assert.strictEqual(shutdownWatch.state, 'resolved');
    assert.strictEqual(shutdownWatch.value, undefined);
    assert.strictEqual(first, ctx.supervisor.shutdown());
    assert.strictEqual(activeTimers(ctx, GRACE_MS).length, 0);
    assert.strictEqual(activeTimers(ctx, KILL_MS).length, 0);
    assert.strictEqual(activeTimers(ctx, SHUTDOWN_MS).length, 0);
    ctx.supervisor.quit();
    assert.deepStrictEqual(killSignals(ctx), []);
  });

  await withFakeHarness({
    onEnd() { throw new Error(`${CANARY} end failed`); },
  }, async (ctx) => {
    bindFake(ctx);
    const first = ctx.supervisor.shutdown();
    const shutdownWatch = ctx.observe(first);
    await flush();
    assert.deepStrictEqual(ctx.calls.filter((call) => call[0] === 'end').length, 1);
    assert.deepStrictEqual(killSignals(ctx), ['SIGTERM']);
    assert.strictEqual(shutdownWatch.state, 'pending');
    ctx.child.emit('close', 0, 'SIGTERM');
    await flush();
    assert.strictEqual(shutdownWatch.state, 'resolved');
    assertNoCanary(ctx.calls.filter((call) => call[0] === 'kill'), 'end-throw fallback');
  });

  await withFakeHarness({ stdinEnd: false }, async (ctx) => {
    bindFake(ctx);
    const first = ctx.supervisor.shutdown();
    const shutdownWatch = ctx.observe(first);
    await flush();
    assert.strictEqual(ctx.calls.filter((call) => call[0] === 'end').length, 0);
    assert.deepStrictEqual(killSignals(ctx), ['SIGTERM']);
    ctx.child.emit('close', 0, 'SIGTERM');
    await flush();
    assert.strictEqual(shutdownWatch.state, 'resolved');
  });
});

async function run(names) {
  const selected = !names || names.length === 0
    ? tests
    : tests.filter((entry) => names.some((name) => entry.name === name || entry.name.includes(name)));
  if (names && names.length > 0 && selected.length === 0) {
    process.stderr.write('no matching wallet account management groups\n');
    process.exit(1);
  }
  let failed = 0;
  for (const { name, fn } of selected) {
    try {
      await fn();
      process.stdout.write(`ok ${name}\n`);
    } catch (error) {
      failed += 1;
      process.stderr.write(`not ok ${name}\n${error && error.stack ? error.stack : error}\n`);
    }
  }
  if (failed) {
    process.stderr.write(`${failed} wallet account management test(s) failed\n`);
    process.exit(1);
  }
  process.stdout.write(`BitBook wallet account management tests passed (${selected.length}).\n`);
}

if (require.main === module) {
  run(process.argv.slice(2)).then(() => {}, (error) => {
    process.stderr.write(`${error && error.stack ? error.stack : error}\n`);
    process.exit(1);
  });
}

module.exports = { tests };
