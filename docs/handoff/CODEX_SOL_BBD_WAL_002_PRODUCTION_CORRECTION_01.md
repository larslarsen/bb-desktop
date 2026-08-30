# Codex Sol Handoff — BBD-WAL-002 Production Correction 01

You are temporary **Sr Dev — Codex Sol**, using `gpt-5.6-sol` at High. This file is the
complete durable production-correction prompt; ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Read completely before acting: `AGENTS.md`, `TESTING.md`,
`docs/engineering/DEVELOPMENT_ROLES.md`, `tickets/BBD-WAL-002.md`,
`docs/architecture/BBD-WAL-001-REVIEW.md`, `docs/handoff/CURRENT_TASK.md`, the original
production handoff, Corrections 04–05, the correction-red handoff/evidence, all accepted
tests/fixture, and all six current wallet-contract modules.

Reviewer accepts the correction expected red at evidence commit
`3207667e276cffcb438988610cfa90a64e130ffd`. Correct only:

- `wallet-contract/model.js`
- `wallet-contract/state-machine.js`
- `wallet-contract/fakes.js`

Do not edit canonical/framing/index, any test or fixture, package/lock files, workflows,
policy, evidence, tickets, handoffs, documentation, or any other path.

The accepted wallet test is 1,697 lines at SHA-256
`3e51281d16da7eec4a178eeb799ec23e2854206a096ed741cba920fc35825ee9`. Preserve the
fixture and protected test/lockfile hashes named in the correction-red handoff.

## 1. Bind account, request, adapter, signer, and prepared review

- Treat the request/account inputs as immutable bound values; do not let caller mutation
  silently retarget an intent.
- Validate and hash the bound `PaymentRequestV1` with the canonical decoder and compute
  its memo SHA-256 from the bound memo bytes.
- Before adapter prepare, require a known account kind, `adapter.asset === account.asset`,
  and `signer.kind === account.kind`. Fail substitution as `CAPABILITY_MISSING` before an
  adapter or signer call.
- After adapter prepare and standalone `ReviewImageV1` validation, compare every
  request/account-derived authoritative value: `account_id`, `request_id`, recomputed
  `payment_request_hash`, payer/payee peer IDs, asset, network, amount, receiver,
  receiver kind, expiry, and recomputed `memo_hash`. Any schema-valid mismatch is
  `INTENT_MISMATCH`; release the account lock and make no signer/broadcast call.
- Preserve the adapter-owned exact fee/bound, IDs, prepared time, change policy,
  transaction version, and pool fields after their standalone schema validation.

## 2. Make the synthetic capability model exact and fail closed

- Accept only `software`, `hardware_backed`, and `watch_only` account kinds and only
  valid asset/network relations.
- WAL-002 is an offline synthetic reference, not a mainnet capability claim. For its
  committed fixtures, require exact `nu6.3-test-fixture` and `v6-fixture` equality for
  ZEC; no substring or truthy-version acceptance. Require exact `xmr-fixture-hf` for the
  synthetic XMR branch as well.
- Hardware ZEC spending additionally requires
  `can_verify_pczt_on_device === true`. Hardware remains explicitly probed and present;
  no vendor-name inference and no software fallback.
- Preserve watch-only receive-but-never-spend, migration, network mismatch, disconnect,
  and quote-independent results and stable error codes.

## 3. Keep recovery locked and require fresh authority

- A machine owning the per-account fake prepare lock keeps it when crashing from
  `signed_unverified` or `verified` into `crash_recovery`.
- A synthetic restored `crash_recovery` machine has no process-local ownership and must
  acquire the same adapter/account lock during `confirmRecovery`. If occupied, return
  `ACCOUNT_BUSY`, remain in `crash_recovery`, do not increment confirmation count, and do
  not verify or broadcast. If free, acquire it before returning to `signed_unverified`.
- `cancel()` and `expire()` are valid terminal transitions from `crash_recovery` and
  release an owned lock.
- Repeated `crash()` while already in `crash_recovery` is an `ok: true` inert no-op that
  leaves the complete snapshot unchanged.
- A machine created from synthetic durable `verified` state may not call adapter
  broadcast until `crash()` moves it to recovery and a fresh `confirmRecovery` plus
  verification completes. A normal in-process verified happy path remains unchanged.
- Preserve `broadcasting -> unknown_needs_scan`, no blind resubmit, and inert fake
  broadcast behavior.

## 4. Normalize injected failures and release locks

Catch exceptions from the injected request-status source, clock, adapter prepare/verify/
broadcast, and signer. Return stable closed failure objects (normally `UNAVAILABLE`),
enter a non-spendable state, release any process-local lock, expose no exception text or
internals, and never retry or make a later broadcast call. Preserve already-normalized
result error codes. Do not catch/normalize the canonical decoder's public API errors;
that API deliberately throws `ContractError`.

## 5. Sanitize field values without invoking code

`sanitizeLog` must inspect own property descriptors and copy only own data-property
strings. Never read/invoke accessors and never copy inherited properties, objects,
arrays, functions, C0/C1/control-bearing strings, or arbitrary canary-shaped values.
Use explicit lexical/enum allowlists for synthetic account/intent/request identifiers,
state, and stable error codes so the accepted valid event survives as an ordinary
`Object.prototype` object. Do not include adapter/signer internals or add new log fields.

## Absolute exclusions

Keep all adapters synthetic and every broadcast incapable of success. No real address,
key, transaction, coin library, dependency, filesystem, socket, network, HTTP, fetch,
WebSocket, process, worker, Electron, USB/HID, device, rate, timer, listener, or generated
artifact. Do not weaken or special-case tests.

Use `apply_patch` for edits. Read-only shell commands are authorized only to read the
required/current files and report `wc -l`/`sha256sum` over the three authorized paths and
protected hashes. Do not execute tests, Node, npm, builds, installs, formatters,
scanners, Git, GitHub, network, wallet, daemon, subprocess, hardware, or device commands.
Do not use `/tmp`, root, `sudo`, deletion, cleanup, `rm`, globs, environment-variable
targets, or unresolved paths.

Stop after authoring. Report each changed path with line count/SHA-256, how every red
cause is closed, lock/recovery and inert-broadcast properties, all protected hashes, and
confirmation that nothing ran and no other path changed. Reviewer XHigh must inspect the
source before Luna may run any green command.
