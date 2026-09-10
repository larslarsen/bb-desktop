'use strict';

const assert = require('assert');
const crypto = require('crypto');
const fs = require('fs');
const path = require('path');
const childProcess = require('child_process');

const { createWalletSupervisor } = require('../wallet-broker/supervisor');

const REPO_ROOT = path.join(__dirname, '..');
const TARGET_ROOT = path.join(REPO_ROOT, 'wallet-broker', 'target');
const DEFAULT_BROKER_PATH = path.join(
  TARGET_ROOT, 'app-resources', 'wallet-broker', 'bitbook-wallet-broker'
);
const X11_HELPER = path.join(__dirname, 'fixtures', 'wallet-broker', 'x11-window.py');
const PYTHON = '/usr/bin/python3.14';
const WINDOW_TITLE = 'BitBook accounts';
const CANARY = 'BBD_WAL_013_NATIVE_WINDOW_CANARY';
const BOGUS_WAYLAND_DISPLAY = 'wal013-bogus-wayland-0';
const VISIBLE_ARTIFACT = path.join(TARGET_ROOT, 'wal013-account-window-visible.png');
const REOPENED_ARTIFACT = path.join(TARGET_ROOT, 'wal013-account-window-reopened.png');
const PNG_SIGNATURE = Buffer.from([0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a]);
const CAPTURE_LIMIT = 1024 * 1024;
const MAX_CAPTURE_DIMENSION = 2048;
const CAPTURE_MARGIN = 8;
const MIN_COLORS = 16;
const MIN_NON_DOMINANT_PIXELS = 1000;
const TEST_TIMEOUT_MS = 45000;
const TEST_TIMEOUT_MESSAGE = 'native account window test exceeded 45 seconds';
const BOUND_MS = 5000;
const MANAGE_MS = 15000;
const WINDOW_MS = 5000;
const REQUEST_MS = 2500;
const HELPER_MS = 1500;
const CLEANUP_MS = 1500;
const DEGRADED = { v: 1, broker: 'degraded', accounts: [], intent_preview: null };

const tests = [];
function test(name, fn) { tests.push({ name, fn }); }

function appendFailure(primary, secondary) {
  if (!primary) return secondary;
  primary.message = `${primary.message}\ncleanup also failed: ${secondary.message}`;
  return primary;
}

function delay(milliseconds) {
  return new Promise((resolve) => setTimeout(resolve, milliseconds));
}

function withDeadline(value, milliseconds, message) {
  let timer;
  const timeout = new Promise((_, reject) => {
    timer = setTimeout(() => reject(new Error(message)), milliseconds);
  });
  return Promise.race([Promise.resolve(value), timeout]).finally(() => clearTimeout(timer));
}

function globalDeadlineRemaining(context) {
  const remaining = context.globalDeadline - Date.now();
  if (remaining <= 0) throw new Error(TEST_TIMEOUT_MESSAGE);
  return remaining;
}

function withGlobalOperationDeadline(context, operation, milliseconds, message) {
  const globalRemaining = globalDeadlineRemaining(context);
  const timeout = Math.min(milliseconds, globalRemaining);
  return withDeadline(
    operation(),
    timeout,
    timeout === globalRemaining ? TEST_TIMEOUT_MESSAGE : message
  );
}

async function delayWithinGlobalDeadline(context, milliseconds) {
  await delay(Math.min(milliseconds, globalDeadlineRemaining(context)));
  globalDeadlineRemaining(context);
}

function hasExited(child) {
  return Boolean(child) && (child.exitCode !== null || child.signalCode !== null);
}

function safeEnvironmentValue(name) {
  const value = process.env[name];
  if (typeof value !== 'string' || value.length === 0 || value.includes('\0') ||
      Buffer.byteLength(value, 'utf8') > 4096) {
    throw new Error(`isolated Xvfb ${name} is unavailable`);
  }
  return value;
}

function requireRegularFile(file, label) {
  let stat;
  try {
    stat = fs.lstatSync(file);
  } catch (_) {
    throw new Error(`${label} is unavailable`);
  }
  if (stat.isSymbolicLink() || !stat.isFile()) throw new Error(`${label} is unavailable`);
}

