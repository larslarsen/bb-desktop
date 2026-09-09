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

## Collected source review and Correction 01 — 2026-09-09

Decision: reject the original test drop; authorize Grok 4.6 High to correct only
test/walletBrokerRuntime.node.js. No execution or integration is authorized.
This section supersedes the original new-file baseline and stop state above.
Protected parent: reviewer publication following 3ce81e57c280e59065b1d99f879a7c8edf167c0c;
one CURRENT-only launch commit may follow. Verify the source identities below.

Collected on owner done through exact-ID `grok export` of session
c0b065f2-55aa-422e-b96c-7ab09a016840. Outer 40380 could not be resumed in the current
tool session; its exit code is not claimed. The exported transcript contains the
completion report, six edits to the authorized file, read/search and identity/status
commands. It shows no tests, builds or Git mutations. It also includes unnecessary
git-log history and reads of existing protocol/supervisor tests and the transcript
fixture beyond the named source list. Correction 01 must use only the reads below.
The Markdown export summarizes tools; this is not a raw tool-result audit.

Measured drop: 711 lines, nine top-level groups, SHA-256
f29dcecdd34453fc7413741b3a07534c7b88e10be201679217041c0b3ae9b15e.
Supervisor and protocol retain the two frozen hashes above. The reviewer inspected
source only; no syntax, test, build, acceptance or falsification command was run.
The independent wire literals, actual compiled-child route, session derivation,
degraded-state checks and explicit supervisor quit are retained.

Blocking findings (line numbers refer to the measured original drop):

1. Lines 427–441 accept a resolved null account.list result as though UNAVAILABLE
   was rejected. Replace the sentinel pattern with an explicit rejection assertion
   that fails for every resolution and checks the fixed error fields.
2. Lines 249–288 recursively erase the cwd, suppress removal errors and omit the
   empty-directory assertion on most negative paths. This can hide runtime file
   writes. After every child is reaped, inspect the cwd before cleanup on every
   path, including failures. Remove only the verified empty directory with rmdir;
   never recursively delete a generated target. Preserve unexpected content and
   report the path. Reap, filesystem and cleanup failures must fail the test while
   retaining the original assertion error where applicable.
3. Lines 88–99, 175–189 and the final assertions do not prove complete output:
   unread bytes are ignored, frame waits accept an arbitrary extra tail, and exit
   does not establish that stdout/stderr have finished draining. Await bounded
   child close/stream completion before final assertions. In every direct case
   assert the exact complete transcript (hello and only expected replies), zero
   unread bytes and no canary in either stream, including bytes emitted on shutdown.
   Apply final canary/output validation to the supervisor case too. Preserve
   bounded emergency reaping and restore the spawn observer in finally. Observe
   child spawn errors without unhandled events; failed spawn must fail promptly.
4. Lines 530–534 send invalid UTF-8 that is also invalid JSON after replacement
   decoding. A lossy UTF-8 decoder therefore still passes. Place invalid bytes
   inside a quoted string in an otherwise valid request to a known unavailable
   method, after a proved bootstrap; lossy decoding would then produce UNAVAILABLE
   rather than exit. Assert prompt numeric nonzero exit without a signal for all
   protocol/deadline failures, so crashes/signals cannot stand in for rejection.
5. Lines 611–628 test only two partial header bytes and allow 1–5 seconds for a
   two-second ack deadline. Keep absent/header cases, add a complete header plus
   incomplete ack body held open, and measure from hello observation with monotonic
   time. Use an explicit 1500–3500 ms observation window around the fixed 2000 ms
   production deadline; this is scheduler tolerance, not a new protocol timeout.
   Also cover EOF with an incomplete body, not only an incomplete header.

Bounded coverage completion in the existing method-error group: include
receiver.fresh and intent.cancel with object params returning exact UNAVAILABLE,
and nonempty sync.subscribe params returning SCHEMA. Keep the rest of the suite
focused; do not copy a codec matrix or change the executable contract.

