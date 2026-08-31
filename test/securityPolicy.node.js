'use strict';

const assert = require('assert');
const fs = require('fs');
const path = require('path');

const repoRoot = path.resolve(__dirname, '..');
const policyPath = path.join(repoRoot, 'scripts', 'security-policy.js');
const sbomValidatorPath = path.join(repoRoot, 'scripts', 'validate-sbom.js');
const rustSbomValidatorPath = path.join(repoRoot, 'scripts', 'validate-rust-sbom.js');

const CHECKOUT_SHA = '3d3c42e5aac5ba805825da76410c181273ba90b1';
const CHECKOUT_TAG = 'v7.0.1';
const SETUP_NODE_SHA = '48b55a011bda9f5d6aeb4c2d9c7362e8dae4041e';
const SETUP_NODE_TAG = 'v6.4.0';
const UPLOAD_SHA = '043fb46d1a93c77aae656e7c1c64a875d1fc6a0a';
const UPLOAD_TAG = 'v7.0.1';
const ELECTRONEGATIVITY = '@doyensec/electronegativity@1.10.3';
const CYCLONEDX_NPM = '@cyclonedx/cyclonedx-npm@6.0.1';
const GITLEAKS_VERSION = '8.30.1';
const GITLEAKS_ARCHIVE_URL =
  'https://github.com/gitleaks/gitleaks/releases/download/v8.30.1/gitleaks_8.30.1_linux_x64.tar.gz';
const GITLEAKS_ARCHIVE_SHA256 =
  '551f6fc83ea457d62a0d98237cbad105af8d557003051f41f3e7ca7b3f2470eb';
const GITLEAKS_ARCHIVE_BYTES = 8230402;
const GITLEAKS_SCAN_CMD = 'gitleaks git --redact=100 --no-banner .';
const GITLEAKS_DIR_SCAN_CMD = 'gitleaks dir --redact=100 --no-banner .';
const GITLEAKS_IGNORE_REL = '.gitleaksignore';
const GITLEAKS_RATCHET_OWNER = 'Lead Engineer/Reviewer — Codex';
const GITLEAKS_RATCHET_RATIONALE =
  'eight inherited 2016–2018 upstream fingerprints plus one reviewer-published WAL-004 synthetic-vector assignment fingerprint; current-tree copies are removed, never ignored';
const GITLEAKS_RATCHET_REMOVAL_CONDITION =
  'remove an exact fingerprint only when an authorized history rewrite makes its commit unreachable; current-tree triggers remain relabeled, never ignored';
const GITLEAKS_RATCHET_FINGERPRINTS = [
  '12a493196bb4304750e4ae44484a7fa604b82ce4:tickets/BBD-WAL-004.md:generic-api-key:110',
  '7f6a71d6d5ec94b0d8ed02a23eddd7d1bfeaf802:index.html:generic-api-key:57',
  '988fcc3da2d2b13689fdd98e936df14e2f989709:js/models/order/Case.js:generic-api-key:107',
  'b0637a03e1eb12e4e5d49c9dfba92dcbf51a0d8c:js/utils/feedback.js:generic-api-key:8',
  'bfd12cbe6e1f586af1f728c6d4e1ba68b8d9d103:js/utils/metrics.js:generic-api-key:13',
  'd38fc4819f1aa16f692394c56acc90db5d4f973a:js/views/modals/wallet/ReceiveMoney.js:generic-api-key:65',
  'e30e2ebbe6cc6198ca3c507167d26ff934ef9deb:js/views/modals/wallet/ReceiveMoney.js:generic-api-key:65',
  'f527597842b38bbe25c36cb42d204f16747e7e72:js/start.js:generic-api-key:409',
  'f83f40146c4bd2eb6da9f7fdd7a8eab8fb660b13:js/views/modals/wallet/ReceiveMoney.js:generic-api-key:63',
];
const GITLEAKS_RATCHET_BODY = `${GITLEAKS_RATCHET_FINGERPRINTS.join('\n')}\n`;
const METRICS_REL = 'js/utils/metrics.js';
const FEEDBACK_REL = 'js/utils/feedback.js';
const MAINTAINED_SOCIAL_PATHS = [
  'social-main.js',
  'social/index.html',
  'social/app.js',
  'social/core.js',
];
const NODE_VERSION = '24';
const ELECTRON_VERSION = '44.0.0';
const ROOT_COMPONENT_NAME = 'bitbook-desktop';
const SBOM_RETENTION_DAYS = 14;

const SOCIAL_PATHS = [
  'social-main.js',
  'social/**',
  'test/**',
  'package.json',
  'package-lock.json',
  'scripts/security-policy.js',
  'scripts/validate-sbom.js',
  'scripts/validate-rust-sbom.js',
  'scripts/build-deb.sh',
  'scripts/build-macos.sh',
  'scripts/build-windows.ps1',
  'wallet-broker/**',
  '.github/workflows/social.yml',
];

const SECURITY_PATHS = [
  'social-main.js',
  'social/**',
  'test/**',
  'scripts/security-policy.js',
  'scripts/validate-sbom.js',
  'scripts/validate-rust-sbom.js',
  'package.json',
  'package-lock.json',
  'wallet-broker/**',
  'deny.toml',
  '.github/workflows/**',
  '.gitleaksignore',
  'js/utils/metrics.js',
  'js/utils/feedback.js',
];

const FORBIDDEN_DOC_PATHS = [
  '**',
  '**/*',
  'docs/**',
  '**/*.md',
  '*.md',
  'README.md',
];

const REQUIRED_PATHS = [
  'scripts/security-policy.js',
  'scripts/validate-sbom.js',
  'scripts/validate-rust-sbom.js',
  '.github/workflows/social.yml',
  '.github/workflows/security.yml',
  '.github/workflows/sbom.yml',
  'test/electronSecurity.node.js',
  'test/securityPolicy.node.js',
  '.gitleaksignore',
  'deny.toml',
];

const ELECTRON_TEST_CMD = 'node test/electronSecurity.node.js';
const POLICY_TEST_CMD = 'node test/securityPolicy.node.js';
const POLICY_CHECK_CMD = 'node scripts/security-policy.js';
const AUDIT_CMD = 'npm audit --audit-level=low';
const ELECTRONEGATIVITY_CMD = `npx --yes ${ELECTRONEGATIVITY} -i social-main.js`;
const BUILD_CMD = 'npm run build';
const SOCIAL_TEST_CMD = 'npm run test:social';
const SECURITY_TEST_CMD = 'npm run test:security';

function loadPolicy() {
  assert.ok(fs.existsSync(policyPath), 'required checker scripts/security-policy.js does not exist');
  return require(policyPath);
}

function loadSbomValidator() {
  assert.ok(
    fs.existsSync(sbomValidatorPath),
    'required validator scripts/validate-sbom.js does not exist'
  );
  return require(sbomValidatorPath);
}

function loadRustSbomValidator() {
  assert.ok(
    fs.existsSync(rustSbomValidatorPath),
    'required validator scripts/validate-rust-sbom.js does not exist'
  );
  return require(rustSbomValidatorPath);
}

function loadWorkflows(policy) {
  return {
    social: policy.loadWorkflow(path.join(repoRoot, '.github/workflows/social.yml')),
    security: policy.loadWorkflow(path.join(repoRoot, '.github/workflows/security.yml')),
    sbom: policy.loadWorkflow(path.join(repoRoot, '.github/workflows/sbom.yml')),
  };
}

function assertRejects(fn, needle) {
  let err;
  try {
    fn();
  } catch (caught) {
    err = caught;
  }
  assert.ok(err, 'expected policy rejection, but the mutated workflow was accepted');
  if (needle) {
    const message = String(err.message || err);
    const matched = needle instanceof RegExp ? needle.test(message) : message.includes(needle);
    assert.ok(matched, `rejection ${JSON.stringify(message)} did not match ${needle}`);
  }
}

function replaceOnce(source, search, replacement) {
  assert.ok(source.includes(search), `mutation target missing: ${JSON.stringify(search)}`);
  const updated = source.replace(search, replacement);
  assert.notStrictEqual(updated, source, 'mutation did not change the workflow text');
  return updated;
}

function insertSecurityRun(source, command) {
  return replaceOnce(
    source,
    `      - run: ${POLICY_CHECK_CMD}\n`,
    `      - run: ${POLICY_CHECK_CMD}\n      - run: ${command}\n`
  );
}

function ratchetBytes(lines, options = {}) {
  const newline = options.newline === undefined ? '\n' : options.newline;
  const ending = options.ending === undefined ? '\n' : options.ending;
  const text = `${lines.join(newline)}${ending}${options.trailing || ''}`;
  const body = Buffer.from(text, 'utf8');
  if (!options.bom) {
    return body;
  }
  return Buffer.concat([Buffer.from([0xef, 0xbb, 0xbf]), body]);
}

function exportedFunctionSource(source, name) {
  const start = source.indexOf(`export function ${name}(`);
  assert.ok(start >= 0, `missing export function ${name}`);
  const brace = source.indexOf('{', start);
  assert.ok(brace > start, `missing body for export function ${name}`);
  let depth = 0;
  for (let index = brace; index < source.length; index += 1) {
    if (source[index] === '{') {
      depth += 1;
    } else if (source[index] === '}') {
      depth -= 1;
      if (depth === 0) {
        return source.slice(start, index + 1);
      }
    }
  }
  assert.fail(`unterminated export function ${name}`);
  return '';
}

const tests = [];

function test(name, fn) {
  tests.push({ name, fn });
}

test('required policy, workflow, and validator sources exist', () => {
  const missing = REQUIRED_PATHS.filter((rel) => !fs.existsSync(path.join(repoRoot, rel)));
  assert.deepStrictEqual(missing, [], `required checker/workflows do not yet exist: ${missing.join(', ')}`);
});

test('checker constants match the ticketed Action and tool pins', () => {
  const policy = loadPolicy();
  assert.strictEqual(policy.CHECKOUT_SHA, CHECKOUT_SHA);
  assert.strictEqual(policy.CHECKOUT_TAG, CHECKOUT_TAG);
  assert.strictEqual(policy.SETUP_NODE_SHA, SETUP_NODE_SHA);
  assert.strictEqual(policy.SETUP_NODE_TAG, SETUP_NODE_TAG);
  assert.strictEqual(policy.UPLOAD_ARTIFACT_SHA, UPLOAD_SHA);
  assert.strictEqual(policy.UPLOAD_ARTIFACT_TAG, UPLOAD_TAG);
  assert.strictEqual(policy.CYCLONEDX_NPM, CYCLONEDX_NPM);
  assert.strictEqual(policy.GITLEAKS_VERSION, GITLEAKS_VERSION);
  assert.strictEqual(policy.GITLEAKS_ARCHIVE_URL, GITLEAKS_ARCHIVE_URL);
  assert.strictEqual(policy.GITLEAKS_ARCHIVE_SHA256, GITLEAKS_ARCHIVE_SHA256);
  assert.strictEqual(policy.GITLEAKS_ARCHIVE_BYTES, GITLEAKS_ARCHIVE_BYTES);
  assert.strictEqual(policy.GITLEAKS_SCAN_CMD, GITLEAKS_SCAN_CMD);
  assert.strictEqual(policy.GITLEAKS_DIR_SCAN_CMD, GITLEAKS_DIR_SCAN_CMD);
  assert.strictEqual(policy.GITLEAKS_IGNORE_REL, GITLEAKS_IGNORE_REL);
  assert.deepStrictEqual(policy.GITLEAKS_RATCHET_FINGERPRINTS, GITLEAKS_RATCHET_FINGERPRINTS);
  assert.strictEqual(policy.GITLEAKS_RATCHET_BODY, GITLEAKS_RATCHET_BODY);
  assert.strictEqual(policy.GITLEAKS_RATCHET_OWNER, GITLEAKS_RATCHET_OWNER);
  assert.strictEqual(policy.GITLEAKS_RATCHET_RATIONALE, GITLEAKS_RATCHET_RATIONALE);
  assert.strictEqual(policy.GITLEAKS_RATCHET_REMOVAL_CONDITION, GITLEAKS_RATCHET_REMOVAL_CONDITION);
  assert.strictEqual(policy.NODE_VERSION, NODE_VERSION);
  assert.strictEqual(policy.ELECTRON_VERSION, ELECTRON_VERSION);
  assert.strictEqual(policy.SBOM_RETENTION_DAYS, SBOM_RETENTION_DAYS);
  assert.deepStrictEqual(policy.SOCIAL_PATHS, SOCIAL_PATHS);
  assert.deepStrictEqual(policy.SECURITY_PATHS, SECURITY_PATHS);
});

test('YAML parser keeps on: as a mapping key and preserves block scalars', () => {
  const policy = loadPolicy();
  const data = policy.parseYaml(
    [
      'on:',
      '  pull_request:',
      '    paths:',
      '      - "social-main.js"',
      '  workflow_dispatch:',
      'jobs:',
      '  scan:',
      '    steps:',
      '      - run: |',
      '          echo one',
      '          echo two',
    ].join('\n')
  );
  assert.ok(Object.prototype.hasOwnProperty.call(data, 'on'));
  assert.notStrictEqual(typeof data.on, 'boolean');
  assert.deepStrictEqual(data.on.pull_request.paths, ['social-main.js']);
  assert.ok(Object.prototype.hasOwnProperty.call(data.on, 'workflow_dispatch'));
  assert.ok(data.jobs.scan.steps[0].run.includes('echo one'));
  assert.ok(data.jobs.scan.steps[0].run.includes('echo two'));
});

test('committed workflows satisfy the fail-closed checker', () => {
  const policy = loadPolicy();
  policy.checkRepository(repoRoot);
});

test('mutable Action tag is rejected', () => {
  const policy = loadPolicy();
  const workflows = loadWorkflows(policy);
  const mutated = replaceOnce(workflows.social.text, CHECKOUT_SHA, CHECKOUT_TAG);
  assert.ok(mutated.includes(`actions/checkout@${CHECKOUT_TAG}`));
  assertRejects(() => policy.checkSocialWorkflow(mutated), /not pinned|mutable|40-character/i);
});

