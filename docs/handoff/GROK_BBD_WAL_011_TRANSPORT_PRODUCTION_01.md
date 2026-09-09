# WAL-011 live transport implementation 01

Actor: Grok Build, grok-4.6 High; no subagents. Source only, no execution or Git.
Protected parent: reviewer publication after ccb47280, one CURRENT-only launch allowed.
Read AGENTS.md, TESTING.md, active CURRENT/ticket and this handoff. Read the fixed
production behavior in GROK_BBD_WAL_011_TRANSPORT_TESTS_01.md and collected review in
HERMES_BBD_WAL_011_TRANSPORT_RED_01.md. No historical reload.

The test-first phase is complete: six focused groups failed against frozen source,
including actual child startup with no framed handshake, missing child-exit handling,
and object writes instead of Buffers. Implement that established contract now.
Bounded harness refinements below accompany implementation; they do not add wallet
features or restart the already-observed red phase. Do not execute any command that
loads production/tests. Hermes owns subsequent execution and falsification.

## Authorized paths and baseline

Only writable paths:
- wallet-broker/supervisor.js — 2634fd116f476998db2cf4a4e948fcd864397d0d26d3fd98e4c2a5e41d2f0430
- test/walletSupervisorTransport.node.js — 284546977bd0a1a11ddc92a9fa743c06884e5157029759dd5003001891510b8b
- test/fixtures/wallet-broker/transport-child.js — 17cdea3eaec6c01f839eb34c86e25d1fd93f8cad165cd8e9fb7f108bb9fa7fef
- test/walletSupervisor.node.js — db02281f9314e382255945d569de4f83be8bc0edbc3ff62e5485f11e3d4e4992

Verify these before editing. The unchanged protocol.js hash is
79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4.
Reuse its strict codec, session validation and error normalization unchanged.
Reuse the Pay snapshot sanitizer unchanged. Preserve all npm/policy edits and both
untracked prior evidence records. No docs, manifest, lock, Rust, main/preload, UI,
package scripts or other files may change. No npm/Cargo, syntax/tests/formatter,
scanner, network, actor or Git mutation. Read/search/identity commands only.

## Implementation requirements

Implement the previously fixed framed-byte transport, bootstrap status request,
correlated Promise dispatch, 32-request bound, two-second deadlines and idempotent
shutdown. Keep closed method/parameter and spawn pin/environment/private-directory
guards. The existing supervisor remains the sole transport; no parallel legacy
object-write path or new generic RPC API. receiveProtocol is a test seam into the
same handler used for decoded stdout. All actual writes are length-prefixed Buffers.
Use CONTROL_FRAME_LIMIT with stream:'protocol'. Enforce outgoing frame size before
writing/consuming a sequence number. Keep bounded outstanding work under backpressure;
do not add an unbounded write queue or treat write(false) as an immediate pipe error.

Start installs stdout data/end/error, stdin error, stderr data/error and child
error/exit/close handling. Every EOF closes even if the decoder buffered a partial
frame; protocol.js need not gain an EOF API for that. Do not buffer diagnostics.
After hello validation, send ack then bootstrap status.get id/seq 1. Public bound
stays false until that request's matching response. Earlier events must not clear
the handshake timer or publish ready/syncing state. Normalize and consume bootstrap
errors without creating an unhandled internal Promise rejection.

The default parent PID is process.pid; the default nonce is fresh
crypto.randomBytes(16).toString('hex') per start. Retain injected parentPid/nonce for
deterministic tests and the existing validation. Spawn still requires explicit
brokerPath, expectedSha256 and dataDir; no environment-selected binary or self-pin.

Maintain pending request IDs and per-direction sequence validation independently.
On matching success, clear that deadline and settle exactly once; status results
are sanitized, other results cloned. Error replies expose fixed normalized fields
only. Unknown/duplicate/session-invalid responses close. A public timeout rejects
that request TIMEOUT and other pending work UNAVAILABLE, then closes. Never replay.
The main-process dispatch change does not authorize forwarding raw responses to
Electron; app async handler/startup wiring remains the next stage.

Teardown rejects outstanding promises, clears deadlines, drops decoder/session
references, detaches data listeners and publishes down once. Preserve temporary
error sinks during stream teardown so late errors do not crash the host. Observe
actual child exit for termination accounting; kill() success is not proof of exit.
SIGTERM first, SIGKILL after 250ms only while still alive; clear escalation on exit.
Duplicate close/quit must not send another signal or create a second timer. Bound
quit attempts tracked intent cancellation before termination and consumes those
Promises. It must also terminate when cancellation dispatch fails or the request
limit is full. A started/failed instance never spawns a second child.

## Bounded test-harness corrections

Keep every existing test and independent child framing/session oracle. Refine the
existing cases before completing production, without changing intended outcomes:
- The 32-request limit case must attach rejection handlers, close its supervisor,
  await all 32 rejections, assert no pending IDs/deadlines, and simulate observed
  child exit to release escalation timers. Put fake cleanup in finally where an
  assertion could otherwise leave work outstanding.
- Prove real supervisor-owned quit/exit cleanup inside a successful live-pipe case:
  call quit, await actual child exit, assert down/unbound/no pending requests, before
  finally's emergency kill. Add fixture support to hold a public request if needed
  to prove quit rejects it. Finally must remain a fallback for assertion failures,
  but must not swallow failure to reap the child or count its own SIGKILL as success.
  Remove the unique temporary directory only after observed exit.
- Consecutive writes can coalesce in OS pipes. Keep that real-pipe coverage, and
  additionally deliver explicit header/body slices through fake stdout data events
  to prove the supervisor routes partial bytes through its decoder. Assert no ack
  until the complete hello, then complete bootstrap and a distinct real result.
- Assert broker errors contain no diagnostic sentinel in any own property including
  stack, and no unexpected stack/debug/seed fields copied from the wire; the local
  Error stack is allowed. Require the normalized LOCKED retryable:false field.
- Retain deadline and request-limit checks. Bound all async waits so a broken
  implementation fails instead of hanging. Keep 1.5s success checks if adequate;
  do not lengthen protocol deadlines to make tests pass.

Stop with a four-path source drop, hashes/line counts and a concise description of
the transport and test corrections. No execution, evidence editing or integration.
Hermes will run the focused suite, regression suites and response-settlement
falsification after reviewer source acceptance. No Rust proof or policy detour.
