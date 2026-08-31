# Current Task

Ticket: BBD-WAL-006

State: PHASE-A TEST SOURCE ACCEPTED — FIXTURE AND EXPECTED RED AUTHORIZED

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Accepted source actor: Principal Dev — Codex Sol (`gpt-5.6-sol`, High)

Authorized integration actor: Jr Dev — Codex Luna (`gpt-5.6-luna`)

Source baseline: `363c0046`

Test-source review:
[BBD-WAL-006-TEST-SOURCE-REVIEW-01.md](../testing/BBD-WAL-006-TEST-SOURCE-REVIEW-01.md)

Active handoff:
[CODEX_LUNA_BBD_WAL_006_FIXTURE_RED.md](CODEX_LUNA_BBD_WAL_006_FIXTURE_RED.md)

Architecture:
[BBD-WAL-006-UPSTREAM-REVIEW.md](../architecture/BBD-WAL-006-UPSTREAM-REVIEW.md)

Ticket: [BBD-WAL-006.md](../../tickets/BBD-WAL-006.md)

BBD-WAL-004 remains complete and reviewer-accepted at `e8894a44`. BBGO-PAY-001 remains
complete in `../bb-go` at production `6bbb0629` and final evidence `801f5d55`.

The exact eight uncommitted Phase-A test/manifest hashes in the review are accepted.
Only Luna may integrate them, resolve and inventory the crates.io graph, run the
upstream-only fixture target, freeze its exact synthetic local-consensus bytes, run the
named Node and Rust expected-red commands, record evidence, and perform the corresponding
Git commit/push under the active handoff.

No production source or policy implementation is authorized. XHigh must accept the
resolved graph, fixture bytes/provenance, and exact expected-red evidence before a
separate Phase-C source handoff exists. No broader tests, falsification, live endpoint,
mainnet, signing, proving, extraction, broadcast, hardware, Electron, package, SBOM, or
other-repository work is authorized in this phase.

This slice remains offline and synthetic. `../go-ipfs` is deprecated and receives no
wallet work.
