# WAL-011 async Electron IPC tests 01

Actor: Grok Build, grok-4.6 High; test source only, no subagents.
Protected parent: reviewer publication following 31a6e540; one CURRENT-only launch
record may follow. Read AGENTS.md, TESTING.md, active CURRENT prefix, ticket and
this handoff. Only writable path: test/electronSecurity.node.js.

Verify baseline identities before editing:

| Path | SHA-256 |
| --- | --- |
| test/electronSecurity.node.js | cc9bd07a687a07bd5852aa6849e19622e52fc56ab2f8e9e76cc32f9796b5cad8 |
| social-main.js | b67a6ba8187776f675714cb0ea26934d4ecbc809df5df72d3c738ab4bddea4df |
| wallet-broker/supervisor.js | 1ac92493cb70cd4ea5e1884f4b96424c7fbc89126308299e7221f2b0ba1a8ff8 |
| wallet-broker/protocol.js | 79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4 |

Additional read-only source: wallet-preload.js, wallet-pay/model.js,
test/fixtures/wallet-pay/snapshots-v1.json. No other source/history discovery.
Separate read-only HEAD/status, named hashes/line counts and bounded reads/searches
are allowed. No execution of Node, tests, syntax checks, Cargo, npm, dependencies,
network, actor tools, config/home/credential discovery, Git mutations or docs edits.
Preserve all unrelated pending changes. Do not edit production even if the fix is
obvious. Report changed path, SHA-256, lines, total groups and cases; then stop.

## Fixed behavior

The supervisor now returns Promises. social-main.js walletHandler currently clones
that Promise synchronously, yielding {} and losing the actual result/rejection.
The later production change must preserve synchronous sender/payload validation
and synchronous dispatch errors, then resolve the dispatch return and clone its
fulfilled value. Rejections must propagate, with no success sentinel or swallowing.
No new error format, result schema, authority, channels, native flow or startup
configuration is introduced. The synchronous injected dispatch seam remains usable.

Exercise the actual callbacks registered in ipcMain.handle by loading the maintained
social-main.js with the existing Electron fixture. Extend its supervisor dispatch
seam to inject controlled outcomes per isolated fixture. Keep its default existing
behavior and all existing security assertions. New fixture instances must not alter
the shared boot() fixture used by older tests. Restore Module._load and the previous
main module cache entry in finally for isolated loads; do not leave global hooks.

Add focused cases covering:

1. All five existing wallet channels dispatch once to the fixed method, validate and
   clone inputs before dispatch, and return a thenable pending until a controlled
   deferred reply resolves. Attach observers immediately and prove no settlement
   after a bounded event-loop turn while the deferred reply remains pending; use
   explicit settlement state, not an arbitrary wall-clock sleep. Resolve with
   independent literal plain-data replies. Snapshot reply should be the truthful
   {v:1,broker:'degraded',accounts:[]} literal. Other channels may use opaque nested
   fixture data; those literals make no native-wallet availability claim. Assert
   exact fulfilled values, deep non-aliasing and mutation isolation in both
   directions after fulfillment. Do not use a production clone/sanitizer as oracle.
2. A delayed supervisor rejection must reject the handler result, never resolve
   {} or another sentinel. Cover UNAVAILABLE and TIMEOUT errors with fixed fixture
   messages and ensure dispatch occurs once. Attach a rejection observer to the
   controlled input Promise as well as the returned result so the intentionally
   broken baseline yields assertion failures, not stray unhandled rejections.
   In this in-process test, assert original error identity; make no claim about
   Electron's renderer-side serialization of custom error fields.
3. Synchronous dispatch failure still throws immediately. Existing synchronous
   rejection assertions for invalid sender/payload/accessors remain unchanged and
   prove zero dispatch. Retain existing synchronous-return mapping/clone test too.

Every deferred operation must be settled/observed in finally even when a baseline
assertion fails. Avoid hanging tests: a bounded timeout guard is allowed for awaited
settlement, cleared in finally; it must fail rather than substitute a result. No
global rejection handlers that suppress unrelated failures. No child processes,
filesystem writes, timers left pending, actual Electron launch or Rust execution.

## Later execution contract (not authorized for this actor)

Hermes expected red: `node test/electronSecurity.node.js`. Expected new failures are
missing Promise/reply settlement or absent rejection propagation through the real
registered callback. Existing unrelated security cases should retain their result.
Source review precedes that command; production follows accepted observed red.

Targeted green: same command. Affected regression: `node test/walletPreload.node.js`.
No dependency, privilege, network, packaging or sink changes are authorized. The
security suite checks the maintained Electron privileges, channels and dangerous
sinks; the pending package-policy failure is neither altered nor waived. Final
application/release security gates remain required at their separately authorized
stage. Do not rerun accepted transport or Rust proof suites.

Falsification after green source review: temporarily restore the synchronous clone
of the unresolved dispatch result in walletHandler only. The new delayed-reply
case must fail for missing settlement, then restore exact production bytes and run
targeted green. Reviewer will give Hermes exact mutation and commands after seeing
the source. Do not implement or execute falsification in this test-only drop.

Startup with a reviewed external binary pin, app shutdown, real Electron-to-Rust
composition and native/account flows are separate later tasks. This suite proves
the registered main handler's async boundary with a controlled supervisor fixture.


## Collected source acceptance — 2026-09-09

Accept the test-only drop for initial expected-red execution; no green behavior
is claimed. Outer 58847 collected on owner done with exit 0. Exact-ID Markdown
export of e1f08576-2e17-4f42-9001-f544249bcab3 shows four edits to the authorized
test path, source reads/searches and read-only identity/status commands; no tests,
syntax checks, builds, production edits or Git mutations appear. The export is a
tool summary, not a raw tool-result audit. Read-scope deviations: CURRENT lines
1–200 included historical text; additional transport/preload test files were read,
search scope was broader than the named files, and two identity command lines
chained commands despite the separate-command instruction. Those deviations are
recorded without expanding future authorization.

Measured test/electronSecurity.node.js: 1223 lines, 23 top-level groups, SHA-256
df0aab1686f872fbc2e77ab106f0003ad1fbe76f7c74bda9c00756f53bc0e4a3.
Diff: 352 insertions, two deletions. The three frozen production hashes match the
contract. Existing twenty groups retain their assertions. New source defines five
successful delayed-reply rows, two rejected-reply rows and one synchronous throw
case in three groups. These are authored cases, not eight executed/passed results.

Source review confirms the actual registered handlers are invoked, isolated module
loads restore the loader/cache, outcomes use independent plain-data literals,
resolved results and inputs have mutation-isolation assertions, deferred operations
are observed/settled in finally and settlement guards clear their timers. The
existing synchronous authorization and payload rejection tests remain intact.
The baseline should fail the first row of each delayed group at the explicit
thenable assertion. Later rows in those groups will not execute on that baseline;
green must complete them all. Expected totals: 21 ok, two not ok, exit 1.

Grok is closed. Only the linked
[Hermes expected-red driver](HERMES_BBD_WAL_011_ASYNC_IPC_RED_01.md) is authorized.
Production, integration, additional tests and replay remain closed. The reviewer
ran no acceptance command.
