'use strict';

const assert = require('assert');
const { EventEmitter } = require('events');
const fixture = require('./fixtures/wallet-broker/transcript-v1.json');
const payFixture = require('./fixtures/wallet-pay/snapshots-v1.json');
const { sanitizeWalletSnapshot } = require('../wallet-pay/model');
const {
  BROKER_METHODS,
  createBrokerDispatcher,
  createWalletSupervisor,
  sanitizeSnapshot,
} = require('../wallet-broker/supervisor');

const PIN = 'a'.repeat(64);
const BROKER_PATH = '/app/resources/bitbook-wallet-broker';
const DATA_DIR = '/user-data/wallet-broker';
const BOOTSTRAP_ID = '00000000000000000000000000000001';
const BOOTSTRAP_SNAPSHOT = { v: 1, broker: 'ready', accounts: [] };
const SETTLE_MS = 1500;
const tests = [];
function test(name, fn) { tests.push({ name, fn }); }

function withDeadline(promise, ms, message) {
  let timer;
  const timeout = new Promise((_, reject) => {
    timer = setTimeout(() => reject(new Error(message)), ms);
  });
  return Promise.race([Promise.resolve(promise), timeout]).finally(() => clearTimeout(timer));
}

function decodeFrame(buffer) {
  assert.ok(Buffer.isBuffer(buffer), 'stdin write must be a Buffer');
  const length = buffer.readUInt32BE(0);
  assert.strictEqual(buffer.length, 4 + length);
  return JSON.parse(buffer.subarray(4, 4 + length).toString('utf8'));
}

function decodeWrites(writes) {
  return writes.map((write) => decodeFrame(write));
}

function harness(overrides = {}) {
  const calls = [];
  const protocolWrites = [];
  const child = new EventEmitter();
  child.pid = 41002;
  child.stdin = new EventEmitter();
  child.stdin.write = function write(value) {
    protocolWrites.push(value);
    calls.push(['protocol', value]);
    return true;
  };
  child.stdout = new EventEmitter();
  child.stderr = new EventEmitter();
  child.kill = function kill(signal) {
    calls.push(['kill', signal || 'SIGTERM']);
  };
  const system = Object.assign({
    mkdir(path, options) { calls.push(['mkdir', path, options]); },
    lstat(path) {
      calls.push(['lstat', path]);
      if (path === DATA_DIR) return { isDirectory: () => true, isFile: () => false, isSymbolicLink: () => false, mode: 0o700 };
      return { isDirectory: () => false, isFile: () => true, isSymbolicLink: () => false, mode: 0o755 };
    },
    access(path, mode) { calls.push(['access', path, mode]); },
    sha256(path) { calls.push(['sha256', path]); return PIN; },
    spawn(path, argv, options) { calls.push(['spawn', path, argv, options]); return child; },
    setTimeout(fn, ms) { const timer = { fn, ms, cleared: false }; calls.push(['timer', ms, timer]); return timer; },
    clearTimeout(timer) {
      calls.push(['clearTimeout', timer]);
      if (timer) timer.cleared = true;
    },
    now() { return 1000; },
  }, overrides.system);
  return {
    calls, child, system, protocolWrites,
    supervisor: createWalletSupervisor(Object.assign({
      brokerPath: BROKER_PATH, expectedSha256: PIN, dataDir: DATA_DIR,
      env: { PATH: '/usr/bin', LANG: 'C.UTF-8', SECRET_TOKEN: 'CANARY' }, system,
      parentPid: 41001,
      nonce: () => fixture.parent_nonce,
    }, overrides.options)),
  };
}

function bindSupervisor(ctx, result) {
  ctx.supervisor.start();
  assert.strictEqual(ctx.supervisor.receiveProtocol(fixture.hello).ok, true);
  const frames = decodeWrites(ctx.protocolWrites);
  assert.deepStrictEqual(frames[0], fixture.hello_ack);
  assert.strictEqual(frames[1].id, BOOTSTRAP_ID);
  assert.strictEqual(frames[1].seq, 1);
  assert.strictEqual(frames[1].kind, 'req');
  assert.strictEqual(frames[1].method, 'status.get');
  assert.deepStrictEqual(frames[1].params, {});
  assert.strictEqual(frames[1].session, fixture.session_id);
  assert.strictEqual(frames[1].expires_ms, 3000);
  assert.strictEqual(ctx.supervisor.receiveProtocol({
    v: 1,
    id: BOOTSTRAP_ID,
    seq: 1,
    kind: 'res',
    result: result || BOOTSTRAP_SNAPSHOT,
    session: fixture.session_id,
  }).ok, true);
  assert.strictEqual(ctx.supervisor.bound, true);
  return frames[1];
}

