# Messages keyboard-input source review 13

Reviewer: Codex. Decision: accept the bounded driver correction for UI verification.
No tests, syntax checks, builds or Electron executed by reviewer.
HEAD `7a31c41cb29692a94acf1f24adb379f3a829d237`; index empty.

| Changed path | Lines | SHA-256 |
| --- | ---: | --- |
| test/paymentInbox.electron.js | 1392 | ace2bddc1286b17ce9a99c4b031ca1ac65765c5417fc9eb9cc35895440e10123 |

Direct comparison against dist/pay001-ui-verify04/driver-source.js shows only the
sendEnter helper and its three call sites changed. The helper checks owned window/
contents availability, requests native focus, waits at most three seconds for actual
focus and asserts the existing summary is activeElement. It sends rawKeyDown Enter,
char carriage return and keyUp Enter. All three callers now await it.

There is no DOM focus override, direct click/open mutation, synthetic DOM keyboard event,
retry loop or product change. Existing pointer/keyboard close/reopen assertions and
three-second deadlines are unchanged. All other driver bytes and the other 20 review-05
source/input hashes are unchanged. The remaining geometry, reconciliation, identity and
recovery assertions still require runtime evidence.

The actual failed Enter run and rationale are retained in
[review 12](BBD-PAY-001-MESSAGES-REVIEW-12.md). Source correctness is not a claim of
successful native focus or a passing UI journey in the execution environment.

Next authority: [Hermes UI verification 05](../handoff/HERMES_BBD_PAY_001_MESSAGES_UI_VERIFY_05.md).
Run syntax and one full sandboxed journey with fresh metadata/manifests and actual images.
No source edits, Node suite reruns, broader scans, publication or user-process restart.
Sol keyboard correction is closed. No actor launched; owner relays. Prior partial UI and
Node evidence stands. Identity/WebRTC/video direction is unchanged.

Governance paths: this review, new Hermes handoff, closed Sol keyboard handoff,
CURRENT_TASK, PAY-001 ticket and end-to-end status map.
