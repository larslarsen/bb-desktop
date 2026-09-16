'use strict';

const assert = require('assert');
const crypto = require('crypto');
const fs = require('fs');
const http = require('http');
const Module = require('module');
const path = require('path');
const { URL, pathToFileURL } = require('url');
const fixtures = require('./fixtures/payment-inbox/records-v1.json');

if (!process.versions.electron) {
  process.stderr.write('paymentInbox.electron.js must run under Electron\n');
  process.exit(1);
}

const { app, BrowserWindow, session } = require('electron');

const ARTIFACT_DIR = process.env.BBD_PAY001_ARTIFACT_DIR;
if (typeof ARTIFACT_DIR !== 'string' || ARTIFACT_DIR.length === 0 || ARTIFACT_DIR.includes('\0') ||
    ARTIFACT_DIR.split(/[\\/]/).includes('..')) {
  process.stderr.write('BBD_PAY001_ARTIFACT_DIR is missing or unsafe\n');
  process.exit(1);
}

const repoRoot = path.join(__dirname, '..');
const artifactRoot = path.join(repoRoot, ARTIFACT_DIR);
fs.mkdirSync(artifactRoot, { recursive: true });
const appDocumentPath = path.join(repoRoot, 'social', 'index.html');
const appDocumentURL = pathToFileURL(appDocumentPath).href;
const bootstrapDiagnosticsPath = path.join(artifactRoot, 'bootstrap-diagnostics.json');
const uiFailureDiagnosticsPath = path.join(artifactRoot, 'ui-failure-diagnostics.json');

const KEY = Buffer.alloc(32, 7).toString('base64');
const SIG = Buffer.alloc(32, 9).toString('base64');
const LOCAL = '12D3KooWQeyPaymentInboxPayerPeerIdentity0001';
const PAYEE = '12D3KooWQeyPaymentInboxPayeePeerIdentity0002';
const OTHER = '12D3KooWQeyPaymentInboxOtherPeerIdentity0003';
const FIXED_NOW = Date.parse('2026-08-30T12:10:00Z');
const blocked = [];
const socialHits = [];
const paymentHits = [];
const fixtureFailures = [];
const blockerCounts = { allowed: 0, expected: 0, unexpected: 0 };
const blockerRecent = [];
const owned = { server: null, social: null, bootstrapTrace: null, patches: [] };
const bootstrapStartedAt = Date.now();
const bootstrap = {
  phase: 'created',
  events: [],
  loadError: null,
  rendererTermination: null,
  win: null,
  outcome: '',
};
let uiFailureStage = 'startup';
let uiAssertionLabel = null;

function vector(name) {
  return fixtures.vectors.find((item) => item.name === name);
}

function rewritePeer(canonical, from, to) {
  return canonical.split(from).join(to);
}

function recordFrom(name, extras = {}) {
  const item = vector(name);
  let canonical = rewritePeer(item.canonical, fixtures.local_peer_id, LOCAL);
  canonical = rewritePeer(canonical, fixtures.payee_peer_id, PAYEE);
  if (extras.payee) canonical = rewritePeer(canonical, PAYEE, extras.payee);
  if (extras.memo) {
    canonical = canonical.replace(`"memo":"${JSON.parse(item.canonical).memo}"`, `"memo":"${extras.memo}"`);
  }
  const domain = item.kind === 'status' ? 'bitbook-payment-status-v1\n' : 'bitbook-payment-request-v1\n';
  const digest = crypto.createHash('sha256').update(domain).update(canonical).digest('hex');
  return {
    signed: {
      version: 1,
      kind: item.kind,
      canonical,
      public_key: KEY,
      signature: SIG,
    },
    digest,
    direction: item.direction,
    received_at: extras.received_at || '2026-08-30T12:01:00.123456789Z',
  };
}

function waitUntil(check, timeout, label) {
  const start = Date.now();
  return new Promise((resolve, reject) => {
    const tick = async () => {
      try {
        const value = await check();
        if (value) return resolve(value);
      } catch (error) {
        if (Date.now() - start > timeout) return reject(error);
      }
      if (Date.now() - start > timeout) return reject(new Error(label || 'timeout'));
      setTimeout(tick, 40);
    };
    tick();
  });
}

function readStatus(pid) {
  const text = fs.readFileSync(`/proc/${pid}/status`, 'utf8');
  const field = (name) => {
    const line = text.split('\n').find((row) => row.startsWith(`${name}:`));
    return line ? line.slice(name.length + 1).trim() : '';
  };
  return {
    pid,
    Seccomp: field('Seccomp'),
    NoNewPrivs: field('NoNewPrivs'),
    NSpid: field('NSpid'),
    Uid: field('Uid'),
  };
}

function cleanup() {
  if (owned.bootstrapTrace) {
    try { owned.bootstrapTrace(); } catch (_) { /* owned */ }
    owned.bootstrapTrace = null;
  }
  try { if (owned.server) owned.server.close(); } catch (_) { /* owned */ }
  try { if (owned.social) owned.social.close(); } catch (_) { /* owned */ }
  for (const win of BrowserWindow.getAllWindows()) {
    try { win.close(); } catch (_) { /* owned */ }
  }
  for (const undo of owned.patches) {
    try { undo(); } catch (_) { /* owned */ }
  }
}

process.on('exit', cleanup);

const home = fs.mkdtempSync(path.join(artifactRoot, 'home-'));
const userData = fs.mkdtempSync(path.join(artifactRoot, 'user-'));
const root = path.join(home, '.bitbook', 'modern');
const privateDir = path.join(root, 'local-client');
fs.mkdirSync(privateDir, { recursive: true, mode: 0o700 });
fs.chmodSync(privateDir, 0o700);

const longMemo = 'fixture-memo-'.padEnd(180, 'm');
let injectedNow = FIXED_NOW;
let socialPeerId = LOCAL;
let conversations = [];
let chatMessages = {};

function longCoffeeRecord() {
  const row = recordFrom('inbound-zec-coffee');
  row.signed.canonical = row.signed.canonical.replace('"memo":"coffee"', `"memo":"${longMemo}"`);
  row.digest = crypto.createHash('sha256')
    .update('bitbook-payment-request-v1\n')
    .update(row.signed.canonical)
    .digest('hex');
  return row;
}

let payloadRecords = [];
let payload = {
  v: 1,
  peer_id: LOCAL,
  instance_id: 'a'.repeat(32),
  records: payloadRecords,
};

function setPayloadRecords(records) {
  payloadRecords = records;
  payload = {
    v: 1,
    peer_id: LOCAL,
    instance_id: 'a'.repeat(32),
    records: payloadRecords,
  };
}

const server = http.createServer((req, res) => {
  const hit = { method: req.method, url: req.url, headers: req.headers, status: null };
  paymentHits.push(hit);
  res.once('finish', () => { hit.status = res.statusCode; });
  if (req.method !== 'GET' || req.url !== '/v1/payment/records' || req.headers.origin !== undefined ||
      req.headers.authorization !== `Bearer ${'b'.repeat(64)}` ||
      req.headers['x-bitbook-instance'] !== 'a'.repeat(32)) {
    fixtureFailures.push({ fixture: 'payment', method: req.method, url: req.url, status: 404, headers: req.headers });
    res.writeHead(404);
    res.end();
    return;
  }
  const body = Buffer.from(JSON.stringify(payload));
  res.writeHead(200, { 'Content-Type': 'application/json', 'Content-Length': body.length });
  res.end(body);
});
owned.server = server;

function corsHeaders(method) {
  return {
    'Access-Control-Allow-Origin': 'null',
    'Access-Control-Allow-Methods': method,
    'Access-Control-Allow-Headers': 'Content-Type',
    'Access-Control-Allow-Private-Network': 'true',
    Vary: 'Origin',
  };
}

function json(res, value, method, status) {
  const body = Buffer.from(JSON.stringify(value));
  res.writeHead(status || 200, Object.assign({
    'Content-Type': 'application/json',
    'Content-Length': body.length,
  }, corsHeaders(method)));
  res.end(body);
}

function socialMethod(pathname) {
  if (['/ob/config', '/ob/profile', '/ob/following', '/ob/posts', '/ob/peers', '/ob/chatconversations'].includes(pathname)) {
    return 'GET';
  }
  const peers = [PAYEE, OTHER];
  if (peers.some((peer) => pathname === `/ob/profile/${encodeURIComponent(peer)}` ||
      pathname === `/ob/chatmessages/${encodeURIComponent(peer)}`)) return 'GET';
  if (peers.some((peer) => pathname === `/ob/markchatasread/${encodeURIComponent(peer)}`)) return 'POST';
  if (pathname === '/ob/chat') return 'POST';
  return '';
}

