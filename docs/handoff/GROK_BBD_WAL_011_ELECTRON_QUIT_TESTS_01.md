# WAL-011 Electron normal quit tests 01

Actor: Grok Build, grok-4.6 High; test source only, no subagents.
Protected parent: reviewer publication following 371379d09c35443239a2e2f28d9a1e722bc637b5,
plus one CURRENT-only launch record. Read CURRENT lines 1–40, this handoff,
AGENTS.md, TESTING.md and tickets/BBD-WAL-011.md. Only edit:

- test/electronSecurity.node.js

Verify these baseline SHA-256 identities before editing:

| Path | SHA-256 |
| --- | --- |
| test/electronSecurity.node.js | df0aab1686f872fbc2e77ab106f0003ad1fbe76f7c74bda9c00756f53bc0e4a3 |
| social-main.js | 2449b0b190a9ad079639e4d4aca628470d93749bdf92200cc796a1ed33fa1aa4 |
| wallet-broker/supervisor.js | c1410bfc5cd4c205ad014a42154c9d9d9714794b134a5acab7ed499934cad51a |
| wallet-preload.js | 3e6a18acf88dd5be4e6a88f326d6ace7a8071066480d9a70a2e8f89df035a1df |
| test/walletPreload.node.js | 60b151344a01776e1d7f38238f69534426883503f9d627466bac7acbf4dc4f9e |

Additional allowed reads: wallet-pay/model.js,
test/fixtures/wallet-pay/snapshots-v1.json, architecture review section 4.2
(docs/architecture/BBD-WAL-001-REVIEW.md lines 438–469). No other discovery.
Separate read-only HEAD/status, named hashes/line counts and bounded searches in
these files are allowed. No chained shell commands, tests, syntax checks, Node,
Cargo, npm, builds, dependencies, network, actor tools, home/config/credential
discovery, Git mutations, production edits or evidence/document edits. Stop on
identity mismatch. Preserve unrelated pending files. Report the changed path,
SHA-256, line count, group count and authored cases; then stop without execution.

## Reviewer decision

The accepted supervisor shutdown() already initiates intent cancellation and
termination, resolves only after child closure (or no child), and rejects TIMEOUT
after 1500 ms without confirmed closure. It returns the same settled Promise on
later calls, including after a timeout. Main must consume that completion before
allowing normal quit. This contract covers the main application's event boundary;
startup artifact selection and packaging remain separate, unresolved work.

Electron's [before-quit event](https://www.electronjs.org/docs/latest/api/app#event-before-quit)
supports synchronous preventDefault to hold normal termination. Register the gate
at module load, after constructing the existing supervisor and before ready.
On the first event, synchronously prevent default and establish pending state
before invoking shutdown() exactly once. Pending repeat events, including a
synchronous event emitted from inside the injected shutdown call, are prevented
without another shutdown call. The event handler must not return a rejected Promise
or throw a supervisor failure into Electron's event emitter.

After successful shutdown, approve quit before calling app.quit() exactly once.
That call re-enters before-quit; the approved event must proceed without prevention
or another shutdown call. A Promise already resolved for an absent child follows
the same sequence. The gate needs no timer of its own and must not call quit(),
close(), kill() or dispatch() on the supervisor in place of shutdown().

On shutdown rejection or synchronous throw, keep normal quit blocked and attempt
one main-process dialog.showErrorBox with these exact literal strings:

- Title: `Unable to close BitBook`
- Content: `Wallet shutdown could not be confirmed. BitBook will keep running.`

