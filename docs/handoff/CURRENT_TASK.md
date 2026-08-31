# Current Task

Ticket: BBD-WAL-004

State: CI GATE CORRECTION — SOL TEST SOURCE AUTHORIZED

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Test/source actor: Principal Dev — Codex Sol (`gpt-5.6-sol`, High)

Integration actor: Jr Dev — Codex Luna (`gpt-5.6-luna`)

Reviewed baseline: `e19af1a50ebc2a6b1f46e504fa02dd168358dbb0`

Production commit: `0e42fb4b477cfe76757ed207d3a561270b9e9efe`

Falsification evidence commit: `e19af1a50ebc2a6b1f46e504fa02dd168358dbb0`

Active handoff: [CODEX_SOL_BBD_WAL_004_CI_GATE_TESTS.md](CODEX_SOL_BBD_WAL_004_CI_GATE_TESTS.md)

The encrypted custody core is locally green, integrated, pushed, and independently
falsified by all seven required temporary mutations. GitHub Social client run
`33357371137` passed with package jobs skipped.

The two manual non-packaging acceptance workflows found two bounded gate defects at the
falsification commit. Security run `33359184973` passed every npm, Node policy, RustSec,
cargo-deny, and installation step, then Gitleaks reported only the reviewer-published
synthetic HKDF vector under the live `key    =` label in the WAL-004 ticket. SBOM run
`33359223628` passed npm SBOM validation, then the Rust validator rejected a
default-feature-only document because it omitted optional direct native components
`eframe` and `rfd`.

The authoritative review is
[BBD-WAL-004-CI-GATE-REVIEW-01.md](../testing/BBD-WAL-004-CI-GATE-REVIEW-01.md). Sol may
now edit only `test/securityPolicy.node.js` to require the exact ninth historical
Gitleaks fingerprint, a repaired live ticket label with unchanged vector bytes, and an
all-features Rust SBOM command. Sol does not execute commands or use Git. Production,
policy, workflows, `.gitleaksignore`, the live ticket label, integration, expected-red
execution, evidence, Git, and GitHub remain frozen pending reviewer inspection.

After test-source acceptance, Luna will integrate the test-only drop and record the
expected two-test red result. A later separate Sol production handoff will be bounded to
the exact policy/workflow/ignore correction. The reviewer owns the one-word governance
ticket relabel. Final acceptance requires both local Gitleaks modes and fresh successful
manual Security and SBOM runs; it does not require any platform package build.

Grok Build is available and remains queued for the independent `../bb-go`
exchange-rate/provider work after this desktop security correction. `../go-ipfs` is
deprecated and receives no wallet work.
