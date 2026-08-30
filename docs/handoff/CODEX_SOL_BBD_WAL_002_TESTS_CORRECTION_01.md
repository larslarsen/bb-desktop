# Codex Sol Handoff — BBD-WAL-002 Test Source Correction 01

You are temporary **Sr Dev — Codex Sol**, using `gpt-5.6-sol` at High. This file is the
complete durable correction prompt; ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Read completely before acting: `AGENTS.md`, `TESTING.md`,
`docs/engineering/DEVELOPMENT_ROLES.md`, `tickets/BBD-WAL-002.md`,
`docs/architecture/BBD-WAL-001-REVIEW.md`, `docs/handoff/CURRENT_TASK.md`, and this file.

The initial drop is bounded correctly but not accepted. Its reviewed source is:

- `test/fixtures/wallet-contract/golden-v1.json`: 77 lines,
  SHA-256 `319bec9c3ac29015328f58f23f29f37466e75b5299c41817ad3ec7e58d02bb7e`
- `test/walletContract.node.js`: 994 lines,
  SHA-256 `55829dd0071ac37ce7fd6d30da079bf62eb89864f02805770d2bdc751e22609b`
- `test/electronSecurity.node.js`: 561 lines,
  SHA-256 `0fca21e2c8a64c03dae896d4c88463b96fcf7179669e162aab56e721f8356dfd`
- `test/securityPolicy.node.js`: 1,289 lines,
  SHA-256 `dc767348df7ad02c522ea12fb3da3b7a1383ce27857e95497687857ce22a40d8`

Correct only these same four authorized paths. Preserve sound existing coverage and make
these changes:

1. Replace the vacuous malformed-UTF-8 signed-object input with an otherwise byte-for-byte
   valid `PaymentRequestV1` whose memo bytes contain an invalid sequence. Prove the
   neighboring valid request succeeds once implementation exists, so missing fields or
   JSON structure cannot explain the failure. Keep BOM, duplicate-key, trailing-data,
   CBOR-looking, and non-object cases.
2. Expand `golden-v1.json` with fixture-driven invalid vectors used directly by the Node
   tests and suitable for later Go/Rust parity. Each records a stable name, object kind,
   classification/expected code, input representation (`input`, `raw_json`, or hex bytes
   where JSON cannot represent it), and reason. Cover at least: duplicate key; malformed
   UTF-8; unknown/missing field; zero/leading-zero/scientific amount; invented Ironwood
   receiver kind; request status field; impossible/out-of-range timestamp; non-NFC memo;
   bidi/format control; invalid status/tx-ref relation; fee above bound; Orchard v1 pool;
   and rate/fiat field. Positive vectors keep exact canonical strings and hashes. The
   fixture preflight must validate the metadata and independently hash positives before
   requiring production.
3. Table-drive every missing field for all three closed schemas. Exercise wrong JSON
   types for every declared scalar field, `v`, and `zec_pools`; apply strict calendar
   rejection to timestamps in request, status, and review objects. Add direct valid XMR
   request and XMR review decoding with `xmr-stagenet`, `xmr-subaddress`, `xmr_change`,
   `tx_version: "0"`, and empty `zec_pools`.
4. Make the intent machine contract take an injected, re-readable request-status source.
   Change it from `open` to `cancelled` after `completeSign` but before verify, and again
   after verify but before broadcast. Both operations must become `CANCELLED` with zero
   broadcast calls. Keep explicit cancellation across every pre-broadcast state. This is
   the source basis for the required cancel-recheck falsification.
5. In crash recovery, mutate a recovered signed artifact and require fresh confirm plus
   revalidation to fail `INTENT_MISMATCH` with zero broadcast. Also prove injected
   cancellation and injected expiry win while in `crash_recovery`.
6. Remove `deferBroadcast` and every fake-adapter success or pending-success path.
   `broadcast()` must always return `ok: false`, `funds_moved: false`, and exactly
   `UNAVAILABLE` or `CAPABILITY_MISSING`. Exercise `broadcasting -> unknown_needs_scan`
   by an explicit restored-state/test-fixture constructor that begins in a synthetic
   durable `broadcasting` state without calling adapter broadcast; crash/resume must not
   submit or resubmit, so broadcast call count stays zero.
7. Strengthen package/security expectations. Require exact `test:wallet`, require the
   top-level `test` script to include it alongside existing social/security suites, and
   require the build syntax path to cover every maintained wallet-contract module.
   Require the policy checker and routine social CI to enforce those commands. Extend
   forbidden-source checks to CommonJS, `node:`-prefixed, and dynamic `import()` forms of
   child-process, network/socket, Electron, USB/HID/device, worker-thread, and filesystem
   capabilities; the pure reference contract needs only safe Node primitives such as
   `crypto` and buffers.
8. Keep test assertions behavioral and non-vacuous. Do not add production stubs, weaken
   errors to “throws anything,” inspect source instead of exercising the contract, or
   make tests conditional on absent implementation.

Authorization remains test source only:

- `test/fixtures/wallet-contract/golden-v1.json`
- `test/walletContract.node.js`
- `test/electronSecurity.node.js`
- `test/securityPolicy.node.js`

Read-only shell commands are authorized solely to read the exact required repository
documents and current/authorized test files, plus `wc -l` and `sha256sum` over the four
authorized paths for the final report. Use `apply_patch` for edits. Do not execute tests,
Node, npm, builds, installs, formatters, scanners, Git, GitHub, network, wallet, node,
subprocess, hardware, USB/HID, or device commands. Do not use `/tmp`, root, `sudo`,
deletion, cleanup, `rm`, globs, environment-variable targets, or unresolved paths.

Stop after correction. Report the four paths, new line counts and SHA-256 hashes, exact
test counts/categories, how each blocker was resolved, expected red causes, and
confirmation that no command or out-of-scope action ran. Reviewer Codex at XHigh must
accept the corrected source before Luna executes anything.