function requireInputs() {
  if (process.argv.length > 3) throw new Error('unexpected broker arguments');
  let brokerPath = DEFAULT_BROKER_PATH;
  if (process.argv[2] !== undefined) {
    if (!path.isAbsolute(process.argv[2])) throw new Error('broker argument must be absolute');
    brokerPath = process.argv[2];
  }
  requireRegularFile(brokerPath, 'staged native wallet broker');
  requireRegularFile(X11_HELPER, 'X11 helper');

  let pythonPath;
  try {
    pythonPath = fs.realpathSync(PYTHON);
    requireRegularFile(pythonPath, 'system Python interpreter');
    fs.accessSync(pythonPath, fs.constants.X_OK);
  } catch (_) {
    throw new Error('system Python interpreter is unavailable');
  }

  let targetStat;
  try {
    targetStat = fs.lstatSync(TARGET_ROOT);
  } catch (_) {
    throw new Error('wallet broker target directory is unavailable');
  }
  if (targetStat.isSymbolicLink() || !targetStat.isDirectory()) {
    throw new Error('wallet broker target directory is unavailable');
  }

  const env = {
    DISPLAY: safeEnvironmentValue('DISPLAY'),
    WAYLAND_DISPLAY: BOGUS_WAYLAND_DISPLAY,
    XAUTHORITY: safeEnvironmentValue('XAUTHORITY'),
    LANG: 'C.UTF-8',
    PATH: '/usr/bin',
  };
  const expectedSha256 = crypto.createHash('sha256').update(fs.readFileSync(brokerPath)).digest('hex');
  return { brokerPath, expectedSha256, env, pythonPath };
}

function createOwnedRoot() {
  const root = fs.mkdtempSync(path.join(TARGET_ROOT, 'bb-wal013-native-window-'));
  const accounts = path.join(root, 'accounts');
  try {
    fs.chmodSync(root, 0o700);
    fs.mkdirSync(accounts, { mode: 0o700 });
    fs.chmodSync(accounts, 0o700);
  } catch (error) {
    try { fs.rmdirSync(accounts); } catch (cleanupError) {
      if (cleanupError.code !== 'ENOENT') error.message = `${error.message}; setup cleanup failed`;
    }
    try { fs.rmdirSync(root); } catch (_) { error.message = `${error.message}; setup cleanup failed`; }
    throw new Error('owned root setup failed');
  }
  return { root, accounts };
}

function assertOwnedRoot(owned) {
  const rootStat = fs.lstatSync(owned.root);
  assert.ok(!rootStat.isSymbolicLink() && rootStat.isDirectory(), 'owned root is not a real directory');
  assert.strictEqual(rootStat.mode & 0o777, 0o700, 'owned root mode changed');
  assert.ok(
    fs.readdirSync(owned.root).length === 1 && fs.readdirSync(owned.root)[0] === 'accounts',
    'owned root contains an unexpected entry'
  );
  const accountsStat = fs.lstatSync(owned.accounts);
  assert.ok(
    !accountsStat.isSymbolicLink() && accountsStat.isDirectory(),
    'accounts path is not a real directory'
  );
  assert.strictEqual(accountsStat.mode & 0o777, 0o700, 'accounts directory mode changed');
  assert.strictEqual(fs.readdirSync(owned.accounts).length, 0, 'accounts directory is not empty');
}

function removeOwnedRoot(owned) {
  assertOwnedRoot(owned);
  fs.rmdirSync(owned.accounts);
  fs.rmdirSync(owned.root);
}

function addCapture(record, stream, name) {
  if (!stream) {
    record[`${name}Closed`] = true;
    return;
  }
  record[`${name}Closed`] = stream.destroyed;
  const onData = (chunk) => {
    const bytes = Buffer.from(chunk);
    if (record[`${name}Bytes`] + bytes.length > CAPTURE_LIMIT) {
      record.captureOverflow = true;
      return;
    }
    record[`${name}Bytes`] += bytes.length;
    record[name].push(bytes);
  };
  const onClose = () => { record[`${name}Closed`] = true; };
  stream.on('data', onData);
  stream.on('close', onClose);
  record.listeners.push({ target: stream, event: 'data', handler: onData });
  record.listeners.push({ target: stream, event: 'close', handler: onClose });
}

