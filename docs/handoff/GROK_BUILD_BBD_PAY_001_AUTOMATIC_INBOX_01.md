# Grok — automatic Payment requests inbox

Actor: Grok Build 4.6 High, owner-relayed. Reviewer: Codex.
Status: CLOSED — report delivered; standalone placement superseded by
[requests in Messages](../architecture/BBD-PAY-MESSAGES-UX-01.md).
Do not execute further edits or commands from this handoff. Retain the saved report
and captures. A new bounded contract is required for Messages integration.
The original instructions below are historical.

## Recorded owner decision and implementation boundary

On 2026-09-15 the owner rejected Connect local daemon and asked why Refresh was
necessary when requests arrive through P2P. The Requests screen must connect and
update automatically. Remove both controls and all instructions to manage a daemon
or select a data folder. Do not replace them with settings or another manual action.
This supersedes remove-connect 01, prepared before the owner's refresh steering.
Hermes UI runtime capture remains paused until the new source is reviewed.

The daemon already receives and stores P2P requests. Its accepted authenticated local
interface provides snapshots only; it has no payment-update subscription. For this
bounded UI change, reread that existing interface while Requests is visible, with a
five-second delay after each completed read. This is UI synchronization, not a new
P2P event API or push subscription. Updates follow that cadence plus request latency.

## Baseline and exact writable paths

Require HEAD `7a31c41cb29692a94acf1f24adb379f3a829d237`, empty index and:

| Writable path | Starting SHA-256 |
| --- | --- |
| social/payment-inbox.js | caac07903a2f97ad4da1f1584f10adef0e6954aafd1f287f897ac44573617db3 |
| social/app.js | 699d66977de0793269dcf045f0442275842c3fb9300cd7a9238035cb5fd8147a |
| test/paymentInbox.electron.js | 89d82e8ad73da7d08e3f03a1e9d953521a75a4f55803d96f1265f87bd8c8ce54 |
| test/paymentInbox.node.js | 9c166540f59498134e544fa3ead3e71dd340316e0c9bdaf44aeb3179c85c2b2b |

Tests first, then production. All other source/test/dependency/daemon files stay frozen
at review-04 identities. No new dependencies, transport, IPC/preload, CSS, public API,
wallet or persistence changes. Preserve unrelated dirty work and prior evidence.
New report only: `docs/testing/BBD-PAY-001-GROK-AUTOMATIC-INBOX-01.md`.
Ignored command captures only: `dist/pay001-automatic-inbox01/`.
No Git mutation, governance edits, actor launches or real app/daemon restart.

## Fixed UI and scheduling semantics

1. Remove Connect and Refresh button creation, action container, busy handling and
   listeners from Requests. Remove `connect` from its bridge in app.js. No manual
   refresh elsewhere. Keep getPaymentInbox, authenticated default connection and
   identity binding unchanged. Do not refactor the frozen internal client/IPC API.
2. Component exposes `activate`, `deactivate`, `dispose`. First activation reads
   immediately. Own one timeout and at most one outstanding bridge read. Schedule
   the next read 5000 ms after settlement only while active and document-visible.
   Repeated activation must not duplicate timers or reads. No unbounded interval or
   immediate retry loop on failures.
3. `showView` activates only for Requests and deactivates when navigating away. A
   visibilitychange listener pauses scheduling when hidden/minimized, then reads
   immediately on visibility return if Requests is still selected. Reactivation while
   a read is pending records one fresh-read need, never another simultaneous call;
   begin that read after the previous one settles. Dispose clears timers, removes
   listeners and ignores late results; app unload invokes disposal.
4. Sequence/generation checks stop reads begun before deactivation/disposal from
   altering the current DOM. Existing main-process reads retain their five-second
   deadline; do not invent cancellation IPC. Hidden views never accumulate work.
5. Show loading only when no usable snapshot exists. Background reads must not blank
   rows, flash a loading state, reset scrolling or close details every five seconds.
   If the DTO has not changed, leave DOM intact. On changes preserve expanded details
   by request_id and viewport position where practical. New requests and status/expiry
   updates appear automatically. A failed read clears stale rows and shows unavailable;
   retry on the same bounded cadence and recover automatically on success.
6. Use these plain-language messages:

   | State | Heading | Explanation |
   | --- | --- | --- |
   | loading | Loading payment requests | Please wait. |
   | empty | No received requests | Requests sent to you will appear here. |
   | unavailable | Payment requests unavailable | BitBook will try again automatically. |
   | unsupported | Payment requests are unavailable on this system | Payment requests are currently supported on Linux. |
   | invalid | Could not load payment requests | BitBook will try again automatically. |
   | too_large | Too many payment requests to display | The request list exceeds the display limit. |
   | identity_changed | BitBook identity changed | Restart BitBook to load requests for your current identity. |

   Default subtitle: "Requests sent to you". Preserve authenticated peer context,
   precise amounts, rows/details and status semantics. Identity changes remain
   fail-closed until a normal full app restart resets the session pin; no dedicated
   reconnect action. No automatic service startup is claimed.

## Focused tests and execution

Add renderer lifecycle cases to existing test/paymentInbox.node.js. Use the real
component in a minimal deterministic DOM/event fixture with controlled timers and
deferred bridge promises, without a dependency. Prove initial read, automatic row/status
updates, absent buttons/picker calls, one outstanding read/one timer, slow-response
coalescing, hide/deactivate/dispose cleanup, reentry/resume, stale rejection, unchanged
DOM/open-details preservation, failure/recovery. Assert DOM outcomes and call counts.
Use a fake clock for five-second boundaries; actual Electron validates browser behavior.

Update the existing Electron smoke: remove manual-button/picker scenarios and Refresh
expectations. Change owned server fixtures while Requests stays open, waiting for
automatic transitions: empty -> populated -> changed status/expiry -> empty ->
disconnected. Use the existing injected main clock, no public P2P or real wallet.
Keep the picker mock as a zero-call trap. Preserve sandbox, actual preload/IPC/HTTP,
social isolation, layout, keyboard details and stale-response checks that apply.
Adapt selectors for the absent action container. Verify background updates preserve
open details. Retain the four real screenshot requirements.

Authorized focused commands:

```text
node test/paymentInbox.node.js
node --check social/payment-inbox.js
node --check social/app.js
node --check test/paymentInbox.electron.js
node test/socialCore.node.js
```

Capture new lifecycle tests failing against current UI before production edits, then
require final focused commands exit 0. No broad suite or duplicate combined run: the
existing wallet runner includes this target. No unchanged transport falsification
rerun or known-failing Electron launch attempts. Hermes actual UI capture resumes
after source review; report that execution pending, never green.

## Completion record

Save final four-file hashes/line counts, unchanged-input verification, behavior summary,
red/green argv/exits/test counts and immutable raw log/metadata links/hashes in the named
report. Retain failures and restoration facts. Distinguish deterministic renderer tests
from unexecuted real Electron smoke. Stop for Codex review and return the report pointer;
no evidence transcription by the owner.
