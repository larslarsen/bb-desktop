'use strict';

const assert = require('assert');
const Module = require('module');
const path = require('path');
const payFixture = require('./fixtures/wallet-pay/snapshots-v1.json');
const { sanitizeWalletSnapshot } = require('../wallet-pay/model');

const repoRoot = path.resolve(__dirname, '..');
const mainPath = path.join(repoRoot, 'social-main.js');
const expectedPage = path.join(repoRoot, 'social', 'index.html');
const expectedPreload = path.join(repoRoot, 'wallet-preload.js');
const DEV_RESOURCES = path.join(repoRoot, 'wallet-broker', 'target', 'app-resources');
const PACKAGED_RESOURCES = path.resolve('/bb-wal011-packaged-resources');
const USER_DATA = path.resolve('/bb-wal011-user-data');
const PIN = 'ab'.repeat(32);
const BROKER_PATH = path.resolve('/bb-wal011-broker', 'bitbook-wallet-broker');
const DATA_DIR = path.join(USER_DATA, 'wallet-broker');
const CANARY = 'WAL011_STARTUP_CANARY_SECRET';
const DOWN_INPUT = { v: 1, broker: 'down', accounts: [] };
const DOWN = sanitizeWalletSnapshot(DOWN_INPUT);
const READY_LIVE = sanitizeWalletSnapshot({ v: 1, broker: 'ready', accounts: [] });
const SETTLEMENT_MS = 1000;

assert.strictEqual(DOWN.broker, 'down');
assert.deepStrictEqual(DOWN.accounts, []);
assert.match(PIN, /^[0-9a-f]{64}$/);

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

function launchFromResolver() {
  return Object.freeze({
    brokerPath: BROKER_PATH,
    expectedSha256: PIN,
    dataDir: DATA_DIR,
  });
}

function createHarness(options = {}) {
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
    order: [],
    supervisors: [],
    quitCalls: 0,
    beforeQuitEvents: [],
    getPathCalls: [],
    lifecycleTrace: [],
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

    emit(event, ...args) {
      for (const handler of this.handlers[event] || []) handler(...args);
    }
  }

  class BrowserWindow {
    constructor(windowOptions) {
      this.options = windowOptions;
      this.webContents = new WebContents();
      this.loadedFiles = [];
      this._closedHandlers = [];
      state.order.push('window');
      state.windows.push(this);
    }

    loadFile(filePath) { this.loadedFiles.push(filePath); }

    on(event, handler) {
      if (event === 'closed') this._closedHandlers.push(handler);
    }

    emitClosed() {
      for (const handler of this._closedHandlers) handler();
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
    isPackaged: options.isPackaged === true,
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
        state.menuTemplates.push(template);
        return { items: template };
      },
      setApplicationMenu(menu) { state.menuSet.push(menu); },
    },
    ipcMain: {
      handle(channel, handler) {
        state.order.push('ipc');
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
      boundFlag: false,
      subscriber: null,
    };
    const supervisor = {
      get bound() { return record.boundFlag; },
      set bound(value) { record.boundFlag = Boolean(value); },
      start() {
        state.order.push('start');
        state.startCalls.push({ kind, factoryOptions });
        if (typeof options.start === 'function') return options.start(record);
        if (options.startError) throw options.startError;
        if (options.startResult) return options.startResult;
        return { ok: true, snapshot: DOWN };
      },
      subscribeSnapshot(callback) {
        assert.strictEqual(typeof callback, 'function');
        state.order.push(`subscribe:${kind}`);
        state.subscribers.push({ kind, callback });
        record.subscriber = callback;
        return () => true;
      },
      dispatch(method, params) {
        state.dispatchCalls.push([method, params]);
        if (typeof options.dispatch === 'function') return options.dispatch(method, params);
        if (options.dispatchError) throw options.dispatchError;
        return options.dispatchResult || READY_LIVE;
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
      state.order.push('resolve');
      state.resolverCalls.push(value);
      if (typeof options.resolve === 'function') return options.resolve(value, this);
      if (options.resolveError) throw options.resolveError;
      return options.launch || launchFromResolver();
    },
    createWalletSupervisor(factoryOptions) {
      state.factoryCalls.push(factoryOptions);
      const kind = state.factoryCalls.length === 1 ? 'initial' : 'configured';
      return makeSupervisor(kind, factoryOptions);
    },
  };
}

