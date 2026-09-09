'use strict';

const assert = require('assert');
const crypto = require('crypto');
const fs = require('fs');
const os = require('os');
const path = require('path');
const childProcess = require('child_process');
const { EventEmitter } = require('events');

const { sanitizeWalletSnapshot } = require('../wallet-pay/model');
const { createWalletSupervisor } = require('../wallet-broker/supervisor');

const FIXTURE_PATH = path.join(__dirname, 'fixtures', 'wallet-broker', 'transport-child.js');
const BROKER_PATH = fs.realpathSync(process.execPath);
const EXEC_PIN = crypto.createHash('sha256').update(fs.readFileSync(BROKER_PATH)).digest('hex');
const PARENT_NONCE = '00112233445566778899aabbccddeeff';
const CHILD_NONCE = 'ffeeddccbbaa99887766554433221100';
const BOOTSTRAP_ID = '00000000000000000000000000000001';
const SENTINEL = 'WALLET_TRANSPORT_DIAGNOSTIC_SENTINEL';
const READY_MS = 1500;
const SETTLE_MS = 1500;
const FAKE_PIN = 'a'.repeat(64);
const FAKE_BROKER = '/app/resources/bitbook-wallet-broker';
const FAKE_DATA = '/user-data/wallet-broker';
const TRANSCRIPT = require('./fixtures/wallet-broker/transcript-v1.json');

const BOOTSTRAP_SNAPSHOT = { v: 1, broker: 'ready', accounts: [] };
const STATUS_SNAPSHOT = {
  v: 1,
  broker: 'ready',
  accounts: [{
    account_id: '00112233445566778899aabbccddeeff',
    label: 'Transport A',
    asset: 'ZEC',
    network: 'zec-testnet',
    kind: 'software',
    privacy: 'private',
    balance_atomic: '1',
    restored_pool: 'ironwood',
    probed_at: '2026-08-30T12:00:00Z',
    probe_source: 'static_fixture',
  }],
};
const RESULT_ALPHA = { fixture_result: 'alpha', n: 1 };
const RESULT_BETA = { fixture_result: 'beta', n: 2 };

const tests = [];
function test(name, fn) { tests.push({ name, fn }); }

function independentSessionId(parentPid, childPid, parentNonce, childNonce) {
  const preimage = `bitbook-wallet-session-v1\n${parentPid}\n${childPid}\n${parentNonce}\n${childNonce}`;
  return crypto.createHash('sha256').update(Buffer.from(preimage, 'utf8')).digest('hex');
}

