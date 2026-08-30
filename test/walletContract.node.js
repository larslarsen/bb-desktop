'use strict';

const assert = require('assert');
const crypto = require('crypto');
const fs = require('fs');
const path = require('path');

const fixturePath = path.join(__dirname, 'fixtures', 'wallet-contract', 'golden-v1.json');
const fixtureBytes = fs.readFileSync(fixturePath);
const fixture = JSON.parse(fixtureBytes.toString('utf8'));

assert.ok(Array.isArray(fixture.vectors), 'golden fixture must contain a vectors array');
assert.strictEqual(fixture.vectors.length, 3, 'golden fixture must contain all three signed objects');
assert.ok(Array.isArray(fixture.invalid_vectors), 'golden fixture must contain invalid_vectors');
assert.ok(fixture.invalid_vectors.length >= 18, 'golden fixture must contain the required invalid families');
const fixtureNames = new Set();
for (const vector of fixture.vectors) {
  assert.match(vector.name, /^[a-z0-9]+(?:-[a-z0-9]+)*$/);
  assert.ok(!fixtureNames.has(vector.name), `duplicate fixture name ${vector.name}`);
  fixtureNames.add(vector.name);
  assert.strictEqual(vector.classification, 'valid');
  assert.strictEqual(typeof vector.canonical, 'string');
  assert.match(vector.digest, /^[0-9a-f]{64}$/);
  const independentDigest = crypto
    .createHash('sha256')
    .update(Buffer.from(vector.domain_separator, 'utf8'))
    .update(Buffer.from(vector.canonical, 'utf8'))
    .digest('hex');
  assert.strictEqual(independentDigest, vector.digest, `fixture digest mismatch for ${vector.name}`);
}
for (const vector of fixture.invalid_vectors) {
  assert.match(vector.name, /^[a-z0-9]+(?:-[a-z0-9]+)*$/);
  assert.ok(!fixtureNames.has(vector.name), `duplicate fixture name ${vector.name}`);
  fixtureNames.add(vector.name);
  assert.ok(
    ['payment_request_v1', 'payment_status_event_v1', 'review_image_v1'].includes(vector.kind),
    `invalid fixture kind is not stable for ${vector.name}`
  );
  assert.strictEqual(vector.classification, 'invalid');
  assert.match(vector.expected_code, /^(SCHEMA|MIGRATION_REQUIRED)$/);
  assert.ok(typeof vector.reason === 'string' && vector.reason.length > 0);
  const representations = ['input', 'raw_json', 'hex_bytes'].filter((key) =>
    Object.prototype.hasOwnProperty.call(vector, key)
  );
  assert.deepStrictEqual(representations.length, 1, `invalid fixture ${vector.name} needs one representation`);
  if (representations[0] === 'input') {
    assert.ok(vector.input && typeof vector.input === 'object' && !Array.isArray(vector.input));
  } else if (representations[0] === 'raw_json') {
    assert.strictEqual(typeof vector.raw_json, 'string');
  } else {
    assert.strictEqual(typeof vector.hex_bytes, 'string');
    assert.match(vector.hex_bytes, /^(?:[0-9a-f]{2})+$/);
  }
}
const requiredInvalidNames = [
  'payment-request-duplicate-key',
  'payment-request-malformed-utf8-memo',
  'payment-request-unknown-field',
  'payment-request-missing-memo',
  'payment-request-zero-amount',
  'payment-request-leading-zero-amount',
  'payment-request-scientific-amount',
  'payment-request-invented-ironwood-receiver',
  'payment-request-status-field',
  'payment-request-impossible-calendar-date',
  'payment-request-out-of-range-timestamp',
  'payment-request-non-nfc-memo',
  'payment-request-bidi-control',
  'payment-request-format-control',
  'payment-status-paid-without-tx-ref',
  'review-fee-above-bound',
  'review-orchard-v1-pool',
  'payment-request-rate-field',
  'review-fiat-field',
];
assert.deepStrictEqual(
  fixture.invalid_vectors.map((vector) => vector.name),
  requiredInvalidNames,
  'golden fixture invalid vector inventory changed'
);
const preflightRequest = fixture.vectors.find((vector) => vector.name === 'payment-request-zec-positive');
const preflightMalformed = fixture.invalid_vectors.find(
  (vector) => vector.name === 'payment-request-malformed-utf8-memo'
);
const preflightNeighbor = Buffer.from(preflightRequest.canonical, 'utf8');
const preflightMemoOffset = preflightNeighbor.indexOf(Buffer.from('coffee', 'utf8'));
assert.ok(preflightMemoOffset >= 0, 'positive request memo bytes are missing');
assert.deepStrictEqual(
  Buffer.from(preflightMalformed.hex_bytes, 'hex'),
  Buffer.concat([
    preflightNeighbor.subarray(0, preflightMemoOffset),
    Buffer.from([0x63, 0xc3, 0x28, 0x66, 0x65, 0x65]),
    preflightNeighbor.subarray(preflightMemoOffset + Buffer.byteLength('coffee', 'utf8')),
  ]),
  'malformed UTF-8 fixture must differ only in memo bytes'
);

// Deliberately load the implementation only after the committed fixture has been read,
// parsed, classified, and independently hashed. The first red must be MODULE_NOT_FOUND.
const walletContract = require('../wallet-contract');

const {
  createFakeAdapter,
  createFakeSigner,
  createFrameDecoder,
  createIntentMachine,
  decodeSignedObject,
  encodeFrame,
  evaluateCapability,
  sanitizeLog,
} = walletContract;

const CONTROL_LIMIT = 64 * 1024;
const ABSOLUTE_LIMIT = 1024 * 1024;
const NOW = '2026-08-30T12:00:30Z';

const goldenRequest = fixture.vectors.find((vector) => vector.kind === 'payment_request_v1');
const goldenStatus = fixture.vectors.find((vector) => vector.kind === 'payment_status_event_v1');
const goldenReview = fixture.vectors.find((vector) => vector.kind === 'review_image_v1');

assert.ok(goldenRequest && goldenStatus && goldenReview, 'fixture kinds are incomplete');
assert.strictEqual(
  crypto.createHash('sha256').update(Buffer.from('coffee', 'utf8')).digest('hex'),
  goldenReview.input.memo_hash,
  'memo hash fixture must be independent SHA-256 of UTF-8 memo bytes'
);

function clone(value) {
  return JSON.parse(JSON.stringify(value));
}

function bytes(value) {
  return Buffer.from(JSON.stringify(value), 'utf8');
}

function invalidFixtureBytes(vector) {
  if (Object.prototype.hasOwnProperty.call(vector, 'input')) return bytes(vector.input);
  if (Object.prototype.hasOwnProperty.call(vector, 'raw_json')) {
    return Buffer.from(vector.raw_json, 'utf8');
  }
  return Buffer.from(vector.hex_bytes, 'hex');
}

function assertCode(fn, expectedCode) {
  let error;
  try {
    fn();
  } catch (caught) {
    error = caught;
  }
  assert.ok(error, `expected ${expectedCode}, but operation succeeded`);
  assert.strictEqual(error.code, expectedCode, error && error.stack ? error.stack : String(error));
}

function assertFailure(result, expectedCode, expectedState = 'failed') {
  assert.ok(result && typeof result === 'object', 'operation must return a result object');
  assert.strictEqual(result.ok, false);
  assert.strictEqual(result.error_code, expectedCode);
  if (expectedState !== null) {
    assert.strictEqual(result.state, expectedState);
  }
}

function mutate(base, changes) {
  return Object.assign(clone(base), changes);
}

function wirePayload(payload) {
  const body = Buffer.isBuffer(payload) ? payload : Buffer.from(payload, 'utf8');
  const prefix = Buffer.alloc(4);
  prefix.writeUInt32BE(body.length, 0);
  return Buffer.concat([prefix, body]);
}

function jsonObjectBytes(size) {
  assert.ok(size >= 8);
  const body = Buffer.from(`{"x":"${'a'.repeat(size - 8)}"}`, 'utf8');
  assert.strictEqual(body.length, size);
  return body;
}

function zecCapabilities(overrides = {}) {
  return Object.assign(
    {
      can_view: true,
      can_derive_fresh_receiver: true,
      can_receive_private: true,
      can_receive_orchard: true,
      can_receive_ironwood: true,
      can_prepare_tx: true,
      can_sign_spend: true,
      can_sign_orchard: true,
      can_sign_ironwood: true,
      can_tx_v6: true,
      can_migrate_orchard_to_ironwood: false,
      can_sign_transparent: false,
      can_display_amount_on_device: false,
      can_display_recipient_on_device: false,
      can_display_network_on_device: false,
      can_verify_pczt_on_device: true,
      can_export_viewing_material: true,
      can_broadcast: false,
      consensus_branch: 'nu6.3-test-fixture',
      pczt_version: 'v6-fixture',
      tx_version_max: '6',
    },
    overrides
  );
}

function xmrCapabilities(overrides = {}) {
  return Object.assign(
    {
      can_view: true,
      can_derive_fresh_receiver: true,
      can_receive_private: true,
      can_receive_orchard: false,
      can_receive_ironwood: false,
      can_prepare_tx: true,
      can_sign_spend: true,
      can_sign_orchard: false,
      can_sign_ironwood: false,
      can_tx_v6: false,
      can_migrate_orchard_to_ironwood: false,
      can_sign_transparent: false,
      can_display_amount_on_device: false,
      can_display_recipient_on_device: false,
      can_display_network_on_device: false,
      can_verify_pczt_on_device: false,
      can_export_viewing_material: true,
      can_broadcast: false,
      consensus_branch: 'xmr-fixture-hf',
      pczt_version: null,
      tx_version_max: null,
    },
    overrides
  );
}

function account(asset, kind, overrides = {}) {
  const isZec = asset === 'ZEC';
  return Object.assign(
    {
      account_id: 'account-test-1',
      asset,
      network: isZec ? 'zec-testnet' : 'xmr-stagenet',
      kind,
      privacy: 'private',
      device_present: true,
      probed_at: '2026-08-30T12:00:00Z',
      probe_source: 'static_fixture',
      restored_pool: isZec ? 'ironwood' : null,
      capabilities: isZec ? zecCapabilities() : xmrCapabilities(),
    },
    overrides
  );
}

function requestFor(asset) {
  if (asset === 'ZEC') {
    return clone(goldenRequest.input);
  }
  return mutate(goldenRequest.input, {
    asset: 'XMR',
    network: 'xmr-stagenet',
    amount_atomic: '1000000000000',
    receiver: '5syntheticxmrsubaddress',
    receiver_kind: 'xmr-subaddress',
  });
}

