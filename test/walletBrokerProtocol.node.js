'use strict';

const assert = require('assert');
const crypto = require('crypto');
const fs = require('fs');
const path = require('path');

const fixture = JSON.parse(fs.readFileSync(
  path.join(__dirname, 'fixtures', 'wallet-broker', 'transcript-v1.json'),
  'utf8'
));
const expectedPreimage = `${fixture.domain_separator}${fixture.parent_pid}\n${fixture.child_pid}\n${fixture.parent_nonce}\n${fixture.child_nonce}`;
assert.strictEqual(
  crypto.createHash('sha256').update(Buffer.from(expectedPreimage, 'utf8')).digest('hex'),
  fixture.session_id,
  'independent transcript fixture hash is wrong'
);

const {
  ABSOLUTE_FRAME_LIMIT,
  CONTROL_FRAME_LIMIT,
  computeSessionId,
  createBrokerFrameDecoder,
  createProtocolSession,
  encodeBrokerFrame,
  normalizeBrokerError,
  validateHello,
  validateHelloAck,
} = require('../wallet-broker/protocol');

function assertCode(fn, code) {
  let error;
  try { fn(); } catch (caught) { error = caught; }
  assert.ok(error, `expected ${code}`);
  assert.strictEqual(error.code, code);
}

function clone(value) {
  return JSON.parse(JSON.stringify(value));
}

function request(overrides = {}) {
  return Object.assign({
    v: 1,
    id: '11112222333344445555666677778888',
    seq: 1,
    kind: 'req',
    method: 'status.get',
    params: {},
    session: fixture.session_id,
    expires_ms: 2000,
  }, overrides);
}

const tests = [];
function test(name, fn) { tests.push({ name, fn }); }

test('transcript: independent fixture preimage and implementation session ID are exact', () => {
  assert.deepStrictEqual(Object.keys(fixture), [
    'name', 'domain_separator', 'parent_pid', 'child_pid', 'parent_nonce',
    'child_nonce', 'session_id', 'hello', 'hello_ack',
  ]);
  assert.strictEqual(fixture.domain_separator, 'bitbook-wallet-session-v1\n');
  assert.strictEqual(computeSessionId(fixture), fixture.session_id);
});

test('handshake: child hello and parent ack are closed v1 schemas', () => {
  assert.deepStrictEqual(validateHello(clone(fixture.hello)), {
    value: fixture.hello,
    negotiated_version: 1,
  });
  assert.deepStrictEqual(validateHelloAck(clone(fixture.hello_ack)), fixture.hello_ack);
  for (const [validator, positive] of [[validateHello, fixture.hello], [validateHelloAck, fixture.hello_ack]]) {
    assertCode(() => validator(Object.assign(clone(positive), { extra: true })), 'SCHEMA');
    for (const key of Object.keys(positive)) {
      const missing = clone(positive);
      delete missing[key];
      assertCode(() => validator(missing), 'SCHEMA');
    }
  }
});

test('handshake: protocol, version overlap, nonce, and canonical PID boundaries fail closed', () => {
  assert.deepStrictEqual(validateHello(Object.assign(clone(fixture.hello), { min: 1, max: 2 })), {
    value: Object.assign(clone(fixture.hello), { min: 1, max: 2 }),
    negotiated_version: 1,
  });
  for (const changes of [
    { protocol: 'bitbook-wallet-broker-lookalike' }, { min: 0 }, { min: -1 },
    { min: 1.5 }, { max: 0 }, { max: -1 }, { max: 1.5 }, { min: 2, max: 2 },
    { min: 2, max: 1 }, { child_nonce: 'A'.repeat(32) },
    { child_nonce: '0'.repeat(31) }, { child_pid: '0' }, { child_pid: '041002' },
    { child_pid: '-1' }, { child_pid: 41002 },
  ]) assertCode(() => validateHello(Object.assign(clone(fixture.hello), changes)), 'SCHEMA');
  for (const changes of [
    { protocol: 'other' }, { version: 0 }, { version: 2 },
    { parent_nonce: 'g'.repeat(32) }, { parent_pid: '0' }, { parent_pid: '041001' },
  ]) assertCode(() => validateHelloAck(Object.assign(clone(fixture.hello_ack), changes)), 'SCHEMA');
});

