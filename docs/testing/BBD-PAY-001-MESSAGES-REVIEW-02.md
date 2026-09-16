# Messages correction review 02

Reviewer: Codex. Decision: further bounded correction required; Sol fallback selected.
Grok is unavailable after weekly usage exhaustion. No acceptance/tests were executed
by the reviewer. Recovered source identities match the recovery handoff, HEAD remains
`7a31c41cb29692a94acf1f24adb379f3a829d237`.

## Recovery record corrections and evidence

[Hermes recovery](BBD-PAY-001-HERMES-CORRECTION-RECOVERY-01.md) correctly distinguishes
recovered work from new execution, but its red counts are wrong. Actual run-01-red
stdout has **56 passing cases**, stderr has **7 failing cases**. Old-Alice-history is
not among those seven; it fails in run-05-falsify (62 pass, 1 fail). Do not repeat the
recovery report's claim of 7 passes/8 failures or eight original red failures.

Reviewer independently read all eleven command metadata/log pairs and verified their
stdout/stderr hashes. Metadata SHA-256 identities:

| Capture directory | Exit | Metadata SHA-256 |
| --- | ---: | --- |
| run-01-red | 1 | d5fc1cb72e077cf55d3eb6514ea85005cebc792bcacba993d873d6a6b1ccdcba |
| run-02-check-app | 0 | 8505680e00b61e9f90ce6fb0ca858aec9a69554dda6ef4de26cd916ca32c42e3 |
| run-02-check-electron | 0 | 15c9ad01c22d1e67ecb6a2e61d00d4d305294871ec55aa9d8929b3dbb5dabd3d |
| run-02-check-inbox | 0 | ab2441a4601d2850d9699c0b7997d7aeb2286c966773a537c8e46cfb9e328f3e |
| run-02-green | 0 | 63386e3fec527d452474d7d30444243214113dce7281b44390ef4a3c334ae6c9 |
| run-02-security | 0 | bff0e632df74363f454376fa6dd8c60367fdcce3b8aeffed3d54185fa13d0ae8 |
| run-02-socialcore | 0 | b07373cab332c185e5935d0823910cf7ff7fa4c2443ebf902c07b8f2dad0252a |
| run-03-falsify | 0 | cc456fed47196a01cd329af756e6d77cbd37f217ee54cef9e2f2fcc85a621a0e |
| run-04-green | 0 | 89025cd8d67b604c6af385ef6096cffcd3bec086c1c7ddaf6466d12015e92210 |
| run-05-falsify | 1 | e8d9c26de4a41c8c628a3b397df5f59521ec5097d83d976b3831f2b440a09c10 |
| run-06-restored | 0 | a8cd9aa0bd449684a9c12fe98071e5616499c0745866a0391732df138aaf81a7 |

Captures: [retained directory](../../dist/pay001-messages-correction01/).
Green/restored logs contain 63 passes, security 32. Missing mutation/restoration source
manifests and test-first chronology remain gaps. The recovery report also omits the
requested raw-log links/hashes and actual Hermes executable version; this review's
independent capture verification is the evidence basis, not the report's conclusions.
One successful high-value falsification was required, not one for every red failure.
The run-05 failure is relevant, but exact mutated/restored bytes remain unproven.

## Improvements established by source inspection

The correction routes chat reads through a shared pass, guards session/peer application,
uses separate HTTP deadlines, permits successful siblings to update independently,
starts reconciliation before ancillary startup reads, clears old chat DOM synchronously,
and guards stale sockets. Send completion preserves another conversation's draft.
Disposal now invalidates the chat pass and removes its visibility/lifecycle listeners.
Peer merge uses a null-prototype map; outgoing previews regain `You:`.
These fixes are retained; the remaining work does not restart the integration.

## Remaining findings

### R1 — read status and repeated text identity

`social/app.js:414–420` keys text by timestamp/direction/body, excluding read state and
message identity. `:518–524` reuses a text node without updating it. When a sent message
changes read=false→true, transcriptKey changes but the reused bubble still lacks its
read label. Two legitimate identical text messages with the same timestamp share a
key; a subsequent rebuild can append the same node twice and lose one displayed entry.
Update mutable text status and use unique stable identity/occurrence handling. Preserve
distinct duplicate messages and unchanged node stability.

### R2 — read receipts bypass ownership and coalescing

`maybeQueueRead` (`:717–727`) posts whenever a history snapshot has unread text, without
an inflight or acknowledged-message guard. A second event/poll while the post is pending
starts another post. Each successful post requests another immediate pass; a repeated
unread snapshot can cause a tight POST→GET loop instead of bounded reconciliation.
Receipt requests are not owned/aborted by disposal or session/peer changes. Coalesce
per captured session/peer and displayed unread set; retry failures on the normal cadence.
Newly arriving unread text must still receive its own acknowledgement after display.

### R3 — viewport/focus proof and test process lifetime

`restoreViewport` (`:454–469`) restores scroll and then calls focus without preventScroll.
With a details summary focused, the user can scroll elsewhere; a background update
removes/reinserts the nodes and refocuses that off-screen summary, pulling the viewport
away from the preserved anchor. The Electron driver still has no scroll/focus-after-
update assertions. Prove this in an actual long scrollable transcript.

`test/paymentInbox.node.js:2400` adds unconditional `process.exit(0)` to the exported
runner. It kills a host caller instead of returning and can conceal live resources or
late failures. Remove the success exit, clean test-owned resources, and prove normal
CLI termination and awaited-runner continuation. Do not treat a forced successful exit
as evidence of lifecycle cleanup. Remaining live typing/API work must be included in
targeted disposal tests, rather than just checking that request counts did not grow.

### R4 — Electron journey still contains deterministic failure paths

- `installBlocker` (`:275–296`) classifies the fixture's normal
  `ws://127.0.0.1:<socialPort>/ws` as unexpected; at `:643` this fails the run. Explicitly
  recognize that exact owned socket attempt, keep it blocked to test HTTP fallback, and
  reject all other traffic. Match expected bootstrap origin/path exactly, not prefixes.
- Identity recovery (`:617–626`) reconnects, which correctly closes active chat, then
  waits for a rendered request card without reopening a conversation. Assert recovered
  peer row, open it through the real UI, then assert that peer's request card.
- The fixture CORS helper reflects arbitrary origins and advertises unused methods;
  routes mostly ignore method. Restrict it to file-origin fixture requests and actual
  required route/method pairs. Unexpected calls must be recorded as test failures.
- Empty readiness, long transcript reading/focus, exact cancellation identity and
  transport-unavailable→recovered behavior still need substantive assertions. Current
  expiry case only loads an already-expired request; exercise the boundary on the same
  card. Reload completion needs a deadline so a failed navigation cannot hang forever.

## Routing

Recovery is closed with the corrections/gaps above. Grok remains paused. The documented
Sol exception applies: Grok stopped without an accepted correction drop and is unavailable.
Select `gpt-5.6-sol`, High, for bounded test/driver source under
[Sol Messages finish tests 01](../handoff/SOL_BBD_PAY_001_MESSAGES_FINISH_TESTS_01.md).
Sol's role does not permit executing tests or writing evidence. Therefore tests are
authored first, reviewed, then Hermes records behavioral red before production repair.
This is role separation required by AGENTS.md/TESTING.md, not a new product decision.
No owner testing, runtime acceptance, publication or further recovery-only loop is needed.
