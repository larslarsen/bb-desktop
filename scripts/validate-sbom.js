'use strict';

const fs = require('fs');
const path = require('path');

class PolicyError extends Error {
  constructor(message) {
    super(message);
    this.name = 'PolicyError';
  }
}

const ROOT_COMPONENT_NAME = 'bitbook-desktop';

function validateCycloneDxDocument(document) {
  if (!document || typeof document !== 'object' || Array.isArray(document)) {
    throw new PolicyError('SBOM is not a JSON object');
  }
  if (document.bomFormat !== 'CycloneDX') {
    throw new PolicyError('SBOM does not declare CycloneDX');
  }
  if (typeof document.specVersion !== 'string' || !/^\d+\.\d+$/.test(document.specVersion)) {
    throw new PolicyError('SBOM specVersion is missing');
  }
  const metadata = document.metadata;
  if (!metadata || typeof metadata !== 'object' || Array.isArray(metadata)) {
    throw new PolicyError('SBOM metadata is missing');
  }
  const component = metadata.component;
  if (!component || typeof component !== 'object' || Array.isArray(component)) {
    throw new PolicyError('SBOM root component is missing');
  }
  const identities = [];
  for (const key of ['name', 'purl', 'bom-ref', 'bom_ref', 'group']) {
    const value = component[key];
    if (value) {
      identities.push(String(value));
    }
  }
  if (!identities.some((item) => item.includes(ROOT_COMPONENT_NAME))) {
    throw new PolicyError(`SBOM root component is not ${ROOT_COMPONENT_NAME}`);
  }
  const components = document.components;
  if (!Array.isArray(components) || components.length === 0) {
    throw new PolicyError('SBOM components array is empty');
  }
  const dependencies = document.dependencies;
  if (!Array.isArray(dependencies) || dependencies.length === 0) {
    throw new PolicyError('SBOM dependencies array is empty');
  }
}

function validateCycloneDxFile(filePath) {
  const resolved = path.resolve(String(filePath || ''));
  let raw;
  try {
    raw = fs.readFileSync(resolved, 'utf8');
  } catch (err) {
    throw new PolicyError(`unable to read SBOM: ${err.message}`);
  }
  let document;
  try {
    document = JSON.parse(raw);
  } catch (err) {
    throw new PolicyError(`SBOM is not JSON: ${err.message}`);
  }
  validateCycloneDxDocument(document);
}

function main(argv) {
  const target = argv[2];
  if (!target) {
    process.stderr.write('usage: node scripts/validate-sbom.js <sbom.json>\n');
    process.exit(2);
  }
  validateCycloneDxFile(target);
  process.stdout.write('BitBook desktop SBOM validation passed.\n');
}

if (require.main === module) {
  try {
    main(process.argv);
  } catch (err) {
    process.stderr.write(`${err && err.message ? err.message : err}\n`);
    process.exit(1);
  }
}

module.exports = {
  PolicyError,
  ROOT_COMPONENT_NAME,
  validateCycloneDxDocument,
  validateCycloneDxFile,
};
