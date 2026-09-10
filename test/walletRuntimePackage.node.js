'use strict';

const assert = require('assert');
const { spawnSync } = require('child_process');
const fs = require('fs');
const os = require('os');
const path = require('path');

const REPO_ROOT = path.resolve(__dirname, '..');
const HELPER_PATH = path.resolve(REPO_ROOT, 'scripts', 'stage-wallet-runtime.js');
const DEB_SCRIPT = path.join(REPO_ROOT, 'scripts', 'build-deb.sh');
const MAC_SCRIPT = path.join(REPO_ROOT, 'scripts', 'build-macos.sh');
const WIN_SCRIPT = path.join(REPO_ROOT, 'scripts', 'build-windows.ps1');
const HELPER_MISSING = 'wallet runtime staging helper is missing';
const CLI_ERROR = 'Unable to stage wallet runtime';
const CHILD_TIMEOUT_MS = 15000;
const POSIX = process.platform !== 'win32';
const BASH_STAGE_CALL = 'node scripts/stage-wallet-runtime.js "$APP_SOURCE"';
const PS_STAGE_CALL = '& node scripts/stage-wallet-runtime.js $AppSource';
const INVENTORY = Object.freeze([
  'wallet-preload.js',
  'wallet-broker/protocol.js',
  'wallet-broker/supervisor.js',
  'wallet-broker/launch-config.js',
  'wallet-pay/model.js',
]);
const CONTENTS = Object.freeze({
  'wallet-preload.js': 'PRELOAD_FIXTURE_BYTES',
  'wallet-broker/protocol.js': 'PROTOCOL_FIXTURE_BYTES',
  'wallet-broker/supervisor.js': 'SUPERVISOR_FIXTURE_BYTES',
  'wallet-broker/launch-config.js': 'LAUNCH_CONFIG_FIXTURE_BYTES',
  'wallet-pay/model.js': 'MODEL_FIXTURE_BYTES',
});
const DEST_PARENTS = Object.freeze(['wallet-broker', 'wallet-pay']);
const DECOY_RELS = Object.freeze([
  'SENTINEL',
  'extra.js',
  'Cargo.toml',
  'wallet-broker/lib.rs',
  'wallet-broker/bitbook-wallet-broker',
  'target',
  'target/release',
  'target/release/bitbook-wallet-broker',
  'wallet-pay/native.rs',
]);

assert.strictEqual(INVENTORY.length, 5);
assert.strictEqual(new Set(Object.values(CONTENTS)).size, 5);

const tests = [];
function test(name, fn) { tests.push({ name, fn }); }

function joinInv(root, rel) {
  return path.join(root, ...String(rel).split('/'));
}

function loadHelper() {
  assert.ok(fs.existsSync(HELPER_PATH), HELPER_MISSING);
  const exported = require(HELPER_PATH);
  assert.strictEqual(typeof exported.stageWalletRuntime, 'function');
  return exported.stageWalletRuntime;
}

function appendCleanup(previous, error) {
  if (!previous) return error;
  previous.message = `${previous.message}\ncleanup also failed: ${error.stack || error.message}`;
  return previous;
}

function withOwnedTemp(fn) {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), 'bb-wal011-runtime-pkg-'));
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

function assertSameFile(abs, before) {
  assert.deepStrictEqual(snapshotStat(abs), before);
}

function assertSameTree(root, before) {
  assert.deepStrictEqual(snapshotTree(root), before);
}

function assertAbsent(abs, label) {
  assert.throws(() => fs.lstatSync(abs), { code: 'ENOENT' }, label || abs);
}

function trackIfPresent(ctx, appRoot) {
  function walk(abs) {
    let stat;
    try {
      stat = fs.lstatSync(abs);
    } catch (_) {
      return;
    }
    if (stat.isSymbolicLink()) {
      ctx.noteLink(abs);
      return;
    }
    if (stat.isFile()) {
      ctx.noteFile(abs);
      return;
    }
    if (!stat.isDirectory()) return;
    ctx.noteDir(abs);
    let names;
    try {
      names = fs.readdirSync(abs);
    } catch (_) {
      return;
    }
    for (const name of names) walk(path.join(abs, name));
  }
  walk(appRoot);
}

function spawnNode(args) {
  return spawnSync(process.execPath, args, {
    encoding: 'utf8',
    timeout: CHILD_TIMEOUT_MS,
    windowsHide: true,
  });
}

function assertNotTimedOut(result, label) {
  assert.ok(!(result.error && result.error.code === 'ETIMEDOUT'), `${label}: timed out`);
}

