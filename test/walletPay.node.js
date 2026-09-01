'use strict';

const assert = require('assert');
const fixture = require('./fixtures/wallet-pay/snapshots-v1.json');
const {
  sanitizeWalletSnapshot,
  derivePayView,
  buildPayeeReceiverParams,
} = require('../wallet-pay/model');

const tests = [];
function test(name, fn) { tests.push({ name, fn }); }

const IDS = Object.freeze({
  zecSoftware: '00112233445566778899aabbccddeeff',
  zecHardware: '11112222333344445555666677778888',
  zecWatch: '22223333444455556666777788889999',
  xmrSoftware: '3333444455556666777788889999aaaa',
  xmrHardware: '444455556666777788889999aaaabbbb',
  xmrWatch: '55556666777788889999aaaabbbbcccc',
  request: 'ffeeddccbbaa99887766554433221100',
  intent: 'abcdefabcdefabcdefabcdefabcdefab',
});

const CAPABILITY_KEYS = Object.freeze([
  'can_view',
  'can_derive_fresh_receiver',
  'can_receive_private',
  'can_receive_orchard',
  'can_receive_ironwood',
  'can_prepare_tx',
  'can_sign_spend',
  'can_sign_orchard',
  'can_sign_ironwood',
  'can_tx_v6',
  'can_migrate_orchard_to_ironwood',
  'can_sign_transparent',
  'can_display_amount_on_device',
  'can_display_recipient_on_device',
  'can_display_network_on_device',
  'can_verify_pczt_on_device',
  'can_export_viewing_material',
  'can_broadcast',
  'consensus_branch',
  'pczt_version',
  'tx_version_max',
]);

const ACCOUNT_KEYS = Object.freeze([
  'account_id', 'label', 'asset', 'network', 'kind', 'privacy',
  'balance_atomic', 'restored_pool', 'probed_at', 'probe_source',
  'capabilities', 'sync', 'device',
]);
const VIEW_KEYS = Object.freeze(['accounts', 'can_begin', 'can_request', 'visible']);
const VIEW_ROW_KEYS = Object.freeze([
  'account_id', 'can_begin', 'can_request', 'reason_code', 'status_label', 'visible',
]);

const LABELS = Object.freeze({
  READY: 'Ready for private BitBook payment',
  REQUEST_READY: 'Ready to create a private BitBook payment request',
  SCHEMA: 'Wallet data is unavailable',
  UNAVAILABLE: 'Wallet is not running',
  LOCKED: 'Unlock in BitBook Wallet',
  WRONG_NETWORK: 'This account is on a different network',
  PRIVATE_MISSING: 'This account is not available for private BitBook payments',
  CAPABILITY_MISSING: 'This account cannot make this BitBook payment',
  DEVICE_DISCONNECTED: 'Connect this device in BitBook Wallet',
  MIGRATION_REQUIRED: 'Restored pre-Ironwood shielded funds cannot be sent until a later consented migration',
  WATCH_ONLY: 'Watch-only accounts cannot send payments',
  PROTOCOL_INCOMPATIBLE: 'This account cannot send on the current Zcash/Monero network',
  SYNCING: 'Wallet is syncing',
});

function clone(value) {
  return JSON.parse(JSON.stringify(value));
}

function zecCapabilities(overrides = {}) {
  return Object.assign({
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
  }, overrides);
}

function xmrCapabilities(overrides = {}) {
  return Object.assign({
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
  }, overrides);
}

function account(asset = 'ZEC', kind = 'software', overrides = {}) {
  const isZec = asset === 'ZEC';
  const ids = isZec
    ? { software: IDS.zecSoftware, hardware_backed: IDS.zecHardware, watch_only: IDS.zecWatch }
    : { software: IDS.xmrSoftware, hardware_backed: IDS.xmrHardware, watch_only: IDS.xmrWatch };
  return Object.assign({
    account_id: ids[kind],
    label: `${asset} ${kind}`,
    asset,
    network: isZec ? 'zec-testnet' : 'xmr-stagenet',
    kind,
    privacy: 'private',
    balance_atomic: '1000',
    restored_pool: isZec ? 'ironwood' : null,
    probed_at: '2026-08-30T12:00:00Z',
    probe_source: 'static_fixture',
    capabilities: isZec ? zecCapabilities() : xmrCapabilities(),
    sync: { state: 'idle', progress: 1 },
    device: {
      present: true,
      label: kind === 'hardware_backed' ? 'Synthetic device' : '',
      verified_fields: kind === 'hardware_backed' ? ['amount', 'recipient', 'network', 'fee'] : [],
    },
  }, overrides);
}

