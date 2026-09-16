# Sol — Messages UI bootstrap correction 01

CLOSED: accepted for execution in [review 07](../testing/BBD-PAY-001-MESSAGES-REVIEW-07.md).
Next authority: [Hermes UI verification 02](HERMES_BBD_PAY_001_MESSAGES_UI_VERIFY_02.md).
Source instructions below are historical. Actor: Codex Sol (`gpt-5.6-sol`, High), owner-relayed.
Reviewer: Codex. Grok remains unavailable after weekly usage exhaustion; retain the
existing documented fallback. Read AGENTS.md, TESTING.md, CURRENT_TASK and
[verification review 06](../testing/BBD-PAY-001-MESSAGES-REVIEW-06.md).

## Baseline and sole writable source

Repository: bb-desktop. HEAD `7a31c41cb29692a94acf1f24adb379f3a829d237`, empty index.
Only edit `test/paymentInbox.electron.js`, currently 903 lines, SHA-256
`d884456fcef8955ca5835da2ae13e6864e334bf4ef6305742385386e478e58bf`.
All other review-05 source/input hashes remain frozen, including app.js at
`539def8f2240023c004b788736888fd021ecb5c12c05997d06b96928011bbdda` and Node suite at
`638b1f4aacf7d75458d9b5592d9e9c8d58a471018a802591b319376baffb8f1a`.
Preserve unrelated dirty work and every artifact/report. No Git operations, execution,
syntax checks, formatter, dependency install, record edits or actor launches. Sol does
not inherit Grok's execution privileges. No Sol completion report is required by its role.

## Observed failure and fixed correction

Hermes's UI run emitted `configured reload did not finish` before sandbox proof or
screenshots. Complete Node green and read-label falsification/restoration already exist.
No repeated Node run is needed for this harness-only change.

The driver invokes renderer location.reload while the actual main attaches a navigation
prevention handler. The old run has insufficient events to establish the exact cause;
do not label it environmental or assume adding time fixes it. Repair the setup as follows:

1. Wait boundedly for the actual initial application document and bootstrap elements
   to be ready. Do not mistake a newly created idle/about:blank webContents for loaded
   social/index.html. Verify the exact local file URL and required DOM identity.
2. Set only the existing fixture bootstrap localStorage values in the actual renderer
   and await that evaluation. Remove renderer `location.reload()` and its dangling
   did-finish-load/did-fail-load promise/listeners.
3. From the driver main process, call and await `win.loadFile(path.join(repoRoot,
   'social', 'index.html'))` under the existing 15-second deadline. Use the real load
   promise, whose rejection remains a failure. The exact local path is reviewer-fixed;
   no renderer-supplied URL, fake document, replacement BrowserWindow or direct production
   state manipulation. Recheck the loaded URL/document and continue all existing ready,
   fixture-hit, sandbox and UI assertions. Initial blocked default bootstrap remains
   expected; no widening of the HTTP/WebSocket fixture allowlist.
4. Retain a bounded fixture-only lifecycle trace for this setup: phase, elapsed time,
   load start/finish/failure and renderer termination. Observe without preventing events
   or removing production listeners. Save a small bootstrap diagnostics JSON under the
   configured artifact root on success and failure, including actual load error code/
   description and current loading/destroyed state when available. Store only allowlisted
   lifecycle facts, never localStorage, descriptor/token, raw HTTP headers or user data.
   Keep event collection bounded and clean up only listeners/timers the harness owns.
   Recording failure diagnostics must preserve the original nonzero test failure.

No product source/security change, guard removal, sandbox override, timeout increase,
catch-and-continue, fabricated success or test assertion removal. Keep all four screenshot
names, viewport/keyboard/pointer assertions, independent fixture endpoints, automatic
updates, cancellation/expiry, focus/anchor/draft/node stability, identity recovery and
unavailable/recovery journey. Keep OS sandbox/webPreferences checks on the renderer that
runs the actual post-configuration journey. Do not move past them on missing data.

The already-recorded failed UI run supplies the setup failure being corrected; this is
not a new feature or permission to redesign the harness. If another frozen file or wider
journey change appears necessary, stop and identify the concrete boundary issue.

## Return and next verification

Stop after the single source-file correction; reviewer inspects it directly. No execution
or repository completion report by Sol. Hermes execution requires the next reviewer-pinned
handoff. It will reuse the existing isolated runtime after full manifest/preflight checks,
write new captures and actual metadata, and run this UI journey with a 300-second outer
deadline. Old artifacts stay immutable. No routine Node suite reruns or real app restart.

The reviewer owns the new Hermes handoff and final acceptance. This source contract does
not authorize publication, final scans or owner manual testing.
