# Wallet Roadmap Model Routing

This is the durable assignment queue for the accepted BBD-WAL-001 architecture. It
changes no product source and does not authorize a queued ticket by itself. The active
repository handoff remains `docs/handoff/CURRENT_TASK.md`.

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

## Priority and dependency order

| Priority | Ticket | Source actor | Why that actor | Start condition |
| --- | --- | --- | --- | --- |
| P0 complete | BBD-WAL-003 | Principal Dev — Codex Sol, High | Electron/native-process trust boundary, authenticated session transcript, spawn ordering, and fail-closed IPC | Reviewer-accepted at `abdd2b19` |
| P1 active | BBD-WAL-004 | Codex Sol, High | Software custody, owner-selected in-broker native unlock/backup surface, encrypted persistence, and zeroization | Correction 6 accepted; final green integration active |
| P1 cross-repo | BBGO-PAY-001 | Sr Dev — Grok Build, High | The reviewer-frozen WAL-002 JCS/signature semantics make this bounded daemon protocol work; no wallet or rate authority | May proceed in `../bb-go` while desktop has a different active ticket |
| P2 | BBD-RATE-001 | Grok Build, High | Bounded untrusted-provider parser/worker and deterministic aggregation after the process split exists | WAL-003 accepted |
| P2 | BBD-WAL-005 | Codex Spark, High for mechanical UI/view-model slices; reviewer re-routes any security-sensitive slice | Sanitized preview, fixed capability labels, and state wiring are mechanical only after broker and payment schemas are fixed | WAL-003 and BBGO-PAY-001 contracts accepted |
| P3 | BBD-WAL-006 | Codex Sol, High | ZEC consensus, PCZT, Ironwood, and librustzcash adapter | WAL-004 accepted |
| P3 | BBD-WAL-007 | Codex Sol, High | XMR wallet/node process isolation, RPC authentication, and persistence | WAL-004 accepted and owner chooses wallet-rpc distribution |
| P3 | BBD-WAL-008 | Codex Sol, High | Hardware capability attestation and device trust | Applicable coin adapter accepted |
| P3 | BBD-WAL-009 | Codex Sol, High | Broadcast, concurrency, cancellation races, and crash recovery | Coin adapters accepted |
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

The model queue does not decide XMR wallet-rpc distribution, a default ZEC light endpoint,
ZEC compact-block IP privacy/Tor policy, or unbound payment requests. Those choices are
separate owner gates. Payer-bound requests remain the v1 baseline.

Broker native toolkit Q10 is resolved: the owner selected a minimal native window inside
the Rust broker, with broker-invoked native file dialogs and no v1 OS credential agent.
Electron never owns unlock, backup, or payment confirmation.
