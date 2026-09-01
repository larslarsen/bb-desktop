'use strict';

const assert = require('assert');
const fs = require('fs');
const path = require('path');

const repoRoot = path.resolve(__dirname, '..');
const fixturePath = path.join(__dirname, 'fixtures', 'rates', 'provider-bodies-v1.json');
const fixture = JSON.parse(fs.readFileSync(fixturePath, 'utf8'));

const { createQuoteSupervisor } = require('../quote-worker/supervisor');
const { encodeRateFrame, validateRateRequest, validateRateResponse } = require('../quote-worker/framing');
const { PROVIDERS } = require('../quote-worker/providers');
const { buildRateSnapshot, normalizeRateQuery, parseProviderBody } = require('../quote-worker/model');

const tests = [];
function test(name, fn) { tests.push({ name, fn }); }

const FETCHED_AT = fixture.fetched_at;
const FRAME_ID = 'bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb';
const COINBASE_ID = 'coinbase-exchange-v1';
const KRAKEN_ID = 'kraken-spot-v1';
const WORKER_PATH = '/app/resources/quote-worker/worker.js';
const EXEC_PATH = '/usr/bin/node';
const CANARIES = fixture.canaries;
const DISPLAY_UNAVAILABLE = Object.freeze(['asset', 'quote_currency', 'method', 'source_ids', 'label']);

function clone(value) {
  return JSON.parse(JSON.stringify(value));
}

function assertClosedKeys(value, expected) {
  assert.deepStrictEqual(Object.keys(value).sort(), [...expected].sort());
  assert.strictEqual(Object.getPrototypeOf(value), Object.prototype);
}

function assertCode(fn, code) {
  let error;
  try { fn(); } catch (caught) { error = caught; }
  assert.ok(error, `expected ${code}`);
  assert.strictEqual(error.code, code);
}

function independentFrame(value) {
  const body = Buffer.from(JSON.stringify(value), 'utf8');
  const prefix = Buffer.alloc(4);
  prefix.writeUInt32BE(body.length, 0);
  return Buffer.concat([prefix, body]);
}

function assertNoCanary(value, label) {
  const text = Buffer.isBuffer(value) || value instanceof Uint8Array
    ? Buffer.from(value).toString('utf8')
    : JSON.stringify(value);
  for (const [name, canary] of Object.entries(CANARIES)) {
    assert.ok(!text.includes(canary), `${label} leaked ${name}`);
  }
}

function unavailable(asset) {
  return {
    v: 1,
    queried_at: FETCHED_AT,
    quotes: [],
    display: {
      asset,
      quote_currency: 'USD',
      method: 'unavailable',
      source_ids: [],
      label: 'fiat estimate unavailable',
    },
  };
}

function assertUnavailableSnapshot(snapshot, asset) {
  assert.deepStrictEqual(snapshot.quotes, []);
  assert.deepStrictEqual(snapshot.display, unavailable(asset).display);
  assertClosedKeys(snapshot.display, DISPLAY_UNAVAILABLE);
  assert.ok(!Object.prototype.hasOwnProperty.call(snapshot.display, 'price'));
}

function zecQuote() {
  return parseProviderBody(
    COINBASE_ID,
    Buffer.from(fixture.bodies.coinbase_zec_valid.raw_json, 'utf8'),
    FETCHED_AT
  );
}

function createTimerHarness() {
  const timers = new Map();
  let nextId = 1;
  return {
    timers,
    setTimeout(fn, ms) {
      const id = nextId;
      nextId += 1;
      timers.set(id, { fn, ms });
      return id;
    },
    clearTimeout(id) {
      timers.delete(id);
    },
  };
}

