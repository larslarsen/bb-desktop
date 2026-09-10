'use strict';

const assert = require('assert');
const crypto = require('crypto');
const fs = require('fs');
const os = require('os');
const path = require('path');
const childProcess = require('child_process');
const { EventEmitter } = require('events');

const { createWalletSupervisor } = require('../wallet-broker/supervisor');

const FIXTURE_PATH = path.join(__dirname, 'fixtures', 'wallet-broker', 'shutdown-child.js');
const BROKER_PATH = fs.realpathSync(process.execPath);
const EXEC_PIN = crypto.createHash('sha256').update(fs.readFileSync(BROKER_PATH)).digest('hex');
const PARENT_NONCE = '00112233445566778899aabbccddeeff';
const CHILD_NONCE = 'ffeeddccbbaa99887766554433221100';
const BOOTSTRAP_ID = '00000000000000000000000000000001';
const INTENT_ID = 'aabbccddeeff00112233445566778899';
const FAKE_PIN = 'a'.repeat(64);
const FAKE_BROKER = '/app/resources/bitbook-wallet-broker';
const FAKE_DATA = '/user-data/wallet-broker';
const OBSERVE_MS = 3000;
const KILL_MS = 250;
const SHUTDOWN_MS = 1500;
const TRANSCRIPT = require('./fixtures/wallet-broker/transcript-v1.json');
const BOOTSTRAP_SNAPSHOT = { v: 1, broker: 'ready', accounts: [] };

const tests = [];
function test(name, fn) { tests.push({ name, fn }); }

function independentSessionId(parentPid, childPid, parentNonce, childNonce) {
  const preimage = `bitbook-wallet-session-v1\n${parentPid}\n${childPid}\n${parentNonce}\n${childNonce}`;
  return crypto.createHash('sha256').update(Buffer.from(preimage, 'utf8')).digest('hex');
}

