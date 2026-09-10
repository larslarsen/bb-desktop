'use strict';

const { spawnSync } = require('child_process');
const crypto = require('crypto');
const fs = require('fs');
const path = require('path');

const BROKER_DIRNAME = 'wallet-broker';
const MANIFEST_NAME = 'manifest.json';
const PLATFORMS = Object.freeze(['linux', 'darwin', 'win32']);
const ARCHES = Object.freeze(['x64', 'arm64']);
const CLI_ERROR = 'Unable to build wallet broker';
const STAGING_PREFIX = 'bb-wallet-broker-';

function invalid() {
  throw new Error('Invalid wallet broker staging request');
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
  } catch (error) {
    if (error && error.code === 'ENOENT') return null;
    throw error;
  }
}

function requireRealDirectory(abs) {
  const stat = lstat(abs);
  if (!stat || stat.isSymbolicLink() || !stat.isDirectory()) invalid();
}

function requireRealRegularFile(abs) {
  const stat = lstat(abs);
  if (!stat || stat.isSymbolicLink() || !stat.isFile()) invalid();
}

function requireAbsentOrRealRegularFile(abs) {
  const stat = lstat(abs);
  if (!stat) return;
  if (stat.isSymbolicLink() || !stat.isFile()) invalid();
}

function stageWalletBroker(sourceBinary, resourcesRoot, platform, arch) {
  if (!absolutePath(sourceBinary) || !absolutePath(resourcesRoot)) invalid();
  if (typeof platform !== 'string' || typeof arch !== 'string') invalid();
  if (!PLATFORMS.includes(platform) || !ARCHES.includes(arch)) invalid();

  requireRealRegularFile(sourceBinary);
  requireRealDirectory(resourcesRoot);

  const basename = brokerBasename(platform);
  const brokerDir = path.join(resourcesRoot, BROKER_DIRNAME);
  const destBinary = path.join(brokerDir, basename);
  const destManifest = path.join(brokerDir, MANIFEST_NAME);

  const brokerStat = lstat(brokerDir);
  if (brokerStat) {
    if (brokerStat.isSymbolicLink() || !brokerStat.isDirectory()) invalid();
    requireAbsentOrRealRegularFile(destBinary);
    requireAbsentOrRealRegularFile(destManifest);
  }

  if (!brokerStat) fs.mkdirSync(brokerDir);

  const stagingDir = fs.mkdtempSync(path.join(resourcesRoot, STAGING_PREFIX));
  const stagedBinary = path.join(stagingDir, basename);
  const stagedManifest = path.join(stagingDir, MANIFEST_NAME);

  fs.copyFileSync(sourceBinary, stagedBinary);
  fs.chmodSync(stagedBinary, 0o755);
  const copied = fs.readFileSync(stagedBinary);
  const sha256 = crypto.createHash('sha256').update(copied).digest('hex');
  fs.writeFileSync(stagedManifest, JSON.stringify({
    v: 1,
    platform,
    arch,
    sha256,
  }));
  fs.chmodSync(stagedManifest, 0o644);

  fs.renameSync(stagedBinary, destBinary);
  fs.renameSync(stagedManifest, destManifest);
}

function main() {
  if (process.argv.length !== 2) invalid();

  const repo = path.resolve(__dirname, '..');
  const result = spawnSync(
    'rustup',
    [
      'run',
      '1.98.0',
      'cargo',
      'build',
      '--manifest-path',
      path.join(repo, 'wallet-broker', 'Cargo.toml'),
      '--target-dir',
      path.join(repo, 'wallet-broker', 'target'),
      '--locked',
      '--offline',
      '--no-default-features',
      '--features',
      'native-ui',
      '--bin',
      'bitbook-wallet-broker',
    ],
    {
      cwd: repo,
      shell: false,
      stdio: 'inherit',
    }
  );
  if (!result || result.status !== 0) invalid();

  const resourcesRoot = path.join(repo, 'wallet-broker', 'target', 'app-resources');
  const resourcesStat = lstat(resourcesRoot);
  if (!resourcesStat) {
    fs.mkdirSync(resourcesRoot);
  } else if (resourcesStat.isSymbolicLink() || !resourcesStat.isDirectory()) {
    invalid();
  }

  const sourceBinary = path.join(
    repo,
    'wallet-broker',
    'target',
    'debug',
    brokerBasename(process.platform)
  );
  stageWalletBroker(sourceBinary, resourcesRoot, process.platform, process.arch);
}

module.exports = { stageWalletBroker };

if (require.main === module) {
  try {
    main();
  } catch (_) {
    process.stderr.write(`${CLI_ERROR}\n`);
    process.exit(1);
  }
}