function snapshot(accounts, broker = 'ready', intentPreview = null) {
  return { v: 1, broker, accounts, intent_preview: intentPreview };
}

function context(asset = 'ZEC', role = 'payer', overrides = {}) {
  return Object.assign({
    role,
    asset,
    network: asset === 'ZEC' ? 'zec-testnet' : 'xmr-stagenet',
    request_valid: true,
  }, overrides);
}

function sanitize(value) {
  return sanitizeWalletSnapshot(value);
}

function viewOf(selectedAccount, selectedContext, broker = 'ready') {
  return derivePayView(sanitize(snapshot([selectedAccount], broker)), selectedContext);
}

function onlyRow(view) {
  assert.strictEqual(view.accounts.length, 1);
  return view.accounts[0];
}

function assertClosedKeys(value, expected) {
  assert.deepStrictEqual(Object.keys(value).sort(), [...expected].sort());
  assert.strictEqual(Object.getPrototypeOf(value), Object.prototype);
}

function assertCode(fn, code) {
  assert.throws(fn, (error) => error && error.code === code &&
    !/CANARY|u1-forbidden|FEE_CANARY|SEED_CANARY|STACK_CANARY/i.test(error.message));
}

test('module exports exactly the three closed deterministic Pay functions', () => {
  const exported = require('../wallet-pay/model');
  assert.deepStrictEqual(Object.keys(exported).sort(), [
    'buildPayeeReceiverParams', 'derivePayView', 'sanitizeWalletSnapshot',
  ]);
  for (const name of Object.keys(exported)) assert.strictEqual(typeof exported[name], 'function');
});

test('sanitizer: committed full fixture is reduced to the exact closed snapshot and preview', () => {
  const input = clone(fixture.valid_full_input);
  const original = JSON.stringify(input);
  const result = sanitize(input);
  assert.deepStrictEqual(result, fixture.valid_full_expected);
  assert.strictEqual(JSON.stringify(input), original);
  assertClosedKeys(result, ['v', 'broker', 'accounts', 'intent_preview']);
  assertClosedKeys(result.accounts[0], ACCOUNT_KEYS);
  assertClosedKeys(result.accounts[0].capabilities, CAPABILITY_KEYS);
  assert.ok(!JSON.stringify(result).includes('SNAPSHOT_SECRET_CANARY'));
  assert.ok(!JSON.stringify(result).includes('u1-forbidden'));
  assert.notStrictEqual(result, input);
  assert.notStrictEqual(result.accounts, input.accounts);
  assert.notStrictEqual(result.accounts[0], input.accounts[0]);
  assert.notStrictEqual(result.accounts[0].capabilities, input.accounts[0].capabilities);
  result.accounts[0].sync.state = 'mutated';
  result.intent_preview.state = 'failed';
  assert.strictEqual(input.accounts[0].sync.state, 'idle');
  assert.strictEqual(input.intent_preview.state, 'awaiting_confirm');
});

test('sanitizer: malformed roots, accessors, exotic prototypes, functions, symbols, and cycles invoke no user code', () => {
  for (const value of [undefined, null, true, 1, 'snapshot', [], () => true, Symbol('snapshot')]) {
    assert.deepStrictEqual(sanitize(value), { v: 1, broker: 'down', accounts: [], intent_preview: null });
  }
  let getterCalls = 0;
  const accessorRoot = {};
  Object.defineProperty(accessorRoot, 'accounts', {
    enumerable: true,
    get() { getterCalls += 1; throw new Error('getter reached'); },
  });
  assert.deepStrictEqual(sanitize(accessorRoot), { v: 1, broker: 'down', accounts: [], intent_preview: null });
  const inheritedRoot = Object.create({ broker: 'ready', accounts: [account()] });
  assert.deepStrictEqual(sanitize(inheritedRoot), { v: 1, broker: 'down', accounts: [], intent_preview: null });
  const cyclic = { v: 1, broker: 'ready', accounts: [] };
  cyclic.accounts.push(cyclic);
  cyclic.intent_preview = cyclic;
  assert.deepStrictEqual(sanitize(cyclic), { v: 1, broker: 'ready', accounts: [], intent_preview: null });
  const symbolAccount = account();
  symbolAccount[Symbol('seed')] = 'CANARY';
  assert.doesNotThrow(() => sanitize(snapshot([symbolAccount])));
  assert.strictEqual(getterCalls, 0);
});

