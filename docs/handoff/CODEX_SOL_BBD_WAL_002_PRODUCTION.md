# Codex Sol Handoff — BBD-WAL-002 Production Source

You are temporary **Sr Dev — Codex Sol**, using `gpt-5.6-sol` at High. This file is the
complete durable production-source prompt; ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Read completely before acting: `AGENTS.md`, `TESTING.md`,
`docs/engineering/DEVELOPMENT_ROLES.md`, `tickets/BBD-WAL-002.md`,
`docs/architecture/BBD-WAL-001-REVIEW.md`, `docs/handoff/CURRENT_TASK.md`,
`docs/testing/BBD-WAL-002-RED-EVIDENCE.md`, and all four accepted test paths.

The expected red is accepted. Author the smallest dependency-free green implementation
that satisfies the observable contract in the accepted tests and architecture. You may
create or edit only:

- `wallet-contract/canonical.js`
- `wallet-contract/framing.js`
- `wallet-contract/model.js`
- `wallet-contract/state-machine.js`
- `wallet-contract/fakes.js`
- `wallet-contract/index.js`
- `package.json`
- `.github/workflows/social.yml`
- `.github/workflows/security.yml`
- `scripts/security-policy.js`

Do not edit the four accepted test paths, `package-lock.json`, evidence, tickets,
handoffs, documentation, packaging scripts, SBOM workflow, Electron UI/main/preload,
`bb-go`, or any other path.

## Required implementation

Expose exactly the CommonJS API consumed by `test/walletContract.node.js`:

- `decodeSignedObject`
- `encodeFrame`
- `createFrameDecoder`
- `evaluateCapability`
- `createIntentMachine`
- `createFakeAdapter`
- `createFakeSigner`
- `sanitizeLog`

Keep the six files meaningfully separated and use only literal allowlisted imports:
crypto/buffer and exact `./wallet-contract` siblings. `index.js` is the façade; do not
duplicate all behavior into it or leave required siblings as token files.

### Canonical objects

- Strictly decode UTF-8 bytes without BOM, replacement, trailing JSON, duplicate object
  keys, non-object roots, comments, or CBOR.
- Implement the exact three closed schemas, field types, lexical constraints, all six
  network enums, asset/network/receiver/change/transaction/pool relations, fee bound,
  status/tx-ref relations, TimestampV1 Gregorian/range/round-trip rules, Unicode/NFC/
  prohibited-codepoint rules, and no-rate fields expressed by the tests and architecture.
- Implement RFC 8785/JCS for the deliberately restricted v1 value domain and the exact
  newline-terminated domain-separated SHA-256 digests. Do not use floating-point money.
- Return stable errors with exact `.code`; Orchard-only v1 review returns
  `MIGRATION_REQUIRED`, while schema failures return `SCHEMA`.

### Framing, capabilities, and state

- Implement incremental four-byte big-endian framing with strict JSON-object UTF-8,
  64-KiB requested control limits, an unraiseable 1-MiB ceiling, exact-bound acceptance,
  preservation of unread bytes, and permanently closed failure state.
- Implement the dual-coin/account-kind capability result contract. Capability flags—not
  vendor names—decide eligibility; watch-only, current NU6.3/v6/Ironwood/PCZT, migration,
  device presence/probe, network mismatch, and quote independence must match tests.
- Implement the pure prepare-before-confirm state machine, injected clock and request-
  status reads, exact state/error transitions, per-account prepare locking, post-sign and
  pre-broadcast cancellation/expiry checks, intent-hash verification, restored crash
  states, fresh recovery confirmation, and no blind resubmit.
- Release any process-local fake account lock when an intent reaches a terminal or
  recovered non-owning state so tests do not leave global state across independent
  machines. Do not add timers, listeners, files, or processes.

### Inert fakes and secrets

- Fake artifacts are tagged synthetic objects only. No raw/mainnet transaction encoding,
  coin library, address parser, key material, RPC, socket, filesystem, subprocess,
  Electron, USB/HID, worker, or device capability.
- Every fake adapter broadcast returns `ok: false`, `funds_moved: false`, and only
  `UNAVAILABLE` or `CAPABILITY_MISSING`. No option or restored state may enable success.
- Watch-only never signs; hardware disconnect never falls back. Verification compares
  every authoritative review field against the confirmed intent hash.
- `sanitizeLog`, failures, and snapshots expose only the accepted non-secret allowlist
  and never copy adapter/signer internals.

### Package, workflow, and policy wiring

- Add exact `test:wallet: node test/walletContract.node.js`.
- Set top-level `test` exactly to
  `npm run test:social && npm run test:security && npm run test:wallet`.
- Add exact `node --check` build segments for all six wallet modules without removing
  existing build checks.
- Add `wallet-contract/**` to the matching ordered path-filter contracts in both routine
  workflows and run `npm run test:wallet` in the routine social check. Do not enable
  routine packaging, artifacts, installs, scanners, or SBOM generation.
- Extend `scripts/security-policy.js` with the exact exported wallet path, build-command,
  and import-allowlist contracts. Refactor/export `checkPackageJson(packageText)` while
  keeping `checkRepository(root)` functional. Implement/export
  `checkWalletContractSource(source, rel)` and invoke it over every required wallet file
  from repository checking. Require the wallet command/path in the existing social and
  security workflow checks. Preserve every existing security/SBOM/Gitleaks invariant.

## Absolute exclusions

No dependency or lockfile change; no Rust, Go, coin package, RPC, socket, HTTP, fetch,
WebSocket, filesystem, process, worker, Electron, hardware, real address/key/transaction,
rate source, mainnet operation, package build, or generated artifact. Do not weaken or
special-case a test. Do not inspect environment secrets.

Read-only shell commands are authorized solely to read the exact required repository
documents, accepted tests, and current authorized production files, plus `wc -l` and
`sha256sum` over the authorized paths for final reporting. Use `apply_patch` for edits.
Do not execute tests, Node, npm, builds, installs, formatters, scanners, Git, GitHub,
network, wallet, node daemon, subprocess, hardware, USB/HID, or device commands. Do not
use `/tmp`, root, `sudo`, deletion, cleanup, `rm`, globs, environment-variable targets,
or unresolved paths.

Stop after authoring. Report every changed path with line count and SHA-256, exported
API/policy contracts, state/fake safety properties, and confirmation that accepted test
hashes and `package-lock.json` remained unchanged and nothing ran. Reviewer Codex at
XHigh must inspect source before Luna executes green or falsification commands.
