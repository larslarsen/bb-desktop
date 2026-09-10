# BBD-WAL-011 — Connect the app to the native wallet broker

Status: Rust source accepted; focused validation Resume 01 authorized after copied-pin stop.
Reviewer: Codex, High. Source: Grok Build, grok-4.6 High. Execution: Hermes only
after reviewer authorization. Baseline: 8020a6d47201f804f5503abd3307092ae5a50c43.

The app currently registers wallet IPC without starting its supervisor. The
supervisor writes JavaScript objects to a byte stream, receives no stream events,
and returns a synthetic acknowledgment. Rust provides library components without
an executable. The objective is a real app-to-broker connection with truthful
status, followed by native wallet flows. Existing library acceptance is retained.

The first bounded change implements live framed child-process transport and actual
request/reply completion. A real child-process fixture proves the pipe boundary;
it is not a wallet implementation or proof of app startup. Subsequent steps supply
the Rust executable and pinned startup configuration, then connect Electron's async
handlers and shutdown. Native windows stay owned by Rust. No signing, broadcasting,
endpoint choice, package-policy expansion or wallet-completion claim is authorized.

Test paths now authorized:
- test/walletSupervisorTransport.node.js (new)
- test/fixtures/wallet-broker/transport-child.js (new)
- test/walletSupervisor.node.js (adapt existing cases to the fixed async contract)

Future production path: wallet-broker/supervisor.js only, after observed red.
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
is accepted from the saved exit-1/nine-failure result. Its evidence draft needs the
collected review's corrections during later integration; no test replay.
The corrected source in the
[Rust executable handoff](../docs/handoff/GROK_BBD_WAL_011_EXECUTABLE_PRODUCTION_01.md)
is accepted for [focused validation](../docs/handoff/HERMES_BBD_WAL_011_EXECUTABLE_GREEN_01.md).
The first gate stopped before any stage after Hermes copied a lockfile pin
incorrectly. Source remains unchanged. Resume 01 extracts the published command
directly and generates its own metadata/evidence. Grok is closed. Hermes alone may
execute the exact Resume 01 launcher once;
integration and broader app/release acceptance remain closed.
The source review records the remaining emergency-cleanup limitation.

Existing package-policy failures and pending npm edits are recorded separate work;
they are not green or waived release checks. This transport stage changes no graph,
package content, renderer privilege or network endpoint. Final application/release
acceptance still requires the repository security gates on its final inputs. Do not
rerun Rust cryptographic proof suites for this JavaScript transport change.
