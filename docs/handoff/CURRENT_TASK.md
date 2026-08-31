# Current Task

Ticket: BBD-WAL-004

State: CI GATE EXPECTED RED ACCEPTED — SOL PRODUCTION CORRECTION AUTHORIZED

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Source actor: Principal Dev — Codex Sol (`gpt-5.6-sol`, High)

Integration actor: Jr Dev — Codex Luna (`gpt-5.6-luna`)

Governance parent: the commit containing this handoff

Reviewed CI baseline: `e19af1a50ebc2a6b1f46e504fa02dd168358dbb0`

Production commit: `0e42fb4b477cfe76757ed207d3a561270b9e9efe`

Expected-red commit: `61d2a239a89384885a95cf353f4d3ccc319670a7`

Active handoff: [CODEX_SOL_BBD_WAL_004_CI_GATE_PRODUCTION.md](CODEX_SOL_BBD_WAL_004_CI_GATE_PRODUCTION.md)

The encrypted custody core is locally green, integrated, pushed, and independently
falsified by all seven required temporary mutations. GitHub Social client run
`33357371137` passed with package jobs skipped.

Manual Security run `33359184973` failed only on the reviewer-published synthetic HKDF
vector's Gitleaks false positive. Manual SBOM run `33359223628` failed because the Rust
document omitted optional native direct dependencies under default features. The
authoritative diagnosis is
[BBD-WAL-004-CI-GATE-REVIEW-01.md](../testing/BBD-WAL-004-CI-GATE-REVIEW-01.md).

The accepted test source and exact 66-green/3-red execution are recorded in
[BBD-WAL-004-CI-GATE-TEST-SOURCE-REVIEW-01.md](../testing/BBD-WAL-004-CI-GATE-TEST-SOURCE-REVIEW-01.md),
[BBD-WAL-004-CI-GATE-EXPECTED-RED.md](../testing/BBD-WAL-004-CI-GATE-EXPECTED-RED.md),
and
[BBD-WAL-004-CI-GATE-RED-REVIEW-01.md](../testing/BBD-WAL-004-CI-GATE-RED-REVIEW-01.md).
The initial invalid native-runner attempt remains durably recorded and made no change.

The reviewer has relabeled only the live synthetic vector from `key    =` to `expand =`
without changing its hex bytes. Sol may now edit only `.gitleaksignore`,
`scripts/security-policy.js`, and `.github/workflows/sbom.yml` under the active handoff.
Tests, validators, wallet source, dependencies, other workflows, execution, integration,
evidence, Git, and GitHub remain frozen until reviewer source acceptance.

Final acceptance requires the local full Node/security gate, both local pinned Gitleaks
modes, and fresh successful manual Security and SBOM runs. It does not require any
platform package build.

Grok Build is available and queued for the independent `../bb-go`
exchange-rate/provider work after this desktop security correction. `../go-ipfs` is
deprecated and receives no wallet work.