function decodeFrames(chunks) {
  const buffers = chunks.map((chunk) => {
    if (!Buffer.isBuffer(chunk)) {
      throw new Error('expected framed Buffer writes, missing live transport framing');
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

function encodeFrame(value) {
  const body = Buffer.from(JSON.stringify(value), 'utf8');
  const header = Buffer.alloc(4);
  header.writeUInt32BE(body.length, 0);
  return Buffer.concat([header, body]);
}

function childHasExited(child) {
  return Boolean(child) && (child.exitCode !== null || child.signalCode !== null);
}

function assertNoDiagnosticLeak(error, label) {
  assert.ok(!Object.prototype.hasOwnProperty.call(error, 'debug'), `${label} copied debug`);
  assert.ok(!Object.prototype.hasOwnProperty.call(error, 'seed'), `${label} copied seed`);
  for (const name of Object.getOwnPropertyNames(error)) {
    const value = error[name];
    if (typeof value === 'string') {
      assert.ok(!value.includes(SENTINEL), `${label} ${name} leaked sentinel`);
    }
  }
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

function waitForExit(child, ms) {
  if (!child) return Promise.resolve();
  if (child.exitCode !== null || child.signalCode !== null) return Promise.resolve();
  return new Promise((resolve, reject) => {
    const timer = setTimeout(() => reject(new Error('child did not exit')), ms);
    child.once('exit', () => {
      clearTimeout(timer);
      resolve();
    });
  });
}

function isClosedCode(code) {
  return ['UNAUTH', 'SCHEMA', 'UNAVAILABLE', 'INTERNAL'].includes(code);
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

function createLiveAdapter(dataDir, mode) {
  const spawnCalls = [];
  const captured = { stdin: [], stdout: [] };
  let spawnError = null;
  let child = null;
  return {
    spawnCalls,
    captured,
    get spawnError() { return spawnError; },
    get child() { return child; },
    spawn(file, argv, options) {
      spawnCalls.push({ file, argv, options });
      try {
        assertSpawnRequest({ file, argv, options }, dataDir);
      } catch (error) {
        spawnError = error;
        throw error;
      }
      child = childProcess.spawn(process.execPath, [FIXTURE_PATH, mode, `child-nonce=${CHILD_NONCE}`], {
        cwd: options.cwd,
        env: options.env,
        shell: false,
        stdio: ['pipe', 'pipe', 'pipe'],
      });
      const originalWrite = child.stdin.write.bind(child.stdin);
      child.stdin.write = function write(chunk, encoding, cb) {
        captured.stdin.push(chunk);
        return originalWrite(chunk, encoding, cb);
      };
      child.stdout.on('data', (chunk) => {
        captured.stdout.push(Buffer.from(chunk));
      });
      return child;
    },
  };
}

async function withLiveSupervisor(mode, fn) {
  const dataDir = fs.mkdtempSync(path.join(os.tmpdir(), 'bb-wal011-'));
  fs.chmodSync(dataDir, 0o700);
  const adapter = createLiveAdapter(dataDir, mode);
  const supervisor = createWalletSupervisor({
    brokerPath: BROKER_PATH,
    expectedSha256: EXEC_PIN,
    dataDir,
    env: { PATH: '/usr/bin', LANG: 'C.UTF-8', SECRET_TOKEN: 'CANARY' },
    parentPid: process.pid,
    nonce: () => PARENT_NONCE,
    system: { spawn: adapter.spawn },
  });
  try {
    return await fn({ supervisor, adapter, dataDir });
  } finally {
    try { supervisor.quit(); } catch (_) { /* teardown */ }
    try { supervisor.unexpectedExit(); } catch (_) { /* teardown */ }
    const child = adapter.child;
    if (child && !childHasExited(child)) {
      try { child.kill('SIGKILL'); } catch (_) { /* already gone */ }
    }
    try { await waitForExit(child, 1000); } catch (_) { /* fallback kill may still be exiting */ }
    if (!child || childHasExited(child)) {
      fs.rmSync(dataDir, { recursive: true, force: true });
    }
  }
}

function waitBound(supervisor) {
  return waitUntil(
    () => supervisor.bound === true,
    READY_MS,
    'missing real framed hello/ack/bootstrap connection'
  );
}

function createFakeHarness() {
  const calls = [];
  const protocolWrites = [];
  const timers = [];
  const child = new EventEmitter();
  child.pid = 41002;
  child.stdin = new EventEmitter();
  child.stdin.write = (value) => {
    protocolWrites.push(value);
    calls.push(['protocol', value]);
    return true;
  };
  child.stdout = new EventEmitter();
  child.stderr = new EventEmitter();
  child.kill = (signal) => { calls.push(['kill', signal || 'SIGTERM']); };
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
    sha256(target) { calls.push(['sha256', target]); return FAKE_PIN; },
    spawn(target, argv, options) {
      calls.push(['spawn', target, argv, options]);
      return child;
    },
    setTimeout(fn, ms) {
      const timer = { fn, ms, cleared: false };
      calls.push(['timer', ms, timer]);
      timers.push(timer);
      return timer;
    },
    clearTimeout(timer) {
      calls.push(['clearTimeout', timer]);
      if (timer) timer.cleared = true;
    },
    now() { return 1000; },
  };
  const supervisor = createWalletSupervisor({
    brokerPath: FAKE_BROKER,
    expectedSha256: FAKE_PIN,
    dataDir: FAKE_DATA,
    env: { PATH: '/usr/bin', LANG: 'C.UTF-8', SECRET_TOKEN: 'CANARY' },
    parentPid: 41001,
    nonce: () => PARENT_NONCE,
    system,
  });
  return { calls, child, protocolWrites, timers, supervisor };
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

function bindFake(ctx, result) {
  const started = ctx.supervisor.start();
  assert.strictEqual(started.ok, true);
  assert.strictEqual(ctx.supervisor.receiveProtocol(TRANSCRIPT.hello).ok, true);
  const frames = decodeWriteList(ctx.protocolWrites);
  assert.deepStrictEqual(frames[0], TRANSCRIPT.hello_ack);
  assert.strictEqual(frames[1].id, BOOTSTRAP_ID);
  assert.strictEqual(frames[1].seq, 1);
  assert.strictEqual(frames[1].method, 'status.get');
  assert.deepStrictEqual(frames[1].params, {});
  assert.strictEqual(frames[1].session, TRANSCRIPT.session_id);
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

function activeTimers(ctx, ms) {
  return ctx.timers.filter((timer) => timer.ms === ms && timer.cleared === false);
}

test('transport: real framed hello, ack, bootstrap and a subsequent request return a distinct fixture result', async () => {
  await withLiveSupervisor('ready', async (ctx) => {
    const snapshots = [];
    ctx.supervisor.subscribeSnapshot((value) => snapshots.push(value));
    const started = ctx.supervisor.start();
    assert.strictEqual(started.ok, true);
    assert.strictEqual(started.snapshot.broker, 'down');
    if (ctx.adapter.spawnError) throw ctx.adapter.spawnError;
    assert.strictEqual(ctx.adapter.spawnCalls.length, 1);
    assertSpawnRequest(ctx.adapter.spawnCalls[0], ctx.dataDir);
    await waitBound(ctx.supervisor);
    const expectedSession = independentSessionId(
      String(process.pid),
      String(ctx.adapter.child.pid),
      PARENT_NONCE,
      CHILD_NONCE
    );
    assert.strictEqual(ctx.supervisor.sessionId, expectedSession);
    const inbound = decodeFrames(ctx.adapter.captured.stdout);
    assert.strictEqual(inbound.values[0].protocol, 'bitbook-wallet-broker');
    assert.strictEqual(inbound.values[0].child_pid, String(ctx.adapter.child.pid));
    assert.strictEqual(inbound.values[0].child_nonce, CHILD_NONCE);
    const outbound = decodeFrames(ctx.adapter.captured.stdin);
    assert.deepStrictEqual(outbound.values[0], {
      protocol: 'bitbook-wallet-broker',
      version: 1,
      parent_nonce: PARENT_NONCE,
      parent_pid: String(process.pid),
    });
    assert.strictEqual(outbound.values[1].v, 1);
    assert.strictEqual(outbound.values[1].id, BOOTSTRAP_ID);
    assert.strictEqual(outbound.values[1].seq, 1);
    assert.strictEqual(outbound.values[1].kind, 'req');
    assert.strictEqual(outbound.values[1].method, 'status.get');
    assert.deepStrictEqual(outbound.values[1].params, {});
    assert.strictEqual(outbound.values[1].session, expectedSession);
    assert.ok(Number.isInteger(outbound.values[1].expires_ms));
    const ready = snapshots.find((value) => value.broker === 'ready');
    assert.ok(ready, 'bootstrap did not publish a ready snapshot');
    assert.deepStrictEqual(ready, sanitizeWalletSnapshot(BOOTSTRAP_SNAPSHOT));
    assert.ok(!JSON.stringify(snapshots).includes(SENTINEL));
    assert.ok(!snapshots.some((value) => value.broker === 'syncing'));
    const pending = ctx.supervisor.dispatch('status.get', {});
    assert.ok(pending && typeof pending.then === 'function', 'public dispatch must return a Promise');
    const result = await withDeadline(
      pending,
      SETTLE_MS,
      'subsequent request did not settle; missing framed matching reply'
    );
    assert.deepStrictEqual(result, sanitizeWalletSnapshot(STATUS_SNAPSHOT));
    assert.notDeepStrictEqual(result, { ok: true });
    assert.notDeepStrictEqual(result, sanitizeWalletSnapshot(BOOTSTRAP_SNAPSHOT));
    const after = decodeFrames(ctx.adapter.captured.stdin).values;
    const publicReq = after.find((value) => value.id !== BOOTSTRAP_ID && value.method === 'status.get');
    assert.ok(publicReq, 'missing public status.get frame');
    assert.strictEqual(publicReq.id, '00000000000000000000000000000002');
    assert.strictEqual(publicReq.seq, 2);
    assert.strictEqual(publicReq.session, expectedSession);
  });
});

test('transport: concurrent requests resolve from reverse-order coalesced replies and broker errors drop diagnostic sentinels', async () => {
  await withLiveSupervisor('reverse', async (ctx) => {
    ctx.supervisor.start();
    if (ctx.adapter.spawnError) throw ctx.adapter.spawnError;
    await waitBound(ctx.supervisor);
    const first = ctx.supervisor.dispatch('account.list', {});
    const second = ctx.supervisor.dispatch('account.list', {});
    assert.ok(first && typeof first.then === 'function');
    assert.ok(second && typeof second.then === 'function');
    const outbound = decodeFrames(ctx.adapter.captured.stdin).values.filter((value) => value.kind === 'req');
    const publicReqs = outbound.filter((value) => value.id !== BOOTSTRAP_ID);
    assert.strictEqual(publicReqs.length, 2);
    const [firstResult, secondResult] = await withDeadline(
      Promise.all([first, second]),
      SETTLE_MS,
      'reverse-order replies did not settle'
    );
    assert.deepStrictEqual(firstResult, RESULT_ALPHA);
    assert.deepStrictEqual(secondResult, RESULT_BETA);
    firstResult.n = 99;
    assert.strictEqual(RESULT_ALPHA.n, 1);
    const locked = ctx.supervisor.dispatch('account.lock', {
      account_id: '00112233445566778899aabbccddeeff',
    });
    const error = await expectReject(
      withDeadline(locked, SETTLE_MS, 'broker error did not settle'),
      'LOCKED',
      'normalized broker error'
    );
    assert.strictEqual(error.message, 'Wallet locked');
    assert.strictEqual(error.retryable, false);
    assert.ok(!JSON.stringify(decodeFrames(ctx.adapter.captured.stdin).values).includes(SENTINEL));
  });
});

test('transport: wrong session or uncorrelated response closes and rejects pending work', async () => {
  await withLiveSupervisor('wrong-session', async (ctx) => {
    ctx.supervisor.start();
    if (ctx.adapter.spawnError) throw ctx.adapter.spawnError;
    await waitBound(ctx.supervisor);
    const pending = ctx.supervisor.dispatch('account.list', {});
    const wrong = await expectReject(
      withDeadline(pending, SETTLE_MS, 'wrong-session reply left a hanging promise'),
      undefined,
      'wrong session'
    );
    assert.ok(isClosedCode(wrong.code), `wrong session code ${wrong.code}`);
    assert.strictEqual(ctx.supervisor.bound, false);
    assert.deepStrictEqual(ctx.supervisor.pendingRequests(), []);
  });
  await withLiveSupervisor('uncorrelated', async (ctx) => {
    ctx.supervisor.start();
    if (ctx.adapter.spawnError) throw ctx.adapter.spawnError;
    await waitBound(ctx.supervisor);
    const pending = ctx.supervisor.dispatch('account.list', {});
    const uncorrelated = await expectReject(
      withDeadline(pending, SETTLE_MS, 'uncorrelated reply left a hanging promise'),
      undefined,
      'uncorrelated response'
    );
    assert.ok(isClosedCode(uncorrelated.code), `uncorrelated code ${uncorrelated.code}`);
    assert.strictEqual(ctx.supervisor.bound, false);
    assert.deepStrictEqual(ctx.supervisor.pendingRequests(), []);
  });
});

test('transport: partial-frame EOF and malformed frames close without a hanging promise', async () => {
  await withLiveSupervisor('partial-hello', async (ctx) => {
    const snapshots = [];
    ctx.supervisor.subscribeSnapshot((value) => snapshots.push(value));
    ctx.supervisor.start();
    if (ctx.adapter.spawnError) throw ctx.adapter.spawnError;
    await waitForExit(ctx.adapter.child, SETTLE_MS);
    assert.ok(snapshots.some((value) => value.broker === 'down'), 'partial hello never published broker-down');
    assert.strictEqual(ctx.supervisor.bound, false);
    await expectCode(() => ctx.supervisor.dispatch('status.get', {}), 'UNAUTH', 'partial hello dispatch');
    assert.deepStrictEqual(ctx.supervisor.pendingRequests(), []);
  });
  await withLiveSupervisor('malformed-hello', async (ctx) => {
    const snapshots = [];
    ctx.supervisor.subscribeSnapshot((value) => snapshots.push(value));
    ctx.supervisor.start();
    if (ctx.adapter.spawnError) throw ctx.adapter.spawnError;
    await waitForExit(ctx.adapter.child, SETTLE_MS);
    assert.ok(snapshots.some((value) => value.broker === 'down'), 'malformed hello never published broker-down');
    assert.strictEqual(ctx.supervisor.bound, false);
    await expectCode(() => ctx.supervisor.dispatch('status.get', {}), 'UNAUTH', 'malformed hello dispatch');
  });
  await withLiveSupervisor('partial-eof', async (ctx) => {
    ctx.supervisor.start();
    if (ctx.adapter.spawnError) throw ctx.adapter.spawnError;
    await waitBound(ctx.supervisor);
    const pending = ctx.supervisor.dispatch('account.list', {});
    const closed = await expectReject(
      withDeadline(pending, SETTLE_MS, 'partial-frame EOF left a hanging promise'),
      undefined,
      'partial EOF'
    );
    assert.ok(isClosedCode(closed.code), `partial EOF code ${closed.code}`);
    assert.strictEqual(ctx.supervisor.bound, false);
    assert.deepStrictEqual(ctx.supervisor.pendingRequests(), []);
  });
  await withLiveSupervisor('malformed', async (ctx) => {
    ctx.supervisor.start();
    if (ctx.adapter.spawnError) throw ctx.adapter.spawnError;
    await waitBound(ctx.supervisor);
    const pending = ctx.supervisor.dispatch('account.list', {});
    const closed = await expectReject(
      withDeadline(pending, SETTLE_MS, 'malformed frame left a hanging promise'),
      undefined,
      'malformed frame'
    );
    assert.ok(isClosedCode(closed.code), `malformed frame code ${closed.code}`);
    assert.strictEqual(ctx.supervisor.bound, false);
    assert.deepStrictEqual(ctx.supervisor.pendingRequests(), []);
  });
});

test('transport: stdout data events reassemble split header/body frames before ack, bootstrap and a distinct result', async () => {
  const ctx = createFakeHarness();
  try {
    const snapshots = [];
    ctx.supervisor.subscribeSnapshot((value) => snapshots.push(value));
    const started = ctx.supervisor.start();
    assert.strictEqual(started.ok, true);
    const helloFrame = encodeFrame(TRANSCRIPT.hello);
    ctx.child.stdout.emit('data', helloFrame.subarray(0, 4));
    assert.strictEqual(ctx.protocolWrites.length, 0, 'ack written before complete hello');
    ctx.child.stdout.emit('data', helloFrame.subarray(4, 6));
    assert.strictEqual(ctx.protocolWrites.length, 0, 'ack written before complete hello');
    ctx.child.stdout.emit('data', helloFrame.subarray(6));
    const afterHello = decodeWriteList(ctx.protocolWrites);
    assert.deepStrictEqual(afterHello[0], TRANSCRIPT.hello_ack);
    assert.strictEqual(afterHello[1].id, BOOTSTRAP_ID);
    assert.strictEqual(afterHello[1].seq, 1);
    assert.strictEqual(afterHello[1].method, 'status.get');
    assert.deepStrictEqual(afterHello[1].params, {});
    assert.strictEqual(ctx.supervisor.bound, false);
    const eventFrame = encodeFrame({
      v: 1,
      id: '11112222333344445555666677778888',
      seq: 1,
      kind: 'evt',
      method: 'sync.subscribe',
      params: { snapshot: { v: 1, broker: 'syncing', accounts: [], seed: SENTINEL } },
      session: TRANSCRIPT.session_id,
    });
    ctx.child.stdout.emit('data', eventFrame.subarray(0, 4));
    assert.strictEqual(ctx.supervisor.bound, false);
    ctx.child.stdout.emit('data', eventFrame.subarray(4));
    assert.strictEqual(ctx.supervisor.bound, false);
    assert.ok(!snapshots.some((value) => value.broker === 'ready' || value.broker === 'syncing'));
    const bootstrapFrame = encodeFrame({
      v: 1,
      id: BOOTSTRAP_ID,
      seq: 2,
      kind: 'res',
      result: BOOTSTRAP_SNAPSHOT,
      session: TRANSCRIPT.session_id,
    });
    ctx.child.stdout.emit('data', bootstrapFrame.subarray(0, 2));
    assert.strictEqual(ctx.supervisor.bound, false);
    ctx.child.stdout.emit('data', bootstrapFrame.subarray(2));
    assert.strictEqual(ctx.supervisor.bound, true);
    assert.deepStrictEqual(snapshots[snapshots.length - 1], sanitizeWalletSnapshot(BOOTSTRAP_SNAPSHOT));
    const pending = ctx.supervisor.dispatch('account.list', {});
    const publicReq = decodeWriteList(ctx.protocolWrites).find((value) => (
      value.kind === 'req' && value.id !== BOOTSTRAP_ID
    ));
    assert.ok(publicReq, 'missing public request after split bootstrap');
    const resultFrame = encodeFrame({
      v: 1,
      id: publicReq.id,
      seq: 3,
      kind: 'res',
      result: RESULT_ALPHA,
      session: TRANSCRIPT.session_id,
    });
    ctx.child.stdout.emit('data', resultFrame.subarray(0, 5));
    ctx.child.stdout.emit('data', resultFrame.subarray(5));
    const result = await withDeadline(pending, SETTLE_MS, 'split result did not settle');
    assert.deepStrictEqual(result, RESULT_ALPHA);
  } finally {
    try { ctx.supervisor.quit(); } catch (_) { /* teardown */ }
    try { ctx.supervisor.unexpectedExit(); } catch (_) { /* teardown */ }
    try { ctx.child.emit('exit', 0, null); } catch (_) { /* teardown */ }
  }
});

test('transport: handshake and request deadlines fail closed', async () => {
  const noHello = createFakeHarness();
  noHello.supervisor.start();
  assert.strictEqual(noHello.supervisor.bound, false);
  const handshake = activeTimers(noHello, 2000);
  assert.strictEqual(handshake.length, 1);
  handshake[0].fn();
  await expectCode(() => noHello.supervisor.dispatch('status.get', {}), 'UNAUTH', 'handshake timeout');
  assert.strictEqual(noHello.supervisor.bound, false);

  const noBootstrap = createFakeHarness();
  noBootstrap.supervisor.start();
  assert.strictEqual(noBootstrap.supervisor.receiveProtocol(TRANSCRIPT.hello).ok, true);
  assert.strictEqual(noBootstrap.supervisor.bound, false);
  const stillOpen = activeTimers(noBootstrap, 2000);
  assert.ok(stillOpen.length >= 1, 'handshake deadline cleared before bootstrap');
  stillOpen[stillOpen.length - 1].fn();
  await expectCode(() => noBootstrap.supervisor.dispatch('status.get', {}), 'UNAUTH', 'bootstrap timeout');
  assert.strictEqual(noBootstrap.supervisor.bound, false);

  const timed = createFakeHarness();
  bindFake(timed);
  const first = timed.supervisor.dispatch('status.get', {});
  const second = timed.supervisor.dispatch('account.list', {});
  assert.ok(first && typeof first.then === 'function');
  const requestTimers = activeTimers(timed, 2000);
  assert.ok(requestTimers.length >= 2, 'missing public request deadlines');
  requestTimers[0].fn();
  await expectReject(first, 'TIMEOUT', 'request timeout');
  await expectReject(second, 'UNAVAILABLE', 'sibling after timeout');
  assert.strictEqual(timed.supervisor.bound, false);
  assert.deepStrictEqual(timed.supervisor.pendingRequests(), []);
});

test('transport: request limit does not write; quit and exit settle pending work and release the child', async () => {
  const limited = createFakeHarness();
  try {
    bindFake(limited);
    const pending = [];
    for (let index = 0; index < 32; index += 1) {
      const request = limited.supervisor.dispatch('account.list', {});
      request.catch(() => {});
      pending.push(request);
    }
    const writes = limited.protocolWrites.length;
    const ids = limited.supervisor.pendingRequests();
    assert.strictEqual(ids.length, 32);
    const copy = limited.supervisor.pendingRequests();
    copy.push('ffff');
    assert.deepStrictEqual(limited.supervisor.pendingRequests(), ids);
    assert.ok(!ids.includes(BOOTSTRAP_ID));
    await expectCode(
      () => limited.supervisor.dispatch('status.get', {}),
      'LIMIT',
      'request limit'
    );
    assert.strictEqual(limited.protocolWrites.length, writes);
    limited.supervisor.quit();
    const settled = await withDeadline(
      Promise.all(pending.map((request) => request.then(
        () => {
          throw new Error('limit request resolved');
        },
        (error) => error
      ))),
      SETTLE_MS,
      'limit requests did not settle after close'
    );
    assert.strictEqual(settled.length, 32);
    for (const error of settled) {
      assert.ok(['UNAVAILABLE', 'CANCELLED'].includes(error.code), `limit pending code ${error.code}`);
      assertNoDiagnosticLeak(error, 'limit pending');
    }
    assert.deepStrictEqual(limited.supervisor.pendingRequests(), []);
    assert.strictEqual(activeTimers(limited, 2000).length, 0);
    limited.child.emit('exit', 0, null);
    assert.ok(limited.timers.filter((timer) => timer.ms === 250).every((timer) => timer.cleared));
  } finally {
    try { limited.supervisor.quit(); } catch (_) { /* teardown */ }
    try { limited.supervisor.unexpectedExit(); } catch (_) { /* teardown */ }
    try { limited.child.emit('exit', 0, null); } catch (_) { /* teardown */ }
  }

  const quitting = createFakeHarness();
  try {
    bindFake(quitting);
    const one = quitting.supervisor.dispatch('account.list', {});
    const two = quitting.supervisor.dispatch('status.get', {});
    one.catch(() => {});
    two.catch(() => {});
    assert.strictEqual(quitting.supervisor.pendingRequests().length, 2);
    quitting.supervisor.quit();
    const quitOne = await expectReject(one, undefined, 'quit first pending');
    const quitTwo = await expectReject(two, undefined, 'quit second pending');
    assert.ok(['UNAVAILABLE', 'CANCELLED'].includes(quitOne.code), `quit first code ${quitOne.code}`);
    assert.ok(['UNAVAILABLE', 'CANCELLED'].includes(quitTwo.code), `quit second code ${quitTwo.code}`);
    assert.deepStrictEqual(quitting.supervisor.pendingRequests(), []);
    assert.strictEqual(activeTimers(quitting, 2000).length, 0);
    assert.ok(quitting.calls.some((call) => call[0] === 'kill' && call[1] === 'SIGTERM'));
    const escalate = quitting.timers.filter((timer) => timer.ms === 250 && timer.cleared === false);
    assert.strictEqual(escalate.length, 1);
    escalate[0].fn();
    assert.ok(quitting.calls.some((call) => call[0] === 'kill' && call[1] === 'SIGKILL'));
    quitting.supervisor.quit();
    quitting.child.stdout.emit('error', new Error('late stdout'));
    quitting.child.stderr.emit('error', new Error('late stderr'));
    quitting.child.stdin.emit('error', new Error('late stdin'));
    quitting.child.emit('error', new Error('late child'));
    assert.deepStrictEqual(quitting.supervisor.pendingRequests(), []);
  } finally {
    try { quitting.supervisor.quit(); } catch (_) { /* teardown */ }
    try { quitting.supervisor.unexpectedExit(); } catch (_) { /* teardown */ }
    try { quitting.child.emit('exit', 0, null); } catch (_) { /* teardown */ }
  }

  const exiting = createFakeHarness();
  try {
    bindFake(exiting);
    const left = exiting.supervisor.dispatch('account.list', {});
    left.catch(() => {});
    exiting.supervisor.unexpectedExit();
    const exitPending = await expectReject(left, undefined, 'exit pending');
    assert.ok(['UNAVAILABLE', 'CANCELLED'].includes(exitPending.code), `exit pending code ${exitPending.code}`);
    assert.deepStrictEqual(exiting.supervisor.pendingRequests(), []);
    assert.strictEqual(exiting.supervisor.bound, false);
    assert.strictEqual(exiting.child.stdout.listenerCount('data'), 0);
    assert.strictEqual(exiting.child.stderr.listenerCount('data'), 0);
  } finally {
    try { exiting.supervisor.quit(); } catch (_) { /* teardown */ }
    try { exiting.supervisor.unexpectedExit(); } catch (_) { /* teardown */ }
    try { exiting.child.emit('exit', 0, null); } catch (_) { /* teardown */ }
  }

  await withLiveSupervisor('hold', async (ctx) => {
    const snapshots = [];
    ctx.supervisor.subscribeSnapshot((value) => snapshots.push(value));
    const started = ctx.supervisor.start();
    assert.strictEqual(started.ok, true);
    if (ctx.adapter.spawnError) throw ctx.adapter.spawnError;
    await waitBound(ctx.supervisor);
    const pending = ctx.supervisor.dispatch('account.list', {});
    pending.catch(() => {});
    assert.strictEqual(ctx.supervisor.pendingRequests().length, 1);
    ctx.supervisor.quit();
    const quitPending = await expectReject(
      withDeadline(pending, SETTLE_MS, 'live quit left a hanging promise'),
      undefined,
      'live quit pending'
    );
    assert.ok(['UNAVAILABLE', 'CANCELLED'].includes(quitPending.code), `live quit code ${quitPending.code}`);
    await waitForExit(ctx.adapter.child, SETTLE_MS);
    assert.ok(childHasExited(ctx.adapter.child), 'supervisor quit did not reap the child');
    assert.ok(snapshots.some((value) => value.broker === 'down'), 'live quit never published broker-down');
    assert.strictEqual(ctx.supervisor.bound, false);
    assert.deepStrictEqual(ctx.supervisor.pendingRequests(), []);
  });
});

async function run() {
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
  process.stdout.write(`BitBook wallet supervisor transport tests passed (${tests.length}).\n`);
}
if (require.main === module) run();
module.exports = { tests };
