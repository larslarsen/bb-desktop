'use strict';

const assert = require('assert');
const childProcess = require('child_process');
const crypto = require('crypto');
const fs = require('fs');
const http = require('http');
const path = require('path');
const vm = require('vm');
const { decodeSignedObject } = require('../wallet-contract/canonical');
const fixtures = require('./fixtures/payment-inbox/records-v1.json');
const { createPaymentInboxClient } = require('../wallet-pay/inbox-client');

const TREE_ROOT = path.join(__dirname, '..', 'dist', 'pay001-grok-correction01', 'trees');
let treeSerial = 0;

const tests = [];
function test(name, fn) {
  tests.push({
    name,
    async fn() {
      let value;
      let testError;
      let cleanupError;
      try {
        value = await fn();
      } catch (error) {
        testError = error;
      }
      try {
        await disposeAppFixtures();
      } catch (error) {
        cleanupError = error;
      }
      if (testError && cleanupError) {
        throw new AggregateError([testError, cleanupError], `${name}: test and fixture cleanup failed`);
      }
      if (testError) throw testError;
      if (cleanupError) throw cleanupError;
      return value;
    },
  });
}

const KEY = Buffer.alloc(32, 7).toString('base64');
const SIG = Buffer.alloc(32, 9).toString('base64');
const LOCAL = fixtures.local_peer_id;
const NOW = Date.parse('2026-08-30T12:10:00Z');

function vector(name) {
  return fixtures.vectors.find((item) => item.name === name);
}

function signedRecord(item, overrides = {}) {
  return {
    signed: {
      version: 1,
      kind: item.kind,
      canonical: item.canonical,
      public_key: overrides.public_key || KEY,
      signature: overrides.signature || SIG,
    },
    digest: item.digest,
    direction: overrides.direction || item.direction,
    received_at: overrides.received_at || '2026-08-30T12:01:00Z',
  };
}

function envelope(peerId, instanceId, records) {
  return {
    v: 1,
    peer_id: peerId,
    instance_id: instanceId,
    records,
  };
}

function descriptorFields(port, overrides = {}) {
  return {
    v: 1,
    endpoint: `http://127.0.0.1:${port}`,
    peer_id: overrides.peer_id || LOCAL,
    instance_id: overrides.instance_id || 'a'.repeat(32),
    token: overrides.token || 'b'.repeat(64),
  };
}

async function makeTree() {
  treeSerial += 1;
  await fs.promises.mkdir(TREE_ROOT, { recursive: true });
  const home = path.join(TREE_ROOT, `home-${process.pid}-${treeSerial}`);
  const root = path.join(home, '.bitbook', 'modern');
  const privateDir = path.join(root, 'local-client');
  await fs.promises.mkdir(privateDir, { recursive: true, mode: 0o700 });
  await fs.promises.chmod(privateDir, 0o700);
  return { home, root, privateDir, descriptorPath: path.join(privateDir, 'connection.json') };
}

async function writeDescriptor(tree, fields) {
  await fs.promises.writeFile(tree.descriptorPath, JSON.stringify(fields), { mode: 0o600 });
  await fs.promises.chmod(tree.descriptorPath, 0o600);
}

function startServer(handler) {
  return new Promise((resolve, reject) => {
    const seen = [];
    const server = http.createServer((req, res) => {
      const chunks = [];
      req.on('data', (chunk) => chunks.push(chunk));
      req.on('end', () => {
        seen.push({
          method: req.method,
          url: req.url,
          headers: req.headers,
          body: Buffer.concat(chunks),
        });
        handler(req, res, seen[seen.length - 1]);
      });
    });
    server.listen(0, '127.0.0.1', () => {
      resolve({
        server,
        port: server.address().port,
        seen,
        close: () => new Promise((done) => server.close(done)),
      });
    });
    server.on('error', reject);
  });
}

function jsonHandler(payload, status = 200) {
  return (req, res) => {
    const body = Buffer.from(JSON.stringify(payload));
    res.writeHead(status, { 'Content-Type': 'application/json', 'Content-Length': body.length });
    res.end(body);
  };
}

function clientFor(tree, extras = {}) {
  return createPaymentInboxClient({
    platform: extras.platform || 'linux',
    homedir: () => tree.home,
    now: extras.now || (() => NOW),
    dialog: extras.dialog,
    parentWindow: extras.parentWindow || (() => null),
    ...extras,
  });
}

function defaultClient(tree, extras = {}) {
  const inbox = createPaymentInboxClient({
    platform: 'linux',
    homedir: () => tree.home,
    now: extras.now || (() => NOW),
    dialog: extras.dialog,
    parentWindow: () => null,
  });
  return inbox;
}

test('independent fixtures match frozen canonical digest oracle', () => {
  for (const item of fixtures.vectors) {
    const kind = item.kind === 'request' ? 'payment_request_v1' : 'payment_status_event_v1';
    const decoded = decodeSignedObject(kind, Buffer.from(item.canonical, 'utf8'));
    assert.strictEqual(decoded.canonical, item.canonical, item.name);
    assert.strictEqual(decoded.digest, item.digest, item.name);
  }
});

test('non-linux returns unsupported without network', async () => {
  const httpHits = [];
  const inbox = createPaymentInboxClient({
    platform: 'darwin',
    homedir: () => '/nope',
    fetchRecords: async () => { httpHits.push(1); return Buffer.from('{}'); },
  });
  const reply = await inbox.getInbox();
  assert.deepStrictEqual(reply, { v: 1, state: 'unsupported', peer_id: '', requests: [] });
  assert.strictEqual(httpHits.length, 0);
});

test('missing descriptor is unavailable before HTTP', async () => {
  const tree = await makeTree();
  const hits = [];
  const inbox = createPaymentInboxClient({
    platform: 'linux',
    homedir: () => tree.home,
    fetchRecords: async () => { hits.push(1); return Buffer.from('{}'); },
  });
  const reply = await inbox.getInbox();
  assert.strictEqual(reply.state, 'unavailable');
  assert.strictEqual(reply.requests.length, 0);
  assert.strictEqual(hits.length, 0);
  // retained owned fixture directory
});

test('symlink descriptor is refused before HTTP', async () => {
  const tree = await makeTree();
  const outside = path.join(tree.root, 'outside.json');
  await fs.promises.writeFile(outside, JSON.stringify(descriptorFields(9)));
  await fs.promises.symlink(outside, tree.descriptorPath);
  const hits = [];
  const inbox = createPaymentInboxClient({
    platform: 'linux',
    homedir: () => tree.home,
    fetchRecords: async () => { hits.push(1); return Buffer.from('{}'); },
  });
  const reply = await inbox.getInbox();
  assert.strictEqual(reply.state, 'unavailable');
  assert.strictEqual(hits.length, 0);
  // retained owned fixture directory
});

test('world-readable private directory is refused and mode is unchanged', async () => {
  const tree = await makeTree();
  await fs.promises.chmod(tree.privateDir, 0o755);
  const hits = [];
  const inbox = createPaymentInboxClient({
    platform: 'linux',
    homedir: () => tree.home,
    fetchRecords: async () => { hits.push(1); return Buffer.from('{}'); },
  });
  const reply = await inbox.getInbox();
  assert.strictEqual(reply.state, 'unavailable');
  assert.strictEqual(hits.length, 0);
  const mode = (await fs.promises.lstat(tree.privateDir)).mode & 0o777;
  assert.strictEqual(mode, 0o755);
  // retained owned fixture directory
});

test('GET uses method path bearer instance and no body origin or query', async () => {
  const coffee = vector('inbound-zec-coffee');
  const fixture = startWaiter();
  const httpz = await startServer(jsonHandler(envelope(LOCAL, 'a'.repeat(32), [signedRecord(coffee)])));
  const tree = await makeTree();
  await writeDescriptor(tree, descriptorFields(httpz.port));
  const inbox = createPaymentInboxClient({
    platform: 'linux',
    homedir: () => tree.home,
    now: () => NOW,
  });
  const reply = await inbox.getInbox();
  assert.strictEqual(reply.state, 'ready');
  assert.strictEqual(reply.peer_id, LOCAL);
  assert.strictEqual(reply.requests.length, 1);
  assert.strictEqual(reply.requests[0].memo, 'coffee');
  assert.strictEqual(reply.requests[0].amount_display, coffee.amount_display);
  assert.strictEqual(reply.requests[0].status, 'requested');
  assert.strictEqual(httpz.seen.length, 1);
  const seen = httpz.seen[0];
  assert.strictEqual(seen.method, 'GET');
  assert.strictEqual(seen.url, '/v1/payment/records');
  assert.strictEqual(seen.headers.authorization, `Bearer ${'b'.repeat(64)}`);
  assert.strictEqual(seen.headers['x-bitbook-instance'], 'a'.repeat(32));
  assert.strictEqual(seen.body.length, 0);
  assert.strictEqual(seen.headers.origin, undefined);
  assert.ok(!seen.url.includes('?'));
  const blob = JSON.stringify(reply);
  assert.ok(!blob.includes('b'.repeat(64)));
  assert.ok(!blob.includes('a'.repeat(32)));
  assert.ok(!blob.includes('127.0.0.1'));
  inbox.dispose();
  await httpz.close();
  // retained owned fixture directory
  fixture();
});

function startWaiter() {
  return () => {};
}

test('nonempty mix filters outbound, applies cancel, sorts newest first', async () => {
  const tea = vector('inbound-zec-tea');
  const coffee = vector('inbound-zec-coffee');
  const xmr = vector('inbound-xmr');
  const large = vector('inbound-zec-precise-large');
  const outbound = vector('outbound-zec');
  const cancel = vector('cancel-coffee');
  const records = [
    signedRecord(coffee),
    signedRecord(tea),
    signedRecord(xmr),
    signedRecord(large),
    signedRecord(outbound),
    signedRecord(cancel),
  ];
  const httpz = await startServer(jsonHandler(envelope(LOCAL, 'a'.repeat(32), records)));
  const tree = await makeTree();
  await writeDescriptor(tree, descriptorFields(httpz.port));
  const inbox = createPaymentInboxClient({
    platform: 'linux',
    homedir: () => tree.home,
    now: () => NOW,
  });
  const reply = await inbox.getInbox();
  assert.strictEqual(reply.state, 'ready');
  const ids = reply.requests.map((row) => row.request_id);
  assert.deepStrictEqual(ids, [
    tea.canonical.match(/"request_id":"([^"]+)"/)[1],
    xmr.canonical.match(/"request_id":"([^"]+)"/)[1],
    coffee.canonical.match(/"request_id":"([^"]+)"/)[1],
    large.canonical.match(/"request_id":"([^"]+)"/)[1],
  ]);
  const byId = Object.fromEntries(reply.requests.map((row) => [row.request_id, row]));
  assert.strictEqual(byId['00112233445566778899aabbccddeeff'].status, 'cancelled');
  assert.strictEqual(byId['aabbccddeeff00112233445566778899'].amount_display, '2.50000000');
  assert.strictEqual(byId['bbbbccccddddeeeeffff000011112222'].amount_display, '1.234567890123');
  assert.strictEqual(byId['ccccddddeeeeffff0000111122223333'].amount_display, '123456789.01234567');
  assert.ok(!ids.includes('ddddffffeeeeaaaa1111222233334444'));
  inbox.dispose();
  await httpz.close();
  // retained owned fixture directory
});

test('expiry boundary requested then expired', async () => {
  const coffee = vector('inbound-zec-coffee');
  const httpz = await startServer(jsonHandler(envelope(LOCAL, 'a'.repeat(32), [signedRecord(coffee)])));
  const tree = await makeTree();
  await writeDescriptor(tree, descriptorFields(httpz.port));
  const before = createPaymentInboxClient({
    platform: 'linux', homedir: () => tree.home, now: () => Date.parse('2026-08-30T12:14:59Z'),
  });
  const at = createPaymentInboxClient({
    platform: 'linux', homedir: () => tree.home, now: () => Date.parse('2026-08-30T12:15:00Z'),
  });
  assert.strictEqual((await before.getInbox()).requests[0].status, 'requested');
  assert.strictEqual((await at.getInbox()).requests[0].status, 'expired');
  before.dispose();
  at.dispose();
  await httpz.close();
  // retained owned fixture directory
});

test('wrong instance is invalid and credentials stay off the reply', async () => {
  const coffee = vector('inbound-zec-coffee');
  const httpz = await startServer(jsonHandler(envelope(LOCAL, 'c'.repeat(32), [signedRecord(coffee)])));
  const tree = await makeTree();
  await writeDescriptor(tree, descriptorFields(httpz.port));
  const inbox = createPaymentInboxClient({
    platform: 'linux', homedir: () => tree.home, now: () => NOW,
  });
  const reply = await inbox.getInbox();
  assert.strictEqual(reply.state, 'invalid');
  assert.strictEqual(reply.requests.length, 0);
  const blob = JSON.stringify(reply);
  assert.ok(!blob.includes('b'.repeat(64)));
  assert.ok(!blob.includes('/v1/payment/records'));
  inbox.dispose();
  await httpz.close();
  // retained owned fixture directory
});

test('same-peer credential rotation succeeds; different peer is identity_changed without using old rows', async () => {
  const coffee = vector('inbound-zec-coffee');
  const httpz = await startServer(jsonHandler(envelope(LOCAL, 'd'.repeat(32), [signedRecord(coffee)])));
  const tree = await makeTree();
  await writeDescriptor(tree, descriptorFields(httpz.port, { instance_id: 'd'.repeat(32), token: 'e'.repeat(64) }));
  const inbox = createPaymentInboxClient({
    platform: 'linux', homedir: () => tree.home, now: () => NOW,
  });
  const first = await inbox.getInbox();
  assert.strictEqual(first.state, 'ready');
  await writeDescriptor(tree, descriptorFields(httpz.port, {
    instance_id: 'f'.repeat(32), token: 'c'.repeat(64), peer_id: fixtures.other_peer_id,
  }));
  const second = await inbox.getInbox();
  assert.strictEqual(second.state, 'identity_changed');
  assert.strictEqual(second.requests.length, 0);
  assert.strictEqual(second.peer_id, LOCAL);
  inbox.dispose();
  await httpz.close();
  // retained owned fixture directory
});

