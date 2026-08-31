# Current Task

Ticket: BBD-WAL-004

State: CI GATE TEST SOURCE ACCEPTED — LUNA EXPECTED RED AUTHORIZED

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Test/source actor: Principal Dev — Codex Sol (`gpt-5.6-sol`, High)

Integration actor: Jr Dev — Codex Luna (`gpt-5.6-luna`)

Governance parent: the commit containing this handoff

Reviewed CI baseline: `e19af1a50ebc2a6b1f46e504fa02dd168358dbb0`

Production commit: `0e42fb4b477cfe76757ed207d3a561270b9e9efe`

Active handoff: [CODEX_LUNA_BBD_WAL_004_CI_GATE_RED.md](CODEX_LUNA_BBD_WAL_004_CI_GATE_RED.md)

The encrypted custody core is locally green, integrated, pushed, and independently
falsified by all seven required temporary mutations. GitHub Social client run
`33357371137` passed with package jobs skipped.

Manual Security run `33359184973` failed only on the reviewer-published synthetic HKDF
vector's Gitleaks false positive. Manual SBOM run `33359223628` failed because the Rust
document omitted optional native direct dependencies under default features. The
authoritative diagnosis is
[BBD-WAL-004-CI-GATE-REVIEW-01.md](../testing/BBD-WAL-004-CI-GATE-REVIEW-01.md).

Sol's exact one-file regression-test drop is reviewer-accepted in
[BBD-WAL-004-CI-GATE-TEST-SOURCE-REVIEW-01.md](../testing/BBD-WAL-004-CI-GATE-TEST-SOURCE-REVIEW-01.md).
Luna may now run only the targeted two-test expected-red command, record evidence, and
integrate the test source under the active handoff. Production policy, workflows,
`.gitleaksignore`, the live ticket label, broad tests, scanners, and all wallet source
remain frozen until the reviewer accepts the red evidence and publishes a separate
production handoff.

After exact red, Sol will receive the bounded security/release-gate production
correction. The reviewer owns the one-word governance ticket relabel. Final acceptance
requires both local Gitleaks modes and fresh successful manual Security and SBOM runs;
it does not require any platform package build.

Grok Build is available and queued for the independent `../bb-go`
exchange-rate/provider work after this desktop security correction. `../go-ipfs` is
deprecated and receives no wallet work.
