'use strict';

const { decodeSignedObject } = require('./canonical');
const { createFrameDecoder, encodeFrame } = require('./framing');
const { evaluateCapability } = require('./model');
const { createIntentMachine } = require('./state-machine');
const { createFakeAdapter, createFakeSigner, sanitizeLog } = require('./fakes');

module.exports = {
  decodeSignedObject,
  encodeFrame,
  createFrameDecoder,
  evaluateCapability,
  createIntentMachine,
  createFakeAdapter,
  createFakeSigner,
  sanitizeLog,
};
