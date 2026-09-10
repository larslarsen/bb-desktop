# BBD-WAL-011 — Connect the app to the native wallet broker

Status: Launch resolver integrated and accepted; packaging and main startup remain.
Reviewer: Codex, High. Source: Grok Build, grok-4.6 High. Execution: Hermes only
after reviewer authorization. Baseline: 8020a6d47201f804f5503abd3307092ae5a50c43.

At the original baseline, the app registered wallet IPC without starting its
supervisor, the supervisor lacked real framed request/reply transport, and Rust
provided only library components. The transport and degraded Rust executable are
now integrated; Electron now waits for resolved replies but still lacks broker startup.
The objective is a real app-to-broker connection with truthful
status, followed by native wallet flows. Existing library acceptance is retained.

The first bounded change implements live framed child-process transport and actual
request/reply completion. A real child-process fixture proves the pipe boundary;
it is not a wallet implementation or proof of app startup. Subsequent steps supply
the Rust executable and pinned startup configuration, then connect Electron's async
handlers and shutdown. Native windows stay owned by Rust. No signing, broadcasting,
endpoint choice, package-policy expansion or wallet-completion claim is authorized.

Completed transport test paths (closed):
- test/walletSupervisorTransport.node.js (new)
- test/fixtures/wallet-broker/transport-child.js (new)
- test/walletSupervisor.node.js (adapt existing cases to the fixed async contract)

Completed transport production path: wallet-broker/supervisor.js (closed).
Reuse wallet-broker/protocol.js framing/session validation and wallet-pay/model.js
snapshot sanitization unchanged. No dependency or renderer-privilege changes.

The [source handoff](../docs/handoff/GROK_BBD_WAL_011_TRANSPORT_TESTS_01.md) fixes
handshake, startup, reply, error, resource and test semantics. Initial red command:
`node test/walletSupervisorTransport.node.js`. Expected red is the missing real
framed handshake/reply connection, not a missing module or broken fixture.

Targeted green: the same command. Affected regression commands:
`node test/walletSupervisor.node.js`, `node test/walletBrokerProtocol.node.js`,
`node test/walletPreload.node.js`, `node test/walletPay.node.js`, and
`node test/electronSecurity.node.js`. Hermes must falsify response correlation by
temporarily suppressing matching-response settlement; the real-pipe success case
must fail for the missing result, then restore exact production bytes. Detailed
execution/integration instructions follow source review; none are authorized now.