function observeStreamClose(record, stream, name) {
  if (!stream) {
    record[`${name}Closed`] = true;
    return;
  }
  record[`${name}Closed`] = stream.destroyed;
  const onClose = () => { record[`${name}Closed`] = true; };
  stream.on('close', onClose);
  record.listeners.push({ target: stream, event: 'close', handler: onClose });
}

function observeBroker(child) {
  const record = {
    child,
    stdout: [],
    stderr: [],
    stdoutBytes: 0,
    stderrBytes: 0,
    stdinClosed: false,
    stdoutClosed: false,
    stderrClosed: false,
    closed: false,
    exited: false,
    spawnError: false,
    captureOverflow: false,
    signalCalls: [],
    listeners: [],
  };
  const originalKill = child.kill;
  child.kill = function recordAndDelegate(signal) {
    record.signalCalls.push(signal || 'SIGTERM');
    return originalKill.call(this, signal);
  };
  const onError = () => { record.spawnError = true; };
  const onExit = () => { record.exited = true; };
  const onClose = () => { record.closed = true; };
  child.on('error', onError);
  child.on('exit', onExit);
  child.on('close', onClose);
  record.listeners.push({ target: child, event: 'error', handler: onError });
  record.listeners.push({ target: child, event: 'exit', handler: onExit });
  record.listeners.push({ target: child, event: 'close', handler: onClose });
  observeStreamClose(record, child.stdin, 'stdin');
  addCapture(record, child.stdout, 'stdout');
  addCapture(record, child.stderr, 'stderr');
  return record;
}

function removeRecordListeners(record) {
  for (const entry of record.listeners) {
    entry.target.removeListener(entry.event, entry.handler);
  }
  record.listeners.length = 0;
}

async function waitUntil(predicate, milliseconds, message, record, context) {
  const deadline = Date.now() + milliseconds;
  while (true) {
    if (record && record.spawnError) throw new Error('native broker spawn failed');
    const globalRemaining = context ? globalDeadlineRemaining(context) : Number.POSITIVE_INFINITY;
    if (predicate()) return;
    if (Date.now() >= deadline) throw new Error(message);
    await delay(Math.min(25, Math.max(1, deadline - Date.now()), globalRemaining));
  }
}

async function waitForBrokerClose(record, milliseconds, message) {
  if (record.closed) return;
  await waitUntil(() => record.closed, milliseconds, message, record);
}

function observeHelper(child) {
  const record = {
    child,
    stdout: [],
    stderr: [],
    stdoutBytes: 0,
    stderrBytes: 0,
    stdoutClosed: false,
    stderrClosed: false,
    closed: false,
    exitCode: null,
    signalCode: null,
    spawnError: false,
    captureOverflow: false,
    listeners: [],
  };
  const onError = () => { record.spawnError = true; };
  const onClose = (code, signal) => {
    record.closed = true;
    record.exitCode = code;
    record.signalCode = signal;
  };
  child.on('error', onError);
  child.on('close', onClose);
  record.listeners.push({ target: child, event: 'error', handler: onError });
  record.listeners.push({ target: child, event: 'close', handler: onClose });
  addCapture(record, child.stdout, 'stdout');
  addCapture(record, child.stderr, 'stderr');
  return record;
}

async function forceCloseHelper(record) {
  if (record.closed) return;
  try { record.child.kill('SIGTERM'); } catch (_) { /* continue to bounded reap */ }
  try {
    await waitUntil(() => record.closed, 250, 'X11 helper did not close after SIGTERM');
  } catch (_) {
    if (!record.closed) {
      try { record.child.kill('SIGKILL'); } catch (_) { /* report through close check */ }
    }
  }
  await waitUntil(() => record.closed, CLEANUP_MS, 'X11 helper could not be reaped');
}

function parseHelperJson(record) {
  if (record.spawnError || record.captureOverflow || record.exitCode !== 0 ||
      record.signalCode !== null || record.stderrBytes !== 0) {
    throw new Error('X11 helper failed');
  }
  let value;
  try {
    value = JSON.parse(Buffer.concat(record.stdout).toString('utf8'));
  } catch (_) {
    throw new Error('X11 helper returned invalid JSON');
  }
  return value;
}