function decodeFrames(chunks) {
  const buffers = chunks.map((chunk) => {
    if (!Buffer.isBuffer(chunk)) {
      throw new Error('expected framed Buffer writes');
    }
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

function flush() {
  return Promise.resolve().then(() => Promise.resolve());
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

function withDeadline(promise, ms, message) {
  let timer;
  const timeout = new Promise((_, reject) => {
    timer = setTimeout(() => reject(new Error(message)), ms);
  });
  return Promise.race([Promise.resolve(promise), timeout]).finally(() => clearTimeout(timer));
}

function waitUntil(predicate, ms, message) {
  return new Promise((resolve, reject) => {
    try {
      if (predicate()) {
        resolve();
        return;
      }
    } catch (error) {
      reject(error);
      return;
    }
    const start = Date.now();
    const timer = setInterval(() => {
      try {
        if (predicate()) {
          clearInterval(timer);
          resolve();
          return;
        }
      } catch (error) {
        clearInterval(timer);
        reject(error);
        return;
      }
      if (Date.now() - start >= ms) {
        clearInterval(timer);
        reject(new Error(message));
      }
    }, 20);
  });
}

function assertShutdownApi(supervisor) {
  assert.strictEqual(typeof supervisor.shutdown, 'function', 'absent shutdown API');
}

function assertNoDiagnosticLeak(error, label) {
  assert.ok(!Object.prototype.hasOwnProperty.call(error, 'debug'), `${label} copied debug`);
  assert.ok(!Object.prototype.hasOwnProperty.call(error, 'seed'), `${label} copied seed`);
}

async function expectReject(promise, code, label) {
  let error;
  try {
    await promise;
  } catch (caught) {
    error = caught;
  }
  assert.ok(error, `${label} resolved`);
  if (code) assert.strictEqual(error.code, code, `${label} code`);
  assertNoDiagnosticLeak(error, label);
  return error;
}

async function expectCode(fn, code, label) {
  try {
    const result = fn();
    if (result && typeof result.then === 'function') {
      await expectReject(result, code, label);
      return;
    }
    assert.fail(`${label} did not fail`);
  } catch (error) {
    if (error && error.code === code) {
      assertNoDiagnosticLeak(error, label);
      return;
    }
    throw error;
  }
}

function killSignals(ctx) {
  return ctx.calls.filter((call) => call[0] === 'kill').map((call) => call[1]);
}

function activeTimers(ctx, ms) {
  return ctx.timers.filter((timer) => timer.ms === ms && timer.cleared !== true && timer.fired !== true);
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
    sha256(target) {
      calls.push(['sha256', target]);
      return Object.prototype.hasOwnProperty.call(config, 'sha256') ? config.sha256 : FAKE_PIN;
    },
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
    brokerPath: Object.prototype.hasOwnProperty.call(config, 'brokerPath') ? config.brokerPath : FAKE_BROKER,
    expectedSha256: Object.prototype.hasOwnProperty.call(config, 'expectedSha256') ? config.expectedSha256 : FAKE_PIN,
    dataDir: Object.prototype.hasOwnProperty.call(config, 'dataDir') ? config.dataDir : FAKE_DATA,
    env: { PATH: '/usr/bin', LANG: 'C.UTF-8', SECRET_TOKEN: 'CANARY' },
    parentPid: 41001,
    nonce: () => PARENT_NONCE,
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
      teardownError = appendCleanup(
        teardownError,
        new Error(`${pending.length} tracked fake promise(s) remained pending after teardown`)
      );
    }
    const leftoverTimers = clock.timers.filter(
      (timer) => timer.cleared !== true && timer.fired !== true
    );
    if (leftoverTimers.length) {
      teardownError = appendCleanup(
        teardownError,
        new Error(`${leftoverTimers.length} unfired/uncleared fake timer(s) remained after teardown`)
      );
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
      cleanupError = appendCleanup(cleanupError, error);
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
  assert.strictEqual(frames[1].id, BOOTSTRAP_ID);
  assert.strictEqual(frames[1].method, 'status.get');
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
  return frames[1];
}

function assertNoSpawnKillTimer(ctx, label) {
  assert.ok(!ctx.calls.some((call) => call[0] === 'spawn'), `${label} spawned`);
  assert.ok(!ctx.calls.some((call) => call[0] === 'kill'), `${label} killed`);
  assert.ok(!ctx.calls.some((call) => call[0] === 'timer'), `${label} set a timer`);
}

function assertSpawnRequest(call, dataDir) {
  assert.ok(call, 'supervisor never requested a spawn');
  assert.strictEqual(call.file, BROKER_PATH);
  assert.deepStrictEqual(call.argv, []);
  assert.strictEqual(call.options.shell, false);
  assert.deepStrictEqual(call.options.stdio, ['pipe', 'pipe', 'pipe']);
  assert.strictEqual(call.options.cwd, dataDir);
  assert.deepStrictEqual(call.options.env, { LANG: 'C.UTF-8', PATH: '/usr/bin' });
  assert.ok(!JSON.stringify(call).includes('CANARY'));
}

function waitBound(supervisor) {
  return waitUntil(
    () => supervisor.bound === true,
    OBSERVE_MS,
    'fixture handshake did not bind'
  );
}

function emptyCwdError(dataDir, entries) {
  return new Error(`temporary cwd ${dataDir} has unexpected contents: ${entries.join(', ')}`);
}

function removeEmptyCwd(dataDir) {
  const entries = fs.readdirSync(dataDir);
  if (entries.length) throw emptyCwdError(dataDir, entries);
  fs.rmdirSync(dataDir);
}

function childStillRunning(child) {
  return Boolean(child) && child.exitCode === null && child.signalCode === null;
}

function waitForMarkedClose(target, flag, ms, message) {
  if (!target || flag.value) return Promise.resolve();
  return new Promise((resolve, reject) => {
    let settled = false;
    let timer = null;
    const onClose = () => finish();
    const finish = (error) => {
      if (settled) return;
      settled = true;
      if (timer !== null) clearTimeout(timer);
      target.removeListener('close', onClose);
      if (error) reject(error);
      else resolve();
    };
    timer = setTimeout(() => finish(new Error(message)), ms);
    target.on('close', onClose);
    if (flag.value) finish();
  });
}

function appendCleanup(previous, error) {
  if (!previous) return error;
  previous.message = `${previous.message}\ncleanup also failed: ${error.stack || error.message}`;
  return previous;
}

async function withRealChild(argvTail, fn) {
  let originalError = null;
  let cleanupError = null;
  const dataDir = fs.mkdtempSync(path.join(os.tmpdir(), 'bb-wal011-shutdown-'));
  try {
    fs.chmodSync(dataDir, 0o700);
  } catch (error) {
    try { fs.rmdirSync(dataDir); } catch (removeError) {
      error.message = `${error.message}\ncwd remove also failed: ${removeError.message}`;
    }
    throw error;
  }

  let child = null;
  let spawnAllowed = false;
  let spawnError = null;
  const spawnCalls = [];
  const closed = { child: { value: false }, stdout: { value: false }, stderr: { value: false }, stdin: { value: false } };
  const trackedPromises = [];
  const ownedListeners = [];

  function observe(promise) {
    const watched = watch(promise);
    trackedPromises.push(watched);
    return watched;
  }

  function captureClose(target, sink) {
    if (!target) return;
    const handler = (...args) => {
      sink(...args);
    };
    target.on('close', handler);
    ownedListeners.push({ target, handler });
  }

  function clearOwnedObservers() {
    for (const { target, handler } of ownedListeners) {
      target.removeListener('close', handler);
    }
    ownedListeners.length = 0;
  }

  async function settleTracked() {
    if (!trackedPromises.length) return;
    await withDeadline(
      Promise.all(trackedPromises.map((watched) => (
        Promise.resolve(watched.promise).then(() => {}, () => {})
      ))),
      OBSERVE_MS,
      'tracked promise did not settle during cleanup'
    );
  }

  function spawn(file, argv, spawnOptions) {
    spawnCalls.push({ file, argv, options: spawnOptions });
    if (!spawnAllowed) {
      spawnError = new Error('real child spawned before shutdown API assertion');
      throw spawnError;
    }
    try {
      assertSpawnRequest({ file, argv, options: spawnOptions }, dataDir);
    } catch (error) {
      spawnError = error;
      throw error;
    }
    child = childProcess.spawn(BROKER_PATH, [FIXTURE_PATH, ...argvTail], {
      cwd: spawnOptions.cwd,
      env: spawnOptions.env,
      shell: false,
      stdio: ['pipe', 'pipe', 'pipe'],
    });
    child.once('error', (error) => { spawnError = error; });
    child.once('close', () => { closed.child.value = true; });
    if (child.stdout) child.stdout.once('close', () => { closed.stdout.value = true; });
    if (child.stderr) child.stderr.once('close', () => { closed.stderr.value = true; });
    if (child.stdin) child.stdin.once('close', () => { closed.stdin.value = true; });
    return child;
  }

  const supervisor = createWalletSupervisor({
    brokerPath: BROKER_PATH,
    expectedSha256: EXEC_PIN,
    dataDir,
    env: { PATH: '/usr/bin', LANG: 'C.UTF-8', SECRET_TOKEN: 'CANARY' },
    parentPid: process.pid,
    nonce: () => PARENT_NONCE,
    system: { spawn },
  });

  let result;
  try {
    assertShutdownApi(supervisor);
    spawnAllowed = true;
    result = await fn({
      supervisor,
      dataDir,
      spawnCalls,
      closed,
      observe,
      captureClose,
      get child() { return child; },
      get spawnError() { return spawnError; },
    });
  } catch (error) {
    originalError = error;
  } finally {
    try {
      if (childStillRunning(child)) {
        try { child.kill('SIGKILL'); } catch (killError) {
          throw new Error(`emergency SIGKILL failed: ${killError.message}`);
        }
      }
      if (child) {
        await waitForMarkedClose(child, closed.child, OBSERVE_MS, 'failed to reap real child close');
        await waitForMarkedClose(child.stdout, closed.stdout, OBSERVE_MS, 'stdout did not close during cleanup');
        await waitForMarkedClose(child.stderr, closed.stderr, OBSERVE_MS, 'stderr did not close during cleanup');
        await waitForMarkedClose(child.stdin, closed.stdin, OBSERVE_MS, 'stdin did not close during cleanup');
      }
    } catch (error) {
      cleanupError = appendCleanup(cleanupError, error);
    }
    try {
      await settleTracked();
    } catch (error) {
      cleanupError = appendCleanup(cleanupError, error);
    }
    try {
      clearOwnedObservers();
    } catch (error) {
      cleanupError = appendCleanup(cleanupError, error);
    }
    if (spawnError && !originalError) originalError = spawnError;
    const cwdReady = !child || (
      closed.child.value === true &&
      closed.stdout.value === true &&
      closed.stderr.value === true &&
      closed.stdin.value === true
    );
    if (cwdReady) {
      try {
        removeEmptyCwd(dataDir);
      } catch (error) {
        cleanupError = appendCleanup(cleanupError, error);
      }
    } else {
      cleanupError = appendCleanup(
        cleanupError,
        new Error(`preserving cwd ${dataDir}; child or streams were not confirmed closed`)
      );
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

test('shutdown: no-start and failed-start resolve one Promise without spawn, kill, or timers', async () => {
  await withFakeHarness(async (idle) => {
    assertShutdownApi(idle.supervisor);
    const first = idle.supervisor.shutdown();
    assert.ok(first && typeof first.then === 'function', 'shutdown must return a Promise');
    const watched = idle.observe(first);
    await flush();
    assert.strictEqual(first, idle.supervisor.shutdown());
    assert.strictEqual(first, idle.supervisor.shutdown());
    assert.strictEqual(idle.supervisor.quit(), undefined);
    assert.strictEqual(first, idle.supervisor.shutdown());
    assert.strictEqual(watched.state, 'resolved');
    assert.strictEqual(watched.value, undefined);
    assertNoSpawnKillTimer(idle, 'no-start shutdown');
    const started = idle.supervisor.start();
    assert.strictEqual(started.ok, false);
    assert.strictEqual(idle.supervisor.bound, false);
    assertNoSpawnKillTimer(idle, 'start after no-start shutdown');
  });

  await withFakeHarness({ sha256: 'c'.repeat(64) }, async (failed) => {
    assertShutdownApi(failed.supervisor);
    const started = failed.supervisor.start();
    assert.strictEqual(started.ok, false);
    assert.ok(!failed.calls.some((call) => call[0] === 'spawn'), 'failed-start spawned');
    const before = failed.calls.length;
    const first = failed.supervisor.shutdown();
    const watched = failed.observe(first);
    await flush();
    assert.strictEqual(first, failed.supervisor.shutdown());
    assert.strictEqual(watched.state, 'resolved');
    assert.strictEqual(watched.value, undefined);
    assert.ok(!failed.calls.slice(before).some((call) => call[0] === 'spawn'), 'failed-start shutdown spawned');
    assert.ok(!failed.calls.slice(before).some((call) => call[0] === 'kill'), 'failed-start shutdown killed');
    assert.ok(!failed.calls.slice(before).some((call) => call[0] === 'timer'), 'failed-start shutdown set a timer');
    const again = failed.supervisor.start();
    assert.strictEqual(again.ok, false);
    assert.ok(!failed.calls.some((call) => call[0] === 'spawn'), 'start after failed-start shutdown spawned');
  });
});

test('shutdown: bound cancel precedes SIGTERM and close, not exit, completes one Promise', async () => {
  await withFakeHarness(async (ctx) => {
    assertShutdownApi(ctx.supervisor);
    const snapshots = [];
    ctx.supervisor.subscribeSnapshot((value) => snapshots.push(value));
    bindFake(ctx);
    ctx.supervisor.trackIntent(INTENT_ID);
    const pending = ctx.supervisor.dispatch('account.list', {});
    const pendingWatch = ctx.observe(pending);
    assert.strictEqual(ctx.supervisor.pendingRequests().length, 1);

    const first = ctx.supervisor.shutdown();
    const shutdownWatch = ctx.observe(first);
    assert.strictEqual(first, ctx.supervisor.shutdown());
    assert.strictEqual(ctx.supervisor.quit(), undefined);
    assert.strictEqual(first, ctx.supervisor.shutdown());
    await flush();

    const frames = decodeWriteList(ctx.protocolWrites);
    const cancel = frames.find((frame) => frame.kind === 'req' && frame.method === 'intent.cancel');
    assert.ok(cancel, 'missing intent.cancel frame');
    assert.strictEqual(cancel.params.intent_id, INTENT_ID);
    const cancelIndex = ctx.calls.findIndex((call) => (
      call[0] === 'protocol' && Buffer.isBuffer(call[1]) &&
      decodeFrames([call[1]]).values.some((frame) => frame.method === 'intent.cancel')
    ));
    const termIndex = ctx.calls.findIndex((call) => call[0] === 'kill' && call[1] === 'SIGTERM');
    assert.ok(cancelIndex >= 0, 'cancel was not written');
    assert.ok(termIndex >= 0, 'missing SIGTERM');
    assert.ok(cancelIndex < termIndex, 'SIGTERM preceded intent.cancel');
    assert.deepStrictEqual(killSignals(ctx), ['SIGTERM']);

    assert.strictEqual(pendingWatch.state, 'rejected');
    assert.strictEqual(pendingWatch.error.code, 'UNAVAILABLE');
    assert.strictEqual(pendingWatch.error.message, 'Unavailable');
    assert.strictEqual(pendingWatch.error.retryable, true);
    assert.deepStrictEqual(ctx.supervisor.pendingRequests(), []);
    assert.strictEqual(ctx.supervisor.bound, false);
    await expectCode(() => ctx.supervisor.dispatch('status.get', {}), 'UNAUTH', 'dispatch after shutdown');
    assert.strictEqual(snapshots.filter((value) => value.broker === 'down').length, 1);
    assert.strictEqual(shutdownWatch.state, 'pending');

    const closeListeners = ctx.child.listenerCount('close');
    ctx.supervisor.shutdown();
    ctx.supervisor.quit();
    assert.strictEqual(ctx.supervisor.shutdown(), first);
    assert.strictEqual(ctx.child.listenerCount('close'), closeListeners);
    assert.deepStrictEqual(killSignals(ctx), ['SIGTERM']);
    assert.strictEqual(
      decodeWriteList(ctx.protocolWrites).filter((frame) => frame.method === 'intent.cancel').length,
      1
    );

    ctx.child.exitCode = 0;
    ctx.child.killed = true;
    ctx.child.emit('exit', 0, null);
    await flush();
    assert.strictEqual(shutdownWatch.state, 'pending');
    ctx.child.emit('close', 0, null);
    await flush();
    assert.strictEqual(shutdownWatch.state, 'resolved');
    assert.strictEqual(shutdownWatch.value, undefined);
    assert.strictEqual(snapshots.filter((value) => value.broker === 'down').length, 1);
    assert.strictEqual(activeTimers(ctx, KILL_MS).length, 0);
    assert.strictEqual(activeTimers(ctx, SHUTDOWN_MS).length, 0);
    assert.strictEqual(ctx.supervisor.shutdown(), first);
  });
});

test('shutdown: prior quit, protocol failure, and close-before-shutdown keep one termination', async () => {
  await withFakeHarness(async (afterQuit) => {
    assertShutdownApi(afterQuit.supervisor);
    bindFake(afterQuit);
    afterQuit.supervisor.trackIntent(INTENT_ID);
    const pending = afterQuit.supervisor.dispatch('account.list', {});
    const pendingWatch = afterQuit.observe(pending);
    assert.strictEqual(afterQuit.supervisor.quit(), undefined);
    await flush();
    assert.strictEqual(pendingWatch.state, 'rejected');
    assert.strictEqual(pendingWatch.error.code, 'UNAVAILABLE');
    assert.deepStrictEqual(killSignals(afterQuit), ['SIGTERM']);
    const cancelCount = decodeWriteList(afterQuit.protocolWrites)
      .filter((frame) => frame.method === 'intent.cancel').length;
    assert.strictEqual(cancelCount, 1);
    const first = afterQuit.supervisor.shutdown();
    const shutdownWatch = afterQuit.observe(first);
    await flush();
    assert.strictEqual(shutdownWatch.state, 'pending');
    assert.strictEqual(first, afterQuit.supervisor.shutdown());
    assert.deepStrictEqual(killSignals(afterQuit), ['SIGTERM']);
    assert.strictEqual(
      decodeWriteList(afterQuit.protocolWrites).filter((frame) => frame.method === 'intent.cancel').length,
      1
    );
    afterQuit.child.emit('close', 0, null);
    await flush();
    assert.strictEqual(shutdownWatch.state, 'resolved');
    assert.strictEqual(shutdownWatch.value, undefined);
  });

  await withFakeHarness(async (afterProtocol) => {
    assertShutdownApi(afterProtocol.supervisor);
    bindFake(afterProtocol);
    const failed = afterProtocol.supervisor.receiveProtocol({
      v: 1,
      id: '99990000111122223333444455556666',
      seq: 99,
      kind: 'res',
      result: { ok: true },
      session: TRANSCRIPT.session_id,
    });
    assert.strictEqual(failed.ok, false);
    assert.deepStrictEqual(killSignals(afterProtocol), ['SIGTERM']);
    const first = afterProtocol.supervisor.shutdown();
    const shutdownWatch = afterProtocol.observe(first);
    await flush();
    assert.strictEqual(shutdownWatch.state, 'pending');
    assert.deepStrictEqual(killSignals(afterProtocol), ['SIGTERM']);
    afterProtocol.child.emit('close', 1, null);
    await flush();
    assert.strictEqual(shutdownWatch.state, 'resolved');
    assert.strictEqual(first, afterProtocol.supervisor.shutdown());
  });

  await withFakeHarness(async (alreadyClosed) => {
    assertShutdownApi(alreadyClosed.supervisor);
    bindFake(alreadyClosed);
    alreadyClosed.child.emit('close', 0, null);
    await flush();
    const first = alreadyClosed.supervisor.shutdown();
    const shutdownWatch = alreadyClosed.observe(first);
    await flush();
    assert.strictEqual(shutdownWatch.state, 'resolved');
    assert.strictEqual(shutdownWatch.value, undefined);
    assert.strictEqual(first, alreadyClosed.supervisor.shutdown());
    assert.deepStrictEqual(killSignals(alreadyClosed), []);
    assert.strictEqual(activeTimers(alreadyClosed, KILL_MS).length, 0);
    assert.strictEqual(activeTimers(alreadyClosed, SHUTDOWN_MS).length, 0);
  });

  await withFakeHarness(async (syncClose) => {
    assertShutdownApi(syncClose.supervisor);
    bindFake(syncClose);
    syncClose.child.kill = (signal) => {
      syncClose.calls.push(['kill', signal || 'SIGTERM']);
      syncClose.child.killed = true;
      syncClose.child.emit('close', 0, signal || 'SIGTERM');
      return true;
    };
    const first = syncClose.supervisor.shutdown();
    const shutdownWatch = syncClose.observe(first);
    await flush();
    assert.strictEqual(shutdownWatch.state, 'resolved');
    assert.strictEqual(shutdownWatch.value, undefined);
    assert.strictEqual(first, syncClose.supervisor.shutdown());
    assert.deepStrictEqual(killSignals(syncClose), ['SIGTERM']);
    assert.strictEqual(activeTimers(syncClose, KILL_MS).length, 0);
    assert.strictEqual(activeTimers(syncClose, SHUTDOWN_MS).length, 0);
  });
});

test('shutdown: stubborn child SIGKILL at 250 ms and TIMEOUT at 1500 ms', async () => {
  await withFakeHarness(async (ctx) => {
    assertShutdownApi(ctx.supervisor);
    bindFake(ctx);
    let testCloses = 0;
    const onTestClose = () => { testCloses += 1; };
    ctx.child.on('close', onTestClose);
    const ownedClose = ctx.child.listenerCount('close');
    const ownedExit = ctx.child.listenerCount('exit');

    const first = ctx.supervisor.shutdown();
    const shutdownWatch = ctx.observe(first);
    await flush();
    assert.strictEqual(shutdownWatch.state, 'pending');
    assert.deepStrictEqual(killSignals(ctx), ['SIGTERM']);
    assert.strictEqual(ctx.child.killed, true);

    ctx.advance(249);
    await flush();
    assert.deepStrictEqual(killSignals(ctx), ['SIGTERM']);
    assert.strictEqual(shutdownWatch.state, 'pending');
    assert.strictEqual(activeTimers(ctx, KILL_MS).length, 1);

    ctx.advance(1);
    await flush();
    assert.deepStrictEqual(killSignals(ctx), ['SIGTERM', 'SIGKILL']);
    ctx.child.exitCode = null;
    ctx.child.signalCode = 'SIGKILL';
    ctx.child.killed = true;
    ctx.child.emit('exit', null, 'SIGKILL');
    ctx.child.stdout.emit('end');
    ctx.child.stderr.emit('end');
    ctx.child.stdin.emit('error', new Error('stdin closed'));
    await flush();
    assert.strictEqual(shutdownWatch.state, 'pending');

    ctx.advance(1249);
    await flush();
    assert.strictEqual(shutdownWatch.state, 'pending');
    assert.strictEqual(activeTimers(ctx, SHUTDOWN_MS).length, 1);

    ctx.advance(1);
    await flush();
    assert.strictEqual(shutdownWatch.state, 'rejected');
    assert.strictEqual(shutdownWatch.error.code, 'TIMEOUT');
    assert.strictEqual(shutdownWatch.error.message, 'Timed out');
    assert.strictEqual(shutdownWatch.error.retryable, true);
    assertNoDiagnosticLeak(shutdownWatch.error, 'shutdown timeout');
    assert.strictEqual(activeTimers(ctx, KILL_MS).length, 0);
    assert.strictEqual(activeTimers(ctx, SHUTDOWN_MS).length, 0);
    assert.strictEqual(ctx.supervisor.shutdown(), first);
    assert.strictEqual(ctx.child.listenerCount('close'), ownedClose);
    assert.strictEqual(ctx.child.listenerCount('exit'), ownedExit);

    ctx.supervisor.shutdown();
    ctx.supervisor.quit();
    assert.strictEqual(ctx.child.listenerCount('close'), ownedClose);
    assert.strictEqual(ctx.supervisor.shutdown(), first);
    assert.strictEqual(shutdownWatch.state, 'rejected');

    ctx.child.emit('close', null, 'SIGKILL');
    await flush();
    assert.strictEqual(testCloses, 1);
    assert.strictEqual(shutdownWatch.state, 'rejected');
    assert.strictEqual(shutdownWatch.error.code, 'TIMEOUT');
    assert.strictEqual(ctx.supervisor.shutdown(), first);
    assert.strictEqual(ctx.child.listenerCount('close'), ownedClose);
  });
});

test('shutdown: real child normal termination observes close and closed streams', async () => {
  await withRealChild(['hold', `child-nonce=${CHILD_NONCE}`], async (ctx) => {
    const started = ctx.supervisor.start();
    assert.strictEqual(started.ok, true);
    if (ctx.spawnError) throw ctx.spawnError;
    assert.strictEqual(ctx.spawnCalls.length, 1);
    assertSpawnRequest(ctx.spawnCalls[0], ctx.dataDir);
    await waitBound(ctx.supervisor);
    const expectedSession = independentSessionId(
      String(process.pid),
      String(ctx.child.pid),
      PARENT_NONCE,
      CHILD_NONCE
    );
    assert.strictEqual(ctx.supervisor.sessionId, expectedSession);
    const pending = ctx.supervisor.dispatch('account.list', {});
    const pendingWatch = ctx.observe(pending);
    assert.strictEqual(ctx.supervisor.pendingRequests().length, 1);

    const closeFacts = { child: null, stdout: null, stderr: null, stdin: null };
    let shutdownWatch = null;
    ctx.captureClose(ctx.child, () => {
      closeFacts.child = {
        pendingState: pendingWatch.state,
        shutdownState: shutdownWatch ? shutdownWatch.state : null,
      };
    });
    ctx.captureClose(ctx.child.stdout, () => {
      closeFacts.stdout = { shutdownState: shutdownWatch ? shutdownWatch.state : null };
    });
    ctx.captureClose(ctx.child.stderr, () => {
      closeFacts.stderr = { shutdownState: shutdownWatch ? shutdownWatch.state : null };
    });
    ctx.captureClose(ctx.child.stdin, () => {
      closeFacts.stdin = { shutdownState: shutdownWatch ? shutdownWatch.state : null };
    });

    shutdownWatch = ctx.observe(ctx.supervisor.shutdown());
    const result = await withDeadline(
      shutdownWatch.promise,
      OBSERVE_MS,
      'real child shutdown did not fulfill after close'
    );
    assert.strictEqual(result, undefined);
    assert.strictEqual(shutdownWatch.state, 'resolved');
    assert.ok(closeFacts.child, 'shutdown fulfilled without observing child close');
    assert.strictEqual(closeFacts.child.pendingState, 'rejected', 'public request still pending at close');
    assert.strictEqual(closeFacts.child.shutdownState, 'pending', 'shutdown fulfilled before child close');
    assert.ok(closeFacts.stdout, 'stdout was not closed');
    assert.strictEqual(closeFacts.stdout.shutdownState, 'pending', 'shutdown fulfilled before stdout close');
    assert.ok(closeFacts.stderr, 'stderr was not closed');
    assert.strictEqual(closeFacts.stderr.shutdownState, 'pending', 'shutdown fulfilled before stderr close');
    assert.ok(closeFacts.stdin, 'stdin was not closed');
    assert.strictEqual(closeFacts.stdin.shutdownState, 'pending', 'shutdown fulfilled before stdin close');
    assert.ok(ctx.closed.child.value, 'harness did not observe child close');
    assert.strictEqual(pendingWatch.state, 'rejected');
    assert.strictEqual(pendingWatch.error.code, 'UNAVAILABLE');
    assert.strictEqual(ctx.supervisor.bound, false);
    assert.ok(!childStillRunning(ctx.child), 'real child still running after shutdown');
    assert.deepStrictEqual(fs.readdirSync(ctx.dataDir), []);
    const again = ctx.supervisor.start();
    assert.strictEqual(again.ok, false);
  });
});

if (process.platform !== 'win32') {
  test('shutdown: real child ignoring SIGTERM terminates with SIGKILL after close', async () => {
    await withRealChild(
      ['hold', `child-nonce=${CHILD_NONCE}`, '--ignore-sigterm'],
      async (ctx) => {
        const started = ctx.supervisor.start();
        assert.strictEqual(started.ok, true);
        if (ctx.spawnError) throw ctx.spawnError;
        await waitBound(ctx.supervisor);
        assert.ok(childStillRunning(ctx.child), 'child exited before ignored-SIGTERM shutdown');

        let closeFacts = null;
        let shutdownWatch = null;
        ctx.captureClose(ctx.child, (code, signal) => {
          closeFacts = {
            signal,
            shutdownState: shutdownWatch ? shutdownWatch.state : null,
          };
        });
        shutdownWatch = ctx.observe(ctx.supervisor.shutdown());
        const result = await withDeadline(
          shutdownWatch.promise,
          OBSERVE_MS,
          'SIGTERM-ignore child did not complete shutdown after close'
        );
        assert.strictEqual(result, undefined);
        assert.strictEqual(shutdownWatch.state, 'resolved');
        assert.ok(closeFacts, 'shutdown fulfilled without observing ignored-SIGTERM close');
        assert.strictEqual(closeFacts.shutdownState, 'pending', 'shutdown fulfilled before ignored-SIGTERM close');
        assert.strictEqual(closeFacts.signal, 'SIGKILL');
        assert.strictEqual(ctx.child.signalCode, 'SIGKILL');
        assert.ok(ctx.closed.child.value, 'harness did not observe child close');
        assert.ok(!childStillRunning(ctx.child), 'SIGTERM-ignore child still running');
        assert.deepStrictEqual(fs.readdirSync(ctx.dataDir), []);
      }
    );
  });
}

async function run() {
  if (process.platform === 'win32') {
    process.stdout.write('skip shutdown: real child ignoring SIGTERM (Windows; not claimed as passed)\n');
  }
  let failed = 0;
  for (const { name, fn } of tests) {
    try {
      await fn();
      process.stdout.write(`ok ${name}\n`);
    } catch (error) {
      failed += 1;
      process.stderr.write(`not ok ${name}\n${error.stack || error}\n`);
    }
  }
  if (failed) process.exit(1);
  process.stdout.write(`BitBook wallet supervisor shutdown tests passed (${tests.length}).\n`);
}
if (require.main === module) run();
module.exports = { tests };