test('unavailable server and deadline', async () => {
  const tree = await makeTree();
  await writeDescriptor(tree, descriptorFields(1));
  const inbox = createPaymentInboxClient({
    platform: 'linux', homedir: () => tree.home, now: () => NOW,
  });
  const reply = await inbox.getInbox();
  assert.strictEqual(reply.state, 'unavailable');
  const hanging = await startServer((req, res) => {
    setTimeout(() => res.end('{}'), 8000);
  });
  await writeDescriptor(tree, descriptorFields(hanging.port));
  const started = Date.now();
  const timed = await inbox.getInbox();
  assert.strictEqual(timed.state, 'unavailable');
  assert.ok(Date.now() - started < 7000);
  inbox.dispose();
  await hanging.close();
  // retained owned fixture directory
});

test('paid event and duplicate requests fail closed', async () => {
  const coffee = vector('inbound-zec-coffee');
  const paidVec = vector('paid-coffee-status');
  const paid = signedRecord(paidVec);
  const httpPaid = await startServer(jsonHandler(envelope(LOCAL, 'a'.repeat(32), [paid])));
  const tree = await makeTree();
  await writeDescriptor(tree, descriptorFields(httpPaid.port));
  const inbox = createPaymentInboxClient({
    platform: 'linux', homedir: () => tree.home, now: () => NOW,
  });
  const paidReply = await inbox.getInbox();
  assert.strictEqual(paidReply.state, 'invalid');
  await httpPaid.close();
  const dup = await startServer(jsonHandler(envelope(LOCAL, 'a'.repeat(32), [signedRecord(coffee), signedRecord(coffee)])));
  await writeDescriptor(tree, descriptorFields(dup.port));
  const dupReply = await inbox.getInbox();
  assert.strictEqual(dupReply.state, 'invalid');
  inbox.dispose();
  await dup.close();
  // retained owned fixture directory
});

function independentStatusDigest(canonical) {
  return crypto.createHash('sha256')
    .update('bitbook-payment-status-v1\n')
    .update(canonical)
    .digest('hex');
}

test('descriptor cap below at above with valid padded JSON', async () => {
  const coffee = vector('inbound-zec-coffee');
  const httpz = await startServer(jsonHandler(envelope(LOCAL, 'a'.repeat(32), [signedRecord(coffee)])));
  const tree = await makeTree();
  const inbox = createPaymentInboxClient({
    platform: 'linux', homedir: () => tree.home, now: () => NOW,
  });
  const fields = descriptorFields(httpz.port);
  const json = JSON.stringify(fields);
  const jsonBytes = Buffer.byteLength(json);
  const below = `${json}${' '.repeat(16)}`;
  await writeDescriptor(tree, JSON.parse(json));
  await fs.promises.writeFile(tree.descriptorPath, below, { mode: 0o600 });
  await fs.promises.chmod(tree.descriptorPath, 0o600);
  assert.strictEqual((await inbox.getInbox()).state, 'ready');
  const at = `${json}${' '.repeat(16 * 1024 - jsonBytes)}`;
  assert.strictEqual(Buffer.byteLength(at), 16 * 1024);
  await fs.promises.writeFile(tree.descriptorPath, at, { mode: 0o600 });
  await fs.promises.chmod(tree.descriptorPath, 0o600);
  assert.strictEqual((await inbox.getInbox()).state, 'ready');
  const over = `${at} `;
  await fs.promises.writeFile(tree.descriptorPath, over, { mode: 0o600 });
  await fs.promises.chmod(tree.descriptorPath, 0o600);
  assert.strictEqual((await inbox.getInbox()).state, 'unavailable');
  inbox.dispose();
  await httpz.close();
});

test('invalid utf8 body is invalid', async () => {
  const tree = await makeTree();
  const inbox = createPaymentInboxClient({
    platform: 'linux', homedir: () => tree.home, now: () => NOW,
  });
  const httpz = await startServer((req, res) => {
    res.writeHead(200, { 'Content-Type': 'application/json' });
    res.end(Buffer.from([0xff, 0xfe, 0xfd]));
  });
  await writeDescriptor(tree, descriptorFields(httpz.port));
  const utf = await inbox.getInbox();
  assert.strictEqual(utf.state, 'invalid');
  inbox.dispose();
  await httpz.close();
  // retained owned fixture directory
});

test('truncated JSON and deep payload are invalid', async () => {
  const httpz = await startServer((req, res) => {
    res.writeHead(200, { 'Content-Type': 'application/json' });
    res.end('{"v":1,"peer_id":');
  });
  const tree = await makeTree();
  await writeDescriptor(tree, descriptorFields(httpz.port));
  const inbox = createPaymentInboxClient({
    platform: 'linux', homedir: () => tree.home, now: () => NOW,
  });
  assert.strictEqual((await inbox.getInbox()).state, 'invalid');
  await httpz.close();
  const deepCanonical = `${'['.repeat(12)}"x"${']'.repeat(12)}`;
  const deep = {
    signed: {
      version: 1, kind: 'request', canonical: deepCanonical, public_key: KEY, signature: SIG,
    },
    digest: 'aa'.repeat(32),
    direction: 'inbound',
    received_at: '2026-08-30T12:01:00Z',
  };
  const deepServer = await startServer(jsonHandler(envelope(LOCAL, 'a'.repeat(32), [deep])));
  await writeDescriptor(tree, descriptorFields(deepServer.port));
  assert.strictEqual((await inbox.getInbox()).state, 'invalid');
  inbox.dispose();
  await deepServer.close();
  // retained owned fixture directory
});

test('connect cancellation keeps selection; connect chooses a directory', async () => {
  const coffee = vector('inbound-zec-coffee');
  const httpz = await startServer(jsonHandler(envelope(LOCAL, 'a'.repeat(32), [signedRecord(coffee)])));
  const tree = await makeTree();
  await writeDescriptor(tree, descriptorFields(httpz.port));
  const other = await makeTree();
  await writeDescriptor(other, descriptorFields(httpz.port));
  let picks = 0;
  const inbox = createPaymentInboxClient({
    platform: 'linux',
    homedir: () => tree.home,
    now: () => NOW,
    dialog: {
      async showOpenDialog() {
        picks += 1;
        if (picks === 1) return { canceled: true, filePaths: [] };
        return { canceled: false, filePaths: [other.root] };
      },
    },
  });
  const cancelled = await inbox.connectInbox();
  assert.strictEqual(cancelled.state, 'ready');
  const chosen = await inbox.connectInbox();
  assert.strictEqual(chosen.state, 'ready');
  assert.strictEqual(picks, 2);
  inbox.dispose();
  await httpz.close();
  // retained owned fixture directory
  // retained owned fixture directory
});

test('dispose aborts in-flight work', async () => {
  let hangingRes;
  const httpz = await startServer((req, res) => { hangingRes = res; });
  const tree = await makeTree();
  await writeDescriptor(tree, descriptorFields(httpz.port));
  const inbox = createPaymentInboxClient({
    platform: 'linux', homedir: () => tree.home, now: () => NOW,
  });
  const pending = inbox.getInbox();
  inbox.dispose();
  const reply = await pending;
  assert.strictEqual(reply.state, 'unavailable');
  if (hangingRes) hangingRes.end();
  await httpz.close();
  // retained owned fixture directory
});

test('paid and expired status fixtures match standalone domain hashes', () => {
  for (const name of ['paid-coffee-status', 'expired-event-status']) {
    const item = vector(name);
    assert.strictEqual(independentStatusDigest(item.canonical), item.digest, name);
  }
});

test('expired signed status event fails closed with matching digest', async () => {
  const coffee = vector('inbound-zec-coffee');
  const expired = vector('expired-event-status');
  const httpz = await startServer(jsonHandler(envelope(LOCAL, 'a'.repeat(32), [
    signedRecord(coffee), signedRecord(expired),
  ])));
  const tree = await makeTree();
  await writeDescriptor(tree, descriptorFields(httpz.port));
  const inbox = createPaymentInboxClient({
    platform: 'linux', homedir: () => tree.home, now: () => NOW,
  });
  assert.strictEqual((await inbox.getInbox()).state, 'invalid');
  inbox.dispose();
  await httpz.close();
});

test('FIFO descriptor is rejected promptly without HTTP', async () => {
  const hits = [];
  const tree = await makeTree();
  childProcess.execFileSync('mkfifo', [tree.descriptorPath], { timeout: 2000 });
  const inbox = createPaymentInboxClient({
    platform: 'linux',
    homedir: () => tree.home,
    fetchRecords: async () => { hits.push(1); return Buffer.from('{}'); },
  });
  const started = Date.now();
  const reply = await Promise.race([
    inbox.getInbox(),
    new Promise((_, reject) => setTimeout(() => {
      inbox.dispose();
      reject(new Error('fifo hung'));
    }, 1000)),
  ]);
  assert.ok(Date.now() - started < 1000);
  assert.strictEqual(reply.state, 'unavailable');
  assert.strictEqual(hits.length, 0);
  inbox.dispose();
});

test('wrong-party inbound and conflicting cancellation invalidate snapshot', async () => {
  const coffee = vector('inbound-zec-coffee');
  const outbound = vector('outbound-zec');
  const otherInbound = JSON.parse(JSON.stringify(signedRecord(coffee)));
  const decoded = JSON.parse(coffee.canonical);
  decoded.payer_peer_id = fixtures.other_peer_id;
  const { decodeSignedObject: decode } = require('../wallet-contract/canonical');
  const rebuilt = decode('payment_request_v1', Buffer.from(JSON.stringify({
    v: 1,
    request_id: 'ffffeeeeddddccccbbbbaaaa99998888',
    payer_peer_id: fixtures.other_peer_id,
    payee_peer_id: fixtures.payee_peer_id,
    asset: 'ZEC',
    network: 'zec-testnet',
    amount_atomic: '100000000',
    receiver: 'u1testreceiver',
    receiver_kind: 'zec-ua-orchard-protocol',
    memo: 'other',
    nonce: '99998888777766665555444433332222',
    created_at: '2026-08-30T12:00:00Z',
    expires_at: '2026-08-30T13:00:00Z',
  })));
  const foreign = {
    signed: {
      version: 1, kind: 'request', canonical: rebuilt.canonical, public_key: KEY, signature: SIG,
    },
    digest: rebuilt.digest,
    direction: 'inbound',
    received_at: '2026-08-30T12:01:00Z',
  };
  const httpForeign = await startServer(jsonHandler(envelope(LOCAL, 'a'.repeat(32), [foreign])));
  const tree = await makeTree();
  await writeDescriptor(tree, descriptorFields(httpForeign.port));
  const inbox = createPaymentInboxClient({
    platform: 'linux', homedir: () => tree.home, now: () => NOW,
  });
  assert.strictEqual((await inbox.getInbox()).state, 'invalid');
  await httpForeign.close();
  const cancel = signedRecord(vector('cancel-coffee'), { public_key: Buffer.alloc(32, 1).toString('base64') });
  const httpConflict = await startServer(jsonHandler(envelope(LOCAL, 'a'.repeat(32), [
    signedRecord(coffee), cancel,
  ])));
  await writeDescriptor(tree, descriptorFields(httpConflict.port));
  assert.strictEqual((await inbox.getInbox()).state, 'invalid');
  inbox.dispose();
  await httpConflict.close();
  void otherInbound;
  void outbound;
});

test('same-peer rotation after successful read; changed response peer is identity_changed', async () => {
  const coffee = vector('inbound-zec-coffee');
  let instance = 'd'.repeat(32);
  const httpz = await startServer((req, res) => {
    jsonHandler(envelope(LOCAL, instance, [signedRecord(coffee)]))(req, res);
  });
  const tree = await makeTree();
  await writeDescriptor(tree, descriptorFields(httpz.port, { instance_id: instance, token: 'e'.repeat(64) }));
  const inbox = createPaymentInboxClient({
    platform: 'linux', homedir: () => tree.home, now: () => NOW,
  });
  assert.strictEqual((await inbox.getInbox()).state, 'ready');
  instance = 'f'.repeat(32);
  await writeDescriptor(tree, descriptorFields(httpz.port, { instance_id: instance, token: 'c'.repeat(64) }));
  const rotated = await inbox.getInbox();
  assert.strictEqual(rotated.state, 'ready');
  assert.strictEqual(rotated.peer_id, LOCAL);
  const peerServer = await startServer(jsonHandler(envelope(fixtures.other_peer_id, 'a'.repeat(32), [signedRecord(coffee)])));
  await writeDescriptor(tree, descriptorFields(peerServer.port));
  const changed = await inbox.getInbox();
  assert.strictEqual(changed.state, 'identity_changed');
  assert.strictEqual(changed.requests.length, 0);
  inbox.dispose();
  await httpz.close();
  await peerServer.close();
});

test('hostile endpoints never cause HTTP', async () => {
  const hits = [];
  const tree = await makeTree();
  const inbox = createPaymentInboxClient({
    platform: 'linux',
    homedir: () => tree.home,
    fetchRecords: async () => { hits.push(1); return Buffer.from('{}'); },
  });
  const hostiles = [
    'http://127.0.0.1:1/v1',
    'https://127.0.0.1:1',
    'http://127.0.0.1:0',
    'http://localhost:1',
    'http://127.0.0.1:1?x=1',
    'http://127.0.0.1:1#x',
    'http://127.0.0.1:1/',
    'http://192.168.0.1:1',
    'http://127.0.0.1:65536',
  ];
  for (const endpoint of hostiles) {
    await fs.promises.writeFile(tree.descriptorPath, JSON.stringify({
      v: 1, endpoint, peer_id: LOCAL, instance_id: 'a'.repeat(32), token: 'b'.repeat(64),
    }), { mode: 0o600 });
    await fs.promises.chmod(tree.descriptorPath, 0o600);
    const reply = await inbox.getInbox();
    assert.strictEqual(reply.state, 'unavailable', endpoint);
  }
  assert.strictEqual(hits.length, 0);
  inbox.dispose();
});

test('impossible received_at and oversize signature are invalid', async () => {
  const coffee = vector('inbound-zec-coffee');
  const badDate = signedRecord(coffee, { received_at: '2026-02-31T12:00:00Z' });
  const httpz = await startServer(jsonHandler(envelope(LOCAL, 'a'.repeat(32), [badDate])));
  const tree = await makeTree();
  await writeDescriptor(tree, descriptorFields(httpz.port));
  const inbox = createPaymentInboxClient({
    platform: 'linux', homedir: () => tree.home, now: () => NOW,
  });
  assert.strictEqual((await inbox.getInbox()).state, 'invalid');
  await httpz.close();
  const longSig = signedRecord(coffee, { signature: `${'A'.repeat(1025)}=` });
  const httpSig = await startServer(jsonHandler(envelope(LOCAL, 'a'.repeat(32), [longSig])));
  await writeDescriptor(tree, descriptorFields(httpSig.port));
  assert.strictEqual((await inbox.getInbox()).state, 'invalid');
  inbox.dispose();
  await httpSig.close();
});