function createSpawnHarness() {
  const calls = [];
  const children = [];
  let responder = null;
  function spawn(file, argv, options) {
    const stdinWrites = [];
    const stdoutData = [];
    const stderrData = [];
    const listeners = { exit: [], error: [], close: [] };
    const stdoutListeners = { data: [], end: [], error: [] };
    const stderrListeners = { data: [], end: [], error: [] };
    const child = {
      pid: 52001,
      killed: false,
      stdin: {
        write(buf) {
          const bytes = Buffer.from(buf);
          stdinWrites.push(bytes);
          if (typeof responder === 'function') {
            const reply = responder(bytes, child);
            if (reply) {
              const chunk = Buffer.from(reply);
              stdoutData.push(chunk);
              (stdoutListeners.data || []).forEach((fn) => fn(chunk));
            }
          }
          return true;
        },
        end() {},
      },
      stdout: {
        on(event, fn) {
          stdoutListeners[event] = stdoutListeners[event] || [];
          stdoutListeners[event].push(fn);
          return child.stdout;
        },
      },
      stderr: {
        on(event, fn) {
          stderrListeners[event] = stderrListeners[event] || [];
          stderrListeners[event].push(fn);
          return child.stderr;
        },
      },
      on(event, fn) {
        listeners[event] = listeners[event] || [];
        listeners[event].push(fn);
        return child;
      },
      kill() { child.killed = true; },
      stdinWrites,
      stdoutData,
      stderrData,
      emitStdout(buf) {
        const chunk = Buffer.from(buf);
        stdoutData.push(chunk);
        (stdoutListeners.data || []).forEach((fn) => fn(chunk));
      },
      emitStdoutEnd() {
        (stdoutListeners.end || []).forEach((fn) => fn());
      },
      emitStderr(buf) {
        const chunk = Buffer.from(buf);
        stderrData.push(chunk);
        (stderrListeners.data || []).forEach((fn) => fn(chunk));
      },
    };
    calls.push([file, argv, options]);
    children.push(child);
    return child;
  }
  return {
    spawn,
    calls,
    children,
    setResponder(fn) { responder = fn; },
  };
}

function harness(overrides = {}) {
  const spawnHarness = createSpawnHarness();
  const timers = createTimerHarness();
  const logs = [];
  const supervisor = createQuoteSupervisor(Object.assign({
    workerPath: WORKER_PATH,
    execPath: EXEC_PATH,
    now: () => Date.UTC(2026, 8, 1, 12, 0, 0),
    spawn: spawnHarness.spawn,
    setTimeout: timers.setTimeout,
    clearTimeout: timers.clearTimeout,
    env: Object.assign({
      PATH: '/usr/bin',
      LANG: 'C.UTF-8',
      HTTP_PROXY: CANARIES.proxy,
      HTTPS_PROXY: CANARIES.proxy,
      ALL_PROXY: CANARIES.proxy,
      NO_PROXY: '*',
      API_KEY: CANARIES.api_key,
      COOKIE: CANARIES.cookie,
      SECRET_TOKEN: CANARIES.secret,
      HOME: CANARIES.os_path,
    }, CANARIES),
    log: (line) => logs.push(String(line)),
  }, overrides));
  return { supervisor, spawnHarness, timers, logs };
}

function matchingUnavailableFrame(bytes) {
  const request = decodeOne(bytes);
  return independentFrame({
    v: 1,
    id: request.id,
    kind: 'res',
    method: 'rate.snapshot',
    result: unavailable('ZEC'),
  });
}

function assertKilledUnavailable(ctx, snapshot, asset, label) {
  const child = ctx.spawnHarness.children[0];
  assert.ok(child, `${label} did not spawn`);
  assert.strictEqual(child.killed, true, `${label} left the child running`);
  assertUnavailableSnapshot(snapshot, asset);
  assertNoCanary(snapshot, `${label} snapshot`);
  assertNoCanary(ctx.logs, `${label} logs`);
  ctx.supervisor.shutdown();
  assert.strictEqual(ctx.timers.timers.size, 0);
}