const social = http.createServer((req, res) => {
  const parsed = new URL(req.url, 'http://127.0.0.1');
  const requiredMethod = parsed.search === '' ? socialMethod(parsed.pathname) : '';
  const hit = { method: req.method, path: parsed.pathname, origin: req.headers.origin || '', status: null };
  socialHits.push(hit);
  res.once('finish', () => { hit.status = res.statusCode; });
  const allowedOrigin = req.headers.origin === 'null' ||
    (req.headers.origin === undefined && req.method === requiredMethod &&
      (requiredMethod === 'GET' || requiredMethod === 'POST'));
  if (!allowedOrigin || !requiredMethod) {
    fixtureFailures.push({ fixture: 'social', reason: 'origin-or-path', method: req.method, path: parsed.pathname, origin: req.headers.origin, status: 403 });
    res.writeHead(403);
    res.end();
    return;
  }
  if (req.method === 'OPTIONS') {
    const requested = req.headers['access-control-request-method'];
    const requestedHeaders = String(req.headers['access-control-request-headers'] || '')
      .split(',').map((value) => value.trim().toLowerCase()).filter(Boolean);
    if (requested !== requiredMethod || (requiredMethod !== 'POST' && requiredMethod !== 'GET') ||
        requestedHeaders.some((value) => value !== 'content-type')) {
      fixtureFailures.push({ fixture: 'social', reason: 'preflight-method', method: req.method, path: parsed.pathname, origin: req.headers.origin, status: 405, requested, requestedHeaders });
      res.writeHead(405);
      res.end();
      return;
    }
    res.writeHead(204, corsHeaders(requiredMethod));
    res.end();
    return;
  }
  if (req.method !== requiredMethod) {
    fixtureFailures.push({ fixture: 'social', reason: 'method', method: req.method, path: parsed.pathname, origin: req.headers.origin, status: 405 });
    res.writeHead(405, corsHeaders(requiredMethod));
    res.end();
    return;
  }
  if (parsed.pathname === '/ob/config') return json(res, { peerID: socialPeerId }, requiredMethod);
  if (parsed.pathname === '/ob/profile') return json(res, { name: 'Fixture user', handle: 'fixture' }, requiredMethod);
  if (parsed.pathname === '/ob/following') return json(res, [], requiredMethod);
  if (parsed.pathname === '/ob/posts') return json(res, [], requiredMethod);
  if (parsed.pathname === '/ob/peers') return json(res, [], requiredMethod);
  if (parsed.pathname === '/ob/chatconversations') return json(res, conversations, requiredMethod);
  if (parsed.pathname.indexOf('/ob/profile/') === 0) {
    return json(res, { peerID: decodeURIComponent(parsed.pathname.slice('/ob/profile/'.length)), name: 'Peer' }, requiredMethod);
  }
  if (parsed.pathname.indexOf('/ob/chatmessages/') === 0) {
    const peer = decodeURIComponent(parsed.pathname.slice('/ob/chatmessages/'.length));
    return json(res, chatMessages[peer] || [], requiredMethod);
  }
  if (parsed.pathname.indexOf('/ob/markchatasread/') === 0) return json(res, {}, requiredMethod);
  if (parsed.pathname === '/ob/chat') return json(res, { queued: false }, requiredMethod);
  fixtureFailures.push({ fixture: 'social', reason: 'unhandled', method: req.method, path: parsed.pathname, origin: req.headers.origin, status: 404 });
  res.writeHead(404, corsHeaders(requiredMethod));
  res.end();
});
owned.social = social;

const origOn = app.on.bind(app);
const heldReady = [];
app.on = function holdReady(event, listener) {
  if (event === 'ready') {
    heldReady.push(listener);
    return app;
  }
  return origOn(event, listener);
};

app.setPath('userData', userData);
app.setPath('home', home);
app.disableHardwareAcceleration();

let pickerCanceled = 0;
const electron = require('electron');
const originalOpen = electron.dialog.showOpenDialog.bind(electron.dialog);
electron.dialog.showOpenDialog = async function harnessPicker() {
  pickerCanceled += 1;
  return { canceled: true, filePaths: [] };
};
owned.patches.push(() => { electron.dialog.showOpenDialog = originalOpen; });

const originalLoad = Module._load;
Module._load = function harnessLoad(request, parent, isMain) {
  if (request === './wallet-broker/supervisor') {
    return {
      createWalletSupervisor() {
        return {
          bound: false,
          subscribeSnapshot() { return () => {}; },
          start() {},
          shutdown() { return Promise.resolve(); },
          dispatch() { return Promise.resolve({ v: 1, broker: 'down', accounts: [] }); },
        };
      },
    };
  }
  if (request === './wallet-broker/launch-config') {
    return {
      resolveWalletBrokerLaunch() {
        return { brokerPath: path.join(userData, 'none'), expectedSha256: '00'.repeat(32), dataDir: userData };
      },
    };
  }
  if (request === './wallet-pay/inbox-client') {
    const actual = originalLoad.call(this, request, parent, isMain);
    return {
      createPaymentInboxClient(opts) {
        return actual.createPaymentInboxClient(Object.assign({}, opts, {
          now: () => injectedNow,
          homedir: () => app.getPath('home'),
        }));
      },
    };
  }
  return originalLoad.call(this, request, parent, isMain);
};
owned.patches.push(() => { Module._load = originalLoad; });

require(path.join(repoRoot, 'social-main.js'));
app.on = origOn;

function recordBlocker(classification, kind, method) {
  blockerCounts[classification] += 1;
  if (blockerRecent.length >= 32) blockerRecent.shift();
  blockerRecent.push({ classification, kind, method: diagnosticString(method, 16) });
}

function installBlocker(socialOrigin) {
  const expected = [];
  const socialSocket = socialOrigin.replace(/^http:/, 'ws:') + '/ws';
  const defaultOrigin = 'http://127.0.0.1:4002';
  const defaultSocket = 'ws://127.0.0.1:4002/ws';
  session.defaultSession.webRequest.onBeforeRequest(
    { urls: ['http://*/*', 'https://*/*', 'ws://*/*', 'wss://*/*'] },
    (details, callback) => {
      const parsed = new URL(details.url);
      const origin = parsed.origin;
      const method = details.method || 'GET';
      const exactURL = parsed.search === '' && parsed.hash === '' && parsed.username === '' && parsed.password === '';
      const required = exactURL ? socialMethod(parsed.pathname) : '';
      const allowedMethod = method === required || (method === 'OPTIONS' && Boolean(required));
      if (origin === socialOrigin && allowedMethod) {
        recordBlocker('allowed', 'owned-social-http', method);
        callback({});
        return;
      }
      if ((details.url === socialSocket || details.url === defaultSocket) && method === 'GET') {
        const kind = details.url === socialSocket ? 'owned-social-ws' : 'default-ws';
        expected.push({ kind, method, url: details.url });
        recordBlocker('expected', kind, method);
        callback({ cancel: true });
        return;
      }
      if (origin === defaultOrigin && allowedMethod) {
        expected.push({ kind: 'default-http', method, url: details.url });
        recordBlocker('expected', 'default-http', method);
        callback({ cancel: true });
        return;
      }
      blocked.push({ method, url: details.url });
      recordBlocker('unexpected', 'other-network', method);
      callback({ cancel: true });
    }
  );
  return expected;
}

function withDeadline(promise, timeout, label) {
  return new Promise((resolve, reject) => {
    const timer = setTimeout(() => reject(new Error(label || 'deadline exceeded')), timeout);
    Promise.resolve(promise).then((value) => {
      clearTimeout(timer);
      resolve(value);
    }, (error) => {
      clearTimeout(timer);
      reject(error);
    });
  });
}

function lifecycleText(value) {
  return String(value || '')
    .split(appDocumentPath).join('<app-document>')
    .split(appDocumentURL).join('<app-document>')
    .replace(/[\r\n\t]+/g, ' ')
    .slice(0, 240);
}

function setBootstrapPhase(phase) {
  bootstrap.phase = phase;
}

