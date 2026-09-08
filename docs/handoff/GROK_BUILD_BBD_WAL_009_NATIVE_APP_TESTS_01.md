# WAL-009 native layout and App lifecycle tests 01

Actor: Grok Build 4.6 High, test-source author only.
Governance parent: the commit containing this handoff.
Add four focused regressions at the existing native dialog/App boundary. No
production repair or test execution. Keep this a bounded two-path source drop.

Read AGENTS.md, TESTING.md, CURRENT_TASK.md's active prefix, this handoff, and
[validation acceptance](../testing/BBD-WAL-009-NATIVE-REVIEW-VALIDATION-01-ACCEPTANCE.md).
Inspect native_ui.rs and its existing zec_review_tests.rs as read-only examples;
read only native.rs's ZecNativeReview definitions as needed. No historical
handoffs, transcripts, cryptographic sources, fixtures, or broad dependency reads.
Consult only relevant pinned egui/eframe/epaint 0.36.1 API definitions.

## Authorized paths and baseline

- wallet-broker/src/native_ui/zec_native_app_tests.rs: new private test module.
- wallet-broker/src/native_ui.rs: append ONLY a blank line, #[cfg(test)], and
  mod zec_native_app_tests; after the existing module declaration. Preserve every
  current byte. Starting 303 lines, SHA-256
  c6d5fc3a4dee46f4700f2a3ae5866a5003746eab5aa6dd6bc20e5ba5c6d726b4.

Read-only baselines: existing zec_review_tests.rs, 233 lines,
2eec3b60eb1b9fbd32235832a867d1189928126ca79373fa9f4fa7fdf2ee87bf;
native.rs, 489 lines,
992138f18cbb0b969296109de3186aab923500c41ed2c0d41cf9c0696d3c3acc.
Verify these identities and new-path absence before editing. All other source,
tests, dependencies, package files, locks, eight evidence records, and governance
are frozen. No new production types, API, instrumentation, hooks, or visibility.

## Fixed semantics and test design

Confirmation buttons must remain usable within the actual available viewport
even when immutable review fields wrap. A smaller available window cannot hide
Confirm/Cancel below a non-scrollable review. Any later repair will keep the
review body scrollable within bounded remaining space and controls visible,
preserving the full immutable review and all existing confirmation semantics.
Do not implement that repair now, shorten review fields, or enlarge the test
window just to make a regression pass.

Use a clearly synthetic long review: receiver is an explicit synthetic prefix
plus 512 ASCII characters; handle/session/request/intent/review/memo identifiers
use fixed 64-character public test values; account is 32 hex characters. Other
fields use fixed small amounts, normal pool labels, and an RFC3339 expiry. These
are UI stress values, not claims of a valid spend/address or real private data.
Use full equality without Debug, and a shorter synthetic review for App tests.

Write exactly these four tests:

1. long_review_at_native_size_keeps_controls_usable: persistent egui Context at
   520x720; call the real dialog.ui inside the same CentralPanel arrangement as
   production. Assert actual returned button rectangles are finite, positive,
   and wholly inside the viewport. Drive pointer move/press/release at the real
   Confirm center and require exactly the original full review once. A fresh
   dialog's Cancel must deny and block later Confirm. Do not bypass clipping or
   directly set the dialog state.
2. long_review_at_small_viewport_keeps_controls_usable: the same assertions at
   360x480. This covers constrained available space even though the native window
   requests a larger initial size. Do not assume either layout case already
   fails: execution must classify the observed result without manufacturing red.
3. native_app_confirm_closes_once_and_preserves_owned_review: instantiate the
   REAL private ZecReviewNativeApp with a fresh Rc<RefCell<ZecReviewDialog>> and
   closing=false. Drive its actual eframe::App::ui through Context::run_ui and
   Frame::_new_kittest on successive frames at 1280x800. No-input frames must
   yield no approval and no Close command. A real Confirm click must emit one
   root ViewportCommand::Close. Feed a subsequent close-request frame, then an
   idle frame; no repeated Close command may be emitted, and the undrained exact
   review must survive until taken once. A fresh App/dialog must start pending.
4. native_app_close_release_denies_and_closes_once: drive pointer move, press,
   then Confirm release WITH a root close event through the actual App callback.
   It must deny, emit Close once, and never approve or repeat the Close command
   on following close/idle/click frames. Do not set closing/state after creation.

Keep helpers local to the new module. Use deterministic time and actual egui
PointerMoved/PointerButton and root ViewportEvent::Close input, not fake callbacks
or source-string matching. Observe outcomes by take_confirmed_review and real
FullOutput.viewport_output root commands; do not assert private enum values as
a substitute for behavior. Clear texture deltas with drop_without_applying_deltas
after extracting observations on every frame.

The App returns no control rectangles. Locate its actual rendered button text
in FullOutput.shapes (Shape::Text, recursively handling Shape::Vec as needed),
intersect visual_bounding_rect with its clip rectangle, require a finite positive
visible rectangle, and click its center. Confirm also appears in the heading:
select the last painted exact-label match, the actual button below the review,
and independently check Cancel is a distinct visible target. Never hardcode pixel
coordinates or render a second fake dialog to infer the App's button position.
This uses actual native App rendering/events without adding production hooks.

Pinned APIs already inspected: eframe 0.36.1 Frame::_new_kittest is a public,
doc-hidden constructor with no real OS handle. App::ui receives &mut egui::Ui
and &mut Frame. FullOutput exposes shapes and viewport_output; root ViewportOutput
has commands. TextShape exposes galley.text() and visual_bounding_rect(). These
are callback/layout tests, not an OS window or owning-thread integration gate.

## Execution reserved to later Hermes authorization

After source review Hermes will run the exact new-module test command once:

```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --features native-ui --lib native_ui::zec_native_app_tests
```

Expected diagnostic to investigate: the constrained layout returns button
rectangles below the visible viewport. App lifecycle tests should pass existing
logic; a failure there is a separate finding. Compilation failure is not a
behavioral red. If all four pass, report that result and do not weaken tests to
force failure. The reviewer decides whether any production correction is needed.

After correction, grouped green must include all existing six dialog tests and
these four tests. Falsifications will remove the bounded review layout to prove
clipping detection and re-enter dialog UI during programmatic close to prove
the App confirmation-preservation assertion fails. Exact mutations and restored
hashes will be frozen in a later Hermes handoff. No execution in this task.

## Stop

Only bounded read-only inspection, initial/final file hashes/counts, and edits to
the two exact paths. No compiler, formatter, tests, Cargo/npm, product/OS launch,
Git, dependency changes, evidence, scratch files, network, or other actors.
No production repairs. Report both final identities, four test names, and any
obstacle concisely, then stop. Reviewer collects once after done; no actor polling.