test('module exports exactly createQuoteSupervisor', () => {
  assert.deepStrictEqual(Object.keys(require('../quote-worker/supervisor')).sort(), ['createQuoteSupervisor']);
  assert.strictEqual(typeof createQuoteSupervisor, 'function');
  assert.strictEqual(PROVIDERS[COINBASE_ID].enabled_by_default, false);
  assert.strictEqual(PROVIDERS[KRAKEN_ID].enabled_by_default, false);
});

test('default-off supervisor never spawns, never contacts a provider, and returns a closed unavailable snapshot', () => {
  const ctx = harness();
  const snapshot = ctx.supervisor.query(clone(fixture.queries.zec));
  assertUnavailableSnapshot(snapshot, 'ZEC');
  assert.strictEqual(ctx.spawnHarness.calls.length, 0);
  assert.strictEqual(ctx.spawnHarness.children.length, 0);
  ctx.supervisor.shutdown();
  assert.strictEqual(ctx.timers.timers.size, 0);
  assertNoCanary(snapshot, 'default snapshot');
  assertNoCanary(ctx.logs, 'default logs');
  assertNoCanary(ctx.spawnHarness.calls, 'default spawn');
});

test('enabled supervisor spawns only the reviewed worker path with clean env, pipes, and provider IDs', () => {
  const ctx = harness({ enabledProviders: [COINBASE_ID] });
  ctx.spawnHarness.setResponder((bytes) => matchingUnavailableFrame(bytes));
  const snapshot = ctx.supervisor.query(clone(fixture.queries.zec));
  assert.strictEqual(ctx.spawnHarness.calls.length, 1);
  const [file, argv, options] = ctx.spawnHarness.calls[0];
  assert.strictEqual(file, EXEC_PATH);
  assert.deepStrictEqual(argv, [WORKER_PATH, COINBASE_ID]);
  assert.strictEqual(options.shell, false);
  assert.deepStrictEqual(options.stdio, ['pipe', 'pipe', 'pipe']);
  assert.ok(!options.detached);
  assert.deepStrictEqual(options.env, { LANG: 'C.UTF-8' });
  assert.ok(!Object.prototype.hasOwnProperty.call(options.env, 'HTTP_PROXY'));
  assert.ok(!Object.prototype.hasOwnProperty.call(options.env, 'PATH'));
  assert.ok(!Object.prototype.hasOwnProperty.call(options.env, 'HOME'));
  assert.ok(!JSON.stringify(argv).includes('wallet'));
  assert.ok(!JSON.stringify(options).includes('wallet-broker'));
  assertNoCanary(argv, 'spawn argv');
  assertNoCanary(options, 'spawn options');
  assertUnavailableSnapshot(snapshot, 'ZEC');
  ctx.supervisor.shutdown();
  assert.strictEqual(ctx.spawnHarness.children[0].killed, true);
  assert.strictEqual(ctx.timers.timers.size, 0);
});

test('enabled query writes the exact rate.query frame and accepts the exact rate.snapshot frame', () => {
  const ctx = harness({ enabledProviders: [COINBASE_ID] });
  const query = clone(fixture.queries.zec);
  const quote = zecQuote();
  const snapshot = buildRateSnapshot(normalizeRateQuery(query), [quote], FETCHED_AT);
  ctx.spawnHarness.setResponder((bytes) => {
    const request = validateRateRequest(decodeOne(bytes));
    const response = {
      v: 1,
      id: request.id,
      kind: 'res',
      method: 'rate.snapshot',
      result: snapshot,
    };
    assert.deepStrictEqual(validateRateResponse(clone(response)), response);
    return independentFrame(response);
  });
  const pending = ctx.supervisor.query(query);
  const child = ctx.spawnHarness.children[0];
  assert.strictEqual(child.stdinWrites.length, 1);
  const request = validateRateRequest(decodeOne(child.stdinWrites[0]));
  assert.strictEqual(request.v, 1);
  assert.strictEqual(request.kind, 'req');
  assert.strictEqual(request.method, 'rate.query');
  assert.match(request.id, /^[0-9a-f]{32}$/);
  assert.deepStrictEqual(request.params, fixture.queries.zec);
  const expectedRequest = independentFrame(request);
  assert.deepStrictEqual(child.stdinWrites[0], expectedRequest);
  assert.deepStrictEqual(encodeRateFrame(request), expectedRequest);
  assert.deepStrictEqual(pending, snapshot);
  assert.deepStrictEqual(pending.display.source_ids, [COINBASE_ID]);
  assert.strictEqual(pending.quotes[0].price, fixture.prices.zec);
  assertNoCanary(child.stdinWrites, 'request frames');
  assertNoCanary(pending, 'response snapshot');
  ctx.supervisor.shutdown();
  assert.strictEqual(child.killed, true);
  assert.strictEqual(ctx.timers.timers.size, 0);
});