function publicFrames(ctx) {
  return decodeWrites(ctx.protocolWrites).filter((message) => message.kind === 'req' && message.id !== BOOTSTRAP_ID);
}

async function expectCode(fn, code) {
  try {
    const result = fn();
    if (result && typeof result.then === 'function') {
      await result;
      assert.fail(`expected ${code}`);
    }
    assert.fail(`expected ${code}`);
  } catch (error) {
    assert.strictEqual(error.code, code);
  }
}

test('launch: private data directory and regular readable pinned binary precede one inert spawn', () => {
  const ctx = harness();
  ctx.supervisor.start();
  assert.deepStrictEqual(ctx.calls.slice(0, 6).map((call) => call[0]), [
    'mkdir', 'lstat', 'lstat', 'access', 'sha256', 'spawn',
  ]);
  assert.deepStrictEqual(ctx.calls[0], ['mkdir', DATA_DIR, { recursive: true, mode: 0o700 }]);
  assert.strictEqual(ctx.calls[1][1], DATA_DIR);
  assert.strictEqual(ctx.calls[2][1], BROKER_PATH);
  const spawn = ctx.calls.find((call) => call[0] === 'spawn');
  assert.strictEqual(spawn[1], BROKER_PATH);
  assert.deepStrictEqual(spawn[2], []);
  assert.strictEqual(spawn[3].cwd, DATA_DIR);
  assert.deepStrictEqual(spawn[3].stdio, ['pipe', 'pipe', 'pipe']);
  assert.strictEqual(spawn[3].shell, false);
  assert.deepStrictEqual(spawn[3].env, { LANG: 'C.UTF-8', PATH: '/usr/bin' });
  assert.ok(!JSON.stringify(spawn).includes('CANARY'));
  assert.ok(ctx.child.stdout.listeners('data').length >= 1);
  assert.ok(ctx.child.stdout.listeners('end').length >= 1);
  assert.ok(ctx.child.stdout.listeners('error').length >= 1);
  assert.ok(ctx.child.stdin.listeners('error').length >= 1);
  assert.ok(ctx.child.stderr.listeners('data').length >= 1);
  assert.ok(ctx.child.stderr.listeners('error').length >= 1);
  assert.ok(ctx.child.listeners('error').length >= 1);
  assert.ok(ctx.child.listeners('exit').length >= 1);
  assert.ok(ctx.child.listeners('close').length >= 1);
  assert.strictEqual(ctx.calls.filter((call) => call[0] === 'spawn').length, 1);
  ctx.supervisor.start();
  assert.strictEqual(ctx.calls.filter((call) => call[0] === 'spawn').length, 1);
});

test('launch: missing, non-file, symlink, unreadable, and hash mismatch never spawn', () => {
  const rows = [
    { lstat(path) { if (path === BROKER_PATH) throw new Error('missing'); return { isDirectory: () => true, isSymbolicLink: () => false, mode: 0o700 }; } },
    { lstat(path) { return path === BROKER_PATH ? { isFile: () => false, isSymbolicLink: () => false } : { isDirectory: () => true, isSymbolicLink: () => false, mode: 0o700 }; } },
    { lstat(path) { return path === BROKER_PATH ? { isFile: () => true, isSymbolicLink: () => true } : { isDirectory: () => true, isSymbolicLink: () => false, mode: 0o700 }; } },
    { access() { throw new Error('unreadable'); } },
    { sha256() { return 'b'.repeat(64); } },
  ];
  for (const system of rows) {
    const ctx = harness({ system });
    const result = ctx.supervisor.start();
    assert.strictEqual(result.ok, false);
    assert.strictEqual(ctx.calls.some((call) => call[0] === 'spawn'), false);
    assert.strictEqual(result.snapshot.broker, 'down');
    assert.ok(!JSON.stringify(result).includes('missing'));
  }
});

