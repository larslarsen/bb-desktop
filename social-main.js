'use strict';

const path = require('path');
const { app, BrowserWindow, Menu, ipcMain, session } = require('electron');
const { createWalletSupervisor } = require('./wallet-broker/supervisor');

app.enableSandbox();

let window;
const walletSupervisor = createWalletSupervisor();
const ID = /^[0-9a-f]{32}$/;

function dataDescriptors(value) {
  if (value === null || typeof value !== 'object' || Array.isArray(value) ||
      Object.getPrototypeOf(value) !== Object.prototype) return null;
  const descriptors = Object.getOwnPropertyDescriptors(value);
  if (Object.values(descriptors).some(
    (descriptor) => !Object.prototype.hasOwnProperty.call(descriptor, 'value')
  )) return null;
  return descriptors;
}

function exactData(value, keys) {
  const descriptors = dataDescriptors(value);
  return descriptors && Object.keys(descriptors).length === keys.length &&
    keys.every((key) => Object.prototype.hasOwnProperty.call(descriptors, key))
    ? descriptors : null;
}

function safeTree(value, seen = new Set()) {
  if (value === null || ['string', 'boolean'].includes(typeof value)) return true;
  if (typeof value === 'number') return Number.isFinite(value);
  if (typeof value !== 'object' || seen.has(value)) return false;
  seen.add(value);
  const descriptors = Object.getOwnPropertyDescriptors(value);
  if (Object.values(descriptors).some(
    (descriptor) => !Object.prototype.hasOwnProperty.call(descriptor, 'value')
  )) return false;
  const prototype = Object.getPrototypeOf(value);
  if (Array.isArray(value) ? prototype !== Array.prototype : prototype !== Object.prototype) return false;
  return Object.values(descriptors).every((descriptor) => safeTree(descriptor.value, seen));
}

function cloneBoundary(value) {
  if (value === null || typeof value !== 'object') return value;
  if (Array.isArray(value)) return value.map(cloneBoundary);
  const result = {};
  for (const [key, descriptor] of Object.entries(Object.getOwnPropertyDescriptors(value))) {
    result[key] = cloneBoundary(descriptor.value);
  }
  return result;
}

function requireFrame(event) {
  if (!window || !event || event.senderFrame !== window.webContents.mainFrame ||
      event.sender !== window.webContents ||
      typeof event.sender.getURL !== 'function' ||
      event.sender.getURL() !== `file://${path.join(__dirname, 'social', 'index.html')}` ||
      event.senderFrame.url !== `file://${path.join(__dirname, 'social', 'index.html')}`) {
    throw new Error('untrusted wallet IPC sender');
  }
}

function closedParams(channel, value) {
  if (channel === 'wallet:snapshot:get' || channel === 'wallet:accounts:list') {
    if (value !== undefined) throw new Error('unexpected wallet IPC payload');
    return undefined;
  }
  let descriptors;
  if (channel === 'wallet:intent:begin') {
    descriptors = exactData(value, ['payment_request']);
    if (!descriptors || !dataDescriptors(descriptors.payment_request.value) ||
        !safeTree(descriptors.payment_request.value)) throw new Error('invalid payment request');
  } else if (channel === 'wallet:intent:cancel') {
    descriptors = exactData(value, ['intent_id']);
    if (!descriptors || typeof descriptors.intent_id.value !== 'string' ||
        !ID.test(descriptors.intent_id.value)) throw new Error('invalid intent id');
  } else if (channel === 'wallet:payee-request:get') {
    descriptors = exactData(value, ['account_id', 'asset', 'network', 'request_id']);
    if (!descriptors) throw new Error('invalid receiver request');
    const asset = descriptors.asset.value;
    const network = descriptors.network.value;
    const networks = asset === 'ZEC'
      ? ['zec-mainnet', 'zec-testnet', 'zec-regtest']
      : asset === 'XMR' ? ['xmr-mainnet', 'xmr-stagenet', 'xmr-testnet'] : [];
    if (typeof descriptors.account_id.value !== 'string' ||
        !ID.test(descriptors.account_id.value) ||
        typeof descriptors.request_id.value !== 'string' ||
        !ID.test(descriptors.request_id.value) ||
        !networks.includes(network)) throw new Error('invalid receiver request');
  } else {
    throw new Error('unknown wallet IPC channel');
  }
  const cloned = cloneBoundary(value);
  if (Buffer.byteLength(JSON.stringify(cloned), 'utf8') > 64 * 1024) {
    throw new Error('wallet IPC payload exceeds limit');
  }
  return cloned;
}

function walletHandler(channel, method) {
  return (event, value) => {
    requireFrame(event);
    const params = closedParams(channel, value);
    const result = params === undefined
      ? walletSupervisor.dispatch(method)
      : walletSupervisor.dispatch(method, params);
    return cloneBoundary(result);
  };
}

function snapshotData(value, key) {
  const descriptor = value && typeof value === 'object'
    ? Object.getOwnPropertyDescriptor(value, key) : null;
  return descriptor && Object.prototype.hasOwnProperty.call(descriptor, 'value')
    ? descriptor.value : undefined;
}

