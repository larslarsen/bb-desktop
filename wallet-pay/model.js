'use strict';

const ACCOUNT_LIMIT = 256;
const STRING_LIMIT = 128;
const U64_MAX = '18446744073709551615';
const ID = /^[0-9a-f]{32}$/;
const DECIMAL = /^(?:0|[1-9][0-9]*)$/;
const CONTROL_OR_FORMAT = /[\u0000-\u001f\u007f-\u009f\u00ad\u061c\u180e\u200b-\u200f\u2028-\u202e\u2060-\u206f\ufeff]/u;

const BROKERS = Object.freeze(['down', 'locked', 'ready', 'syncing', 'degraded']);
const KINDS = Object.freeze(['software', 'hardware_backed', 'watch_only']);
const PRIVACY = Object.freeze(['private', 'transparent_not_private', 'unknown']);
const RESTORED_POOLS = Object.freeze(['orchard', 'ironwood']);
const PROBE_SOURCES = Object.freeze(['static_fixture', 'library', 'device_app']);
const SYNC_STATES = Object.freeze(['unknown', 'idle', 'syncing', 'ready', 'error', 'degraded']);
const VERIFIED_FIELDS = Object.freeze(['amount', 'recipient', 'network', 'fee']);
const NETWORKS = Object.freeze({
  ZEC: Object.freeze(['zec-mainnet', 'zec-testnet', 'zec-regtest']),
  XMR: Object.freeze(['xmr-mainnet', 'xmr-stagenet', 'xmr-testnet']),
});

const CAPABILITY_FLAGS = Object.freeze([
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
]);
const CAPABILITY_STRINGS = Object.freeze([
  'consensus_branch',
  'pczt_version',
  'tx_version_max',
]);

const PREVIEW_STATES = Object.freeze({
  preparing: Object.freeze(['Preparing in BitBook Wallet', true]),
  prepared: Object.freeze(['Confirm in BitBook Wallet', true]),
  awaiting_confirm: Object.freeze(['Confirm in BitBook Wallet', true]),
  signing: Object.freeze(['Sending in BitBook Wallet', true]),
  signed_unverified: Object.freeze(['Sending in BitBook Wallet', true]),
  verified: Object.freeze(['Sending in BitBook Wallet', true]),
  broadcasting: Object.freeze(['Sending in BitBook Wallet', true]),
  crash_recovery: Object.freeze([
    'Wallet restarted. Confirm again in BitBook Wallet to send, or cancel.',
    true,
  ]),
  cancelled: Object.freeze(['Payment cancelled', false]),
  expired: Object.freeze(['Payment request expired', false]),
  failed: Object.freeze(['Payment status unavailable', false]),
  unknown_needs_scan: Object.freeze(['Payment status unavailable', false]),
});

const ERROR_CODES = Object.freeze(new Set([
  'SCHEMA', 'UNAUTH', 'UNAVAILABLE', 'LOCKED', 'SYNCING', 'NODE_UNAVAILABLE',
  'DEVICE_DISCONNECTED', 'CAPABILITY_MISSING', 'PROTOCOL_INCOMPATIBLE',
  'INTENT_MISMATCH', 'EXPIRED', 'CANCELLED', 'REPLAY', 'WRONG_NETWORK',
  'AMOUNT_INVALID', 'TRANSPARENT_DOWNGRADE', 'ACCOUNT_BUSY', 'WATCH_ONLY',
  'MIGRATION_REQUIRED', 'LIMIT', 'STATE_CORRUPT', 'TIMEOUT', 'INTERNAL',
]));

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

class PayModelError extends Error {
  constructor(code) {
    super(`wallet Pay unavailable: ${code}`);
    this.name = 'PayModelError';
    this.code = code;
  }
}

function plainDescriptors(value) {
  if (value === null || typeof value !== 'object' || Array.isArray(value) ||
      Object.getPrototypeOf(value) !== Object.prototype) return null;
  return Object.getOwnPropertyDescriptors(value);
}

function dataValue(descriptors, key) {
  const descriptor = descriptors && descriptors[key];
  return descriptor && Object.prototype.hasOwnProperty.call(descriptor, 'value')
    ? descriptor.value : undefined;
}

