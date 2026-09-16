# BBD-PAY-001 Messages source review 01

Reviewer: Codex. Result: CHANGES REQUIRED; no Hermes acceptance authorized.
Reviewed Grok's [saved report](BBD-PAY-001-GROK-MESSAGES-01.md), source and retained
captures. No tests or Electron were executed by the reviewer.

## Evidence checked

HEAD remains `7a31c41cb29692a94acf1f24adb379f3a829d237`; index empty. All six delivered
hashes and line counts match the report. All fifteen inspected frozen boundary,
fixture, dependency and security inputs match prior identities. All ten command
captures have matching stdout/stderr hashes; nine metadata hashes appear in the report.
The first harness-error capture is retained and explicitly excluded from behavioral red.
Behavioral red, 54-test green/restored green, 32 security tests and syntax/social checks
are retained. Identity-gate falsification fails on the expected incorrectly visible row.
This is valid evidence for the assertions exercised, not proof of the omitted cases.

Messages placement, request-only rows, exact displayed values, identity equality and
basic periodic updates are implemented. The following defects block source acceptance.

## 1. P1 — stale work can cross conversations and sessions

`social/app.js:786` assigns history after an await without capturing session or peer.
The socket/read-receipt/send paths still call it directly (`:608`, `:795`, `:851`).
Example: an Alice socket event starts a slow history read; the user opens Bob; Alice's
response overwrites `state.messages`, which the next render displays under Bob.
The caller's later guard cannot undo the mutation. A delayed send also clears the
current composer's draft after the user changes conversations (`:863`).

Profile and startup conversation loaders likewise mutate after unguarded awaits
(`:139`, `:150`, `loadConversations`). Old socket message callbacks do not check socket
identity before entering the handler (`:587`); they capture the *new* session generation.
`beginSession` clears state but does not clear the rendered old conversation before
awaiting config (`:489`, `:650`), leaving old request content visible while switching.
Require guards at mutation boundaries, immediate DOM invalidation, and tests with
actually suspended old responses. The current switch test suspends none of them.

## 2. P1 — the bounded update loop has bypasses and cleanup failures

`openChat` starts an independent history request without a deadline (`:823`), while
socket/read-receipt/send paths start further unbounded reads. These bypass coalescing.
Startup waits for every ancillary request before starting chat reconciliation (`:667`),
so a hung profile/following request prevents automatic text updates entirely.

`reconcileChat` uses Promise.all (`:540`): if one request rejects early, finally clears
the timeout without aborting or awaiting the sibling request. That sibling can hang
while later passes start. A history failure also discards a successful conversation
list response. An old pass unconditionally resets `chatInflight` (`:559`), even after a
new session starts another pass. Chat switching does not abort an in-progress pass.

Disposal has no terminal flag or generation invalidation for chat and leaves the
visibility listener installed (`:927`, `:940`). An aborted pass's finally can schedule
another timer after disposal. Tests claiming cleanup end with `assert.ok(before >= 0)`
without dispatching disposal; the hung-history test never asserts abort or recovery.
Use one owned coordinator for every chat read trigger, bounded per-request cleanup,
independent successful results, and executable lifecycle assertions.

## 3. P2 — background rendering loses reading position and focus

`renderMessages` clears and rebuilds the whole transcript on a changed snapshot
(`:392–422`). It restores expanded state only, with no visible-entry/offset or focused
element restoration. Clearing can clamp scrollTop, and inserting history ahead of
the reader changes the visible entry. `loadActiveMessages` also clears transcriptKey
on unchanged reads (`:792`), causing forced bottom-follow. A focused details summary
is removed on a cancellation update. Add browser geometry/focus assertions and retain
stable keyed nodes or explicitly restore an anchored viewport and focus.

Related merge robustness: `payment-inbox.js:54` uses `{}` indexed by untrusted social
peer IDs; `__proto__`/`constructor` resolve inherited objects. Use Map or a null-prototype
dictionary. Restore the existing `You:` prefix for outgoing conversation previews,
lost at `app.js:363`. These fit the same bounded merge/render correction.

## 4. P1 for acceptance — Electron driver cannot establish the claimed journey

Source inspection identifies independent fixture/assertion errors:

- Social HTTP fixture (`test/paymentInbox.electron.js:165`) supplies no CORS response
  for the file-origin renderer with webSecurity enabled. Empty profile (`:175`) also
  opens the real profile modal, preventing the later outside-modal keyboard interaction.
- At `:458`, body.innerText is used to find the second peer while conversationList is
  hidden by the open chat. Inspect the hidden row's data/text for arrival, then navigate
  back and prove it is visible and clickable.
- At `:466`, cancel-coffee is supplied alongside only tea. Their request IDs differ;
  tea expires and no displayed request becomes cancelled. Assert cancellation on a
  matching request and expiry separately, with exact request IDs/statuses.
- At `:475`, a renderer-style `state` field is inserted into the closed daemon envelope.
  Main rejects it as invalid, so the expected renderer identity-mismatch notice cannot
  occur. Keep the payment envelope valid and exercise a mismatching social config via
  the real connection flow; test malformed/unavailable transport separately.
- Layout checks dropped positive geometry and child/overlap checks; the narrow capture
  occurs after the check at a different size. Recovery, anchored scroll/focus and actual
  empty readiness are not asserted. Restore non-vacuous checks at each measured size.
- The request blocker accepts URL prefixes and also permits the main-only payment
  fixture in renderer traffic; it should allow only the exact social fixture origin.
  Unexpected traffic must fail the test, with expected bootstrap/socket attempts
  explicitly enumerated. Do not treat `blocked.length > 0` as adequate isolation proof.

## Next authorized work

[Grok Messages correction 01](../handoff/GROK_BUILD_BBD_PAY_001_MESSAGES_CORRECTION_01.md)
is the sole source authorization. Retain the current evidence. No UI acceptance claim,
manual owner retest, runtime launch or publication is authorized by this review.
