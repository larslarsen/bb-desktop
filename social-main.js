'use strict';

const path = require('path');
const { app, BrowserWindow, Menu, session } = require('electron');

app.enableSandbox();

let window;

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