function hasOwnDescriptor(descriptors, key) {
  return Boolean(descriptors && Object.prototype.hasOwnProperty.call(descriptors, key));
}

function exactDescriptors(value, keys) {
  const descriptors = plainDescriptors(value);
  if (!descriptors || Reflect.ownKeys(descriptors).some((key) => typeof key !== 'string') ||
      Object.keys(descriptors).length !== keys.length ||
      keys.some((key) => !Object.prototype.hasOwnProperty.call(descriptors, key)) ||
      Object.values(descriptors).some(
        (descriptor) => !Object.prototype.hasOwnProperty.call(descriptor, 'value')
      )) return null;
  return descriptors;
}

function arrayValues(value) {
  if (!Array.isArray(value) || Object.getPrototypeOf(value) !== Array.prototype) return null;
  const descriptors = Object.getOwnPropertyDescriptors(value);
  const length = descriptors.length;
  if (!length || !Object.prototype.hasOwnProperty.call(length, 'value')) return null;
  const values = [];
  for (let index = 0; index < length.value; index += 1) {
    const descriptor = descriptors[String(index)];
    if (!descriptor) continue;
    if (!Object.prototype.hasOwnProperty.call(descriptor, 'value')) return null;
    values.push(descriptor.value);
  }
  return values;
}

function safeString(value) {
  return typeof value === 'string' &&
    Buffer.byteLength(value, 'utf8') <= STRING_LIMIT &&
    !CONTROL_OR_FORMAT.test(value)
    ? value : null;
}

function canonicalU64(value, positive) {
  if (typeof value !== 'string' || !DECIMAL.test(value) ||
      value.length > U64_MAX.length ||
      (value.length === U64_MAX.length && value > U64_MAX) ||
      (positive && value === '0')) return null;
  return value;
}

function matchingNetwork(asset, network) {
  return Object.prototype.hasOwnProperty.call(NETWORKS, asset) && NETWORKS[asset].includes(network);
}

function defaultCapabilities() {
  const result = {};
  for (const key of CAPABILITY_FLAGS) result[key] = false;
  for (const key of CAPABILITY_STRINGS) result[key] = null;
  return result;
}

function sanitizeCapabilities(value) {
  const result = defaultCapabilities();
  const descriptors = plainDescriptors(value);
  if (!descriptors) return result;
  for (const key of CAPABILITY_FLAGS) {
    const candidate = dataValue(descriptors, key);
    if (typeof candidate === 'boolean') result[key] = candidate;
  }
  for (const key of CAPABILITY_STRINGS) {
    const candidate = dataValue(descriptors, key);
    if (candidate === null) result[key] = null;
    else {
      const safe = safeString(candidate);
      result[key] = safe === null ? null : safe;
    }
  }
  return result;
}

function sanitizeSync(value) {
  const descriptors = plainDescriptors(value);
  if (!descriptors) return { state: 'unknown', progress: 0 };
  const state = dataValue(descriptors, 'state');
  const progress = dataValue(descriptors, 'progress');
  return {
    state: typeof state === 'string' && SYNC_STATES.includes(state) ? state : 'unknown',
    progress: typeof progress === 'number' && Number.isFinite(progress) &&
      progress >= 0 && progress <= 1 ? progress : 0,
  };
}

function sanitizeDevice(value) {
  const descriptors = plainDescriptors(value);
  if (!descriptors) return { present: false, label: '', verified_fields: [] };
  const present = dataValue(descriptors, 'present');
  const label = safeString(dataValue(descriptors, 'label'));
  const sourceFields = arrayValues(dataValue(descriptors, 'verified_fields'));
  const fields = sourceFields && sourceFields.every(
    (field) => typeof field === 'string' && VERIFIED_FIELDS.includes(field)
  ) ? sourceFields.slice() : [];
  return {
    present: typeof present === 'boolean' ? present : false,
    label: label === null ? '' : label,
    verified_fields: fields,
  };
}

