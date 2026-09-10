'use strict';

const assert = require('assert');
const crypto = require('crypto');
const fs = require('fs');
const os = require('os');
const path = require('path');
const vm = require('vm');
const { resolveWalletBrokerLaunch } = require('../wallet-broker/launch-config');

const REPO = path.resolve(__dirname, '..');
const MODULE_PATH = path.join(REPO, 'scripts', 'build-wallet-broker.js');
const MODULE_MISSING = 'wallet broker build helper is missing';
const CLI_ERROR = 'Unable to build wallet broker';
const CANARY = 'WAL011_BUILD_CANARY_SECRET';
const SENTINEL = 'WAL011_UNRELATED_SENTINEL';
const POSIX = process.platform !== 'win32';
const TARGET_ROOT = path.join(REPO, 'wallet-broker', 'target');
const IDENTITIES = Object.freeze([
  Object.freeze({ platform: 'linux', arch: 'x64', basename: 'bitbook-wallet-broker' }),
  Object.freeze({ platform: 'linux', arch: 'arm64', basename: 'bitbook-wallet-broker' }),
  Object.freeze({ platform: 'darwin', arch: 'x64', basename: 'bitbook-wallet-broker' }),
  Object.freeze({ platform: 'darwin', arch: 'arm64', basename: 'bitbook-wallet-broker' }),
  Object.freeze({ platform: 'win32', arch: 'x64', basename: 'bitbook-wallet-broker.exe' }),
  Object.freeze({ platform: 'win32', arch: 'arm64', basename: 'bitbook-wallet-broker.exe' }),
]);
const LINUX_X64 = IDENTITIES[0];
const HOST_BASENAME = process.platform === 'win32' ? 'bitbook-wallet-broker.exe' : 'bitbook-wallet-broker';

assert.strictEqual(IDENTITIES.length, 6);

const tests = [];
function test(name, fn) { tests.push({ name, fn }); }

function loadStager() {
  assert.ok(fs.existsSync(MODULE_PATH), MODULE_MISSING);
  const exported = require(MODULE_PATH);
  assert.strictEqual(typeof exported.stageWalletBroker, 'function');
  return exported.stageWalletBroker;
}

function appendCleanup(previous, error) {
  if (!previous) return error;
  previous.message = `${previous.message}\ncleanup also failed: ${error.stack || error.message}`;
  return previous;
}

function withOwnedTemp(fn) {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), 'bb-wal011-broker-build-'));
  const ownedFiles = [];
  const ownedLinks = [];
  const ownedDirs = [root];
  const ownedDirSet = new Set([root]);
  const ctx = {
    root,
    dir(rel) {
      let cursor = root;
      for (const part of String(rel).split(path.sep).filter(Boolean)) {
        cursor = path.join(cursor, part);
        if (!ownedDirSet.has(cursor)) {
          fs.mkdirSync(cursor);
          ownedDirs.push(cursor);
          ownedDirSet.add(cursor);
        }
      }
      return cursor;
    },
    file(rel, contents, mode) {
      const abs = path.resolve(root, rel);
      fs.writeFileSync(abs, contents);
      if (mode != null) fs.chmodSync(abs, mode);
      if (!ownedFiles.includes(abs)) ownedFiles.push(abs);
      return abs;
    },
    link(rel, target) {
      const abs = path.resolve(root, rel);
      fs.symlinkSync(target, abs);
      if (!ownedLinks.includes(abs)) ownedLinks.push(abs);
      return abs;
    },
    noteFile(abs) {
      if (!ownedFiles.includes(abs)) ownedFiles.push(abs);
      return abs;
    },
    noteLink(abs) {
      if (!ownedLinks.includes(abs)) ownedLinks.push(abs);
      return abs;
    },
    noteDir(abs) {
      if (!ownedDirSet.has(abs)) {
        ownedDirs.push(abs);
        ownedDirSet.add(abs);
      }
      return abs;
    },
  };
  let originalError = null;
  try {
    return fn(ctx);
  } catch (error) {
    originalError = error;
  } finally {
    let cleanupError = null;
    try { trackTree(ctx, root); } catch (error) { cleanupError = appendCleanup(cleanupError, error); }
    for (const abs of ownedLinks.slice().reverse()) {
      try { fs.unlinkSync(abs); } catch (error) { cleanupError = appendCleanup(cleanupError, error); }
    }
    for (const abs of ownedFiles.slice().reverse()) {
      try { fs.unlinkSync(abs); } catch (error) { cleanupError = appendCleanup(cleanupError, error); }
    }
    for (const abs of ownedDirs.slice().reverse()) {
      try { fs.rmdirSync(abs); } catch (error) { cleanupError = appendCleanup(cleanupError, error); }
    }
    if (cleanupError) {
      if (originalError) throw appendCleanup(originalError, cleanupError);
      throw cleanupError;
    }
  }
  if (originalError) throw originalError;
}

