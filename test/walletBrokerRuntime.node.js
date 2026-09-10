'use strict';

const assert = require('assert');
const crypto = require('crypto');
const fs = require('fs');
const os = require('os');
const path = require('path');
const childProcess = require('child_process');

const { createWalletSupervisor } = require('../wallet-broker/supervisor');

const DEFAULT_BROKER_PATH = path.join(
  __dirname, '..', 'wallet-broker', 'target', 'debug', 'bitbook-wallet-broker'
);
const MISSING_EXECUTABLE = 'native wallet broker executable is missing';
const CANARY = 'WALLET_RUNTIME_CANARY_SECRET';
const SESSION_DOMAIN = 'bitbook-wallet-session-v1\n';
const HEX32 = /^[0-9a-f]{32}$/;
const HEX64 = /^[0-9a-f]{64}$/;
const PID = /^[1-9][0-9]*$/;
const CONTROL_FRAME_LIMIT = 64 * 1024;
const ACK_PARENT_PID = '41001';
const PARENT_NONCE = '00112233445566778899aabbccddeeff';
const READY_MS = 2500;
const SETTLE_MS = 2500;
const EXIT_MS = 1500;
const ACK_DEADLINE_MS = 2000;
const ACK_MIN_MS = 1500;
const ACK_MAX_MS = 3500;
const CLEANUP_MS = 1500;

const DEGRADED_WIRE = { v: 1, broker: 'degraded', accounts: [] };
const DEGRADED_PUBLIC = { v: 1, broker: 'degraded', accounts: [], intent_preview: null };
const ERROR_UNAVAILABLE = { code: 'UNAVAILABLE', message: 'Unavailable', retryable: true };
const ERROR_SCHEMA = { code: 'SCHEMA', message: 'Invalid request', retryable: false };
const ERROR_TIMEOUT = { code: 'TIMEOUT', message: 'Timed out', retryable: true };
const HELLO_KEYS = ['child_nonce', 'child_pid', 'max', 'min', 'protocol'];
const RESPONSE_KEYS = ['id', 'kind', 'result', 'seq', 'session', 'v'];
const ERROR_KEYS = ['error', 'id', 'kind', 'seq', 'session', 'v'];
const ERROR_FIELD_KEYS = ['code', 'message', 'retryable'];

const spawnErrors = new WeakMap();
const tests = [];
function test(name, fn) { tests.push({ name, fn }); }

function requireBrokerPath() {
  const candidate = process.argv[2] ? path.resolve(process.argv[2]) : DEFAULT_BROKER_PATH;
  let stat;
  try {
    stat = fs.lstatSync(candidate);
  } catch (_) {
    throw new Error(MISSING_EXECUTABLE);
  }
  if (stat.isSymbolicLink() || !stat.isFile()) throw new Error(MISSING_EXECUTABLE);
  return candidate;
}

function pinBroker(brokerPath) {
  return crypto.createHash('sha256').update(fs.readFileSync(brokerPath)).digest('hex');
}

function deriveSession(parentPid, childPid, parentNonce, childNonce) {
  const preimage = `${SESSION_DOMAIN}${parentPid}\n${childPid}\n${parentNonce}\n${childNonce}`;
  return crypto.createHash('sha256').update(Buffer.from(preimage, 'utf8')).digest('hex');
}

function encodeFrame(value) {
  const body = Buffer.from(JSON.stringify(value), 'utf8');
  const header = Buffer.alloc(4);
  header.writeUInt32BE(body.length, 0);
  return Buffer.concat([header, body]);
}

function frameFromBody(body) {
  const header = Buffer.alloc(4);
  header.writeUInt32BE(body.length, 0);
  return Buffer.concat([header, body]);
}

function oversizeHeader() {
  const header = Buffer.alloc(4);
  header.writeUInt32BE(CONTROL_FRAME_LIMIT + 1, 0);
  return header;
}

function zeroLengthHeader() {
  return Buffer.alloc(4);
}

function decodeAll(chunks) {
  const bytes = Buffer.concat(chunks.map((chunk) => Buffer.from(chunk)));
  const values = [];
  let offset = 0;
  while (offset + 4 <= bytes.length) {
    const length = bytes.readUInt32BE(offset);
    if (length === 0 || offset + 4 + length > bytes.length) break;
    const body = bytes.subarray(offset + 4, offset + 4 + length);
    values.push(JSON.parse(body.toString('utf8')));
    offset += 4 + length;
  }
  return { values, unread: bytes.length - offset, bytes };
}

function writeSplit(stream, frame) {
  const first = Math.min(2, frame.length);
  const second = Math.min(first + 3, frame.length);
  stream.write(frame.subarray(0, first));
  if (second > first) stream.write(frame.subarray(first, second));
  if (frame.length > second) stream.write(frame.subarray(second));
}

function requestId(n) {
  return n.toString(16).padStart(32, '0');
}

function makeRequest(id, seq, method, params, session, expiresMs) {
  return {
    v: 1,
    id,
    seq,
    kind: 'req',
    method,
    params,
    session,
    expires_ms: expiresMs,
  };
}