function traceBootstrap(event, facts) {
  if (bootstrap.events.length >= 32) return;
  bootstrap.events.push(Object.assign({
    event,
    phase: bootstrap.phase,
    elapsed_ms: Date.now() - bootstrapStartedAt,
  }, facts || {}));
}

function loadErrorFacts(error) {
  let code = null;
  if (error && (typeof error.code === 'string' || Number.isInteger(error.code))) code = error.code;
  else if (error && Number.isInteger(error.errno)) code = error.errno;
  let description = error && typeof error.description === 'string' ? error.description : '';
  if (!description && error && error.message) description = String(error.message).replace(/\s+loading\s+['"].*$/i, '');
  return { code, description: lifecycleText(description) };
}

function bootstrapState() {
  const win = bootstrap.win;
  if (!win) {
    return {
      window_destroyed: null,
      web_contents_destroyed: null,
      loading: null,
      url_matches_expected: null,
    };
  }
  try {
    const windowDestroyed = win.isDestroyed();
    const contents = windowDestroyed ? null : win.webContents;
    const contentsDestroyed = !contents || contents.isDestroyed();
    return {
      window_destroyed: windowDestroyed,
      web_contents_destroyed: contentsDestroyed,
      loading: contentsDestroyed ? null : contents.isLoading(),
      url_matches_expected: contentsDestroyed ? null : contents.getURL() === appDocumentURL,
    };
  } catch (_) {
    return {
      window_destroyed: true,
      web_contents_destroyed: true,
      loading: null,
      url_matches_expected: null,
    };
  }
}

function writeBootstrapDiagnostics(outcome) {
  const report = {
    version: 1,
    outcome,
    phase: bootstrap.phase,
    elapsed_ms: Date.now() - bootstrapStartedAt,
    load_error: bootstrap.loadError,
    renderer_termination: bootstrap.rendererTermination,
    current: bootstrapState(),
    events: bootstrap.events.slice(),
  };
  fs.writeFileSync(bootstrapDiagnosticsPath, `${JSON.stringify(report, null, 2)}\n`);
  bootstrap.outcome = outcome;
}

function observeBootstrapLifecycle(win) {
  const contents = win.webContents;
  const onStart = () => traceBootstrap('did-start-loading');
  const onFinish = () => traceBootstrap('did-finish-load');
  const onFail = (_event, code, description, _validatedURL, isMainFrame) => {
    const facts = {
      code,
      description: lifecycleText(description),
      main_frame: Boolean(isMainFrame),
    };
    traceBootstrap('did-fail-load', facts);
    if (isMainFrame) bootstrap.loadError = { code, description: facts.description };
  };
  const onGone = (_event, details) => {
    bootstrap.rendererTermination = {
      reason: lifecycleText(details && details.reason),
      exit_code: details && Number.isInteger(details.exitCode) ? details.exitCode : null,
    };
    traceBootstrap('render-process-gone', bootstrap.rendererTermination);
  };
  const onDestroyed = () => traceBootstrap('web-contents-destroyed');
  contents.on('did-start-loading', onStart);
  contents.on('did-finish-load', onFinish);
  contents.on('did-fail-load', onFail);
  contents.on('render-process-gone', onGone);
  contents.on('destroyed', onDestroyed);
  traceBootstrap('observer-attached');
  let active = true;
  owned.bootstrapTrace = () => {
    if (!active) return;
    active = false;
    contents.removeListener('did-start-loading', onStart);
    contents.removeListener('did-finish-load', onFinish);
    contents.removeListener('did-fail-load', onFail);
    contents.removeListener('render-process-gone', onGone);
    contents.removeListener('destroyed', onDestroyed);
  };
}

function inspectApplicationDocument(win) {
  return win.webContents.executeJavaScript(`
    (function() {
      var required = [
        'conversationList', 'messageList', 'apiURLInput', 'saveConnectionButton',
        'sendMessageButton', 'statusDot', 'paymentNotice'
      ];
      return {
        urlMatches: window.location.href === ${JSON.stringify(appDocumentURL)},
        readyState: document.readyState,
        domIdentity: document.title === 'BitBook' &&
          required.every(function(id) { return Boolean(document.getElementById(id)); }) &&
          Boolean(document.querySelector('script[src="app.js"]'))
      };
    })()
  `);
}

function diagnosticString(value, limit) {
  if (typeof value !== 'string') return null;
  return value.replace(/[\r\n\t]+/g, ' ').slice(0, limit);
}

function diagnosticStatus(value) {
  return Number.isInteger(value) && value >= 100 && value <= 599 ? value : null;
}

function paymentPath(value) {
  if (typeof value !== 'string') return null;
  try {
    return diagnosticString(new URL(value, 'http://fixture.invalid').pathname, 240);
  } catch (_) {
    return null;
  }
}

function fixtureActivitySnapshot() {
  const socialRecent = socialHits.slice(-32).map((item) => ({
    method: diagnosticString(item.method, 16),
    path: diagnosticString(item.path, 240),
    origin: diagnosticString(item.origin, 160),
    status: diagnosticStatus(item.status),
  }));
  const paymentRecent = paymentHits.slice(-32).map((item) => ({
    method: diagnosticString(item.method, 16),
    path: paymentPath(item.url),
    status: diagnosticStatus(item.status),
  }));
  const failuresRecent = fixtureFailures.slice(-32).map((item) => {
    if (item && item.fixture === 'payment') {
      return {
        fixture: 'payment',
        method: diagnosticString(item.method, 16),
        path: paymentPath(item.url),
        status: diagnosticStatus(item.status),
      };
    }
    return {
      fixture: item && item.fixture === 'social' ? 'social' : null,
      reason: diagnosticString(item && item.reason, 40),
      method: diagnosticString(item && item.method, 16),
      path: diagnosticString(item && item.path, 240),
      origin: diagnosticString(item && item.origin, 160),
      status: diagnosticStatus(item && item.status),
    };
  });
  return {
    social: { count: socialHits.length, recent: socialRecent },
    payment: { count: paymentHits.length, recent: paymentRecent },
    fixture_failures: { count: fixtureFailures.length, recent: failuresRecent },
    blocker: {
      count: blockerCounts.allowed + blockerCounts.expected + blockerCounts.unexpected,
      classifications: {
        allowed: blockerCounts.allowed,
        expected: blockerCounts.expected,
        unexpected: blockerCounts.unexpected,
      },
      recent: blockerRecent.slice(),
    },
  };
}

function diagnosticAttempt(work, timeout) {
  return new Promise((resolve) => {
    let settled = false;
    const finish = (outcome, value) => {
      if (settled) return;
      settled = true;
      clearTimeout(timer);
      resolve({ outcome, value });
    };
    const timer = setTimeout(() => finish('timeout'), timeout);
    Promise.resolve().then(work).then(
      (value) => finish('success', value),
      () => finish('rejection')
    );
  });
}

function usableRenderer(win) {
  try {
    return Boolean(win && !win.isDestroyed() && win.webContents && !win.webContents.isDestroyed());
  } catch (_) {
    return false;
  }
}

async function captureFailureDOM(win) {
  if (!usableRenderer(win)) return { outcome: 'unavailable' };
  const attempt = await diagnosticAttempt(() => win.webContents.executeJavaScript(`
    (function() {
      function label(id) {
        var node = document.getElementById(id);
        if (!node) return null;
        return String(node.innerText || node.textContent || '').replace(/[\\r\\n\\t]+/g, ' ').slice(0, 160);
      }
      var status = document.getElementById('statusDot');
      var notice = document.getElementById('paymentNotice');
      var paymentModule = window.BitBookPaymentInbox;
      var preload = window.bitbookWallet;
      return {
        readyState: document.readyState,
        visibilityState: document.visibilityState,
        hidden: Boolean(document.hidden),
        online: status ? status.classList.contains('online') : null,
        paymentNoticeHidden: notice ? notice.classList.contains('hidden') : null,
        requestCardCount: document.querySelectorAll('.paymentRequest').length,
        conversationCount: document.querySelectorAll('.conversationRow').length,
        paymentModule: Boolean(paymentModule),
        paymentCreateReader: Boolean(paymentModule && typeof paymentModule.createReader === 'function'),
        paymentBindSnapshot: Boolean(paymentModule && typeof paymentModule.bindSnapshot === 'function'),
        preload: Boolean(preload),
        preloadGetPaymentInbox: Boolean(preload && typeof preload.getPaymentInbox === 'function'),
        preloadConnectPaymentInbox: Boolean(preload && typeof preload.connectPaymentInbox === 'function'),
        paymentNoticeLabel: label('paymentNotice'),
        connectionLabel: label('connectionText')
      };
    })()
  `), 3000);
  if (attempt.outcome !== 'success') return { outcome: attempt.outcome };
  const value = attempt.value || {};
  const readyStates = ['loading', 'interactive', 'complete'];
  const visibilityStates = ['hidden', 'visible', 'prerender'];
  const boolean = (item) => typeof item === 'boolean' ? item : null;
  return {
    outcome: 'success',
    document: {
      ready_state: readyStates.includes(value.readyState) ? value.readyState : null,
      visibility_state: visibilityStates.includes(value.visibilityState) ? value.visibilityState : null,
      hidden: boolean(value.hidden),
    },
    ready_conditions: {
      online_status: boolean(value.online),
      payment_notice_hidden: boolean(value.paymentNoticeHidden),
      request_card_count: Number.isInteger(value.requestCardCount) ? value.requestCardCount : null,
    },
    modules: {
      payment_module: boolean(value.paymentModule),
      payment_create_reader: boolean(value.paymentCreateReader),
      payment_bind_snapshot: boolean(value.paymentBindSnapshot),
      preload: boolean(value.preload),
      preload_get_payment_inbox: boolean(value.preloadGetPaymentInbox),
      preload_connect_payment_inbox: boolean(value.preloadConnectPaymentInbox),
    },
    labels: {
      payment_notice: diagnosticString(value.paymentNoticeLabel, 160),
      connection: diagnosticString(value.connectionLabel, 160),
    },
    conversation_count: Number.isInteger(value.conversationCount) ? value.conversationCount : null,
  };
}

async function captureFailureScreenshot(win) {
  if (!usableRenderer(win)) return { outcome: 'unavailable' };
  const attempt = await diagnosticAttempt(async () => {
    const image = await win.webContents.capturePage();
    const png = image.toPNG();
    const size = image.getSize();
    const name = 'fixture-failure.png';
    fs.writeFileSync(path.join(artifactRoot, name), png);
    return {
      name,
      width: size.width,
      height: size.height,
      sha256: crypto.createHash('sha256').update(png).digest('hex'),
    };
  }, 5000);
  if (attempt.outcome !== 'success') return { outcome: attempt.outcome };
  return {
    outcome: 'success',
    name: attempt.value.name,
    width: Number.isInteger(attempt.value.width) ? attempt.value.width : null,
    height: Number.isInteger(attempt.value.height) ? attempt.value.height : null,
    sha256: /^[0-9a-f]{64}$/.test(attempt.value.sha256) ? attempt.value.sha256 : null,
  };
}

async function probePaymentInbox(win) {
  if (!usableRenderer(win)) return { outcome: 'unavailable' };
  const allowedStates = ['ready', 'unavailable', 'unsupported', 'invalid', 'identity_changed', 'too_large'];
  const attempt = await diagnosticAttempt(() => win.webContents.executeJavaScript(`
    (function() {
      if (!window.bitbookWallet || typeof window.bitbookWallet.getPaymentInbox !== 'function') {
        return { methodAvailable: false };
      }
      return window.bitbookWallet.getPaymentInbox().then(function(dto) {
        var states = ${JSON.stringify(allowedStates)};
        return {
          methodAvailable: true,
          state: dto && states.indexOf(dto.state) !== -1 ? dto.state : null,
          rowCount: dto && Array.isArray(dto.requests) ? dto.requests.length : null,
          peerMatchesFixture: dto && typeof dto.peer_id === 'string' ? dto.peer_id === ${JSON.stringify(LOCAL)} : null
        };
      });
    })()
  `), 5000);
  if (attempt.outcome !== 'success') return { outcome: attempt.outcome };
  const value = attempt.value || {};
  if (value.methodAvailable !== true) return { outcome: 'unavailable' };
  return {
    outcome: 'success',
    state: allowedStates.includes(value.state) ? value.state : null,
    row_count: Number.isInteger(value.rowCount) ? value.rowCount : null,
    peer_matches_fixture: typeof value.peerMatchesFixture === 'boolean' ? value.peerMatchesFixture : null,
  };
}

async function writeUIFailureDiagnostics() {
  const fixtureActivity = fixtureActivitySnapshot();
  const [dom, screenshot] = await Promise.all([
    captureFailureDOM(bootstrap.win),
    captureFailureScreenshot(bootstrap.win),
  ]);
  const payment_probe = await probePaymentInbox(bootstrap.win);
  const report = {
    version: 1,
    failure: {
      stage: diagnosticString(uiFailureStage, 80),
      assertion_label: diagnosticString(uiAssertionLabel, 160),
    },
    dom,
    fixture_activity: fixtureActivity,
    payment_probe,
    screenshot,
  };
  fs.writeFileSync(uiFailureDiagnosticsPath, `${JSON.stringify(report, null, 2)}\n`);
}

async function capture(win, name, width, height) {
  await win.setSize(width, height);
  await new Promise((resolve) => setTimeout(resolve, 250));
  const image = await win.webContents.capturePage();
  const size = image.getSize();
  const dest = path.join(artifactRoot, name);
  fs.writeFileSync(dest, image.toPNG());
  return {
    dest: path.relative(repoRoot, dest),
    width: size.width,
    height: size.height,
    sha256: crypto.createHash('sha256').update(fs.readFileSync(dest)).digest('hex'),
  };
}

async function sendEnter(win) {
  assert.ok(win && !win.isDestroyed(), 'owned BrowserWindow unavailable for Enter input');
  const contents = win.webContents;
  assert.ok(contents && !contents.isDestroyed(), 'owned webContents unavailable for Enter input');
  win.focus();
  contents.focus();
  await waitUntil(
    () => !win.isDestroyed() && !contents.isDestroyed() && win.isFocused() && contents.isFocused(),
    3000,
    'owned BrowserWindow/webContents did not gain focus'
  );
  assert.strictEqual(
    await contents.executeJavaScript(
      'document.activeElement === document.querySelector(".paymentRequestDetails summary")'
    ),
    true,
    'payment details summary is not the activeElement'
  );
  contents.sendInputEvent({ type: 'rawKeyDown', keyCode: 'Enter' });
  // Chromium's summary activation consumes the carriage-return char event.
  contents.sendInputEvent({ type: 'char', keyCode: String.fromCharCode(13) });
  contents.sendInputEvent({ type: 'keyUp', keyCode: 'Enter' });
}

async function clickElement(win, selector) {
  const deadline = Date.now() + 3000;
  const contents = win.webContents;
  const probe = (scroll, expectedPoint) => contents.executeJavaScript(`
    (function() {
      var selector = ${JSON.stringify(selector)};
      var node = document.querySelector(selector);
      var viewport = { left: 0, top: 0, right: window.innerWidth, bottom: window.innerHeight };
      function rectValue(rect) {
        return {
          left: rect.left,
          top: rect.top,
          right: rect.right,
          bottom: rect.bottom,
          width: rect.width,
          height: rect.height
        };
      }
      function describe(element) {
        if (!element) return null;
        return {
          tag: element.tagName ? element.tagName.toLowerCase() : '',
          id: element.id || '',
          class_name: element.getAttribute && element.getAttribute('class') || '',
          text: String(element.textContent || '').trim().slice(0, 120)
        };
      }
      var diagnostic = { selector: selector, viewport: viewport };
      if (!node) {
        diagnostic.reason = 'missing';
        diagnostic.rect = null;
        diagnostic.visible = null;
        diagnostic.point = null;
        diagnostic.hit = null;
        diagnostic.ok = false;
        return diagnostic;
      }
      if (${scroll ? 'true' : 'false'}) {
        node.scrollIntoView({ behavior: 'instant', block: 'nearest', inline: 'nearest' });
      }
      var rect = node.getBoundingClientRect();
      var visible = {
        left: Math.max(rect.left, viewport.left),
        top: Math.max(rect.top, viewport.top),
        right: Math.min(rect.right, viewport.right),
        bottom: Math.min(rect.bottom, viewport.bottom)
      };
      var clipping = [];
      for (var ancestor = node.parentElement; ancestor; ancestor = ancestor.parentElement) {
        var ancestorStyle = window.getComputedStyle(ancestor);
        var clipsX = /^(auto|scroll|hidden|clip|overlay)$/.test(ancestorStyle.overflowX);
        var clipsY = /^(auto|scroll|hidden|clip|overlay)$/.test(ancestorStyle.overflowY);
        if (!clipsX && !clipsY) continue;
        var ancestorRect = ancestor.getBoundingClientRect();
        var clip = {
          left: ancestorRect.left + ancestor.clientLeft,
          top: ancestorRect.top + ancestor.clientTop,
          right: ancestorRect.left + ancestor.clientLeft + ancestor.clientWidth,
          bottom: ancestorRect.top + ancestor.clientTop + ancestor.clientHeight
        };
        clipping.push({
          element: describe(ancestor),
          overflow_x: ancestorStyle.overflowX,
          overflow_y: ancestorStyle.overflowY,
          rect: clip
        });
        if (clipsX) {
          visible.left = Math.max(visible.left, clip.left);
          visible.right = Math.min(visible.right, clip.right);
        }
        if (clipsY) {
          visible.top = Math.max(visible.top, clip.top);
          visible.bottom = Math.min(visible.bottom, clip.bottom);
        }
      }
      visible.width = Math.max(0, visible.right - visible.left);
      visible.height = Math.max(0, visible.bottom - visible.top);
      var style = window.getComputedStyle(node);
      var point = ${JSON.stringify(expectedPoint)};
      if (!point && visible.width >= 1 && visible.height >= 1) {
        var minX = Math.ceil(visible.left);
        var maxX = Math.ceil(visible.right) - 1;
        var minY = Math.ceil(visible.top);
        var maxY = Math.ceil(visible.bottom) - 1;
        if (minX <= maxX && minY <= maxY) {
          point = {
            x: Math.max(minX, Math.min(maxX, Math.floor((visible.left + visible.right) / 2))),
            y: Math.max(minY, Math.min(maxY, Math.floor((visible.top + visible.bottom) / 2)))
          };
        }
      }
      var pointVisible = Boolean(point) && point.x >= visible.left && point.x < visible.right &&
        point.y >= visible.top && point.y < visible.bottom;
      var hit = pointVisible ? document.elementFromPoint(point.x, point.y) : null;
      var hitMatches = Boolean(hit) && (hit === node || node.contains(hit));
      var visuallyShown = style.display !== 'none' && style.visibility !== 'hidden' &&
        style.visibility !== 'collapse' && Number(style.opacity) > 0;
      diagnostic.rect = rectValue(rect);
      diagnostic.visible = visible;
      diagnostic.clipping = clipping;
      diagnostic.point = point;
      diagnostic.point_visible = pointVisible;
      diagnostic.hit = describe(hit);
      diagnostic.hit_matches = hitMatches;
      diagnostic.style = {
        display: style.display,
        visibility: style.visibility,
        opacity: style.opacity,
        pointer_events: style.pointerEvents
      };
      diagnostic.ok = visuallyShown && visible.width >= 1 && visible.height >= 1 &&
        pointVisible && hitMatches && style.pointerEvents !== 'none';
      if (!diagnostic.ok) diagnostic.reason = 'not-visible-or-hittable';
      return diagnostic;
    })()
  `);
  let lastProbe = await probe(true, null);
  assert.notStrictEqual(
    lastProbe.reason,
    'missing',
    `pointer target missing: ${JSON.stringify(lastProbe)}`
  );
  let stableSignature = lastProbe.ok ? JSON.stringify({
    rect: lastProbe.rect,
    visible: lastProbe.visible,
    point: lastProbe.point,
  }) : '';
  let stableMeasurements = lastProbe.ok ? 1 : 0;
  let target;
  try {
    target = await waitUntil(async () => {
      lastProbe = await probe(false, null);
      if (!lastProbe.ok) {
        stableSignature = '';
        stableMeasurements = 0;
        return false;
      }
      const signature = JSON.stringify({
        rect: lastProbe.rect,
        visible: lastProbe.visible,
        point: lastProbe.point,
      });
      stableMeasurements = signature === stableSignature ? stableMeasurements + 1 : 1;
      stableSignature = signature;
      return stableMeasurements >= 2 ? lastProbe : false;
    }, Math.max(1, deadline - Date.now()), `pointer target did not settle: ${selector}`);
  } catch (error) {
    assert.fail(`pointer target unavailable: ${JSON.stringify(lastProbe)}; ${error.message}`);
  }
  const immediate = await probe(false, target.point);
  assert.ok(immediate.ok, `pointer target changed before input: ${JSON.stringify(immediate)}`);
  contents.sendInputEvent({ type: 'mouseMove', x: target.point.x, y: target.point.y });
  contents.sendInputEvent({ type: 'mouseDown', x: target.point.x, y: target.point.y, button: 'left', clickCount: 1 });
  contents.sendInputEvent({ type: 'mouseUp', x: target.point.x, y: target.point.y, button: 'left', clickCount: 1 });
}

function transcriptState(win) {
  return win.webContents.executeJavaScript(`
    (function() {
      var list = document.getElementById('messageList');
      var top = list.getBoundingClientRect().top;
      var visible = Array.prototype.find.call(list.children, function(child) {
        return child.getBoundingClientRect().bottom > top + 1;
      });
      return {
        key: visible && visible.getAttribute('data-entry-key'),
        offset: visible ? visible.getBoundingClientRect().top - top : 0,
        scrollTop: list.scrollTop,
        scrollHeight: list.scrollHeight,
        clientHeight: list.clientHeight,
        bottomGap: list.scrollHeight - list.scrollTop - list.clientHeight,
        summaryFocused: document.activeElement === document.querySelector('.paymentRequestDetails summary'),
        detailsOpen: document.querySelector('.paymentRequestDetails').open,
        stableTextNode: window.__stableTextNode === Array.prototype.find.call(
          document.querySelectorAll('.message'),
          function(node) { return node.querySelector('p').textContent.indexOf('long-message-20') !== -1; }
        )
      };
    })()
  `);
}

function layoutOk(win, selector) {
  return win.webContents.executeJavaScript(`
    (function() {
      var nodes = Array.prototype.slice.call(document.querySelectorAll(${JSON.stringify(selector)}));
      if (!nodes.length) return { ok: false, reason: 'missing', selector: ${JSON.stringify(selector)} };
      function box(el) { return el.getBoundingClientRect(); }
      function overlap(a, b) {
        return a.left < b.right - 1 && a.right > b.left + 1 && a.top < b.bottom - 1 && a.bottom > b.top + 1;
      }
      for (var i = 0; i < nodes.length; i += 1) {
        var node = nodes[i];
        var r = box(node);
        if (r.width <= 1 || r.height <= 1) {
          return { ok: false, reason: 'zero-geometry', i: i, width: r.width, height: r.height };
        }
        if (node.scrollWidth > node.clientWidth + 1) {
          return { ok: false, reason: 'scroll', i: i, scrollWidth: node.scrollWidth, clientWidth: node.clientWidth };
        }
        if (r.right > window.innerWidth + 1 || r.left < -1) {
          return { ok: false, reason: 'viewport', i: i };
        }
        var children = Array.prototype.slice.call(node.children);
        for (var c = 0; c < children.length; c += 1) {
          var child = children[c];
          var details = child.closest && child.closest('details');
          if (details && !details.open && child.tagName !== 'SUMMARY') continue;
          var cr = box(child);
          if (cr.width <= 1 || cr.height <= 1) continue;
          if (cr.right > r.right + 1 || cr.left < r.left - 1) {
            return { ok: false, reason: 'child-overflow', i: i, c: c };
          }
          if (c > 0 && overlap(box(children[c - 1]), cr)) {
            return { ok: false, reason: 'overlap', i: i, c: c };
          }
        }
      }
      return { ok: true, count: nodes.length };
    })()
  `);
}

async function main() {
  await new Promise((resolve, reject) => {
    server.listen(0, '127.0.0.1', () => resolve());
    server.on('error', reject);
  });
  await new Promise((resolve, reject) => {
    social.listen(0, '127.0.0.1', () => resolve());
    social.on('error', reject);
  });
  const payPort = server.address().port;
  const socialPort = social.address().port;
  const socialOrigin = `http://127.0.0.1:${socialPort}`;
  fs.writeFileSync(path.join(privateDir, 'connection.json'), JSON.stringify({
    v: 1,
    endpoint: `http://127.0.0.1:${payPort}`,
    peer_id: LOCAL,
    instance_id: 'a'.repeat(32),
    token: 'b'.repeat(64),
  }), { mode: 0o600 });
  fs.chmodSync(path.join(privateDir, 'connection.json'), 0o600);

  let expectedBlocked = [];
  if (app.isReady()) expectedBlocked = installBlocker(socialOrigin);
  else origOn('ready', () => { expectedBlocked = installBlocker(socialOrigin); });
  for (const handler of heldReady) {
    if (app.isReady()) handler();
    else origOn('ready', handler);
  }

  setBootstrapPhase('wait-initial-window');
  const win = await waitUntil(() => BrowserWindow.getAllWindows()[0], 15000, 'window missing');
  bootstrap.win = win;
  observeBootstrapLifecycle(win);
  traceBootstrap('window-found');

  setBootstrapPhase('wait-initial-document');
  const initialDocument = await waitUntil(async () => {
    if (win.isDestroyed() || win.webContents.isDestroyed() || win.webContents.isLoading()) return false;
    if (win.webContents.getURL() !== appDocumentURL) return false;
    const identity = await inspectApplicationDocument(win);
    return identity.urlMatches && identity.readyState === 'complete' && identity.domIdentity ? identity : false;
  }, 15000, 'initial application document missing');
  assert.strictEqual(win.webContents.getURL(), appDocumentURL, 'initial application URL mismatch');
  assert.ok(initialDocument.domIdentity, 'initial application DOM identity mismatch');
  traceBootstrap('initial-document-ready', {
    url_matches_expected: initialDocument.urlMatches,
    ready_state: initialDocument.readyState,
    dom_identity: initialDocument.domIdentity,
  });

  setBootstrapPhase('set-fixture-storage');
  const stored = await win.webContents.executeJavaScript(`
    localStorage.setItem('bitbook.apiURL', ${JSON.stringify(socialOrigin)});
    localStorage.setItem('bitbook.profilePrompted', 'true');
    ({
      apiURL: localStorage.getItem('bitbook.apiURL') === ${JSON.stringify(socialOrigin)},
      profilePrompted: localStorage.getItem('bitbook.profilePrompted') === 'true'
    })
  `);
  assert.ok(stored.apiURL && stored.profilePrompted, 'fixture bootstrap storage was not retained');
  traceBootstrap('fixture-storage-ready');

  setBootstrapPhase('main-process-load');
  traceBootstrap('load-file-started');
  try {
    await withDeadline(win.loadFile(appDocumentPath), 15000, 'configured main-process load did not finish');
    traceBootstrap('load-file-resolved');
  } catch (error) {
    if (!bootstrap.loadError) bootstrap.loadError = loadErrorFacts(error);
    traceBootstrap('load-file-rejected', bootstrap.loadError);
    throw error;
  }

  setBootstrapPhase('verify-configured-document');
  const configuredDocument = await waitUntil(async () => {
    if (win.isDestroyed() || win.webContents.isDestroyed() || win.webContents.isLoading()) return false;
    if (win.webContents.getURL() !== appDocumentURL) return false;
    const identity = await inspectApplicationDocument(win);
    return identity.urlMatches && identity.readyState === 'complete' && identity.domIdentity ? identity : false;
  }, 3000, 'configured application document missing');
  assert.strictEqual(win.webContents.getURL(), appDocumentURL, 'configured application URL mismatch');
  assert.ok(configuredDocument.domIdentity, 'configured application DOM identity mismatch');
  traceBootstrap('configured-document-ready', {
    url_matches_expected: configuredDocument.urlMatches,
    ready_state: configuredDocument.readyState,
    dom_identity: configuredDocument.domIdentity,
  });
  setBootstrapPhase('complete');
  writeBootstrapDiagnostics('success');
  if (owned.bootstrapTrace) {
    owned.bootstrapTrace();
    owned.bootstrapTrace = null;
  }

  const rendererPid = win.webContents.getOSProcessId();
  const browserStatus = readStatus(process.pid);
  const rendererStatus = readStatus(rendererPid);
  assert.strictEqual(rendererStatus.Seccomp, '2', rendererStatus);
  assert.strictEqual(rendererStatus.NoNewPrivs, '1', rendererStatus);
  const browserNs = browserStatus.NSpid.split(/\s+/);
  const rendererNs = rendererStatus.NSpid.split(/\s+/);
  assert.ok(rendererNs.length > browserNs.length, JSON.stringify({ browserNs, rendererNs }));
  fs.writeFileSync(path.join(artifactRoot, 'sandbox-status.json'), `${JSON.stringify({
    browser: browserStatus,
    renderer: rendererStatus,
  }, null, 2)}\n`);

  const prefs = win.webContents.getLastWebPreferences();
  assert.strictEqual(prefs.sandbox, true);
  assert.strictEqual(prefs.contextIsolation, true);
  assert.strictEqual(prefs.nodeIntegration, false);
  assert.strictEqual(prefs.webSecurity, true);

  await waitUntil(
    () => win.webContents.executeJavaScript('document.querySelector(\'[data-view="requests"]\') === null'),
    3000,
    'Requests tab still present'
  );
  assert.ok(await win.webContents.executeJavaScript('Boolean(document.getElementById("sendMessageButton"))'));

  uiFailureStage = 'empty-messages-ready';
  uiAssertionLabel = 'empty messages state missing';
  await waitUntil(
    () => win.webContents.executeJavaScript('document.getElementById("conversationList").innerText.indexOf("No conversations") !== -1'),
    8000,
    'empty messages state missing'
  );
  uiFailureStage = 'configured-ready-empty';
  uiAssertionLabel = 'configured ready empty payment state missing';
  await waitUntil(
    () => win.webContents.executeJavaScript(`
      document.getElementById('statusDot').classList.contains('online') &&
      document.getElementById('paymentNotice').classList.contains('hidden') &&
      document.querySelectorAll('.paymentRequest').length === 0
    `),
    8000,
    'configured ready empty payment state missing'
  );
  uiFailureStage = 'ui-journey';
  uiAssertionLabel = null;
  assert.ok(socialHits.some((item) => item.path === '/ob/config' && item.method === 'GET'));
  assert.ok(paymentHits.some((item) => item.method === 'GET' && item.url === '/v1/payment/records'));
  const shots = [];
  await win.setSize(1440, 1000);
  const emptyWide = await layoutOk(win, '#conversationList');
  assert.ok(emptyWide.ok, JSON.stringify(emptyWide));
  shots.push(await capture(win, 'fixture-empty-messages-1440x1000.png', 1440, 1000));

  const coffee = longCoffeeRecord();
  const coffeeId = JSON.parse(coffee.signed.canonical).request_id;
  conversations = [{
    peerId: PAYEE,
    timestamp: '2026-08-30T12:00:00Z',
    lastMessage: 'hello-from-fixture',
    outgoing: false,
    unread: 0,
  }];
  chatMessages[PAYEE] = Array.from({ length: 80 }, (_unused, index) => ({
    messageId: `fixture-long-${String(index).padStart(3, '0')}`,
    message: `long-message-${String(index).padStart(2, '0')} ${'scroll proof '.repeat(7)}`,
    timestamp: new Date(Date.parse('2026-08-30T10:00:00Z') + index * 60000).toISOString(),
    outgoing: index % 3 === 0,
    read: true,
    peerId: PAYEE,
  }));
  setPayloadRecords([coffee]);
  await waitUntil(
    () => win.webContents.executeJavaScript(
      'document.body.innerText.indexOf("Payment request ·") !== -1'
    ),
    12000,
    'request-only conversation missing'
  );
  await clickElement(win, `.conversationRow[data-peer-id="${PAYEE}"]`);
  await waitUntil(
    () => win.webContents.executeJavaScript(`
      Boolean(document.querySelector('.paymentRequest[data-request-id="${coffeeId}"]')) &&
      document.getElementById('messageList').innerText.indexOf('long-message-79') !== -1
    `),
    8000,
    'long request transcript missing'
  );
  const cardText = await win.webContents.executeJavaScript(
    `document.querySelector('.paymentRequest[data-request-id="${coffeeId}"]').textContent`
  );
  assert.ok(cardText.includes('Payment request'), cardText);
  assert.ok(cardText.includes(longMemo), cardText);
  assert.ok(cardText.includes('1.00000000 ZEC'), cardText);
  assert.ok(cardText.includes('zec-testnet'), cardText);
  assert.ok(cardText.includes('requested'), cardText);
  assert.strictEqual(
    await win.webContents.executeJavaScript('document.querySelector(".paymentRequestDetails").open === true'),
    false
  );
  assert.strictEqual(await win.webContents.executeJavaScript(
    'document.querySelector(".paymentRequest").getAttribute("data-request-id")'
  ), coffeeId);

  await clickElement(win, '.paymentRequestDetails summary');
  await waitUntil(
    () => win.webContents.executeJavaScript('document.querySelector(".paymentRequestDetails").open === true'),
    3000,
    'pointer did not open details'
  );
  await sendEnter(win);
  await waitUntil(
    () => win.webContents.executeJavaScript('document.querySelector(".paymentRequestDetails").open === false'),
    3000,
    'keyboard did not close details'
  );
  await sendEnter(win);
  await waitUntil(
    () => win.webContents.executeJavaScript('document.querySelector(".paymentRequestDetails").open === true'),
    3000,
    'details did not open'
  );
  const openText = await win.webContents.executeJavaScript('document.querySelector(".paymentRequestDetails").innerText');
  assert.ok(openText.includes('Request ID'), openText);
  assert.ok(openText.includes(coffeeId), openText);
  assert.ok(openText.includes(PAYEE), openText);

  await win.setSize(1440, 1000);
  const wideScroll = await transcriptState(win);
  assert.ok(wideScroll.scrollHeight > wideScroll.clientHeight, JSON.stringify(wideScroll));
  const messagesWide = await layoutOk(win, '#messageList');
  assert.ok(messagesWide.ok, JSON.stringify(messagesWide));
  await win.setSize(980, 720);
  const narrowScroll = await transcriptState(win);
  assert.ok(narrowScroll.scrollHeight > narrowScroll.clientHeight, JSON.stringify(narrowScroll));
  const openLayout = await layoutOk(win, '.paymentRequestDetails[open]');
  assert.ok(openLayout.ok, JSON.stringify(openLayout));
  const cardLayoutNarrow = await layoutOk(win, '.paymentRequest');
  assert.ok(cardLayoutNarrow.ok, JSON.stringify(cardLayoutNarrow));
  assert.ok(await win.webContents.executeJavaScript(`
    (function() {
      var card = document.querySelector('.paymentRequest[data-request-id="${coffeeId}"]');
      if (!card) return false;
      card.scrollIntoView({ behavior: 'instant', block: 'start', inline: 'nearest' });
      return true;
    })()
  `), 'payment card missing for capture');
  const framedCard = await waitUntil(
    async () => {
      const frame = await win.webContents.executeJavaScript(`
        (function() {
          var transcript = document.getElementById('messageList');
          var card = document.querySelector('.paymentRequest[data-request-id="${coffeeId}"]');
          var heading = card && card.querySelector('.paymentRequestKicker');
          var amount = card && card.querySelector('.paymentRequestAmount');
          if (!transcript || !card || !heading || !amount) {
            return { ok: false, reason: 'missing-card-content' };
          }
          var transcriptBox = transcript.getBoundingClientRect();
          var transcriptRect = {
            left: transcriptBox.left + transcript.clientLeft,
            top: transcriptBox.top + transcript.clientTop,
            right: transcriptBox.left + transcript.clientLeft + transcript.clientWidth,
            bottom: transcriptBox.top + transcript.clientTop + transcript.clientHeight
          };
          var viewport = { left: 0, top: 0, right: window.innerWidth, bottom: window.innerHeight };
          function visibleIntersection(node) {
            var rect = node.getBoundingClientRect();
            var intersection = {
              left: Math.max(rect.left, transcriptRect.left, viewport.left),
              top: Math.max(rect.top, transcriptRect.top, viewport.top),
              right: Math.min(rect.right, transcriptRect.right, viewport.right),
              bottom: Math.min(rect.bottom, transcriptRect.bottom, viewport.bottom)
            };
            intersection.width = Math.max(0, intersection.right - intersection.left);
            intersection.height = Math.max(0, intersection.bottom - intersection.top);
            return {
              rect: { left: rect.left, top: rect.top, right: rect.right, bottom: rect.bottom },
              intersection: intersection,
              visible: intersection.width >= 1 && intersection.height >= 1
            };
          }
          var headingVisibility = visibleIntersection(heading);
          var amountVisibility = visibleIntersection(amount);
          return {
            ok: headingVisibility.visible && amountVisibility.visible,
            transcript: {
              left: transcriptRect.left,
              top: transcriptRect.top,
              right: transcriptRect.right,
              bottom: transcriptRect.bottom
            },
            viewport: viewport,
            heading: headingVisibility,
            amount: amountVisibility
          };
        })()
      `);
      return frame.ok ? frame : false;
    },
    3000,
    'payment card heading and amount were not visible for capture'
  );
  assert.ok(framedCard.ok, JSON.stringify(framedCard));
  shots.push(await capture(win, 'fixture-request-card-980x720.png', 980, 720));

  await clickElement(win, '#messageInput');
  win.webContents.insertText('preserved composer draft');
  await waitUntil(
    () => win.webContents.executeJavaScript('document.getElementById("messageInput").value === "preserved composer draft"'),
    3000,
    'keyboard draft input missing'
  );

  await clickElement(win, '.paymentRequestDetails summary');
  await waitUntil(
    () => win.webContents.executeJavaScript('document.querySelector(".paymentRequestDetails").open === false'),
    3000,
    'pointer did not close details before focus proof'
  );
  await sendEnter(win);
  await waitUntil(
    () => win.webContents.executeJavaScript('document.querySelector(".paymentRequestDetails").open === true'),
    3000,
    'keyboard did not reopen details before focus proof'
  );
  await win.webContents.executeJavaScript(`
    (function() {
      var list = document.getElementById('messageList');
      window.__stableTextNode = Array.prototype.find.call(document.querySelectorAll('.message'), function(node) {
        return node.querySelector('p').textContent.indexOf('long-message-20') !== -1;
      });
      list.scrollTop = Math.floor((list.scrollHeight - list.clientHeight) * 0.55);
      return true;
    })()
  `);
  const anchorBefore = await transcriptState(win);
  assert.ok(anchorBefore.key, JSON.stringify(anchorBefore));
  assert.ok(anchorBefore.summaryFocused, JSON.stringify(anchorBefore));
  assert.ok(anchorBefore.stableTextNode, JSON.stringify(anchorBefore));

  chatMessages[PAYEE] = chatMessages[PAYEE].concat([{
    messageId: 'fixture-status-injected-081',
    message: 'status-and-text-update',
    timestamp: '2026-08-30T12:10:00Z',
    outgoing: false,
    read: true,
    peerId: PAYEE,
  }]);
  setPayloadRecords([coffee, recordFrom('cancel-coffee')]);
  await waitUntil(
    () => win.webContents.executeJavaScript(`
      document.querySelector('.paymentRequest[data-request-id="${coffeeId}"] .paymentRequestMeta').innerText.indexOf('cancelled') !== -1 &&
      document.getElementById('messageList').innerText.indexOf('status-and-text-update') !== -1
    `),
    12000,
    'combined status and text update missing'
  );
  const anchorAfter = await transcriptState(win);
  assert.ok(anchorAfter.summaryFocused, JSON.stringify(anchorAfter));
  assert.ok(anchorAfter.detailsOpen, JSON.stringify(anchorAfter));
  assert.strictEqual(anchorAfter.key, anchorBefore.key, JSON.stringify({ anchorBefore, anchorAfter }));
  assert.ok(Math.abs(anchorAfter.offset - anchorBefore.offset) <= 1, JSON.stringify({ anchorBefore, anchorAfter }));
  assert.ok(Math.abs(anchorAfter.scrollTop - anchorBefore.scrollTop) <= 1, JSON.stringify({ anchorBefore, anchorAfter }));
  assert.ok(anchorAfter.stableTextNode, 'unchanged text message node was replaced');
  assert.strictEqual(
    await win.webContents.executeJavaScript('document.getElementById("messageInput").value'),
    'preserved composer draft'
  );

  await win.webContents.executeJavaScript(`
    var list = document.getElementById('messageList');
    list.scrollTop = list.scrollHeight;
    true
  `);
  chatMessages[PAYEE] = chatMessages[PAYEE].concat([{
    messageId: 'fixture-bottom-follow-082',
    message: 'bottom-follow-update',
    timestamp: '2026-08-30T12:11:00Z',
    outgoing: false,
    read: true,
    peerId: PAYEE,
  }]);
  await waitUntil(
    () => win.webContents.executeJavaScript('document.getElementById("messageList").innerText.indexOf("bottom-follow-update") !== -1'),
    12000,
    'bottom follow text missing'
  );
  const bottomAfter = await transcriptState(win);
  assert.ok(bottomAfter.bottomGap <= 24, JSON.stringify(bottomAfter));

  conversations = conversations.concat([{
    peerId: OTHER,
    timestamp: '2026-08-30T12:20:00Z',
    lastMessage: 'second-peer',
    outgoing: false,
    unread: 1,
  }]);
  await waitUntil(
    () => win.webContents.executeJavaScript(`
      Array.prototype.some.call(document.querySelectorAll('.conversationRow'), function(row) {
        return row.getAttribute('data-peer-id') === ${JSON.stringify(OTHER)};
      })
    `),
    12000,
    'second peer row missing from conversation list'
  );
  await win.webContents.executeJavaScript('document.getElementById("closeChatButton").click(); true');
  await waitUntil(
    () => win.webContents.executeJavaScript('document.getElementById("conversation").classList.contains("hidden")'),
    3000,
    'conversation did not close'
  );
  const secondVisible = await win.webContents.executeJavaScript(`
    (function() {
      var row = Array.prototype.find.call(document.querySelectorAll('.conversationRow'), function(item) {
        return item.getAttribute('data-peer-id') === ${JSON.stringify(OTHER)};
      });
      if (!row) return { ok: false };
      var r = row.getBoundingClientRect();
      return { ok: r.height > 1 && row.innerText.indexOf('second-peer') !== -1 };
    })()
  `);
  assert.ok(secondVisible.ok, JSON.stringify(secondVisible));
  await win.setSize(1440, 1000);
  const mixedLayout = await layoutOk(win, '.conversationRow');
  assert.ok(mixedLayout.ok, JSON.stringify(mixedLayout));
  shots.push(await capture(win, 'fixture-mixed-1440x1000.png', 1440, 1000));

  await clickElement(win, `.conversationRow[data-peer-id="${PAYEE}"]`);
  await waitUntil(
    () => win.webContents.executeJavaScript(
      `document.querySelector('.paymentRequest[data-request-id="${coffeeId}"] .paymentRequestMeta').innerText.indexOf('cancelled') !== -1`
    ),
    8000,
    'exact cancelled request missing'
  );
  const late = recordFrom('inbound-zec-late');
  const lateId = JSON.parse(late.signed.canonical).request_id;
  injectedNow = Date.parse('2026-08-30T12:04:59Z');
  setPayloadRecords([coffee, recordFrom('cancel-coffee'), late]);
  await waitUntil(
    () => win.webContents.executeJavaScript(
      `Array.prototype.some.call(document.querySelectorAll('.paymentRequest'), function(card) {
        return card.getAttribute('data-request-id') === ${JSON.stringify(lateId)} &&
          card.innerText.indexOf('requested') !== -1;
      })`
    ),
    12000,
    'pre-expiry status missing'
  );
  await win.webContents.executeJavaScript(`
    window.__lateCard = document.querySelector('.paymentRequest[data-request-id="${lateId}"]');
    true
  `);
  injectedNow = Date.parse('2026-08-30T12:05:01Z');
  await waitUntil(
    () => win.webContents.executeJavaScript(
      `var card = document.querySelector('.paymentRequest[data-request-id="${lateId}"]');
       card && card === window.__lateCard && card.innerText.indexOf('expired') !== -1`
    ),
    12000,
    'existing card did not cross expiry boundary in place'
  );

  socialPeerId = 'other-local-identity';
  await win.webContents.executeJavaScript(`
    document.getElementById('apiURLInput').value = ${JSON.stringify(socialOrigin)};
    document.getElementById('saveConnectionButton').click();
    true
  `);
  await waitUntil(
    () => win.webContents.executeJavaScript(
      'document.getElementById("paymentNotice").innerText.indexOf("this identity") !== -1'
    ),
    12000,
    'mismatched identity notice missing'
  );
  assert.strictEqual(
    await win.webContents.executeJavaScript('document.querySelectorAll(".paymentRequest").length'),
    0
  );

  socialPeerId = LOCAL;
  await win.webContents.executeJavaScript(`
    document.getElementById('apiURLInput').value = ${JSON.stringify(socialOrigin)};
    document.getElementById('saveConnectionButton').click();
    true
  `);
  await waitUntil(
    () => win.webContents.executeJavaScript(`
      Array.prototype.some.call(document.querySelectorAll('.conversationRow'), function(row) {
        return row.getAttribute('data-peer-id') === ${JSON.stringify(PAYEE)};
      })
    `),
    12000,
    'identity recovery peer row missing'
  );
  await clickElement(win, `.conversationRow[data-peer-id="${PAYEE}"]`);
  await waitUntil(
    () => win.webContents.executeJavaScript(
      `Boolean(document.querySelector('.paymentRequest[data-request-id="${coffeeId}"]'))`
    ),
    8000,
    'identity recovery request card missing after reopening peer'
  );

  await new Promise((resolve) => server.close(() => resolve()));
  owned.server = null;
  await waitUntil(
    () => win.webContents.executeJavaScript(
      'document.getElementById("paymentNotice").innerText.indexOf("Payment requests unavailable") !== -1'
    ),
    12000,
    'payment unavailable notice missing'
  );
  await win.setSize(980, 720);
  shots.push(await capture(win, 'fixture-unavailable-980x720.png', 980, 720));
  await new Promise((resolve, reject) => {
    server.once('error', reject);
    server.listen(payPort, '127.0.0.1', resolve);
  });
  owned.server = server;
  assert.strictEqual(server.address().port, payPort, 'payment fixture restarted on a different endpoint');
  await waitUntil(
    () => win.webContents.executeJavaScript(`
      document.getElementById('paymentNotice').classList.contains('hidden') &&
      Boolean(document.querySelector('.paymentRequest[data-request-id="${coffeeId}"]'))
    `),
    12000,
    'payment fixture did not recover automatically at the same endpoint'
  );
  assert.strictEqual(pickerCanceled, 0);
  assert.ok(socialHits.some((item) => item.path === '/ob/config' && item.method === 'GET'));
  assert.ok(expectedBlocked.some((item) => item.kind === 'default-http'), JSON.stringify(expectedBlocked));
  assert.ok(expectedBlocked.some((item) => item.kind === 'owned-social-ws'), JSON.stringify(expectedBlocked));
  assert.strictEqual(blocked.length, 0, JSON.stringify(blocked.slice(0, 8)));
  assert.strictEqual(fixtureFailures.length, 0, JSON.stringify(fixtureFailures.slice(0, 8)));
  assert.ok(paymentHits.length >= 3, JSON.stringify(paymentHits));
  assert.ok(paymentHits.every((item) => item.headers.origin === undefined), JSON.stringify(paymentHits));

  fs.writeFileSync(path.join(artifactRoot, 'electron-smoke.json'), `${JSON.stringify({
    note: 'Screenshots show in-process HTTP fixtures, not a live wallet or owner daemon.',
    social_hits: socialHits.slice(0, 12).map((item) => ({ method: item.method, path: item.path, origin: item.origin })),
    payment_hits: paymentHits.slice(0, 12).map((item) => ({ method: item.method, url: item.url })),
    fixture_failures: fixtureFailures.slice(0, 8),
    expected_blocked: expectedBlocked.slice(0, 8),
    unexpected_blocked: blocked.slice(0, 8),
    shots,
  }, null, 2)}\n`);

  cleanup();
  app.exit(0);
}

main().catch(async (error) => {
  if (!bootstrap.outcome) {
    try {
      setBootstrapPhase(bootstrap.phase === 'complete' ? 'diagnostics-failure' : bootstrap.phase);
      writeBootstrapDiagnostics('failure');
    } catch (diagnosticError) {
      process.stderr.write(`bootstrap diagnostics failed: ${diagnosticError && diagnosticError.message ? diagnosticError.message : diagnosticError}\n`);
    }
  }
  try {
    await writeUIFailureDiagnostics();
  } catch (diagnosticError) {
    process.stderr.write(`UI failure diagnostics failed: ${diagnosticError && diagnosticError.message ? diagnosticError.message : diagnosticError}\n`);
  }
  process.stderr.write(`${error && error.stack ? error.stack : error}\n`);
  cleanup();
  app.exit(1);
});
