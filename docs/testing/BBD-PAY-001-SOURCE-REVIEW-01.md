# BBD-PAY-001 source review 01

Reviewer: Codex. Date: 2026-09-15. Decision: CORRECTION REQUIRED.
Reviewed drop: [Grok report](BBD-PAY-001-GROK-01.md).
No Hermes acceptance or publication authorized.

## Verified evidence and visible status

All thirteen source hashes, sizes and line counts in Grok's final table match the
working tree. All eight retained log hashes match. The six frozen desktop inputs
and frozen daemon server hash match the original handoff. Desktop HEAD remains
`7a31c41cb29692a94acf1f24adb379f3a829d237`; index is empty.

Raw logs show 17 inbox, 6 preload, 31 Electron security and 37 combined wallet/inbox
tests passing. Falsification log shows the wrong-instance assertion failing with
ready versus invalid; the restored source matches its reported hash and green log.
These are inspected existing results, not reviewer-executed tests. Exit values are
actor-reported; separate process metadata was not retained. The report admits that
tests and production were authored together and no initial test-first red was saved.
Do not reconstruct that history as if it happened. Corrections need real test-first red.

The Requests tab and screen exist in source. Neither Electron attempt reached UI
validation: both logs show Chromium's misconfigured SUID helper abort. Current helper
is owned by the local non-root user, mode 0755. No screenshot exists, so the visual
outcome and actual Electron path are not accepted. The user's app was not restarted.

## Blocking findings

### R1 — HTTP error paths leave responses running (P1)

`wallet-pay/inbox-client.js:256`–`291` calls `res.resume()` then `finish()` for
rejected headers, non-200 statuses and encoding. `finish()` clears the absolute
deadline without destroying the response/socket. A continuously streaming rejected
response can outlive the reported failure; its controller is removed from the client
set and dispose no longer owns it. The 503 path retains only 256 bytes but consumes
unlimited streamed bytes until its deadline. `maxHeaderSize` is not passed to HTTP;
post-parse rawHeaders counting is not the specified parser limit. Abort listeners
also remain attached after settlement. Fix termination and cleanup on every branch,
bound all response bodies, and prove socket closure with actual loopback regressions.

### R2 — disposal/reselection races escape ownership (P1)

At `inbox-client.js:568`, descriptor I/O happens before an AbortController exists.
Disposing or selecting another directory during that await does not prevent old work
from starting HTTP at line 579. Generation/disposal is checked only after success.
The test named "dispose aborts in-flight work" calls dispose immediately after get;
it permits waiting for the five-second deadline and never asserts zero post-dispose
HTTP, so it misses this race. Separately, lines 605–607 clear any pendingGet when an
old generation settles, including a newer generation's request, breaking coalescing.
The renderer's `run()` also applies every response without sequencing.

At `social-main.js:210`, a quit attempt permanently disposes the inbox before wallet
shutdown is approved. If wallet shutdown fails, the existing app deliberately remains
open, but payment access is permanently unavailable. Dispose on committed quit/window
destruction, preserving the wallet's failed-quit behavior. Add lifecycle regressions.

### R3 — opening a non-regular descriptor can hang (P2)

`inbox-client.js:153` opens O_RDONLY before checking the file type. A FIFO named
connection.json can block the open indefinitely; fstat never gets to reject it.
Descriptor work has no timeout and cannot be cancelled. Reject non-regular files
before opening and use O_NONBLOCK with O_NOFOLLOW plus opened-file validation so a
FIFO replacement cannot turn the type check into a blocking open. The 16 KiB tests
currently use invalid JSON at every size and do not prove acceptance at the limit.

### R4 — contradictory records are silently discarded (P2)

`inbox-client.js:467` silently skips inbound requests for another payer. Lines
495–496 silently ignore a linked cancellation whose direction/public key contradicts
its request. Outbound requests are discarded before cancellation association, so
contradictions linked to those requests cannot be checked. The ticket requires a
whole-snapshot failure for malformed/contradictory records, while permitting genuine
orphan cancellations. Validate all retained request associations before filtering rows.
Signature shape is only checked for a nonempty string, and received_at accepts
Date.parse normalization of impossible dates; implement the bounded schema checks.

The paid-event test uses an incorrect all-zero digest; digest rejection alone makes
it pass even if paid-event rejection is removed. The test named same-peer rotation
does not rotate credentials for the same peer after an initial successful read.
Hostile endpoints, at-limit valid responses, wrong response peer, stale generations,
cancelled-after-expiry and several cleanup cases required by the ticket are missing.
Fix these proofs within the existing focused suite, using otherwise-valid fixtures.

### R5 — fixed IPC arity and closed failures are incomplete (P2)

`social-main.js:278` accepts `(event, value)` and checks only value. Extra arguments
after an explicit undefined value are ignored and side effects proceed. Use an exact
zero-payload arity check. Both channels need wrong sender, wrong frame, wrong URL and
extra-argument cases with counters proving zero disk/HTTP/dialog side effects.
Current security test covers only one malformed sender and one payload per channel,
and counts only dialogs. A rejected native picker also escapes as a raw rejected
promise from `connectInbox`; contain it into a closed non-ready DTO.

### R6 — actual Electron harness is not ready to prove the UI (P1)

`test/paymentInbox.electron.js:163` loads the app only after asynchronous filesystem
and server setup; the app registers `app.on('ready')`, which the harness can miss.
The harness must register production startup before readiness and set disposable
paths before any startup I/O. Do not change production startup merely to repair it.
It never blocks renderer social HTTP/WebSocket traffic, although social/app.js
automatically connects to 127.0.0.1:4002. Running this harness as written could contact
the owner's daemon. Block that traffic before page load and assert isolation.

It assigns process.env.HOME, contrary to the task environment rule; use app.getPath
overrides with disposable paths without repurposing HOME. It opens details by assigning
`.open` rather than exercising keyboard input, and captures without asserting viewport
visibility or runtime sandbox state. Success-path cleanup recursively deletes computed
paths; retain owned temporary directories instead, and close owned processes, servers,
requests and timers through a failure-safe finalizer. No screenshot was generated.

## Corrective authorization and environment decision

Execute only [Grok correction 01](../handoff/GROK_BUILD_BBD_PAY_001_CORRECTION_01.md).
It fixes R1–R6 and records a new report, preserving this drop's evidence. Same source
actor and product scope. No broad Rust suite, new dependencies or daemon changes.

For the isolated Linux UI smoke only, permit `--disable-setuid-sandbox` while retaining
the user-namespace and seccomp sandbox, verified at runtime. Chromium documents user
namespaces as the alternative to its SUID helper and this flag as disabling that helper:
[Chromium Linux sandbox documentation](https://chromium.googlesource.com/chromium/src/sandbox/+/refs/heads/main/linux/).
This is a conditional test-runner route, not proof this host supports it. If runtime
sandbox checks fail or namespaces are denied, retain the failure and report the exact
environment gap. No `--no-sandbox`, privileged helper modification, host security
changes or product launch-script changes are authorized. The original failed attempts
must remain recorded; their absence of screenshots is not acceptance.

Reviewer changed only this review, correction handoff, active CURRENT_TASK prefix,
ticket status/correction reference and integration status map. Decisions and execution
authority reside in these documents under the owner's existing control plane.
