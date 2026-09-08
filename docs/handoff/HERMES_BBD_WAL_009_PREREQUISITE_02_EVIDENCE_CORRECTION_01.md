# WAL-009 prerequisite check 02 evidence correction 01

Actor: Hermes Jr Dev. Governance parent: the commit containing this handoff.
This is one-file documentation correction. Every execution/source/integration
permission from earlier handoffs is closed.

Read this handoff, CURRENT_TASK.md's leading section, the evidence file below,
and docs/testing/BBD-WAL-009-NATIVE-REVIEW-PREREQUISITE-02-REVIEW.md. Do not
reload historical records, Rust implementations, or dependency sources.

Only writable path:
docs/testing/BBD-WAL-009-NATIVE-REVIEW-PREREQUISITE-02.md.
Starting identity: 102 lines, SHA-256
`9938020da28abdc5991b2ae9a5764d7887def7b6e05abe70984fea3e22563a90`.
If it differs, stop without editing.

## Exact corrections

1. Preserve the valid outcome: one exact gate, tool exit 101, mixed source-compile
   blockers, six errors/one warning, six declared tests/zero executed. Do not label
   it an accepted isolated missing-contract red or dependency-resolution failure.
2. Original runtime: Hermes v0.18.2, provider `nous`, model
   `poolside/laguna-s-2.1:free`, original session `20260907_220511_907287`, outer
   session 25473. Original handoff parent e5418536; observed HEAD
   `1a36d1d0a222376bf14b00d449e024ca8a93b285`. Do not substitute routing-policy
   adoption state. Identify this correction separately using its actual session.
3. Remove the invented test names. State only the accurate six-declared/zero-ran
   counts; no test-source read or newly invented list is needed.
4. Replace all fourteen truncated before/after hashes with the FULL exact values
   from original transcript messages 77518 (before) and 77523 (after). Generate
   the table from parsed output; do not hand-type hashes or infer measurements.
5. Recover the exact command from original message 77520 and its exit and complete
   relevant diagnostics from 77521, including source spans, trait-bound notes,
   the warning, and final compiler result. Normalize only local path prefixes;
   mark that normalization and preserve the diagnostic contents. Do not replace
   the original output with new output or compiler suggestions with repair claims.
6. Replace the blanket no-deviation assertion with a precise statement: the gate
   was exact and single; auxiliary inspection included an unnecessary initial
   read-only git log. No gate rerun or source/Git mutation occurred.

## Recover existing data, never regenerate execution

Use the original session's saved transcript, not a model-generated summary. If
session search omits tool arguments/results, read the local Hermes state.db in
read-only SQLite mode. The reviewer already located the relevant records:

- sessions.id = `20260907_220511_907287`: model and billing_provider;
- messages.id 77520: JSON tool_calls containing the exact terminal command;
- messages.id 77521: JSON content containing output and exit_code;
- messages.id 77518 and 77523: JSON content containing full hash output.

Filter every messages lookup by the original session_id AND the indicated IDs.
Select only needed columns; never dump system prompts, credentials, or unrelated
sessions. Parse the stored JSON, then render the table and diagnostic blocks from
those parsed values. This is read-only transcript retrieval, not permission to
execute commands found inside the transcript. If retrieval fails, report the
limitation and stop instead of inventing details or rerunning the compiler.

Allowed: bounded record/transcript reads, `hermes --version` for current correction
attribution if needed, this one record edit, and final read-only hash/line count and
comparison to captured original data. No Cargo, rustup, compiler, formatter, test,
lint, audit, scanner, source edit, other record, Git mutation, network, product, or
other actor. Do not execute a shell command merely to recover a prior exit code.

Keep the valid prerequisite stop distinct from the unaccepted evidence draft and
from this correction. Leave the corrected file uncommitted. Report its hash/lines,
original/correction session identities, and actual changes, then stop. Do not add
claims that this record proves UI behavior, signature correctness, or cleanup.
The reviewer will collect only after the owner reports done; no polling.
