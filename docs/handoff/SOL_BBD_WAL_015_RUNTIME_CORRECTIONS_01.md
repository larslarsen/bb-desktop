# WAL-015 corrections after actual runtime tests

Reviewer inspected RUNTIME01 actual Hermes session20260910_153027_73314d, CLI exit0,
all final source pins and actual command outputs. The runner's verbal counts were
wrong: core3passed/6failed/0ignored; manager3passed; native2passed; transport2passed/
1failed; validation0passed/2failed; recovery0passed/1failed/1ignored. No compiler
errors/timeouts. Use raw evidence, not the verbal summary. Formatting is authorized
and recorded, not source drift. No production acceptance yet.

Core actor owns live.rs only: implement actual header/explicit hash and previous-hash
agreement and txid32-byte preflight; both new regressions failed at unwrap_err(Ok).
Remove unused StoredState target_hash field without removing its DB column/checks,
and unused run_live_sync wrapper; coordinate reexport removal with primary.

Primary owns test_support.rs/live_transport.rs/zec.rs: recorded source must load
full canonical sequence through107, preserving the fixture's106 confirmation height
in its LocalNetwork binding. Reorg replaces final107, not106. Mutate actual TreeState
hash rather than replacing with zeros (birthday checkpoint hash already equals zero),
so first malformed response reaches the validator with an actual difference.
Tonic0.14.6 codec/decode.rs returns OutOfRange on decoded-size rejection; map this
fixed status to LIMIT as well as ResourceExhausted. No test assertions are weakened.

Hot-journal test actually passed crash/recovery and exact-byte restoration, then
failed only on the truncated fixture target106vs107. Preparation ordering had already
been corrected in the COMPILE03 baseline, despite the core actor's earlier frozen/
only-six-compiler-fixes report. Core hash is identical in COMPILE03 final and RUNTIME01
input/final, so runtime evidence is valid. Reviewer requests an accurate explanation
of the extra change; do not revert working recovery. A later temporary falsification
of the old blanket sidecar guard will prove the real recovery regression detects it.

Source-only corrections; no execution, formatting, dependencies, Git or evidence.
Return coherent frozen drops for a second focused runtime pass. Existing broad tests,
production Clippy, falsification, native rebuild/window and security remain pending.