function writeValidSource(ctx, sourceRel) {
  const sourceRoot = ctx.dir(sourceRel);
  ctx.dir(path.join(sourceRel, 'wallet-broker'));
  ctx.dir(path.join(sourceRel, 'wallet-pay'));
  ctx.dir(path.join(sourceRel, 'target', 'release'));
  const files = {};
  for (const rel of INVENTORY) {
    files[rel] = ctx.file(path.join(sourceRel, rel), CONTENTS[rel]);
    if (POSIX) fs.chmodSync(files[rel], 0o755);
  }
  const decoys = {
    sentinel: ctx.file(path.join(sourceRel, 'SENTINEL'), 'SOURCE_SENTINEL'),
    extraJs: ctx.file(path.join(sourceRel, 'extra.js'), 'not-staged-extra-js'),
    cargo: ctx.file(path.join(sourceRel, 'Cargo.toml'), '[package]\nname="decoy"\n'),
    rust: ctx.file(path.join(sourceRel, 'wallet-broker', 'lib.rs'), 'pub fn decoy() {}'),
    native: ctx.file(path.join(sourceRel, 'wallet-broker', 'bitbook-wallet-broker'), 'NATIVE_BINARY_DECOY'),
    targetBin: ctx.file(path.join(sourceRel, 'target', 'release', 'bitbook-wallet-broker'), 'TARGET_NATIVE_DECOY'),
    payRs: ctx.file(path.join(sourceRel, 'wallet-pay', 'native.rs'), 'mod decoy;'),
  };
  return { sourceRoot, files, decoys };
}

function writeValidApp(ctx, appRel) {
  const appRoot = ctx.dir(appRel);
  ctx.dir(path.join(appRel, 'social'));
  ctx.dir(path.join(appRel, 'imgs'));
  const files = {
    pkg: ctx.file(path.join(appRel, 'package.json'), '{"name":"existing-package"}'),
    main: ctx.file(path.join(appRel, 'social-main.js'), 'EXISTING_SOCIAL_MAIN'),
    social: ctx.file(path.join(appRel, 'social', 'index.js'), 'EXISTING_SOCIAL'),
    img: ctx.file(path.join(appRel, 'imgs', 'icon.png'), 'EXISTING_ICON'),
    extra: ctx.file(path.join(appRel, 'APP_SENTINEL'), 'APP_UNCHANGED'),
  };
  return { appRoot, files };
}

function writeSourceWithDefect(ctx, sourceRel, defect) {
  const sourceRoot = ctx.dir(sourceRel);
  const parentLink = defect.parentLink || null;
  let realParent = null;
  let parentSentinel = null;
  let fileLinkTarget = null;
  let dirSentinel = null;

  if (parentLink === 'wallet-broker') {
    realParent = ctx.dir(path.join(sourceRel, 'real-wallet-broker'));
    parentSentinel = ctx.file(path.join(sourceRel, 'real-wallet-broker', 'SENTINEL'), 'SRC_PARENT_SENTINEL');
  } else {
    ctx.dir(path.join(sourceRel, 'wallet-broker'));
  }
  if (parentLink === 'wallet-pay') {
    realParent = ctx.dir(path.join(sourceRel, 'real-wallet-pay'));
    parentSentinel = ctx.file(path.join(sourceRel, 'real-wallet-pay', 'SENTINEL'), 'SRC_PARENT_SENTINEL');
  } else {
    ctx.dir(path.join(sourceRel, 'wallet-pay'));
  }

  for (const rel of INVENTORY) {
    if (defect.missing === rel) continue;
    if (defect.asDir === rel) {
      ctx.dir(path.join(sourceRel, rel));
      dirSentinel = ctx.file(path.join(sourceRel, rel, 'SENTINEL'), 'SRC_AS_DIR_SENTINEL');
      continue;
    }
    if (defect.asLink === rel) {
      fileLinkTarget = ctx.file(
        path.join(sourceRel, `link-target-${path.basename(rel)}`),
        'SRC_FILE_LINK_SENTINEL'
      );
      ctx.link(path.join(sourceRel, rel), fileLinkTarget);
      continue;
    }
    const destRel = parentLink && rel.startsWith(`${parentLink}/`)
      ? path.join(sourceRel, `real-${parentLink}`, path.basename(rel))
      : path.join(sourceRel, rel);
    ctx.file(destRel, CONTENTS[rel]);
  }

  if (parentLink) ctx.link(path.join(sourceRel, parentLink), realParent);
  return { sourceRoot, realParent, parentSentinel, fileLinkTarget, dirSentinel };
}

