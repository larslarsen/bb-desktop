'use strict';

const assert = require('assert');
const fs = require('fs');
const Module = require('module');
const path = require('path');

const repoRoot = path.resolve(__dirname, '..');
const mainPath = path.join(repoRoot, 'social-main.js');
const expectedPage = path.join(repoRoot, 'social', 'index.html');
const packagePath = path.join(repoRoot, 'package.json');

const FORBIDDEN_ELECTRON_APIS = [
  'ipcMain',
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

function createElectronMock() {
  const accessed = new Set();
  const state = {
    enableSandboxCalls: 0,
    windows: [],
    permissionRequestHandler: null,
    permissionCheckHandler: null,
    menuSet: [],
    openExternalCalls: [],
    appHandlers: Object.create(null),
    quitCalls: 0,
  };

  class WebContents {
    constructor() {
      this.handlers = Object.create(null);
      this.windowOpenHandler = null;
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

  const app = {
    enableSandbox() {
      state.enableSandboxCalls += 1;
    },
    on(event, handler) {
      if (!state.appHandlers[event]) {
        state.appHandlers[event] = [];
      }
      state.appHandlers[event].push(handler);
    },
    quit() {
      state.quitCalls += 1;
    },
  };

  const Menu = {
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

  const target = { app, BrowserWindow, Menu, session, shell };

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
  };
}

let runtime;

function loadMaintainedMain() {
  assert.ok(fs.existsSync(mainPath), 'maintained Electron entry social-main.js is missing');
  const mock = createElectronMock();
  const originalLoad = Module._load;
  Module._load = function loadWithElectronMock(request, parent, isMain) {
    if (request === 'electron') {
      return mock.electron;
    }
    return originalLoad.call(this, request, parent, isMain);
  };
  try {
    delete require.cache[mainPath];
    require(mainPath);
  } finally {
    Module._load = originalLoad;
  }
  mock.emitApp('ready');
  return mock;
}

function boot() {
  if (!runtime) {
    runtime = loadMaintainedMain();
  }
  return runtime;
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
  assert.ok(prefs.preload == null, 'preload script is not authorized');
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

test('no preload, webview, IPC bridge, or remote Electron API is introduced', () => {
  const ctx = boot();
  for (const api of FORBIDDEN_ELECTRON_APIS) {
    assert.ok(!ctx.accessed.has(api), `maintained main accessed forbidden Electron API ${api}`);
  }
  const prefs = windowUnderTest().options.webPreferences;
  assert.ok(!Object.prototype.hasOwnProperty.call(prefs, 'preload'));
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

function run() {
  let failed = 0;
  for (const { name, fn } of tests) {
    try {
      fn();
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
