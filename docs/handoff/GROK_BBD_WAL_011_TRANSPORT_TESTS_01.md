# WAL-011 live wallet transport tests 01

Actor: Grok Build, grok-4.6 High; no subagents. Test source only.
Read AGENTS.md, TESTING.md, active CURRENT prefix and BBD-WAL-011. No historical
reload. Protected baseline: reviewer publication following 8020a6d4, with one
CURRENT-only launch record allowed. No execution, Git, evidence or production edits.

Only writable paths: test/walletSupervisorTransport.node.js,
test/fixtures/wallet-broker/transport-child.js, test/walletSupervisor.node.js.
The first two are new. Existing supervisor tests SHA-256:
82d5a7a9e352697bf4fe32871e808bd804201b3ecf85d5a592440f684065a16c.
Verify identities before editing. Frozen production supervisor SHA-256:
2634fd116f476998db2cf4a4e948fcd864397d0d26d3fd98e4c2a5e41d2f0430.
Frozen protocol SHA-256:
79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4.
Preserve every other pending edit/untracked evidence. Read/search/hash commands
only; no Node execution, test/syntax command, npm, Cargo, network or actor launch.

## Fixed production behavior to test

Retain the spawn path/hash, private-directory, environment, no-shell and method
allowlist checks. start() keeps its synchronous {ok,snapshot} result. Production
must attach real stdout data/end/error, stdin error, stderr data/error and child
error/exit/close handlers. Reuse the existing strict decoder and frame encoder;
every stdin write must be a Buffer. Drain diagnostics without forwarding/retaining
them. Partial/coalesced chunks must work. EOF, malformed/oversize protocol input,
session/sequence/PID mismatch, stream error and child exit fail closed.

After validating child-first hello, write the framed ack, create the session and
send one internal status.get request (parent seq 1, id 1 encoded as 32 hex digits,
params {}). This bootstrap bypasses only the not-yet-bound public dispatcher gate.
Its matching response makes both directions bound and publishes the sanitized
result snapshot. The result is a snapshot object directly, not {snapshot:...}.
Accept an earlier valid child event as session traffic but do not publish a ready
snapshot until bootstrap succeeds. The existing two-second handshake deadline
must cover hello through bootstrap completion; clear it only on that completion.
No public request is accepted before bootstrap success. Bootstrap error closes.

After bootstrap, dispatch(method,params) returns a Promise that settles with the
matching response result, or rejects with a fixed normalized broker error. Retain
synchronous parameter/authority validation errors. IDs and per-direction sequences
remain independent, increasing and session-checked. ReceiveProtocol remains an
object-fed test seam, routed through exactly the same envelope handler as bytes.
No synthetic {ok:true} result for public requests. Clone results across callers;
status.get results and sync.subscribe snapshots use the existing sanitizer.
Other response payloads remain main-process data; this stage does not authorize
forwarding them to a renderer or changing Electron IPC handlers.

At most 32 public requests may be pending; reject the next with LIMIT without a
write. Each request has a two-second deadline. A timed-out request rejects TIMEOUT
and closes the session; other pending requests reject UNAVAILABLE. This deliberately
avoids accepting late replies in a partially expired session. pendingRequests()
returns a copy of pending public request IDs only. No replay or automatic restart.
Out-of-order matching replies work; unknown/duplicate responses fail closed.

close/quit/exit are idempotent: publish broker-down, settle pending requests once,
clear deadlines and release stream handlers. Ensure late error events are safely
absorbed during child teardown. Bound quit may best-effort send tracked intent
cancellations before teardown; it must handle their promises without unhandled
rejections. Send SIGTERM, escalating to SIGKILL after 250ms only if the child has
not exited; clear that timer on exit. Prevent duplicate start from spawning another
child. No silent restart of a failed supervisor; a new instance starts a new session.

## Tests that exercise the actual connection