function writeAppWithDefect(ctx, appRel, defect) {
  const installed = writeValidApp(ctx, appRel);
  let destParentReal = null;
  let destParentSentinel = null;
  let destParentFile = null;
  let danglingTarget = null;
  let existingFile = null;

  if (defect.destParentLink) {
    destParentReal = ctx.dir(path.join(appRel, `real-${defect.destParentLink}`));
    destParentSentinel = ctx.file(
      path.join(appRel, `real-${defect.destParentLink}`, 'SENTINEL'),
      'DEST_PARENT_SENTINEL'
    );
    ctx.link(path.join(appRel, defect.destParentLink), destParentReal);
  } else if (defect.destParentFile) {
    destParentFile = ctx.file(path.join(appRel, defect.destParentFile), 'DEST_PARENT_IS_FILE');
  } else if (defect.existing) {
    const parent = path.posix.dirname(defect.existing);
    if (parent !== '.') ctx.dir(path.join(appRel, parent));
    existingFile = ctx.file(path.join(appRel, defect.existing), `EXISTING:${defect.existing}`);
  } else if (defect.dangling) {
    const parent = path.posix.dirname(defect.dangling);
    if (parent !== '.') ctx.dir(path.join(appRel, parent));
    danglingTarget = path.join(ctx.root, 'dangling-target-absent');
    ctx.link(path.join(appRel, defect.dangling), danglingTarget);
  }

  return Object.assign(installed, {
    destParentReal,
    destParentSentinel,
    destParentFile,
    danglingTarget,
    existingFile,
  });
}

function assertNoInventory(appRoot) {
  for (const rel of INVENTORY) assertAbsent(joinInv(appRoot, rel), rel);
}

function assertNoDecoys(appRoot) {
  for (const rel of DECOY_RELS) assertAbsent(joinInv(appRoot, rel), rel);
}

function assertStagedExact(appRoot, sourceFiles) {
  assert.deepStrictEqual(fs.readdirSync(appRoot).sort(), [
    'APP_SENTINEL',
    'imgs',
    'package.json',
    'social',
    'social-main.js',
    'wallet-broker',
    'wallet-pay',
    'wallet-preload.js',
  ]);
  assert.deepStrictEqual(fs.readdirSync(path.join(appRoot, 'wallet-broker')).sort(), [
    'launch-config.js',
    'protocol.js',
    'supervisor.js',
  ]);
  assert.deepStrictEqual(fs.readdirSync(path.join(appRoot, 'wallet-pay')).sort(), [
    'model.js',
  ]);
  for (const rel of INVENTORY) {
    const dest = joinInv(appRoot, rel);
    const stat = fs.lstatSync(dest);
    assert.strictEqual(stat.isSymbolicLink(), false, `${rel}: not symlink`);
    assert.strictEqual(stat.isFile(), true, `${rel}: file`);
    assert.deepStrictEqual(fs.readFileSync(dest), fs.readFileSync(sourceFiles[rel]));
    assert.deepStrictEqual(fs.readFileSync(dest), Buffer.from(CONTENTS[rel]));
    if (POSIX) assert.strictEqual(stat.mode & 0o777, 0o644, `${rel}: mode 0644`);
  }
  assertNoDecoys(appRoot);
}

function indexOnce(text, snippet, label) {
  const first = text.indexOf(snippet);
  assert.notStrictEqual(first, -1, `${label}: missing ${snippet}`);
  assert.strictEqual(text.indexOf(snippet, first + 1), -1, `${label}: duplicate ${snippet}`);
  return first;
}

function nextNonEmptyLine(text, start) {
  const rest = text.slice(start);
  const lines = rest.split(/\r?\n/);
  for (let i = 1; i < lines.length; i += 1) {
    const line = lines[i].trim();
    if (line) return line;
  }
  return '';
}

function assertCliFailure(result, label) {
  assertNotTimedOut(result, label);
  assert.strictEqual(result.status, 1, `${label}: exit 1`);
  const lines = String(result.stderr || '').split(/\r?\n/).filter((line) => line.length > 0);
  assert.ok(lines.includes(CLI_ERROR), `${label}: stderr ${result.stderr}`);
}