test('sanitizer: invalid accounts drop, duplicates keep the first valid entry, and account count caps at 256', () => {
  const reduced = sanitize(snapshot([
    Object.assign(account(), { account_id: 'not-an-id' }),
    Object.assign(account(), { asset: 'BTC' }),
    Object.assign(account(), { network: 'xmr-stagenet' }),
    Object.assign(account(), { kind: 'custodial' }),
    account('ZEC', 'software', { label: 'first' }),
    account('ZEC', 'software', { label: 'duplicate' }),
  ]));
  assert.strictEqual(reduced.accounts.length, 1);
  assert.strictEqual(reduced.accounts[0].label, 'first');
  const many = [];
  for (let index = 0; index < 300; index += 1) {
    many.push(account('XMR', 'software', {
      account_id: index.toString(16).padStart(32, '0'),
      label: `account ${index}`,
    }));
  }
  const result = sanitize(snapshot(many));
  assert.strictEqual(result.accounts.length, 256);
  assert.strictEqual(new Set(result.accounts.map((item) => item.account_id)).size, 256);
});

test('sanitizer: labels, u64 decimals, nested enums, and numeric bounds fail closed to safe defaults', () => {
  const unsafe = account('XMR', 'hardware_backed', {
    label: `safe\nunsafe\u202e${'é'.repeat(70)}`,
    balance_atomic: '18446744073709551616',
    restored_pool: 'sapling',
    probed_at: 'x'.repeat(129),
    probe_source: 'remote_rpc',
    capabilities: Object.assign(xmrCapabilities(), {
      can_view: 'true',
      consensus_branch: 'x'.repeat(129),
      pczt_version: 6,
      tx_version_max: 6,
      unknown: true,
    }),
    sync: { state: 'attacker-controlled', progress: Infinity },
    device: {
      present: 'yes',
      label: 'bad\u0085label',
      verified_fields: ['amount', 'seed', 'amount', 'recipient', 7],
    },
  });
  const result = sanitize(snapshot([unsafe])).accounts[0];
  assert.deepStrictEqual(result, {
    account_id: IDS.xmrHardware,
    label: '',
    asset: 'XMR',
    network: 'xmr-stagenet',
    kind: 'hardware_backed',
    privacy: 'private',
    balance_atomic: '0',
    restored_pool: null,
    probed_at: null,
    probe_source: null,
    capabilities: Object.assign(xmrCapabilities(), {
      can_view: false,
      consensus_branch: null,
      tx_version_max: null,
    }),
    sync: { state: 'unknown', progress: 0 },
    device: { present: false, label: '', verified_fields: [] },
  });
  assert.deepStrictEqual(sanitize(snapshot([account('ZEC', 'software', { balance_atomic: '00' })])).accounts[0].balance_atomic, '0');
  assert.strictEqual(Buffer.byteLength(sanitize(snapshot([account('ZEC', 'software', { label: 'é'.repeat(64) })])).accounts[0].label), 128);
  assert.strictEqual(sanitize(snapshot([account('ZEC', 'software', { label: `é${'a'.repeat(127)}` })])).accounts[0].label, '');
  for (const forbidden of ['\u0000', '\u001f', '\u0085', '\u202e', '\u200d', '\r', '\n']) {
    const sanitized = sanitize(snapshot([account('ZEC', 'software', { label: `safe${forbidden}unsafe` })]));
    assert.strictEqual(sanitized.accounts[0].label, '');
  }
});

test('sanitizer: nested accessors and hostile prototypes become defaults without invocation', () => {
  let calls = 0;
  const source = account();
  const caps = {};
  Object.defineProperty(caps, 'can_view', { enumerable: true, get() { calls += 1; return true; } });
  source.capabilities = caps;
  source.sync = Object.create({ state: 'idle', progress: 1 });
  const device = {};
  Object.defineProperty(device, 'present', { enumerable: true, get() { calls += 1; return true; } });
  source.device = device;
  const result = sanitize(snapshot([source])).accounts[0];
  assert.strictEqual(calls, 0);
  assert.deepStrictEqual(result.capabilities, Object.assign(
    Object.fromEntries(CAPABILITY_KEYS.slice(0, 18).map((key) => [key, false])),
    { consensus_branch: null, pczt_version: null, tx_version_max: null }
  ));
  assert.deepStrictEqual(result.sync, { state: 'unknown', progress: 0 });
  assert.deepStrictEqual(result.device, { present: false, label: '', verified_fields: [] });
});

