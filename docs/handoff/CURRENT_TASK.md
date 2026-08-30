# Current Task

Ticket: BBD-WAL-002

State: COMPLETE — REVIEWER ACCEPTED

Reviewer: Lead Engineer/Reviewer — Codex

Source actor: Temporary Sr Dev — Codex Sol (`gpt-5.6-sol`, High)

Integration actor: Jr Dev — Codex Luna (`gpt-5.6-luna`)

[BBD-WAL-002](../../tickets/BBD-WAL-002.md) is the only authorized work. Its
expected-red evidence is reviewer-accepted at commit
`3a60b33c2b4c2f8355007fbd3535066cd0a0d1c6`. The first bounded production drop is
present but reviewer-rejected before execution. Correction 05 test source is accepted at
1,697 lines and SHA-256
`3e51281d16da7eec4a178eeb799ec23e2854206a096ed741cba920fc35825ee9`, with 45 wallet
tests total. Its exact 38-pass/7-fail correction red is accepted at evidence commit
`3207667e276cffcb438988610cfa90a64e130ffd`. Production Correction 01 closes those seven
causes but is reviewer-rejected before green. Correction 06 is accepted at 1,803 lines,
SHA-256 `43830b1caec19904d23b400974c77c1edbebe32b4927b2f31ee4279611a46dbf`, and 48 wallet
tests. Its exact 45-pass/3-fail expected red is accepted at evidence commit
`12061f5c1ab24a90dda9f7e74c846b0174d0e039`. Production Correction 02 is XHigh-accepted
at the exact hashes in
[`CODEX_LUNA_BBD_WAL_002_TARGETED_GREEN.md`](CODEX_LUNA_BBD_WAL_002_TARGETED_GREEN.md).
Targeted green passed 48 wallet, 14 Electron-security, and 54 policy tests with no drift.
Broader acceptance, audit, three falsifications/restores, evidence, and the exact 15-path
integration commit completed at `62ad0ceb90a96805cabff0296fd6980a739554fd`. GitHub
Actions Social client run `33338667462` passed. Reviewer acceptance is recorded in
[`BBD-WAL-002-ACCEPTANCE.md`](../testing/BBD-WAL-002-ACCEPTANCE.md). All source and
integration actors are stopped; no further implementation is authorized.

The owner approved this ticket-specific substitution after Grok Build returned a
quota-exhausted 402 before reading or changing source. The reviewer operates at XHigh;
Sol authors at High. Standing project roles are otherwise unchanged.

The accepted implementation uses only Node built-ins, remains offline/inert, preserves
`package-lock.json`, and closes the tested schema, lifecycle, recovery, capability,
dependency-error, and structured-log boundaries. Historical red and correction evidence
hashes remain recorded in the ticket and testing documents.

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
