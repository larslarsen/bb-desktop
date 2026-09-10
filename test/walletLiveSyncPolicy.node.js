'use strict';

const assert = require('assert');
const fs = require('fs');
const path = require('path');

const repoRoot = path.resolve(__dirname, '..');
const manifestPath = path.join(repoRoot, 'wallet-broker', 'Cargo.toml');
const policy = require(path.join(repoRoot, 'scripts', 'security-policy.js'));

const LIVE_DEPENDENCIES = [
  {
    name: 'zcash_client_backend',
    declaration:
      'zcash_client_backend = { version = "=0.24.0", default-features = false, features = ["lightwalletd-tonic", "pczt"] }',
    changedVersion: '=0.24.1',
    extraFeature: 'lightwalletd-tonic-transport',
  },
  {
    name: 'tonic',
    declaration:
      'tonic = { version = "=0.14.6", default-features = false, features = ["channel", "tls-ring", "tls-webpki-roots"] }',
    changedVersion: '=0.14.7',
    extraFeature: 'server',
  },
  {
    name: 'tokio',
    declaration:
      'tokio = { version = "=1.52.3", default-features = false, features = ["rt", "net", "time", "macros"] }',
    changedVersion: '=1.52.4',
    extraFeature: 'signal',
  },
  {
    name: 'prost',
    declaration:
      'prost = { version = "=0.14.4", default-features = false, features = ["std"] }',
    changedVersion: '=0.14.5',
    extraFeature: 'derive',
  },
  {
    name: 'incrementalmerkletree',
    declaration:
      'incrementalmerkletree = { version = "=0.8.2", default-features = false, features = ["legacy-api"] }',
    changedVersion: '=0.8.3',
    extraFeature: 'std',
  },
];

const tests = [];

function test(name, fn) {
  tests.push({ name, fn });
}

function manifest() {
  return fs.readFileSync(manifestPath, 'utf8');
}

function replaceOnce(source, search, replacement) {
  assert.strictEqual(
    source.split(search).length - 1,
    1,
    `mutation target must occur exactly once: ${JSON.stringify(search)}`
  );
  const updated = source.replace(search, replacement);
  assert.notStrictEqual(updated, source, 'manifest mutation must change the source');
  return updated;
}

function replaceDeclaration(source, declaration, replacement) {
  return replaceOnce(source, `${declaration}\n`, replacement ? `${replacement}\n` : '');
}

function assertManifestRejects(mutated, label) {
  let error;
  try {
    policy.checkWalletBrokerManifest(mutated, {
      requireLibrary: true,
      requireLockfile: false,
    });
  } catch (caught) {
    error = caught;
  }
  assert.ok(error instanceof policy.PolicyError, `${label} did not raise PolicyError`);
  assert.match(
    String(error.message),
    /wallet|Rust|manifest|dependency|feature|pin|authority|duplicate|displaced/i,
    `${label} failed for an unrelated reason`
  );
  assert.doesNotMatch(
    String(error.message),
    /missing target|syntax|cannot find module|import/i,
    `${label} reached an unrelated failure`
  );
}

test('WAL-015 current manifest has each reviewed live declaration exactly once and passes policy', () => {
  const source = manifest();
  for (const dependency of LIVE_DEPENDENCIES) {
    assert.strictEqual(
      source.split(`${dependency.declaration}\n`).length - 1,
      1,
      `${dependency.name} declaration must be exact and unique`
    );
  }
  policy.checkWalletBrokerManifest(source, {
    requireLibrary: true,
    requireLockfile: false,
  });
});

for (const dependency of LIVE_DEPENDENCIES) {
  test(`WAL-015 rejects removing ${dependency.name}`, () => {
    const source = manifest();
    assertManifestRejects(
      replaceDeclaration(source, dependency.declaration, ''),
      `${dependency.name} removal`
    );
  });

  test(`WAL-015 rejects changing the ${dependency.name} version pin`, () => {
    const source = manifest();
    const changed = dependency.declaration.replace(
      /version = "=[^"]+"/,
      `version = "${dependency.changedVersion}"`
    );
    assert.notStrictEqual(changed, dependency.declaration, 'version mutation must change the line');
    assertManifestRejects(
      replaceDeclaration(source, dependency.declaration, changed),
      `${dependency.name} version mutation`
    );
  });

  test(`WAL-015 rejects enabling default features for ${dependency.name}`, () => {
    const source = manifest();
    const changed = dependency.declaration.replace(
      'default-features = false',
      'default-features = true'
    );
    assert.notStrictEqual(changed, dependency.declaration, 'default-feature mutation must change the line');
    assertManifestRejects(
      replaceDeclaration(source, dependency.declaration, changed),
      `${dependency.name} default-feature mutation`
    );
  });

  test(`WAL-015 rejects an extra ${dependency.name} feature`, () => {
    const source = manifest();
    const changed = dependency.declaration.replace(
      '] }',
      `, "${dependency.extraFeature}"] }`
    );
    assert.notStrictEqual(changed, dependency.declaration, 'feature mutation must change the line');
    assertManifestRejects(
      replaceDeclaration(source, dependency.declaration, changed),
      `${dependency.name} feature mutation`
    );
  });

  test(`WAL-015 rejects a duplicate ${dependency.name} declaration`, () => {
    const source = manifest();
    assertManifestRejects(
      replaceDeclaration(
        source,
        dependency.declaration,
        `${dependency.declaration}\n${dependency.declaration}`
      ),
      `${dependency.name} duplicate`
    );
  });

  test(`WAL-015 rejects a displaced ${dependency.name} declaration`, () => {
    const source = manifest();
    const removed = replaceDeclaration(source, dependency.declaration, '');
    const displaced = replaceOnce(
      removed,
      '[dependencies]\n',
      `${dependency.declaration}\n\n[dependencies]\n`
    );
    assertManifestRejects(displaced, `${dependency.name} displacement`);
  });
}

test('WAL-015 rejects removing either reviewed Tonic TLS trust feature', () => {
  const source = manifest();
  const tonic = LIVE_DEPENDENCIES.find((dependency) => dependency.name === 'tonic');
  for (const feature of ['tls-ring', 'tls-webpki-roots']) {
    const changed = tonic.declaration.replace(`, "${feature}"`, '');
    assert.notStrictEqual(changed, tonic.declaration, `${feature} mutation must change the line`);
    assertManifestRejects(
      replaceDeclaration(source, tonic.declaration, changed),
      `tonic ${feature} removal`
    );
  }
});

test('WAL-015 rejects backend sync and Tor feature authority independently', () => {
  const source = manifest();
  const backend = LIVE_DEPENDENCIES.find(
    (dependency) => dependency.name === 'zcash_client_backend'
  );
  for (const feature of ['sync', 'tor']) {
    const changed = backend.declaration.replace('"pczt"]', `"pczt", "${feature}"]`);
    assert.notStrictEqual(changed, backend.declaration, `${feature} mutation must change the line`);
    assertManifestRejects(
      replaceDeclaration(source, backend.declaration, changed),
      `backend ${feature} feature`
    );
  }
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
  if (failed > 0) {
    process.exitCode = 1;
    return;
  }
  process.stdout.write(`BitBook WAL-015 manifest policy tests passed (${tests.length}).\n`);
}

run();