test('staging: exact five-file inventory preserves bytes and excludes native data', () => {
  const stage = loadHelper();
  withOwnedTemp((ctx) => {
    const source = writeValidSource(ctx, 'src');
    const app = writeValidApp(ctx, 'app');
    const sourceBefore = snapshotTree(source.sourceRoot);
    const appBefore = snapshotTree(app.appRoot);
    const appFileBefore = {
      pkg: snapshotStat(app.files.pkg),
      main: snapshotStat(app.files.main),
      social: snapshotStat(app.files.social),
      img: snapshotStat(app.files.img),
      extra: snapshotStat(app.files.extra),
    };
    for (const rel of INVENTORY) {
      if (POSIX) assert.strictEqual(fs.statSync(source.files[rel]).mode & 0o111, 0o111);
    }
    try {
      stage(source.sourceRoot, app.appRoot);
    } finally {
      trackIfPresent(ctx, app.appRoot);
    }
    assertStagedExact(app.appRoot, source.files);
    assertSameTree(source.sourceRoot, sourceBefore);
    assertSameFile(app.files.pkg, appFileBefore.pkg);
    assertSameFile(app.files.main, appFileBefore.main);
    assertSameFile(app.files.social, appFileBefore.social);
    assertSameFile(app.files.img, appFileBefore.img);
    assertSameFile(app.files.extra, appFileBefore.extra);
    assert.strictEqual(fs.readFileSync(app.files.extra, 'utf8'), 'APP_UNCHANGED');
    assert.strictEqual(fs.readFileSync(source.decoys.sentinel, 'utf8'), 'SOURCE_SENTINEL');
    const appAfter = snapshotTree(app.appRoot);
    for (const rel of Object.keys(appBefore)) {
      assert.deepStrictEqual(appAfter[rel], appBefore[rel], `preserved ${rel}`);
    }
  });
});

test('preflight: missing and hostile source entries leave destination unchanged', () => {
  const stage = loadHelper();
  const rows = [
    { name: 'missing wallet-preload.js', missing: 'wallet-preload.js' },
    { name: 'missing protocol.js', missing: 'wallet-broker/protocol.js' },
    { name: 'missing supervisor.js', missing: 'wallet-broker/supervisor.js' },
    { name: 'missing launch-config.js', missing: 'wallet-broker/launch-config.js' },
    { name: 'missing model.js', missing: 'wallet-pay/model.js' },
    { name: 'source model.js is directory', asDir: 'wallet-pay/model.js' },
    { name: 'source launch-config.js is symlink', asLink: 'wallet-broker/launch-config.js' },
    { name: 'source parent wallet-broker is symlink', parentLink: 'wallet-broker' },
    { name: 'source parent wallet-pay is symlink', parentLink: 'wallet-pay' },
  ];
  assert.strictEqual(rows.length, 9);
  for (const row of rows) {
    withOwnedTemp((ctx) => {
      const source = writeSourceWithDefect(ctx, 'src', row);
      const app = writeValidApp(ctx, 'app');
      const sourceBefore = snapshotTree(source.sourceRoot);
      const appBefore = snapshotTree(app.appRoot);
      const realParentBefore = source.realParent ? snapshotTree(source.realParent) : null;
      const parentSentinelBefore = source.parentSentinel ? snapshotStat(source.parentSentinel) : null;
      const fileLinkBefore = source.fileLinkTarget ? snapshotStat(source.fileLinkTarget) : null;
      const dirSentinelBefore = source.dirSentinel ? snapshotStat(source.dirSentinel) : null;
      try {
        assert.throws(() => stage(source.sourceRoot, app.appRoot), `${row.name}: expected throw`);
      } finally {
        trackIfPresent(ctx, app.appRoot);
      }
      assertSameTree(app.appRoot, appBefore);
      assertSameTree(source.sourceRoot, sourceBefore);
      assertNoInventory(app.appRoot);
      if (parentSentinelBefore) {
        assertSameFile(source.parentSentinel, parentSentinelBefore);
        assertSameTree(source.realParent, realParentBefore);
        assert.strictEqual(fs.readFileSync(source.parentSentinel, 'utf8'), 'SRC_PARENT_SENTINEL');
        assert.strictEqual(fs.lstatSync(joinInv(source.sourceRoot, row.parentLink)).isSymbolicLink(), true);
        assertAbsent(joinInv(app.appRoot, path.join(row.parentLink, 'SENTINEL')), `${row.name}: no recursive copy`);
      }
      if (fileLinkBefore) {
        assertSameFile(source.fileLinkTarget, fileLinkBefore);
        assert.strictEqual(fs.readFileSync(source.fileLinkTarget, 'utf8'), 'SRC_FILE_LINK_SENTINEL');
        assert.strictEqual(fs.lstatSync(joinInv(source.sourceRoot, row.asLink)).isSymbolicLink(), true);
        assertAbsent(joinInv(app.appRoot, row.asLink), `${row.name}: no followed symlink copy`);
      }
      if (dirSentinelBefore) {
        assertSameFile(source.dirSentinel, dirSentinelBefore);
        assert.strictEqual(fs.readFileSync(source.dirSentinel, 'utf8'), 'SRC_AS_DIR_SENTINEL');
        assert.strictEqual(fs.lstatSync(joinInv(source.sourceRoot, row.asDir)).isDirectory(), true);
        assertAbsent(joinInv(app.appRoot, path.join(row.asDir, 'SENTINEL')), `${row.name}: no recursive dir copy`);
      }
    });
  }
});