test('launch: missing, symlinked, non-directory, or non-0700 data directories never verify or spawn', () => {
  const rows = [
    { lstat(path) { if (path === DATA_DIR) throw new Error('missing data dir'); } },
    { lstat(path) { if (path === DATA_DIR) return { isDirectory: () => true, isSymbolicLink: () => true, mode: 0o700 }; } },
    { lstat(path) { if (path === DATA_DIR) return { isDirectory: () => false, isSymbolicLink: () => false, mode: 0o700 }; } },
    { lstat(path) { if (path === DATA_DIR) return { isDirectory: () => true, isSymbolicLink: () => false, mode: 0o755 }; } },
  ];
  for (const system of rows) {
    const ctx = harness({ system });
    const result = ctx.supervisor.start();
    assert.strictEqual(result.ok, false);
    assert.strictEqual(ctx.calls.some((call) => call[0] === 'sha256'), false);
    assert.strictEqual(ctx.calls.some((call) => call[0] === 'spawn'), false);
  }
});

test('handshake: real child-first fixture transcript binds both directions within two seconds', async () => {
  const ctx = harness();
  const snapshots = [];
  ctx.supervisor.subscribeSnapshot((value) => snapshots.push(value));
  const started = ctx.supervisor.start();
  assert.strictEqual(started.ok, true);
  assert.strictEqual(started.snapshot.broker, 'down');
  assert.strictEqual(ctx.supervisor.bound, false);
  assert.strictEqual(ctx.calls.find((call) => call[0] === 'timer')[1], 2000);
  assert.strictEqual(ctx.supervisor.receiveDiagnostic(JSON.stringify(fixture.hello)), undefined);
  assert.strictEqual(ctx.supervisor.receiveDiagnostic(Buffer.from(JSON.stringify(fixture.hello), 'utf8')), undefined);
  assert.strictEqual(ctx.supervisor.bound, false);
  assert.strictEqual(ctx.protocolWrites.length, 0);
  assert.strictEqual(ctx.supervisor.receiveProtocol(fixture.hello).ok, true);
  const afterHello = decodeWrites(ctx.protocolWrites);
  assert.deepStrictEqual(afterHello[0], fixture.hello_ack);
  assert.strictEqual(afterHello[1].id, BOOTSTRAP_ID);
  assert.strictEqual(afterHello[1].seq, 1);
  assert.strictEqual(afterHello[1].method, 'status.get');
  assert.deepStrictEqual(afterHello[1].params, {});
  assert.strictEqual(ctx.supervisor.sessionId, fixture.session_id);
  await expectCode(() => ctx.supervisor.dispatch('status.get', {}), 'UNAUTH');
  const handshakeTimer = ctx.calls.find((call) => call[0] === 'timer' && call[1] === 2000)[2];
  assert.strictEqual(handshakeTimer.cleared, false);
  assert.strictEqual(ctx.supervisor.receiveProtocol({
    v: 1, id: '11112222333344445555666677778888', seq: 1, kind: 'evt',
    method: 'sync.subscribe',
    params: { snapshot: { v: 1, broker: 'syncing', accounts: [], seed: 'CANARY' } },
    session: fixture.session_id,
  }).ok, true);
  assert.strictEqual(ctx.supervisor.bound, false);
  assert.strictEqual(handshakeTimer.cleared, false);
  assert.ok(!snapshots.some((value) => value.broker === 'ready' || value.broker === 'syncing'));
  await expectCode(() => ctx.supervisor.dispatch('status.get', {}), 'UNAUTH');
  assert.strictEqual(ctx.supervisor.receiveProtocol({
    v: 1,
    id: BOOTSTRAP_ID,
    seq: 2,
    kind: 'res',
    result: BOOTSTRAP_SNAPSHOT,
    session: fixture.session_id,
  }).ok, true);
  assert.strictEqual(ctx.supervisor.bound, true);
  assert.strictEqual(handshakeTimer.cleared, true);
  assert.deepStrictEqual(snapshots[snapshots.length - 1], sanitizeSnapshot(BOOTSTRAP_SNAPSHOT));
  const pending = ctx.supervisor.dispatch('status.get', {});
  assert.ok(pending && typeof pending.then === 'function');
  const publicReq = publicFrames(ctx)[0];
  assert.strictEqual(publicReq.session, fixture.session_id);
  assert.strictEqual(publicReq.id, '00000000000000000000000000000002');
  assert.strictEqual(publicReq.seq, 2);
  assert.strictEqual(ctx.supervisor.receiveProtocol({
    v: 1, id: publicReq.id, seq: 3, kind: 'res',
    result: { v: 1, broker: 'ready', accounts: [] },
    session: fixture.session_id,
  }).ok, true);
  const result = await withDeadline(pending, SETTLE_MS, 'matching status reply did not settle');
  assert.deepStrictEqual(result, sanitizeSnapshot({ v: 1, broker: 'ready', accounts: [] }));
  assert.notDeepStrictEqual(result, { ok: true });
});

