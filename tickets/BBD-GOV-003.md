# BBD-GOV-003 — Establish the Complete Agent Role Workflow

Status: ACCEPTED

Reviewer: Lead Engineer/Reviewer — Codex

Implementation actor: Reviewer (governance-only publication)

Source baseline: `80ac4ca6ffcd807a9846efd5717d64850ab77cc0`

## Objective

Add the missing Codex Spark implementation and Hermes integration roles and correct the
Grok Build and reviewer boundaries.

## Authorized Paths

- `AGENTS.md`
- `TESTING.md`
- `docs/engineering/DEVELOPMENT_ROLES.md`
- `docs/handoff/CURRENT_TASK.md`
- `tickets/BBD-GOV-003.md`

## Acceptance

- Codex Spark owns bounded mechanical source and test-source authoring.
- Grok Build owns bounded senior source and test-source authoring.
- Hermes owns integration, command execution, evidence records, and developer-drop Git.
- Reviewer acceptance and next-ticket authorization remain exclusive.
- No source, dependency, CI, generated artifact, or client behavior changes.
- `git diff --check` passes for the authorized paths.

## Reviewer Acceptance

Accepted as a governance-only role correction. No implementation task is authorized.
