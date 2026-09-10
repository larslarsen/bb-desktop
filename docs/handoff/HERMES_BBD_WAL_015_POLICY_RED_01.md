# WAL-015 exact manifest policy RED
Execute ONLY `python3 docs/handoff/HERMES_BBD_WAL_015_POLICY_RED_01.py` once in bb-desktop.
Read exactdriver, terminal background=true notify_on_complete=true, process.wait<=60
until EXIT. No extra commands/source edits/Git/dependencies/user data. New33groups,
expect exactly1RED for currentmanifest unrecognizedpins; negative mutation checks must
pass. Source frozen. Exact raw/evidence artifacts/pins in driver. No Rust rerun/build.