function reviewFor(asset, request) {
  if (asset === 'ZEC') {
    return clone(goldenReview.input);
  }
  const requestHash = decodeSignedObject('payment_request_v1', bytes(request)).digest;
  return mutate(goldenReview.input, {
    payment_request_hash: requestHash,
    asset: 'XMR',
    network: 'xmr-stagenet',
    amount_atomic: '1000000000000',
    receiver: '5syntheticxmrsubaddress',
    receiver_kind: 'xmr-subaddress',
    change_policy: 'xmr_change',
    tx_version: '0',
    zec_pools: [],
  });
}

function lifecycle(options = {}) {
  const selectedAccount = options.account || account('ZEC', 'software');
  const selectedRequest = options.request || clone(goldenRequest.input);
  const adapter = options.adapter ||
    createFakeAdapter(selectedAccount.asset, {
      review: reviewFor(selectedAccount.asset, selectedRequest),
      internals: options.internals,
    });
  const signer = options.signer ||
    createFakeSigner(selectedAccount.kind, {
      mutation: options.mutation,
      failCode: options.signFailCode,
      disconnected: Boolean(options.disconnected),
      internals: options.internals,
    });
  const machine = createIntentMachine({
    account: selectedAccount,
    request: selectedRequest,
    adapter,
    signer,
    now: options.clock || (() => options.now || NOW),
    getRequestStatus: options.requestStatusSource || (() => options.requestStatus || 'open'),
    restoredState: options.restoredState,
  });
  return { machine, adapter, signer };
}

const STATE_STEPS = {
  preparing: ['begin'],
  prepared: ['begin', 'prepare'],
  awaiting_confirm: ['begin', 'prepare', 'showReview'],
  signing: ['begin', 'prepare', 'showReview', 'confirm'],
  signed_unverified: ['begin', 'prepare', 'showReview', 'confirm', 'completeSign'],
  verified: ['begin', 'prepare', 'showReview', 'confirm', 'completeSign', 'verifySigned'],
};

function advanceTo(ctx, state) {
  for (const method of STATE_STEPS[state]) {
    const result = ctx.machine[method]();
    assert.strictEqual(result.ok, true, `${method} failed while advancing to ${state}`);
  }
  assert.strictEqual(ctx.machine.snapshot().state, state);
}

const tests = [];

function test(name, fn) {
  tests.push({ name, fn });
}

test('golden: fixture domains, strings, hashes, and classifications are exact independent oracles', () => {
  assert.deepStrictEqual(
    fixture.vectors.map((vector) => [vector.kind, vector.domain_separator, vector.digest]),
    [
      ['payment_request_v1', 'bitbook-payment-request-v1\n', 'c21d03fcacab9128ce5d058b6b3b9b95adbf22de222df6d70d92390361ca60dc'],
      ['payment_status_event_v1', 'bitbook-payment-status-v1\n', '9e4b7b6ef01506b93aa76ef1e609a90a70d6fb491dd996e0660881de5c38c3aa'],
      ['review_image_v1', 'bitbook-intent-hash-v1\n', 'ad55816f327c002be813a29d41f9a7ae429782b6856a4d3bb2e6c498c6f9e3c0'],
    ]
  );
});

test('canonical: all three positive golden objects match exact canonical UTF-8 and digests', () => {
  for (const vector of fixture.vectors) {
    const decoded = decodeSignedObject(vector.kind, bytes(vector.input));
    assert.deepStrictEqual(decoded.value, vector.input);
    assert.strictEqual(decoded.canonical, vector.canonical);
    assert.deepStrictEqual(Buffer.from(decoded.canonical_bytes), Buffer.from(vector.canonical, 'utf8'));
    assert.strictEqual(decoded.digest, vector.digest);
  }
});

test('canonical: whitespace and key-order permutations converge to identical bytes and digest', () => {
  const reverse = Object.fromEntries(Object.entries(goldenRequest.input).reverse());
  const pretty = Buffer.from(JSON.stringify(reverse, null, 4), 'utf8');
  const compact = bytes(goldenRequest.input);
  const left = decodeSignedObject('payment_request_v1', pretty);
  const right = decodeSignedObject('payment_request_v1', compact);
  assert.strictEqual(left.canonical, goldenRequest.canonical);
  assert.strictEqual(right.canonical, goldenRequest.canonical);
  assert.strictEqual(left.digest, goldenRequest.digest);
  assert.strictEqual(right.digest, goldenRequest.digest);
});

test('canonical: raw JSON rejects duplicate keys, trailing bytes, BOM, malformed UTF-8, and non-object roots', () => {
  const duplicateVector = fixture.invalid_vectors.find((vector) => vector.name === 'payment-request-duplicate-key');
  const malformedVector = fixture.invalid_vectors.find(
    (vector) => vector.name === 'payment-request-malformed-utf8-memo'
  );
  assert.ok(duplicateVector && malformedVector);
  const neighboringBytes = Buffer.from(goldenRequest.canonical, 'utf8');
  const neighboringValid = decodeSignedObject('payment_request_v1', neighboringBytes);
  assert.strictEqual(neighboringValid.digest, goldenRequest.digest);
  assert.deepStrictEqual(neighboringValid.value, goldenRequest.input);
  const malformedUtf8 = invalidFixtureBytes(malformedVector);
  const memoOffset = neighboringBytes.indexOf(Buffer.from('coffee', 'utf8'));
  assert.ok(memoOffset >= 0);
  const expectedMalformed = Buffer.concat([
    neighboringBytes.subarray(0, memoOffset),
    Buffer.from([0x63, 0xc3, 0x28, 0x66, 0x65, 0x65]),
    neighboringBytes.subarray(memoOffset + Buffer.byteLength('coffee', 'utf8')),
  ]);
  assert.deepStrictEqual(malformedUtf8, expectedMalformed);
  assert.ok(malformedUtf8.includes(Buffer.from([0xc3, 0x28])));
  const cases = [
    invalidFixtureBytes(duplicateVector),
    Buffer.concat([bytes(goldenRequest.input), Buffer.from(' true', 'utf8')]),
    Buffer.concat([Buffer.from([0xef, 0xbb, 0xbf]), bytes(goldenRequest.input)]),
    malformedUtf8,
    Buffer.from([0xa1, 0x61, 0x76, 0x01]),
    Buffer.from('[]', 'utf8'),
    Buffer.from('null', 'utf8'),
    Buffer.from('"object"', 'utf8'),
  ];
  for (const input of cases) {
    assertCode(() => decodeSignedObject('payment_request_v1', input), 'SCHEMA');
  }
});

test('fixture: every committed invalid vector is consumed directly with its exact error code', () => {
  for (const vector of fixture.invalid_vectors) {
    assertCode(() => decodeSignedObject(vector.kind, invalidFixtureBytes(vector)), vector.expected_code);
  }
});

test('schema: all three closed objects require every field and reject wrong JSON types field by field', () => {
  const rows = [
    ['payment_request_v1', goldenRequest.input, []],
    ['payment_status_event_v1', goldenStatus.input, []],
    ['review_image_v1', goldenReview.input, ['zec_pools']],
  ];
  const wrongScalarTypes = [0, 1.5, null, true, false, {}, []];
  const wrongVersionTypes = ['1', 0, 2, -1, 1.5, null, true, false, {}, []];
  const wrongArrayTypes = ['', 0, 1.5, null, true, false, {}];
  const wrongArrayMembers = [0, 1.5, null, true, false, {}, []];
  for (const [kind, positive, arrayFields] of rows) {
    assertCode(() => decodeSignedObject(kind, bytes(mutate(positive, { unexpected: 'closed' }))), 'SCHEMA');
    for (const field of Object.keys(positive)) {
      const missing = clone(positive);
      delete missing[field];
      assertCode(() => decodeSignedObject(kind, bytes(missing)), 'SCHEMA');
      if (arrayFields.includes(field)) continue;
      const candidates = field === 'v' ? wrongVersionTypes : wrongScalarTypes;
      for (const value of candidates) {
        assertCode(() => decodeSignedObject(kind, bytes(mutate(positive, { [field]: value }))), 'SCHEMA');
      }
    }
    for (const field of arrayFields) {
      for (const value of wrongArrayTypes) {
        assertCode(() => decodeSignedObject(kind, bytes(mutate(positive, { [field]: value }))), 'SCHEMA');
      }
      for (const value of wrongArrayMembers) {
        assertCode(() => decodeSignedObject(kind, bytes(mutate(positive, { [field]: [value] }))), 'SCHEMA');
      }
    }
  }
});

test('schema: asset, network, receiver, enum, identifier, and amount combinations fail closed', () => {
  const maximumAmount = mutate(goldenRequest.input, { amount_atomic: '9'.repeat(20) });
  assert.strictEqual(
    decodeSignedObject('payment_request_v1', bytes(maximumAmount)).value.amount_atomic,
    maximumAmount.amount_atomic
  );
  const cases = [
    { asset: 'BTC' },
    { network: 'zec-main' },
    { receiver_kind: 'zec-ua-ironwood' },
    { asset: 'XMR' },
    { network: 'xmr-stagenet' },
    { receiver_kind: 'xmr-subaddress' },
    { receiver: '' },
    { payer_peer_id: '' },
    { request_id: 'ABCDEF' },
    { nonce: '0'.repeat(31) },
    { amount_atomic: '0' },
    { amount_atomic: '00' },
    { amount_atomic: '01' },
    { amount_atomic: '1'.repeat(21) },
    { amount_atomic: '-1' },
    { amount_atomic: '1.0' },
    { amount_atomic: '1e8' },
    { amount_atomic: '1,000' },
    { status: 'open' },
    { fiat: 'USD' },
    { rate: '1.25' },
    { provider: 'synthetic' },
    { quote: {} },
  ];
  for (const changes of cases) {
    assertCode(
      () => decodeSignedObject('payment_request_v1', bytes(mutate(goldenRequest.input, changes))),
      'SCHEMA'
    );
  }
});

