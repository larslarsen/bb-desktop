# WAL-011 Electron normal quit production 01

Active amendment: Correction 01 is accepted below. Grok is closed; only the linked
Hermes focused validation is authorized. No integration yet.

Actor: Grok Build, grok-4.6 High, production source only, no subagents.
Protected parent: reviewer publication following 384ba2398337540d4085e6a2a2a0b444c2d9f365,
plus one CURRENT-only launch record. Read CURRENT lines 1–40, this handoff,
AGENTS.md, TESTING.md and tickets/BBD-WAL-011.md. Only edit social-main.js.

Verify all named SHA-256 identities before editing:

| Path | SHA-256 |
| --- | --- |
| social-main.js | 2449b0b190a9ad079639e4d4aca628470d93749bdf92200cc796a1ed33fa1aa4 |
| test/electronSecurity.node.js | 7f759f810762ccdfa5e29d69dc3bf2fa1b1502d2c416ca2b324442b2544ad18c |
| wallet-broker/supervisor.js | c1410bfc5cd4c205ad014a42154c9d9d9714794b134a5acab7ed499934cad51a |
| wallet-preload.js | 3e6a18acf88dd5be4e6a88f326d6ace7a8071066480d9a70a2e8f89df035a1df |
| test/walletPreload.node.js | 60b151344a01776e1d7f38238f69534426883503f9d627466bac7acbf4dc4f9e |

These five files are also the only source-read scope. The collected acceptance
section of HERMES_BBD_WAL_011_ELECTRON_QUIT_RED_01.md may be read. Separate named
read-only HEAD/status/hash/line commands and bounded source reads/searches are
allowed. No history, configuration/credential discovery, other source reads,
tests, syntax checks, Node/Cargo/npm execution, dependencies, builds, network,
actor tools, evidence/docs edits or Git mutations. Stop on identity mismatch.
Preserve all pending files, including the test drop and expected-red evidence.

## Accepted red and fixed behavior

Hermes ran the unchanged main against the accepted 30-group test source once:
23 existing groups passed; seven new groups failed at the absent before-quit
handler, Node exit 1. Reviewer accepted that result. Implement the gate now;
do not edit tests or run any command to execute the code.

Add main-process dialog to the Electron import. Keep the existing supervisor
construction and register a before-quit handler at module load, before ready.
Use a small explicit lifecycle state with these transitions:

1. On the first event, synchronously preventDefault and mark shutdown pending
   before calling walletSupervisor.shutdown() exactly once. Call shutdown inside
   the handler so re-entry from it encounters pending state. Contain synchronous
   throws and handle asynchronous rejection. The event handler must return
   normally without throwing or returning a rejected Promise into Electron.
2. Pending repeated/reentrant events are prevented with no additional shutdown,
   app.quit or dialog call. Invoke no legacy supervisor quit/close/kill/dispatch
   method; shutdown already owns cancellation, signals and the completion timeout.
3. Fulfillment, including an already-fulfilled no-child Promise, marks quit
   approved before calling app.quit once. The resulting before-quit event proceeds
   without prevention, shutdown or another app.quit. Add no main-process timer.
4. Rejection or synchronous shutdown throw marks quit failed before attempting
   dialog.showErrorBox once, with title `Unable to close BitBook` and content
   `Wallet shutdown could not be confirmed. BitBook will keep running.`
   Prevent all later quit requests without retrying shutdown or repeating the box.
   The supervisor returns its same terminal Promise, so retries cannot establish
   new closure evidence. A nested event during the dialog must remain prevented.
5. Contain a throwing error dialog as well. Handle the entire Promise continuation
   so no detached rejection escapes; never stringify/log/forward raw error details.
   No force-exit, relaunch, renderer API, authority or alternate success path.

Preserve window-all-closed behavior (app.quit on non-macOS, residence on macOS),
activate behavior, sandbox/window/security settings, IPC channels and current
async reply cloning. Keep changes local and small; no new module or exported API.

This is normal-quit wiring only. Do not add broker startup, user-data directory
creation, runtime executable selection, hashing/self-pinning, package changes or
native flows. Startup needs a separate packaging-pin/platform contract, including
avoiding a spawn during pending/completed quit. Forced OS/process termination,
Windows logout/system shutdown, updater lifecycle and native descendant cleanup
remain outside this proof. No whole-ticket or release completion is claimed.

## Stop and later validation

