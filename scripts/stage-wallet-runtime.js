'use strict';

const fs = require('fs');
const path = require('path');

const INVENTORY = Object.freeze([
  'wallet-preload.js',
  'wallet-broker/protocol.js',
  'wallet-broker/supervisor.js',
  'wallet-broker/launch-config.js',
  'wallet-pay/model.js',
]);
const SOURCE_PARENTS = Object.freeze([
  'wallet-broker',
  'wallet-pay',
]);

function joinInv(root, rel) {
  return path.join(root, ...String(rel).split('/'));
}

function assertAbsolutePath(value) {
  if (typeof value !== 'string' || value === '' || !path.isAbsolute(value)) {
    throw new Error('root must be a nonempty absolute path');
  }
}

function assertRealDirectory(abs) {
  const st = fs.lstatSync(abs);
  if (st.isSymbolicLink() || !st.isDirectory()) {
    throw new Error(`not a real directory: ${abs}`);
  }
}

function assertRegularFile(abs) {
  const st = fs.lstatSync(abs);
  if (st.isSymbolicLink() || !st.isFile()) {
    throw new Error(`not a regular file: ${abs}`);
  }
}

function assertAbsent(abs) {
  let st;
  try {
    st = fs.lstatSync(abs);
  } catch (error) {
    if (error.code === 'ENOENT') return;
    throw error;
  }
  throw new Error(`destination already exists: ${abs}`);
}

function assertRealDirectoryOrAbsent(abs) {
  let st;
  try {
    st = fs.lstatSync(abs);
  } catch (error) {
    if (error.code === 'ENOENT') return;
    throw error;
  }
  if (st.isSymbolicLink() || !st.isDirectory()) {
    throw new Error(`destination parent is not a real directory: ${abs}`);
  }
}

function stageWalletRuntime(sourceRoot, appRoot) {
  assertAbsolutePath(sourceRoot);
  assertAbsolutePath(appRoot);
  assertRealDirectory(sourceRoot);
  assertRealDirectory(appRoot);

  for (const rel of SOURCE_PARENTS) {
    assertRealDirectory(joinInv(sourceRoot, rel));
  }
  for (const rel of INVENTORY) {
    assertRegularFile(joinInv(sourceRoot, rel));
  }
  for (const rel of SOURCE_PARENTS) {
    assertRealDirectoryOrAbsent(joinInv(appRoot, rel));
  }
  for (const rel of INVENTORY) {
    assertAbsent(joinInv(appRoot, rel));
  }

  for (const rel of SOURCE_PARENTS) {
    const destParent = joinInv(appRoot, rel);
    try {
      fs.lstatSync(destParent);
    } catch (error) {
      if (error.code !== 'ENOENT') throw error;
      fs.mkdirSync(destParent);
    }
  }
  for (const rel of INVENTORY) {
    const dest = joinInv(appRoot, rel);
    fs.copyFileSync(joinInv(sourceRoot, rel), dest, fs.constants.COPYFILE_EXCL);
    fs.chmodSync(dest, 0o644);
  }
}

if (require.main === module) {
  try {
    if (process.argv.length !== 3) {
      throw new Error('exactly one argument required');
    }
    stageWalletRuntime(path.resolve(__dirname, '..'), process.argv[2]);
  } catch (error) {
    process.stderr.write('Unable to stage wallet runtime\n');
    process.exit(1);
  }
}

module.exports = { stageWalletRuntime };
