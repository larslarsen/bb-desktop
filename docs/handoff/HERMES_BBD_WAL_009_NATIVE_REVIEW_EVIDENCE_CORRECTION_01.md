# WAL-009 native review evidence correction 01

Actor: Hermes Jr Dev. Governance parent: the commit containing this handoff.
This is documentation-only. Initial-red execution permission is closed.

Read this handoff, the leading CURRENT_TASK section, and
docs/testing/BBD-WAL-009-NATIVE-REVIEW-EXPECTED-RED-01-REJECTION.md.
Edit only docs/testing/BBD-WAL-009-NATIVE-REVIEW-EXPECTED-RED-01.md, starting hash
`a567b4fdab9b4f164d06ba3cd4d7d009063563a1297b842593c0ac50efbee2ad`.

Required corrections:

1. Lead with REJECTED RUN / PREREQUISITE-BLOCKED and link the rejection. Remove
   every assertion of exact-command compliance, single execution, and no rerun.
2. Identify the original execution as session `20260907_211536_310238`, outer
   session 24886, Hermes v0.18.2, provider nous, model poolside/laguna-s-2.1:free.
   Preserve its original governance parent 7afe2800. Identify this correction
   separately; do not replace the original execution attribution.
3. Preserve the authorized command as the contract, explicitly distinguish it from
   both actual command strings recovered from your transcript, and record each
   result. The first appended ` 2>&1` and returned 101. The second added that
   redirection plus a shell echo of EXIT_CODE=$?; the compiler failed with 101 but
   the tool returned 0 after the echo. Copy the actual strings faithfully, including
   the shell quoting. Disclose the prohibited rerun and read-scope deviations.
4. Preserve the unclosed-delimiter diagnostic and six-declared/zero-executed counts.
   No missing-dialog, Context::run, or input-behavior failure was observed. The
   diagnostic is useful for triage; neither run is accepted execution evidence.
5. Correct Cargo.toml to 122 lines. Replace the malformed native.rs table hashes
   with `992138f18cbb0b969296109de3186aab923500c41ed2c0d41cf9c0696d3c3acc`.
6. Disclose that the lock-sync evidence was omitted from the original hash command.
   Its reviewer-observed current hash matches the earlier acceptance:
   `ab25ffd2ac6ec7c26f7c21e42978697ec10da7ff395a9569afd9e78d1cd27ed2`.
   Do not label this an original actor pre/postflight measurement.
7. Replace avoidable local absolute paths in prose/compiler output with relative
   paths or a labeled normalized repository prefix. Preserve the exact execution
   command paths where required to document the command.

Allowed: read-only inspection of these records and the original transcript, editing
this one evidence file, and read-only hash/line-count/diff verification of the result.
No Cargo/rustup/compiler/formatter/test/lint/scanner command, source edit, other
document edit, Git mutation, network, product, or other actor. Do not rerun anything
to recover old output. If transcript retrieval fails, report that limitation and
stop instead of inventing details. Report resulting path, hash, line count, and
correction summary, then stop. Leave evidence uncommitted for reviewer review.
