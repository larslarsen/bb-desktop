# Sol — Messages pointer and social-fixture correction 01

CLOSED: bounded correction accepted in [review 15](../testing/BBD-PAY-001-MESSAGES-REVIEW-15.md).
Next authority: [Hermes UI verification 06](HERMES_BBD_PAY_001_MESSAGES_UI_VERIFY_06.md).
Source instructions below are historical.
Actor: Codex Sol (`gpt-5.6-sol`, High), owner-relayed. Reviewer: Codex.
Grok remains unavailable after weekly usage exhaustion; retain the recorded fallback.
Read AGENTS.md, TESTING.md, CURRENT_TASK and
[UI review 14](../testing/BBD-PAY-001-MESSAGES-REVIEW-14.md).

## Sole writable path and frozen baseline

bb-desktop HEAD `7a31c41cb29692a94acf1f24adb379f3a829d237`, empty index.
Only `test/paymentInbox.electron.js`, 1392 lines, SHA-256
`ace2bddc1286b17ce9a99c4b031ca1ac65765c5417fc9eb9cc35895440e10123`.
Before copy: dist/pay001-ui-verify05/driver-source.js. Preserve all evidence.
Other 20 review-05 input pins stay frozen. No product, Node suite, dependency, CSS,
transport, policy or daemon changes. No execution (tests/syntax/build/formatter), Git,
reports or actor launch by Sol. Stop after source edits for reviewer inspection.

## Bounded changes

### 1. Make native pointer input target a visible element

Correct clickElement so its native mouse sequence is sent only after establishing a
visible hit target. Before input, scroll the requested element into view with immediate
(non-smooth) scrolling as test setup; remeasure after layout has settled boundedly.
Use at most the existing three-second interaction budget. Account for the element's
visible portion inside the viewport and clipping scroll ancestors, including messageList.
Choose an integer point inside that visible portion and assert document.elementFromPoint
resolves to the element or its descendant. Check again immediately before the native
mouse sequence. A missing, hidden, clipped or covered target must fail with selector,
rect/viewport/point and hit-target diagnostics rather than sending input blindly.
Do not retry clicks until state changes. Preserve one mouseMove/down/up sequence,
existing open/closed waits and the corrected sendEnter helper.

Use normal scrolling only for action setup and screenshot framing. Do not mutate
styles, dimensions, details.open, focus state or app data, call DOM .click(), or substitute
synthetic DOM events for the pointer/keyboard proofs. The helper must work for existing
conversation rows, composer and summary callers. Do not broaden unrelated input flows.

### 2. Capture the card actually in view

Before fixture-request-card-980x720.png, position the payment card so its heading and
amount are visible inside the transcript. Assert their visible intersections with the
transcript/viewport before capture; horizontal geometry checks alone are insufficient.
A long expanded card may exceed the scrollport height: do not require the whole card to
fit or reduce content. Keep the open-details geometry checks, 80 text messages, long
memo, both window sizes and all four existing screenshot names. An additional details
capture is allowed only if needed to show a separately scrolled part of the same card.
Keep actual captured dimensions in evidence.

Scrolling here must occur before anchorBefore. Between anchorBefore and anchorAfter,
no helper scroll, focus repair, content mutation or screenshot setup may conceal product
movement. Preserve the focus, open-state, draft, stable-node, anchor and bottom-follow
assertions exactly. New helper checks may fail early with actionable diagnostics.

### 3. Accept observed social POST traffic within the existing exact fixture routes

The absent-Origin allowance currently covers only matching GET. Extend it to matching
GET or POST when socialMethod already authorizes that exact path/method and query is
empty. This includes the existing /ob/chat typing notification and the exact allowed
peer mark-read routes. Keep literal null accepted as before; reject empty/other supplied
origins, absent-Origin OPTIONS, unknown paths, queries and mismatched methods. Preserve
OPTIONS preflight validation, CORS response headers, all response payloads, fixture
failure accounting, network blocker and final zero-fixture-failures assertion.
Never classify this 403 as expected or suppress typing to avoid it. Do not alter the
separate authenticated payment server, actual app Origin behavior or security settings.

## Verification boundary

Retained UI05 is red evidence for pointer preparation; its screenshot and POST403 also
expose the other two driver deficiencies. Keep all subsequent journey assertions and
deadlines. No product defect is established by this failure. If a product change or
wider source scope appears necessary, stop and leave it for reviewer decision.

Reviewer will compare this single file with the retained before copy and pin the result.
Then Hermes will receive a fresh one-run syntax/full-UI handoff with required metadata
and full manifests verified before execution. Missing old metadata stays an explicit
gap; no reconstruction or rerun of unchanged Node suites. No owner manual test needed.