test('wrong Action SHA is rejected', () => {
  const policy = loadPolicy();
  const workflows = loadWorkflows(policy);
  const mutated = replaceOnce(
    workflows.security.text,
    SETUP_NODE_SHA,
    'aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa'
  );
  assertRejects(() => policy.checkSecurityWorkflow(mutated), /must be pinned|wrong/i);
});

test('unpinned CycloneDX tool is rejected', () => {
  const policy = loadPolicy();
  const workflows = loadWorkflows(policy);
  const cyclonedxMutated = replaceOnce(
    workflows.sbom.text,
    CYCLONEDX_NPM,
    '@cyclonedx/cyclonedx-npm@latest'
  );
  assertRejects(() => policy.checkSbomWorkflow(cyclonedxMutated), /pin|latest|cyclonedx/i);
});

test('missing contents: read permissions are rejected', () => {
  const policy = loadPolicy();
  const workflows = loadWorkflows(policy);
  const mutated = replaceOnce(workflows.security.text, 'contents: read', 'contents: write');
  assertRejects(() => policy.checkSecurityWorkflow(mutated), /contents: read|permissions/i);
});

test('job-level write permissions are rejected', () => {
  const policy = loadPolicy();
  const workflows = loadWorkflows(policy);
  const mutated = replaceOnce(
    workflows.social.text,
    '    runs-on: ubuntu-latest\n    steps:\n',
    '    runs-on: ubuntu-latest\n    permissions:\n      contents: write\n    steps:\n'
  );
  assertRejects(() => policy.checkSocialWorkflow(mutated), /permissions/i);
});

test('routine native packaging in the check job is rejected', () => {
  const policy = loadPolicy();
  const workflows = loadWorkflows(policy);
  const mutated = replaceOnce(
    workflows.social.text,
    `      - run: ${SECURITY_TEST_CMD}\n`,
    `      - run: ${SECURITY_TEST_CMD}\n      - run: npm run package:linux\n`
  );
  assertRejects(() => policy.checkSocialWorkflow(mutated), /package|native|binary/i);
});

test('routine artifact upload in the check job is rejected', () => {
  const policy = loadPolicy();
  const workflows = loadWorkflows(policy);
  const mutated = replaceOnce(
    workflows.social.text,
    `      - run: ${SECURITY_TEST_CMD}\n`,
    [
      `      - run: ${SECURITY_TEST_CMD}`,
      '      - uses: actions/upload-artifact@' + UPLOAD_SHA + ' # ' + UPLOAD_TAG,
      '        with:',
      '          name: leaked-binary',
      '          path: dist/*.deb',
      '',
    ].join('\n')
  );
  assertRejects(() => policy.checkSocialWorkflow(mutated), /upload|artifact|package|binary/i);
});

test('package jobs without manual-only guards are rejected', () => {
  const policy = loadPolicy();
  const workflows = loadWorkflows(policy);
  const mutated = replaceOnce(
    workflows.social.text,
    "    if: github.event_name == 'workflow_dispatch'\n    needs: check\n    runs-on: ubuntu-latest\n",
    '    needs: check\n    runs-on: ubuntu-latest\n'
  );
  assertRejects(() => policy.checkSocialWorkflow(mutated), /workflow_dispatch|manual|package/i);
});

test('security workflow push trigger is rejected', () => {
  const policy = loadPolicy();
  const workflows = loadWorkflows(policy);
  const mutated = replaceOnce(workflows.security.text, 'on:\n', 'on:\n  push:\n');
  assertRejects(() => policy.checkSecurityWorkflow(mutated), /push/i);
});

test('SBOM workflow push or pull_request trigger is rejected', () => {
  const policy = loadPolicy();
  const workflows = loadWorkflows(policy);
  const pushMutated = replaceOnce(workflows.sbom.text, 'on:\n', 'on:\n  push:\n');
  assertRejects(() => policy.checkSbomWorkflow(pushMutated), /push|workflow_dispatch/i);
  const prMutated = replaceOnce(workflows.sbom.text, 'on:\n', 'on:\n  pull_request:\n');
  assertRejects(() => policy.checkSbomWorkflow(prMutated), /pull_request|workflow_dispatch/i);
});

test('missing relevant path filters are rejected', () => {
  const policy = loadPolicy();
  const workflows = loadWorkflows(policy);
  const mutated = replaceOnce(
    workflows.social.text,
    '      - "social-main.js"\n',
    ''
  );
  assertRejects(() => policy.checkSocialWorkflow(mutated), /path/i);
});

test('omitting the Windows native build script from routine path filters is rejected', () => {
  const policy = loadPolicy();
  const workflows = loadWorkflows(policy);
  const mutated = replaceOnce(
    workflows.social.text,
    '      - "scripts/build-windows.ps1"\n',
    ''
  );
  assert.ok(SOCIAL_PATHS.includes('scripts/build-windows.ps1'));
  assert.ok(SOCIAL_PATHS.includes('scripts/build-deb.sh'));
  assert.ok(SOCIAL_PATHS.includes('scripts/build-macos.sh'));
  assertRejects(() => policy.checkSocialWorkflow(mutated), /path|build-windows/i);
});

test('documentation-only CI path is rejected', () => {
  const policy = loadPolicy();
  const workflows = loadWorkflows(policy);
  const mutated = replaceOnce(
    workflows.social.text,
    '      - "social-main.js"\n',
    '      - "social-main.js"\n      - "docs/**"\n'
  );
  assert.ok(FORBIDDEN_DOC_PATHS.includes('docs/**'));
  assertRejects(() => policy.checkSocialWorkflow(mutated), /docs|documentation|path/i);
});

test('missing npm audit is rejected', () => {
  const policy = loadPolicy();
  const workflows = loadWorkflows(policy);
  const mutated = replaceOnce(workflows.security.text, `      - run: ${AUDIT_CMD}\n`, '');
  assertRejects(() => policy.checkSecurityWorkflow(mutated), /npm audit/i);
});

test('missing maintained Electron test is rejected', () => {
  const policy = loadPolicy();
  const workflows = loadWorkflows(policy);
  const mutated = replaceOnce(
    workflows.security.text,
    `      - run: ${ELECTRON_TEST_CMD}\n`,
    ''
  );
  assertRejects(() => policy.checkSecurityWorkflow(mutated), /electron/i);
});

test('missing policy test is rejected', () => {
  const policy = loadPolicy();
  const workflows = loadWorkflows(policy);
  const mutated = replaceOnce(
    workflows.security.text,
    `      - run: ${POLICY_TEST_CMD}\n`,
    ''
  );
  assertRejects(() => policy.checkSecurityWorkflow(mutated), /policy/i);
});

test('pinned Electronegativity or ElectroNG reintroduction is rejected', () => {
  const policy = loadPolicy();
  const workflows = loadWorkflows(policy);
  const electronegativityMutated = insertSecurityRun(workflows.security.text, ELECTRONEGATIVITY_CMD);
  assertRejects(
    () => policy.checkSecurityWorkflow(electronegativityMutated),
    /electronegativity|electro-?ng/i
  );
  const electrongMutated = insertSecurityRun(workflows.security.text, 'npx --yes @doyensec/electrong');
  assertRejects(
    () => policy.checkSecurityWorkflow(electrongMutated),
    /electronegativity|electro-?ng/i
  );
});

test('obsolete Electron-SAST input of social-main.js, repository root, main.js, or js/ is rejected', () => {
  const policy = loadPolicy();
  const workflows = loadWorkflows(policy);
  const socialMainMutated = insertSecurityRun(workflows.security.text, 'sast -i social-main.js');
  assertRejects(
    () => policy.checkSecurityWorkflow(socialMainMutated),
    /social-main\.js|inherited|obsolete/i
  );
  const rootMutated = insertSecurityRun(workflows.security.text, 'sast -i .');
  assertRejects(() => policy.checkSecurityWorkflow(rootMutated), /inherited|obsolete/i);
  const mainMutated = insertSecurityRun(workflows.security.text, 'sast -i main.js');
  assertRejects(() => policy.checkSecurityWorkflow(mainMutated), /main\.js|inherited|obsolete/i);
  const jsMutated = insertSecurityRun(workflows.security.text, 'sast -i js/');
  assertRejects(() => policy.checkSecurityWorkflow(jsMutated), /js\/|inherited|obsolete/i);
});

test('obsolete Electron-SAST SARIF, CSV, exclusion, and eng-disable forms are rejected', () => {
  const policy = loadPolicy();
  const workflows = loadWorkflows(policy);
  const sarifMutated = insertSecurityRun(
    workflows.security.text,
    `${ELECTRONEGATIVITY_CMD} --sarif findings.sarif`
  );
  assertRejects(() => policy.checkSecurityWorkflow(sarifMutated), /sarif/i);
  const csvMutated = insertSecurityRun(
    workflows.security.text,
    `${ELECTRONEGATIVITY_CMD} --output findings.csv`
  );
  assertRejects(() => policy.checkSecurityWorkflow(csvMutated), /csv/i);
  const excludeXMutated = insertSecurityRun(
    workflows.security.text,
    `${ELECTRONEGATIVITY_CMD} -x CSPGlobalCheck`
  );
  assertRejects(() => policy.checkSecurityWorkflow(excludeXMutated), /suppress|exclude|ignore|-x/i);
  const excludeChecksMutated = insertSecurityRun(
    workflows.security.text,
    `${ELECTRONEGATIVITY_CMD} --exclude-checks AUXCLICK_JS_CHECK`
  );
  assertRejects(
    () => policy.checkSecurityWorkflow(excludeChecksMutated),
    /exclude-checks|exclude|suppress/i
  );
  const engDisableMutated = insertSecurityRun(
    workflows.security.text,
    'echo eng-disable AUXCLICK_JS_CHECK'
  );
  assertRejects(() => policy.checkSecurityWorkflow(engDisableMutated), /eng-disable/i);
});

test('missing complete-history Gitleaks checkout is rejected', () => {
  const policy = loadPolicy();
  const workflows = loadWorkflows(policy);
  const mutated = replaceOnce(workflows.security.text, 'fetch-depth: 0', 'fetch-depth: 1');
  assertRejects(() => policy.checkSecurityWorkflow(mutated), /fetch-depth|complete history|gitleaks/i);
});

test('missing pinned Gitleaks install or complete-history scan is rejected', () => {
  const policy = loadPolicy();
  const workflows = loadWorkflows(policy);
  const missingScan = replaceOnce(
    workflows.security.text,
    `      - run: ${GITLEAKS_SCAN_CMD}\n`,
    ''
  );
  assertRejects(() => policy.checkSecurityWorkflow(missingScan), /gitleaks/i);
  const missingInstall = replaceOnce(
    workflows.security.text,
    `          curl --fail --silent --show-error --location --output "\${archive}" "${GITLEAKS_ARCHIVE_URL}"\n`,
    ''
  );
  assertRejects(() => policy.checkSecurityWorkflow(missingInstall), /gitleaks/i);
});

test('wrong Gitleaks archive URL is rejected', () => {
  const policy = loadPolicy();
  const workflows = loadWorkflows(policy);
  const mutated = replaceOnce(
    workflows.security.text,
    GITLEAKS_ARCHIVE_URL,
    'https://example.invalid/gitleaks_8.30.1_linux_x64.tar.gz'
  );
  assertRejects(() => policy.checkSecurityWorkflow(mutated), /url|archive|gitleaks/i);
});

test('wrong Gitleaks archive SHA-256 is rejected', () => {
  const policy = loadPolicy();
  const workflows = loadWorkflows(policy);
  const mutated = replaceOnce(
    workflows.security.text,
    GITLEAKS_ARCHIVE_SHA256,
    'ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff'
  );
  assertRejects(() => policy.checkSecurityWorkflow(mutated), /sha-256|hash|archive|gitleaks/i);
});

test('wrong Gitleaks archive size is rejected', () => {
  const policy = loadPolicy();
  const workflows = loadWorkflows(policy);
  const mutated = replaceOnce(
    workflows.security.text,
    String(GITLEAKS_ARCHIVE_BYTES),
    '8230403'
  );
  assertRejects(() => policy.checkSecurityWorkflow(mutated), /8230402|size|archive|gitleaks/i);
});

test('wrong Gitleaks version is rejected', () => {
  const policy = loadPolicy();
  const workflows = loadWorkflows(policy);
  const mutated = replaceOnce(workflows.security.text, 'v8.30.1', 'v8.30.0');
  assertRejects(() => policy.checkSecurityWorkflow(mutated), /8\.30\.1|version|url|gitleaks/i);
});

test('mutable Gitleaks release name is rejected', () => {
  const policy = loadPolicy();
  const workflows = loadWorkflows(policy);
  const mutated = replaceOnce(
    workflows.security.text,
    GITLEAKS_ARCHIVE_URL,
    'https://github.com/gitleaks/gitleaks/releases/latest/download/gitleaks_8.30.1_linux_x64.tar.gz'
  );
  assertRejects(() => policy.checkSecurityWorkflow(mutated), /mutable|latest|url|gitleaks/i);
});

test('Gitleaks extraction other than gitleaks under RUNNER_TEMP is rejected', () => {
  const policy = loadPolicy();
  const workflows = loadWorkflows(policy);
  const extraMember = replaceOnce(
    workflows.security.text,
    'tar --extract --gzip --file "${archive}" --directory "${RUNNER_TEMP}" gitleaks',
    'tar --extract --gzip --file "${archive}" --directory "${RUNNER_TEMP}" gitleaks README.md'
  );
  assertRejects(() => policy.checkSecurityWorkflow(extraMember), /extract|RUNNER_TEMP|gitleaks/i);
  const wrongDir = replaceOnce(
    workflows.security.text,
    'tar --extract --gzip --file "${archive}" --directory "${RUNNER_TEMP}" gitleaks',
    'tar --extract --gzip --file "${archive}" --directory "/tmp" gitleaks'
  );
  assertRejects(() => policy.checkSecurityWorkflow(wrongDir), /extract|RUNNER_TEMP|gitleaks/i);
});

test('Gitleaks install cleanup or deletion is rejected', () => {
  const policy = loadPolicy();
  const workflows = loadWorkflows(policy);
  const mutated = replaceOnce(
    workflows.security.text,
    'echo "${RUNNER_TEMP}" >> "${GITHUB_PATH}"',
    'echo "${RUNNER_TEMP}" >> "${GITHUB_PATH}"\n          rm -f "${archive}"'
  );
  assertRejects(() => policy.checkSecurityWorkflow(mutated), /clean|delete|rm|gitleaks/i);
});