function trackTree(ctx, root) {
  let stat;
  try {
    stat = fs.lstatSync(root);
  } catch (_) {
    return;
  }
  if (stat.isSymbolicLink()) {
    ctx.noteLink(root);
    return;
  }
  if (stat.isFile()) {
    ctx.noteFile(root);
    return;
  }
  if (!stat.isDirectory()) return;
  ctx.noteDir(root);
  let names;
  try {
    names = fs.readdirSync(root);
  } catch (_) {
    return;
  }
  for (const name of names) trackTree(ctx, path.join(root, name));
}

function snapshotStat(abs) {
  const stat = fs.lstatSync(abs);
  const isSymbolicLink = stat.isSymbolicLink();
  const isDirectory = stat.isDirectory();
  const isFile = stat.isFile();
  return {
    isFile,
    isDirectory,
    isSymbolicLink,
    size: stat.size,
    mode: stat.mode,
    bytes: isFile && !isSymbolicLink ? fs.readFileSync(abs) : null,
    target: isSymbolicLink ? fs.readlinkSync(abs) : null,
  };
}

function snapshotTree(root) {
  const map = Object.create(null);
  function walk(abs, rel) {
    for (const name of fs.readdirSync(abs).sort()) {
      const childRel = rel ? `${rel}/${name}` : name;
      const childAbs = path.join(abs, name);
      const snap = snapshotStat(childAbs);
      map[childRel] = snap;
      if (snap.isDirectory && !snap.isSymbolicLink) walk(childAbs, childRel);
    }
  }
  walk(root, '');
  return map;
}

function assertSameTree(root, before) {
  assert.deepStrictEqual(snapshotTree(root), before);
}

function independentHash(bytes) {
  return crypto.createHash('sha256').update(bytes).digest('hex');
}

function writeSource(ctx, rel, body, mode) {
  return ctx.file(rel, body, mode == null ? 0o644 : mode);
}

function layout(ctx, prefix, identity) {
  const resourcesRoot = ctx.dir(path.join(prefix, 'resources'));
  const userDataPath = ctx.dir(path.join(prefix, 'user-data'));
  const brokerDir = ctx.dir(path.join(prefix, 'resources', 'wallet-broker'));
  return { resourcesRoot, userDataPath, brokerDir, identity };
}

function brokerPaths(resourcesRoot, identity) {
  const brokerDir = path.join(resourcesRoot, 'wallet-broker');
  return {
    brokerDir,
    binary: path.join(brokerDir, identity.basename),
    manifest: path.join(brokerDir, 'manifest.json'),
  };
}