test('handshake: PID, session, diagnostics, timeout, and early exit failures never dispatch', async () => {
  const wrongPid = harness();
  wrongPid.supervisor.start();
  assert.strictEqual(wrongPid.supervisor.receiveProtocol(Object.assign({}, fixture.hello, { child_pid: '41003' })).ok, false);
  const wrongSession = harness();
  wrongSession.supervisor.start();
  wrongSession.supervisor.receiveProtocol(Object.assign({}, fixture.hello, { child_nonce: '0'.repeat(32) }));
  const wrongSessionWrites = wrongSession.protocolWrites.length;
  assert.strictEqual(wrongSession.supervisor.receiveProtocol({
    v: 1, id: '11112222333344445555666677778888', seq: 1, kind: 'evt',
    method: 'sync.subscribe', params: {}, session: fixture.session_id,
  }).ok, false);
  for (const ctx of [wrongPid, wrongSession]) {
    const before = ctx.protocolWrites.length;
    await expectCode(() => ctx.supervisor.dispatch('status.get', {}), 'UNAUTH');
    assert.strictEqual(ctx.protocolWrites.length, before);
  }
  assert.strictEqual(wrongSession.protocolWrites.length, wrongSessionWrites);
  const timed = harness();
  timed.supervisor.start();
  timed.calls.find((call) => call[0] === 'timer')[2].fn();
  await expectCode(() => timed.supervisor.dispatch('status.get', {}), 'UNAUTH');
  const exited = harness();
  exited.supervisor.start();
  exited.supervisor.unexpectedExit({});
  await expectCode(() => exited.supervisor.dispatch('status.get', {}), 'UNAUTH');
  const mixed = harness();
  mixed.supervisor.start();
  assert.strictEqual(mixed.supervisor.receiveDiagnostic(fixture.hello).ok, false);
  await expectCode(() => mixed.supervisor.dispatch('status.get', {}), 'UNAUTH');
  assert.strictEqual(mixed.protocolWrites.length, 0);
});

