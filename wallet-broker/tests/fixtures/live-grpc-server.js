'use strict';

const fs = require('node:fs');
const http2 = require('node:http2');

const MAX_CONFIG_BYTES = 4 * 1024 * 1024;
const MAX_REQUEST_BYTES = 64 * 1024;
const PREFIX = '/cash.z.wallet.sdk.rpc.CompactTxStreamer/';
const METHODS = new Set([
  `${PREFIX}GetLightdInfo`,
  `${PREFIX}GetLatestBlock`,
  `${PREFIX}GetTreeState`,
  `${PREFIX}GetBlockRange`,
]);

let input = Buffer.alloc(0);
process.stdin.on('data', (chunk) => {
  if (input.length + chunk.length > MAX_CONFIG_BYTES) {
    process.stderr.write('configuration exceeds fixture limit\n');
    process.exit(2);
  }
  input = Buffer.concat([input, chunk]);
});

process.stdin.on('end', () => {
  let config;
  try {
    config = JSON.parse(input.toString('utf8'));
    validateConfig(config);
  } catch (error) {
    process.stderr.write(`invalid fixture configuration: ${error.message}\n`);
    process.exit(2);
    return;
  }

  const server = http2.createSecureServer({
    key: fs.readFileSync(config.key_path),
    cert: fs.readFileSync(config.cert_path),
    allowHTTP1: false,
  });

  server.on('stream', (stream, headers) => {
    const path = headers[':path'];
    const chunks = [];
    let length = 0;
    let rejected = false;

    // Client-side deadline/cancellation resets are expected in the stall scenario.
    stream.on('error', () => {});

    stream.on('data', (chunk) => {
      if (rejected) return;
      length += chunk.length;
      if (length > MAX_REQUEST_BYTES) {
        rejected = true;
        stream.close(http2.constants.NGHTTP2_CANCEL);
        return;
      }
      chunks.push(chunk);
    });

    stream.on('end', () => {
      if (rejected) return;
      const body = Buffer.concat(chunks);
      report({ event: 'request', path, body: body.toString('base64') });
      if (headers[':method'] !== 'POST' || !METHODS.has(path)) {
        finish(stream, [], '12');
        return;
      }

      if (path === `${PREFIX}GetBlockRange`) {
        const messages = config.blocks.map(decodeBase64);
        if (config.range_mode === 'stall') {
          stream.respond({ ':status': 200, 'content-type': 'application/grpc' },
            { waitForTrailers: true });
          stream.write(frame(messages[0]));
          report({ event: 'stalled', path });
          return;
        }
        finish(stream, messages, '0');
        return;
      }

      const field = path.endsWith('GetLightdInfo')
        ? 'lightd_info'
        : path.endsWith('GetLatestBlock')
          ? 'latest_block'
          : 'tree_state';
      finish(stream, [decodeBase64(config[field])], '0');
    });
  });

  server.on('error', (error) => {
    process.stderr.write(`fixture server error: ${error.message}\n`);
    process.exitCode = 3;
  });
  server.listen(0, '127.0.0.1', () => {
    const address = server.address();
    report({ event: 'ready', port: address.port });
  });
});

function validateConfig(config) {
  if (!config || typeof config !== 'object') throw new Error('object required');
  for (const field of ['cert_path', 'key_path', 'lightd_info', 'latest_block', 'tree_state']) {
    if (typeof config[field] !== 'string' || config[field].length === 0) {
      throw new Error(`${field} is required`);
    }
  }
  if (!Array.isArray(config.blocks) || config.blocks.length < 1 || config.blocks.length > 2) {
    throw new Error('one or two block messages required');
  }
  if (!['complete', 'stall'].includes(config.range_mode)) {
    throw new Error('invalid range mode');
  }
  for (const value of [config.lightd_info, config.latest_block, config.tree_state, ...config.blocks]) {
    decodeBase64(value);
  }
}

function decodeBase64(value) {
  if (typeof value !== 'string' || !/^(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?$/.test(value)) {
    throw new Error('non-canonical base64');
  }
  return Buffer.from(value, 'base64');
}

function frame(message) {
  const header = Buffer.alloc(5);
  header.writeUInt32BE(message.length, 1);
  return Buffer.concat([header, message]);
}

function finish(stream, messages, status) {
  stream.respond({ ':status': 200, 'content-type': 'application/grpc' },
    { waitForTrailers: true });
  stream.on('wantTrailers', () => stream.sendTrailers({ 'grpc-status': status }));
  for (const message of messages) stream.write(frame(message));
  stream.end();
}

function report(value) {
  process.stdout.write(`${JSON.stringify(value)}\n`);
}
