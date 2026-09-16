# BBD-PAY-001 UI finish review 04

Reviewer: Codex. Date: 2026-09-15.
Decision: harness correction source accepted for execution; actual UI validation pending.
Input: [UI finish report](BBD-PAY-001-GROK-UI-FINISH-01.md).

## Verified evidence

Final harness SHA-256 `89d82e8ad73da7d08e3f03a1e9d953521a75a4f55803d96f1265f87bd8c8ce54`
and 580 lines match. Stylesheet and the other eighteen correction-02 source/input
identities remain unchanged. Preflight hash matches. All three command metadata records
match their stdout/stderr hashes; the report supplies hashes for the smoke and final
syntax metadata, and a directory pointer for the intermediate syntax check.

Preflight records a verified installed root-owned Chrome helper. The actual smoke
still selected Electron's adjacent unprivileged helper and exited 1/SIGTRAP. No UI or
screenshot result exists. Final harness syntax exited 0. Inspection confirms the
collapsed-details exception is narrow and each row is also checked expanded through
keyboard input. This is source acceptance for execution, not a visual pass.

Transport and Node source acceptance from review 03 stands. No new payment source
correction is authorized or needed for this execution step. Reviewer ran no tests.

## Revised runtime decision

The previous environment override did not select the installed helper while Electron's
adjacent helper existed. Do not change the checkout helper's ownership or permissions.
Prepare a separate, ignored copy of the existing Electron runtime that omits only its
unconfigured chrome-sandbox file; use the already installed verified helper through
CHROME_DEVEL_SANDBOX. Keep every other copied runtime byte/mode unchanged, verify a
complete manifest, and run the existing frozen fixture harness there. This is a test
runtime, not a product package or security waiver. No host security policy or installed
file changes. Actual sandbox proof and complete UI assertions still govern success.

Hermes owns this bounded runtime staging/capture:
[UI runtime handoff](../handoff/HERMES_BBD_PAY_001_UI_RUNTIME_01.md).
No implementation edits, broader acceptance or Git actions are authorized by it.

## Owner usability check

The owner volunteered to test. A supplementary manual checklist is recorded at
[owner UI check](BBD-PAY-001-OWNER-UI-CHECK-01.md). It uses the owner's normal app
launch, without requesting security-setting changes or test evidence pasted into chat.
The owner may record observations directly in that document. This does not replace
sandboxed fixture coverage or Hermes acceptance and does not authorize spending.

Reviewer governance paths: this review, the owner checklist, Hermes runtime handoff,
CURRENT_TASK active prefix, PAY-001 ticket status and integration status map. All
authorizations live in these repository documents under the existing control plane.