test('dispatch: exact supervisor methods and closed parameter schemas are enforced after binding', () => {
  assert.deepStrictEqual(BROKER_METHODS, [
    'status.get', 'account.list', 'account.lock', 'account.manage', 'receiver.fresh',
    'intent.begin', 'intent.cancel', 'sync.subscribe',
  ]);
  const calls = [];
  const dispatch = createBrokerDispatcher({ bound: () => true, send: (method, params) => { calls.push([method, params]); return { ok: true }; } });
  const positives = [
    ['status.get', {}],
    ['account.list', {}],
    ['sync.subscribe', {}],
    ['account.lock', { account_id: '00112233445566778899aabbccddeeff' }],
    ['account.manage', {}],
    ['receiver.fresh', {
      account_id: '00112233445566778899aabbccddeeff', asset: 'ZEC',
      network: 'zec-testnet', request_id: 'ffeeddccbbaa99887766554433221100',
    }],
    ['receiver.fresh', {
      account_id: '00112233445566778899aabbccddeeff', asset: 'XMR',
      network: 'xmr-stagenet', request_id: 'ffeeddccbbaa99887766554433221100',
    }],
    ['intent.begin', { payment_request: { v: 1, request_id: '00112233445566778899aabbccddeeff' } }],
    ['intent.cancel', { intent_id: '00112233445566778899aabbccddeeff' }],
  ];
  for (const [method, params] of positives) assert.strictEqual(dispatch(method, params).ok, true);
  assert.strictEqual(calls.length, positives.length);
  assert.deepStrictEqual(
    calls.filter(([method]) => method === 'receiver.fresh'),
    positives.filter(([method]) => method === 'receiver.fresh')
  );
  for (const asset of ['ZEC', 'XMR']) {
    assert.strictEqual(
      calls.filter(([method, params]) => method === 'receiver.fresh' && params.asset === asset).length,
      1
    );
  }
  for (const method of ['intent.confirm', 'account.unlock', 'account.exportBackup', 'account.createSoftware', 'signer.sign', 'tx.broadcast', 'intent.broadcast', 'rate.fetch', 'rpc.raw', 'http.proxy']) {
    assert.throws(() => dispatch(method, {}), (error) => error.code === 'SCHEMA');
    assert.strictEqual(calls.length, positives.length);
  }
  const invalid = [
    ['status.get', { extra: true }], ['account.list', []], ['sync.subscribe', null],
    ['account.lock', {}], ['account.lock', { account_id: '0'.repeat(31) }],
    ['account.lock', { account_id: '0'.repeat(32), extra: true }],
    ['account.manage', { extra: true }],
    ['receiver.fresh', {}],
    ['receiver.fresh', { account_id: '0'.repeat(32), asset: 'ZEC', network: 'zec-testnet' }],
    ['receiver.fresh', { account_id: '0'.repeat(32), asset: 'ZEC', network: 'zec-testnet', request_id: '1'.repeat(32), extra: true }],
    ['receiver.fresh', { account_id: 1, asset: 'ZEC', network: 'zec-testnet', request_id: '1'.repeat(32) }],
    ['receiver.fresh', { account_id: '0'.repeat(31), asset: 'ZEC', network: 'zec-testnet', request_id: '1'.repeat(32) }],
    ['receiver.fresh', { account_id: '0'.repeat(32), asset: 'ZEC', network: 'zec-testnet', request_id: 'g'.repeat(32) }],
    ['receiver.fresh', { account_id: '0'.repeat(32), asset: 'BTC', network: 'zec-testnet', request_id: '1'.repeat(32) }],
    ['receiver.fresh', { account_id: '0'.repeat(32), asset: 'ZEC', network: 'xmr-stagenet', request_id: '1'.repeat(32) }],
    ['receiver.fresh', { account_id: '0'.repeat(32), asset: 'XMR', network: 'zec-testnet', request_id: '1'.repeat(32) }],
    ['intent.begin', {}], ['intent.begin', { payment_request: null }],
    ['intent.begin', { payment_request: [] }],
    ['intent.begin', { payment_request: {}, extra: true }],
    ['intent.begin', { payment_request: Object.create(null) }],
    ['intent.cancel', { intent_id: 1 }],
  ];
  const inherited = Object.create({ account_id: '0'.repeat(32) });
  invalid.push(['account.lock', inherited]);
  const accessor = {};
  Object.defineProperty(accessor, 'intent_id', { enumerable: true, get() { assert.fail('accessor invoked'); } });
  invalid.push(['intent.cancel', accessor]);
  let nestedGetterCalls = 0;
  const nestedAccessor = {};
  Object.defineProperty(nestedAccessor, 'request_id', {
    enumerable: true,
    get() { nestedGetterCalls += 1; return '0'.repeat(32); },
  });
  invalid.push(['intent.begin', { payment_request: nestedAccessor }]);
  for (const [method, params] of invalid) {
    assert.throws(() => dispatch(method, params), (error) => error.code === 'SCHEMA');
    assert.strictEqual(calls.length, positives.length);
  }
  assert.strictEqual(nestedGetterCalls, 0);
});

test('dispatch: pre-bind and oversize calls fail before broker send', () => {
  let sends = 0;
  const unbound = createBrokerDispatcher({ bound: () => false, send: () => { sends += 1; } });
  assert.throws(() => unbound('status.get', {}), (error) => error.code === 'UNAUTH');
  const bound = createBrokerDispatcher({ bound: () => true, send: () => { sends += 1; } });
  assert.throws(
    () => bound('intent.begin', { payment_request: { memo: 'x'.repeat(64 * 1024) } }),
    (error) => error.code === 'LIMIT'
  );
  assert.strictEqual(sends, 0);
});

