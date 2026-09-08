# WAL-009 native review test-source review 04

Result: ACCEPTED for the bounded initial-red execution only. No behavioral,
native-window, signing, or Phase-A3 acceptance is implied.

Reviewed at governance HEAD `d7ca3f1fd51f1529a74f80a816f65b4eff155656`, against
the [Correction-04 contract](../handoff/GROK_BUILD_BBD_WAL_009_NATIVE_REVIEW_TESTS_04.md)
launched from `33de1fb2`. The owner confirmed completion. Outer terminal session
27099 was unavailable after context reset; the saved Grok session
`01a07f2e-f45a-78e2-8f05-7526ce1f62dd` supplied the completion report and tool
record. Its final report identifies `grok-4.6-build`, High.

## Reviewed identities

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/native_ui.rs` | 201 | `600e2fb34134a276d31da9c5e746cc6478337bae73590fd742cf2d36ff785524` |
| `wallet-broker/src/native_ui/zec_review_tests.rs` | 233 | `2eec3b60eb1b9fbd32235832a867d1189928126ca79373fa9f4fa7fdf2ee87bf` |
| `wallet-broker/src/native.rs` (read-only) | 489 | `992138f18cbb0b969296109de3186aab923500c41ed2c0d41cf9c0696d3c3acc` |

The two drop hashes and line counts match the saved completion report. Removing
only the appended blank line and private cfg(test) declaration from native_ui.rs
recovers the exact 198-line production hash
`04882d21ca5e4ae21f61aa101b1f9a9610baf7eb1a736effcbb44f2b103e0735`.

## Source findings

No blocking source defect found in the six required regressions:

1. `no_input_frames_and_outside_click_yield_no_confirmed_review`
2. `confirm_click_returns_exact_owned_review_and_cannot_rearm`
3. `cancel_click_denies_and_blocks_later_confirm`
4. `viewport_close_denies_including_same_frame_confirm_release`
5. `escape_denies_and_blocks_later_confirm`
6. `fresh_dialog_is_independent_and_cancel_clears_undrained_confirm`

The helper retains one real egui Context per dialog, supplies deterministic time
and screen geometry, and delegates every frame to the future production ui method.
It checks finite, positive, on-screen button rectangles before targeting them with
pointer move, press, and release frames. It does not construct replacement widgets
or inject confirmation state. Equality compares all ZecNativeReview fields, including
two distinct reviews sharing a hash. Cancel, Escape, close, close concurrent with
Confirm release, single consumption, and fresh-dialog independence have explicit
assertions. Explicit cancel also clears an undrained confirmation.

The locally pinned egui/emath 0.36.1 definitions support run_ui, output-delta disposal,
the constant rectangle constructor, and the input types used here. RawInput::default
includes the root viewport, so close-event insertion reaches that viewport. These
are static API checks, not compile or runtime evidence.

The saved tool record contains no test/compiler/formatter, Git, network, product
launch, or other-actor execution. It does record procedural deviations: write and
search_replace were used instead of the named apply_patch tool; read-only searches
expanded beyond the requested files, including lib.rs and the native-ui manifest
feature. These do not invalidate the inspected two-path source drop, but the run
is not recorded as exact procedural compliance. No whole-worktree before/after
identity claim is made for unrelated pending files.

## Next boundary

Hermes alone may execute the [initial-red handoff](../handoff/HERMES_BBD_WAL_009_NATIVE_REVIEW_EXPECTED_RED_01.md).
Missing ZecReviewDialog/ZecReviewControls is the intended absent-contract diagnostic.
The existing production Context::run call is a separately known incompatible API;
that diagnostic or any other prerequisite failure cannot prove an input regression.
No source repair is authorized until the resulting evidence is reviewed.

The suite cannot prove an OS window launch, owning-thread modal lifetime, the
programmatic-close distinction, or the adapter's capability minting. Those require
the later production review and separately authorized native integration gate.
Confirm-transition and close-precedence falsification remain required after repair.
Actual-secret cleanup and independent transaction-effect verification remain A3
blockers. Pending source, lockfile/evidence, package changes, and WAL-007 are preserved.

Reviewer publication scope is exactly this review, the new Hermes handoff,
docs/handoff/CURRENT_TASK.md, and tickets/BBD-WAL-009.md. Developer source and
implementation evidence are excluded from this governance commit.
