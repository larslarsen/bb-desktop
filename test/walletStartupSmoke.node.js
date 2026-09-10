'use strict';

const assert = require('assert');
const crypto = require('crypto');
const fs = require('fs');
const os = require('os');
const path = require('path');
const Module = require('module');
const childProcess = require('child_process');

const repoRoot = path.resolve(__dirname, '..');
const mainPath = path.join(repoRoot, 'social-main.js');
const expectedPage = path.join(repoRoot, 'social', 'index.html');
const RESOURCES_PATH = path.join(repoRoot, 'wallet-broker', 'target', 'app-resources');
const BROKER_BASENAME = process.platform === 'win32'
  ? 'bitbook-wallet-broker.exe'
  : 'bitbook-wallet-broker';
const BROKER_DIR = path.join(RESOURCES_PATH, 'wallet-broker');
const BROKER_PATH = path.join(BROKER_DIR, BROKER_BASENAME);
const MANIFEST_PATH = path.join(BROKER_DIR, 'manifest.json');
const EXPECTED_SNAPSHOT = Object.freeze({
  v: 1,
  broker: 'degraded',
  accounts: Object.freeze([]),
  intent_preview: null,
});
const SNAPSHOT_MS = 4000;
const QUIT_MS = 2500;
const CLEANUP_MS = 1500;
const MAX_BEFORE_QUIT_REENTRY = 8;
const MAX_BEFORE_QUIT_EVENTS = 12;

const tests = [];
function test(name, fn) { tests.push({ name, fn }); }

function sha256File(abs) {
  return crypto.createHash('sha256').update(fs.readFileSync(abs)).digest('hex');
}

function requireRegularFile(abs, label) {
  let stat;
  try {
    stat = fs.lstatSync(abs);
  } catch (_) {
    throw new Error(`${label} is missing: ${abs}`);
  }
  if (stat.isSymbolicLink() || !stat.isFile()) {
    throw new Error(`${label} is not a regular file: ${abs}`);
  }
  return stat;
}

function appendCleanup(previous, error) {
  if (!previous) return error;
  previous.message = `${previous.message}\ncleanup also failed: ${error.stack || error.message}`;
  return previous;
}

function withTimeout(promise, ms, message) {
  let timer = null;
  const timeout = new Promise((_, reject) => {
    timer = setTimeout(() => reject(new Error(message)), ms);
  });
  return Promise.race([Promise.resolve(promise), timeout]).finally(() => {
    if (timer !== null) clearTimeout(timer);
  });
}

function waitUntil(predicate, ms, message) {
  return new Promise((resolve, reject) => {
    const start = Date.now();
    let timer = null;
    let done = false;
    const finish = (error) => {
      if (done) return;
      done = true;
      if (timer !== null) {
        clearInterval(timer);
        timer = null;
      }
      if (error) reject(error);
      else resolve();
    };
    const tick = () => {
      try {
        if (predicate()) {
          finish();
          return;
        }
      } catch (error) {
        finish(error);
        return;
      }
      if (Date.now() - start >= ms) finish(new Error(message));
    };
    tick();
    if (!done) timer = setInterval(tick, 20);
  });
}

function hasExited(child) {
  return Boolean(child) && (child.exitCode !== null || child.signalCode !== null);
}

function collectOwnedTree(root) {
  const files = [];
  const dirs = [];
  function visit(abs) {
    let stat;
    try {
      stat = fs.lstatSync(abs);
    } catch (_) {
      return;
    }
    if (stat.isSymbolicLink()) {
      files.push(abs);
      return;
    }
    if (stat.isDirectory()) {
      dirs.push(abs);
      let names;
      try {
        names = fs.readdirSync(abs);
      } catch (_) {
        return;
      }
      for (const name of names) visit(path.join(abs, name));
      return;
    }
    files.push(abs);
  }
  visit(root);
  return { files, dirs };
}

function removeOwnedTree(root) {
  const { files, dirs } = collectOwnedTree(root);
  let cleanupError = null;
  for (const abs of files.slice().reverse()) {
    try { fs.unlinkSync(abs); } catch (error) { cleanupError = appendCleanup(cleanupError, error); }
  }
  for (const abs of dirs.slice().reverse()) {
    try { fs.rmdirSync(abs); } catch (error) { cleanupError = appendCleanup(cleanupError, error); }
  }
  return cleanupError;
}

