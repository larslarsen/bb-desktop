'use strict';

const path = require('path');
const { app, BrowserWindow, Menu, session, shell } = require('electron');

app.enableSandbox();

let window;

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
    },
  });
  window.loadFile(path.join(__dirname, 'social', 'index.html'));
  window.webContents.on('will-navigate', (event) => event.preventDefault());
  window.webContents.setWindowOpenHandler(({ url }) => {
    try {
      const target = new URL(url);
      if (target.protocol === 'https:' || target.protocol === 'http:') {
        shell.openExternal(target.toString());
      }
    } catch (_error) {
      // Malformed and non-web destinations remain blocked.
    }
    return { action: 'deny' };
  });
  window.on('closed', () => {
    window = null;
  });
}

app.on('ready', () => {
  session.defaultSession.setPermissionRequestHandler((_contents, _permission, callback) => callback(false));
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