test('payer view: software, hardware, and watch-only outcomes are explicit for both assets', () => {
  const rows = [
    [account('ZEC', 'software'), true, null, LABELS.READY],
    [account('ZEC', 'hardware_backed'), true, null, LABELS.READY],
    [account('ZEC', 'watch_only', { capabilities: zecCapabilities({ can_sign_spend: false, can_sign_ironwood: false }) }), false, 'WATCH_ONLY', LABELS.WATCH_ONLY],
    [account('XMR', 'software'), true, null, LABELS.READY],
    [account('XMR', 'hardware_backed'), true, null, LABELS.READY],
    [account('XMR', 'watch_only', { capabilities: xmrCapabilities({ can_sign_spend: false }) }), false, 'WATCH_ONLY', LABELS.WATCH_ONLY],
  ];
  for (const [selected, canBegin, reason, label] of rows) {
    const view = viewOf(selected, context(selected.asset));
    assertClosedKeys(view, VIEW_KEYS);
    const row = onlyRow(view);
    assertClosedKeys(row, VIEW_ROW_KEYS);
    assert.strictEqual(view.visible, true);
    assert.strictEqual(view.can_begin, canBegin);
    assert.strictEqual(view.can_request, false);
    assert.deepStrictEqual(row, {
      account_id: selected.account_id,
      visible: true,
      can_begin: canBegin,
      can_request: false,
      reason_code: reason,
      status_label: label,
    });
  }
});

test('payer view: fixed blocker precedence covers broker, request, network, privacy, device, migration, and syncing', () => {
  const base = account('ZEC', 'hardware_backed');
  const rows = [
    ['down', base, context('ZEC'), 'UNAVAILABLE', LABELS.UNAVAILABLE, false],
    ['degraded', base, context('ZEC'), 'UNAVAILABLE', LABELS.UNAVAILABLE, false],
    ['locked', base, context('ZEC'), 'LOCKED', LABELS.LOCKED, false],
    ['ready', base, context('ZEC', 'payer', { request_valid: false }), 'SCHEMA', LABELS.SCHEMA, false],
    ['ready', base, context('XMR'), 'WRONG_NETWORK', LABELS.WRONG_NETWORK, false],
    ['ready', Object.assign({}, base, { privacy: 'transparent_not_private' }), context('ZEC'), 'CAPABILITY_MISSING', LABELS.PRIVATE_MISSING, false],
    ['ready', Object.assign({}, base, { device: Object.assign({}, base.device, { present: false }) }), context('ZEC'), 'DEVICE_DISCONNECTED', LABELS.DEVICE_DISCONNECTED, false],
    ['ready', Object.assign({}, base, { restored_pool: 'orchard' }), context('ZEC'), 'MIGRATION_REQUIRED', LABELS.MIGRATION_REQUIRED, false],
    ['syncing', base, context('ZEC'), 'SYNCING', LABELS.SYNCING, true],
  ];
  for (const [broker, selected, selectedContext, reason, label, visible] of rows) {
    const view = viewOf(selected, selectedContext, broker);
    const row = onlyRow(view);
    assert.strictEqual(view.visible, visible);
    assert.strictEqual(view.can_begin, false);
    assert.strictEqual(row.visible, visible);
    assert.strictEqual(row.can_begin, false);
    assert.strictEqual(row.reason_code, reason);
    assert.strictEqual(row.status_label, label);
  }
});

test('payer view: malformed contexts and compound blockers preserve the exact precedence order', () => {
  const selected = account('ZEC', 'hardware_backed');
  let getterCalls = 0;
  const accessorContext = {};
  Object.defineProperty(accessorContext, 'role', {
    enumerable: true,
    get() { getterCalls += 1; return 'payer'; },
  });
  for (const malformed of [
    null,
    [],
    accessorContext,
    Object.assign(context('ZEC'), { role: 'sender' }),
    Object.assign(context('ZEC'), { request_valid: 'true' }),
    Object.assign(context('ZEC'), { extra: true }),
  ]) {
    const view = derivePayView(sanitize(snapshot([selected])), malformed);
    assert.strictEqual(view.visible, false);
    assert.strictEqual(view.can_begin, false);
    assert.deepStrictEqual(onlyRow(view), {
      account_id: selected.account_id,
      visible: false,
      can_begin: false,
      can_request: false,
      reason_code: 'SCHEMA',
      status_label: LABELS.SCHEMA,
    });
  }
  assert.strictEqual(getterCalls, 0);

  const compound = account('ZEC', 'watch_only', {
    privacy: 'transparent_not_private',
    restored_pool: 'orchard',
    capabilities: zecCapabilities({
      consensus_branch: 'nu6.2-fixture',
      can_receive_private: false,
      can_prepare_tx: false,
      can_sign_spend: false,
      can_sign_ironwood: false,
    }),
  });
  const rows = [
    ['degraded', context('XMR', 'payer', { request_valid: false }), compound, 'UNAVAILABLE'],
    ['locked', context('XMR', 'payer', { request_valid: false }), compound, 'LOCKED'],
    ['ready', context('XMR', 'payer', { request_valid: false }), compound, 'SCHEMA'],
    ['ready', context('XMR'), compound, 'WRONG_NETWORK'],
    ['ready', context('ZEC'), compound, 'CAPABILITY_MISSING'],
    ['ready', context('ZEC'), Object.assign({}, compound, { privacy: 'private', kind: 'hardware_backed', device: { present: false, label: '', verified_fields: [] } }), 'DEVICE_DISCONNECTED'],
    ['ready', context('ZEC'), Object.assign({}, compound, { privacy: 'private' }), 'MIGRATION_REQUIRED'],
    ['syncing', context('ZEC'), account('ZEC', 'software', { capabilities: zecCapabilities({ can_prepare_tx: false }) }), 'CAPABILITY_MISSING'],
  ];
  for (const [broker, selectedContext, selectedAccount, reason] of rows) {
    assert.strictEqual(onlyRow(viewOf(selectedAccount, selectedContext, broker)).reason_code, reason);
  }
});