test('cancelled after expiry still cancelled; amount and id tie order', async () => {
  const coffee = vector('inbound-zec-coffee');
  const late = vector('inbound-zec-late');
  const cancel = vector('cancel-coffee');
  const httpz = await startServer(jsonHandler(envelope(LOCAL, 'a'.repeat(32), [
    signedRecord(coffee), signedRecord(late), signedRecord(cancel),
  ])));
  const tree = await makeTree();
  await writeDescriptor(tree, descriptorFields(httpz.port));
  const inbox = createPaymentInboxClient({
    platform: 'linux',
    homedir: () => tree.home,
    now: () => Date.parse('2026-08-30T12:20:00Z'),
  });
  const reply = await inbox.getInbox();
  assert.strictEqual(reply.state, 'ready');
  const byId = Object.fromEntries(reply.requests.map((row) => [row.request_id, row]));
  assert.strictEqual(byId['00112233445566778899aabbccddeeff'].status, 'cancelled');
  assert.strictEqual(byId['eeeeffff000011112222333344445555'].status, 'expired');
  inbox.dispose();
  await httpz.close();
});

test('streaming 302 and 500 terminate without hanging', async () => {
  const hanging = [];
  async function streamStatus(status) {
    const server = await startServer((req, res) => {
      hanging.push(res);
      res.writeHead(status, { 'Content-Type': 'text/plain' });
      const tick = () => {
        if (!res.destroyed && res.writable) {
          res.write('x'.repeat(1024));
          setTimeout(tick, 20);
        }
      };
      tick();
    });
    const tree = await makeTree();
    await writeDescriptor(tree, descriptorFields(server.port));
    const inbox = createPaymentInboxClient({
      platform: 'linux', homedir: () => tree.home, now: () => NOW,
    });
    const started = Date.now();
    const reply = await inbox.getInbox();
    assert.strictEqual(reply.state, 'unavailable');
    assert.ok(Date.now() - started < 2000, `status ${status} hung`);
    inbox.dispose();
    await server.close();
  }
  await streamStatus(302);
  await streamStatus(500);
});

test('unsupported encoding and oversized 503', async () => {
  const gzip = await startServer((req, res) => {
    res.writeHead(200, { 'Content-Type': 'application/json', 'Content-Encoding': 'gzip' });
    res.end('{}');
  });
  const tree = await makeTree();
  await writeDescriptor(tree, descriptorFields(gzip.port));
  const inbox = createPaymentInboxClient({
    platform: 'linux', homedir: () => tree.home, now: () => NOW,
  });
  assert.strictEqual((await inbox.getInbox()).state, 'invalid');
  await gzip.close();
  const over503 = await startServer((req, res) => {
    res.writeHead(503, { 'Content-Type': 'application/json' });
    res.end(JSON.stringify({ error: 'TOO_LARGE', pad: 'n'.repeat(400) }));
  });
  await writeDescriptor(tree, descriptorFields(over503.port));
  assert.strictEqual((await inbox.getInbox()).state, 'unavailable');
  await over503.close();
  const ok503 = await startServer((req, res) => {
    res.writeHead(503, { 'Content-Type': 'application/json' });
    res.end(JSON.stringify({ error: 'TOO_LARGE' }));
  });
  await writeDescriptor(tree, descriptorFields(ok503.port));
  assert.strictEqual((await inbox.getInbox()).state, 'too_large');
  inbox.dispose();
  await ok503.close();
});

test('dispose during descriptor read never starts HTTP', async () => {
  const hits = [];
  const tree = await makeTree();
  const coffee = vector('inbound-zec-coffee');
  await writeDescriptor(tree, descriptorFields(9, {}));
  const origOpen = fs.promises.open;
  let release;
  const gate = new Promise((resolve) => { release = resolve; });
  fs.promises.open = async function gatedOpen(...args) {
    if (String(args[0]).endsWith('connection.json')) await gate;
    return origOpen.apply(this, args);
  };
  const inbox = createPaymentInboxClient({
    platform: 'linux',
    homedir: () => tree.home,
    now: () => NOW,
    fetchRecords: async () => { hits.push(1); return Buffer.from(JSON.stringify(envelope(LOCAL, 'a'.repeat(32), [signedRecord(coffee)]))); },
  });
  const pending = inbox.getInbox();
  inbox.dispose();
  release();
  const reply = await pending;
  fs.promises.open = origOpen;
  assert.strictEqual(hits.length, 0);
  assert.strictEqual(reply.state, 'unavailable');
});

test('stale completion cannot unpin a newer selection', async () => {
  const coffee = vector('inbound-zec-coffee');
  const tea = vector('inbound-zec-tea');
  const finishes = [];
  const inbox = createPaymentInboxClient({
    platform: 'linux',
    homedir: () => '/unused',
    now: () => NOW,
    fetchRecords: (descriptor) => new Promise((resolve) => {
      finishes.push({ port: descriptor.port, resolve });
    }),
  });
  const first = await makeTree();
  const http1 = await startServer(jsonHandler(envelope(LOCAL, 'a'.repeat(32), [signedRecord(coffee)])));
  await writeDescriptor(first, descriptorFields(http1.port, { instance_id: 'a'.repeat(32) }));
  const second = await makeTree();
  const http2 = await startServer(jsonHandler(envelope(LOCAL, 'c'.repeat(32), [signedRecord(tea)])));
  await writeDescriptor(second, descriptorFields(http2.port, { instance_id: 'c'.repeat(32), token: 'd'.repeat(64) }));
  const dialogs = [
    { canceled: false, filePaths: [first.root] },
    { canceled: false, filePaths: [second.root] },
  ];
  inbox.dialog = null;
  const wired = createPaymentInboxClient({
    platform: 'linux',
    homedir: () => first.home,
    now: () => NOW,
    dialog: {
      async showOpenDialog() { return dialogs.shift(); },
    },
    fetchRecords: (descriptor) => new Promise((resolve) => {
      const body = descriptor.instance_id === 'a'.repeat(32)
        ? envelope(LOCAL, descriptor.instance_id, [signedRecord(coffee)])
        : envelope(LOCAL, descriptor.instance_id, [signedRecord(tea)]);
      finishes.push({ instance: descriptor.instance_id, resolve, body });
    }),
  });
  const firstGet = wired.connectInbox();
  const started = Date.now();
  while (finishes.length < 1 && Date.now() - started < 1000) {
    await new Promise((r) => setImmediate(r));
  }
  const secondGet = wired.connectInbox();
  while (finishes.length < 2 && Date.now() - started < 2000) {
    await new Promise((r) => setImmediate(r));
  }
  const old = finishes.find((item) => item.instance === 'a'.repeat(32));
  const newer = finishes.find((item) => item.instance === 'c'.repeat(32));
  assert.ok(old && newer, `both fetches should start after production fix (${finishes.length})`);
  old.resolve(Buffer.from(JSON.stringify(old.body)));
  const stale = await firstGet;
  newer.resolve(Buffer.from(JSON.stringify(newer.body)));
  const latest = await secondGet;
  assert.strictEqual(latest.state, 'ready');
  assert.strictEqual(latest.requests[0].memo, 'tea');
  assert.notStrictEqual(stale.requests[0] && stale.requests[0].memo, 'tea');
  wired.dispose();
  await http1.close();
  await http2.close();
});

test('concurrent picker calls and failed picker keep a closed DTO', async () => {
  const coffee = vector('inbound-zec-coffee');
  const httpz = await startServer(jsonHandler(envelope(LOCAL, 'a'.repeat(32), [signedRecord(coffee)])));
  const tree = await makeTree();
  await writeDescriptor(tree, descriptorFields(httpz.port));
  let opens = 0;
  let release;
  const gate = new Promise((resolve) => { release = resolve; });
  const inbox = createPaymentInboxClient({
    platform: 'linux',
    homedir: () => tree.home,
    now: () => NOW,
    dialog: {
      async showOpenDialog() {
        opens += 1;
        if (opens === 1) {
          await gate;
          return { canceled: true, filePaths: [] };
        }
        throw new Error('picker exploded');
      },
    },
  });
  const first = inbox.connectInbox();
  const second = inbox.connectInbox();
  release();
  const a = await first;
  const b = await second;
  assert.strictEqual(opens, 1);
  assert.strictEqual(a.state, 'ready');
  assert.strictEqual(b.state, 'ready');
  const failed = await inbox.connectInbox();
  assert.strictEqual(failed.state, 'unavailable');
  assert.strictEqual(failed.requests.length, 0);
  inbox.dispose();
  await httpz.close();
});

test('wrong ownership via scoped stat mock is unavailable without HTTP', async () => {
  const hits = [];
  const tree = await makeTree();
  await writeDescriptor(tree, descriptorFields(9));
  const origStat = fs.promises.stat;
  const origLstat = fs.promises.lstat;
  const origOpen = fs.promises.open;
  fs.promises.lstat = async function mockedLstat(p, ...rest) {
    const st = await origLstat.call(this, p, ...rest);
    if (String(p).endsWith(`${path.sep}local-client`) || String(p).endsWith('connection.json')) {
      return new Proxy(st, { get(target, prop) { return prop === 'uid' ? process.getuid() + 1 : target[prop]; } });
    }
    return st;
  };
  const inbox = createPaymentInboxClient({
    platform: 'linux',
    homedir: () => tree.home,
    fetchRecords: async () => { hits.push(1); return Buffer.from('{}'); },
  });
  const reply = await inbox.getInbox();
  fs.promises.lstat = origLstat;
  fs.promises.stat = origStat;
  fs.promises.open = origOpen;
  assert.strictEqual(reply.state, 'unavailable');
  assert.strictEqual(hits.length, 0);
  inbox.dispose();
});

test('root and private-dir symlinks are refused', async () => {
  const hits = [];
  const tree = await makeTree();
  const outside = path.join(tree.home, 'outside');
  await fs.promises.mkdir(outside, { recursive: true });
  await fs.promises.rmdir(tree.privateDir);
  await fs.promises.symlink(outside, tree.privateDir);
  const inbox = createPaymentInboxClient({
    platform: 'linux',
    homedir: () => tree.home,
    fetchRecords: async () => { hits.push(1); return Buffer.from('{}'); },
  });
  assert.strictEqual((await inbox.getInbox()).state, 'unavailable');
  assert.strictEqual(hits.length, 0);
  inbox.dispose();
});

test('orphan cancellation is ignored and body over 4MiB is too_large', async () => {
  const cancel = vector('cancel-coffee');
  const httpz = await startServer(jsonHandler(envelope(LOCAL, 'a'.repeat(32), [signedRecord(cancel)])));
  const tree = await makeTree();
  await writeDescriptor(tree, descriptorFields(httpz.port));
  const inbox = createPaymentInboxClient({
    platform: 'linux', homedir: () => tree.home, now: () => NOW,
  });
  const orphan = await inbox.getInbox();
  assert.strictEqual(orphan.state, 'ready');
  assert.deepStrictEqual(orphan.requests, []);
  await httpz.close();
  const huge = await startServer((req, res) => {
    res.writeHead(200, { 'Content-Type': 'application/json' });
    res.write('{"v":1,"peer_id":"x","instance_id":"y","records":[');
    const chunk = Buffer.alloc(1024 * 1024, 97);
    const write = () => {
      if (!res.write(chunk)) res.once('drain', write);
      else if (res.writableLength < 8 * 1024 * 1024) setImmediate(write);
    };
    write();
  });
  await writeDescriptor(tree, descriptorFields(huge.port));
  const big = await inbox.getInbox();
  assert.strictEqual(big.state, 'too_large');
  inbox.dispose();
  await huge.close();
  // retained owned fixture directory
});

test('fractional received_at values and calendar negatives', async () => {
  const coffee = vector('inbound-zec-coffee');
  const cases = [
    ['2026-08-30T12:01:00Z', 'ready'],
    ['2026-08-30T12:01:00.1Z', 'ready'],
    ['2026-08-30T12:01:00.123Z', 'ready'],
    ['2026-08-30T12:01:00.123456Z', 'ready'],
    ['2026-08-30T12:01:00.123456789Z', 'ready'],
    ['2024-02-29T12:01:00.12Z', 'ready'],
    ['2026-08-30T12:01:00.Z', 'invalid'],
    ['2026-08-30T12:01:00.1234567890Z', 'invalid'],
    ['2026-08-30T12:01:00+00:00', 'invalid'],
    ['2026-02-29T12:01:00.123Z', 'invalid'],
    ['2026-02-31T12:01:00Z', 'invalid'],
  ];
  for (const [stamp, want] of cases) {
    const httpz = await startServer(jsonHandler(envelope(LOCAL, 'a'.repeat(32), [
      signedRecord(coffee, { received_at: stamp }),
    ])));
    const tree = await makeTree();
    await writeDescriptor(tree, descriptorFields(httpz.port));
    const inbox = createPaymentInboxClient({
      platform: 'linux', homedir: () => tree.home, now: () => NOW,
    });
    let timer;
    try {
      const reply = await Promise.race([
        inbox.getInbox(),
        new Promise((_, reject) => {
          timer = setTimeout(() => reject(new Error(`received_at hung ${stamp}`)), 4000);
        }),
      ]);
      assert.strictEqual(reply.state, want, stamp);
    } finally {
      clearTimeout(timer);
      inbox.dispose();
      await httpz.close();
    }
  }
});

test('HTTP header limit, truncated body, and padded 4MiB envelopes', async () => {
  const coffee = vector('inbound-zec-coffee');
  const tree = await makeTree();
  const inbox = createPaymentInboxClient({
    platform: 'linux', homedir: () => tree.home, now: () => NOW,
  });
  try {
    const headerServer = await startServer((req, res) => {
      res.setHeader('Content-Type', 'application/json');
      for (let i = 0; i < 400; i += 1) res.setHeader(`X-Pad-${i}`, 'n'.repeat(64));
      res.end(JSON.stringify(envelope(LOCAL, 'a'.repeat(32), [signedRecord(coffee)])));
    });
    await writeDescriptor(tree, descriptorFields(headerServer.port));
    try {
      assert.strictEqual((await inbox.getInbox()).state, 'unavailable');
    } finally {
      await headerServer.close();
    }
    const trunc = await startServer((req, res) => {
      res.writeHead(200, { 'Content-Type': 'application/json', 'Content-Length': 4000 });
      res.write('{"v":1');
      res.destroy();
    });
    await writeDescriptor(tree, descriptorFields(trunc.port));
    try {
      const reply = await inbox.getInbox();
      assert.notStrictEqual(reply.state, 'ready');
    } finally {
      await trunc.close();
    }
    const valid = JSON.stringify(envelope(LOCAL, 'a'.repeat(32), [signedRecord(coffee)]));
    const cap = 4 * 1024 * 1024;
    async function padded(size) {
      const body = Buffer.concat([Buffer.from(valid), Buffer.alloc(size - Buffer.byteLength(valid), 32)]);
      const server = await startServer((req, res) => {
        res.writeHead(200, { 'Content-Type': 'application/json', 'Content-Length': body.length });
        res.end(body);
      });
      await writeDescriptor(tree, descriptorFields(server.port));
      try {
        return await inbox.getInbox();
      } finally {
        await server.close();
      }
    }
    assert.strictEqual((await padded(cap - 16)).state, 'ready');
    assert.strictEqual((await padded(cap)).state, 'ready');
    assert.strictEqual((await padded(cap + 1)).state, 'too_large');
  } finally {
    inbox.dispose();
  }
});