function sanitizeAccount(value) {
  const descriptors = plainDescriptors(value);
  if (!descriptors) return null;
  const accountId = dataValue(descriptors, 'account_id');
  const asset = dataValue(descriptors, 'asset');
  const network = dataValue(descriptors, 'network');
  const kind = dataValue(descriptors, 'kind');
  if (typeof accountId !== 'string' || !ID.test(accountId) ||
      !matchingNetwork(asset, network) || !KINDS.includes(kind)) return null;

  const label = safeString(dataValue(descriptors, 'label'));
  const privacy = dataValue(descriptors, 'privacy');
  const balance = canonicalU64(dataValue(descriptors, 'balance_atomic'), false);
  const restored = dataValue(descriptors, 'restored_pool');
  const probedAtValue = dataValue(descriptors, 'probed_at');
  const probedAt = probedAtValue === null ? null : safeString(probedAtValue);
  const probeSource = dataValue(descriptors, 'probe_source');
  return {
    account_id: accountId,
    label: label === null ? '' : label,
    asset,
    network,
    kind,
    privacy: PRIVACY.includes(privacy) ? privacy : 'unknown',
    balance_atomic: balance === null ? '0' : balance,
    restored_pool: asset === 'ZEC' && RESTORED_POOLS.includes(restored) ? restored : null,
    probed_at: probedAt === null ? null : probedAt,
    probe_source: PROBE_SOURCES.includes(probeSource) ? probeSource : null,
    capabilities: sanitizeCapabilities(dataValue(descriptors, 'capabilities')),
    sync: sanitizeSync(dataValue(descriptors, 'sync')),
    device: sanitizeDevice(dataValue(descriptors, 'device')),
  };
}

function normalizeErrorCode(descriptors) {
  if (hasOwnDescriptor(descriptors, 'error_code')) {
    const direct = dataValue(descriptors, 'error_code');
    if (direct === null) return null;
    return typeof direct === 'string' && ERROR_CODES.has(direct) ? direct : 'INTERNAL';
  }
  if (!hasOwnDescriptor(descriptors, 'error')) return null;
  const errorDescriptors = plainDescriptors(dataValue(descriptors, 'error'));
  const code = dataValue(errorDescriptors, 'code');
  return typeof code === 'string' && ERROR_CODES.has(code) ? code : 'INTERNAL';
}

function sanitizePreview(value) {
  const descriptors = plainDescriptors(value);
  if (!descriptors) return null;
  const intentId = dataValue(descriptors, 'intent_id');
  const state = dataValue(descriptors, 'state');
  const asset = dataValue(descriptors, 'asset');
  const network = dataValue(descriptors, 'network');
  const amount = canonicalU64(dataValue(descriptors, 'amount_atomic'), true);
  if (typeof intentId !== 'string' || !ID.test(intentId) ||
      !Object.prototype.hasOwnProperty.call(PREVIEW_STATES, state) ||
      !matchingNetwork(asset, network) || amount === null) return null;
  const presentation = PREVIEW_STATES[state];
  return {
    intent_id: intentId,
    state,
    asset,
    network,
    amount_atomic: amount,
    status_label: presentation[0],
    can_cancel: presentation[1],
    error_code: normalizeErrorCode(descriptors),
  };
}

function sanitizeWalletSnapshot(value) {
  const descriptors = plainDescriptors(value);
  if (!descriptors) return { v: 1, broker: 'down', accounts: [], intent_preview: null };
  const brokerValue = dataValue(descriptors, 'broker');
  const broker = BROKERS.includes(brokerValue) ? brokerValue : 'down';
  const accounts = [];
  const seen = new Set();
  const sourceAccounts = arrayValues(dataValue(descriptors, 'accounts'));
  if (sourceAccounts) {
    for (const source of sourceAccounts) {
      if (accounts.length >= ACCOUNT_LIMIT) break;
      const candidate = sanitizeAccount(source);
      if (!candidate || seen.has(candidate.account_id)) continue;
      seen.add(candidate.account_id);
      accounts.push(candidate);
    }
  }
  return {
    v: 1,
    broker,
    accounts,
    intent_preview: sanitizePreview(dataValue(descriptors, 'intent_preview')),
  };
}

function capabilityShape(value) {
  const descriptors = exactDescriptors(value, [...CAPABILITY_FLAGS, ...CAPABILITY_STRINGS]);
  if (!descriptors) return false;
  return CAPABILITY_FLAGS.every((key) => typeof dataValue(descriptors, key) === 'boolean') &&
    CAPABILITY_STRINGS.every((key) => {
      const candidate = dataValue(descriptors, key);
      return candidate === null || safeString(candidate) !== null;
    });
}