test('calendar: request, status, and review timestamps enforce UTC seconds, Gregorian dates, and range', () => {
  const validPairs = [
    ['2020-01-01T00:00:00Z', '2020-01-01T00:00:01Z'],
    ['2024-02-29T12:00:00Z', '2024-02-29T12:00:01Z'],
    ['2100-12-31T23:59:58Z', '2100-12-31T23:59:59Z'],
  ];
  for (const [created_at, expires_at] of validPairs) {
    const input = mutate(goldenRequest.input, { created_at, expires_at });
    assert.strictEqual(decodeSignedObject('payment_request_v1', bytes(input)).value.created_at, created_at);
  }

  const leapStatus = mutate(goldenStatus.input, { at: '2024-02-29T12:00:00Z' });
  assert.strictEqual(decodeSignedObject('payment_status_event_v1', bytes(leapStatus)).value.at, leapStatus.at);
  const leapReview = mutate(goldenReview.input, {
    prepared_at: '2024-02-29T12:00:00Z',
    expires_at: '2024-02-29T12:00:01Z',
  });
  assert.strictEqual(
    decodeSignedObject('review_image_v1', bytes(leapReview)).value.prepared_at,
    leapReview.prepared_at
  );

  const invalidTimestamps = [
    '2026-02-30T00:00:00Z',
    '2026-02-29T00:00:00Z',
    '2026-04-31T00:00:00Z',
    '2026-01-01T24:00:00Z',
    '2026-01-01T23:59:60Z',
    '2026-01-01T00:00:00+00:00',
    '2026-01-01T00:00:00.000Z',
    '0000-01-01T00:00:00Z',
    '2019-12-31T23:59:59Z',
    '2101-01-01T00:00:00Z',
    '2026-13-01T00:00:00Z',
    '2026-00-01T00:00:00Z',
  ];
  for (const timestamp of invalidTimestamps) {
    for (const field of ['created_at', 'expires_at']) {
      assertCode(
        () => decodeSignedObject('payment_request_v1', bytes(mutate(goldenRequest.input, { [field]: timestamp }))),
        'SCHEMA'
      );
    }
    assertCode(
      () => decodeSignedObject('payment_status_event_v1', bytes(mutate(goldenStatus.input, { at: timestamp }))),
      'SCHEMA'
    );
    for (const field of ['prepared_at', 'expires_at']) {
      assertCode(
        () => decodeSignedObject('review_image_v1', bytes(mutate(goldenReview.input, { [field]: timestamp }))),
        'SCHEMA'
      );
    }
  }
  for (const expires_at of ['2026-08-30T12:00:00Z', '2026-08-30T11:59:59Z']) {
    const input = mutate(goldenRequest.input, { expires_at });
    assertCode(() => decodeSignedObject('payment_request_v1', bytes(input)), 'SCHEMA');
  }
});

test('unicode: signed strings reject malformed, control, noncharacter, surrogate, bidi, and format codepoints', () => {
  const forbidden = [
    0x202a, 0x202b, 0x202c, 0x202d, 0x202e,
    0x2066, 0x2067, 0x2068, 0x2069,
    0x200b, 0x200c, 0x200d, 0x200e, 0x200f,
    0x061c, 0x2060, 0x206a, 0x206b, 0x206c, 0x206d, 0x206e, 0x206f,
    0xfeff, 0xfff9, 0xfffa, 0xfffb,
  ];
  for (let codepoint = 0x0000; codepoint <= 0x001f; codepoint += 1) forbidden.push(codepoint);
  for (let codepoint = 0x007f; codepoint <= 0x009f; codepoint += 1) forbidden.push(codepoint);
  for (let codepoint = 0xfdd0; codepoint <= 0xfdef; codepoint += 1) forbidden.push(codepoint);
  for (let plane = 0; plane <= 16; plane += 1) {
    forbidden.push((plane * 0x10000) + 0xfffe, (plane * 0x10000) + 0xffff);
  }
  for (let codepoint = 0xe0001; codepoint <= 0xe007f; codepoint += 1) forbidden.push(codepoint);
  for (const codepoint of forbidden) {
    const input = mutate(goldenRequest.input, { memo: `safe${String.fromCodePoint(codepoint)}text` });
    assertCode(() => decodeSignedObject('payment_request_v1', bytes(input)), 'SCHEMA');
  }
  const unpaired = mutate(goldenRequest.input, { memo: 'safe\ud800text' });
  assertCode(() => decodeSignedObject('payment_request_v1', bytes(unpaired)), 'SCHEMA');
  const nonAsciiPeer = mutate(goldenRequest.input, { payer_peer_id: '12D3KooWPayeré' });
  assertCode(() => decodeSignedObject('payment_request_v1', bytes(nonAsciiPeer)), 'SCHEMA');

  for (const vector of fixture.vectors) {
    for (const [field, value] of Object.entries(vector.input)) {
      if (typeof value !== 'string') continue;
      const input = mutate(vector.input, { [field]: `${value}\u202e` });
      assertCode(() => decodeSignedObject(vector.kind, bytes(input)), 'SCHEMA');
    }
  }
});

test('unicode: memo must already be NFC and is bounded by UTF-8 bytes, not code units', () => {
  const nfcAtLimit = mutate(goldenRequest.input, { memo: 'é'.repeat(256) });
  assert.strictEqual(Buffer.byteLength(nfcAtLimit.memo, 'utf8'), 512);
  assert.strictEqual(decodeSignedObject('payment_request_v1', bytes(nfcAtLimit)).value.memo, nfcAtLimit.memo);

  const nonNfc = mutate(goldenRequest.input, { memo: 'cafe\u0301' });
  const overLimit = mutate(goldenRequest.input, { memo: 'é'.repeat(257) });
  assertCode(() => decodeSignedObject('payment_request_v1', bytes(nonNfc)), 'SCHEMA');
  assertCode(() => decodeSignedObject('payment_request_v1', bytes(overLimit)), 'SCHEMA');
});

test('schema: status and review objects are closed and enforce status, fee, tx, pool, and rate absence', () => {
  const statusCases = [
    mutate(goldenStatus.input, { unknown: 'x' }),
    mutate(goldenStatus.input, { status: 'open' }),
    mutate(goldenStatus.input, { status: 'paid', tx_ref: '' }),
    mutate(goldenStatus.input, { status: 'cancelled', tx_ref: 'synthetic-ref' }),
    mutate(goldenStatus.input, { v: '1' }),
  ];
  const missingEventId = clone(goldenStatus.input);
  delete missingEventId.event_id;
  statusCases.push(missingEventId);
  for (const input of statusCases) {
    assertCode(() => decodeSignedObject('payment_status_event_v1', bytes(input)), 'SCHEMA');
  }

  const reviewCases = [
    { fee_atomic: '12001' },
    { fee_atomic: '01' },
    { fee_atomic: 10000 },
    { fee_atomic: 1.25 },
    { fee_atomic: null },
    { fee_atomic: true },
    { fee_bound_atomic: '0' },
    { fee_bound_atomic: false },
    { tx_version: '5' },
    { zec_pools: [] },
    { zec_pools: ['sapling'] },
    { zec_pools: ['ironwood', 'orchard'] },
    { zec_pools: ['ironwood', 'ironwood'] },
    { zec_pools: ['orchard', 'orchard'] },
    { zec_pools: null },
    { zec_pools: true },
    { change_policy: 'transparent' },
    { receiver_kind: 'zec-ua-ironwood' },
    { rate: '1.25' },
    { fiat: 'USD' },
    { quote_provider: 'synthetic' },
    { status: 'prepared' },
  ];
  for (const changes of reviewCases) {
    assertCode(
      () => decodeSignedObject('review_image_v1', bytes(mutate(goldenReview.input, changes))),
      'SCHEMA'
    );
  }
  const missingFee = clone(goldenReview.input);
  delete missingFee.fee_atomic;
  assertCode(() => decodeSignedObject('review_image_v1', bytes(missingFee)), 'SCHEMA');
  assertCode(
    () => decodeSignedObject('review_image_v1', bytes(mutate(goldenReview.input, { zec_pools: ['orchard'] }))),
    'MIGRATION_REQUIRED'
  );
});

test('schema: direct XMR request and review decode with stagenet subaddress and no ZEC fields', () => {
  const request = requestFor('XMR');
  const decodedRequest = decodeSignedObject('payment_request_v1', bytes(request));
  assert.deepStrictEqual(decodedRequest.value, request);
  assert.strictEqual(decodedRequest.value.network, 'xmr-stagenet');
  assert.strictEqual(decodedRequest.value.receiver_kind, 'xmr-subaddress');
  assert.match(decodedRequest.digest, /^[0-9a-f]{64}$/);

  const review = reviewFor('XMR', request);
  const decodedReview = decodeSignedObject('review_image_v1', bytes(review));
  assert.deepStrictEqual(decodedReview.value, review);
  assert.strictEqual(decodedReview.value.network, 'xmr-stagenet');
  assert.strictEqual(decodedReview.value.receiver_kind, 'xmr-subaddress');
  assert.strictEqual(decodedReview.value.change_policy, 'xmr_change');
  assert.strictEqual(decodedReview.value.tx_version, '0');
  assert.deepStrictEqual(decodedReview.value.zec_pools, []);
  assert.match(decodedReview.digest, /^[0-9a-f]{64}$/);
});

test('schema: every Network enum decodes with its matching asset, receiver, and review fields', () => {
  const rows = [
    ['ZEC', 'zec-mainnet', 'zec-ua-orchard-protocol', 'shielded_internal', '6', ['ironwood']],
    ['ZEC', 'zec-testnet', 'zec-ua-orchard-protocol', 'shielded_internal', '6', ['ironwood']],
    ['ZEC', 'zec-regtest', 'zec-ua-orchard-protocol', 'shielded_internal', '6', ['ironwood']],
    ['XMR', 'xmr-mainnet', 'xmr-subaddress', 'xmr_change', '0', []],
    ['XMR', 'xmr-stagenet', 'xmr-subaddress', 'xmr_change', '0', []],
    ['XMR', 'xmr-testnet', 'xmr-subaddress', 'xmr_change', '0', []],
  ];
  for (const [asset, network, receiverKind, changePolicy, txVersion, pools] of rows) {
    const request = mutate(requestFor(asset), { network });
    const decodedRequest = decodeSignedObject('payment_request_v1', bytes(request));
    assert.deepStrictEqual(decodedRequest.value, request);
    assert.strictEqual(decodedRequest.value.asset, asset);
    assert.strictEqual(decodedRequest.value.network, network);
    assert.strictEqual(decodedRequest.value.receiver_kind, receiverKind);

    const review = mutate(reviewFor(asset, request), {
      network,
      payment_request_hash: decodedRequest.digest,
    });
    const decodedReview = decodeSignedObject('review_image_v1', bytes(review));
    assert.deepStrictEqual(decodedReview.value, review);
    assert.strictEqual(decodedReview.value.asset, asset);
    assert.strictEqual(decodedReview.value.network, network);
    assert.strictEqual(decodedReview.value.receiver_kind, receiverKind);
    assert.strictEqual(decodedReview.value.change_policy, changePolicy);
    assert.strictEqual(decodedReview.value.tx_version, txVersion);
    assert.deepStrictEqual(decodedReview.value.zec_pools, pools);
  }
});