Report changed path, SHA-256, lines and implementation summary, then stop. Source
review precedes any execution. Hermes will later run node test/electronSecurity.node.js
(all 30 groups/eight new cases) and node test/walletPreload.node.js (six groups).
The exact focused falsification will resume app.quit while shutdown is pending;
the pending/order group must fail, then restore source and repeat targeted green.
That driver, mutation, evidence and integration require separate authorization.
Retain closed supervisor/transport/Rust validation; no replay here. Existing
package-policy failures are not waived, and final application/release security
gates still apply to their final inputs.

## Collected source review and Correction 01 — 2026-09-09

Hold acceptance for one small source correction. Outer 21766 collected on owner
done, exit 0. Exact-ID export 9ca89e48-d26e-4f9f-8aef-9a58c53d6c22 reports two
edits only to social-main.js and read-only commands; no tests, production/test
integration or Git mutations appear. This is a tool summary. Read deviations:
CURRENT again used 2–41, and extra publication hashes/blob/branch queries included
a chained command beyond the separate HEAD/status/hash/count scope. Search entries
omit paths. These do not expand this correction's read authority.

Measured source: social-main.js, 211 lines, SHA-256
fe116de535f3ee0796fddac94ee9f5178c467cc1102862368e64a5a18b4ef2c4.
The four frozen test/supervisor/preload identities still match. Source adds dialog,
four-state quit bookkeeping, module-load registration, pending before shutdown,
approved before app.quit, failed before a fixed error box and contained dialog
failure. Existing window and IPC behavior remains unchanged.

One branch is outside the fixed contract: after checking for a thenable, main
calls approveNormalQuit directly when the shutdown result is missing/non-thenable.
That approves exit without observing shutdown completion. The accepted supervisor
always returns a Promise, so this fallback is unnecessary; no current supervisor
failure is claimed. Additionally, the then lookup/subscription sits outside the
synchronous failure guard. Remove the unsupported branch rather than introducing
another success contract or expanding the tests for it.

Only Grok Build, grok-4.6 High without subagents may correct social-main.js:

- Verify editable SHA-256 fe116de5 above and the four initial frozen identities.
  Protected parent: reviewer publication following 792a38ed, plus one CURRENT-only
  launch record. Read CURRENT lines 1–40 exactly, this handoff, ticket, AGENTS.md,
  TESTING.md and only the original five named source paths. No history queries.
- Subscribe directly to the accepted shutdown Promise inside the same try that
  invokes shutdown. Remove the conditional thenable branch and direct synchronous
  approveNormalQuit fallback. A missing/non-thenable result or subscription throw
  must follow the existing failNormalQuit path through that try/catch; it must
  never approve quit. Keep fulfillment/rejection callbacks and containment of their
  returned Promise rejection. Preserve immediate pending state before invocation.
- Change nothing else. The four-state logic, fixed strings, dialog containment,
  window/IPC behavior and all test bytes remain frozen. This removes an unneeded
  branch; no new API, test scenario or production design is authorized.

No tests, syntax checks, execution, builds, evidence, docs edits, Git mutations,
network, actor tools or config/credential discovery. Separate named read-only
HEAD/status/hash/count and bounded reads/searches only. Report path/hash/lines and
the correction, then stop. Reviewer source acceptance precedes Hermes validation.

## Correction 01 source acceptance — 2026-09-09

Accept source for focused validation, not as executed green. Outer 17320 collected
on owner done, exit 0. Exact-session export f2bc7d42-ab64-46be-adc0-0e470ec55489
shows one source edit, named reads and separate read-only HEAD/status/hash/count
commands, with no tests, syntax checks, evidence edits or Git mutations. It is a
tool summary; search paths are omitted. CURRENT again read 2–41 rather than 1–40,
including one historical heading. No scope expansion follows.

Measured social-main.js: 205 lines, SHA-256
c7687b52ee45b17c3063cc2b39071be3d4e613a9d1e86558f2a7bbc1567db0a3.
The conditional/non-Promise success path is gone. shutdown invocation and direct
Promise subscription share the try/catch; pending precedes the invocation,
approval precedes resumed app.quit and failure precedes the fixed dialog. Both
dialog throw and returned continuation rejection are contained. Existing window,
sandbox and async IPC behavior is preserved. All four frozen source/test hashes
and accepted red evidence 229c1a97 still match. No test bytes changed.

Only [Hermes focused green/falsification](HERMES_BBD_WAL_011_ELECTRON_QUIT_GREEN_01.md)
is authorized next. Exact stages are 30 Electron groups, six preload groups,
one selected early-quit falsification and restored 30-group Electron green.
The actor may only apply/restore the specified temporary mutant through that
driver; no repairs or additional tests. Source acceptance is conditional on those
results. Integration, startup/packaging/native flows and release gates remain
closed. Reviewer ran no tests or acceptance commands.