test('preflight: existing and hostile destinations are never overwritten', () => {
  const stage = loadHelper();
  const rows = [
    { name: 'existing dest wallet-preload.js', existing: 'wallet-preload.js' },
    { name: 'existing dest protocol.js', existing: 'wallet-broker/protocol.js' },
    { name: 'existing dest supervisor.js', existing: 'wallet-broker/supervisor.js' },
    { name: 'existing dest launch-config.js', existing: 'wallet-broker/launch-config.js' },
    { name: 'existing dest model.js', existing: 'wallet-pay/model.js' },
    { name: 'dangling dest model.js symlink', dangling: 'wallet-pay/model.js' },
    { name: 'sourceRoot is symlink', sourceSymlink: true },
    { name: 'appRoot is symlink', appSymlink: true },
    { name: 'dest parent wallet-broker is symlink', destParentLink: 'wallet-broker' },
    { name: 'dest parent wallet-pay is symlink', destParentLink: 'wallet-pay' },
    { name: 'dest parent wallet-broker is file', destParentFile: 'wallet-broker' },
    { name: 'dest parent wallet-pay is file', destParentFile: 'wallet-pay' },
  ];
  assert.strictEqual(rows.length, 12);
  for (const row of rows) {
    withOwnedTemp((ctx) => {
      const source = writeValidSource(ctx, row.sourceSymlink ? 'real-src' : 'src');
      const app = writeAppWithDefect(ctx, row.appSymlink ? 'real-app' : 'app', row);
      const sourceRoot = row.sourceSymlink ? ctx.link('src-link', source.sourceRoot) : source.sourceRoot;
      const appRoot = row.appSymlink ? ctx.link('app-link', app.appRoot) : app.appRoot;
      const sourceBefore = snapshotTree(source.sourceRoot);
      const appBefore = snapshotTree(app.appRoot);
      const existingBefore = row.existing ? snapshotStat(joinInv(app.appRoot, row.existing)) : null;
      const destParentSentinelBefore = app.destParentSentinel ? snapshotStat(app.destParentSentinel) : null;
      const destParentFileBefore = app.destParentFile ? snapshotStat(app.destParentFile) : null;
      const destParentRealBefore = app.destParentReal ? snapshotTree(app.destParentReal) : null;
      try {
        assert.throws(() => stage(sourceRoot, appRoot), `${row.name}: expected throw`);
      } finally {
        trackIfPresent(ctx, app.appRoot);
      }
      assertSameTree(app.appRoot, appBefore);
      assertSameTree(source.sourceRoot, sourceBefore);
      if (row.sourceSymlink) {
        assert.strictEqual(fs.lstatSync(sourceRoot).isSymbolicLink(), true);
      }
      if (row.appSymlink) {
        assert.strictEqual(fs.lstatSync(appRoot).isSymbolicLink(), true);
      }
      if (existingBefore) {
        assertSameFile(joinInv(app.appRoot, row.existing), existingBefore);
        assert.strictEqual(
          fs.readFileSync(joinInv(app.appRoot, row.existing), 'utf8'),
          `EXISTING:${row.existing}`
        );
        for (const rel of INVENTORY) {
          if (rel === row.existing) continue;
          assertAbsent(joinInv(app.appRoot, rel), `${row.name}: no extra ${rel}`);
        }
      }
      if (row.dangling) {
        const dest = joinInv(app.appRoot, row.dangling);
        assert.strictEqual(fs.lstatSync(dest).isSymbolicLink(), true, `${row.name}: still symlink`);
        assert.strictEqual(fs.readlinkSync(dest), app.danglingTarget);
        assertAbsent(app.danglingTarget, `${row.name}: dangling target stays absent`);
      }
      if (destParentSentinelBefore) {
        assertSameFile(app.destParentSentinel, destParentSentinelBefore);
        assert.strictEqual(fs.readFileSync(app.destParentSentinel, 'utf8'), 'DEST_PARENT_SENTINEL');
        assertSameTree(app.destParentReal, destParentRealBefore);
        assert.deepStrictEqual(fs.readdirSync(app.destParentReal), ['SENTINEL']);
        assert.strictEqual(fs.lstatSync(path.join(app.appRoot, row.destParentLink)).isSymbolicLink(), true);
      }
      if (destParentFileBefore) {
        assertSameFile(app.destParentFile, destParentFileBefore);
        assert.strictEqual(fs.readFileSync(app.destParentFile, 'utf8'), 'DEST_PARENT_IS_FILE');
      }
    });
  }
});

