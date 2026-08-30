# BBD-WAL-003 Falsification Evidence

Timestamp: 2026-08-30T16:58:06-0700 (PDT)
Accepted implementation commit: `584019e9a89022d77b4bbb6710c2b7670e42d95b`
GitHub Social client run `33342988248`: routine check passed; package jobs skipped.

## Falsifications and restorations

Each mutation was applied with `apply_patch`, tested only with its named Node suite,
then restored immediately and hash-checked. Each restoration rerun passed.

1. Swapped `parentPid`/`childPid` in `computeSessionId()`: protocol exit 1, 10 pass /
   1 fail, only the independent transcript preimage test failed. Restored
   `wallet-broker/protocol.js` SHA-256 `79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4`;
   rerun 11/11 passed.
2. Spawned once before hash verification in the injected supervisor boundary: supervisor
   exit 1, 9 pass / 2 fail, only the two launch-order/no-spawn tests failed. Restored
   `wallet-broker/supervisor.js` SHA-256 `773a4815e9ca89752b28fcdbaaf19dcc347e648e4226314f7da78241f23d5520`;
   rerun 11/11 passed.
3. Added `wallet:intent:confirm` IPC registration: Electron exit 1, 17 pass / 2 fail,
   only exact-channel and forbidden-authority tests failed. Restored `social-main.js`
   SHA-256 `ef3c12eb00fe5ea990399bc8f4821d5574aa7bc79c353554e42851d8407e8397`;
   rerun 19/19 passed.

Final required `npm test` exited 0: social, security, wallet, and wallet-broker suites
passed. No canary appeared, no real process ran, and no out-of-scope change occurred.
