'use strict';

const assert = require('assert');
const crypto = require('crypto');
const fs = require('fs');
const os = require('os');
const path = require('path');

const MODULE_PATH = path.resolve(__dirname, '..', 'wallet-broker', 'launch-config.js');
const UNAVAILABLE_MESSAGE = 'Wallet broker is unavailable';
const FIXED_PIN = '0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef';
const ARTIFACT_BODY = 'inert-packaged-broker-bytes';
const CANARY = 'LAUNCH_CONFIG_CANARY_SECRET';
const SENTINEL = 'LAUNCH_CONFIG_SENTINEL_UNCHANGED';
const UNSET = Symbol('no-return');
const IDENTITIES = Object.freeze([
  Object.freeze({ platform: 'linux', arch: 'x64', basename: 'bitbook-wallet-broker' }),
  Object.freeze({ platform: 'linux', arch: 'arm64', basename: 'bitbook-wallet-broker' }),
  Object.freeze({ platform: 'darwin', arch: 'x64', basename: 'bitbook-wallet-broker' }),
  Object.freeze({ platform: 'darwin', arch: 'arm64', basename: 'bitbook-wallet-broker' }),
  Object.freeze({ platform: 'win32', arch: 'x64', basename: 'bitbook-wallet-broker.exe' }),
  Object.freeze({ platform: 'win32', arch: 'arm64', basename: 'bitbook-wallet-broker.exe' }),
]);
const LINUX_X64 = IDENTITIES[0];
const ARTIFACT_DIGEST = crypto.createHash('sha256').update(ARTIFACT_BODY, 'utf8').digest('hex');

assert.match(FIXED_PIN, /^[0-9a-f]{64}$/);
assert.match(ARTIFACT_DIGEST, /^[0-9a-f]{64}$/);
assert.notStrictEqual(ARTIFACT_DIGEST, FIXED_PIN);
assert.strictEqual(IDENTITIES.length, 6);

const tests = [];
function test(name, fn) { tests.push({ name, fn }); }

function loadResolver() {
  assert.ok(fs.existsSync(MODULE_PATH), 'packaged broker launch resolver is missing');
  const exported = require(MODULE_PATH);
  assert.strictEqual(typeof exported.resolveWalletBrokerLaunch, 'function');
  return exported.resolveWalletBrokerLaunch;
}

function appendCleanup(previous, error) {
  if (!previous) return error;
  previous.message = `${previous.message}\ncleanup also failed: ${error.stack || error.message}`;
  return previous;
}