test('arguments: invalid roots fail before staging', () => {
  const stage = loadHelper();
  withOwnedTemp((ctx) => {
    const source = writeValidSource(ctx, 'src');
    const app = writeValidApp(ctx, 'app');
    const sourceBefore = snapshotTree(source.sourceRoot);
    const appBefore = snapshotTree(app.appRoot);
    const missingSource = path.join(ctx.root, 'missing-source-root');
    const missingApp = path.join(ctx.root, 'missing-app-root');
    const sourceAsFile = ctx.file('source-as-file', 'not-a-directory');
    const appAsFile = ctx.file('app-as-file', 'not-a-directory');
    const sourceAsFileBefore = snapshotStat(sourceAsFile);
    const appAsFileBefore = snapshotStat(appAsFile);
    const relSource = path.relative(process.cwd(), source.sourceRoot);
    const relApp = path.relative(process.cwd(), app.appRoot);
    assert.ok(relSource, 'relative sourceRoot is nonempty');
    assert.strictEqual(path.isAbsolute(relSource), false, 'relative sourceRoot is not absolute');
    assert.strictEqual(path.resolve(relSource), source.sourceRoot);
    assert.ok(relApp, 'relative appRoot is nonempty');
    assert.strictEqual(path.isAbsolute(relApp), false, 'relative appRoot is not absolute');
    assert.strictEqual(path.resolve(relApp), app.appRoot);
    assertAbsent(missingSource);
    assertAbsent(missingApp);
    const rows = [
      { name: 'missing both arguments', call: () => stage() },
      { name: 'missing appRoot', call: () => stage(source.sourceRoot) },
      { name: 'null sourceRoot', call: () => stage(null, app.appRoot) },
      { name: 'null appRoot', call: () => stage(source.sourceRoot, null) },
      { name: 'number sourceRoot', call: () => stage(1, app.appRoot) },
      { name: 'number appRoot', call: () => stage(source.sourceRoot, 1) },
      { name: 'object sourceRoot', call: () => stage({ path: source.sourceRoot }, app.appRoot) },
      { name: 'boolean appRoot', call: () => stage(source.sourceRoot, false) },
      { name: 'empty sourceRoot', call: () => stage('', app.appRoot) },
      { name: 'empty appRoot', call: () => stage(source.sourceRoot, '') },
      { name: 'relative sourceRoot', call: () => stage(relSource, app.appRoot) },
      { name: 'relative appRoot', call: () => stage(source.sourceRoot, relApp) },
      { name: 'missing sourceRoot directory', call: () => stage(missingSource, app.appRoot) },
      { name: 'missing appRoot directory', call: () => stage(source.sourceRoot, missingApp) },
      { name: 'sourceRoot as file', call: () => stage(sourceAsFile, app.appRoot) },
      { name: 'appRoot as file', call: () => stage(source.sourceRoot, appAsFile) },
    ];
    assert.strictEqual(rows.length, 16);
    for (const row of rows) {
      try {
        assert.throws(row.call, `${row.name}: expected throw`);
      } finally {
        trackIfPresent(ctx, app.appRoot);
      }
      assertSameTree(app.appRoot, appBefore);
      assertSameTree(source.sourceRoot, sourceBefore);
      assertNoInventory(app.appRoot);
      assertAbsent(missingSource, `${row.name}: missing source still absent`);
      assertAbsent(missingApp, `${row.name}: missing app still absent`);
      assertSameFile(sourceAsFile, sourceAsFileBefore);
      assertSameFile(appAsFile, appAsFileBefore);
    }
  });
});