function assertStaged(resourcesRoot, identity, sourceBytes, userDataPath) {
  const dest = brokerPaths(resourcesRoot, identity);
  const staged = fs.readFileSync(dest.binary);
  assert.deepStrictEqual(staged, Buffer.from(sourceBytes));
  const pin = independentHash(staged);
  assert.match(pin, /^[0-9a-f]{64}$/);
  assert.strictEqual(pin, pin.toLowerCase());
  const manifest = JSON.parse(fs.readFileSync(dest.manifest, 'utf8'));
  assert.deepStrictEqual(manifest, {
    v: 1,
    platform: identity.platform,
    arch: identity.arch,
    sha256: pin,
  });
  const binaryStat = fs.lstatSync(dest.binary);
  const manifestStat = fs.lstatSync(dest.manifest);
  assert.strictEqual(binaryStat.isSymbolicLink(), false);
  assert.strictEqual(manifestStat.isSymbolicLink(), false);
  assert.strictEqual(binaryStat.isFile(), true);
  assert.strictEqual(manifestStat.isFile(), true);
  if (POSIX) {
    assert.strictEqual(binaryStat.mode & 0o777, 0o755, `${identity.platform}/${identity.arch} binary mode`);
    assert.strictEqual(manifestStat.mode & 0o777, 0o644, `${identity.platform}/${identity.arch} manifest mode`);
  }
  const resolved = resolveWalletBrokerLaunch({
    resourcesPath: resourcesRoot,
    userDataPath,
    platform: identity.platform,
    arch: identity.arch,
  });
  assert.strictEqual(resolved.brokerPath, dest.binary);
  assert.strictEqual(resolved.expectedSha256, pin);
  assert.strictEqual(resolved.dataDir, path.resolve(userDataPath, 'wallet-broker'));
  return { pin, dest, resolved };
}

function remapUnder(fromRoot, toRoot, value) {
  if (typeof value !== 'string' || value.length === 0) return value;
  if (value === fromRoot) return toRoot;
  const prefix = fromRoot.endsWith(path.sep) ? fromRoot : fromRoot + path.sep;
  if (value.startsWith(prefix)) return path.join(toRoot, value.slice(prefix.length));
  return value;
}

function overlayFs(fromRoot, toRoot) {
  const twoPath = new Set(['renameSync', 'copyFileSync', 'linkSync', 'symlinkSync', 'rename', 'copyFile']);
  const wrapped = {
    constants: fs.constants,
    promises: fs.promises,
  };
  for (const key of Object.keys(fs)) {
    const value = fs[key];
    if (typeof value !== 'function') {
      if (wrapped[key] === undefined) wrapped[key] = value;
      continue;
    }
    wrapped[key] = function overlayed(...args) {
      const mapped = args.slice();
      if (typeof mapped[0] === 'string') mapped[0] = remapUnder(fromRoot, toRoot, mapped[0]);
      if (twoPath.has(key) && typeof mapped[1] === 'string') {
        mapped[1] = remapUnder(fromRoot, toRoot, mapped[1]);
      }
      return value.apply(fs, mapped);
    };
  }
  return wrapped;
}

function runCli(options = {}) {
  assert.ok(fs.existsSync(MODULE_PATH), MODULE_MISSING);
  const code = fs.readFileSync(MODULE_PATH, 'utf8');
  const moduleObject = { exports: {}, filename: MODULE_PATH, id: MODULE_PATH };
  const stderr = [];
  const spawns = [];
  let exitCode;
  function vmRequire(id) {
    if (id === 'child_process') {
      return {
        spawnSync(command, args, opts) {
          spawns.push({ command, args, options: opts || {} });
          if (typeof options.spawnSync === 'function') return options.spawnSync(command, args, opts);
          return options.spawnResult || { status: 0 };
        },
      };
    }
    if (id === 'fs') return options.fs || fs;
    return require(id);
  }
  vmRequire.main = moduleObject;
  const processMock = {
    argv: options.argv || [process.execPath, MODULE_PATH],
    platform: options.platform || process.platform,
    arch: options.arch || process.arch,
    env: process.env,
    exit(code) {
      exitCode = code == null ? 0 : code;
      const error = new Error('cli-exit');
      error.code = exitCode;
      throw error;
    },
    stderr: { write(chunk) { stderr.push(String(chunk)); } },
    stdout: { write() {} },
  };
  const sandbox = {
    require: vmRequire,
    module: moduleObject,
    exports: moduleObject.exports,
    __dirname: path.dirname(MODULE_PATH),
    __filename: MODULE_PATH,
    process: processMock,
    Buffer,
    console,
    setTimeout,
    clearTimeout,
  };
  try {
    vm.runInNewContext(code, sandbox, { filename: MODULE_PATH });
  } catch (error) {
    if (!error || error.message !== 'cli-exit') throw error;
  }
  return {
    exitCode: exitCode !== undefined ? exitCode : processMock.exitCode,
    stderr: stderr.join(''),
    spawns,
    exports: moduleObject.exports,
  };
}