function invalidUtf8UnavailableRequest(id, seq, session) {
  const prefix = Buffer.from(
    `{"v":1,"id":"${id}","seq":${seq},"kind":"req","method":"account.list","params":{"note":"`,
    'utf8'
  );
  const invalid = Buffer.from([0xc3, 0x28]);
  const suffix = Buffer.from(
    `"},"session":"${session}","expires_ms":9999999999999}`,
    'utf8'
  );
  return frameFromBody(Buffer.concat([prefix, invalid, suffix]));
}

function hasExited(child) {
  return Boolean(child) && (child.exitCode !== null || child.signalCode !== null);
}

function sortedKeys(value) {
  return Object.keys(value).sort();
}

function monotonicMs() {
  return Number(process.hrtime.bigint() / 1000000n);
}

function childSpawnError(child) {
  return child ? spawnErrors.get(child) : undefined;
}

function observeSpawnError(child) {
  if (!child) return;
  child.on('error', (error) => {
    if (!spawnErrors.has(child)) spawnErrors.set(child, error);
  });
}

function throwIfSpawnFailed(child) {
  const error = childSpawnError(child);
  if (error) throw error;
}

function withDeadline(promise, ms, message) {
  let timer;
  const timeout = new Promise((_, reject) => {
    timer = setTimeout(() => reject(new Error(message)), ms);
  });
  return Promise.race([Promise.resolve(promise), timeout]).finally(() => clearTimeout(timer));
}

