# WAL-019 test-source review 01

Decision: changes required. Codex reviewed source only; no acceptance commands ran.
Do not send this drop to Hermes or start production implementation.

Desktop HEAD remains `2b5ad193f01af31f2a2bdcc7b0d1080dd5a2f68e`.
Reviewed `wallet-broker/tests/account_native_ui.rs`: 2581 lines, SHA-256
`d69962abb680ea7e9a1fee31b691928bdfc9e67e50ff1ec5c14d3e705a9110ca`.
The other six WAL-019 input hashes still match the original ticket. Existing test
lint corrections are preserved. The drop adds five `wal019_` tests and adapts six
old navigation clicks to Balance; it uses real egui pointer events and port records.

## Findings

1. **P1 — duplicate-start regression requires a hidden control.** At lines
   2417–2418, `wal019_one_job_across_frames_and_balance_navigation_starts_zero`
   calls `click_label(SYNC)` after observing a running job. `click_label` requires
   an exact painted label (`visible_label` at lines 312–329), while the existing
   progress view deliberately omits Sync when running (`account_ui.rs:937`). The
   contract permits that behavior. A correct implementation retaining this guard
   would fail the test before its duplicate-call assertion. Capture the initial
   Sync location before dispatch and exercise a genuine second pointer click
   without requiring the old control to remain painted. Keep exact start-call
   assertions and progress/job binding; do not change production to fit the test.

2. **P2 — start-error leak check is vacuous.** Lines 2508–2514 inject
   `Err("UNAVAILABLE")` but check absence of `RAW_ERROR_CANARY`. That canary never
   enters this failure path. Inject the hostile canary as the actual start error
   and assert safe visible feedback and its absence across the observed UI output.
   Retain the explicit-retry and no-automatic-start assertions. The separate
   account-list error test already injects its canary correctly.

3. **P2 — active endpoint editing is untested.** The drop checks editable idle
   endpoints and retention, but never attempts editing while a job is running.
   Contract item 4 explicitly disables it. Add a pointer/keyboard attempt against
   the real running server field and prove the endpoint remains unchanged, the
   same account/job remains active, and no additional start occurs. After explicit
   cancellation, prove editing works again and only the next explicit Sync uses
   the newly displayed endpoint. This distinguishes a disabled field from a test
   that simply never interacts with it.

## Disposition

The source review does not establish compile success, focused test counts or a
formal expected red. Those execution results were not supplied in this completion
message. The correction handoff retains Grok's authorized focused offline
format/test loop and requires exact results in the returned report. No additional
test suites or production paths are authorized.

Active next handoff:
`docs/handoff/GROK_BUILD_BBD_WAL_019_SYNC_TESTS_CORRECTION_01.md`.
Reviewer record scope: this review, that handoff, WAL-019 ticket status, desktop
CURRENT_TASK.md, payment integration status-map routing and bb-go CURRENT_TASK.md.
Unrelated work remains preserved.
