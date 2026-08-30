# Grok Build Handoff — BBD-SEC-001

You are **Sr Dev — Grok Build**, using Grok 4.6 High. This file is the complete durable
prompt; ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Production baseline: `f40d8404`

Governance baseline: `b64a7d00`

Read completely before editing: `AGENTS.md`, `TESTING.md`,
`docs/engineering/DEVELOPMENT_ROLES.md`, `tickets/BBD-SEC-001.md`, and
`docs/handoff/CURRENT_TASK.md`.

Implement only BBD-SEC-001's authorized source paths. Author both test files completely
before any production or policy source. The Electron test must exercise the actual
maintained main module with controlled Electron mocks and independently prove the runtime
invariants; the workflow/policy test must use mutation tests rather than success-string
checks. Then make the smallest runtime/CSP hardening and create the fail-closed workflow,
policy, and SBOM validator sources.

Preserve `package-lock.json`, Electron 44.0.0, daemon connectivity, social behavior,
manual native packaging jobs, the dev-only `npm start --no-sandbox` workaround, and all
unrelated source. Do not add dependencies, revive or scan the inherited marketplace as
maintained code, change packaging/fuses, or touch `go-ipfs`.

Do not execute tests, validators, npm/npx, scanners, formatters, builds, Git, GitHub, or
install commands. Do not use `/tmp`, clean, delete, or inspect secret values. Final
read-only hashes and line counts of authorized paths are allowed.

Report test-source-first order; test names and non-vacuous oracles; exact workflow
triggers, commands and pins; production behavior; SBOM validation; final path hashes and
line counts; ambiguities; and confirmation of no out-of-scope action. Codex Luna owns
execution, evidence, Git, and push.

## Delivered source report — 2026-08-29

Grok authored both test files first (12 Electron tests and 34 policy/SBOM tests), then the
eight production/policy paths, and stopped without execution, installs, Git, or GitHub
operations. Reported hashes and line counts matched reviewer read-only verification:

- `test/electronSecurity.node.js`: 463 lines,
  `7ef0b8939622d014580f47a6e3eac75314aad9a5fc1d62e125eb2d1496f905ea`
- `test/securityPolicy.node.js`: 590 lines,
  `007c5998999bd955590ae876bce88709747b6d2ed1d5a81258183bbda21b82e5`
- `social-main.js`: 60 lines,
  `5d8ffabbffb03a58d159c1d86136b8924de3e8e4c4f1bbb3cbc12fede58720b8`
- `social/index.html`: 186 lines,
  `0c30c232b06ae92019b441fa8a51b9817b7ade1346a7ddae974dde3be36ac931`
- `package.json`: 34 lines,
  `833e497427cec330e764c539569c72553d10aa5927420530f43add6df5c6e136`
- `.github/workflows/social.yml`: 130 lines,
  `05e2ffb72c4f9360d1959bf903c6b46f021c023979eedb172b5f5f459dd77a65`
- `.github/workflows/security.yml`: 44 lines,
  `7068b02075508dbdbf643e7a68d9acc703703569f4633b34e3cdae4b2581289e`
- `.github/workflows/sbom.yml`: 35 lines,
  `4b00884d9a022ca60a71fe621065b3bef4f19356695ed1962e4232f8ffb795f6`
- `scripts/security-policy.js`: 1,248 lines,
  `6c81b30728b6e90963b49eb51c50f9659b1085b10182319e8bb6dfccbb652b42`
- `scripts/validate-sbom.js`: 94 lines,
  `418f33dca4d85e93a85a584305aec41e15bc97236e2ca1d218fddfac8f637657`

`package-lock.json` remained unchanged at
`7d347693664abe2639dfd1cd04f5f7e8757adb2dd71098b0a23b49aedade5413`.
Reviewer accepted the Electron runtime/CSP and SBOM design for execution, but rejected the
Gitleaks Action coverage and missing Windows build-script path filter. Correction 1 is
authoritative for those two defects; Luna must not execute before its report is appended.
