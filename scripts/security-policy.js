'use strict';

const fs = require('fs');
const path = require('path');

class PolicyError extends Error {
  constructor(message) {
    super(message);
    this.name = 'PolicyError';
  }
}

class YAMLParseError extends Error {
  constructor(message) {
    super(message);
    this.name = 'YAMLParseError';
  }
}

const CHECKOUT_ACTION = 'actions/checkout';
const CHECKOUT_SHA = '3d3c42e5aac5ba805825da76410c181273ba90b1';
const CHECKOUT_TAG = 'v7.0.1';
const SETUP_NODE_ACTION = 'actions/setup-node';
const SETUP_NODE_SHA = '48b55a011bda9f5d6aeb4c2d9c7362e8dae4041e';
const SETUP_NODE_TAG = 'v6.4.0';
const UPLOAD_ARTIFACT_ACTION = 'actions/upload-artifact';
const UPLOAD_ARTIFACT_SHA = '043fb46d1a93c77aae656e7c1c64a875d1fc6a0a';
const UPLOAD_ARTIFACT_TAG = 'v7.0.1';
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
const GITLEAKS_RATCHET_FINGERPRINTS = Object.freeze([
  '7f6a71d6d5ec94b0d8ed02a23eddd7d1bfeaf802:index.html:generic-api-key:57',
  '988fcc3da2d2b13689fdd98e936df14e2f989709:js/models/order/Case.js:generic-api-key:107',
  'b0637a03e1eb12e4e5d49c9dfba92dcbf51a0d8c:js/utils/feedback.js:generic-api-key:8',
  'bfd12cbe6e1f586af1f728c6d4e1ba68b8d9d103:js/utils/metrics.js:generic-api-key:13',
  'd38fc4819f1aa16f692394c56acc90db5d4f973a:js/views/modals/wallet/ReceiveMoney.js:generic-api-key:65',
  'e30e2ebbe6cc6198ca3c507167d26ff934ef9deb:js/views/modals/wallet/ReceiveMoney.js:generic-api-key:65',
  'f527597842b38bbe25c36cb42d204f16747e7e72:js/start.js:generic-api-key:409',
  'f83f40146c4bd2eb6da9f7fdd7a8eab8fb660b13:js/views/modals/wallet/ReceiveMoney.js:generic-api-key:63',
]);
const GITLEAKS_RATCHET_BODY = `${GITLEAKS_RATCHET_FINGERPRINTS.join('\n')}\n`;
const GITLEAKS_COMMIT_FINGERPRINT_RE = /^[0-9a-f]{40}:[^:\n]+:[A-Za-z0-9_-]+:\d+$/;
const GITLEAKS_CURRENT_TREE_FINGERPRINT_RE = /^[^:\n]+:[A-Za-z0-9_-]+:\d+$/;
const APPROVED_GITLEAKS_IGNORE_PATH_LINE = /^[ \t]*-[ \t]+"\.gitleaksignore"[ \t]*$/gm;
const METRICS_PUBLIC_EXPORTS = Object.freeze([
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
]);
const GITLEAKS_INSTALL_SCRIPT = [
  'archive="${RUNNER_TEMP}/gitleaks_8.30.1_linux_x64.tar.gz"',
  `curl --fail --silent --show-error --location --output "\${archive}" "${GITLEAKS_ARCHIVE_URL}"`,
  `test "$(stat --format=%s "\${archive}")" -eq ${GITLEAKS_ARCHIVE_BYTES}`,
  `echo "${GITLEAKS_ARCHIVE_SHA256}  \${archive}" | sha256sum --check --strict`,
  'tar --extract --gzip --file "${archive}" --directory "${RUNNER_TEMP}" gitleaks',
  'echo "${RUNNER_TEMP}" >> "${GITHUB_PATH}"',
].join('\n');
const NODE_VERSION = '24';
const ELECTRON_VERSION = '44.0.0';
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

const FORBIDDEN_DOC_PATHS = new Set([
  '**',
  '**/*',
  'docs/**',
  '**/*.md',
  '*.md',
  'README.md',
]);

const ELECTRON_TEST_CMD = 'node test/electronSecurity.node.js';
const POLICY_TEST_CMD = 'node test/securityPolicy.node.js';
const POLICY_CHECK_CMD = 'node scripts/security-policy.js';
const AUDIT_CMD = 'npm audit --audit-level=low';
const BUILD_CMD = 'npm run build';
const SOCIAL_TEST_CMD = 'npm run test:social';
const SECURITY_TEST_CMD = 'npm run test:security';
const NPM_CI_CMD = 'npm ci';
const MANUAL_IF = "github.event_name == 'workflow_dispatch'";
const TEST_SECURITY_SCRIPT = 'node test/electronSecurity.node.js && node test/securityPolicy.node.js';

