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
  'eight inherited 2016–2018 upstream fingerprints plus one reviewer-published WAL-004 synthetic-vector assignment fingerprint; current-tree copies are removed, never ignored';
const GITLEAKS_RATCHET_REMOVAL_CONDITION =
  'remove an exact fingerprint only when an authorized history rewrite makes its commit unreachable; current-tree triggers remain relabeled, never ignored';
const GITLEAKS_RATCHET_FINGERPRINTS = Object.freeze([
  '12a493196bb4304750e4ae44484a7fa604b82ce4:tickets/BBD-WAL-004.md:generic-api-key:110',
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
  'scripts/validate-rust-sbom.js',
  'scripts/build-deb.sh',
  'scripts/build-macos.sh',
  'scripts/build-windows.ps1',
  'wallet-broker/**',
  'wallet-pay/**',
  'quote-worker/**',
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
  'wallet-pay/**',
  'quote-worker/**',
  'deny.toml',
  '.github/workflows/**',
  '.gitleaksignore',
  'js/utils/metrics.js',
  'js/utils/feedback.js',
];

const WALLET_TEST_SCRIPT = 'test:wallet';
const WALLET_TEST_CMD = 'node test/walletContract.node.js';
const WALLET_CI_CMD = 'npm run test:wallet';
const TOP_LEVEL_TEST_CMD = 'npm run test:social && npm run test:security && npm run test:wallet && npm run test:wallet-broker && npm run test:wallet-pay && npm run test:rate';
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
const PAY_MODEL_PATHS = ['wallet-pay/model.js'];
const PAY_TEST_SCRIPT = 'test:wallet-pay';
const PAY_TEST_COMMAND = 'node test/walletPay.node.js';
const PAY_CI_COMMAND = `npm run ${PAY_TEST_SCRIPT}`;
const PAY_BUILD_COMMANDS = PAY_MODEL_PATHS.map((rel) => `node --check ${rel}`);
const PAY_SOURCE_FILTER = 'wallet-pay/**';
const PAY_IMPORT_ALLOWLIST = [];
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
    '../wallet-pay/model', '../wallet-pay/model.js',
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
const QUOTE_WORKER_PATHS = [
  'quote-worker/providers.js',
  'quote-worker/model.js',
  'quote-worker/framing.js',
  'quote-worker/worker.js',
  'quote-worker/supervisor.js',
];
const RATE_TEST_SCRIPT = 'test:rate';
const RATE_TEST_COMMAND = 'node test/rateWorker.node.js && node test/rateSupervisor.node.js';
const RATE_CI_COMMAND = `npm run ${RATE_TEST_SCRIPT}`;
const RATE_BUILD_COMMANDS = QUOTE_WORKER_PATHS.map((rel) => `node --check ${rel}`);
const RATE_SOURCE_FILTER = 'quote-worker/**';
const RATE_IMPORT_ALLOWLISTS = {
  'quote-worker/providers.js': [],
  'quote-worker/model.js': ['buffer', 'node:buffer'],
  'quote-worker/framing.js': ['buffer', 'node:buffer'],
  'quote-worker/worker.js': [
    'https', 'node:https', 'buffer', 'node:buffer',
    './providers', './providers.js', './model', './model.js', './framing', './framing.js',
  ],
  'quote-worker/supervisor.js': [
    'buffer', 'node:buffer', 'path', 'node:path',
    'child_process', 'node:child_process',
    './framing', './framing.js', './model', './model.js', './providers', './providers.js',
  ],
};
const RATE_PROVIDER_URLS = [
  'https://api.exchange.coinbase.com/products/ZEC-USD/ticker',
  'https://api.kraken.com/0/public/Ticker?pair=XMRUSD',
];
const SOCIAL_WORKFLOW_PATHS = [
  ...SOCIAL_PATHS.slice(0, 2),
  WALLET_SOURCE_FILTER,
  'wallet-broker/**',
  PAY_SOURCE_FILTER,
  RATE_SOURCE_FILTER,
  'wallet-preload.js',
  ...SOCIAL_PATHS.slice(2).filter(
    (item) => item !== 'wallet-broker/**' && item !== PAY_SOURCE_FILTER && item !== RATE_SOURCE_FILTER
  ),
];
const SECURITY_WORKFLOW_PATHS = [
  ...SECURITY_PATHS.slice(0, 2),
  WALLET_SOURCE_FILTER,
  'wallet-broker/**',
  PAY_SOURCE_FILTER,
  RATE_SOURCE_FILTER,
  'wallet-preload.js',
  ...SECURITY_PATHS.slice(2).filter(
    (item) => item !== 'wallet-broker/**' && item !== PAY_SOURCE_FILTER && item !== RATE_SOURCE_FILTER
  ),
];

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
const WAL004_REQUIRED_FILES = [WAL004_MANIFEST, WAL004_LOCKFILE, 'wallet-broker/src/lib.rs'];
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
  'CC0-1.0',
  'ISC',
  'Zlib',
  '0BSD',
  'Unlicense',
  'Unicode-3.0',
  'OFL-1.1',
  'Ubuntu-font-1.0',
];
const WAL004_DIRECT_DEPENDENCIES = {
  argon2: { version: '=0.5.3', default_features: false, features: ['alloc'], optional: false },
  base64ct: { version: '=1.8.3', default_features: false, features: ['alloc'], optional: false },
  chacha20poly1305: { version: '=0.10.1', default_features: false, features: ['alloc'], optional: false },
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
const WAL006_DIRECT_DEPENDENCIES = {
  zcash_client_backend: {
    version: '=0.24.0', default_features: false, features: ['pczt'], optional: false,
  },
  zcash_client_sqlite: {
    version: '=0.22.0',
    default_features: false,
    features: ['orchard', 'serde', 'test-dependencies', 'transparent-inputs'],
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
const WAL006_SUPPORT_DEPENDENCIES = {
  rand_core: {
    version: '=0.6.4', default_features: false, features: ['std'], optional: false,
  },
  rusqlite: {
    version: '=0.37.0', default_features: false, features: [], optional: false,
  },
};
const WAL006_PREPARE_DEPENDENCIES = {
  'unicode-normalization': {
    version: '=0.1.25', default_features: false, features: ['std'], optional: false,
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
const WAL006_ALLOWED_RUST_SOURCE_PATHS = [
  'wallet-broker/src/zec.rs',
  'wallet-broker/src/zec/address.rs',
  'wallet-broker/src/zec/fixture.rs',
  'wallet-broker/src/zec/prepare.rs',
  'wallet-broker/src/zec/scan.rs',
  'wallet-broker/src/zec/store.rs',
  'wallet-broker/src/zec/test_support.rs',
];
const WAL008_TEST_TARGETS = [
  'zec_hardware',
];
const WAL008_ZEC_RUST_SOURCE_PATHS = [
  'wallet-broker/src/zec.rs',
  'wallet-broker/src/zec/address.rs',
  'wallet-broker/src/zec/fixture.rs',
  'wallet-broker/src/zec/hardware.rs',
  'wallet-broker/src/zec/prepare.rs',
  'wallet-broker/src/zec/scan.rs',
  'wallet-broker/src/zec/store.rs',
  'wallet-broker/src/zec/test_support.rs',
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
  'scripts/validate-rust-sbom.js',
  '.github/workflows/social.yml',
  '.github/workflows/security.yml',
  '.github/workflows/sbom.yml',
  'test/electronSecurity.node.js',
  'test/securityPolicy.node.js',
  'test/walletPay.node.js',
  'wallet-pay/model.js',
  '.gitleaksignore',
  ...WAL004_REQUIRED_FILES,
  'deny.toml',
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
    checkPathFilter(triggerPaths(data, event), SOCIAL_WORKFLOW_PATHS, name, event);
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

  for (const command of [
    BUILD_CMD, SOCIAL_TEST_CMD, SECURITY_TEST_CMD, WALLET_CI_CMD, BROKER_CI_COMMAND,
    PAY_CI_COMMAND, RATE_CI_COMMAND,
    WAL004_ROUTINE_TEST, WAL004_FMT, WAL004_CLIPPY, WAL004_NATIVE_CHECK,
  ]) {
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
    throw new PolicyError('.gitleaksignore is missing a reviewed commit fingerprint');
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
  throw new PolicyError('.gitleaksignore must be the exact nine lexically sorted reviewed commit fingerprints');
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
  checkPathFilter(triggerPaths(data, 'pull_request'), SECURITY_WORKFLOW_PATHS, name, 'pull_request');
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
  requireCommand(data, `cargo install cargo-audit --version ${CARGO_AUDIT_VERSION} --locked`);
  requireCommand(data, `cargo install cargo-deny --version ${CARGO_DENY_VERSION} --locked`);
  requireCommand(data, WAL004_AUDIT);
  requireCommand(data, WAL004_DENY);
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
  if (!runText.includes(`cargo install cargo-cyclonedx --version ${CARGO_CYCLONEDX_VERSION} --locked`)) {
    throw new PolicyError(`${name} must install the exact reviewed cargo-cyclonedx version`);
  }
  const hasExactRustSbom = iterSteps(data)
    .some(([, , step]) => stepRunLines(step).includes(WAL004_SBOM));
  if (!hasExactRustSbom) {
    throw new PolicyError(`${name} must generate the exact Rust CycloneDX document`);
  }
  if (!runText.includes('node scripts/validate-rust-sbom.js')) {
    throw new PolicyError(`${name} must validate the Rust CycloneDX document`);
  }
  const uploads = sbomUploads(data);
  if (uploads.length !== 2) {
    throw new PolicyError(`${name} must upload exactly two validated SBOM artifacts`);
  }
  const paths = [];
  for (const step of uploads) {
    const uploaded = step.with || {};
    const uploadPath = uploaded.path;
    if (typeof uploadPath !== 'string' || !uploadPath.endsWith('.cdx.json')) {
      throw new PolicyError(`${name} must upload only .cdx.json JSON documents`);
    }
    if (uploadPath.includes('*') || uploadPath.includes('?') || uploadPath.includes('\n')) {
      throw new PolicyError(`${name} upload paths must each name one CycloneDX JSON file`);
    }
    const retention = Number.parseInt(String(uploaded['retention-days']), 10);
    if (retention !== SBOM_RETENTION_DAYS) {
      throw new PolicyError(`${name} upload retention-days must be ${SBOM_RETENTION_DAYS}`);
    }
    paths.push(uploadPath);
  }
  const expectedPaths = [
    '${{ runner.temp }}/bitbook-desktop.cdx.json',
    '${{ runner.temp }}/bitbook-wallet-broker.cdx.json',
  ];
  if (JSON.stringify(paths.sort()) !== JSON.stringify(expectedPaths)) {
    throw new PolicyError(`${name} must upload the exact npm and Rust CycloneDX documents`);
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

function literalModuleSpecifier(expression) {
  const match = expression.trim().match(/^(['"])([^'"]+)\1$/);
  return match ? match[2] : null;
}

function checkWalletContractSource(source, rel) {
  if (typeof source !== 'string' || !source.trim()) {
    throw new PolicyError(`wallet source ${rel} is empty`);
  }
  const allowed = new Set(WALLET_IMPORT_ALLOWLIST);
  const callPattern = /\b(require|import)\s*\(([^)]*)\)/g;
  let match;
  while ((match = callPattern.exec(source)) !== null) {
    const specifier = literalModuleSpecifier(match[2]);
    if (!specifier || !allowed.has(specifier)) {
      throw new PolicyError(`${rel} contains a computed or non-allowlisted ${match[1]} module load`);
    }
  }
  const staticPattern = /\bimport\s+(?!\s*\()([^;\n]+)/g;
  while ((match = staticPattern.exec(source)) !== null) {
    const clause = match[1].trim();
    const direct = clause.match(/^(['"])([^'"]+)\1$/);
    const from = clause.match(/\bfrom\s+(['"])([^'"]+)\1$/);
    const specifier = direct ? direct[2] : from ? from[2] : null;
    if (!specifier || !allowed.has(specifier)) {
      throw new PolicyError(`${rel} contains a computed or non-allowlisted static import`);
    }
  }
  if (/\bfetch\s*\(/.test(source) || /\b(?:new\s+)?WebSocket\s*\(/.test(source)) {
    throw new PolicyError(`${rel} contains a forbidden network capability`);
  }
}

function checkQuoteWorkerSource(source, rel) {
  if (typeof source !== 'string' || !source.trim()) {
    throw new PolicyError(`quote-worker source ${rel} is empty`);
  }
  const allowed = RATE_IMPORT_ALLOWLISTS[rel];
  if (!allowed || !QUOTE_WORKER_PATHS.includes(rel)) {
    throw new PolicyError(`unknown quote-worker path ${rel}`);
  }
  const allow = new Set(allowed);
  const callPattern = /\b(require|import)\s*\(([^)]*)\)/g;
  let match;
  while ((match = callPattern.exec(source)) !== null) {
    const specifier = literalModuleSpecifier(match[2]);
    if (!specifier || !allow.has(specifier)) {
      throw new PolicyError(`${rel} contains a computed or non-allowlisted ${match[1]} module load`);
    }
  }
  const staticPattern = /\bimport\s+(?!\s*\()([^;\n]+)/g;
  while ((match = staticPattern.exec(source)) !== null) {
    const clause = match[1].trim();
    const direct = clause.match(/^(['"])([^'"]+)\1$/);
    const from = clause.match(/\bfrom\s+(['"])([^'"]+)\1$/);
    const specifier = direct ? direct[2] : from ? from[2] : null;
    if (!specifier || !allow.has(specifier)) {
      throw new PolicyError(`${rel} contains a computed or non-allowlisted static import`);
    }
  }
  if (/\bfetch\s*\(/.test(source) || /\b(?:new\s+)?WebSocket\s*\(/.test(source)) {
    throw new PolicyError(`${rel} contains a forbidden network capability`);
  }
  if (/enabled_by_default\s*:\s*true/.test(source)) {
    throw new PolicyError(`${rel} enables a provider by default`);
  }
  if (/\bshell\s*:\s*true\b/.test(source)) {
    throw new PolicyError(`${rel} contains forbidden spawn shell`);
  }
  if (/\benv\s*:\s*process\.env\b|\bprocess\.env\b/.test(source) && rel === 'quote-worker/supervisor.js') {
    throw new PolicyError(`${rel} inherits process environment`);
  }
  if (rel === 'quote-worker/worker.js' || rel === 'quote-worker/providers.js') {
    const urls = source.match(/https?:\/\/[^\s'"`]+/g) || [];
    for (const url of urls) {
      const cleaned = url.replace(/[.,;]+$/, '');
      if (!RATE_PROVIDER_URLS.includes(cleaned)) {
        throw new PolicyError(`${rel} contains unreviewed provider pin ${cleaned}`);
      }
      if (cleaned.startsWith('http://')) {
        throw new PolicyError(`${rel} contains an http downgrade`);
      }
    }
    const requestBlocks = source.match(/https\.request\s*\(\s*\{[^}]*\}/g) || [];
    for (const block of requestBlocks) {
      const protocol = (block.match(/protocol\s*:\s*['"]([^'"]+)['"]/) || [])[1];
      const hostname = (block.match(/hostname\s*:\s*['"]([^'"]+)['"]/) || [])[1];
      const pathValue = (block.match(/path\s*:\s*['"]([^'"]+)['"]/) || [])[1];
      if (protocol && protocol !== 'https:') {
        throw new PolicyError(`${rel} contains an http downgrade`);
      }
      if (hostname && hostname !== 'api.exchange.coinbase.com' && hostname !== 'api.kraken.com') {
        throw new PolicyError(`${rel} contains an unreviewed provider host`);
      }
      if (pathValue && pathValue !== '/products/ZEC-USD/ticker' &&
          pathValue !== '/0/public/Ticker?pair=XMRUSD') {
        throw new PolicyError(`${rel} contains an unreviewed provider path or pair`);
      }
    }
  }
}

function checkWalletPaySource(source, rel) {
  if (typeof source !== 'string' || !source.trim() || !PAY_MODEL_PATHS.includes(rel)) {
    throw new PolicyError(`wallet Pay model source ${rel} is empty or unreviewed`);
  }
  if (/\b(?:require|import)\s*\(/.test(source) || /\bimport\s+(?!\s*\()/.test(source)) {
    throw new PolicyError(`${rel} contains a forbidden wallet Pay model import`);
  }
  if (/\bfetch\s*\(|\b(?:new\s+)?WebSocket\s*\(|\bprocess\.env\b/.test(source)) {
    throw new PolicyError(`${rel} contains forbidden wallet Pay model I/O or process authority`);
  }
  if (/\bDate\.now\s*\(|\bMath\.random\s*\(|\bset(?:Timeout|Interval)\s*\(/.test(source)) {
    throw new PolicyError(`${rel} contains forbidden wallet Pay model nondeterminism or timer authority`);
  }
  for (const authority of ['intent.confirm', 'tx.broadcast']) {
    if (source.includes(authority)) {
      throw new PolicyError(`${rel} contains forbidden wallet Pay model authority ${authority}`);
    }
  }
}

function checkWalletBoundarySource(source, rel) {
  if (typeof source !== 'string' || !source.trim()) {
    throw new PolicyError(`wallet boundary source ${rel} is empty`);
  }
  const configured = BROKER_IMPORT_ALLOWLISTS[rel];
  if (!configured || !BROKER_BOUNDARY_PATHS.includes(rel)) {
    throw new PolicyError(`unknown wallet boundary path ${rel}`);
  }
  const allowed = new Set(configured);
  const callPattern = /\b(require|import)\s*\(([^)]*)\)/g;
  let match;
  while ((match = callPattern.exec(source)) !== null) {
    if (match[1] === 'import') {
      throw new PolicyError(`${rel} contains forbidden dynamic import`);
    }
    const specifier = literalModuleSpecifier(match[2]);
    if (!specifier || !allowed.has(specifier)) {
      throw new PolicyError(`${rel} contains a computed or non-allowlisted module load`);
    }
  }
  const staticPattern = /\bimport\s+(?!\s*\()([^;\n]+)/g;
  while ((match = staticPattern.exec(source)) !== null) {
    const clause = match[1].trim();
    const direct = clause.match(/^(['"])([^'"]+)\1$/);
    const from = clause.match(/\bfrom\s+(['"])([^'"]+)\1$/);
    const specifier = direct ? direct[2] : from ? from[2] : null;
    if (!specifier || !allowed.has(specifier)) {
      throw new PolicyError(`${rel} contains a computed or non-allowlisted static import`);
    }
  }
  if (/\bfetch\s*\(|\b(?:new\s+)?WebSocket\s*\(|\bcreateServer\s*\(|\.listen\s*\(/.test(source)) {
    throw new PolicyError(`${rel} contains a forbidden network or listener capability`);
  }
  if (/\b(?:exec|execFile|execSync|execFileSync|spawnSync|fork)\s*\(/.test(source)) {
    throw new PolicyError(`${rel} contains a forbidden process capability`);
  }
  if (/\bprocess\.env\b|\bshell\s*:\s*true\b|['"]inherit['"]/.test(source)) {
    throw new PolicyError(`${rel} inherits process authority, shell, environment, or stdio`);
  }
  for (const forbidden of [
    'intent.confirm', 'account.unlock', 'account.exportBackup', 'account.createSoftware',
    'signer.sign', 'tx.broadcast', 'intent.broadcast', 'rpc.raw', 'rate.fetch',
    'http.proxy', 'wallet.raw',
  ]) {
    if (source.includes(forbidden)) throw new PolicyError(`${rel} contains forbidden wallet authority ${forbidden}`);
  }

  if (rel === 'wallet-broker/protocol.js' &&
      !/\brequire\s*\(\s*['"](?:crypto|node:crypto|buffer|node:buffer)['"]\s*\)/.test(source)) {
    throw new PolicyError(`${rel} does not contain a reviewed pure protocol capability`);
  }

  if (rel === 'wallet-broker/supervisor.js') {
    if (!/\brequire\s*\(\s*['"]\.\.\/wallet-pay\/model(?:\.js)?['"]\s*\)/.test(source)) {
      throw new PolicyError(`${rel} must use the reviewed shared wallet Pay sanitizer`);
    }
    const spawnCount = (source.match(/\bspawn\s*\(/g) || []).length;
    if (spawnCount === 0) throw new PolicyError(`${rel} does not contain the reviewed inert spawn boundary`);
    const spawnCalls = source.match(/\bspawn\s*\([^;]*\)/g) || [];
    if (spawnCount !== spawnCalls.length) throw new PolicyError(`${rel} contains an unreviewable spawn call`);
    for (const call of spawnCalls) {
      if (!/\bspawn\s*\(\s*[^,]+,\s*\[\s*\]\s*,\s*\{/.test(call) ||
          !/\bshell\s*:\s*false\b/.test(call) ||
          !/\bstdio\s*:\s*\[\s*['"]pipe['"]\s*,\s*['"]pipe['"]\s*,\s*['"]pipe['"]\s*\]/.test(call) ||
          !/\benv\s*:\s*cleanEnv\b/.test(call)) {
        throw new PolicyError(`${rel} spawn must use empty argv, clean env, no shell, and three pipes`);
      }
    }
  }

  if (rel === 'wallet-preload.js') {
    if (/\bipcRenderer\s*\.\s*(?:send|sendSync)\s*\(/.test(source)) {
      throw new PolicyError(`${rel} contains forbidden generic IPC send`);
    }
    const invokes = [];
    const invokePattern = /\bipcRenderer\s*\.\s*invoke\s*\(\s*([^,)]+)/g;
    while ((match = invokePattern.exec(source)) !== null) {
      const channel = literalModuleSpecifier(match[1]);
      if (!channel || !PRELOAD_INVOKE_CHANNELS.includes(channel)) {
        throw new PolicyError(`${rel} contains dynamic or unlisted IPC invoke`);
      }
      invokes.push(channel);
    }
    if (invokes.length !== PRELOAD_INVOKE_CHANNELS.length ||
        PRELOAD_INVOKE_CHANNELS.some((channel) => !invokes.includes(channel))) {
      throw new PolicyError(`${rel} must contain every fixed wallet invoke channel exactly once`);
    }
    const onChannels = [];
    const removeChannels = [];
    const listenerPattern = /\bipcRenderer\s*\.\s*(on|removeListener)\s*\(\s*([^,)]+)/g;
    while ((match = listenerPattern.exec(source)) !== null) {
      const channel = literalModuleSpecifier(match[2]);
      if (!channel || channel !== PRELOAD_SUBSCRIBE_CHANNEL) {
        throw new PolicyError(`${rel} contains dynamic or mismatched subscription IPC`);
      }
      (match[1] === 'on' ? onChannels : removeChannels).push(channel);
    }
    if (onChannels.length !== 1 || removeChannels.length !== 1 ||
        !/contextBridge\s*\.\s*exposeInMainWorld\s*\(\s*['"]bitbookWallet['"]/.test(source)) {
      throw new PolicyError(`${rel} must expose one bounded wallet subscription bridge`);
    }
  }
}

function checkWalletBrokerManifest(manifestText, options = {}) {
  if (typeof manifestText !== 'string' || !manifestText.trim()) {
    throw new PolicyError('wallet Rust manifest is empty');
  }
  for (const required of [
    'name = "bitbook-wallet-broker"',
    'edition = "2024"',
    'rust-version = "1.98.0"',
    'publish = false',
    'license = "MIT"',
    'default = []',
    'native-ui = ["dep:eframe", "dep:rfd"]',
    'xmr-local-gate = []',
  ]) {
    if (!manifestText.split('\n').includes(required)) {
      throw new PolicyError(`wallet Rust manifest omits exact ${required}`);
    }
  }
  const expectedDependencies = [
    'argon2 = { version = "=0.5.3", default-features = false, features = ["alloc"] }',
    'base64ct = { version = "=1.8.3", default-features = false, features = ["alloc"] }',
    'chacha20poly1305 = { version = "=0.10.1", default-features = false, features = ["alloc"] }',
    'eframe = { version = "=0.36.1", optional = true, default-features = false, features = ["default_fonts", "glow", "wayland", "x11"] }',
    'getrandom = { version = "=0.4.3", default-features = false, features = ["std"] }',
    'hkdf = { version = "=0.12.4", default-features = false }',
    'md-5 = { version = "=0.11.0-pre.4", default-features = false, features = ["zeroize"] }',
    'rfd = { version = "=0.17.2", optional = true, default-features = false, features = ["xdg-portal", "wayland"] }',
    'secrecy = { version = "=0.10.3", default-features = false }',
    'serde = { version = "=1.0.229", default-features = false, features = ["alloc", "derive"] }',
    'serde_json = { version = "=1.0.151", default-features = false, features = ["alloc"] }',
    'sha2 = { version = "=0.10.9", default-features = false }',
    'zeroize = { version = "=1.9.0", default-features = false, features = ["alloc"] }',
    'zcash_client_backend = { version = "=0.24.0", default-features = false, features = ["pczt"] }',
    'zcash_client_sqlite = { version = "=0.22.0", default-features = false, features = ["orchard", "serde", "test-dependencies", "transparent-inputs"] }',
    'pczt = { version = "=0.9.3", default-features = false }',
    'zcash_primitives = { version = "=0.30.1", default-features = false }',
    'zcash_protocol = { version = "=0.10.5", default-features = false, features = ["local-consensus"] }',
    'zcash_keys = { version = "=0.16.1", default-features = false, features = ["orchard"] }',
    'rand_core = { version = "=0.6.4", default-features = false, features = ["std"] }',
    'rusqlite = { version = "=0.37.0", default-features = false }',
    'unicode-normalization = { version = "=0.1.25", default-features = false, features = ["std"] }',
  ];
  const dependencyBlock = manifestText.split('[dependencies]\n')[1];
  if (!dependencyBlock) {
    throw new PolicyError('wallet Rust manifest is missing dependencies');
  }
  const actualDependencies = dependencyBlock
    .split('\n[[test]]')[0]
    .split('\n')
    .filter((line) => line.trim());
  if (JSON.stringify(actualDependencies) !== JSON.stringify(expectedDependencies)) {
    throw new PolicyError('wallet Rust manifest dependency pins or features differ from review');
  }
  const expectedDependencyNames = new Set(
    expectedDependencies.map((line) => line.slice(0, line.indexOf(' = ')))
  );
  const reviewedAssignments = manifestText
    .split('\n')
    .filter((line) => {
      const assignment = line.match(/^\s*([A-Za-z0-9_-]+)\s*=/);
      return assignment && expectedDependencyNames.has(assignment[1]);
    });
  if (JSON.stringify(reviewedAssignments) !== JSON.stringify(expectedDependencies)) {
    throw new PolicyError(
      'wallet Rust manifest dependency assignments contain a duplicate or displaced declaration'
    );
  }
  const tests = [...manifestText.matchAll(/\[\[test\]\]\nname = "([^"]+)"\npath = "([^"]+)"/g)]
    .map((match) => `${match[1]}:${match[2]}`);
  const expectedTests = [
    'vault_crypto:tests/vault_crypto.rs',
    'vault_format:tests/vault_format.rs',
    'vault_store:tests/vault_store.rs',
    'vault_session:tests/vault_session.rs',
    'native_surface:tests/native_surface.rs',
    'secret_hygiene:tests/secret_hygiene.rs',
    'zec_fixture_builder:tests/zec_fixture_builder.rs',
    'zec_address:tests/zec_address.rs',
    'zec_store:tests/zec_store.rs',
    'zec_scan:tests/zec_scan.rs',
    'zec_prepare:tests/zec_prepare.rs',
    'zec_hygiene:tests/zec_hygiene.rs',
    'zec_hardware:tests/zec_hardware.rs',
    'xmr_distribution:tests/xmr_distribution.rs',
    'xmr_process:tests/xmr_process.rs',
    'xmr_rpc:tests/xmr_rpc.rs',
    'xmr_account:tests/xmr_account.rs',
    'xmr_receiver:tests/xmr_receiver.rs',
    'xmr_hygiene:tests/xmr_hygiene.rs',
    'xmr_local_gate:tests/xmr_local_gate.rs',
  ];
  if (JSON.stringify(tests) !== JSON.stringify(expectedTests)) {
    throw new PolicyError('wallet Rust manifest integration-test targets differ from review');
  }
  if (options.requireLibrary && !manifestText.includes('license = "MIT"')) {
    throw new PolicyError('wallet Rust production manifest is incomplete');
  }
  if (/\bgit\s*=|\*"|reqwest|tokio|keyring|monero|openssl/i.test(manifestText)) {
    throw new PolicyError('wallet Rust manifest contains forbidden dependency authority');
  }
}

function checkRustWalletSourceInventory(actual) {
  if (!Array.isArray(actual)) {
    throw new PolicyError('wallet Rust source inventory must be an array');
  }
  if (actual.some((rel) => typeof rel !== 'string' || !rel)) {
    throw new PolicyError('wallet Rust source inventory contains a malformed path');
  }
  if (new Set(actual).size !== actual.length) {
    throw new PolicyError('wallet Rust source inventory contains a duplicate path');
  }
  const legacy = actual.length === WAL004_RUST_SOURCE_PATHS.length &&
    sameSet(actual, WAL004_RUST_SOURCE_PATHS);
  const withWal006 = [...WAL004_RUST_SOURCE_PATHS, 'wallet-broker/src/zec.rs'];
  const extended = actual.length === withWal006.length && sameSet(actual, withWal006);
  const withWal007 = [...withWal006, 'wallet-broker/src/xmr.rs'];
  const xmrExtended = actual.length === withWal007.length && sameSet(actual, withWal007);
  if (!legacy && !extended && !xmrExtended) {
    throw new PolicyError('wallet Rust source inventory is missing or extra');
  }
}

function checkWal006ResolvedFeatures(resolved) {
  const expectedKeys = [
    'direct',
    'compiled_pczt_capabilities',
    'bitbook_authority',
  ];
  if (resolved && Object.prototype.hasOwnProperty.call(resolved, 'enabled_features')) {
    expectedKeys.push('enabled_features');
  }
  if (!resolved || typeof resolved !== 'object' || Array.isArray(resolved) ||
      Object.getPrototypeOf(resolved) !== Object.prototype ||
      !sameSet(Object.keys(resolved), expectedKeys)) {
    throw new PolicyError('WAL-006 Zcash resolved-feature contract must be a closed object');
  }

  const direct = resolved.direct;
  if (!direct || typeof direct !== 'object' || Array.isArray(direct) ||
      Object.getPrototypeOf(direct) !== Object.prototype ||
      !sameSet(Object.keys(direct), Object.keys(WAL006_DIRECT_DEPENDENCIES))) {
    throw new PolicyError('WAL-006 Zcash direct dependency contract differs from review');
  }
  for (const [name, expected] of Object.entries(WAL006_DIRECT_DEPENDENCIES)) {
    const actual = direct[name];
    if (!actual || typeof actual !== 'object' || Array.isArray(actual) ||
        Object.getPrototypeOf(actual) !== Object.prototype ||
        !sameSet(Object.keys(actual), Object.keys(expected)) ||
        actual.version !== expected.version ||
        actual.default_features !== expected.default_features ||
        actual.optional !== expected.optional ||
        !Array.isArray(actual.features) ||
        new Set(actual.features).size !== actual.features.length ||
        !sameSet(actual.features, expected.features)) {
      throw new PolicyError(`WAL-006 Zcash direct dependency ${name} differs from review`);
    }
  }

  const enabled = resolved.enabled_features === undefined ? [] : resolved.enabled_features;
  if (!Array.isArray(enabled) || enabled.some((feature) => typeof feature !== 'string' || !feature) ||
      new Set(enabled).size !== enabled.length) {
    throw new PolicyError('WAL-006 Zcash enabled-feature inventory is malformed or duplicated');
  }
  const reviewedFeatures = new Set([
    ...Object.values(WAL006_DIRECT_DEPENDENCIES).flatMap((dependency) => dependency.features),
    ...WAL006_EXPECTED_COMPILED_PCZT_CAPABILITIES,
  ]);
  for (const feature of enabled) {
    if (WAL006_FORBIDDEN_FEATURES.includes(feature)) {
      throw new PolicyError(`WAL-006 Zcash feature ${feature} is forbidden network authority`);
    }
    if (!reviewedFeatures.has(feature)) {
      throw new PolicyError(`WAL-006 Zcash feature ${feature} is unknown`);
    }
  }

  const compiled = resolved.compiled_pczt_capabilities;
  if (!Array.isArray(compiled) || compiled.some((item) => typeof item !== 'string' || !item) ||
      new Set(compiled).size !== compiled.length ||
      compiled.length !== WAL006_EXPECTED_COMPILED_PCZT_CAPABILITIES.length ||
      !sameSet(compiled, WAL006_EXPECTED_COMPILED_PCZT_CAPABILITIES)) {
    throw new PolicyError('WAL-006 compiled PCZT capability inventory differs from review');
  }

  const authority = resolved.bitbook_authority;
  const expectedAuthority = ['receiver.fresh', 'fixture.scan', 'pczt.prepare'];
  if (!Array.isArray(authority) || authority.some((item) => typeof item !== 'string' || !item) ||
      new Set(authority).size !== authority.length ||
      authority.length !== expectedAuthority.length || !sameSet(authority, expectedAuthority)) {
    throw new PolicyError(
      'WAL-006 Zcash authority contains raw, sign, prove, finalize, extract, broadcast, or network capability'
    );
  }
}

function checkWal006RustSourceInventory(actual) {
  if (!Array.isArray(actual)) {
    throw new PolicyError('WAL-006 Zcash Rust source inventory must be an array');
  }
  if (actual.some((rel) => typeof rel !== 'string' || !rel)) {
    throw new PolicyError('WAL-006 Zcash Rust source inventory contains a malformed path');
  }
  if (new Set(actual).size !== actual.length) {
    throw new PolicyError('WAL-006 Zcash Rust source inventory contains a duplicate path');
  }
  if (JSON.stringify(actual) !== JSON.stringify(WAL006_ALLOWED_RUST_SOURCE_PATHS)) {
    throw new PolicyError('WAL-006 Zcash Rust source inventory is missing, unlisted, or extra');
  }
}

function checkWal008RustSourceInventory(actual) {
  if (!Array.isArray(actual)) {
    throw new PolicyError('WAL-008 Zcash Rust source inventory must be an array');
  }
  if (actual.some((rel) => typeof rel !== 'string' || !rel)) {
    throw new PolicyError('WAL-008 Zcash Rust source inventory contains a malformed string path');
  }
  if (new Set(actual).size !== actual.length) {
    throw new PolicyError('WAL-008 Zcash Rust source inventory contains a duplicate path');
  }
  if (!actual.includes('wallet-broker/src/zec/hardware.rs')) {
    throw new PolicyError('WAL-008 Zcash Rust source inventory is missing hardware');
  }
  if (JSON.stringify(actual) !== JSON.stringify(WAL008_ZEC_RUST_SOURCE_PATHS) &&
      actual.length === WAL008_ZEC_RUST_SOURCE_PATHS.length &&
      sameSet(actual, WAL008_ZEC_RUST_SOURCE_PATHS)) {
    throw new PolicyError('WAL-008 Zcash Rust source inventory must retain exact sorted order');
  }
  const unlisted = actual.filter((rel) => !WAL008_ZEC_RUST_SOURCE_PATHS.includes(rel));
  if (unlisted.length) {
    throw new PolicyError('WAL-008 Zcash Rust source inventory contains an unlisted extra path');
  }
  if (JSON.stringify(actual) !== JSON.stringify(WAL008_ZEC_RUST_SOURCE_PATHS)) {
    throw new PolicyError('WAL-008 Zcash Rust source inventory is missing a reviewed path');
  }
}

function checkRustWalletSource(source, rel) {
  if (typeof source !== 'string' || !source.trim()) {
    throw new PolicyError(`wallet Rust source ${rel} is empty`);
  }
  const reviewedXmrPickerTitle = '"Select monero-wallet-rpc"';
  let screened = source.replace(/#!\[forbid\(unsafe_code\)\]/g, '');
  if (rel === 'wallet-broker/src/native_ui.rs') {
    const titleOccurrences = source.split(reviewedXmrPickerTitle).length - 1;
    if (titleOccurrences > 1) {
      throw new PolicyError('wallet native source repeats the reviewed XMR picker title');
    }
    if (titleOccurrences === 1) {
      screened = screened.replace(reviewedXmrPickerTitle, '');
    }
  }
  const wal006Path = /^wallet-broker\/(?:src\/zec(?:\.rs|\/.*\.rs)|tests\/zec[^/]*\.rs)$/.test(rel);
  const forbidden = [
    [/\bunsafe\b/, 'unsafe'],
    [/extern\s+"C"/, 'FFI'],
    [/std::(?:net|os::unix::net)|TcpListener|TcpStream|UnixListener|UnixStream/, 'network listener'],
    [wal006Path ? /(?:\b(?:reqwest|tokio|keyring)\b|monero)/i :
      /(?:\b(?:reqwest|tokio|keyring|zcash_client_backend)\b|monero)/i,
    'unreviewed authority'],
    [/std::env::temp_dir\s*\(/, 'temporary-directory authority'],
    [/\bCommand::new\s*\(/, 'process authority'],
    [/\b(?:fetch|WebSocket)\s*\(/, 'network authority'],
  ];
  for (const [pattern, label] of forbidden) {
    if (pattern.test(screened)) {
      throw new PolicyError(`wallet Rust source ${rel} contains forbidden ${label}`);
    }
  }
  if (wal006Path) {
    const wal006Forbidden = [
      [/zcash_client_backend(?:::[A-Za-z0-9_]+)*::proto::service|compact_tx_streamer_client|lightwalletd/i,
        'WAL-006 Zcash lightwalletd service-client authority'],
      [/\b(?:endpoint|url|uri)\s*(?::[^=;\n]+)?=/i, 'WAL-006 Zcash endpoint authority'],
      [/\b(?:broadcast|send_transaction)\s*!?\s*\(|\.broadcast\s*\(/i,
        'WAL-006 Zcash broadcast authority'],
      [/\b(?:connect|listen)\s*\(|\.(?:connect|listen)\s*\(/i,
        'WAL-006 Zcash network authority'],
    ];
    for (const [pattern, label] of wal006Forbidden) {
      if (pattern.test(screened)) {
        throw new PolicyError(`wallet Rust source ${rel} contains forbidden ${label}`);
      }
    }
    if (/\.(?:sign|prove|extract)\s*\(|\b(?:sign|prove|extract)\s*\(/i.test(screened) ||
        /\b(?:pczt|transaction|tx|prepared_pczt|artifact)\s*\.\s*finalize\s*\(/i.test(screened)) {
      throw new PolicyError(`wallet Rust source ${rel} contains forbidden WAL-006 Zcash authority`);
    }
    if (rel.startsWith('wallet-broker/src/zec') &&
        rel !== 'wallet-broker/src/zec/test_support.rs' && /\bNetwork::MainNetwork\b/.test(screened)) {
      throw new PolicyError(`wallet Rust source ${rel} contains forbidden WAL-006 Zcash mainnet authority`);
    }
  }
  if (rel === 'wallet-broker/src/vault.rs') {
    for (const primitive of [
      'Base64Unpadded',
      'Encoding',
      'SecretSlice',
      'ExposeSecret',
      'ExposeSecretMut',
    ]) {
      if (!new RegExp(`\\b${primitive}\\b`).test(source)) {
        throw new PolicyError(`wallet Rust vault omits reviewed ${primitive} primitive`);
      }
    }
    if (/\b(?:encode_base64|decode_base64)\b/.test(source)) {
      throw new PolicyError('wallet Rust vault contains a handwritten Base64 helper');
    }
  }
  if (rel === 'wallet-broker/src/native_ui.rs' && /\bto_string_lossy\b/.test(source)) {
    throw new PolicyError('wallet native path conversion must be exact UTF-8');
  }
}

function parseWalletBrokerDenyPolicy(text) {
  if (typeof text !== 'string' || !text.trim()) {
    throw new PolicyError('wallet cargo-deny policy must be nonempty text');
  }
  const sections = Object.create(null);
  let section = null;
  let pending = null;

  function parseValue(raw) {
    const value = raw.trim();
    try {
      if (value.startsWith('[') && value.endsWith(']')) {
        return JSON.parse(value.replace(/,\s*]$/, ']'));
      }
      if (/^"(?:[^"\\]|\\.)*"$/.test(value)) {
        return JSON.parse(value);
      }
      if (/^(?:0|[1-9]\d*)(?:\.\d+)?$/.test(value)) {
        return Number(value);
      }
    } catch (_error) {
      throw new PolicyError('wallet cargo-deny policy contains a malformed value');
    }
    throw new PolicyError('wallet cargo-deny policy contains an unsupported value');
  }

  function assign(key, raw) {
    if (!section) {
      throw new PolicyError('wallet cargo-deny policy contains a key outside a section');
    }
    if (Object.prototype.hasOwnProperty.call(sections[section], key)) {
      throw new PolicyError(`wallet cargo-deny policy duplicates ${section}.${key}`);
    }
    sections[section][key] = parseValue(raw);
  }

  for (const rawLine of text.split(/\r?\n/)) {
    const line = rawLine.trim();
    if (pending) {
      pending.value += `\n${line}`;
      if (line.endsWith(']')) {
        assign(pending.key, pending.value);
        pending = null;
      }
      continue;
    }
    if (!line || line.startsWith('#')) {
      continue;
    }
    const sectionMatch = line.match(/^\[([a-z-]+)]$/);
    if (sectionMatch) {
      section = sectionMatch[1];
      if (Object.prototype.hasOwnProperty.call(sections, section)) {
        throw new PolicyError(`wallet cargo-deny policy duplicates [${section}]`);
      }
      sections[section] = Object.create(null);
      continue;
    }
    const keyMatch = line.match(/^([a-z-]+)\s*=\s*(.+)$/);
    if (!keyMatch) {
      throw new PolicyError('wallet cargo-deny policy contains malformed syntax');
    }
    const [, key, value] = keyMatch;
    if (value.startsWith('[') && !value.endsWith(']')) {
      pending = { key, value };
    } else {
      assign(key, value);
    }
  }
  if (pending) {
    throw new PolicyError('wallet cargo-deny policy contains an unterminated array');
  }
  return sections;
}

function checkWalletBrokerDenyPolicy(text) {
  const sections = parseWalletBrokerDenyPolicy(text);
  const expectedKeys = {
    advisories: ['version', 'yanked', 'ignore'],
    licenses: ['version', 'confidence-threshold', 'allow', 'exceptions'],
    bans: ['multiple-versions', 'wildcards', 'highlight', 'allow', 'deny', 'skip', 'skip-tree'],
    sources: ['unknown-registry', 'unknown-git', 'allow-registry', 'allow-git'],
  };
  if (!sameSet(Object.keys(sections), Object.keys(expectedKeys))) {
    throw new PolicyError('wallet cargo-deny policy sections differ from review');
  }
  for (const [name, keys] of Object.entries(expectedKeys)) {
    if (!sameSet(Object.keys(sections[name]), keys)) {
      throw new PolicyError(`wallet cargo-deny [${name}] contains an unknown or missing bypass key`);
    }
  }

  const advisories = sections.advisories;
  if (advisories.version !== 2 || advisories.yanked !== 'deny' ||
      JSON.stringify(advisories.ignore) !== '[]') {
    throw new PolicyError('wallet cargo-deny advisories must deny yanked crates with empty ignore');
  }
  const licenses = sections.licenses;
  if (licenses.version !== 2 || licenses['confidence-threshold'] !== 0.93 ||
      JSON.stringify(licenses.allow) !== JSON.stringify(WAL004_ALLOWED_LICENSES) ||
      JSON.stringify(licenses.exceptions) !== '[]') {
    throw new PolicyError('wallet cargo-deny licenses differ from the exact reviewed policy');
  }
  const bans = sections.bans;
  if (bans['multiple-versions'] !== 'warn' || bans.wildcards !== 'deny' ||
      bans.highlight !== 'all' || JSON.stringify(bans.allow) !== '[]' ||
      JSON.stringify(bans.deny) !== '[]' || JSON.stringify(bans.skip) !== '[]' ||
      JSON.stringify(bans['skip-tree']) !== '[]') {
    throw new PolicyError('wallet cargo-deny bans contain a duplicate, wildcard, or skip bypass');
  }
  const sources = sections.sources;
  if (sources['unknown-registry'] !== 'deny' || sources['unknown-git'] !== 'deny' ||
      JSON.stringify(sources['allow-registry']) !==
        JSON.stringify(['https://github.com/rust-lang/crates.io-index']) ||
      JSON.stringify(sources['allow-git']) !== '[]') {
    throw new PolicyError('wallet cargo-deny sources contain an unreviewed registry or git source');
  }
}

function checkPackageJson(packageText) {
  let pkg;
  try {
    pkg = JSON.parse(packageText);
  } catch (error) {
    throw new PolicyError(`package.json is not valid JSON: ${error.message}`);
  }
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
  if (pkg.scripts[WALLET_TEST_SCRIPT] !== WALLET_TEST_CMD) {
    throw new PolicyError(`package.json must expose ${WALLET_TEST_SCRIPT} as ${WALLET_TEST_CMD}`);
  }
  if (pkg.scripts[BROKER_TEST_SCRIPT] !== BROKER_TEST_COMMAND) {
    throw new PolicyError(`package.json must expose ${BROKER_TEST_SCRIPT} as the exact broker tests`);
  }
  if (pkg.scripts[PAY_TEST_SCRIPT] !== PAY_TEST_COMMAND) {
    throw new PolicyError(`package.json must expose ${PAY_TEST_SCRIPT} as ${PAY_TEST_COMMAND}`);
  }
  if (pkg.scripts[RATE_TEST_SCRIPT] !== RATE_TEST_COMMAND) {
    throw new PolicyError(`package.json must expose ${RATE_TEST_SCRIPT} as the exact quote-worker tests`);
  }
  if (pkg.scripts.test !== TOP_LEVEL_TEST_CMD) {
    throw new PolicyError('package.json top-level npm test must include the exact rate test command');
  }
  if (typeof pkg.scripts.build !== 'string') {
    throw new PolicyError('package.json build script is missing');
  }
  const buildCommands = pkg.scripts.build.split(/\s*&&\s*/);
  for (const command of WALLET_BUILD_COMMANDS) {
    if (!buildCommands.includes(command)) {
      throw new PolicyError(`package.json wallet build syntax omits ${command}`);
    }
  }
  for (const command of BROKER_BUILD_COMMANDS) {
    if (!buildCommands.includes(command)) {
      throw new PolicyError(`package.json wallet broker build syntax omits ${command}`);
    }
  }
  for (const command of PAY_BUILD_COMMANDS) {
    if (!buildCommands.includes(command)) {
      throw new PolicyError(`package.json wallet Pay build syntax omits ${command}`);
    }
  }
  for (const command of RATE_BUILD_COMMANDS) {
    if (!buildCommands.includes(command)) {
      throw new PolicyError(`package.json quote-worker build syntax omits ${command}`);
    }
  }
  if (!buildCommands.includes('node --check scripts/validate-rust-sbom.js')) {
    throw new PolicyError('package.json build syntax omits the Rust SBOM validator');
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
  checkPackageJson(fs.readFileSync(path.join(root, 'package.json'), 'utf8'));
  for (const rel of WALLET_CONTRACT_PATHS) {
    const sourcePath = path.join(root, rel);
    if (!fs.existsSync(sourcePath)) throw new PolicyError(`missing ${rel}`);
    checkWalletContractSource(fs.readFileSync(sourcePath, 'utf8'), rel);
  }
  for (const rel of PAY_MODEL_PATHS) {
    const sourcePath = path.join(root, rel);
    if (!fs.existsSync(sourcePath)) throw new PolicyError(`missing ${rel}`);
    checkWalletPaySource(fs.readFileSync(sourcePath, 'utf8'), rel);
  }
  for (const rel of BROKER_BOUNDARY_PATHS) {
    const sourcePath = path.join(root, rel);
    if (!fs.existsSync(sourcePath)) throw new PolicyError(`missing ${rel}`);
    checkWalletBoundarySource(fs.readFileSync(sourcePath, 'utf8'), rel);
  }
  for (const rel of QUOTE_WORKER_PATHS) {
    const sourcePath = path.join(root, rel);
    if (!fs.existsSync(sourcePath)) throw new PolicyError(`missing ${rel}`);
    checkQuoteWorkerSource(fs.readFileSync(sourcePath, 'utf8'), rel);
  }
  checkWalletBrokerManifest(fs.readFileSync(path.join(root, WAL004_MANIFEST), 'utf8'), {
    requireLibrary: true,
    requireLockfile: true,
  });
  const rustSourceDirectory = path.join(root, 'wallet-broker', 'src');
  const actualRustSources = fs.readdirSync(rustSourceDirectory, { withFileTypes: true })
    .filter((entry) => entry.isFile() && entry.name.endsWith('.rs'))
    .map((entry) => `wallet-broker/src/${entry.name}`);
  checkRustWalletSourceInventory(actualRustSources);
  for (const rel of WAL004_RUST_SOURCE_PATHS) {
    const sourcePath = path.join(root, rel);
    if (!fs.existsSync(sourcePath)) throw new PolicyError(`missing ${rel}`);
    checkRustWalletSource(fs.readFileSync(sourcePath, 'utf8'), rel);
  }
  const recursiveRustSources = [];
  const collectRustSources = (directory, relative) => {
    for (const entry of fs.readdirSync(directory, { withFileTypes: true })) {
      const childRelative = path.posix.join(relative, entry.name);
      if (entry.isDirectory()) {
        collectRustSources(path.join(directory, entry.name), childRelative);
      } else if (entry.isFile() && entry.name.endsWith('.rs')) {
        recursiveRustSources.push(childRelative);
      }
    }
  };
  collectRustSources(rustSourceDirectory, 'wallet-broker/src');
  const wal008RustSources = recursiveRustSources
    .filter((rel) => /^wallet-broker\/src\/zec(?:[_.\/])/.test(rel))
    .sort();
  checkWal008RustSourceInventory(wal008RustSources);
  for (const rel of WAL008_ZEC_RUST_SOURCE_PATHS) {
    const sourcePath = path.join(root, rel);
    if (!fs.existsSync(sourcePath)) throw new PolicyError(`missing ${rel}`);
    checkRustWalletSource(fs.readFileSync(sourcePath, 'utf8'), rel);
  }
  checkWal006ResolvedFeatures({
    direct: WAL006_DIRECT_DEPENDENCIES,
    compiled_pczt_capabilities: WAL006_EXPECTED_COMPILED_PCZT_CAPABILITIES,
    bitbook_authority: ['receiver.fresh', 'fixture.scan', 'pczt.prepare'],
  });
  checkWalletBrokerDenyPolicy(fs.readFileSync(path.join(root, 'deny.toml'), 'utf8'));
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
  QUOTE_WORKER_PATHS,
  RATE_BUILD_COMMANDS,
  RATE_TEST_COMMAND,
  RATE_TEST_SCRIPT,
  RATE_CI_COMMAND,
  RATE_IMPORT_ALLOWLISTS,
  RATE_PROVIDER_URLS,
  WALLET_CONTRACT_PATHS,
  WALLET_BUILD_COMMANDS,
  WALLET_IMPORT_ALLOWLIST,
  PAY_MODEL_PATHS,
  PAY_TEST_COMMAND,
  PAY_BUILD_COMMANDS,
  PAY_IMPORT_ALLOWLIST,
  BROKER_BOUNDARY_PATHS,
  BROKER_TEST_COMMANDS,
  BROKER_TEST_SCRIPT,
  BROKER_TEST_COMMAND,
  BROKER_CI_COMMAND,
  BROKER_BUILD_COMMANDS,
  BROKER_IMPORT_ALLOWLISTS,
  PRELOAD_INVOKE_CHANNELS,
  PRELOAD_SUBSCRIBE_CHANNEL,
  WAL004_TOOLCHAIN,
  WAL004_PLATFORM,
  WAL004_ROUTINE_TEST,
  WAL004_FMT,
  WAL004_CLIPPY,
  WAL004_NATIVE_CHECK,
  CARGO_AUDIT_VERSION,
  CARGO_DENY_VERSION,
  CARGO_CYCLONEDX_VERSION,
  WAL004_AUDIT,
  WAL004_DENY,
  WAL004_SBOM,
  WAL004_REQUIRED_FILES,
  WAL004_RUST_SOURCE_PATHS,
  WAL004_ALLOWED_LICENSES,
  WAL004_DIRECT_DEPENDENCIES,
  WAL006_DIRECT_DEPENDENCIES,
  WAL006_SUPPORT_DEPENDENCIES,
  WAL006_PREPARE_DEPENDENCIES,
  WAL006_TEST_TARGETS,
  WAL006_FORBIDDEN_FEATURES,
  WAL006_EXPECTED_COMPILED_PCZT_CAPABILITIES,
  WAL006_ALLOWED_RUST_SOURCE_PATHS,
  WAL008_TEST_TARGETS,
  WAL008_ZEC_RUST_SOURCE_PATHS,
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
  checkPackageJson,
  checkWalletContractSource,
  checkWalletPaySource,
  checkQuoteWorkerSource,
  checkWalletBoundarySource,
  checkWalletBrokerManifest,
  checkRustWalletSourceInventory,
  checkWal006ResolvedFeatures,
  checkWal006RustSourceInventory,
  checkWal008RustSourceInventory,
  checkRustWalletSource,
  checkWalletBrokerDenyPolicy,
  checkRepository,
  checkGitleaksRatchetBytes,
  checkInheritedLoaderNeutralization,
};