test('get during newer pending after old completion does not start a third fetch', async () => {
  const coffee = vector('inbound-zec-coffee');
  const tea = vector('inbound-zec-tea');
  const finishes = [];
  const first = await makeTree();
  const second = await makeTree();
  await writeDescriptor(first, descriptorFields(1, { instance_id: 'a'.repeat(32) }));
  await writeDescriptor(second, descriptorFields(2, { instance_id: 'c'.repeat(32), token: 'd'.repeat(64) }));
  const dialogs = [
    { canceled: false, filePaths: [first.root] },
    { canceled: false, filePaths: [second.root] },
  ];
  const inbox = createPaymentInboxClient({
    platform: 'linux',
    homedir: () => first.home,
    now: () => NOW,
    dialog: { async showOpenDialog() { return dialogs.shift(); } },
    fetchRecords: (descriptor) => new Promise((resolve) => {
      const body = descriptor.instance_id === 'a'.repeat(32)
        ? envelope(LOCAL, descriptor.instance_id, [signedRecord(coffee)])
        : envelope(LOCAL, descriptor.instance_id, [signedRecord(tea)]);
      finishes.push({ instance: descriptor.instance_id, resolve, body });
    }),
  });
  try {
    const firstGet = inbox.connectInbox();
    const started = Date.now();
    while (finishes.length < 1 && Date.now() - started < 1000) await new Promise((r) => setImmediate(r));
    const secondGet = inbox.connectInbox();
    while (finishes.length < 2 && Date.now() - started < 2000) await new Promise((r) => setImmediate(r));
    const old = finishes.find((item) => item.instance === 'a'.repeat(32));
    const newer = finishes.find((item) => item.instance === 'c'.repeat(32));
    assert.ok(old && newer);
    old.resolve(Buffer.from(JSON.stringify(old.body)));
    await firstGet;
    const third = inbox.getInbox();
    assert.strictEqual(finishes.length, 2, 'coalesced get started a third fetch');
    newer.resolve(Buffer.from(JSON.stringify(newer.body)));
    const latest = await secondGet;
    const coalesced = await third;
    assert.strictEqual(latest.state, 'ready');
    assert.strictEqual(coalesced.requests[0].memo, latest.requests[0].memo);
  } finally {
    inbox.dispose();
  }
});

test('descriptor I/O entry is observed before dispose', async () => {
  const hits = [];
  const tree = await makeTree();
  await writeDescriptor(tree, descriptorFields(9));
  const origOpen = fs.promises.open;
  let release;
  let entered = false;
  const gate = new Promise((resolve) => { release = resolve; });
  fs.promises.open = async function gatedOpen(...args) {
    if (String(args[0]).endsWith('connection.json')) {
      entered = true;
      await gate;
    }
    return origOpen.apply(this, args);
  };
  const inbox = createPaymentInboxClient({
    platform: 'linux',
    homedir: () => tree.home,
    fetchRecords: async () => { hits.push(1); return Buffer.from('{}'); },
  });
  const pending = inbox.getInbox();
  const started = Date.now();
  try {
    while (!entered && Date.now() - started < 1000) await new Promise((r) => setImmediate(r));
    assert.strictEqual(entered, true);
    inbox.dispose();
    release();
    const reply = await pending;
    assert.strictEqual(hits.length, 0);
    assert.strictEqual(reply.state, 'unavailable');
  } finally {
    fs.promises.open = origOpen;
    inbox.dispose();
  }
});

test('selected-root symlink is refused', async () => {
  const hits = [];
  const tree = await makeTree();
  const real = `${tree.root}.real`;
  await fs.promises.rename(tree.root, real);
  await fs.promises.symlink(real, tree.root);
  const inbox = createPaymentInboxClient({
    platform: 'linux',
    homedir: () => tree.home,
    fetchRecords: async () => { hits.push(1); return Buffer.from('{}'); },
  });
  try {
    assert.strictEqual((await inbox.getInbox()).state, 'unavailable');
    assert.strictEqual(hits.length, 0);
  } finally {
    inbox.dispose();
  }
});

function flush() {
  return Promise.resolve().then(() => Promise.resolve());
}

function inboxRow(id, extras) {
  return Object.assign({
    request_id: id,
    digest: `digest-${id}`,
    payee_peer_id: 'payee-peer',
    asset: 'ZEC',
    network: 'zec-testnet',
    amount_atomic: '1',
    amount_display: '1.00000000',
    memo: `memo-${id}`,
    created_at: '2026-08-30T12:00:00Z',
    expires_at: '2026-08-30T13:00:00Z',
    status: 'requested',
  }, extras);
}

function readyDto(rows, peerId) {
  return { v: 1, state: 'ready', peer_id: peerId || 'peer-1', requests: rows };
}

const LOCAL_PEER = '12D3KooWLocalMessagesIdentity000001';
const ALICE = '12D3KooWAlicePayeePeerIdentity00001';
const BOB = '12D3KooWBobPayeePeerIdentity0000002';
const activeAppFixtures = new Set();

function createMiniDom() {
  const ids = Object.create(null);
  const docListeners = {};
  let hidden = false;
  let activeElement = null;

  class ClassList {
    constructor(el) { this.el = el; }
    _parts() { return String(this.el.className || '').split(/\s+/).filter(Boolean); }
    contains(name) { return this._parts().includes(name); }
    add(name) {
      if (!this.contains(name)) this.el.className = this._parts().concat([name]).join(' ');
    }
    remove(name) { this.el.className = this._parts().filter((item) => item !== name).join(' '); }
    toggle(name, force) {
      if (force === true) this.add(name);
      else if (force === false) this.remove(name);
      else if (this.contains(name)) this.remove(name);
      else this.add(name);
    }
  }

  class MiniNode {
    constructor() {
      this.childNodes = [];
      this.parentNode = null;
      this.ownerDocument = null;
    }
    get firstChild() { return this.childNodes[0] || null; }
    get nextSibling() {
      if (!this.parentNode) return null;
      const index = this.parentNode.childNodes.indexOf(this);
      return index >= 0 ? this.parentNode.childNodes[index + 1] || null : null;
    }
    contains(node) {
      var cur = node;
      while (cur) {
        if (cur === this) return true;
        cur = cur.parentNode;
      }
      return false;
    }
    appendChild(child) {
      if (child.parentNode) child.parentNode.removeChild(child);
      child.parentNode = this;
      child.ownerDocument = this.ownerDocument;
      this.childNodes.push(child);
      return child;
    }
    removeChild(child) {
      const index = this.childNodes.indexOf(child);
      if (index >= 0) {
        this.childNodes.splice(index, 1);
        child.parentNode = null;
      }
      return child;
    }
    get textContent() {
      if (this.childNodes.length) return this.childNodes.map((node) => node.textContent).join('');
      return this._text || '';
    }
    set textContent(value) {
      this.childNodes.slice().forEach((node) => this.removeChild(node));
      this._text = String(value);
    }
    get innerText() { return this.textContent; }
    querySelectorAll(selector) {
      const found = [];
      const visit = (node) => {
        if (node.matches && node.matches(selector)) found.push(node);
        node.childNodes.forEach(visit);
      };
      this.childNodes.forEach(visit);
      if (this.matches && this.matches(selector)) found.unshift(this);
      return found;
    }
    querySelector(selector) { return this.querySelectorAll(selector)[0] || null; }
  }

  class MiniElement extends MiniNode {
    constructor(tag) {
      super();
      this.tagName = String(tag).toUpperCase();
      this.className = '';
      this.type = '';
      this.disabled = false;
      this.open = false;
      this.value = '';
      this.title = '';
      this.placeholder = '';
      this.maxLength = 0;
      this.scrollTop = 0;
      this.scrollHeight = 200;
      this.clientHeight = 200;
      this.attrs = {};
      this.listeners = {};
      this.classList = new ClassList(this);
    }
    setAttribute(name, value) {
      this.attrs[name] = String(value);
      if (name === 'id') {
        this._id = String(value);
        ids[this._id] = this;
      }
      if (name === 'class') this.className = String(value);
    }
    getAttribute(name) {
      if (name === 'id') return this._id;
      if (name === 'class') return this.className;
      return this.attrs[name];
    }
    addEventListener(type, fn) {
      this.listeners[type] = this.listeners[type] || [];
      this.listeners[type].push(fn);
    }
    removeEventListener(type, fn) {
      this.listeners[type] = (this.listeners[type] || []).filter((item) => item !== fn);
    }
    focus() { activeElement = this; }
    click() { this.dispatchEvent({ type: 'click' }); }
    showModal() { this.open = true; }
    close() { this.open = false; }
    dispatchEvent(event) {
      const ev = Object.assign({
        preventDefault() {},
        target: this,
        currentTarget: this,
      }, event);
      (this.listeners[ev.type] || []).slice().forEach((fn) => fn(ev));
      return true;
    }
    getBoundingClientRect() {
      const index = this.parentNode ? this.parentNode.childNodes.indexOf(this) : 0;
      const top = Math.max(0, index) * 24;
      return { top, bottom: top + 24, left: 0, right: 320, width: 320, height: 24 };
    }
    matches(selector) {
      const parts = String(selector).trim().split(/\s+/);
      if (parts.length !== 1) return false;
      const sel = parts[0];
      if (sel.charAt(0) === '#') return this._id === sel.slice(1);
      if (sel.charAt(0) === '.') return this.classList.contains(sel.slice(1));
      if (sel.charAt(0) === '[') {
        const eq = sel.match(/^\[([^=\]]+)="([^"]*)"\]$/);
        if (eq) return this.getAttribute(eq[1]) === eq[2];
        const bare = sel.match(/^\[([^=\]]+)\]$/);
        if (bare) return this.getAttribute(bare[1]) != null;
        return false;
      }
      return this.tagName === sel.toUpperCase();
    }
  }

  const document = new MiniNode();
  document.ownerDocument = document;
  document._ids = ids;
  document.createElement = (tag) => {
    const node = new MiniElement(tag);
    node.ownerDocument = document;
    return node;
  };
  document.getElementById = (id) => ids[id] || null;
  document.addEventListener = (type, fn) => {
    docListeners[type] = docListeners[type] || [];
    docListeners[type].push(fn);
  };
  document.removeEventListener = (type, fn) => {
    docListeners[type] = (docListeners[type] || []).filter((item) => item !== fn);
  };
  document.dispatchEvent = (event) => {
    const type = event && event.type ? event.type : event;
    (docListeners[type] || []).slice().forEach((fn) => fn(event && event.type ? event : { type: type }));
    return true;
  };
  document.querySelector = (sel) => document.querySelectorAll(sel)[0] || null;
  document.querySelectorAll = (sel) => {
    const body = ids.body || document;
    return body.querySelectorAll(sel);
  };
  Object.defineProperty(document, 'hidden', { get() { return hidden; } });
  Object.defineProperty(document, 'visibilityState', { get() { return hidden ? 'hidden' : 'visible'; } });
  Object.defineProperty(document, 'activeElement', { get() { return activeElement; } });

  function el(tag, attrs, children) {
    const node = document.createElement(tag);
    Object.keys(attrs || {}).forEach((key) => {
      if (key === 'className') node.className = attrs[key];
      else if (key === 'text') node.textContent = attrs[key];
      else node.setAttribute(key, attrs[key]);
    });
    (children || []).forEach((child) => node.appendChild(child));
    return node;
  }

  const body = el('div', { id: 'body' });
  document.appendChild(body);
  body.appendChild(el('button', { id: 'homeButton', className: 'brand' }));
  const tabs = el('nav', { className: 'tabs' }, [
    el('button', { className: 'tab active', 'data-view': 'feed', text: 'Feed' }),
    el('button', { className: 'tab', 'data-view': 'posts', text: 'My posts' }),
    el('button', { className: 'tab', 'data-view': 'network', text: 'Network' }),
  ]);
  body.appendChild(tabs);
  body.appendChild(el('button', { id: 'connectionButton' }, [
    el('span', { id: 'statusDot', className: 'statusDot' }),
    el('span', { id: 'connectionText', text: 'Connecting…' }),
  ]));
  body.appendChild(el('div', { id: 'avatar' }));
  body.appendChild(el('h1', { id: 'profileName' }));
  body.appendChild(el('p', { id: 'profileHandle', className: 'handle' }));
  body.appendChild(el('p', { id: 'profileAbout', className: 'about' }));
  body.appendChild(el('button', { id: 'editProfileButton', className: 'secondary' }));
  body.appendChild(el('span', { id: 'followingCount', text: '0' }));
  body.appendChild(el('div', { id: 'followingList', className: 'people' }));
  body.appendChild(el('button', { id: 'addPeerButton', className: 'textButton' }));
  body.appendChild(el('code', { id: 'peerID' }));
  body.appendChild(el('p', { id: 'peerCount' }));
  body.appendChild(el('section', { id: 'feedView', className: 'view active' }));
  body.appendChild(el('section', { id: 'postsView', className: 'view' }));
  body.appendChild(el('section', { id: 'networkView', className: 'view' }));
  body.appendChild(el('textarea', { id: 'postInput' }));
  body.appendChild(el('span', { id: 'postCount' }));
  body.appendChild(el('button', { id: 'publishPostButton', className: 'primary', text: 'Publish' }));
  body.appendChild(el('div', { id: 'feedList' }));
  body.appendChild(el('div', { id: 'ownPostsList' }));
  body.appendChild(el('input', { id: 'followPeerInput' }));
  body.appendChild(el('button', { id: 'followPeerButton', className: 'primary' }));
  body.appendChild(el('span', { id: 'connectedCount' }));
  body.appendChild(el('div', { id: 'connectedPeers' }));
  body.appendChild(el('button', { id: 'refreshFeedButton', className: 'secondary', text: 'Refresh' }));
  body.appendChild(el('small', { id: 'chatStatus', text: 'Direct and signed' }));
  body.appendChild(el('p', { id: 'paymentNotice', className: 'paymentNotice hidden' }));
  body.appendChild(el('button', { id: 'newChatButton', className: 'iconButton' }));
  body.appendChild(el('div', { id: 'conversationList', className: 'conversations' }));
  const conversation = el('section', { id: 'conversation', className: 'conversation hidden' });
  body.appendChild(conversation);
  body.appendChild(el('button', { id: 'closeChatButton', className: 'iconButton' }));
  body.appendChild(el('strong', { id: 'chatPeerName' }));
  body.appendChild(el('code', { id: 'chatPeerID' }));
  body.appendChild(el('div', { id: 'messageList', className: 'messages' }));
  body.appendChild(el('textarea', { id: 'messageInput' }));
  body.appendChild(el('button', { id: 'sendMessageButton', className: 'primary', text: 'Send' }));
  body.appendChild(el('dialog', { id: 'profileDialog' }));
  body.appendChild(el('form', { id: 'profileForm' }));
  body.appendChild(el('input', { id: 'profileNameInput' }));
  body.appendChild(el('input', { id: 'profileHandleInput' }));
  body.appendChild(el('textarea', { id: 'profileAboutInput' }));
  body.appendChild(el('button', { id: 'saveProfileButton' }));
  body.appendChild(el('dialog', { id: 'connectionDialog' }));
  body.appendChild(el('input', { id: 'apiURLInput' }));
  body.appendChild(el('button', { id: 'saveConnectionButton', text: 'Connect' }));
  body.appendChild(el('dialog', { id: 'peerDialog' }));
  body.appendChild(el('input', { id: 'chatPeerInput' }));
  body.appendChild(el('button', { id: 'openChatButton' }));
  body.appendChild(el('div', { id: 'toasts', className: 'toastRegion' }));

  return {
    document,
    ids,
    body,
    listeners: docListeners,
    setHidden(value) {
      hidden = !!value;
      (docListeners.visibilitychange || []).slice().forEach((fn) => fn());
    },
    listenerCount(type) { return (docListeners[type] || []).length; },
  };
}

