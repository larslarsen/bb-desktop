# Current Task

Ticket: BBD-WAL-007

State: PHASE C SLICE 1 CORRECTION AUTHORIZED

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Authorized source actor: Principal Dev — Codex Sol (`gpt-5.6-sol`, High)

Authorized integration actor: none

Protected governance parent: the commit containing this task update

Ticket: [BBD-WAL-007.md](../../tickets/BBD-WAL-007.md)

Owner decision:
[BBD-WAL-007-XMR-RPC-DISTRIBUTION-DECISION.md](../architecture/BBD-WAL-007-XMR-RPC-DISTRIBUTION-DECISION.md)

Active handoff:
[CODEX_SOL_BBD_WAL_007_PHASE_C_SLICE_01_CORRECTION_01.md](CODEX_SOL_BBD_WAL_007_PHASE_C_SLICE_01_CORRECTION_01.md)

Slice-1 source requires the exact corrections in
[BBD-WAL-007-SLICE-01-SOURCE-REVIEW-01.md](../testing/BBD-WAL-007-SLICE-01-SOURCE-REVIEW-01.md).
Sol alone may add the missing verifier facade, close its path accessor, and repair the
incremental-inventory policy contradiction. Execution, integration, later slices, and
the real local-Monero gate remain unauthorized.

BBD-RATE-001 remains complete and accepted at `c7d91c69`; its final evidence is
[BBD-RATE-001-FINAL-ACCEPTANCE-01.md](../testing/BBD-RATE-001-FINAL-ACCEPTANCE-01.md).

Jr Dev routing:
[HERMES_JR_DEV_ROUTING.md](../engineering/HERMES_JR_DEV_ROUTING.md)

BBD-WAL-005 remains complete and accepted at `54cc0ccc`; none of its source is open.
