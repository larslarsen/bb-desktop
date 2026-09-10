# WAL-011 Electron normal quit tests 01

Active amendment: Correction 01 at the end of this document supersedes the initial
editable-file baseline. The initial actor is collected; no execution is authorized.

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

## Collected source review and Correction 01 — 2026-09-09

Hold source acceptance; authorize only a bounded test-source correction by Grok
Build, grok-4.6 High, without subagents. Outer 59284 collected on owner done,
exit 0. Exact-session export 9030ad8c-83ac-4a78-a4c6-d135e2470109 reports edits
only to the authorized test file, named source reads and read-only Git/hash/line
commands. No test execution, production edit or Git mutation appears. The export
is a tool summary, not a raw result audit. Read-scope deviations: CURRENT was read
as lines 2–41 and architecture as 439–470, each one line beyond its authorized
range; parent/last-commit inspection exceeded the explicitly named HEAD/status
commands. Search entries omit paths, so their scope cannot be independently
confirmed from this summary. These observations do not widen this correction.

Measured drop: test/electronSecurity.node.js SHA-256
`a8ca18c77b17b75a6e361640640ca6c16526fb93010fc08e2706f3499393d861`,
1977 lines, 30 top-level groups (23 retained, seven added), eight authored new
cases including two rejection rows. Diff is 758 insertions/four deletions. All four
frozen production/preload identities in the initial table still match. These are
source observations, not executed results. The reviewer ran no acceptance command.

The fixture invokes actual registered main callbacks, enforces nonempty handlers,
records lifecycle events and observes deferred outcomes. Existing test assertions
are preserved. Two defects prevent acceptance:

1. At reviewed lines 1790–1794 the injected showErrorBox asserts that its nested
   quit was prevented, then throws its intended fixture error. Production must
   catch dialog errors, so it can also swallow this AssertionError. The outer
   assertions only check that the callback was entered and never independently
   check that nested event. A handler that wrongly allows the dialog's nested
   quit can pass this group. Nor is the intended throw proven to have been reached.
2. Every new group's finally restores process.exit and removes its rejection
   observer before settling deferred shutdown and draining continuations (for
   example reviewed lines 1838–1846). An early failed assertion can leave production
   cleanup executing after the real exit function is restored. The fixture must
   contain the code under test until cleanup ends, and report cleanup-time errors
   before removing its observer; otherwise a broken path can exit the test runner
   instead of producing the required failure evidence.

Correction authorization, only test/electronSecurity.node.js:

- Verify the editable baseline a8ca18c7 above and the four unchanged identities
  from the original table before editing. Protected parent is reviewer publication
  following 2908e3e640d42683873e3549923fe5bd3b36c645, plus one CURRENT-only launch
  commit. Read CURRENT lines 1–40 (use the tool's correct line/offset convention),
  this handoff, ticket, AGENTS.md and TESTING.md. Keep original source-read limits.
- In the compound case, record the nested event/observation without asserting
  inside showErrorBox, and record a marker immediately before throwing one stable
  fixture Error object. After settlement, outside production's catch, assert that
  the nested call returned, its event was prevented, the intended throw marker was
  reached exactly once, and no extra app.quit/shutdown/dialog occurred. Assert
  nested record counts/provenance as appropriate. Keep the later repeat assertion.
- Keep process.exit interception and rejection observation active throughout each
  new group's finally cleanup, including a bounded event-loop drain after settling
  all deferreds and observing their continuations. Check captured rejections and
  forbidden force paths after that drain. Restore both hooks in an outer finally
  that runs even if settlement, draining or assertions fail. A small shared cleanup
  helper is appropriate; avoid seven more copies of cleanup plumbing. Preserve an
  earlier assertion failure when cleanup succeeds; never convert cleanup failure
  or captured rejection into success. Do not add a swallowing global handler.
- Attach observers promptly to any Promise derived by the new fixture so cleanup
  does not create unobserved rejections of its own. Preserve all 30 groups and the
  actual application-callback boundary; do not rewrite unrelated security tests or
  broaden scenarios. Keep the existing pending/ordering and fixed-message checks.

No production, tests/syntax execution, builds, evidence, integration, Git mutations,
network, actor tools or discovery outside the original read scope. Separate named
read-only HEAD/status/hash/count commands remain allowed. No history inspection is
needed. Report exact changed path/hash/lines/group count and correction summary;
then stop. Hermes expected-red authorization follows corrected source acceptance.