test('Pay view: malformed unsanitized snapshots return one closed inert SCHEMA view', () => {
  let getterCalls = 0;
  const accessor = {};
  Object.defineProperty(accessor, 'accounts', {
    enumerable: true,
    get() { getterCalls += 1; return [account()]; },
  });
  for (const malformed of [null, [], fixture.valid_full_input, accessor, Object.create({
    v: 1, broker: 'ready', accounts: [account()], intent_preview: null,
  })]) {
    const view = derivePayView(malformed, context('ZEC'));
    assertClosedKeys(view, VIEW_KEYS);
    assert.deepStrictEqual(view, {
      visible: false,
      can_begin: false,
      can_request: false,
      accounts: [],
    });
  }
  assert.strictEqual(getterCalls, 0);
});

test('payer view: transparent-only Zcash hardware and every current Zcash protocol omission fail closed', () => {
  const trezor = account('ZEC', 'hardware_backed', {
    label: 'Trezor transparent-only',
    privacy: 'transparent_not_private',
    capabilities: zecCapabilities({
      can_receive_private: false,
      can_receive_ironwood: false,
      can_sign_spend: false,
      can_sign_ironwood: false,
      can_sign_transparent: true,
    }),
  });
  const trezorRow = onlyRow(viewOf(trezor, context('ZEC')));
  assert.strictEqual(trezorRow.can_begin, false);
  assert.strictEqual(trezorRow.reason_code, 'CAPABILITY_MISSING');
  assert.strictEqual(trezorRow.status_label, LABELS.PRIVATE_MISSING);
  const omissions = [
    [zecCapabilities({ consensus_branch: 'nu6.2-fixture' }), 'PROTOCOL_INCOMPATIBLE'],
    [zecCapabilities({ can_tx_v6: false }), 'PROTOCOL_INCOMPATIBLE'],
    [zecCapabilities({ tx_version_max: '5' }), 'PROTOCOL_INCOMPATIBLE'],
    [zecCapabilities({ pczt_version: null }), 'PROTOCOL_INCOMPATIBLE'],
    [zecCapabilities({ can_prepare_tx: false }), 'CAPABILITY_MISSING'],
    [zecCapabilities({ can_sign_spend: false }), 'CAPABILITY_MISSING'],
    [zecCapabilities({ can_sign_ironwood: false }), 'CAPABILITY_MISSING'],
    [zecCapabilities({ can_verify_pczt_on_device: false }), 'CAPABILITY_MISSING'],
  ];
  for (const [capabilities, reason] of omissions) {
    const row = onlyRow(viewOf(account('ZEC', 'hardware_backed', { capabilities }), context('ZEC')));
    assert.strictEqual(row.can_begin, false);
    assert.strictEqual(row.reason_code, reason);
    assert.strictEqual(row.status_label, reason === 'PROTOCOL_INCOMPATIBLE'
      ? LABELS.PROTOCOL_INCOMPATIBLE : LABELS.CAPABILITY_MISSING);
  }
});

test('payer view: quote, fiat, provider, receiver, and self-reported eligibility fields never affect results', () => {
  const source = snapshot([Object.assign(account('XMR', 'software'), {
    can_begin: false,
    eligible: false,
    receiver: 'forbidden-subaddress',
    quote: { price: '0' },
  })]);
  source.fiat_estimate = { status: 'unavailable', price: '0' };
  source.provider = 'hostile';
  const absent = derivePayView(sanitize(snapshot([account('XMR', 'software')])), context('XMR'));
  const hostile = derivePayView(sanitize(source), context('XMR'));
  assert.deepStrictEqual(hostile, absent);
  assert.strictEqual(hostile.can_begin, true);
});

