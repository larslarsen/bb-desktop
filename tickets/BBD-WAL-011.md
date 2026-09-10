# BBD-WAL-011 — Connect the app to the native wallet broker

Status: Initial shutdown tests rejected; bounded Grok correction authorized.
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
The initial six-group test drop is rejected for a pre-handler SIGTERM race,
assertions escaping asynchronous callbacks, a timeout listener leak and incomplete
teardown. The handoff's Correction 01 authorizes edits only to
test/walletSupervisorShutdown.node.js; shutdown-child.js is frozen.
Future shutdown() returns one shared Promise, preserves
existing quit/cancellation/250 ms escalation, resolves on observed child close or
absence, and rejects TIMEOUT at 1500 ms if closure remains unconfirmed. Production
supervisor edits follow observed-red review; no execution or integration is now
authorized. The handoff fixes fake-clock and real-process tests, targeted commands,
affected lifecycle regressions and early-completion falsification.

Startup pin provenance remains as specified in architecture section 4.2: reviewed
packaging creates the pin before runtime, without runtime self-pinning or arbitrary
environment-selected executables. Private storage is a wallet-broker child directory
under Electron userData. Platform artifact inventory and app startup/before-quit
composition require subsequent bounded contracts; this shutdown task adds no package
content or runtime configuration. Tests use a Node child fixture, not a coin binary.
The whole ticket remains incomplete; no app startup or release acceptance is claimed.

Existing package-policy failures and pending npm edits are recorded separate work;
they are not green or waived release checks. This transport stage changes no graph,
package content, renderer privilege or network endpoint. Final application/release
acceptance still requires the repository security gates on its final inputs. Do not
rerun Rust cryptographic proof suites for this JavaScript transport change.
