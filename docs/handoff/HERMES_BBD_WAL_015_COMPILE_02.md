# WAL-015 compiler pass02 after parser correction

AUTHORIZED Hermes command, once in bb-desktop:
`python3 docs/handoff/HERMES_BBD_WAL_015_COMPILE_02.py`
Read exact driver, terminal background=true notify_on_complete=true, then
process.wait <=60 seconds until actual EXIT. No other commands, source edits,
copying driver, Cargo resolution, Git or user data. Target ext4 confirmed.

COMPILE01 stopped at two test-support missing semicolons after formatting five
production files; Cargo did not run. Primary corrected those semicolons and made
the deep-fork metadata tip hash consistent with its raw TreeState. Both source
actors are frozen. Driver pins70 relevant existing files, formats only the remaining
five changed Rust paths, and compiles the four selected live targets with locked,
offline pinned Rust1.98.0. A separately delegated NEW zec_live_validation.rs test
may be authored concurrently; it is not selected or pinned in this diagnostic pass.
Probe helper remains absent for expected RED. This is compiler diagnostics, not
production acceptance. Authorized evidence is target/wal015-live-compile-02.json
and docs/testing/BBD-WAL-015-LIVE-COMPILE-02.md. Record actual metadata and exits.