test('Gitleaks scan must immediately follow install with the exact root command', () => {
  const policy = loadPolicy();
  const workflows = loadWorkflows(policy);
  const delayed = replaceOnce(
    workflows.security.text,
    `      - run: ${GITLEAKS_SCAN_CMD}\n`,
    `      - run: echo between-install-and-scan\n      - run: ${GITLEAKS_SCAN_CMD}\n`
  );
  assertRejects(() => policy.checkSecurityWorkflow(delayed), /immediately|follow|gitleaks/i);
  const altered = replaceOnce(
    workflows.security.text,
    GITLEAKS_SCAN_CMD,
    'gitleaks detect --redact=100 --no-banner .'
  );
  assertRejects(() => policy.checkSecurityWorkflow(altered), /gitleaks git --redact=100 --no-banner \.|exactly|scan/i);
});

test('Gitleaks range or log opts are rejected', () => {
  const policy = loadPolicy();
  const workflows = loadWorkflows(policy);
  const rangeMutated = replaceOnce(
    workflows.security.text,
    GITLEAKS_SCAN_CMD,
    `${GITLEAKS_SCAN_CMD} --log-opts=HEAD~1..HEAD`
  );
  assertRejects(() => policy.checkSecurityWorkflow(rangeMutated), /log-opts|range|log opts|gitleaks/i);
  const dirRangeMutated = replaceOnce(
    workflows.security.text,
    GITLEAKS_DIR_SCAN_CMD,
    `${GITLEAKS_DIR_SCAN_CMD} --log-opts=HEAD~1..HEAD`
  );
  assertRejects(() => policy.checkSecurityWorkflow(dirRangeMutated), /log-opts|range|log opts|gitleaks/i);
});

test('non-blocking scanner behavior is rejected', () => {
  const policy = loadPolicy();
  const workflows = loadWorkflows(policy);
  const mutated = replaceOnce(
    workflows.security.text,
    `      - run: ${AUDIT_CMD}\n`,
    `      - run: ${AUDIT_CMD}\n        continue-on-error: true\n`
  );
  assertRejects(() => policy.checkSecurityWorkflow(mutated), /continue-on-error|non-blocking/i);
});

test('ignore, baseline, and suppression flags are rejected', () => {
  const policy = loadPolicy();
  const workflows = loadWorkflows(policy);
  assert.ok(workflows.security.text.includes(`      - "${GITLEAKS_IGNORE_REL}"\n`));
  const baselineMutated = replaceOnce(
    workflows.security.text,
    GITLEAKS_SCAN_CMD,
    `${GITLEAKS_SCAN_CMD} --baseline-path baseline.json`
  );
  assertRejects(() => policy.checkSecurityWorkflow(baselineMutated), /config|baseline|ignore|suppress/i);
  const configMutated = replaceOnce(
    workflows.security.text,
    GITLEAKS_SCAN_CMD,
    `${GITLEAKS_SCAN_CMD} --config .gitleaks.toml`
  );
  assertRejects(() => policy.checkSecurityWorkflow(configMutated), /config|baseline|ignore|suppress/i);
  const gitleaksIgnoreMutated = replaceOnce(
    workflows.security.text,
    GITLEAKS_SCAN_CMD,
    `${GITLEAKS_SCAN_CMD} --gitleaks-ignore-path .gitleaksignore`
  );
  assertRejects(() => policy.checkSecurityWorkflow(gitleaksIgnoreMutated), /config|baseline|ignore|suppress/i);
  const dirBaselineMutated = replaceOnce(
    workflows.security.text,
    GITLEAKS_DIR_SCAN_CMD,
    `${GITLEAKS_DIR_SCAN_CMD} --baseline-path baseline.json`
  );
  assertRejects(() => policy.checkSecurityWorkflow(dirBaselineMutated), /config|baseline|ignore|suppress/i);
  const dirConfigMutated = replaceOnce(
    workflows.security.text,
    GITLEAKS_DIR_SCAN_CMD,
    `${GITLEAKS_DIR_SCAN_CMD} --config .gitleaks.toml`
  );
  assertRejects(() => policy.checkSecurityWorkflow(dirConfigMutated), /config|baseline|ignore|suppress/i);
  const dirIgnoreMutated = replaceOnce(
    workflows.security.text,
    GITLEAKS_DIR_SCAN_CMD,
    `${GITLEAKS_DIR_SCAN_CMD} --gitleaks-ignore-path .gitleaksignore`
  );
  assertRejects(() => policy.checkSecurityWorkflow(dirIgnoreMutated), /config|baseline|ignore|suppress/i);
});

test('Gitleaks report path, artifact, or summary is rejected', () => {
  const policy = loadPolicy();
  const workflows = loadWorkflows(policy);
  const reportMutated = replaceOnce(
    workflows.security.text,
    GITLEAKS_SCAN_CMD,
    `${GITLEAKS_SCAN_CMD} --report-path gitleaks.json`
  );
  assertRejects(() => policy.checkSecurityWorkflow(reportMutated), /report|upload|artifact|summary/i);
  const formatMutated = replaceOnce(
    workflows.security.text,
    GITLEAKS_SCAN_CMD,
    `${GITLEAKS_SCAN_CMD} --report-format json`
  );
  assertRejects(() => policy.checkSecurityWorkflow(formatMutated), /report|upload|artifact|summary/i);
  const dirReportMutated = replaceOnce(
    workflows.security.text,
    GITLEAKS_DIR_SCAN_CMD,
    `${GITLEAKS_DIR_SCAN_CMD} --report-path gitleaks.json`
  );
  assertRejects(() => policy.checkSecurityWorkflow(dirReportMutated), /report|upload|artifact|summary/i);
  const dirFormatMutated = replaceOnce(
    workflows.security.text,
    GITLEAKS_DIR_SCAN_CMD,
    `${GITLEAKS_DIR_SCAN_CMD} --report-format json`
  );
  assertRejects(() => policy.checkSecurityWorkflow(dirFormatMutated), /report|upload|artifact|summary/i);
  const summaryMutated = replaceOnce(
    workflows.security.text,
    `      - run: ${GITLEAKS_SCAN_CMD}\n`,
    `      - run: ${GITLEAKS_SCAN_CMD}\n        env:\n          GITHUB_STEP_SUMMARY: gitleaks.md\n`
  );
  assertRejects(
    () => policy.checkSecurityWorkflow(summaryMutated),
    /report|upload|artifact|summar(?:y|ies)/i
  );
  const artifactMutated = replaceOnce(
    workflows.security.text,
    `      - run: ${GITLEAKS_SCAN_CMD}\n`,
    [
      `      - run: ${GITLEAKS_SCAN_CMD}`,
      '        env:',
      '          GITLEAKS_ENABLE_UPLOAD_ARTIFACT: "true"',
      '',
    ].join('\n')
  );
  assertRejects(() => policy.checkSecurityWorkflow(artifactMutated), /report|upload|artifact|summary/i);
});

test('Gitleaks Action, token, or comment environment is rejected', () => {
  const policy = loadPolicy();
  const workflows = loadWorkflows(policy);
  const actionMutated = replaceOnce(
    workflows.security.text,
    `      - run: ${GITLEAKS_SCAN_CMD}\n`,
    [
      `      - run: ${GITLEAKS_SCAN_CMD}`,
      '      - uses: gitleaks/gitleaks-action@e0c47f4f8be36e29cdc102c57e68cb5cbf0e8d1e # v3.0.0',
      '',
    ].join('\n')
  );
  assertRejects(() => policy.checkSecurityWorkflow(actionMutated), /gitleaks|unapproved|action/i);
  const tokenMutated = replaceOnce(
    workflows.security.text,
    `      - run: ${GITLEAKS_SCAN_CMD}\n`,
    [
      `      - run: ${GITLEAKS_SCAN_CMD}`,
      '        env:',
      '          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}',
      '',
    ].join('\n')
  );
  assertRejects(() => policy.checkSecurityWorkflow(tokenMutated), /token|gitleaks|GITHUB_TOKEN/i);
  const commentMutated = replaceOnce(
    workflows.security.text,
    `      - run: ${GITLEAKS_SCAN_CMD}\n`,
    [
      `      - run: ${GITLEAKS_SCAN_CMD}`,
      '        env:',
      '          GITLEAKS_ENABLE_COMMENTS: "true"',
      '',
    ].join('\n')
  );
  assertRejects(() => policy.checkSecurityWorkflow(commentMutated), /comment|token|gitleaks/i);
  const versionEnvMutated = replaceOnce(
    workflows.security.text,
    `      - run: ${GITLEAKS_SCAN_CMD}\n`,
    [
      `      - run: ${GITLEAKS_SCAN_CMD}`,
      '        env:',
      `          GITLEAKS_VERSION: "${GITLEAKS_VERSION}"`,
      '',
    ].join('\n')
  );
  assertRejects(() => policy.checkSecurityWorkflow(versionEnvMutated), /version environment|token|comment|gitleaks/i);
});

test('altered scanner exit behavior is rejected', () => {
  const policy = loadPolicy();
  const workflows = loadWorkflows(policy);
  const mutated = replaceOnce(
    workflows.security.text,
    `      - run: ${AUDIT_CMD}\n`,
    `      - run: ${AUDIT_CMD} || true\n`
  );
  assertRejects(() => policy.checkSecurityWorkflow(mutated), /\|\| true|exit|non-blocking/i);
});

test('altered Gitleaks exit behavior is rejected', () => {
  const policy = loadPolicy();
  const workflows = loadWorkflows(policy);
  const orTrue = replaceOnce(
    workflows.security.text,
    GITLEAKS_SCAN_CMD,
    `${GITLEAKS_SCAN_CMD} || true`
  );
  assertRejects(() => policy.checkSecurityWorkflow(orTrue), /\|\| true|exit|non-blocking|gitleaks/i);
  const exitCode = replaceOnce(
    workflows.security.text,
    GITLEAKS_SCAN_CMD,
    `${GITLEAKS_SCAN_CMD} --exit-code 0`
  );
  assertRejects(() => policy.checkSecurityWorkflow(exitCode), /exit|non-blocking|gitleaks/i);
  const dirOrTrue = replaceOnce(
    workflows.security.text,
    GITLEAKS_DIR_SCAN_CMD,
    `${GITLEAKS_DIR_SCAN_CMD} || true`
  );
  assertRejects(() => policy.checkSecurityWorkflow(dirOrTrue), /\|\| true|exit|non-blocking|gitleaks/i);
  const dirExitCode = replaceOnce(
    workflows.security.text,
    GITLEAKS_DIR_SCAN_CMD,
    `${GITLEAKS_DIR_SCAN_CMD} --exit-code 0`
  );
  assertRejects(() => policy.checkSecurityWorkflow(dirExitCode), /exit|non-blocking|gitleaks/i);
});

test('SBOM output other than one validated CycloneDX JSON artifact is rejected', () => {
  const policy = loadPolicy();
  const workflows = loadWorkflows(policy);
  const xmlMutated = replaceOnce(
    workflows.sbom.text,
    '--output-format JSON',
    '--output-format XML'
  );
  assertRejects(() => policy.checkSbomWorkflow(xmlMutated), /JSON|CycloneDX|format/i);
  const pathMutated = replaceOnce(
    workflows.sbom.text,
    'path: ${{ runner.temp }}/bitbook-desktop.cdx.json',
    'path: ${{ runner.temp }}/bitbook-desktop'
  );
  assertRejects(() => policy.checkSbomWorkflow(pathMutated), /cdx\.json|JSON|binary/i);
  const retentionMutated = replaceOnce(
    workflows.sbom.text,
    'retention-days: 14',
    'retention-days: 90'
  );
  assertRejects(() => policy.checkSbomWorkflow(retentionMutated), /14|retention/i);
});

test('SBOM workflow npm ci, audit, generation, and validation stay required', () => {
  const policy = loadPolicy();
  const workflows = loadWorkflows(policy);
  assertRejects(
    () => policy.checkSbomWorkflow(replaceOnce(workflows.sbom.text, '      - run: npm ci\n', '')),
    /npm ci/i
  );
  assertRejects(
    () => policy.checkSbomWorkflow(replaceOnce(workflows.sbom.text, `      - run: ${AUDIT_CMD}\n`, '')),
    /npm audit/i
  );
  assertRejects(
    () => policy.checkSbomWorkflow(replaceOnce(workflows.sbom.text, 'node scripts/validate-sbom.js', 'echo skip-validate')),
    /validate-sbom/i
  );
});

test('SBOM workflow must not package a native application binary', () => {
  const policy = loadPolicy();
  const workflows = loadWorkflows(policy);
  const mutated = replaceOnce(
    workflows.sbom.text,
    '      - run: npm ci\n',
    '      - run: npm ci\n      - run: npm run package:linux\n'
  );
  assertRejects(() => policy.checkSbomWorkflow(mutated), /package|binary|native/i);
});

test('security workflow does not upload artifacts or package binaries', () => {
  const policy = loadPolicy();
  const workflows = loadWorkflows(policy);
  const mutated = replaceOnce(
    workflows.security.text,
    `      - run: ${POLICY_CHECK_CMD}\n`,
    `      - run: ${POLICY_CHECK_CMD}\n      - run: npm run package:windows\n`
  );
  assertRejects(() => policy.checkSecurityWorkflow(mutated), /package|binary|native/i);
});

test('CycloneDX validator accepts a bitbook-desktop JSON document', () => {
  const validator = loadSbomValidator();
  validator.validateCycloneDxDocument({
    bomFormat: 'CycloneDX',
    specVersion: '1.6',
    metadata: {
      component: {
        name: ROOT_COMPONENT_NAME,
        purl: 'pkg:npm/bitbook-desktop@0.1.0',
      },
    },
    components: [{ name: 'electron', version: ELECTRON_VERSION }],
    dependencies: [{ ref: ROOT_COMPONENT_NAME, dependsOn: ['electron'] }],
  });
});