test('dispatch: matching replies settle promises out of order with cloned sanitized results', async () => {
  const ctx = harness();
  bindSupervisor(ctx);
  const firstPayload = { fixture_result: 'first', nested: { n: 1 } };
  const secondPayload = JSON.parse(JSON.stringify(payFixture.valid_full_input));
  const first = ctx.supervisor.dispatch('account.list', {});
  const second = ctx.supervisor.dispatch('status.get', {});
  assert.ok(first && typeof first.then === 'function');
  assert.ok(second && typeof second.then === 'function');
  const reqs = publicFrames(ctx);
  assert.strictEqual(reqs.length, 2);
  assert.strictEqual(reqs[0].seq, 2);
  assert.strictEqual(reqs[1].seq, 3);
  assert.deepStrictEqual(ctx.supervisor.pendingRequests(), [reqs[0].id, reqs[1].id]);
  const pendingCopy = ctx.supervisor.pendingRequests();
  pendingCopy.push('ffff');
  assert.deepStrictEqual(ctx.supervisor.pendingRequests(), [reqs[0].id, reqs[1].id]);
  assert.ok(!ctx.supervisor.pendingRequests().includes(BOOTSTRAP_ID));
  assert.strictEqual(ctx.supervisor.receiveProtocol({
    v: 1, id: reqs[1].id, seq: 2, kind: 'res', result: secondPayload, session: fixture.session_id,
  }).ok, true);
  assert.strictEqual(ctx.supervisor.receiveProtocol({
    v: 1, id: reqs[0].id, seq: 3, kind: 'res', result: firstPayload, session: fixture.session_id,
  }).ok, true);
  const [firstResult, secondResult] = await withDeadline(
    Promise.all([first, second]),
    SETTLE_MS,
    'out-of-order replies did not settle'
  );
  assert.deepStrictEqual(firstResult, firstPayload);
  firstResult.nested.n = 9;
  assert.strictEqual(firstPayload.nested.n, 1);
  assert.deepStrictEqual(secondResult, payFixture.valid_full_expected);
  assert.ok(!JSON.stringify(secondResult).includes('SNAPSHOT_SECRET_CANARY'));
  assert.deepStrictEqual(ctx.supervisor.pendingRequests(), []);
});

test('lifecycle: exit publishes only sanitized down state and restart never buffers spend requests', () => {
  const ctx = harness();
  ctx.supervisor.start();
  const down = ctx.supervisor.unexpectedExit({ stderr: 'CANARY secret', path: '/wallet' });
  assert.deepStrictEqual(down, sanitizeSnapshot({ v: 1, broker: 'down', accounts: [] }));
  assert.ok(!JSON.stringify(down).includes('CANARY'));
  assert.deepStrictEqual(ctx.supervisor.pendingRequests(), []);
  assert.deepStrictEqual(ctx.supervisor.restartDelays(6), [250, 500, 1000, 2000, 4000, 5000]);
  assert.ok(ctx.calls.some((call) => call[0] === 'kill' && call[1] === 'SIGTERM'));
  assert.strictEqual(ctx.child.stdout.listenerCount('data'), 0);
  assert.strictEqual(ctx.child.stderr.listenerCount('data'), 0);
  const spawns = ctx.calls.filter((call) => call[0] === 'spawn').length;
  ctx.supervisor.start();
  assert.strictEqual(ctx.calls.filter((call) => call[0] === 'spawn').length, spawns);
});

