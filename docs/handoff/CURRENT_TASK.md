# Current Task

Ticket: BBD-WAL-002

State: CORRECTION 01 AUTHORIZED — TEST SOURCE ONLY

Reviewer: Lead Engineer/Reviewer — Codex

Source actor: Temporary Sr Dev — Codex Sol (`gpt-5.6-sol`, High)

Integration actor: Jr Dev — Codex Luna (`gpt-5.6-luna`)

[BBD-WAL-002](../../tickets/BBD-WAL-002.md) is the only authorized work. The current
phase authorizes temporary Sr Dev — Codex Sol to correct its test source only under the
exact durable contract in
[`CODEX_SOL_BBD_WAL_002_TESTS_CORRECTION_01.md`](CODEX_SOL_BBD_WAL_002_TESTS_CORRECTION_01.md).
It must stop
without execution, production source, dependencies, Git, network, wallets, nodes,
devices, keys, or transaction capability.

The owner approved this ticket-specific substitution after Grok Build returned a
quota-exhausted 402 before reading or changing source. The reviewer operates at XHigh;
Sol authors at High. Standing project roles are otherwise unchanged.

The initial Sol drop is not accepted. Reviewer XHigh inspection found the exact blockers
recorded in the correction handoff: vacuous malformed-UTF-8 coverage, insufficient
fixture-driven cross-language negatives, no injected post-sign cancellation rechecks,
incomplete crash-recovery revalidation, an inert-fake contradiction, and incomplete
package/security wiring assertions. Codex Luna must not run the expected-red commands
until the corrected source is reviewer-accepted. Production remains unauthorized and a
failing test tree must not be committed.

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
