# Grok — Messages correction 01

PAUSED — owner reports weekly usage exhausted. Source edits and captures exist;
completion report missing. Execute only
[Hermes record recovery](HERMES_BBD_PAY_001_CORRECTION_RECORD_RECOVERY_01.md).
The correction specification below is retained; no further Grok execution authorized.
Grok Build 4.6 High, owner-relayed. Reviewer: Codex.
Read AGENTS.md, TESTING.md, CURRENT_TASK and
[Messages review 01](../testing/BBD-PAY-001-MESSAGES-REVIEW-01.md).
The original [Messages contract](GROK_BUILD_BBD_PAY_001_MESSAGES_01.md) remains the
behavior specification; this handoff replaces its execution scope. Fix all four review
groups in one source pass. No product redesign or extra owner testing is needed.

## Baseline and scope

HEAD `7a31c41cb29692a94acf1f24adb379f3a829d237`, empty index, dirty work preserved.
Only these four source/test files may change, tests first:

| Path | Starting SHA-256 |
| --- | --- |
| test/paymentInbox.node.js | fd6f1eab04e7121bc6ed68f5ca8785465ee9f810265ed3efb09455f8027ad1ae |
| test/paymentInbox.electron.js | c5350be482f84bd276289e017accf3aaa9a3927f209deee0d67c8b3885d3502f |
| social/app.js | b2ff043b5645f9c19bcf79ed965dfa7d758f9fe51b2a9a65dcb65c01cd32b17e |
| social/payment-inbox.js | 3aa647a76f66c90c7b2929aa67f041b565675f4ea57eb4d7dd4978d1ac841884 |

Freeze HTML `6fe484ea8692fe661f13b100496fd9472aab7f18bfded0f7735f2b52b0162f24`
and CSS `f2572a739fa836cef9b1a2b7f32fd5aa3aced8e8d41bc72b107e22409c1c3898`.
All other source, dependencies, fixture JSON and security inputs remain frozen at the
Messages-01 baseline. Preserve the 38 transport cases and the useful Messages cases.
Do not relax tests to fit implementation or edit previously retained reports/captures.

Sole report: `docs/testing/BBD-PAY-001-GROK-MESSAGES-CORRECTION-01.md`.
New ignored captures: `dist/pay001-messages-correction01/`, numbered immutable runs.
No Git, actor launches, broad acceptance, Electron launch, host/sandbox modification,
real daemon/wallet data or other governance edits.

## Fixed correction approach

1. Route startup, open/close conversation, socket message/read, successful send,
   read-receipt completion and periodic/visibility triggers through one owned chat
   reconciliation coordinator. Remove parallel unbounded history reload paths.
   Start reconciliation after matching/current config is obtained, before awaiting
   ancillary profile/feed work. Opening renders cached request cards immediately.

2. Capture immutable session generation, API URL and a conversation generation in each
   operation. Check before each state/DOM mutation and after every await that can lead
   to another request. Reject old socket events at the socket callback itself. Guard
   profile/startup loader mutations too. Session invalidation clears rendered content
   synchronously. A send completion from an obsolete conversation must not erase a
   newer draft, trigger current-peer reads or display a misleading current-chat result.
   Preserve the sent draft semantics in the still-current chat (do not erase text typed
   while a send was pending). Do not change the message protocol.

3. Retain at most one owned reconciliation pass, at most two requests in that pass,
   one scheduled next pass and one pending-trigger flag. Every read has a five-second
   deadline covering headers and body. An early failure must not cancel the sibling's
   deadline or orphan it. Apply successful conversation/history results independently;
   preserve same-session cached data on transient failure. A failure in history cannot
   prevent other-peer conversation discovery. Next polling is five seconds after all
   owned work settles. Abort obsolete reads on peer/session changes; a stale finally
   can clear only its own ownership, never a newer pass's inflight state.

