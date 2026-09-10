# WAL-015 focused green after actual runtime corrections

AUTHORIZED exact command once in bb-desktop:
`python3 docs/handoff/HERMES_BBD_WAL_015_RUNTIME_02.py`
Hermes reads this and exact driver, terminal background=true notify_on_complete=true,
then process.wait <=60s until actual EXIT. No extra commands/source edits, copying
driver, dependencies, Git or user data. Target ext4 confirmed. Source frozen/pinned.

Reviewer read runtime corrections: actual header/txid preflight, full canonical107
fixture/reorg, distinct malformed TreeState hash, and tonic OutOfRange size mapping.
Core removed two unused items, preserving checkpoint schema. Format exact four paths
and run core9, transport3, validation2 and recovery1 (plus ignored child). All must
pass; stop on first failure. RUNTIME01 manager3/native2 green remains applicable and
will be covered by required broader full targets. No complete/release acceptance yet.

Hot-journal assertions already passed in RUNTIME01; final target mismatch was fixture
106vs107. Actor clarifies the journal ordering correction belonged to its earlier
pre-freeze refinement; its subsequent acknowledgement of the old defect was stale.
Actual COMPILE03/RUNTIME01 core bytes agree. Preserve working recovery, then falsify
the old blanket guard in later authorized verification and restore exact bytes.

Authorized artifacts: target/wal015-live-runtime-02.json and
 docs/testing/BBD-WAL-015-LIVE-RUNTIME-02.md. Record actual metadata and hashes.
