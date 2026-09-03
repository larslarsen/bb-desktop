# BBD-WAL-008 Slice-02 Acceptance 01

Reviewer: Lead Engineer/Reviewer — Codex at High

Implementation commit: `369d811cfdb7d659eaba13e8b58d1c07c3624c84`

Evidence-correction commit: `9668689050796639620b9c62401240b4c2d4d4e4`

Result: **SLICE-02 PERSISTENCE AND POLICY ACCEPTED**

The complete required sequence is accepted: formatter; non-vacuous stale-expansion
falsification/restoration; 18 focused hardware tests; 27 affected ZEC regressions;
warning-denied scoped Clippy; native compilation; 48 wallet-contract tests; 87 policy
tests; and the production policy checker. All required outcomes are exact and the four
source paths were integrated and pushed at `369d811c`.

The corrected evidence now discloses the two post-integration hash checks and two
repeated focused tests from Hermes session `20260903_152543_9421ad`. Those commands
violated the stop/once-only contract but exited cleanly and mutated nothing. The
correction session itself ran no gate, though its final Git proof unnecessarily added
`git log --oneline -3`; that read-only command is recorded here and does not require
another correction cycle.

GitHub run `33813477614` for the implementation commit independently passed `npm run
build`, every maintained Node test group, full no-default Rust tests, and Rust formatting.
It failed only at the repository-wide all-targets/all-features Clippy step on the
already parked WAL-007 `xmr_local_gate` absent Phase-D API contract plus the XMR-only
`chunks_exact_to_as_chunks` lint in `tests/xmr_rpc.rs`; native check was skipped after
that failure. These are outside the accepted WAL-008 paths, predate this slice, and are
not permission to edit or execute Monero while its gate remains parked.

This acceptance proves the ticket's fake-device capability, route-selection, durable
narrowing, reopen/fault, and closed-policy boundaries. It does not claim a real device,
transport, signing, PCZT exchange, broadcast, mainnet, or release-wide green CI. The
remaining independent audit/license/secret scans are authorized separately before the
ticket is marked locally complete.