function expectedCargoSpawn() {
  return {
    command: 'rustup',
    args: [
      'run',
      '1.98.0',
      'cargo',
      'build',
      '--manifest-path',
      path.join(REPO, 'wallet-broker', 'Cargo.toml'),
      '--target-dir',
      path.join(REPO, 'wallet-broker', 'target'),
      '--locked',
      '--offline',
      '--no-default-features',
      '--features',
      'native-ui',
      '--bin',
      'bitbook-wallet-broker',
    ],
    cwd: REPO,
  };
}

function assertInheritedStdio(stdio) {
  if (stdio === 'inherit') return;
  assert.ok(Array.isArray(stdio), 'stdio is not inherited');
  assert.ok(stdio.length >= 3 && stdio[0] === 'inherit' && stdio[1] === 'inherit' && stdio[2] === 'inherit');
}

test('module: exports synchronous stageWalletBroker', () => {
  const stage = loadStager();
  assert.strictEqual(typeof stage, 'function');
});

test('stage: copied bytes pin independently, modes match identity, resolver accepts layout', () => {
  const stage = loadStager();
  withOwnedTemp((ctx) => {
    for (const identity of IDENTITIES) {
      const prefix = `${identity.platform}-${identity.arch}`;
      const installed = layout(ctx, prefix, identity);
      const body = Buffer.from(`inert-broker-${prefix}`);
      const source = writeSource(ctx, path.join(prefix, 'src-bin'), body, 0o644);
      const sourceBefore = snapshotStat(source);
      try {
        stage(source, installed.resourcesRoot, identity.platform, identity.arch);
      } finally {
        trackTree(ctx, installed.resourcesRoot);
      }
      const staged = assertStaged(installed.resourcesRoot, identity, body, installed.userDataPath);
      assert.deepStrictEqual(snapshotStat(source), sourceBefore);
      assert.notStrictEqual(staged.pin, independentHash(Buffer.from('wrong-pin')));
      const leftover = fs.readdirSync(installed.resourcesRoot).filter((name) => name !== 'wallet-broker');
      assert.ok(leftover.length >= 1, `${prefix}: staging directory was not retained`);
    }
  });
});

test('stage: repeat replacement preserves unrelated files and retains staging leftovers', () => {
  const stage = loadStager();
  withOwnedTemp((ctx) => {
    const installed = layout(ctx, 'repeat', LINUX_X64);
    const unrelated = ctx.file(path.join('repeat', 'resources', 'wallet-broker', 'UNRELATED'), SENTINEL);
    const firstBody = Buffer.from('first-broker-bytes');
    const secondBody = Buffer.from('second-broker-bytes-distinct');
    const firstSource = writeSource(ctx, 'repeat/src-a', firstBody, 0o644);
    const secondSource = writeSource(ctx, 'repeat/src-b', secondBody, 0o644);
    try {
      stage(firstSource, installed.resourcesRoot, LINUX_X64.platform, LINUX_X64.arch);
      const first = assertStaged(installed.resourcesRoot, LINUX_X64, firstBody, installed.userDataPath);
      stage(secondSource, installed.resourcesRoot, LINUX_X64.platform, LINUX_X64.arch);
      const second = assertStaged(installed.resourcesRoot, LINUX_X64, secondBody, installed.userDataPath);
      assert.notStrictEqual(first.pin, second.pin);
      assert.strictEqual(fs.readFileSync(unrelated, 'utf8'), SENTINEL);
      assert.strictEqual(fs.lstatSync(unrelated).isFile(), true);
    } finally {
      trackTree(ctx, installed.resourcesRoot);
    }
    const leftover = fs.readdirSync(installed.resourcesRoot).filter((name) => name !== 'wallet-broker');
    assert.ok(leftover.length >= 1, 'repeat: staging directory was not retained');
  });
});

