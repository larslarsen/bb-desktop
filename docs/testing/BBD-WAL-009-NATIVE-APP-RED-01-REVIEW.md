# WAL-009 native App/layout red review

Decision: ACCEPT the observed layout red and three passing regressions. Authorize
one-file layout repair. High is sufficient; no cryptographic change is involved.
Reviewer ran no test/compiler/formatter command.

Hermes outer 86077 was collected once after Continue, exit 0. Completed session
20260908_141026_ac52d5, Hermes 0.18.2, nous/poolside/laguna-s-2.1:free.
Authorization 148ef213; execution checkpoint 11270e18d557f44d8495911bd02f6ff6384fff06.
Saved launch 78567/78568, process proc_d0674bb9920f; completion 78570 exit 101;
full exited log 78572 contains all 43 lines. Exactly one test run occurred.

Result: 3 passed, 1 failed, 12 filtered, 0 ignored; 0.10 seconds. The 360x480
long-review case fails viewport.contains_rect(rect) at zec_native_app_tests.rs:152.
The 520x720 case and both actual App callback lifecycle cases pass. This is a
real control-containment failure, not compilation, setup, or paint-helper red.
All 25 frozen file identities match reviewer measurement. New evidence:
184 lines, e749d709d8d2bd8f5e72a8509c156a2053dbed5983849964348eefbc3429733b.

Report/procedure errata are recorded without another correction-only turn:
the actual Cargo command prepended cd/&&; extra Git/version/environment/directory
probes followed execution, including an unauthorized cargo --version. The actor
did not measure the complete 25-path inventory/counts as requested, although the
reviewer independently verified it. The report lists only 23 rows, omitting
zec_sign_verify.rs and external_binding_tests.rs; it corrupts the two retained-
spend evidence hashes. The frozen inventory in the execution handoff remains
authoritative. Baseline had 11 modified and 13 untracked files, not 11/11.
Captured-output formatting was reconstructed and retains a local absolute path.
Lifecycle prose also misstates consumption order/fresh-instance coverage; actual
test source and saved results govern. No source write or test rerun occurred.
These errata must accompany eventual evidence integration; do not repeat the test.

Open [bounded layout repair](../handoff/GROK_BUILD_BBD_WAL_009_NATIVE_LAYOUT_REPAIR_01.md).
Make the immutable review body vertically scrollable within space left after
reserving both controls. Preserve all confirmation/cancellation/close semantics.
The existing ten tests suffice for this repair; no new coverage slice is opened.
After source review, group ten-test green, layout/close falsifications, and native
compilation. Prior broader native/security results remain valid where unchanged.

This does not accept OS/capability integration, independently recovered effects,
actual-secret cleanup, or remaining wallet security. Implementation and nine
evidence records remain pending and uncommitted. No usable send flow is claimed.
Publication scope: this review, linked handoff, CURRENT_TASK.md, and ticket only.
Launch once; no actor polling. Existing owner scope is unchanged.
