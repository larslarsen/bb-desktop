# WAL-011 real application/native smoke source contract

Actor Grok grok-4.6 High, source only, no subagents/execution/Git/evidence.
Only new test/walletStartupSmoke.node.js authorized. Read startup contract, main,
resolver, supervisor, model, new startup test for mock style, existing native runtime
test for safe process cleanup. Existing source frozen. This proof completes local
composition after build; it does not require GUI/display or mutate user data.

One async test loads actual social-main.js, real resolver, real supervisor, and
spawns the actual built Rust executable from fixed development app-resources path.
Mock ONLY Electron (app.isPackaged=false, getPath userData to fresh owned temporary
root, sandbox/menu/permissions/window/IPC event facilities). Wrap child_process.spawn
only to RECORD and delegate to original spawn unchanged; restore on every outcome.
No supervisor/resolver/protocol mocks, no runtime self-pin or manifest writes.
Before emitting ready assert no child. Require actual manifest+binary; missing
artifacts fail explicitly, never skip. Emit actual registered ready callback, verify
one spawned actual fixed binary, empty argv/env, cwd userData/wallet-broker 0700,
shell=false, three pipes. Independently compare artifact SHA256 to manifest.
Await actual sanitized subscription snapshot exactly
{v:1,broker:'degraded',accounts:[],intent_preview:null}, timeout 4000ms cleared on
settlement. Invoke actual registered snapshot IPC with real mock main-frame event;
assert same resolved value with distinct identity. account.list must reject exact
UNAVAILABLE; it does NOT return [] in current degraded executable. Invoke real
before-quit event, assert prevented synchronously, app.quit not yet called, then
await recorded child CLOSE and resumed app.quit. Assert close precedes resumed quit;
mock app.quit emits reentrant before-quit and verifies it is allowed. At end there
must be one spawn, no leaked children and no error dialogs. Observe subprocess
error events and fail, do not swallow rejection or count surviving process as proof.

Cleanup on every outcome: request normal quit when possible; if failure leaves
child, explicitly SIGTERM then bounded wait for close, SIGKILL then bounded wait.
Never remove cwd before confirmed close. Restore Module._load, require.cache entry,
child_process.spawn and clear all test timers/listeners. Remove only individually
owned files/dirs with unlink/rmdir (do not recursively delete or follow symlinks).
Preserve compiled artifact and resources. No application user data access.
Export {tests} with one {name,fn}; main-only async runner prints ok/not ok and exits
nonzero on failure. No process.exit before cleanup; use process.exitCode.
Target command later: node test/walletStartupSmoke.node.js.
Falsification of main configured start is already required by startup contract;
additionally this smoke must fail when that call is suppressed if executed as the
selected falsification (bounded timeout, followed by exact restoration).

## Review correction 01 (before any execution)

Initial smoke drop is held for three concrete cleanup defects. Only same test path.
1. waitUntil calls tick synchronously and then starts an interval even if tick already
settled. Add explicit settled state and only schedule if not settled; finish idempotent.
2. The contract requires observed CLOSE, not exitCode/signalCode or a spawn error as
closure proof. All waits/reaping predicates and cwd deletion must require the recorded
close event for every returned child. A spawn error must remain a test failure, never
substitute for close. May avoid signalling an already-exited child while still waiting
for its close. Do not detach error/close observers from an unclosed child; retain cwd
and report cleanup failure if closure cannot be confirmed. Success asserts closed.
3. If primary test AND cleanup fail, append cleanup diagnostic to primary error rather
than discard it (current finally throws testError first). No swallowed cleanup error.
Also select the latest matching degraded snapshot rather than assuming the first
subscription message is already degraded. No production, commands or Git authorized.

Initial test SHA256: cba2feed7acaf2292d8039eac6a9324a76361b3edf6bc6cf1b9062b8e456a727
