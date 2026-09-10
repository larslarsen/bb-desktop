# Current Task

BBD-WAL-014 is IN PROGRESS. Focused red accepted: only missing receive APIs.
Sol gpt-5.6-sol High owns production under SOL_BBD_WAL_014_RECEIVE_PRODUCTION_01.md
(four production files and two narrow cleanup-helper refinements). No execution/Git.
Owner reconfirmed the original broker-with-built-in-Zcash
architecture and authorized continuing receive/balance work. Grok may author ONLY
the two test paths in GROK_BBD_WAL_014_RECEIVE_TESTS_01.md. Production is frozen
until reviewed red. Native receive uses existing adapter; balance remains explicitly
unavailable until live sync exists. See tickets/BBD-WAL-014.md. No user profile access.

Previous completed task:

BBD-WAL-013 native account management and actual-desktop empty-window correction
are COMPLETE. Correction source 25113d5dbf9bc72c058c9f22e9398afde1576f24 and evidence
f2973f0931c978eb47f3985a16dc606b0bb0e120 are pushed to origin/master.
Owner confirmed opening the window through Wallet → Manage accounts.

Broker uses X11/XWayland plus fixed Mesa software GLX on Linux. Actual host and
Xvfb rendered controls, hide/reopen/normal EOF and process cleanup passed; reviewer
inspected both host PNGs. Blank-render and renderer-setting falsifications passed.
Reviewer independently checked the exact 12 correction commit blobs and input pins.
All actors exited. No background work or further source authoring is authorized.

See tickets/BBD-WAL-013.md and the BLANK-WINDOW-GREEN-02 / INTEGRATION-01 evidence.
Six inherited policy failures remain release blockers. Four unrelated npm/policy
edits and two historical evidence drafts are preserved. No installer/driver changes.
