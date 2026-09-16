# Sol — Messages UI failure diagnostics 01

CLOSED: accepted for diagnostic execution in [review 09](../testing/BBD-PAY-001-MESSAGES-REVIEW-09.md).
Next authority: [Hermes diagnostic 03](HERMES_BBD_PAY_001_MESSAGES_UI_DIAGNOSTIC_03.md).
Source instructions below are historical. Actor: Codex Sol (`gpt-5.6-sol`, High), owner-relayed.
Grok remains unavailable; retain the documented fallback. Read AGENTS.md, TESTING.md,
CURRENT_TASK and [review 08](../testing/BBD-PAY-001-MESSAGES-REVIEW-08.md).

## Frozen baseline and sole writable path

bb-desktop HEAD `7a31c41cb29692a94acf1f24adb379f3a829d237`, empty index.
Only `test/paymentInbox.electron.js` may change, currently 1110 lines, SHA-256
`ead9f500b8eaafbc19d30961e8327b992437838b09d033e3edacf0d4f79192e1`.
All other 20 review-05 source/input hashes remain frozen. No product, Node suite,
permissions, dependency or daemon changes. No tests/syntax/build/formatter execution,
Git, reports or actor launches by Sol. Preserve every existing artifact.

## Evidence-driven scope

The configured document and sandbox checks passed. The preceding No conversations
wait passed. The next combined ready-empty predicate failed; its individual values
and transport observations were lost when the driver closed. The actor report guessed
the wrong predicate. Add failure observability; do not guess a behavior correction.

1. Retain the failed assertion label/stage and capture actual values of the three
   readiness conditions separately: online status, payment notice hidden, request-card
   count. Include document visibility/ready state, existence of payment module/preload
   methods, notice/connection labels and conversation count. Observe existing DOM only;
   no state writes, synthetic ready class, fake DTO, navigation or automatic retry.
2. Save bounded sanitized fixture activity on failure: social method/path/origin and
   response status, payment method/path/status only, and allowed/expected/unexpected
   blocker classification. Counts plus at most 32 recent entries per collection suffice.
   Never serialize paymentHits' raw headers or fixtureFailures' headers; they contain
   bearer/instance material. Sanitize by explicit field allowlist, not deleting known
   secret keys from an arbitrary object. No response bodies, canonical/signature/receiver,
   descriptor, localStorage dump or user data.
3. After recording the original failed state, optionally call the existing renderer
   getPaymentInbox bridge once as a failure-only read probe, bounded to five seconds.
   Record only allowed state enum, row count and whether its peer matches fixture LOCAL;
   no raw DTO/peer identifiers/errors or additional calls. Distinguish probe success,
   rejection and timeout. This must never feed state back into the app, rerun the failed
   assertion or turn the run green. It helps separate IPC/transport readiness from DOM
   binding while preserving the original observed failure.
4. Capture a screenshot of the actual failure state, named fixture-failure.png, before
   cleanup where possible. It is explicitly a failed-run diagnostic image, not one of
   the four required success screenshots. Preserve success screenshot names/assertions.
5. Emit one ui-failure-diagnostics.json under the configured artifact root in the terminal
   failure handler, with the above facts, screenshot identity and original assertion
   label. Bound every asynchronous capture, including destroyed-renderer cases, so
   diagnostics cannot hang cleanup. If diagnostics fail, retain the original error/exit;
   record unavailable fields rather than fabricate defaults. Preserve bootstrap proof.

Keep code minimal and local to this diagnostic task. No broad framework, unrestricted
console/header/body logging or future identity/WebRTC implementation. Permission-policy
changes, CORS relaxations, timeout increases, dropped assertions and speculative fixture
repairs are outside scope. The full successful journey stays identical. Diagnostics
may observe fixture responses but must not change their bytes/status or traffic allowlist.

## Completion

Stop after source edits; reviewer inspects the single file directly. No Sol repository
report. A new reviewer-pinned Hermes task will execute once with fresh metadata/artifacts,
reuse the verified runtime and retain the observed failure or successful UI result.
No Node suite reruns or owner manual test. The wider identity/transport design is separate
recorded planning; it does not authorize edits to production recipient binding here.
