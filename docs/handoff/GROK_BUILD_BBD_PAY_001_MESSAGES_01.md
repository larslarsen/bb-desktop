# Grok — payment requests in Messages

CLOSED — drop reviewed; changes required. Execute only
[Messages correction 01](GROK_BUILD_BBD_PAY_001_MESSAGES_CORRECTION_01.md).
The behavior specification below remains; its execution scope is superseded.
Actor: Grok Build 4.6 High, owner-relayed. Reviewer: Codex.
Read AGENTS.md, TESTING.md, CURRENT_TASK, tickets/BBD-PAY-001.md and
[the Messages decision](../architecture/BBD-PAY-MESSAGES-UX-01.md).
This is the sole active source authorization. Earlier inbox handoffs are closed.

## Outcome

A received request appears in the requesting peer's Messages conversation alongside
text messages. A peer with no text history still appears in the conversation list.
Text messages and requests arrive and update automatically while the app is visible,
without reopening a conversation. Owner explicitly confirmed automatic text updates.
Remove the
Requests tab and separate screen. No payment Connect, Refresh, settings, Pay or Send
controls. The existing text-message composer and its Send button retain their purpose.

This is received-request display only. Preserve the accepted main/preload/daemon
boundary, exact amounts, cancellation/expiry semantics and prior transport tests.
No source or test changes to the daemon, wallet, IPC or dependencies are authorized.

## Baseline and writable paths

Desktop HEAD: `7a31c41cb29692a94acf1f24adb379f3a829d237`, empty index, existing dirty
work preserved. Reviewer checked these starting identities on 2026-09-15.

| Writable path | Starting SHA-256 |
| --- | --- |
| test/paymentInbox.node.js | 4b6a858ccf3c4ae3bfcf637e11cad1cc2bd1b21c20597b4ce040f732d13ba7eb |
| test/paymentInbox.electron.js | 113904ede628e660246a294ccaaa38972f3f77cada92ffd306f9530d649b801b |
| social/payment-inbox.js | 2de139dbd8bbbc3a8a6a7d03ad1f3e5003ede6b374f6d432582025a4994071b0 |
| social/app.js | d26bf465457a47b9a72e6e0bc0a68d0fb7f8b2ff3108919f15899774ef327214 |
| social/index.html | a9f3a2e8a08f420b9ae04b9b339211d48511c8a4607bdce3ac1882e4f9691774 |
| social/styles.css | 4673ece3b62454458c0cc2f538a120073b8d9e0d73f83834c1d63716c5a31b31 |

Author tests before production. Rework the existing payment renderer module into a
snapshot controller and card rendering helpers; keep its path to preserve maintained
security coverage. Integrate it through app.js's existing Messages functions. No new
production module, dependency or generic UI framework. Scope CSS to Messages/cards and
remove obsolete payment-screen rules; preserve Feed, posts, Network and wallet layout.

Frozen boundaries verified by the reviewer:

| Path | SHA-256 |
| --- | --- |
| wallet-pay/inbox-client.js | a41463f491fdf1aa317d972f69438b62c273ccb59ad8e2a99444fabe563de8e5 |
| social-main.js | 298362de5c8c6f846ac63beb582574cea6194fa819efa9e4e10f99508431e9f6 |
| wallet-preload.js | 6c22b08bed294c5eac750f96d12f7227548574eee178765954195585cae6f025 |
| social/core.js | 4d27e4701ff299ef9753c02174bd8868aa01c94509b84d5a8187978fa239fb8f |
| test/socialCore.node.js | ee54f31e2845c70f34cb2d6969f8955bd0d309f5fbfd0d03aa33e6d201663521 |

Other inputs stay at correction-02/review-04 identities; preserve fixture JSON and all
38 existing transport tests. Replace only superseded standalone UI assertions, mapping
each retained scheduling invariant to its Messages replacement in the report.

Sole writable report: `docs/testing/BBD-PAY-001-GROK-MESSAGES-01.md`.
New ignored capture directory: `dist/pay001-messages01/` (inspect filesystem first).
Retain separate numbered command captures; never overwrite old evidence. No other
governance edits, Git mutation, actor launches or real user-process restart.

## Fixed behavior and identity semantics

1. **Session binding.** The payment DTO's `peer_id` comes from the authenticated
   main-process bridge. Show its requests only after `/ob/config` for the current
   social connection returns the same nonempty `peerID` (exact string equality).
   Missing/mismatched identity or non-ready payment state hides payment cards and
   synthetic conversation rows immediately. Never substitute a profile ID/name.
   Config equality is a display consistency guard, not authentication of the social
   HTTP server. Profiles/text remain untrusted social content; this renderer check
   grants no payment authority. The authenticated request card retains its exact payee
   ID in details. No request data, IDs or memo is posted to social HTTP/WebSocket,
   localStorage, logs or new analytics; do not fetch profiles merely because a request
   arrived. Use cached names where available, otherwise the existing shortened peer ID.

