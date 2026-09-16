# Sol — Messages social-fixture Origin correction 01

CLOSED: exact correction accepted in [review 11](../testing/BBD-PAY-001-MESSAGES-REVIEW-11.md).
Next authority: [Hermes UI verification 04](HERMES_BBD_PAY_001_MESSAGES_UI_VERIFY_04.md).
Source instructions below are historical. Actor: Codex Sol (`gpt-5.6-sol`, High), owner-relayed.
Reviewer: Codex. Grok remains unavailable; retain the documented fallback.
Read AGENTS.md, TESTING.md, CURRENT_TASK and
[diagnostic review 10](../testing/BBD-PAY-001-MESSAGES-REVIEW-10.md).

## Sole writable source and baseline

bb-desktop HEAD `7a31c41cb29692a94acf1f24adb379f3a829d237`, empty index.
Only `test/paymentInbox.electron.js` may change. Current 1371 lines, SHA-256
`698ae487068f24c4bdaff22912268acd3fa81cd9c6f917b38f06fdf6bc1181e9`.
The exact before copy exists at `dist/pay001-ui-diagnostic03/driver-source.js`;
preserve it and all other evidence. All other 20 review-05 pins remain frozen.

No production/Node/daemon/dependency/permission changes. No test/syntax/build/formatter
execution, repository reports, Git operations or actor launches by Sol.

## Exact correction

The real configured renderer sent GET /ob/config to the owned social fixture with no
Origin header. The fixture rejected it with 403 before returning its local peer identity.
Payment transport and the later read-only probe were ready. This is captured behavioral
red, not a guessed timeout or empty-label defect.

Change only the social HTTP fixture's origin admission condition and a short explanatory
comment/local boolean if useful. Permit exactly:

- The existing literal Origin `null` case; retain its later route/method/preflight checks.
- An **absent** Origin header (`req.headers.origin === undefined`) when both
  `req.method === 'GET'` and `requiredMethod === 'GET'` for an already-listed route.

Reject every other supplied origin, including an explicitly empty header. Do not extend
absent-Origin admission to POST, OPTIONS or unknown/query-bearing routes. Keep the existing
requiredMethod check, exact method check, preflight header/method validation and all
CORS response headers unchanged. Preserve the Chromium exact-origin/path/method blocker;
this fixture server is loopback-only and does not define product access-control policy.

Do not inject an Origin header into browser requests, fabricate config responses in the
renderer, change production permissions/CSP/webSecurity, widen network destinations,
lengthen deadlines, remove assertions or bypass identity binding. No changes to fixture
payloads, request/status expectations, diagnostics, UI journey or screenshot requirements.
This should be a small localized edit. If another failure needs a broader fix, stop at
this boundary; do not preemptively accommodate hypothetical requests.

## Return

Stop after the source edit. No Sol report required; reviewer compares the file with the
retained before copy. The next reviewer-pinned Hermes assignment will run syntax and the
full sandboxed UI journey, retaining success screenshots or exact failure diagnostics.
Prior Node green/falsification stands; no Node reruns or new test framework needed for
this fixture correction. No owner manual test or user-process restart.
