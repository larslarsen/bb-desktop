# Messages UI bootstrap source review 07

Reviewer: Codex. Decision: accept the bounded driver correction for UI execution.
No syntax/test/build/Electron commands executed by reviewer. HEAD remains
`7a31c41cb29692a94acf1f24adb379f3a829d237`; index empty.

## Reviewed change

Only the Electron driver differs from the 21 review-05 source/input pins:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| test/paymentInbox.electron.js | 1110 | ead9f500b8eaafbc19d30961e8327b992437838b09d033e3edacf0d4f79192e1 |

All other 20 identities still match [review 05](BBD-PAY-001-MESSAGES-REVIEW-05.md).

The driver now verifies the actual local application URL, complete document and required
DOM elements before setting the two fixture bootstrap values. It awaits main-process
loadFile of that exact application file under the existing 15-second deadline, then
rechecks document identity. The renderer reload and dangling load listeners are gone.
Production main, navigation guards, preload, renderer and privileges are unchanged.

Diagnostics retain at most 32 lifecycle events, setup phase/timing, load failure and
renderer-termination facts, plus current loading/destroyed/document-match state. The
observer removes only its own listeners. Success/failure writes bootstrap-diagnostics.json;
diagnostic failure preserves the original test failure. A bootstrap success describes
setup only: sandbox and complete UI assertions still must pass afterward.

Read-only reverse audit: removing precisely the new import/constants/state, diagnostics
helpers/cleanup, bootstrap replacement and catch diagnostics from an in-memory string,
then reinstating the previously reviewed bootstrap, reproduces the entire prior
903-line driver SHA-256 `d884456fcef8955ca5835da2ae13e6864e334bf4ef6305742385386e478e58bf`.
No file was written or driver executed for this audit. Therefore the existing fixture,
network, sandbox, UI interaction/geometry and four screenshot assertions are byte-for-byte
preserved outside the reviewed setup change.

The old isolated runtime was independently compared again: 74 original entries, 73
copied entries; only chrome-sandbox omitted; every included type/mode/file size/hash
matches now. This permits reuse after Hermes saves and verifies current preflight and
before/after manifests. It is not proof of historical metadata or sandbox success.

## Next authority and limits

[Hermes Messages UI verification 02](../handoff/HERMES_BBD_PAY_001_MESSAGES_UI_VERIFY_02.md)
authorizes one syntax check and the corrected sandboxed UI run, with retained command
metadata, diagnostics and screenshots. Sol bootstrap correction 01 is closed.
The prior Node results/falsification stand under [review 06](BBD-PAY-001-MESSAGES-REVIEW-06.md);
no unchanged Node suite rerun is requested. UI success, security scans and publication
remain pending. No manual owner test, product edit or user-process restart.

Reviewer governance paths: this review, new Hermes UI handoff, closed Sol bootstrap
handoff, CURRENT_TASK, tickets/BBD-PAY-001.md and end-to-end status map.
