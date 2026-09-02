# Current Task

Ticket: BBD-WAL-007

State: PHASE A MD5 PIN CORRECTION AUTHORIZED

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Authorized source actor: Principal Dev — Codex Sol (`gpt-5.6-sol`, High)

Authorized integration actor: none

Protected governance parent: the commit containing this task update

Ticket: [BBD-WAL-007.md](../../tickets/BBD-WAL-007.md)

Owner decision:
[BBD-WAL-007-XMR-RPC-DISTRIBUTION-DECISION.md](../architecture/BBD-WAL-007-XMR-RPC-DISTRIBUTION-DECISION.md)

Active handoff:
[CODEX_SOL_BBD_WAL_007_MD5_PIN_CORRECTION_01.md](CODEX_SOL_BBD_WAL_007_MD5_PIN_CORRECTION_01.md)

Hermes stopped before lock mutation because the final `md-5 0.11.0` pin conflicts with
the Zcash graph's exact prerelease Digest. Sol may change only the exact dependency line
and its Node policy mirror to `md-5 0.11.0-pre.4`. All other accepted test bytes,
production, execution, integration, and the real local-Monero gate remain unauthorized.

BBD-RATE-001 remains complete and accepted at `c7d91c69`; its final evidence is
[BBD-RATE-001-FINAL-ACCEPTANCE-01.md](../testing/BBD-RATE-001-FINAL-ACCEPTANCE-01.md).

Jr Dev routing:
[HERMES_JR_DEV_ROUTING.md](../engineering/HERMES_JR_DEV_ROUTING.md)

BBD-WAL-005 remains complete and accepted at `54cc0ccc`; none of its source is open.