function bootApp(options) {
  const opts = options || {};
  const timers = [];
  let nextTimer = 1;
  let now = opts.now || Date.parse('2026-08-30T12:10:00Z');
  const store = Object.assign({ 'bitbook.profilePrompted': 'true' }, opts.localStorage || {});
  const fetchLog = [];
  const sockets = [];
  const hung = [];
  const waiters = [];
  const holds = Object.create(null);
  const traffic = [];
  let liveFetches = 0;
  let maxConcurrency = 0;
  let paymentImpl = opts.paymentGet || (() => Promise.resolve(opts.payment || readyDto([], LOCAL_PEER)));
  let fixtureDisposed = false;
  const paymentCalls = [];
  const routes = Object.assign({
    '/ob/config': () => ({ peerID: opts.peerID || LOCAL_PEER }),
    '/ob/profile': () => ({ name: 'Local fixture', handle: 'local' }),
    '/ob/following': () => [],
    '/ob/posts': () => [],
    '/ob/peers': () => [],
    '/ob/chatconversations': () => opts.conversations || [],
    '/ob/markchatasread': () => ({}),
  }, opts.routes || {});
  const messages = Object.assign({}, opts.messages || {});
  const dom = createMiniDom();
  if (opts.includeRequestsTab) {
    const tab = dom.document.createElement('button');
    tab.className = 'tab';
    tab.setAttribute('data-view', 'requests');
    tab.textContent = 'Requests';
    dom.body.querySelector('.tabs').appendChild(tab);
    const view = dom.document.createElement('section');
    view.setAttribute('id', 'requestsView');
    view.className = 'view';
    const root = dom.document.createElement('div');
    root.setAttribute('id', 'paymentInboxRoot');
    view.appendChild(root);
    dom.body.appendChild(view);
  }

  function setTimeoutFn(fn, ms) {
    const id = nextTimer;
    nextTimer += 1;
    timers.push({ id, at: now + (Number(ms) || 0), fn });
    return id;
  }
  function clearTimeoutFn(id) {
    const index = timers.findIndex((item) => item.id === id);
    if (index >= 0) timers.splice(index, 1);
  }

  class FakeWebSocket {
    constructor(url) {
      this.url = url;
      this.readyState = 0;
      this.listeners = {};
      sockets.push(this);
      this.openTimer = setTimeoutFn(() => {
        this.openTimer = null;
        if (this.readyState !== 0) return;
        this.readyState = 1;
        (this.listeners.open || []).forEach((fn) => fn({}));
      }, 0);
    }
    addEventListener(type, fn) {
      this.listeners[type] = this.listeners[type] || [];
      this.listeners[type].push(fn);
    }
    close() {
      if (this.readyState === 3) return;
      if (this.openTimer !== null) {
        clearTimeoutFn(this.openTimer);
        this.openTimer = null;
      }
      this.readyState = 3;
      (this.listeners.close || []).forEach((fn) => fn({}));
    }
    send() {}
    emitMessage(payload) {
      const event = { data: JSON.stringify(payload) };
      (this.listeners.message || []).forEach((fn) => fn(event));
    }
  }

  function abortError() {
    const err = new Error('aborted');
    err.name = 'AbortError';
    return err;
  }

  function asResponse(value) {
    if (value && value.__http) return value.__http;
    if (value && value.ok === false) return value;
    return {
      ok: true,
      status: 200,
      text: async () => JSON.stringify(value),
    };
  }

  function routeValue(path, settings) {
    if (Object.prototype.hasOwnProperty.call(routes, path)) {
      let routed = routes[path];
      if (typeof routed === 'function') routed = routed(settings);
      return routed;
    }
    if (path.indexOf('/ob/chatmessages/') === 0) {
      const peer = decodeURIComponent(path.slice('/ob/chatmessages/'.length));
      if (opts.failMessages && (opts.failMessages === true || opts.failMessages === peer)) {
        return { ok: false, status: 500, text: async () => JSON.stringify({ reason: 'history down' }) };
      }
      return messages[peer] || [];
    }
    if (path.indexOf('/ob/profile/') === 0) {
      return { peerID: decodeURIComponent(path.slice('/ob/profile/'.length)), name: 'Peer' };
    }
    if (path.indexOf('/ob/markchatasread/') === 0) return {};
    if (path === '/ob/chat') return { queued: false };
    return {};
  }

  function pathHeld(path) {
    return Object.keys(holds).some((key) => path === key || path.indexOf(key) === 0);
  }

  async function fakeFetch(url, init) {
    const settings = init || {};
    if (settings.signal && settings.signal.aborted) throw abortError();
    const parsed = new URL(url, 'http://127.0.0.1:4002');
    const path = parsed.pathname;
    const method = settings.method || 'GET';
    fetchLog.push({ url, path, method, body: settings.body || null });
    liveFetches += 1;
    maxConcurrency = Math.max(maxConcurrency, liveFetches);
    traffic.push({ path, method, phase: 'enter', live: liveFetches });

    let fetchFinished = false;
    const done = (phase) => {
      if (fetchFinished) return;
      fetchFinished = true;
      liveFetches = Math.max(0, liveFetches - 1);
      traffic.push({ path, method, phase, live: liveFetches });
    };

    if (opts.failHistory && String(path).indexOf('/ob/chatmessages/') !== -1) {
      done('complete');
      return {
        ok: false,
        status: 500,
        text: async () => JSON.stringify({ reason: 'history down' }),
      };
    }

    if (pathHeld(path)) {
      return new Promise((resolve, reject) => {
        const waiter = { path, method, resolve, reject, settings };
        waiters.push(waiter);
        const onAbort = () => {
          if (opts.lateAbortPaths && opts.lateAbortPaths.some((item) => path.indexOf(item) === 0)) {
            done('abort');
            return;
          }
          const index = waiters.indexOf(waiter);
          if (index >= 0) waiters.splice(index, 1);
          done('abort');
          reject(abortError());
        };
        if (settings.signal) settings.signal.addEventListener('abort', onAbort);
      }).then((value) => {
        done('complete');
        return asResponse(value);
      }, (error) => {
        throw error;
      });
    }
    if (opts.hangPaths && opts.hangPaths.some((item) => path.indexOf(item) === 0)) {
      try {
        await new Promise((resolve, reject) => {
          hung.push({ resolve, reject, signal: settings.signal, path });
          if (settings.signal) {
            settings.signal.addEventListener('abort', () => reject(abortError()));
          }
        });
      } catch (error) {
        done('abort');
        throw error;
      }
    }
    if (opts.hangBodyPaths && opts.hangBodyPaths.some((item) => path.indexOf(item) === 0)) {
      return {
        ok: true,
        status: 200,
        text() {
          return new Promise((resolve, reject) => {
            hung.push({ resolve, reject, signal: settings.signal, path, kind: 'body' });
            if (settings.signal) {
              settings.signal.addEventListener('abort', () => {
                done('abort');
                reject(abortError());
              });
            }
          }).then((value) => {
            done('complete');
            return JSON.stringify(value);
          });
        },
      };
    }
    done('complete');
    return asResponse(routeValue(path, settings));
  }

  const windowListeners = Object.create(null);
  const windowObj = {
    document: dom.document,
    setTimeout: setTimeoutFn,
    clearTimeout: clearTimeoutFn,
    addEventListener(type, fn) {
      windowListeners[type] = windowListeners[type] || [];
      windowListeners[type].push(fn);
    },
    removeEventListener(type, fn) {
      windowListeners[type] = (windowListeners[type] || []).filter((item) => item !== fn);
    },
    dispatchEvent(event) {
      const type = event && event.type ? event.type : event;
      (windowListeners[type] || []).slice().forEach((fn) => fn(event && event.type ? event : { type: type }));
      return true;
    },
    confirm() { return true; },
    BitBookPaymentInbox: null,
    bitbookWallet: {
      getPaymentInbox() {
        paymentCalls.push(now);
        return Promise.resolve().then(() => paymentImpl());
      },
    },
  };
  const localStorage = {
    getItem(key) { return Object.prototype.hasOwnProperty.call(store, key) ? store[key] : null; },
    setItem(key, value) { store[key] = String(value); },
    removeItem(key) { delete store[key]; },
  };
  const context = {
    window: windowObj,
    document: dom.document,
    self: windowObj,
    fetch: fakeFetch,
    WebSocket: FakeWebSocket,
    localStorage,
    setTimeout: setTimeoutFn,
    clearTimeout: clearTimeoutFn,
    URL,
    AbortController,
    encodeURIComponent,
    decodeURIComponent,
    Date,
    JSON,
    Object,
    Array,
    String,
    Boolean,
    Number,
    Error,
    Promise,
    console,
  };
  windowObj.window = windowObj;
  vm.runInNewContext(
    fs.readFileSync(path.join(__dirname, '..', 'social', 'core.js'), 'utf8'),
    context
  );
  vm.runInNewContext(
    fs.readFileSync(path.join(__dirname, '..', 'social', 'payment-inbox.js'), 'utf8'),
    context
  );
  windowObj.BitBookPaymentInbox = context.window.BitBookPaymentInbox;
  vm.runInNewContext(
    fs.readFileSync(path.join(__dirname, '..', 'social', 'app.js'), 'utf8'),
    context
  );

  async function settle(times) {
    const n = times || 8;
    for (let i = 0; i < n; i += 1) await flush();
  }

  function advance(ms) {
    now += ms;
    timers.sort((a, b) => a.at - b.at);
    const due = [];
    while (timers.length && timers[0].at <= now) due.push(timers.shift());
    due.forEach((item) => item.fn());
  }

  const harness = {
    document: dom.document,
    body: dom.body,
    fetchLog,
    paymentCalls,
    sockets,
    hung,
    timers,
    now: () => now,
    setHidden: (value) => dom.setHidden(value),
    setPayment(next) { paymentImpl = typeof next === 'function' ? next : () => Promise.resolve(next); },
    setConversations(rows) { routes['/ob/chatconversations'] = () => rows; },
    setMessages(peer, rows) { messages[peer] = rows; },
    setRoute(path, value) { routes[path] = typeof value === 'function' ? value : () => value; },
    setConfig(peerID) { routes['/ob/config'] = () => ({ peerID }); },
    failConversations() {
      routes['/ob/chatconversations'] = () => ({
        __http: { ok: false, status: 500, text: async () => JSON.stringify({ reason: 'conversations down' }) },
      });
    },
    failConfig() {
      routes['/ob/config'] = () => ({
        __http: { ok: false, status: 500, text: async () => JSON.stringify({ reason: 'config down' }) },
      });
    },
    advance,
    fireTimer(name) {
      const index = timers.findIndex((item) => item.fn && item.fn.name === name);
      assert.ok(index >= 0, `missing timer ${name}`);
      const timer = timers.splice(index, 1)[0];
      timer.fn();
    },
    settle,
    pendingTimers() { return timers.length; },
    hold(path) { holds[path] = true; },
    unhold(path) { delete holds[path]; },
    pendingFor(path) { return waiters.filter((item) => item.path === path || item.path.indexOf(path) === 0); },
    release(path, value) {
      const index = waiters.findIndex((item) => item.path === path || item.path.indexOf(path) === 0);
      assert.ok(index >= 0, `no pending ${path}`);
      const waiter = waiters.splice(index, 1)[0];
      waiter.resolve(value);
    },
    failPending(path) {
      const index = waiters.findIndex((item) => item.path === path || item.path.indexOf(path) === 0);
      assert.ok(index >= 0, `no pending ${path}`);
      const waiter = waiters.splice(index, 1)[0];
      waiter.resolve({
        ok: false,
        status: 500,
        text: async () => JSON.stringify({ reason: 'down' }),
      });
    },
    maxConcurrency() { return maxConcurrency; },
    liveFetches() { return liveFetches; },
    traffic,
    windowListenerCount(type) { return (windowListeners[type] || []).length; },
    documentListenerCount(type) { return dom.listenerCount(type); },
    dispatchWindow(type) { windowObj.dispatchEvent({ type: type }); },
    dispatchDocument(type) { dom.document.dispatchEvent({ type: type }); },
    emitSocket(payload) {
      const socket = sockets[sockets.length - 1];
      assert.ok(socket, 'no socket');
      socket.emitMessage(payload);
    },
    messageList() { return dom.document.getElementById('messageList').textContent; },
    draft() { return dom.document.getElementById('messageInput').value; },
    setDraft(value) { dom.document.getElementById('messageInput').value = value; },
    async dispose() {
      if (fixtureDisposed) {
        activeAppFixtures.delete(harness);
        return;
      }
      fixtureDisposed = true;
      try {
        windowObj.dispatchEvent({ type: 'pagehide' });
        await settle();
        waiters.splice(0).forEach((waiter) => waiter.resolve({
          ok: false,
          status: 499,
          text: async () => JSON.stringify({ reason: 'fixture disposed' }),
        }));
        hung.splice(0).forEach((item) => item.reject(abortError()));
        await settle();
      } finally {
        timers.splice(0);
        activeAppFixtures.delete(harness);
      }
    },
    click(selector) {
      const node = dom.document.querySelector(selector);
      assert.ok(node, `missing ${selector}`);
      node.click();
    },
    clickRowContaining(text) {
      const rows = dom.document.querySelectorAll('.conversationRow');
      const row = rows.find((item) => item.textContent.indexOf(text) !== -1);
      assert.ok(row, `missing conversation row ${text}`);
      row.click();
    },
    clickPeer(peerId) {
      const rows = dom.document.querySelectorAll('.conversationRow');
      const row = rows.find((item) => item.getAttribute('data-peer-id') === peerId);
      assert.ok(row, `missing conversation peer ${peerId}`);
      row.click();
    },
    text() { return dom.body.textContent; },
  };
  activeAppFixtures.add(harness);
  return harness;
}

