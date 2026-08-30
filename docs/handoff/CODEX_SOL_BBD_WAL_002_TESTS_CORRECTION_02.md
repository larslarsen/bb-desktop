# Codex Sol Handoff — BBD-WAL-002 Test Source Correction 02

You are temporary **Sr Dev — Codex Sol**, using `gpt-5.6-sol` at High. This file is the
complete durable correction prompt; ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Read completely before acting: `AGENTS.md`, `TESTING.md`,
`docs/engineering/DEVELOPMENT_ROLES.md`, `tickets/BBD-WAL-002.md`,
`docs/architecture/BBD-WAL-001-REVIEW.md`, `docs/handoff/CURRENT_TASK.md`, and this file.

Correction 01 source under review is:

- `test/fixtures/wallet-contract/golden-v1.json`: 231 lines,
  SHA-256 `08905d2b082fa6370211ebd494130d62e3696db0f8314636f3685ba5887f929f`
- `test/walletContract.node.js`: 1,219 lines,
  SHA-256 `ec0cf16f086efae2cfe521cf45b80879113d2606c7e02ed8eed896a2dbcf8a95`
- `test/electronSecurity.node.js`: 598 lines,
  SHA-256 `91f600e9b7504e02572b81aea2fcce9f9713f2ae70aa2abc8cb85da11cbc7931`
- `test/securityPolicy.node.js`: 1,373 lines,
  SHA-256 `0eede48259be4357de755f1640221e9c23506536da52c3675ccd774b77d90fe8`

Preserve every sound Correction 01 change. Correct only the same four authorized test
paths:

1. Restore every pre-existing assertion byte-for-byte. Specifically remove the added
   `npm run test:wallet` assertion from the old `routine social check keeps offline syntax
   and Node tests only` test. Keep the wallet command requirement in appended wallet tests
   after all inherited tests. Do not otherwise rewrite, reorder, or weaken old tests.
2. Add integer `0`, `2`, and `-1` to wrong-version cases for every signed schema. Add
   explicit ZEC review rejection for `zec_pools: []`, unknown pool members, and mixed or
   duplicate pools, preserving `MIGRATION_REQUIRED` for the exact Orchard-only case.
3. Add table-driven valid decode cases for all six `Network` enum values with the correct
   asset and receiver kind: `zec-mainnet`, `zec-testnet`, `zec-regtest`, `xmr-mainnet`,
   `xmr-stagenet`, and `xmr-testnet`. Add cross-asset/network/receiver mismatch failures
   for request and review objects. These are pure schema bytes only; do not add mainnet
   transaction, adapter, capability, signing, or broadcast behavior.
4. Add lexical negative cases for status `request_id`, `event_id`, and `nonce`; review
   `request_id`, `payment_request_hash`, and `memo_hash`; and blank review intent,
   prepared, account, payer, payee, and receiver strings. Complete invalid review
   asset/network/receiver/change/tx/pool combinations. Add positive decoding of
   `status: "paid"` with a nonempty synthetic `tx_ref` and `status: "expired"` with an
   empty `tx_ref`; no key-like or real transaction value.
5. Replace the forbidden-module blacklist contract with a fail-closed import allowlist in
   both appended security-test layers. Accepted module loads are literal
   `require`/static import/dynamic import of only `crypto`, `node:crypto`, `buffer`, and
   `node:buffer`. Reject computed `require(name)`, concatenated/template module names,
   computed `import(name)`, every non-allowlisted literal module, and the existing
   `fetch`/`WebSocket` capabilities. Include mutation assertions for the computed forms
   and positive assertions for all four allowed literal names. The later checker API may
   remain `checkWalletContractSource(source, rel)`.
6. Keep the fixture preflight before the absent implementation import, preserve all
   Correction 01 lifecycle/recovery/inert-fake assertions, and do not add a stub or make
   any test conditional.

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