test('payee view: software, connected hardware, and watch-only can request for both assets', () => {
  for (const asset of ['ZEC', 'XMR']) {
    for (const kind of ['software', 'hardware_backed', 'watch_only']) {
      const capabilities = asset === 'ZEC'
        ? zecCapabilities(kind === 'watch_only' ? { can_sign_spend: false, can_sign_ironwood: false } : {})
        : xmrCapabilities(kind === 'watch_only' ? { can_sign_spend: false } : {});
      const selected = account(asset, kind, { capabilities });
      const view = viewOf(selected, context(asset, 'payee', { request_valid: false }));
      assert.strictEqual(view.visible, true);
      assert.strictEqual(view.can_begin, false);
      assert.strictEqual(view.can_request, true);
      assert.deepStrictEqual(onlyRow(view), {
        account_id: selected.account_id,
        visible: true,
        can_begin: false,
        can_request: true,
        reason_code: null,
        status_label: LABELS.REQUEST_READY,
      });
    }
  }
});

test('payee view: receive privacy, fresh derivation, Ironwood, device, broker, and network gates remain closed', () => {
  const base = account('ZEC', 'hardware_backed');
  const rows = [
    ['ready', Object.assign({}, base, { privacy: 'unknown' }), context('ZEC', 'payee'), 'CAPABILITY_MISSING', LABELS.PRIVATE_MISSING],
    ['ready', Object.assign({}, base, { capabilities: zecCapabilities({ can_receive_private: false }) }), context('ZEC', 'payee'), 'CAPABILITY_MISSING', LABELS.CAPABILITY_MISSING],
    ['ready', Object.assign({}, base, { capabilities: zecCapabilities({ can_derive_fresh_receiver: false }) }), context('ZEC', 'payee'), 'CAPABILITY_MISSING', LABELS.CAPABILITY_MISSING],
    ['ready', Object.assign({}, base, { capabilities: zecCapabilities({ can_receive_ironwood: false }) }), context('ZEC', 'payee'), 'CAPABILITY_MISSING', LABELS.CAPABILITY_MISSING],
    ['ready', Object.assign({}, base, { device: Object.assign({}, base.device, { present: false }) }), context('ZEC', 'payee'), 'DEVICE_DISCONNECTED', LABELS.DEVICE_DISCONNECTED],
    ['down', base, context('ZEC', 'payee'), 'UNAVAILABLE', LABELS.UNAVAILABLE],
    ['locked', base, context('ZEC', 'payee'), 'LOCKED', LABELS.LOCKED],
    ['ready', base, context('XMR', 'payee'), 'WRONG_NETWORK', LABELS.WRONG_NETWORK],
  ];
  for (const [broker, selected, selectedContext, reason, label] of rows) {
    const view = viewOf(selected, selectedContext, broker);
    assert.strictEqual(view.visible, false);
    assert.strictEqual(view.can_request, false);
    assert.strictEqual(onlyRow(view).reason_code, reason);
    assert.strictEqual(onlyRow(view).status_label, label);
  }
  const restored = viewOf(Object.assign({}, base, { restored_pool: 'orchard' }), context('ZEC', 'payee'));
  assert.strictEqual(restored.can_request, true, 'restored pool blocks spend, not fresh private receive');
});

test('payee parameters: selected eligible account returns only exact receiver.fresh parameters', () => {
  const source = sanitize(snapshot([account('ZEC', 'software'), account('XMR', 'watch_only', {
    capabilities: xmrCapabilities({ can_sign_spend: false }),
  })]));
  for (const selected of [
    { account_id: IDS.zecSoftware, asset: 'ZEC', network: 'zec-testnet' },
    { account_id: IDS.xmrWatch, asset: 'XMR', network: 'xmr-stagenet' },
  ]) {
    const result = buildPayeeReceiverParams(source, Object.assign({
      role: 'payee', request_valid: false, request_id: IDS.request,
    }, selected));
    assert.deepStrictEqual(result, Object.assign({}, selected, { request_id: IDS.request }));
    assertClosedKeys(result, ['account_id', 'asset', 'network', 'request_id']);
    assert.ok(!Object.prototype.hasOwnProperty.call(result, 'receiver'));
  }
});

