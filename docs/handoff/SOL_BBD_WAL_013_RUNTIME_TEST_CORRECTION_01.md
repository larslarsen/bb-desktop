# WAL-013 runtime test correction — Sol escalation

Grokcecfa62a outer80082 stopped exit1 Max turns reached without completed usable drop.
Reviewer found native startup fixture race, sync/rejection mismatch and incomplete
cleanup/env proofs. AGENTS permits Sol gpt-5.6-sol High fill-in; no subagents.
ONLY writable four test paths in pins below. Source-only no execution/checks/formatters,
production, other tests, Git/evidence/actors. Read AGENTS,TESTING and original
GROK_BBD_WAL_013_RUNTIME_01.md, these tests and relevant existing production APIs only.
Avoid broad research; precise correction below. Account/UI actors own Rust paths.

Pins:
- test/walletAccountManagement.node.js: 1e112c17a8f554d58b1d18ab916c7cecac15f681f8fef1a40ab8e108668c1e85
- test/walletBrokerRuntime.node.js: a58c8b9c9e16541d4ba8f33db356ab2bfb62a50439fdc6b47c7456d32f492f06
- test/walletStartupSmoke.node.js: 0fcfdd53ed6660cc712449a4fd0d12d072056d382baa15b8284d28b597437dee
- test/walletSupervisorShutdown.node.js: b1bd7fb8f3aa5113092c98137ecfa74f50f8be94234b3a47976e107a4d607e9b

1 walletBrokerRuntime newly added assert.rejects callbacks call synchronous validating
supervisor.dispatch. Wrap in async ()=> or use assert.throws for sync schemas, preserving
async unavailable brokerreply proof. Do not change production to fit wrong fixtures.
2 withDirectChild currently spawns BEFORE writeLockedAccounts creates accounts directory:
new production open may win mkdir race. Add bounded setup callback run in owned0700cwd
BEFORE real spawn; install all persisted/hostile fixtures there. No retries maskingrace.
3 Preserve prior always-on cleanup inventory assertions: old withTempCwd rejected any
unexpected write. New collector silently skipsIOerrors and deletesarbitrarytrees. Track
explicit fixture-owned paths/IDs via harness registration; ordinary wire tests may only
create empty accounts0700. Before cleanup assert exact allowed inventory (root/accounts,
registered fixture files), no unexpected secret/plaintext/nested directory. Fail+retain
unexpected artifacts for review. Unlink individually known files and dirs reversed,
never follow symlinks or recursive-delete arbitrary trees. Missing/readerrorsfail except
knownabsentcleanup paths. Cleanup must wait observed child CLOSE and closedstdio, not
merely exit, including failedtest paths. Own listeners/timers removed afterclose; if
notclosedretaincwd and compounderror. Existing startup smoke observedclose helper is
reference. No unrelated test expansion; apply this to modified runtime harness only.
4 walletAccountManagement env test currently puts invalid values on unallowlisted names,
which doesn't prove allowlisted-key filtering. Add compacttable for actual allowed key
(e.g.DISPLAY) accessor (getternevercalled),nonstring,embeddedNUL,4097bytes plus4096valid,
and Unicode byte-count boundary. Existing sevenvalidroutes and no secret/leakedidentity
retained. Main processenv knownstrings test is fine; no realchildspawns under injected
HOME/NODE_OPTIONS etc. Restore all modifiedenv in finally.
5 In new graceful tests prove intent.cancel frame precedes stdin.end via calls index,
not mere presence. Add normal asynchronousclose case: end=>no signals, exit=>promise
pending, close=>resolved; no timers or latersignals. Existing immediatefallback/protocol
fault,1000/1250/1500 boundaries and synchronousclose/endthrow tests retained.
6 Actual walletStartupSmoke should deliberately run headless: save descriptors/values
for DISPLAY and WAYLAND_DISPLAY, delete just these before main ready, restore in finally
after childclose (includingfailure). This avoids nondeterministic hostGUI initialization;
separate Xvfb actualwindowQA is planned. Keep strict allowedenv assertion and actual
main/resolver/supervisor/Rust composition/pin/close proofs.
7 Review new shutdown ignoreEOF wrapper remains real: interval+SIGTERMignore, ignores
fixture stdin end handler, fixedfixture require, and selected ignoreEof option actually
used. Move wrapper filecreation into existing try/finally or guard it so a write/setup
failure cannot leak ownedwrapperdirs; remove only after observedchildclose. No edits
to production transportfixture authorized. Preserve wrapper and original failures.

Other reviewed deltas are accepted: Menu.buildFromTemplate mocks in threeharnesses,
--features native-ui buildargv, account.list[] and malformedaccount.lockSCHEMA contract
changes. They are frozen. No new general testinglibrary/fixtures/dependencies. Stop
with exactsourcehashes/line/groupcounts and enumerated correctedbehavior, noexecution.
