'use strict';

const assert = require('assert');
const core = require('../social/core');

assert.strictEqual(core.normalizeAPIURL(''), 'http://127.0.0.1:4002');
assert.strictEqual(core.normalizeAPIURL('https://example.test/base///?ignored=yes'), 'https://example.test/base');
assert.throws(() => core.normalizeAPIURL('file:///tmp/socket'), /HTTP or HTTPS/);
assert.strictEqual(core.socketURL('http://127.0.0.1:4002'), 'ws://127.0.0.1:4002/ws');
assert.strictEqual(core.socketURL('https://example.test/api'), 'wss://example.test/ws');
assert.strictEqual(core.shortID('1234567890abcdef', 4), '1234…cdef');
assert.strictEqual(core.postText({ status: 'hello' }), 'hello');
assert.strictEqual(core.displayName({ handle: 'ada' }, 'peer'), '@ada');

const sorted = core.sortFeed([
  { status: 'old', timestamp: '2025-01-01T00:00:00Z' },
  { status: 'new', timestamp: '2026-01-01T00:00:00Z' },
]);
assert.deepStrictEqual(sorted.map(item => item.status), ['new', 'old']);

process.stdout.write('BitBook social core tests passed.\n');
