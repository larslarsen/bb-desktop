# Codex Sol Handoff — BBD-WAL-002 Test Source Correction 04

You are temporary **Sr Dev — Codex Sol**, using `gpt-5.6-sol` at High. This file is the
complete durable correction prompt; ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Read completely before acting: `AGENTS.md`, `TESTING.md`,
`docs/engineering/DEVELOPMENT_ROLES.md`, `tickets/BBD-WAL-002.md`,
`docs/architecture/BBD-WAL-001-REVIEW.md`, `docs/handoff/CURRENT_TASK.md`, all prior
BBD-WAL-002 test handoffs, `docs/testing/BBD-WAL-002-RED-EVIDENCE.md`, the accepted
fixture, the accepted test source, and the ten unaccepted production/wiring paths.

The first production drop is bounded correctly but is **not accepted**. Reviewer source
inspection found architecture-level fail-open behavior not exercised by the accepted
tests. Tests must lead the correction. Modify only:

- `test/walletContract.node.js`

Do not edit the accepted fixture, either security test, production source, package or
lock files, workflows, policy, evidence, tickets, handoffs, or any other path.

Preserve all 38 accepted wallet tests and add the smallest deterministic behavioral
coverage for every requirement below. Use the existing helpers and only synthetic data.
Do not inspect implementation source from assertions.

## 1. Bind a prepared review to the request and selected account

Table-drive otherwise schema-valid prepared reviews whose authoritative values differ
from the bound request/account. Cover at least:

- `account_id`;
- `request_id` and `payment_request_hash`;
- payer and payee peer IDs;
- amount and receiver;
- expiry and memo hash; and
- a fully schema-valid XMR review returned while a ZEC request/account is selected (or
  the inverse), so asset/network/receiver/change/transaction/pool relations are not
  rejected merely by the standalone review schema.

After `begin`, `prepare` must fail closed as `INTENT_MISMATCH`, with one adapter prepare,
zero signer calls, and zero broadcast calls. A standalone schema error may still be
`SCHEMA`; the vectors above must remain individually schema-valid so they specifically
prove cross-object binding. Also prove the positive review still prepares.

The contract must recompute the payment-request digest and memo digest from the bound
request; it must not trust hashes supplied only by the adapter.

## 2. Prevent account, adapter, and signer substitution

Add behavioral cases proving:

- an unknown account kind cannot spend;
- a hardware-backed account paired with a software signer cannot prepare or sign and
  returns `CAPABILITY_MISSING` with zero adapter/sign calls;
- an adapter whose declared asset differs from the selected account/request cannot
  prepare; and
- hardware ZEC needs the positively probed current PCZT verification capability in
  addition to the existing v6/Ironwood flags.

Keep vendor names irrelevant. Do not add a software fallback.

For this deliberately synthetic WAL-002 reference, branch and PCZT compatibility must
be exact rather than substring/truthiness based: reject NU6.3 lookalike strings and
unsupported non-empty PCZT strings while retaining the committed exact fixture values.
This does not claim a real mainnet capability table; later broker tickets replace the
synthetic pins with reviewed shipped pins.

## 3. Preserve concurrency safety through crash recovery

Using one shared fake adapter and the same account ID, prove:

- a machine that owns the prepare lock and crashes from `signed_unverified` or
  `verified` keeps the account unavailable to a second prepare while it is in
  `crash_recovery`;
- a machine constructed in synthetic restored `crash_recovery` must acquire the account
  lock at fresh confirmation, and gets `ACCOUNT_BUSY` without verification or broadcast
  when another machine owns it;
- explicit `cancel()` and `expire()` are valid from `crash_recovery`, terminal, and
  release any owned process-local fake lock; and
- another crash/restart while already in `crash_recovery` stays there without changing
  the confirmation count, signing, verifying, or broadcasting.

Also prove a synthetic durable `verified` restore cannot call broadcast before entering
`crash_recovery` and receiving a fresh confirmation. Preserve the existing explicit
`broadcasting -> unknown_needs_scan` fixture contract and zero-resubmit assertion.

## 4. Fail closed on injected dependency exceptions

Add compact cases in which the injected request-status source, adapter prepare/verify/
broadcast, or signer throws. Public state-machine methods must return a stable failure
object instead of letting the exception escape, must enter an appropriate non-spendable
state, must release any process-local lock, and must never make a later broadcast call.
Use `UNAVAILABLE` unless an already-normalized contract error is deliberately preserved.

## 5. Sanitize values as well as field names

Prove `sanitizeLog` does not invoke getters and never copies objects, arrays, functions,
control-bearing strings, or synthetic secret-canary strings placed inside an allowlisted
field. Valid synthetic `account_id`, `intent_id`, 32-hex `request_id`, state, and error
code strings must still survive. The result remains an ordinary object containing only
accepted own data properties.

## Absolute exclusions

Do not weaken, remove, rename, or reorder existing coverage to hide a failure. Do not
add production helpers, dependencies, real addresses, keys, transactions, coin code,
network, files, timers, listeners, processes, Electron, devices, rates, or quote data.
Do not modify the existing fixture or its oracle hashes.

Read-only shell commands are authorized solely to read the exact required repository
documents and authorized/current source, plus `wc -l` and `sha256sum` over the authorized
test path for final reporting. Use `apply_patch` for edits. Do not execute tests, Node,
npm, builds, installs, formatters, scanners, Git, GitHub, network, wallet, node daemon,
subprocess, hardware, USB/HID, or device commands. Do not use `/tmp`, root, `sudo`,
deletion, cleanup, `rm`, globs, environment-variable targets, or unresolved paths.

Stop after authoring. Report the changed path with line count and SHA-256, exact added
test names/count, how each blocker is asserted, expected-red failures against the current
unaccepted production drop, and confirmation that the accepted fixture, both security
tests, all production/wiring paths, and `package-lock.json` remained unchanged and
nothing ran. Reviewer Codex at XHigh must inspect the corrected test source before Luna
executes expected-red commands.
