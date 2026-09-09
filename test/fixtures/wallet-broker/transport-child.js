'use strict';

const crypto = require('crypto');

const SENTINEL = 'WALLET_TRANSPORT_DIAGNOSTIC_SENTINEL';
const BOOTSTRAP_ID = '00000000000000000000000000000001';
const DOMAIN = 'bitbook-wallet-session-v1\n';
const HEX32 = /^[0-9a-f]{32}$/;

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
const EARLY_EVENT_SNAPSHOT = {
  v: 1,
  broker: 'syncing',
  accounts: [],
  seed: SENTINEL,
};

function encodeFrame(value) {
  const body = Buffer.from(JSON.stringify(value), 'utf8');
  const header = Buffer.alloc(4);
  header.writeUInt32BE(body.length, 0);
  return Buffer.concat([header, body]);
}

function createDecoder() {
  let pending = Buffer.alloc(0);
  return {
    push(chunk) {
      pending = Buffer.concat([pending, Buffer.from(chunk)]);
      const values = [];
      while (pending.length >= 4) {
        const length = pending.readUInt32BE(0);
        if (length === 0 || pending.length < 4 + length) {
          if (length === 0) throw new Error('empty frame');
          break;
        }
        const body = pending.subarray(4, 4 + length);
        pending = pending.subarray(4 + length);
        values.push(JSON.parse(body.toString('utf8')));
      }
      return values;
    },
  };
}

function computeSession(parentPid, childPid, parentNonce, childNonce) {
  const preimage = `${DOMAIN}${parentPid}\n${childPid}\n${parentNonce}\n${childNonce}`;
  return crypto.createHash('sha256').update(Buffer.from(preimage, 'utf8')).digest('hex');
}

function writeSplit(frame) {
  const first = Math.min(2, frame.length);
  const second = Math.min(first + 3, frame.length);
  process.stdout.write(frame.subarray(0, first));
  if (second > first) process.stdout.write(frame.subarray(first, second));
  if (frame.length > second) process.stdout.write(frame.subarray(second));
}

function parseArgs(argv) {
  const mode = argv[2] || 'ready';
  const args = {};
  for (const entry of argv.slice(3)) {
    const index = entry.indexOf('=');
    if (index > 0) args[entry.slice(0, index)] = entry.slice(index + 1);
  }
  return { mode, args };
}

function lockedError() {
  return {
    code: 'LOCKED',
    message: `hidden ${SENTINEL}`,
    stack: SENTINEL,
    debug: SENTINEL,
    seed: SENTINEL,
    retryable: true,
  };
}

const { mode, args } = parseArgs(process.argv);
const childNonce = HEX32.test(args['child-nonce'] || '')
  ? args['child-nonce']
  : crypto.randomBytes(16).toString('hex');

const hello = {
  protocol: 'bitbook-wallet-broker',
  min: 1,
  max: 1,
  child_nonce: childNonce,
  child_pid: String(process.pid),
};

if (mode === 'partial-hello') {
  process.stdout.write(encodeFrame(hello).subarray(0, 2));
  process.exit(0);
}

if (mode === 'malformed-hello') {
  const body = Buffer.from('{', 'utf8');
  const header = Buffer.alloc(4);
  header.writeUInt32BE(body.length, 0);
  process.stdout.write(Buffer.concat([header, body]));
  process.exit(0);
}

if (mode === 'reverse') process.stderr.write(`${SENTINEL}\n`);

writeSplit(encodeFrame(hello));

let session = null;
let childSeq = 1;
let bootstrapped = false;
const queued = [];
const decoder = createDecoder();

function reply(envelope) {
  process.stdout.write(encodeFrame(envelope));
}

function handle(value) {
  if (!session) {
    if (!value || value.protocol !== 'bitbook-wallet-broker' || value.version !== 1 ||
        typeof value.parent_nonce !== 'string' || !HEX32.test(value.parent_nonce) ||
        typeof value.parent_pid !== 'string') {
      process.exit(1);
    }
    session = computeSession(value.parent_pid, hello.child_pid, value.parent_nonce, childNonce);
    return;
  }

  if (!bootstrapped) {
    if (!value || value.kind !== 'req' || value.method !== 'status.get' ||
        value.id !== BOOTSTRAP_ID || value.seq !== 1 || value.session !== session ||
        !value.params || Object.keys(value.params).length !== 0) {
      process.exit(1);
    }
    if (mode === 'no-bootstrap') return;
    const event = {
      v: 1,
      id: '11112222333344445555666677778888',
      seq: childSeq,
      kind: 'evt',
      method: 'sync.subscribe',
      params: { snapshot: EARLY_EVENT_SNAPSHOT },
      session,
    };
    childSeq += 1;
    const bootstrap = {
      v: 1,
      id: BOOTSTRAP_ID,
      seq: childSeq,
      kind: 'res',
      result: BOOTSTRAP_SNAPSHOT,
      session,
    };
    childSeq += 1;
    if (mode === 'ready') {
      process.stdout.write(Buffer.concat([encodeFrame(event), encodeFrame(bootstrap)]));
    } else {
      reply(event);
      reply(bootstrap);
    }
    bootstrapped = true;
    return;
  }

  if (!value || value.kind !== 'req') return;
  if (mode === 'hold') return;

  if (mode === 'partial-eof') {
    process.stdout.write(Buffer.from([0x00, 0x00]));
    process.exit(0);
  }
  if (mode === 'malformed') {
    const body = Buffer.from('{', 'utf8');
    const header = Buffer.alloc(4);
    header.writeUInt32BE(body.length, 0);
    process.stdout.write(Buffer.concat([header, body]));
    return;
  }
  if (mode === 'wrong-session') {
    reply({
      v: 1, id: value.id, seq: childSeq, kind: 'res', result: RESULT_ALPHA, session: '0'.repeat(64),
    });
    childSeq += 1;
    return;
  }
  if (mode === 'uncorrelated') {
    reply({
      v: 1,
      id: '99990000111122223333444455556666',
      seq: childSeq,
      kind: 'res',
      result: RESULT_ALPHA,
      session,
    });
    childSeq += 1;
    return;
  }
  if (mode === 'reverse') {
    if (queued.length < 2) {
      queued.push(value);
      if (queued.length === 2) {
        const first = queued[0];
        const second = queued[1];
        const resSecond = {
          v: 1, id: second.id, seq: childSeq, kind: 'res', result: RESULT_BETA, session,
        };
        childSeq += 1;
        const resFirst = {
          v: 1, id: first.id, seq: childSeq, kind: 'res', result: RESULT_ALPHA, session,
        };
        childSeq += 1;
        process.stdout.write(Buffer.concat([encodeFrame(resSecond), encodeFrame(resFirst)]));
      }
      return;
    }
    reply({
      v: 1, id: value.id, seq: childSeq, kind: 'error', error: lockedError(), session,
    });
    childSeq += 1;
    return;
  }

  const result = value.method === 'status.get' ? STATUS_SNAPSHOT : RESULT_ALPHA;
  const envelope = {
    v: 1, id: value.id, seq: childSeq, kind: 'res', result, session,
  };
  childSeq += 1;
  if (mode === 'ready') writeSplit(encodeFrame(envelope));
  else reply(envelope);
}

process.stdin.on('data', (chunk) => {
  let values;
  try {
    values = decoder.push(chunk);
  } catch (_) {
    process.exit(1);
  }
  for (const value of values) handle(value);
});

process.stdin.on('end', () => {
  process.exit(0);
});