Record failed state before displaying the error. Subsequent quit events remain
prevented without another shutdown call or dialog. Do not retry a permanently
rejected supervisor Promise, allow exit after the timeout, or claim a retry will
recover. A throwing error dialog must also leave quit blocked and its failure must
be contained. Never interpolate, log or send raw failure messages, stacks, child
diagnostics or paths to a renderer. The fixed native error notification adds no
wallet authority or renderer API. Electron documents
[showErrorBox](https://www.electronjs.org/docs/latest/api/dialog#dialogshowerrorboxtitle-content)
as usable before ready (Linux writes the early message to stderr).

Preserve existing window-all-closed platform behavior: it requests app.quit on
non-macOS; macOS remains resident. Both app.quit requests and menu/OS normal quit
events reach the same gate. Forced termination, Windows logout/system shutdown,
updater lifecycle, native descendants and window-veto recovery are not proven by
this bounded change. No force-exit route is added. Before broker startup is later
authorized, its separate contract must also prevent spawning during a pending or
completed quit. This task does not introduce broker startup.

## Test-source contract

Reuse createElectronMock, loadIsolatedMaintainedMain, deferred and settlement
helpers. Preserve the default ready behavior and all 23 existing groups/assertions.
Allow an isolated fixture to omit ready and inject shutdown/error-dialog behavior.
Count shutdown calls separately from existing dispatch calls. Add an app.quit
fixture route that emits the actual registered before-quit callback with a fresh
preventable event and records whether it was prevented; it must not implement the
production gate, shutdown, approval or failure decisions itself. Record ordered
events at their occurrence. Bound accidental re-entry so a broken implementation
fails an assertion rather than recursing indefinitely. Add forbidden force-exit
spies if needed. Do not create a parallel copy of the main implementation.

Add seven focused top-level groups:

1. After ready, initial before-quit is prevented synchronously; a deferred shutdown
   is invoked once. Repeat events remain prevented and no application quit occurs
   during a bounded event-loop turn. Resolve the deferred, then assert one app.quit
   call, an unprevented reentrant event, one total shutdown call, no error dialog
   and no force-exit. Trace proves shutdown fulfillment precedes resumed app.quit.
2. Before ready, there are no windows, but the registered gate prevents the initial
   event. A fulfilled Promise representing no child allows precisely one resumed
   app.quit and an unprevented reentrant event after settlement. No ready event or
   window creation is needed to finish this case.
3. Have shutdown synchronously emit another before-quit before returning its pending
   Promise. Both initial and nested events are prevented, shutdown is called once,
   and quit remains held until resolution; then it resumes once. Explicitly assert
   the fixture emitted the nested event so this cannot pass vacuously.
4. Deferred rejection rows for TIMEOUT and UNAVAILABLE: initial event prevented,
   zero resumed quit before and after rejection, exactly one shutdown and one
   literal error box. Use private-data canaries in error message/stack and assert
   only the fixed title/content appear in the observable dialog/renderer payloads.
   Further events stay prevented without retry or another error box.
5. Synchronous shutdown throw: invocation does not escape the event handler;
   the same fixed failure behavior holds, including repeated blocked requests.
6. Compound failure: rejected shutdown followed by a throwing showErrorBox. The
   attempted dialog is counted once before it throws; no exception or unhandled
   rejection, resumed quit or forced exit. Later events remain blocked. Have the
   dialog emit a nested quit event before throwing to prove failed-state re-entry.
7. Emit window-all-closed through the actual registered handler. On the host's
   non-macOS branch, app.quit emits a prevented before-quit event, shutdown is
   pending once, and after completion there is exactly one additional resumed
   app.quit with an unprevented event. On macOS assert window-all-closed alone
   does not quit or shut down, then exercise explicit quit through the gate.
   Do not overwrite process.platform or claim cross-platform native execution.

Assert registered callbacks exist; the baseline's absent before-quit gate must
produce clear assertion failures, not pass through empty listener arrays. Inputs
and expected observations must be independent literal data; production code must
not be parsed as an assertion oracle. Assert ordering at event time where useful,
and observe at least one bounded event-loop turn before testing pending outcomes.
The shutdown fixture should also expose a legacy quit spy that must stay unused.

Observe each controlled Promise immediately, settle and observe every deferred in
finally even on baseline failure, and clear timeout guards. Restore Module._load
and the original main cache entry for isolated loads. Preserve shared boot state.
No global rejection swallowing, new child processes, actual Electron launch,
filesystem writes or persistent timers/listeners. Reuse helpers and keep the
fixture extension small; do not create a second lifecycle test framework.

## Later execution, not authorized for this actor

After source acceptance Hermes runs expected red once:
`node test/electronSecurity.node.js`. Expected new failures concern the missing
before-quit gate; the existing 23 groups should retain their results. The reviewer
will fix exact expected totals after inspecting the authored source. Production
work is a separate later handoff restricted to social-main.js after observed red.

Targeted green: the same command. Affected regression:
`node test/walletPreload.node.js`. This main-process/security fixture suite covers
unchanged sandbox, channels, sender validation, async IPC and dangerous sinks.
No dependency, renderer privilege, package, network or native artifact change is
authorized. Existing package-policy failures remain unwaived; final application
and release security gates still require a separate authorization. Retain accepted
supervisor, transport and Rust evidence without replay for this main-only wiring.

High-value falsification: temporarily bypass the wait so main resumes app.quit
while the controlled shutdown is still pending. The first group's ordering/pending
assertion must fail; restore exact production bytes and rerun targeted green.
Reviewer will supply the exact mutation and execution driver after source review.
No falsification code or execution is part of this test-only drop.

This proves callbacks registered by the real main module with controlled Electron
and supervisor fixtures. Actual Electron-to-Rust startup, package pins, native
account flows and whole-ticket/release completion are not claimed.
