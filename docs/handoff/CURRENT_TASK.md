# Current Task

Ticket: BBD-WAL-001

State: CORRECTION 02 — SOURCE ONLY

Reviewer: Lead Engineer/Reviewer — Codex

Source actor: Sr Dev — Grok Build (Grok 4.6 High)

Integration actor: Jr Dev — Codex Luna (`gpt-5.6-luna`)

[BBD-WAL-001](../../tickets/BBD-WAL-001.md) is the only authorized work. It records the
owner's decision to support built-in shielded ZEC and optional local-node XMR from the
beginning, with software, hardware-backed, and watch-only accounts shaped by a common
contract. Ledger and Trezor support are first-class requirements.

Correction 01 resolved the protocol-fact, canonicalization, state-machine, XMR RPC, IPC,
and Electron authorization blockers. Before integration, the owner identified the
inherited OB1 exchange-rate dependency. Only the bounded rate architecture and validation
fixes in
[GROK_BUILD_BBD_WAL_001_CORRECTION_02.md](GROK_BUILD_BBD_WAL_001_CORRECTION_02.md) are
now authorized. Grok may edit only
[BBD-WAL-001-REVIEW.md](../architecture/BBD-WAL-001-REVIEW.md) and must run nothing.
Codex Luna remains stopped.

The maintained `../bb-go/modern` social daemon remains wallet-free. The local wallet
broker belongs to the desktop product boundary. Electron may supervise and request a
review but may not carry confirmation, unlock credentials, backup material, sign, or
broadcast authority. The broker owns the native authorization surface. Product/generic
wallet HTTP is forbidden; authenticated loopback XMR wallet RPC remains contained behind
the broker. The inherited OpenBazaar wallet and deprecated `../go-ipfs` are not
implementation foundations.

Exchange rates are optional, untrusted presentation data. Exact atomic ZEC/XMR amounts
remain authoritative, and payments must work when every quote source is unavailable. The
legacy daemon route, `ticker.openbazaar.org`, and old exchange fallbacks are rejected.

BBD-SEC-001 remains complete and accepted at implementation commit
`47bf45884d737b4b89571f06d8ba3b4e20238bfb`, with documentation acceptance commit
`20c7f7e7e71a5d98c1e236fea9d7d3dc1eeffb8a`.
