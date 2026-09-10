# WAL-011 awaitable broker shutdown tests 01

Actor: Grok Build, grok-4.6 High; test source only, no subagents.
Protected parent: reviewer publication following a9b92048, plus one CURRENT-only
launch commit. Read CURRENT lines 1–40 only, this handoff, ticket, AGENTS.md and
TESTING.md. Only write these two new files:

- test/walletSupervisorShutdown.node.js
- test/fixtures/wallet-broker/shutdown-child.js

Read-only source identities to verify before editing:

| Path | SHA-256 |
| --- | --- |
| wallet-broker/supervisor.js | 1ac92493cb70cd4ea5e1884f4b96424c7fbc89126308299e7221f2b0ba1a8ff8 |
| wallet-broker/protocol.js | 79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4 |
| test/fixtures/wallet-broker/transport-child.js | ba11ba8a37d4ee7a6df2e3499c40c972ca4a1cfd8ef0eed377afe1c7a1e3ba91 |
| test/walletSupervisorTransport.node.js | e69fdcfa4aaeeca03b4abf8ac100888ff0b42fbb6001f53e461a35a5b7550700 |

Additional allowed reads: test/fixtures/wallet-broker/transcript-v1.json,
wallet-pay/model.js, social-main.js and the process-launch section (lines 438–469)
of docs/architecture/BBD-WAL-001-REVIEW.md. No other source/history/config discovery.
Separate read-only HEAD/status, named hashes/line counts and bounded source searches
are allowed. No chained commands, tests, syntax checks, Node/Cargo/npm execution,
builds, dependencies, network tools, actor launch, evidence/docs edits or Git
mutations. Preserve every unrelated pending file. Stop on identity mismatch.

## Reviewer architecture decision

Existing quit() cancels tracked intents and initiates SIGTERM, with SIGKILL after
250 ms if the child has not exited, but returns undefined immediately. Main cannot
yet use it to prove cleanup completed before normal application exit. Add a new
shutdown() method as a prerequisite to app startup/lifecycle composition. Keep the
existing quit() return contract and call sites unchanged in this slice.

