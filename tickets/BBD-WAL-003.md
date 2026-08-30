# BBD-WAL-003 — Fail-Closed Wallet Broker Boundary

Status: TEST SOURCE REVIEWER ACCEPTED — EXPECTED RED RESUME 01 AUTHORIZED

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Test source actor: Principal Dev — Codex Sol (`gpt-5.6-sol`, High)

Integration actor: Jr Dev — Codex Luna (`gpt-5.6-luna`)

Source baseline: `d472785ab896bb5d1367c4117ffd659a9a8512ae`

Architecture: `docs/architecture/BBD-WAL-001-REVIEW.md` §§4.1–4.3 and 5.3–5.4

## Objective

Define and then implement the smallest Electron-to-native-broker boundary that can
supervise a future Rust wallet broker without putting wallet authority in Electron. The
boundary verifies a packaged binary before spawn, uses inherited anonymous pipes with a
separate diagnostics stream, binds the version-one session transcript, exposes only six
fixed renderer capabilities, and fails closed on every generic or authority-bearing IPC
attempt.

This ticket does not create a wallet. Missing broker binaries produce an unavailable
snapshot. No key, wallet, node, device, transaction, rate provider, or real coin binary is
used. A later reviewed production phase may add an inert native broker bootstrap; this
test phase must not install Rust or claim that a Node child is the wallet broker.

## Fixed security contract

- Broker frames are four-byte big-endian length plus one UTF-8 JSON object. The absolute
  limit is 1 MiB and non-transaction control methods are capped at 64 KiB. Unknown fields,
  duplicate IDs, invalid sequence numbers, invalid kinds, mixed diagnostics, and trailing
  JSON fail closed.
- Child speaks first. Protocol is exactly `bitbook-wallet-broker`, version overlap is
  exactly v1, nonces are 32 lowercase hex, and PIDs are canonical positive decimal
  strings. The first post-hello message in each direction binds the exact session ID.
- The independent transcript fixture uses parent PID `41001`, child PID `41002`, parent
  nonce `00112233445566778899aabbccddeeff`, child nonce
  `ffeeddccbbaa99887766554433221100`, and session ID
  `d1427a9ddeb9ed176859f95a6ccf9912e98e44377c8f2b173c40fa48245571df`.
- The broker path is an explicit packaged path plus an exact SHA-256 pin. Missing,
  non-regular, symlinked, unreadable, or hash-mismatched input fails before any spawn
  call. The test system boundary is injected; routine tests do not spawn a real binary.
- Spawn uses an explicit broker data directory, a cleaned allowlisted environment, no
  secret-bearing argv/environment value, and distinct inherited stdin/stdout protocol
  pipes plus stderr diagnostics. No TCP, HTTP, UDS, named-pipe listener, shell, or social
  daemon is a wallet dependency.
- Supervisor-callable broker methods are exactly `status.get`, `account.list`,
  `account.lock`, `receiver.fresh`, `intent.begin`, `intent.cancel`, and
  `sync.subscribe`. The extra `account.lock` path is supervisor idle/quit only and is not
  renderer-callable.
- The frozen renderer API is exactly `getSnapshot`, `subscribeSnapshot`, `beginIntent`,
  `cancelIntent`, `listAccounts`, and `getPayeeRequest`. Each maps to a fixed channel; the
  page supplies no channel or broker-method string.
- `intent.confirm`, `account.unlock`, `account.exportBackup`,
  `account.createSoftware`, `signer.sign`, `tx.broadcast`, and `intent.broadcast` are
  absent from preload and `ipcMain`. Native confirm is broker-owned and cannot be
  represented by an Electron channel, handler, window, or callback.
- Main-frame identity, local-page origin, closed input shapes, structured cloning, and
  the applicable byte limit are checked before dispatch. Subscription callbacks receive
  cloned data without Electron event objects and return a bounded unsubscribe function.
- Broker down/handshake failure emits only a sanitized unavailable snapshot. Restart
  backoff does not buffer or replay spend requests. Quit cancels in-flight intents before
  termination.
- Quote workers, rates, fiat values, provider hosts, exchange routes, coin adapters,
  transaction construction, custody, unlock, backup, signing, and broadcast are absent.

## Current authorization — test source only

Codex Sol may create or edit only:

