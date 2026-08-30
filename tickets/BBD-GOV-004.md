# BBD-GOV-004 — Correct Jr Dev Role to Codex Luna

Status: ACCEPTED

Reviewer: Lead Engineer/Reviewer — Codex

Implementation actor: Reviewer (governance-only publication)

Source baseline: `dea71e1996445a7e4c118307edc3afe9f33a12df`

## Objective

Correct the obsolete Hermes Jr Dev label to Codex Luna using `gpt-5.6-luna` while
preserving the established integration and acceptance boundaries.

## Authorized Paths

- `AGENTS.md`
- `TESTING.md`
- `docs/engineering/DEVELOPMENT_ROLES.md`
- `docs/handoff/CURRENT_TASK.md`
- `tickets/BBD-GOV-004.md`

## Acceptance

- Codex Luna owns integration, command execution, evidence, and developer-drop Git.
- It does not design or author tests.
- No implementation, dependency, package, generated artifact, or CI behavior changes.
- `git diff --check` passes.

## Reviewer Acceptance

Accepted as a governance-only role correction which supersedes the Jr Dev naming in
`BBD-GOV-003` without rewriting that historical record.
