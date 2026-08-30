# Codex Luna Handoff — BBD-SEC-001

You are **Jr Dev — Codex Luna** (`gpt-5.6-luna`). This file is the durable integration
contract; ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Governance baseline: `b64a7d00`

Read completely: `AGENTS.md`, `TESTING.md`, `docs/engineering/DEVELOPMENT_ROLES.md`,
`tickets/BBD-SEC-001.md`, `docs/handoff/CURRENT_TASK.md`, and the completed
`docs/handoff/GROK_BUILD_BBD_SEC_001.md` source report.

Verify every Grok hash before execution. Reconstruct and record bounded red without
deletion or unresolved targets, restore exact hashes, run green and every ticketed
falsification, then execute acceptance commands in order. Stop immediately on a changed
hash, test/policy/scanner failure, npm vulnerability, Electronegativity finding, Gitleaks
finding, or SBOM validation failure. Do not repair, suppress, baseline, or continue past a
finding.

Use the existing Gitleaks v8.30.1 binary under
`/home/lars/OpenBazaar/.security-tools/bbgo-sec-tools-20260829`. Create any substantial
cache/temp/artifact state only at explicit paths beneath:

- `/home/lars/OpenBazaar/.security-cache/bbd-sec-001-20260829`
- `/home/lars/OpenBazaar/.security-tmp/bbd-sec-001-20260829`
- `/home/lars/OpenBazaar/.security-artifacts/bbd-sec-001-20260829`

Do not use local `/tmp`, install globally, clean, delete, or run `rm` in any form. Never
record a secret or match body. Native packaging and all GitHub manual workflows remain
undispatched.

If every gate passes, author only `docs/security/BBD-SEC-001-EVIDENCE.md` and update
`docs/handoff/CURRENT_TASK.md`, run `git diff --check`, commit exactly the authorized
ticket change, push `origin/master`, and stop for reviewer inspection. Record exact
commands, versions, exit codes, counts, hashes, safe summaries, commit, and push result.
