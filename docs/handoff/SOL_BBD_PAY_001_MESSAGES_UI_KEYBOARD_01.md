# Sol — Messages keyboard-input correction 01

CLOSED: bounded correction accepted in [review 13](../testing/BBD-PAY-001-MESSAGES-REVIEW-13.md).
Next authority: [Hermes UI verification 05](HERMES_BBD_PAY_001_MESSAGES_UI_VERIFY_05.md).
Source instructions below are historical. Actor: Codex Sol (`gpt-5.6-sol`, High), owner-relayed.
Reviewer: Codex. Grok remains unavailable; retain the recorded fallback.
Read AGENTS.md, TESTING.md, CURRENT_TASK and
[UI review 12](../testing/BBD-PAY-001-MESSAGES-REVIEW-12.md).

## Sole writable path

bb-desktop HEAD `7a31c41cb29692a94acf1f24adb379f3a829d237`, empty index.
Only test/paymentInbox.electron.js, currently 1373 lines, SHA-256
`1e47205c5961a3d5f24ebadd42052e5bb8c7d0afcaff4ee2812b56ecb94a3927`.
Exact before copy: dist/pay001-ui-verify04/driver-source.js; preserve it and all evidence.
Other 20 review-05 input pins stay frozen. No production/Node/fixture-response/policy
changes. No test/syntax/build/formatter execution, Git, reports or actor launches by Sol.

## Evidence and bounded correction

Origin fix worked. The real UI shows a request in Messages; pointer input opens its
native details element. It fails when sendEnter is expected to close details. Hermes
incorrectly described Escape: source uses Enter, and no Escape behavior is authorized.
The helper sends only keyDown/keyUp; review 12 links the primary input/activation sources.

Change only sendEnter and its three existing call sites:

1. Make the helper awaitable. Focus the owned BrowserWindow/webContents through Electron's
   native focus APIs and wait boundedly (at most three seconds) for actual focus. If focus
   is unavailable, fail explicitly; never fake focus state or weaken the assertion.
2. Before sending keys, assert the existing payment details summary is the actual DOM
   activeElement. It should have focus from the preceding real pointer action or retained
   keyboard action. Do not call DOM summary.focus() to conceal lost product focus.
3. Send one complete native Enter sequence through webContents.sendInputEvent, in order:
   rawKeyDown with keyCode Enter; char with keyCode String.fromCharCode(13) (carriage return);
   keyUp with keyCode Enter. No synthetic DOM KeyboardEvent, direct .click(), setting
   details.open or inserting text as a replacement for native keyboard activation.
4. Await the helper at each of its three existing call sites. Preserve the existing
   close/reopen assertions, their three-second deadlines, pointer events and the later
   focus/anchor preservation proof. No key-repeat loops or retry-until-open shortcuts.

A concise comment explaining the char event is sufficient. No additional diagnostic
framework, screenshot/layout redesign, fixture change or product keyboard handler.
If other source work appears necessary, stop at this boundary. The retained failed
native UI run is the red evidence; no new Node suite for a test-driver input correction.

## Return

Stop after source edits. Reviewer compares the single file with the retained before
copy; no Sol report. A new reviewer-pinned Hermes task will run syntax and the full
sandboxed UI journey with fresh artifacts and actual command metadata. Existing partial
proof and Node evidence stand. No owner manual test or user-process restart.
