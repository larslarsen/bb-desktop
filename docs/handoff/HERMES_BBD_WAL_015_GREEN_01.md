# WAL-015 broader acceptance
Read and execute ONLY `python3 docs/handoff/HERMES_BBD_WAL_015_GREEN_01.py` once in
bb-desktop via terminal background=true notify_on_complete=true; process.wait<=60
until actual exit. No copying driver, extra commands, dependencies, Git, user data.
Core correction reviewed: actual existing orphan projection, checked per-pool arithmetic,
actual scanned tip+1, readonly connection, no upstream row deletion. Test stays120M.
Format only live.rs/scan.rs; run core9 and eight broader Rust targets, production Clippy.
Falsify genuine interrupted-transaction recovery via exact temporary blanket sidecar
check in preparation, require runtime STATE_CORRUPT, restore exact bytes finally,
then rerun recovery. This proves transaction recovery in addition to actual malformed
preflight RED and RUNTIME02 reorg balance RED. No release or final acceptance yet.
Authorized artifacts exact green01 raw JSON/evidence in driver. All source frozen.
