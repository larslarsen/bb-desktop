'use strict';

const path = require('path');
const { app, BrowserWindow, Menu, ipcMain, session, dialog } = require('electron');
const { resolveWalletBrokerLaunch } = require('./wallet-broker/launch-config');
const { createWalletSupervisor } = require('./wallet-broker/supervisor');
const { sanitizeWalletSnapshot } = require('./wallet-pay/model');

app.enableSandbox();

let window;
let quitState = 'idle';
let walletSupervisor = createWalletSupervisor();
let walletStartupAttempted = false;
const ID = /^[0-9a-f]{32}$/;
const WALLET_ENV = Object.freeze([
  'LANG',
  'PATH',
  'DISPLAY',
  'WAYLAND_DISPLAY',
  'XDG_RUNTIME_DIR',
  'XAUTHORITY',
  'DBUS_SESSION_BUS_ADDRESS',
]);
const ENV_VALUE_LIMIT = 4096;

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

function walletEnvironment(source) {
  const result = {};
  for (const name of WALLET_ENV) {
    const descriptor = Object.getOwnPropertyDescriptor(source, name);
    if (!descriptor || !Object.prototype.hasOwnProperty.call(descriptor, 'value') ||
        typeof descriptor.value !== 'string' || descriptor.value.includes('\0') ||
        Buffer.byteLength(descriptor.value, 'utf8') > ENV_VALUE_LIMIT) continue;
    result[name] = descriptor.value;
  }
  return result;
}

let cachedWalletStatus = cloneBoundary(sanitizeWalletSnapshot({ v: 1, broker: 'down', accounts: [] }));

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
    if (channel === 'wallet:snapshot:get' && !walletSupervisor.bound) {
      return Promise.resolve(cloneBoundary(cachedWalletStatus));
    }
    const result = params === undefined
      ? walletSupervisor.dispatch(method)
      : walletSupervisor.dispatch(method, params);
    return Promise.resolve(result).then(cloneBoundary);
  };
}

function denyNavigation(event) {
  event.preventDefault();
}

function approveNormalQuit() {
  if (quitState !== 'pending') return;
  quitState = 'approved';
  app.quit();
}

function failNormalQuit() {
  if (quitState !== 'pending') return;
  quitState = 'failed';
  try {
    dialog.showErrorBox(
      'Unable to close BitBook',
      'Wallet shutdown could not be confirmed. BitBook will keep running.'
    );
  } catch (_) {}
}

function showAccountsUnavailable() {
  if (quitState !== 'idle') return;
  try {
    dialog.showErrorBox(
      'Accounts unavailable',
      'The account window could not be opened. Please restart BitBook.'
    );
  } catch (_) {}
}

function manageAccounts() {
  if (quitState !== 'idle') return;
  if (!walletSupervisor.bound) {
    showAccountsUnavailable();
    return;
  }
  let result;
  try {
    result = walletSupervisor.dispatch('account.manage', {});
  } catch (_) {
    showAccountsUnavailable();
    return;
  }
  Promise.resolve(result).then(() => {}, () => showAccountsUnavailable());
}

function installApplicationMenu() {
  const menu = Menu.buildFromTemplate([
    {
      label: 'Wallet',
      submenu: [
        { label: 'Manage accounts', click: manageAccounts },
      ],
    },
  ]);
  Menu.setApplicationMenu(menu);
}

app.on('before-quit', (event) => {
  if (quitState === 'approved') return;
  event.preventDefault();
  if (quitState !== 'idle') return;
  quitState = 'pending';
  try {
    walletSupervisor.shutdown().then(approveNormalQuit, failNormalQuit).then(() => {}, () => {});
  } catch (_) {
    failNormalQuit();
    return;
  }
});

function startWalletBroker() {
  if (quitState !== 'idle') return;
  try {
    const launch = resolveWalletBrokerLaunch({
      resourcesPath: app.isPackaged === true
        ? process.resourcesPath
        : path.join(__dirname, 'wallet-broker', 'target', 'app-resources'),
      userDataPath: app.getPath('userData'),
      platform: process.platform,
      arch: process.arch,
    });
    const configured = createWalletSupervisor({
      brokerPath: launch.brokerPath,
      expectedSha256: launch.expectedSha256,
      dataDir: launch.dataDir,
      env: walletEnvironment(process.env),
    });
    if (quitState !== 'idle') return;
    walletSupervisor = configured;
    configured.subscribeSnapshot((value) => {
      cachedWalletStatus = cloneBoundary(sanitizeWalletSnapshot(value));
      if (!window) return;
      window.webContents.send('wallet:snapshot:subscribe', cloneBoundary(cachedWalletStatus));
    });
    configured.start();
  } catch (_) {}
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
  installApplicationMenu();
  if (quitState !== 'idle' || walletStartupAttempted) return;
  walletStartupAttempted = true;
  createWindow();
  ipcMain.handle('wallet:snapshot:get', walletHandler('wallet:snapshot:get', 'status.get'));
  ipcMain.handle('wallet:accounts:list', walletHandler('wallet:accounts:list', 'account.list'));
  ipcMain.handle('wallet:intent:begin', walletHandler('wallet:intent:begin', 'intent.begin'));
  ipcMain.handle('wallet:intent:cancel', walletHandler('wallet:intent:cancel', 'intent.cancel'));
  ipcMain.handle('wallet:payee-request:get', walletHandler('wallet:payee-request:get', 'receiver.fresh'));
  startWalletBroker();
});

app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') {
    app.quit();
  }
});

app.on('activate', () => {
  if (quitState !== 'idle') return;
  if (window === null) {
    createWindow();
  }
});