async function runHelper(context, args, milliseconds = HELPER_MS) {
  const globalRemaining = globalDeadlineRemaining(context);
  const timeout = Math.min(milliseconds, globalRemaining);
  const timeoutMessage = timeout === globalRemaining ? TEST_TIMEOUT_MESSAGE : 'X11 helper timed out';
  const brokerPid = context.brokerRecord && context.brokerRecord.child.pid;
  if (!Number.isInteger(brokerPid) || brokerPid <= 0 || brokerPid > 0x7fffffff) {
    throw new Error('native broker PID is unavailable');
  }
  const child = childProcess.spawn(context.pythonPath, [X11_HELPER, ...args, String(brokerPid)], {
    cwd: context.owned.root,
    env: context.env,
    shell: false,
    stdio: ['ignore', 'pipe', 'pipe'],
  });
  const record = observeHelper(child);
  context.helpers.push(record);
  try {
    await waitUntil(() => record.closed, timeout, timeoutMessage, undefined, context);
  } catch (error) {
    await forceCloseHelper(record);
    throw error;
  } finally {
    if (record.closed) removeRecordListeners(record);
  }
  return parseHelperJson(record);
}

function exactKeys(value, keys) {
  return value && typeof value === 'object' && !Array.isArray(value) &&
    Object.getPrototypeOf(value) === Object.prototype &&
    Object.keys(value).length === keys.length &&
    keys.every((key) => Object.prototype.hasOwnProperty.call(value, key));
}

async function inspectWindows(context, milliseconds = HELPER_MS) {
  const result = await runHelper(context, ['inspect'], milliseconds);
  assert.ok(exactKeys(result, ['windows']) && Array.isArray(result.windows), 'invalid X11 inspection result');
  let previous = 0;
  const ids = new Set();
  for (const window of result.windows) {
    assert.ok(exactKeys(window, ['id', 'map_state']), 'invalid X11 window record');
    assert.ok(Number.isSafeInteger(window.id) && window.id > 0, 'invalid X11 window id');
    assert.ok(Number.isInteger(window.map_state) && window.map_state >= 0 && window.map_state <= 2,
      'invalid X11 map state');
    assert.ok(window.id > previous && !ids.has(window.id), 'X11 windows are not unique and sorted');
    previous = window.id;
    ids.add(window.id);
  }
  return result.windows;
}

async function closeWindow(context, windowId) {
  const result = await runHelper(context, ['close', String(windowId)]);
  assert.ok(exactKeys(result, ['closed']) && result.closed === windowId, 'X11 close was not acknowledged');
}

function validatedCapture(result) {
  assert.ok(exactKeys(result, [
    'width', 'height', 'color_count', 'non_dominant_pixels', 'png_base64',
  ]), 'invalid X11 capture result');
  assert.ok(Number.isSafeInteger(result.width) && result.width > CAPTURE_MARGIN * 2 &&
    result.width <= MAX_CAPTURE_DIMENSION, 'invalid X11 capture width');
  assert.ok(Number.isSafeInteger(result.height) && result.height > CAPTURE_MARGIN * 2 &&
    result.height <= MAX_CAPTURE_DIMENSION, 'invalid X11 capture height');
  const sampledPixels = (result.width - CAPTURE_MARGIN * 2) *
    (result.height - CAPTURE_MARGIN * 2);
  assert.ok(Number.isSafeInteger(result.color_count) && result.color_count >= 1 &&
    result.color_count <= sampledPixels, 'invalid X11 capture color count');
  assert.ok(Number.isSafeInteger(result.non_dominant_pixels) &&
    result.non_dominant_pixels >= 0 && result.non_dominant_pixels < sampledPixels,
    'invalid X11 capture non-dominant count');
  assert.ok(typeof result.png_base64 === 'string' && result.png_base64.length > 0 &&
    result.png_base64.length <= CAPTURE_LIMIT && result.png_base64.length % 4 === 0 &&
    /^[A-Za-z0-9+/]+={0,2}$/.test(result.png_base64), 'invalid X11 capture base64');
  const png = Buffer.from(result.png_base64, 'base64');
  assert.ok(png.length > 24 && png.length <= CAPTURE_LIMIT &&
    png.toString('base64') === result.png_base64, 'invalid X11 capture PNG encoding');
  assert.ok(png.subarray(0, PNG_SIGNATURE.length).equals(PNG_SIGNATURE),
    'invalid X11 capture PNG signature');
  assert.strictEqual(png.subarray(12, 16).toString('ascii'), 'IHDR',
    'invalid X11 capture PNG header');
  assert.strictEqual(png.readUInt32BE(16), result.width, 'X11 capture PNG width mismatch');
  assert.strictEqual(png.readUInt32BE(20), result.height, 'X11 capture PNG height mismatch');
  return png;
}

