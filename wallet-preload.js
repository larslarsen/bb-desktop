'use strict';

const { contextBridge, ipcRenderer } = require('electron');

function cloneBoundary(value, seen = new Map()) {
  if (value === null || ['string', 'boolean', 'undefined'].includes(typeof value)) return value;
  if (typeof value === 'number' && Number.isFinite(value)) return value;
  if (typeof value !== 'object') throw new TypeError('unsupported bridge value');
  if (seen.has(value)) throw new TypeError('cyclic bridge value');
  const descriptors = Object.getOwnPropertyDescriptors(value);
  if (Object.values(descriptors).some(
    (descriptor) => !Object.prototype.hasOwnProperty.call(descriptor, 'value')
  )) throw new TypeError('accessors are not bridge values');
  const result = Array.isArray(value) ? [] : {};
  const prototype = Object.getPrototypeOf(value);
  if (Array.isArray(value) ? prototype !== Array.prototype : prototype !== Object.prototype) {
    throw new TypeError('non-plain bridge value');
  }
  seen.set(value, result);
  for (const [key, descriptor] of Object.entries(descriptors)) {
    result[key] = cloneBoundary(descriptor.value, seen);
  }
  return result;
}

function cloneResult(result) {
  return Promise.resolve(result).then((response) => cloneBoundary(response));
}

const getSnapshot = () => cloneResult(ipcRenderer.invoke('wallet:snapshot:get'));
const subscribeSnapshot = (callback) => {
  if (typeof callback !== 'function') throw new TypeError('snapshot callback must be a function');
  const listener = (_event, value) => {
    try { callback(cloneBoundary(value)); } catch (_) { /* renderer callback isolation */ }
  };
  ipcRenderer.on('wallet:snapshot:subscribe', listener);
  let active = true;
  return () => {
    if (!active) return false;
    active = false;
    ipcRenderer.removeListener('wallet:snapshot:subscribe', listener);
    return true;
  };
};
const beginIntent = (value) => cloneResult(
  ipcRenderer.invoke('wallet:intent:begin', cloneBoundary(value))
);
const cancelIntent = (value) => cloneResult(
  ipcRenderer.invoke('wallet:intent:cancel', cloneBoundary(value))
);
const listAccounts = () => cloneResult(ipcRenderer.invoke('wallet:accounts:list'));
const getPayeeRequest = (value) => cloneResult(
  ipcRenderer.invoke('wallet:payee-request:get', cloneBoundary(value))
);

const api = Object.create(null);
Object.assign(api, {
  getSnapshot,
  subscribeSnapshot,
  beginIntent,
  cancelIntent,
  listAccounts,
  getPayeeRequest,
});
for (const value of Object.values(api)) Object.freeze(value);
Object.freeze(api);

contextBridge.exposeInMainWorld('bitbookWallet', api);
