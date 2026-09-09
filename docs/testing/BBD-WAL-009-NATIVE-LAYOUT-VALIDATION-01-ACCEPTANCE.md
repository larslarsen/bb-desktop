# WAL-009 native layout validation acceptance

Decision: ACCEPT the native widget/layout validation outcomes and restoration.
Do not claim full execution-protocol compliance or a complete raw transcript in
Hermes's evidence. No test rerun or evidence-only correction task is warranted.

Authorization: 504b4953; original launch: 0e60e76e. Capacity continuation:
ad6c325b; launch checkpoint: 6de87b53. Outer 10169 previously completed after
an upstream HTTP 429; outer 14430 now collected once after owner done, exit 0.
Original Hermes session: 20260908_145855_f82684. Resumed runtime session:
20260908_150421_ce19a4, provider nous, model poolside/laguna-s-2.1:free.
No actor remains active or authorized. No actor polling or reviewer test execution.

## Accepted execution

Saved message IDs below identify the reviewed actor transcript. The first stage
is retained from the original session; it was not repeated during continuation.

| Stage | Command / launch / completion IDs | Accepted result |
| --- | --- | --- |
| Initial native UI group | 78608 / 78610 / 78612 | Exit 0; 10 passed, 6 filtered; 0.15 s |
| Unbounded review body | 78639 / 78640 / 78642 | Exit 101; 1 failed, 15 filtered; viewport containment assertion at zec_native_app_tests.rs:152 |
| Close re-entry | 78653 / 78654 / 78656 | Exit 101; 1 failed, 15 filtered; full review equality assertion at zec_native_app_tests.rs:333 |
| Restored native UI group | 78663 / 78664 / 78666 | Exit 0; 10 passed, 6 filtered; 0.19 s |
| Native compile | 78671 / 78672 / 78674 | Exit 0; three dead-code warnings; no warning-denied claim |

The exact commands are defined in the
[closed validation handoff](../handoff/HERMES_BBD_WAL_009_NATIVE_LAYOUT_VALIDATION_01.md).
Both deliberate faults reached the intended behavioral assertions. The restored
ten-test group covers the six dialog tests plus long reviews at 520x720 and
360x480, application confirmation/close behavior, and denial on close release.
These tests exercise egui and App::ui; they do not prove a real OS window,
owning-thread behavior, or end-to-end capability integration.

## Restoration and identities

The unbounded-body mutation measured at 78634 was 321 lines, SHA-256
cf18eb0a012fb485ec091f6a142a34be479b6368516e51e7c73de59e1c63a366.
Restoration was recorded at 78643/78646. The close re-entry mutation measured at
78652 was 321 lines, SHA-256
3788efe82a143601e7197f2769180c2972002ca4ea090b11056c7ef01db1a9bc.
Restoration was recorded at 78657/78660.

Reviewer independently compared all 26 frozen source/evidence identities with
the handoff inventory: all match. Final wallet-broker/src/native_ui.rs is 321
lines, SHA-256 d132a164a6413165291abd0582abf3eca4cacd3f33ce6eb4ac2b467e8774d960.
No residual fault, source repair, dependency change, or integration occurred.

New pending actor evidence:
[BBD-WAL-009-NATIVE-LAYOUT-VALIDATION-01.md](BBD-WAL-009-NATIVE-LAYOUT-VALIDATION-01.md),
354 lines, SHA-256 ffcfc14837ff02aba33a5a24fa6a58d610055b18b4efd150787c6b8acba7dc43.
The original full handoff inventory and reviewer comparison govern identities;
the actor report's shortened inventory is not authoritative.

## Execution and report exceptions

Before the accepted stage-2 invocation, 78635 omitted the separator before
--exact. Completion 78638 exited 1 with a Cargo argument error; no test ran.
Hermes corrected and retried at 78639 without the required stop/restoration.
This was a procedural violation. The later intended failure and restoration
remain independently supported; the failed invocation is not a falsification.

Hermes made no process.log call during continuation. Stage-2 completion 78642
is capped at 2000 characters and lacks its beginning, but retains the test name,
intended panic, counts, and exit code. The evidence's reconstructed prefix is
not accepted as raw output. Stage 1 is summarized rather than reproduced in
that report; its original saved completion remains accepted. Other completed
stage outputs retain the necessary results.

Hermes repeated prohibited preflight/version/Git/transcript discovery and extra
read-only measurements. Its claims of no such repeats and of personally
measuring all 26 before/after identities are inaccurate. The report also omits
the new runtime session ID and initial malformed invocation, contains inventory
length/hash-prefix errors and an incorrect upstream version hash, changes some
output formatting, and gives inaccurate pending-file counts. Original accepted
version output identified upstream fef0e16f and local 10b6d1a9. Local absolute
paths in pending evidence require normalization during eventual integration.
No Git mutation or additional test run followed the required final compile.
These exceptions are recorded here without another correction-only actor cycle.

## Next boundary

High was sufficient for this review. Pause for the owner's requested switch to
XHigh before defining the independent transaction-effects verification repair.
The current verifier compares prepared inspection metadata with a clone instead
of recovering all authoritative effects from the decoded signed artifact.
Actual-secret cleanup, native OS/capability integration, and remaining security
also remain open. Previously accepted gates remain valid; no speculative UI
coverage or old expensive suite repeat is authorized.

Implementation and ten actor evidence records remain uncommitted. This is a
bounded validation acceptance, not broader WAL-009 acceptance or a delivered
wallet send flow. Network/broadcast/mainnet/hardware/Monero remain parked.

Reviewer publication scope: this review, docs/handoff/CURRENT_TASK.md, and
tickets/BBD-WAL-009.md only. No source, execution, evidence, or integration actor
is authorized by this acceptance.
