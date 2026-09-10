# BBD-WAL-013 — Native account management

Status: COMPLETE — empty-window correction verified on the actual desktop and pushed.
Correction source: 25113d5dbf9bc72c058c9f22e9398afde1576f24.
Correction evidence: f2973f0931c978eb47f3985a16dc606b0bb0e120.
Owner confirmed the normal application opens the account window through
Wallet → Manage accounts after finding the top menu.

The original acceptance missed two real desktop failures: winit Wayland hide/show
is unsupported, and the NVIDIA host failed native GLX initialization with
GLXBadFBConfig. Both JS launch allowlists now omit WAYLAND_DISPLAY, and the
supervisor adds fixed LIBGL_ALWAYS_SOFTWARE=1 and __GLX_VENDOR_LIBRARY_NAME=mesa
on Linux with a nonempty sanitized DISPLAY. Ambient GPU variables cannot override
these values. The native wallet requires XWayland and system Mesa software GLX;
Electron launch flags and dependencies were not changed.

The corrected native smoke inspects real rendered pixels on Xvfb and the owner's
host XWayland display, identifies the client by the owned broker PID, excludes
GNOME decoration frames, and verifies first open, hide, reopen, normal EOF and
child/stdio cleanup. Reviewer inspected both actual-host screenshots: Create account
and Restore backup are visible. Captures share SHA256
070924bb3e51e5cc75dcd93c5866057751475b9e3353c1e6d53fc0355727ae25.
The 27 other focused JS groups passed. Blank-content and fixed-renderer-setting
falsifications were detected and restored. Native binary remains unchanged.
Directory and committed-content Gitleaks scans found no leaks; six inherited policy
failures are unchanged. See docs/testing/BBD-WAL-013-BLANK-WINDOW-GREEN-02.md and
docs/testing/BBD-WAL-013-BLANK-WINDOW-INTEGRATION-01.md. Reviewer independently
verified all 12 correction commit blobs and preserved unrelated edits. All actors
exited. No further implementation is authorized.

Initial implementation record follows.
Source commit: b2e160634147c00023b258ba523dacf70d97f0f1.
Integration evidence: 8b5a60fe7d0f861f063124f2ae53a72e5fba2f70.

Wallet → Manage accounts opens a native window for Zcash TESTNET software accounts.
Users can create encrypted accounts, list, unlock, manually lock, export encrypted
backups, and inspect then explicitly confirm restore. Idle expiry locks sessions;
closing the window clears private state and locks accounts. The broker survives
window close and can reopen it; normal application quit closes it through EOF.
No seed/address export, mainnet, network, payment, signing/broadcast, hardware, or
XMR onboarding was enabled. All payment capabilities remain disabled.

Reviewer accepted 103 distinct Rust tests and 104 JavaScript groups across focused
records, including actual persisted accounts, native widgets, application startup,
and staged native window open/hide/reopen/EOF. Origin, idle deadline, restore
confirmation, menu dispatch, EOF, and native opening suppression were falsified,
detected, restored, and verified green. Native Clippy and no-default binary check
passed. Final redundant manifest registration was removed; all 12 account tests
remain discovered. No dependency changes were needed.

Final evidence: docs/testing/BBD-WAL-013-FINAL-WINDOW-SECURITY-03.md and -04.md,
plus docs/testing/BBD-WAL-013-INTEGRATION-01.md. Intermediate failed runs are retained
and superseded by the described corrections, not credited as passing evidence.
Final staged Linux x64 binary SHA256:
929751c8f956c8e764466279b9adb4ab8007c391206dd6478b18f1794853a11c.
Npm audit found zero vulnerabilities; Cargo audit had no vulnerability failures
and retained the existing atomic-polyfill unmaintained warning. Pinned Gitleaks
working-directory and committed-history scans found no leaks.

Release remains blocked by the six inherited security-policy failures and Rust
source inventory mismatch. This task did not repair those unrelated policy edits,
build installers, or claim full release/all-feature/proving validation. Native
framework startup may briefly map a blank first frame; the initialized account
window is verified hidden before Manage accounts. File-picker portal and live-account custody QA were not performed; the owner
confirmed the normal application menu opens the account window. Unrelated npm/policy edits and two
historical evidence drafts were preserved.

Reviewer independently verified all 34 integrated source/test/evidence blob hashes,
current input pins, actual Hermes execution closure, exact Git scope, and successful
source/evidence pushes. Source actors used documented Grok-to-Sol escalation;
Hermes owned test execution, integration, evidence, commits, and pushes.