test('supervisor never mixes broker methods, wallet handles, or private context into the quote child', () => {
  const ctx = harness({
    enabledProviders: [COINBASE_ID, KRAKEN_ID],
    brokerHandle: { dispatch() { throw new Error('broker reached'); } },
    walletDataDir: CANARIES.os_path,
    session: CANARIES.request_id,
  });
  ctx.spawnHarness.setResponder((bytes) => {
    const request = decodeOne(bytes);
    return independentFrame({
      v: 1,
      id: request.id,
      kind: 'res',
      method: 'rate.snapshot',
      result: unavailable('XMR'),
    });
  });
  ctx.supervisor.query(clone(fixture.queries.xmr));
  const [file, argv, options] = ctx.spawnHarness.calls[0];
  assert.strictEqual(file, EXEC_PATH);
  assert.deepStrictEqual(argv, [WORKER_PATH, COINBASE_ID, KRAKEN_ID]);
  assert.ok(!argv.includes(CANARIES.account_id));
  assertNoCanary(argv, 'multi-provider argv');
  assertNoCanary(options.env, 'multi-provider env');
  const request = decodeOne(ctx.spawnHarness.children[0].stdinWrites[0]);
  assert.strictEqual(request.method, 'rate.query');
  assert.ok(!Object.prototype.hasOwnProperty.call(request, 'session'));
  assert.ok(!Object.prototype.hasOwnProperty.call(request.params, 'account_id'));
  for (const method of ['intent.begin', 'intent.confirm', 'account.export-backup', 'rate.fetch', 'status.get']) {
    assertCode(() => validateRateRequest(Object.assign({
      v: 1, id: FRAME_ID, kind: 'req', params: clone(fixture.queries.zec),
    }, { method })), 'SCHEMA');
  }
  ctx.supervisor.shutdown();
});

test('supervisor: unknown or duplicate enabled provider IDs never spawn', () => {
  for (const enabledProviders of [
    ['unknown-v1'],
    [COINBASE_ID, COINBASE_ID],
    [KRAKEN_ID, KRAKEN_ID],
    [COINBASE_ID, 'coinbase-exchange-v1'],
    ['ticker-openbazaar-v1'],
  ]) {
    const spawnHarness = createSpawnHarness();
    const timers = createTimerHarness();
    assertCode(() => createQuoteSupervisor({
      workerPath: WORKER_PATH,
      execPath: EXEC_PATH,
      enabledProviders,
      spawn: spawnHarness.spawn,
      now: () => Date.UTC(2026, 8, 1, 12, 0, 0),
      setTimeout: timers.setTimeout,
      clearTimeout: timers.clearTimeout,
    }), 'SCHEMA');
    assert.strictEqual(spawnHarness.calls.length, 0);
    assert.strictEqual(timers.timers.size, 0);
  }
});