test('payee parameters: malformed IDs, wrong role/network, missing account, and ineligible selection throw stable codes', () => {
  const eligible = account('ZEC', 'software');
  const fallback = account('ZEC', 'software', { account_id: '9999aaaabbbbccccddddeeeeffff0000' });
  const ineligible = account('ZEC', 'hardware_backed', {
    account_id: IDS.zecHardware,
    device: { present: false, label: 'CANARY device', verified_fields: [] },
  });
  const source = sanitize(snapshot([ineligible, fallback, eligible]));
  const base = Object.assign(context('ZEC', 'payee'), {
    account_id: IDS.zecHardware,
    request_id: IDS.request,
  });
  assertCode(() => buildPayeeReceiverParams(source, base), 'DEVICE_DISCONNECTED');
  assertCode(() => buildPayeeReceiverParams(source, Object.assign({}, base, { account_id: 'a'.repeat(31) })), 'SCHEMA');
  assertCode(() => buildPayeeReceiverParams(source, Object.assign({}, base, { request_id: 'G'.repeat(32) })), 'SCHEMA');
  assertCode(() => buildPayeeReceiverParams(source, Object.assign({}, base, { role: 'payer' })), 'SCHEMA');
  assertCode(() => buildPayeeReceiverParams(source, Object.assign({}, base, { network: 'zec-mainnet' })), 'WRONG_NETWORK');
  assertCode(() => buildPayeeReceiverParams(source, Object.assign({}, base, { account_id: 'f'.repeat(32) })), 'SCHEMA');
});

test('preview: every state receives the fixed code-owned label and Cancel-only outcome', () => {
  const rows = [
    ['preparing', 'Preparing in BitBook Wallet', true],
    ['prepared', 'Confirm in BitBook Wallet', true],
    ['awaiting_confirm', 'Confirm in BitBook Wallet', true],
    ['signing', 'Sending in BitBook Wallet', true],
    ['signed_unverified', 'Sending in BitBook Wallet', true],
    ['verified', 'Sending in BitBook Wallet', true],
    ['broadcasting', 'Sending in BitBook Wallet', true],
    ['crash_recovery', 'Wallet restarted. Confirm again in BitBook Wallet to send, or cancel.', true],
    ['cancelled', 'Payment cancelled', false],
    ['expired', 'Payment request expired', false],
    ['failed', 'Payment status unavailable', false],
    ['unknown_needs_scan', 'Payment status unavailable', false],
  ];
  for (const [state, statusLabel, canCancel] of rows) {
    const input = {
      intent_id: IDS.intent,
      state,
      asset: 'XMR',
      network: 'xmr-stagenet',
      amount_atomic: '1',
      status_label: 'ATTACKER LABEL',
      can_cancel: !canCancel,
      confirm: true,
      actions: ['confirm', 'cancel'],
    };
    const preview = sanitize(snapshot([], 'ready', input)).intent_preview;
    assert.deepStrictEqual(preview, {
      intent_id: IDS.intent,
      state,
      asset: 'XMR',
      network: 'xmr-stagenet',
      amount_atomic: '1',
      status_label: statusLabel,
      can_cancel: canCancel,
      error_code: null,
    });
    assert.ok(!JSON.stringify(preview).toLowerCase().includes('confirm":true'));
    assert.ok(!Object.prototype.hasOwnProperty.call(preview, 'actions'));
  }
});

