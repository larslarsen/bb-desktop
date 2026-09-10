'use strict';

const fs = require('fs');
const path = require('path');

const UNAVAILABLE_MESSAGE = 'Wallet broker is unavailable';
const BROKER_DIRNAME = 'wallet-broker';
const MANIFEST_NAME = 'manifest.json';
const MANIFEST_MIN_BYTES = 1;
const MANIFEST_MAX_BYTES = 4096;
const OPTION_KEYS = Object.freeze(['resourcesPath', 'userDataPath', 'platform', 'arch']);
const MANIFEST_KEYS = Object.freeze(['v', 'platform', 'arch', 'sha256']);
const PLATFORMS = Object.freeze(['linux', 'darwin', 'win32']);
const ARCHES = Object.freeze(['x64', 'arm64']);
const SHA256 = /^[0-9a-f]{64}$/;

function unavailable() {
  const error = new Error(UNAVAILABLE_MESSAGE);
  error.code = 'UNAVAILABLE';
  throw error;
}

function closedData(value, keys) {
  if (value === null || typeof value !== 'object' || Array.isArray(value) ||
      Object.getPrototypeOf(value) !== Object.prototype) {
    return null;
  }
  // Count symbols too; read descriptors so accessors are never invoked.
  if (Reflect.ownKeys(value).length !== keys.length) return null;
  const descriptors = Object.getOwnPropertyDescriptors(value);
  for (const key of keys) {
    if (!Object.prototype.hasOwnProperty.call(descriptors, key)) return null;
    if (!Object.prototype.hasOwnProperty.call(descriptors[key], 'value')) return null;
  }
  return descriptors;
}

function absolutePath(value) {
  return typeof value === 'string' && value.length > 0 && path.isAbsolute(value);
}

function brokerBasename(platform) {
  return platform === 'win32' ? 'bitbook-wallet-broker.exe' : 'bitbook-wallet-broker';
}

function lstat(abs) {
  try {
    return fs.lstatSync(abs);
  } catch (_) {
    unavailable();
  }
}

function requireDirectory(abs) {
  const stat = lstat(abs);
  if (stat.isSymbolicLink() || !stat.isDirectory()) unavailable();
}

function requireRegularFile(abs) {
  const stat = lstat(abs);
  if (stat.isSymbolicLink() || !stat.isFile()) unavailable();
  return stat;
}

function parseManifest(bytes) {
  let parsed;
  try {
    parsed = JSON.parse(bytes.toString('utf8'));
  } catch (_) {
    unavailable();
  }
  const descriptors = closedData(parsed, MANIFEST_KEYS);
  if (!descriptors) unavailable();
  const version = descriptors.v.value;
  const platform = descriptors.platform.value;
  const arch = descriptors.arch.value;
  const sha256 = descriptors.sha256.value;
  if (typeof version !== 'number' || version !== 1) unavailable();
  if (typeof platform !== 'string' || typeof arch !== 'string') unavailable();
  // Length is required because JS `$` can match before a trailing newline.
  if (typeof sha256 !== 'string' || sha256.length !== 64 || !SHA256.test(sha256)) {
    unavailable();
  }
  return { platform, arch, sha256 };
}

function resolveWalletBrokerLaunchUnchecked(options) {
  const descriptors = closedData(options, OPTION_KEYS);
  if (!descriptors) unavailable();
  const resourcesPath = descriptors.resourcesPath.value;
  const userDataPath = descriptors.userDataPath.value;
  const platform = descriptors.platform.value;
  const arch = descriptors.arch.value;
  if (!absolutePath(resourcesPath) || !absolutePath(userDataPath)) unavailable();
  if (typeof platform !== 'string' || typeof arch !== 'string') unavailable();
  if (!PLATFORMS.includes(platform) || !ARCHES.includes(arch)) unavailable();

  const basename = brokerBasename(platform);
  const brokerDir = path.resolve(resourcesPath, BROKER_DIRNAME);
  const manifestPath = path.resolve(brokerDir, MANIFEST_NAME);
  const brokerPath = path.resolve(brokerDir, basename);

  requireDirectory(brokerDir);
  const manifestStat = requireRegularFile(manifestPath);
  if (!Number.isInteger(manifestStat.size) ||
      manifestStat.size < MANIFEST_MIN_BYTES ||
      manifestStat.size > MANIFEST_MAX_BYTES) {
    unavailable();
  }
  let bytes;
  try {
    bytes = fs.readFileSync(manifestPath);
  } catch (_) {
    unavailable();
  }
  if (!Buffer.isBuffer(bytes) ||
      bytes.length < MANIFEST_MIN_BYTES ||
      bytes.length > MANIFEST_MAX_BYTES) {
    unavailable();
  }
  const manifest = parseManifest(bytes);
  if (manifest.platform !== platform || manifest.arch !== arch) unavailable();
  requireRegularFile(brokerPath);

  return Object.freeze({
    brokerPath,
    expectedSha256: manifest.sha256,
    dataDir: path.resolve(userDataPath, BROKER_DIRNAME),
  });
}

function resolveWalletBrokerLaunch(options) {
  try {
    return resolveWalletBrokerLaunchUnchecked(options);
  } catch (_) {
    unavailable();
  }
}

module.exports = { resolveWalletBrokerLaunch };
