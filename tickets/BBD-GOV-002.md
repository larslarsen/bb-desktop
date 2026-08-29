# BBD-GOV-002 — Adopt SQLite/Keel Test-First Strategy

Status: ACCEPTED

Reviewer: Lead Engineer/Reviewer — Codex

Implementation actor: Reviewer (governance-only publication)

Source baseline: `7512eafcc64914809cbb5ec2bc606d4acca7c1fc`

## Objective

Make test-first development, test falsification, regression coverage, hostile-boundary
testing, security scanning, release SBOM evidence, and a coverage ratchet standing
requirements for future desktop implementation tickets while keeping native packaging
manual.

## Authorized Paths

- `AGENTS.md`
- `TESTING.md`
- `docs/handoff/CURRENT_TASK.md`
- `tickets/BBD-GOV-002.md`

## Acceptance

- The policy requires red-before-green evidence and falsification of important tests.
- It covers unit, property, fixture, failure-injection, Electron security, regression,
  and targeted native testing.
- It requires pinned source, dependency, and secret scanning plus per-platform
  release-time SBOM and package evidence, without packaging on routine pushes.
- No production or test source, dependency, generated state, or packaging behavior changes.
- `git diff --check` passes for the authorized paths.

## Reviewer Acceptance

Accepted as a governance-only testing baseline. No implementation task is authorized.
