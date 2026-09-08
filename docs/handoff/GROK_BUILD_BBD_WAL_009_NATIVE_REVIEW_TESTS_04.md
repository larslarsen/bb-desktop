# WAL-009 Correction 04 — native review event tests only

Actor: Grok Build 4.6 High. Governance parent: commit containing this handoff.
Keep this small: one test module and its cfg(test) declaration, no implementation.

Read AGENTS.md, TESTING.md, CURRENT_TASK.md's leading active section, this handoff,
and the ticket's native authority/security requirements. Inspect native_ui.rs and
native.rs's ZecNativeReview/capability/confirmation definitions only. Do not reload
historical handoffs, cryptographic sources, test_support.rs, or dependency trees.
Consult only the locally pinned egui/eframe 0.36.1 API definitions needed for input.

## Exact edits and baseline

- wallet-broker/src/native_ui.rs: append only a cfg(test) module declaration for
  `zec_review_tests`; keep every existing production byte unchanged. Starting SHA-256
  `04882d21ca5e4ae21f61aa101b1f9a9610baf7eb1a736effcbb44f2b103e0735` (198 lines).
- wallet-broker/src/native_ui/zec_review_tests.rs: new private test module.

native.rs is read-only, SHA-256
`992138f18cbb0b969296109de3186aab923500c41ed2c0d41cf9c0696d3c3acc`.
All other files, including existing pending Rust source, Cargo manifest/lockfile,
uncommitted lock evidence, and unrelated package files, are frozen.

## Reviewer-fixed future contract (do not implement yet)

Use a dedicated modal native review dialog, not repeated calls polling a boolean.
One call to EframeSurface::confirm_zec_review will own one immutable review and run
a real eframe native event loop on its owning/main thread with run_and_return=true.
It returns only when the dialog closes. Unsupported thread/event-loop use or launch
failure must fail closed; no worker-thread GUI workaround, process exit, fallback
confirmation, public approval setter, or retained previous approval is allowed.
The exact accepted review must match all fields of the supplied review, not just
its claimed review_hash. The existing native adapter alone mints the capability.
The signing boundary still owns current-session/expiry/cancellation rechecks;
displaying or clicking this dialog does not itself authorize a transaction.

Tests define this private dialog API for later production authoring:

- `ZecReviewDialog::new(review: ZecNativeReview) -> Self`
- `dialog.ui(ui: &mut egui::Ui) -> ZecReviewControls`
- `ZecReviewControls` contains `confirm: egui::Rect` and `cancel: egui::Rect`;
  these are the actual rendered button response rectangles, not approval setters.
- `dialog.take_confirmed_review() -> Option<ZecNativeReview>` drains a terminal
  confirmed review exactly once; a subsequent click cannot re-arm the same dialog.
- `dialog.cancel()` terminally denies and clears any undrained approval.

The later eframe::App::ui must delegate to this SAME ui method. It renders the
existing immutable review fields and real Confirm/Cancel widgets. A viewport close
request or Escape is processed before Confirm and cancels the dialog. This includes
close+click in one frame. A programmatic close after normal confirmation must not
erase that confirmation. Denial is terminal; a new review requires a new dialog.
No review replacement API or public approval injection is introduced.

## Write six focused regression tests

Use one persistent real egui::Context across frames per dialog, `Context::run_ui`,
a fixed generous screen rectangle, deterministic time, and actual pointer move /
button press / release events in successive frames. First render and assert finite,
nonempty, on-screen button rectangles, then click their centers. Do not directly
set internal approval flags, call a fake confirm callback, or use source-text
matching as behavioral proof. The test helper may drive frames but must call the
production dialog ui method, not recreate its logic or widgets.

1. Multiple no-input frames and an outside click yield no confirmed review.
2. A real Confirm click yields exactly the original review (all fields); the second
   take and subsequent click yield None. Mutating the caller's original clone after
   construction must not change the dialog-owned review.
3. A real Cancel click denies; a later Confirm click cannot resurrect approval.
4. A root viewport close event, including close plus Confirm release in the same
   frame, denies and cannot resurrect approval.
5. Escape denies; a subsequent Confirm click cannot resurrect approval.
6. Fresh dialogs start denied/pending even after a previous dialog confirmed; an
   explicit cancel after an undrained confirmation clears it. Include two reviews
   with the same review_hash but different receiver/amount to prove full review
   identity in returned values, not hash-only equality.

Use a clearly synthetic review with explicit field values; compare field by field
or `assert!(actual == expected)` because ZecNativeReview deliberately lacks Debug.
No funds, secrets, fixtures, custody, mainnet, hardware, or Monero are involved.
These are widget/event regressions, not evidence of an OS window launch. Later
source review and a separately authorized native integration check must verify
the modal runner and native capability bridge; do not claim that this suite does.

## Execution contract — reserved to a later Hermes handoff

Targeted red/green command:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --features native-ui --lib native_ui::zec_review_tests
```

Intended initial red: missing private ZecReviewDialog/ZecReviewControls contract.
The pending old implementation also uses nonexistent egui 0.36.1 Context::run.
That or any unrelated compile/dependency failure must be separately classified;
it is NOT accepted evidence that an input regression failed. No production repair
is authorized until reviewer acceptance of test source and appropriate red evidence.
If prerequisites block execution, retain this source drop and report the blocker.

After production repair, falsify Confirm click processing by suppressing its
state transition and require test 2 to fail; falsify close-before-click precedence
and require test 4 to fail. Restore exact production source after each mutation.
Broader native regression and compile commands (also not executable in this task):

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --features native-ui --test native_surface
/home/lars/.cargo/bin/rustup run 1.98.0 cargo check --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --features native-ui
```

No dependency/build-feature/security-boundary permission changes are authorized.
The ticket's broader signing/security acceptance gates remain pending, not waived.

## Stop

Read-only inspection and apply_patch edits only. No formatter, compiler, tests,
Cargo/npm, Git, network, product launch, other actors, evidence, or integration.
Do not implement missing types to make tests compile. Do not fix other pending
errors. Report the two path hashes/line counts, six test names, and any obstacle;
stop. The reviewer will collect once when the owner reports done; no polling.