const USES_RE = /^\s*(?:-\s+)?uses:\s+(\S+)(?:\s+#\s*(\S+))?\s*$/;
const SHA_RE = /^[0-9a-f]{40}$/;
const PACKAGE_RE = /\b(npm\s+run\s+package(?::\S*)?|scripts\/build-deb\.sh|scripts\/build-macos\.sh|scripts\/build-windows\.ps1|dpkg-deb|electron-builder|electron-packager)\b/;
const SUPPRESSION_PATTERNS = [
  [/continue-on-error\s*:/i, 'continue-on-error'],
  [/\|\|\s*true\b/, '|| true'],
  [/\bset\s+\+e\b/, 'set +e'],
  [/--exit-code\s*=?\s*0/, '--exit-code 0'],
  [/--no-fail\b/, '--no-fail'],
  [/\ballowlist\b/i, 'allowlist'],
  [/\.gitleaksignore\b/, '.gitleaksignore'],
  [/GITLEAKS_CONFIG\s*:/, 'GITLEAKS_CONFIG'],
  [/\beng-disable\b/, 'eng-disable'],
  [/\s-x\s+/, '-x'],
  [/--exclude-checks\b/, '--exclude-checks'],
  [/--severity\b/, '--severity'],
  [/--confidence\b/, '--confidence'],
  [/--audit-level\s*=?\s*none/, 'npm audit none'],
  [/\bnpm\s+audit\b[^\n]*--ignore/, 'npm audit --ignore'],
];

const REQUIRED_FILES = [
  'scripts/security-policy.js',
  'scripts/validate-sbom.js',
  '.github/workflows/social.yml',
  '.github/workflows/security.yml',
  '.github/workflows/sbom.yml',
  'test/electronSecurity.node.js',
  'test/securityPolicy.node.js',
  '.gitleaksignore',
];

class Workflow {
  constructor(filePath, text, data) {
    this.path = filePath;
    this.text = text;
    this.data = data;
  }
}

class ActionUse {
  constructor(line, spec, action, ref, comment) {
    this.line = line;
    this.spec = spec;
    this.action = action;
    this.ref = ref;
    this.comment = comment;
  }
}

class Parser {
  constructor(text) {
    if (text.charCodeAt(0) === 0xfeff) {
      text = text.slice(1);
    }
    this.lines = text.split('\n');
    this.i = 0;
  }

  error(message) {
    throw new YAMLParseError(`${message} at line ${this.i + 1}`);
  }

  skipNoise() {
    while (this.i < this.lines.length) {
      const stripped = this.lines[this.i].trim();
      if (stripped === '' || stripped.startsWith('#') || stripped === '---') {
        this.i += 1;
        continue;
      }
      return;
    }
  }

  peek() {
    this.skipNoise();
    if (this.i >= this.lines.length) {
      return null;
    }
    return this.lines[this.i];
  }

  pop() {
    const line = this.peek();
    if (line === null) {
      this.error('unexpected end of YAML');
    }
    this.i += 1;
    return line;
  }

  parseDocument() {
    this.skipNoise();
    if (this.peek() === null) {
      return {};
    }
    const value = this.parseValue(indentOf(this.peek()));
    this.skipNoise();
    if (this.i < this.lines.length) {
      this.error('trailing YAML content');
    }
    return value;
  }

  parseValue(indent) {
    const line = this.peek();
    if (line === null) {
      return null;
    }
    const actual = indentOf(line);
    if (actual < indent) {
      return null;
    }
    const content = stripComment(line.slice(actual));
    if (content.startsWith('- ') || content === '-') {
      return this.parseSequence(actual);
    }
    return this.parseMapping(actual);
  }

  parseMapping(indent, firstContent) {
    const mapping = {};
    if (firstContent !== undefined) {
      this.addMappingEntry(mapping, firstContent, indent);
    }
    while (true) {
      const line = this.peek();
      if (line === null) {
        return mapping;
      }
      const actual = indentOf(line);
      if (actual < indent) {
        return mapping;
      }
      if (actual > indent) {
        this.error('unexpected indent');
      }
      const content = stripComment(line.slice(actual));
      if (content.startsWith('- ') || content === '-') {
        return mapping;
      }
      this.pop();
      this.addMappingEntry(mapping, content, indent);
    }
  }

  addMappingEntry(mapping, content, indent) {
    const { key, value } = splitKeyValue(content);
    if (Object.prototype.hasOwnProperty.call(mapping, key)) {
      this.error(`duplicate key ${JSON.stringify(key)}`);
    }
    mapping[key] = this.parseEntryValue(value, indent);
  }

  parseEntryValue(valueText, indent) {
    if (valueText === null || valueText === '') {
      const nxt = this.peek();
      if (nxt !== null && indentOf(nxt) > indent) {
        return this.parseValue(indentOf(nxt));
      }
      return null;
    }
    if (isBlockScalar(valueText)) {
      return this.parseBlockScalar(indent, valueText);
    }
    if (valueText.startsWith('[') || valueText.startsWith('{')) {
      return parseFlow(valueText);
    }
    return parseScalar(valueText);
  }

  parseSequence(indent) {
    const sequence = [];
    while (true) {
      const line = this.peek();
      if (line === null) {
        return sequence;
      }
      const actual = indentOf(line);
      if (actual !== indent) {
        return sequence;
      }
      const content = stripComment(line.slice(actual));
      if (!(content.startsWith('- ') || content === '-')) {
        return sequence;
      }
      this.pop();
      const item = content === '-' ? '' : content.slice(2);
      if (item === '') {
        const nxt = this.peek();
        if (nxt !== null && indentOf(nxt) > indent) {
          sequence.push(this.parseValue(indentOf(nxt)));
        } else {
          sequence.push(null);
        }
        continue;
      }
      if (looksLikeMappingEntry(item)) {
        sequence.push(this.parseMapping(indent + 2, item));
        continue;
      }
      if (isBlockScalar(item)) {
        sequence.push(this.parseBlockScalar(indent, item));
        continue;
      }
      if (item.startsWith('[') || item.startsWith('{')) {
        sequence.push(parseFlow(item));
        continue;
      }
      sequence.push(parseScalar(item));
    }
  }

  parseBlockScalar(parentIndent, indicator) {
    const collected = [];
    let contentIndent = null;
    while (this.i < this.lines.length) {
      const line = this.lines[this.i];
      if (line.trim() === '') {
        collected.push('');
        this.i += 1;
        continue;
      }
      const actual = indentOf(line);
      if (actual <= parentIndent) {
        break;
      }
      if (contentIndent === null) {
        contentIndent = actual;
      }
      if (actual < contentIndent) {
        break;
      }
      collected.push(line.slice(contentIndent));
      this.i += 1;
    }
    while (collected.length && collected[collected.length - 1] === '') {
      collected.pop();
    }
    if (indicator.startsWith('>')) {
      return collected.filter((item) => item !== '').join(' ');
    }
    return collected.join('\n');
  }
}

function indentOf(line) {
  let count = 0;
  for (const char of line) {
    if (char === ' ') {
      count += 1;
      continue;
    }
    if (char === '\t') {
      throw new YAMLParseError('tabs are not allowed in workflow YAML');
    }
    break;
  }
  return count;
}

function stripComment(text) {
  let inSingle = false;
  let inDouble = false;
  let inExpr = false;
  let index = 0;
  while (index < text.length) {
    const char = text[index];
    if (inExpr) {
      if (text.startsWith('}}', index)) {
        inExpr = false;
        index += 2;
        continue;
      }
      index += 1;
      continue;
    }
    if (!inSingle && !inDouble && text.startsWith('${{', index)) {
      inExpr = true;
      index += 3;
      continue;
    }
    if (inSingle) {
      if (char === "'") {
        inSingle = false;
      }
      index += 1;
      continue;
    }
    if (inDouble) {
      if (char === '\\' && index + 1 < text.length) {
        index += 2;
        continue;
      }
      if (char === '"') {
        inDouble = false;
      }
      index += 1;
      continue;
    }
    if (char === "'") {
      inSingle = true;
      index += 1;
      continue;
    }
    if (char === '"') {
      inDouble = true;
      index += 1;
      continue;
    }
    if (char === '#') {
      return text.slice(0, index).trimEnd();
    }
    index += 1;
  }
  return text.trimEnd();
}

function splitKeyValue(content) {
  if (content.includes(': ')) {
    const [key, ...rest] = content.split(': ');
    return { key: unquoteKey(key), value: rest.join(': ') };
  }
  if (content.endsWith(':')) {
    return { key: unquoteKey(content.slice(0, -1)), value: '' };
  }
  throw new YAMLParseError(`expected key: value in ${JSON.stringify(content)}`);
}

function unquoteKey(key) {
  key = key.trim();
  if (key.length >= 2 && key[0] === key[key.length - 1] && (key[0] === '"' || key[0] === "'")) {
    return parseScalar(key);
  }
  return key;
}

function looksLikeMappingEntry(content) {
  content = content.trim();
  if (!content || '[{|>'.includes(content[0])) {
    return false;
  }
  return content.includes(': ') || content.endsWith(':');
}

function isBlockScalar(value) {
  return value === '|' || value === '|-' || value === '|+' || value === '>' || value === '>-' || value === '>+';
}

function decodeDoubleQuoted(text) {
  const out = [];
  const escapes = { n: '\n', t: '\t', r: '\r', '"': '"', '\\': '\\' };
  let index = 0;
  while (index < text.length) {
    const char = text[index];
    if (char === '\\' && index + 1 < text.length) {
      out.push(escapes[text[index + 1]] || text[index + 1]);
      index += 2;
      continue;
    }
    out.push(char);
    index += 1;
  }
  return out.join('');
}

function parseScalar(text) {
  text = text.trim();
  if (text === '') {
    return null;
  }
  if (text.length >= 2 && text[0] === '"' && text[text.length - 1] === '"') {
    return decodeDoubleQuoted(text.slice(1, -1));
  }
  if (text.length >= 2 && text[0] === "'" && text[text.length - 1] === "'") {
    return text.slice(1, -1).replace(/''/g, "'");
  }
  if (text === 'true' || text === 'True' || text === 'TRUE') {
    return true;
  }
  if (text === 'false' || text === 'False' || text === 'FALSE') {
    return false;
  }
  if (text === 'null' || text === 'Null' || text === 'NULL' || text === '~') {
    return null;
  }
  if (/^-?\d+$/.test(text)) {
    return Number.parseInt(text, 10);
  }
  if (/^-?\d+\.\d+$/.test(text)) {
    return Number.parseFloat(text);
  }
  return text;
}

function splitFlowItems(text) {
  const items = [];
  let current = [];
  let depth = 0;
  let inSingle = false;
  let inDouble = false;
  for (const char of text) {
    if (inSingle) {
      current.push(char);
      if (char === "'") {
        inSingle = false;
      }
      continue;
    }
    if (inDouble) {
      current.push(char);
      if (char === '"') {
        inDouble = false;
      }
      continue;
    }
    if (char === "'") {
      inSingle = true;
      current.push(char);
      continue;
    }
    if (char === '"') {
      inDouble = true;
      current.push(char);
      continue;
    }
    if (char === '[' || char === '{') {
      depth += 1;
      current.push(char);
      continue;
    }
    if (char === ']' || char === '}') {
      depth -= 1;
      current.push(char);
      continue;
    }
    if (char === ',' && depth === 0) {
      items.push(current.join('').trim());
      current = [];
      continue;
    }
    current.push(char);
  }
  if (current.length) {
    items.push(current.join('').trim());
  }
  return items;
}

function parseFlow(text) {
  text = text.trim();
  if (text.startsWith('[') && text.endsWith(']')) {
    const inner = text.slice(1, -1).trim();
    if (inner === '') {
      return [];
    }
    return splitFlowItems(inner).map((part) => parseScalar(part.trim()));
  }
  if (text.startsWith('{') && text.endsWith('}')) {
    const inner = text.slice(1, -1).trim();
    if (inner === '') {
      return {};
    }
    const mapping = {};
    for (const part of splitFlowItems(inner)) {
      const { key, value } = splitKeyValue(part.trim());
      mapping[key] = value === '' ? null : parseScalar(value);
    }
    return mapping;
  }
  throw new YAMLParseError(`unsupported flow YAML ${JSON.stringify(text)}`);
}

function parseYaml(text) {
  return new Parser(text).parseDocument();
}

function loadWorkflow(filePath) {
  const resolved = path.resolve(filePath);
  if (!fs.existsSync(resolved) || !fs.statSync(resolved).isFile()) {
    throw new PolicyError(`missing workflow ${resolved}`);
  }
  const text = fs.readFileSync(resolved, 'utf8');
  let data;
  try {
    data = parseYaml(text);
  } catch (err) {
    throw new PolicyError(`unable to parse ${resolved}: ${err.message}`);
  }
  if (!data || typeof data !== 'object' || Array.isArray(data)) {
    throw new PolicyError(`${resolved} is not a mapping`);
  }
  return new Workflow(resolved, text, data);
}

function eventTriggers(data) {
  if (!data || typeof data !== 'object' || !Object.prototype.hasOwnProperty.call(data, 'on')) {
    throw new PolicyError('workflow is missing on:');
  }
  const on = data.on;
  if (typeof on === 'boolean') {
    throw new PolicyError('workflow on: was parsed as a boolean');
  }
  if (typeof on === 'string') {
    return { [on]: null };
  }
  if (Array.isArray(on)) {
    const result = {};
    for (const item of on) {
      result[String(item)] = null;
    }
    return result;
  }
  if (on && typeof on === 'object') {
    return on;
  }
  throw new PolicyError('workflow on: must be a mapping');
}

function triggerPaths(data, event) {
  const triggers = eventTriggers(data);
  if (!Object.prototype.hasOwnProperty.call(triggers, event)) {
    throw new PolicyError(`missing ${event} trigger`);
  }
  const body = triggers[event];
  if (!body || typeof body !== 'object' || Array.isArray(body)) {
    throw new PolicyError(`${event} trigger is missing a path filter`);
  }
  if (Object.prototype.hasOwnProperty.call(body, 'paths-ignore')) {
    throw new PolicyError(`${event} uses paths-ignore; inclusive paths are required`);
  }
  const paths = body.paths;
  if (!Array.isArray(paths) || paths.length === 0) {
    throw new PolicyError(`${event} trigger is missing paths`);
  }
  const normalized = [];
  for (const item of paths) {
    if (typeof item !== 'string' || !item) {
      throw new PolicyError(`${event} path filter entries must be strings`);
    }
    normalized.push(item);
  }
  return normalized;
}

function iterSteps(data) {
  const jobs = data.jobs;
  if (!jobs || typeof jobs !== 'object' || Array.isArray(jobs) || Object.keys(jobs).length === 0) {
    throw new PolicyError('workflow is missing jobs');
  }
  const steps = [];
  for (const [jobName, job] of Object.entries(jobs)) {
    if (!job || typeof job !== 'object' || Array.isArray(job)) {
      throw new PolicyError(`job ${jobName} is not a mapping`);
    }
    if (!Array.isArray(job.steps) || job.steps.length === 0) {
      throw new PolicyError(`job ${jobName} is missing steps`);
    }
    job.steps.forEach((step, index) => {
      if (!step || typeof step !== 'object' || Array.isArray(step)) {
        throw new PolicyError(`job ${jobName} step ${index} is not a mapping`);
      }
      steps.push([jobName, job, step]);
    });
  }
  return steps;
}

function stepRunText(step) {
  const run = step.run;
  if (run === undefined || run === null) {
    return '';
  }
  return String(run);
}

function stepRunLines(step) {
  const lines = [];
  for (const raw of stepRunText(step).split('\n')) {
    const line = raw.trim();
    if (!line || line.startsWith('#')) {
      continue;
    }
    lines.push(line);
  }
  return lines;
}

function iterActionUses(text) {
  const uses = [];
  text.split('\n').forEach((raw, index) => {
    const match = USES_RE.exec(raw);
    if (!match) {
      return;
    }
    const spec = match[1];
    let action;
    let ref;
    if (spec.includes('@')) {
      const parts = spec.split('@');
      ref = parts.pop();
      action = parts.join('@');
    } else {
      action = spec;
      ref = '';
    }
    uses.push(new ActionUse(index + 1, spec, action, ref, match[2] || ''));
  });
  return uses;
}

function sameSet(left, right) {
  const a = new Set(left);
  const b = new Set(right);
  if (a.size !== b.size) {
    return false;
  }
  for (const item of a) {
    if (!b.has(item)) {
      return false;
    }
  }
  return true;
}

function checkActionPins(text, requiredActions, allowUploadArtifact) {
  const uses = iterActionUses(text);
  if (!uses.length) {
    throw new PolicyError('workflow references no GitHub Actions');
  }
  const pinned = {
    [CHECKOUT_ACTION]: [CHECKOUT_SHA, CHECKOUT_TAG],
    [SETUP_NODE_ACTION]: [SETUP_NODE_SHA, SETUP_NODE_TAG],
  };
  if (allowUploadArtifact) {
    pinned[UPLOAD_ARTIFACT_ACTION] = [UPLOAD_ARTIFACT_SHA, UPLOAD_ARTIFACT_TAG];
  }
  const seen = new Set();
  for (const use of uses) {
    if (!SHA_RE.test(use.ref)) {
      throw new PolicyError(
        `line ${use.line}: ${use.action} is not pinned to a 40-character commit SHA (${use.ref})`
      );
    }
    if (!Object.prototype.hasOwnProperty.call(pinned, use.action)) {
      throw new PolicyError(`line ${use.line}: unapproved action ${use.action}`);
    }
    const [sha, tag] = pinned[use.action];
    if (use.ref !== sha) {
      throw new PolicyError(
        `line ${use.line}: ${use.action} must be pinned to ${sha} (${tag}), not ${use.ref}`
      );
    }
    if (use.comment !== tag) {
      throw new PolicyError(
        `line ${use.line}: ${use.action} must retain adjacent comment ${tag}`
      );
    }
    seen.add(use.action);
  }
  const missing = [...requiredActions].filter((action) => !seen.has(action));
  if (missing.length) {
    throw new PolicyError(`missing required actions: ${missing.sort().join(', ')}`);
  }
  const extra = [...seen].filter((action) => !requiredActions.has(action));
  if (extra.length) {
    throw new PolicyError(`unexpected actions: ${extra.sort().join(', ')}`);
  }
}

function checkReadOnlyPermissions(data, workflowName) {
  const permissions = data.permissions;
  if (!permissions || typeof permissions !== 'object' || Array.isArray(permissions)) {
    throw new PolicyError(
      `${workflowName} permissions must be exactly contents: read, not ${JSON.stringify(permissions)}`
    );
  }
  const keys = Object.keys(permissions);
  if (keys.length !== 1 || keys[0] !== 'contents' || permissions.contents !== 'read') {
    throw new PolicyError(
      `${workflowName} permissions must be exactly contents: read, not ${JSON.stringify(permissions)}`
    );
  }
  const jobs = data.jobs || {};
  for (const [jobName, job] of Object.entries(jobs)) {
    if (job && typeof job === 'object' && Object.prototype.hasOwnProperty.call(job, 'permissions')) {
      throw new PolicyError(`${workflowName} job ${jobName} must not override permissions`);
    }
  }
}

function checkSetupNode(data, workflowName) {
  let found = false;
  for (const [, , step] of iterSteps(data)) {
    const uses = String(step.uses || '');
    if (uses.split('@')[0] !== SETUP_NODE_ACTION) {
      continue;
    }
    found = true;
    const with_ = step.with || {};
    if (String(with_['node-version']) !== NODE_VERSION) {
      throw new PolicyError(`${workflowName} must pin Node ${NODE_VERSION}`);
    }
  }
  if (!found) {
    throw new PolicyError(`${workflowName} is missing actions/setup-node`);
  }
}

function checkCheckoutFetchDepth(data, workflowName, expected) {
  let found = false;
  for (const [, , step] of iterSteps(data)) {
    const uses = String(step.uses || '');
    if (uses.split('@')[0] !== CHECKOUT_ACTION) {
      continue;
    }
    found = true;
    const fetchDepth = (step.with || {})['fetch-depth'];
    if (fetchDepth !== expected) {
      throw new PolicyError(
        `${workflowName} checkout fetch-depth must be ${expected} for complete history`
      );
    }
  }
  if (!found) {
    throw new PolicyError(`${workflowName} is missing actions/checkout`);
  }
}

function hasCommand(data, command) {
  for (const [, , step] of iterSteps(data)) {
    if (stepRunLines(step).includes(command) || stepRunText(step).includes(command)) {
      return true;
    }
  }
  return false;
}

function requireCommand(data, command) {
  if (!hasCommand(data, command)) {
    throw new PolicyError(`missing command ${command}`);
  }
}

function jobRunText(job) {
  const parts = [];
  for (const step of job.steps || []) {
    parts.push(stepRunText(step));
  }
  return parts.join('\n');
}

function jobUsesUpload(job) {
  for (const step of job.steps || []) {
    const uses = String(step.uses || '');
    if (uses.split('@')[0] === UPLOAD_ARTIFACT_ACTION) {
      return true;
    }
  }
  return false;
}

function jobPackages(job) {
  return PACKAGE_RE.test(jobRunText(job));
}

function isManualJob(job) {
  return String(job.if || '') === MANUAL_IF;
}

function withoutApprovedGitleaksIgnorePathFilter(text) {
  return text.replace(APPROVED_GITLEAKS_IGNORE_PATH_LINE, '');
}

function checkNoSuppression(text, data, workflowName) {
  const screened = withoutApprovedGitleaksIgnorePathFilter(text);
  for (const [pattern, label] of SUPPRESSION_PATTERNS) {
    if (pattern.test(screened)) {
      throw new PolicyError(`${workflowName} suppresses findings via ${label}`);
    }
  }
  for (const [jobName, job, step] of iterSteps(data)) {
    if (Object.prototype.hasOwnProperty.call(job, 'continue-on-error') ||
        Object.prototype.hasOwnProperty.call(step, 'continue-on-error')) {
      throw new PolicyError(
        `${workflowName} job ${jobName} converts a finding to non-blocking`
      );
    }
  }
}

function checkPathFilter(paths, expected, workflowName, event) {
  if (paths.length !== expected.length || paths.some((item, index) => item !== expected[index])) {
    throw new PolicyError(`${workflowName} ${event} paths must match the ticketed filter`);
  }
  const forbidden = paths.filter((item) => FORBIDDEN_DOC_PATHS.has(item));
  if (forbidden.length) {
    throw new PolicyError(
      `${workflowName} ${event} paths include documentation globs ${forbidden.sort().join(', ')}`
    );
  }
}

function checkSocialWorkflow(text, data) {
  if (data === undefined) {
    data = parseYaml(text);
  }
  const name = 'social.yml';
  const triggers = eventTriggers(data);
  if (!sameSet(Object.keys(triggers), ['push', 'pull_request', 'workflow_dispatch'])) {
    throw new PolicyError(`${name} must trigger on push, pull_request, and workflow_dispatch`);
  }
  for (const event of ['push', 'pull_request']) {
    checkPathFilter(triggerPaths(data, event), SOCIAL_PATHS, name, event);
  }
  const dispatch = triggers.workflow_dispatch;
  if (dispatch !== null && !(dispatch && typeof dispatch === 'object' && !Array.isArray(dispatch) && Object.keys(dispatch).length === 0)) {
    if (dispatch !== null && dispatch !== undefined && Object.keys(dispatch || {}).length) {
      throw new PolicyError(`${name} workflow_dispatch must not add extra configuration`);
    }
  }
  checkReadOnlyPermissions(data, name);
  checkActionPins(
    text,
    new Set([CHECKOUT_ACTION, SETUP_NODE_ACTION, UPLOAD_ARTIFACT_ACTION]),
    true
  );
  checkSetupNode(data, name);
  checkNoSuppression(text, data, name);

  const jobs = data.jobs || {};
  if (!jobs.check) {
    throw new PolicyError(`${name} is missing the routine check job`);
  }
  if (jobs.check.if) {
    throw new PolicyError(`${name} check job must run on filtered push/PR events`);
  }

  const routineCommands = [];
  for (const [jobName, job] of Object.entries(jobs)) {
    const packages = jobPackages(job);
    const uploads = jobUsesUpload(job);
    if (!job.if) {
      for (const step of job.steps || []) {
        routineCommands.push(...stepRunLines(step));
      }
      if (packages) {
        throw new PolicyError(`${name} job ${jobName} must not perform native packaging`);
      }
      if (uploads) {
        throw new PolicyError(`${name} job ${jobName} must not upload artifacts`);
      }
      if (/\bnpm\s+ci\b/.test(jobRunText(job))) {
        throw new PolicyError(`${name} job ${jobName} must stay package-free`);
      }
      continue;
    }
    if (!isManualJob(job)) {
      throw new PolicyError(`${name} job ${jobName} must be manual-only via workflow_dispatch`);
    }
    if (!packages && !uploads && !/package:/.test(jobRunText(job))) {
      throw new PolicyError(`${name} manual job ${jobName} is missing native packaging`);
    }
  }

  for (const command of [BUILD_CMD, SOCIAL_TEST_CMD, SECURITY_TEST_CMD]) {
    if (!routineCommands.includes(command)) {
      throw new PolicyError(`${name} missing command ${command}`);
    }
  }
  if (routineCommands.some((line) => /electronegativity|cyclonedx|gitleaks/.test(line))) {
    throw new PolicyError(`${name} routine check must stay offline syntax and Node tests`);
  }
}

function collectGitleaksVersions(text) {
  const versions = [];
  const pattern = /(?:gitleaks[_-]|download\/v)(\d+\.\d+\.\d+)/g;
  let match;
  while ((match = pattern.exec(text)) !== null) {
    versions.push(match[1]);
  }
  return versions;
}

function gitleaksForbiddenReason(text) {
  const screened = withoutApprovedGitleaksIgnorePathFilter(text);
  const patterns = [
    [/gitleaks\/gitleaks-action/i, 'must not use Gitleaks Action'],
    [/\bGITHUB_TOKEN\b/, 'must not supply a Gitleaks token'],
    [/GITLEAKS_ENABLE_COMMENTS/, 'must not enable Gitleaks comments'],
    [/GITLEAKS_ENABLE_SUMMARY|GITHUB_STEP_SUMMARY/, 'must not enable Gitleaks comments or summaries'],
    [/GITLEAKS_ENABLE_UPLOAD_ARTIFACT/, 'must not upload a Gitleaks report artifact'],
    [/GITLEAKS_VERSION\s*:/, 'must not set a GITLEAKS_VERSION environment contract'],
    [/GITLEAKS_CONFIG|--config\b/, 'Gitleaks must not set a config'],
    [/--baseline(?:-path)?\b/, 'Gitleaks must not use a baseline'],
    [/\.gitleaksignore|--gitleaks-ignore-path|--ignore\b/, 'Gitleaks must not set an ignore'],
    [/--log-opts/, 'Gitleaks must not pass range or log opts'],
    [/--report-path/, 'Gitleaks must not write a report path'],
    [/--report-format/, 'Gitleaks must not write a report'],
    [/--exit-code/, 'Gitleaks must not alter exit behavior'],
  ];
  for (const [pattern, message] of patterns) {
    if (pattern.test(screened)) {
      return message;
    }
  }
  return null;
}

function checkGitleaks(data, text, workflowName) {
  checkCheckoutFetchDepth(data, workflowName, 0);

  const forbidden = gitleaksForbiddenReason(text);
  if (forbidden) {
    throw new PolicyError(`${workflowName} ${forbidden}`);
  }

  const steps = iterSteps(data).map(([, , step]) => step);
  const allRun = steps.map((step) => stepRunText(step)).join('\n');
  let installIndex = -1;
  for (let index = 0; index < steps.length; index += 1) {
    if (stepRunText(steps[index]) === GITLEAKS_INSTALL_SCRIPT) {
      installIndex = index;
      break;
    }
  }

  if (installIndex < 0) {
    if (/releases\/latest|\/latest\/download|gitleaks[_-]latest/i.test(allRun)) {
      throw new PolicyError(`${workflowName} must not use a mutable Gitleaks release name`);
    }
    const versions = collectGitleaksVersions(allRun);
    if (versions.some((version) => version !== GITLEAKS_VERSION)) {
      throw new PolicyError(`${workflowName} Gitleaks version must be ${GITLEAKS_VERSION}`);
    }
    if (!allRun.includes(GITLEAKS_ARCHIVE_URL)) {
      throw new PolicyError(
        `${workflowName} Gitleaks archive URL must be ${GITLEAKS_ARCHIVE_URL}`
      );
    }
    if (!allRun.includes(GITLEAKS_ARCHIVE_SHA256)) {
      throw new PolicyError(
        `${workflowName} Gitleaks archive SHA-256 must be ${GITLEAKS_ARCHIVE_SHA256}`
      );
    }
    if (!allRun.includes(String(GITLEAKS_ARCHIVE_BYTES))) {
      throw new PolicyError(
        `${workflowName} Gitleaks archive size must be ${GITLEAKS_ARCHIVE_BYTES}`
      );
    }
    if (/\brm\b|\bunlink\b|\bshred\b|--delete\b/.test(allRun)) {
      throw new PolicyError(`${workflowName} Gitleaks install must not clean or delete`);
    }
    if (!/tar --extract --gzip --file "\$\{archive\}" --directory "\$\{RUNNER_TEMP\}" gitleaks\s*$/m.test(allRun)) {
      throw new PolicyError(
        `${workflowName} must extract only gitleaks beneath \${RUNNER_TEMP}`
      );
    }
    throw new PolicyError(`${workflowName} is missing the pinned Gitleaks install`);
  }

  const gitScanIndex = installIndex + 1;
  const dirScanIndex = installIndex + 2;
  requireExactGitleaksStep(
    steps,
    gitScanIndex,
    GITLEAKS_SCAN_CMD,
    workflowName,
    'Gitleaks scan must immediately follow install',
    'Gitleaks scan must immediately follow install'
  );
  requireExactGitleaksStep(
    steps,
    dirScanIndex,
    GITLEAKS_DIR_SCAN_CMD,
    workflowName,
    'current-tree Gitleaks dir scan must immediately follow the complete-history scan',
    'current-tree Gitleaks dir scan must immediately follow the complete-history scan'
  );

  for (let index = 0; index < steps.length; index += 1) {
    if (index === installIndex || index === gitScanIndex || index === dirScanIndex) {
      continue;
    }
    if (/\bgitleaks\b/i.test(stepRunText(steps[index]))) {
      throw new PolicyError(`${workflowName} has extra Gitleaks behavior`);
    }
  }

  for (const step of [steps[installIndex], steps[gitScanIndex], steps[dirScanIndex]]) {
    const env = step.env;
    if (env && typeof env === 'object' && Object.keys(env).length) {
      throw new PolicyError(
        `${workflowName} Gitleaks steps must not set token, comment, or version environment contracts`
      );
    }
  }
}

function classifyForbiddenGitleaksScan(scanRun, workflowName) {
  if (/--log-opts/.test(scanRun)) {
    throw new PolicyError(`${workflowName} Gitleaks must not pass range or log opts`);
  }
  if (/--baseline/.test(scanRun)) {
    throw new PolicyError(`${workflowName} Gitleaks must not use a baseline`);
  }
  if (/--config|\.gitleaks\.toml/.test(scanRun)) {
    throw new PolicyError(`${workflowName} Gitleaks must not set a config`);
  }
  if (/--ignore|\.gitleaksignore/.test(scanRun)) {
    throw new PolicyError(`${workflowName} Gitleaks must not set an ignore`);
  }
  if (/--report-path|--report-format/.test(scanRun)) {
    throw new PolicyError(`${workflowName} Gitleaks must not write a report path`);
  }
  if (/\|\|\s*true|--exit-code/.test(scanRun)) {
    throw new PolicyError(`${workflowName} Gitleaks must not alter exit behavior`);
  }
}

function requireExactGitleaksStep(steps, index, expected, workflowName, missingMessage, laterMessage) {
  if (index >= steps.length) {
    throw new PolicyError(`${workflowName} ${missingMessage}`);
  }
  const scanRun = stepRunText(steps[index]);
  if (scanRun === expected) {
    return;
  }
  const later = steps.slice(index + 1).some((step) => stepRunText(step) === expected);
  if (later) {
    throw new PolicyError(`${workflowName} ${laterMessage}`);
  }
  classifyForbiddenGitleaksScan(scanRun, workflowName);
  throw new PolicyError(`${workflowName} Gitleaks scan must be exactly ${expected}`);
}

function parseCommitFingerprint(line) {
  const match = /^([0-9a-f]{40}):(.+):([A-Za-z0-9_-]+):(\d+)$/.exec(line);
  if (!match) {
    return null;
  }
  return {
    commit: match[1],
    filePath: match[2],
    rule: match[3],
    line: match[4],
  };
}

function isSecretBearingRatchetLine(line) {
  if (GITLEAKS_COMMIT_FINGERPRINT_RE.test(line) || GITLEAKS_CURRENT_TREE_FINGERPRINT_RE.test(line)) {
    return false;
  }
  if (/(?:secret|token|password|credential|app(?:key|_key)|api[_-]?key)\s*[:=]/i.test(line)) {
    return true;
  }
  if (/['"][A-Za-z0-9+/=_-]{16,}['"]/.test(line)) {
    return true;
  }
  return false;
}

function checkGitleaksRatchetBytes(raw) {
  if (!Buffer.isBuffer(raw)) {
    throw new PolicyError('.gitleaksignore must be read as exact bytes');
  }
  if (raw.length >= 3 && raw[0] === 0xef && raw[1] === 0xbb && raw[2] === 0xbf) {
    throw new PolicyError('.gitleaksignore must not include a BOM');
  }
  if (raw.includes(0x0d)) {
    throw new PolicyError('.gitleaksignore must not use CRLF');
  }
  const expected = Buffer.from(GITLEAKS_RATCHET_BODY, 'utf8');
  if (raw.equals(expected)) {
    return;
  }
  const text = raw.toString('utf8');
  if (!text.endsWith('\n')) {
    throw new PolicyError('.gitleaksignore must not contain trailing bytes');
  }
  const lines = text.slice(0, -1).split('\n');
  for (const line of lines) {
    if (line === '') {
      throw new PolicyError('.gitleaksignore must not contain blank lines');
    }
    if (line.startsWith('#') || line.includes('#')) {
      throw new PolicyError('.gitleaksignore must not contain comments');
    }
    if (line.includes('*') || line.includes('?') || line.includes('[')) {
      throw new PolicyError('.gitleaksignore must not contain wildcards');
    }
    if (isSecretBearingRatchetLine(line)) {
      throw new PolicyError('.gitleaksignore must not contain secret-bearing text');
    }
    if (GITLEAKS_CURRENT_TREE_FINGERPRINT_RE.test(line) && !GITLEAKS_COMMIT_FINGERPRINT_RE.test(line)) {
      throw new PolicyError('.gitleaksignore must not contain a global or current-tree fingerprint');
    }
    if (!GITLEAKS_COMMIT_FINGERPRINT_RE.test(line)) {
      throw new PolicyError('.gitleaksignore contains a malformed fingerprint');
    }
  }
  const unique = new Set(lines);
  if (unique.size !== lines.length) {
    throw new PolicyError('.gitleaksignore must not contain duplicate fingerprints');
  }
  if (lines.length < GITLEAKS_RATCHET_FINGERPRINTS.length) {
    throw new PolicyError('.gitleaksignore is missing an inherited commit fingerprint');
  }
  if (lines.length > GITLEAKS_RATCHET_FINGERPRINTS.length) {
    throw new PolicyError('.gitleaksignore contains extra fingerprints');
  }
  const expectedSet = new Set(GITLEAKS_RATCHET_FINGERPRINTS);
  const sorted = [...lines].sort();
  if (
    sorted.every((line, index) => line === GITLEAKS_RATCHET_FINGERPRINTS[index]) &&
    lines.some((line, index) => line !== GITLEAKS_RATCHET_FINGERPRINTS[index])
  ) {
    throw new PolicyError('.gitleaksignore fingerprints must be in lexical order');
  }
  for (let index = 0; index < lines.length; index += 1) {
    if (lines[index] === GITLEAKS_RATCHET_FINGERPRINTS[index]) {
      continue;
    }
    const actual = parseCommitFingerprint(lines[index]);
    const wanted = parseCommitFingerprint(GITLEAKS_RATCHET_FINGERPRINTS[index]);
    if (!expectedSet.has(lines[index]) && actual && wanted) {
      if (actual.commit !== wanted.commit) {
        throw new PolicyError('.gitleaksignore has a wrong commit fingerprint');
      }
      if (actual.filePath !== wanted.filePath) {
        throw new PolicyError('.gitleaksignore has a wrong path fingerprint');
      }
      if (actual.rule !== wanted.rule) {
        throw new PolicyError('.gitleaksignore has a wrong rule fingerprint');
      }
      if (actual.line !== wanted.line) {
        throw new PolicyError('.gitleaksignore has a wrong line fingerprint');
      }
    }
  }
  throw new PolicyError('.gitleaksignore must be the exact eight lexically sorted commit fingerprints');
}

function extractExportedFunction(source, name) {
  const start = source.indexOf(`export function ${name}(`);
  if (start < 0) {
    throw new PolicyError(`missing export function ${name}`);
  }
  const brace = source.indexOf('{', start);
  if (brace < 0) {
    throw new PolicyError(`missing body for export function ${name}`);
  }
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
  throw new PolicyError(`unterminated export function ${name}`);
}

function checkInheritedLoaderNeutralization(metricsSource, feedbackSource) {
  for (const name of METRICS_PUBLIC_EXPORTS) {
    if (
      !metricsSource.includes(`export const ${name}`) &&
      !metricsSource.includes(`export function ${name}(`)
    ) {
      throw new PolicyError(`metrics public export ${name} is missing`);
    }
  }
  const addMetrics = extractExportedFunction(metricsSource, 'addMetrics');
  if (/createElement|appendChild|Countly|script|onload|localStorage|app_key|https?:\/\//.test(addMetrics)) {
    throw new PolicyError('addMetrics must not retain a remote Countly loader');
  }
  if (addMetrics !== 'export function addMetrics() {\n}') {
    throw new PolicyError('addMetrics must be an immediate no-op');
  }
  if (/import\s/.test(feedbackSource) || /createElement|appendChild|doorbell|script|https?:\/\//i.test(feedbackSource)) {
    throw new PolicyError('addFeedback must not retain a doorbell loader or import');
  }
  if (feedbackSource !== 'export function addFeedback() {\n}\n') {
    throw new PolicyError('addFeedback must be an immediate no-op');
  }
}

function obsoleteElectronSastReason(text) {
  const patterns = [
    [/\.sarif\b|--sarif\b/i, 'must not emit obsolete Electron-SAST SARIF output'],
    [/\.csv\b|--csv\b/i, 'must not emit obsolete Electron-SAST CSV output'],
    [/\beng-disable\b/, 'must not use obsolete Electron-SAST eng-disable annotations'],
    [/--exclude-checks\b/, 'must not use obsolete Electron-SAST --exclude-checks'],
    [/\s-x\s+/, 'must not use obsolete Electron-SAST -x exclusions'],
    [/-i\s+\.(?:\s|$)/, 'must not use obsolete Electron-SAST inherited input .'],
    [/-i\s+main\.js\b/, 'must not use obsolete Electron-SAST inherited input main.js'],
    [/-i\s+js\//, 'must not use obsolete Electron-SAST inherited input js/'],
    [/electronegativity/i, 'must not reintroduce Electronegativity'],
    [/electro-?ng/i, 'must not reintroduce ElectroNG'],
    [/-i\s+social-main\.js\b/, 'must not use obsolete Electron-SAST input social-main.js'],
  ];
  for (const [pattern, message] of patterns) {
    if (pattern.test(text)) {
      return message;
    }
  }
  return null;
}

function checkObsoleteElectronSast(text, workflowName) {
  const reason = obsoleteElectronSastReason(text);
  if (reason) {
    throw new PolicyError(`${workflowName} ${reason}`);
  }
}

function checkNoPackageOrUpload(data, text, workflowName) {
  for (const use of iterActionUses(text)) {
    if (use.action === UPLOAD_ARTIFACT_ACTION) {
      throw new PolicyError(`${workflowName} must not upload artifacts`);
    }
  }
  for (const [jobName, job] of Object.entries(data.jobs || {})) {
    if (jobPackages(job)) {
      throw new PolicyError(`${workflowName} job ${jobName} must not perform native packaging`);
    }
  }
  if (/action-gh-release/.test(text)) {
    throw new PolicyError(`${workflowName} must not publish a release`);
  }
}

function checkSecurityWorkflow(text, data) {
  if (data === undefined) {
    data = parseYaml(text);
  }
  const name = 'security.yml';
  const triggers = eventTriggers(data);
  if (Object.prototype.hasOwnProperty.call(triggers, 'push')) {
    throw new PolicyError(`${name} must not use a push trigger`);
  }
  if (!sameSet(Object.keys(triggers), ['pull_request', 'workflow_dispatch'])) {
    throw new PolicyError(`${name} must trigger only on pull_request and workflow_dispatch`);
  }
  checkPathFilter(triggerPaths(data, 'pull_request'), SECURITY_PATHS, name, 'pull_request');
  const dispatch = triggers.workflow_dispatch;
  if (dispatch !== null && dispatch !== undefined && Object.keys(dispatch || {}).length) {
    throw new PolicyError(`${name} workflow_dispatch must not add extra configuration`);
  }
  checkReadOnlyPermissions(data, name);
  checkActionPins(
    text,
    new Set([CHECKOUT_ACTION, SETUP_NODE_ACTION]),
    false
  );
  checkSetupNode(data, name);
  checkObsoleteElectronSast(text, name);
  requireCommand(data, AUDIT_CMD);
  requireCommand(data, ELECTRON_TEST_CMD);
  requireCommand(data, POLICY_TEST_CMD);
  requireCommand(data, POLICY_CHECK_CMD);
  checkGitleaks(data, text, name);
  checkNoPackageOrUpload(data, text, name);
  checkNoSuppression(text, data, name);
}

function sbomUploads(data) {
  const uploads = [];
  for (const [, , step] of iterSteps(data)) {
    const uses = String(step.uses || '');
    if (uses.split('@')[0] === UPLOAD_ARTIFACT_ACTION) {
      uploads.push(step);
    }
  }
  return uploads;
}

function checkSbomWorkflow(text, data) {
  if (data === undefined) {
    data = parseYaml(text);
  }
  const name = 'sbom.yml';
  const triggers = eventTriggers(data);
  if (Object.prototype.hasOwnProperty.call(triggers, 'push')) {
    throw new PolicyError(`${name} must not use a push trigger`);
  }
  if (Object.prototype.hasOwnProperty.call(triggers, 'pull_request')) {
    throw new PolicyError(`${name} must not use a pull_request trigger`);
  }
  if (!sameSet(Object.keys(triggers), ['workflow_dispatch'])) {
    throw new PolicyError(`${name} must trigger only on workflow_dispatch`);
  }
  checkReadOnlyPermissions(data, name);
  checkActionPins(
    text,
    new Set([CHECKOUT_ACTION, SETUP_NODE_ACTION, UPLOAD_ARTIFACT_ACTION]),
    true
  );
  checkSetupNode(data, name);
  requireCommand(data, NPM_CI_CMD);
  requireCommand(data, AUDIT_CMD);
  const runText = iterSteps(data).map(([, , step]) => stepRunText(step)).join('\n');
  if (!runText.includes(CYCLONEDX_NPM)) {
    throw new PolicyError(`${name} must generate a CycloneDX document with ${CYCLONEDX_NPM}`);
  }
  if (!/--output-format\s+JSON\b/.test(runText) && !/--of\s+JSON\b/.test(runText)) {
    throw new PolicyError(`${name} must emit CycloneDX JSON`);
  }
  if (/--output-format\s+XML\b/.test(runText) || /\bSPDX\b/.test(runText)) {
    throw new PolicyError(`${name} must not emit XML or SPDX`);
  }
  if (!runText.includes('node scripts/validate-sbom.js')) {
    throw new PolicyError(`${name} must validate the CycloneDX document with validate-sbom.js`);
  }
  const uploads = sbomUploads(data);
  if (uploads.length !== 1) {
    throw new PolicyError(`${name} must upload exactly one artifact`);
  }
  const uploaded = uploads[0].with || {};
  const uploadPath = uploaded.path;
  if (typeof uploadPath !== 'string' || !uploadPath.endsWith('.cdx.json')) {
    throw new PolicyError(`${name} must upload only a .cdx.json JSON document, not ${JSON.stringify(uploadPath)}`);
  }
  if (uploadPath.includes('*') || uploadPath.includes('?') || uploadPath.includes('\n')) {
    throw new PolicyError(`${name} upload path must be a single CycloneDX JSON file`);
  }
  const retention = Number.parseInt(String(uploaded['retention-days']), 10);
  if (retention !== SBOM_RETENTION_DAYS) {
    throw new PolicyError(`${name} upload retention-days must be ${SBOM_RETENTION_DAYS}`);
  }
  checkNoPackageOrUploadExceptSbom(data, text, name);
  checkNoSuppression(text, data, name);
}

function checkNoPackageOrUploadExceptSbom(data, text, workflowName) {
  for (const [jobName, job] of Object.entries(data.jobs || {})) {
    if (jobPackages(job)) {
      throw new PolicyError(`${workflowName} job ${jobName} must not perform native packaging`);
    }
  }
  if (/action-gh-release/.test(text)) {
    throw new PolicyError(`${workflowName} must not publish a release`);
  }
}

function checkPackageJson(root) {
  const pkgPath = path.join(root, 'package.json');
  const pkg = JSON.parse(fs.readFileSync(pkgPath, 'utf8'));
  if (pkg.main !== 'social-main.js') {
    throw new PolicyError('package.json main must remain social-main.js');
  }
  const dev = pkg.devDependencies || {};
  if (dev.electron !== ELECTRON_VERSION) {
    throw new PolicyError(`package.json must pin electron ${ELECTRON_VERSION}`);
  }
  if (JSON.stringify(Object.keys(dev).sort()) !== JSON.stringify(['electron'])) {
    throw new PolicyError('package.json must not add dependencies');
  }
  if (pkg.dependencies && Object.keys(pkg.dependencies).length) {
    throw new PolicyError('package.json must not add runtime dependencies');
  }
  if (!pkg.scripts || pkg.scripts['test:security'] !== TEST_SECURITY_SCRIPT) {
    throw new PolicyError('package.json must expose test:security');
  }
  if (!pkg.scripts.start || !pkg.scripts.start.includes('--no-sandbox')) {
    throw new PolicyError('package.json must preserve the labeled dev-only --no-sandbox start workaround');
  }
}

function checkRepository(root) {
  root = path.resolve(root);
  for (const rel of REQUIRED_FILES) {
    if (!fs.existsSync(path.join(root, rel))) {
      throw new PolicyError(`missing ${rel}`);
    }
  }
  checkGitleaksRatchetBytes(fs.readFileSync(path.join(root, GITLEAKS_IGNORE_REL)));
  checkInheritedLoaderNeutralization(
    fs.readFileSync(path.join(root, 'js/utils/metrics.js'), 'utf8'),
    fs.readFileSync(path.join(root, 'js/utils/feedback.js'), 'utf8')
  );
  checkPackageJson(root);
  const social = loadWorkflow(path.join(root, '.github/workflows/social.yml'));
  const security = loadWorkflow(path.join(root, '.github/workflows/security.yml'));
  const sbom = loadWorkflow(path.join(root, '.github/workflows/sbom.yml'));
  checkSocialWorkflow(social.text, social.data);
  checkSecurityWorkflow(security.text, security.data);
  checkSbomWorkflow(sbom.text, sbom.data);
}

function main() {
  checkRepository(path.resolve(__dirname, '..'));
  process.stdout.write('BitBook desktop security policy checks passed.\n');
}

if (require.main === module) {
  try {
    main();
  } catch (err) {
    process.stderr.write(`${err && err.message ? err.message : err}\n`);
    process.exit(1);
  }
}

module.exports = {
  PolicyError,
  YAMLParseError,
  CHECKOUT_SHA,
  CHECKOUT_TAG,
  SETUP_NODE_SHA,
  SETUP_NODE_TAG,
  UPLOAD_ARTIFACT_SHA,
  UPLOAD_ARTIFACT_TAG,
  CYCLONEDX_NPM,
  GITLEAKS_VERSION,
  GITLEAKS_ARCHIVE_URL,
  GITLEAKS_ARCHIVE_SHA256,
  GITLEAKS_ARCHIVE_BYTES,
  GITLEAKS_SCAN_CMD,
  GITLEAKS_DIR_SCAN_CMD,
  GITLEAKS_IGNORE_REL,
  GITLEAKS_RATCHET_OWNER,
  GITLEAKS_RATCHET_RATIONALE,
  GITLEAKS_RATCHET_REMOVAL_CONDITION,
  GITLEAKS_RATCHET_FINGERPRINTS,
  GITLEAKS_RATCHET_BODY,
  NODE_VERSION,
  ELECTRON_VERSION,
  SBOM_RETENTION_DAYS,
  SOCIAL_PATHS,
  SECURITY_PATHS,
  parseYaml,
  loadWorkflow,
  eventTriggers,
  triggerPaths,
  iterSteps,
  stepRunText,
  stepRunLines,
  iterActionUses,
  checkSocialWorkflow,
  checkSecurityWorkflow,
  checkSbomWorkflow,
  checkRepository,
  checkGitleaksRatchetBytes,
  checkInheritedLoaderNeutralization,
};
