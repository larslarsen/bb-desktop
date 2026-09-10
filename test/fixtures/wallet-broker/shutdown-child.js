'use strict';

// Install before requiring transport-child so hello cannot precede the handler.
if (process.argv.includes('--ignore-sigterm')) {
  process.on('SIGTERM', () => {});
}

require('./transport-child.js');