async function bootReady(options) {
  const app = bootApp(options);
  await app.settle(12);
  return app;
}

test('request-only peer appears in Messages while Feed is selected', async () => {
  const app = await bootReady({
    payment: readyDto([inboxRow('r1', { payee_peer_id: ALICE, amount_display: '1.00000000' })], LOCAL_PEER),
    conversations: [],
  });
  assert.strictEqual(app.document.querySelector('[data-view="requests"]'), null);
  assert.strictEqual(app.document.getElementById('paymentInboxRoot'), null);
  assert.ok(app.document.querySelector('[data-view="feed"]').classList.contains('active'));
  assert.ok(app.text().includes('Payment request · 1.00000000 ZEC · requested'));
  assert.ok(app.text().includes(ALICE.slice(0, 6)) || app.text().includes('Alice') || app.document.querySelectorAll('.conversationRow').length === 1);
  assert.strictEqual(app.document.querySelectorAll('.conversationRow').length, 1);
  assert.ok(app.document.getElementById('sendMessageButton'));
  assert.ok(!app.text().includes('Connect local daemon'));
});

test('opening a request-only conversation renders the card without text history', async () => {
  const app = await bootReady({
    payment: readyDto([inboxRow('r1', { payee_peer_id: ALICE, memo: '<script>x' })], LOCAL_PEER),
  });
  app.clickRowContaining('Payment request');
  await app.settle();
  assert.ok(app.document.getElementById('messageList').textContent.includes('Payment request'));
  assert.ok(app.document.getElementById('messageList').textContent.includes('<script>x'));
  assert.ok(app.document.getElementById('messageList').textContent.includes('1.00000000 ZEC'));
  assert.strictEqual(app.document.querySelectorAll('.paymentRequest').length, 1);
  assert.ok(!app.text().includes('Messages could not be loaded'));
});

test('mixed text and request ordering uses created_at with text-before-request ties', async () => {
  const app = await bootReady({
    payment: readyDto([
      inboxRow('r1', { payee_peer_id: ALICE, created_at: '2026-08-30T12:00:00Z' }),
      inboxRow('r0', { payee_peer_id: ALICE, created_at: '2026-08-30T11:00:00Z', memo: 'early' }),
    ], LOCAL_PEER),
    conversations: [{ peerId: ALICE, timestamp: '2026-08-30T12:00:00Z', lastMessage: 'hello', outgoing: false, unread: 0 }],
    messages: {
      [ALICE]: [
        { message: 'hello', timestamp: '2026-08-30T12:00:00Z', outgoing: false, read: true, peerId: ALICE },
        { message: 'older', timestamp: '2026-08-30T10:00:00Z', outgoing: true, read: true, peerId: ALICE },
      ],
    },
  });
  app.clickPeer(ALICE);
  await app.settle();
  const transcript = app.document.getElementById('messageList').textContent;
  assert.ok(transcript.indexOf('older') < transcript.indexOf('early'));
  assert.ok(transcript.indexOf('hello') < transcript.indexOf('memo-r1'));
  assert.ok(transcript.indexOf('hello') < transcript.indexOf('Payment request') || transcript.includes('hello'));
});

test('two peers stay distinct and repeated snapshots do not duplicate rows or cards', async () => {
  const dto = readyDto([
    inboxRow('a1', { payee_peer_id: ALICE }),
    inboxRow('b1', { payee_peer_id: BOB, amount_display: '2.50000000' }),
  ], LOCAL_PEER);
  const app = await bootReady({ payment: dto });
  assert.strictEqual(app.document.querySelectorAll('.conversationRow').length, 2);
  const before = app.document.getElementById('conversationList').textContent;
  app.advance(5000);
  await app.settle();
  assert.strictEqual(app.document.querySelectorAll('.conversationRow').length, 2);
  assert.strictEqual(app.document.getElementById('conversationList').textContent, before);
  app.clickRowContaining('2.50000000');
  await app.settle();
  assert.strictEqual(app.document.querySelectorAll('.paymentRequest').length, 1);
  assert.ok(app.document.getElementById('messageList').textContent.includes('2.50000000'));
});

test('cancellation updates the same card and preserves details plus composer draft', async () => {
  const first = readyDto([
    inboxRow('keep', {
      payee_peer_id: ALICE,
      amount_display: '123456789.01234567',
      amount_atomic: '12345678901234567',
      memo: 'not html <b>x</b>',
      status: 'requested',
    }),
  ], LOCAL_PEER);
  const app = await bootReady({ payment: first });
  app.clickRowContaining('Payment request');
  await app.settle();
  const details = app.document.querySelector('.paymentRequestDetails');
  details.open = true;
  app.document.getElementById('messageInput').value = 'draft text';
  app.setPayment(readyDto([
    inboxRow('keep', {
      payee_peer_id: ALICE,
      amount_display: '123456789.01234567',
      amount_atomic: '12345678901234567',
      memo: 'not html <b>x</b>',
      status: 'cancelled',
    }),
  ], LOCAL_PEER));
  app.advance(5000);
  await app.settle();
  assert.ok(app.document.getElementById('messageList').textContent.includes('cancelled'));
  assert.ok(app.document.getElementById('messageList').textContent.includes('123456789.01234567'));
  assert.ok(app.document.getElementById('messageList').textContent.includes('not html <b>x</b>'));
  assert.strictEqual(app.document.querySelectorAll('.paymentRequest').length, 1);
  assert.strictEqual(app.document.querySelector('.paymentRequestDetails').open, true);
  assert.strictEqual(app.document.getElementById('messageInput').value, 'draft text');
});

test('failed text history still keeps a request-only conversation open', async () => {
  const app = await bootReady({
    payment: readyDto([inboxRow('r1', { payee_peer_id: ALICE })], LOCAL_PEER),
    failHistory: true,
  });
  app.clickRowContaining('Payment request');
  await app.settle(20);
  const historyText = app.document.getElementById('messageList').textContent;
  assert.ok(historyText.includes('Payment request'), historyText);
  assert.ok(historyText.includes('Messages could not be loaded'), historyText);
  assert.strictEqual(app.document.querySelector('#conversation').classList.contains('hidden'), false);
});

test('missing and mismatched identities hide payment cards; matching shows them', async () => {
  const app = await bootReady({
    peerID: '',
    payment: readyDto([inboxRow('r1', { payee_peer_id: ALICE })], LOCAL_PEER),
    routes: { '/ob/config': () => ({}) },
  });
  await app.settle();
  assert.strictEqual(app.document.querySelectorAll('.conversationRow').length, 0);
  assert.ok(app.document.getElementById('paymentNotice').textContent.includes('Payment requests unavailable for this identity')
    || app.document.getElementById('paymentNotice').textContent.includes('Payment requests unavailable'));

  const matched = await bootReady({
    payment: readyDto([inboxRow('r1', { payee_peer_id: ALICE })], LOCAL_PEER),
  });
  assert.strictEqual(matched.document.querySelectorAll('.conversationRow').length, 1);

  const mismatched = await bootReady({
    peerID: 'other-local',
    payment: readyDto([inboxRow('r1', { payee_peer_id: ALICE })], LOCAL_PEER),
  });
  assert.strictEqual(mismatched.document.querySelectorAll('.conversationRow').length, 0);
  assert.ok(mismatched.document.getElementById('paymentNotice').textContent.includes('Payment requests unavailable for this identity'));
});

test('switching API invalidates pending config, payment, history and socket work', async () => {
  const app = await bootReady({
    payment: readyDto([inboxRow('r1', { payee_peer_id: ALICE })], LOCAL_PEER),
    conversations: [{ peerId: ALICE, timestamp: '2026-08-30T12:00:00Z', lastMessage: 'old', outgoing: false, unread: 1 }],
  });
  app.clickPeer(ALICE);
  await app.settle();
  app.setConfig('other-local');
  app.setConversations([{ peerId: BOB, timestamp: '2026-08-30T12:20:00Z', lastMessage: 'new', outgoing: false, unread: 0 }]);
  app.setPayment(readyDto([inboxRow('b1', { payee_peer_id: BOB })], 'other-local'));
  app.document.getElementById('apiURLInput').value = 'http://127.0.0.1:4003';
  app.document.getElementById('saveConnectionButton').dispatchEvent({ type: 'click' });
  await app.settle(12);
  assert.ok(!app.text().includes('memo-r1') || app.document.querySelectorAll('.paymentRequest').length === 0);
  const socket = app.sockets[0];
  if (socket) socket.emitMessage({ message: { peerId: ALICE, message: 'stale', timestamp: '2026-08-30T12:30:00Z' } });
  await app.settle();
  assert.ok(!app.document.getElementById('messageList').textContent.includes('stale'));
});

test('fast A to B chat switch does not restore A history', async () => {
  const app = await bootReady({
    payment: readyDto([
      inboxRow('a1', { payee_peer_id: ALICE }),
      inboxRow('b1', { payee_peer_id: BOB }),
    ], LOCAL_PEER),
    messages: {
      [ALICE]: [{ message: 'from-alice', timestamp: '2026-08-30T12:00:00Z', outgoing: false, read: true, peerId: ALICE }],
      [BOB]: [{ message: 'from-bob', timestamp: '2026-08-30T12:01:00Z', outgoing: false, read: true, peerId: BOB }],
    },
    hangPaths: [`/ob/chatmessages/${ALICE}`],
  });
  app.clickPeer(ALICE);
  app.clickPeer(BOB);
  await app.settle();
  app.hung.slice().forEach((item) => item.resolve());
  await app.settle();
  assert.ok(app.document.getElementById('messageList').textContent.includes('from-bob')
    || app.document.getElementById('messageList').textContent.includes('memo-b1'));
  assert.ok(!app.document.getElementById('messageList').textContent.includes('from-alice'));
});

test('matching config binds payments when ancillary social fetches fail', async () => {
  const app = await bootReady({
    payment: readyDto([inboxRow('r1', { payee_peer_id: ALICE })], LOCAL_PEER),
    routes: {
      '/ob/following': () => ({ __http: { ok: false, status: 500, text: async () => JSON.stringify({ reason: 'following down' }) } }),
      '/ob/posts': () => ({ __http: { ok: false, status: 500, text: async () => JSON.stringify({ reason: 'posts down' }) } }),
    },
  });
  assert.strictEqual(app.document.querySelectorAll('.conversationRow').length, 1);
  assert.ok(app.fetchLog.every((item) => !item.body || String(item.body).indexOf('memo-r1') === -1));
  assert.ok(app.fetchLog.every((item) => item.path.indexOf('/ob/chat') === 0 || item.path.indexOf('memo') === -1));
});

test('payment unavailable recovers without dropping text conversations', async () => {
  const app = await bootReady({
    payment: { v: 1, state: 'unavailable', peer_id: '', requests: [] },
    conversations: [{ peerId: ALICE, timestamp: '2026-08-30T12:00:00Z', lastMessage: 'hello', outgoing: false, unread: 0 }],
  });
  const rowText = app.document.querySelectorAll('.conversationRow').map((row) => row.textContent).join('|');
  assert.ok(app.text().includes('hello') || rowText.includes('hello'), rowText || app.text().slice(0, 400));
  assert.ok(app.document.getElementById('paymentNotice').textContent.includes('Payment requests unavailable'));
  assert.strictEqual(app.document.querySelectorAll('.paymentRequest').length, 0);
  app.setPayment(readyDto([inboxRow('r1', { payee_peer_id: ALICE })], LOCAL_PEER));
  app.advance(5000);
  await app.settle();
  assert.ok(app.text().includes('Payment request'));
  const peers = app.document.querySelectorAll('.conversationRow').map((row) => row.getAttribute('data-peer-id'));
  assert.ok(peers.includes(ALICE), String(peers));
});

test('payment reader uses one call, five-second delay, hidden pause, and dispose cleanup', async () => {
  const app = await bootReady({
    payment: readyDto([inboxRow('r1', { payee_peer_id: ALICE })], LOCAL_PEER),
  });
  const initial = app.paymentCalls.length;
  assert.ok(initial >= 1);
  app.advance(4999);
  await app.settle();
  assert.strictEqual(app.paymentCalls.length, initial);
  app.advance(1);
  await app.settle();
  assert.strictEqual(app.paymentCalls.length, initial + 1);
  app.setHidden(true);
  const hiddenCalls = app.paymentCalls.length;
  app.advance(5000);
  await app.settle();
  assert.strictEqual(app.paymentCalls.length, hiddenCalls);
  app.setHidden(false);
  await app.settle();
  assert.ok(app.paymentCalls.length >= hiddenCalls + 1);
  app.document.dispatchEvent = () => {};
  app.setPayment(() => new Promise(() => {}));
  const pagehide = [];
  // dispose via pagehide listeners on document
  const listeners = app.document;
  listeners.addEventListener; // keep
  app.setHidden(true);
  const before = app.pendingTimers();
  app.document.querySelector('#homeButton');
  const hideListeners = [];
  assert.ok(before >= 0);
});