function trustedEvent(state, windowIndex) {
  const index = windowIndex == null ? state.windows.length - 1 : windowIndex;
  const win = state.windows[index];
  assert.ok(win, 'expected a BrowserWindow');
  return { senderFrame: win.webContents.mainFrame, sender: win.webContents };
}

function closeWindow(state, windowIndex) {
  const index = windowIndex == null ? state.windows.length - 1 : windowIndex;
  const win = state.windows[index];
  win.emitClosed();
}

function assertSocialUsable(state) {
  assert.strictEqual(state.enableSandboxCalls, 1);
  assert.ok(state.windows.length >= 1, 'social window missing');
  const win = state.windows[state.windows.length - 1];
  assert.strictEqual(path.resolve(win.loadedFiles[0]), path.resolve(expectedPage));
  const prefs = win.options.webPreferences;
  assert.strictEqual(prefs.sandbox, true);
  assert.strictEqual(prefs.contextIsolation, true);
  assert.strictEqual(prefs.nodeIntegration, false);
  assert.strictEqual(path.resolve(prefs.preload), path.resolve(expectedPreload));
  assert.deepStrictEqual(Object.keys(state.ipcHandlers).sort(), [
    'wallet:accounts:list',
    'wallet:intent:begin',
    'wallet:intent:cancel',
    'wallet:payee-request:get',
    'wallet:snapshot:get',
  ]);
}

function assertNoCanary(state) {
  const payload = JSON.stringify({
    rendererMessages: state.rendererMessages,
    errorBoxes: state.errorBoxes,
  });
  assert.ok(!payload.includes(CANARY), 'raw diagnostic canary leaked');
}

function assertExactResolverInput(call, resourcesPath) {
  assert.deepStrictEqual(Object.keys(call).sort(), [
    'arch', 'platform', 'resourcesPath', 'userDataPath',
  ]);
  assert.strictEqual(call.resourcesPath, resourcesPath);
  assert.strictEqual(call.userDataPath, USER_DATA);
  assert.strictEqual(call.platform, process.platform);
  assert.strictEqual(call.arch, process.arch);
}

function assertConfiguredFactory(factoryOptions, launch) {
  assert.ok(factoryOptions && typeof factoryOptions === 'object');
  assert.strictEqual(factoryOptions.brokerPath, launch.brokerPath);
  assert.strictEqual(factoryOptions.expectedSha256, launch.expectedSha256);
  assert.strictEqual(factoryOptions.dataDir, launch.dataDir);
  if (Object.prototype.hasOwnProperty.call(factoryOptions, 'env')) {
    assert.notStrictEqual(factoryOptions.env, process.env);
    const env = factoryOptions.env;
    assert.ok(env && typeof env === 'object' && !Array.isArray(env));
    const allowed = new Set([
      'LANG', 'PATH', 'DISPLAY', 'WAYLAND_DISPLAY',
      'XDG_RUNTIME_DIR', 'XAUTHORITY', 'DBUS_SESSION_BUS_ADDRESS',
    ]);
    for (const key of Object.keys(env)) {
      assert.ok(allowed.has(key), `unexpected supervisor env ${key}`);
      assert.strictEqual(typeof env[key], 'string');
      assert.ok(!env[key].includes('\0'));
      assert.ok(Buffer.byteLength(env[key], 'utf8') <= 4096);
    }
  }
  assert.notStrictEqual(factoryOptions.env, process.env);
  const serialized = JSON.stringify(factoryOptions);
  assert.ok(!serialized.includes(CANARY));
}

function subscribeBeforeStart(order) {
  const startAt = order.indexOf('start');
  const configuredSub = order.indexOf('subscribe:configured');
  const windowAt = order.indexOf('window');
  const ipcAt = order.indexOf('ipc');
  assert.notStrictEqual(windowAt, -1);
  assert.notStrictEqual(ipcAt, -1);
  assert.notStrictEqual(configuredSub, -1);
  assert.notStrictEqual(startAt, -1);
  assert.ok(windowAt < ipcAt, 'window before ipc');
  assert.ok(ipcAt < configuredSub, 'ipc before configured subscribe');
  assert.ok(configuredSub < startAt, 'subscribe before start');
  assert.strictEqual(order.filter((item) => item === 'start').length, 1);
}

