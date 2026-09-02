# Current Task

Ticket: BBD-WAL-007

State: PHASE C SLICE 3 RPC/NODE SOURCE AUTHORIZED

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Authorized source actor: Principal Dev — Codex Sol (`gpt-5.6-sol`, High)

Authorized integration actor: none

Protected governance parent: the commit containing this task update

Ticket: [BBD-WAL-007.md](../../tickets/BBD-WAL-007.md)

Owner decision:
[BBD-WAL-007-XMR-RPC-DISTRIBUTION-DECISION.md](../architecture/BBD-WAL-007-XMR-RPC-DISTRIBUTION-DECISION.md)

Active handoff:
[CODEX_SOL_BBD_WAL_007_PHASE_C_SLICE_03_RPC.md](CODEX_SOL_BBD_WAL_007_PHASE_C_SLICE_03_RPC.md)

Slice 1 is complete and accepted at `c139641a` in
[BBD-WAL-007-SLICE-01-ACCEPTANCE-01.md](../testing/BBD-WAL-007-SLICE-01-ACCEPTANCE-01.md).
Sol stopped Slice 2 before editing because stable safe Rust cannot signal a process
group. The XHigh decision is
[BBD-WAL-007-SLICE-02-OWNED-CHILD-DECISION.md](../architecture/BBD-WAL-007-SLICE-02-OWNED-CHILD-DECISION.md).
The exact one-path correction is accepted in
[BBD-WAL-007-SLICE-02-OWNED-CHILD-TEST-SOURCE-REVIEW-01.md](../testing/BBD-WAL-007-SLICE-02-OWNED-CHILD-TEST-SOURCE-REVIEW-01.md).
The corrected expected red is accepted in
[BBD-WAL-007-SLICE-02-EXPECTED-RED-ACCEPTANCE-01.md](../testing/BBD-WAL-007-SLICE-02-EXPECTED-RED-ACCEPTANCE-01.md).
The initial four-path process source drop was rejected at XHigh in
[BBD-WAL-007-SLICE-02-SOURCE-REVIEW-01.md](../testing/BBD-WAL-007-SLICE-02-SOURCE-REVIEW-01.md).
The impossible atomic-listener handoff clause is replaced by
[BBD-WAL-007-SLICE-02-PORT-PREFLIGHT-DECISION.md](../architecture/BBD-WAL-007-SLICE-02-PORT-PREFLIGHT-DECISION.md).
The corrected five-path drop is accepted in
[BBD-WAL-007-SLICE-02-SOURCE-REVIEW-02.md](../testing/BBD-WAL-007-SLICE-02-SOURCE-REVIEW-02.md).
Hermes stopped before execution because the formatter check failed without mutation;
that stop is reviewed in
[BBD-WAL-007-SLICE-02-GREEN-STOP-REVIEW-01.md](../testing/BBD-WAL-007-SLICE-02-GREEN-STOP-REVIEW-01.md).
The exact mechanical correction is accepted at XHigh in
[BBD-WAL-007-SLICE-02-FORMAT-CORRECTION-SOURCE-REVIEW-01.md](../testing/BBD-WAL-007-SLICE-02-FORMAT-CORRECTION-SOURCE-REVIEW-01.md).
Hermes's resumed formatter check still failed without source mutation; that stop is
reviewed in
[BBD-WAL-007-SLICE-02-GREEN-STOP-REVIEW-02.md](../testing/BBD-WAL-007-SLICE-02-GREEN-STOP-REVIEW-02.md).
The second formatting handoff named one region that was already in the prescribed
layout. Sol stopped without editing; that stop is reviewed in
[BBD-WAL-007-SLICE-02-FORMAT-CORRECTION-02-STOP-REVIEW-01.md](../testing/BBD-WAL-007-SLICE-02-FORMAT-CORRECTION-02-STOP-REVIEW-01.md).
The retained Rust 1.98 formatter output is preserved in
[BBD-WAL-007-SLICE-02-FORMATTER-DIFF-01.md](../testing/BBD-WAL-007-SLICE-02-FORMATTER-DIFF-01.md).
The exact recorded drop is accepted at XHigh in
[BBD-WAL-007-SLICE-02-FORMAT-CORRECTION-SOURCE-REVIEW-02.md](../testing/BBD-WAL-007-SLICE-02-FORMAT-CORRECTION-SOURCE-REVIEW-02.md).
Hermes completed the Slice-2 focused formatter, falsification, and green gate on resume 02.
The complete green evidence is recorded in
[BBD-WAL-007-SLICE-02-GREEN-01.md](../testing/BBD-WAL-007-SLICE-02-GREEN-01.md).
Slice 2 is accepted at `d0a14dd5` in
[BBD-WAL-007-SLICE-02-ACCEPTANCE-01.md](../testing/BBD-WAL-007-SLICE-02-ACCEPTANCE-01.md).
The acceptance records Hermes's non-mutating post-integration command-scope deviation.
Sol alone may author the exact four-path RPC/local-node Slice-3 drop. Hermes execution
and integration, Slices 4–5, broader acceptance, and the real local-Monero gate remain
unauthorized pending XHigh source review.

BBD-RATE-001 remains complete and accepted at `c7d91c69`; its final evidence is
[BBD-RATE-001-FINAL-ACCEPTANCE-01.md](../testing/BBD-RATE-001-FINAL-ACCEPTANCE-01.md).

Jr Dev routing:
[HERMES_JR_DEV_ROUTING.md](../engineering/HERMES_JR_DEV_ROUTING.md)

BBD-WAL-005 remains complete and accepted at `54cc0ccc`; none of its source is open.
