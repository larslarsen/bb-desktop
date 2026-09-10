# WAL-015 test-source authorization

You ARE the appointed Principal Dev Sol, gpt-5.6-sol High; parent remains reviewer.
Read AGENTS.md, TESTING.md and tickets/BBD-WAL-015.md. The owner delegated product
defaults; there is no pending owner question. Escalation is documented in the ticket.

Author ONLY the three authorized test files before any production change. Reuse
existing fixtures and native/account helpers. No execution, formatter, Cargo changes,
Git, evidence, other actors or user data. Do not invoke tests even to inspect errors.

Define a compact Rust API for the fixed semantics in the ticket via test usage. It is
acceptable that those imports/methods do not exist yet: Hermes obtains the red result.
Core fixture tests must use the same intended production scan loop and real WalletDb,
not a separate fake scanner or a test-only balance return. Test support may expose a
thin local-consensus harness later, but it may not duplicate production mechanisms.
Use existing recorded compact bytes and upstream decoding APIs already available.
Native mock ports can model job progress; service and engine tests must prove actual
account binding, worker cancellation and durable nonzero scanning independently.

Keep the suite focused (roughly 6-10 core tests, 2-3 account tests and 2-3 UI tests),
with grouped boundary cases. Include actual path/owner protections and exact cleanup
of owned files; no recursive removal or following symlink parents. All test artifacts
under wallet-broker/target. Don't add execution-only gRPC dependencies in this phase;
transport boundary tests follow in the same ticket after dependency review.

Report edited paths, the proposed API signatures and any concrete contract conflict.
Read focused spans, write one coherent drop, stop. Do not re-read whole large files
or make cosmetic iteration passes; Hermes owns formatting.