test('quit: every in-flight intent is cancelled before child termination', async () => {
  const ctx = harness();
  bindSupervisor(ctx);
  const unhandled = [];
  const onUnhandled = (error) => unhandled.push(error);
  process.on('unhandledRejection', onUnhandled);
  try {
    ctx.supervisor.trackIntent('00112233445566778899aabbccddeeff');
    ctx.supervisor.trackIntent('ffeeddccbbaa99887766554433221100');
    ctx.supervisor.quit();
    await withDeadline(new Promise((resolve) => setTimeout(resolve, 0)), SETTLE_MS, 'quit microtask did not run');
    const frames = decodeWrites(ctx.protocolWrites);
    assert.deepStrictEqual(frames.filter((message) => message.method === 'intent.cancel').map((message) => [message.method, message.params]), [
      ['intent.cancel', { intent_id: '00112233445566778899aabbccddeeff' }],
      ['intent.cancel', { intent_id: 'ffeeddccbbaa99887766554433221100' }],
    ]);
    assert.deepStrictEqual(
      ctx.calls.filter((call) => call[0] === 'protocol' || call[0] === 'kill').slice(-3).map((call) => call[0] === 'kill' ? ['kill', call[1]] : call[0]),
      ['protocol', 'protocol', ['kill', 'SIGTERM']]
    );
    for (const message of frames.filter((entry) => entry.method === 'intent.cancel')) {
      assert.strictEqual(message.session, fixture.session_id);
    }
    assert.deepStrictEqual(unhandled, []);
    assert.deepStrictEqual(ctx.supervisor.pendingRequests(), []);
  } finally {
    process.removeListener('unhandledRejection', onUnhandled);
  }
});

test('quit: an unbound child terminates without any application frame', () => {
  const ctx = harness();
  ctx.supervisor.start();
  ctx.supervisor.trackIntent('00112233445566778899aabbccddeeff');
  ctx.supervisor.quit();
  assert.strictEqual(ctx.protocolWrites.length, 0);
  assert.deepStrictEqual(
    ctx.calls.filter((call) => call[0] === 'protocol' || call[0] === 'kill').map((call) => call[0] === 'kill' ? ['kill', call[1]] : call[0]),
    [['kill', 'SIGTERM']]
  );
});

test('snapshot: supervisor exports the shared Pay sanitizer and removes every fixture canary', () => {
  assert.strictEqual(sanitizeSnapshot, sanitizeWalletSnapshot);
  const input = JSON.parse(JSON.stringify(payFixture.valid_full_input));
  const result = sanitizeSnapshot(input);
  assert.deepStrictEqual(result, payFixture.valid_full_expected);
  assert.notStrictEqual(result.accounts, input.accounts);
  assert.notStrictEqual(result.accounts[0].sync, input.accounts[0].sync);
  result.accounts[0].sync.state = 'changed';
  assert.strictEqual(input.accounts[0].sync.state, 'idle');
  assert.ok(!JSON.stringify(result).includes('SNAPSHOT_SECRET_CANARY'));
  assert.ok(!JSON.stringify(result).includes('u1-forbidden'));
});

test('snapshot: sync publication traverses the shared sanitizer before every subscriber', () => {
  const ctx = harness();
  bindSupervisor(ctx);
  const source = JSON.parse(JSON.stringify(payFixture.valid_full_input));
  const received = [];
  const unsubscribe = ctx.supervisor.subscribeSnapshot((value) => received.push(value));
  assert.strictEqual(ctx.supervisor.receiveProtocol({
    v: 1,
    id: '22223333444455556666777788889999',
    seq: 2,
    kind: 'evt',
    method: 'sync.subscribe',
    params: { snapshot: source },
    session: fixture.session_id,
  }).ok, true);
  assert.deepStrictEqual(received, [payFixture.valid_full_expected]);
  assert.notStrictEqual(received[0], source);
  assert.notStrictEqual(received[0].accounts, source.accounts);
  received[0].accounts[0].label = 'renderer mutation';
  assert.strictEqual(source.accounts[0].label, 'Shielded ZEC');
  assert.ok(!JSON.stringify(received).includes('SNAPSHOT_SECRET_CANARY'));
  assert.ok(!JSON.stringify(received).includes('u1-forbidden'));
  assert.strictEqual(unsubscribe(), true);
  assert.strictEqual(unsubscribe(), false);
});

async function run() {
  let failed = 0;
  for (const { name, fn } of tests) {
    try { await fn(); process.stdout.write(`ok ${name}\n`); }
    catch (error) { failed += 1; process.stderr.write(`not ok ${name}\n${error.stack || error}\n`); }
  }
  if (failed) process.exit(1);
  process.stdout.write(`BitBook wallet supervisor tests passed (${tests.length}).\n`);
}
if (require.main === module) run();
module.exports = { tests };
