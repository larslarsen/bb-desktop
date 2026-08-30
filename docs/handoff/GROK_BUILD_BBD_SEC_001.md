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
