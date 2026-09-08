# WAL-009 native review production 01

Actor: Grok Build 4.6 High, source author only.
Governance parent: the commit containing this handoff.
Implement the already tested private dialog contract and synchronous native runner
in one file. No tests, execution, evidence, Git, or integration in this task.

Read AGENTS.md, TESTING.md, CURRENT_TASK.md's active prefix, this handoff, the
ticket's native authority requirements, and the linked
[review](../testing/BBD-WAL-009-RETAINED-SPEND-ACCEPTANCE-AND-NATIVE-REVIEW.md).
Inspect native_ui.rs, its private zec_review_tests.rs module, and only the relevant
native.rs review/surface/capability definitions. Do not reload historical handoffs,
cryptographic sources, test_support.rs, transcripts, or dependency trees.
Consult only locally pinned egui/eframe 0.36.1 API definitions as needed.

## Exact scope and baselines

The ONLY writable path is wallet-broker/src/native_ui.rs, starting at 201 lines,
SHA-256 600e2fb34134a276d31da9c5e746cc6478337bae73590fd742cf2d36ff785524.

Verify that and these two read-only identities before editing:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| wallet-broker/src/native_ui/zec_review_tests.rs | 233 | 2eec3b60eb1b9fbd32235832a867d1189928126ca79373fa9f4fa7fdf2ee87bf |
| wallet-broker/src/native.rs | 489 | 992138f18cbb0b969296109de3186aab923500c41ed2c0d41cf9c0696d3c3acc |

All other pending source, private tests, manifests, package files, locks, evidence,
and governance are frozen. No dependency/feature/API permission change is needed.
Within native_ui.rs preserve BrokerNativeApp, NativeSurfacePort password/restore/
result behavior, RfdDialog, the immutable review renderer, and the cfg(test)
module declaration. Add private dialog/state/native-app types and needed imports;
replace only the Zcash confirmation implementation and its two stale private
approval fields. Keep the public restore confirmation field's existing semantics.

## Test-first status and fixed security contract

Six real-widget tests were authored before this implementation. The saved native
execution is a classified compile red: missing ZecReviewDialog/ZecReviewControls
and nonexistent Context::run, with now-repaired independent prerequisites.
No widget assertion is claimed to have run. Do not edit tests or execute anything
to establish green here. Their exact production API and event semantics are fixed.

One EframeSurface::confirm_zec_review call owns one immutable full review and runs
a real synchronous eframe native event loop on its owning/main thread with
run_and_return explicitly true. Return only after that window loop returns.
Only the same call's successful user confirmation of that full review may yield
true. No automatic confirmation, public approval setter, hash-only identity,
retained approval, synthetic RawInput loop, renderer authority, or worker GUI.
The existing native adapter alone mints the capability; signing still performs
current-session, ownership, cancellation, and expiry rechecks. Do not change it.

Use the following private, single-thread ownership design:

1. ZecReviewDialog owns a ZecNativeReview snapshot and a private terminal-state
   enum with Pending, Confirmed, Consumed, Denied states. No replacement API,
   public setters, Debug derivation, logging, serialization, or external handles.
2. new(review) starts Pending. take_confirmed_review returns the exact full owned
   snapshot once (a clone is permitted), transitions Confirmed to Consumed, and
   returns None otherwise. Neither Consumed nor Denied can confirm again.
   cancel always sets Denied, including after an undrained confirmation.
3. ui(&mut egui::Ui) -> ZecReviewControls renders the existing full immutable
   review and real Confirm Zcash send / Cancel buttons. Controls contain the
   actual response rectangles as private confirm/cancel fields. Keep widgets
   present across terminal frames so the existing event tests can click them;
   terminal state prevents rearming.
4. Process root viewport close_requested or Escape BEFORE accepting Confirm.
   A simultaneous close plus Confirm release must deny. Cancel also wins over
   Confirm. Only Pending can transition to Confirmed. No-input frames or outside
   clicks cannot confirm. Tests and production must call this SAME ui method.
5. A private eframe::App wrapper holds the dialog in a fresh per-call
   Rc<RefCell<ZecReviewDialog>> and delegates App::ui to dialog.ui. After a
   terminal decision it requests ViewportCommand::Close once. Use a private
   closing flag so later frames caused by this programmatic close cannot cancel
   the normal completed confirmation. The initial frame must still process all
   close/Escape/cancel input before its confirmation decision.
6. The synchronous runner retains only that call's Rc, uses eframe::run_native
   with NativeOptions { run_and_return: true, .. } and a bounded review window,
   and drains the result only after successful return. Compare the accepted
   ZecNativeReview to the supplied review using full PartialEq, never just hash.
   Pending, denied, consumed, or mismatched results yield false.
7. Unsupported thread/event-loop use, launch errors, or unwind panics fail closed
   with NativeError::locked and clear/discard the call's decision. A narrow
   catch_unwind(AssertUnwindSafe(...)) around the native runner is permitted for
   default winit affinity/event-loop panics. Do not spawn a GUI thread, enable
   any-thread platform overrides, use unsafe code, process exit, or fall back to
   a boolean. Thread names are not proof of OS main-thread affinity. Use the
   default platform event-loop restrictions and handle failure without approval.

The dialog must return its own complete snapshot even when two inputs share a
claimed review_hash but differ in receiver/amount. No borrowed mutable caller
review or previous surface state may influence a later call.

## Pinned API notes

eframe 0.36.1 App::ui receives &mut egui::Ui and &mut eframe::Frame.
AppCreator is a boxed FnOnce returning Result<Box<dyn App>, DynError>; the App
need not be Send. NativeOptions::run_and_return defaults true but set it explicitly.
egui 0.36.1 provides Context::run_ui (tests), InputState::viewport().close_requested(),
InputState::key_pressed(Key::Escape), and Context::send_viewport_cmd(Close).
CentralPanel::show receives the root Ui. Production must use run_native, not an
off-screen run_ui substitute. Existing native-ui/glow/wayland/x11 features suffice.

## Reserved validation, not authorized execution

After source review Hermes will run the targeted green, two isolated mutations
with exact restoration, the broader native surface suite, and native compilation.
Do not run these commands in this source-authoring task:

```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --features native-ui --lib native_ui::zec_review_tests
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --features native-ui --test native_surface
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo check --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --features native-ui
```

Falsify the actual Confirm transition so the confirmation test fails; separately
falsify close-before-Confirm precedence so the simultaneous-close test fails.
The reviewer will freeze exact patches/hashes after inspecting the final source.
These widget tests do not prove OS window launch, platform lifecycle, owning-thread
integration, or the native capability bridge. Those require a separate integration
gate. No source-level facade or visual claim may substitute for that evidence.
Full recovered effects, actual-secret cleanup, broader security, real hardware,
network, broadcast, mainnet, and Monero stay outside this slice.

## Stop

Only bounded read-only inspection, initial/final hashes and line counts, and
source edits to the single authorized path. No compiler, formatter, tests,
Cargo/npm, product launch, shell configuration, scratch artifacts, Git, network,
other actors, evidence, or integration. If a frozen prerequisite blocks this
contract, report it without repairing it. Report changed path, final hash/count,
the state/runner implementation, and any obstacle in a short final reply; stop.
The reviewer collects once after done/explicit collection. Do not poll actors.