test('CycloneDX validator rejects SPDX, empty, and wrong-root documents', () => {
  const validator = loadSbomValidator();
  const valid = {
    bomFormat: 'CycloneDX',
    specVersion: '1.6',
    metadata: {
      component: { name: ROOT_COMPONENT_NAME },
    },
    components: [{ name: 'electron' }],
    dependencies: [{ ref: ROOT_COMPONENT_NAME }],
  };
  assertRejects(() => validator.validateCycloneDxDocument(['CycloneDX']), /JSON object|not a JSON object/i);
  assertRejects(
    () => validator.validateCycloneDxDocument(Object.assign({}, valid, { bomFormat: 'SPDX' })),
    /CycloneDX/i
  );
  const wrongRoot = JSON.parse(JSON.stringify(valid));
  wrongRoot.metadata.component.name = 'openbazaar-desktop';
  assertRejects(() => validator.validateCycloneDxDocument(wrongRoot), /bitbook-desktop/i);
  const emptyComponents = JSON.parse(JSON.stringify(valid));
  emptyComponents.components = [];
  assertRejects(() => validator.validateCycloneDxDocument(emptyComponents), /components/i);
  const emptyDeps = JSON.parse(JSON.stringify(valid));
  emptyDeps.dependencies = [];
  assertRejects(() => validator.validateCycloneDxDocument(emptyDeps), /dependencies/i);
  assertRejects(
    () => validator.validateCycloneDxFile(path.join(repoRoot, 'does-not-exist.cdx.json')),
    /unable to read|not JSON|missing/i
  );
});

test('routine social check keeps offline syntax and Node tests only', () => {
  const policy = loadPolicy();
  const workflows = loadWorkflows(policy);
  const commands = [];
  for (const [, job, step] of policy.iterSteps(workflows.social.data)) {
    if (job.if) {
      continue;
    }
    commands.push(...policy.stepRunLines(step));
  }
  assert.ok(commands.includes(BUILD_CMD));
  assert.ok(commands.includes(SOCIAL_TEST_CMD));
  assert.ok(commands.includes(SECURITY_TEST_CMD));
  assert.ok(!commands.some((line) => /package:|npm ci|electronegativity|cyclonedx|gitleaks/.test(line)));
});

test('strict nine-line reviewed Gitleaks ratchet bytes and content are enforced', () => {
  const policy = loadPolicy();
  const lexical = [...GITLEAKS_RATCHET_FINGERPRINTS].sort();
  assert.deepStrictEqual(GITLEAKS_RATCHET_FINGERPRINTS, lexical);
  assert.strictEqual(GITLEAKS_RATCHET_FINGERPRINTS.length, 9);
  assert.strictEqual(policy.GITLEAKS_RATCHET_OWNER, GITLEAKS_RATCHET_OWNER);
  assert.strictEqual(policy.GITLEAKS_RATCHET_RATIONALE, GITLEAKS_RATCHET_RATIONALE);
  assert.strictEqual(policy.GITLEAKS_RATCHET_REMOVAL_CONDITION, GITLEAKS_RATCHET_REMOVAL_CONDITION);
  const ignorePath = path.join(repoRoot, GITLEAKS_IGNORE_REL);
  assert.ok(fs.existsSync(ignorePath), 'committed .gitleaksignore is missing');
  const committed = fs.readFileSync(ignorePath);
  assert.deepStrictEqual(committed, Buffer.from(GITLEAKS_RATCHET_BODY, 'utf8'));
  policy.checkGitleaksRatchetBytes(committed);
  policy.checkRepository(repoRoot);

  const ticketText = fs.readFileSync(path.join(repoRoot, 'tickets', 'BBD-WAL-004.md'), 'utf8');
  const vectorHex = '142e48008e3e99568fbbdb4c4534bc67f9666fe4853e6b57c1517be00b24f320';
  assert.ok(ticketText.includes(`expand = ${vectorHex}`));
  assert.ok(!ticketText.includes(`${'key'}    = ${vectorHex}`));

  const missing = GITLEAKS_RATCHET_FINGERPRINTS.slice(0, 8);
  assertRejects(() => policy.checkGitleaksRatchetBytes(ratchetBytes(missing)), /missing/i);

  const extra = GITLEAKS_RATCHET_FINGERPRINTS.concat([
    '0123456789abcdef0123456789abcdef01234567:js/utils/synthetic.js:generic-api-key:1',
  ]);
  assertRejects(() => policy.checkGitleaksRatchetBytes(ratchetBytes(extra)), /extra/i);

  const duplicate = GITLEAKS_RATCHET_FINGERPRINTS.slice();
  duplicate[1] = duplicate[0];
  assertRejects(() => policy.checkGitleaksRatchetBytes(ratchetBytes(duplicate)), /duplicate/i);

  const malformed = GITLEAKS_RATCHET_FINGERPRINTS.slice();
  malformed[0] = 'not-a-fingerprint';
  assertRejects(() => policy.checkGitleaksRatchetBytes(ratchetBytes(malformed)), /malformed/i);

  const globalCurrentTree = GITLEAKS_RATCHET_FINGERPRINTS.slice();
  globalCurrentTree[3] = 'js/utils/synthetic.js:generic-api-key:1';
  assertRejects(
    () => policy.checkGitleaksRatchetBytes(ratchetBytes(globalCurrentTree)),
    /global|current-tree/i
  );

  const wrongCommit = GITLEAKS_RATCHET_FINGERPRINTS.slice();
  wrongCommit[0] = `0${wrongCommit[0].slice(1)}`;
  assertRejects(() => policy.checkGitleaksRatchetBytes(ratchetBytes(wrongCommit)), /wrong commit|commit/i);

  const wrongPath = GITLEAKS_RATCHET_FINGERPRINTS.slice();
  wrongPath[0] = wrongPath[0].replace('tickets/BBD-WAL-004.md', 'tickets/BBD-WAL-004.mdx');
  assert.notStrictEqual(
    wrongPath[0],
    GITLEAKS_RATCHET_FINGERPRINTS[0],
    'wrong-path mutation must change its fingerprint'
  );
  assertRejects(() => policy.checkGitleaksRatchetBytes(ratchetBytes(wrongPath)), /wrong path|path/i);

  const wrongRule = GITLEAKS_RATCHET_FINGERPRINTS.slice();
  wrongRule[0] = wrongRule[0].replace('generic-api-key', 'generic-api-token');
  assertRejects(() => policy.checkGitleaksRatchetBytes(ratchetBytes(wrongRule)), /wrong rule|rule/i);

  const wrongLine = GITLEAKS_RATCHET_FINGERPRINTS.slice();
  wrongLine[0] = wrongLine[0].replace(/:110$/, ':1');
  assert.notStrictEqual(
    wrongLine[0],
    GITLEAKS_RATCHET_FINGERPRINTS[0],
    'wrong-line mutation must change its fingerprint'
  );
  assertRejects(() => policy.checkGitleaksRatchetBytes(ratchetBytes(wrongLine)), /wrong line|line/i);

  const unsorted = GITLEAKS_RATCHET_FINGERPRINTS.slice();
  const swapped = unsorted[0];
  unsorted[0] = unsorted[1];
  unsorted[1] = swapped;
  assertRejects(() => policy.checkGitleaksRatchetBytes(ratchetBytes(unsorted)), /lexical|unsorted|order/i);

  const commented = GITLEAKS_RATCHET_FINGERPRINTS.slice();
  commented[0] = `# ${commented[0]}`;
  assertRejects(() => policy.checkGitleaksRatchetBytes(ratchetBytes(commented)), /comment/i);

  const blank = GITLEAKS_RATCHET_FINGERPRINTS.slice();
  blank[2] = '';
  assertRejects(() => policy.checkGitleaksRatchetBytes(ratchetBytes(blank)), /blank/i);

  const wildcard = GITLEAKS_RATCHET_FINGERPRINTS.slice();
  wildcard[0] = wildcard[0].replace('tickets/BBD-WAL-004.md', '*');
  assert.notStrictEqual(
    wildcard[0],
    GITLEAKS_RATCHET_FINGERPRINTS[0],
    'wildcard mutation must change its fingerprint'
  );
  assertRejects(() => policy.checkGitleaksRatchetBytes(ratchetBytes(wildcard)), /wildcard/i);

  const secretBearing = GITLEAKS_RATCHET_FINGERPRINTS.slice();
  secretBearing[0] = 'token=AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA';
  assertRejects(
    () => policy.checkGitleaksRatchetBytes(ratchetBytes(secretBearing)),
    /secret-bearing|secret/i
  );

  assertRejects(
    () => policy.checkGitleaksRatchetBytes(ratchetBytes(GITLEAKS_RATCHET_FINGERPRINTS, { bom: true })),
    /BOM/i
  );
  assertRejects(
    () => policy.checkGitleaksRatchetBytes(ratchetBytes(GITLEAKS_RATCHET_FINGERPRINTS, { newline: '\r\n', ending: '\r\n' })),
    /CRLF/i
  );
  assertRejects(
    () => policy.checkGitleaksRatchetBytes(
      ratchetBytes(GITLEAKS_RATCHET_FINGERPRINTS, { trailing: 'x' })
    ),
    /trailing/i
  );
});

test('exact current-tree Gitleaks dir scan presence, order, and flags are enforced', () => {
  const policy = loadPolicy();
  const workflows = loadWorkflows(policy);
  assert.strictEqual(policy.GITLEAKS_DIR_SCAN_CMD, GITLEAKS_DIR_SCAN_CMD);
  const steps = [...policy.iterSteps(workflows.security.data)].map(([, , step]) => step);
  const runs = steps.map((step) => policy.stepRunText(step));
  const installIndex = runs.findIndex((run) => run.includes(GITLEAKS_ARCHIVE_URL));
  assert.ok(installIndex >= 0, 'pinned Gitleaks install step is missing');
  assert.strictEqual(runs[installIndex + 1], GITLEAKS_SCAN_CMD);
  assert.strictEqual(runs[installIndex + 2], GITLEAKS_DIR_SCAN_CMD);
  assert.ok(workflows.security.text.includes(`      - "${GITLEAKS_IGNORE_REL}"\n`));
  assert.ok(workflows.security.text.includes(`      - "${METRICS_REL}"\n`));
  assert.ok(workflows.security.text.includes(`      - "${FEEDBACK_REL}"\n`));

  const missingDir = replaceOnce(
    workflows.security.text,
    `      - run: ${GITLEAKS_DIR_SCAN_CMD}\n`,
    ''
  );
  assertRejects(() => policy.checkSecurityWorkflow(missingDir), /gitleaks dir --redact=100 --no-banner \.|dir scan|immediately|follow/i);

  const reordered = replaceOnce(
    workflows.security.text,
    `      - run: ${GITLEAKS_SCAN_CMD}\n      - run: ${GITLEAKS_DIR_SCAN_CMD}\n`,
    `      - run: ${GITLEAKS_DIR_SCAN_CMD}\n      - run: ${GITLEAKS_SCAN_CMD}\n`
  );
  assertRejects(() => policy.checkSecurityWorkflow(reordered), /immediately|follow|order|gitleaks/i);

  const delayedDir = replaceOnce(
    workflows.security.text,
    `      - run: ${GITLEAKS_SCAN_CMD}\n      - run: ${GITLEAKS_DIR_SCAN_CMD}\n`,
    `      - run: ${GITLEAKS_SCAN_CMD}\n      - run: echo between-git-and-dir\n      - run: ${GITLEAKS_DIR_SCAN_CMD}\n`
  );
  assertRejects(() => policy.checkSecurityWorkflow(delayedDir), /immediately|follow|gitleaks/i);

  const missingRedact = replaceOnce(
    workflows.security.text,
    GITLEAKS_DIR_SCAN_CMD,
    'gitleaks dir --no-banner .'
  );
  assertRejects(
    () => policy.checkSecurityWorkflow(missingRedact),
    /gitleaks dir --redact=100 --no-banner \.|exactly|dir/i
  );

  const extraFlag = replaceOnce(
    workflows.security.text,
    GITLEAKS_DIR_SCAN_CMD,
    `${GITLEAKS_DIR_SCAN_CMD} --verbose`
  );
  assertRejects(
    () => policy.checkSecurityWorkflow(extraFlag),
    /gitleaks dir --redact=100 --no-banner \.|exactly|dir/i
  );

  assertRejects(
    () => policy.checkSecurityWorkflow(replaceOnce(workflows.security.text, `      - "${GITLEAKS_IGNORE_REL}"\n`, '')),
    /path/i
  );
  assertRejects(
    () => policy.checkSecurityWorkflow(replaceOnce(workflows.security.text, `      - "${METRICS_REL}"\n`, '')),
    /path/i
  );
  assertRejects(
    () => policy.checkSecurityWorkflow(replaceOnce(workflows.security.text, `      - "${FEEDBACK_REL}"\n`, '')),
    /path/i
  );
});