Use an async test runner with nonzero failures and guaranteed finally cleanup.
Use real child_process.spawn(process.execPath,[fixture,...test mode arguments])
inside a test-only system.spawn adapter. Independently assert that the supervisor
requested a pinned binary, zero arguments, shell:false, private cwd, filtered env
and three pipes before the adapter launches the fixture. Hash process.execPath
for the test pin. Fixture arguments are test controls, never production API.
Temporary test directories must be unique, private and removed after child exit.
The fixture uses its actual PID and independently builds/parses four-byte BE JSON
frames and derives the session hash with Node crypto; do not import production
framing/session helpers as its oracle. No real wallet, key, daemon or network.

Cover these meaningful cases, combining related assertions where clear:
- Real framed hello/ack/bootstrap and a subsequent request return a distinct
  fixture result; verify exact handshake identity, session, IDs and sequences.
  Deliberately split frames across header/body boundaries and coalesce responses.
- Two concurrent requests receive replies in reverse order and resolve to their
  respective distinct results. A broker error is normalized; diagnostic sentinel
  text and unknown error fields never escape in errors/snapshots.
- A wrong session or uncorrelated response closes and rejects pending work.
- Partial-frame EOF and malformed frame input close without a hanging promise.
- No bootstrap response hits the handshake deadline; no public response hits its
  request deadline. Use injected timers for deterministic deadline tests where
  useful; at least success and disconnect must traverse actual child pipes.
- Request limit does not write the rejected request. Quit/exit during pending work
  settles once and leaves no pending requests, active deadlines or surviving child.

Adapt existing object-fed tests to framed output, bootstrap and async replies while
preserving their spawn/authority/sanitization/cancellation assertions. Give fake
streams EventEmitter behavior, decode captured Buffer frames in test code, and
explicitly complete requests. Do not delete tests merely because the contract is
now async. The primary real-pipe test must fail on the current production supervisor
for absent wiring/framing, with an explicit deadline and child cleanup, not hang.

Stop after the three-path test drop. Report changed paths, hashes/line counts,
case names and expected baseline failure. No execution or integration. Reviewer
then hands the focused red command to Hermes, followed by bounded implementation.

## Collected review — 2026-09-09

Grok outer 68774 was collected once on owner done, exit 0; source authorization is
closed. Accept this drop for initial expected-red execution only. The real-pipe
success and reverse-order reply assertions exercise the missing boundary, with an
independent child codec/session oracle. Existing authority cases are retained.
Production supervisor/protocol identities remain unchanged. Transcript inspection
found source edits limited to the three authorized files, with no tests, dependency
operations or Git mutation. Two command batches included extra read-only Git status,
HEAD and log inspection; this does not affect the source result.

Reviewed test identities:
- test/walletSupervisorTransport.node.js: 630 lines,
  284546977bd0a1a11ddc92a9fa743c06884e5157029759dd5003001891510b8b
- test/fixtures/wallet-broker/transport-child.js: 263 lines,
  17cdea3eaec6c01f839eb34c86e25d1fd93f8cad165cd8e9fb7f108bb9fa7fef
- test/walletSupervisor.node.js: 507 lines,
  db02281f9314e382255945d569de4f83be8bc0edbc3ff62e5485f11e3d4e4992

Six transport groups and thirteen supervisor groups exist. This is not final test
acceptance: the limit case leaves its 32 fake requests unsettled, and live cleanup
force-kills in finally without first proving supervisor-owned shutdown. Correct
these harness gaps before green acceptance. Consecutive fixture writes also do not
guarantee distinct OS read chunks; retain real-pipe tests and prove split decoder
delivery deterministically at the stream-event seam. Error assertions should check
that extra diagnostic fields are absent, not only the message. These are bounded
test corrections to carry into the next source handoff, not unrelated new work.

Hermes may now run only the focused transport suite against frozen production and
record the observed failure under HERMES_BBD_WAL_011_TRANSPORT_RED_01.md.
