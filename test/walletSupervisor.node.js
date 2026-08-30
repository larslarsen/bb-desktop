'use strict';

const assert = require('assert');
const fixture = require('./fixtures/wallet-broker/transcript-v1.json');
const {
  BROKER_METHODS,
  createBrokerDispatcher,
  createWalletSupervisor,
  sanitizeSnapshot,
} = require('../wallet-broker/supervisor');

const PIN = 'a'.repeat(64);
const BROKER_PATH = '/app/resources/bitbook-wallet-broker';
const DATA_DIR = '/user-data/wallet-broker';
const tests = [];
function test(name, fn) { tests.push({ name, fn }); }

function harness(overrides = {}) {
  const calls = [];
  const protocolWrites = [];
  const child = {
    pid: 41002,
    stdin: { kind: 'protocol-in', write(value) { protocolWrites.push(value); calls.push(['protocol', value]); } },
    stdout: { kind: 'protocol-out' }, stderr: { kind: 'diagnostics' },
    terminate() { calls.push(['terminate']); },
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
    setTimeout(fn, ms) { const timer = { fn, ms }; calls.push(['timer', ms, timer]); return timer; },
    clearTimeout() {},
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

function bindSupervisor(ctx) {
  ctx.supervisor.start();
  assert.strictEqual(ctx.supervisor.receiveProtocol(fixture.hello).ok, true);
  assert.strictEqual(ctx.supervisor.receiveProtocol({
    v: 1, id: '11112222333344445555666677778888', seq: 1, kind: 'evt',
    method: 'sync.subscribe', params: {}, session: fixture.session_id,
  }).ok, true);
  assert.strictEqual(ctx.supervisor.dispatch('status.get', {}).ok, true);
  assert.strictEqual(ctx.supervisor.bound, true);
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

test('handshake: real child-first fixture transcript binds both directions within two seconds', () => {
  const ctx = harness();
  ctx.supervisor.start();
  assert.strictEqual(ctx.supervisor.bound, false);
  assert.strictEqual(ctx.calls.find((call) => call[0] === 'timer')[1], 2000);
  assert.strictEqual(ctx.supervisor.receiveDiagnostic(JSON.stringify(fixture.hello)), undefined);
  assert.strictEqual(ctx.supervisor.receiveDiagnostic(Buffer.from(JSON.stringify(fixture.hello), 'utf8')), undefined);
  assert.strictEqual(ctx.supervisor.bound, false);
  assert.strictEqual(ctx.protocolWrites.length, 0);
  assert.strictEqual(ctx.supervisor.receiveProtocol(fixture.hello).ok, true);
  assert.deepStrictEqual(ctx.protocolWrites[0], fixture.hello_ack);
  assert.strictEqual(ctx.supervisor.sessionId, fixture.session_id);
  assert.throws(() => ctx.supervisor.dispatch('status.get', {}), (error) => error.code === 'UNAUTH');
  assert.strictEqual(ctx.supervisor.receiveProtocol({
    v: 1, id: '11112222333344445555666677778888', seq: 1, kind: 'evt',
    method: 'sync.subscribe', params: {}, session: fixture.session_id,
  }).ok, true);
  assert.strictEqual(ctx.supervisor.dispatch('status.get', {}).ok, true);
  assert.strictEqual(ctx.protocolWrites[1].session, fixture.session_id);
  assert.strictEqual(ctx.supervisor.bound, true);
});

test('handshake: PID, session, diagnostics, timeout, and early exit failures never dispatch', () => {
  const wrongPid = harness();
  wrongPid.supervisor.start();
  assert.strictEqual(wrongPid.supervisor.receiveProtocol(Object.assign({}, fixture.hello, { child_pid: '41003' })).ok, false);
  const wrongSession = harness();
  wrongSession.supervisor.start();
  wrongSession.supervisor.receiveProtocol(Object.assign({}, fixture.hello, { child_nonce: '0'.repeat(32) }));
  assert.strictEqual(wrongSession.supervisor.receiveProtocol({
    v: 1, id: '11112222333344445555666677778888', seq: 1, kind: 'evt',
    method: 'sync.subscribe', params: {}, session: fixture.session_id,
  }).ok, false);
  for (const ctx of [wrongPid, wrongSession]) {
    assert.throws(() => ctx.supervisor.dispatch('status.get', {}));
    assert.strictEqual(ctx.protocolWrites.some((message) => message.method === 'status.get'), false);
  }
  const timed = harness();
  timed.supervisor.start();
  timed.calls.find((call) => call[0] === 'timer')[2].fn();
  assert.throws(() => timed.supervisor.dispatch('status.get', {}));
  const exited = harness();
  exited.supervisor.start();
  exited.supervisor.unexpectedExit({});
  assert.throws(() => exited.supervisor.dispatch('status.get', {}));
  const mixed = harness();
  mixed.supervisor.start();
  assert.strictEqual(mixed.supervisor.receiveDiagnostic(fixture.hello).ok, false);
  assert.throws(() => mixed.supervisor.dispatch('status.get', {}));
  assert.strictEqual(mixed.protocolWrites.some((message) => message.method === 'status.get'), false);
});

test('dispatch: exact supervisor methods and closed parameter schemas are enforced after binding', () => {
  assert.deepStrictEqual(BROKER_METHODS, [
    'status.get', 'account.list', 'account.lock', 'receiver.fresh',
    'intent.begin', 'intent.cancel', 'sync.subscribe',
  ]);
  const calls = [];
  const dispatch = createBrokerDispatcher({ bound: () => true, send: (method, params) => { calls.push([method, params]); return { ok: true }; } });
  const positives = [
    ['status.get', {}],
    ['account.list', {}],
    ['sync.subscribe', {}],
    ['account.lock', { account_id: '00112233445566778899aabbccddeeff' }],
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

test('lifecycle: exit publishes only sanitized down state and restart never buffers spend requests', () => {
  const ctx = harness();
  ctx.supervisor.start();
  const down = ctx.supervisor.unexpectedExit({ stderr: 'CANARY secret', path: '/wallet' });
  assert.deepStrictEqual(down, sanitizeSnapshot({ v: 1, broker: 'down', accounts: [] }));
  assert.ok(!JSON.stringify(down).includes('CANARY'));
  assert.deepStrictEqual(ctx.supervisor.pendingRequests(), []);
  assert.deepStrictEqual(ctx.supervisor.restartDelays(6), [250, 500, 1000, 2000, 4000, 5000]);
});

test('quit: every in-flight intent is cancelled before child termination', () => {
  const ctx = harness();
  bindSupervisor(ctx);
  const beforeQuit = ctx.calls.length;
  ctx.supervisor.trackIntent('00112233445566778899aabbccddeeff');
  ctx.supervisor.trackIntent('ffeeddccbbaa99887766554433221100');
  ctx.supervisor.quit();
  assert.deepStrictEqual(ctx.protocolWrites.slice(-2).map((message) => [message.method, message.params]), [
    ['intent.cancel', { intent_id: '00112233445566778899aabbccddeeff' }],
    ['intent.cancel', { intent_id: 'ffeeddccbbaa99887766554433221100' }],
  ]);
  assert.deepStrictEqual(
    ctx.calls.slice(beforeQuit).filter((call) => call[0] === 'protocol' || call[0] === 'terminate').map((call) => call[0]),
    ['protocol', 'protocol', 'terminate']
  );
  for (const message of ctx.protocolWrites.slice(-2)) {
    assert.strictEqual(message.session, fixture.session_id);
  }
});

test('quit: an unbound child terminates without any application frame', () => {
  const ctx = harness();
  ctx.supervisor.start();
  ctx.supervisor.trackIntent('00112233445566778899aabbccddeeff');
  ctx.supervisor.quit();
  assert.strictEqual(ctx.protocolWrites.some((message) => message.method), false);
  assert.deepStrictEqual(
    ctx.calls.filter((call) => call[0] === 'protocol' || call[0] === 'terminate').map((call) => call[0]),
    ['terminate']
  );
});

test('snapshot: keys, receivers, backup, RPC, raw data, and canaries are removed', () => {
  let getterCalls = 0;
  const input = {
    v: 1, broker: 'ready',
    accounts: [{
      account_id: '00112233445566778899aabbccddeeff', label: 'Shielded ZEC', asset: 'ZEC', network: 'zec-testnet',
      kind: 'software', privacy: 'private', capabilities: { can_view: true }, balance_atomic: '0',
      sync: { state: 'idle', progress: 1, secret: 'CANARY' },
      device: { present: false, label: 'Synthetic', verified_fields: [], raw: 'CANARY' },
      receivers: ['secret'], backup: {}, rpc: {}, raw_transaction: 'raw',
    }],
    seed: 'CANARY', receivers: ['secret'], backup: {}, rpc: {}, raw_transaction: 'raw', confirm: () => true,
  };
  Object.defineProperty(input.accounts[0], 'spend_key', { enumerable: true, get() { getterCalls += 1; return 'CANARY'; } });
  const result = sanitizeSnapshot(input);
  assert.deepStrictEqual(result, {
    v: 1, broker: 'ready', accounts: [{
      account_id: '00112233445566778899aabbccddeeff', label: 'Shielded ZEC', asset: 'ZEC', network: 'zec-testnet',
      kind: 'software', privacy: 'private', capabilities: { can_view: true }, balance_atomic: '0',
      sync: { state: 'idle', progress: 1 },
      device: { present: false, label: 'Synthetic', verified_fields: [] },
    }],
  });
  assert.strictEqual(getterCalls, 0);
  assert.notStrictEqual(result.accounts, input.accounts);
  assert.notStrictEqual(result.accounts[0].sync, input.accounts[0].sync);
  result.accounts[0].sync.state = 'changed';
  assert.strictEqual(input.accounts[0].sync.state, 'idle');
  assert.ok(!JSON.stringify(result).includes('CANARY'));
});

function run() {
  let failed = 0;
  for (const { name, fn } of tests) {
    try { fn(); process.stdout.write(`ok ${name}\n`); }
    catch (error) { failed += 1; process.stderr.write(`not ok ${name}\n${error.stack || error}\n`); }
  }
  if (failed) process.exit(1);
  process.stdout.write(`BitBook wallet supervisor tests passed (${tests.length}).\n`);
}
if (require.main === module) run();
module.exports = { tests };