test('schema: request and review reject cross-asset network, receiver, change, tx, and pool combinations', () => {
  const xmrRequest = requestFor('XMR');
  const requestCases = [
    mutate(goldenRequest.input, { asset: 'XMR' }),
    mutate(goldenRequest.input, { network: 'xmr-stagenet' }),
    mutate(goldenRequest.input, { receiver_kind: 'xmr-subaddress' }),
    mutate(xmrRequest, { asset: 'ZEC' }),
    mutate(xmrRequest, { network: 'zec-testnet' }),
    mutate(xmrRequest, { receiver_kind: 'zec-ua-orchard-protocol' }),
  ];
  for (const input of requestCases) {
    assertCode(() => decodeSignedObject('payment_request_v1', bytes(input)), 'SCHEMA');
  }

  const xmrReview = reviewFor('XMR', xmrRequest);
  const reviewCases = [
    mutate(goldenReview.input, { asset: 'BTC' }),
    mutate(goldenReview.input, { network: 'zec-main' }),
    mutate(goldenReview.input, { asset: 'XMR' }),
    mutate(goldenReview.input, { network: 'xmr-stagenet' }),
    mutate(goldenReview.input, { receiver_kind: 'xmr-subaddress' }),
    mutate(goldenReview.input, { change_policy: 'xmr_change' }),
    mutate(goldenReview.input, { tx_version: '0' }),
    mutate(xmrReview, { asset: 'ZEC' }),
    mutate(xmrReview, { network: 'zec-testnet' }),
    mutate(xmrReview, { receiver_kind: 'zec-ua-orchard-protocol' }),
    mutate(xmrReview, { change_policy: 'shielded_internal' }),
    mutate(xmrReview, { tx_version: '6' }),
    mutate(xmrReview, { zec_pools: ['ironwood'] }),
    mutate(xmrReview, { zec_pools: ['orchard'] }),
  ];
  for (const input of reviewCases) {
    assertCode(() => decodeSignedObject('review_image_v1', bytes(input)), 'SCHEMA');
  }
});

test('schema: status identifiers are lexical and paid or expired tx-ref relations decode positively', () => {
  for (const field of ['request_id', 'event_id', 'nonce']) {
    for (const value of ['', '0'.repeat(31), 'A'.repeat(32), `${'0'.repeat(31)}g`]) {
      assertCode(
        () => decodeSignedObject('payment_status_event_v1', bytes(mutate(goldenStatus.input, { [field]: value }))),
        'SCHEMA'
      );
    }
  }
  const paid = mutate(goldenStatus.input, {
    event_id: '22223333444455556666777788889999',
    nonce: 'aaaabbbbccccddddeeeeffff00001111',
    status: 'paid',
    tx_ref: 'synthetic-paid-reference',
  });
  assert.deepStrictEqual(decodeSignedObject('payment_status_event_v1', bytes(paid)).value, paid);
  const expired = mutate(goldenStatus.input, {
    event_id: '3333444455556666777788889999aaaa',
    nonce: 'bbbbccccddddeeeeffff000011112222',
    status: 'expired',
    tx_ref: '',
  });
  assert.deepStrictEqual(decodeSignedObject('payment_status_event_v1', bytes(expired)).value, expired);
});

test('schema: review identifiers and hashes reject malformed lexemes and blank authoritative strings', () => {
  for (const [field, width] of [
    ['request_id', 32],
    ['payment_request_hash', 64],
    ['memo_hash', 64],
  ]) {
    for (const value of ['', '0'.repeat(width - 1), 'A'.repeat(width), `${'0'.repeat(width - 1)}g`]) {
      assertCode(
        () => decodeSignedObject('review_image_v1', bytes(mutate(goldenReview.input, { [field]: value }))),
        'SCHEMA'
      );
    }
  }
  for (const field of [
    'intent_id',
    'prepared_id',
    'account_id',
    'payer_peer_id',
    'payee_peer_id',
    'receiver',
  ]) {
    assertCode(
      () => decodeSignedObject('review_image_v1', bytes(mutate(goldenReview.input, { [field]: '' }))),
      'SCHEMA'
    );
  }
});

test('framing: one-byte delivery, split prefix, multiple frames, and unread bytes are preserved', () => {
  const values = [{ v: 1, kind: 'first' }, {}, { v: 1, kind: 'third' }];
  const frames = values.map((value) => encodeFrame(value));
  const oneByte = createFrameDecoder({ limitBytes: CONTROL_LIMIT });
  const emitted = [];
  for (const byte of frames[0]) {
    emitted.push(...oneByte.push(Buffer.from([byte])));
  }
  assert.deepStrictEqual(emitted, [values[0]]);

  const split = createFrameDecoder({ limitBytes: CONTROL_LIMIT });
  assert.deepStrictEqual(split.push(frames[1].subarray(0, 2)), []);
  assert.deepStrictEqual(split.push(frames[1].subarray(2)), [values[1]]);

  const multiple = createFrameDecoder({ limitBytes: CONTROL_LIMIT });
  assert.deepStrictEqual(multiple.push(Buffer.concat(frames)), values);

  const unread = createFrameDecoder({ limitBytes: CONTROL_LIMIT });
  const firstPlusPrefix = Buffer.concat([frames[0], frames[2].subarray(0, 3)]);
  assert.deepStrictEqual(unread.push(firstPlusPrefix), [values[0]]);
  assert.deepStrictEqual(unread.push(frames[2].subarray(3)), [values[2]]);
});

test('framing: control and absolute frame limits accept exact bytes and reject limit plus one', () => {
  for (const limitBytes of [CONTROL_LIMIT, ABSOLUTE_LIMIT]) {
    const exact = createFrameDecoder({ limitBytes });
    const exactBody = jsonObjectBytes(limitBytes);
    const decoded = exact.push(wirePayload(exactBody));
    assert.strictEqual(decoded.length, 1);
    assert.strictEqual(decoded[0].x.length, limitBytes - 8);
    assert.strictEqual(exact.closed, false);

    const oversize = createFrameDecoder({ limitBytes });
    assertCode(() => oversize.push(wirePayload(jsonObjectBytes(limitBytes + 1))), 'LIMIT');
    assert.strictEqual(oversize.closed, true);
  }
  const cannotRaiseAbsoluteCeiling = createFrameDecoder({ limitBytes: ABSOLUTE_LIMIT + 1 });
  assertCode(
    () => cannotRaiseAbsoluteCeiling.push(wirePayload(jsonObjectBytes(ABSOLUTE_LIMIT + 1))),
    'LIMIT'
  );
  assert.strictEqual(cannotRaiseAbsoluteCeiling.closed, true);
});

test('framing: zero, invalid, malformed, trailing, and oversize frames close permanently', () => {
  const zero = Buffer.alloc(4);
  const invalidLength = Buffer.alloc(4);
  invalidLength.writeUInt32BE(0xffffffff, 0);
  const malformedUtf8 = wirePayload(Buffer.from([0x7b, 0x22, 0xc3, 0x28, 0x22, 0x7d]));
  const invalidJson = wirePayload('{');
  const trailingJson = wirePayload('{}{}');
  const cases = [
    [zero, 'SCHEMA'],
    [invalidLength, 'LIMIT'],
    [malformedUtf8, 'SCHEMA'],
    [invalidJson, 'SCHEMA'],
    [trailingJson, 'SCHEMA'],
    [wirePayload(jsonObjectBytes(CONTROL_LIMIT + 1)), 'LIMIT'],
  ];
  const valid = encodeFrame({});
  for (const [input, code] of cases) {
    const decoder = createFrameDecoder({ limitBytes: CONTROL_LIMIT });
    assertCode(() => decoder.push(input), code);
    assert.strictEqual(decoder.closed, true);
    assertCode(() => decoder.push(valid), 'SCHEMA');
  }
});

test('capabilities: both assets and all account kinds have explicit receive and spend outcomes', () => {
  const zecWatch = account('ZEC', 'watch_only', {
    capabilities: zecCapabilities({ can_sign_spend: false, can_sign_ironwood: false }),
  });
  const xmrWatch = account('XMR', 'watch_only', {
    capabilities: xmrCapabilities({ can_sign_spend: false }),
  });
  const rows = [
    [account('ZEC', 'software'), requestFor('ZEC'), true, true, null],
    [account('ZEC', 'hardware_backed'), requestFor('ZEC'), true, true, null],
    [zecWatch, requestFor('ZEC'), true, false, 'WATCH_ONLY'],
    [account('XMR', 'software'), requestFor('XMR'), true, true, null],
    [account('XMR', 'hardware_backed'), requestFor('XMR'), true, true, null],
    [xmrWatch, requestFor('XMR'), true, false, 'WATCH_ONLY'],
  ];
  for (const [selectedAccount, request, canReceive, canSpend, errorCode] of rows) {
    const result = evaluateCapability(selectedAccount, request);
    assert.strictEqual(result.can_receive, canReceive);
    assert.strictEqual(result.can_spend, canSpend);
    assert.strictEqual(result.error_code, errorCode);
    assert.strictEqual(result.signer_kind, selectedAccount.kind);
  }
});

test('capabilities: current private ZEC requires NU6.3, v6, Ironwood, PCZT, and no restored Orchard spend', () => {
  const request = requestFor('ZEC');
  const rows = [
    [
      account('ZEC', 'software', { network: 'zec-mainnet' }),
      mutate(request, { network: 'zec-testnet' }),
      'WRONG_NETWORK',
    ],
    [
      account('ZEC', 'software', { capabilities: zecCapabilities({ consensus_branch: 'nu6.2-fixture' }) }),
      request,
      'PROTOCOL_INCOMPATIBLE',
    ],
    [
      account('ZEC', 'software', { capabilities: zecCapabilities({ can_tx_v6: false, tx_version_max: '5' }) }),
      request,
      'PROTOCOL_INCOMPATIBLE',
    ],
    [
      account('ZEC', 'hardware_backed', { capabilities: zecCapabilities({ can_sign_ironwood: false }) }),
      request,
      'CAPABILITY_MISSING',
    ],
    [
      account('ZEC', 'hardware_backed', { capabilities: zecCapabilities({ pczt_version: null }) }),
      request,
      'PROTOCOL_INCOMPATIBLE',
    ],
    [
      account('ZEC', 'software', { restored_pool: 'orchard' }),
      request,
      'MIGRATION_REQUIRED',
    ],
  ];
  for (const [selectedAccount, selectedRequest, errorCode] of rows) {
    const result = evaluateCapability(selectedAccount, selectedRequest);
    assert.strictEqual(result.can_spend, false);
    assert.strictEqual(result.error_code, errorCode);
  }
});