async function waitForRenderedWindow(context, windowId) {
  const deadline = Date.now() + WINDOW_MS;
  let observedBlank = false;
  while (true) {
    globalDeadlineRemaining(context);
    const remaining = deadline - Date.now();
    if (remaining <= 0) throw new Error('native account window remained blank');
    let result;
    try {
      result = await runHelper(
        context,
        ['capture', String(windowId)],
        Math.min(HELPER_MS, remaining)
      );
    } catch (error) {
      if (observedBlank && Date.now() >= deadline &&
          error && error.message === 'X11 helper timed out') {
        throw new Error('native account window remained blank');
      }
      throw error;
    }
    const png = validatedCapture(result);
    if (result.color_count >= MIN_COLORS &&
        result.non_dominant_pixels >= MIN_NON_DOMINANT_PIXELS) {
      return png;
    }
    observedBlank = true;
    if (Date.now() >= deadline) throw new Error('native account window remained blank');
    await delay(Math.min(
      100,
      Math.max(1, deadline - Date.now()),
      globalDeadlineRemaining(context)
    ));
  }
}

function writeQaArtifact(file, png) {
  const flags = fs.constants.O_WRONLY | fs.constants.O_CREAT | fs.constants.O_NOFOLLOW;
  let descriptor;
  try {
    descriptor = fs.openSync(file, flags, 0o600);
    const stat = fs.fstatSync(descriptor);
    assert.ok(stat.isFile() && stat.nlink === 1 && stat.uid === process.getuid(),
      'QA artifact is not an owned regular file');
    fs.fchmodSync(descriptor, 0o600);
    fs.ftruncateSync(descriptor, 0);
    fs.writeFileSync(descriptor, png);
  } finally {
    if (descriptor !== undefined) fs.closeSync(descriptor);
  }
}

async function waitForVisibleWindows(context, expected, milliseconds, message) {
  const deadline = Date.now() + milliseconds;
  while (true) {
    globalDeadlineRemaining(context);
    const remaining = deadline - Date.now();
    if (remaining <= 0) throw new Error(message);
    const windows = await inspectWindows(context, Math.min(HELPER_MS, remaining));
    const visible = windows.filter((window) => window.map_state === 2);
    if (visible.length > 1) throw new Error('more than one visible account window was observed');
    if (visible.length === expected) return visible;
    if (Date.now() >= deadline) throw new Error(message);
    await delay(Math.min(
      100,
      Math.max(1, deadline - Date.now()),
      globalDeadlineRemaining(context)
    ));
  }
}

async function waitForHiddenInitializedWindow(context) {
  const deadline = Date.now() + WINDOW_MS;
  let stableWindowId = null;
  let stableSince = null;
  while (true) {
    globalDeadlineRemaining(context);
    const remaining = deadline - Date.now();
    if (remaining <= 0) throw new Error('native account window did not initialize hidden');
    let windows;
    try {
      windows = await inspectWindows(context, Math.min(HELPER_MS, remaining));
    } catch (error) {
      if (error && error.message === TEST_TIMEOUT_MESSAGE) throw error;
      if (Date.now() >= deadline) {
        throw new Error('native account window did not initialize hidden');
      }
      throw error;
    }
    if (windows.length > 1) {
      throw new Error('more than one account window was observed during native startup');
    }
    const observedAt = Date.now();
    const window = windows[0];
    if (window && window.map_state === 0) {
      if (window.id !== stableWindowId) {
        stableWindowId = window.id;
        stableSince = observedAt;
      } else if (observedAt - stableSince >= 250) {
        return window;
      }
    } else {
      stableWindowId = null;
      stableSince = null;
    }
    if (observedAt >= deadline) {
      throw new Error('native account window did not initialize hidden');
    }
    await delay(Math.min(
      100,
      Math.max(1, deadline - observedAt),
      globalDeadlineRemaining(context)
    ));
  }
}