function withOwnedTemp(fn) {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), 'bb-wal011-launch-'));
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
    file(rel, contents) {
      const abs = path.resolve(root, rel);
      fs.writeFileSync(abs, contents);
      ownedFiles.push(abs);
      return abs;
    },
    link(rel, target) {
      const abs = path.resolve(root, rel);
      fs.symlinkSync(target, abs);
      ownedLinks.push(abs);
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

function manifestJson(platform, arch, sha256) {
  return JSON.stringify({
    v: 1,
    platform,
    arch,
    sha256: sha256 === undefined ? FIXED_PIN : sha256,
  });
}

function writeValidLayout(ctx, prefix, identity, manifestContents) {
  const resourcesPath = ctx.dir(path.join(prefix, 'resources'));
  const userDataPath = ctx.dir(path.join(prefix, 'user-data'));
  ctx.dir(path.join(prefix, 'resources', 'wallet-broker'));
  const manifestPath = manifestContents === null
    ? null
    : ctx.file(
      path.join(prefix, 'resources', 'wallet-broker', 'manifest.json'),
      manifestContents === undefined
        ? manifestJson(identity.platform, identity.arch)
        : manifestContents
    );
  const brokerPath = identity
    ? ctx.file(
      path.join(prefix, 'resources', 'wallet-broker', identity.basename),
      ARTIFACT_BODY
    )
    : null;
  return { resourcesPath, userDataPath, manifestPath, brokerPath };
}

function snapshotFile(abs) {
  const stat = fs.lstatSync(abs);
  assert.strictEqual(stat.isSymbolicLink(), false);
  assert.strictEqual(stat.isFile(), true);
  return {
    bytes: fs.readFileSync(abs),
    size: stat.size,
    mode: stat.mode,
    isFile: true,
    isSymbolicLink: false,
  };
}

function assertSameFile(abs, before) {
  const after = snapshotFile(abs);
  assert.deepStrictEqual(after, before);
}

function assertDataDirAbsent(userDataPath) {
  const dataDir = path.resolve(userDataPath, 'wallet-broker');
  assert.throws(() => fs.lstatSync(dataDir), { code: 'ENOENT' });
  assert.deepStrictEqual(fs.readdirSync(userDataPath), []);
}

function assertUnavailable(fn, details) {
  const name = details && details.name ? details.name : 'call';
  let returned = UNSET;
  let error;
  try {
    returned = fn();
  } catch (caught) {
    error = caught;
  }
  assert.ok(error, `${name}: expected UNAVAILABLE`);
  assert.strictEqual(returned, UNSET, `${name}: no returned configuration`);
  assert.ok(error instanceof Error, `${name}: Error instance`);
  assert.strictEqual(error.code, 'UNAVAILABLE', `${name}: code`);
  assert.strictEqual(error.message, UNAVAILABLE_MESSAGE, `${name}: message`);
  assert.strictEqual(error.cause, undefined, `${name}: no cause`);
  assert.ok(!Object.prototype.hasOwnProperty.call(error, 'cause'), `${name}: no own cause`);
  const secrets = [];
  if (details && details.canary) secrets.push(details.canary);
  if (details && details.resourcesPath) secrets.push(details.resourcesPath);
  if (details && details.userDataPath) secrets.push(details.userDataPath);
  if (details && details.secrets) secrets.push(...details.secrets);
  for (const secret of secrets) {
    if (!secret) continue;
    assert.ok(!error.message.includes(secret), `${name}: message leaked secret`);
    if (details && details.canary && secret === details.canary && typeof error.stack === 'string') {
      assert.ok(!error.stack.includes(secret), `${name}: stack leaked canary`);
    }
    for (const key of Object.getOwnPropertyNames(error)) {
      if (key === 'stack') continue;
      assert.ok(!key.includes(secret), `${name}: own key leaked secret`);
      const value = error[key];
      if (typeof value === 'string') {
        assert.ok(!value.includes(secret), `${name}: own property leaked secret`);
      }
    }
  }
}

function assertValidLaunch(result, options, identity, resourcesPath, userDataPath) {
  const expected = Object.freeze({
    brokerPath: path.resolve(resourcesPath, 'wallet-broker', identity.basename),
    expectedSha256: FIXED_PIN,
    dataDir: path.resolve(userDataPath, 'wallet-broker'),
  });
  assert.ok(Object.isFrozen(result), `${identity.platform}/${identity.arch}: frozen`);
  assert.strictEqual(Object.getPrototypeOf(result), Object.prototype);
  assert.deepStrictEqual(
    Object.getOwnPropertyNames(result).sort(),
    ['brokerPath', 'dataDir', 'expectedSha256']
  );
  assert.deepStrictEqual(result, expected);
  assert.strictEqual(path.basename(result.brokerPath), identity.basename);
  assert.strictEqual(result.expectedSha256, FIXED_PIN);
  assert.notStrictEqual(result.expectedSha256, ARTIFACT_DIGEST);
  assert.deepStrictEqual(options, {
    resourcesPath,
    userDataPath,
    platform: identity.platform,
    arch: identity.arch,
  });
  assertDataDirAbsent(userDataPath);
}

test('valid configuration: six platform identities return frozen supervisor options', () => {
  const resolve = loadResolver();
  withOwnedTemp((ctx) => {
    for (const identity of IDENTITIES) {
      const prefix = `${identity.platform}-${identity.arch}`;
      const layout = writeValidLayout(ctx, prefix, identity);
      const options = {
        resourcesPath: layout.resourcesPath,
        userDataPath: layout.userDataPath,
        platform: identity.platform,
        arch: identity.arch,
      };
      const inputBefore = JSON.stringify(options);
      const manifestBefore = snapshotFile(layout.manifestPath);
      const artifactBefore = snapshotFile(layout.brokerPath);
      assert.strictEqual(manifestBefore.isFile, true);
      assert.strictEqual(manifestBefore.isSymbolicLink, false);
      assert.strictEqual(artifactBefore.isFile, true);
      assert.strictEqual(artifactBefore.isSymbolicLink, false);
      assert.deepStrictEqual(artifactBefore.bytes, Buffer.from(ARTIFACT_BODY, 'utf8'));
      assert.notStrictEqual(
        crypto.createHash('sha256').update(artifactBefore.bytes).digest('hex'),
        FIXED_PIN
      );
      assertDataDirAbsent(layout.userDataPath);

      const result = resolve(options);

      assertValidLaunch(result, options, identity, layout.resourcesPath, layout.userDataPath);
      assert.strictEqual(JSON.stringify(options), inputBefore);
      assertSameFile(layout.manifestPath, manifestBefore);
      assertSameFile(layout.brokerPath, artifactBefore);
      assert.strictEqual(
        fs.readFileSync(layout.manifestPath, 'utf8'),
        manifestJson(identity.platform, identity.arch)
      );
    }
  });
});

test('invalid options: reject missing extra accessor prototype and path identities', () => {
  const resolve = loadResolver();
  withOwnedTemp((ctx) => {
    const layout = writeValidLayout(ctx, 'opts', LINUX_X64);
    const base = {
      resourcesPath: layout.resourcesPath,
      userDataPath: layout.userDataPath,
      platform: LINUX_X64.platform,
      arch: LINUX_X64.arch,
    };
    const leak = {
      resourcesPath: layout.resourcesPath,
      userDataPath: layout.userDataPath,
      canary: CANARY,
    };
    const unsupportedPlatformLayout = writeValidLayout(ctx, 'opts-freebsd-x64', {
      platform: 'freebsd',
      arch: 'x64',
      basename: 'bitbook-wallet-broker',
    });
    const unsupportedArchLayout = writeValidLayout(ctx, 'opts-linux-amd64', {
      platform: 'linux',
      arch: 'amd64',
      basename: 'bitbook-wallet-broker',
    });
    const unsupportedPlatformOptions = {
      resourcesPath: unsupportedPlatformLayout.resourcesPath,
      userDataPath: unsupportedPlatformLayout.userDataPath,
      platform: 'freebsd',
      arch: 'x64',
    };
    const unsupportedArchOptions = {
      resourcesPath: unsupportedArchLayout.resourcesPath,
      userDataPath: unsupportedArchLayout.userDataPath,
      platform: 'linux',
      arch: 'amd64',
    };
    let getterCalls = 0;
    let protoGets = 0;
    const accessorOptions = {};
    for (const key of ['resourcesPath', 'userDataPath', 'platform', 'arch']) {
      Object.defineProperty(accessorOptions, key, {
        enumerable: true,
        configurable: true,
        get() {
          getterCalls += 1;
          return base[key];
        },
      });
    }
    const proto = {};
    Object.defineProperty(proto, 'env', {
      enumerable: true,
      configurable: true,
      get() {
        protoGets += 1;
        return CANARY;
      },
    });
    const rows = [
      { name: 'absent options', call: () => resolve() },
      { name: 'null options', call: () => resolve(null) },
      { name: 'array options', call: () => resolve(['resourcesPath', 'userDataPath', 'platform', 'arch']) },
      { name: 'number options', call: () => resolve(1) },
      {
        name: 'extra own key',
        call: () => resolve(Object.assign({}, base, { env: { PATH: CANARY } })),
      },
      { name: 'accessor own keys', call: () => resolve(accessorOptions), getters: true },
      {
        name: 'null prototype',
        call: () => resolve(Object.assign(Object.create(null), base)),
      },
      {
        name: 'foreign prototype',
        call: () => resolve(Object.assign(Object.create(proto), base)),
        proto: true,
      },
      {
        name: 'missing resourcesPath',
        call: () => {
          const options = Object.assign({}, base);
          delete options.resourcesPath;
          return resolve(options);
        },
      },
      {
        name: 'missing userDataPath',
        call: () => {
          const options = Object.assign({}, base);
          delete options.userDataPath;
          return resolve(options);
        },
      },
      {
        name: 'missing platform',
        call: () => {
          const options = Object.assign({}, base);
          delete options.platform;
          return resolve(options);
        },
      },
      {
        name: 'missing arch',
        call: () => {
          const options = Object.assign({}, base);
          delete options.arch;
          return resolve(options);
        },
      },
      {
        name: 'relative resourcesPath',
        call: () => resolve(Object.assign({}, base, { resourcesPath: 'relative-resources' })),
      },
      {
        name: 'relative userDataPath',
        call: () => resolve(Object.assign({}, base, { userDataPath: 'relative-user-data' })),
      },
      {
        name: 'empty resourcesPath',
        call: () => resolve(Object.assign({}, base, { resourcesPath: '' })),
      },
      {
        name: 'empty userDataPath',
        call: () => resolve(Object.assign({}, base, { userDataPath: '' })),
      },
      {
        name: 'non-string resourcesPath',
        call: () => resolve(Object.assign({}, base, { resourcesPath: 42 })),
      },
      {
        name: 'non-string userDataPath',
        call: () => resolve(Object.assign({}, base, { userDataPath: false })),
      },
      {
        name: 'unsupported platform',
        layout: unsupportedPlatformLayout,
        options: unsupportedPlatformOptions,
        call: () => resolve(unsupportedPlatformOptions),
      },
      {
        name: 'unsupported arch',
        layout: unsupportedArchLayout,
        options: unsupportedArchOptions,
        call: () => resolve(unsupportedArchOptions),
      },
    ];
    assert.strictEqual(rows.length, 20);
    const manifestBefore = snapshotFile(layout.manifestPath);
    const artifactBefore = snapshotFile(layout.brokerPath);
    const unsupportedPlatformManifestBefore = snapshotFile(unsupportedPlatformLayout.manifestPath);
    const unsupportedPlatformArtifactBefore = snapshotFile(unsupportedPlatformLayout.brokerPath);
    const unsupportedArchManifestBefore = snapshotFile(unsupportedArchLayout.manifestPath);
    const unsupportedArchArtifactBefore = snapshotFile(unsupportedArchLayout.brokerPath);
    for (const row of rows) {
      getterCalls = 0;
      protoGets = 0;
      const rowLayout = row.layout || layout;
      const rowManifestBefore = row.layout === unsupportedPlatformLayout
        ? unsupportedPlatformManifestBefore
        : row.layout === unsupportedArchLayout
          ? unsupportedArchManifestBefore
          : manifestBefore;
      const rowArtifactBefore = row.layout === unsupportedPlatformLayout
        ? unsupportedPlatformArtifactBefore
        : row.layout === unsupportedArchLayout
          ? unsupportedArchArtifactBefore
          : artifactBefore;
      const rowLeak = row.layout
        ? {
          name: row.name,
          resourcesPath: rowLayout.resourcesPath,
          userDataPath: rowLayout.userDataPath,
          canary: CANARY,
        }
        : Object.assign({ name: row.name }, leak);
      if (row.options) {
        const parsed = JSON.parse(fs.readFileSync(rowLayout.manifestPath, 'utf8'));
        assert.deepStrictEqual(parsed, {
          v: 1,
          platform: row.options.platform,
          arch: row.options.arch,
          sha256: FIXED_PIN,
        });
        assert.strictEqual(typeof parsed.v, 'number');
        assert.strictEqual(parsed.v, 1);
        assert.strictEqual(parsed.sha256, FIXED_PIN);
        assert.deepStrictEqual(
          Object.getOwnPropertyNames(parsed).sort(),
          ['arch', 'platform', 'sha256', 'v']
        );
        assert.strictEqual(row.options.platform, parsed.platform);
        assert.strictEqual(row.options.arch, parsed.arch);
        const brokerStat = fs.lstatSync(rowLayout.brokerPath);
        assert.strictEqual(brokerStat.isFile(), true);
        assert.strictEqual(brokerStat.isSymbolicLink(), false);
        assert.strictEqual(path.basename(rowLayout.brokerPath), 'bitbook-wallet-broker');
        assert.deepStrictEqual(fs.readFileSync(rowLayout.brokerPath), Buffer.from(ARTIFACT_BODY, 'utf8'));
      }
      assertUnavailable(row.call, rowLeak);
      if (row.getters) assert.strictEqual(getterCalls, 0, `${row.name}: zero getter calls`);
      else assert.strictEqual(getterCalls, 0);
      if (row.proto) assert.strictEqual(protoGets, 0, `${row.name}: zero prototype getter calls`);
      else assert.strictEqual(protoGets, 0);
      assert.deepStrictEqual(Object.keys(base).sort(), ['arch', 'platform', 'resourcesPath', 'userDataPath']);
      assertDataDirAbsent(rowLayout.userDataPath);
      assertSameFile(rowLayout.manifestPath, rowManifestBefore);
      assertSameFile(rowLayout.brokerPath, rowArtifactBefore);
    }
  });
});

test('invalid manifest: reject malformed extra mismatched and hostile digest rows', () => {
  const resolve = loadResolver();
  withOwnedTemp((ctx) => {
    const extra = (key, value) => {
      const body = {
        v: 1,
        platform: LINUX_X64.platform,
        arch: LINUX_X64.arch,
        sha256: FIXED_PIN,
      };
      body[key] = value;
      return JSON.stringify(body);
    };
    const missing = (key) => {
      const body = {
        v: 1,
        platform: LINUX_X64.platform,
        arch: LINUX_X64.arch,
        sha256: FIXED_PIN,
      };
      delete body[key];
      return JSON.stringify(body);
    };
    const rows = [
      { name: 'absent manifest', body: null, canary: false },
      {
        name: 'malformed JSON',
        body: `{"v":1,"platform":"linux","arch":"x64","sha256":"${FIXED_PIN}","note":"${CANARY}"`,
        canary: true,
      },
      { name: 'null root', body: 'null', canary: false },
      { name: 'array root', body: JSON.stringify([CANARY]), canary: true },
      { name: 'scalar root', body: JSON.stringify(CANARY), canary: true },
      { name: 'missing v', body: missing('v'), canary: false },
      { name: 'missing platform', body: missing('platform'), canary: false },
      { name: 'missing arch', body: missing('arch'), canary: false },
      { name: 'missing sha256', body: missing('sha256'), canary: false },
      { name: 'extra executable', body: extra('executable', CANARY), canary: true },
      { name: 'extra path', body: extra('path', CANARY), canary: true },
      { name: 'extra env', body: extra('env', { SECRET: CANARY }), canary: true },
      { name: 'extra argv', body: extra('argv', [CANARY]), canary: true },
      { name: 'extra dataDir', body: extra('dataDir', CANARY), canary: true },
      { name: 'wrong version', body: JSON.stringify({
        v: 2,
        platform: LINUX_X64.platform,
        arch: LINUX_X64.arch,
        sha256: FIXED_PIN,
      }), canary: false },
      { name: 'wrong version type', body: JSON.stringify({
        v: '1',
        platform: LINUX_X64.platform,
        arch: LINUX_X64.arch,
        sha256: FIXED_PIN,
      }), canary: false },
      { name: 'platform mismatch', body: manifestJson('darwin', 'x64'), canary: false },
      { name: 'arch mismatch', body: manifestJson('linux', 'arm64'), canary: false },
      { name: 'uppercase digest', body: manifestJson('linux', 'x64', FIXED_PIN.toUpperCase()), canary: false },
      { name: 'short digest', body: manifestJson('linux', 'x64', FIXED_PIN.slice(0, 63)), canary: false },
      { name: 'nonhex digest', body: manifestJson('linux', 'x64', `g${FIXED_PIN.slice(1)}`), canary: false },
      { name: 'non-string digest', body: JSON.stringify({
        v: 1,
        platform: LINUX_X64.platform,
        arch: LINUX_X64.arch,
        sha256: 1,
      }), canary: false },
    ];
    assert.strictEqual(rows.length, 22);
    const optionsBase = { platform: LINUX_X64.platform, arch: LINUX_X64.arch };
    for (let index = 0; index < rows.length; index += 1) {
      const row = rows[index];
      const layout = writeValidLayout(ctx, `manifest-${index}`, LINUX_X64, row.body);
      const options = Object.assign({
        resourcesPath: layout.resourcesPath,
        userDataPath: layout.userDataPath,
      }, optionsBase);
      const inputBefore = JSON.stringify(options);
      const artifactBefore = snapshotFile(layout.brokerPath);
      let manifestBefore = null;
      if (layout.manifestPath) manifestBefore = snapshotFile(layout.manifestPath);
      else {
        assert.throws(
          () => fs.lstatSync(path.join(layout.resourcesPath, 'wallet-broker', 'manifest.json')),
          { code: 'ENOENT' }
        );
      }
      assertUnavailable(() => resolve(options), {
        name: row.name,
        canary: row.canary ? CANARY : undefined,
        resourcesPath: layout.resourcesPath,
        userDataPath: layout.userDataPath,
        secrets: [FIXED_PIN.toUpperCase(), `g${FIXED_PIN.slice(1)}`],
      });
      assert.strictEqual(JSON.stringify(options), inputBefore);
      assertSameFile(layout.brokerPath, artifactBefore);
      if (manifestBefore) assertSameFile(layout.manifestPath, manifestBefore);
      assertDataDirAbsent(layout.userDataPath);
    }
  });
});

test('manifest size: 4096 bytes succeed; 4097 and empty reject', () => {
  const resolve = loadResolver();
  withOwnedTemp((ctx) => {
    const compact = manifestJson(LINUX_X64.platform, LINUX_X64.arch);
    const compactBytes = Buffer.byteLength(compact, 'utf8');
    assert.ok(compactBytes >= 1);
    assert.ok(compactBytes < 4096);
    const padded4096 = compact + ' '.repeat(4096 - compactBytes);
    const padded4097 = compact + ' '.repeat(4097 - compactBytes);
    const empty = Buffer.alloc(0);
    assert.strictEqual(Buffer.byteLength(padded4096, 'utf8'), 4096);
    assert.strictEqual(Buffer.byteLength(padded4097, 'utf8'), 4097);
    assert.strictEqual(empty.length, 0);
    assert.strictEqual(padded4096.trim(), compact);
    assert.ok(/^[ ]+$/.test(padded4096.slice(compactBytes)));

    const okLayout = writeValidLayout(ctx, 'size-4096', LINUX_X64, padded4096);
    assert.strictEqual(fs.lstatSync(okLayout.manifestPath).size, 4096);
    assert.strictEqual(fs.readFileSync(okLayout.manifestPath).length, 4096);
    const okOptions = {
      resourcesPath: okLayout.resourcesPath,
      userDataPath: okLayout.userDataPath,
      platform: LINUX_X64.platform,
      arch: LINUX_X64.arch,
    };
    const okBefore = JSON.stringify(okOptions);
    const okManifestBefore = snapshotFile(okLayout.manifestPath);
    const okArtifactBefore = snapshotFile(okLayout.brokerPath);
    const okResult = resolve(okOptions);
    assertValidLaunch(okResult, okOptions, LINUX_X64, okLayout.resourcesPath, okLayout.userDataPath);
    assert.strictEqual(JSON.stringify(okOptions), okBefore);
    assertSameFile(okLayout.manifestPath, okManifestBefore);
    assertSameFile(okLayout.brokerPath, okArtifactBefore);

    const overLayout = writeValidLayout(ctx, 'size-4097', LINUX_X64, padded4097);
    assert.strictEqual(fs.lstatSync(overLayout.manifestPath).size, 4097);
    assert.strictEqual(Buffer.byteLength(fs.readFileSync(overLayout.manifestPath)), 4097);
    const overOptions = {
      resourcesPath: overLayout.resourcesPath,
      userDataPath: overLayout.userDataPath,
      platform: LINUX_X64.platform,
      arch: LINUX_X64.arch,
    };
    const overBefore = JSON.stringify(overOptions);
    assertUnavailable(() => resolve(overOptions), {
      name: '4097-byte manifest',
      resourcesPath: overLayout.resourcesPath,
      userDataPath: overLayout.userDataPath,
    });
    assert.strictEqual(JSON.stringify(overOptions), overBefore);
    assert.strictEqual(fs.lstatSync(overLayout.manifestPath).size, 4097);
    assertDataDirAbsent(overLayout.userDataPath);

    const emptyLayout = writeValidLayout(ctx, 'size-empty', LINUX_X64, empty);
    assert.strictEqual(fs.lstatSync(emptyLayout.manifestPath).size, 0);
    assert.strictEqual(fs.readFileSync(emptyLayout.manifestPath).length, 0);
    const emptyOptions = {
      resourcesPath: emptyLayout.resourcesPath,
      userDataPath: emptyLayout.userDataPath,
      platform: LINUX_X64.platform,
      arch: LINUX_X64.arch,
    };
    assertUnavailable(() => resolve(emptyOptions), {
      name: 'empty manifest',
      resourcesPath: emptyLayout.resourcesPath,
      userDataPath: emptyLayout.userDataPath,
    });
    assert.strictEqual(fs.lstatSync(emptyLayout.manifestPath).size, 0);
    assertDataDirAbsent(emptyLayout.userDataPath);
  });
});

test('inventory: missing symlink and directory entries fail closed', () => {
  const resolve = loadResolver();
  withOwnedTemp((ctx) => {
    function optionsFor(resourcesPath, userDataPath) {
      return {
        resourcesPath,
        userDataPath,
        platform: LINUX_X64.platform,
        arch: LINUX_X64.arch,
      };
    }

    const missingDirPrefix = 'inv-missing-dir';
    const missingDirResources = ctx.dir(path.join(missingDirPrefix, 'resources'));
    const missingDirUserData = ctx.dir(path.join(missingDirPrefix, 'user-data'));
    ctx.file(path.join(missingDirPrefix, 'resources', 'manifest.json'), manifestJson('linux', 'x64'));
    ctx.file(path.join(missingDirPrefix, 'resources', LINUX_X64.basename), ARTIFACT_BODY);
    assertUnavailable(() => resolve(optionsFor(missingDirResources, missingDirUserData)), {
      name: 'missing broker directory',
      resourcesPath: missingDirResources,
      userDataPath: missingDirUserData,
    });
    assertDataDirAbsent(missingDirUserData);

    const dirLinkPrefix = 'inv-dir-link';
    const dirLinkResources = ctx.dir(path.join(dirLinkPrefix, 'resources'));
    const dirLinkUserData = ctx.dir(path.join(dirLinkPrefix, 'user-data'));
    const realBrokerDir = ctx.dir(path.join(dirLinkPrefix, 'real-broker'));
    const dirSentinelPath = ctx.file(path.join(dirLinkPrefix, 'real-broker', 'SENTINEL'), SENTINEL);
    ctx.file(path.join(dirLinkPrefix, 'real-broker', 'manifest.json'), manifestJson('linux', 'x64'));
    ctx.file(path.join(dirLinkPrefix, 'real-broker', LINUX_X64.basename), ARTIFACT_BODY);
    ctx.link(path.join(dirLinkPrefix, 'resources', 'wallet-broker'), realBrokerDir);
    const dirSentinelBefore = snapshotFile(dirSentinelPath);
    assert.strictEqual(fs.lstatSync(realBrokerDir).isDirectory(), true);
    assert.strictEqual(fs.lstatSync(realBrokerDir).isSymbolicLink(), false);
    assert.strictEqual(fs.lstatSync(path.join(dirLinkResources, 'wallet-broker')).isSymbolicLink(), true);
    assertUnavailable(() => resolve(optionsFor(dirLinkResources, dirLinkUserData)), {
      name: 'broker directory symlink',
      resourcesPath: dirLinkResources,
      userDataPath: dirLinkUserData,
    });
    assertSameFile(dirSentinelPath, dirSentinelBefore);
    assert.strictEqual(fs.readFileSync(dirSentinelPath, 'utf8'), SENTINEL);
    assert.strictEqual(fs.lstatSync(realBrokerDir).isDirectory(), true);
    assert.strictEqual(fs.lstatSync(realBrokerDir).isSymbolicLink(), false);
    assertDataDirAbsent(dirLinkUserData);

    const manifestDirPrefix = 'inv-manifest-dir';
    const manifestDirResources = ctx.dir(path.join(manifestDirPrefix, 'resources'));
    const manifestDirUserData = ctx.dir(path.join(manifestDirPrefix, 'user-data'));
    ctx.dir(path.join(manifestDirPrefix, 'resources', 'wallet-broker'));
    const manifestAsDir = ctx.dir(path.join(manifestDirPrefix, 'resources', 'wallet-broker', 'manifest.json'));
    ctx.file(path.join(manifestDirPrefix, 'resources', 'wallet-broker', LINUX_X64.basename), ARTIFACT_BODY);
    const manifestDirSentinel = ctx.file(
      path.join(manifestDirPrefix, 'resources', 'wallet-broker', 'manifest.json', 'SENTINEL'),
      SENTINEL
    );
    const manifestDirSentinelBefore = snapshotFile(manifestDirSentinel);
    assert.strictEqual(fs.lstatSync(manifestAsDir).isDirectory(), true);
    assertUnavailable(() => resolve(optionsFor(manifestDirResources, manifestDirUserData)), {
      name: 'manifest directory',
      resourcesPath: manifestDirResources,
      userDataPath: manifestDirUserData,
    });
    assertSameFile(manifestDirSentinel, manifestDirSentinelBefore);
    assertDataDirAbsent(manifestDirUserData);

    const manifestLinkPrefix = 'inv-manifest-link';
    const manifestLinkResources = ctx.dir(path.join(manifestLinkPrefix, 'resources'));
    const manifestLinkUserData = ctx.dir(path.join(manifestLinkPrefix, 'user-data'));
    ctx.dir(path.join(manifestLinkPrefix, 'resources', 'wallet-broker'));
    const realManifest = ctx.file(
      path.join(manifestLinkPrefix, 'real-manifest.json'),
      manifestJson(LINUX_X64.platform, LINUX_X64.arch)
    );
    const manifestLinkSentinel = ctx.file(
      path.join(manifestLinkPrefix, 'SENTINEL'),
      SENTINEL
    );
    ctx.file(path.join(manifestLinkPrefix, 'resources', 'wallet-broker', LINUX_X64.basename), ARTIFACT_BODY);
    ctx.link(path.join(manifestLinkPrefix, 'resources', 'wallet-broker', 'manifest.json'), realManifest);
    const realManifestBefore = snapshotFile(realManifest);
    const manifestLinkSentinelBefore = snapshotFile(manifestLinkSentinel);
    const realManifestParsed = JSON.parse(realManifestBefore.bytes.toString('utf8'));
    assert.deepStrictEqual(realManifestParsed, {
      v: 1,
      platform: LINUX_X64.platform,
      arch: LINUX_X64.arch,
      sha256: FIXED_PIN,
    });
    assert.strictEqual(typeof realManifestParsed.v, 'number');
    assert.strictEqual(realManifestParsed.v, 1);
    assert.strictEqual(realManifestParsed.sha256, FIXED_PIN);
    assert.deepStrictEqual(
      Object.getOwnPropertyNames(realManifestParsed).sort(),
      ['arch', 'platform', 'sha256', 'v']
    );
    assert.ok(realManifestBefore.size >= 1);
    assert.ok(realManifestBefore.size <= 4096);
    assert.strictEqual(realManifestBefore.size, realManifestBefore.bytes.length);
    assert.strictEqual(
      realManifestBefore.bytes.toString('utf8'),
      manifestJson(LINUX_X64.platform, LINUX_X64.arch)
    );
    assert.strictEqual(
      fs.lstatSync(path.join(manifestLinkResources, 'wallet-broker', 'manifest.json')).isSymbolicLink(),
      true
    );
    assertUnavailable(() => resolve(optionsFor(manifestLinkResources, manifestLinkUserData)), {
      name: 'manifest symlink',
      resourcesPath: manifestLinkResources,
      userDataPath: manifestLinkUserData,
      secrets: [SENTINEL],
    });
    assertSameFile(realManifest, realManifestBefore);
    assertSameFile(manifestLinkSentinel, manifestLinkSentinelBefore);
    assert.strictEqual(fs.readFileSync(manifestLinkSentinel, 'utf8'), SENTINEL);
    assertDataDirAbsent(manifestLinkUserData);

    const missingBinPrefix = 'inv-missing-binary';
    const missingBinResources = ctx.dir(path.join(missingBinPrefix, 'resources'));
    const missingBinUserData = ctx.dir(path.join(missingBinPrefix, 'user-data'));
    ctx.dir(path.join(missingBinPrefix, 'resources', 'wallet-broker'));
    ctx.file(
      path.join(missingBinPrefix, 'resources', 'wallet-broker', 'manifest.json'),
      manifestJson('linux', 'x64')
    );
    const decoyPath = ctx.file(
      path.join(missingBinPrefix, 'resources', 'wallet-broker', 'bitbook-wallet-broker.exe'),
      ARTIFACT_BODY
    );
    const decoyBefore = snapshotFile(decoyPath);
    assert.throws(
      () => fs.lstatSync(path.join(missingBinResources, 'wallet-broker', LINUX_X64.basename)),
      { code: 'ENOENT' }
    );
    assertUnavailable(() => resolve(optionsFor(missingBinResources, missingBinUserData)), {
      name: 'missing binary',
      resourcesPath: missingBinResources,
      userDataPath: missingBinUserData,
    });
    assertSameFile(decoyPath, decoyBefore);
    assertDataDirAbsent(missingBinUserData);

    const binDirPrefix = 'inv-binary-dir';
    const binDirResources = ctx.dir(path.join(binDirPrefix, 'resources'));
    const binDirUserData = ctx.dir(path.join(binDirPrefix, 'user-data'));
    ctx.dir(path.join(binDirPrefix, 'resources', 'wallet-broker'));
    ctx.file(
      path.join(binDirPrefix, 'resources', 'wallet-broker', 'manifest.json'),
      manifestJson('linux', 'x64')
    );
    const binaryAsDir = ctx.dir(path.join(binDirPrefix, 'resources', 'wallet-broker', LINUX_X64.basename));
    const binaryDirSentinel = ctx.file(
      path.join(binDirPrefix, 'resources', 'wallet-broker', LINUX_X64.basename, 'SENTINEL'),
      SENTINEL
    );
    const binaryDirSentinelBefore = snapshotFile(binaryDirSentinel);
    assert.strictEqual(fs.lstatSync(binaryAsDir).isDirectory(), true);
    assertUnavailable(() => resolve(optionsFor(binDirResources, binDirUserData)), {
      name: 'binary directory',
      resourcesPath: binDirResources,
      userDataPath: binDirUserData,
    });
    assertSameFile(binaryDirSentinel, binaryDirSentinelBefore);
    assertDataDirAbsent(binDirUserData);

    const binLinkPrefix = 'inv-binary-link';
    const binLinkResources = ctx.dir(path.join(binLinkPrefix, 'resources'));
    const binLinkUserData = ctx.dir(path.join(binLinkPrefix, 'user-data'));
    ctx.dir(path.join(binLinkPrefix, 'resources', 'wallet-broker'));
    ctx.file(
      path.join(binLinkPrefix, 'resources', 'wallet-broker', 'manifest.json'),
      manifestJson('linux', 'x64')
    );
    const realBinary = ctx.file(path.join(binLinkPrefix, 'real-binary'), `${ARTIFACT_BODY}\n${SENTINEL}`);
    ctx.link(path.join(binLinkPrefix, 'resources', 'wallet-broker', LINUX_X64.basename), realBinary);
    const realBinaryBefore = snapshotFile(realBinary);
    assert.strictEqual(
      fs.lstatSync(path.join(binLinkResources, 'wallet-broker', LINUX_X64.basename)).isSymbolicLink(),
      true
    );
    assertUnavailable(() => resolve(optionsFor(binLinkResources, binLinkUserData)), {
      name: 'binary symlink',
      resourcesPath: binLinkResources,
      userDataPath: binLinkUserData,
      canary: SENTINEL,
    });
    assertSameFile(realBinary, realBinaryBefore);
    assert.ok(fs.readFileSync(realBinary, 'utf8').includes(SENTINEL));
    assertDataDirAbsent(binLinkUserData);
  });
});

test('reflection errors: revoked proxy is sanitized', () => {
  const resolve = loadResolver();
  const resourcesPath = path.resolve('/bb-wal011-inert-resources');
  const userDataPath = path.resolve('/bb-wal011-inert-user-data');
  const target = {
    resourcesPath,
    userDataPath,
    platform: 'linux',
    arch: 'x64',
  };
  const { proxy, revoke } = Proxy.revocable(target, {});
  revoke();
  assertUnavailable(() => resolve(proxy), {
    name: 'revoked proxy',
    canary: CANARY,
    resourcesPath,
    userDataPath,
  });
});

test('reflection errors: getPrototypeOf trap is sanitized', () => {
  const resolve = loadResolver();
  const resourcesPath = path.resolve('/bb-wal011-inert-resources');
  const userDataPath = path.resolve('/bb-wal011-inert-user-data');
  const target = {
    resourcesPath,
    userDataPath,
    platform: 'linux',
    arch: 'x64',
  };
  let trapCalls = 0;
  const options = new Proxy(target, {
    getPrototypeOf() {
      trapCalls += 1;
      throw new Error(CANARY);
    },
  });
  assertUnavailable(() => resolve(options), {
    name: 'getPrototypeOf trap',
    canary: CANARY,
    resourcesPath,
    userDataPath,
  });
  assert.strictEqual(trapCalls, 1, 'getPrototypeOf trap: ran once');
});

test('reflection errors: ownKeys trap is sanitized', () => {
  const resolve = loadResolver();
  const resourcesPath = path.resolve('/bb-wal011-inert-resources');
  const userDataPath = path.resolve('/bb-wal011-inert-user-data');
  const target = {
    resourcesPath,
    userDataPath,
    platform: 'linux',
    arch: 'x64',
  };
  let trapCalls = 0;
  const options = new Proxy(target, {
    ownKeys() {
      trapCalls += 1;
      throw new Error(CANARY);
    },
  });
  assertUnavailable(() => resolve(options), {
    name: 'ownKeys trap',
    canary: CANARY,
    resourcesPath,
    userDataPath,
  });
  assert.strictEqual(trapCalls, 1, 'ownKeys trap: ran once');
});

test('reflection errors: getOwnPropertyDescriptor trap is sanitized', () => {
  const resolve = loadResolver();
  const resourcesPath = path.resolve('/bb-wal011-inert-resources');
  const userDataPath = path.resolve('/bb-wal011-inert-user-data');
  const target = {
    resourcesPath,
    userDataPath,
    platform: 'linux',
    arch: 'x64',
  };
  let trapCalls = 0;
  const options = new Proxy(target, {
    getOwnPropertyDescriptor() {
      trapCalls += 1;
      throw new Error(CANARY);
    },
  });
  assertUnavailable(() => resolve(options), {
    name: 'getOwnPropertyDescriptor trap',
    canary: CANARY,
    resourcesPath,
    userDataPath,
  });
  assert.strictEqual(trapCalls, 1, 'getOwnPropertyDescriptor trap: ran once');
});

function run() {
  let failed = 0;
  for (const { name, fn } of tests) {
    try { fn(); process.stdout.write(`ok ${name}\n`); }
    catch (error) { failed += 1; process.stderr.write(`not ok ${name}\n${error.stack || error}\n`); }
  }
  if (failed) process.exit(1);
  process.stdout.write(`BitBook wallet broker launch configuration tests passed (${tests.length}).\n`);
}

if (require.main === module) run();
module.exports = { tests };