test('capabilities: transparent Trezor, unverified Ledger, disconnect, and signer fallback fail closed', () => {
  const trezor = account('ZEC', 'hardware_backed', {
    privacy: 'transparent_not_private',
    vendor: 'trezor',
    capabilities: zecCapabilities({
      can_receive_private: false,
      can_receive_ironwood: false,
      can_sign_spend: false,
      can_sign_ironwood: false,
      can_sign_transparent: true,
    }),
  });
  const trezorResult = evaluateCapability(trezor, requestFor('ZEC'));
  assert.strictEqual(trezorResult.can_spend, false);
  assert.strictEqual(trezorResult.privacy, 'transparent_not_private');
  assert.strictEqual(trezorResult.error_code, 'CAPABILITY_MISSING');

  const ledger = account('ZEC', 'hardware_backed', {
    vendor: 'ledger',
    capabilities: zecCapabilities({ can_sign_spend: false, can_sign_ironwood: false }),
  });
  assert.strictEqual(evaluateCapability(ledger, requestFor('ZEC')).error_code, 'CAPABILITY_MISSING');

  const xmrDisconnected = account('XMR', 'hardware_backed', { device_present: false });
  const disconnected = evaluateCapability(xmrDisconnected, requestFor('XMR'));
  assert.strictEqual(disconnected.can_spend, false);
  assert.strictEqual(disconnected.error_code, 'DEVICE_DISCONNECTED');
  assert.strictEqual(disconnected.signer_kind, 'hardware_backed');

  const xmrUnprobed = account('XMR', 'hardware_backed', {
    probed_at: null,
    probe_source: null,
  });
  const unprobed = evaluateCapability(xmrUnprobed, requestFor('XMR'));
  assert.strictEqual(unprobed.can_spend, false);
  assert.strictEqual(unprobed.error_code, 'CAPABILITY_MISSING');
});

test('capabilities: quote absence and arbitrary presentation quotes cannot affect eligibility', () => {
  for (const asset of ['ZEC', 'XMR']) {
    const selectedAccount = account(asset, 'software');
    const request = requestFor(asset);
    const absent = evaluateCapability(selectedAccount, request);
    const unavailable = evaluateCapability(selectedAccount, request, { rate: null });
    const hostilePresentation = evaluateCapability(selectedAccount, request, {
      rate: { price: '999999999999', provider: 'synthetic-untrusted' },
    });
    assert.deepStrictEqual(unavailable, absent);
    assert.deepStrictEqual(hostilePresentation, absent);
    assert.strictEqual(absent.can_spend, true);
  }
});

test('lifecycle: prepare creates exact review before one confirmation and inert broadcast fails', () => {
  const ctx = lifecycle();
  assertFailure(ctx.machine.confirm(), 'SCHEMA', 'idle');
  assert.strictEqual(ctx.machine.begin().state, 'preparing');
  assertFailure(ctx.machine.confirm(), 'SCHEMA', 'preparing');

  const prepared = ctx.machine.prepare();
  assert.strictEqual(prepared.ok, true);
  assert.strictEqual(prepared.state, 'prepared');
  assert.deepStrictEqual(prepared.review, goldenReview.input);
  assert.strictEqual(prepared.review.fee_atomic, '10000');
  assert.strictEqual(prepared.review.fee_bound_atomic, '12000');
  assert.strictEqual(ctx.adapter.calls.prepare, 1);

  assertFailure(ctx.machine.confirm(), 'SCHEMA', 'prepared');
  assert.strictEqual(ctx.machine.showReview().state, 'awaiting_confirm');
  const confirmation = ctx.machine.confirm();
  assert.strictEqual(confirmation.ok, true);
  assert.strictEqual(confirmation.state, 'signing');
  assert.strictEqual(confirmation.intent_hash, goldenReview.digest);
  assert.strictEqual(ctx.machine.snapshot().confirmation_count, 1);

  assert.strictEqual(ctx.machine.completeSign().state, 'signed_unverified');
  assert.strictEqual(ctx.signer.calls.sign, 1);
  assert.strictEqual(ctx.machine.verifySigned().state, 'verified');
  assert.strictEqual(ctx.adapter.calls.verify, 1);
  const broadcast = ctx.machine.broadcast();
  assertFailure(broadcast, 'UNAVAILABLE');
  assert.strictEqual(ctx.adapter.calls.broadcast, 1);
  assert.strictEqual(broadcast.funds_moved, false);
});

test('lifecycle: every post-sign authoritative-field mutation is INTENT_MISMATCH with zero broadcasts', () => {
  const mutations = [
    { receiver: 'u1syntheticchangedreceiver' },
    { amount_atomic: '100000001' },
    { network: 'zec-mainnet' },
    { fee_atomic: '10001' },
    { request_id: '11112222333344445555666677778888' },
    { memo_hash: '0'.repeat(64) },
    { change_policy: 'none' },
  ];
  for (const mutation of mutations) {
    const ctx = lifecycle({ mutation });
    advanceTo(ctx, 'signed_unverified');
    const verified = ctx.machine.verifySigned();
    assertFailure(verified, 'INTENT_MISMATCH');
    assert.strictEqual(ctx.adapter.calls.verify, 1);
    assert.strictEqual(ctx.adapter.calls.broadcast, 0);
  }
});

test('lifecycle: cancellation wins in every pre-broadcast state including signed and verified', () => {
  for (const state of Object.keys(STATE_STEPS)) {
    const ctx = lifecycle();
    advanceTo(ctx, state);
    const cancelled = ctx.machine.cancel();
    assert.strictEqual(cancelled.ok, true);
    assert.strictEqual(cancelled.state, 'cancelled');
    assert.strictEqual(ctx.adapter.calls.broadcast, 0);
    assertFailure(ctx.machine.broadcast(), 'CANCELLED', 'cancelled');
    assert.strictEqual(ctx.adapter.calls.broadcast, 0);
  }
});

test('lifecycle: expiry wins in every pre-broadcast state including signed and verified', () => {
  for (const state of Object.keys(STATE_STEPS)) {
    const ctx = lifecycle();
    advanceTo(ctx, state);
    const expired = ctx.machine.expire();
    assert.strictEqual(expired.ok, true);
    assert.strictEqual(expired.state, 'expired');
    assert.strictEqual(ctx.adapter.calls.broadcast, 0);
    assertFailure(ctx.machine.broadcast(), 'EXPIRED', 'expired');
    assert.strictEqual(ctx.adapter.calls.broadcast, 0);
  }
});

test('lifecycle: injected clock is re-read after signing and immediately before broadcast', () => {
  let now = NOW;
  const afterSign = lifecycle({ clock: () => now });
  advanceTo(afterSign, 'signed_unverified');
  now = '2026-08-30T12:15:00Z';
  assertFailure(afterSign.machine.verifySigned(), 'EXPIRED', 'expired');
  assert.strictEqual(afterSign.adapter.calls.broadcast, 0);

  now = NOW;
  const beforeBroadcast = lifecycle({ clock: () => now });
  advanceTo(beforeBroadcast, 'verified');
  now = '2026-08-30T12:15:01Z';
  assertFailure(beforeBroadcast.machine.broadcast(), 'EXPIRED', 'expired');
  assert.strictEqual(beforeBroadcast.adapter.calls.broadcast, 0);
});

test('lifecycle: injected request status is re-read after signing and immediately before broadcast', () => {
  let requestStatus = 'open';
  const afterSign = lifecycle({ requestStatusSource: () => requestStatus });
  advanceTo(afterSign, 'signed_unverified');
  requestStatus = 'cancelled';
  assertFailure(afterSign.machine.verifySigned(), 'CANCELLED', 'cancelled');
  assert.strictEqual(afterSign.adapter.calls.broadcast, 0);

  requestStatus = 'open';
  const beforeBroadcast = lifecycle({ requestStatusSource: () => requestStatus });
  advanceTo(beforeBroadcast, 'verified');
  requestStatus = 'cancelled';
  assertFailure(beforeBroadcast.machine.broadcast(), 'CANCELLED', 'cancelled');
  assert.strictEqual(beforeBroadcast.adapter.calls.broadcast, 0);
});

test('lifecycle: watch-only, disconnect, signer, capability, and concurrent prepare failures are closed', () => {
  const watchAccount = account('ZEC', 'watch_only', {
    capabilities: zecCapabilities({ can_sign_spend: false, can_sign_ironwood: false }),
  });
  const watch = lifecycle({
    account: watchAccount,
    signer: createFakeSigner('watch_only'),
  });
  assert.strictEqual(watch.machine.begin().ok, true);
  assertFailure(watch.machine.prepare(), 'WATCH_ONLY');
  assert.strictEqual(watch.signer.calls.sign, 0);
  assert.strictEqual(watch.adapter.calls.broadcast, 0);

  const disconnected = lifecycle({
    account: account('ZEC', 'hardware_backed'),
    signer: createFakeSigner('hardware_backed', { disconnected: true }),
  });
  advanceTo(disconnected, 'signing');
  assertFailure(disconnected.machine.completeSign(), 'DEVICE_DISCONNECTED');
  assert.strictEqual(disconnected.adapter.calls.broadcast, 0);

  const failedSigner = lifecycle({ signFailCode: 'UNAVAILABLE' });
  advanceTo(failedSigner, 'signing');
  assertFailure(failedSigner.machine.completeSign(), 'UNAVAILABLE');
  assert.strictEqual(failedSigner.adapter.calls.broadcast, 0);

  const incapableAccount = account('ZEC', 'software', {
    capabilities: zecCapabilities({ can_prepare_tx: false }),
  });
  const incapable = lifecycle({ account: incapableAccount });
  assert.strictEqual(incapable.machine.begin().ok, true);
  assertFailure(incapable.machine.prepare(), 'CAPABILITY_MISSING');
  assert.strictEqual(incapable.adapter.calls.prepare, 0);

  const sharedAdapter = createFakeAdapter('ZEC', { review: clone(goldenReview.input) });
  const first = lifecycle({ adapter: sharedAdapter });
  const second = lifecycle({ adapter: sharedAdapter });
  assert.strictEqual(first.machine.begin().ok, true);
  assert.strictEqual(first.machine.prepare().ok, true);
  assert.strictEqual(second.machine.begin().ok, true);
  assertFailure(second.machine.prepare(), 'ACCOUNT_BUSY');
  assert.strictEqual(sharedAdapter.calls.prepare, 1);
  assert.strictEqual(sharedAdapter.calls.broadcast, 0);
});

test('lifecycle: durable paid, cancelled, and expired request status blocks replay before prepare', () => {
  for (const [requestStatus, errorCode, state] of [
    ['paid', 'REPLAY', 'failed'],
    ['cancelled', 'CANCELLED', 'cancelled'],
    ['expired', 'EXPIRED', 'expired'],
  ]) {
    const ctx = lifecycle({ requestStatus });
    assertFailure(ctx.machine.begin(), errorCode, state);
    assert.strictEqual(ctx.adapter.calls.prepare, 0);
    assert.strictEqual(ctx.signer.calls.sign, 0);
    assert.strictEqual(ctx.adapter.calls.broadcast, 0);
  }
});

