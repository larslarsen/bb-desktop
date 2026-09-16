# BBD-PAY-001 source review 02

Reviewer: Codex. Date: 2026-09-15. Decision: correction 02 required; no final acceptance.
Input: [Grok correction 01](BBD-PAY-001-GROK-CORRECTION-01.md).

## Verified progress

All eight changed-file hashes/line counts, five frozen drop hashes and six original
frozen input hashes match. All six retained command metadata hashes and their twelve
stdout/stderr hashes match. Metadata records green exits for 33 inbox, 6 preload,
32 Electron security and 53 combined wallet/inbox tests. Instance-binding falsification
records exit 1 and the expected wrong-instance assertion; restored source matches.
Original eight logs remain hash-identical. The correction-red logs retain FIFO,
wrong-party and impossible-date failures; their missing process metadata is an explicit
historical gap, not a green result. Reviewer ran no implementation tests.

Source addresses the earlier HTTP error draining, generation/coalescing, non-regular
descriptor open, contradictory record handling, fixed IPC arity, picker exception and
failed-wallet-shutdown problems. Those repairs should be preserved. The renderer now
sequences responses, and the UI harness isolates social traffic and holds startup.

## Remaining blockers

### 1. Real daemon receipt timestamps are rejected (P1)

`wallet-pay/inbox-client.js:20` now accepts only whole-second received_at strings.
`../bb-go/modern/payment/types.go:101` declares this field as time.Time;
`service.go:365` and `:409` store `s.now().UTC()` without truncation. Go time.Time
MarshalJSON emits RFC3339 with sub-second precision (verified in the installed Go
standard-library source, time/time.go:1587). A normal value such as
`2026-09-15T12:01:00.123456789Z` therefore invalidates the whole inbox at line 361.
The signed request timestamp codec's whole-second format does not govern this
unsigned receipt metadata. Preserve strict calendar validation while accepting zero
or one-to-nine fractional digits in UTC. Add an otherwise-valid loopback regression.

### 2. UI smoke still cannot start; cause is not fully established

The retained smoke metadata records exit 1 in 0.22 seconds; stderr says "No usable
sandbox" and SIGTRAP. No renderer, OS sandbox proof or screenshot exists. The report
attributes this to AppArmor/user namespaces, but the Chromium message presents that
as one possible cause. Read-only reviewer diagnostics show user namespaces enabled,
AppArmor's unprivileged-userns restriction enabled, and the review tool itself confined
under `bwrap//&unpriv_bwrap`. These do not establish the exact confinement of Grok's
failed child. A failed nested agent-sandbox run must not be treated as proof the host
cannot run the isolated smoke. Correction 02 permits the exact fixture command through
the tool's normal escalation mechanism, retaining Chromium sandbox verification and
without changing host policy. If denied/unavailable, preserve that distinct blocker.

### 3. Visual checks can pass while text is clipped (P2)

`social/styles.css:499` hides horizontal overflow on each request/list/status container,
while `.paymentRequestMemo` has no wrapping for long unbroken values. The smoke itself
uses a 180-character unbroken memo. `noClip()` at test/paymentInbox.electron.js:236
only checks the outer element rectangle; hidden overflowing text still passes.
The full daemon peer subtitle also lacks a wrapping rule. Wrap these values and
assert content bounds/scrollWidth, not just card bounds. Screenshots must demonstrate
the specified long inputs at both sizes. At lines 353–355, picker invocation is awaited
but the resulting render is not; await the completed state to avoid a loading race.

### 4. Several named regression obligations remain unproved

The correction suite adds useful cases but still lacks the requested HTTP header-cap
and abruptly closed response checks, valid response bodies below/at the 4 MiB cap,
and a get call between stale completion and newer completion to prove coalescing.
Its descriptor-read race releases the gate immediately without waiting for entry;
its "root and private-dir symlinks" case only exercises the private directory.
The IPC hostile frame/URL cases exercise get but not connect; DTO cloning is not
asserted for the new channels. Correct these specific assertions in the existing
targets. No broad suite expansion or new test framework is authorized.

## Next authorization

[Grok correction 02](../handoff/GROK_BUILD_BBD_PAY_001_CORRECTION_02.md) is the only
active execution handoff. It repairs the timestamp regression and remaining UI/test
issues, then attempts the isolated UI smoke through an appropriate execution context.
No Hermes final acceptance, source publication, dependency change or user app restart.

All decisions and scope are recorded here and in CURRENT_TASK/ticket before execution.
Reviewer governance paths: this review, correction-02 handoff, CURRENT_TASK active
prefix, BBD-PAY-001 ticket status and payment integration status map.
