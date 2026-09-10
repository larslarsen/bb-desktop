'use strict';

const assert = require('assert');
const fs = require('fs');
const Module = require('module');
const path = require('path');
const payFixture = require('./fixtures/wallet-pay/snapshots-v1.json');
const { sanitizeWalletSnapshot } = require('../wallet-pay/model');

const repoRoot = path.resolve(__dirname, '..');
const mainPath = path.join(repoRoot, 'social-main.js');
const expectedPage = path.join(repoRoot, 'social', 'index.html');
const expectedPreload = path.join(repoRoot, 'wallet-preload.js');
const packagePath = path.join(repoRoot, 'package.json');

const FORBIDDEN_ELECTRON_APIS = [
  'ipcRenderer',
  'remote',
  'BrowserView',
  'protocol',
  'webContents',
];

const DENIED_PERMISSIONS = [
  'media',
  'geolocation',
  'notifications',
  'midiSysex',
  'pointerLock',
  'fullscreen',
  'openExternal',
  'clipboard-read',
  'clipboard-sanitized-write',
  'serial',
  'hid',
  'usb',
  'mediaKeySystem',
  'display-capture',
];

const HOSTILE_URLS = [
  'https://evil.example/steal',
  'http://evil.example/steal',
  'file:///etc/passwd',
  'javascript:alert(1)',
  'data:text/html,<script>alert(1)</script>',
];

const WINDOW_OPEN_DISPOSITIONS = ['background-tab', 'foreground-tab', 'new-window'];

const MAINTAINED_SOURCE_PATHS = [
  'social-main.js',
  path.join('social', 'index.html'),
  path.join('social', 'app.js'),
  path.join('social', 'core.js'),
];

const FORBIDDEN_MAINTAINED_SINKS = [
  'innerHTML',
  'outerHTML',
  'insertAdjacentHTML',
  'document.write',
  'eval(',
  'new Function',
  'javascript:',
];

class PolicyError extends Error {
  constructor(message) {
    super(message);
    this.name = 'PolicyError';
  }
}

const MAX_BEFORE_QUIT_REENTRY = 8;
const MAX_BEFORE_QUIT_EVENTS = 12;
const QUIT_ERROR_TITLE = 'Unable to close BitBook';
const QUIT_ERROR_CONTENT = 'Wallet shutdown could not be confirmed. BitBook will keep running.';

function createElectronMock(options = {}) {
  const accessed = new Set();
  const state = {
    enableSandboxCalls: 0,
    windows: [],
    permissionRequestHandler: null,
    permissionCheckHandler: null,
    menuSet: [],
    menuTemplates: [],
    openExternalCalls: [],
    appHandlers: Object.create(null),
    quitCalls: 0,
    ipcHandlers: Object.create(null),
    supervisorCalls: [],
    supervisorResults: [],
    supervisorSubscribers: [],
    supervisorStartCalls: [],
    resolverCalls: [],
    rendererMessages: [],
    sanitizerCalls: [],
    shutdownCalls: [],
    supervisorQuitCalls: 0,
    supervisorCloseCalls: 0,
    supervisorKillCalls: 0,
    errorBoxes: [],
    beforeQuitEvents: [],
    lifecycleTrace: [],
    forceExitCalls: [],
    beforeQuitDepth: 0,
    readyEmitted: false,
  };

  class WebContents {
    constructor() {
      this.handlers = Object.create(null);
      this.windowOpenHandler = null;
      this.mainFrame = { url: `file://${expectedPage}` };
      this.getURL = () => `file://${expectedPage}`;
      this.send = (channel, payload) => state.rendererMessages.push([this.mainFrame, channel, payload]);
    }

    on(event, handler) {
      assert.strictEqual(typeof event, 'string');
      assert.strictEqual(typeof handler, 'function');
      if (!this.handlers[event]) {
        this.handlers[event] = [];
      }
      this.handlers[event].push(handler);
    }

    setWindowOpenHandler(handler) {
      assert.strictEqual(typeof handler, 'function');
      this.windowOpenHandler = handler;
    }

    emit(event, ...args) {
      for (const handler of this.handlers[event] || []) {
        handler(...args);
      }
    }
  }

  class BrowserWindow {
    constructor(options) {
      this.options = options;
      this.webContents = new WebContents();
      this.loadedFiles = [];
      this.loadedURLs = [];
      this._closedHandlers = [];
      state.windows.push(this);
    }

    loadFile(filePath) {
      this.loadedFiles.push(filePath);
    }

    loadURL(url) {
      this.loadedURLs.push(url);
    }

    on(event, handler) {
      if (event === 'closed') {
        this._closedHandlers.push(handler);
      }
    }
  }

  const session = {
    defaultSession: {
      setPermissionRequestHandler(handler) {
        state.permissionRequestHandler = handler;
      },
      setPermissionCheckHandler(handler) {
        state.permissionCheckHandler = handler;
      },
    },
  };

  function emitRegisteredBeforeQuit(viaAppQuit) {
    const handlers = state.appHandlers['before-quit'];
    assert.ok(Array.isArray(handlers) && handlers.length > 0, 'before-quit handler is missing');
    assert.ok(
      state.beforeQuitEvents.length < MAX_BEFORE_QUIT_EVENTS,
      'too many before-quit events'
    );
    assert.ok(
      state.beforeQuitDepth < MAX_BEFORE_QUIT_REENTRY,
      'before-quit re-entered too many times'
    );
    state.beforeQuitDepth += 1;
    const event = {
      defaultPrevented: false,
      preventDefault() {
        this.defaultPrevented = true;
      },
    };
    const record = {
      defaultPrevented: false,
      returned: undefined,
      depth: state.beforeQuitDepth,
      viaAppQuit: viaAppQuit === true,
    };
    state.beforeQuitEvents.push(record);
    state.lifecycleTrace.push({
      type: 'before-quit',
      depth: state.beforeQuitDepth,
      viaAppQuit: record.viaAppQuit,
    });
    try {
      for (const handler of handlers) {
        record.returned = handler(event);
      }
    } finally {
      record.defaultPrevented = event.defaultPrevented;
      state.lifecycleTrace.push({
        type: event.defaultPrevented ? 'before-quit-prevented' : 'before-quit-allowed',
        depth: state.beforeQuitDepth,
        viaAppQuit: record.viaAppQuit,
      });
      state.beforeQuitDepth -= 1;
    }
    return event;
  }

  const app = {
    enableSandbox() {
      state.enableSandboxCalls += 1;
    },
    getPath(name) {
      if (name !== 'userData') {
        throw new Error(`unexpected app.getPath(${String(name)})`);
      }
      return options.userDataPath || path.resolve('/bb-electron-security-user-data');
    },
    isPackaged: options.isPackaged === true,
    on(event, handler) {
      if (!state.appHandlers[event]) {
        state.appHandlers[event] = [];
      }
      state.appHandlers[event].push(handler);
    },
    quit() {
      state.quitCalls += 1;
      state.lifecycleTrace.push({ type: 'app.quit' });
      emitRegisteredBeforeQuit(true);
    },
    exit(code) {
      state.forceExitCalls.push({ api: 'app.exit', code });
    },
    relaunch(opts) {
      state.forceExitCalls.push({ api: 'app.relaunch', opts });
    },
  };

  const Menu = {
    buildFromTemplate(template) {
      state.menuTemplates = state.menuTemplates || [];
      state.menuTemplates.push(template);
      return { items: template };
    },
    setApplicationMenu(menu) {
      state.menuSet.push(menu);
    },
  };

  const shell = {
    openExternal(url) {
      state.openExternalCalls.push(url);
      return Promise.resolve();
    },
  };

  const ipcMain = {
    handle(channel, handler) {
      assert.strictEqual(typeof channel, 'string');
      assert.strictEqual(typeof handler, 'function');
      assert.ok(!state.ipcHandlers[channel], `duplicate ipcMain channel ${channel}`);
      state.ipcHandlers[channel] = handler;
    },
  };

  const dialog = {
    showErrorBox(title, content) {
      state.errorBoxes.push({ title, content });
      state.lifecycleTrace.push({ type: 'showErrorBox', title, content });
      if (typeof options.showErrorBox === 'function') {
        return options.showErrorBox(title, content);
      }
      return undefined;
    },
  };

  const target = { app, BrowserWindow, Menu, session, shell, ipcMain, dialog };

  const electron = new Proxy(target, {
    get(receiver, prop) {
      accessed.add(String(prop));
      if (prop in receiver) {
        return receiver[prop];
      }
      return undefined;
    },
    has(receiver, prop) {
      accessed.add(String(prop));
      return prop in receiver;
    },
  });

  return {
    electron,
    state,
    accessed,
    emitApp(event, ...args) {
      for (const handler of state.appHandlers[event] || []) {
        handler(...args);
      }
    },
    emitBeforeQuit() {
      return emitRegisteredBeforeQuit(false);
    },
  };
}

let runtime;

