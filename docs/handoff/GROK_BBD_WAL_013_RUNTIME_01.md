# WAL-013 account runtime and app entry — tests first

Grok grok-4.6 High, no subagents. Read AGENTS.md, TESTING.md, this file,
GROK_BBD_WAL_013_ACCOUNTS_01.md and GROK_BBD_WAL_013_NATIVE_UI_01.md for fixed APIs.
Only read relevant runtime/supervisor/main/build source, protocol/model/launch config,
and authorized tests below, installed eframe0.36.1 APIs, vault format fixtures as needed.
No unrelated history, dependency/config discovery, web or other actors.
Source-only test authorization: NEW test/walletAccountManagement.node.js; bounded
necessary test-first changes to test/{walletBrokerRuntime,walletSupervisorShutdown,
walletStartup,walletStartupSmoke,electronSecurity,walletBrokerBuild}.node.js.
No other writes, production/stubs, execution, formatting, evidence or Git. Independent
Grok actors own account_management.rs and account_native_ui.rs; never modify these or
Cargo/lib. Stop with hashes/counts and a precise list of prior expectations changed.

Later production ONLY social-main.js, wallet-broker/supervisor.js,
wallet-broker/src/runtime.rs, scripts/build-wallet-broker.js. Source actor cannot
redesign custody, protocol, concurrency, env or shutdown semantics below.

## Fixed app/menu/environment behavior

Application menu built on ready contains Wallet / Manage accounts (fixed labels).
Callback only when quitState idle and supervisor.bound true, dispatch('account.manage',{}).
No renderer IPC/preload addition, secret form, payload, address/seed or new navigation.
Not bound/rejection => native Electron dialog fixed title "Accounts unavailable" and
fixed message "The account window could not be opened. Please restart BitBook.".
Catch sync and promise rejection, no raw error. No focus/reopen after quit begins.
Existing ready-once/sender/quit/cache security retained. Menu build mock support may
be added to three existing test harnesses without weakening security assertions.

Configured supervisor env is an own-data-only COPY from process.env of exactly these
allowlisted strings: LANG, PATH, DISPLAY, WAYLAND_DISPLAY, XDG_RUNTIME_DIR, XAUTHORITY,
DBUS_SESSION_BUS_ADDRESS. Reject/omit nonstrings, accessors, embeddedNUL or >4096bytes.
No other variable, secret, DYLD/LD injection, NODE_OPTIONS, HOME, executable override
or raw process.env reference passes. Supervisor independently reapplies same allowlist
and validation. OS GUI routing variables enable the native window/file picker; paths
are not wallet secret inputs. Fixed binary/pin/cwd/emptyargv/shellfalse/3pipes unchanged.
No dependency, package manifest, lockfile or packaging changes. Builder adds only
--features native-ui to fixed Rust1.98 locked/offline/no-default-features bin build;
staging/pinning exact copied bytes and existing safe filesystem behavior unchanged.

## Fixed supervisor methods/shutdown

Add account.manage to BROKER_METHODS; exact empty params/default{} accepted, otherwise
SCHEMA. account.list stays exact empty; account.lock exact valid32lowerhex account_id.
No create/unlock/export/restore wire methods. Existing model remains unchanged; all
capabilities false, degraded state; raw list returns only public AccountSummary array.

Normal quit: cancel tracked intents first as today, stop accepting work/settle pending,
publish down once/detach input data observers, then child.stdin.end() ONCE to signal EOF.
For a still-running child, allow1000ms grace; if not observed exited then SIGTERM,
250ms later SIGKILL; overall shutdown1500ms unchanged, promise resolves ONLY observed
child close, never exit or a successful end(). Clear grace/kill/shutdown timers on close.
Alreadyfailed protocol/transport path retains immediate SIGTERM and250ms escalation.
If stdin.end missing/throws, immediate termination fallback. Repeated quit/shutdown
idempotent including reentrant synchronous close inside end(). No races resolve early,
leave rejection unhandled, fail to reap or produce late signals after observed close.
Normal graceful EOF does not require a wire acknowledgment or expose secrets.
Existing stubborn-child test fixture that ignores EOF must continue to exercise real
SIGKILL fallback. Tests with no end() may keep existing immediate fallback expectations;
new dedicated graceful fixtures prove end, timing, synchronous close and failures.

## Fixed Rust runtime behavior

Use real LocalAccountManager::open(current_dir absolute). Invalid root/store/catalog
never silently appears empty/ready; status/list return closed UNAVAILABLE, no raw paths.
No current_dir or display variable in responses. No native privileged inputs overwire.
No-default build also provides safe persisted account.list/account.lock; account.manage
returns UNAVAILABLE. Headless native-feature process (Linux with neither DISPLAY nor
WAYLAND_DISPLAY) uses same headless path and never initializes GUI. No environment
changes seed generation, custody APIs, security checks or payment capability.

