# Grok Build — WAL-019 test correction 01

Actor: Sr Dev Grok Build 4.6 High, manually relayed by the owner. Read AGENTS.md,
TESTING.md, CURRENT_TASK.md, tickets/BBD-WAL-019.md and
docs/testing/BBD-WAL-019-TEST-SOURCE-REVIEW-01.md. This supersedes the first test
handoff as active routing. Codex has not accepted the test drop.

## Exact starting point

HEAD: `2b5ad193f01af31f2a2bdcc7b0d1080dd5a2f68e`. For this correction only,
replace the ticket's original test-file pin with:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/tests/account_native_ui.rs` | 2581 | `d69962abb680ea7e9a1fee31b691928bdfc9e67e50ff1ec5c14d3e705a9110ca` |

Verify all other six original ticket pins unchanged. Preserve all unrelated
working-tree changes, including the pre-existing test lint corrections.
Only `wallet-broker/tests/account_native_ui.rs` is writable.

## Required corrections

1. Fix the repeated-click test so it does not look up an exact Sync label after
   entering the running view, where Sync may correctly be absent. Capture the
   real list button geometry before starting, perform the first pointer click,
   observe its actual start call, then replay a second pointer click at the
   captured position. Assert no second start across that click and subsequent
   frames. A disabled or hidden Sync control is valid. Do not impose a new
   production requirement that the running screen retain a Sync button.
2. Inject `RAW_ERROR_CANARY` into `sync_start_results` for the start-error case.
   Prove safe visible feedback and no leaked canary, plus no automatic retry and
   exactly one additional start after the next explicit retry.
3. Add a `wal019_` regression that really attempts to edit the server field while
   a known account/job is running. Use pointer focus, select-all and text input;
   assert the displayed endpoint is unchanged and the start-call record still
   contains exactly the original account/endpoint. Observe the same job's progress
   and cancellation binding. After explicit cancellation, edit the field, observe
   the new endpoint without any start, and show the next explicit Sync sends that
   endpoint exactly once. Use fake port jobs and real AccountWindow UI events.

Keep the existing independent old and new assertions. Do not author production,
replace behavior tests with source-text assertions, weaken tests to turn red green,
or change helper behavior globally to hide missing widgets. Current production is
still expected to fail on the absent list controls; correct source must also be
capable of passing once the specified UI is implemented.

## Focused execution and return

From bb-desktop, verify the existing `wallet-broker/target` is disk-backed, then
use the same authorized commands. You may repeat them to repair your own test
format/compiler errors. Expected missing-feature assertion failures remain red.

```bash
rustup run 1.98.0 rustfmt --edition 2024 wallet-broker/tests/account_native_ui.rs
timeout --signal=TERM --kill-after=10s 600s rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --features native-ui --test account_native_ui wal019_ -- --nocapture
```

Return the changed path, line count and SHA-256; a correction-to-test mapping;
exact commands, exits, selected/passed/failed test counts and actual failure
messages. Distinguish compilation/infrastructure errors from expected behavioral
red. No broader tests, Hermes execution, evidence-file edits, other actors,
production edits, network, wallet data, Git, commits or pushes. Stop for Codex review.
