# WAL-019 corrected test-source review 02

Decision: test source accepted for formal expected-red execution. Production
implementation and final runtime acceptance remain unauthorized.

Reviewer: Codex. HEAD: `2b5ad193f01af31f2a2bdcc7b0d1080dd5a2f68e`.
Accepted `wallet-broker/tests/account_native_ui.rs`: 2654 lines, SHA-256
`e7ac08d26fad3eb0302c49b8983b62cb28a6aab5bad657b142c2e84d59a11446`.
The six other WAL-019 production/build pins match the original ticket. Existing
unpublished source and test lint changes remain preserved.

## Correction review

- The duplicate-start test captures the actual list Sync label geometry before
  starting, dispatches pointer events at that position, then replays that position
  without requiring a Sync label in the running view. It checks exact start calls
  and observed progress through additional frames.
- The start failure now injects `RAW_ERROR_CANARY` into the actual fake port result.
  Immediate and subsequent observations check safe visible feedback and canary
  absence; the exact start-call list proves retry requires another user action.
- The new running-endpoint test attempts pointer focus, select-all and text input
  during job 61. It requires the original endpoint and progress to remain visible,
  exactly one start, and cancellation bound to that account/job. It then repeats
  the editing gesture after cancellation and checks the next explicit start uses
  the changed endpoint. This positive control makes the editing attempt meaningful.

All six `wal019_` tests use the real AccountWindow/egui event path with fake network
work. The existing navigation tests retain their independent assertions with the
authorized Balance label adaptation. No production mechanism or global observation
helper was changed to conceal a failure. The prior three findings are resolved.

## Execution boundary

This is source acceptance, not a claim that tests have run or passed. Codex did not
execute Rust commands, and the owner's completion message contained no command
transcript. Hermes must record formatter success, actual test execution and the
intended runtime red on these frozen bytes. Compiler errors, missing dependencies,
zero selected tests and timeout cannot establish that red.

Active handoff: `docs/handoff/HERMES_BBD_WAL_019_SYNC_EXPECTED_RED_01.md`.
No source integration is necessary: the accepted test drop is already in the
working tree. Hermes may capture evidence only. After review of that evidence,
Codex can authorize the one-path Grok production task.

Reviewer governance paths: this review, the Hermes handoff, WAL-019 ticket status,
desktop CURRENT_TASK.md, payment status-map routing and bb-go CURRENT_TASK.md.