test('preflight: invalid source, roots, and identities leave destination bytes unchanged', () => {
  const stage = loadStager();
  withOwnedTemp((ctx) => {
    const installed = layout(ctx, 'preflight-src', LINUX_X64);
    const goodBody = Buffer.from('preflight-source-body');
    const goodSource = writeSource(ctx, 'preflight-src/good-bin', goodBody, 0o644);
    const destFile = ctx.file(path.join('preflight-src', 'resources', 'wallet-broker', 'UNRELATED'), SENTINEL);
    const destBefore = snapshotTree(installed.resourcesRoot);
    const sourceTarget = ctx.file('preflight-src/link-target', 'LINK_TARGET_BYTES');
    const sourceLink = ctx.link('preflight-src/source-link', sourceTarget);
    const sourceDir = ctx.dir('preflight-src/source-dir');
    const resourcesLink = ctx.link('preflight-src/resources-link', installed.resourcesRoot);
    const resourcesFile = ctx.file('preflight-src/resources-as-file', 'not-a-directory');
    const missingSource = path.join(ctx.root, 'preflight-src', 'missing-bin');
    const missingRoot = path.join(ctx.root, 'preflight-src', 'missing-resources');
    const relSource = path.relative(process.cwd(), goodSource);
    const relRoot = path.relative(process.cwd(), installed.resourcesRoot);
    const rows = [
      { name: 'missing source', call: () => stage(missingSource, installed.resourcesRoot, 'linux', 'x64') },
      { name: 'source symlink', call: () => stage(sourceLink, installed.resourcesRoot, 'linux', 'x64') },
      { name: 'source directory', call: () => stage(sourceDir, installed.resourcesRoot, 'linux', 'x64') },
      { name: 'relative source', call: () => stage(relSource, installed.resourcesRoot, 'linux', 'x64') },
      { name: 'empty source', call: () => stage('', installed.resourcesRoot, 'linux', 'x64') },
      { name: 'resources symlink', call: () => stage(goodSource, resourcesLink, 'linux', 'x64') },
      { name: 'resources file', call: () => stage(goodSource, resourcesFile, 'linux', 'x64') },
      { name: 'missing resources', call: () => stage(goodSource, missingRoot, 'linux', 'x64') },
      { name: 'relative resources', call: () => stage(goodSource, relRoot, 'linux', 'x64') },
      { name: 'empty resources', call: () => stage(goodSource, '', 'linux', 'x64') },
      { name: 'unsupported platform', call: () => stage(goodSource, installed.resourcesRoot, 'freebsd', 'x64') },
      { name: 'unsupported arch', call: () => stage(goodSource, installed.resourcesRoot, 'linux', 'amd64') },
    ];
    for (const row of rows) {
      assert.throws(row.call, `${row.name}: expected throw`);
      assertSameTree(installed.resourcesRoot, destBefore);
      assert.strictEqual(fs.readFileSync(destFile, 'utf8'), SENTINEL);
    }
    assert.strictEqual(fs.lstatSync(sourceLink).isSymbolicLink(), true);
    assert.strictEqual(fs.readFileSync(sourceTarget, 'utf8'), 'LINK_TARGET_BYTES');
    trackTree(ctx, installed.resourcesRoot);
  });
});

