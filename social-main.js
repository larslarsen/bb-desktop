'use strict';

const path = require('path');
const { app, BrowserWindow, Menu, shell } = require('electron');

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
    },
  });
  window.loadFile(path.join(__dirname, 'social', 'index.html'));
  window.webContents.setWindowOpenHandler(({ url }) => {
    shell.openExternal(url);
    return { action: 'deny' };
  });
  window.on('closed', () => {
    window = null;
  });
}

app.on('ready', () => {
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