function syncShape(value) {
  const descriptors = exactDescriptors(value, ['state', 'progress']);
  if (!descriptors) return false;
  const state = dataValue(descriptors, 'state');
  const progress = dataValue(descriptors, 'progress');
  return typeof state === 'string' && SYNC_STATES.includes(state) &&
    typeof progress === 'number' && Number.isFinite(progress) && progress >= 0 && progress <= 1;
}

function deviceShape(value) {
  const descriptors = exactDescriptors(value, ['present', 'label', 'verified_fields']);
  if (!descriptors || typeof dataValue(descriptors, 'present') !== 'boolean' ||
      safeString(dataValue(descriptors, 'label')) === null) return false;
  const fields = arrayValues(dataValue(descriptors, 'verified_fields'));
  return Boolean(fields) && fields.every(
    (field) => typeof field === 'string' && VERIFIED_FIELDS.includes(field)
  );
}

function accountShape(value) {
  const keys = [
    'account_id', 'label', 'asset', 'network', 'kind', 'privacy', 'balance_atomic',
    'restored_pool', 'probed_at', 'probe_source', 'capabilities', 'sync', 'device',
  ];
  const descriptors = exactDescriptors(value, keys);
  if (!descriptors) return false;
  const accountId = dataValue(descriptors, 'account_id');
  const label = dataValue(descriptors, 'label');
  const asset = dataValue(descriptors, 'asset');
  const network = dataValue(descriptors, 'network');
  const kind = dataValue(descriptors, 'kind');
  const privacy = dataValue(descriptors, 'privacy');
  const restored = dataValue(descriptors, 'restored_pool');
  const probedAt = dataValue(descriptors, 'probed_at');
  const probeSource = dataValue(descriptors, 'probe_source');
  return typeof accountId === 'string' && ID.test(accountId) && safeString(label) !== null &&
    matchingNetwork(asset, network) && KINDS.includes(kind) && PRIVACY.includes(privacy) &&
    canonicalU64(dataValue(descriptors, 'balance_atomic'), false) !== null &&
    (restored === null || (asset === 'ZEC' && RESTORED_POOLS.includes(restored))) &&
    (probedAt === null || safeString(probedAt) !== null) &&
    (probeSource === null || PROBE_SOURCES.includes(probeSource)) &&
    capabilityShape(dataValue(descriptors, 'capabilities')) &&
    syncShape(dataValue(descriptors, 'sync')) && deviceShape(dataValue(descriptors, 'device'));
}

function previewShape(value) {
  if (value === null) return true;
  const descriptors = exactDescriptors(value, [
    'intent_id', 'state', 'asset', 'network', 'amount_atomic',
    'status_label', 'can_cancel', 'error_code',
  ]);
  if (!descriptors) return false;
  const intentId = dataValue(descriptors, 'intent_id');
  const state = dataValue(descriptors, 'state');
  const asset = dataValue(descriptors, 'asset');
  const network = dataValue(descriptors, 'network');
  const presentation = PREVIEW_STATES[state];
  const errorCode = dataValue(descriptors, 'error_code');
  return typeof intentId === 'string' && ID.test(intentId) && presentation &&
    matchingNetwork(asset, network) &&
    canonicalU64(dataValue(descriptors, 'amount_atomic'), true) !== null &&
    dataValue(descriptors, 'status_label') === presentation[0] &&
    dataValue(descriptors, 'can_cancel') === presentation[1] &&
    (errorCode === null || ERROR_CODES.has(errorCode));
}

function sanitizedSnapshotShape(value) {
  const descriptors = exactDescriptors(value, ['v', 'broker', 'accounts', 'intent_preview']);
  if (!descriptors || dataValue(descriptors, 'v') !== 1 ||
      !BROKERS.includes(dataValue(descriptors, 'broker'))) return null;
  const accounts = arrayValues(dataValue(descriptors, 'accounts'));
  if (!accounts || accounts.length > ACCOUNT_LIMIT || !previewShape(dataValue(descriptors, 'intent_preview'))) {
    return null;
  }
  const seen = new Set();
  for (const account of accounts) {
    if (!accountShape(account)) return null;
    const accountId = dataValue(plainDescriptors(account), 'account_id');
    if (seen.has(accountId)) return null;
    seen.add(accountId);
  }
  return { broker: dataValue(descriptors, 'broker'), accounts };
}

