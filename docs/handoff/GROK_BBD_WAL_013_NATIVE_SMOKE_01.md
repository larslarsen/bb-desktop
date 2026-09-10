# WAL-013 actual native-window proof — pending source authorization

This contract fixes follow-on QA, not authorization for any currently running actor.
After current test actors stop, reviewer may authorize Grok source-only in precisely
NEW test/walletAccountWindowSmoke.node.js and NEW test/fixtures/wallet-broker/x11-window.py.
No production edits. Tests execute only through Hermes after reviewer source review.
Read AGENTS.md,TESTING.md, original service/UI/runtime contracts, supervisor/protocol,
actual-child harness from test/walletBrokerRuntime.node.js and startup smoke cleanup.

Goal: prove actual staged native-feature Rust binary and real supervisor open/hide/
reopen the actual eframe account window on a private Xvfb server. Complements actual
widget->real AccountManager persistence test; does not claim OS input-buffer erasure,
real file-picker portal QA or full Electron installer/release acceptance.

Node test accepts optional absolute broker binary arg like existing runtime suite;
default fixed staged target/app-resources/wallet-broker/bitbook-wallet-broker. Verify
regular nonsymlink and compute actual pin, real createWalletSupervisor, real spawn.
Owned unique root under wallet-broker/target mode0700; no real profile/data touched.
Fixed child env only DISPLAY and XAUTHORITY from isolated test Xvfb,
LANG=C.UTF-8, PATH=/usr/bin;
no hostbus/credentials/backend override. Capture diagnostics privately, redact failures.
Native window title exactly BitBook accounts. Helper never handles passphrases/seeds.

Python helper uses ONLY stdlib ctypes for installed libX11.so.6; no new dependency,
unsafe Rust, shell, network, xdotool installation or clipboard. Modes inspect and
close with one validated positive integer window ID. Connect current DISPLAY, bounded
XQueryTree recursion <=4096windows, XFetchName and XGetWindowAttributes; report JSON
matching-title windows with id and map_state (IsViewable=2). Proper C arg/restype,
free allocated X data and close display on every path. close sends WM_PROTOCOLS /
WM_DELETE_WINDOW ClientMessage via XSendEvent/XFlush to actual discovered ID. Reject
unknown args, absent display, zero/overflow IDs. Never sends keys or touches host X.
Do not fake exit success on X errors or helper failures.

One real lifecycle group:
1 start actual supervisor, await bound <=5sec, account.list=>[],degraded status.
2 wait for GUI initialization by bounded attempts account.manage every100ms <=15sec
(UNAVAILABLE allowed only while context registers). Once accepted must observe exactly
one visible titled window <=5sec. No success merely because broker survives/responds.
3 helper close actual window, observe no visible titled windows <=5sec, then actual
account.list still[] and supervisor bound. Child must still be running.
4 repeat account.manage and observe window visible again (same broker PID/norespawn).
5 supervisor.shutdown; observe actual child close before promise completion, native
window absent, no forced signals on healthy EOF path. Capture signalcalls by record+
delegate real child.kill and assert[] for normal shutdown. Exit0, no protocol/frame
leak/canary/diagnostic success substitution. Assert pendingrequests0 and no postclose
open/focus event. Cleanup own listeners/timers, close all helperchildren, unlink only
owned files then rmdir accounts/root. If shutdown fails, bounded SIGTERM thenSIGKILL,
observe close before cleanup, never silently succeed or delete live cwd. Test timeout
<=45sec with compound cleanup failures preserved. No recursive filesystem deletion.

Hermes exact command AFTER native-feature build/stage and source review:
xvfb-run -a -s '-screen 0 1024x768x24' node test/walletAccountWindowSmoke.node.js
The private X server is owned by xvfb-run and must be reaped on completion. No GUI app
shown on user's desktop by this QA. Existing Xvfb and Python ctypes available, actual
libX11 path checked by execution; do not install tools without a follow-on contract.
Falsification: in Rust runtime account.manage keep a successful {} response but suppress
control.request_open only. Rebuild/stage, this real-window test MUST fail to observe
visible window. Restore exactsource/rebuild/restage then same testpass. This separates
real GUI effect from a plausible protocol reply. Exact mutation located by reviewer
in final production source; Hermes may execute only pinned bounded mutation/restoration.
