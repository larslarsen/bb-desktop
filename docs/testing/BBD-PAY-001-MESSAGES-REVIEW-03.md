# Messages finish test-source review 03

Reviewer: Codex. Decision: authorize the bounded Node behavioral-red run.
No tests, syntax checks or Electron were executed by the reviewer.

Sol changed the two authorized test files. HEAD remains
`7a31c41cb29692a94acf1f24adb379f3a829d237`, index empty. Production app.js and
payment-inbox.js retain review-02 hashes. HTML/CSS, main/preload/transport, canonical
fixture, combined/security/social test inputs inspected retain their frozen identities.

| Test source | Lines | SHA-256 |
| --- | ---: | --- |
| test/paymentInbox.node.js | 2703 | 4a14a37fa89ccc5a0f046e9f47518a4a320a836dec3a27ae60cf1eb30bff6931 |
| test/paymentInbox.electron.js | 903 | d884456fcef8955ca5835da2ae13e6864e334bf4ef6305742385386e478e58bf |

The Node source registers 70 cases (count from source, not an execution result).
New cases exercise actual app.js for outgoing read-state changes through live events
and polling, distinct same-time message IDs, receipt coalescing/retry/new unread IDs,
peer/session/disposal cancellation and typing timer disposal. Their `messageId` field
matches `bb-go/modern/direct/types.go`; no new production protocol is assumed.
The runner no longer calls process.exit on success, awaits per-case fixture teardown
and has nonrecursive subprocess coverage for awaited completion/error propagation.
Existing transport and Messages regressions are retained.

The Electron source repairs exact-origin/socket classification, route/method/CORS
fixtures, bounded reload, ready empty state, identity-recovery conversation reopening,
same-card expiry and transport recovery. It adds a long transcript, real pointer/key
interaction, focus/anchor, draft and node-stability assertions. These checks have not
run; no browser geometry or runtime acceptance is claimed.

## Limits to carry into the next source/acceptance gate

- `test/walletPay.node.js` invokes exported `paymentInbox.tests[].fn` directly. The new
  per-case teardown currently lives only in `paymentInbox.run()`. Before the combined
  suite is accepted, share teardown with exported case invocation (or an explicitly
  bounded runner integration change). Do not declare the standalone runner proof to
  cover the combined caller. This does not prevent the current standalone red run.
- Fixture teardown clears synthetic timers after disposal for isolation. The explicit
  lifecycle regressions, not that cleanup operation, must prove production ownership.
- Actual Electron execution remains a later gate. The driver must not be weakened to
  accommodate the known focus/viewport defect in current production.

Next: [Hermes finish red 01](../handoff/HERMES_BBD_PAY_001_MESSAGES_FINISH_RED_01.md).
Only this execution handoff is active. Source remains frozen. Review its actual failure
output before authorizing Sol production fixes; syntax errors, runner hangs and blanket
timeouts do not establish behavioral red.