test('preview: stable errors normalize and all authority, receiver, fee, request, rate, raw, and secret canaries disappear', () => {
  const forbidden = {
    receiver: 'RECEIVER_CANARY',
    receivers: ['RECEIVER_CANARY'],
    fee_atomic: 'FEE_CANARY',
    memo: 'MEMO_CANARY',
    peer_id: 'PEER_CANARY',
    request_id: 'REQUEST_CANARY',
    rate: { price: 'RATE_CANARY' },
    provider: 'PROVIDER_CANARY',
    raw_transaction: 'RAW_TX_CANARY',
    pczt: 'PCZT_CANARY',
    seed: 'SEED_CANARY',
    viewing_key: 'VIEW_KEY_CANARY',
    passphrase: 'PASSPHRASE_CANARY',
    pin: 'PIN_CANARY',
    confirm: true,
  };
  const base = Object.assign({
    intent_id: IDS.intent,
    state: 'failed',
    asset: 'ZEC',
    network: 'zec-testnet',
    amount_atomic: '2',
  }, forbidden);
  const known = sanitize(snapshot([], 'ready', Object.assign({}, base, {
    error: { code: 'DEVICE_DISCONNECTED', message: 'ERROR_CANARY', stack: 'STACK_CANARY', data: forbidden },
  }))).intent_preview;
  assert.strictEqual(known.error_code, 'DEVICE_DISCONNECTED');
  const stableCodes = [
    'SCHEMA', 'UNAUTH', 'UNAVAILABLE', 'LOCKED', 'SYNCING', 'NODE_UNAVAILABLE',
    'DEVICE_DISCONNECTED', 'CAPABILITY_MISSING', 'PROTOCOL_INCOMPATIBLE',
    'INTENT_MISMATCH', 'EXPIRED', 'CANCELLED', 'REPLAY', 'WRONG_NETWORK',
    'AMOUNT_INVALID', 'TRANSPARENT_DOWNGRADE', 'ACCOUNT_BUSY', 'WATCH_ONLY',
    'MIGRATION_REQUIRED', 'LIMIT', 'STATE_CORRUPT', 'TIMEOUT', 'INTERNAL',
  ];
  for (const code of stableCodes) {
    const preview = sanitize(snapshot([], 'ready', Object.assign({}, base, {
      error: { code, message: 'ERROR_CANARY' },
    }))).intent_preview;
    assert.strictEqual(preview.error_code, code);
    assert.ok(!JSON.stringify(preview).includes('ERROR_CANARY'));
  }
  const direct = sanitize(snapshot([], 'ready', Object.assign({}, base, {
    error_code: 'LOCKED',
  }))).intent_preview;
  assert.strictEqual(direct.error_code, 'LOCKED');
  const unknown = sanitize(snapshot([], 'ready', Object.assign({}, base, {
    error: { code: 'ATTACKER_CODE', retry_text: 'RETRY_CANARY' },
  }))).intent_preview;
  assert.strictEqual(unknown.error_code, 'INTERNAL');
  const rate = sanitize(snapshot([], 'ready', Object.assign({}, base, {
    error_code: 'RATE_UNAVAILABLE',
  }))).intent_preview;
  assert.strictEqual(rate.error_code, 'INTERNAL');
  const serialized = JSON.stringify([known, unknown]);
  for (const canary of Object.values(forbidden).filter((value) => typeof value === 'string')) {
    assert.ok(!serialized.includes(canary), `preview leaked ${canary}`);
  }
  for (const key of ['receiver', 'fee_atomic', 'memo', 'peer_id', 'request_id', 'rate', 'raw_transaction', 'pczt', 'confirm']) {
    assert.ok(!Object.prototype.hasOwnProperty.call(known, key));
  }
});

test('preview: malformed IDs, states, asset/network pairs, amounts, accessors, and prototypes become null', () => {
  const valid = {
    intent_id: IDS.intent,
    state: 'preparing',
    asset: 'ZEC',
    network: 'zec-testnet',
    amount_atomic: '1',
  };
  const malformed = [
    null,
    [],
    Object.create(valid),
    Object.assign({}, valid, { intent_id: 'a'.repeat(31) }),
    Object.assign({}, valid, { state: 'confirmed' }),
    Object.assign({}, valid, { asset: 'BTC' }),
    Object.assign({}, valid, { network: 'xmr-stagenet' }),
    Object.assign({}, valid, { amount_atomic: '0' }),
    Object.assign({}, valid, { amount_atomic: '01' }),
    Object.assign({}, valid, { amount_atomic: '18446744073709551616' }),
  ];
  let calls = 0;
  const accessor = Object.assign({}, valid);
  Object.defineProperty(accessor, 'amount_atomic', {
    enumerable: true,
    get() { calls += 1; return '1'; },
  });
  malformed.push(accessor);
  for (const candidate of malformed) {
    assert.strictEqual(sanitize(snapshot([], 'ready', candidate)).intent_preview, null);
  }
  assert.strictEqual(calls, 0);
});

test('derive and parameter results are fresh and retain neither sanitized snapshot nor context', () => {
  const source = sanitize(snapshot([account('ZEC', 'software')]));
  const selectedContext = context('ZEC');
  const view = derivePayView(source, selectedContext);
  view.accounts[0].status_label = 'mutated';
  assert.notStrictEqual(view.accounts, source.accounts);
  assert.strictEqual(source.accounts[0].label, 'ZEC software');
  assert.strictEqual(selectedContext.role, 'payer');
  assert.ok(!Object.values(view).includes(source));
  assert.ok(!Object.values(view).includes(selectedContext));
  const paramsContext = Object.assign(context('ZEC', 'payee'), {
    account_id: IDS.zecSoftware,
    request_id: IDS.request,
  });
  const params = buildPayeeReceiverParams(source, paramsContext);
  params.account_id = 'f'.repeat(32);
  assert.strictEqual(paramsContext.account_id, IDS.zecSoftware);
  assert.strictEqual(source.accounts[0].account_id, IDS.zecSoftware);
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
  if (failed) process.exit(1);
  process.stdout.write(`BitBook wallet Pay tests passed (${tests.length}).\n`);
}

if (require.main === module) run();
module.exports = { tests };