Share real manager Arc<Mutex<LocalAccountManager>> with SharedAccountPort from account_ui.
Wire list/status/lock uses try_lock: busy/poisoned => UNAVAILABLE promptly, never block
2sec request deadline on native crypto/file dialog. On poison recover ONLY to lock_all.
Status/sync.subscribe exact{} => {v:1,broker:"degraded",accounts:[AccountSummary...]};
account.list exact{} => AccountSummary array (<=256); account.lock exact{account_id}
=> {} after actual service lock; service failure maps to closed wire UNAVAILABLE.
Native manage exact{} calls windowcontrol.request_open; true=>{},false=>UNAVAILABLE.
Malformed method params => SCHEMA. Unsupported receiver/intent remain UNAVAILABLE;
unknown privileged methods=>SCHEMA. Parse/account ID beforeeffects. Preserve strict
JSON duplicate handling, exact envelope, seq/session/replay/deadline/framing/ACKlimits.
Snapshot/list activity never refreshes unlock deadlines. Return no false balances or
payment readiness. Enforce64KiB output frame limit (catalog256 must fit).

Protocol loop uses bounded recv_timeout <=1second, ticks sessions using try_lock even
with no messages. Native event loop stays main thread. With display+nativefeature,
start protocol worker (hello does not wait for GUI), run eframe persistent initially
hidden AccountWindow, registration control enables manage. Only worker owns stdout
framed writer, GUI never logs there. EOF/protocol error locks all and requests GUIquit;
worker completion wakes hidden GUI. GUI fatal error/user programmatic quit requests
workerstop; worker checks <=1second, returns, then locks/drops manager and joins worker.
Detached bounded stdin reader can remain blocked only until process exits as baseline.
No indefinite worker join; never join stdin thread. Root windowclose is hide+lock from
UI contract, so broker remains available and menu can reopen. No lost pre-context quit:
if EOF arrives before GUI registers context, first logic observes quit and exits.
Run return and existing main exit happen only after manager lock/drop. Native startup
failure returns closed diagnostic/failure, not a fake successful management response.
No signal handler/dependency/unsafe code, shell or child GUI command. Platform-native
configuration uses pinned eframe already present; actual platform QA Linux only.

## Tests and exact acceptance boundaries

New Node suite ~6-8 focused groups through real main menu callback and real supervisor
with explicit fake child transport as needed:
1 ready menu registration; callback makes only account.manage{}; unbound/sync/async
failure closed dialog; quit/repeatedready can't open; six-method preload unchanged.
2 own-data env routing passes seven allowed GUI strings, omits secrets/accessors/NUL/
oversized values and never passes process.env identity; keep binarypin/emptyargv.
3 dispatcher accepts manage{} only while bound; rejectssecret-bearing/unlock/restore.
4 graceful EOF order,1000/1250/1500 deadlines, lateclosevs.exit, onepromise/no pending
requests/listeners/timers; synchronous close fromend and endthrow fallback asserted.
5 existing startup proof expects account.list [] in fresh0700root, still degraded;
actual supervisor→compiled Rust child list and safe lock data tested in runtime suite.
6 build stub proves --features native-ui fixedargv, no staging/pin regressions.

Update existing runtime actual-child tests old UNAVAILABLE-list expectations to [];
add persisted canonical testnet fixture under ownedroot/accounts0600 (canonical syntax
fixtures valid for locked metadata only, no fake authentication claim). Verify sorted
metadata/locked, status sameaccounts, restartlocked, actual lock{}, invalidparamSCHEMA,
headless manageUNAVAILABLE, all secret methodsSCHEMA, hostilecatalogUNAVAILABLE;
assert no canary/paths output. Use independent fixture bytes from vault_format shape
and standard JSON serialization; NOT service helper-generated expectations. Existing
framing/EOF/deadline/replay attacks retained. Old account.lock with extra secret now
SCHEMA, documented intentional schema tightening. Preserve cleanup close observation,
listeners/timers/process and ownedfile cleanup, no recursive deletion. Native runtime
window proof follows separate QA, do not claim fakechild proves realGUI.

Hermes expected red later node test/walletAccountManagement.node.js (missing menu,
manage and graceful behavior) plus bounded modified existing regression commands.
Green adds actual Rust build then Node walletAccountManagement, walletStartup,
walletStartupSmoke, walletBrokerRuntime, walletSupervisorShutdown,
walletSupervisorTransport,walletSupervisor,walletBrokerBuild,electronSecurity,
walletPreload,walletLaunchConfig (use actual filenames reviewed at execution).
Falsify menu dispatch suppression + graceful EOF suppression or real runtime listing
suppression, prove boundary tests fail then restore. Preserve inherited security-policy
baseline, run changed native Clippy and audit/Gitleaks under final execution contract.