4. Dispose terminally: invalidate generations, abort pending chat work, stop timers,
   detach owned visibility/socket listeners and prevent finally from restarting work.
   Keep the payment reader independent and maintain its one-call/one-timer bounds.
   Render incoming active-visible text before invoking read handling. Coalesce read
   posts and their reloads; no automatic receipts for closed/hidden conversations or
   requests alone. Assert no calls/state/DOM updates after disposal and late completions.

5. Preserve unchanged transcript nodes. On changes, retain keyed request/text nodes
   where practical; otherwise restore the same visible entry and pixel offset, expanded
   details and focused summary. Include local identity and counterparty in request keys.
   Respect the existing 24-pixel bottom threshold and initial-open behavior. Remove
   unconditional transcript-key resets on ordinary history reads. Use a safe peer map,
   preserve `You:` on outgoing text previews and literal rendering of hostile text.

6. Repair the Electron fixture defects enumerated in review. Serve narrowly scoped CORS
   for the owned social fixture, a nonempty fixture profile, exact-origin renderer
   allowlisting and method/path assertions. The main-only payment fixture must not be
   renderer-accessible. Enumerate expected bootstrap/socket rejection separately from
   unexpected traffic; unexpected requests fail the driver. Await reload completion
   using navigation events registered before reload, not a transient isLoading poll.
   Use valid daemon envelopes, matching cancellation/request IDs, separate expiry and
   a real social-connection mismatch/recovery flow. Identify hidden rows by data/text,
   then use navigation to demonstrate visibility. Preserve actual keyboard interaction,
   all sandbox checks, exact displayed fields and both viewport captures. At each size,
   assert positive geometry, no clipping/overlap, expanded details and reading/focus
   stability. Prove empty readiness and failure→recovery, not just screenshots.

## Required regression evidence

Extend the actual app.js DOM harness to defer and resolve config, profile, conversations,
history and send responses individually, including body reads and failures. Record
request entry/completion/abort and maximum concurrency. Expose real window lifecycle
dispatch and listener/timer accounting. Do not replace document.dispatchEvent or end
cleanup tests with tautologies. Required behavioral red cases include:

- old Alice socket/history/read/send completion after Bob is open; A→B→A; old session
  config/profile/conversation completion and old socket event; no old content while
  new config is pending; newer draft survives old send completion;
- one fast-failing request paired with a hanging sibling, hung startup ancillary fetch,
  repeated event/poll triggers, peer switch during a pass, and delayed old finally;
- successful conversation list despite failed active history, current history despite
  failed conversations, timeout/recovery without a click, visible→hidden receipt rules;
- disposal during both readers' inflight work followed by late success/failure and
  visibility events: zero further requests, owned timers/listeners or DOM mutations;
- unchanged text read keeps nodes; changed status/history preserves details and focus;
  safe `__proto__`/`constructor` peer handling, outgoing text preview and exact peer rows.

Browser geometry/scroll behavior must additionally be asserted by the corrected Electron
driver for later execution; the minimal DOM is not evidence of real browser layout.
Tests must assert exact expected peer, message, draft and state. Remove permissive OR
assertions that can pass merely because a request card exists instead of required text.

Exact commands, cwd `bb-desktop`:

```text
node test/paymentInbox.node.js
node test/socialCore.node.js
node test/electronSecurity.node.js
node --check social/app.js
node --check social/payment-inbox.js
node --check test/paymentInbox.electron.js
```

Run the first command with regression tests before production changes and retain
behavioral failures (not harness errors). Then fix source and run the full list green;
repeat affected commands for in-scope repairs. Falsify one new stale-history regression
by temporarily bypassing its apply-time session/peer guard, require the exact test to
fail, restore the exact source hash and rerun green. No need to repeat the unchanged
transport instance or identity-equality falsifications.

Write the named report with final path hashes/line counts, requirement→test mapping,
exact argv/cwd/exits/counts, red/falsification/restoration and raw-log links/hashes.
State Electron remains unexecuted. If a required boundary needs an out-of-scope edit,
document the concrete blocker instead of weakening it. Stop for reviewer source review;
Hermes Messages acceptance and runtime staging remain pending and unauthorized.