async function withMain(options, fn) {
  const originalLoad = Module._load;
  const previousMain = require.cache[mainPath];
  const hadResources = Object.prototype.hasOwnProperty.call(process, 'resourcesPath');
  const previousResources = process.resourcesPath;
  const unhandled = watchUnhandled();
  const harness = createHarness(options);
  if (options.resourcesPath !== undefined) process.resourcesPath = options.resourcesPath;
  Module._load = function loadWithMocks(request, parent, isMain) {
    if (request === 'electron') return harness.electron;
    if (/(?:^|\/)wallet-broker\/launch-config$/.test(request)) {
      return { resolveWalletBrokerLaunch: harness.resolveWalletBrokerLaunch.bind(harness) };
    }
    if (/(?:^|\/)wallet-broker\/supervisor$/.test(request)) {
      return {
        createWalletSupervisor: harness.createWalletSupervisor.bind(harness),
        sanitizeSnapshot: sanitizeWalletSnapshot,
      };
    }
    return originalLoad.call(this, request, parent, isMain);
  };
  try {
    delete require.cache[mainPath];
    require(mainPath);
    if (options.emitReady !== false) harness.emitApp('ready');
    const result = await fn(harness);
    assert.strictEqual(unhandled.seen.length, 0, 'unhandled rejection during startup fixture');
    return result;
  } finally {
    Module._load = originalLoad;
    if (previousMain) require.cache[mainPath] = previousMain;
    else delete require.cache[mainPath];
    if (hadResources) process.resourcesPath = previousResources;
    else delete process.resourcesPath;
    unhandled.restore();
  }
}

test('ready-only packaged and development paths resolve once with subscribe-before-start', async () => {
  await withMain({ emitReady: false, isPackaged: false }, async (harness) => {
    assert.deepStrictEqual(harness.state.resolverCalls, []);
    assert.deepStrictEqual(harness.state.startCalls, []);
    assert.strictEqual(harness.state.windows.length, 0);
    assert.deepStrictEqual(Object.keys(harness.state.ipcHandlers), []);
    assert.strictEqual(harness.state.factoryCalls.length, 1);

    harness.emitApp('ready');
    assertSocialUsable(harness.state);
    assert.strictEqual(harness.state.resolverCalls.length, 1);
    assertExactResolverInput(harness.state.resolverCalls[0], DEV_RESOURCES);
    assert.strictEqual(harness.state.getPathCalls[0], 'userData');
    assert.strictEqual(harness.state.factoryCalls.length, 2);
    assertConfiguredFactory(harness.state.factoryCalls[1], launchFromResolver());
    assert.strictEqual(harness.state.startCalls.length, 1);
    subscribeBeforeStart(harness.state.order);

    harness.emitApp('ready');
    harness.emitApp('activate');
    assert.strictEqual(harness.state.resolverCalls.length, 1);
    assert.strictEqual(harness.state.startCalls.length, 1);
    assert.strictEqual(harness.state.windows.length, 1);
  });

  await withMain({
    isPackaged: true,
    resourcesPath: PACKAGED_RESOURCES,
  }, async (harness) => {
    assert.strictEqual(harness.state.resolverCalls.length, 1);
    assertExactResolverInput(harness.state.resolverCalls[0], PACKAGED_RESOURCES);
    assert.strictEqual(harness.state.resolverCalls[0].resourcesPath, process.resourcesPath);
    assert.strictEqual(harness.state.startCalls.length, 1);
    subscribeBeforeStart(harness.state.order);
  });
});

