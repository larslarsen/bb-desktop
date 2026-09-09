# WAL-011 Rust executable boundary tests 01

Actor: Grok Build, grok-4.6 High; no subagents. Tests only, no execution or Git.
Baseline: 5aada7eff5df4568232cd284dadd097c713a9c23 plus reviewer publication and
one CURRENT-only launch. Read active CURRENT, ticket, AGENTS.md and TESTING.md.
No historical reload, runtime/config/credential discovery or home-directory reads.

## Purpose and scope

The JavaScript supervisor now passes real-pipe tests. Rust still has no executable.
Add tests for the actual compiled bitbook-wallet-broker process using that accepted
supervisor plus independent direct-pipe protocol cases. This first executable
provides truthful process availability, not an account service or a spend feature.
Do not add fake accounts or imply that the existing custody libraries are connected.
Electron startup and native/account service composition follow this executable.

Only writable path: test/walletBrokerRuntime.node.js (new). Do not change any existing
test, fixture, source, Cargo manifest/lock, docs or pending npm/policy files. Source
reads may include wallet-broker/protocol.js, supervisor.js, Cargo.toml, Cargo.lock,
src/lib.rs and the accepted transport tests/fixture. Do not copy the large existing
test harness wholesale; keep this suite focused on the new executable boundary.

Frozen supervisor hash: 1ac92493cb70cd4ea5e1884f4b96424c7fbc89126308299e7221f2b0ba1a8ff8.
Frozen protocol hash: 79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4.
Verify before writing. Read/search/hash only; no syntax, tests, build, npm, Cargo,
network, scanner, evidence, Git mutation or actor commands.

## Fixed executable contract for subsequent implementation

Future production paths are wallet-broker/src/main.rs and src/runtime.rs (new).
main.rs owns a standalone binary module; no manifest, dependency or library-module
change is required for Cargo's automatic binary target. Existing pinned serde,
serde_json, sha2 and getrandom may be reused. No unsafe code. This stage is Linux
development execution; no packaged OS artifact or native UI claim.

The binary takes no arguments, uses inherited stdin/stdout framing and stderr only
for fixed generic diagnostics. It must not read/write wallet files, open listeners,
launch processes or contact services. Its supplied private cwd stays unchanged.
No environment-supplied keys, protocol settings, endpoints or fixture modes.

Send child-first hello with exact v1 protocol fields, actual decimal PID and fresh
16-byte nonce as 32 lowercase hex. Flush every frame. Accept exact ack fields and
derive session with the existing transcript's domain/PID/nonce SHA-256 preimage.
Parent PID comes from ack; this is not an OS peer-identity attestation. Ack has a
two-second deadline, including partial ack bytes. Incomplete/malformed ack, input
EOF or protocol violation terminates promptly with nonzero exit; clean EOF after
a valid session may exit 0. A reader thread feeding a bounded channel may implement
the deadline; process exit must not wait forever for a blocked stdin reader. Keep
the main-thread native-event-loop requirement available for later UI composition.

Frames: four-byte big-endian length, one UTF-8 JSON object, 64 KiB control cap.
Reject zero/oversize length before body allocation; reject invalid UTF-8, duplicate
JSON names at any depth, trailing JSON and unknown envelope fields. Retain bounded
parser depth. Split headers/bodies and coalesced requests work. No protocol logs on
stdout. Requests have exactly v,id,seq,kind,method,params,session,expires_ms, v=1,
kind=req, lowercase 32-hex id, session equal to the derived 64-hex value, positive
integer seq starting 1/increasing by one per parent frame, integer deadline in ms.
Reject duplicate request IDs, wrong session/sequence/kind/shape by closing. This
read-only stage accepts req only; cancellation frames cannot grant authority.

The first valid request is the supervisor bootstrap status.get. Respond with
{v:1,id:<request id>,seq:1,kind:'res',result:{v:1,broker:'degraded',accounts:[]},session}.
Child sequence increases independently for each response. A later status.get or
sync.subscribe with empty params returns the same snapshot. No ready/locked/account
state is fabricated. No unsolicited events or durable subscription state yet.

account.list, account.lock, receiver.fresh, intent.begin and intent.cancel return
fixed UNAVAILABLE errors because their runtime services are not connected. Unknown
methods, including unlock/backup/confirm/sign/broadcast/create, return fixed SCHEMA.
status.get/sync.subscribe require empty object params. For unavailable known methods,
an object params value is sufficient to return UNAVAILABLE; do not parse/store secret
fields or call custody. Valid expired requests return fixed TIMEOUT. Error envelope
is v,id,seq,kind:'error',error:{code,message,retryable},session, using the existing
protocol.js error table verbatim. Error contents never echo input or native internals.

## Test implementation

Use the real executable at wallet-broker/target/debug/bitbook-wallet-broker by
default, with one explicit test-command argument allowing another built binary path.
No download or build from test code. Assert a missing executable with the explicit
diagnostic `native wallet broker executable is missing`; that is the expected red
for the current absent binary, not a broken fixture. Hash the built binary for the
supervisor pin in tests only. Production startup pin configuration remains future work.

Prove these bounded cases, with async deadlines and finally cleanup:
- Start the compiled Rust child through the unmodified createWalletSupervisor with
  real default spawn, actual binary pin and unique private cwd. Wait for bootstrap,
  assert bound and exactly the degraded snapshot, then complete a public status.get.
  account.list rejects UNAVAILABLE, never a fabricated empty successful account list.
  Quit must settle and terminate the actual child. A test-only observer around
  child_process.spawn may capture the returned child while delegating unchanged
  arguments to the original real spawn; restore the observer in finally. No fake
  process or substituted script. Direct-pipe cases also verify the hello's actual PID.
- Directly spawn the same binary, independently encode/decode frames and derive the
  session using Node crypto. Verify actual child PID/hello keys and fresh nonce across
  two starts. Send split ack and coalesced status requests; assert exact IDs, result
  shapes and independently increasing child sequences.
- Malformed/duplicate-key/oversize frames and wrong session or sequence terminate
  promptly. Exercise representative header/body splits without copying the complete
  already-tested JS codec matrix. Keep malformed-input checks outside the supervisor
  so Rust, not the JS validator, must reject them.
- Absent or partial ack hits the two-second deadline; partial-frame EOF exits.
- Unknown authority method returns SCHEMA and an expired status request TIMEOUT.
  Input canaries do not appear in stdout errors or stderr diagnostics. Verify the
  temporary cwd has no files after execution. No real wallet/key/daemon/endpoint.

Use independent expected literals for hello/envelopes/snapshot/errors, not production
sanitizer output as the sole oracle. Do not replace the Rust binary with a JS fixture.
Retain emergency cleanup for assertion failures and fail if the child cannot be
reaped; remove test directories only after exit. No swallowed cleanup timeout.

## Stop and later execution

Report the one-file test drop, SHA-256/lines and cases; stop without execution.
Hermes initial red: `node test/walletBrokerRuntime.node.js` against absent executable.
After red review the production actor supplies main.rs/runtime.rs. Planned green:
`rustup run 1.98.0 cargo build --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --bin bitbook-wallet-broker`,
then `node test/walletBrokerRuntime.node.js`. Existing target directory is disk-backed.
Focused source format/lint and actual-Rust session falsification will be specified
after source review; do not rerun cryptographic proof suites or package-policy gates.
