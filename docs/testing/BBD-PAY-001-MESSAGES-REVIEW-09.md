# Messages failure-diagnostics source review 09

Reviewer: Codex. Decision: accept the diagnostic driver for one bounded execution.
No syntax/test/build/Electron commands executed by reviewer. HEAD remains
`7a31c41cb29692a94acf1f24adb379f3a829d237`; index empty.

Only the Electron driver differs from review 07's input set:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| test/paymentInbox.electron.js | 1371 | 698ae487068f24c4bdaff22912268acd3fa81cd9c6f917b38f06fdf6bc1181e9 |

All other 20 review-05 source/input hashes match. Production, Node tests, dependencies
and permission policy stay frozen.

## Reviewed behavior

- Known empty-conversation and configured-ready waits now record distinct failure labels.
  Their predicate expressions and eight-second deadlines remain unchanged.
- Fixture response finish listeners record observed status. Blocker observations record
  classifications without changing the allowlist or callbacks. Responses remain unchanged.
- Failure JSON separately records online, notice-hidden and request-card count, document
  visibility, module/preload availability and bounded labels/conversation count.
- Fixture evidence is constructed using explicit field allowlists and at most 32 recent
  entries per collection. Payment headers/credentials, descriptor and raw signed bodies
  are omitted. The in-memory existing payment headers do not enter the new failure JSON.
- Original fixture activity is captured before the failure-only probe. DOM and actual
  screenshot capture run first (three/five-second bounds); then at most one existing
  read-only preload call is allowed five seconds. Probe output contains only outcome,
  permitted DTO state, row count and fixture-peer equality. It does not update app state
  or rerun assertions. The pre-probe fixture snapshot intentionally excludes probe traffic.
- Terminal failure handling saves ui-failure-diagnostics.json and fixture-failure.png
  when available, then retains the original error and exit 1. Unavailable/rejected/timed-out
  observations are explicit; diagnostic errors do not grant success. A screenshot/probe
  timeout does not cancel its underlying operation, but the terminal path stays bounded
  and exits after diagnostics; no new background retry or product action is introduced.

The source retains bootstrap/sandbox proof and the existing successful UI journey,
including its four success screenshots. Diagnostic screenshot success is not UI acceptance.
Later journey failures may have a generic ui-journey stage and null assertion label;
raw original stderr remains required to identify those exact assertions. The known
readiness failure has explicit stage/label and individual condition values.

## Next authority

[Hermes UI diagnostic execution 03](../handoff/HERMES_BBD_PAY_001_MESSAGES_UI_DIAGNOSTIC_03.md)
authorizes syntax and one sandboxed UI run with new captures. No speculative production
fix, Node rerun, final scan or publication. A repeat of the known failure is useful only
with the new observations retained; it must not be called green. The prior Node results
and bootstrap/sandbox evidence stand with their documented metadata limits.

Sol diagnostics 01 is closed. No actor launched; owner relays. The separately recorded
portable-identity/WebRTC/video direction remains in force and outside this source scope.

Governance paths: this review, new Hermes handoff, closed Sol diagnostics handoff,
CURRENT_TASK, PAY-001 ticket and end-to-end status map.
