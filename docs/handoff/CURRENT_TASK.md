# Current Task

Ticket: BBD-WAL-002

State: EXPECTED RED ACCEPTED — PRODUCTION SOURCE AUTHORIZED

Reviewer: Lead Engineer/Reviewer — Codex

Source actor: Temporary Sr Dev — Codex Sol (`gpt-5.6-sol`, High)

Integration actor: Jr Dev — Codex Luna (`gpt-5.6-luna`)

[BBD-WAL-002](../../tickets/BBD-WAL-002.md) is the only authorized work. Its
expected-red evidence is reviewer-accepted at commit
`3a60b33c2b4c2f8355007fbd3535066cd0a0d1c6`. The current phase authorizes temporary Sr
Dev — Codex Sol only under
[`CODEX_SOL_BBD_WAL_002_PRODUCTION.md`](CODEX_SOL_BBD_WAL_002_PRODUCTION.md) to author
the bounded production source and wiring. Sol must not execute commands, edit the
accepted tests, use Git, or create dependencies. Luna remains stopped.

The owner approved this ticket-specific substitution after Grok Build returned a
quota-exhausted 402 before reading or changing source. The reviewer operates at XHigh;
Sol authors at High. Standing project roles are otherwise unchanged.

Correction 03 test source remains accepted at the hashes in the ticket. Red evidence is
31 lines at SHA-256
`f8b33e5f5c510188676deee11a073d6c053ae3d9b4de9279a3cfca25a68e2819`. The green
source must use only Node built-ins, remain offline/inert, preserve `package-lock.json`,
and satisfy the accepted tests without weakening their schemas, lifecycle, or security
boundaries.

BBD-WAL-001 architecture is integrated and reviewer-accepted in commit
`ffebcc179d3b2aaca687213de54d3c3298ac0696` at exactly 2,271 lines and SHA-256
`aae487b169689f310b222640427c1cdae62850d39ebb0243e29f10568d6fcb3f`.

The maintained `../bb-go/modern` social daemon remains wallet-free. The local wallet
broker belongs to the desktop product boundary. Electron may supervise and request a
review but may not carry confirmation, unlock credentials, backup material, sign, or
broadcast authority. The broker owns the native authorization surface. Product/generic
wallet HTTP is forbidden; authenticated loopback XMR wallet RPC remains contained behind
the broker. The inherited OpenBazaar wallet and deprecated `../go-ipfs` are not
implementation foundations.

Exchange rates are optional, untrusted presentation data and are deliberately absent from
WAL-002 signed objects and eligibility. Exact atomic ZEC/XMR amounts remain authoritative.
The legacy daemon route, `ticker.openbazaar.org`, and old exchange fallbacks are rejected.

BBD-SEC-001 remains complete and accepted at implementation commit
`47bf45884d737b4b89571f06d8ba3b4e20238bfb`, with documentation acceptance commit
`20c7f7e7e71a5d98c1e236fea9d7d3dc1eeffb8a`.