function contextShape(value, withSelection) {
  const keys = withSelection
    ? ['role', 'asset', 'network', 'request_valid', 'account_id', 'request_id']
    : ['role', 'asset', 'network', 'request_valid'];
  const descriptors = exactDescriptors(value, keys);
  if (!descriptors) return null;
  const role = dataValue(descriptors, 'role');
  const asset = dataValue(descriptors, 'asset');
  const network = dataValue(descriptors, 'network');
  const requestValid = dataValue(descriptors, 'request_valid');
  if (!['payer', 'payee'].includes(role) || !matchingNetwork(asset, network) ||
      typeof requestValid !== 'boolean') return null;
  if (withSelection) {
    const accountId = dataValue(descriptors, 'account_id');
    const requestId = dataValue(descriptors, 'request_id');
    if (typeof accountId !== 'string' || !ID.test(accountId) ||
        typeof requestId !== 'string' || !ID.test(requestId)) return null;
  }
  return {
    role,
    asset,
    network,
    request_valid: requestValid,
    account_id: withSelection ? dataValue(descriptors, 'account_id') : undefined,
    request_id: withSelection ? dataValue(descriptors, 'request_id') : undefined,
  };
}

function viewRow(accountId, visible, canBegin, canRequest, reasonCode, statusLabel) {
  return {
    account_id: accountId,
    visible,
    can_begin: canBegin,
    can_request: canRequest,
    reason_code: reasonCode,
    status_label: statusLabel,
  };
}

function blockedRow(account, code, label, visible = false) {
  return viewRow(account.account_id, visible, false, false, code, label);
}

function protocolReason(account) {
  const capabilities = account.capabilities;
  if (account.asset === 'ZEC') {
    if (capabilities.consensus_branch !== 'nu6.3-test-fixture' ||
        capabilities.can_tx_v6 !== true || capabilities.tx_version_max !== '6' ||
        capabilities.pczt_version !== 'v6-fixture') return 'PROTOCOL_INCOMPATIBLE';
    if (capabilities.can_receive_private !== true ||
        capabilities.can_derive_fresh_receiver !== true ||
        capabilities.can_receive_ironwood !== true ||
        capabilities.can_prepare_tx !== true || capabilities.can_sign_spend !== true ||
        capabilities.can_sign_ironwood !== true ||
        (account.kind === 'hardware_backed' && capabilities.can_verify_pczt_on_device !== true)) {
      return 'CAPABILITY_MISSING';
    }
    return null;
  }
  if (capabilities.consensus_branch !== 'xmr-fixture-hf') return 'PROTOCOL_INCOMPATIBLE';
  if (capabilities.can_receive_private !== true ||
      capabilities.can_derive_fresh_receiver !== true ||
      capabilities.can_prepare_tx !== true || capabilities.can_sign_spend !== true) {
    return 'CAPABILITY_MISSING';
  }
  return null;
}

function requestReason(account) {
  const capabilities = account.capabilities;
  if (account.asset === 'ZEC' && capabilities.consensus_branch !== 'nu6.3-test-fixture') {
    return 'PROTOCOL_INCOMPATIBLE';
  }
  if (account.asset === 'XMR' && capabilities.consensus_branch !== 'xmr-fixture-hf') {
    return 'PROTOCOL_INCOMPATIBLE';
  }
  if (capabilities.can_receive_private !== true ||
      capabilities.can_derive_fresh_receiver !== true ||
      (account.asset === 'ZEC' && capabilities.can_receive_ironwood !== true)) {
    return 'CAPABILITY_MISSING';
  }
  return null;
}

