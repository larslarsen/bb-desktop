'use strict';

const fs = require('fs');
const path = require('path');

class PolicyError extends Error {
  constructor(message) {
    super(message);
    this.name = 'PolicyError';
  }
}

const ROOT_COMPONENT_NAME = 'bitbook-wallet-broker';
const MAX_SBOM_BYTES = 16 * 1024 * 1024;
const DIRECT_COMPONENTS = Object.freeze([
  'argon2',
  'base64ct',
  'chacha20poly1305',
  'eframe',
  'getrandom',
  'hkdf',
  'rfd',
  'secrecy',
  'serde',
  'serde_json',
  'sha2',
  'zeroize',
]);

function validateRustCycloneDxDocument(document) {
  if (!document || typeof document !== 'object' || Array.isArray(document)) {
    throw new PolicyError('Rust SBOM is not a JSON object');
  }
  if (document.bomFormat !== 'CycloneDX') {
    throw new PolicyError('Rust SBOM does not declare CycloneDX');
  }
  if (typeof document.specVersion !== 'string' || !/^\d+\.\d+$/.test(document.specVersion)) {
    throw new PolicyError('Rust SBOM specVersion is missing or invalid');
  }
  const metadata = document.metadata;
  if (!metadata || typeof metadata !== 'object' || Array.isArray(metadata)) {
    throw new PolicyError('Rust SBOM metadata is missing');
  }
  const root = metadata.component;
  if (!root || typeof root !== 'object' || Array.isArray(root)) {
    throw new PolicyError('Rust SBOM root component is missing');
  }
  if (root.name !== ROOT_COMPONENT_NAME) {
    throw new PolicyError(`Rust SBOM root component is not ${ROOT_COMPONENT_NAME}`);
  }
  if (root.name === 'bitbook-desktop' || String(root.purl || '').includes('npm')) {
    throw new PolicyError('Rust SBOM must not use the npm desktop root');
  }
  if (!Array.isArray(document.components) || document.components.length === 0) {
    throw new PolicyError('Rust SBOM components array is empty');
  }
  if (!Array.isArray(document.dependencies) || document.dependencies.length === 0) {
    throw new PolicyError('Rust SBOM dependencies array is empty');
  }
  const names = new Set(document.components.map((component) => component && component.name));
  const missing = DIRECT_COMPONENTS.filter((name) => !names.has(name));
  if (missing.length) {
    throw new PolicyError(`Rust SBOM omits direct components: ${missing.join(', ')}`);
  }
}

function validateRustCycloneDxFile(filePath) {
  const resolved = path.resolve(String(filePath || ''));
  let stat;
  try {
    stat = fs.lstatSync(resolved);
  } catch (error) {
    throw new PolicyError(`unable to inspect Rust SBOM: ${error.message}`);
  }
  if (!stat.isFile() || stat.isSymbolicLink()) {
    throw new PolicyError('Rust SBOM input must be one regular file');
  }
  if (stat.size <= 0 || stat.size > MAX_SBOM_BYTES) {
    throw new PolicyError(`Rust SBOM size must be 1-${MAX_SBOM_BYTES} bytes`);
  }
  let document;
  try {
    document = JSON.parse(fs.readFileSync(resolved, 'utf8'));
  } catch (error) {
    throw new PolicyError(`Rust SBOM is not bounded valid JSON: ${error.message}`);
  }
  validateRustCycloneDxDocument(document);
}

function main(argv) {
  if (!argv[2]) {
    process.stderr.write('usage: node scripts/validate-rust-sbom.js <sbom.json>\n');
    process.exit(2);
  }
  validateRustCycloneDxFile(argv[2]);
  process.stdout.write('BitBook wallet-broker Rust SBOM validation passed.\n');
}

if (require.main === module) {
  try {
    main(process.argv);
  } catch (error) {
    process.stderr.write(`${error && error.message ? error.message : error}\n`);
    process.exit(1);
  }
}

module.exports = {
  PolicyError,
  ROOT_COMPONENT_NAME,
  MAX_SBOM_BYTES,
  DIRECT_COMPONENTS,
  validateRustCycloneDxDocument,
  validateRustCycloneDxFile,
};