test('recovery: unsigned crashes abort while signed artifacts require fresh confirmation and revalidation', () => {
  for (const state of ['preparing', 'prepared', 'awaiting_confirm', 'signing']) {
    const ctx = lifecycle();
    advanceTo(ctx, state);
    const crashed = ctx.machine.crash();
    assert.strictEqual(crashed.state, 'failed');
    assert.strictEqual(crashed.error_code, 'UNAVAILABLE');
    assert.strictEqual(ctx.adapter.calls.broadcast, 0);
  }

  for (const state of ['signed_unverified', 'verified']) {
    const ctx = lifecycle();
    advanceTo(ctx, state);
    assert.strictEqual(ctx.machine.crash().state, 'crash_recovery');
    assert.strictEqual(ctx.adapter.calls.broadcast, 0);
    const recovered = ctx.machine.confirmRecovery();
    assert.strictEqual(recovered.ok, true);
    assert.strictEqual(recovered.state, 'signed_unverified');
    assert.strictEqual(ctx.machine.snapshot().confirmation_count, 2);
    assert.strictEqual(ctx.machine.verifySigned().state, 'verified');
    assert.strictEqual(ctx.adapter.calls.broadcast, 0);
  }
});

test('recovery: recovered artifact mutation fails fresh revalidation without broadcasting', () => {
  const mutatedReview = mutate(goldenReview.input, { receiver: 'u1syntheticchangedreceiver' });
  const ctx = lifecycle({
    restoredState: {
      v: 1,
      state: 'crash_recovery',
      review: clone(goldenReview.input),
      intent_hash: goldenReview.digest,
      confirmation_count: 1,
      signed_artifact: {
        kind: 'synthetic-signed-artifact',
        review: mutatedReview,
      },
    },
  });
  assert.strictEqual(ctx.machine.snapshot().state, 'crash_recovery');
  assert.strictEqual(ctx.adapter.calls.broadcast, 0);
  const recovered = ctx.machine.confirmRecovery();
  assert.strictEqual(recovered.ok, true);
  assert.strictEqual(recovered.state, 'signed_unverified');
  assert.strictEqual(ctx.machine.snapshot().confirmation_count, 2);
  assertFailure(ctx.machine.verifySigned(), 'INTENT_MISMATCH');
  assert.strictEqual(ctx.adapter.calls.verify, 1);
  assert.strictEqual(ctx.adapter.calls.broadcast, 0);
});

test('recovery: re-read cancellation and expiry win while the machine is in crash_recovery', () => {
  let requestStatus = 'open';
  const cancelled = lifecycle({ requestStatusSource: () => requestStatus });
  advanceTo(cancelled, 'signed_unverified');
  assert.strictEqual(cancelled.machine.crash().state, 'crash_recovery');
  requestStatus = 'cancelled';
  assertFailure(cancelled.machine.confirmRecovery(), 'CANCELLED', 'cancelled');
  assert.strictEqual(cancelled.adapter.calls.broadcast, 0);

  let now = NOW;
  const expired = lifecycle({ clock: () => now });
  advanceTo(expired, 'signed_unverified');
  assert.strictEqual(expired.machine.crash().state, 'crash_recovery');
  now = '2026-08-30T12:15:01Z';
  assertFailure(expired.machine.confirmRecovery(), 'EXPIRED', 'expired');
  assert.strictEqual(expired.adapter.calls.broadcast, 0);
});

test('recovery: a broadcasting crash becomes unknown_needs_scan and never blindly resubmits', () => {
  const ctx = lifecycle({
    restoredState: {
      v: 1,
      state: 'broadcasting',
      review: clone(goldenReview.input),
      intent_hash: goldenReview.digest,
      confirmation_count: 1,
      signed_artifact: {
        kind: 'synthetic-signed-artifact',
        review: clone(goldenReview.input),
      },
    },
  });
  assert.strictEqual(ctx.machine.snapshot().state, 'broadcasting');
  assert.strictEqual(ctx.adapter.calls.broadcast, 0);
  assert.strictEqual(ctx.machine.crash().state, 'unknown_needs_scan');
  assertFailure(ctx.machine.resume(), 'UNAVAILABLE', 'unknown_needs_scan');
  assert.strictEqual(ctx.adapter.calls.broadcast, 0);
});

test('fakes: ZEC/XMR adapters and all signer kinds expose only synthetic artifacts and no success broadcast', () => {
  for (const asset of ['ZEC', 'XMR']) {
    const adapter = createFakeAdapter(asset, { review: clone(goldenReview.input) });
    const result = adapter.broadcast({ kind: 'synthetic-signed-artifact', asset });
    assert.strictEqual(result.ok, false);
    assert.ok(['UNAVAILABLE', 'CAPABILITY_MISSING'].includes(result.error_code));
    assert.strictEqual(result.funds_moved, false);
    assert.strictEqual(adapter.calls.broadcast, 1);
    assert.ok(!Object.prototype.hasOwnProperty.call(result, 'txid'));
    assert.ok(!Object.prototype.hasOwnProperty.call(result, 'raw_transaction'));
  }
  for (const kind of ['software', 'hardware_backed', 'watch_only']) {
    const signer = createFakeSigner(kind);
    const signed = signer.sign({ kind: 'synthetic-prepared-artifact', review: clone(goldenReview.input) });
    if (kind === 'watch_only') {
      assert.strictEqual(signed.ok, false);
      assert.strictEqual(signed.error_code, 'WATCH_ONLY');
    } else {
      assert.strictEqual(signed.ok, true);
      assert.strictEqual(signed.artifact.kind, 'synthetic-signed-artifact');
      assert.ok(!Object.prototype.hasOwnProperty.call(signed.artifact, 'raw_transaction'));
      assert.ok(!Object.prototype.hasOwnProperty.call(signed.artifact, 'key_material'));
    }
  }
});

test('secrets: normalized logs, snapshots, and fake failures omit every synthetic canary', () => {
  const canaries = [
    'CANARY-SEED-WORDS-alpha-beta-gamma-delta',
    `CANARY_SPEND_KEY_${'a'.repeat(64)}`,
    'CANARY_PIN_123456',
    'CANARY_PASSPHRASE_DO_NOT_LOG',
    `CANARY_RAW_TRANSACTION_${'b'.repeat(96)}`,
    'CANARY_RECEIVER_SYNTHETIC_DO_NOT_USE',
  ];
  const internals = {
    seed: canaries[0],
    spend_key: canaries[1],
    pin: canaries[2],
    passphrase: canaries[3],
    raw_transaction: canaries[4],
    receiver: canaries[5],
  };
  const normalized = sanitizeLog({
    account_id: 'account-test-1',
    intent_id: 'intent-test-1',
    request_id: goldenRequest.input.request_id,
    state: 'failed',
    error_code: 'UNAVAILABLE',
    ...internals,
  });
  assert.deepStrictEqual(Object.keys(normalized).sort(), [
    'account_id',
    'error_code',
    'intent_id',
    'request_id',
    'state',
  ]);

  const ctx = lifecycle({ internals, signFailCode: 'UNAVAILABLE' });
  advanceTo(ctx, 'signing');
  const failure = ctx.machine.completeSign();
  assertFailure(failure, 'UNAVAILABLE');
  const observable = JSON.stringify({ normalized, failure, snapshot: ctx.machine.snapshot() });
  for (const canary of canaries) {
    assert.ok(!observable.includes(canary), 'normalized observable leaked a synthetic canary');
  }
});

test('rates: request, review, capability, prepare, confirm, sign, and verify work with no quote component', () => {
  assert.ok(!Object.keys(goldenRequest.input).some((key) => /fiat|rate|quote|provider/i.test(key)));
  assert.ok(!Object.keys(goldenReview.input).some((key) => /fiat|rate|quote|provider/i.test(key)));
  assert.strictEqual(decodeSignedObject('payment_request_v1', bytes(goldenRequest.input)).digest, goldenRequest.digest);
  assert.strictEqual(decodeSignedObject('review_image_v1', bytes(goldenReview.input)).digest, goldenReview.digest);
  for (const asset of ['ZEC', 'XMR']) {
    const selectedAccount = account(asset, 'software');
    const request = requestFor(asset);
    assert.strictEqual(evaluateCapability(selectedAccount, request).can_spend, true);
    const ctx = lifecycle({ account: selectedAccount, request });
    advanceTo(ctx, 'verified');
    const snapshot = ctx.machine.snapshot();
    assert.strictEqual(snapshot.state, 'verified');
    assert.match(snapshot.intent_hash, /^[0-9a-f]{64}$/);
    if (asset === 'ZEC') assert.strictEqual(snapshot.intent_hash, goldenReview.digest);
    assert.ok(!Object.keys(snapshot).some((key) => /fiat|rate|quote|provider/i.test(key)));
    assert.strictEqual(ctx.adapter.calls.prepare, 1);
    assert.strictEqual(ctx.signer.calls.sign, 1);
    assert.strictEqual(ctx.adapter.calls.verify, 1);
    assert.strictEqual(ctx.adapter.calls.broadcast, 0);
  }
});

test('binding: prepared reviews are recomputed and bound to the selected request and account', () => {
  const positive = lifecycle();
  assert.strictEqual(positive.machine.begin().ok, true);
  const prepared = positive.machine.prepare();
  assert.strictEqual(prepared.ok, true);
  assert.deepStrictEqual(prepared.review, goldenReview.input);
  assert.strictEqual(positive.adapter.calls.prepare, 1);
  assert.strictEqual(positive.signer.calls.sign, 0);
  assert.strictEqual(positive.adapter.calls.broadcast, 0);
  assert.strictEqual(positive.machine.cancel().state, 'cancelled');

  const alternateRequestId = '1234567890abcdef1234567890abcdef';
  const schemaValidMismatches = [
    ['account_id', mutate(goldenReview.input, { account_id: 'account-test-2' })],
    ['request_id', mutate(goldenReview.input, { request_id: alternateRequestId })],
    ['payment_request_hash', mutate(goldenReview.input, { payment_request_hash: '0'.repeat(64) })],
    ['payer_peer_id', mutate(goldenReview.input, { payer_peer_id: '12D3KooWOtherPayer' })],
    ['payee_peer_id', mutate(goldenReview.input, { payee_peer_id: '12D3KooWOtherPayee' })],
    ['amount_atomic', mutate(goldenReview.input, { amount_atomic: '100000001' })],
    ['receiver', mutate(goldenReview.input, { receiver: 'u1syntheticalternatereceiver' })],
    ['expires_at', mutate(goldenReview.input, { expires_at: '2026-08-30T12:14:59Z' })],
    ['memo_hash', mutate(goldenReview.input, { memo_hash: '0'.repeat(64) })],
  ];
  const xmrRequest = requestFor('XMR');
  schemaValidMismatches.push(['cross_asset_review', reviewFor('XMR', xmrRequest)]);

  for (const [field, review] of schemaValidMismatches) {
    assert.deepStrictEqual(
      decodeSignedObject('review_image_v1', bytes(review)).value,
      review,
      `${field} mismatch must remain a standalone schema-valid review`
    );
    const adapter = createFakeAdapter('ZEC', { review });
    const ctx = lifecycle({ adapter });
    assert.strictEqual(ctx.machine.begin().ok, true);
    assertFailure(ctx.machine.prepare(), 'INTENT_MISMATCH');
    assert.strictEqual(adapter.calls.prepare, 1);
    assert.strictEqual(ctx.signer.calls.sign, 0);
    assert.strictEqual(adapter.calls.broadcast, 0);
  }
});

