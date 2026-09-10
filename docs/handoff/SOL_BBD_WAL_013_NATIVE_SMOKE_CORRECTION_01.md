# WAL-013 native window smoke source correction — authorized after prior actor closes

Sol gpt-5.6-sol High source-only continuation, exact test/walletAccountWindowSmoke.node.js
only. Python helper accepted source review; do not edit it. No tests/checks/formatters,
Git, evidence or other actors. Existing native UI/runtime actors own disjoint source.
Do not reread broad history; correct these bounded reviewer findings and stop.

1. Prior draft rejected /usr/bin/python3 because it is a symlink; actor removed that
check in final drop. Reviewer confirms OS target /usr/bin/python3.14. Resolve the fixed
system interpreter through fs.realpathSync, validate resulting regular nonsymlink
executable, and spawn that resolved path. Broker/helper strict nonsymlink checks remain.
No PATH/environment-discovered interpreter or dependency change.
2. Explicitly observe broker stdin close as well as stdout/stderr/child. Include it
in normal completion assertion and cleanup guard, remove tracked observers afterclose.
3. After broker status/binding and BEFORE first manage, inspect X11 and assert zero
visible matching windows. This tests initially-hidden behavior without assuming
context registration timing. Keep actual visible/open/hide/reopen/close assertions.
4. Outer Promise.race currently can report45sec failure while entry.fn() continues
creating windows/cleanup in background. Replace this with a cooperative globaldeadline
checked in ongoing wait loops and BEFORE any new request/helper/start action; keep
per-operation deadlines. Ensure finally cleanup is fully awaited before run reports
failure. Cleanup has its own bounded allowance and must not be suppressed by deadline.
Do not expand source scope or weaken checks. Both helper AND broker close must be
confirmed before deleting owned cwd; current helpersClosed guard retained.
Stop with final one-file hash/count. No implementation execution.

Collected baseline Node bc08e5362d2bfb4b093175dd34b08091b90fa2c2f7b23b28397ab52c3750c600 (596lines), Python 040d8214487216e6b91ec514c9473e95cd66811c26306792f5ca7b081696b5d4 (269lines).
