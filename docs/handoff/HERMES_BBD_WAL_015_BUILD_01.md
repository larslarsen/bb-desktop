# WAL-015 runnable app validation
Execute ONLY `python3 docs/handoff/HERMES_BBD_WAL_015_BUILD_01.py` once in bb-desktop,
after GREEN01 accepted. Read exact driver; terminal background=true notify_on_complete=true,
process.wait<=60 until actual EXIT. No other commands/source/dependency/Git/user-data work.
Trusted cargo PATH correction for existing native build command already proven WAL014.
Rebuild/stage native binary, verify manifest/hash, run four JS groups plus native window
smoke Xvfb and actual DISPLAY :0 using owned empty profile/PID only. Preserve user app.
Record actual new screenshot hashes. Run policy ratchet exactly6 inherited failures,
pinned Gitleaks dir, diff check. Stop on any additional failure. No packages/installers.
Authorized raw/evidence and stagedbinary/screenshots are exact paths named in driver.
