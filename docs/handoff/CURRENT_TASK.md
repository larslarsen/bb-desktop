# Current Task

Ticket: BBD-WAL-007

State: PHASE C SLICE 2 OWNED-CHILD TEST CORRECTION AUTHORIZED

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Authorized source actor: Principal Dev — Codex Sol (`gpt-5.6-sol`, High)

Authorized integration actor: none

Protected governance parent: the commit containing this task update

Ticket: [BBD-WAL-007.md](../../tickets/BBD-WAL-007.md)

Owner decision:
[BBD-WAL-007-XMR-RPC-DISTRIBUTION-DECISION.md](../architecture/BBD-WAL-007-XMR-RPC-DISTRIBUTION-DECISION.md)

Active handoff:
[CODEX_SOL_BBD_WAL_007_SLICE_02_OWNED_CHILD_TEST_CORRECTION.md](CODEX_SOL_BBD_WAL_007_SLICE_02_OWNED_CHILD_TEST_CORRECTION.md)

Slice 1 is complete and accepted at `c139641a` in
[BBD-WAL-007-SLICE-01-ACCEPTANCE-01.md](../testing/BBD-WAL-007-SLICE-01-ACCEPTANCE-01.md).
Sol stopped Slice 2 before editing because stable safe Rust cannot signal a process
group. The XHigh decision is
[BBD-WAL-007-SLICE-02-OWNED-CHILD-DECISION.md](../architecture/BBD-WAL-007-SLICE-02-OWNED-CHILD-DECISION.md).
Sol alone may make the exact one-path test correction. Process source, Hermes execution/
integration, Slice 3, broader acceptance, and the real local-Monero gate remain
unauthorized pending XHigh test-source review.

BBD-RATE-001 remains complete and accepted at `c7d91c69`; its final evidence is
[BBD-RATE-001-FINAL-ACCEPTANCE-01.md](../testing/BBD-RATE-001-FINAL-ACCEPTANCE-01.md).

Jr Dev routing:
[HERMES_JR_DEV_ROUTING.md](../engineering/HERMES_JR_DEV_ROUTING.md)

BBD-WAL-005 remains complete and accepted at `54cc0ccc`; none of its source is open.