- `test/fixtures/wallet-broker/transcript-v1.json`
- `test/walletBrokerProtocol.node.js`
- `test/walletSupervisor.node.js`
- `test/walletPreload.node.js`
- `test/electronSecurity.node.js`
- `test/securityPolicy.node.js`

The two existing security files must preserve all accepted BBD-SEC-001 and BBD-WAL-002
coverage. Their former blanket no-preload/no-IPC assertions must become a stronger exact
allowlist: every unlisted Electron API, channel, method, sender frame, shape, and
authority-bearing operation remains rejected.

No production source, package/lock file, workflow, policy implementation, Rust source,
binary, build, install, evidence, documentation, handoff, Git, GitHub, network, wallet,
node, child process, hardware, or device action is authorized in this phase.

## Required test groups

1. **Protocol and transcript:** golden session ID, field closure, version/PID/nonce
   boundaries, monotonic sequence and unique IDs, first-message session binding,
   correlation/cancel/deadline rules, split/coalesced frames, 64-KiB and 1-MiB boundaries,
   malformed UTF-8/JSON, diagnostics contamination, terminal decoder behavior, and safe
   error normalization.
2. **Supervisor ordering and lifecycle:** verify-before-spawn call order; all packaged
   binary failure modes; cleaned env/empty secret argv; protocol/diagnostic separation;
   child-first two-second handshake timeout; pre-bind method rejection; unexpected-exit
   down snapshot; bounded deterministic backoff with no request buffering; cancel then
   terminate on quit; and injected clocks/spawn/filesystem/hash boundaries with no real
   process or filesystem listener.
3. **Broker dispatch:** the exact supervisor method allowlist, strict parameter schemas,
   limits, session requirement, unknown method rejection, and explicit proof that
   confirm/unlock/backup/create/sign/broadcast/rate/raw/proxy methods never dispatch.
4. **Preload:** exactly six own frozen functions on one frozen object; fixed channels;
   no generic invoke/send/sendSync exposure; structured cloned arguments/results;
   bounded subscription/unsubscribe; event-object stripping; hostile callback and
   mutation cases; no Node/Electron/child handles on the page.
5. **Electron main:** sandbox remains enabled, the preload path is explicit and local,
   exact channel registration only, sender main-frame/origin checks, malformed/oversize
   payload rejection before supervisor calls, sanitized results, no Electron confirmation
   surface, and all existing navigation/permission/CSP/injection protections preserved.
6. **Policy/CI contract:** later maintained supervisor/preload paths, their syntax checks,
   exact test commands, routine workflow filters, and import/capability restrictions are
   required without adding a dependency or loosening the renderer CSP.

Tests use only Node built-ins and deterministic committed fixtures. They inspect behavior,
not production source text, except the existing security/policy tests whose accepted role
is enforcing repository invariants.

## Expected red and later acceptance

After reviewer acceptance of the test drop, Codex Luna will run these independently:

```text
node test/walletBrokerProtocol.node.js
node test/walletSupervisor.node.js
node test/walletPreload.node.js
node test/electronSecurity.node.js
node test/securityPolicy.node.js
```

The first three must exit nonzero because their reserved production modules do not exist;
the Electron and policy suites must exit nonzero only on the new exact-boundary
requirements, with every pre-existing assertion still passing. The source actor must not
run these commands.

A later production handoff will name the exact green and broader commands after test
source review. Acceptance will include targeted green, `npm test`, `npm run build`,
`npm audit --audit-level=high`, the existing security scanners/policy suite, and at least
these falsifications with exact restoration: session transcript input order changed,
binary spawn moved before hash verification, and one forbidden generic/confirm channel
added. No routine cross-platform package build is required.

## Test-source review status

The first six-path Sol drop and Corrections 01–02 were reviewer-rejected before execution.
Correction 03 is reviewer-accepted at the exact six hashes in
`docs/handoff/CODEX_LUNA_BBD_WAL_003_RED.md`. Only Codex Luna's expected-red integration
is authorized. The first four commands are accepted after the raw Electron inventory
confirmed 13 pass / 6 expected fail; only
`docs/handoff/CODEX_LUNA_BBD_WAL_003_RED_RESUME_01.md` may resume the policy red and
record sequence. Production remains unauthorized until the reviewer accepts that evidence.
