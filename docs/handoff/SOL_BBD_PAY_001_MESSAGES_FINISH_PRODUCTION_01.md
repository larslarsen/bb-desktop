# Sol — Messages finish production 01

CLOSED: source reviewed in [review 05](../testing/BBD-PAY-001-MESSAGES-REVIEW-05.md).
Next authority: [Hermes verification 01](HERMES_BBD_PAY_001_MESSAGES_FINISH_VERIFY_01.md).
The source authorization below is historical. Actor: Codex Sol (`gpt-5.6-sol`, High), owner-relayed.
Reviewer: Codex. Grok remains unavailable due weekly usage exhaustion; the documented
fallback remains selected. Read AGENTS.md, TESTING.md, CURRENT_TASK and
[red review 04](../testing/BBD-PAY-001-MESSAGES-REVIEW-04.md).

Repository: bb-desktop. HEAD `7a31c41cb29692a94acf1f24adb379f3a829d237`, empty index;
preserve unrelated dirty work. The accepted red is 65 pass / 5 fail, all 70 completed.

## Exactly two writable files

| Path | Starting SHA-256 | Scope |
| --- | --- | --- |
| social/app.js | daa737ff2be1b89b6bbf6c74c51c2ffaaeaed2339e28796302b6d787ace08e5e | Fix the behavior below |
| test/paymentInbox.node.js | 4a14a37fa89ccc5a0f046e9f47518a4a320a836dec3a27ae60cf1eb30bff6931 | Shared fixture teardown only |

Freeze payment-inbox.js at
`437541ac7d161ad78380fded0adac1c2486f64116f166e908b7cacdb57aa6e64`
and the Electron driver at
`d884456fcef8955ca5835da2ae13e6864e334bf4ef6305742385386e478e58bf`.
All other production/test/fixture/dependency/security files retain review-03 identities.
No protocol, daemon, wallet, IPC, HTML/CSS or dependency change.

No test/syntax/format execution, installs, Git, actor launches or repository-record edits.
Sol must not inherit Grok's test-execution privileges. Verify source identities read-only.

## Fixed production requirements

1. **Message identity and read labels.** Key text bubbles using the actual `messageId`
   field, scoped to local session/peer. Include identity in transcript-change detection
   so equal body/timestamp with distinct IDs remain distinct. If an older fixture or
   response lacks an ID, preserve each occurrence with deterministic fallback keys;
   never merge legitimate identical messages. Read-state changes update the reused
   bubble's status text in place. Keep literal text rendering, original ordering and
   unchanged-node stability. Do not put `read` in the identity key to bypass node reuse.

2. **One owned read receipt.** Track the current receipt's session, API URL, conversation
   generation, peer, AbortController and the displayed incoming unread message IDs
   captured when posting. At most one receipt in flight for that active conversation;
   history polls/events coalesce while it is pending. On success, remember the captured
   acknowledged IDs, so repeated stale unread snapshots cannot immediately POST again.
   Newly displayed unread IDs still need acknowledgement, including arrivals during
   the pending receipt. Retain the existing bodyless markchatasread endpoint; no new
   API arguments or message protocol. Use stable occurrence fallback for missing IDs.

3. **Retry and cleanup.** A failed receipt retries on the normal five-second cadence,
   not an immediate event/POST/GET loop. Receipt completion can request one reconciliation
   only while its captured context is current. Abort owned receipt/retry work on peer
   close/switch, session change or disposal; stale finally/completion cannot clear a
   newer owner or enqueue work. Hidden/closed/disposed views never start receipts.
   Clear the typing reset timer on disposal and peer/session invalidation, and guard
   its callback against stale context. Preserve independent bounded payment/chat readers.
   Avoid wall-clock-only retry logic that bypasses the existing timer-driven lifecycle.

4. **Reading position and focus.** Implement the reviewed fix at capture/restoreViewport
   and transcript reconciliation. Preserve focused details summary without browser
   focus scrolling away from the reader's saved entry/offset (`focus({preventScroll:true})`
   or equivalent ordering with final anchor restoration). Preserve expanded details,
   composer draft and unchanged nodes. Readers scrolled up stay anchored through status
   changes/new text; readers within 24 pixels of bottom follow new entries. No blanket
   scroll-to-bottom or replacing browser assertions with synthetic geometry.

## Narrow test integration repair

`test/walletPay.node.js` calls the exported `paymentInbox.tests[].fn` directly, bypassing
the standalone runner's finally teardown. In paymentInbox.node.js, make per-case fixture
cleanup apply to both exported case invocation and run(). Prefer wrapping registered
case functions in an awaited try/finally teardown; keep cleanup idempotent and errors
visible. Preserve selected/synthetic runner cases and subprocess proof of natural return.
Do not edit walletPay.node.js, remove assertions, rename/skip cases, change fixtures,
introduce process.exit success, or manufacture a passing result. Exactly 70 cases stay
registered, with the five known red assertions intact. This is shared harness lifecycle
plumbing only; it is not permission to repair production by weakening tests.

## Completion and next verification

Stop after source edits. Point to the two edited files; no Sol completion/evidence report
is required or authorized under its role. The reviewer reads the source directly.
If a frozen path needs a change, stop and identify the concrete scope issue.

After source review, Hermes will execute (not authorized by this handoff):

```text
node --check social/app.js
node --check test/paymentInbox.node.js
timeout --signal=TERM --kill-after=5s 120s node test/paymentInbox.node.js
timeout --signal=TERM --kill-after=5s 120s npm run test:wallet-pay
node test/socialCore.node.js
node test/electronSecurity.node.js
```

It must retain exact source manifests, command metadata and raw logs. The later bounded
falsification will suppress only the new read-label update, require its regression to
fail, restore exact source bytes and rerun green. A separate reviewer-pinned Hermes
handoff will combine the remaining acceptance/security checks and corrected Electron
driver with the previously documented isolated sandbox runtime route. No old runtime
handoff is active, no sandbox disabling, and no real app/daemon restart.

Reviewer governance paths for this authorization: this handoff, Messages review 04,
CURRENT_TASK, Hermes finish-red handoff, tickets/BBD-PAY-001.md and the status map.
