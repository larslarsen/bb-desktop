# WAL-015 combined compiler pass03 and missing-helper RED

AUTHORIZED exact command once in bb-desktop:
`python3 docs/handoff/HERMES_BBD_WAL_015_COMPILE_03.py`
Hermes reads this and exact driver; terminal background=true notify_on_complete=true,
then process.wait <=60s until actual EXIT. No other commands/source corrections,
copying driver, dependencies, Git, or user data. Target ext4 confirmed.

COMPILE02 source compiler errors were reviewed. Core corrected six BlockHash slice
accesses; primary corrected its BlockHash access, Merkle generic bounds, Linux
metadata methods and unused import. All source actors froze. Reviewer read the two
new validation tests and one real crash-recovery test plus ignored owned child.

Driver pins72 relevant files, formats only the four newly changed files, then
compiles six selected targets. Expect missing probe_live_transport_for_test,
validate_live_batch_for_test and LiveSyncHarness::reopen_via_preparation. Only those
errors qualify as expected helper RED; other failures return for correction. No
production acceptance claim. Exact artifacts: target/wal015-live-compile-03.json
and docs/testing/BBD-WAL-015-LIVE-COMPILE-03.md. Record actual metadata and pins.