2. **Connection lifetime.** At the start of connect/reconnect or API selection change,
   invalidate the old session and clear payment projection, old conversations/messages,
   active peer and old profile cache before awaiting network work. Capture the connection
   generation and URL for asynchronous config/chat/profile/conversation reads and socket
   callbacks; stale work cannot restore old state. A successful current config binds
   payments even if text-history or another ancillary social fetch fails. A later config
   failure invalidates binding. Preserve a known matching config through an ordinary
   socket disconnect. Keep payment retry independent of text-history success. Do not
   leave a prior peer's messages visible while switching conversations; guard late
   history/read-receipt/send completions with the captured session and active chat.
   Do not change social endpoint schemas or message signing/delivery behavior.

3. **Automatic reads.** Own one payment controller for the app session, independent of
   Feed/posts/Network selection and active chat. Read at startup and when the document
   becomes visible, then 5000 ms after each settlement. At most one outstanding bridge
   call and one timeout. Hidden windows pause scheduling; visibility return queues at
   most one fresh read if a call is still pending. Session invalidation clears projected
   rows immediately and rejects old results. Dispose removes listeners/timers and ignores
   all late completions. No manual refresh or new P2P subscription. Status remains the
   authenticated DTO's requested/cancelled/expired value, recomputed by main on reread.

4. **Conversation list.** Merge text conversations and requests by exact `payee_peer_id`.
   One row per peer, including request-only peers. Latest activity is the later of text
   timestamp and request `created_at`; sort descending, peer ID ascending for ties.
   A request preview reads `Payment request · <amount_display> <asset> · <status>` when
   it is the latest activity. Keep existing text unread counts; do not fabricate read
   receipts or persist request unread state. Status changes update the preview/card
   without changing its original activity time. Preserve existing text rows when
   payment retrieval fails; remove only the payment projection.

5. **Conversation contents.** Merge text and request entries chronologically ascending
   (request time is `created_at`). For equal times, text entries precede request entries;
   retain text input order and sort requests by request_id then digest for stable ties.
   Key request cards by local identity, counterparty and request_id, retaining digest
   as content, so a status change updates one card. Never insert cancellation as another
   chat message. The card says Payment request and displays exact amount/currency,
   network, memo and status, with expandable existing ID/payee/time/digest details.
   Render all values using text nodes. No paid claim or renderer-originated spend.

6. **Failures and reading.** Opening a request-only conversation renders its cards
   immediately and keeps it open when text history fails. Show a compact inline
   `Messages could not be loaded` state without erasing its request cards. Payment
   failure is a separate compact Messages status: `Payment requests unavailable`;
   missing/mismatched session identity: `Payment requests unavailable for this identity`.
   Automatic recovery clears the notice. No empty payment panel when there are no
   requests; use the normal Messages empty state. Keep existing text Send/typing rules,
   and disable text sending when the social connection is unavailable.

7. **Stable display.** Unchanged snapshots do not rebuild the transcript. Changed
   snapshots preserve expanded details, focus, composer text and the reader's visible
   entry/offset if scrolled up. Follow new entries only when already at the bottom
   (within 24 CSS pixels); opening a conversation initially starts at its bottom.
   Handle existing renderAll/socket paths without collapsing cards or forcing scroll.
   A request arriving from another peer updates its conversation row without stealing
   focus or opening the conversation. No repeated toast on each poll.

8. **Automatic text updates.** Preserve immediate existing message/read/typing events,
   and reconcile `/ob/chatconversations` plus the active peer's `/ob/chatmessages/<peer>`
   automatically at startup after config, on opening a chat, on visibility return and
   5000 ms after each completed reconciliation. This also runs when the socket is down
   or claims to be live but misses an event. Conversation-list updates include messages
   from other peers; never require reopening the active conversation. No Feed refresh,
   new event protocol or payment payload sent through chat. Do not fetch all peers'
   histories. Preserve signed message order, outgoing/read state and existing typing.

   Own one chat reconciliation pass/timer, separate from the payment reader so failure
   in either cannot stall the other. A pass fetches at most conversations and the active
   history in parallel; coalesce event-driven reloads with it, queue at most one follow-up
   for newly observed work. Bound these fixture-testable HTTP reads to five seconds
   using AbortController and cleanup; never allow a hung request to stop future updates.
   Switching session/peer invalidates and aborts obsolete chat reads. Hidden windows
   stop periodic scheduling; disposal clears timers/listeners and aborts owned work.
   Apply results only to their captured session/peer. Do not clear known same-session
   text history on a transient failure; show the inline load notice and recover on the
   next pass. Repeated snapshots must not duplicate text, reset details, discard drafts,
   scroll a reader up or down, or emit repeated notifications. Only newly unread text
   actually displayed in the active visible conversation may trigger existing automatic
   read handling; polling a closed/hidden conversation or a payment change must not.
   Do not add separate read-receipt loops; coalesce any resulting reload into this pass.

## Test-first work and focused commands

Use the real renderer module AND app.js in the deterministic DOM/fetch/socket/fake-clock
harness. Source-string checks alone cannot prove integration. Extend the existing
Node harness within its authorized path; do not add a runtime test flag to production.

Required cases:

- arrival while Feed is selected, request-only peer row, opening it, mixed text/card
  ordering, two peers, deterministic ties and no duplication after repeated snapshots;
- cancellation/expiry update the same request, precise large amount and hostile memo
  stay literal, collapsed/expanded details and text composer remain usable;
- no prior chat, empty history and failed history each preserve discoverability;
- missing config, mismatched config, current matching config, switching API while old
  config/payment/history/profile work is pending, old socket events, fast A→B chat switch;
- social ancillary-fetch failure with matching config, payment unavailable→recovered,
  no loss of ordinary text rows or payment data sent through social network calls;
- initial poll, five-second settlement delay, one call/timer, unchanged DOM, hidden
  pause/return, pending read plus session switch/disposal; no surviving owned resources;
- no Requests tab/view or payment setup/Refresh controls, preserved text Send, no
  payment-induced focus jump, request-only peer arrival while another chat is open.
- text arrival/read update via live events, missed event while the socket remains live,
  socket loss with HTTP available, new other-peer conversation, and current history
  recovery without a click; use the fake clock to prove fallback scheduling and bounds;
- simultaneous event/poll, unchanged text snapshots, draft/scroll/details preservation,
  hung history timeout/abort, hidden-window read-receipt suppression and timer cleanup.

Exact focused commands, cwd `bb-desktop`:

```text
node test/paymentInbox.node.js
node test/socialCore.node.js
node test/electronSecurity.node.js
node --check social/payment-inbox.js
node --check social/app.js
node --check test/paymentInbox.electron.js
```

Run the first command after new tests and before production edits. Require failures
that demonstrate missing Messages behavior while the original 38 transport cases pass;
missing harness APIs/syntax alone are insufficient red evidence. Run the full listed
set after implementation; repair only in-scope failures and repeat affected commands.
For falsification, temporarily bypass the local identity equality gate only: run
`node test/paymentInbox.node.js` and require the mismatched-identity case to fail with
an incorrectly visible request. Restore the exact pre-mutation source hash and rerun
green. Preserve failure output and restoration hashes. Do not deliver mutated source.

## Actual Electron driver and later acceptance

Author the updated `test/paymentInbox.electron.js` now; Hermes executes it after source
review. Replace obsolete standalone-screen assertions with the actual Messages journey.
Use disposable home/userData, the existing authenticated payment fixture HTTP server,
mock wallet supervisor and a second owned loopback social fixture server. Bootstrap
that fixture API URL in the disposable renderer's localStorage before normal app
startup (an isolated same-origin bootstrap/reload is acceptable). Never use a real
daemon or replace production fetch/render/bridge functions to manufacture the UI.
Allowlist only that fixture server in the renderer request blocker, deny all other
HTTP/WS traffic, and serve config/profile/empty social lists/chat history locally.
Prove social reads actually hit the fixture and unexpected requests fail the driver.

Drive empty Messages → incoming request-only peer → open conversation → mixed text
and request cards → automatic cancellation/expiry → failure/recovery. Change the owned
social fixture's history and conversation list without sending a socket event; assert
new text and a second peer appear automatically, and unchanged rereads do not duplicate
either. Exercise a
second peer and mismatched local identity. Use real clicks/keyboard for details and
navigation. Identify collapsed cards by stable data attributes or textContent, not
hidden innerText; assert actual rendered text/geometry after expansion. Preserve the
existing non-vacuous layout checks, narrow 980×720 and wide 1440×1000 captures, long
memo/identifier wrapping, no horizontal clipping, expanded details and reading position.
Retain sandbox/contextIsolation/Node-disabled assertions and OS renderer sandbox proof.
These are fixture screenshots, not evidence of a live peer payment.

Do not run the known-failing Electron installation or change sandbox permissions.
The old Hermes UI runtime handoff stays closed. After reviewing final source identities,
Codex will issue one Messages acceptance handoff using the previously documented
isolated Electron-copy/helper route with the revised driver. It will execute:

```text
npm run test:wallet-pay
node test/walletPreload.node.js
node test/electronSecurity.node.js
npm run test:social
npm run build
node --check social/payment-inbox.js
node scripts/security-policy.js
node test/securityPolicy.node.js
npm audit --json
gitleaks version
```

Its exact runtime/capture path and Gitleaks manifest scan command must be frozen there
before execution. Preserve BBD-PAY-001's final-acceptance scan thresholds: no new
security/dependency findings, zero secrets with pinned Gitleaks 8.30.1; inherited four
policy failures remain release blockers. No scanner waivers or unrelated fixes.
No Rust/Go rebuild is needed for this JS/HTML/CSS slice; normal Electron loads checkout
resources directly. No acceptance, publication or changed running UI is claimed now.

## Completion record

Write the named Grok report with changed paths, hashes/line counts, exact command argv,
cwd, exits, test counts, red/falsification/restoration and immutable raw-log links/hashes.
Include a short mapping from requirements to actual test cases. State plainly that
Electron execution is pending. Record scope blockers and missing evidence without
inventing results. The owner relays the report pointer; never ask for logs in chat.