test('supervisor: malformed stdout JSON kills the child and returns no quote', () => {
  const ctx = harness({ enabledProviders: [COINBASE_ID] });
  ctx.spawnHarness.setResponder((bytes, child) => {
    child.emitStderr(Buffer.from(`CANARY ${CANARIES.secret}\n`, 'utf8'));
    const body = Buffer.from('{', 'utf8');
    const prefix = Buffer.alloc(4);
    prefix.writeUInt32BE(body.length, 0);
    child.emitStdout(Buffer.concat([prefix, body]));
    return null;
  });
  const snapshot = ctx.supervisor.query(clone(fixture.queries.zec));
  const child = ctx.spawnHarness.children[0];
  assert.ok(Buffer.concat(child.stderrData).includes(Buffer.from(CANARIES.secret)));
  assertKilledUnavailable(ctx, snapshot, 'ZEC', 'malformed JSON');
});

test('supervisor: oversized frame kills the child and returns no quote', () => {
  const ctx = harness({ enabledProviders: [COINBASE_ID] });
  ctx.spawnHarness.setResponder((bytes, child) => {
    const prefix = Buffer.alloc(4);
    prefix.writeUInt32BE(65537, 0);
    child.emitStdout(prefix);
    return null;
  });
  const snapshot = ctx.supervisor.query(clone(fixture.queries.zec));
  assertKilledUnavailable(ctx, snapshot, 'ZEC', 'oversized frame');
});

test('supervisor: unsolicited wrong-ID response kills the child and returns no quote', () => {
  const ctx = harness({ enabledProviders: [COINBASE_ID] });
  ctx.spawnHarness.setResponder((bytes, child) => {
    child.emitStdout(independentFrame({
      v: 1,
      id: FRAME_ID,
      kind: 'res',
      method: 'rate.snapshot',
      result: unavailable('ZEC'),
    }));
    return null;
  });
  const snapshot = ctx.supervisor.query(clone(fixture.queries.zec));
  const request = decodeOne(ctx.spawnHarness.children[0].stdinWrites[0]);
  assert.notStrictEqual(request.id, FRAME_ID);
  assertKilledUnavailable(ctx, snapshot, 'ZEC', 'wrong ID');
});

test('supervisor: duplicate response ID kills the child and returns no stale quote', () => {
  const ctx = harness({ enabledProviders: [COINBASE_ID] });
  ctx.spawnHarness.setResponder((bytes, child) => {
    const frame = matchingUnavailableFrame(bytes);
    child.emitStdout(frame);
    child.emitStdout(frame);
    return null;
  });
  const snapshot = ctx.supervisor.query(clone(fixture.queries.zec));
  assertKilledUnavailable(ctx, snapshot, 'ZEC', 'duplicate ID');
});

test('supervisor: stdout diagnostics kill the child and keep canaries out of logs and snapshots', () => {
  const ctx = harness({ enabledProviders: [COINBASE_ID] });
  ctx.spawnHarness.setResponder((bytes, child) => {
    child.emitStderr(Buffer.from(`wallet ${CANARIES.os_path} ${CANARIES.secret}\n`, 'utf8'));
    child.emitStdout(Buffer.from(`debug ${CANARIES.api_key}\n`, 'utf8'));
    return null;
  });
  const snapshot = ctx.supervisor.query(clone(fixture.queries.zec));
  const child = ctx.spawnHarness.children[0];
  assert.ok(Buffer.concat(child.stderrData).includes(Buffer.from(CANARIES.secret)));
  assert.ok(Buffer.concat(child.stdoutData).includes(Buffer.from(CANARIES.api_key)));
  assertKilledUnavailable(ctx, snapshot, 'ZEC', 'stdout diagnostics');
});

test('supervisor: partial frame at EOF kills the child and returns no quote', () => {
  const ctx = harness({ enabledProviders: [COINBASE_ID] });
  ctx.spawnHarness.setResponder((bytes, child) => {
    child.emitStdout(Buffer.from([0x00, 0x00]));
    child.emitStdoutEnd();
    return null;
  });
  const snapshot = ctx.supervisor.query(clone(fixture.queries.zec));
  assertKilledUnavailable(ctx, snapshot, 'ZEC', 'partial EOF');
});

