'use strict';

const assert = require('assert');
const Module = require('module');
const path = require('path');

const preloadPath = path.resolve(__dirname, '..', 'wallet-preload.js');
const CHANNELS = {
  getSnapshot: 'wallet:snapshot:get',
  beginIntent: 'wallet:intent:begin',
  cancelIntent: 'wallet:intent:cancel',
  listAccounts: 'wallet:accounts:list',
  getPayeeRequest: 'wallet:payee-request:get',
  subscribeSnapshot: 'wallet:snapshot:subscribe',
};
const tests = [];
function test(name, fn) { tests.push({ name, fn }); }

function load() {
  const calls = [];
  const listeners = new Map();
  let exposed;
  const electron = {
    contextBridge: { exposeInMainWorld(name, value) { exposed = { name, value }; } },
    ipcRenderer: {
      invoke(channel, ...args) { calls.push(['invoke', channel, ...args]); return Promise.resolve({ ok: true, payload: args[0] }); },
      on(channel, callback) { calls.push(['on', channel]); listeners.set(channel, callback); },
      removeListener(channel, callback) { calls.push(['removeListener', channel]); if (listeners.get(channel) === callback) listeners.delete(channel); },
    },
  };
  const original = Module._load;
  Module._load = function mocked(request, parent, isMain) {
    if (request === 'electron') return electron;
    return original.call(this, request, parent, isMain);
  };
  try { delete require.cache[preloadPath]; require(preloadPath); } finally { Module._load = original; }
  return { calls, listeners, exposed };
}

test('preload: one frozen bitbookWallet object exposes exactly six frozen own functions', () => {
  const ctx = load();
  assert.strictEqual(ctx.exposed.name, 'bitbookWallet');
  assert.strictEqual(Object.isFrozen(ctx.exposed.value), true);
  assert.deepStrictEqual(Object.keys(ctx.exposed.value), [
    'getSnapshot', 'subscribeSnapshot', 'beginIntent', 'cancelIntent', 'listAccounts', 'getPayeeRequest',
  ]);
  for (const value of Object.values(ctx.exposed.value)) {
    assert.strictEqual(typeof value, 'function');
    assert.strictEqual(Object.isFrozen(value), true);
  }
});

test('preload: each callable API uses only its fixed channel and page supplies no method string', async () => {
  const ctx = load();
  await ctx.exposed.value.getSnapshot();
  await ctx.exposed.value.beginIntent({ payment_request: { v: 1, request_id: '0'.repeat(32) } });
  await ctx.exposed.value.cancelIntent({ intent_id: '1'.repeat(32) });
  await ctx.exposed.value.listAccounts();
  await ctx.exposed.value.getPayeeRequest({
    account_id: '2'.repeat(32), asset: 'ZEC', network: 'zec-testnet', request_id: '3'.repeat(32),
  });
  const invokes = ctx.calls.filter((call) => call[0] === 'invoke');
  assert.deepStrictEqual(invokes.map((call) => call[1]), [
    CHANNELS.getSnapshot, CHANNELS.beginIntent, CHANNELS.cancelIntent,
    CHANNELS.listAccounts, CHANNELS.getPayeeRequest,
  ]);
  assert.strictEqual(invokes[0].length, 2, 'getSnapshot smuggled a payload');
  assert.strictEqual(invokes[3].length, 2, 'listAccounts smuggled a payload');
  assert.ok(!Object.keys(ctx.exposed.value).some((key) => /invoke|send|confirm|unlock|backup|sign|broadcast/i.test(key)));
});

test('preload: arguments and results are structured clones immune to caller mutation', async () => {
  const ctx = load();
  const input = { payment_request: { v: 1, request_id: '0'.repeat(32), memo: 'before' } };
  const promise = ctx.exposed.value.beginIntent(input);
  input.payment_request.memo = 'after';
  const sent = ctx.calls.find((call) => call[0] === 'invoke')[2];
  assert.deepStrictEqual(sent, { payment_request: { v: 1, request_id: '0'.repeat(32), memo: 'before' } });
  assert.notStrictEqual(sent, input);
  const result = await promise;
  assert.notStrictEqual(result.payload, sent);
  result.payload.payment_request.memo = 'renderer-change';
  assert.strictEqual(sent.payment_request.memo, 'before');
});

test('preload: invalid callback types register no listener', () => {
  for (const callback of [null, {}, [], 'callback', 1]) {
    const ctx = load();
    assert.throws(() => ctx.exposed.value.subscribeSnapshot(callback), TypeError);
    assert.strictEqual(ctx.calls.some((call) => call[0] === 'on'), false);
  }
});

test('preload: subscription strips event objects, clones values, and unsubscribe is bounded', () => {
  const ctx = load();
  const received = [];
  const unsubscribe = ctx.exposed.value.subscribeSnapshot((value) => { received.push(value); value.broker = 'mutated'; });
  assert.strictEqual(typeof unsubscribe, 'function');
  const listener = ctx.listeners.get(CHANNELS.subscribeSnapshot);
  const source = { v: 1, broker: 'ready', accounts: [] };
  listener({ sender: 'electron-event', secret: 'CANARY' }, source);
  assert.deepStrictEqual(received, [{ v: 1, broker: 'mutated', accounts: [] }]);
  assert.strictEqual(source.broker, 'ready');
  assert.ok(!JSON.stringify(received).includes('electron-event'));
  assert.strictEqual(unsubscribe(), true);
  assert.strictEqual(unsubscribe(), false);
  assert.strictEqual(ctx.calls.filter((call) => call[0] === 'removeListener').length, 1);
});

test('preload: hostile callbacks cannot retain Electron events or widen the bridge', () => {
  const ctx = load();
  let calls = 0;
  ctx.exposed.value.subscribeSnapshot(() => { calls += 1; throw new Error('hostile callback'); });
  const listener = ctx.listeners.get(CHANNELS.subscribeSnapshot);
  assert.doesNotThrow(() => listener(Object.freeze({ sender: {} }), Object.freeze({ v: 1, broker: 'down', accounts: [] })));
  assert.strictEqual(calls, 1);
  assert.strictEqual(Object.getPrototypeOf(ctx.exposed.value), null);
  assert.deepStrictEqual(Object.keys(ctx.exposed.value), Object.getOwnPropertyNames(ctx.exposed.value));
  assert.strictEqual('ipcRenderer' in ctx.exposed.value, false);
  assert.strictEqual('require' in ctx.exposed.value, false);
  assert.strictEqual('process' in ctx.exposed.value, false);
});

async function run() {
  let failed = 0;
  for (const { name, fn } of tests) {
    try { await fn(); process.stdout.write(`ok ${name}\n`); }
    catch (error) { failed += 1; process.stderr.write(`not ok ${name}\n${error.stack || error}\n`); }
  }
  if (failed) process.exit(1);
  process.stdout.write(`BitBook wallet preload tests passed (${tests.length}).\n`);
}
if (require.main === module) run();
module.exports = { tests };
