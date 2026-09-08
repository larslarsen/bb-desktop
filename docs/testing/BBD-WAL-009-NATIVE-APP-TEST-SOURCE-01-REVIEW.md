# WAL-009 native App/layout test-source review

Decision: ACCEPT for one focused execution. Reviewer: Codex at XHigh.
Baseline: 4d245026c231ece436c4f203f5257c688e26733f.
Grok session e282f23a-dcf7-4c0a-9497-e3be9503c221, outer 99382, collected once
after Continue, exit 0. Authorization b6cc3c02/checkpoint c2581fec; runtime
grok-4.6-build High. No reviewer acceptance command was executed.

The new zec_native_app_tests.rs has 376 lines, SHA-256
485ce9691f873af9304e1db36de74d49627279abf9c550ccce4c74bada22a5a4.
native_ui.rs has 306 lines, SHA-256
b0d4da8770a400315889e5d69ad830e3e08d707c5bc62ca1f6e633798703349c.
Removing exactly the appended blank line/cfg(test)/module declaration reproduces
the accepted 303-line production hash c6d5fc3a4dee46f4700f2a3ae5866a5003746eab5aa6dd6bc20e5ba5c6d726b4.
All 23 other frozen source/evidence identities match. Transcript inspection shows
source-only writes, baseline/final measurements, and read-only inspection; no
test/compiler/formatter/Git execution. Reads included some broader registry/crate
searches and configuration listing; no additional correction cycle is needed.

The two layout cases use actual dialog control rectangles and real pointer input
at 520x720 and 360x480. They assert viewport containment, full-review confirmation
once, and terminal cancellation. Long values are explicit synthetic UI stress
data, not a valid transaction claim.

The two lifecycle cases call the real eframe App::ui with the pinned mock Frame,
derive click targets from clipped painted button text, and observe actual root
Close commands. They test no-input denial, one Close after confirmation, preserved
undrained approval through later close/idle frames, fresh-instance isolation, and
same-frame close-release denial. No state/approval injection or fake dialog logic
substitutes for these behaviors. The initial draft's premature consumption was
corrected before the final drop; the preservation assertion now runs after close.

Open [one-command Hermes execution](../handoff/HERMES_BBD_WAL_009_NATIVE_APP_EXPECTED_RED_01.md).
Classify all four observed outcomes; constrained-layout containment is the
anticipated red, while App lifecycle results are independently reported. All four
passing is also valid evidence and must not trigger a manufactured failure.
No repair, old-suite repetition, or integration is authorized in this execution.

The owner has not requested a new demo priority. Keep the existing wallet scope
and avoid report-only cycles or additional speculative coverage. Implementation
remains uncommitted and no usable send flow is delivered. OS/native integration,
independent recovered effects, actual-secret cleanup, and remaining security
still require acceptance. This review does not close those gates.

High is sufficient for the next bounded result review; flag a return to XHigh
before cryptographic/custody architecture work. The setting has not been changed
by the reviewer. Publication scope: this review, linked Hermes handoff,
CURRENT_TASK.md, and tickets/BBD-WAL-009.md only. No actor polling.