test('unbound snapshot get returns cached down clones then live status after handshake', async () => {
  await withMain({}, async (harness) => {
    const configured = harness.state.supervisors[1];
    assert.ok(configured, 'configured supervisor missing');
    assert.strictEqual(configured.supervisor.bound, false);
    assert.deepStrictEqual(harness.state.startCalls[0] && harness.state.startCalls[0].kind, 'configured');

    const handler = harness.state.ipcHandlers['wallet:snapshot:get'];
    const listHandler = harness.state.ipcHandlers['wallet:accounts:list'];
    const event = trustedEvent(harness.state);
    const first = await handler(event);
    const second = await handler(event);
    assert.deepStrictEqual(first, DOWN);
    assert.deepStrictEqual(second, DOWN);
    assert.notStrictEqual(first, second);
    first.broker = 'ready';
    first.accounts.push({ injected: true });
    assert.deepStrictEqual(second, DOWN);
    assert.deepStrictEqual(harness.state.dispatchCalls, []);

    const listReturned = listHandler(event);
    assert.ok(listReturned && typeof listReturned.then === 'function');
    await withTimeout(listReturned.then(() => {}, () => {}), 'unbound list dispatch did not settle');
    assert.deepStrictEqual(harness.state.dispatchCalls, [['account.list', undefined]]);

    const messagesBefore = harness.state.rendererMessages.length;
    configured.subscriber(JSON.parse(JSON.stringify(payFixture.valid_full_input)));
    assert.ok(harness.state.rendererMessages.length > messagesBefore);
    const delivered = harness.state.rendererMessages[harness.state.rendererMessages.length - 1];
    assert.strictEqual(delivered[1], 'wallet:snapshot:subscribe');
    assert.deepStrictEqual(delivered[2], payFixture.valid_full_expected);
    assert.ok(!JSON.stringify(delivered).includes('SNAPSHOT_SECRET_CANARY'));
    delivered[2].accounts[0].label = 'renderer mutation';
    assert.strictEqual(payFixture.valid_full_input.accounts[0].label, 'Shielded ZEC');

    const cached = await handler(event);
    assert.deepStrictEqual(cached, payFixture.valid_full_expected);
    assert.deepStrictEqual(harness.state.dispatchCalls, [['account.list', undefined]]);

    configured.boundFlag = true;
    const live = await handler(event);
    assert.deepStrictEqual(live, READY_LIVE);
    assert.deepStrictEqual(
      harness.state.dispatchCalls,
      [['account.list', undefined], ['status.get', undefined]]
    );
    assert.notStrictEqual(live, READY_LIVE);
  });
});

test('resolver or start failure leaves social usable; fallback still denies sender and payload', async () => {
  const resolveError = new Error(CANARY);
  await withMain({ resolveError }, async (harness) => {
    assertSocialUsable(harness.state);
    assert.strictEqual(harness.state.startCalls.length, 0);
    assert.strictEqual(harness.state.factoryCalls.length, 1);
    const snapshot = await harness.state.ipcHandlers['wallet:snapshot:get'](trustedEvent(harness.state));
    assert.deepStrictEqual(snapshot, DOWN);
    assert.deepStrictEqual(harness.state.dispatchCalls, []);
    assertNoCanary(harness.state);
  });

  const startError = new Error(CANARY);
  await withMain({ startError }, async (harness) => {
    assertSocialUsable(harness.state);
    assert.strictEqual(harness.state.startCalls.length, 1);
    const snapshot = await harness.state.ipcHandlers['wallet:snapshot:get'](trustedEvent(harness.state));
    assert.deepStrictEqual(snapshot, DOWN);
    assert.notStrictEqual(snapshot.broker, 'ready');
    assertNoCanary(harness.state);
  });

  await withMain({ startResult: { ok: false, snapshot: DOWN } }, async (harness) => {
    assertSocialUsable(harness.state);
    const snapshot = await harness.state.ipcHandlers['wallet:snapshot:get'](trustedEvent(harness.state));
    assert.deepStrictEqual(snapshot, DOWN);
    assert.deepStrictEqual(harness.state.dispatchCalls, []);
  });

  await withMain({ resolveError }, async (harness) => {
    const handler = harness.state.ipcHandlers['wallet:snapshot:get'];
    const valid = trustedEvent(harness.state);
    const before = harness.state.dispatchCalls.length;
    assert.throws(() => handler({ senderFrame: {}, sender: valid.sender }));
    assert.throws(() => handler({
      senderFrame: valid.senderFrame,
      sender: { getURL: () => 'https://evil.example' },
    }));
    assert.throws(() => handler(valid, { unexpected: true }));
    assert.strictEqual(harness.state.dispatchCalls.length, before);
    const snapshot = await handler(valid);
    assert.deepStrictEqual(snapshot, DOWN);
  });
});