async function waitForManage(context) {
  const deadline = Date.now() + MANAGE_MS;
  while (Date.now() < deadline) {
    globalDeadlineRemaining(context);
    try {
      const remaining = Math.max(1, Math.min(REQUEST_MS, deadline - Date.now()));
      const result = await withGlobalOperationDeadline(
        context,
        () => context.supervisor.dispatch('account.manage', {}),
        remaining,
        'account.manage did not settle'
      );
      assert.ok(exactKeys(result, []), 'account.manage returned an invalid result');
      return;
    } catch (error) {
      if (error && error.message === TEST_TIMEOUT_MESSAGE) throw error;
      if (!error || error.code !== 'UNAVAILABLE') throw new Error('account.manage failed unexpectedly');
    }
    await delay(Math.min(
      100,
      Math.max(1, deadline - Date.now()),
      globalDeadlineRemaining(context)
    ));
  }
  throw new Error('native account window context did not register');
}

function assertSpawn(context, inputs) {
  assert.strictEqual(context.spawnCalls.length, 1, 'supervisor did not spawn exactly one broker');
  const call = context.spawnCalls[0];
  assert.strictEqual(call.file, inputs.brokerPath, 'supervisor spawned the wrong broker');
  assert.ok(Array.isArray(call.args) && call.args.length === 0, 'broker argv was not empty');
  assert.strictEqual(call.options.cwd, context.owned.root, 'broker cwd was not the owned root');
  assert.strictEqual(call.options.shell, false, 'broker enabled a shell');
  assert.ok(
    Array.isArray(call.options.stdio) && call.options.stdio.join(',') === 'pipe,pipe,pipe',
    'broker stdio was not three pipes'
  );
  assert.notStrictEqual(call.options.env, inputs.env, 'broker reused the supervisor input env');
  assert.deepStrictEqual(call.options.env, {
    LANG: inputs.env.LANG,
    PATH: inputs.env.PATH,
    DISPLAY: inputs.env.DISPLAY,
    XAUTHORITY: inputs.env.XAUTHORITY,
    LIBGL_ALWAYS_SOFTWARE: '1',
    __GLX_VENDOR_LIBRARY_NAME: 'mesa',
  }, 'broker environment was not the fixed isolated X11 software-renderer environment');
  assert.strictEqual(inputs.env.WAYLAND_DISPLAY, BOGUS_WAYLAND_DISPLAY,
    'supervisor input WAYLAND_DISPLAY changed');
  assert.ok(!Object.prototype.hasOwnProperty.call(call.options.env, 'WAYLAND_DISPLAY'),
    'broker inherited WAYLAND_DISPLAY');
  assert.ok(!Object.prototype.hasOwnProperty.call(call.options.env, 'DBUS_SESSION_BUS_ADDRESS'),
    'broker inherited a host bus');
}

function assertCompleteProtocol(record, ownedRoot) {
  assert.strictEqual(record.captureOverflow, false, 'broker diagnostics exceeded the private capture limit');
  const stdout = Buffer.concat(record.stdout);
  const stderr = Buffer.concat(record.stderr);
  assert.ok(stdout.length > 0, 'broker emitted no protocol transcript');
  assert.ok(!stdout.includes(Buffer.from(CANARY)) && !stderr.includes(Buffer.from(CANARY)),
    'broker leaked the canary');
  assert.ok(!stdout.includes(Buffer.from(ownedRoot)) && !stderr.includes(Buffer.from(ownedRoot)),
    'broker leaked the owned root');
  let offset = 0;
  let frames = 0;
  while (offset < stdout.length) {
    assert.ok(offset + 4 <= stdout.length, 'broker left a partial protocol header');
    const length = stdout.readUInt32BE(offset);
    assert.ok(length > 0 && offset + 4 + length <= stdout.length, 'broker left a partial protocol frame');
    try {
      JSON.parse(stdout.subarray(offset + 4, offset + 4 + length).toString('utf8'));
    } catch (_) {
      throw new Error('broker emitted a non-JSON protocol frame');
    }
    offset += 4 + length;
    frames += 1;
  }
  assert.ok(frames >= 6, 'broker protocol transcript was unexpectedly short');
}

