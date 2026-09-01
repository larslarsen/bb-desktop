'use strict';

const path = require('path');
const { app, BrowserWindow, Menu, ipcMain, session } = require('electron');
const { createWalletSupervisor } = require('./wallet-broker/supervisor');
const { sanitizeWalletSnapshot } = require('./wallet-pay/model');

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
    const snapshot = cloneBoundary(sanitizeWalletSnapshot(value));
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