test('cli: real checkout stages modules whose nested imports resolve', () => {
  const stage = loadHelper();
  assert.strictEqual(typeof stage, 'function');
  withOwnedTemp((ctx) => {
    const app = writeValidApp(ctx, 'app');
    const probe = ctx.file('require-probe.js', [
      "'use strict';",
      "const assert = require('assert');",
      "const path = require('path');",
      'const appRoot = process.argv[2];',
      "const supervisor = require(path.join(appRoot, 'wallet-broker', 'supervisor.js'));",
      "const protocol = require(path.join(appRoot, 'wallet-broker', 'protocol.js'));",
      "const launchConfig = require(path.join(appRoot, 'wallet-broker', 'launch-config.js'));",
      "const model = require(path.join(appRoot, 'wallet-pay', 'model.js'));",
      "assert.strictEqual(typeof supervisor.createWalletSupervisor, 'function');",
      "assert.strictEqual(typeof supervisor.createBrokerDispatcher, 'function');",
      "assert.strictEqual(typeof supervisor.sanitizeSnapshot, 'function');",
      "assert.strictEqual(typeof protocol.encodeBrokerFrame, 'function');",
      "assert.strictEqual(typeof protocol.createProtocolSession, 'function');",
      "assert.strictEqual(typeof protocol.validateHello, 'function');",
      "assert.strictEqual(typeof protocol.validateHelloAck, 'function');",
      "assert.strictEqual(typeof protocol.createBrokerFrameDecoder, 'function');",
      "assert.strictEqual(typeof protocol.computeSessionId, 'function');",
      "assert.strictEqual(typeof protocol.normalizeBrokerError, 'function');",
      "assert.strictEqual(typeof launchConfig.resolveWalletBrokerLaunch, 'function');",
      "assert.strictEqual(typeof model.sanitizeWalletSnapshot, 'function');",
      "assert.strictEqual(typeof model.derivePayView, 'function');",
      "assert.strictEqual(typeof model.buildPayeeReceiverParams, 'function');",
      "process.stdout.write('nested-imports-resolved\\n');",
      '',
    ].join('\n'));
    const appBefore = snapshotTree(app.appRoot);
    let staged;
    try {
      staged = spawnNode([HELPER_PATH, app.appRoot]);
    } finally {
      trackIfPresent(ctx, app.appRoot);
    }
    assertNotTimedOut(staged, 'stage cli');
    assert.strictEqual(staged.status, 0, `stage cli stderr: ${staged.stderr}`);
    for (const rel of INVENTORY) {
      const dest = joinInv(app.appRoot, rel);
      const fromRepo = joinInv(REPO_ROOT, rel);
      const stat = fs.lstatSync(dest);
      assert.strictEqual(stat.isSymbolicLink(), false, `${rel}: copy not symlink`);
      assert.strictEqual(stat.isFile(), true, `${rel}: staged file`);
      assert.deepStrictEqual(fs.readFileSync(dest), fs.readFileSync(fromRepo));
      if (POSIX) assert.strictEqual(stat.mode & 0o777, 0o644, `${rel}: mode 0644`);
    }
    assert.deepStrictEqual(fs.readdirSync(path.join(app.appRoot, 'wallet-broker')).sort(), [
      'launch-config.js',
      'protocol.js',
      'supervisor.js',
    ]);
    assert.deepStrictEqual(fs.readdirSync(path.join(app.appRoot, 'wallet-pay')).sort(), [
      'model.js',
    ]);
    for (const rel of Object.keys(appBefore)) {
      assert.deepStrictEqual(snapshotStat(joinInv(app.appRoot, rel)), appBefore[rel], `preserved ${rel}`);
    }
    const resolved = spawnNode([probe, app.appRoot]);
    assertNotTimedOut(resolved, 'nested require');
    assert.strictEqual(resolved.status, 0, `nested require stderr: ${resolved.stderr}`);
    assert.ok(
      String(resolved.stdout).includes('nested-imports-resolved'),
      `nested require stdout: ${resolved.stdout}`
    );
  });
});

test('cli: missing and extra arguments fail without staging', () => {
  const stage = loadHelper();
  assert.strictEqual(typeof stage, 'function');
  const rows = [
    { name: 'no arguments', args: [HELPER_PATH] },
    { name: 'appRoot plus extra argument', extra: true },
  ];
  assert.strictEqual(rows.length, 2);
  for (const row of rows) {
    withOwnedTemp((ctx) => {
      const app = writeValidApp(ctx, 'app');
      const appBefore = snapshotTree(app.appRoot);
      const args = row.extra ? [HELPER_PATH, app.appRoot, 'extra'] : row.args;
      let result;
      try {
        result = spawnNode(args);
      } finally {
        trackIfPresent(ctx, app.appRoot);
      }
      assertCliFailure(result, row.name);
      assertSameTree(app.appRoot, appBefore);
      assertNoInventory(app.appRoot);
    });
  }
});