function plainSnapshotObject(value) {
  return value !== null && typeof value === 'object' && !Array.isArray(value) &&
    Object.getPrototypeOf(value) === Object.prototype;
}

function snapshotCapabilities(value) {
  const result = {};
  if (!plainSnapshotObject(value)) return result;
  for (const [key, descriptor] of Object.entries(Object.getOwnPropertyDescriptors(value))) {
    if (Object.prototype.hasOwnProperty.call(descriptor, 'value') &&
        /^[a-z][a-z0-9_]*$/.test(key) && typeof descriptor.value === 'boolean') {
      result[key] = descriptor.value;
    }
  }
  return result;
}

function sanitizeSnapshot(value) {
  const broker = ['down', 'locked', 'ready', 'syncing', 'degraded'].includes(snapshotData(value, 'broker'))
    ? snapshotData(value, 'broker') : 'down';
  const sourceAccounts = snapshotData(value, 'accounts');
  const accounts = [];
  if (Array.isArray(sourceAccounts)) {
    for (const source of sourceAccounts.slice(0, 256)) {
      if (!plainSnapshotObject(source)) continue;
      const accountId = snapshotData(source, 'account_id');
      if (typeof accountId !== 'string' || !ID.test(accountId)) continue;
      const account = { account_id: accountId };
      for (const key of ['label', 'asset', 'network', 'kind', 'privacy', 'balance_atomic']) {
        if (typeof snapshotData(source, key) === 'string') account[key] = snapshotData(source, key);
      }
      account.capabilities = snapshotCapabilities(snapshotData(source, 'capabilities'));
      const sync = snapshotData(source, 'sync');
      if (plainSnapshotObject(sync)) {
        account.sync = {};
        if (typeof snapshotData(sync, 'state') === 'string') account.sync.state = snapshotData(sync, 'state');
        if (typeof snapshotData(sync, 'progress') === 'number' &&
            Number.isFinite(snapshotData(sync, 'progress'))) account.sync.progress = snapshotData(sync, 'progress');
      }
      const device = snapshotData(source, 'device');
      if (plainSnapshotObject(device)) {
        account.device = {};
        if (typeof snapshotData(device, 'present') === 'boolean') account.device.present = snapshotData(device, 'present');
        if (typeof snapshotData(device, 'label') === 'string') account.device.label = snapshotData(device, 'label');
        const verified = snapshotData(device, 'verified_fields');
        if (Array.isArray(verified) && verified.every((field) => typeof field === 'string')) {
          account.device.verified_fields = verified.slice();
        }
      }
      accounts.push(account);
    }
  }
  return { v: 1, broker, accounts };
}

function denyNavigation(event) {
  event.preventDefault();
}

function createWindow() {
  window = new BrowserWindow({
    width: 1180,
    height: 780,
    minWidth: 860,
    minHeight: 620,
    center: true,
    title: 'BitBook',
    backgroundColor: '#0d1117',
    icon: path.join(__dirname, 'imgs', 'icon.png'),
    webPreferences: {
      preload: path.join(__dirname, 'wallet-preload.js'),
      nodeIntegration: false,
      contextIsolation: true,
      sandbox: true,
      webSecurity: true,
      allowRunningInsecureContent: false,
      experimentalFeatures: false,
    },
  });
  window.loadFile(path.join(__dirname, 'social', 'index.html'));
  window.webContents.on('will-navigate', denyNavigation);
  window.webContents.on('will-redirect', denyNavigation);
  window.webContents.on('will-attach-webview', denyNavigation);
  window.webContents.setWindowOpenHandler(() => ({ action: 'deny' }));
  window.on('closed', () => {
    window = null;
  });
}

app.on('ready', () => {
  session.defaultSession.setPermissionRequestHandler((_contents, _permission, callback) => callback(false));
  session.defaultSession.setPermissionCheckHandler(() => false);
  Menu.setApplicationMenu(null);
  createWindow();
  ipcMain.handle('wallet:snapshot:get', walletHandler('wallet:snapshot:get', 'status.get'));
  ipcMain.handle('wallet:accounts:list', walletHandler('wallet:accounts:list', 'account.list'));
  ipcMain.handle('wallet:intent:begin', walletHandler('wallet:intent:begin', 'intent.begin'));
  ipcMain.handle('wallet:intent:cancel', walletHandler('wallet:intent:cancel', 'intent.cancel'));
  ipcMain.handle('wallet:payee-request:get', walletHandler('wallet:payee-request:get', 'receiver.fresh'));
  walletSupervisor.subscribeSnapshot((value) => {
    if (!window) return;
    const snapshot = cloneBoundary(sanitizeSnapshot(value));
    window.webContents.send('wallet:snapshot:subscribe', snapshot);
  });
});

app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') {
    app.quit();
  }
});

app.on('activate', () => {
  if (window === null) {
    createWindow();
  }
});
