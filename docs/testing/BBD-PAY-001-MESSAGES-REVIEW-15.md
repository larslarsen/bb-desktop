# Messages pointer/fixture source review 15

Reviewer: Codex. Decision: accept the bounded driver correction for UI verification.
No tests, syntax checks, builds or Electron executed by reviewer.
HEAD `7a31c41cb29692a94acf1f24adb379f3a829d237`; index empty.

| Changed path | Lines | SHA-256 |
| --- | ---: | --- |
| test/paymentInbox.electron.js | 1599 | f09c2dcc214b4fb97699d77eecf7407391cdcff5d1785cf850a1d59455abaf54 |

Compared directly with dist/pay001-ui-verify05/driver-source.js. Changes are confined
to the three authorized driver areas:

- Social fixture accepts absent Origin only when actual method equals the exact route's
  required GET or POST method. Missing-Origin OPTIONS, supplied empty/other Origin,
  unknown/query routes and method mismatches remain rejected. The preflight handling,
  response payloads, network blocker and payment authentication are unchanged.
- clickElement scrolls once with instant/nearest positioning, intersects the target with
  viewport and clipping ancestor client rectangles, selects an integer visible point,
  and checks elementFromPoint against the target or descendant. It waits for two stable
  measurements within the three-second polling budget, then rechecks the same point
  immediately before one native mouseMove/down/up sequence. Failure includes selector,
  rectangles, clipping, point and hit-target diagnostics. There is no click retry loop,
  DOM click, direct open-state/focus mutation or product change.
- The narrow card capture scrolls the exact request card to the top and waits for visible
  heading/amount intersections with the transcript and viewport. Those selectors match
  the real renderCard output. The existing long content, open-details geometry checks,
  screenshot names and window sizes remain. Actual visibility and screenshot quality
  require the next run; source review alone does not establish them.

The sendEnter helper is byte-identical. The complete anchorBefore-through-remaining-
journey section is byte-identical. No setup scrolling was inserted between anchorBefore
and anchorAfter. All existing pointer/keyboard state, draft/focus, stable-node, scroll,
status, identity, recovery and final zero-fixture-failure assertions remain intact.
The other 20 review-05 source/input hashes are unchanged. No product rebuild is needed.

[Review 14](BBD-PAY-001-MESSAGES-REVIEW-14.md) retains the actual failed UI05 evidence
and its limitations. This source correction does not imply a passing UI journey.

Next authority: [Hermes UI verification 06](../handoff/HERMES_BBD_PAY_001_MESSAGES_UI_VERIFY_06.md).
Run syntax and one full sandboxed journey against these frozen inputs. Verify required
preflight files and complete command metadata on disk before UI launch; absent evidence
stops execution. Prior metadata gaps stay explicit. Prior Node and partial UI proof
stands; no unchanged Node reruns, wider scans, Git or user-process restart.
Sol pointer correction is closed. No actor launched; owner relays. Identity/WebRTC/video
direction is unchanged. Full acceptance and publication remain pending.

Governance paths: this review, new Hermes handoff, closed Sol pointer handoff,
CURRENT_TASK, PAY-001 ticket and end-to-end status map.
