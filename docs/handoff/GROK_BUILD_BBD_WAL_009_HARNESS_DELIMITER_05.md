# WAL-009 harness delimiter correction 05

Actor: Grok Build 4.6 High. Governance parent: the commit containing this handoff.
This is a single-delimiter source drop. Do not execute or integrate it.

Read AGENTS.md, TESTING.md, CURRENT_TASK.md's leading active section, this handoff,
and BBD-WAL-009-NATIVE-REVIEW-EVIDENCE-CORRECTION-01-REVIEW.md.
For source inspection, read only the final 30 lines of
wallet-broker/src/zec/test_support.rs and the impl declaration around line 3638.
Do not reload historical records, dependencies, or other implementations.

Only writable path: wallet-broker/src/zec/test_support.rs.
Starting identity: 4358 lines, SHA-256
`21489e5cda159d670fbdd3b96b196bb0e40f8227fa8770a80b6bd7a3e1c23d83`.

The observed compiler diagnostic is an unclosed `impl SignVerifyHarness` beginning
at line 3638, reported at EOF line 4358. The final method is:

```rust
    pub fn invoke_operation_for_test(&self, _operation: &str) -> Result<(), ZecError> {
        Err(ZecError::capability_missing())
    }
```

Append exactly the two bytes `}\n` after the existing terminal newline. Retain
every existing byte. Do not add a blank line, change indentation, reformat the file,
add helpers, alter behavior, modify tests, or address any other diagnostic.

Expected resulting identity: 4359 lines, SHA-256
`461fdd070318cc5f31a29af2cdceaab1d0b63dc23473641b61ea9aafc145cd1b`.
This expected hash was calculated from the existing bytes plus the exact suffix;
it is not evidence of a compiler run.

The initial-red executions are rejected for altered commands and a prohibited
rerun, but their syntax diagnostic is preserved for prerequisite triage. This
mechanical repair does not authorize native production behavior. The six native
regression tests remain frozen; no behavioral test design is needed for adding
only this parser delimiter. A later Hermes handoff will specify fresh execution
and classify the remaining prerequisites separately from intended dialog red.

Allowed: bounded read-only source inspection, source edit, read-only before/after
hash/line-count and exact-prefix comparison. No compiler, test, formatter, Cargo,
rustup, npm, lint, scan, product, network, Git, other document, or other actor.
All other pending source, tests, package files, lockfiles, and evidence are frozen.
If the starting bytes differ, stop without editing. If the resulting identity
differs, report the mismatch and stop; do not widen the change.

Report path, starting/resulting hashes, line counts, exact appended bytes, and
any obstacle, then stop. No polling by the reviewer: collection waits for the
owner to report completion or explicitly request it. Do not launch a duplicate.