Only reads: AGENTS.md, TESTING.md, active CURRENT prefix, ticket, this handoff,
test/walletBrokerRuntime.node.js, wallet-broker/supervisor.js and protocol.js.
Read/search/hash/line-count and exact HEAD/status queries only. No history reload,
home/config/credential/runtime discovery, actor commands, network, execution,
syntax checks, builds, dependencies, docs/evidence edits or Git mutations.
Only write: test/walletBrokerRuntime.node.js. Preserve every unrelated pending file.
Report the resulting hash, lines, groups and corrections, then stop. Hermes red
remains future work after reviewer acceptance; do not run it.

### Correction 01 incomplete run and resume — 2026-09-09

Outer 9893 returned exit 0 when collected on owner done, but supplied only two
progress messages and no corrected drop. Exact-ID export of session
f53cb3ef-f7c1-4598-a892-f3c6e7871611 ends at a combined baseline-check Execute request.
The export does not establish whether that request ran or why the session stopped.
That request included an unnecessary git-log command despite the narrow read scope.
The test remains exactly f29dcecdd34453fc7413741b3a07534c7b88e10be201679217041c0b3ae9b15e;
supervisor/protocol identities and pending path inventory are unchanged. No result
is accepted and no execution is authorized. This is an incomplete run, not a
completed correction or evidence of a source-design inability.

Resume the same Grok 4.6 High session once from the reviewer publication following
7950ba55; one CURRENT-only launch commit may follow. Use automatic tool approval
instead of acceptEdits for this noninteractive invocation. This is a launcher
adjustment, not a claim that a particular permission denial was observed, and does
not widen authorized commands or paths. Do not repeat history/log discovery.
Allowed shell commands are separate read-only HEAD/status, sha256sum of the three
named files, wc -l of the test file, and bounded reads/searches of the already named
files. No chained shell commands or extra identity/history commands. Read and edit
only within the existing Correction 01 contract. Complete the correction, report
its hash/lines/groups, then stop. No tests, builds, Git mutation or integration.

### Correction 01 source review and Correction 02 — 2026-09-09

Outer 61377 collected on owner done, exit 0, with a completed one-file drop:
test/walletBrokerRuntime.node.js, 947 lines, nine top-level groups, SHA-256
74469b561b8a82155a2dec9a30f129a0f94badd9556d7f00e8921f062cff3da3.
Supervisor/protocol still match their frozen hashes. The exact-ID transcript export
for f53cb3ef-f7c1-4598-a892-f3c6e7871611 shows the resumed run performing named reads,
separate identity/status commands, one test-file edit and its final report. It shows
no tests, builds or Git mutations. Its phrase "no Git operations" means no Git
mutations; read-only Git commands are present. No raw tool-result audit is claimed.

The five Correction 01 repairs and bounded method coverage are present in source:
explicit UNAVAILABLE rejection, empty-only cwd removal, final drained output checks,
non-vacuous malformed UTF-8 input, numeric failure exits, partial-body deadline/EOF
cases and the remaining method-error cases. These improvements are retained.

Decision: reject the complete drop for one remaining call-contract defect.
waitNewFrames at lines 526–528 expects a context with stdout and child properties,
but all seven callers pass ctx.stdout. That array has no stdout property, so the
decoder reaches undefined.map instead of testing real replies. The defect also
existed in the original drop and was missed by the first reviewer pass. An absent
binary red would hide it; source correction is required before executing that red.

Correction 02 actor: Grok 4.6 High, same session, source only, no subagents.
Protected parent: reviewer publication after 08fa1941; one CURRENT-only launch
commit may follow. Verify the 74469b56 test hash and frozen production hashes.
Only write test/walletBrokerRuntime.node.js. Make exactly seven literal changes:
replace `waitNewFrames(ctx.stdout,` with `waitNewFrames(ctx,` at the current lines
642, 698, 750, 833, 846, 890 and 898. Preserve the helper, all other bytes, 947-line
count and nine-group count. Do not redesign, format, execute or add coverage.

Read only active CURRENT prefix, this final section, AGENTS.md/TESTING.md if needed,
and the affected test source. Separate read-only HEAD/status, named source hashes,
line count and call-site searches are allowed; no command chains, history/logs,
runtime/config/credential/home discovery, tests, syntax checks, builds, dependencies,
network tools, evidence edits or Git mutation. Preserve unrelated pending files.
Report the seven replacements, resulting SHA-256/lines/groups, then stop. Hermes
execution and integration and all production edits remain unauthorized.