Node distinguishes child exit from closed stdio; the completion boundary is its
[close event](https://nodejs.org/api/child_process.html#event-close).
Electron permits delaying normal quit via
[before-quit](https://www.electronjs.org/docs/latest/api/app#event-before-quit).
Later main wiring will wait for shutdown before completing a normal quit. Forced
OS/process termination and native descendant cleanup are not proven by this slice.

Startup provenance remains governed by architecture section 4.2: a packaging stage
must create pins from reviewed build inputs, independent of runtime file selection.
No runtime self-pinning, environment-selected executable, new package content or
real coin binary is authorized here. Fixtures exercise the real pipe/process
boundary. User-data location remains a private wallet-broker child directory under
Electron's userData path. Startup options, platform artifact inventory and main
before-quit wiring follow in separate bounded contracts after this prerequisite.

## Fixed shutdown semantics for future production

Future production path is wallet-broker/supervisor.js only, after accepted red.

- shutdown() returns the same Promise on every call, pending or settled. It starts
  existing quit semantics exactly once; repeated shutdown/quit calls must not
  duplicate intent cancellation, SIGTERM or the existing SIGKILL escalation.
- The first call immediately rejects all pending public requests through existing
  fixed UNAVAILABLE handling, publishes existing down state once, clears bound
  state, stops further dispatch and prevents future start. Existing cancellation
  attempts precede SIGTERM. No wait for cancellation replies or new RPC authority.
- Resolve with undefined only when there is no child object or child close has
  actually been observed, including close before shutdown. child.killed, exitCode,
  signalCode, exit event, stream error/end or a successful kill() return alone do
  not establish close. Handle synchronous close during signal delivery without
  losing notification or producing extra timers/signals.
- If close remains unobserved 1500 ms after the first shutdown call, reject with
  the existing normalized TIMEOUT error (code TIMEOUT, message Timed out,
  retryable true). Retain the existing 250 ms SIGKILL schedule. No process.exit,
  retry loop, restart or silent success. A later close cleans up but must not
  change the already-rejected shared Promise to success.
- After completion, clear timers introduced by shutdown. Repeated calls must not
  accumulate listeners. Preserve existing safety listeners needed for a late close
  after timeout; do not remove observers owned by tests or callers. Existing
  onChildClose can notify completion without introducing per-call subscriptions.
- Calling shutdown before start, after failed start, after prior quit or after
  protocol failure follows the same rules. Await existing termination if a child
  exists; do not spawn, reset failure or repeat termination just to shut down.

## Test source contract

Keep tests focused on lifecycle completion; do not copy the full transport matrix.
Use independently controlled EventEmitter child/stream fixtures and a deterministic
clock via the existing system seam for ordering, 249/250 ms escalation and
1499/1500 ms shutdown deadline assertions. Drive the actual supervisor start and
framed hello/bootstrap path when bound state/pending requests are required. No
private-state shortcut, Promise-return sentinel standing in for closure, or oracle
derived solely from production output. Assert real effects, not just API presence.

Cover at least these focused groups (combine parallel subcases sensibly):

1. No-start and failed-start shutdown: Promise identity, undefined resolution,
   no spawn/kill/timer, and start remains unavailable afterward.
2. Bound supervisor with tracked intent and a held public request: cancellation
   frame precedes first SIGTERM; public request rejects UNAVAILABLE, down state
   publishes once, dispatch is unavailable, and repeated calls return one Promise.
   Keep it pending through child exit and resolve only after explicit close.
3. Existing quit/protocol failure before shutdown and close-before-shutdown:
   no repeated signals or cancellation; correct pending versus resolved outcomes.
   Include close synchronously emitted by fake kill().
4. Stubborn child: fake-clock SIGKILL exactly at 250 ms, never at 249; no false
   success from kill/exit fields. At 1499 ms shutdown remains pending; at 1500
   reject exact normalized TIMEOUT. Late close preserves rejected outcome and
   repeated-call identity. Timer/listener ownership assertions must be meaningful.
5. Real child normal termination: accepted fixture handshake, held public request,
   explicit call to shutdown, observed close and closed streams before fulfillment,
   no remaining child. Do not substitute only fake kill events for this case.
6. Real child ignoring SIGTERM: prove the handler is installed before handshake,
   then observe real termination with SIGKILL on POSIX and shutdown fulfillment
   after close. No broad sleeping; use bounded events and cleanup deadlines.

The new shutdown-child.js should be a small wrapper around the frozen existing
transport-child.js, preserving its hold mode. Install the optional SIGTERM ignore
handler before requiring that fixture, selected only by explicit test argv. It
must not create files, endpoints or descendants. The harness may inject system.spawn
to launch the exact Node executable with this fixture's argv, as established in
the accepted transport tests; assert original requested path/options. Computing
the Node fixture's hash in the test is a fixture-only seam, not production pinning.

Use realpath(process.execPath) for the executable fixture. Create a private empty
temporary cwd; verify it remains empty after every child closes and remove only
that empty directory using rmdir. Never recursively erase generated paths. Preserve
unexpected contents and report them. Observe spawn failures and stream completion,
attach rejection observers immediately, settle fixture promises in finally, clear
test timers and reap a real child on every failure. Emergency SIGKILL is permitted
in harness cleanup, but timeout or failed reaping must fail the test; do not hide
the original assertion error. Assert shutdown exists before spawning any real
child, so expected red cannot leak processes. Do not swallow cleanup failure.

Use up to 3000 ms real-process observation guards, with all timers cleared. The
1500 ms production deadline is proved with the deterministic clock, not tight
wall-clock assertions. The SIGTERM-ignore/SIGKILL real case is POSIX-only; explicitly
report a skip on Windows without claiming it passed. This handoff's Linux execution
must run both real cases. No actual Electron or Rust invocation and no network.

## Later execution and falsification

Report two-path hashes, lines, groups/cases and stop. Hermes runs expected red only
after source review: `node test/walletSupervisorShutdown.node.js`. Expected failure
is the absent shutdown API, not missing modules, fixture errors, hang or leaking
child. Production follows accepted observed red.

Targeted green: the same command. Affected regressions:
`node test/walletSupervisorTransport.node.js` and `node test/walletSupervisor.node.js`
because their shared lifecycle changes. Do not replay unrelated Rust or app tests.
Falsify completion by temporarily resolving shutdown when termination is initiated
before child close; the held-close test must fail. Restore exact source and rerun
targeted green. Reviewer will fix exact mutation/commands after source review.

No dependency, build input, renderer privilege, endpoint or release-content change
is authorized. Existing security-policy failures remain separate and unwaived.
Final application/release security gates remain required at that later stage.


## Collected source review and Correction 01 — 2026-09-09

Decision: reject the initial drop for execution; authorize bounded test-source
correction only. Outer 20254 collected on owner done with exit 0. The exact-ID
Markdown export of 6d0b7d41-29c1-4ed8-bda9-7225b190f34b shows writes only to the
two authorized new files, named source reads/searches and read-only status/identity
commands. Two existence-check commands used a shell separator/echo despite the
separate-command instruction. Exported read ranges were shifted by one line.
No tests, syntax/build execution, production edits or Git mutations appear. This
is a summarized tool inventory, not a raw result audit or execution acceptance.

Measured test/walletSupervisorShutdown.node.js: 886 lines, six Linux groups,
SHA-256 f4531ced2c6851a7dac480519b5ec696e407b2c8aa63838eba4492272bcffac8.
Measured shutdown-child.js: eight lines, SHA-256
0d8fbfa8179338a8fc05e6c41f4cbbf0ef652c4ccadd73db8876591a51f96dcb.
All four baseline production/transport identities remain unchanged. Independent
review confirms the intent ID is valid (32 hex characters). No suite was executed.

The fake-clock boundary cases, cancellation ordering, shared-Promise expectations,
late-close outcome and empty-only cwd cleanup are retained. The six groups/ten
subcases are authored coverage, not passed tests. Missing-API red would skip the
following defects, so repair them before authorizing that red.

1. The spawn wrapper sends SIGTERM immediately after childProcess.spawn, before
   the child has run shutdown-child.js. The ignored-SIGTERM group enables it with
   sigtermBeforeAttach:true (original lines 406 and 863). That can terminate the
   process under the default signal disposition before its handler is installed.
   Remove the option and this early kill entirely, including now-unused options
   plumbing and misleading assertion text. Use successful handshake as the
   readiness barrier: the frozen wrapper installs the handler before producing
   hello. Then let supervisor.shutdown() send SIGTERM normally and require actual
   close with SIGKILL. Do not add a sleep or a second probe signal.
2. Real-child close callbacks at original lines 783–803 and 845–847 throw assertions
   from asynchronous EventEmitter callbacks. Such throws bypass the async test's
   try/finally and may terminate the runner before cleanup. Callbacks must only
   capture event-time facts (including pending-request/shutdown state and signal)
   or settle an already-observed event Promise; put assertions in the awaited test
   flow. Preserve the event-time ordering checks and bounded cleanup on failure.
   After shutdown fulfillment, assert actual captured child/stdin/stdout/stderr
   close facts; remove the permissive `closed || stream.destroyed` substitutions.
   Do not substitute a post-cleanup close for closure observed at fulfillment.
3. waitForMarkedClose (original lines 343–364) leaves its close listener registered
   after a timeout. Use one settle/cleanup path that always clears its timer and
   removes only its own listener on close, timeout and the already-closed race.
   Keep rejection on timeout, and preserve unrelated listeners. No suppression of
   failure or unbounded waits.
4. Complete teardown of tracked resources: fake-harness finally blocks currently
   only call quit(), leaving any newly created shutdown Promise pending if an
   assertion fails before simulated close. In teardown, signal fake close and
   flush observed Promise settlement without altering the assertions being proved.
   Real harness cleanup must also observe outstanding public/shutdown Promises and
   clear any test-owned event observers/timers on failure. Track them as needed;
   bound any wait. Preserve the original assertion alongside cleanup errors and
   do not report a timeout, failed reaping or unobserved closure as success.

Correction actor: Grok Build, grok-4.6 High, no subagents. Protected parent is the
reviewer publication following a12e2b6d, plus one CURRENT-only launch commit.
Only write test/walletSupervisorShutdown.node.js. The eight-line shutdown-child.js
is now frozen; no fixture/protocol/production semantics change or new case matrix.
Keep six Linux groups and the existing Windows skip behavior. Bounded helper
refactoring for the four findings is permitted; no unrelated cleanup.

Read CURRENT lines 1–40 only, this final review, AGENTS.md/TESTING.md if needed,
the affected test and frozen shutdown-child.js. Named baseline hashes and separate
read-only HEAD/status/count commands are allowed; no history, broader source reads,
command chains, Node/test/syntax/build execution, network, credential/config/home
discovery, docs/evidence edits or Git mutations. Report resulting hash, lines,
groups and corrections, then stop. Hermes execution/integration remains closed.
