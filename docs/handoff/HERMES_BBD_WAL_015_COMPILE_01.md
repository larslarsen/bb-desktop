# WAL-015 combined source compile and transport RED

AUTHORIZED: both source actors confirmed freeze and reviewer pinned the combined
source in the matching driver. Execute `python3 docs/handoff/HERMES_BBD_WAL_015_COMPILE_01.py`. Hermes alone executes the exact driver once with terminal
background=true notify_on_complete=true in bb-desktop, then process.wait <=60s
until actual EXIT. No other commands, source design/edits outside exact formatting, Cargo resolution,
Git, copying driver, or user profile/running app access. Target is confirmed ext4.

The source follows reviewed initial RED. Compile all four live test targets with
native-ui in one invocation to expose shared source and integration compiler errors.
This first compile is diagnostic, not a green acceptance claim. The transport probe
API intentionally remains absent; its test-specific missing import constitutes RED
only after all shared production compile errors are resolved. Do not characterize
unrelated compilation failures as expected-red success.

Authorized artifacts: target/wal015-live-compile-01.json and matching docs/testing
record. Exact command uses pinned Rust1.98.0, --locked --offline, no-default-features,
features native-ui, test zec_live_sync/account_management/account_native_ui/
zec_live_transport, --no-run, 360-second whole-process-group bound. Record actual
metadata and input/final hashes. Stop on source drift or timeout. Return all compiler
errors to reviewer; Hermes must not fix source itself.

The exact driver additionally formats only its enumerated ten changed Rust paths
through pinned rustfmt stdin/stdout before compilation, recording input and formatted
hashes. This is authorized formatting only; do not recursively format modules or
change any test meaning. A formatter parse failure is diagnostic and stops the pass.