function createElectronMock(userData) {
  const state = {
    enableSandboxCalls: 0,
    windows: [],
    permissionRequestHandler: null,
    permissionCheckHandler: null,
    menuSet: [],
    appHandlers: Object.create(null),
    quitCalls: 0,
    ipcHandlers: Object.create(null),
    rendererMessages: [],
    errorBoxes: [],
    beforeQuitEvents: [],
    lifecycleTrace: [],
    beforeQuitDepth: 0,
  };

  class WebContents {
    constructor() {
      this.handlers = Object.create(null);
      this.windowOpenHandler = null;
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

    setWindowOpenHandler(handler) {
      this.windowOpenHandler = handler;
    }
  }

  class BrowserWindow {
    constructor(options) {
      this.options = options;
      this.webContents = new WebContents();
      this.loadedFiles = [];
      this._closedHandlers = [];
      state.windows.push(this);
    }

    loadFile(filePath) {
      this.loadedFiles.push(filePath);
    }

    on(event, handler) {
      if (event === 'closed') this._closedHandlers.push(handler);
    }
  }

  function emitRegisteredBeforeQuit(viaAppQuit) {
    const handlers = state.appHandlers['before-quit'];
    assert.ok(Array.isArray(handlers) && handlers.length > 0, 'before-quit handler is missing');
    assert.ok(state.beforeQuitEvents.length < MAX_BEFORE_QUIT_EVENTS, 'too many before-quit events');
    assert.ok(state.beforeQuitDepth < MAX_BEFORE_QUIT_REENTRY, 'before-quit re-entered too many times');
    state.beforeQuitDepth += 1;
    const event = {
      defaultPrevented: false,
      preventDefault() { this.defaultPrevented = true; },
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
      for (const handler of handlers) record.returned = handler(event);
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
    isPackaged: false,
    enableSandbox() { state.enableSandboxCalls += 1; },
    getPath(name) {
      if (name !== 'userData') throw new Error(`unexpected getPath ${name}`);
      return userData;
    },
    on(event, handler) {
      if (!state.appHandlers[event]) state.appHandlers[event] = [];
      state.appHandlers[event].push(handler);
    },
    quit() {
      state.quitCalls += 1;
      state.lifecycleTrace.push({ type: 'app.quit' });
      emitRegisteredBeforeQuit(true);
    },
  };

  const electron = {
    app,
    BrowserWindow,
    Menu: {
      setApplicationMenu(menu) { state.menuSet.push(menu); },
    },
    session: {
      defaultSession: {
        setPermissionRequestHandler(handler) { state.permissionRequestHandler = handler; },
        setPermissionCheckHandler(handler) { state.permissionCheckHandler = handler; },
      },
    },
    ipcMain: {
      handle(channel, handler) {
        assert.ok(!state.ipcHandlers[channel], `duplicate ipcMain channel ${channel}`);
        state.ipcHandlers[channel] = handler;
      },
    },
    dialog: {
      showErrorBox(title, content) {
        state.errorBoxes.push({ title, content });
        state.lifecycleTrace.push({ type: 'showErrorBox', title, content });
      },
    },
  };

  return {
    electron,
    state,
    emitApp(event, ...args) {
      for (const handler of state.appHandlers[event] || []) handler(...args);
    },
    emitBeforeQuit() {
      return emitRegisteredBeforeQuit(false);
    },
  };
}

function trustedWalletEvent(ctx) {
  assert.strictEqual(ctx.state.windows.length, 1, 'smoke must create exactly one BrowserWindow');
  const win = ctx.state.windows[0];
  return { senderFrame: win.webContents.mainFrame, sender: win.webContents };
}

function subscribedSnapshot(state, broker) {
  let latest;
  for (let index = 0; index < state.rendererMessages.length; index += 1) {
    const entry = state.rendererMessages[index];
    if (entry[1] !== 'wallet:snapshot:subscribe') continue;
    if (broker && (!entry[2] || entry[2].broker !== broker)) continue;
    latest = entry[2];
  }
  return latest;
}

test('ready starts the pinned development broker, degraded snapshot, UNAVAILABLE list, and awaited quit', async () => {
  requireRegularFile(MANIFEST_PATH, 'wallet broker manifest');
  requireRegularFile(BROKER_PATH, 'wallet broker binary');
  const manifest = JSON.parse(fs.readFileSync(MANIFEST_PATH, 'utf8'));
  assert.ok(manifest && typeof manifest.sha256 === 'string', 'wallet broker manifest sha256 is missing');

  const userData = fs.mkdtempSync(path.join(os.tmpdir(), 'bb-wal011-smoke-'));
  const expectedCwd = path.join(userData, 'wallet-broker');
  const originalLoad = Module._load;
  const originalSpawn = childProcess.spawn;
  const previousMain = require.cache[mainPath];
  const spawnCalls = [];
  const children = [];
  const spawnErrors = new WeakMap();
  const closed = new WeakMap();
  let ctx = null;
  let testError = null;
  let quitRequested = false;

  function onChildError(error) {
    if (!spawnErrors.has(this)) spawnErrors.set(this, error);
  }

  function onChildClose() {
    closed.set(this, true);
    if (ctx) ctx.state.lifecycleTrace.push({ type: 'child.close' });
  }

  function childError(child) {
    return child ? spawnErrors.get(child) : undefined;
  }

  function throwIfSpawnFailed(child) {
    const error = childError(child);
    if (error) throw error;
  }

  childProcess.spawn = function recordAndDelegate(...args) {
    spawnCalls.push({ file: args[0], argv: args[1], options: args[2] });
    const child = originalSpawn.apply(this, args);
    children.push(child);
    child.on('error', onChildError);
    child.on('close', onChildClose);
    return child;
  };

  try {
    assert.ok(fs.existsSync(mainPath), 'maintained Electron entry social-main.js is missing');
    ctx = createElectronMock(userData);
    Module._load = function loadWithElectronMock(request, parent, isMain) {
      if (request === 'electron') return ctx.electron;
      return originalLoad.call(this, request, parent, isMain);
    };
    delete require.cache[mainPath];
    require(mainPath);

    assert.strictEqual(spawnCalls.length, 0, 'broker spawned before ready');
    assert.strictEqual(children.length, 0, 'child existed before ready');
    ctx.emitApp('ready');

    assert.strictEqual(spawnCalls.length, 1, 'ready did not spawn one broker');
    const spawned = spawnCalls[0];
    const child = children[0];
    assert.ok(child, 'observer did not capture the real child');
    throwIfSpawnFailed(child);
    assert.strictEqual(spawned.file, BROKER_PATH);
    assert.deepStrictEqual(spawned.argv, []);
    assert.deepStrictEqual(spawned.options.env, {});
    assert.strictEqual(spawned.options.cwd, expectedCwd);
    assert.strictEqual(spawned.options.shell, false);
    assert.deepStrictEqual(spawned.options.stdio, ['pipe', 'pipe', 'pipe']);
    const cwdStat = fs.lstatSync(expectedCwd);
    assert.ok(!cwdStat.isSymbolicLink() && cwdStat.isDirectory(), 'broker cwd is not a real directory');
    assert.strictEqual(cwdStat.mode & 0o777, 0o700);
    assert.strictEqual(sha256File(spawned.file), manifest.sha256);
    assert.strictEqual(sha256File(BROKER_PATH), manifest.sha256);

    await waitUntil(() => {
      throwIfSpawnFailed(child);
      return Boolean(subscribedSnapshot(ctx.state, 'degraded'));
    }, SNAPSHOT_MS, 'sanitized degraded snapshot subscription did not settle');
    const published = subscribedSnapshot(ctx.state, 'degraded');
    assert.deepStrictEqual(published, EXPECTED_SNAPSHOT);
    throwIfSpawnFailed(child);

    const event = trustedWalletEvent(ctx);
    const snapshotHandler = ctx.state.ipcHandlers['wallet:snapshot:get'];
    assert.strictEqual(typeof snapshotHandler, 'function');
    const resolved = await withTimeout(
      Promise.resolve().then(() => snapshotHandler(event)),
      SNAPSHOT_MS,
      'wallet:snapshot:get did not settle'
    );
    assert.deepStrictEqual(resolved, EXPECTED_SNAPSHOT);
    assert.notStrictEqual(resolved, published);
    throwIfSpawnFailed(child);

    const listHandler = ctx.state.ipcHandlers['wallet:accounts:list'];
    assert.strictEqual(typeof listHandler, 'function');
    await assert.rejects(
      async () => {
        const value = await withTimeout(
          Promise.resolve().then(() => listHandler(event)),
          SNAPSHOT_MS,
          'account.list did not settle'
        );
        assert.fail(`account.list resolved with ${JSON.stringify(value)}`);
      },
      (error) => {
        assert.strictEqual(error && error.code, 'UNAVAILABLE');
        return true;
      }
    );
    throwIfSpawnFailed(child);

    const prevented = ctx.emitBeforeQuit();
    quitRequested = true;
    assert.strictEqual(prevented.defaultPrevented, true, 'before-quit was not prevented');
    assert.strictEqual(ctx.state.quitCalls, 0, 'app.quit ran before child close');

    await waitUntil(() => {
      throwIfSpawnFailed(child);
      const types = ctx.state.lifecycleTrace.map((entry) => entry.type);
      return types.includes('child.close') && types.includes('app.quit');
    }, QUIT_MS, 'child close and resumed app.quit did not settle');

    const types = ctx.state.lifecycleTrace.map((entry) => entry.type);
    const closeAt = types.indexOf('child.close');
    const quitAt = types.indexOf('app.quit');
    assert.ok(closeAt !== -1 && quitAt !== -1 && closeAt < quitAt, 'close must precede resumed quit');
    assert.strictEqual(ctx.state.quitCalls, 1);
    assert.ok(ctx.state.beforeQuitEvents.length >= 2, 'reentrant before-quit was missing');
    const reentrant = ctx.state.beforeQuitEvents[ctx.state.beforeQuitEvents.length - 1];
    assert.strictEqual(reentrant.viaAppQuit, true);
    assert.strictEqual(reentrant.defaultPrevented, false, 'reentrant before-quit was not allowed');
    assert.strictEqual(ctx.state.errorBoxes.length, 0, 'error dialog was shown');
    assert.strictEqual(spawnCalls.length, 1);
    assert.ok(closed.get(child), 'child leaked after quit');
    throwIfSpawnFailed(child);
  } catch (error) {
    testError = error;
  } finally {
    let cleanupError = null;
    const live = children.filter((child) => child && !closed.get(child));
    if (live.length > 0 && ctx && !quitRequested) {
      try {
        ctx.emitBeforeQuit();
        quitRequested = true;
        await waitUntil(
          () => live.every((child) => closed.get(child)),
          QUIT_MS,
          'normal quit did not close the child'
        );
      } catch (error) {
        cleanupError = appendCleanup(cleanupError, error);
      }
    }
    for (const child of children) {
      if (!child || closed.get(child) || hasExited(child)) continue;
      try { child.kill('SIGTERM'); } catch (error) { cleanupError = appendCleanup(cleanupError, error); }
    }
    try {
      await waitUntil(
        () => children.every((child) => !child || closed.get(child)),
        CLEANUP_MS,
        'child did not close after SIGTERM'
      );
    } catch (_) { /* escalate */ }
    for (const child of children) {
      if (!child || closed.get(child) || hasExited(child)) continue;
      try { child.kill('SIGKILL'); } catch (error) { cleanupError = appendCleanup(cleanupError, error); }
    }
    try {
      await waitUntil(
        () => children.every((child) => !child || closed.get(child)),
        CLEANUP_MS,
        'child did not close after SIGKILL'
      );
    } catch (error) {
      cleanupError = appendCleanup(cleanupError, error);
    }

    Module._load = originalLoad;
    childProcess.spawn = originalSpawn;
    if (previousMain) require.cache[mainPath] = previousMain;
    else delete require.cache[mainPath];
    for (const child of children) {
      if (!child || !closed.get(child)) continue;
      child.removeListener('error', onChildError);
      child.removeListener('close', onChildClose);
    }

    const unreaped = children.some((child) => child && !closed.get(child));
    if (unreaped) {
      cleanupError = appendCleanup(
        cleanupError,
        new Error('refusing to remove broker cwd before confirmed close')
      );
    } else {
      const removed = removeOwnedTree(userData);
      if (removed) cleanupError = appendCleanup(cleanupError, removed);
    }

    if (cleanupError) testError = appendCleanup(testError, cleanupError);
    if (testError) throw testError;
  }
});

async function run() {
  let failed = 0;
  for (const { name, fn } of tests) {
    try {
      await fn();
      process.stdout.write(`ok ${name}\n`);
    } catch (error) {
      failed += 1;
      process.stderr.write(`not ok ${name}\n${error && error.stack ? error.stack : error}\n`);
    }
  }
  if (failed) {
    process.stderr.write(`${failed} wallet startup smoke test(s) failed\n`);
    process.exitCode = 1;
    return;
  }
  process.stdout.write(`BitBook wallet startup smoke tests passed (${tests.length}).\n`);
}

if (require.main === module) {
  run().catch((error) => {
    process.stderr.write(`${error && error.stack ? error.stack : error}\n`);
    process.exitCode = 1;
  });
}

module.exports = { tests };