Completed execution is the focused expected-red handoff
[HERMES_BBD_WAL_011_TRANSPORT_RED_01.md](../docs/handoff/HERMES_BBD_WAL_011_TRANSPORT_RED_01.md).
Its collected review accepts the observed missing-transport failures. Grok completed
[production implementation](../docs/handoff/GROK_BBD_WAL_011_TRANSPORT_PRODUCTION_01.md),
including bounded harness corrections. The collected source review permits
[focused validation/integration](../docs/handoff/HERMES_BBD_WAL_011_TRANSPORT_GREEN_01.md).
That completed task is closed; its collected acceptance retains 77 passing checks.
The corrected executable test source is accepted in
[the executable handoff](../docs/handoff/GROK_BBD_WAL_011_EXECUTABLE_TESTS_01.md#correction-02-acceptance--2026-09-09).
The [initial executable expected red](../docs/handoff/HERMES_BBD_WAL_011_EXECUTABLE_RED_01.md)
is accepted from the saved exit-1/nine-failure result. Its evidence was corrected
from the exact saved transcript during integration; no test replay.
The corrected source in the
[Rust executable handoff](../docs/handoff/GROK_BBD_WAL_011_EXECUTABLE_PRODUCTION_01.md)
is accepted for [focused validation](../docs/handoff/HERMES_BBD_WAL_011_EXECUTABLE_GREEN_01.md).
Resume 01 passed build, warning-denied Clippy, session-check falsification and all
nine runtime groups after exact source restoration/rebuild. The
[five-path integration](../docs/handoff/HERMES_BBD_WAL_011_EXECUTABLE_INTEGRATION_01.md#collected-integration-acceptance--2026-09-09)
is accepted and pushed at 31a6e54095a0b5519f8ce0b03833ac8862ddf32f. Hermes is closed.
No validation replay. App startup/account/native composition and broader release
acceptance remain future work.
The executable source review records the remaining emergency-cleanup limitation.

The active [async IPC test contract](../docs/handoff/GROK_BBD_WAL_011_ASYNC_IPC_TESTS_01.md)
accepts Grok's test/electronSecurity.node.js drop. It tests delayed reply
settlement, cloned resolved values and propagated failures through handlers actually
registered by social-main.js. The
[observed red](../docs/handoff/HERMES_BBD_WAL_011_ASYNC_IPC_RED_01.md#collected-expected-red-acceptance--2026-09-09)
is accepted: 21 passing groups, two missing-thenable failures, Node exit 1.
The [one-line Grok production change](../docs/handoff/GROK_BBD_WAL_011_ASYNC_IPC_PRODUCTION_01.md#collected-source-acceptance--2026-09-09)
is accepted: resolved replies are cloned; synchronous validation remains intact.
[Focused validation](../docs/handoff/HERMES_BBD_WAL_011_ASYNC_IPC_GREEN_01.md#collected-validation-acceptance--2026-09-09)
passed all 23 Electron and six preload groups. One-line reversal produced exactly
the expected failures, then restored Electron green passed all 23.
[Four-path integration](../docs/handoff/HERMES_BBD_WAL_011_ASYNC_IPC_INTEGRATION_01.md#collected-integration-acceptance--2026-09-09)
is accepted and pushed at 3aa5e3d844feac54eece517d5c4226dd8608452e. Its source,
validation and integration actors are closed.

The next bounded prerequisite is
[awaitable supervisor shutdown](../docs/handoff/GROK_BBD_WAL_011_SHUTDOWN_TESTS_01.md).
Existing quit() returns before child closure, so main cannot yet wait for cleanup.
Corrections 01 and 02 are accepted: signal readiness, event-time assertions,
close-wait cleanup, preserved cwd after incomplete reaping and explicit fake
Promise/timer cleanup. Test source is frozen at 69eb8bcb (983 lines, six Linux
groups); the wrapper stays at 0d8fbfa8.
[Observed expected red](../docs/handoff/HERMES_BBD_WAL_011_SHUTDOWN_RED_01.md#collected-expected-red-acceptance--2026-09-09)
is accepted: zero ok/six absent-API failures, Node exit 1, unchanged inputs.
The [Grok supervisor source](../docs/handoff/GROK_BBD_WAL_011_SHUTDOWN_PRODUCTION_01.md#collected-source-acceptance--2026-09-09)
is accepted at c1410bfc (631 lines).
[Focused validation](../docs/handoff/HERMES_BBD_WAL_011_SHUTDOWN_GREEN_01.md#collected-validation-acceptance--2026-09-09)
passed six shutdown, seven transport and 13 supervisor groups, premature-completion
falsification and restored six-group shutdown green.
[Five-path integration](../docs/handoff/HERMES_BBD_WAL_011_SHUTDOWN_INTEGRATION_01.md#collected-integration-acceptance--2026-09-09)
is accepted and pushed at 1e45d6d2bf78df6b998cc2062604f23dd1252061. Its actors
are closed; no shutdown implementation or validation replay is authorized.
The integrated shutdown() returns one shared Promise, preserves
existing quit/cancellation/250 ms escalation, resolves on observed child close or
absence, and rejects TIMEOUT at 1500 ms if closure remains unconfirmed. Production
source, validation and integration are accepted. The handoff fixes fake-clock and real-process tests, targeted commands,
affected lifecycle regressions and early-completion falsification.

Startup pin provenance remains as specified in architecture section 4.2: reviewed
packaging creates the pin before runtime, without runtime self-pinning or arbitrary
environment-selected executables. Private storage is a wallet-broker child directory
under Electron userData. Platform artifact inventory and startup composition
require a later reviewer contract.
The main process now invokes shutdown() for normal quit; it does not start a pinned broker.
This completed shutdown prerequisite adds no package
content or runtime configuration. Tests use a Node child fixture, not a coin binary.
The whole ticket remains incomplete; no app startup or release acceptance is claimed.

The active [Electron normal-quit test contract](../docs/handoff/GROK_BBD_WAL_011_ELECTRON_QUIT_TESTS_01.md)
authorizes Grok High to edit only test/electronSecurity.node.js. Seven new groups
must exercise the real registered before-quit handler: synchronous prevention,
one awaited shutdown, repeated/reentrant requests, completion before resumed quit,
pre-ready no-child completion, shutdown failure and throwing error-dialog cleanup.
Failure keeps normal quit blocked and attempts one fixed native error notification;
later quit events neither retry the permanently rejected Promise nor repeat the box.
Existing window-all-closed platform behavior and all 23 security/IPC groups remain.
Expected red and targeted green: node test/electronSecurity.node.js. Affected
regression: node test/walletPreload.node.js. Falsify by resuming app.quit before
shutdown settles, then restore and repeat targeted green. All execution awaits
separate source acceptance/Hermes authorization. Later production is social-main.js
only; broker startup, packaging and native flows remain outside this test contract.
The first quit test drop (a8ca18c7, 1977 lines, 30 groups) required Correction 01:
the compound dialog callback can swallow its own nested-event assertion, and test
hooks are restored before asynchronous cleanup finishes. The active handoff fixes
both corrections in the same test path. Corrected source is accepted at 7f759f81,
2026 lines/30 groups. Only the [Hermes expected-red handoff](../docs/handoff/HERMES_BBD_WAL_011_ELECTRON_QUIT_RED_01.md)
now authorizes one Electron suite run: expected 23 existing passes/seven missing
before-quit-handler failures. No production or integration authorization follows
until the observed red is accepted. That red is now accepted: 23 existing passes,
seven absent-handler failures, Node exit 1, unchanged inputs and matching evidence.
Hermes is closed. The [Grok production handoff](../docs/handoff/GROK_BBD_WAL_011_ELECTRON_QUIT_PRODUCTION_01.md)
authorizes only social-main.js to await shutdown through the fixed normal-quit gate.
Tests, execution, evidence and integration stay frozen pending production review.
Initial production fe116de5 (211 lines) needs one bounded correction: remove the
non-Promise success fallback and keep subscription inside the synchronous failure
guard. Only social-main.js is authorized under the appended production handoff.
Correction c7687b52 (205 lines) is accepted: the fallback is removed and shutdown
subscription shares the failure guard. Grok is closed. Only the
[Hermes focused validation handoff](../docs/handoff/HERMES_BBD_WAL_011_ELECTRON_QUIT_GREEN_01.md)
authorizes 30 Electron groups, six preload groups, selected premature-quit
falsification and exact restoration followed by 30-group Electron green.
No production repair or integration is authorized before collected acceptance.
Collected validation is now accepted: 30 Electron and six preload groups passed,
selected early-quit falsification failed at the intended assertion, exact main
restoration was verified, and restored Electron green passed all 30 again.
Only the [four-path Hermes integration](../docs/handoff/HERMES_BBD_WAL_011_ELECTRON_QUIT_INTEGRATION_01.md)
is authorized next, without source/evidence rewrite or validation replay.
That integration is accepted and pushed at 21e2e4d546adb06321e820ea77a52147fa47a71a:
exact main, test and two evidence hashes match committed bytes. Normal quit waits
for closure, holds failure and handles re-entry through the approved gate.
All actors are closed; no result is pending. Pinned startup, private data-directory
setup, startup failure/status behavior and real application composition require
the next reviewer contract. Later startup must avoid spawning during pending or
completed quit. Native account flows and whole-ticket/release acceptance remain
incomplete. No validation replay or source change is currently authorized.

The next active [launch configuration contract](../docs/handoff/GROK_BBD_WAL_011_LAUNCH_CONFIG_TESTS_01.md)
authorizes only Grok's new test/walletBrokerLaunchConfig.node.js. It fixes a
four-field build-generated manifest under resources/wallet-broker, fixed per-platform
executable names, strict pin/identity validation and a private userData child path.
The resolver reads the expected pin; existing supervisor code still verifies bytes
before spawn. Five filesystem fixture groups precede the disconnected production
module. Expected red/green: node test/walletBrokerLaunchConfig.node.js; affected
regression: node test/walletSupervisor.node.js. Falsify target-identity enforcement.
No execution yet. The handoff also records missing runtime JS in all three current
packagers. Build-time pin generation/inventory repair and main startup composition
are later contracts; no runtime self-pinning or existing package repair is implied.
Initial launch test source 32409f6a (752 lines, five groups/58 rows) is held for
two fixture corrections: make the symlink target valid JSON, and make unsupported
option identities agree with their manifests. This isolates the intended rejection
mechanisms. Only the same test path is writable; no execution or production yet.
Correction 1b6a7d8c (851 lines, five groups/58 rows) is accepted: symlink target
is otherwise valid and unsupported target identities agree with their manifests.
Grok is closed. Only the [Hermes expected-red handoff](../docs/handoff/HERMES_BBD_WAL_011_LAUNCH_CONFIG_RED_01.md)
authorizes one new suite run, expecting five explicit absent-resolver assertions.
No production stub, broader execution or integration is authorized.
Observed expected red is accepted: zero ok/five missing-resolver assertions,
Node exit 1, unchanged inputs and production still absent. Hermes is closed.
Only the [Grok resolver production handoff](../docs/handoff/GROK_BBD_WAL_011_LAUNCH_CONFIG_PRODUCTION_01.md)
authorizes new wallet-broker/launch-config.js. The module remains disconnected
from main and packaging until later contracts; tests/evidence stay frozen.

Existing package-policy failures and pending npm edits are recorded separate work;
they are not green or waived release checks. This transport stage changes no graph,
package content, renderer privilege or network endpoint. Final application/release
acceptance still requires the repository security gates on its final inputs. Do not
rerun Rust cryptographic proof suites for this JavaScript transport change.

Resolver production drop 4a639b16 is held for uncaught reflection exceptions at
the public error boundary. Only the [reflection regression tests handoff](../docs/handoff/GROK_BBD_WAL_011_LAUNCH_CONFIG_REFLECTION_TESTS_01.md)
is authorized. Production is frozen pending focused red; no validation or
integration is authorized yet.

Launch resolver integration accepted at 05a0ffdac4d32f0c842035132b6f3da5ad33b4c1.
[Integration acceptance](../docs/handoff/HERMES_BBD_WAL_011_LAUNCH_CONFIG_INTEGRATION_01.md)
records nine resolver groups/62 rows, 13 supervisor groups, two detected
falsifications, exact restoration and final green. Resolver component complete.
Remaining ticket scope: package runtime inventory, build-generated final-artifact
pins, main startup/status/quit composition and applicable package proof.