function accountView(account, broker, context) {
  if (broker === 'down' || broker === 'degraded') {
    return blockedRow(account, 'UNAVAILABLE', LABELS.UNAVAILABLE);
  }
  if (broker === 'locked') return blockedRow(account, 'LOCKED', LABELS.LOCKED);
  if (context.role === 'payer' && !context.request_valid) {
    return blockedRow(account, 'SCHEMA', LABELS.SCHEMA);
  }
  if (account.asset !== context.asset || account.network !== context.network) {
    return blockedRow(account, 'WRONG_NETWORK', LABELS.WRONG_NETWORK);
  }
  if (account.privacy !== 'private') {
    return blockedRow(account, 'CAPABILITY_MISSING', LABELS.PRIVATE_MISSING);
  }
  if (account.kind === 'hardware_backed' && account.device.present !== true) {
    return blockedRow(account, 'DEVICE_DISCONNECTED', LABELS.DEVICE_DISCONNECTED);
  }

  if (context.role === 'payee') {
    if (account.kind === 'hardware_backed' && (!account.probed_at || !account.probe_source)) {
      return blockedRow(account, 'CAPABILITY_MISSING', LABELS.CAPABILITY_MISSING);
    }
    const reason = requestReason(account);
    if (reason) {
      return blockedRow(account, reason, reason === 'PROTOCOL_INCOMPATIBLE'
        ? LABELS.PROTOCOL_INCOMPATIBLE : LABELS.CAPABILITY_MISSING);
    }
    return viewRow(account.account_id, true, false, true, null, LABELS.REQUEST_READY);
  }

  if (account.asset === 'ZEC' && account.restored_pool === 'orchard') {
    return blockedRow(account, 'MIGRATION_REQUIRED', LABELS.MIGRATION_REQUIRED);
  }
  if (account.kind === 'watch_only') {
    return viewRow(account.account_id, true, false, false, 'WATCH_ONLY', LABELS.WATCH_ONLY);
  }
  if (account.kind === 'hardware_backed' && (!account.probed_at || !account.probe_source)) {
    return blockedRow(account, 'CAPABILITY_MISSING', LABELS.CAPABILITY_MISSING);
  }
  const reason = protocolReason(account);
  if (reason) {
    return blockedRow(account, reason, reason === 'PROTOCOL_INCOMPATIBLE'
      ? LABELS.PROTOCOL_INCOMPATIBLE : LABELS.CAPABILITY_MISSING);
  }
  if (broker === 'syncing') {
    return viewRow(account.account_id, true, false, false, 'SYNCING', LABELS.SYNCING);
  }
  return viewRow(account.account_id, true, true, false, null, LABELS.READY);
}

function derivePayView(snapshot, context) {
  const sanitized = sanitizedSnapshotShape(snapshot);
  if (!sanitized) {
    return { visible: false, can_begin: false, can_request: false, accounts: [] };
  }
  const checkedContext = contextShape(context, false);
  let accounts;
  if (!checkedContext) {
    accounts = sanitized.accounts.map(
      (account) => blockedRow(account, 'SCHEMA', LABELS.SCHEMA)
    );
  } else {
    accounts = sanitized.accounts.map(
      (account) => accountView(account, sanitized.broker, checkedContext)
    );
  }
  return {
    visible: accounts.some((account) => account.visible),
    can_begin: accounts.some((account) => account.can_begin),
    can_request: accounts.some((account) => account.can_request),
    accounts,
  };
}

function buildPayeeReceiverParams(snapshot, context) {
  const sanitized = sanitizedSnapshotShape(snapshot);
  const checkedContext = contextShape(context, true);
  if (!sanitized || !checkedContext || checkedContext.role !== 'payee') {
    throw new PayModelError('SCHEMA');
  }
  const selected = sanitized.accounts.find(
    (account) => account.account_id === checkedContext.account_id
  );
  if (!selected) throw new PayModelError('SCHEMA');
  const viewContext = {
    role: checkedContext.role,
    asset: checkedContext.asset,
    network: checkedContext.network,
    request_valid: checkedContext.request_valid,
  };
  const row = accountView(selected, sanitized.broker, viewContext);
  if (!row.can_request) throw new PayModelError(row.reason_code || 'SCHEMA');
  return {
    account_id: checkedContext.account_id,
    asset: checkedContext.asset,
    network: checkedContext.network,
    request_id: checkedContext.request_id,
  };
}

module.exports = {
  sanitizeWalletSnapshot,
  derivePayView,
  buildPayeeReceiverParams,
};