function loadMaintainedMain(options = {}) {
  assert.ok(fs.existsSync(mainPath), 'maintained Electron entry social-main.js is missing');
  const mock = createElectronMock(options);
  const originalLoad = Module._load;
  const previousMain = require.cache[mainPath];
  const trackedSanitizer = (value) => {
    mock.state.sanitizerCalls.push(value);
    return sanitizeWalletSnapshot(value);
  };
  Module._load = function loadWithElectronMock(request, parent, isMain) {
    if (request === 'electron') {
      return mock.electron;
    }
    if (/(?:^|\/)wallet-broker\/launch-config$/.test(request)) {
      return {
        resolveWalletBrokerLaunch(value) {
          mock.state.resolverCalls.push(value);
          if (typeof options.resolveWalletBrokerLaunch === 'function') {
            return options.resolveWalletBrokerLaunch(value);
          }
          return Object.freeze({
            brokerPath: path.resolve('/bb-electron-security-broker'),
            expectedSha256: 'a'.repeat(64),
            dataDir: path.resolve('/bb-electron-security-user-data', 'wallet-broker'),
          });
        },
      };
    }
    if (/(?:^|\/)wallet-broker\/supervisor$/.test(request)) {
      return {
        sanitizeSnapshot: trackedSanitizer,
        createWalletSupervisor() {
          return {
            get bound() {
              return true;
            },
            start() {
              mock.state.supervisorStartCalls.push({});
              if (typeof options.start === 'function') return options.start();
              return { ok: true, snapshot: { v: 1, broker: 'down', accounts: [] } };
            },
            dispatch(method, params) {
              mock.state.supervisorCalls.push([method, params]);
              const result = typeof options.dispatch === 'function'
                ? options.dispatch(method, params)
                : { ok: true, value: params };
              mock.state.supervisorResults.push(result);
              return result;
            },
            subscribeSnapshot(callback) {
              mock.state.supervisorSubscribers.push(callback);
              return () => true;
            },
            shutdown() {
              mock.state.shutdownCalls.push({});
              mock.state.lifecycleTrace.push({ type: 'shutdown' });
              let result;
              if (typeof options.shutdown === 'function') {
                result = options.shutdown();
              } else {
                result = Promise.resolve();
              }
              if (result && typeof result.then === 'function') {
                result.then(
                  () => {
                    mock.state.lifecycleTrace.push({ type: 'shutdown-fulfilled' });
                  },
                  () => {
                    mock.state.lifecycleTrace.push({ type: 'shutdown-rejected' });
                  }
                );
              }
              return result;
            },
            quit() {
              mock.state.supervisorQuitCalls += 1;
              mock.state.lifecycleTrace.push({ type: 'supervisor.quit' });
            },
            close() {
              mock.state.supervisorCloseCalls += 1;
              mock.state.lifecycleTrace.push({ type: 'supervisor.close' });
            },
            kill() {
              mock.state.supervisorKillCalls += 1;
              mock.state.lifecycleTrace.push({ type: 'supervisor.kill' });
            },
          };
        },
      };
    }
    if (/(?:^|\/)wallet-pay\/model$/.test(request)) {
      return { sanitizeWalletSnapshot: trackedSanitizer };
    }
    return originalLoad.call(this, request, parent, isMain);
  };
  try {
    delete require.cache[mainPath];
    require(mainPath);
  } finally {
    Module._load = originalLoad;
    if (options.restoreMainCache) {
      if (previousMain) require.cache[mainPath] = previousMain;
      else delete require.cache[mainPath];
    }
  }
  if (options.emitReady !== false) {
    mock.state.readyEmitted = true;
    mock.emitApp('ready');
  }
  return mock;
}

function boot() {
  if (!runtime) {
    runtime = loadMaintainedMain();
  }
  return runtime;
}

const SETTLEMENT_TIMEOUT_MS = 1000;

function loadIsolatedMaintainedMain(options = {}) {
  return loadMaintainedMain(Object.assign({}, options, { restoreMainCache: true }));
}

function copyJson(value) {
  return JSON.parse(JSON.stringify(value));
}

function fixtureSupervisorError(code, message) {
  const error = new Error(message);
  error.name = 'SupervisorError';
  error.code = code;
  return error;
}

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

function waitEventLoopTurn() {
  return new Promise((resolve) => {
    setImmediate(resolve);
  });
}

function withSettlementTimeout(promise, message) {
  let timer = null;
  const timeout = new Promise((_, reject) => {
    timer = setTimeout(() => {
      reject(new Error(message));
    }, SETTLEMENT_TIMEOUT_MS);
  });
  return Promise.race([Promise.resolve(promise), timeout]).finally(() => {
    if (timer !== null) clearTimeout(timer);
  });
}

async function observeFinally(values, message) {
  await withSettlementTimeout(
    Promise.all(
      values.filter((value) => value != null).map((value) => Promise.resolve(value).then(() => {}, () => {}))
    ),
    message
  );
}

function trustedWalletEvent(ctx) {
  assert.strictEqual(ctx.state.windows.length, 1, 'isolated client must create exactly one BrowserWindow');
  const win = ctx.state.windows[0];
  return { senderFrame: win.webContents.mainFrame, sender: win.webContents };
}

async function settleDeferreds(deferreds, observed, currentDeferred, message) {
  if (currentDeferred && !currentDeferred.settled) {
    currentDeferred.reject(new Error(message));
  }
  for (const deferred of deferreds) {
    if (!deferred.settled) deferred.reject(new Error(message));
  }
  await observeFinally(
    deferreds.map((deferred) => deferred.promise).concat(observed),
    message
  );
}

function watchForceExits(state) {
  const originalExit = process.exit;
  process.exit = function patchedProcessExit(code) {
    state.forceExitCalls.push({ api: 'process.exit', code });
  };
  return function restoreForceExits() {
    process.exit = originalExit;
  };
}

function watchUnhandledRejections() {
  const seen = [];
  function onUnhandled(reason) {
    seen.push(reason);
  }
  process.on('unhandledRejection', onUnhandled);
  return {
    seen,
    restore() {
      process.removeListener('unhandledRejection', onUnhandled);
    },
  };
}

function requireAppHandlers(ctx, eventName) {
  const handlers = ctx.state.appHandlers[eventName];
  assert.ok(Array.isArray(handlers) && handlers.length > 0, `${eventName} handler is missing`);
  return handlers;
}

function assertUnusedForcePaths(ctx) {
  assert.strictEqual(ctx.state.supervisorQuitCalls, 0, 'supervisor.quit must stay unused');
  assert.strictEqual(ctx.state.supervisorCloseCalls, 0, 'supervisor.close must stay unused');
  assert.strictEqual(ctx.state.supervisorKillCalls, 0, 'supervisor.kill must stay unused');
  assert.strictEqual(ctx.state.forceExitCalls.length, 0, 'force-exit was used');
  assert.deepStrictEqual(ctx.state.supervisorCalls, []);
}

async function cleanupQuitHooks(options) {
  const deferreds = options.deferreds || [];
  const observed = options.observed || [];
  const currentDeferred = options.currentDeferred || null;
  try {
    await settleDeferreds(deferreds, observed, currentDeferred, options.message);
    await withSettlementTimeout(waitEventLoopTurn(), `${options.message} cleanup drain timed out`);
    assert.strictEqual(
      options.unhandled.seen.length,
      0,
      `unhandled rejection during ${options.message} cleanup`
    );
    assertUnusedForcePaths(options.ctx);
  } finally {
    options.restoreExit();
    options.unhandled.restore();
  }
}

function assertFixedQuitErrorBox(box) {
  assert.strictEqual(box.title, QUIT_ERROR_TITLE);
  assert.strictEqual(box.content, QUIT_ERROR_CONTENT);
}

function assertNoPrivateQuitCanary(ctx, canary) {
  const payloads = JSON.stringify({
    errorBoxes: ctx.state.errorBoxes,
    rendererMessages: ctx.state.rendererMessages,
  });
  assert.ok(!payloads.includes(canary), `private canary leaked into dialog/renderer payloads: ${canary}`);
}

function lifecycleTypes(ctx) {
  return ctx.state.lifecycleTrace.map((entry) => entry.type);
}

function windowUnderTest() {
  const ctx = boot();
  assert.strictEqual(ctx.state.windows.length, 1, 'maintained client must create exactly one BrowserWindow');
  return ctx.state.windows[0];
}

function parseCsp(policy) {
  assert.strictEqual(typeof policy, 'string');
  assert.ok(policy.trim(), 'CSP is empty');
  const directives = new Map();
  for (const segment of policy.split(';')) {
    const tokens = segment.trim().split(/\s+/).filter(Boolean);
    if (tokens.length === 0) {
      continue;
    }
    const name = tokens[0].toLowerCase();
    assert.ok(!directives.has(name), `duplicate CSP directive ${name}`);
    directives.set(name, tokens.slice(1));
  }
  return directives;
}

