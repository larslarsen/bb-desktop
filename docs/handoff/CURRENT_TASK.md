# Current Task

Ticket: BBD-WAL-004

State: CI GATE GREEN RUN 01 — SOL TEST FIXTURE CORRECTION AUTHORIZED

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Source actor: Principal Dev — Codex Sol (`gpt-5.6-sol`, High)

Integration actor: Jr Dev — Codex Luna (`gpt-5.6-luna`)

Governance parent: the commit containing this handoff

Reviewed CI baseline: `e19af1a50ebc2a6b1f46e504fa02dd168358dbb0`

Expected-red commit: `61d2a239a89384885a95cf353f4d3ccc319670a7`

Active handoff: [CODEX_SOL_BBD_WAL_004_CI_GATE_TESTS_CORRECTION_01.md](CODEX_SOL_BBD_WAL_004_CI_GATE_TESTS_CORRECTION_01.md)

The encrypted custody core is locally green, integrated, pushed, and independently
falsified by all seven required temporary mutations. GitHub Social client run
`33357371137` passed with package jobs skipped.

The accepted CI-gate regression source produced the exact 66-green/3-red result. Sol's
exact three-file Gitleaks-policy and all-features SBOM correction remains reviewer-
accepted and unstaged with hashes frozen in
[BBD-WAL-004-CI-GATE-PRODUCTION-SOURCE-REVIEW-01.md](../testing/BBD-WAL-004-CI-GATE-PRODUCTION-SOURCE-REVIEW-01.md).

Green Run 01 preflight passed, then the first policy command produced 68 `ok` and one
`not ok`. The exact diagnosis is
[BBD-WAL-004-CI-GATE-GREEN-RUN-01.md](../testing/BBD-WAL-004-CI-GATE-GREEN-RUN-01.md):
three old mutation fixtures still targeted the former index-0 `index.html:57` entry, so
they became no-ops after the new `tickets/BBD-WAL-004.md:110` entry was inserted first.
Luna stopped before any other command, edit, integration, Git, or GitHub action.

Sol may now edit only `test/securityPolicy.node.js` to make the wrong-path, wrong-line,
and wildcard mutations target the new first fingerprint and explicitly prove each
mutation is non-vacuous. The three accepted production paths and every other path remain
frozen. Luna receives no resume authority until reviewer test-source acceptance.

No platform package build is authorized or required. Grok Build remains available and
queued for the independent `../bb-go` exchange-rate/provider work after this desktop
security correction. `../go-ipfs` is deprecated and receives no wallet work.
