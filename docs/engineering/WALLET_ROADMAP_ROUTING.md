# Wallet Roadmap Model Routing

This is the durable assignment queue for the accepted BBD-WAL-001 architecture. It
changes no product source and does not authorize a queued ticket by itself. The active
repository handoff remains `docs/handoff/CURRENT_TASK.md`.

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

## Priority and dependency order

| Priority | Ticket | Source actor | Why that actor | Start condition |
| --- | --- | --- | --- | --- |
| P0 complete | BBD-WAL-003 | Principal Dev — Codex Sol, High | Electron/native-process trust boundary, authenticated session transcript, spawn ordering, and fail-closed IPC | Reviewer-accepted at `abdd2b19` |
| P1 complete | BBD-WAL-004 | Codex Sol, High | Software custody, owner-selected in-broker native unlock/backup surface, encrypted persistence, and zeroization | Reviewer-accepted at `e8894a44` |
| P1 complete, cross-repo | BBGO-PAY-001 | Sr Dev — Grok Build, High | The reviewer-frozen WAL-002 JCS/signature semantics made this bounded daemon protocol work; no wallet or rate authority | Production `6bbb0629`; final evidence `801f5d55` in `../bb-go` |
| P2 complete | BBD-RATE-001 | Grok Build, High | Bounded untrusted-provider parser/worker and deterministic single-source selection after the process split exists | Production accepted at `c7d91c69`; local falsification/green evidence and GitHub Actions run `33578112536` passed |
| P2 complete | BBD-WAL-005 | Codex Sol, High | The view model touches the security-sensitive shared snapshot sanitizer; two bounded Grok Build attempts produced no file changes and were stopped, so the reviewer re-routed the unchanged source task | Production accepted at `0b51c73f`; final falsification evidence at `3e1e0b14` |
| P2 complete | BBD-WAL-006 | Codex Sol, High | ZEC consensus, PCZT, Ironwood, SQLite viewing state, and librustzcash adapter | Reviewer-accepted at `996444e9`; final evidence at `14a68187` |
| P3 ticket ready | BBD-WAL-007 | Codex Sol, High | XMR wallet/node process isolation, RPC authentication, viewing/recovery, and durable subaddresses | Ticket frozen at `tickets/BBD-WAL-007.md`; implementation authorization pending |
| P3 | BBD-WAL-008 | Codex Sol, High | Hardware capability attestation and device trust | Applicable coin adapter accepted |
| P3 | BBD-WAL-009 | Codex Sol, High | XMR prepare/sign/verify plus cross-coin broadcast, concurrency, cancellation races, and crash recovery | Coin adapters accepted |
| P4 | BBD-WAL-010 | Codex Spark, High for DOM/CSS/state boilerplate; Grok or Sol for any authority-bearing correction | User-facing composition after authority and state-machine behavior are executable | WAL-005, RATE-001, and payment flow accepted |
| P4 | BBD-WAL-011 | Codex Sol, High | Native sidecar packaging, binary pins, SBOM, scanners, and sandbox preservation | Native components accepted |
| P5 | BBD-WAL-012 | Codex Sol, High | Mainnet release gate and money-safety evidence | Every preceding wallet gate accepted |

## Routing guardrails

- Exactly one source actor owns a ticket. Spark, Grok, and Sol never edit the same drop.
- Tests lead every source phase. The source actor writes tests and stops; Codex Luna owns
  execution, evidence, Git, commit, and push.
- Codex Sol is not spent on rote fixture copying, CSS, static tables, or already-fixed
  schema wiring. Codex Spark never owns signature verification, IPC authority, native
  process verification, custody, persistent state, hardware capability decisions, or
  transaction/broadcast behavior.
- Grok Build may implement a bounded protocol or rate contract only after the reviewer
  fixes its schemas, trust direction, failure behavior, and forbidden capabilities.
- Work in `bb-desktop` and `bb-go` may proceed concurrently only when their authorized
  paths and contracts do not overlap. A cross-repository baseline is recorded in both
  tickets before integration.
- `go-ipfs` is deprecated and receives no wallet work.

## Owner decisions still open

The XMR wallet-rpc distribution gate is resolved in
`../architecture/BBD-WAL-007-XMR-RPC-DISTRIBUTION-DECISION.md`. The remaining owner gates
include a default ZEC light endpoint, ZEC compact-block IP privacy/Tor policy, and unbound
payment requests. Payer-bound requests remain the v1 baseline. WAL-006 deliberately uses
only frozen local-consensus fixtures, so the open endpoint and Tor decisions do not
silently become implementation defaults.

Broker native toolkit Q10 is resolved: the owner selected a minimal native window inside
the Rust broker, with broker-invoked native file dialogs and no v1 OS credential agent.
Electron never owns unlock, backup, or payment confirmation.