test('inherited metrics and feedback loaders are structurally neutralized and unused by maintained social source', () => {
  const policy = loadPolicy();
  const metricsSource = fs.readFileSync(path.join(repoRoot, METRICS_REL), 'utf8');
  const feedbackSource = fs.readFileSync(path.join(repoRoot, FEEDBACK_REL), 'utf8');
  const publicMetricsExports = [
    'mVersion',
    'isNewerVersion',
    'prettyRAM',
    'freeRAMPercentage',
    'userStats',
    'isMetricRestartNeeded',
    'addMetrics',
    'changeMetrics',
    'showMetricsModal',
    'recordEvent',
    'startAjaxEvent',
    'endAjaxEvent',
    'recordPrefixedEvent',
    'startPrefixedAjaxEvent',
    'endPrefixedAjaxEvent',
  ];
  for (const name of publicMetricsExports) {
    const exported =
      metricsSource.includes(`export const ${name}`) ||
      metricsSource.includes(`export function ${name}(`);
    assert.ok(exported, `metrics public export ${name} is missing`);
  }

  const addMetricsSource = exportedFunctionSource(metricsSource, 'addMetrics');
  assert.strictEqual(addMetricsSource, 'export function addMetrics() {\n}');
  assert.ok(!/createElement|appendChild|Countly|script|onload|localStorage|app_key|https?:\/\//.test(addMetricsSource));
  assert.strictEqual(feedbackSource, 'export function addFeedback() {\n}\n');
  policy.checkInheritedLoaderNeutralization(metricsSource, feedbackSource);

  const syntheticLoader = replaceOnce(
    metricsSource,
    'export function addMetrics() {\n}',
    [
      'export function addMetrics() {',
      '  const scriptEl = document.createElement("script");',
      '  scriptEl.src = "https://example.invalid/sdk.js";',
      '  window.Countly = { app_key: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa" };',
      '}',
    ].join('\n')
  );
  assertRejects(
    () => policy.checkInheritedLoaderNeutralization(syntheticLoader, feedbackSource),
    /loader|Countly|createElement|script/i
  );
  assertRejects(
    () => policy.checkInheritedLoaderNeutralization(
      metricsSource,
      'import x from "./metrics";\nexport function addFeedback() {\n}\n'
    ),
    /import|doorbell|loader/i
  );
  assertRejects(
    () => policy.checkInheritedLoaderNeutralization(
      metricsSource,
      'export function addFeedback() {\n  window.doorbellOptions = { appKey: "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb" };\n}\n'
    ),
    /doorbell|loader|appKey/i
  );

  for (const rel of MAINTAINED_SOCIAL_PATHS) {
    const source = fs.readFileSync(path.join(repoRoot, rel), 'utf8');
    assert.ok(
      !/utils\/metrics|utils\/feedback|addMetrics|addFeedback/.test(source),
      `${rel} must not import inherited metrics or feedback loaders`
    );
  }
});

const WALLET_TEST_SCRIPT = 'test:wallet';
const WALLET_TEST_CMD = 'node test/walletContract.node.js';
const WALLET_CI_CMD = 'npm run test:wallet';
const TOP_LEVEL_TEST_CMD = 'npm run test:social && npm run test:security && npm run test:wallet && npm run test:wallet-broker';
const WALLET_SOURCE_FILTER = 'wallet-contract/**';
const WALLET_CONTRACT_PATHS = [
  'wallet-contract/canonical.js',
  'wallet-contract/framing.js',
  'wallet-contract/model.js',
  'wallet-contract/state-machine.js',
  'wallet-contract/fakes.js',
  'wallet-contract/index.js',
];
const WALLET_BUILD_COMMANDS = WALLET_CONTRACT_PATHS.map((rel) => `node --check ${rel}`);
const WALLET_IMPORT_ALLOWLIST = [
  'crypto',
  'node:crypto',
  'buffer',
  'node:buffer',
  './canonical',
  './canonical.js',
  './framing',
  './framing.js',
  './model',
  './model.js',
  './state-machine',
  './state-machine.js',
  './fakes',
  './fakes.js',
  './index',
  './index.js',
];

function assertedTriggerPaths(workflow, workflowName) {
  const triggers = workflow.data.on;
  assert.ok(triggers && typeof triggers === 'object', `${workflowName} workflow triggers are missing`);
  const paths = [];
  for (const triggerName of ['push', 'pull_request']) {
    const trigger = triggers[triggerName];
    if (!trigger) {
      continue;
    }
    assert.ok(Array.isArray(trigger.paths), `${workflowName} ${triggerName} paths are missing`);
    paths.push([triggerName, trigger.paths]);
  }
  assert.ok(paths.length > 0, `${workflowName} has no maintained-source path-filtered trigger`);
  return paths;
}

test('wallet contract package command and maintained-source policy are exact and fail closed', () => {
  const policy = loadPolicy();
  const packageText = fs.readFileSync(path.join(repoRoot, 'package.json'), 'utf8');
  const pkg = JSON.parse(packageText);
  assert.ok(pkg.scripts && typeof pkg.scripts === 'object', 'package scripts are missing');
  assert.strictEqual(pkg.scripts[WALLET_TEST_SCRIPT], WALLET_TEST_CMD);
  assert.strictEqual(pkg.scripts.test, TOP_LEVEL_TEST_CMD);
  assert.strictEqual(typeof pkg.scripts.build, 'string');
  const buildCommands = pkg.scripts.build.split(/\s*&&\s*/);
  for (const command of WALLET_BUILD_COMMANDS) {
    assert.ok(buildCommands.includes(command), `build syntax path omits ${command}`);
  }
  assert.deepStrictEqual(policy.WALLET_CONTRACT_PATHS, WALLET_CONTRACT_PATHS);
  assert.deepStrictEqual(policy.WALLET_BUILD_COMMANDS, WALLET_BUILD_COMMANDS);
  assert.strictEqual(typeof policy.checkPackageJson, 'function');
  policy.checkPackageJson(packageText);

  const mutatedPackage = JSON.stringify(
    Object.assign({}, pkg, { scripts: Object.assign({}, pkg.scripts, { [WALLET_TEST_SCRIPT]: 'echo skipped' }) })
  );
  assertRejects(() => policy.checkPackageJson(mutatedPackage), /wallet|test:wallet|walletContract/i);

  const missingTopLevelWallet = JSON.stringify(
    Object.assign({}, pkg, {
      scripts: Object.assign({}, pkg.scripts, {
        test: 'npm run test:social && npm run test:security',
      }),
    })
  );
  assertRejects(() => policy.checkPackageJson(missingTopLevelWallet), /wallet|top-level|npm test/i);

  for (const command of WALLET_BUILD_COMMANDS) {
    const missingBuildCommand = JSON.stringify(
      Object.assign({}, pkg, {
        scripts: Object.assign({}, pkg.scripts, {
          build: pkg.scripts.build
            .split(/\s*&&\s*/)
            .filter((candidate) => candidate !== command)
            .join(' && '),
        }),
      })
    );
    assertRejects(() => policy.checkPackageJson(missingBuildCommand), /wallet|build|syntax|node --check/i);
  }
});

test('wallet maintained-source checker permits only exact literal pure sibling, crypto, and buffer module loads', () => {
  const policy = loadPolicy();
  assert.strictEqual(typeof policy.checkWalletContractSource, 'function');
  assert.deepStrictEqual(policy.WALLET_IMPORT_ALLOWLIST, WALLET_IMPORT_ALLOWLIST);
  for (const specifier of WALLET_IMPORT_ALLOWLIST) {
    for (const source of [
      `module.exports = require('${specifier}');`,
      `import '${specifier}';`,
      `async function load() { return import('${specifier}'); }`,
    ]) {
      policy.checkWalletContractSource(source, 'wallet-contract/synthetic.js');
    }
  }
  for (const specifier of [
    '../canonical',
    '../wallet-contract/canonical',
    '/wallet-contract/canonical.js',
    'C:/wallet-contract/canonical.js',
    './other',
    'left-pad',
    'path',
    'child_process',
    'node:fs',
    'net',
    'electron',
    'usb',
    'node-hid',
    'worker_threads',
  ]) {
    for (const source of [
      `module.exports = require('${specifier}');`,
      `import '${specifier}';`,
      `async function load() { return import('${specifier}'); }`,
    ]) {
      assertRejects(
        () => policy.checkWalletContractSource(source, 'wallet-contract/synthetic.js'),
        /wallet|allowlist|computed|module|source/i
      );
    }
  }
  for (const source of [
    "const name = 'crypto'; module.exports = require(name);",
    "module.exports = require('child_' + 'process');",
    'module.exports = require(`crypto`);',
    "const name = 'crypto'; async function load() { return import(name); }",
    "async function load() { return import('child_' + 'process'); }",
    'async function load() { return import(`crypto`); }',
    "fetch('https://example.invalid')",
    "new WebSocket('wss://example.invalid')",
  ]) {
    assertRejects(
      () => policy.checkWalletContractSource(source, 'wallet-contract/synthetic.js'),
      /wallet|forbidden|allowlist|computed|capability|module|network|source/i
    );
  }
});

test('wallet maintained-source filters are required on every routine social and security trigger', () => {
  const policy = loadPolicy();
  const workflows = loadWorkflows(policy);
  for (const [name, workflow, checker] of [
    ['social', workflows.social, policy.checkSocialWorkflow],
    ['security', workflows.security, policy.checkSecurityWorkflow],
  ]) {
    for (const [, paths] of assertedTriggerPaths(workflow, name)) {
      assert.ok(paths.includes(WALLET_SOURCE_FILTER), `${name} workflow omits ${WALLET_SOURCE_FILTER}`);
    }
    assert.ok(workflow.text.includes(`      - "${WALLET_SOURCE_FILTER}"\n`));
    const mutated = replaceOnce(workflow.text, `      - "${WALLET_SOURCE_FILTER}"\n`, '');
    assertRejects(() => checker.call(policy, mutated), /wallet-contract|wallet|path/i);
  }
});

test('routine CI executes the exact wallet contract command and rejects its removal', () => {
  const policy = loadPolicy();
  const workflows = loadWorkflows(policy);
  const routineCommands = [];
  for (const [, job, step] of policy.iterSteps(workflows.social.data)) {
    if (!job.if) {
      routineCommands.push(...policy.stepRunLines(step));
    }
  }
  assert.ok(routineCommands.includes(WALLET_CI_CMD), `routine CI omits ${WALLET_CI_CMD}`);
  const mutated = replaceOnce(workflows.social.text, `      - run: ${WALLET_CI_CMD}\n`, '');
  assertRejects(() => policy.checkSocialWorkflow(mutated), /wallet|test:wallet|walletContract/i);
});

const BROKER_BOUNDARY_PATHS = [
  'wallet-broker/protocol.js',
  'wallet-broker/supervisor.js',
  'wallet-preload.js',
];
const BROKER_TEST_COMMANDS = [
  'node test/walletBrokerProtocol.node.js',
  'node test/walletSupervisor.node.js',
  'node test/walletPreload.node.js',
];
const BROKER_TEST_SCRIPT = 'test:wallet-broker';
const BROKER_TEST_COMMAND = BROKER_TEST_COMMANDS.join(' && ');
const BROKER_CI_COMMAND = `npm run ${BROKER_TEST_SCRIPT}`;
const BROKER_BUILD_COMMANDS = BROKER_BOUNDARY_PATHS.map((rel) => `node --check ${rel}`);
const BROKER_IMPORT_ALLOWLISTS = {
  'wallet-broker/protocol.js': ['crypto', 'node:crypto', 'buffer', 'node:buffer'],
  'wallet-broker/supervisor.js': [
    'crypto', 'node:crypto', 'buffer', 'node:buffer', 'fs', 'node:fs', 'path', 'node:path',
    'child_process', 'node:child_process', './protocol', './protocol.js',
  ],
  'wallet-preload.js': ['electron'],
};
const PRELOAD_INVOKE_CHANNELS = [
  'wallet:snapshot:get',
  'wallet:intent:begin',
  'wallet:intent:cancel',
  'wallet:accounts:list',
  'wallet:payee-request:get',
];
const PRELOAD_SUBSCRIBE_CHANNEL = 'wallet:snapshot:subscribe';

test('wallet broker boundary package scripts and syntax checks are exact', () => {
  const policy = loadPolicy();
  const packageText = fs.readFileSync(path.join(repoRoot, 'package.json'), 'utf8');
  const pkg = JSON.parse(packageText);
  assert.strictEqual(pkg.scripts[BROKER_TEST_SCRIPT], BROKER_TEST_COMMAND);
  assert.ok(pkg.scripts.test.split(/\s*&&\s*/).includes(BROKER_CI_COMMAND));
  const build = pkg.scripts.build.split(/\s*&&\s*/);
  for (const command of BROKER_BUILD_COMMANDS) assert.ok(build.includes(command), `build omits ${command}`);
  assert.deepStrictEqual(policy.BROKER_BOUNDARY_PATHS, BROKER_BOUNDARY_PATHS);
  assert.deepStrictEqual(policy.BROKER_TEST_COMMANDS, BROKER_TEST_COMMANDS);
  policy.checkPackageJson(packageText);
  const missingScript = JSON.stringify(Object.assign({}, pkg, {
    scripts: Object.assign({}, pkg.scripts, { [BROKER_TEST_SCRIPT]: 'echo skipped' }),
  }));
  assertRejects(() => policy.checkPackageJson(missingScript), /wallet|broker|test/i);
  const missingTopLevel = JSON.stringify(Object.assign({}, pkg, {
    scripts: Object.assign({}, pkg.scripts, {
      test: pkg.scripts.test.split(/\s*&&\s*/).filter((item) => item !== BROKER_CI_COMMAND).join(' && '),
    }),
  }));
  assertRejects(() => policy.checkPackageJson(missingTopLevel), /wallet|broker|top-level|npm test/i);
  for (const command of BROKER_BUILD_COMMANDS) {
    const mutated = JSON.stringify(Object.assign({}, pkg, {
      scripts: Object.assign({}, pkg.scripts, {
        build: pkg.scripts.build.split(/\s*&&\s*/).filter((item) => item !== command).join(' && '),
      }),
    }));
    assertRejects(() => policy.checkPackageJson(mutated), /wallet|broker|build|syntax/i);
  }
});

test('wallet broker and preload paths are required on every routine workflow trigger', () => {
  const policy = loadPolicy();
  const workflows = loadWorkflows(policy);
  for (const [name, workflow, checker] of [
    ['social', workflows.social, policy.checkSocialWorkflow],
    ['security', workflows.security, policy.checkSecurityWorkflow],
  ]) {
    for (const [, paths] of assertedTriggerPaths(workflow, name)) {
      assert.ok(paths.includes('wallet-broker/**'));
      assert.ok(paths.includes('wallet-preload.js'));
    }
    for (const filter of ['wallet-broker/**', 'wallet-preload.js']) {
      const mutated = replaceOnce(workflow.text, `      - "${filter}"\n`, '');
      assertRejects(() => checker.call(policy, mutated), /wallet|broker|preload|path/i);
    }
  }
});

test('routine CI executes the named wallet broker suite and rejects omission', () => {
  const policy = loadPolicy();
  const workflows = loadWorkflows(policy);
  const commands = [];
  for (const [, job, step] of policy.iterSteps(workflows.social.data)) {
    if (!job.if) commands.push(...policy.stepRunLines(step));
  }
  assert.ok(commands.includes(BROKER_CI_COMMAND), `routine CI omits ${BROKER_CI_COMMAND}`);
  assertRejects(
    () => policy.checkSocialWorkflow(replaceOnce(workflows.social.text, `      - run: ${BROKER_CI_COMMAND}\n`, '')),
    /wallet|broker|preload|test/i
  );
});

test('wallet boundary source policy allows only reviewed built-ins and forbids listeners, shell, and generic IPC', () => {
  const policy = loadPolicy();
  assert.strictEqual(typeof policy.checkWalletBoundarySource, 'function');
  assert.deepStrictEqual(policy.BROKER_IMPORT_ALLOWLISTS, BROKER_IMPORT_ALLOWLISTS);
  assert.deepStrictEqual(policy.PRELOAD_INVOKE_CHANNELS, PRELOAD_INVOKE_CHANNELS);
  assert.strictEqual(policy.PRELOAD_SUBSCRIBE_CHANNEL, PRELOAD_SUBSCRIBE_CHANNEL);
  policy.checkWalletBoundarySource("const crypto = require('crypto');", 'wallet-broker/protocol.js');
  policy.checkWalletBoundarySource(
    "const { spawn } = require('child_process'); spawn(file, [], { shell: false, stdio: ['pipe','pipe','pipe'], env: cleanEnv });",
    'wallet-broker/supervisor.js'
  );
  policy.checkWalletBoundarySource(
    [
      "const { contextBridge, ipcRenderer } = require('electron');",
      "const api = {",
      "  getSnapshot: () => ipcRenderer.invoke('wallet:snapshot:get'),",
      "  beginIntent: (value) => ipcRenderer.invoke('wallet:intent:begin', value),",
      "  cancelIntent: (value) => ipcRenderer.invoke('wallet:intent:cancel', value),",
      "  listAccounts: () => ipcRenderer.invoke('wallet:accounts:list'),",
      "  getPayeeRequest: (value) => ipcRenderer.invoke('wallet:payee-request:get', value),",
      "};",
      "const listener = (_event, value) => callback(value);",
      "ipcRenderer.on('wallet:snapshot:subscribe', listener);",
      "ipcRenderer.removeListener('wallet:snapshot:subscribe', listener);",
      "contextBridge.exposeInMainWorld('bitbookWallet', api);",
    ].join('\n'),
    'wallet-preload.js'
  );
  const wrongPathLoads = [
    ['wallet-broker/protocol.js', "require('fs')"],
    ['wallet-broker/protocol.js', "require('child_process')"],
    ['wallet-broker/protocol.js', "require('electron')"],
    ['wallet-broker/supervisor.js', "require('electron')"],
    ['wallet-preload.js', "require('fs')"],
    ['wallet-preload.js', "require('child_process')"],
  ];
  const forbidden = wrongPathLoads.concat([
    ['wallet-broker/supervisor.js', "const c=require('child_process'); c.exec('wallet')"],
    ['wallet-broker/supervisor.js', "const c=require('child_process'); c.execFile(file)"],
    ['wallet-broker/supervisor.js', "const c=require('child_process'); c.spawnSync(file)"],
    ['wallet-broker/supervisor.js', "const c=require('child_process'); c.execSync('wallet')"],
    ['wallet-broker/supervisor.js', "const c=require('child_process'); c.execFileSync(file)"],
    ['wallet-broker/supervisor.js', "const c=require('child_process'); c.fork(file)"],
    ['wallet-broker/supervisor.js', "spawn(file, [], { shell: true, env: cleanEnv })"],
    ['wallet-broker/supervisor.js', "spawn(file, [], { shell: false, env: process.env })"],
    ['wallet-broker/supervisor.js', "spawn(file, [], { shell: false, env: { PATH: process.env.PATH } })"],
    ['wallet-broker/supervisor.js', "spawn(file, ['--token=secret'], { shell: false, env: cleanEnv })"],
    ['wallet-broker/supervisor.js', "spawn(file, ['--verbose'], { shell: false, env: cleanEnv })"],
    ['wallet-broker/supervisor.js', "spawn(file, [], { shell: false, stdio: 'inherit', env: cleanEnv })"],
    ['wallet-broker/supervisor.js', "spawn(file, [], { shell: false, stdio: ['pipe','inherit','pipe'], env: cleanEnv })"],
    ['wallet-broker/supervisor.js', "require('net').createServer().listen('/tmp/wallet.sock')"],
    ['wallet-broker/supervisor.js', "require('net').createServer().listen('\\\\.\\pipe\\wallet')"],
    ['wallet-broker/supervisor.js', "require('net').createServer().listen(8123, '127.0.0.1')"],
    ['wallet-broker/supervisor.js', "require('http').createServer().listen(8080)"],
    ['wallet-broker/protocol.js', "const name='crypto'; require(name)"],
    ['wallet-broker/protocol.js', "import('crypto')"],
    ['wallet-preload.js', "ipcRenderer.invoke(channel, payload)"],
    ['wallet-preload.js', "ipcRenderer.invoke('wallet:intent:confirm')"],
    ['wallet-preload.js', "ipcRenderer.invoke('wallet:account:unlock')"],
    ['wallet-preload.js', "ipcRenderer.invoke('wallet:account:export-backup')"],
    ['wallet-preload.js', "ipcRenderer.invoke('wallet:account:create-software')"],
    ['wallet-preload.js', "ipcRenderer.invoke('wallet:signer:sign')"],
    ['wallet-preload.js', "ipcRenderer.invoke('wallet:tx:broadcast')"],
    ['wallet-preload.js', "ipcRenderer.invoke('wallet:intent:broadcast')"],
    ['wallet-preload.js', "ipcRenderer.invoke('wallet:anything')"],
    ['wallet-preload.js', "ipcRenderer.send('wallet:snapshot:get')"],
    ['wallet-preload.js', "ipcRenderer.sendSync('wallet:any')"],
    ['wallet-preload.js', "ipcRenderer.on('wallet:snapshot:subscribe', listener); ipcRenderer.removeListener('wallet:other', listener)"],
    ['wallet-broker/supervisor.js', "fetch('https://provider.invalid')"],
    ['wallet-broker/supervisor.js', "new WebSocket('ws://127.0.0.1')"],
    ['wallet-broker/supervisor.js', "require('worker_threads')"],
    ['wallet-broker/supervisor.js', "require('usb')"],
    ['wallet-broker/supervisor.js', "require('node-hid')"],
    ['wallet-broker/supervisor.js', "dispatch('rpc.raw', params)"],
    ['wallet-broker/supervisor.js', "dispatch('rate.fetch', params)"],
    ['wallet-broker/supervisor.js', "dispatch('http.proxy', params)"],
    ['wallet-broker/supervisor.js', "dispatch('wallet.raw', params)"],
  ]);
  for (const [rel, source] of forbidden) assertRejects(
    () => policy.checkWalletBoundarySource(source, rel),
    /wallet|boundary|forbidden|allowlist|listener|generic|module|capability|spawn|ipc/i
  );
});

const WAL004_MANIFEST = 'wallet-broker/Cargo.toml';
const WAL004_LOCKFILE = 'wallet-broker/Cargo.lock';
const WAL004_TOOLCHAIN = '1.98.0';
const WAL004_PLATFORM = 'linux';
const WAL004_ROUTINE_TEST =
  'cargo +1.98.0 test --manifest-path wallet-broker/Cargo.toml --locked --no-default-features';
const WAL004_FMT = 'cargo +1.98.0 fmt --manifest-path wallet-broker/Cargo.toml --check';
const WAL004_CLIPPY =
  'cargo +1.98.0 clippy --manifest-path wallet-broker/Cargo.toml --locked --all-targets --all-features -- -D warnings';
const WAL004_NATIVE_CHECK =
  'cargo +1.98.0 check --manifest-path wallet-broker/Cargo.toml --locked --features native-ui --test native_surface';
const CARGO_AUDIT_VERSION = '0.22.2';
const CARGO_DENY_VERSION = '0.20.2';
const CARGO_CYCLONEDX_VERSION = '0.5.9';
const WAL004_AUDIT = 'cargo +1.98.0 audit --file wallet-broker/Cargo.lock';
const WAL004_DENY =
  'cargo +1.98.0 deny --manifest-path wallet-broker/Cargo.toml --all-features check advisories bans licenses sources';
const WAL004_SBOM =
  'cargo +1.98.0 cyclonedx --manifest-path wallet-broker/Cargo.toml --format json --all-features';
const WAL004_DIRECT_DEPENDENCIES = {
  argon2: { version: '=0.5.3', default_features: false, features: ['alloc'], optional: false },
  base64ct: { version: '=1.8.3', default_features: false, features: ['alloc'], optional: false },
  chacha20poly1305: { version: '=0.11.0', default_features: false, features: ['alloc'], optional: false },
  eframe: {
    version: '=0.36.1', default_features: false,
    features: ['default_fonts', 'glow', 'wayland', 'x11'], optional: true,
  },
  getrandom: { version: '=0.4.3', default_features: false, features: ['std'], optional: false },
  hkdf: { version: '=0.12.4', default_features: false, features: [], optional: false },
  rfd: {
    version: '=0.17.2', default_features: false,
    features: ['xdg-portal', 'wayland'], optional: true,
  },
  secrecy: { version: '=0.10.3', default_features: false, features: [], optional: false },
  serde: {
    version: '=1.0.229', default_features: false, features: ['alloc', 'derive'], optional: false,
  },
  serde_json: { version: '=1.0.151', default_features: false, features: ['alloc'], optional: false },
  sha2: { version: '=0.10.9', default_features: false, features: [], optional: false },
  zeroize: { version: '=1.9.0', default_features: false, features: ['alloc'], optional: false },
};
const WAL004_RUST_SOURCE_PATHS = [
  'wallet-broker/src/lib.rs',
  'wallet-broker/src/vault.rs',
  'wallet-broker/src/store.rs',
  'wallet-broker/src/session.rs',
  'wallet-broker/src/native.rs',
  'wallet-broker/src/native_ui.rs',
  'wallet-broker/src/hygiene.rs',
];
const WAL004_ALLOWED_LICENSES = [
  'MIT',
  'Apache-2.0',
  'Apache-2.0 WITH LLVM-exception',
  'BSD-2-Clause',
  'BSD-3-Clause',
  'BSL-1.0',
  'ISC',
  'Zlib',
  '0BSD',
  'Unlicense',
  'Unicode-3.0',
  'OFL-1.1',
  'Ubuntu-font-1.0',
];
const WAL004_DIRECT_COMPONENTS = Object.freeze(Object.keys(WAL004_DIRECT_DEPENDENCIES));

test('WAL-004 Rust test-harness manifest pins the exact toolchain, dependencies, and native features', () => {
  const policy = loadPolicy();
  const manifestText = fs.readFileSync(path.join(repoRoot, WAL004_MANIFEST), 'utf8');
  assert.strictEqual(policy.WAL004_TOOLCHAIN, WAL004_TOOLCHAIN);
  assert.strictEqual(policy.WAL004_PLATFORM, WAL004_PLATFORM);
  assert.deepStrictEqual(policy.WAL004_DIRECT_DEPENDENCIES, WAL004_DIRECT_DEPENDENCIES);
  assert.strictEqual(typeof policy.checkWalletBrokerManifest, 'function');
  policy.checkWalletBrokerManifest(manifestText, { requireLibrary: false, requireLockfile: false });
  for (const [fromLine, toLine] of [
    [
      'argon2 = { version = "=0.5.3", default-features = false, features = ["alloc"] }',
      'argon2 = { version = "=0.6.0", default-features = false, features = ["alloc"] }',
    ],
    [
      'argon2 = { version = "=0.5.3", default-features = false, features = ["alloc"] }',
      'argon2 = { version = "0.5", default-features = false, features = ["alloc"] }',
    ],
    [
      'hkdf = { version = "=0.12.4", default-features = false }',
      'hkdf = { version = "=0.13.0", default-features = false }',
    ],
    [
      'hkdf = { version = "=0.12.4", default-features = false }',
      'hkdf = { version = "0.12", default-features = false }',
    ],
    [
      'sha2 = { version = "=0.10.9", default-features = false }',
      'sha2 = { version = "=0.11.0", default-features = false }',
    ],
    [
      'sha2 = { version = "=0.10.9", default-features = false }',
      'sha2 = { version = "0.10", default-features = false }',
    ],
  ]) {
    const from = `${fromLine}\n`;
    const to = `${toLine}\n`;
    assert.strictEqual(
      manifestText.split(from).length - 1,
      1,
      `manifest dependency line must be unique: ${fromLine}`
    );
    const mutated = replaceOnce(manifestText, from, to);
    assert.notStrictEqual(mutated, manifestText, `dependency mutation did not change ${fromLine}`);
    assertRejects(
      () => policy.checkWalletBrokerManifest(mutated, {
        requireLibrary: false, requireLockfile: false,
      }),
      /wallet|rust|manifest|dependency|feature|pin|native/i
    );
  }
  for (const [from, to] of [
    ['rust-version = "1.98.0"', 'rust-version = "1.97.0"'],
    ['version = "=0.11.0"', 'version = "=0.10.0"'],
    ['default-features = false', 'default-features = true'],
    [
      'secrecy = { version = "=0.10.3", default-features = false }',
      'secrecy = { version = "=0.10.3", default-features = false, features = ["serde"] }',
    ],
    ['"default_fonts", "glow", "wayland", "x11"', '"default_fonts", "glow", "persistence", "wayland", "x11"'],
    ['"xdg-portal", "wayland"', '"xdg-portal", "tokio", "wayland"'],
    ['default = []', 'default = ["native-ui"]'],
    ['publish = false', 'publish = true'],
  ]) {
    assertRejects(
      () => policy.checkWalletBrokerManifest(replaceOnce(manifestText, from, to), {
        requireLibrary: false, requireLockfile: false,
      }),
      /wallet|rust|manifest|dependency|feature|pin|native/i
    );
  }
  for (const addition of [
    '\nreqwest = "=0.13.0"\n',
    '\ntokio = "=1.0.0"\n',
    '\nkeyring = "=3.0.0"\n',
    '\nzcash_client_backend = "=0.20.0"\n',
    '\nmonero = { git = "https://example.invalid/monero" }\n',
  ]) {
    assertRejects(
      () => policy.checkWalletBrokerManifest(`${manifestText}${addition}`, {
        requireLibrary: false, requireLockfile: false,
      }),
      /wallet|dependency|network|coin|keyring|git|manifest/i
    );
  }
});

test('WAL-004 Rust first-party source policy forbids unsafe and unreviewed authority', () => {
  const policy = loadPolicy();
  assert.strictEqual(typeof policy.checkRustWalletSource, 'function');
  policy.checkRustWalletSource(
    'use zeroize::Zeroize; pub fn wipe(bytes: &mut [u8]) { bytes.zeroize(); }',
    'wallet-broker/src/synthetic.rs'
  );
  for (const source of [
    'unsafe fn touch_secret() {}',
    'unsafe { core::ptr::read(ptr) }',
    'extern "C" { fn wallet(); }',
    'use std::net::TcpListener;',
    'use std::os::unix::net::UnixListener;',
    'use reqwest::Client;',
    'use tokio::net::TcpStream;',
    'use keyring::Entry;',
    'use zcash_client_backend::data_api;',
    'use monero::Wallet;',
    'std::env::temp_dir()',
    'Command::new("curl")',
  ]) {
    assertRejects(
      () => policy.checkRustWalletSource(source, 'wallet-broker/src/synthetic.rs'),
      /wallet|rust|unsafe|network|listener|coin|keyring|temp|process|authority/i
    );
  }
});

test('WAL-004 exact Rust test build lint and native compile commands are reserved', () => {
  const policy = loadPolicy();
  assert.strictEqual(policy.WAL004_ROUTINE_TEST, WAL004_ROUTINE_TEST);
  assert.strictEqual(policy.WAL004_FMT, WAL004_FMT);
  assert.strictEqual(policy.WAL004_CLIPPY, WAL004_CLIPPY);
  assert.strictEqual(policy.WAL004_NATIVE_CHECK, WAL004_NATIVE_CHECK);
  assert.deepStrictEqual(policy.WAL004_REQUIRED_FILES, [
    WAL004_MANIFEST,
    WAL004_LOCKFILE,
    'wallet-broker/src/lib.rs',
  ]);
});

test('WAL-004 routine Linux CI is single-platform, locked, package-free, and path-filtered', () => {
  const policy = loadPolicy();
  const social = loadWorkflows(policy).social;
  for (const [, paths] of assertedTriggerPaths(social, 'social')) {
    assert.ok(paths.includes('wallet-broker/**'));
  }
  const routine = [];
  for (const [, job, step] of policy.iterSteps(social.data)) {
    if (!job.if) {
      assert.strictEqual(job['runs-on'], 'ubuntu-latest');
      routine.push(...policy.stepRunLines(step));
    }
  }
  assert.ok(routine.includes(WAL004_ROUTINE_TEST));
  assert.ok(!routine.some((line) => /package:|cargo\s+install|native-ui.*run/.test(line)));
  const missing = replaceOnce(social.text, `      - run: ${WAL004_ROUTINE_TEST}\n`, '');
  assertRejects(() => policy.checkSocialWorkflow(missing), /wallet|rust|cargo|test/i);
});

test('WAL-004 RustSec and cargo-deny gates use exact tool versions and locked inputs', () => {
  const policy = loadPolicy();
  const security = loadWorkflows(policy).security;
  for (const [, paths] of assertedTriggerPaths(security, 'security')) {
    assert.ok(paths.includes('wallet-broker/**'));
  }
  assert.strictEqual(policy.CARGO_AUDIT_VERSION, CARGO_AUDIT_VERSION);
  assert.strictEqual(policy.CARGO_DENY_VERSION, CARGO_DENY_VERSION);
  assert.strictEqual(policy.CARGO_CYCLONEDX_VERSION, CARGO_CYCLONEDX_VERSION);
  assert.strictEqual(policy.WAL004_AUDIT, WAL004_AUDIT);
  assert.strictEqual(policy.WAL004_DENY, WAL004_DENY);
  for (const required of [
    `cargo install cargo-audit --version ${CARGO_AUDIT_VERSION} --locked`,
    `cargo install cargo-deny --version ${CARGO_DENY_VERSION} --locked`,
    WAL004_AUDIT,
    WAL004_DENY,
  ]) assert.ok(security.text.includes(required), `security workflow omits ${required}`);
  assertRejects(
    () => policy.checkSecurityWorkflow(replaceOnce(security.text, CARGO_AUDIT_VERSION, 'latest')),
    /cargo-audit|RustSec|version|pin/i
  );
  assertRejects(
    () => policy.checkSecurityWorkflow(replaceOnce(security.text, '--all-features check advisories bans licenses sources', 'check advisories')),
    /cargo-deny|bans|licenses|sources|features/i
  );
});

test('WAL-004 manual SBOM contains separately validated npm and Rust CycloneDX JSON artifacts', () => {
  const policy = loadPolicy();
  const sbom = loadWorkflows(policy).sbom;
  assert.strictEqual(policy.WAL004_SBOM, WAL004_SBOM);
  assert.ok(sbom.text.includes(`cargo install cargo-cyclonedx --version ${CARGO_CYCLONEDX_VERSION} --locked`));
  assert.ok(sbom.text.includes(WAL004_SBOM));
  assert.ok(sbom.text.includes('node scripts/validate-sbom.js'));
  assert.ok(sbom.text.includes('node scripts/validate-rust-sbom.js'));
  const uploadPaths = [];
  for (const [, , step] of policy.iterSteps(sbom.data)) {
    if (String(step.uses || '').startsWith('actions/upload-artifact@')) {
      uploadPaths.push(step.with && step.with.path);
    }
  }
  assert.deepStrictEqual(uploadPaths.sort(), [
    '${{ runner.temp }}/bitbook-desktop.cdx.json',
    '${{ runner.temp }}/bitbook-wallet-broker.cdx.json',
  ]);
  assertRejects(
    () => policy.checkSbomWorkflow(replaceOnce(sbom.text, 'node scripts/validate-rust-sbom.js', 'echo skip-rust-validation')),
    /Rust|wallet|CycloneDX|validate/i
  );
  assertRejects(
    () => policy.checkSbomWorkflow(replaceOnce(
      sbom.text,
      WAL004_SBOM,
      WAL004_SBOM.slice(0, -' --all-features'.length)
    )),
    /Rust|wallet|CycloneDX|feature|exact/i
  );
});

test('WAL-004 policy and validator changes trigger every applicable routine workflow', () => {
  const policy = loadPolicy();
  const workflows = loadWorkflows(policy);
  const requiredByWorkflow = [
    [
      'social',
      workflows.social,
      policy.checkSocialWorkflow,
      ['scripts/validate-rust-sbom.js'],
    ],
    [
      'security',
      workflows.security,
      policy.checkSecurityWorkflow,
      ['scripts/validate-rust-sbom.js', 'deny.toml'],
    ],
  ];

  for (const [name, workflow, checker, required] of requiredByWorkflow) {
    for (const [triggerName, paths] of assertedTriggerPaths(workflow, name)) {
      for (const rel of required) {
        assert.ok(paths.includes(rel), `${name} ${triggerName} omits ${rel}`);
        const mutatedData = JSON.parse(JSON.stringify(workflow.data));
        mutatedData.on[triggerName].paths = mutatedData.on[triggerName].paths
          .filter((candidate) => candidate !== rel);
        assertRejects(
          () => checker.call(policy, workflow.text, mutatedData),
          /path|Rust|SBOM|deny|wallet/i
        );
      }
    }
  }
  assert.deepStrictEqual(Object.keys(workflows.sbom.data.on), ['workflow_dispatch']);
});

test('WAL-004 Rust source inventory is exported closed and enumerated by repository policy', () => {
  const policy = loadPolicy();
  assert.deepStrictEqual(policy.WAL004_RUST_SOURCE_PATHS, WAL004_RUST_SOURCE_PATHS);
  assert.strictEqual(typeof policy.checkRustWalletSourceInventory, 'function');

  const actual = fs.readdirSync(path.join(repoRoot, 'wallet-broker', 'src'), {
    withFileTypes: true,
  }).filter((entry) => entry.isFile() && entry.name.endsWith('.rs'))
    .map((entry) => `wallet-broker/src/${entry.name}`);
  policy.checkRustWalletSourceInventory(actual);
  const freshCheckoutOrder = [...WAL004_RUST_SOURCE_PATHS].sort();
  policy.checkRustWalletSourceInventory(freshCheckoutOrder);
  assertRejects(
    () => policy.checkRustWalletSourceInventory(WAL004_RUST_SOURCE_PATHS.slice(1)),
    /Rust|source|inventory|missing/i
  );
  assertRejects(
    () => policy.checkRustWalletSourceInventory([
      ...WAL004_RUST_SOURCE_PATHS,
      'wallet-broker/src/extra.rs',
    ]),
    /Rust|source|inventory|extra|unknown/i
  );
  assertRejects(
    () => policy.checkRustWalletSourceInventory([
      ...WAL004_RUST_SOURCE_PATHS,
      WAL004_RUST_SOURCE_PATHS[0],
    ]),
    /Rust|source|inventory|duplicate/i
  );
  const malformed = [...WAL004_RUST_SOURCE_PATHS];
  malformed[3] = null;
  assertRejects(
    () => policy.checkRustWalletSourceInventory(malformed),
    /Rust|source|inventory|malformed|string/i
  );

  const repositoryChecker = policy.checkRepository.toString();
  assert.match(repositoryChecker, /readdirSync/);
  assert.match(repositoryChecker, /checkRustWalletSourceInventory/);
  assert.match(repositoryChecker, /for\s*\(const rel of WAL004_RUST_SOURCE_PATHS\)/);
});

test('WAL-004 vault and native source policy requires reviewed secret and path primitives', () => {
  const policy = loadPolicy();
  const reviewedVault = [
    'use base64ct::{Base64Unpadded, Encoding};',
    'use secrecy::{ExposeSecret, ExposeSecretMut, SecretSlice};',
    'use zeroize::Zeroize;',
    'pub fn reviewed(secret: &mut SecretSlice<u8>) {',
    '  let _ = Base64Unpadded::encode_string(secret.expose_secret());',
    '  secret.expose_secret_mut().zeroize();',
    '}',
  ].join('\n');
  policy.checkRustWalletSource(reviewedVault, 'wallet-broker/src/vault.rs');
  for (const primitive of [
    'Base64Unpadded',
    'Encoding',
    'SecretSlice',
    'ExposeSecret',
    'ExposeSecretMut',
  ]) {
    const mutated = reviewedVault.replaceAll(primitive, `Removed${primitive}`);
    assert.notStrictEqual(mutated, reviewedVault, `mutation target missing: ${primitive}`);
    assertRejects(
      () => policy.checkRustWalletSource(
        mutated,
        'wallet-broker/src/vault.rs'
      ),
      /base64|secrecy|secret|primitive|vault/i
    );
  }
  for (const helper of ['encode_base64', 'decode_base64']) {
    assertRejects(
      () => policy.checkRustWalletSource(
        `${reviewedVault}\nfn ${helper}(_bytes: &[u8]) {}`,
        'wallet-broker/src/vault.rs'
      ),
      /base64|handwritten|helper|vault/i
    );
  }

  const reviewedNativePath = 'let selected = path.to_str().ok_or_else(NativeError::locked)?;';
  policy.checkRustWalletSource(reviewedNativePath, 'wallet-broker/src/native_ui.rs');
  assertRejects(
    () => policy.checkRustWalletSource(
      'let selected = path.to_string_lossy().into_owned();',
      'wallet-broker/src/native_ui.rs'
    ),
    /path|lossy|UTF-8|native/i
  );

  policy.checkRustWalletSource(
    fs.readFileSync(path.join(repoRoot, 'wallet-broker/src/vault.rs'), 'utf8'),
    'wallet-broker/src/vault.rs'
  );
  policy.checkRustWalletSource(
    fs.readFileSync(path.join(repoRoot, 'wallet-broker/src/native_ui.rs'), 'utf8'),
    'wallet-broker/src/native_ui.rs'
  );
});

test('WAL-004 cargo-deny policy is exact fail-closed and has no bypass lists', () => {
  const policy = loadPolicy();
  const denyText = fs.readFileSync(path.join(repoRoot, 'deny.toml'), 'utf8');
  assert.deepStrictEqual(policy.WAL004_ALLOWED_LICENSES, WAL004_ALLOWED_LICENSES);
  assert.strictEqual(typeof policy.checkWalletBrokerDenyPolicy, 'function');
  policy.checkWalletBrokerDenyPolicy(denyText);

  const mutations = [
    ['ignore = []', 'ignore = ["RUSTSEC-0000-0000"]'],
    ['yanked = "deny"', 'yanked = "warn"'],
    ['  "MIT",\n', ''],
    ['  "MIT",\n', '  "MIT",\n  "GPL-3.0-only",\n'],
    ['exceptions = []', 'exceptions = [{ allow = ["GPL-3.0-only"], name = "synthetic" }]'],
    ['multiple-versions = "warn"', 'multiple-versions = "allow"'],
    ['wildcards = "deny"', 'wildcards = "allow"'],
    ['skip = []', 'skip = [{ name = "synthetic", version = "=1.0.0" }]'],
    ['skip-tree = []', 'skip-tree = [{ name = "synthetic", depth = 1 }]'],
    ['unknown-registry = "deny"', 'unknown-registry = "allow"'],
    ['unknown-git = "deny"', 'unknown-git = "allow"'],
    [
      'allow-registry = ["https://github.com/rust-lang/crates.io-index"]',
      'allow-registry = ["https://github.com/rust-lang/crates.io-index", "https://example.invalid/index"]',
    ],
    ['allow-git = []', 'allow-git = ["https://example.invalid/repository"]'],
  ];
  for (const [from, to] of mutations) {
    assertRejects(
      () => policy.checkWalletBrokerDenyPolicy(replaceOnce(denyText, from, to)),
      /deny|advisory|yanked|license|exception|duplicate|wildcard|skip|registry|git|source/i
    );
  }
  assertRejects(
    () => policy.checkWalletBrokerDenyPolicy(
      replaceOnce(denyText, 'ignore = []', 'ignore = []\nvulnerability = "allow"')
    ),
    /deny|advisory|deprecated|vulnerability/i
  );
});

test('WAL-004 Rust SBOM validator accepts only a complete broker CycloneDX graph', () => {
  const validator = loadRustSbomValidator();
  assert.deepStrictEqual(validator.DIRECT_COMPONENTS, WAL004_DIRECT_COMPONENTS);
  const componentRefs = WAL004_DIRECT_COMPONENTS.map((name) => `pkg:cargo/${name}@1.0.0`);
  const rootRef = 'pkg:cargo/bitbook-wallet-broker@0.1.0';
  const document = {
    bomFormat: 'CycloneDX',
    specVersion: '1.6',
    metadata: {
      component: {
        type: 'application',
        name: 'bitbook-wallet-broker',
        version: '0.1.0',
        'bom-ref': rootRef,
        purl: rootRef,
      },
    },
    components: WAL004_DIRECT_COMPONENTS.map((name, index) => ({
      type: 'library',
      name,
      version: '1.0.0',
      'bom-ref': componentRefs[index],
      purl: componentRefs[index],
    })),
    dependencies: [
      { ref: rootRef, dependsOn: componentRefs },
      ...componentRefs.map((ref) => ({ ref, dependsOn: [] })),
    ],
  };
  validator.validateRustCycloneDxDocument(document);

  const desktopRoot = JSON.parse(JSON.stringify(document));
  desktopRoot.metadata.component.name = 'bitbook-desktop';
  desktopRoot.metadata.component.purl = 'pkg:npm/bitbook-desktop@0.1.0';
  assertRejects(
    () => validator.validateRustCycloneDxDocument(desktopRoot),
    /Rust|broker|npm|root/i
  );
  for (const malformed of [
    null,
    {},
    Object.assign({}, document, { components: [] }),
    Object.assign({}, document, { dependencies: [] }),
  ]) {
    assertRejects(
      () => validator.validateRustCycloneDxDocument(malformed),
      /Rust|SBOM|CycloneDX|metadata|component|dependencies|graph/i
    );
  }
  for (const omitted of WAL004_DIRECT_COMPONENTS) {
    const incomplete = Object.assign({}, document, {
      components: document.components.filter((component) => component.name !== omitted),
    });
    assertRejects(
      () => validator.validateRustCycloneDxDocument(incomplete),
      /Rust|SBOM|direct|component|omit/i
    );
  }
});

const WAL006_DIRECT_DEPENDENCIES = {
  zcash_client_backend: {
    version: '=0.24.0', default_features: false, features: ['pczt'], optional: false,
  },
  zcash_client_sqlite: {
    version: '=0.22.0',
    default_features: false,
    features: ['orchard', 'test-dependencies', 'transparent-inputs'],
    optional: false,
  },
  pczt: { version: '=0.9.3', default_features: false, features: [], optional: false },
  zcash_primitives: {
    version: '=0.30.1', default_features: false, features: [], optional: false,
  },
  zcash_protocol: {
    version: '=0.10.5', default_features: false, features: ['local-consensus'], optional: false,
  },
  zcash_keys: {
    version: '=0.16.1', default_features: false, features: ['orchard'], optional: false,
  },
};
const WAL006_TEST_TARGETS = [
  'zec_fixture_builder',
  'zec_address',
  'zec_store',
  'zec_scan',
  'zec_prepare',
  'zec_hygiene',
];
const WAL006_FORBIDDEN_FEATURES = [
  'sync',
  'lightwalletd-tonic',
  'lightwalletd-tonic-tls-webpki-roots',
  'lightwalletd-tonic-transport',
  'tor',
  'zcashd-compat',
  'zewif',
  'non-standard-fees',
];
const WAL006_EXPECTED_COMPILED_PCZT_CAPABILITIES = [
  'io-finalizer',
  'orchard',
  'prover',
  'sapling',
  'signer',
  'spend-finalizer',
  'transparent',
  'transparent-inputs',
  'tx-extractor',
  'zcp-builder',
];
const WAL006_ALLOWED_RUST_SOURCE_PATHS = [];

test('WAL-006 manifest requires six exact defaults-off pins and the minimum direct feature union', () => {
  const policy = loadPolicy();
  const manifestText = fs.readFileSync(path.join(repoRoot, WAL004_MANIFEST), 'utf8');
  assert.deepStrictEqual(policy.WAL006_DIRECT_DEPENDENCIES, WAL006_DIRECT_DEPENDENCIES);
  assert.deepStrictEqual(policy.WAL006_TEST_TARGETS, WAL006_TEST_TARGETS);
  policy.checkWalletBrokerManifest(manifestText, { requireLibrary: true, requireLockfile: false });

  for (const [name, expected] of Object.entries(WAL006_DIRECT_DEPENDENCIES)) {
    const escapedName = name.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
    const line = manifestText.match(new RegExp(`^${escapedName}\\s*=\\s*\\{[^\\n]+\\}$`, 'm'));
    assert.ok(line, `manifest omits direct dependency ${name}`);
    assert.ok(line[0].includes(`version = "${expected.version}"`));
    assert.ok(line[0].includes('default-features = false'));
  }
  for (const target of WAL006_TEST_TARGETS) {
    assert.ok(manifestText.includes(`name = "${target}"`));
    assert.ok(manifestText.includes(`path = "tests/${target}.rs"`));
  }

  for (const name of Object.keys(WAL006_DIRECT_DEPENDENCIES)) {
    const escapedName = name.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
    const line = manifestText.match(new RegExp(`^${escapedName}\\s*=\\s*\\{[^\\n]+\\}$`, 'm'));
    assert.ok(line, `manifest omits direct dependency ${name}`);
    const enabledDefaults = line[0].replace('default-features = false', 'default-features = true');
    assert.notStrictEqual(enabledDefaults, line[0]);
    assertRejects(
      () => policy.checkWalletBrokerManifest(replaceOnce(manifestText, line[0], enabledDefaults), {
        requireLibrary: true, requireLockfile: false,
      }),
      /WAL-006|Zcash|dependency|default|manifest/i
    );
  }

  for (const [from, to] of [
    ['version = "=0.24.0"', 'version = "0.24"'],
    ['version = "=0.22.0"', 'version = "0.22"'],
    ['version = "=0.9.3"', 'version = "0.9"'],
    ['version = "=0.30.1"', 'version = "0.30"'],
    ['version = "=0.10.5"', 'version = "0.10"'],
    ['version = "=0.16.1"', 'version = "0.16"'],
    ['features = ["pczt"]', 'features = ["pczt", "sync"]'],
    [
      'features = ["orchard", "test-dependencies", "transparent-inputs"]',
      'features = ["orchard", "test-dependencies", "transparent-inputs", "zewif"]',
    ],
    [
      'pczt = { version = "=0.9.3", default-features = false }',
      'pczt = { version = "=0.9.3", default-features = false, features = ["signer"] }',
    ],
    [
      'zcash_primitives = { version = "=0.30.1", default-features = false }',
      'zcash_primitives = { version = "=0.30.1", default-features = false, features = ["non-standard-fees"] }',
    ],
    ['features = ["local-consensus"]', 'features = ["local-consensus", "zcashd-compat"]'],
    ['features = ["orchard"]', 'features = ["orchard", "zcashd-compat"]'],
  ]) {
    assertRejects(
      () => policy.checkWalletBrokerManifest(replaceOnce(manifestText, from, to), {
        requireLibrary: true, requireLockfile: false,
      }),
      /WAL-006|Zcash|dependency|feature|pin|git|default|network|manifest/i
    );
  }
  const manifestPatch = `${manifestText}\n[patch.crates-io]\npczt = { git = "https://example.invalid/pczt" }\n`;
  assertRejects(
    () => policy.checkWalletBrokerManifest(manifestPatch, {
      requireLibrary: true, requireLockfile: false,
    }),
    /WAL-006|Zcash|patch|dependency|manifest/i
  );
  const gitDependency = replaceOnce(
    manifestText,
    'zcash_client_sqlite = { version = "=0.22.0", default-features = false, features = ["orchard", "test-dependencies", "transparent-inputs"] }',
    'zcash_client_sqlite = { git = "https://example.invalid/librustzcash", default-features = false, features = ["orchard", "test-dependencies", "transparent-inputs"] }'
  );
  assertRejects(
    () => policy.checkWalletBrokerManifest(gitDependency, {
      requireLibrary: true, requireLockfile: false,
    }),
    /WAL-006|Zcash|git|dependency|manifest/i
  );
  for (const addition of [
    'reqwest = "=0.13.0"',
    'tokio = { version = "=1.0.0", features = ["net"] }',
    'openssl = "=0.10.0"',
  ]) {
    const mutated = replaceOnce(
      manifestText,
      '[dependencies]\n',
      `[dependencies]\n${addition}\n`
    );
    assertRejects(
      () => policy.checkWalletBrokerManifest(mutated, {
        requireLibrary: true, requireLockfile: false,
      }),
      /WAL-006|Zcash|patch|network|transport|OpenSSL|dependency|manifest/i
    );
  }
});

test('WAL-006 feature policy distinguishes compiled upstream PCZT capability from BitBook authority', () => {
  const policy = loadPolicy();
  assert.deepStrictEqual(policy.WAL006_FORBIDDEN_FEATURES, WAL006_FORBIDDEN_FEATURES);
  assert.deepStrictEqual(
    policy.WAL006_EXPECTED_COMPILED_PCZT_CAPABILITIES,
    WAL006_EXPECTED_COMPILED_PCZT_CAPABILITIES
  );
  assert.strictEqual(typeof policy.checkWal006ResolvedFeatures, 'function');
  policy.checkWal006ResolvedFeatures({
    direct: WAL006_DIRECT_DEPENDENCIES,
    compiled_pczt_capabilities: WAL006_EXPECTED_COMPILED_PCZT_CAPABILITIES,
    bitbook_authority: ['receiver.fresh', 'fixture.scan', 'pczt.prepare'],
  });
  for (const forbidden of WAL006_FORBIDDEN_FEATURES) {
    assertRejects(
      () => policy.checkWal006ResolvedFeatures({
        direct: WAL006_DIRECT_DEPENDENCIES,
        enabled_features: [forbidden],
        compiled_pczt_capabilities: WAL006_EXPECTED_COMPILED_PCZT_CAPABILITIES,
        bitbook_authority: ['receiver.fresh', 'fixture.scan', 'pczt.prepare'],
      }),
      /WAL-006|Zcash|feature|network|forbidden/i
    );
  }
  for (const authority of ['pczt.raw', 'sign', 'prove', 'finalize', 'extract', 'broadcast', 'network.connect']) {
    assertRejects(
      () => policy.checkWal006ResolvedFeatures({
        direct: WAL006_DIRECT_DEPENDENCIES,
        compiled_pczt_capabilities: WAL006_EXPECTED_COMPILED_PCZT_CAPABILITIES,
        bitbook_authority: ['receiver.fresh', 'fixture.scan', 'pczt.prepare', authority],
      }),
      /WAL-006|Zcash|authority|sign|prove|final|extract|broadcast|network|raw/i
    );
  }
});

test('WAL-006 Rust ZEC product source inventory remains empty during test-only Phase A', () => {
  const policy = loadPolicy();
  assert.deepStrictEqual(policy.WAL006_ALLOWED_RUST_SOURCE_PATHS, WAL006_ALLOWED_RUST_SOURCE_PATHS);
  assert.strictEqual(typeof policy.checkWal006RustSourceInventory, 'function');
  const sourceRoot = path.join(repoRoot, 'wallet-broker', 'src');
  const rustSources = [];
  const collectRustSources = (directory, relative) => {
    for (const entry of fs.readdirSync(directory, { withFileTypes: true })) {
      const childRelative = path.posix.join(relative, entry.name);
      if (entry.isDirectory()) {
        collectRustSources(path.join(directory, entry.name), childRelative);
      } else if (entry.isFile() && entry.name.endsWith('.rs')) {
        rustSources.push(`wallet-broker/src/${childRelative}`);
      }
    }
  };
  collectRustSources(sourceRoot, '');
  const actual = rustSources.filter((relative) => /^wallet-broker\/src\/zec(?:[_.\/])/.test(relative));
  assert.deepStrictEqual(actual, []);
  policy.checkWal006RustSourceInventory(actual);
  for (const unlisted of [
    'wallet-broker/src/zec.rs',
    'wallet-broker/src/zec_network.rs',
    'wallet-broker/src/zec/raw.rs',
  ]) {
    assertRejects(
      () => policy.checkWal006RustSourceInventory([unlisted]),
      /WAL-006|Zcash|source|inventory|unlisted|extra/i
    );
  }
});

test('WAL-006 policy rejects live-network and authority-bearing Rust snippets without denying upstream transitives', () => {
  const policy = loadPolicy();
  const allowed = [
    'use zcash_keys::keys::UnifiedAddressRequest;',
    'let request = UnifiedAddressRequest::ORCHARD;',
    'let _prepared_handle = prepare_unsigned_ironwood_pczt(request);',
  ].join('\n');
  policy.checkRustWalletSource(allowed, 'wallet-broker/tests/zec_prepare.rs');
  for (const source of [
    'use std::net::TcpStream;',
    'use zcash_client_backend::proto::service::compact_tx_streamer_client;',
    'let endpoint = "https://lightwalletd.example";',
    'pczt.sign(spending_key);',
    'pczt.prove(proving_key);',
    'pczt.finalize();',
    'pczt.extract();',
    'broadcast(raw_transaction);',
    'Network::MainNetwork',
  ]) {
    assertRejects(
      () => policy.checkRustWalletSource(source, 'wallet-broker/src/zec.rs'),
      /WAL-006|Zcash|network|endpoint|sign|prove|final|extract|broadcast|mainnet|authority/i
    );
  }
  assert.deepStrictEqual(
    WAL006_EXPECTED_COMPILED_PCZT_CAPABILITIES,
    [
      'io-finalizer',
      'orchard',
      'prover',
      'sapling',
      'signer',
      'spend-finalizer',
      'transparent',
      'transparent-inputs',
      'tx-extractor',
      'zcp-builder',
    ]
  );
});

function run() {
  let failed = 0;
  for (const { name, fn } of tests) {
    try {
      fn();
      process.stdout.write(`ok ${name}\n`);
    } catch (err) {
      failed += 1;
      process.stderr.write(`not ok ${name}\n${err && err.stack ? err.stack : err}\n`);
    }
  }
  if (failed) {
    process.stderr.write(`${failed} security policy test(s) failed\n`);
    process.exit(1);
  }
  process.stdout.write(`BitBook security policy tests passed (${tests.length}).\n`);
}

if (require.main === module) {
  run();
}

module.exports = {
  tests,
  SOCIAL_PATHS,
  SECURITY_PATHS,
};