async function closeBrokerAfterFailure(context) {
  const record = context.brokerRecord;
  if (!record || record.closed) return;
  if (context.supervisor) {
    try {
      await withDeadline(context.supervisor.shutdown(), 1800, 'supervisor cleanup shutdown timed out');
    } catch (_) { /* bounded signal escalation follows */ }
  }
  if (!record.closed) {
    try { record.child.kill('SIGTERM'); } catch (_) { /* bounded signal escalation follows */ }
    try {
      await waitForBrokerClose(record, 500, 'broker did not close after SIGTERM');
    } catch (_) { /* escalate */ }
  }
  if (!record.closed) {
    try { record.child.kill('SIGKILL'); } catch (_) { /* report through close check */ }
  }
  await waitForBrokerClose(record, CLEANUP_MS, 'broker could not be reaped');
}

async function cleanupContext(context, completedNormally) {
  let cleanupError = null;
  if (!completedNormally) {
    try { await closeBrokerAfterFailure(context); } catch (error) {
      cleanupError = appendFailure(cleanupError, error);
    }
  }
  for (const helper of context.helpers) {
    if (!helper.closed) {
      try { await forceCloseHelper(helper); } catch (error) {
        cleanupError = appendFailure(cleanupError, error);
      }
    }
    if (helper.closed && helper.listeners.length > 0) removeRecordListeners(helper);
  }

  const brokerClosed = !context.brokerRecord || (
    context.brokerRecord.closed && context.brokerRecord.stdinClosed &&
      context.brokerRecord.stdoutClosed && context.brokerRecord.stderrClosed
  );
  const helpersClosed = context.helpers.every((helper) => helper.closed);
  if (context.brokerRecord && brokerClosed) removeRecordListeners(context.brokerRecord);
  if (!brokerClosed || !helpersClosed) {
    cleanupError = appendFailure(
      cleanupError,
      new Error('owned root preserved because child close was not observed')
    );
  } else {
    try { removeOwnedRoot(context.owned); } catch (error) {
      cleanupError = appendFailure(cleanupError, new Error('owned root cleanup failed'));
    }
  }
  if (cleanupError) throw cleanupError;
}