test('preflight: destination directory and pin links leave bytes unchanged', () => {
  const stage = loadStager();
  withOwnedTemp((ctx) => {
    const goodBody = Buffer.from('pin-link-source');
    ctx.dir('pin');
    const goodSource = writeSource(ctx, 'pin/src', goodBody, 0o644);

    const dirLinkResources = ctx.dir('pin-dir/resources');
    const realBroker = ctx.dir('pin-dir/real-broker');
    const realSentinel = ctx.file('pin-dir/real-broker/SENTINEL', SENTINEL);
    const brokerLink = ctx.link(path.join('pin-dir', 'resources', 'wallet-broker'), realBroker);
    const dirLinkBefore = snapshotTree(dirLinkResources);
    const realBefore = snapshotTree(realBroker);
    assert.throws(
      () => stage(goodSource, dirLinkResources, 'linux', 'x64'),
      'wallet-broker directory symlink: expected throw'
    );
    assertSameTree(dirLinkResources, dirLinkBefore);
    assertSameTree(realBroker, realBefore);
    assert.strictEqual(fs.lstatSync(brokerLink).isSymbolicLink(), true);
    assert.strictEqual(fs.readFileSync(realSentinel, 'utf8'), SENTINEL);

    const binaryLayout = layout(ctx, 'pin-bin', LINUX_X64);
    const binaryTarget = ctx.file('pin-bin/binary-target', 'EXISTING_BINARY_TARGET');
    const binaryLink = ctx.link(
      path.join('pin-bin', 'resources', 'wallet-broker', LINUX_X64.basename),
      binaryTarget
    );
    const binaryBefore = snapshotTree(binaryLayout.resourcesRoot);
    const binaryTargetBefore = snapshotStat(binaryTarget);
    assert.throws(
      () => stage(goodSource, binaryLayout.resourcesRoot, 'linux', 'x64'),
      'destination binary symlink: expected throw'
    );
    assertSameTree(binaryLayout.resourcesRoot, binaryBefore);
    assert.deepStrictEqual(snapshotStat(binaryTarget), binaryTargetBefore);
    assert.strictEqual(fs.lstatSync(binaryLink).isSymbolicLink(), true);
    assert.strictEqual(fs.readFileSync(binaryTarget, 'utf8'), 'EXISTING_BINARY_TARGET');

    const manifestLayout = layout(ctx, 'pin-manifest', LINUX_X64);
    const manifestTarget = ctx.file('pin-manifest/manifest-target', '{"v":1}');
    const manifestLink = ctx.link(
      path.join('pin-manifest', 'resources', 'wallet-broker', 'manifest.json'),
      manifestTarget
    );
    const manifestBefore = snapshotTree(manifestLayout.resourcesRoot);
    const manifestTargetBefore = snapshotStat(manifestTarget);
    assert.throws(
      () => stage(goodSource, manifestLayout.resourcesRoot, 'linux', 'x64'),
      'destination manifest symlink: expected throw'
    );
    assertSameTree(manifestLayout.resourcesRoot, manifestBefore);
    assert.deepStrictEqual(snapshotStat(manifestTarget), manifestTargetBefore);
    assert.strictEqual(fs.lstatSync(manifestLink).isSymbolicLink(), true);

    const danglingLayout = layout(ctx, 'pin-dangling', LINUX_X64);
    const danglingTarget = path.join(ctx.root, 'pin-dangling', 'absent-target');
    const danglingBinary = ctx.link(
      path.join('pin-dangling', 'resources', 'wallet-broker', LINUX_X64.basename),
      danglingTarget
    );
    const danglingManifest = ctx.link(
      path.join('pin-dangling', 'resources', 'wallet-broker', 'manifest.json'),
      danglingTarget
    );
    const danglingBefore = snapshotTree(danglingLayout.resourcesRoot);
    assert.throws(
      () => stage(goodSource, danglingLayout.resourcesRoot, 'linux', 'x64'),
      'dangling destination pin links: expected throw'
    );
    assertSameTree(danglingLayout.resourcesRoot, danglingBefore);
    assert.strictEqual(fs.lstatSync(danglingBinary).isSymbolicLink(), true);
    assert.strictEqual(fs.lstatSync(danglingManifest).isSymbolicLink(), true);
    assert.strictEqual(fs.readlinkSync(danglingBinary), danglingTarget);
    assert.throws(() => fs.lstatSync(danglingTarget), { code: 'ENOENT' });

    trackTree(ctx, dirLinkResources);
    trackTree(ctx, realBroker);
    trackTree(ctx, binaryLayout.resourcesRoot);
    trackTree(ctx, manifestLayout.resourcesRoot);
    trackTree(ctx, danglingLayout.resourcesRoot);
  });
});

