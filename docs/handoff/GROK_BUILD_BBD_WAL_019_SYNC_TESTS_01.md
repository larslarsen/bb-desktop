# Grok Build — WAL-019 native Sync regression tests

You are Sr Dev Grok Build 4.6 High, manually relayed by the owner. Read `AGENTS.md`,
`TESTING.md`, `docs/handoff/CURRENT_TASK.md` and `tickets/BBD-WAL-019.md`.

Author the WAL-019 test source only in `wallet-broker/tests/account_native_ui.rs`.
Verify the ticket's exact HEAD and seven working-tree pins before editing. Preserve
the existing lint corrections and every unrelated working-tree change.

The reviewer fixed the UI semantics in the ticket: editable server/disclosure
before a single list Sync click; separate Balance navigation; no implicit network
work; selected-account/job binding; endpoint retention within this window; safe
failure and explicit retry; small-window usability. Use the real egui pointer path
and call-count assertions. New tests must compile and expose the missing behavior.

You may execute and repeat the ticket's exact rustfmt and focused offline test
commands while fixing your own test-source format/compiler errors. Expected red
is allowed and required for missing production behavior. Do not edit production
or turn the red green by weakening the tests. No integration/evidence documents,
broader tests, other actors, user wallet data, network, Git, commits or pushes.

Return changed paths, SHA-256 and line count; test names and invariant coverage;
exact focused commands, exits and actual test counts; intended failures versus
infrastructure errors; any blocked contract issue. Stop for Codex test-source review.
Hermes receives formal-red authorization only after that review.
