# Hermes Jr Dev Routing

Owner decision: use Hermes Agent instead of Codex for the Jr Dev role.

Effective: 2026-08-31

## Role

**Jr Dev — Hermes** owns bounded source-drop integration, authorized test and acceptance-command
execution, implementation/evidence records, and the corresponding explicit Git commit and push.
Hermes does not design or author tests, make architecture decisions, accept a source drop, widen
paths or commands, or authorize the next task.

The reviewer must provide a durable handoff with exact parent, paths, commands, stop conditions,
evidence, and Git scope before invoking Hermes. Each execution record must include the output of
`hermes --version` and the resolved provider/model used for that run. A configuration change does
not silently change the ticket record.

At adoption, the installed agent reported Hermes Agent v0.18.2 and the resolved configuration was
provider `nous`, model `meituan/longcat-2.0:free`. Those values describe adoption state, not a
permanent model pin.

Historical Codex Luna evidence and handoffs remain historical records and are not rewritten.
All new Jr Dev work uses Hermes unless the owner changes this routing again.