test('packagers: shared staging precedes signing and archive with Linux disk storage', () => {
  loadHelper();
  const deb = fs.readFileSync(DEB_SCRIPT, 'utf8');
  const mac = fs.readFileSync(MAC_SCRIPT, 'utf8');
  const win = fs.readFileSync(WIN_SCRIPT, 'utf8');

  const debApp = indexOnce(deb, 'APP_SOURCE="$APP_DIR/resources/app"', 'deb APP_SOURCE');
  const debIcon = indexOnce(deb, 'install -m 0644 imgs/icon.png "$APP_SOURCE/imgs/icon.png"', 'deb icon');
  const debCall = indexOnce(deb, BASH_STAGE_CALL, 'deb helper');
  const debDeb = indexOnce(deb, 'dpkg-deb --build --root-owner-group "$PACKAGE_ROOT" "$OUTPUT"', 'deb dpkg-deb');
  assert.ok(debApp < debCall, 'deb: APP_SOURCE before helper');
  assert.ok(debIcon < debCall, 'deb: app copies before helper');
  assert.ok(debCall < debDeb, 'deb: helper before dpkg-deb');
  assert.strictEqual((deb.match(/stage-wallet-runtime\.js/g) || []).length, 1);

  const distDir = indexOnce(deb, 'install -d "$PROJECT_ROOT/dist"', 'deb dist');
  const mktemp = indexOnce(
    deb,
    'BUILD_ROOT="$(mktemp -d "$PROJECT_ROOT/dist/bitbook-deb.XXXXXX")"',
    'deb mktemp'
  );
  assert.ok(distDir < mktemp, 'deb: create dist before mktemp');
  assert.ok(!deb.includes('${TMPDIR:-/tmp}/bitbook-deb'), 'deb: no TMPDIR build root');
  assert.ok(!/trap\s+['"][^'"]*rm\s+-rf/.test(deb), 'deb: no recursive EXIT trap');
  assert.ok(!deb.includes('rm -rf -- "$BUILD_ROOT"'), 'deb: no recursive BUILD_ROOT deletion');
  assert.match(deb, /echo[^\n]*\$BUILD_ROOT/, 'deb: print staging path');

  const macApp = indexOnce(mac, 'APP_SOURCE="$APP_BUNDLE/Contents/Resources/app"', 'mac APP_SOURCE');
  const macIcon = indexOnce(mac, 'install -m 0644 imgs/icon.png "$APP_SOURCE/imgs/icon.png"', 'mac icon');
  const macCall = indexOnce(mac, BASH_STAGE_CALL, 'mac helper');
  const macSign = indexOnce(mac, 'codesign --force --deep --sign - "$APP_BUNDLE"', 'mac codesign');
  assert.ok(macApp < macCall, 'mac: APP_SOURCE before helper');
  assert.ok(macIcon < macCall, 'mac: app copies before helper');
  assert.ok(macCall < macSign, 'mac: helper before codesign');
  assert.strictEqual((mac.match(/stage-wallet-runtime\.js/g) || []).length, 1);

  const winApp = indexOnce(win, "$AppSource = Join-Path $BundleDir 'resources/app'", 'win AppSource');
  const winIcon = indexOnce(win, "Copy-Item imgs/icon.png (Join-Path $AppSource 'imgs/icon.png')", 'win icon');
  const winCall = indexOnce(win, PS_STAGE_CALL, 'win helper');
  const winZip = indexOnce(win, 'Compress-Archive', 'win archive');
  assert.ok(winApp < winCall, 'win: AppSource before helper');
  assert.ok(winIcon < winCall, 'win: app copies before helper');
  assert.ok(winCall < winZip, 'win: helper before Compress-Archive');
  assert.strictEqual((win.match(/stage-wallet-runtime\.js/g) || []).length, 1);
  const guard = nextNonEmptyLine(win, winCall + PS_STAGE_CALL.length);
  assert.match(guard, /\$LASTEXITCODE/, 'win: LASTEXITCODE guard');
  assert.match(guard, /-ne\s*0/, 'win: nonzero check');
  assert.match(guard, /throw/, 'win: throw on nonzero');
  const guardIndex = win.indexOf(guard, winCall);
  assert.ok(guardIndex !== -1 && guardIndex < winZip, 'win: guard before archive');
});

function run() {
  assert.strictEqual(tests.length, 7);
  let failed = 0;
  for (const { name, fn } of tests) {
    try { fn(); process.stdout.write(`ok ${name}\n`); }
    catch (error) { failed += 1; process.stderr.write(`not ok ${name}\n${error.stack || error}\n`); }
  }
  if (failed) process.exit(1);
  process.stdout.write('BitBook wallet runtime package tests passed (7).\n');
}

if (require.main === module) run();
module.exports = { tests };