test('capabilities: account, signer, adapter, and exact synthetic protocol pins cannot be substituted', () => {
  const unknownKind = account('ZEC', 'synthetic_unknown_kind');
  const unknownResult = evaluateCapability(unknownKind, requestFor('ZEC'));
  assert.strictEqual(unknownResult.can_spend, false);
  assert.strictEqual(unknownResult.error_code, 'CAPABILITY_MISSING');

  const hardware = account('ZEC', 'hardware_backed');
  const softwareSigner = createFakeSigner('software');
  const hardwareAdapter = createFakeAdapter('ZEC', { review: clone(goldenReview.input) });
  const signerMismatch = lifecycle({
    account: hardware,
    adapter: hardwareAdapter,
    signer: softwareSigner,
  });
  assert.strictEqual(signerMismatch.machine.begin().ok, true);
  assertFailure(signerMismatch.machine.prepare(), 'CAPABILITY_MISSING');
  assert.strictEqual(hardwareAdapter.calls.prepare, 0);
  assert.strictEqual(softwareSigner.calls.sign, 0);
  assert.strictEqual(hardwareAdapter.calls.broadcast, 0);

  const wrongAssetAdapter = createFakeAdapter('XMR', { review: clone(goldenReview.input) });
  const adapterMismatch = lifecycle({ adapter: wrongAssetAdapter });
  assert.strictEqual(adapterMismatch.machine.begin().ok, true);
  assertFailure(adapterMismatch.machine.prepare(), 'CAPABILITY_MISSING');
  assert.strictEqual(wrongAssetAdapter.calls.prepare, 0);
  assert.strictEqual(adapterMismatch.signer.calls.sign, 0);
  assert.strictEqual(wrongAssetAdapter.calls.broadcast, 0);

  const noPcztVerification = account('ZEC', 'hardware_backed', {
    capabilities: zecCapabilities({ can_verify_pczt_on_device: false }),
  });
  const noPcztResult = evaluateCapability(noPcztVerification, requestFor('ZEC'));
  assert.strictEqual(noPcztResult.can_spend, false);
  assert.strictEqual(noPcztResult.error_code, 'CAPABILITY_MISSING');

  for (const [capabilities, errorCode] of [
    [zecCapabilities({ consensus_branch: 'nu6.3-test-fixture-lookalike' }), 'PROTOCOL_INCOMPATIBLE'],
    [zecCapabilities({ pczt_version: 'v6-fixture-unsupported' }), 'PROTOCOL_INCOMPATIBLE'],
  ]) {
    const selectedAccount = account('ZEC', 'hardware_backed', { capabilities });
    const result = evaluateCapability(selectedAccount, requestFor('ZEC'));
    assert.strictEqual(result.can_spend, false);
    assert.strictEqual(result.error_code, errorCode);
  }
  assert.strictEqual(evaluateCapability(hardware, requestFor('ZEC')).can_spend, true);
});

test('recovery locking: crash_recovery retains ownership and restored confirmation acquires it', () => {
  for (const state of ['signed_unverified', 'verified']) {
    const sharedAdapter = createFakeAdapter('ZEC', { review: clone(goldenReview.input) });
    const owner = lifecycle({ adapter: sharedAdapter });
    advanceTo(owner, state);
    assert.strictEqual(owner.machine.crash().state, 'crash_recovery');

    const blocked = lifecycle({ adapter: sharedAdapter });
    assert.strictEqual(blocked.machine.begin().ok, true);
    assertFailure(blocked.machine.prepare(), 'ACCOUNT_BUSY');
    assert.strictEqual(sharedAdapter.calls.prepare, 1);
    assert.strictEqual(blocked.signer.calls.sign, 0);
    assert.strictEqual(sharedAdapter.calls.broadcast, 0);

    assert.strictEqual(owner.machine.cancel().state, 'cancelled');
    const afterRelease = lifecycle({ adapter: sharedAdapter });
    assert.strictEqual(afterRelease.machine.begin().ok, true);
    assert.strictEqual(afterRelease.machine.prepare().ok, true);
    assert.strictEqual(sharedAdapter.calls.prepare, 2);
    assert.strictEqual(afterRelease.machine.cancel().state, 'cancelled');
  }

  const sharedAdapter = createFakeAdapter('ZEC', { review: clone(goldenReview.input) });
  const holder = lifecycle({ adapter: sharedAdapter });
  assert.strictEqual(holder.machine.begin().ok, true);
  assert.strictEqual(holder.machine.prepare().ok, true);
  const recovered = lifecycle({
    adapter: sharedAdapter,
    restoredState: {
      v: 1,
      state: 'crash_recovery',
      review: clone(goldenReview.input),
      intent_hash: goldenReview.digest,
      confirmation_count: 1,
      signed_artifact: {
        kind: 'synthetic-signed-artifact',
        review: clone(goldenReview.input),
      },
    },
  });
  assertFailure(recovered.machine.confirmRecovery(), 'ACCOUNT_BUSY', 'crash_recovery');
  assert.strictEqual(sharedAdapter.calls.verify, 0);
  assert.strictEqual(sharedAdapter.calls.broadcast, 0);
  assert.strictEqual(recovered.signer.calls.sign, 0);
  assert.strictEqual(holder.machine.cancel().state, 'cancelled');
  assert.strictEqual(recovered.machine.confirmRecovery().state, 'signed_unverified');
  assert.strictEqual(recovered.machine.cancel().state, 'cancelled');
});

test('recovery terminal: cancellation and expiry release crash_recovery account locks', () => {
  for (const terminalMethod of ['cancel', 'expire']) {
    const sharedAdapter = createFakeAdapter('ZEC', { review: clone(goldenReview.input) });
    const owner = lifecycle({ adapter: sharedAdapter });
    advanceTo(owner, 'signed_unverified');
    assert.strictEqual(owner.machine.crash().state, 'crash_recovery');
    const terminal = owner.machine[terminalMethod]();
    assert.strictEqual(terminal.ok, true);
    assert.strictEqual(terminal.state, terminalMethod === 'cancel' ? 'cancelled' : 'expired');
    assert.strictEqual(sharedAdapter.calls.broadcast, 0);

    const next = lifecycle({ adapter: sharedAdapter });
    assert.strictEqual(next.machine.begin().ok, true);
    assert.strictEqual(next.machine.prepare().ok, true);
    assert.strictEqual(sharedAdapter.calls.prepare, 2);
    assert.strictEqual(next.machine.cancel().state, 'cancelled');
  }
});

test('recovery restart: repeated recovery crash is inert and durable verified restore cannot broadcast', () => {
  const recovering = lifecycle();
  advanceTo(recovering, 'signed_unverified');
  assert.strictEqual(recovering.machine.crash().state, 'crash_recovery');
  const before = recovering.machine.snapshot();
  const signCalls = recovering.signer.calls.sign;
  const verifyCalls = recovering.adapter.calls.verify;
  const restarted = recovering.machine.crash();
  assert.strictEqual(restarted.ok, true);
  assert.strictEqual(restarted.state, 'crash_recovery');
  assert.deepStrictEqual(recovering.machine.snapshot(), before);
  assert.strictEqual(recovering.machine.snapshot().confirmation_count, before.confirmation_count);
  assert.strictEqual(recovering.signer.calls.sign, signCalls);
  assert.strictEqual(recovering.adapter.calls.verify, verifyCalls);
  assert.strictEqual(recovering.adapter.calls.broadcast, 0);
  assert.strictEqual(recovering.machine.cancel().state, 'cancelled');

  const durable = lifecycle({
    restoredState: {
      v: 1,
      state: 'verified',
      review: clone(goldenReview.input),
      intent_hash: goldenReview.digest,
      confirmation_count: 1,
      signed_artifact: {
        kind: 'synthetic-signed-artifact',
        review: clone(goldenReview.input),
      },
    },
  });
  const premature = durable.machine.broadcast();
  assert.strictEqual(premature.ok, false);
  assert.strictEqual(premature.state, 'verified');
  assert.strictEqual(durable.adapter.calls.broadcast, 0);
  assert.strictEqual(durable.machine.crash().state, 'crash_recovery');
  assert.strictEqual(durable.machine.confirmRecovery().state, 'signed_unverified');
  assert.strictEqual(durable.machine.verifySigned().state, 'verified');
  assert.strictEqual(durable.machine.snapshot().confirmation_count, 2);
  assert.strictEqual(durable.adapter.calls.broadcast, 0);
  assert.strictEqual(durable.machine.cancel().state, 'cancelled');
});