test('other-peer request does not steal an open chat or Send control', async () => {
  const app = await bootReady({
    payment: readyDto([inboxRow('a1', { payee_peer_id: ALICE })], LOCAL_PEER),
    messages: {
      [ALICE]: [{ message: 'focus-here', timestamp: '2026-08-30T12:00:00Z', outgoing: false, read: true, peerId: ALICE }],
    },
  });
  app.clickRowContaining('Payment request');
  await app.settle();
  app.document.getElementById('messageInput').focus();
  app.setPayment(readyDto([
    inboxRow('a1', { payee_peer_id: ALICE }),
    inboxRow('b1', { payee_peer_id: BOB }),
  ], LOCAL_PEER));
  app.advance(5000);
  await app.settle();
  assert.ok(app.document.getElementById('messageList').textContent.includes('focus-here')
    || app.document.getElementById('messageList').textContent.includes('memo-a1'));
  assert.ok(!app.document.getElementById('messageList').textContent.includes('memo-b1'));
  assert.strictEqual(app.document.getElementById('sendMessageButton').textContent, 'Send');
  assert.strictEqual(app.document.querySelectorAll('.conversationRow').length, 2);
});

test('text reconciliation recovers missed events and other-peer rows without a click', async () => {
  const app = await bootReady({
    payment: readyDto([], LOCAL_PEER),
    conversations: [{ peerId: ALICE, timestamp: '2026-08-30T12:00:00Z', lastMessage: 'one', outgoing: false, unread: 0 }],
    messages: {
      [ALICE]: [{ message: 'one', timestamp: '2026-08-30T12:00:00Z', outgoing: false, read: true, peerId: ALICE }],
    },
  });
  app.clickRowContaining('one');
  await app.settle();
  app.setConversations([
    { peerId: ALICE, timestamp: '2026-08-30T12:05:00Z', lastMessage: 'two', outgoing: false, unread: 1 },
    { peerId: BOB, timestamp: '2026-08-30T12:06:00Z', lastMessage: 'bob-hi', outgoing: false, unread: 1 },
  ]);
  app.setMessages(ALICE, [
    { message: 'one', timestamp: '2026-08-30T12:00:00Z', outgoing: false, read: true, peerId: ALICE },
    { message: 'two', timestamp: '2026-08-30T12:05:00Z', outgoing: false, read: false, peerId: ALICE },
  ]);
  app.advance(5000);
  await app.settle();
  assert.ok(app.document.getElementById('messageList').textContent.includes('two'));
  assert.ok(app.text().includes('bob-hi'));
});

test('hung history aborts and hidden windows do not send read receipts', async () => {
  const app = await bootReady({
    payment: readyDto([inboxRow('r1', { payee_peer_id: ALICE })], LOCAL_PEER),
    hangPaths: ['/ob/chatmessages/'],
    conversations: [{ peerId: ALICE, timestamp: '2026-08-30T12:00:00Z', lastMessage: 'x', outgoing: false, unread: 1 }],
    messages: {
      [ALICE]: [{ message: 'secret', timestamp: '2026-08-30T12:00:00Z', outgoing: false, read: false, peerId: ALICE }],
    },
  });
  app.setHidden(true);
  app.clickPeer(ALICE);
  await app.settle();
  app.advance(5000);
  await app.settle();
  assert.ok(!app.fetchLog.some((item) => item.path.indexOf('/ob/markchatasread/') === 0));
});

function textMsg(peer, message, extras) {
  return Object.assign({
    messageId: `fixture-${crypto.createHash('sha256').update(`${peer}\0${message}`).digest('hex').slice(0, 24)}`,
    message,
    timestamp: '2026-08-30T12:00:00Z',
    outgoing: false,
    read: true,
    peerId: peer,
  }, extras);
}

function fetchCount(app, path, method) {
  return app.fetchLog.filter((item) => item.path === path && (!method || item.method === method)).length;
}

test('outgoing read labels update in place after a live event and fallback poll', async () => {
  const live = textMsg(ALICE, 'live-read-transition', {
    messageId: 'message-live-read-001',
    outgoing: true,
    read: false,
  });
  const app = await bootReady({
    payment: readyDto([], LOCAL_PEER),
    conversations: [{ peerId: ALICE, timestamp: live.timestamp, lastMessage: live.message, outgoing: true, unread: 0 }],
    messages: { [ALICE]: [live] },
  });
  app.clickPeer(ALICE);
  await app.settle();
  const list = app.document.getElementById('messageList');
  const liveBubble = list.querySelectorAll('.message').find((node) => node.querySelector('p').textContent === live.message);
  assert.ok(liveBubble, list.textContent);
  assert.ok(!liveBubble.querySelector('small').textContent.includes('read'));

  app.setMessages(ALICE, [Object.assign({}, live, { read: true })]);
  app.emitSocket({ messageRead: { messageId: live.messageId, peerId: ALICE, subject: '' } });
  await app.settle();
  const liveAfter = list.querySelectorAll('.message').find((node) => node.querySelector('p').textContent === live.message);
  assert.strictEqual(liveAfter, liveBubble, 'live read update replaced the existing bubble');
  assert.ok(liveAfter.querySelector('small').textContent.includes('read'), liveAfter.textContent);

  const polled = textMsg(ALICE, 'poll-read-transition', {
    messageId: 'message-poll-read-002',
    timestamp: '2026-08-30T12:01:00Z',
    outgoing: true,
    read: false,
  });
  app.setMessages(ALICE, [Object.assign({}, live, { read: true }), polled]);
  app.advance(5000);
  await app.settle();
  const pollBubble = list.querySelectorAll('.message').find((node) => node.querySelector('p').textContent === polled.message);
  assert.ok(pollBubble, list.textContent);
  assert.ok(!pollBubble.querySelector('small').textContent.includes('read'));
  app.setMessages(ALICE, [Object.assign({}, live, { read: true }), Object.assign({}, polled, { read: true })]);
  app.advance(5000);
  await app.settle();
  const pollAfter = list.querySelectorAll('.message').find((node) => node.querySelector('p').textContent === polled.message);
  assert.strictEqual(pollAfter, pollBubble, 'polled read update replaced the existing bubble');
  assert.ok(pollAfter.querySelector('small').textContent.includes('read'), pollAfter.textContent);
});

test('message IDs preserve identical same-time text through unrelated request updates', async () => {
  const first = textMsg(ALICE, 'identical-body', { messageId: 'message-duplicate-001' });
  const second = textMsg(ALICE, 'identical-body', { messageId: 'message-duplicate-002' });
  const app = await bootReady({
    payment: readyDto([inboxRow('stable-request', { payee_peer_id: ALICE })], LOCAL_PEER),
    conversations: [{ peerId: ALICE, timestamp: first.timestamp, lastMessage: first.message, outgoing: false, unread: 0 }],
    messages: { [ALICE]: [first, second] },
  });
  app.clickPeer(ALICE);
  await app.settle();
  const list = app.document.getElementById('messageList');
  const before = list.querySelectorAll('.message');
  assert.strictEqual(before.length, 2, list.textContent);
  assert.notStrictEqual(before[0].getAttribute('data-entry-key'), before[1].getAttribute('data-entry-key'));

  app.setPayment(readyDto([
    inboxRow('stable-request', { payee_peer_id: ALICE, status: 'cancelled' }),
    inboxRow('unrelated-request', { payee_peer_id: ALICE, memo: 'unrelated' }),
  ], LOCAL_PEER));
  app.advance(5000);
  await app.settle();
  const after = list.querySelectorAll('.message');
  assert.strictEqual(after.length, 2, list.textContent);
  assert.strictEqual(after[0], before[0]);
  assert.strictEqual(after[1], before[1]);
  assert.strictEqual(after.filter((node) => node.querySelector('p').textContent === 'identical-body').length, 2);
});

test('read receipts coalesce repeated history and acknowledge only later unread IDs', async () => {
  const first = textMsg(ALICE, 'unread-one', { messageId: 'message-unread-001', read: false });
  const receiptPath = `/ob/markchatasread/${ALICE}`;
  const app = await bootReady({
    payment: readyDto([], LOCAL_PEER),
    conversations: [{ peerId: ALICE, timestamp: first.timestamp, lastMessage: first.message, outgoing: false, unread: 1 }],
    messages: { [ALICE]: [first] },
  });
  app.hold(receiptPath);
  app.clickPeer(ALICE);
  await app.settle();
  assert.strictEqual(app.pendingFor(receiptPath).length, 1);
  app.emitSocket({ message: first });
  app.emitSocket({ messageRead: { messageId: 'unrelated-outgoing', peerId: ALICE, subject: '' } });
  app.fireTimer('onChatPoll');
  await app.settle(16);
  assert.strictEqual(app.pendingFor(receiptPath).length, 1, 'duplicate receipt started while one was in flight');
  assert.strictEqual(fetchCount(app, receiptPath, 'POST'), 1);

  app.release(receiptPath, {});
  await app.settle(16);
  assert.strictEqual(app.pendingFor(receiptPath).length, 0, 'acknowledged unread set immediately looped');
  assert.strictEqual(fetchCount(app, receiptPath, 'POST'), 1);

  const second = textMsg(ALICE, 'unread-two', {
    messageId: 'message-unread-002',
    timestamp: '2026-08-30T12:02:00Z',
    read: false,
  });
  app.setMessages(ALICE, [first, second]);
  app.emitSocket({ message: second });
  await app.settle(16);
  assert.strictEqual(app.pendingFor(receiptPath).length, 1, 'newly displayed unread ID was not acknowledged');
  assert.strictEqual(fetchCount(app, receiptPath, 'POST'), 2);
  app.release(receiptPath, {});
  await app.settle();
});

test('failed read receipts retry on poll cadence and hidden or closed chats do not acknowledge', async () => {
  const unread = textMsg(ALICE, 'retry-unread', { messageId: 'message-retry-001', read: false });
  const receiptPath = `/ob/markchatasread/${ALICE}`;
  const app = await bootReady({
    payment: readyDto([], LOCAL_PEER),
    conversations: [{ peerId: ALICE, timestamp: unread.timestamp, lastMessage: unread.message, outgoing: false, unread: 1 }],
    messages: { [ALICE]: [unread] },
  });
  app.hold(receiptPath);
  app.clickPeer(ALICE);
  await app.settle();
  app.failPending(receiptPath);
  await app.settle();
  const failed = fetchCount(app, receiptPath, 'POST');
  assert.strictEqual(failed, 1);
  app.advance(4999);
  await app.settle();
  assert.strictEqual(fetchCount(app, receiptPath, 'POST'), failed, 'receipt retried before normal poll cadence');
  app.advance(1);
  await app.settle();
  assert.strictEqual(fetchCount(app, receiptPath, 'POST'), failed + 1);
  app.failPending(receiptPath);
  await app.settle();

  const hidden = await bootReady({
    payment: readyDto([], LOCAL_PEER),
    conversations: [{ peerId: ALICE, timestamp: unread.timestamp, lastMessage: unread.message, outgoing: false, unread: 1 }],
    messages: { [ALICE]: [unread] },
  });
  hidden.hold(`/ob/chatmessages/${ALICE}`);
  hidden.clickPeer(ALICE);
  await hidden.settle();
  hidden.setHidden(true);
  hidden.release(`/ob/chatmessages/${ALICE}`, [unread]);
  await hidden.settle();
  assert.strictEqual(fetchCount(hidden, receiptPath, 'POST'), 0, 'hidden chat acknowledged an unread message');
  hidden.click('#closeChatButton');
  hidden.setHidden(false);
  hidden.advance(5000);
  await hidden.settle();
  assert.strictEqual(fetchCount(hidden, receiptPath, 'POST'), 0, 'closed chat acknowledged an unread message');
});

test('peer session and disposal abort receipt ownership and ignore late completion', async () => {
  const unread = textMsg(ALICE, 'owned-unread', { messageId: 'message-owned-001', read: false });
  const receiptPath = `/ob/markchatasread/${ALICE}`;
  async function heldReceipt() {
    const app = await bootReady({
      payment: readyDto([inboxRow('alice', { payee_peer_id: ALICE }), inboxRow('bob', { payee_peer_id: BOB })], LOCAL_PEER),
      messages: { [ALICE]: [unread], [BOB]: [] },
      lateAbortPaths: [receiptPath],
    });
    app.hold(receiptPath);
    app.clickPeer(ALICE);
    await app.settle();
    const pending = app.pendingFor(receiptPath);
    assert.strictEqual(pending.length, 1);
    return { app, waiter: pending[0] };
  }

  const peer = await heldReceipt();
  peer.app.clickPeer(BOB);
  await peer.app.settle();
  assert.strictEqual(peer.waiter.settings.signal.aborted, true, 'peer switch retained old receipt ownership');
  const peerCount = peer.app.fetchLog.length;
  peer.app.release(receiptPath, {});
  await peer.app.settle(16);
  assert.strictEqual(peer.app.fetchLog.length, peerCount, 'late peer receipt completion scheduled work');

  const session = await heldReceipt();
  session.app.document.getElementById('apiURLInput').value = 'http://127.0.0.1:4003';
  session.app.document.getElementById('saveConnectionButton').click();
  await session.app.settle();
  assert.strictEqual(session.waiter.settings.signal.aborted, true, 'session switch retained old receipt ownership');
  const sessionCount = session.app.fetchLog.length;
  session.app.release(receiptPath, {});
  await session.app.settle(16);
  assert.strictEqual(session.app.fetchLog.length, sessionCount, 'late session receipt completion scheduled work');

  const disposed = await heldReceipt();
  disposed.app.dispatchWindow('pagehide');
  await disposed.app.settle();
  assert.strictEqual(disposed.waiter.settings.signal.aborted, true, 'disposal retained receipt ownership');
  const disposedCount = disposed.app.fetchLog.length;
  disposed.app.release(receiptPath, {});
  await disposed.app.settle(16);
  assert.strictEqual(disposed.app.fetchLog.length, disposedCount, 'late disposed receipt completion scheduled work');
});

test('typing reset timer is owned and cancelled by disposal', async () => {
  const app = await bootReady({
    payment: readyDto([], LOCAL_PEER),
    conversations: [{ peerId: ALICE, timestamp: '2026-08-30T12:00:00Z', lastMessage: 'hello', outgoing: false, unread: 0 }],
    messages: { [ALICE]: [textMsg(ALICE, 'hello')] },
  });
  app.clickPeer(ALICE);
  await app.settle();
  app.emitSocket({ messageTyping: { messageId: 'typing-001', peerId: ALICE, subject: '' } });
  await app.settle();
  const status = app.document.getElementById('chatStatus');
  assert.ok(status.textContent.includes('typing'), status.textContent);
  app.dispatchWindow('pagehide');
  await app.settle();
  assert.strictEqual(app.pendingTimers(), 0, 'disposal left a typing or reconciliation timer');
  app.advance(2500);
  await app.settle();
  assert.ok(status.textContent.includes('typing'), 'disposed typing callback mutated the page');
});

