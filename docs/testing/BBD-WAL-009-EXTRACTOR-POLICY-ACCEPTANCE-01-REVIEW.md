# WAL-009 extractor policy acceptance 01 — review

Reviewer: Codex. Local date: 2026-09-12 (capture: 2026-09-13 UTC).
Decision: exact extractor correction and focused acceptance accepted. Global policy red.

All fifteen capture-file sizes/hashes match the actor report. Process metadata retains
literal argv (including the full focused JavaScript), actual exits, sequential UTC times,
PIDs and matching log hashes. Syntax and both focused mutation-test groups exited 0.
The checker and full policy suite each exited 1, as expected. All 23 pre/post inputs
match and independently match the current tree; HEAD is unchanged at
`2b5ad193f01af31f2a2bdcc7b0d1080dd5a2f68e`.

The full log records 88 passed / 4 failed out of 92. Both extractor groups now pass,
including their independent negative mutations. Remaining failed names are exactly:

- committed workflows satisfy the fail-closed checker;
- strict nine-line reviewed Gitleaks ratchet bytes and content are enforced;
- WAL-004 Rust source inventory is exported closed and enumerated by repository policy;
- BBD-WAL-008 closes the hardware target and current fourteen-path WAL-008/WAL-009 ZEC policy inventory.

The first three still encounter the same root-inventory mismatch; the fourth sees the
policy's eight-entry ZEC list against its fourteen-entry fixture. Current ZEC discovery
has sixteen files. No ignore-file or workflow relaxation is warranted by these failures.
Full log SHA-256: `447656f24aed1e7563eaf6ea655c9669c21534e1fc468ac2f3ed67c5b5452372`.

The report's "all gates passed" means its mixed green/expected-red driver completed;
it is not a green repository checker. Runtime version is captured, provider/model are
actor-reported. CURRENT_TASK was left on its prior execution authorization rather than
marked REVIEW PENDING; this review replaces it and closes that handoff.

## Next bounded policy contract

Authorize `docs/handoff/GROK_BUILD_BBD_WAL_009_CURRENT_INVENTORY_01.md` for test-first
changes in only the policy and its test file. Add a distinct strict current-root checker
for all thirteen existing root Rust files, retaining historical seven/eight/nine-file
helper behavior separately. checkRepository must use the strict current checker, never
the permissive historical variants. Extend the current ZEC inventory to the exact
sixteen present files while preserving the historical WAL-006 seven-file inventory.

This is inventory recognition only, not permission to skip scanning or weaken authority
checks. Read-only inspection shows live.rs uses std::net::IpAddr and live_transport.rs
uses reviewed WAL-015 Tokio/service-client code: the existing blanket scanner will need
a separately reviewed, test-first capability contract once inventory dispatch reaches
them. The current root scan also only covers the historical WAL-004 loop; expanded
authority-scan coverage is not silently claimed by this inventory slice. Record these
follow-up boundaries rather than changing predicates to force global green.

No Rust/Monero implementation, endpoint change, dependency/ignore/workflow mutation,
global lint waiver, release or Git integration is authorized. Reviewer performed only
read-only inspection/hash/Git diagnostics. Governance edits: this review, the named
Grok handoff, CURRENT_TASK.md active prefix and tickets/BBD-WAL-009.md status prefix.
