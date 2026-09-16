# Grok — remove the payment inbox connection control

SUPERSEDED before execution: the owner also requires automatic updates without Refresh.
Only GROK_BUILD_BBD_PAY_001_AUTOMATIC_INBOX_01.md is active. This earlier scope is
historical; do not implement or execute it separately.

Actor: Grok Build 4.6 High, owner-relayed. Reviewer: Codex.
Status: active, owner-directed UI simplification.

## Recorded product decision

On 2026-09-15 the owner rejected the Connect local daemon control: other distributed
app features work without such a button, and explaining the implementation did not
make it useful. Remove it from the Requests screen and remove instructions to choose
a daemon or data folder. Normal request loading already uses the default local
connection automatically. Do not add a replacement button or an advanced setting.
This supersedes the earlier suggestion to move the selector into settings.

Read AGENTS.md, CURRENT_TASK and tickets/BBD-PAY-001.md. This is the only active
source handoff. The pending Hermes UI runtime handoff is paused until these final
UI bytes are reviewed; do not execute it against obsolete hashes.

## Baseline and writable paths

Require HEAD `7a31c41cb29692a94acf1f24adb379f3a829d237` and empty index. Verify:

| Writable path | Starting SHA-256 |
| --- | --- |
| social/payment-inbox.js | caac07903a2f97ad4da1f1584f10adef0e6954aafd1f287f897ac44573617db3 |
| social/app.js | 699d66977de0793269dcf045f0442275842c3fb9300cd7a9238035cb5fd8147a |
| test/paymentInbox.electron.js | 89d82e8ad73da7d08e3f03a1e9d953521a75a4f55803d96f1265f87bd8c8ce54 |

All other source/test/dependency files stay frozen at review 04 identities: the nineteen
paths from correction-02 report, with the final harness hash above. Preserve unrelated
dirty work and all old evidence. No transport, daemon, IPC/preload, CSS or wallet changes.
The renderer must simply stop offering/calling the manual selection operation; do not
refactor the already reviewed internal client/IPC lifecycle for this cosmetic removal.

Only new repository report: `docs/testing/BBD-PAY-001-GROK-REMOVE-CONNECT-01.md`.
New ignored command captures: `dist/pay001-remove-connect01/`.
No governance edits, Git mutation, actor launches or real process restart.

## Exact change

1. Remove creation, placement, busy-state handling and event binding of the Connect
   local daemon button in social/payment-inbox.js. Refresh is the only inbox action.
   Keep automatic loading on Requests activation and the existing refresh sequencing.
2. Remove the `connect` function from the bridge passed by social/app.js. Keep its
   `getPaymentInbox()` integration and every unrelated social feature unchanged.
3. Use these plain-language state messages on the Requests screen:

   | State | Heading | Explanation |
   | --- | --- | --- |
   | loading | Loading payment requests | Please wait. |
   | empty | No received requests | Requests sent to you will appear here. |
   | unavailable | Payment requests unavailable | Try Refresh in a moment. |
   | unsupported | Payment requests are unavailable on this system | Payment requests are currently supported on Linux. |
   | invalid | Could not load payment requests | Try Refresh again. |
   | too_large | Too many payment requests to display | The request list exceeds the display limit. |
   | identity_changed | BitBook identity changed | Restart BitBook to load requests for your current identity. |

   Default subtitle becomes "Requests sent to you". Preserve the authenticated peer
   identity context in ready replies, all request rows/details and status semantics.
   No text on this screen should tell the user to connect a daemon or choose a folder.
   Do not claim the app starts a stopped service or retry automatically; behavior stays
   automatic default selection plus activation/Refresh reads. Restart remains the
   existing way to reset the session identity pin after a local identity change.
4. Update the existing Electron smoke for this UI change. Replace old unavailable
   message expectations with the new text. Remove the picker-click/cancel scenario
   and the unused connect function on the isolated stale-result bridge. Keep the
   native picker mock as a trap/counter and assert it is never called by normal inbox
   loading, refresh, empty/disconnected states or the stale-result fixture. Assert
   the inbox actions contain exactly the Refresh button and no folder/daemon selection
   instruction. Preserve every populated/empty/disconnected, layout, keyboard details,
   sandbox, real IPC/HTTP and stale-response assertion that still applies.

## Verification and return

This is a reversible UI control/copy removal. Update the existing smoke assertions;
no new test suite, test project or fabricated historical red is required. This explicit
bounded contract supplies the verification scope for the UI-only change.

Allowed focused commands:

```text
node --check social/payment-inbox.js
node --check social/app.js
node --check test/paymentInbox.electron.js
node test/socialCore.node.js
```

Require exit 0. Do not rerun payment transport/security/preload/combined suites or the
known failing Electron launch context. The pending isolated Hermes runtime will execute
the revised existing UI smoke after source review; do not claim screenshots here.

Save the three final hashes/line counts, exact diff summary, command argv/exits/log
hashes and unchanged-input verification in the named report. Capture into fresh files,
retain failures and distinguish syntax/social checks from unexecuted Electron behavior.
Return the report pointer; Codex reads it directly. No owner evidence transcription.
