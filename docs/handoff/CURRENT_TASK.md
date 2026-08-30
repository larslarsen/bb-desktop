# Current Task

Ticket: BBD-WAL-001

State: AUTHORIZED — DESIGN ONLY

Reviewer: Lead Engineer/Reviewer — Codex

Source actor: Sr Dev — Grok Build (Grok 4.6 High)

Integration actor: Jr Dev — Codex Luna (`gpt-5.6-luna`)

[BBD-WAL-001](../../tickets/BBD-WAL-001.md) is the only authorized work. It records the
owner's decision to support built-in shielded ZEC and optional local-node XMR from the
beginning, with software, hardware-backed, and watch-only accounts shaped by a common
contract. Ledger and Trezor support are first-class requirements.

This is a design-only gate. Grok Build may author only
[BBD-WAL-001-REVIEW.md](../architecture/BBD-WAL-001-REVIEW.md) under the complete prompt
in [GROK_BUILD_BBD_WAL_001.md](GROK_BUILD_BBD_WAL_001.md). No implementation, test,
dependency, wallet, node, hardware, network, mainnet, package, or other-repository work is
authorized.

The maintained `../bb-go/modern` social daemon remains wallet-free. The proposed local
wallet broker belongs to the desktop product boundary but must run outside the Electron
renderer and expose no generic wallet HTTP API. The inherited OpenBazaar wallet and
deprecated `../go-ipfs` are not implementation foundations.

BBD-SEC-001 remains complete and accepted at implementation commit
`47bf45884d737b4b89571f06d8ba3b4e20238bfb`, with documentation acceptance commit
`20c7f7e7e71a5d98c1e236fea9d7d3dc1eeffb8a`.
