'use strict';

const assert = require('assert');
const fs = require('fs');
const path = require('path');

const repoRoot = path.resolve(__dirname, '..');
const policyPath = path.join(repoRoot, 'scripts', 'security-policy.js');
const sbomValidatorPath = path.join(repoRoot, 'scripts', 'validate-sbom.js');

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
  'eight inherited 2016–2018 upstream commit fingerprints only; current-tree copies are disabled and removed, never ignored';
const GITLEAKS_RATCHET_REMOVAL_CONDITION =
  'delete .gitleaksignore when a later authorized ticket removes the inherited OpenBazaar marketplace tree (js/, old root index.html, and its unused renderer entry)';
const GITLEAKS_RATCHET_FINGERPRINTS = [
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
  'scripts/build-deb.sh',
  'scripts/build-macos.sh',
  'scripts/build-windows.ps1',
  '.github/workflows/social.yml',
];

const SECURITY_PATHS = [
  'social-main.js',
  'social/**',
  'test/**',
  'scripts/security-policy.js',
  'scripts/validate-sbom.js',
  'package.json',
  'package-lock.json',
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
  '.github/workflows/social.yml',
  '.github/workflows/security.yml',
  '.github/workflows/sbom.yml',
  'test/electronSecurity.node.js',
  'test/securityPolicy.node.js',
  '.gitleaksignore',
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

test('strict eight-line inherited Gitleaks ratchet bytes and content are enforced', () => {
  const policy = loadPolicy();
  const lexical = [...GITLEAKS_RATCHET_FINGERPRINTS].sort();
  assert.deepStrictEqual(GITLEAKS_RATCHET_FINGERPRINTS, lexical);
  assert.strictEqual(GITLEAKS_RATCHET_FINGERPRINTS.length, 8);
  assert.strictEqual(policy.GITLEAKS_RATCHET_OWNER, GITLEAKS_RATCHET_OWNER);
  assert.strictEqual(policy.GITLEAKS_RATCHET_RATIONALE, GITLEAKS_RATCHET_RATIONALE);
  assert.strictEqual(policy.GITLEAKS_RATCHET_REMOVAL_CONDITION, GITLEAKS_RATCHET_REMOVAL_CONDITION);
  const ignorePath = path.join(repoRoot, GITLEAKS_IGNORE_REL);
  assert.ok(fs.existsSync(ignorePath), 'committed .gitleaksignore is missing');
  const committed = fs.readFileSync(ignorePath);
  assert.deepStrictEqual(committed, Buffer.from(GITLEAKS_RATCHET_BODY, 'utf8'));
  policy.checkGitleaksRatchetBytes(committed);
  policy.checkRepository(repoRoot);

  const missing = GITLEAKS_RATCHET_FINGERPRINTS.slice(0, 7);
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
  wrongPath[0] = wrongPath[0].replace('index.html', 'index.htm');
  assertRejects(() => policy.checkGitleaksRatchetBytes(ratchetBytes(wrongPath)), /wrong path|path/i);

  const wrongRule = GITLEAKS_RATCHET_FINGERPRINTS.slice();
  wrongRule[0] = wrongRule[0].replace('generic-api-key', 'generic-api-token');
  assertRejects(() => policy.checkGitleaksRatchetBytes(ratchetBytes(wrongRule)), /wrong rule|rule/i);

  const wrongLine = GITLEAKS_RATCHET_FINGERPRINTS.slice();
  wrongLine[0] = wrongLine[0].replace(/:57$/, ':1');
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
  wildcard[0] = wildcard[0].replace('index.html', '*');
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
const TOP_LEVEL_TEST_CMD = 'npm run test:social && npm run test:security && npm run test:wallet';
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