test('framing: split prefixes, bytewise bodies, coalesced frames, and unread bytes survive', () => {
  const values = [fixture.hello, {}, request()];
  const frames = values.map((value) => encodeBrokerFrame(value));
  const decoder = createBrokerFrameDecoder({ limitBytes: CONTROL_FRAME_LIMIT, stream: 'protocol' });
  assert.deepStrictEqual(decoder.push(frames[0].subarray(0, 2)), []);
  assert.deepStrictEqual(decoder.push(frames[0].subarray(2)), [values[0]]);
  const emitted = [];
  for (const byte of frames[1]) emitted.push(...decoder.push(Buffer.from([byte])));
  assert.deepStrictEqual(emitted, [values[1]]);
  assert.deepStrictEqual(decoder.push(Buffer.concat([frames[2], frames[1]])), [values[2], values[1]]);
});

test('framing: control and absolute byte limits are exact and diagnostics never parse as protocol', () => {
  assert.strictEqual(CONTROL_FRAME_LIMIT, 64 * 1024);
  assert.strictEqual(ABSOLUTE_FRAME_LIMIT, 1024 * 1024);
  for (const limitBytes of [CONTROL_FRAME_LIMIT, ABSOLUTE_FRAME_LIMIT]) {
    const exactBody = Buffer.from(`{"x":"${'a'.repeat(limitBytes - 8)}"}`, 'utf8');
    const exact = createBrokerFrameDecoder({ limitBytes, stream: 'protocol' });
    assert.strictEqual(exact.push(Buffer.concat([
      Buffer.from([(limitBytes >>> 24) & 255, (limitBytes >>> 16) & 255, (limitBytes >>> 8) & 255, limitBytes & 255]),
      exactBody,
    ])).length, 1);
    const over = createBrokerFrameDecoder({ limitBytes, stream: 'protocol' });
    const prefix = Buffer.alloc(4);
    prefix.writeUInt32BE(limitBytes + 1, 0);
    assertCode(() => over.push(prefix), 'LIMIT');
    assert.strictEqual(over.closed, true);
  }
  assertCode(
    () => createBrokerFrameDecoder({ limitBytes: CONTROL_FRAME_LIMIT, stream: 'diagnostics' }).push(encodeBrokerFrame({})),
    'SCHEMA'
  );
});

test('framing: malformed UTF-8, JSON, trailing JSON, zero length, and later bytes close permanently', () => {
  const bodies = [
    Buffer.from([0xc3, 0x28]), Buffer.from('{'), Buffer.from('{}{}'), Buffer.alloc(0),
    Buffer.from('{"v":1,"v":1}', 'utf8'),
  ];
  for (const body of bodies) {
    const decoder = createBrokerFrameDecoder({ limitBytes: CONTROL_FRAME_LIMIT, stream: 'protocol' });
    const prefix = Buffer.alloc(4);
    prefix.writeUInt32BE(body.length, 0);
    assertCode(() => decoder.push(Buffer.concat([prefix, body])), 'SCHEMA');
    assert.strictEqual(decoder.closed, true);
    assertCode(() => decoder.push(encodeBrokerFrame({})), 'SCHEMA');
  }
});

test('session: first post-hello message binds session before any method is accepted', () => {
  const session = createProtocolSession({ sessionId: fixture.session_id, now: () => 1000 });
  assertCode(() => session.accept('parent', request({ session: undefined })), 'UNAUTH');
  assert.strictEqual(session.bound.parent, false);
  assert.deepStrictEqual(session.accept('parent', request()), request());
  assert.strictEqual(session.bound.parent, true);
  assertCode(() => session.accept('child', request({ kind: 'res', method: undefined, params: undefined, result: {}, session: '0'.repeat(64) })), 'UNAUTH');
});

test('session: valid correlated response survives wrong ID and direction while duplicates fail closed', () => {
  const requestId = '11112222333344445555666677778888';
  const session = createProtocolSession({ sessionId: fixture.session_id, now: () => 1000 });
  assert.deepStrictEqual(session.accept('parent', request({ id: requestId })), request({ id: requestId }));
  const response = {
    v: 1, id: requestId, seq: 1, kind: 'res', result: { broker: 'ready' }, session: fixture.session_id,
  };
  assertCode(
    () => session.accept('child', Object.assign({}, response, { id: '22223333444455556666777788889999' })),
    'SCHEMA'
  );
  assertCode(() => session.accept('parent', Object.assign({}, response, { seq: 2 })), 'SCHEMA');
  const secondRequest = request({
    id: '22223333444455556666777788889999', seq: 2, method: 'account.list',
  });
  assert.deepStrictEqual(session.accept('parent', secondRequest), secondRequest);
  assert.deepStrictEqual(session.accept('child', response), response);
  assertCode(() => session.accept('child', Object.assign({}, response, { seq: 2 })), 'SCHEMA');
  const nextEvent = {
    v: 1, id: '3333444455556666777788889999aaaa', seq: 2, kind: 'evt',
    method: 'sync.subscribe', params: {}, session: fixture.session_id,
  };
  assert.deepStrictEqual(session.accept('child', nextEvent), nextEvent);
});