test('cli: vm-mocked cargo command is exact and nonzero skips staging', () => {
  const expected = expectedCargoSpawn();
  withOwnedTemp((ctx) => {
    const tempTarget = ctx.dir('cli-target');
    const debugDir = ctx.dir('cli-target/debug');
    const body = Buffer.from('cli-debug-broker-bytes');
    ctx.file(path.join('cli-target', 'debug', HOST_BASENAME), body, 0o755);
    const overlay = overlayFs(TARGET_ROOT, tempTarget);
    const identity = IDENTITIES.find(
      (item) => item.platform === process.platform && item.arch === process.arch
    ) || { platform: process.platform, arch: process.arch, basename: HOST_BASENAME };
    const userDataPath = ctx.dir('cli-user-data');

    const success = runCli({ fs: overlay, spawnResult: { status: 0 } });
    trackTree(ctx, tempTarget);
    assert.ok(success.exitCode === undefined || success.exitCode === 0, 'cli success exited nonzero');
    assert.strictEqual(success.spawns.length, 1);
    const spawn = success.spawns[0];
    assert.strictEqual(spawn.command, expected.command);
    assert.deepStrictEqual(Array.from(spawn.args), expected.args);
    assert.strictEqual(spawn.options.cwd, expected.cwd);
    assert.strictEqual(spawn.options.shell, false);
    assertInheritedStdio(spawn.options.stdio);
    const resourcesRoot = path.join(tempTarget, 'app-resources');
    assertStaged(resourcesRoot, identity, body, userDataPath);
    assert.ok(!success.stderr.includes(CANARY));

    const blockedRoot = ctx.dir('cli-blocked-target');
    ctx.dir('cli-blocked-target/debug');
    ctx.file(path.join('cli-blocked-target', 'debug', HOST_BASENAME), body, 0o755);
    const blockedOverlay = overlayFs(TARGET_ROOT, blockedRoot);
    const beforeBlocked = snapshotTree(blockedRoot);
    const failed = runCli({
      fs: blockedOverlay,
      spawnResult: { status: 1, error: new Error(CANARY), stderr: CANARY },
    });
    trackTree(ctx, blockedRoot);
    assert.strictEqual(failed.exitCode, 1);
    assert.strictEqual(failed.spawns.length, 1);
    const lines = failed.stderr.split(/\r?\n/).filter((line) => line.length > 0);
    assert.deepStrictEqual(lines, [CLI_ERROR]);
    assert.ok(!failed.stderr.includes(CANARY));
    assertSameTree(blockedRoot, beforeBlocked);
    assert.throws(() => fs.lstatSync(path.join(blockedRoot, 'app-resources', 'wallet-broker', HOST_BASENAME)), {
      code: 'ENOENT',
    });

    const extra = runCli({
      fs: overlayFs(TARGET_ROOT, ctx.dir('cli-args-target')),
      argv: [process.execPath, MODULE_PATH, '--unexpected'],
    });
    assert.strictEqual(extra.exitCode, 1);
    assert.strictEqual(extra.spawns.length, 0);
    const extraLines = extra.stderr.split(/\r?\n/).filter((line) => line.length > 0);
    assert.deepStrictEqual(extraLines, [CLI_ERROR]);
  });
});

function run() {
  let failed = 0;
  for (const { name, fn } of tests) {
    try {
      fn();
      process.stdout.write(`ok ${name}\n`);
    } catch (error) {
      failed += 1;
      process.stderr.write(`not ok ${name}\n${error && error.stack ? error.stack : error}\n`);
    }
  }
  if (failed) {
    process.stderr.write(`${failed} wallet broker build test(s) failed\n`);
    process.exit(1);
  }
  process.stdout.write(`BitBook wallet broker build tests passed (${tests.length}).\n`);
}

if (require.main === module) run();
module.exports = { tests };
