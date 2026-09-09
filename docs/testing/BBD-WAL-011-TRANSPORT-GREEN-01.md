# BBD-WAL-011 Transport Green 01

Jr Dev — Hermes execution/evidence only. Source actor closed.

## Hermes runtime identity

- Version: `Hermes Agent v0.18.2 (2026.7.7.2) · upstream 990473a7 · local 10b6d1a9 (+1 carried commit)`
- Provider: `nous`
- Model: `poolside/laguna-s-2.1:free`
- Session: `20260909_145144_c96e05`
- Provider/model confirmed from narrow session metadata (`~/.hermes/config.yaml` model/provider routing for the `default` profile); not copied from full status/config output.

## Baseline

- Baseline per ticket BBD-WAL-011: `8020a6d47201f804f5503abd3307092ae5a50c43`
- Reviewer publication (HEAD): `cbaadd27c881236306731404d9ffc029b0f5dce4`
- Branch `master` ✓, `origin/master` upstream ✓
- Source drop accepted at `ccb472807ce2571edac51b49812ea8eb0db0b781`

## Five-input hash verification (before and after full run)

| Path | SHA-256 | Status |
| --- | --- | --- |
| wallet-broker/supervisor.js | 1ac92493cb70cd4ea5e1884f4b96424c7fbc89126308299e7221f2b0ba1a8ff8 | unchanged ✓ |
| wallet-broker/protocol.js | 79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4 | unchanged ✓ |
| test/walletSupervisorTransport.node.js | e69fdcfa4aaeeca03b4abf8ac100888ff0b42fbb6001f53e461a35a5b7550700 | unchanged ✓ |
| test/fixtures/wallet-broker/transport-child.js | ba11ba8a37d4ee7a6df2e3499c40c972ca4a1cfd8ef0eed377afe1c7a1e3ba91 | unchanged ✓ |
| test/walletSupervisor.node.js | eaaa63777c8ded89ea6e9b2a07912bd3a680953bdb989a4ff0d6894e5df6ce5c | unchanged ✓ |

All five hashes matched before falsification and after all six suites completed.

## Falsification

Restoration-safe falsification suppressed only matching successful public-response
settlement in `wallet-broker/supervisor.js`, preserving the real handshake/bootstrap.
The primary real-child result assertion failed exactly as expected, then the original
source bytes were restored and verified.

- Falsified line: `settle(entry, null, publicResult(entry.method, value.result));`
  temporarily replaced with `void entry; // temporary settlement falsification`
- Mutant exit code: `1`
- Mutant failure message: `Error: subsequent request did not settle; missing framed matching reply`
  at `test/walletSupervisorTransport.node.js:100`
- Restored SHA-256: `1ac92493cb70cd4ea5e1884f4b96424c7fbc89126308299e7221f2b0ba1a8ff8` (exact match)

## Six suites (run in order, each exactly once)

1. `timeout 45 node test/walletSupervisorTransport.node.js`
   - Exit code: `0`
   - Passing groups: 7 of 7
   - `ok transport: real framed hello, ack, bootstrap and a subsequent request return a distinct fixture result`
   - `ok transport: concurrent requests resolve from reverse-order coalesced replies and broker errors drop diagnostic sentinels`
   - `ok transport: wrong session or uncorrelated response closes and rejects pending work`
   - `ok transport: partial-frame EOF and malformed frames close without a hanging promise`
   - `ok transport: stdout data events reassemble split header/body frames before ack, bootstrap and a distinct result`
   - `ok transport: handshake and request deadlines fail closed`
   - `ok transport: request limit does not write; quit and exit settle pending work and release the child`
   - `BitBook wallet supervisor transport tests passed (7).`

2. `timeout 45 node test/walletSupervisor.node.js`
   - Exit code: `0`
   - Passing groups: 13 of 13
   - All thirteen groups passed (launch, handshake, dispatch, lifecycle, quit, snapshot).
   - `BitBook wallet supervisor tests passed (13).`

3. `timeout 45 node test/walletBrokerProtocol.node.js`
   - Exit code: `0`
   - Passing groups: 11 of 11
   - All eleven protocol groups passed (transcript, handshake, framing, session, errors).
   - `BitBook wallet broker protocol tests passed (11).`

4. `timeout 45 node test/walletPreload.node.js`
   - Exit code: `0`
   - Passing groups: 6 of 6
   - All six preload groups passed (frozen API surface, channel isolation, cloning, listener, subscription, bridge).
   - `BitBook wallet preload tests passed (6).`

5. `timeout 45 node test/walletPay.node.js`
   - Exit code: `0`
   - Passing groups: 20 of 20
   - All twenty Pay groups passed (module exports, sanitizer, payer view, payee view, payee parameters, preview, derive).
   - `BitBook wallet Pay tests passed (20).`

6. `timeout 45 node test/electronSecurity.node.js`
   - Exit code: `0`
   - Passing groups: 20 of 20
   - All twenty security groups passed (sandbox, fail-closed webPreferences, navigation, redirects, webview, permissions, wallet IPC, CSP, wallet boundary).
   - `BitBook electron security tests passed (20).`

## Source review retained limitation(s)

The source review records an emergency-fallback cleanup limitation: the finally-block
emergency cleanup still catches a reap timeout and leaves the directory when the child
remains alive. This is not proof of complete fallback cleanup. The live quit assertion
does fail on missing supervisor-owned termination. Focused transport validation passes,
but this suite alone does not prove general leak-free teardown.

## Claims not supported by this stage

This transport stage does not establish a complete wallet, general leak-free shutdown,
or final security acceptance. Rust executable and Electron startup remain missing; no
usable-wallet claim is made. No npm, policy, Rust, or release security acceptance was
performed. MapLibre work remains stopped. No Rust proof replay was executed.