function waitUntil(predicate, ms, message, child) {
  return new Promise((resolve, reject) => {
    try {
      throwIfSpawnFailed(child);
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
        throwIfSpawnFailed(child);
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

function waitForExit(child, ms, message) {
  if (!child) return Promise.resolve();
  throwIfSpawnFailed(child);
  if (hasExited(child)) return Promise.resolve();
  return new Promise((resolve, reject) => {
    const timer = setTimeout(() => reject(new Error(message || 'child could not be reaped')), ms);
    const onError = (error) => {
      if (!spawnErrors.has(child)) spawnErrors.set(child, error);
      clearTimeout(timer);
      reject(error);
    };
    child.once('error', onError);
    child.once('exit', () => {
      child.removeListener('error', onError);
      clearTimeout(timer);
      resolve();
    });
  });
}

function streamFinished(stream) {
  return !stream || stream.readableEnded || stream.destroyed;
}

function waitStreamFinished(stream) {
  if (streamFinished(stream)) return Promise.resolve();
  return new Promise((resolve) => {
    const done = () => resolve();
    stream.once('end', done);
    stream.once('close', done);
  });
}

function waitForClose(child) {
  return new Promise((resolve) => {
    if (hasExited(child) && streamFinished(child.stdout) && streamFinished(child.stderr)) {
      resolve();
      return;
    }
    child.once('close', resolve);
  });
}

async function waitForCloseAndStreams(child, ms, message) {
  if (!child) return;
  await withDeadline(
    Promise.all([
      waitForClose(child),
      waitStreamFinished(child.stdout),
      waitStreamFinished(child.stderr),
    ]),
    ms,
    message || 'child streams did not finish draining'
  );
}

async function waitForChildSettled(child, ms, message) {
  if (!child) return;
  throwIfSpawnFailed(child);
  await waitForExit(child, ms, message);
  throwIfSpawnFailed(child);
  await waitForCloseAndStreams(child, ms, message || 'child close did not complete');
}

async function waitFrames(chunks, count, ms, message, child) {
  await waitUntil(() => {
    const decoded = decodeAll(chunks);
    if (decoded.values.length > count) {
      throw new Error(`${message}: extra protocol frames`);
    }
    return decoded.values.length === count;
  }, ms, message, child);
  return decodeAll(chunks).values.slice(0, count);
}

function assertHello(hello, childPid) {
  assert.deepStrictEqual(sortedKeys(hello), HELLO_KEYS.slice().sort());
  assert.strictEqual(hello.protocol, 'bitbook-wallet-broker');
  assert.strictEqual(hello.min, 1);
  assert.strictEqual(hello.max, 1);
  assert.strictEqual(typeof hello.child_nonce, 'string');
  assert.ok(HEX32.test(hello.child_nonce), 'hello nonce is not 32 lowercase hex');
  assert.strictEqual(hello.child_pid, String(childPid));
  assert.ok(PID.test(hello.child_pid), 'hello PID is not a decimal process id');
}

function assertResponse(value, id, seq, session, result) {
  assert.deepStrictEqual(sortedKeys(value), RESPONSE_KEYS.slice().sort());
  assert.strictEqual(value.v, 1);
  assert.strictEqual(value.id, id);
  assert.strictEqual(value.seq, seq);
  assert.strictEqual(value.kind, 'res');
  assert.strictEqual(value.session, session);
  assert.ok(HEX64.test(value.session));
  assert.deepStrictEqual(value.result, result);
}

function assertErrorEnvelope(value, id, seq, session, error) {
  assert.deepStrictEqual(sortedKeys(value), ERROR_KEYS.slice().sort());
  assert.strictEqual(value.v, 1);
  assert.strictEqual(value.id, id);
  assert.strictEqual(value.seq, seq);
  assert.strictEqual(value.kind, 'error');
  assert.strictEqual(value.session, session);
  assert.deepStrictEqual(sortedKeys(value.error), ERROR_FIELD_KEYS.slice().sort());
  assert.deepStrictEqual(value.error, error);
  assert.ok(!JSON.stringify(value).includes(CANARY), 'error envelope echoed canary');
}

function assertNoCanary(chunks, label) {
  const text = Buffer.concat(chunks.map((chunk) => Buffer.from(chunk))).toString('utf8');
  assert.ok(!text.includes(CANARY), `${label} leaked canary`);
}

function assertEmptyCwd(dataDir) {
  assert.deepStrictEqual(fs.readdirSync(dataDir), [], 'broker wrote files into its private cwd');
}

function assertProtocolFailureExit(child, label) {
  assert.ok(hasExited(child), `${label} still running`);
  assert.strictEqual(child.signalCode, null, `${label} exited by signal ${child.signalCode}`);
  assert.ok(
    Number.isInteger(child.exitCode) && child.exitCode > 0,
    `${label} did not exit with a numeric nonzero status`
  );
}

function assertDrainedTranscript(ctx, expectedCount, label) {
  assertNoCanary(ctx.stdout, `${label} stdout`);
  assertNoCanary(ctx.stderr, `${label} stderr`);
  const decoded = decodeAll(ctx.stdout);
  assert.strictEqual(decoded.unread, 0, `${label} left unread protocol bytes`);
  assert.strictEqual(
    decoded.values.length,
    expectedCount,
    `${label} transcript length ${decoded.values.length} !== ${expectedCount}`
  );
  return decoded.values;
}

async function assertPromptProtocolFailure(ctx, ms, label, expectedCount) {
  await waitForChildSettled(ctx.child, ms, `${label} did not terminate promptly`);
  assertProtocolFailureExit(ctx.child, label);
  return assertDrainedTranscript(ctx, expectedCount, label);
}

function retainCause(error, originalError) {
  if (originalError) error.cause = originalError;
  return error;
}

function parentAck(parentNonce, parentPid) {
  return {
    protocol: 'bitbook-wallet-broker',
    version: 1,
    parent_nonce: parentNonce,
    parent_pid: parentPid,
  };
}

async function withTempCwd(fn) {
  const dataDir = fs.mkdtempSync(path.join(os.tmpdir(), 'bb-wal011-exec-'));
  fs.chmodSync(dataDir, 0o700);
  const children = [];
  let originalError = null;
  try {
    return await fn({
      dataDir,
      track(child) {
        if (child) children.push(child);
        return child;
      },
    });
  } catch (error) {
    originalError = error;
    throw error;
  } finally {
    for (const child of children) {
      if (child && !hasExited(child)) {
        try { child.kill('SIGKILL'); } catch (_) { /* already gone */ }
      }
    }
    let cleanupError = null;
    for (const child of children) {
      if (!child) continue;
      try {
        await waitForExit(child, CLEANUP_MS, 'child could not be reaped');
      } catch (error) {
        if (!cleanupError) cleanupError = error;
      }
    }
    const unreaped = children.some((child) => child && !hasExited(child));
    if (unreaped) {
      const reapError = cleanupError || new Error('child could not be reaped');
      throw retainCause(reapError, originalError);
    }
    let entries;
    try {
      entries = fs.readdirSync(dataDir);
    } catch (error) {
      throw retainCause(error, originalError);
    }
    if (entries.length !== 0) {
      const leftoverError = new Error(
        `broker wrote files into its private cwd (${dataDir}): ${entries.join(', ')}`
      );
      leftoverError.path = dataDir;
      leftoverError.entries = entries.slice();
      throw retainCause(leftoverError, originalError);
    }
    try {
      fs.rmdirSync(dataDir);
    } catch (error) {
      throw retainCause(error, originalError);
    }
    if (cleanupError) throw retainCause(cleanupError, originalError);
  }
}

function attachCapture(child, stdout, stderr) {
  observeSpawnError(child);
  if (child.stdout) {
    child.stdout.on('data', (chunk) => stdout.push(Buffer.from(chunk)));
    child.stdout.on('error', () => {});
  }
  if (child.stderr) {
    child.stderr.on('data', (chunk) => stderr.push(Buffer.from(chunk)));
    child.stderr.on('error', () => {});
  }
  if (child.stdin) child.stdin.on('error', () => {});
}

function spawnBroker(brokerPath, dataDir) {
  return childProcess.spawn(brokerPath, [], {
    cwd: dataDir,
    env: { LANG: 'C.UTF-8', PATH: '/usr/bin' },
    shell: false,
    stdio: ['pipe', 'pipe', 'pipe'],
  });
}

async function withSupervisor(fn) {
  const brokerPath = requireBrokerPath();
  const expectedSha256 = pinBroker(brokerPath);
  return withTempCwd(async (ctx) => {
    const originalSpawn = childProcess.spawn;
    const spawnCalls = [];
    const stdout = [];
    const stderr = [];
    let child = null;
    childProcess.spawn = function observe(...args) {
      spawnCalls.push({ file: args[0], argv: args[1], options: args[2] });
      child = originalSpawn.apply(this, args);
      ctx.track(child);
      attachCapture(child, stdout, stderr);
      return child;
    };
    try {
      const supervisor = createWalletSupervisor({
        brokerPath,
        expectedSha256,
        dataDir: ctx.dataDir,
        env: { LANG: 'C.UTF-8', PATH: '/usr/bin', SECRET_TOKEN: CANARY },
      });
      try {
        return await fn({
          supervisor,
          spawnCalls,
          stdout,
          stderr,
          dataDir: ctx.dataDir,
          brokerPath,
          expectedSha256,
          get child() { return child; },
        });
      } finally {
        try { supervisor.quit(); } catch (_) { /* still reap */ }
      }
    } finally {
      childProcess.spawn = originalSpawn;
    }
  });
}

async function withDirectChild(fn) {
  const brokerPath = requireBrokerPath();
  return withTempCwd(async (ctx) => {
    const stdout = [];
    const stderr = [];
    const child = ctx.track(spawnBroker(brokerPath, ctx.dataDir));
    attachCapture(child, stdout, stderr);
    return fn({ child, stdout, stderr, dataDir: ctx.dataDir, brokerPath });
  });
}

async function readHello(ctx) {
  const values = await waitFrames(
    ctx.stdout, 1, READY_MS, 'compiled broker did not send hello', ctx.child
  );
  return values[0];
}

async function handshake(ctx, parentNonce, parentPid, splitAck) {
  const hello = await readHello(ctx);
  assertHello(hello, ctx.child.pid);
  const ack = parentAck(parentNonce, parentPid);
  const session = deriveSession(parentPid, hello.child_pid, parentNonce, hello.child_nonce);
  assert.ok(HEX64.test(session));
  const frame = encodeFrame(ack);
  if (splitAck) writeSplit(ctx.child.stdin, frame);
  else ctx.child.stdin.write(frame);
  return { hello, ack, session };
}

async function waitNewFrames(ctx, previous, add, ms, message) {
  const values = await waitFrames(ctx.stdout, previous + add, ms, message, ctx.child);
  return values.slice(previous, previous + add);
}

test('runtime: compiled native executable is present', () => {
  requireBrokerPath();
});

test('supervisor: real compiled child binds, degraded snapshot, status.get, account.list UNAVAILABLE, quit reaps', async () => {
  await withSupervisor(async (ctx) => {
    const snapshots = [];
    ctx.supervisor.subscribeSnapshot((value) => snapshots.push(value));
    const started = ctx.supervisor.start();
    assert.strictEqual(started.ok, true, 'compiled broker did not start');
    assert.strictEqual(started.snapshot.broker, 'down');
    assert.strictEqual(ctx.spawnCalls.length, 1);
    assert.strictEqual(ctx.spawnCalls[0].file, ctx.brokerPath);
    assert.deepStrictEqual(ctx.spawnCalls[0].argv, []);
    assert.strictEqual(ctx.spawnCalls[0].options.shell, false);
    assert.deepStrictEqual(ctx.spawnCalls[0].options.stdio, ['pipe', 'pipe', 'pipe']);
    assert.strictEqual(ctx.spawnCalls[0].options.cwd, ctx.dataDir);
    assert.deepStrictEqual(ctx.spawnCalls[0].options.env, { LANG: 'C.UTF-8', PATH: '/usr/bin' });
    assert.ok(!JSON.stringify(ctx.spawnCalls[0]).includes(CANARY));
    assert.ok(ctx.child, 'observer did not capture the real child');
    assert.ok(Number.isInteger(ctx.child.pid) && ctx.child.pid > 0);
    await waitUntil(() => {
      throwIfSpawnFailed(ctx.child);
      return ctx.supervisor.bound === true;
    }, READY_MS, 'compiled broker did not bind', ctx.child);
    assert.strictEqual(ctx.supervisor.bound, true);
    const hello = decodeAll(ctx.stdout).values[0];
    assertHello(hello, ctx.child.pid);
    const degraded = snapshots.filter((value) => value.broker === 'degraded');
    assert.ok(degraded.length >= 1, 'bootstrap did not publish the degraded snapshot');
    assert.deepStrictEqual(degraded[0], DEGRADED_PUBLIC);
    assert.ok(!snapshots.some((value) => value.broker === 'ready' || value.broker === 'locked'));
    assert.ok(!snapshots.some((value) => value.accounts && value.accounts.length > 0));

    const status = await withDeadline(
      ctx.supervisor.dispatch('status.get', {}),
      SETTLE_MS,
      'public status.get did not settle'
    );
    assert.deepStrictEqual(status, DEGRADED_PUBLIC);
    assert.deepStrictEqual(status.accounts, []);

    await assert.rejects(
      async () => {
        const value = await withDeadline(
          ctx.supervisor.dispatch('account.list', {}),
          SETTLE_MS,
          'account.list did not settle'
        );
        assert.fail(`account.list resolved with ${JSON.stringify(value)}`);
      },
      (error) => {
        assert.strictEqual(error.code, 'UNAVAILABLE');
        assert.strictEqual(error.message, 'Unavailable');
        assert.strictEqual(error.retryable, true);
        assert.ok(!JSON.stringify(error).includes(CANARY));
        return true;
      }
    );

    ctx.supervisor.quit();
    await waitForChildSettled(ctx.child, SETTLE_MS, 'supervisor quit did not reap the child');
    assert.ok(hasExited(ctx.child), 'supervisor quit did not terminate the actual child');
    assert.strictEqual(ctx.supervisor.bound, false);
    const values = assertDrainedTranscript(ctx, 4, 'supervisor');
    assertHello(values[0], ctx.child.pid);
    assert.strictEqual(values[1].kind, 'res');
    assert.deepStrictEqual(values[1].result, DEGRADED_WIRE);
    assert.strictEqual(values[2].kind, 'res');
    assert.deepStrictEqual(values[2].result, DEGRADED_WIRE);
    assert.strictEqual(values[2].seq, values[1].seq + 1);
    assert.strictEqual(values[3].kind, 'error');
    assert.deepStrictEqual(values[3].error, ERROR_UNAVAILABLE);
    assert.strictEqual(values[3].seq, values[2].seq + 1);
    assertEmptyCwd(ctx.dataDir);
  });
});

test('direct: hello uses exact v1 keys, actual PID, and a fresh nonce on each start', async () => {
  const first = await withDirectChild(async (ctx) => {
    const hello = await readHello(ctx);
    assertHello(hello, ctx.child.pid);
    assert.notStrictEqual(hello.child_pid, String(process.pid));
    ctx.child.kill('SIGKILL');
    await waitForChildSettled(ctx.child, EXIT_MS, 'first hello child did not exit');
    assertDrainedTranscript(ctx, 1, 'first hello');
    assertEmptyCwd(ctx.dataDir);
    return hello.child_nonce;
  });
  const second = await withDirectChild(async (ctx) => {
    const hello = await readHello(ctx);
    assertHello(hello, ctx.child.pid);
    ctx.child.kill('SIGKILL');
    await waitForChildSettled(ctx.child, EXIT_MS, 'second hello child did not exit');
    assertDrainedTranscript(ctx, 1, 'second hello');
    return hello.child_nonce;
  });
  assert.notStrictEqual(first, second, 'child nonce was reused across starts');
});

test('direct: split ack and coalesced status requests return exact IDs, degraded results, and independent child sequences', async () => {
  await withDirectChild(async (ctx) => {
    const { session } = await handshake(ctx, PARENT_NONCE, ACK_PARENT_PID, true);
    const first = makeRequest(requestId(1), 1, 'status.get', {}, session, Date.now() + 10000);
    const second = makeRequest(requestId(2), 2, 'sync.subscribe', {}, session, Date.now() + 10000);
    const firstFrame = encodeFrame(first);
    writeSplit(ctx.child.stdin, firstFrame);
    ctx.child.stdin.write(Buffer.concat([
      encodeFrame(second),
      encodeFrame(makeRequest(requestId(3), 3, 'status.get', {}, session, Date.now() + 10000)),
    ]));
    const replies = await waitNewFrames(ctx, 1, 3, SETTLE_MS, 'coalesced status requests did not complete');
    assertResponse(replies[0], first.id, 1, session, DEGRADED_WIRE);
    assertResponse(replies[1], second.id, 2, session, DEGRADED_WIRE);
    assertResponse(replies[2], requestId(3), 3, session, DEGRADED_WIRE);
    assert.strictEqual(replies[1].seq, replies[0].seq + 1);
    assert.strictEqual(replies[2].seq, replies[1].seq + 1);
    assert.ok(!replies.some((value) => value.kind === 'evt'), 'unsolicited event on a read-only subscribe');
    assert.ok(!replies.some((value) => value.result && value.result.broker === 'ready'));
    ctx.child.stdin.end();
    await waitForChildSettled(ctx.child, EXIT_MS, 'clean EOF after a valid session did not exit');
    assert.strictEqual(ctx.child.exitCode, 0);
    assert.strictEqual(ctx.child.signalCode, null);
    const values = assertDrainedTranscript(ctx, 4, 'coalesced status');
    assertHello(values[0], ctx.child.pid);
    assertResponse(values[1], first.id, 1, session, DEGRADED_WIRE);
    assertResponse(values[2], second.id, 2, session, DEGRADED_WIRE);
    assertResponse(values[3], requestId(3), 3, session, DEGRADED_WIRE);
    assertEmptyCwd(ctx.dataDir);
  });
});

test('direct: malformed, duplicate-key, and oversize frames terminate promptly', async () => {
  await withDirectChild(async (ctx) => {
    await readHello(ctx);
    ctx.child.stdin.write(frameFromBody(Buffer.from('{', 'utf8')));
    const values = await assertPromptProtocolFailure(ctx, EXIT_MS, 'malformed ack', 1);
    assertHello(values[0], ctx.child.pid);
  });
  await withDirectChild(async (ctx) => {
    await handshake(ctx, PARENT_NONCE, ACK_PARENT_PID, false);
    ctx.child.stdin.write(zeroLengthHeader());
    const values = await assertPromptProtocolFailure(ctx, EXIT_MS, 'zero-length frame', 1);
    assertHello(values[0], ctx.child.pid);
  });
  await withDirectChild(async (ctx) => {
    await handshake(ctx, PARENT_NONCE, ACK_PARENT_PID, false);
    ctx.child.stdin.write(oversizeHeader());
    const values = await assertPromptProtocolFailure(ctx, EXIT_MS, 'oversize length', 1);
    assertHello(values[0], ctx.child.pid);
  });
  await withDirectChild(async (ctx) => {
    const { session } = await handshake(ctx, PARENT_NONCE, ACK_PARENT_PID, false);
    const duplicate = Buffer.from(
      '{"v":1,"id":"11112222333344445555666677778888","seq":1,"kind":"req","method":"status.get","params":{"x":1},"params":{},"session":"' +
      session +
      '","expires_ms":9999999999999}',
      'utf8'
    );
    ctx.child.stdin.write(frameFromBody(duplicate));
    const values = await assertPromptProtocolFailure(ctx, EXIT_MS, 'duplicate JSON names', 1);
    assertHello(values[0], ctx.child.pid);
  });
  await withDirectChild(async (ctx) => {
    const { session } = await handshake(ctx, PARENT_NONCE, ACK_PARENT_PID, false);
    const status = makeRequest(requestId(1), 1, 'status.get', {}, session, Date.now() + 10000);
    ctx.child.stdin.write(encodeFrame(status));
    const replies = await waitNewFrames(ctx, 1, 1, SETTLE_MS, 'bootstrap status.get did not complete');
    assertResponse(replies[0], status.id, 1, session, DEGRADED_WIRE);
    ctx.child.stdin.write(invalidUtf8UnavailableRequest(requestId(2), 2, session));
    const values = await assertPromptProtocolFailure(ctx, EXIT_MS, 'invalid UTF-8', 2);
    assertHello(values[0], ctx.child.pid);
    assertResponse(values[1], status.id, 1, session, DEGRADED_WIRE);
  });
  await withDirectChild(async (ctx) => {
    const { session } = await handshake(ctx, PARENT_NONCE, ACK_PARENT_PID, false);
    const request = makeRequest(requestId(1), 1, 'status.get', {}, session, 9999999999999);
    ctx.child.stdin.write(frameFromBody(Buffer.from(JSON.stringify(request) + '{}', 'utf8')));
    const values = await assertPromptProtocolFailure(ctx, EXIT_MS, 'trailing JSON', 1);
    assertHello(values[0], ctx.child.pid);
  });
  await withDirectChild(async (ctx) => {
    const { session } = await handshake(ctx, PARENT_NONCE, ACK_PARENT_PID, false);
    const nested = Buffer.from(
      '{"v":1,"id":"11112222333344445555666677778888","seq":1,"kind":"req","method":"status.get","params":{"x":1,"x":2},"session":"' +
      session +
      '","expires_ms":9999999999999}',
      'utf8'
    );
    ctx.child.stdin.write(frameFromBody(nested));
    const values = await assertPromptProtocolFailure(ctx, EXIT_MS, 'nested duplicate JSON names', 1);
    assertHello(values[0], ctx.child.pid);
  });
});

test('direct: wrong session, sequence, kind, and duplicate ids terminate promptly', async () => {
  await withDirectChild(async (ctx) => {
    const { session } = await handshake(ctx, PARENT_NONCE, ACK_PARENT_PID, false);
    const wrongSession = makeRequest(
      requestId(1), 1, 'status.get', {}, '0'.repeat(64), Date.now() + 10000
    );
    assert.notStrictEqual(wrongSession.session, session);
    ctx.child.stdin.write(encodeFrame(wrongSession));
    const values = await assertPromptProtocolFailure(ctx, EXIT_MS, 'wrong session', 1);
    assertHello(values[0], ctx.child.pid);
  });
  await withDirectChild(async (ctx) => {
    const { session } = await handshake(ctx, PARENT_NONCE, ACK_PARENT_PID, false);
    ctx.child.stdin.write(encodeFrame(
      makeRequest(requestId(1), 2, 'status.get', {}, session, Date.now() + 10000)
    ));
    const values = await assertPromptProtocolFailure(ctx, EXIT_MS, 'wrong sequence', 1);
    assertHello(values[0], ctx.child.pid);
  });
  await withDirectChild(async (ctx) => {
    const { session } = await handshake(ctx, PARENT_NONCE, ACK_PARENT_PID, false);
    const first = makeRequest(requestId(1), 1, 'status.get', {}, session, Date.now() + 10000);
    const duplicate = makeRequest(requestId(1), 2, 'status.get', {}, session, Date.now() + 10000);
    ctx.child.stdin.write(encodeFrame(first));
    await waitNewFrames(ctx, 1, 1, SETTLE_MS, 'first duplicate-id request did not complete');
    ctx.child.stdin.write(encodeFrame(duplicate));
    const values = await assertPromptProtocolFailure(ctx, EXIT_MS, 'duplicate request id', 2);
    assertHello(values[0], ctx.child.pid);
    assertResponse(values[1], first.id, 1, session, DEGRADED_WIRE);
  });
  await withDirectChild(async (ctx) => {
    const { session } = await handshake(ctx, PARENT_NONCE, ACK_PARENT_PID, false);
    ctx.child.stdin.write(encodeFrame({
      v: 1,
      id: requestId(1),
      seq: 1,
      kind: 'cancel',
      cancel_of: requestId(2),
      session,
    }));
    const values = await assertPromptProtocolFailure(ctx, EXIT_MS, 'cancellation frame', 1);
    assertHello(values[0], ctx.child.pid);
  });
  await withDirectChild(async (ctx) => {
    const { session } = await handshake(ctx, PARENT_NONCE, ACK_PARENT_PID, false);
    const extra = makeRequest(requestId(1), 1, 'status.get', {}, session, Date.now() + 10000);
    extra.debug = CANARY;
    ctx.child.stdin.write(encodeFrame(extra));
    const values = await assertPromptProtocolFailure(ctx, EXIT_MS, 'unknown envelope field', 1);
    assertHello(values[0], ctx.child.pid);
  });
});

test('direct: absent and partial ack hit the two-second deadline', async () => {
  await withDirectChild(async (ctx) => {
    await readHello(ctx);
    const started = monotonicMs();
    await waitForExit(ctx.child, ACK_MAX_MS, 'absent ack did not hit the two-second deadline');
    const elapsed = monotonicMs() - started;
    assert.ok(
      elapsed >= ACK_MIN_MS && elapsed <= ACK_MAX_MS,
      `absent ack window ${elapsed}ms is outside ${ACK_MIN_MS}-${ACK_MAX_MS} around ${ACK_DEADLINE_MS}`
    );
    assertProtocolFailureExit(ctx.child, 'absent ack');
    await waitForCloseAndStreams(ctx.child, EXIT_MS, 'absent ack streams did not finish');
    const values = assertDrainedTranscript(ctx, 1, 'absent ack');
    assertHello(values[0], ctx.child.pid);
  });
  await withDirectChild(async (ctx) => {
    await readHello(ctx);
    ctx.child.stdin.write(encodeFrame(parentAck(PARENT_NONCE, ACK_PARENT_PID)).subarray(0, 2));
    const started = monotonicMs();
    await waitForExit(ctx.child, ACK_MAX_MS, 'partial ack header did not hit the two-second deadline');
    const elapsed = monotonicMs() - started;
    assert.ok(
      elapsed >= ACK_MIN_MS && elapsed <= ACK_MAX_MS,
      `partial ack header window ${elapsed}ms is outside ${ACK_MIN_MS}-${ACK_MAX_MS} around ${ACK_DEADLINE_MS}`
    );
    assertProtocolFailureExit(ctx.child, 'partial ack header');
    await waitForCloseAndStreams(ctx.child, EXIT_MS, 'partial ack header streams did not finish');
    const values = assertDrainedTranscript(ctx, 1, 'partial ack header');
    assertHello(values[0], ctx.child.pid);
  });
  await withDirectChild(async (ctx) => {
    await readHello(ctx);
    const frame = encodeFrame(parentAck(PARENT_NONCE, ACK_PARENT_PID));
    ctx.child.stdin.write(frame.subarray(0, 4));
    ctx.child.stdin.write(frame.subarray(4, Math.min(frame.length - 1, 12)));
    const started = monotonicMs();
    await waitForExit(ctx.child, ACK_MAX_MS, 'incomplete ack body did not hit the two-second deadline');
    const elapsed = monotonicMs() - started;
    assert.ok(
      elapsed >= ACK_MIN_MS && elapsed <= ACK_MAX_MS,
      `incomplete ack body window ${elapsed}ms is outside ${ACK_MIN_MS}-${ACK_MAX_MS} around ${ACK_DEADLINE_MS}`
    );
    assertProtocolFailureExit(ctx.child, 'incomplete ack body');
    await waitForCloseAndStreams(ctx.child, EXIT_MS, 'incomplete ack body streams did not finish');
    const values = assertDrainedTranscript(ctx, 1, 'incomplete ack body');
    assertHello(values[0], ctx.child.pid);
  });
});

test('direct: partial-frame EOF after a valid session exits', async () => {
  await withDirectChild(async (ctx) => {
    const { session } = await handshake(ctx, PARENT_NONCE, ACK_PARENT_PID, false);
    const request = makeRequest(requestId(1), 1, 'status.get', {}, session, Date.now() + 10000);
    ctx.child.stdin.write(encodeFrame(request));
    const replies = await waitNewFrames(ctx, 1, 1, SETTLE_MS, 'status.get before partial EOF did not complete');
    assertResponse(replies[0], request.id, 1, session, DEGRADED_WIRE);
    ctx.child.stdin.write(Buffer.from([0x00, 0x00]));
    ctx.child.stdin.end();
    const values = await assertPromptProtocolFailure(ctx, EXIT_MS, 'partial-header EOF', 2);
    assertHello(values[0], ctx.child.pid);
    assertResponse(values[1], request.id, 1, session, DEGRADED_WIRE);
    assertEmptyCwd(ctx.dataDir);
  });
  await withDirectChild(async (ctx) => {
    const { session } = await handshake(ctx, PARENT_NONCE, ACK_PARENT_PID, false);
    const request = makeRequest(requestId(1), 1, 'status.get', {}, session, Date.now() + 10000);
    ctx.child.stdin.write(encodeFrame(request));
    const replies = await waitNewFrames(ctx, 1, 1, SETTLE_MS, 'status.get before incomplete-body EOF did not complete');
    assertResponse(replies[0], request.id, 1, session, DEGRADED_WIRE);
    const next = encodeFrame(makeRequest(requestId(2), 2, 'status.get', {}, session, Date.now() + 10000));
    ctx.child.stdin.write(next.subarray(0, 4 + Math.min(8, next.length - 5)));
    ctx.child.stdin.end();
    const values = await assertPromptProtocolFailure(ctx, EXIT_MS, 'incomplete-body EOF', 2);
    assertHello(values[0], ctx.child.pid);
    assertResponse(values[1], request.id, 1, session, DEGRADED_WIRE);
    assertEmptyCwd(ctx.dataDir);
  });
});

test('direct: SCHEMA, TIMEOUT, and UNAVAILABLE stay fixed, canaries stay off the wire, and cwd stays empty', async () => {
  await withDirectChild(async (ctx) => {
    const { session } = await handshake(ctx, PARENT_NONCE, ACK_PARENT_PID, false);
    const status = makeRequest(requestId(1), 1, 'status.get', {}, session, Date.now() + 10000);
    const listed = makeRequest(requestId(2), 2, 'account.list', {}, session, Date.now() + 10000);
    const lock = makeRequest(
      requestId(3), 3, 'account.lock', { account_id: CANARY, secret: CANARY }, session, Date.now() + 10000
    );
    const begin = makeRequest(
      requestId(4), 4, 'intent.begin', { seed: CANARY, payment_request: CANARY }, session, Date.now() + 10000
    );
    const unlock = makeRequest(
      requestId(5), 5, 'account.unlock', { mnemonic: CANARY }, session, Date.now() + 10000
    );
    const broadcast = makeRequest(
      requestId(6), 6, 'intent.broadcast', { raw: CANARY }, session, Date.now() + 10000
    );
    const expired = makeRequest(requestId(7), 7, 'status.get', {}, session, 1);
    const extraParams = makeRequest(
      requestId(8), 8, 'status.get', { probe: CANARY }, session, Date.now() + 10000
    );
    const fresh = makeRequest(
      requestId(9), 9, 'receiver.fresh', { account_id: CANARY, asset: CANARY }, session, Date.now() + 10000
    );
    const cancel = makeRequest(
      requestId(10), 10, 'intent.cancel', { intent_id: CANARY }, session, Date.now() + 10000
    );
    const subscribe = makeRequest(
      requestId(11), 11, 'sync.subscribe', { topic: CANARY }, session, Date.now() + 10000
    );

    ctx.child.stdin.write(encodeFrame(status));
    const first = await waitNewFrames(ctx, 1, 1, SETTLE_MS, 'direct status.get did not complete');
    assertResponse(first[0], status.id, 1, session, DEGRADED_WIRE);

    ctx.child.stdin.write(Buffer.concat([
      encodeFrame(listed), encodeFrame(lock), encodeFrame(begin),
      encodeFrame(unlock), encodeFrame(broadcast), encodeFrame(expired), encodeFrame(extraParams),
      encodeFrame(fresh), encodeFrame(cancel), encodeFrame(subscribe),
    ]));
    const replies = await waitNewFrames(ctx, 2, 10, SETTLE_MS, 'fixed method errors did not complete');
    assertErrorEnvelope(replies[0], listed.id, 2, session, ERROR_UNAVAILABLE);
    assertErrorEnvelope(replies[1], lock.id, 3, session, ERROR_UNAVAILABLE);
    assertErrorEnvelope(replies[2], begin.id, 4, session, ERROR_UNAVAILABLE);
    assertErrorEnvelope(replies[3], unlock.id, 5, session, ERROR_SCHEMA);
    assertErrorEnvelope(replies[4], broadcast.id, 6, session, ERROR_SCHEMA);
    assertErrorEnvelope(replies[5], expired.id, 7, session, ERROR_TIMEOUT);
    assertErrorEnvelope(replies[6], extraParams.id, 8, session, ERROR_SCHEMA);
    assertErrorEnvelope(replies[7], fresh.id, 9, session, ERROR_UNAVAILABLE);
    assertErrorEnvelope(replies[8], cancel.id, 10, session, ERROR_UNAVAILABLE);
    assertErrorEnvelope(replies[9], subscribe.id, 11, session, ERROR_SCHEMA);
    assert.ok(!replies.some((value) => value.kind === 'res' && Array.isArray(value.result && value.result.accounts)));
    ctx.child.stdin.end();
    await waitForChildSettled(ctx.child, EXIT_MS, 'clean EOF after error replies did not exit');
    assert.strictEqual(ctx.child.exitCode, 0);
    assert.strictEqual(ctx.child.signalCode, null);
    const values = assertDrainedTranscript(ctx, 12, 'method-error');
    assertHello(values[0], ctx.child.pid);
    assertResponse(values[1], status.id, 1, session, DEGRADED_WIRE);
    assertErrorEnvelope(values[2], listed.id, 2, session, ERROR_UNAVAILABLE);
    assertErrorEnvelope(values[3], lock.id, 3, session, ERROR_UNAVAILABLE);
    assertErrorEnvelope(values[4], begin.id, 4, session, ERROR_UNAVAILABLE);
    assertErrorEnvelope(values[5], unlock.id, 5, session, ERROR_SCHEMA);
    assertErrorEnvelope(values[6], broadcast.id, 6, session, ERROR_SCHEMA);
    assertErrorEnvelope(values[7], expired.id, 7, session, ERROR_TIMEOUT);
    assertErrorEnvelope(values[8], extraParams.id, 8, session, ERROR_SCHEMA);
    assertErrorEnvelope(values[9], fresh.id, 9, session, ERROR_UNAVAILABLE);
    assertErrorEnvelope(values[10], cancel.id, 10, session, ERROR_UNAVAILABLE);
    assertErrorEnvelope(values[11], subscribe.id, 11, session, ERROR_SCHEMA);
    assertEmptyCwd(ctx.dataDir);
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
  process.stdout.write(`BitBook wallet broker runtime tests passed (${tests.length}).\n`);
}

if (require.main === module) run();
module.exports = { tests };