test('exceptions: injected status, prepare, signer, verify, and broadcast throws return closed failures', () => {
  function returnedFailure(call, expectedState = 'failed') {
    let value;
    assert.doesNotThrow(() => {
      value = call();
    });
    assertFailure(value, 'UNAVAILABLE', expectedState);
    return value;
  }

  {
    let throwStatus = false;
    const sharedAdapter = createFakeAdapter('ZEC', { review: clone(goldenReview.input) });
    const ctx = lifecycle({
      adapter: sharedAdapter,
      requestStatusSource: () => {
        if (throwStatus) throw new Error('synthetic status failure');
        return 'open';
      },
    });
    advanceTo(ctx, 'signed_unverified');
    throwStatus = true;
    returnedFailure(() => ctx.machine.verifySigned());
    assert.strictEqual(sharedAdapter.calls.verify, 0);
    assert.strictEqual(sharedAdapter.calls.broadcast, 0);
    const next = lifecycle({ adapter: sharedAdapter });
    assert.strictEqual(next.machine.begin().ok, true);
    assert.strictEqual(next.machine.prepare().ok, true);
    assert.strictEqual(next.machine.cancel().state, 'cancelled');
  }

  {
    const sharedAdapter = createFakeAdapter('ZEC', { review: clone(goldenReview.input) });
    const originalPrepare = sharedAdapter.prepare;
    sharedAdapter.prepare = () => {
      sharedAdapter.calls.prepare += 1;
      throw new Error('synthetic prepare failure');
    };
    const ctx = lifecycle({ adapter: sharedAdapter });
    assert.strictEqual(ctx.machine.begin().ok, true);
    returnedFailure(() => ctx.machine.prepare());
    assert.strictEqual(sharedAdapter.calls.prepare, 1);
    assert.strictEqual(ctx.signer.calls.sign, 0);
    assert.strictEqual(sharedAdapter.calls.broadcast, 0);
    sharedAdapter.prepare = originalPrepare;
    const next = lifecycle({ adapter: sharedAdapter });
    assert.strictEqual(next.machine.begin().ok, true);
    assert.strictEqual(next.machine.prepare().ok, true);
    assert.strictEqual(next.machine.cancel().state, 'cancelled');
  }

  {
    const sharedAdapter = createFakeAdapter('ZEC', { review: clone(goldenReview.input) });
    const throwingSigner = createFakeSigner('software');
    throwingSigner.sign = () => {
      throwingSigner.calls.sign += 1;
      throw new Error('synthetic signer failure');
    };
    const ctx = lifecycle({ adapter: sharedAdapter, signer: throwingSigner });
    advanceTo(ctx, 'signing');
    returnedFailure(() => ctx.machine.completeSign());
    assert.strictEqual(throwingSigner.calls.sign, 1);
    assert.strictEqual(sharedAdapter.calls.broadcast, 0);
    const next = lifecycle({ adapter: sharedAdapter });
    assert.strictEqual(next.machine.begin().ok, true);
    assert.strictEqual(next.machine.prepare().ok, true);
    assert.strictEqual(next.machine.cancel().state, 'cancelled');
  }

  {
    const sharedAdapter = createFakeAdapter('ZEC', { review: clone(goldenReview.input) });
    const originalVerify = sharedAdapter.verify;
    sharedAdapter.verify = () => {
      sharedAdapter.calls.verify += 1;
      throw new Error('synthetic verify failure');
    };
    const ctx = lifecycle({ adapter: sharedAdapter });
    advanceTo(ctx, 'signed_unverified');
    returnedFailure(() => ctx.machine.verifySigned());
    assert.strictEqual(sharedAdapter.calls.verify, 1);
    assert.strictEqual(sharedAdapter.calls.broadcast, 0);
    sharedAdapter.verify = originalVerify;
    const next = lifecycle({ adapter: sharedAdapter });
    assert.strictEqual(next.machine.begin().ok, true);
    assert.strictEqual(next.machine.prepare().ok, true);
    assert.strictEqual(next.machine.cancel().state, 'cancelled');
  }

  {
    const adapter = createFakeAdapter('ZEC', { review: clone(goldenReview.input) });
    adapter.broadcast = () => {
      adapter.calls.broadcast += 1;
      throw new Error('synthetic broadcast failure');
    };
    const ctx = lifecycle({ adapter });
    advanceTo(ctx, 'verified');
    returnedFailure(() => ctx.machine.broadcast());
    assert.strictEqual(adapter.calls.broadcast, 1);
    assert.strictEqual(ctx.machine.broadcast().ok, false);
    assert.strictEqual(adapter.calls.broadcast, 1);
    const next = lifecycle({ adapter });
    assert.strictEqual(next.machine.begin().ok, true);
    assert.strictEqual(next.machine.prepare().ok, true);
    assert.strictEqual(next.machine.cancel().state, 'cancelled');
  }
});

test('secrets: sanitization validates allowlisted values without invoking accessors', () => {
  const valid = sanitizeLog({
    account_id: 'account-test-1',
    intent_id: 'intent-test-1',
    request_id: goldenRequest.input.request_id,
    state: 'failed',
    error_code: 'UNAVAILABLE',
  });
  assert.deepStrictEqual(valid, {
    account_id: 'account-test-1',
    intent_id: 'intent-test-1',
    request_id: goldenRequest.input.request_id,
    state: 'failed',
    error_code: 'UNAVAILABLE',
  });
  assert.strictEqual(Object.getPrototypeOf(valid), Object.prototype);
  for (const descriptor of Object.values(Object.getOwnPropertyDescriptors(valid))) {
    assert.ok(Object.prototype.hasOwnProperty.call(descriptor, 'value'));
    assert.strictEqual(typeof descriptor.get, 'undefined');
  }

  let getterCalls = 0;
  const accessorInput = {};
  Object.defineProperty(accessorInput, 'account_id', {
    enumerable: true,
    get() {
      getterCalls += 1;
      return 'account-test-1';
    },
  });
  const accessorResult = sanitizeLog(accessorInput);
  assert.strictEqual(getterCalls, 0);
  assert.deepStrictEqual(accessorResult, {});

  const inherited = { account_id: 'account-test-1' };
  const invalid = Object.create(inherited);
  invalid.intent_id = ['intent-test-1'];
  invalid.request_id = () => goldenRequest.input.request_id;
  invalid.state = 'failed\u0000hidden';
  invalid.error_code = 'CANARY_PASSPHRASE_DO_NOT_LOG';
  invalid.account_id = { value: 'account-test-1' };
  const sanitized = sanitizeLog(invalid);
  assert.strictEqual(Object.getPrototypeOf(sanitized), Object.prototype);
  assert.deepStrictEqual(sanitized, {});
  assert.ok(!JSON.stringify(sanitized).includes('CANARY'));
});

test('recovery authority: durable signed_unverified restore requires crash recovery and fresh confirmation', () => {
  const restored = lifecycle({
    restoredState: {
      v: 1,
      state: 'signed_unverified',
      review: clone(goldenReview.input),
      intent_hash: goldenReview.digest,
      confirmation_count: 1,
      signed_artifact: {
        kind: 'synthetic-signed-artifact',
        review: clone(goldenReview.input),
      },
    },
  });
  const direct = restored.machine.verifySigned();
  assert.strictEqual(direct.ok, false);
  assert.strictEqual(direct.state, 'signed_unverified');
  assert.strictEqual(restored.adapter.calls.verify, 0);
  assert.strictEqual(restored.machine.broadcast().ok, false);
  assert.strictEqual(restored.adapter.calls.broadcast, 0);

  assert.strictEqual(restored.machine.crash().state, 'crash_recovery');
  assert.strictEqual(restored.machine.confirmRecovery().state, 'signed_unverified');
  assert.strictEqual(restored.machine.snapshot().confirmation_count, 2);
  assert.strictEqual(restored.machine.verifySigned().state, 'verified');
  assert.strictEqual(restored.adapter.calls.verify, 1);
  assert.strictEqual(restored.adapter.calls.broadcast, 0);
  assert.strictEqual(restored.machine.cancel().state, 'cancelled');
});

test('capabilities: watch-only receive requires exact synthetic consensus compatibility', () => {
  for (const [asset, exactCapabilities, lookalikeBranch] of [
    ['ZEC', zecCapabilities({ can_sign_spend: false, can_sign_ironwood: false }), 'nu6.3-test-fixture-lookalike'],
    ['XMR', xmrCapabilities({ can_sign_spend: false }), 'xmr-fixture-hf-lookalike'],
  ]) {
    const request = requestFor(asset);
    const exact = evaluateCapability(
      account(asset, 'watch_only', { capabilities: exactCapabilities }),
      request
    );
    assert.strictEqual(exact.can_receive, true);
    assert.strictEqual(exact.can_spend, false);
    assert.strictEqual(exact.error_code, 'WATCH_ONLY');

    const incompatible = evaluateCapability(
      account(asset, 'watch_only', {
        capabilities: Object.assign({}, exactCapabilities, { consensus_branch: lookalikeBranch }),
      }),
      request
    );
    assert.strictEqual(incompatible.can_receive, false);
    assert.strictEqual(incompatible.can_spend, false);
    assert.strictEqual(incompatible.error_code, 'PROTOCOL_INCOMPATIBLE');
  }
});

test('exceptions: untrusted dependency error codes normalize without leaking or retaining locks', () => {
  const canary = 'CANARY_UNTRUSTED_ERROR_CODE_DO_NOT_EXPOSE';

  {
    const sharedAdapter = createFakeAdapter('ZEC', { review: clone(goldenReview.input) });
    const originalPrepare = sharedAdapter.prepare;
    sharedAdapter.prepare = () => {
      sharedAdapter.calls.prepare += 1;
      return { ok: false, error_code: canary };
    };
    const ctx = lifecycle({ adapter: sharedAdapter });
    assert.strictEqual(ctx.machine.begin().ok, true);
    const failed = ctx.machine.prepare();
    assert.ok(failed && typeof failed === 'object');
    assert.strictEqual(failed.ok, false);
    assert.ok(failed.error_code === 'UNAVAILABLE', 'untrusted prepare error code was not normalized');
    assert.strictEqual(failed.state, 'failed');
    const snapshot = ctx.machine.snapshot();
    assert.ok(snapshot.error_code === 'UNAVAILABLE', 'prepare snapshot error code was not normalized');
    assert.ok(!JSON.stringify({ failed, snapshot }).includes(canary));
    assert.strictEqual(sharedAdapter.calls.broadcast, 0);

    sharedAdapter.prepare = originalPrepare;
    const next = lifecycle({ adapter: sharedAdapter });
    assert.strictEqual(next.machine.begin().ok, true);
    assert.strictEqual(next.machine.prepare().ok, true);
    assert.strictEqual(next.machine.cancel().state, 'cancelled');
  }

  {
    const sharedAdapter = createFakeAdapter('ZEC', { review: clone(goldenReview.input) });
    const ctx = lifecycle({ adapter: sharedAdapter, signFailCode: canary });
    advanceTo(ctx, 'signing');
    const failed = ctx.machine.completeSign();
    assert.ok(failed && typeof failed === 'object');
    assert.strictEqual(failed.ok, false);
    assert.ok(failed.error_code === 'UNAVAILABLE', 'untrusted signer error code was not normalized');
    assert.strictEqual(failed.state, 'failed');
    const snapshot = ctx.machine.snapshot();
    assert.ok(snapshot.error_code === 'UNAVAILABLE', 'signer snapshot error code was not normalized');
    assert.ok(!JSON.stringify({ failed, snapshot }).includes(canary));
    assert.strictEqual(sharedAdapter.calls.broadcast, 0);

    const next = lifecycle({ adapter: sharedAdapter });
    assert.strictEqual(next.machine.begin().ok, true);
    assert.strictEqual(next.machine.prepare().ok, true);
    assert.strictEqual(next.machine.cancel().state, 'cancelled');
  }
});

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
  if (failed) {
    process.stderr.write(`${failed} wallet contract test(s) failed\n`);
    process.exit(1);
  }
  process.stdout.write(`BitBook wallet contract tests passed (${tests.length}).\n`);
}

if (require.main === module) {
  run();
}

module.exports = {
  tests,
};
