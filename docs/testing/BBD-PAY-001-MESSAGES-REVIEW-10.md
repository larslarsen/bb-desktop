# Messages UI diagnostic review 10

Reviewer: Codex. Decision: the captured failure establishes a social-fixture origin
mismatch; authorize a narrow test-fixture correction. No production repair is indicated
by this failure. The complete UI journey remains unaccepted. No tests executed by reviewer.

HEAD remains `7a31c41cb29692a94acf1f24adb379f3a829d237`; index empty. All 21 current
review-09 pins match. Retained driver-source.js also matches the 1371-line driver SHA-256
`698ae487068f24c4bdaff22912268acd3fa81cd9c6f917b38f06fdf6bc1181e9`.

## Actual failure, now identified

Read [Hermes diagnostic report](BBD-PAY-001-HERMES-UI-DIAGNOSTIC-03.md), raw stderr,
bootstrap/sandbox/failure JSON, and inspected the real failed-run screenshot.

1. Bootstrap completed and renderer sandbox checks passed again.
2. The failed stage is configured-ready-empty. The original DOM snapshot was visible,
   fully loaded, with payment/preload modules present. Online=false, notice-hidden=false,
   cards=0. The labels were Daemon offline and Payment requests unavailable for this identity.
3. The fixture received GET /ob/config without an Origin header and answered **403**.
   Its failure reason was origin-or-path. The route is explicitly allowed, leaving the
   strict `req.headers.origin !== 'null'` check as the cause of this rejection.
4. The Chromium blocker allowed that owned social request. It recorded zero unexpected
   requests; the initial default bootstrap request was blocked as intended.
5. Two payment GET /v1/payment/records requests returned **200** before diagnostics.
   The later, separately recorded preload probe returned ready, zero rows, matching
   fixture identity. This confirms payment readiness at probe time; it does not replace
   or retroactively change the original failed UI state.

Production connect() cannot bind the social identity after the fixture rejects config.
The resulting offline/identity notice explains the failed ready-empty predicate. The
fixture wrongly assumes every allowed GET from the actual local app supplies Origin:null.
Accept the observed absent-header case only for its existing GET routes; preserve all
other method/origin/path restrictions. No CORS wildcard or production permission change.

Hermes again copied the incorrect No conversations diagnosis and old line numbers.
The preceding No conversations wait passed; the screenshot visibly contains that text.
The verdict is corrected here. No extra recovery/report-rewrite relay is requested.

## Evidence identities and limits

| Retained artifact | SHA-256 |
| --- | --- |
| [Failure diagnostics](../../dist/pay001-ui-diagnostic03/ui-01/ui-failure-diagnostics.json) | 77401104afd7c4642630c42489e4b24054de84dece6c2ba8508700b748c63dee |
| [Failure screenshot](../../dist/pay001-ui-diagnostic03/ui-01/fixture-failure.png) | eb78d8edb4a970f5e0660f51a733466377064089b0e5b21cd3b9d241c18e8bda |
| [UI stderr](../../dist/pay001-ui-diagnostic03/ui.stderr.log) | 7de8d52a35be84a73226d6472c0dd9121b8816462319066b224f2a998a1e99fd |
| [Bootstrap](../../dist/pay001-ui-diagnostic03/ui-01/bootstrap-diagnostics.json) | b098282b78dc70f2b65c5bddb85f5c5aae0b5b9ad470a96d5a91eeade0779b74 |
| [Sandbox](../../dist/pay001-ui-diagnostic03/ui-01/sandbox-status.json) | baec4b3601efe621d82e48016fb53b51d4aa08df98660fb7bdd86bb52e10d515 |
| [Source before](../../dist/pay001-ui-diagnostic03/source-before.json) | 61222de9143213e5a95463ff22726c992d289efc370a36aaf016899cb1aed269 |

PNG dimensions independently read: 1180 x 714. It is a fixture failure screenshot,
showing the offline social/empty Messages state, not the requested populated success UI.

The 21 before-manifest hashes match the current files and expected pins. Its line counts
include a trailing empty split entry (one greater than physical-line counts), not byte
drift. Runtime-before.json is a count/comparison summary rather than the required per-path
manifest. Syntax metadata exists but omits the timeout wrapper from argv and log hashes;
its timestamps have no retained capture-driver provenance. UI metadata and after source/
runtime manifests are absent. Exit status remains actor-reported. Do not claim complete
execution metadata or reconstruct missing facts. Those deficiencies do not negate the
specific driver-generated failure/fixture observations used for this source decision.

## Next authority

[Sol social-fixture Origin correction 01](../handoff/SOL_BBD_PAY_001_MESSAGES_UI_ORIGIN_01.md)
authorizes only the observed GET Origin mismatch in the Electron driver. Preserve the
1371-line driver as a before snapshot via the retained driver-source.js. No source, Node,
permissions, timeouts, assertion or wider protocol change. Existing runtime failure is
the red evidence; no new speculative diagnostic feature or repeated Node suite.

After source review, Hermes runs the full unchanged UI journey with new artifacts.
UI acceptance/scans/publication remain pending. Identity/transport/video planning remains
in force and is separate from this fixture correction. No manual owner test requested.

Governance paths: this review, new Sol handoff, closed Hermes diagnostic-03 handoff,
CURRENT_TASK, PAY-001 ticket and end-to-end status map.
