# Messages UI verification review 12

Reviewer: Codex. Decision: Origin correction and populated-rendering progress verified;
full UI acceptance still fails at keyboard activation. Authorize bounded driver-input
correction only. No tests, builds, syntax checks or Electron run by reviewer.

HEAD `7a31c41cb29692a94acf1f24adb379f3a829d237`, index empty. All 21 review-11
input pins match now; all 21 saved before-manifest hashes and retained driver-source.js
match. Driver remains 1373 lines, SHA-256
`1e47205c5961a3d5f24ebadd42052e5bb8c7d0afcaff4ee2812b56ecb94a3927`.

## Verified progress and actual failure

Read [Hermes UI04 report](BBD-PAY-001-HERMES-UI-VERIFY-04.md), retained stderr,
diagnostics/proofs, and inspected the populated screenshot.

- Bootstrap and renderer sandbox checks passed. Social GETs now succeed: nine captured
  social responses are 200, with four successful payment reads, zero fixture failures
  and zero unexpected blocked requests.
- Ready-empty checks passed, then the driver displayed the long transcript and one
  payment request in Messages. Amount, memo, asset/network, requested status and exact
  request ID assertions passed before pointer interaction opened details.
- The actual failure is `keyboard did not close details` after **Enter**. The driver
  never sends Escape. Hermes's Escape diagnosis is incorrect; do not add Escape product
  behavior based on that report.
- The screenshot visibly shows the request and expanded details. Subsequent keyboard
  reopen, viewport/anchor, cancellation/expiry and recovery assertions have not all run.
  Neither a complete journey pass nor final acceptance is claimed.

## Driver-input diagnosis and correction

The driver sendEnter helper at line 785 sends only keyDown/keyUp. Electron exposes a
separate char event and requires the containing BrowserWindow to be focused for injected
input. Chromium summary activation delegates to keyboard activation, whose Enter path
handles keypress carriage return. The pinned Electron v44 converter populates character
text for char events; a carriage-return character is an explicit encoding for this path.
[Electron input contract](https://www.electronjs.org/docs/latest/api/web-contents#contentssendinputeventinputevent),
[pinned converter](https://raw.githubusercontent.com/electron/electron/v44.0.0/shell/common/gin_converters/blink_converter.cc),
[Chromium keyboard activation](https://chromium.googlesource.com/chromium/src/+/e8324e4457466252853f8f6b7731de6bca797e2a/third_party/blink/renderer/core/html/html_element.cc).

Inference: the incomplete injected sequence is a concrete harness defect consistent
with this failure. Actual OS window focus was not captured, although the screenshot
shows a summary focus ring. Correct both input preconditions without altering product
handlers or assertions. This does not yet prove the corrected journey will pass.

## Retained evidence

| Artifact | SHA-256 |
| --- | --- |
| [Populated failure screenshot](../../dist/pay001-ui-verify04/ui-01/fixture-failure.png) | aec564221de3f28fce72677991ea87c6b5657ac3deb65c0c0f0c3b9a3d77674c |
| [Empty screenshot](../../dist/pay001-ui-verify04/ui-01/fixture-empty-messages-1440x1000.png) | 823a825e90b8f6553857416fac68ae7c1394db3f125768229d44daecbf1c25ee |
| [Failure diagnostics](../../dist/pay001-ui-verify04/ui-01/ui-failure-diagnostics.json) | 870b0b1bb499cb32d708dd63e56306bf809e661866ee177af9b8c044cf2e3cc8 |
| [UI stderr](../../dist/pay001-ui-verify04/ui.stderr.log) | 6995b6620f1a48176b95aec795f3356e07ec175ce6cc7f404d03d964d6d62431 |
| [Bootstrap](../../dist/pay001-ui-verify04/ui-01/bootstrap-diagnostics.json) | 9564f5036344d201b02068c34c94719a8c0b1edec50e5ca6bb2e6d2e8bf82ebf |
| [Sandbox](../../dist/pay001-ui-verify04/ui-01/sandbox-status.json) | c6fc74ddcdd6130d17b3b27f0566d3ce2a3a05da700afdaa561c5568aec24b05 |

Both PNG headers are **1440 x 934**. The report's 1440x1000 for the empty PNG is the
requested outer-window size encoded in its filename, not its actual captured content
size. Correct the interpretation here; preserve original files/names. These are fixture
images, not owner/live requests or successful end-to-end payment proof.

No syntax/UI metadata or source/runtime after-manifests were retained. Runtime-before
is again a summary, not a per-entry manifest. Source hashes/driver copy and direct
artifacts support the findings above, but exact command exit/timing and before/after
runtime assertions remain actor-reported. No metadata reconstruction or repeated Node
suite requested. Final acceptance still requires complete execution evidence.

## Next authority

[Sol keyboard-input correction 01](../handoff/SOL_BBD_PAY_001_MESSAGES_UI_KEYBOARD_01.md)
authorizes only the helper and its three call sites, with focus preconditions and the
complete native Enter sequence. Preserve the UI interaction assertions and all remaining
journey behavior. Hermes UI04 is closed. No product changes, Node rerun, owner manual
test, scans or publication. Identity/WebRTC/video planning remains unchanged.

Governance paths: this review, new Sol handoff, closed Hermes UI04 handoff, CURRENT_TASK,
PAY-001 ticket and end-to-end status map.
