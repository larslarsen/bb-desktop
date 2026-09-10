# WAL-015 focused runtime diagnostics

AUTHORIZED: reviewer read the three thin helpers/shared transport and pinned the
frozen source. Exact command: `python3 docs/handoff/HERMES_BBD_WAL_015_RUNTIME_01.py`. Hermes alone executes exact matching
driver once in bb-desktop, terminal background=true notify_on_complete=true, then
process.wait <=60s until actual EXIT. No other execution, source corrections,
copying driver, dependencies, Git or user data. Target ext4 confirmed.

Format exact five primary helper paths and run six bounded focused test commands.
Core9, manager3, native2 and transport3 should pass; header/txid2 and preparation
hot-journal1 should expose the already-reviewed core bugs. One ignored crash-child
test is invoked only by its exact owned parent regression, not as a normal test.
Record all runtime outcomes; expected RED requires failure at the intended boundary,
not setup/compile errors. Stop immediately for a compiler error or timeout; do not
repeat all targets against a broken library. No production acceptance claim.

Authorized artifacts: target/wal015-live-runtime-01.json and
 docs/testing/BBD-WAL-015-LIVE-RUNTIME-01.md. Actual metadata and source hashes are
mandatory. All child processes belong to test roots under target; user app/profile
remain untouched. Core stays frozen so the intended negative tests remain red.