test('session: known in-flight cancellation succeeds while unknown and late frames preserve state', () => {
  const requestId = '11112222333344445555666677778888';
  const session = createProtocolSession({ sessionId: fixture.session_id, now: () => 1000 });
  session.accept('parent', request({ id: requestId }));
  const cancel = {
    v: 1, id: '22223333444455556666777788889999', seq: 2, kind: 'cancel',
    cancel_of: requestId, session: fixture.session_id,
  };
  assert.deepStrictEqual(session.accept('parent', cancel), cancel);
  assertCode(() => session.accept('parent', {
    v: 1, id: '3333444455556666777788889999aaaa', seq: 3, kind: 'cancel',
    cancel_of: '99990000111122223333444455556666', session: fixture.session_id,
  }), 'SCHEMA');
  const nextRequest = request({
    id: '3333444455556666777788889999aaaa', seq: 3, method: 'account.list',
  });
  assert.deepStrictEqual(session.accept('parent', nextRequest), nextRequest);
  assertCode(() => session.accept('child', {
    v: 1, id: requestId, seq: 1, kind: 'res', result: {}, session: fixture.session_id,
  }), 'SCHEMA');
  const nextEvent = {
    v: 1, id: '444455556666777788889999aaaabbbb', seq: 1, kind: 'evt',
    method: 'sync.subscribe', params: {}, session: fixture.session_id,
  };
  assert.deepStrictEqual(session.accept('child', nextEvent), nextEvent);
});

test('session: sequence, IDs, kinds, fields, deadlines, correlation, and cancellation are strict', () => {
  const session = createProtocolSession({ sessionId: fixture.session_id, now: () => 1000 });
  session.accept('parent', request());
  assertCode(() => session.accept('parent', request({ id: '22223333444455556666777788889999', seq: 1 })), 'SCHEMA');
  assertCode(() => session.accept('parent', request({ seq: 2 })), 'SCHEMA');
  assertCode(() => session.accept('parent', request({ id: '22223333444455556666777788889999', seq: 2, kind: 'other' })), 'SCHEMA');
  assertCode(() => session.accept('parent', request({ id: '22223333444455556666777788889999', seq: 2, extra: true })), 'SCHEMA');
  assertCode(() => session.accept('parent', request({ id: '22223333444455556666777788889999', seq: 2, expires_ms: 0 })), 'TIMEOUT');
  assertCode(() => session.accept('parent', {
    v: 1, id: '3333444455556666777788889999aaaa', seq: 2, kind: 'cancel',
    cancel_of: '99990000111122223333444455556666', session: fixture.session_id,
  }), 'SCHEMA');
});

test('errors: only stable safe fields survive and arbitrary internals normalize', () => {
  assert.deepStrictEqual(normalizeBrokerError({ code: 'LOCKED', message: 'Wallet locked', retryable: false }), {
    code: 'LOCKED', message: 'Wallet locked', retryable: false,
  });
  const hostile = normalizeBrokerError({
    code: 'CANARY_SECRET_CODE', message: 'CANARY wallet path', retryable: 'yes',
    stack: '/secret/wallet', rpc: { seed: 'CANARY' },
  });
  assert.deepStrictEqual(hostile, { code: 'INTERNAL', message: 'Unavailable', retryable: false });
  assert.ok(!JSON.stringify(hostile).includes('CANARY'));
  const recognizedHostile = normalizeBrokerError({
    code: 'LOCKED',
    message: 'CANARY /home/user/wallet backtrace',
    retryable: true,
  });
  assert.deepStrictEqual(recognizedHostile, {
    code: 'LOCKED', message: 'Wallet locked', retryable: false,
  });
  assert.ok(!JSON.stringify(recognizedHostile).includes('CANARY'));
});

function run() {
  let failed = 0;
  for (const { name, fn } of tests) {
    try { fn(); process.stdout.write(`ok ${name}\n`); }
    catch (error) { failed += 1; process.stderr.write(`not ok ${name}\n${error.stack || error}\n`); }
  }
  if (failed) process.exit(1);
  process.stdout.write(`BitBook wallet broker protocol tests passed (${tests.length}).\n`);
}

if (require.main === module) run();
module.exports = { tests };
