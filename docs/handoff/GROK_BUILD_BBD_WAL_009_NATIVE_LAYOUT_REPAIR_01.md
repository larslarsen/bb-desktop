# WAL-009 bounded native layout repair 01

Actor: Grok Build 4.6 High, source only. Governance parent: this handoff's commit.
Fix the observed off-screen controls in one file. No new tests or wider redesign.
Read AGENTS.md, TESTING.md, CURRENT_TASK's active prefix, this handoff, and
[red review](../testing/BBD-WAL-009-NATIVE-APP-RED-01-REVIEW.md). Inspect native_ui.rs
and its two existing test modules; consult only relevant pinned egui 0.36.1
ScrollArea/button sizing APIs. No historical handoffs, cryptography, or broad searches.

ONLY writable path: wallet-broker/src/native_ui.rs, 306 lines, SHA-256
b0d4da8770a400315889e5d69ad830e3e08d707c5bc62ca1f6e633798703349c.
Verify read-only tests before editing:
- zec_native_app_tests.rs: 376 lines,
  485ce9691f873af9304e1db36de74d49627279abf9c550ccce4c74bada22a5a4.
- zec_review_tests.rs: 233 lines,
  2eec3b60eb1b9fbd32235832a867d1189928126ca79373fa9f4fa7fdf2ee87bf.
All other source, dependencies, tests, evidence, and governance are frozen.

The understood red is the small-viewport test's containment assertion. The normal
layout and two App lifecycle tests already passed. Production correction is now
authorized only within ZecReviewDialog::ui's review-body layout, plus a small
private sizing helper in this file if needed:

- Reserve vertical room for the two existing single-line buttons and intervening
  spacing using actual style/font/button-padding and minimum interaction heights.
  Derive the remaining body height from ui.available_height and clamp nonnegative.
- Render the existing immutable review helper inside a vertical egui::ScrollArea
  bounded to that remaining height, with a stable private ID. Keep every field
  available through scrolling. Avoid horizontal expansion beyond the viewport.
- Render the existing Confirm and Cancel buttons OUTSIDE the scrolling body,
  returning their real response rectangles as before. Keep their labels/order.
- Preserve close/Escape/Cancel priority, pending-only confirmation, full snapshot,
  one-shot consumption, and terminal denial byte-for-byte. Do not change the App
  wrapper, runner, viewport size, public fields, native authority, or file dialogs.
- Do not shrink fonts, truncate/omit fields, enlarge test/native windows, hide the
  controls, add approval hooks, change tests, or suppress the failing assertion.

Pinned ScrollArea provides vertical(), max_height(), id_salt(), auto_shrink(),
and show(). Use existing dependencies only. Keep the change small and follow
surrounding formatting manually; no formatter/compiler/test execution by Grok.

Reserved later Hermes validation (not executable here):
```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --features native-ui --lib native_ui::
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo check --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --features native-ui
```
Require all ten native UI tests green. Later isolated falsifications remove the
body-height constraint and re-enter dialog UI during programmatic close, requiring
the corresponding existing layout/preservation tests to fail. Reviewer will pin
exact mutation bytes/hashes after source review; restore on every outcome.

Stop after the one-file drop. No Git, evidence, scratch files, Cargo/npm, formatter,
tests, product launch, network, other actors, or integration. Report final file
hash/count and the short layout change summary. No actor polling.