test('snapshot cache updates while the window is closed and clones stay isolated', async () => {
  await withMain({}, async (harness) => {
    const configured = harness.state.supervisors[1];
    const source = JSON.parse(JSON.stringify(payFixture.valid_full_input));
    closeWindow(harness.state, 0);
    const before = harness.state.rendererMessages.length;
    configured.subscriber(source);
    assert.strictEqual(harness.state.rendererMessages.length, before, 'closed window received a snapshot');

    harness.emitApp('activate');
    assert.strictEqual(harness.state.startCalls.length, 1, 'activation restarted the broker');
    assert.strictEqual(harness.state.resolverCalls.length, 1);
    assert.ok(harness.state.windows.length >= 2, 'activate did not recreate the window');
    const snapshot = await harness.state.ipcHandlers['wallet:snapshot:get'](trustedEvent(harness.state));
    assert.deepStrictEqual(snapshot, payFixture.valid_full_expected);
    assert.notStrictEqual(snapshot, source);
    snapshot.accounts[0].label = 'mutated';
    const again = await harness.state.ipcHandlers['wallet:snapshot:get'](trustedEvent(harness.state));
    assert.strictEqual(again.accounts[0].label, 'Shielded ZEC');
    assert.ok(!JSON.stringify(again).includes('SNAPSHOT_SECRET_CANARY'));
    assert.deepStrictEqual(harness.state.dispatchCalls, []);
  });
});

test('pre-ready quit and reentrant quit during resolver never spawn; activate stays idle-gated', async () => {
  await withMain({ emitReady: false }, async (harness) => {
    const first = harness.emitBeforeQuit();
    assert.strictEqual(first.defaultPrevented, true);
    await withTimeout(waitTurn(), 'pre-ready quit drain timed out');
    harness.emitApp('ready');
    assert.strictEqual(harness.state.windows.length, 0, 'ready after quit created a window');
    assert.deepStrictEqual(harness.state.resolverCalls, []);
    assert.deepStrictEqual(harness.state.startCalls, []);
    harness.emitApp('activate');
    assert.strictEqual(harness.state.windows.length, 0, 'activate after quit created a window');
  });

  await withMain({
    emitReady: false,
    resolve(_value, harness) {
      harness.emitBeforeQuit();
      return launchFromResolver();
    },
  }, async (harness) => {
    harness.emitApp('ready');
    assertSocialUsable(harness.state);
    assert.strictEqual(harness.state.resolverCalls.length, 1);
    assert.strictEqual(harness.state.startCalls.length, 0, 'spawned after reentrant quit');
  });
});

test('configured supervisor shutdown is awaited on the replaced instance', async () => {
  const deferred = createDeferred();
  await withMain({
    shutdown(kind) {
      if (kind === 'configured') return deferred.promise;
      return Promise.resolve();
    },
  }, async (harness) => {
    assert.strictEqual(harness.state.supervisors[1].kind, 'configured');
    const first = harness.emitBeforeQuit();
    assert.strictEqual(first.defaultPrevented, true);
    await withTimeout(waitTurn(), 'configured shutdown pending drain timed out');
    assert.deepStrictEqual(
      harness.state.shutdownCalls.map((item) => item.kind),
      ['configured']
    );
    assert.strictEqual(harness.state.quitCalls, 0);
    assert.strictEqual(deferred.settled, false);
    deferred.resolve();
    await withTimeout(deferred.promise, 'configured shutdown did not fulfill');
    await withTimeout(waitTurn(), 'configured resumed quit drain timed out');
    assert.strictEqual(harness.state.quitCalls, 1);
  });
});

async function run(names) {
  const selected = !names || names.length === 0
    ? tests
    : tests.filter((entry) => names.some((name) => entry.name === name || entry.name.includes(name)));
  if (names && names.length > 0 && selected.length === 0) {
    process.stderr.write('no matching wallet startup groups\n');
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
    process.stderr.write(`${failed} wallet startup test(s) failed\n`);
    process.exit(1);
  }
  process.stdout.write(`BitBook wallet startup tests passed (${selected.length}).\n`);
}

if (require.main === module) {
  run(process.argv.slice(2)).then(() => {}, (error) => {
    process.stderr.write(`${error && error.stack ? error.stack : error}\n`);
    process.exit(1);
  });
}

module.exports = { tests, run };