test('actual supervisor opens, hides, reopens, and gracefully closes the native account window', async () => {
  const globalDeadline = Date.now() + TEST_TIMEOUT_MS;
  const inputs = requireInputs();
  const context = {
    env: inputs.env,
    pythonPath: inputs.pythonPath,
    globalDeadline,
    owned: createOwnedRoot(),
    helpers: [],
    spawnCalls: [],
    brokerRecord: null,
    supervisor: null,
  };
  let testError = null;
  let completedNormally = false;
  try {
    context.supervisor = createWalletSupervisor({
      brokerPath: inputs.brokerPath,
      expectedSha256: inputs.expectedSha256,
      dataDir: context.owned.root,
      env: inputs.env,
      system: {
        spawn(file, args, options) {
          context.spawnCalls.push({ file, args, options });
          const child = childProcess.spawn(file, args, options);
          context.brokerRecord = observeBroker(child);
          return child;
        },
      },
    });

    globalDeadlineRemaining(context);
    const started = context.supervisor.start();
    assert.strictEqual(started.ok, true, 'native broker did not start');
    assert.ok(context.brokerRecord, 'native broker was not observed');
    assertSpawn(context, inputs);
    const brokerPid = context.brokerRecord.child.pid;
    assert.ok(Number.isInteger(brokerPid) && brokerPid > 0 && brokerPid <= 0x7fffffff,
      'native broker PID is invalid');
    await waitUntil(
      () => context.supervisor.bound === true,
      BOUND_MS,
      'native broker did not bind',
      context.brokerRecord,
      context
    );

    const listed = await withGlobalOperationDeadline(
      context,
      () => context.supervisor.dispatch('account.list', {}),
      REQUEST_MS,
      'account.list did not settle'
    );
    assert.ok(Array.isArray(listed) && listed.length === 0, 'fresh native account list was not empty');
    const status = await withGlobalOperationDeadline(
      context,
      () => context.supervisor.dispatch('status.get', {}),
      REQUEST_MS,
      'status.get did not settle'
    );
    assert.deepStrictEqual(status, DEGRADED, 'native broker status was not degraded');

    await waitForHiddenInitializedWindow(context);

    await waitForManage(context);
    const firstVisible = await waitForVisibleWindows(
      context, 1, WINDOW_MS, 'native account window did not become visible'
    );
    assert.strictEqual(firstVisible.length, 1, `exactly one ${WINDOW_TITLE} window was required`);
    const firstCapture = await waitForRenderedWindow(context, firstVisible[0].id);
    await closeWindow(context, firstVisible[0].id);
    await waitForVisibleWindows(context, 0, WINDOW_MS, 'native account window did not hide');

    assert.strictEqual(context.supervisor.bound, true, 'window close unbound the broker');
    assert.strictEqual(hasExited(context.brokerRecord.child), false, 'window close exited the broker');
    const afterCloseList = await withGlobalOperationDeadline(
      context,
      () => context.supervisor.dispatch('account.list', {}),
      REQUEST_MS,
      'post-close account.list did not settle'
    );
    assert.ok(Array.isArray(afterCloseList) && afterCloseList.length === 0,
      'post-close native account list was not empty');

    const reopen = await withGlobalOperationDeadline(
      context,
      () => context.supervisor.dispatch('account.manage', {}),
      REQUEST_MS,
      'reopen account.manage did not settle'
    );
    assert.ok(exactKeys(reopen, []), 'reopen account.manage returned an invalid result');
    const reopenedVisible = await waitForVisibleWindows(
      context, 1, WINDOW_MS, 'native account window did not reopen'
    );
    const reopenedCapture = await waitForRenderedWindow(context, reopenedVisible[0].id);
    writeQaArtifact(VISIBLE_ARTIFACT, firstCapture);
    writeQaArtifact(REOPENED_ARTIFACT, reopenedCapture);
    assert.strictEqual(context.spawnCalls.length, 1, 'reopen respawned the native broker');
    assert.strictEqual(context.brokerRecord.child.pid, brokerPid, 'reopen changed the native broker PID');

    await withGlobalOperationDeadline(
      context,
      () => context.supervisor.shutdown(),
      REQUEST_MS,
      'supervisor shutdown did not observe child close'
    );
    assert.strictEqual(context.brokerRecord.closed, true, 'shutdown resolved before child close');
    assert.strictEqual(context.brokerRecord.stdinClosed, true, 'broker stdin did not close');
    assert.strictEqual(context.brokerRecord.stdoutClosed, true, 'broker stdout did not close');
    assert.strictEqual(context.brokerRecord.stderrClosed, true, 'broker stderr did not close');
    assert.strictEqual(context.brokerRecord.child.exitCode, 0, 'native broker did not exit zero');
    assert.strictEqual(context.brokerRecord.child.signalCode, null, 'native broker exited by signal');
    assert.deepStrictEqual(context.brokerRecord.signalCalls, [], 'healthy EOF shutdown forced a signal');
    assert.strictEqual(context.supervisor.bound, false, 'shutdown left the supervisor bound');
    assert.deepStrictEqual(context.supervisor.pendingRequests(), [], 'shutdown left pending requests');

    await waitForVisibleWindows(context, 0, WINDOW_MS, 'native account window survived child close');
    await delayWithinGlobalDeadline(context, 200);
    await waitForVisibleWindows(context, 0, HELPER_MS, 'post-close account window reopened or focused');
    assert.deepStrictEqual(context.brokerRecord.signalCalls, [], 'a late shutdown signal was delivered');
    assertCompleteProtocol(context.brokerRecord, context.owned.root);
    assertOwnedRoot(context.owned);
    completedNormally = true;
  } catch (error) {
    testError = error;
  } finally {
    try {
      await cleanupContext(context, completedNormally);
    } catch (error) {
      testError = appendFailure(testError, error);
    }
  }
  if (testError) throw testError;
});

async function run() {
  let failed = 0;
  for (const entry of tests) {
    try {
      await entry.fn();
      process.stdout.write(`ok ${entry.name}\n`);
    } catch (error) {
      failed += 1;
      process.stderr.write(`not ok ${entry.name}\n${error && error.stack ? error.stack : error}\n`);
    }
  }
  if (failed) {
    process.stderr.write(`${failed} native account window smoke test(s) failed\n`);
    process.exitCode = 1;
    return;
  }
  process.stdout.write(`BitBook native account window smoke tests passed (${tests.length}).\n`);
}

if (require.main === module) {
  run().catch((error) => {
    process.stderr.write(`${error && error.stack ? error.stack : error}\n`);
    process.exitCode = 1;
  });
}

module.exports = { tests };
