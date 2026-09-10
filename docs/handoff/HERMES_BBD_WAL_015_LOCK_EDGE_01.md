# WAL-015 existing dependency edge resolution

Hermes alone executes exactly `python3 docs/handoff/HERMES_BBD_WAL_015_LOCK_EDGE_01.py`
once in bb-desktop, terminal background=true notify_on_complete=true, then
process.wait <=60 seconds until actual exit. Read this handoff and its exact driver.
No other execution, source changes, copying driver, formatting, Git or user data.
Authorized writes: Cargo.lock, target/wal015-live-lock-edge-01.json and matching
docs/testing evidence. Target is confirmed ext4. Baseline and manifest are pinned
inside driver. Resolve only the new direct edge to already audited
incrementalmerkletree0.8.2 legacy-api. Driver verifies every transitive package,
version, checksum and dependency list unchanged; this preserves AUDIT01 coverage.
Stop on drift or failure. Collect actual session/provider/model and process exit.
