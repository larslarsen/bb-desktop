# BBD-WAL-006 Store Integration Review 01

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Integrated commit: `b450cd78c9e2e74597a0724741d7d3cade0a55b2`

Result: **SOURCE/TEST GATE EXACT — EVIDENCE CORRECTION REQUIRED**

The seven-path integration commit has the authorized source/test identities and commit message,
is pushed at `HEAD == origin/master`, and leaves a clean index/tracked worktree. Hermes retained
the exact successful Resume 05 results: formatter and Clippy passed cleanly, `zec_store` and
`zec_address` each passed 8/0, and the complete 74-test Node inventory produced the corrected
expected 68/6 partial red with only the six frozen failures.

Store source/test runtime acceptance is supported, but final integration acceptance is deferred
because the two integrated governance paths are not yet exact:

1. `docs/handoff/CURRENT_TASK.md` was replaced with a 19-line summary. The authorized update was
   a state/evidence transition, not deletion of the 446-line historical audit trail at protected
   parent `2d2a52ef`. The parent file has SHA-256
   `2e0a0642ab249a965643ffe476594d805fe4bb200c2f90b3770e3c2337f84dde`.
2. Store Gate Evidence 01 labels an impossible pre-commit time as UTC. It must honestly record
   that the exact command start time was not retained and use the integration commit timestamp,
   `2026-08-31T18:00:17-07:00`, as the recorded completion time.
3. The evidence's “exact execution” section omits the mandatory `env TMPDIR=... CARGO_TARGET_DIR=...`
   prefixes from the four Cargo command lines.
4. The durability test name contains a duplicated `file_`, and viewing-only authority is
   attributed to the binding/reopen test instead of the secret-exclusion test that actually calls
   `open_viewing_context()` and asserts false spending authority.
5. The integration and final-state sections omit the exact integrated commit hash.

Hermes may correct only the evidence and current-task governance files. It must not rerun a gate
command or edit source/tests. The reviewer will inspect the evidence-only correction before final
store acceptance or authorization of the next vertical.