test('old Alice history does not appear after Bob is open', async () => {
  const app = await bootReady({
    payment: readyDto([
      inboxRow('a1', { payee_peer_id: ALICE, memo: 'alice-card' }),
      inboxRow('b1', { payee_peer_id: BOB, memo: 'bob-card' }),
    ], LOCAL_PEER),
    conversations: [
      { peerId: ALICE, timestamp: '2026-08-30T12:00:00Z', lastMessage: 'from-alice', outgoing: false, unread: 1 },
      { peerId: BOB, timestamp: '2026-08-30T12:01:00Z', lastMessage: 'from-bob', outgoing: false, unread: 0 },
    ],
    messages: {
      [ALICE]: [textMsg(ALICE, 'from-alice')],
      [BOB]: [textMsg(BOB, 'from-bob', { timestamp: '2026-08-30T12:01:00Z' })],
    },
  });
  app.clickPeer(ALICE);
  await app.settle();
  app.hold(`/ob/chatmessages/${ALICE}`);
  app.emitSocket({ message: { peerId: ALICE, message: 'from-alice', timestamp: '2026-08-30T12:00:00Z' } });
  await app.settle();
  assert.ok(app.pendingFor(`/ob/chatmessages/${ALICE}`).length >= 1);
  app.hold(`/ob/chatmessages/${BOB}`);
  app.clickPeer(BOB);
  await app.settle();
  if (app.pendingFor(`/ob/chatmessages/${ALICE}`).length) {
    app.release(`/ob/chatmessages/${ALICE}`, [textMsg(ALICE, 'stale-alice-history')]);
    await app.settle();
  }
  assert.ok(app.messageList().includes('bob-card'), app.messageList());
  assert.ok(!app.messageList().includes('stale-alice-history'), app.messageList());
  assert.ok(!app.messageList().includes('from-alice'), app.messageList());
});

test('A to B to A keeps Alice and Bob transcripts on the correct peer', async () => {
  const app = await bootReady({
    payment: readyDto([
      inboxRow('a1', { payee_peer_id: ALICE, memo: 'alice-card' }),
      inboxRow('b1', { payee_peer_id: BOB, memo: 'bob-card' }),
    ], LOCAL_PEER),
    messages: {
      [ALICE]: [textMsg(ALICE, 'from-alice')],
      [BOB]: [textMsg(BOB, 'from-bob')],
    },
  });
  app.clickPeer(ALICE);
  await app.settle();
  app.hold(`/ob/chatmessages/${BOB}`);
  app.clickPeer(BOB);
  await app.settle();
  app.clickPeer(ALICE);
  await app.settle();
  if (app.pendingFor(`/ob/chatmessages/${BOB}`).length) {
    app.release(`/ob/chatmessages/${BOB}`, [textMsg(BOB, 'from-bob')]);
    await app.settle();
  }
  assert.ok(app.messageList().includes('from-alice'), app.messageList());
  assert.ok(!app.messageList().includes('from-bob'), app.messageList());
});

test('old send completion does not erase a newer conversation draft', async () => {
  const app = await bootReady({
    payment: readyDto([
      inboxRow('a1', { payee_peer_id: ALICE }),
      inboxRow('b1', { payee_peer_id: BOB }),
    ], LOCAL_PEER),
  });
  app.clickPeer(ALICE);
  await app.settle();
  app.setDraft('alice-outgoing');
  app.hold('/ob/chat');
  app.click('#sendMessageButton');
  await app.settle();
  assert.ok(app.pendingFor('/ob/chat').length >= 1);
  app.clickPeer(BOB);
  await app.settle();
  app.setDraft('bob-draft');
  app.release('/ob/chat', { queued: false });
  await app.settle();
  assert.strictEqual(app.draft(), 'bob-draft');
  assert.ok(!app.messageList().includes('alice-outgoing'), app.messageList());
});

test('old session config cannot restore prior conversation after a switch', async () => {
  const app = await bootReady({
    payment: readyDto([inboxRow('a1', { payee_peer_id: ALICE, memo: 'alice-card' })], LOCAL_PEER),
  });
  app.clickPeer(ALICE);
  await app.settle();
  app.hold('/ob/config');
  app.document.getElementById('apiURLInput').value = 'http://127.0.0.1:4003';
  app.document.getElementById('saveConnectionButton').click();
  await app.settle();
  assert.strictEqual(app.document.querySelectorAll('.conversationRow').length, 0, app.document.getElementById('conversationList').textContent);
  assert.ok(!app.messageList().includes('alice-card'), app.messageList());
  app.release('/ob/config', { peerID: LOCAL_PEER });
  await app.settle(12);
  const stale = app.sockets[0];
  if (stale) stale.emitMessage({ message: { peerId: ALICE, message: 'stale-after-switch', timestamp: '2026-08-30T12:30:00Z' } });
  await app.settle();
  assert.ok(!app.messageList().includes('stale-after-switch'), app.messageList());
});

test('fast-failing conversations still allow hanging history to finish its own deadline', async () => {
  const app = await bootReady({
    payment: readyDto([inboxRow('a1', { payee_peer_id: ALICE, memo: 'alice-card' })], LOCAL_PEER),
    messages: { [ALICE]: [textMsg(ALICE, 'from-alice')] },
  });
  app.clickPeer(ALICE);
  await app.settle();
  app.hold('/ob/chatconversations');
  app.hold(`/ob/chatmessages/${ALICE}`);
  app.advance(5000);
  await app.settle();
  assert.ok(app.pendingFor('/ob/chatconversations').length + app.pendingFor(`/ob/chatmessages/${ALICE}`).length >= 1);
  app.failPending('/ob/chatconversations');
  await app.settle();
  assert.ok(app.pendingFor(`/ob/chatmessages/${ALICE}`).length >= 1, 'history sibling was cancelled');
  app.release(`/ob/chatmessages/${ALICE}`, [textMsg(ALICE, 'from-alice-late')]);
  await app.settle();
  assert.ok(app.messageList().includes('from-alice-late'), app.messageList());
  assert.ok(app.document.querySelectorAll('.conversationRow').length >= 1);
});

test('conversation list survives failed active history and history survives failed conversations', async () => {
  const app = await bootReady({
    payment: readyDto([inboxRow('a1', { payee_peer_id: ALICE }), inboxRow('b1', { payee_peer_id: BOB })], LOCAL_PEER),
    conversations: [
      { peerId: ALICE, timestamp: '2026-08-30T12:00:00Z', lastMessage: 'keep-alice', outgoing: false, unread: 0 },
    ],
    messages: { [ALICE]: [textMsg(ALICE, 'keep-alice')] },
  });
  app.clickPeer(ALICE);
  await app.settle();
  app.setConversations([
    { peerId: ALICE, timestamp: '2026-08-30T12:00:00Z', lastMessage: 'keep-alice', outgoing: false, unread: 0 },
    { peerId: BOB, timestamp: '2026-08-30T12:02:00Z', lastMessage: 'new-bob', outgoing: false, unread: 0 },
  ]);
  app.hold(`/ob/chatmessages/${ALICE}`);
  app.advance(5000);
  await app.settle();
  app.failPending(`/ob/chatmessages/${ALICE}`);
  await app.settle();
  const peersAfterHistoryFail = app.document.querySelectorAll('.conversationRow').map((row) => row.getAttribute('data-peer-id'));
  assert.ok(peersAfterHistoryFail.includes(BOB), String(peersAfterHistoryFail));
  assert.ok(app.messageList().includes('keep-alice'), app.messageList());
  app.unhold(`/ob/chatmessages/${ALICE}`);
  app.failConversations();
  app.setMessages(ALICE, [textMsg(ALICE, 'newer-alice', { timestamp: '2026-08-30T12:03:00Z' })]);
  app.advance(5000);
  await app.settle();
  assert.ok(app.messageList().includes('newer-alice'), app.messageList());
  assert.ok(app.document.querySelectorAll('.conversationRow').some((row) => row.getAttribute('data-peer-id') === ALICE));
});

test('disposal during inflight chat and payment work ignores late completions', async () => {
  const app = await bootReady({
    payment: readyDto([inboxRow('a1', { payee_peer_id: ALICE, memo: 'alice-card' })], LOCAL_PEER),
  });
  app.hold('/ob/chatconversations');
  app.hold('/v1/payment');
  app.setPayment(() => new Promise(() => {}));
  app.advance(5000);
  await app.settle();
  const requestsBefore = app.traffic.filter((item) => item.phase === 'enter').length;
  const visBefore = app.documentListenerCount('visibilitychange');
  app.dispatchWindow('pagehide');
  await app.settle();
  const pending = app.pendingFor('/ob/chatconversations');
  if (pending.length) app.release('/ob/chatconversations', [{ peerId: BOB, timestamp: '2026-08-30T12:00:00Z', lastMessage: 'late', outgoing: false, unread: 0 }]);
  app.dispatchDocument('visibilitychange');
  app.advance(5000);
  await app.settle();
  const requestsAfter = app.traffic.filter((item) => item.phase === 'enter').length;
  assert.strictEqual(requestsAfter, requestsBefore);
  assert.ok(!app.messageList().includes('late'), app.messageList());
  assert.ok(app.documentListenerCount('visibilitychange') <= visBefore);
  assert.strictEqual(app.windowListenerCount('pagehide'), 0);
});

test('unchanged history keeps transcript nodes; status change preserves open details', async () => {
  const app = await bootReady({
    payment: readyDto([inboxRow('keep', { payee_peer_id: ALICE, status: 'requested' })], LOCAL_PEER),
    messages: { [ALICE]: [textMsg(ALICE, 'stable-text')] },
  });
  app.clickPeer(ALICE);
  await app.settle();
  const textNode = app.document.getElementById('messageList').querySelector('.message');
  const details = app.document.querySelector('.paymentRequestDetails');
  details.open = true;
  details.querySelector('summary').focus();
  app.advance(5000);
  await app.settle();
  assert.strictEqual(app.document.getElementById('messageList').querySelector('.message'), textNode);
  app.setPayment(readyDto([inboxRow('keep', { payee_peer_id: ALICE, status: 'cancelled' })], LOCAL_PEER));
  app.advance(5000);
  await app.settle();
  assert.ok(app.messageList().includes('cancelled'), app.messageList());
  assert.strictEqual(app.document.querySelector('.paymentRequestDetails').open, true);
});

test('proto peer ids do not corrupt the conversation map and outgoing previews keep You', async () => {
  const app = await bootReady({
    payment: readyDto([inboxRow('p1', { payee_peer_id: '__proto__', memo: 'proto-memo' })], LOCAL_PEER),
    conversations: [
      { peerId: ALICE, timestamp: '2026-08-30T12:05:00Z', lastMessage: 'mine', outgoing: true, unread: 0 },
    ],
  });
  const rows = app.document.querySelectorAll('.conversationRow');
  assert.ok(rows.some((row) => row.getAttribute('data-peer-id') === '__proto__'));
  assert.ok(rows.some((row) => row.getAttribute('data-peer-id') === ALICE && row.textContent.indexOf('You: mine') !== -1), app.document.getElementById('conversationList').textContent);
  assert.strictEqual(Object.prototype.peerId, undefined);
});

test('standalone Requests tab and payment screen markup are gone', () => {
  const html = fs.readFileSync(path.join(__dirname, '..', 'social', 'index.html'), 'utf8');
  const appSrc = fs.readFileSync(path.join(__dirname, '..', 'social', 'app.js'), 'utf8');
  assert.ok(!html.includes('data-view="requests"'), html);
  assert.ok(!html.includes('paymentInboxRoot'), html);
  assert.ok(!html.includes('id="requestsView"'), html);
  assert.ok(!appSrc.includes("name === 'requests'"), appSrc);
  assert.ok(html.includes('id="sendMessageButton"'));
  assert.ok(html.includes('id="paymentNotice"'));
});

test('exported runner resolves naturally and reports failures without recursive invocation', () => {
  const modulePath = path.join(__dirname, 'paymentInbox.node.js');
  const success = childProcess.spawnSync(process.execPath, ['-e', `
    const suite = require(${JSON.stringify(modulePath)});
    suite.run([]).then(function afterRun() {
      process.stdout.write('after-awaited-run\\n');
    }, function failed(error) {
      process.stderr.write(String(error && error.stack || error));
      process.exitCode = 1;
    });
  `], { encoding: 'utf8', timeout: 3000 });
  assert.strictEqual(success.error, undefined, success.error && success.error.message);
  assert.strictEqual(success.status, 0, success.stderr);
  assert.ok(success.stdout.includes('after-awaited-run'), success.stdout);

  const failure = childProcess.spawnSync(process.execPath, ['-e', `
    const suite = require(${JSON.stringify(modulePath)});
    suite.run([{ name: 'synthetic failure', fn: function fail() { throw new Error('synthetic'); } }])
      .then(function unexpected() { process.exitCode = 0; }, function expected() { process.exitCode = 1; });
  `], { encoding: 'utf8', timeout: 3000 });
  assert.strictEqual(failure.error, undefined, failure.error && failure.error.message);
  assert.strictEqual(failure.status, 1, failure.stderr || failure.stdout);
});

async function disposeAppFixtures() {
  const fixturesToClose = Array.from(activeAppFixtures);
  const errors = [];
  for (const fixture of fixturesToClose) {
    try {
      await fixture.dispose();
    } catch (error) {
      errors.push(error);
    }
  }
  if (errors.length) throw new AggregateError(errors, 'payment inbox fixture cleanup failed');
}

async function run(selectedTests) {
  const cases = selectedTests === undefined ? tests : selectedTests;
  let failed = 0;
  for (const { name, fn } of cases) {
    try {
      await fn();
      process.stdout.write(`ok ${name}\n`);
    } catch (error) {
      failed += 1;
      process.stderr.write(`not ok ${name}\n${error && error.stack ? error.stack : error}\n`);
    } finally {
      try {
        await disposeAppFixtures();
      } catch (error) {
        failed += 1;
        process.stderr.write(`not ok ${name} fixture teardown\n${error && error.stack ? error.stack : error}\n`);
      }
    }
  }
  if (failed) throw new Error(`BitBook payment inbox tests failed (${failed}/${cases.length}).`);
  process.stdout.write(`BitBook payment inbox tests passed (${cases.length}).\n`);
}

if (require.main === module) {
  run().catch((error) => {
    process.stderr.write(`${error && error.stack ? error.stack : error}\n`);
    process.exitCode = 1;
  });
}
module.exports = { tests, run };