function extractCsp(html) {
  const metas = [];
  const metaRe = /<meta\b([^>]*)>/gi;
  let match;
  while ((match = metaRe.exec(html)) !== null) {
    const attrs = match[1];
    if (!/http-equiv\s*=\s*(['"]?)Content-Security-Policy\1/i.test(attrs)) {
      continue;
    }
    const contentMatch = attrs.match(/\bcontent\s*=\s*"([^"]*)"/i) || attrs.match(/\bcontent\s*=\s*'([^']*)'/i);
    assert.ok(contentMatch, 'CSP meta tag is missing a content attribute');
    metas.push(contentMatch[1]);
  }
  assert.strictEqual(metas.length, 1, 'renderer must declare exactly one CSP meta tag');
  return metas[0];
}

function assertSelfOnly(directives, name) {
  const values = directives.get(name);
  assert.ok(values, `CSP is missing ${name}`);
  assert.deepStrictEqual(values, ["'self'"], `${name} must be self-only, not ${JSON.stringify(values)}`);
}

function assertDenied(directives, name) {
  const values = directives.get(name);
  assert.ok(values, `CSP is missing ${name}`);
  assert.deepStrictEqual(values, ["'none'"], `${name} must be 'none', not ${JSON.stringify(values)}`);
}

function assertHasSource(directives, name, source) {
  const values = directives.get(name);
  assert.ok(values, `CSP is missing ${name}`);
  assert.ok(values.includes(source), `${name} must keep ${source} for daemon/social connectivity`);
}

function preventableEvent() {
  const event = {
    defaultPrevented: false,
    preventDefault() {
      this.defaultPrevented = true;
    },
  };
  return event;
}

const tests = [];

function test(name, fn) {
  tests.push({ name, fn });
}

test('app.enableSandbox is invoked before the window is created', () => {
  const ctx = boot();
  assert.strictEqual(ctx.state.enableSandboxCalls, 1);
  assert.ok(ctx.state.appHandlers.ready && ctx.state.appHandlers.ready.length > 0);
});

test('BrowserWindow explicitly sets the fail-closed webPreferences', () => {
  const prefs = windowUnderTest().options && windowUnderTest().options.webPreferences;
  assert.ok(prefs && typeof prefs === 'object', 'BrowserWindow webPreferences are missing');
  assert.strictEqual(prefs.nodeIntegration, false);
  assert.strictEqual(prefs.contextIsolation, true);
  assert.strictEqual(prefs.sandbox, true);
  assert.strictEqual(prefs.webSecurity, true);
  assert.strictEqual(prefs.allowRunningInsecureContent, false);
  assert.strictEqual(prefs.experimentalFeatures, false);
  assert.strictEqual(path.resolve(prefs.preload), path.resolve(expectedPreload));
  assert.notStrictEqual(prefs.webviewTag, true, 'webviewTag must not be enabled');
  assert.notStrictEqual(prefs.nodeIntegrationInWorker, true);
  assert.notStrictEqual(prefs.nodeIntegrationInSubFrames, true);
  assert.notStrictEqual(prefs.enableRemoteModule, true);
});

test('only the repository social/index.html is loaded', () => {
  const win = windowUnderTest();
  assert.deepStrictEqual(win.loadedURLs, [], 'remote loadURL is not authorized');
  assert.strictEqual(win.loadedFiles.length, 1, 'exactly one local page must be loaded');
  assert.strictEqual(path.resolve(win.loadedFiles[0]), path.resolve(expectedPage));
  assert.ok(fs.existsSync(win.loadedFiles[0]), 'loaded renderer page does not exist');
});

test('package.json keeps the maintained social-main.js entry point', () => {
  const pkg = JSON.parse(fs.readFileSync(packagePath, 'utf8'));
  assert.strictEqual(pkg.main, 'social-main.js');
  assert.strictEqual(pkg.devDependencies.electron, '44.0.0');
  assert.notStrictEqual(pkg.main, 'main.js');
});

test('renderer navigation is denied', () => {
  const win = windowUnderTest();
  assert.ok(win.webContents.handlers['will-navigate'], 'will-navigate handler is missing');
  for (const url of HOSTILE_URLS) {
    const event = preventableEvent();
    win.webContents.emit('will-navigate', event, url);
    assert.strictEqual(event.defaultPrevented, true, `will-navigate did not deny ${url}`);
  }
});

test('renderer redirects are denied', () => {
  const win = windowUnderTest();
  assert.ok(win.webContents.handlers['will-redirect'], 'will-redirect handler is missing');
  for (const url of HOSTILE_URLS) {
    const event = preventableEvent();
    win.webContents.emit('will-redirect', event, url);
    assert.strictEqual(event.defaultPrevented, true, `will-redirect did not deny ${url}`);
  }
});

test('webview attachment is denied', () => {
  const win = windowUnderTest();
  assert.ok(win.webContents.handlers['will-attach-webview'], 'will-attach-webview handler is missing');
  const event = preventableEvent();
  win.webContents.emit('will-attach-webview', event, {}, { src: 'https://evil.example' });
  assert.strictEqual(event.defaultPrevented, true);
});

test('new-window creation is denied and shell.openExternal is unreachable', () => {
  const ctx = boot();
  const win = windowUnderTest();
  const handler = win.webContents.windowOpenHandler;
  assert.strictEqual(typeof handler, 'function', 'setWindowOpenHandler is missing');
  const before = ctx.state.openExternalCalls.length;
  for (const url of HOSTILE_URLS) {
    for (const disposition of WINDOW_OPEN_DISPOSITIONS) {
      const result = handler({ url, disposition });
      assert.deepStrictEqual(
        result,
        { action: 'deny' },
        `new-window did not deny ${url} (${disposition})`
      );
    }
  }
  assert.strictEqual(
    ctx.state.openExternalCalls.length,
    before,
    `shell.openExternal was reached with ${JSON.stringify(ctx.state.openExternalCalls.slice(before))}`
  );
});

test('permission request handler denies every permission', () => {
  const ctx = boot();
  const handler = ctx.state.permissionRequestHandler;
  assert.strictEqual(typeof handler, 'function', 'setPermissionRequestHandler is missing');
  for (const permission of DENIED_PERMISSIONS) {
    let decided = null;
    handler({}, permission, (allowed) => {
      decided = allowed;
    });
    assert.strictEqual(decided, false, `permission request allowed ${permission}`);
  }
});

test('permission check handler denies every permission', () => {
  const ctx = boot();
  const handler = ctx.state.permissionCheckHandler;
  assert.strictEqual(typeof handler, 'function', 'setPermissionCheckHandler is missing');
  for (const permission of DENIED_PERMISSIONS) {
    const allowed = handler({}, permission, 'https://evil.example', { isMainFrame: true });
    assert.strictEqual(allowed, false, `permission check allowed ${permission}`);
  }
});

test('only the explicit local wallet preload and exact ipcMain bridge are introduced', () => {
  const ctx = boot();
  for (const api of FORBIDDEN_ELECTRON_APIS) {
    assert.ok(!ctx.accessed.has(api), `maintained main accessed forbidden Electron API ${api}`);
  }
  const prefs = windowUnderTest().options.webPreferences;
  assert.strictEqual(path.resolve(prefs.preload), path.resolve(expectedPreload));
  assert.ok(fs.existsSync(prefs.preload), 'wallet preload is missing');
  assert.ok(ctx.accessed.has('ipcMain'), 'maintained main did not register the exact wallet IPC boundary');
  assert.notStrictEqual(prefs.webviewTag, true);
});

test('CSP keeps self-only script/style and denies objects, frames, base, and forms', () => {
  const loaded = windowUnderTest().loadedFiles[0];
  const html = fs.readFileSync(loaded, 'utf8');
  const policy = extractCsp(html);
  const directives = parseCsp(policy);
  const joined = policy.toLowerCase();
  assert.ok(!joined.includes('unsafe-inline'), 'CSP adds unsafe-inline');
  assert.ok(!joined.includes('unsafe-eval'), 'CSP adds unsafe-eval');
  assert.ok(!joined.includes('wasm-unsafe-eval'), 'CSP adds wasm-unsafe-eval');
  assertSelfOnly(directives, 'script-src');
  assertSelfOnly(directives, 'style-src');
  assertDenied(directives, 'object-src');
  assertDenied(directives, 'frame-src');
  assertDenied(directives, 'frame-ancestors');
  assertDenied(directives, 'base-uri');
  assertDenied(directives, 'form-action');
  assertHasSource(directives, 'default-src', "'self'");
  assertHasSource(directives, 'connect-src', 'http:');
  assertHasSource(directives, 'connect-src', 'https:');
  assertHasSource(directives, 'connect-src', 'ws:');
  assertHasSource(directives, 'connect-src', 'wss:');
  assertHasSource(directives, 'img-src', "'self'");
  assertHasSource(directives, 'img-src', 'data:');
  assertHasSource(directives, 'img-src', 'http:');
  assertHasSource(directives, 'img-src', 'https:');
  for (const [name, values] of directives) {
    for (const value of values) {
      assert.notStrictEqual(value, "'unsafe-inline'", `${name} allows unsafe-inline`);
      assert.notStrictEqual(value, "'unsafe-eval'", `${name} allows unsafe-eval`);
    }
  }
  for (const forbiddenHost of [
    'coingecko', 'coinpaprika', 'kraken', 'ticker.openbazaar.org',
    'lightwalletd', 'monerod', 'wallet-rpc', 'wallet-broker',
  ]) {
    assert.ok(!joined.includes(forbiddenHost), `renderer CSP grants forbidden rate/wallet endpoint ${forbiddenHost}`);
  }
});

test('maintained source has no HTML injection, eval, or javascript: sinks', () => {
  assert.deepStrictEqual(
    MAINTAINED_SOURCE_PATHS.map((rel) => rel.split(path.sep).join('/')),
    ['social-main.js', 'social/index.html', 'social/app.js', 'social/core.js']
  );
  const scanned = [];
  for (const rel of MAINTAINED_SOURCE_PATHS) {
    const abs = path.join(repoRoot, rel);
    assert.ok(fs.existsSync(abs), `maintained source ${rel} is missing`);
    const text = fs.readFileSync(abs, 'utf8');
    assert.ok(text.trim(), `maintained source ${rel} is empty`);
    scanned.push(rel);
    for (const sink of FORBIDDEN_MAINTAINED_SINKS) {
      assert.ok(!text.includes(sink), `${rel} contains forbidden sink ${JSON.stringify(sink)}`);
    }
  }
  assert.deepStrictEqual(scanned, MAINTAINED_SOURCE_PATHS);
  assert.strictEqual(scanned.length, 4);
});

const WALLET_IPC_CHANNELS = [
  'wallet:accounts:list',
  'wallet:intent:begin',
  'wallet:intent:cancel',
  'wallet:payee-request:get',
  'wallet:snapshot:get',
];
const WALLET_PRELOAD_METHODS = [
  'beginIntent',
  'cancelIntent',
  'getPayeeRequest',
  'getSnapshot',
  'listAccounts',
  'subscribeSnapshot',
];

test('wallet preload retains exactly six frozen methods and no Electron confirmation action', () => {
  let exposed;
  const listeners = new Map();
  const electron = {
    contextBridge: {
      exposeInMainWorld(name, value) {
        assert.strictEqual(name, 'bitbookWallet');
        exposed = value;
      },
    },
    ipcRenderer: {
      invoke() { return Promise.resolve({ ok: true }); },
      on(channel, listener) { listeners.set(channel, listener); },
      removeListener(channel, listener) {
        if (listeners.get(channel) === listener) listeners.delete(channel);
      },
    },
  };
  const originalLoad = Module._load;
  Module._load = function loadPreloadMock(request, parent, isMain) {
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
  assert.deepStrictEqual(Object.keys(exposed).sort(), WALLET_PRELOAD_METHODS);
  assert.strictEqual(Object.isFrozen(exposed), true);
  for (const method of WALLET_PRELOAD_METHODS) {
    assert.strictEqual(typeof exposed[method], 'function');
    assert.strictEqual(Object.isFrozen(exposed[method]), true);
  }
  for (const forbidden of ['confirmIntent', 'confirm', 'unlock', 'exportBackup', 'sign', 'broadcast']) {
    assert.strictEqual(exposed[forbidden], undefined);
  }
});

test('wallet IPC registers only the exact renderer channel allowlist', () => {
  const ctx = boot();
  assert.deepStrictEqual(Object.keys(ctx.state.ipcHandlers).sort(), WALLET_IPC_CHANNELS);
  for (const forbidden of [
    'wallet:invoke', 'wallet:intent:confirm', 'wallet:account:unlock',
    'wallet:account:export-backup', 'wallet:account:create-software',
    'wallet:signer:sign', 'wallet:tx:broadcast', 'wallet:intent:broadcast',
  ]) assert.strictEqual(ctx.state.ipcHandlers[forbidden], undefined);
});

test('wallet IPC rejects non-main frames, non-local origins, malformed shapes, and oversize input', () => {
  const ctx = boot();
  const handler = ctx.state.ipcHandlers['wallet:intent:begin'];
  assert.strictEqual(typeof handler, 'function');
  const valid = { senderFrame: windowUnderTest().webContents.mainFrame, sender: windowUnderTest().webContents };
  const before = ctx.state.supervisorCalls.length;
  assert.throws(() => handler({ senderFrame: {}, sender: valid.sender }, {}));
  assert.throws(() => handler({ senderFrame: valid.senderFrame, sender: { getURL: () => 'https://evil.example' } }, {}));
  assert.throws(() => handler(valid, { unexpected: true }));
  assert.throws(() => handler(valid, { payment_request: { memo: 'x'.repeat(64 * 1024) } }));
  let getterCalls = 0;
  const nested = {};
  Object.defineProperty(nested, 'request_id', {
    enumerable: true,
    get() { getterCalls += 1; return '0'.repeat(32); },
  });
  assert.throws(() => handler(valid, { payment_request: nested }));
  assert.strictEqual(getterCalls, 0);
  assert.strictEqual(ctx.state.supervisorCalls.length, before);
});

test('wallet IPC valid calls map once to fixed supervisor methods with cloned parameters', async () => {
  const ctx = boot();
  const win = windowUnderTest();
  const event = { senderFrame: win.webContents.mainFrame, sender: win.webContents };
  const rows = [
    ['wallet:snapshot:get', 'status.get', undefined],
    ['wallet:accounts:list', 'account.list', undefined],
    ['wallet:intent:begin', 'intent.begin', { payment_request: { v: 1, request_id: '0'.repeat(32) } }],
    ['wallet:intent:cancel', 'intent.cancel', { intent_id: '1'.repeat(32) }],
    ['wallet:payee-request:get', 'receiver.fresh', {
      account_id: '2'.repeat(32), asset: 'ZEC', network: 'zec-testnet', request_id: '3'.repeat(32),
    }],
  ];
  for (const [channel, method, params] of rows) {
    const input = params === undefined ? undefined : JSON.parse(JSON.stringify(params));
    const inputBytes = input === undefined ? undefined : JSON.stringify(input);
    const output = await ctx.state.ipcHandlers[channel](event, input);
    const received = ctx.state.supervisorCalls[ctx.state.supervisorCalls.length - 1][1];
    const rawResult = ctx.state.supervisorResults[ctx.state.supervisorResults.length - 1];
    if (input) {
      assert.notStrictEqual(received, input);
      assert.deepStrictEqual(received, params);
      received.observer_mutation = true;
      assert.strictEqual(Object.prototype.hasOwnProperty.call(input, 'observer_mutation'), false);
      delete received.observer_mutation;
      if (method === 'intent.begin') {
        assert.notStrictEqual(received.payment_request, input.payment_request);
        const originalRequestId = received.payment_request.request_id;
        received.payment_request.request_id = 'f'.repeat(32);
        assert.strictEqual(JSON.stringify(input), inputBytes);
        received.payment_request.request_id = originalRequestId;
      }
    }
    assert.notStrictEqual(output, rawResult);
    assert.deepStrictEqual(output, { ok: true, value: params });
    if (params) {
      assert.notStrictEqual(output.value, rawResult.value);
      const rawBytes = JSON.stringify(rawResult);
      output.value.renderer_nested_mutation = true;
      assert.strictEqual(JSON.stringify(rawResult), rawBytes);
      delete output.value.renderer_nested_mutation;
      if (method === 'intent.begin') {
        assert.notStrictEqual(output.value.payment_request, rawResult.value.payment_request);
        output.value.payment_request.request_id = 'e'.repeat(32);
        assert.strictEqual(JSON.stringify(rawResult), rawBytes);
      }
    }
    output.renderer_mutation = true;
    assert.strictEqual(Object.prototype.hasOwnProperty.call(rawResult, 'renderer_mutation'), false);
  }
  assert.deepStrictEqual(ctx.state.supervisorCalls, rows.map(([, method, params]) => [method, params]));
});

test('wallet IPC delayed supervisor replies stay pending then clone fulfilled values for every channel', async () => {
  const deferreds = [];
  const observed = [];
  let currentDeferred = null;
  const rows = [
    ['wallet:snapshot:get', 'status.get', undefined, { v: 1, broker: 'degraded', accounts: [] }],
    ['wallet:accounts:list', 'account.list', undefined, {
      accounts: [{
        account_id: 'aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa',
        nested: { label: 'opaque-list-fixture', tags: ['alpha'] },
      }],
    }],
    ['wallet:intent:begin', 'intent.begin', { payment_request: { v: 1, request_id: '0'.repeat(32) } }, {
      intent_id: 'bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb',
      nested: { state: 'opaque-begin-fixture', items: [{ n: 1 }] },
    }],
    ['wallet:intent:cancel', 'intent.cancel', { intent_id: '1'.repeat(32) }, {
      intent_id: 'cccccccccccccccccccccccccccccccc',
      nested: { state: 'opaque-cancel-fixture', ok: true },
    }],
    ['wallet:payee-request:get', 'receiver.fresh', {
      account_id: '2'.repeat(32), asset: 'ZEC', network: 'zec-testnet', request_id: '3'.repeat(32),
    }, {
      request_id: 'dddddddddddddddddddddddddddddddd',
      nested: { uri: 'opaque-payee-fixture', asset: 'ZEC' },
    }],
  ];
  const ctx = loadIsolatedMaintainedMain({
    dispatch() {
      assert.ok(currentDeferred, 'supervisor dispatch had no deferred fixture');
      const deferred = currentDeferred;
      currentDeferred = null;
      deferreds.push(deferred);
      return deferred.promise;
    },
  });
  try {
    assert.deepStrictEqual(rows.map(([channel]) => channel).sort(), WALLET_IPC_CHANNELS);
    const event = trustedWalletEvent(ctx);
    for (const [channel, method, params, replyLiteral] of rows) {
      const handler = ctx.state.ipcHandlers[channel];
      assert.strictEqual(typeof handler, 'function');
      const deferred = createDeferred();
      currentDeferred = deferred;
      const input = params === undefined ? undefined : copyJson(params);
      const inputBytes = input === undefined ? undefined : JSON.stringify(input);
      const returned = handler(event, input);
      assert.strictEqual(currentDeferred, null, `${channel} did not dispatch once`);
      const resultState = { status: 'pending' };
      const resultObserved = Promise.resolve(returned).then(
        (value) => {
          resultState.status = 'fulfilled';
          resultState.value = value;
          return value;
        },
        (reason) => {
          resultState.status = 'rejected';
          resultState.reason = reason;
          throw reason;
        }
      );
      observed.push(resultObserved);
      assert.ok(returned && typeof returned.then === 'function', `${channel} did not return a thenable`);
      const received = ctx.state.supervisorCalls[ctx.state.supervisorCalls.length - 1][1];
      assert.strictEqual(ctx.state.supervisorCalls[ctx.state.supervisorCalls.length - 1][0], method);
      if (input) {
        assert.notStrictEqual(received, input);
        assert.deepStrictEqual(received, params);
        received.observer_mutation = true;
        assert.strictEqual(Object.prototype.hasOwnProperty.call(input, 'observer_mutation'), false);
        delete received.observer_mutation;
        assert.strictEqual(JSON.stringify(input), inputBytes);
        if (method === 'intent.begin') {
          assert.notStrictEqual(received.payment_request, input.payment_request);
          const originalRequestId = received.payment_request.request_id;
          received.payment_request.request_id = 'f'.repeat(32);
          assert.strictEqual(JSON.stringify(input), inputBytes);
          received.payment_request.request_id = originalRequestId;
        }
      } else {
        assert.strictEqual(received, undefined);
      }
      await withSettlementTimeout(waitEventLoopTurn(), `${channel} event-loop turn timed out`);
      assert.strictEqual(deferred.settled, false, `${channel} fixture settled before resolve`);
      assert.strictEqual(resultState.status, 'pending', `${channel} handler settled before the fixture resolved`);
      const reply = copyJson(replyLiteral);
      const expected = copyJson(replyLiteral);
      deferred.resolve(reply);
      const output = await withSettlementTimeout(
        resultObserved,
        `${channel} did not settle after the fixture resolved`
      );
      assert.strictEqual(resultState.status, 'fulfilled', `${channel} did not fulfill`);
      assert.deepStrictEqual(output, expected);
      assert.notStrictEqual(output, reply);
      const replyBytes = JSON.stringify(reply);
      output.renderer_mutation = true;
      assert.strictEqual(JSON.stringify(reply), replyBytes);
      delete output.renderer_mutation;
      reply.source_mutation = true;
      assert.deepStrictEqual(output, expected);
      delete reply.source_mutation;
      for (const key of Object.keys(expected)) {
        if (!expected[key] || typeof expected[key] !== 'object') continue;
        assert.notStrictEqual(output[key], reply[key], `${channel} aliased nested ${key}`);
        const nestedReplyBytes = JSON.stringify(reply);
        if (Array.isArray(output[key])) {
          output[key].push({ renderer_nested_mutation: true });
          assert.strictEqual(JSON.stringify(reply), nestedReplyBytes);
          output[key].pop();
          reply[key].push({ source_nested_mutation: true });
          assert.deepStrictEqual(output, expected);
          reply[key].pop();
          if (output[key][0] && typeof output[key][0] === 'object') {
            assert.notStrictEqual(output[key][0], reply[key][0], `${channel} aliased ${key}[0]`);
            if (output[key][0].nested) {
              assert.notStrictEqual(output[key][0].nested, reply[key][0].nested, `${channel} aliased ${key}[0].nested`);
              const deepBytes = JSON.stringify(reply);
              output[key][0].nested.renderer_nested_mutation = true;
              assert.strictEqual(JSON.stringify(reply), deepBytes);
              delete output[key][0].nested.renderer_nested_mutation;
              reply[key][0].nested.source_nested_mutation = true;
              assert.deepStrictEqual(output, expected);
              delete reply[key][0].nested.source_nested_mutation;
            }
          }
        } else {
          output[key].renderer_nested_mutation = true;
          assert.strictEqual(JSON.stringify(reply), nestedReplyBytes);
          delete output[key].renderer_nested_mutation;
          reply[key].source_nested_mutation = true;
          assert.deepStrictEqual(output, expected);
          delete reply[key].source_nested_mutation;
        }
      }
    }
    assert.deepStrictEqual(
      ctx.state.supervisorCalls,
      rows.map(([, method, params]) => [method, params])
    );
    assert.strictEqual(currentDeferred, null);
  } finally {
    await settleDeferreds(
      deferreds,
      observed,
      currentDeferred,
      'unsettled delayed wallet IPC reply fixture'
    );
  }
});

test('wallet IPC delayed supervisor rejections propagate original UNAVAILABLE and TIMEOUT errors', async () => {
  const cases = [
    ['wallet:snapshot:get', 'status.get', fixtureSupervisorError('UNAVAILABLE', 'fixture broker unavailable')],
    ['wallet:accounts:list', 'account.list', fixtureSupervisorError('TIMEOUT', 'fixture request timeout')],
  ];
  for (const [channel, method, error] of cases) {
    const deferreds = [];
    const observed = [];
    let currentDeferred = null;
    const ctx = loadIsolatedMaintainedMain({
      dispatch() {
        assert.ok(currentDeferred, 'supervisor dispatch had no deferred fixture');
        const deferred = currentDeferred;
        currentDeferred = null;
        deferreds.push(deferred);
        return deferred.promise;
      },
    });
    try {
      const handler = ctx.state.ipcHandlers[channel];
      assert.strictEqual(typeof handler, 'function');
      const deferred = createDeferred();
      currentDeferred = deferred;
      const returned = handler(trustedWalletEvent(ctx));
      assert.strictEqual(currentDeferred, null, `${channel} did not dispatch once`);
      const inputState = { status: 'pending' };
      const inputObserved = deferred.promise.then(
        (value) => {
          inputState.status = 'fulfilled';
          inputState.value = value;
          return value;
        },
        (reason) => {
          inputState.status = 'rejected';
          inputState.reason = reason;
          return reason;
        }
      );
      observed.push(inputObserved);
      const resultState = { status: 'pending' };
      const resultObserved = Promise.resolve(returned).then(
        (value) => {
          resultState.status = 'fulfilled';
          resultState.value = value;
          return value;
        },
        (reason) => {
          resultState.status = 'rejected';
          resultState.reason = reason;
          return reason;
        }
      );
      observed.push(resultObserved);
      assert.ok(returned && typeof returned.then === 'function', `${channel} did not return a thenable`);
      await withSettlementTimeout(waitEventLoopTurn(), `${channel} event-loop turn timed out`);
      assert.strictEqual(deferred.settled, false, `${channel} fixture settled before reject`);
      assert.strictEqual(inputState.status, 'pending', `${channel} input promise settled before the fixture rejected`);
      assert.strictEqual(resultState.status, 'pending', `${channel} handler settled before the fixture rejected`);
      deferred.reject(error);
      await withSettlementTimeout(
        Promise.all([inputObserved, resultObserved]),
        `${channel} rejection did not settle`
      );
      assert.strictEqual(inputState.status, 'rejected');
      assert.strictEqual(inputState.reason, error);
      assert.strictEqual(
        resultState.status,
        'rejected',
        `${channel} resolved ${JSON.stringify(resultState.value)} instead of rejecting`
      );
      assert.strictEqual(resultState.reason, error);
      assert.deepStrictEqual(ctx.state.supervisorCalls, [[method, undefined]]);
    } finally {
      await settleDeferreds(
        deferreds,
        observed,
        currentDeferred,
        'unsettled delayed wallet IPC rejection fixture'
      );
    }
  }
});

test('wallet IPC synchronous dispatch failure throws immediately', () => {
  const error = fixtureSupervisorError('SCHEMA', 'fixture synchronous dispatch failure');
  const ctx = loadIsolatedMaintainedMain({
    dispatch() {
      throw error;
    },
  });
  const handler = ctx.state.ipcHandlers['wallet:snapshot:get'];
  assert.strictEqual(typeof handler, 'function');
  let returned;
  let thrown;
  try {
    returned = handler(trustedWalletEvent(ctx));
  } catch (caught) {
    thrown = caught;
  }
  assert.strictEqual(thrown, error);
  assert.strictEqual(returned, undefined);
  assert.deepStrictEqual(ctx.state.supervisorCalls, [['status.get', undefined]]);
});

test('wallet snapshot subscription targets only the maintained main frame with sanitized cloned data', () => {
  const ctx = boot();
  const win = windowUnderTest();
  assert.strictEqual(ctx.state.supervisorSubscribers.length, 1);
  const source = JSON.parse(JSON.stringify(payFixture.valid_full_input));
  const sanitizerCallsBefore = ctx.state.sanitizerCalls.length;
  ctx.state.supervisorSubscribers[0](source);
  assert.strictEqual(ctx.state.sanitizerCalls.length, sanitizerCallsBefore + 1);
  assert.strictEqual(ctx.state.sanitizerCalls[sanitizerCallsBefore], source);
  assert.deepStrictEqual(ctx.state.rendererMessages, [[
    win.webContents.mainFrame,
    'wallet:snapshot:subscribe',
    payFixture.valid_full_expected,
  ]]);
  const delivered = ctx.state.rendererMessages[0][2];
  assert.notStrictEqual(delivered, source);
  assert.notStrictEqual(delivered.accounts, source.accounts);
  delivered.accounts[0].label = 'renderer mutation';
  delivered.intent_preview.state = 'failed';
  assert.strictEqual(source.accounts[0].label, 'Shielded ZEC');
  assert.strictEqual(source.intent_preview.state, 'awaiting_confirm');
  const serialized = JSON.stringify(ctx.state.rendererMessages);
  assert.ok(!serialized.includes('SNAPSHOT_SECRET_CANARY'));
  assert.ok(!serialized.includes('u1-forbidden'));
  assert.strictEqual(delivered.intent_preview.can_cancel, true);
  for (const key of ['confirm', 'actions', 'receiver', 'fee_atomic', 'memo', 'request_id', 'raw_transaction', 'pczt']) {
    assert.strictEqual(Object.prototype.hasOwnProperty.call(delivered.intent_preview, key), false);
  }
});

test('wallet Electron boundary exposes no confirmation, unlock, backup, sign, or broadcast surface', () => {
  const ctx = boot();
  const observable = JSON.stringify({ channels: Object.keys(ctx.state.ipcHandlers), windows: ctx.state.windows.length });
  for (const authority of ['confirm', 'unlock', 'backup', 'sign', 'broadcast']) {
    assert.ok(!observable.toLowerCase().includes(authority));
  }
  assert.strictEqual(ctx.state.windows.length, 1, 'Electron confirmation window is forbidden');
});

const WALLET_CONTRACT_MAINTAINED_PATHS = [
  path.join('wallet-contract', 'canonical.js'),
  path.join('wallet-contract', 'framing.js'),
  path.join('wallet-contract', 'model.js'),
  path.join('wallet-contract', 'state-machine.js'),
  path.join('wallet-contract', 'fakes.js'),
  path.join('wallet-contract', 'index.js'),
];

const WALLET_IMPORT_ALLOWLIST = new Set([
  'crypto',
  'node:crypto',
  'buffer',
  'node:buffer',
  './canonical',
  './canonical.js',
  './framing',
  './framing.js',
  './model',
  './model.js',
  './state-machine',
  './state-machine.js',
  './fakes',
  './fakes.js',
  './index',
  './index.js',
]);

function literalModuleSpecifier(expression) {
  const match = expression.trim().match(/^(['"])([^'"]+)\1$/);
  return match ? match[2] : null;
}

function assertWalletImportAllowlist(source, rel) {
  const callPattern = /\b(require|import)\s*\(([^)]*)\)/g;
  let match;
  while ((match = callPattern.exec(source)) !== null) {
    const specifier = literalModuleSpecifier(match[2]);
    if (!specifier || !WALLET_IMPORT_ALLOWLIST.has(specifier)) {
      throw new PolicyError(`${rel} contains non-allowlisted or computed ${match[1]} module load`);
    }
  }

  const staticPattern = /\bimport\s+(?!\s*\()([^;\n]+)/g;
  while ((match = staticPattern.exec(source)) !== null) {
    const clause = match[1].trim();
    const direct = clause.match(/^(['"])([^'"]+)\1$/);
    const from = clause.match(/\bfrom\s+(['"])([^'"]+)\1$/);
    const specifier = direct ? direct[2] : from ? from[2] : null;
    if (!specifier || !WALLET_IMPORT_ALLOWLIST.has(specifier)) {
      throw new PolicyError(`${rel} contains non-allowlisted static import`);
    }
  }

  if (/\bfetch\s*\(/.test(source) || /\b(?:new\s+)?WebSocket\s*\(/.test(source)) {
    throw new PolicyError(`${rel} contains a forbidden network capability`);
  }
}

test('wallet reference contract is maintained source and retains an offline inert boundary', () => {
  assert.deepStrictEqual(
    WALLET_CONTRACT_MAINTAINED_PATHS.map((rel) => rel.split(path.sep).join('/')),
    [
      'wallet-contract/canonical.js',
      'wallet-contract/framing.js',
      'wallet-contract/model.js',
      'wallet-contract/state-machine.js',
      'wallet-contract/fakes.js',
      'wallet-contract/index.js',
    ]
  );
  for (const rel of WALLET_CONTRACT_MAINTAINED_PATHS) {
    const abs = path.join(repoRoot, rel);
    assert.ok(fs.existsSync(abs), `maintained wallet reference source ${rel} is missing`);
    const source = fs.readFileSync(abs, 'utf8');
    assert.ok(source.trim(), `maintained wallet reference source ${rel} is empty`);
    for (const sink of FORBIDDEN_MAINTAINED_SINKS) {
      assert.ok(!source.includes(sink), `${rel} contains forbidden maintained-source sink ${JSON.stringify(sink)}`);
    }
    assertWalletImportAllowlist(source, rel);
  }

  for (const specifier of WALLET_IMPORT_ALLOWLIST) {
    for (const source of [
      `require('${specifier}')`,
      `import '${specifier}'`,
      `import('${specifier}')`,
    ]) {
      assert.doesNotThrow(() => assertWalletImportAllowlist(source, 'wallet-contract/synthetic.js'));
    }
  }
  for (const specifier of [
    '../canonical',
    '../wallet-contract/canonical',
    '/wallet-contract/canonical.js',
    'C:/wallet-contract/canonical.js',
    './other',
    'left-pad',
    'path',
    'node:fs',
    'child_process',
    'electron',
  ]) {
    for (const source of [
      `require('${specifier}')`,
      `import '${specifier}'`,
      `import('${specifier}')`,
    ]) {
      assert.throws(
        () => assertWalletImportAllowlist(source, 'wallet-contract/synthetic.js'),
        PolicyError
      );
    }
  }
  for (const source of [
    "const name = 'crypto'; require(name)",
    "require('child_' + 'process')",
    'require(`crypto`)',
    "const name = 'crypto'; import(name)",
    "import('child_' + 'process')",
    'import(`crypto`)',
    "fetch('https://example.invalid')",
    "new WebSocket('wss://example.invalid')",
  ]) {
    assert.throws(
      () => assertWalletImportAllowlist(source, 'wallet-contract/synthetic.js'),
      PolicyError
    );
  }
});

test('after ready, initial before-quit is prevented until deferred shutdown fulfills and resumes quit', async () => {
  const deferred = createDeferred();
  const deferreds = [deferred];
  const observed = [];
  const shutdownState = { status: 'pending' };
  const shutdownObserved = deferred.promise.then(
    (value) => {
      shutdownState.status = 'fulfilled';
      shutdownState.value = value;
      return value;
    },
    (reason) => {
      shutdownState.status = 'rejected';
      shutdownState.reason = reason;
      throw reason;
    }
  );
  observed.push(shutdownObserved);
  const ctx = loadIsolatedMaintainedMain({
    shutdown() {
      return deferred.promise;
    },
  });
  const restoreExit = watchForceExits(ctx.state);
  const unhandled = watchUnhandledRejections();
  try {
    requireAppHandlers(ctx, 'before-quit');
    const first = ctx.emitBeforeQuit();
    assert.strictEqual(first.defaultPrevented, true, 'initial before-quit was not prevented');
    assert.strictEqual(ctx.state.shutdownCalls.length, 1);
    assert.strictEqual(ctx.state.quitCalls, 0);
    assert.strictEqual(ctx.state.errorBoxes.length, 0);

    const repeat = ctx.emitBeforeQuit();
    assert.strictEqual(repeat.defaultPrevented, true, 'repeat before-quit was not prevented');
    assert.strictEqual(ctx.state.shutdownCalls.length, 1);
    assert.strictEqual(ctx.state.quitCalls, 0);

    await withSettlementTimeout(waitEventLoopTurn(), 'pending quit event-loop turn timed out');
    assert.strictEqual(deferred.settled, false);
    assert.strictEqual(shutdownState.status, 'pending');
    assert.strictEqual(ctx.state.quitCalls, 0, 'application quit occurred while shutdown was pending');
    assert.strictEqual(ctx.state.errorBoxes.length, 0);
    assert.strictEqual(ctx.state.forceExitCalls.length, 0);

    deferred.resolve();
    await withSettlementTimeout(shutdownObserved, 'shutdown did not fulfill');
    await withSettlementTimeout(waitEventLoopTurn(), 'resumed quit event-loop turn timed out');
    assert.strictEqual(shutdownState.status, 'fulfilled');

    const types = lifecycleTypes(ctx);
    const fulfilledAt = types.indexOf('shutdown-fulfilled');
    const quitAt = types.indexOf('app.quit');
    assert.ok(fulfilledAt !== -1, 'shutdown fulfillment was not recorded');
    assert.ok(quitAt !== -1, 'resumed app.quit was not recorded');
    assert.ok(fulfilledAt < quitAt, 'resumed app.quit preceded shutdown fulfillment');

    assert.strictEqual(ctx.state.quitCalls, 1);
    assert.strictEqual(ctx.state.shutdownCalls.length, 1);
    assert.strictEqual(ctx.state.errorBoxes.length, 0);
    assert.strictEqual(ctx.state.beforeQuitEvents.length, 3);
    assert.strictEqual(ctx.state.beforeQuitEvents[0].defaultPrevented, true);
    assert.strictEqual(ctx.state.beforeQuitEvents[1].defaultPrevented, true);
    assert.strictEqual(ctx.state.beforeQuitEvents[2].defaultPrevented, false);
    assert.strictEqual(ctx.state.beforeQuitEvents[2].viaAppQuit, true);
    assert.strictEqual(unhandled.seen.length, 0);
    assertUnusedForcePaths(ctx);
  } finally {
    await cleanupQuitHooks({
      deferreds,
      observed,
      currentDeferred: deferred,
      restoreExit,
      unhandled,
      ctx,
      message: 'unsettled after-ready quit shutdown fixture',
    });
  }
});

test('before ready, fulfilled no-child shutdown allows one resumed quit without windows', async () => {
  const observed = [];
  const shutdownState = { status: 'pending' };
  const ctx = loadIsolatedMaintainedMain({
    emitReady: false,
    shutdown() {
      const promise = Promise.resolve();
      const shutdownObserved = promise.then(
        (value) => {
          shutdownState.status = 'fulfilled';
          shutdownState.value = value;
          return value;
        },
        (reason) => {
          shutdownState.status = 'rejected';
          shutdownState.reason = reason;
          throw reason;
        }
      );
      observed.push(shutdownObserved);
      return promise;
    },
  });
  const restoreExit = watchForceExits(ctx.state);
  const unhandled = watchUnhandledRejections();
  try {
    assert.strictEqual(ctx.state.readyEmitted, false);
    assert.strictEqual(ctx.state.windows.length, 0);
    assert.deepStrictEqual(Object.keys(ctx.state.ipcHandlers), []);
    assert.strictEqual(ctx.state.permissionRequestHandler, null);
    requireAppHandlers(ctx, 'before-quit');

    const first = ctx.emitBeforeQuit();
    assert.strictEqual(first.defaultPrevented, true, 'initial before-quit was not prevented');
    assert.strictEqual(ctx.state.shutdownCalls.length, 1);
    assert.strictEqual(ctx.state.quitCalls, 0);
    assert.strictEqual(ctx.state.windows.length, 0);
    assert.strictEqual(ctx.state.readyEmitted, false);

    await withSettlementTimeout(Promise.all(observed), 'no-child shutdown did not settle');
    await withSettlementTimeout(waitEventLoopTurn(), 'no-child resumed quit event-loop turn timed out');
    assert.strictEqual(shutdownState.status, 'fulfilled');
    assert.strictEqual(ctx.state.quitCalls, 1);
    assert.strictEqual(ctx.state.shutdownCalls.length, 1);
    assert.strictEqual(ctx.state.errorBoxes.length, 0);
    assert.strictEqual(ctx.state.beforeQuitEvents.length, 2);
    assert.strictEqual(ctx.state.beforeQuitEvents[0].defaultPrevented, true);
    assert.strictEqual(ctx.state.beforeQuitEvents[1].defaultPrevented, false);
    assert.strictEqual(ctx.state.beforeQuitEvents[1].viaAppQuit, true);
    assert.strictEqual(ctx.state.windows.length, 0);
    assert.strictEqual(ctx.state.readyEmitted, false);
    assert.strictEqual(unhandled.seen.length, 0);
    assertUnusedForcePaths(ctx);
  } finally {
    await cleanupQuitHooks({
      observed,
      restoreExit,
      unhandled,
      ctx,
      message: 'unsettled before-ready quit shutdown fixture',
    });
  }
});

test('shutdown-emitted nested before-quit stays prevented until the original shutdown fulfills', async () => {
  const deferred = createDeferred();
  const deferreds = [deferred];
  const observed = [];
  const shutdownState = { status: 'pending' };
  const shutdownObserved = deferred.promise.then(
    (value) => {
      shutdownState.status = 'fulfilled';
      shutdownState.value = value;
      return value;
    },
    (reason) => {
      shutdownState.status = 'rejected';
      shutdownState.reason = reason;
      throw reason;
    }
  );
  observed.push(shutdownObserved);
  let nestedEmitted = false;
  let ctx;
  ctx = loadIsolatedMaintainedMain({
    shutdown() {
      nestedEmitted = true;
      const nested = ctx.emitBeforeQuit();
      assert.strictEqual(nested.defaultPrevented, true, 'nested before-quit was not prevented');
      return deferred.promise;
    },
  });
  const restoreExit = watchForceExits(ctx.state);
  const unhandled = watchUnhandledRejections();
  try {
    requireAppHandlers(ctx, 'before-quit');
    const first = ctx.emitBeforeQuit();
    assert.strictEqual(nestedEmitted, true, 'fixture did not emit nested before-quit');
    assert.strictEqual(first.defaultPrevented, true, 'initial before-quit was not prevented');
    assert.strictEqual(ctx.state.shutdownCalls.length, 1);
    assert.strictEqual(ctx.state.quitCalls, 0);
    assert.strictEqual(ctx.state.beforeQuitEvents.length, 2);
    assert.strictEqual(ctx.state.beforeQuitEvents[0].defaultPrevented, true);
    assert.strictEqual(ctx.state.beforeQuitEvents[1].defaultPrevented, true);
    assert.strictEqual(ctx.state.beforeQuitEvents[1].viaAppQuit, false);

    await withSettlementTimeout(waitEventLoopTurn(), 'nested quit event-loop turn timed out');
    assert.strictEqual(deferred.settled, false);
    assert.strictEqual(shutdownState.status, 'pending');
    assert.strictEqual(ctx.state.quitCalls, 0);
    assert.strictEqual(ctx.state.shutdownCalls.length, 1);

    deferred.resolve();
    await withSettlementTimeout(shutdownObserved, 'nested-quit shutdown did not fulfill');
    await withSettlementTimeout(waitEventLoopTurn(), 'nested resumed quit event-loop turn timed out');
    assert.strictEqual(shutdownState.status, 'fulfilled');
    assert.strictEqual(ctx.state.quitCalls, 1);
    assert.strictEqual(ctx.state.shutdownCalls.length, 1);
    assert.strictEqual(ctx.state.errorBoxes.length, 0);
    assert.strictEqual(ctx.state.beforeQuitEvents.length, 3);
    assert.strictEqual(ctx.state.beforeQuitEvents[2].defaultPrevented, false);
    assert.strictEqual(ctx.state.beforeQuitEvents[2].viaAppQuit, true);
    assert.strictEqual(unhandled.seen.length, 0);
    assertUnusedForcePaths(ctx);
  } finally {
    await cleanupQuitHooks({
      deferreds,
      observed,
      currentDeferred: deferred,
      restoreExit,
      unhandled,
      ctx,
      message: 'unsettled nested before-quit shutdown fixture',
    });
  }
});

test('rejected shutdown keeps quit blocked and shows one fixed error box for TIMEOUT and UNAVAILABLE', async () => {
  const rows = [
    [
      'TIMEOUT',
      'WAL011_QUIT_CANARY_TIMEOUT_MSG /secret/wallet-broker.sock',
      'WAL011_QUIT_CANARY_TIMEOUT_STACK',
    ],
    [
      'UNAVAILABLE',
      'WAL011_QUIT_CANARY_UNAVAILABLE_MSG /var/lib/bitbook/private',
      'WAL011_QUIT_CANARY_UNAVAILABLE_STACK',
    ],
  ];
  for (const [code, messageCanary, stackCanary] of rows) {
    const deferred = createDeferred();
    const deferreds = [deferred];
    const observed = [];
    const shutdownState = { status: 'pending' };
    const shutdownObserved = deferred.promise.then(
      (value) => {
        shutdownState.status = 'fulfilled';
        shutdownState.value = value;
        return value;
      },
      (reason) => {
        shutdownState.status = 'rejected';
        shutdownState.reason = reason;
        return reason;
      }
    );
    observed.push(shutdownObserved);
    const error = fixtureSupervisorError(code, messageCanary);
    error.stack = `${stackCanary}\n${error.stack || ''}`;
    const handlerReturnState = { status: 'absent' };
    const ctx = loadIsolatedMaintainedMain({
      shutdown() {
        return deferred.promise;
      },
    });
    const restoreExit = watchForceExits(ctx.state);
    const unhandled = watchUnhandledRejections();
    try {
      requireAppHandlers(ctx, 'before-quit');
      let thrown;
      try {
        ctx.emitBeforeQuit();
      } catch (caught) {
        thrown = caught;
      }
      assert.strictEqual(thrown, undefined, `${code} shutdown rejection escaped the before-quit handler`);
      assert.strictEqual(ctx.state.beforeQuitEvents[0].defaultPrevented, true);
      assert.strictEqual(ctx.state.shutdownCalls.length, 1);
      assert.strictEqual(ctx.state.quitCalls, 0);
      assert.strictEqual(ctx.state.errorBoxes.length, 0);
      const returned = ctx.state.beforeQuitEvents[0].returned;
      if (returned && typeof returned.then === 'function') {
        handlerReturnState.status = 'pending';
        observed.push(Promise.resolve(returned).then(
          (value) => {
            handlerReturnState.status = 'fulfilled';
            return value;
          },
          (reason) => {
            handlerReturnState.status = 'rejected';
            return reason;
          }
        ));
      }

      await withSettlementTimeout(waitEventLoopTurn(), `${code} pending quit event-loop turn timed out`);
      assert.strictEqual(deferred.settled, false);
      assert.strictEqual(shutdownState.status, 'pending');
      assert.strictEqual(ctx.state.quitCalls, 0);

      deferred.reject(error);
      await withSettlementTimeout(shutdownObserved, `${code} shutdown rejection did not settle`);
      await withSettlementTimeout(waitEventLoopTurn(), `${code} rejected quit event-loop turn timed out`);
      assert.strictEqual(shutdownState.status, 'rejected');
      assert.strictEqual(shutdownState.reason, error);
      assert.notStrictEqual(
        handlerReturnState.status,
        'rejected',
        `${code} before-quit handler returned a rejected Promise`
      );
      assert.strictEqual(ctx.state.quitCalls, 0, `${code} resumed quit after shutdown rejection`);
      assert.strictEqual(ctx.state.shutdownCalls.length, 1);
      assert.strictEqual(ctx.state.errorBoxes.length, 1);
      assertFixedQuitErrorBox(ctx.state.errorBoxes[0]);
      assertNoPrivateQuitCanary(ctx, messageCanary);
      assertNoPrivateQuitCanary(ctx, stackCanary);
      assert.strictEqual(unhandled.seen.length, 0);

      const repeat = ctx.emitBeforeQuit();
      assert.strictEqual(repeat.defaultPrevented, true, `${code} repeat before-quit was not prevented`);
      assert.strictEqual(ctx.state.shutdownCalls.length, 1);
      assert.strictEqual(ctx.state.errorBoxes.length, 1);
      assert.strictEqual(ctx.state.quitCalls, 0);
      assert.strictEqual(unhandled.seen.length, 0);
      assertUnusedForcePaths(ctx);
    } finally {
      await cleanupQuitHooks({
        deferreds,
        observed,
        currentDeferred: deferred,
        restoreExit,
        unhandled,
        ctx,
        message: `unsettled ${code} quit shutdown fixture`,
      });
    }
  }
});

test('synchronous shutdown throw is contained and keeps quit blocked with one fixed error box', async () => {
  const observed = [];
  const messageCanary = 'WAL011_QUIT_CANARY_THROW_MSG /tmp/bitbook-broker';
  const stackCanary = 'WAL011_QUIT_CANARY_THROW_STACK';
  const error = fixtureSupervisorError('UNAVAILABLE', messageCanary);
  error.stack = `${stackCanary}\n${error.stack || ''}`;
  const handlerReturnState = { status: 'absent' };
  const ctx = loadIsolatedMaintainedMain({
    shutdown() {
      throw error;
    },
  });
  const restoreExit = watchForceExits(ctx.state);
  const unhandled = watchUnhandledRejections();
  try {
    requireAppHandlers(ctx, 'before-quit');
    let thrown;
    try {
      ctx.emitBeforeQuit();
    } catch (caught) {
      thrown = caught;
    }
    assert.strictEqual(thrown, undefined, 'synchronous shutdown throw escaped the before-quit handler');
    assert.strictEqual(ctx.state.beforeQuitEvents[0].defaultPrevented, true);
    assert.strictEqual(ctx.state.shutdownCalls.length, 1);
    assert.strictEqual(ctx.state.quitCalls, 0);
    const returned = ctx.state.beforeQuitEvents[0].returned;
    if (returned && typeof returned.then === 'function') {
      handlerReturnState.status = 'pending';
      observed.push(Promise.resolve(returned).then(
        (value) => {
          handlerReturnState.status = 'fulfilled';
          return value;
        },
        (reason) => {
          handlerReturnState.status = 'rejected';
          return reason;
        }
      ));
    }

    await withSettlementTimeout(waitEventLoopTurn(), 'synchronous throw quit event-loop turn timed out');
    assert.notStrictEqual(
      handlerReturnState.status,
      'rejected',
      'before-quit handler returned a rejected Promise after a shutdown throw'
    );
    assert.strictEqual(ctx.state.quitCalls, 0);
    assert.strictEqual(ctx.state.shutdownCalls.length, 1);
    assert.strictEqual(ctx.state.errorBoxes.length, 1);
    assertFixedQuitErrorBox(ctx.state.errorBoxes[0]);
    assertNoPrivateQuitCanary(ctx, messageCanary);
    assertNoPrivateQuitCanary(ctx, stackCanary);
    assert.strictEqual(unhandled.seen.length, 0);

    const repeat = ctx.emitBeforeQuit();
    assert.strictEqual(repeat.defaultPrevented, true, 'repeat before-quit was not prevented after throw');
    assert.strictEqual(ctx.state.shutdownCalls.length, 1);
    assert.strictEqual(ctx.state.errorBoxes.length, 1);
    assert.strictEqual(ctx.state.quitCalls, 0);
    assert.strictEqual(unhandled.seen.length, 0);
    assertUnusedForcePaths(ctx);
  } finally {
    await cleanupQuitHooks({
      observed,
      restoreExit,
      unhandled,
      ctx,
      message: 'unsettled synchronous shutdown throw fixture',
    });
  }
});

test('rejected shutdown plus throwing error box stays contained without resumed quit', async () => {
  const deferred = createDeferred();
  const deferreds = [deferred];
  const observed = [];
  const shutdownState = { status: 'pending' };
  const shutdownObserved = deferred.promise.then(
    (value) => {
      shutdownState.status = 'fulfilled';
      shutdownState.value = value;
      return value;
    },
    (reason) => {
      shutdownState.status = 'rejected';
      shutdownState.reason = reason;
      return reason;
    }
  );
  observed.push(shutdownObserved);
  const messageCanary = 'WAL011_QUIT_CANARY_COMPOUND_MSG child pid 4242';
  const stackCanary = 'WAL011_QUIT_CANARY_COMPOUND_STACK';
  const error = fixtureSupervisorError('TIMEOUT', messageCanary);
  error.stack = `${stackCanary}\n${error.stack || ''}`;
  const dialogError = new Error('fixture showErrorBox failure');
  const dialogNested = {
    returned: false,
    defaultPrevented: null,
    viaAppQuit: null,
  };
  let dialogThrowMarkerCount = 0;
  let ctx;
  ctx = loadIsolatedMaintainedMain({
    shutdown() {
      return deferred.promise;
    },
    showErrorBox() {
      const nested = ctx.emitBeforeQuit();
      const record = ctx.state.beforeQuitEvents[ctx.state.beforeQuitEvents.length - 1];
      dialogNested.returned = true;
      dialogNested.defaultPrevented = nested.defaultPrevented;
      dialogNested.viaAppQuit = record ? record.viaAppQuit : null;
      dialogThrowMarkerCount += 1;
      throw dialogError;
    },
  });
  const restoreExit = watchForceExits(ctx.state);
  const unhandled = watchUnhandledRejections();
  try {
    requireAppHandlers(ctx, 'before-quit');
    let thrown;
    try {
      ctx.emitBeforeQuit();
    } catch (caught) {
      thrown = caught;
    }
    assert.strictEqual(thrown, undefined, 'compound quit failure escaped the before-quit handler');
    assert.strictEqual(ctx.state.beforeQuitEvents[0].defaultPrevented, true);
    assert.strictEqual(ctx.state.shutdownCalls.length, 1);
    assert.strictEqual(ctx.state.quitCalls, 0);
    assert.strictEqual(ctx.state.errorBoxes.length, 0);

    await withSettlementTimeout(waitEventLoopTurn(), 'compound quit event-loop turn timed out');
    assert.strictEqual(shutdownState.status, 'pending');
    assert.strictEqual(ctx.state.quitCalls, 0);

    deferred.reject(error);
    await withSettlementTimeout(shutdownObserved, 'compound shutdown rejection did not settle');
    await withSettlementTimeout(waitEventLoopTurn(), 'compound dialog event-loop turn timed out');
    assert.strictEqual(dialogNested.returned, true, 'dialog nested before-quit did not return');
    assert.strictEqual(dialogNested.defaultPrevented, true, 'failed-state nested before-quit was not prevented');
    assert.strictEqual(dialogNested.viaAppQuit, false);
    assert.strictEqual(dialogThrowMarkerCount, 1, 'intended dialog throw marker was not reached exactly once');
    assert.strictEqual(ctx.state.beforeQuitEvents.length, 2);
    assert.strictEqual(ctx.state.beforeQuitEvents[1].defaultPrevented, true);
    assert.strictEqual(ctx.state.beforeQuitEvents[1].viaAppQuit, false);
    assert.strictEqual(ctx.state.errorBoxes.length, 1);
    assertFixedQuitErrorBox(ctx.state.errorBoxes[0]);
    assertNoPrivateQuitCanary(ctx, messageCanary);
    assertNoPrivateQuitCanary(ctx, stackCanary);
    assert.strictEqual(ctx.state.quitCalls, 0);
    assert.strictEqual(ctx.state.shutdownCalls.length, 1);
    assert.strictEqual(ctx.state.forceExitCalls.length, 0);
    assert.strictEqual(unhandled.seen.length, 0);
    assert.strictEqual(thrown, undefined);

    const repeat = ctx.emitBeforeQuit();
    assert.strictEqual(repeat.defaultPrevented, true, 'repeat before-quit was not prevented after compound failure');
    assert.strictEqual(ctx.state.shutdownCalls.length, 1);
    assert.strictEqual(ctx.state.errorBoxes.length, 1);
    assert.strictEqual(ctx.state.quitCalls, 0);
    assert.strictEqual(ctx.state.beforeQuitEvents.length, 3);
    assert.strictEqual(ctx.state.beforeQuitEvents[2].defaultPrevented, true);
    assert.strictEqual(unhandled.seen.length, 0);
    assertUnusedForcePaths(ctx);
  } finally {
    await cleanupQuitHooks({
      deferreds,
      observed,
      currentDeferred: deferred,
      restoreExit,
      unhandled,
      ctx,
      message: 'unsettled compound quit failure fixture',
    });
  }
});

test('window-all-closed uses the host platform branch and the same before-quit gate', async () => {
  const deferred = createDeferred();
  const deferreds = [deferred];
  const observed = [];
  const shutdownState = { status: 'pending' };
  const shutdownObserved = deferred.promise.then(
    (value) => {
      shutdownState.status = 'fulfilled';
      shutdownState.value = value;
      return value;
    },
    (reason) => {
      shutdownState.status = 'rejected';
      shutdownState.reason = reason;
      throw reason;
    }
  );
  observed.push(shutdownObserved);
  const ctx = loadIsolatedMaintainedMain({
    shutdown() {
      return deferred.promise;
    },
  });
  const restoreExit = watchForceExits(ctx.state);
  const unhandled = watchUnhandledRejections();
  try {
    requireAppHandlers(ctx, 'window-all-closed');
    requireAppHandlers(ctx, 'before-quit');

    if (process.platform === 'darwin') {
      ctx.emitApp('window-all-closed');
      await withSettlementTimeout(waitEventLoopTurn(), 'macOS window-all-closed event-loop turn timed out');
      assert.strictEqual(ctx.state.quitCalls, 0, 'macOS window-all-closed quit the app');
      assert.strictEqual(ctx.state.shutdownCalls.length, 0, 'macOS window-all-closed invoked shutdown');
      assert.strictEqual(ctx.state.beforeQuitEvents.length, 0);

      const first = ctx.emitBeforeQuit();
      assert.strictEqual(first.defaultPrevented, true, 'explicit macOS before-quit was not prevented');
      assert.strictEqual(ctx.state.shutdownCalls.length, 1);
      await withSettlementTimeout(waitEventLoopTurn(), 'macOS explicit quit event-loop turn timed out');
      assert.strictEqual(deferred.settled, false);
      assert.strictEqual(ctx.state.quitCalls, 0);

      deferred.resolve();
      await withSettlementTimeout(shutdownObserved, 'macOS explicit shutdown did not fulfill');
      await withSettlementTimeout(waitEventLoopTurn(), 'macOS resumed quit event-loop turn timed out');
      assert.strictEqual(shutdownState.status, 'fulfilled');
      assert.strictEqual(ctx.state.quitCalls, 1);
      assert.strictEqual(ctx.state.shutdownCalls.length, 1);
      assert.strictEqual(ctx.state.errorBoxes.length, 0);
      assert.strictEqual(ctx.state.beforeQuitEvents[ctx.state.beforeQuitEvents.length - 1].defaultPrevented, false);
      assert.strictEqual(ctx.state.beforeQuitEvents[ctx.state.beforeQuitEvents.length - 1].viaAppQuit, true);
    } else {
      ctx.emitApp('window-all-closed');
      assert.strictEqual(ctx.state.quitCalls, 1);
      assert.strictEqual(ctx.state.beforeQuitEvents.length, 1);
      assert.strictEqual(ctx.state.beforeQuitEvents[0].defaultPrevented, true);
      assert.strictEqual(ctx.state.beforeQuitEvents[0].viaAppQuit, true);
      assert.strictEqual(ctx.state.shutdownCalls.length, 1);

      await withSettlementTimeout(waitEventLoopTurn(), 'window-all-closed pending event-loop turn timed out');
      assert.strictEqual(deferred.settled, false);
      assert.strictEqual(shutdownState.status, 'pending');
      assert.strictEqual(ctx.state.quitCalls, 1, 'additional quit occurred while shutdown was pending');

      deferred.resolve();
      await withSettlementTimeout(shutdownObserved, 'window-all-closed shutdown did not fulfill');
      await withSettlementTimeout(waitEventLoopTurn(), 'window-all-closed resumed quit event-loop turn timed out');
      assert.strictEqual(shutdownState.status, 'fulfilled');
      assert.strictEqual(ctx.state.quitCalls, 2);
      assert.strictEqual(ctx.state.shutdownCalls.length, 1);
      assert.strictEqual(ctx.state.errorBoxes.length, 0);
      assert.strictEqual(ctx.state.beforeQuitEvents.length, 2);
      assert.strictEqual(ctx.state.beforeQuitEvents[1].defaultPrevented, false);
      assert.strictEqual(ctx.state.beforeQuitEvents[1].viaAppQuit, true);

      const types = lifecycleTypes(ctx);
      const fulfilledAt = types.indexOf('shutdown-fulfilled');
      const quitIndexes = [];
      for (let index = 0; index < types.length; index += 1) {
        if (types[index] === 'app.quit') quitIndexes.push(index);
      }
      assert.strictEqual(quitIndexes.length, 2);
      assert.ok(fulfilledAt !== -1 && fulfilledAt > quitIndexes[0] && fulfilledAt < quitIndexes[1]);
    }

    assert.strictEqual(unhandled.seen.length, 0);
    assertUnusedForcePaths(ctx);
  } finally {
    await cleanupQuitHooks({
      deferreds,
      observed,
      currentDeferred: deferred,
      restoreExit,
      unhandled,
      ctx,
      message: 'unsettled window-all-closed quit fixture',
    });
  }
});

async function run() {
  let failed = 0;
  for (const { name, fn } of tests) {
    try {
      await fn();
      process.stdout.write(`ok ${name}\n`);
    } catch (err) {
      failed += 1;
      process.stderr.write(`not ok ${name}\n${err && err.stack ? err.stack : err}\n`);
    }
  }
  if (failed) {
    process.stderr.write(`${failed} electron security test(s) failed\n`);
    process.exit(1);
  }
  process.stdout.write(`BitBook electron security tests passed (${tests.length}).\n`);
}

if (require.main === module) {
  run();
}

module.exports = {
  PolicyError,
  tests,
  parseCsp,
  extractCsp,
};