test('supervisor: a second pending request overflows, kills the child, and returns no quote', () => {
  const ctx = harness({ enabledProviders: [COINBASE_ID] });
  let nested = null;
  ctx.spawnHarness.setResponder(() => {
    nested = ctx.supervisor.query(clone(fixture.queries.zec));
    return null;
  });
  const first = ctx.supervisor.query(clone(fixture.queries.zec));
  assert.ok(nested);
  assertUnavailableSnapshot(nested, 'ZEC');
  assertUnavailableSnapshot(first, 'ZEC');
  assertKilledUnavailable(ctx, first, 'ZEC', 'pending overflow');
});

test('shutdown terminates the child, clears timers, and leaves no handles', () => {
  const ctx = harness({ enabledProviders: [KRAKEN_ID] });
  ctx.spawnHarness.setResponder((bytes) => {
    const request = decodeOne(bytes);
    return independentFrame({
      v: 1,
      id: request.id,
      kind: 'res',
      method: 'rate.snapshot',
      result: unavailable('XMR'),
    });
  });
  ctx.supervisor.query(clone(fixture.queries.xmr));
  assert.strictEqual(ctx.spawnHarness.children.length, 1);
  assert.strictEqual(ctx.spawnHarness.children[0].killed, false);
  ctx.supervisor.shutdown();
  assert.strictEqual(ctx.spawnHarness.children[0].killed, true);
  assert.strictEqual(ctx.timers.timers.size, 0);
  const again = ctx.supervisor.query(clone(fixture.queries.xmr));
  assertUnavailableSnapshot(again, 'XMR');
});

test('zero enabled providers is the documented default even when PROVIDERS exist', () => {
  const ctx = harness({ enabledProviders: [] });
  assert.deepStrictEqual(Object.keys(PROVIDERS).sort(), [COINBASE_ID, KRAKEN_ID].sort());
  const snapshot = ctx.supervisor.query(clone(fixture.queries.zec));
  assert.strictEqual(ctx.spawnHarness.calls.length, 0);
  assertUnavailableSnapshot(snapshot, 'ZEC');
  ctx.supervisor.shutdown();
});

test('supervisor source may spawn but must not import Electron, wallet, HTTPS, or filesystem', () => {
  const source = fs.readFileSync(path.join(repoRoot, 'quote-worker/supervisor.js'), 'utf8');
  assert.ok(source.trim(), 'quote-worker/supervisor.js is empty');
  for (const specifier of [
    'electron', 'fs', 'node:fs', 'https', 'node:https', 'http', 'net',
    '../wallet-contract', '../wallet-pay/model', '../wallet-broker/protocol',
    '../wallet-broker/supervisor', '../wallet-preload', '../social-main',
  ]) {
    const escaped = specifier.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
    assert.ok(
      !new RegExp(`require\\((['"])${escaped}\\1\\)`).test(source),
      `supervisor imports ${specifier}`
    );
  }
  assert.ok(/child_process|node:child_process/.test(source), 'supervisor does not spawn through child_process');
  assert.ok(/shell:\s*false/.test(source), 'supervisor spawn is missing shell: false');
});

function decodeOne(bytes) {
  const buffer = Buffer.from(bytes);
  const length = buffer.readUInt32BE(0);
  assert.strictEqual(buffer.length, length + 4);
  return JSON.parse(buffer.subarray(4).toString('utf8'));
}

function run() {
  let failed = 0;
  for (const { name, fn } of tests) {
    try {
      fn();
      process.stdout.write(`ok ${name}\n`);
    } catch (error) {
      failed += 1;
      process.stderr.write(`not ok ${name}\n${error && error.stack ? error.stack : error}\n`);
    }
  }
  if (failed) process.exit(1);
  process.stdout.write(`BitBook rate supervisor tests passed (${tests.length}).\n`);
}

if (require.main === module) run();
module.exports = { tests };
