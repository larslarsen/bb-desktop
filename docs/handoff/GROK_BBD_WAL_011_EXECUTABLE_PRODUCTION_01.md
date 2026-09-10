# WAL-011 Rust executable source 01

Actor: Grok Build, grok-4.6 High. Source only, no execution, Git or subagents.
Protected parent: reviewer publication directly after 9ce6e669; one CURRENT-only
launch commit may follow. Read AGENTS.md, TESTING.md, active CURRENT prefix, ticket,
this handoff and only the fixed executable-contract section of the executable test
handoff. No historical reload or runtime/config/credential/home discovery.

The actual expected red is accepted: all nine runtime groups failed through
requireBrokerPath because the binary is absent, test exit 1. The supervisor's 77
accepted transport checks are retained. Implement the already fixed executable
contract now. Existing custody/native libraries remain unconnected in this slice.

## Authorized paths and frozen inputs

Only writable paths (both currently absent):
- wallet-broker/src/main.rs
- wallet-broker/src/runtime.rs

Verify these read-only identities before editing:

| Path | SHA-256 |
| --- | --- |
| test/walletBrokerRuntime.node.js | a49c3e0c49ff664997f78222d979e20603d9a2e9985735c6aa07c5c02308f39e |
| wallet-broker/supervisor.js | 1ac92493cb70cd4ea5e1884f4b96424c7fbc89126308299e7221f2b0ba1a8ff8 |
| wallet-broker/protocol.js | 79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4 |
| wallet-broker/Cargo.toml | 73e5e585eb2fd1ca867d66962460cfa010443886a4b59d8a33556ef2a4ff3503 |
| wallet-broker/Cargo.lock | b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71 |
| wallet-broker/src/lib.rs | 08dd09d23a8c18cdb9a50968ade153a2118b60132f2b7b66a36c6913596de925 |

These files may be read but never modified. Tests remain frozen. Preserve pending
npm/policy changes and both WAL-009/WAL-011 evidence drafts. No other source reads
are needed beyond the named inputs and the two new files. If a required input is
missing or mismatched, report and stop; no repair/discovery detour.

## Implementation boundary

main.rs is the automatic Cargo binary target and declares its own runtime module.
Use #![forbid(unsafe_code)] and pinned existing serde/serde_json/sha2/getrandom APIs.
No manifest, lockfile, library-module or dependency changes. Do not import the
wallet library into the binary just to bring account functionality into scope.
No new test-only modes, fixtures, settings, command-line arguments or environment
overrides. The binary must not access wallet files, create files/directories, open
listeners, launch subprocesses or contact services. OS entropy for the nonce is
allowed. Keep the main-thread native-loop requirement available for later work;
do not attempt native windows or account composition in this slice.

Implement the exact fixed executable contract in
GROK_BBD_WAL_011_EXECUTABLE_TESTS_01.md, including all behaviors rather than only
those exercised by the nine-group suite:

- Child-first flushed hello, actual decimal PID and fresh 16-byte nonce in lowercase
  hex. Exact ack fields, fresh transcript-derived SHA-256 session using the frozen
  protocol.js preimage. No claim of OS parent-identity attestation.
- An absolute two-second ack deadline that covers incomplete headers and bodies.
  A dedicated stdin reader with a bounded channel (capacity one is sufficient) is
  allowed. Never join a blocked reader when terminating. Return/exit promptly on
  malformed input, failed entropy, failed writes/flushes or deadline. No panic as
  normal protocol rejection. Keep diagnostics fixed and generic.
- Four-byte big-endian framing, nonzero body at most 65536 bytes checked before
  allocation, strict UTF-8, exactly one JSON object, no duplicate decoded member
  names at any depth, no trailing JSON. A recursive Serde visitor/seed can retain
  duplicate detection; deserializing straight into Value loses duplicate keys.
  Explicitly bound nesting (128 or stricter) and keep buffered input bounded.
  Correctly handle split/coalesced input and distinguish clean EOF from partial EOF.
- Exact request envelope, v=1/kind=req, object params, lowercase 32-hex ID, exact
  session, positive consecutive parent sequence, integer expires_ms. Reject wrong
  session/shape/sequence/kind and duplicate IDs by closing with numeric nonzero exit.
  Valid expired requests receive TIMEOUT and consume their normal sequence/ID.
  Child response sequence is independent and increments once per emitted reply.
  Avoid arithmetic wraparound. Retain only IDs needed for replay rejection, never
  request params or secrets after dispatch. To bound this development session's
  replay set, close before accepting a 4097th unique request; never evict an ID and
  allow it to replay. This is a resource bound, not durable replay storage.
- Bootstrap status.get and later status.get/sync.subscribe with empty object params
  return exactly {v:1,broker:'degraded',accounts:[]}. No fabricated ready, locked,
  loaded accounts, unsolicited events or durable subscriptions.
- account.list/account.lock/receiver.fresh/intent.begin/intent.cancel return fixed
  UNAVAILABLE for object params. No custody parsing or calls. Unknown methods return
  SCHEMA; nonempty status.get/sync.subscribe params return SCHEMA. Copy only the
  fixed error literals from protocol.js (including retryable), never input details.
  Structural violations close; valid expired requests get TIMEOUT before dispatch.
- Stdout is framing only. Flush each complete reply. Clean EOF after a session may
  return zero; incomplete frame, malformed ack/protocol and ack timeout must exit
  nonzero without a signal. Drop transport-local data on exit. No environment keys,
  endpoint selection, signing, broadcasting, account creation or native UI.

## Stop and subsequent execution

Author only the two source files. Use separate read/search/hash/line-count and
HEAD/status commands; no history/logs or chained shell commands. Do not run Node,
Rust, Cargo, rustfmt, syntax checks, tests, scanners, npm, network tools or another
actor. Do not write implementation evidence or perform Git mutations.

Report both paths, SHA-256/line counts, framing/parser/deadline/exit design and any
contract limitation; stop without execution. Hermes owns later compilation,
format/lint, actual-Rust session falsification, focused green, evidence correction
and integration after reviewer source acceptance. No Rust cryptographic proof
suite replay or package-policy detour is authorized by this source task.

## Collected source review and Source Correction 01 — 2026-09-09

Outer 4288 collected on owner done, exit 0. Session
a6f2c27a-175b-4f44-a414-6698acefa08b reports the two-path drop:

| Path | SHA-256 | Lines |
| --- | --- | --- |
| wallet-broker/src/main.rs | 19b1651d88d85e5d968597eb26eeb62f76a11af56eaa311d60a55d3ef282736d | 7 |
| wallet-broker/src/runtime.rs | 828ab74b3a72d6ea8dcf229665bb1b584c47af8a2c4f7bd3ddd8e12b500fbf0b | 692 |

All six frozen inputs match. The exact-ID Markdown transcript export shows edits
only to these two source paths and no execution or Git mutation. Full CURRENT,
oversized test-handoff reads and directory listings exceed the intended narrow
read scope; Correction 01 must use the named prefix/section and source regions.
The export is a tool summary, not a raw tool-result audit.

The bounded channel, frame-length checks before allocation, strict UTF-8 and
recursive duplicate detection, fixed transcript hash, consecutive sequences,
4096-ID bound, fixed error replies, degraded snapshot and detached-reader exit
design are present. Custody/native libraries and inputs remain untouched.

Decision: reject for two source-visible compile/warning blockers:

1. runtime.rs:547, field(), has both object and name reference inputs and an
   elided reference return. Rust lifetime elision cannot select the owner. Bind
   the return explicitly to object, with no name lifetime coupling:
   `fn field<'a>(object: &'a [(String, JsonValue)], name: &str) -> Result<&'a JsonValue, ProtocolClose>`.
2. runtime.rs:61/64, Bool(bool) and Array(Vec<JsonValue>) store fields never read
   anywhere. They will produce dead-code diagnostics in the planned warning-denied
   gate. Retain their distinct JSON type tags without unused payload storage.
   Bool becomes a unit variant; visit_bool ignores its argument and returns that
   tag. Array becomes a unit variant; visit_seq still fully traverses every child
   through the same depth-incremented ValueSeed and propagates every parse error,
   but drops each parsed child instead of collecting a vector. Do not replace the
   recursive visitor with IgnoredAny or weaken nested duplicate/depth validation.

These are source-review findings, not observed compiler/test results. No command
that loads or compiles production/test code was run. No behavioral success is claimed.

Correction 01: Grok 4.6 High, same session, runtime.rs only. Protected parent is the
reviewer publication following 5485ffac; one CURRENT-only launch may follow. Verify
the two drop hashes and six frozen input hashes first. Fix exactly the lifetime,
two enum variants and corresponding bool/sequence visitor bodies. A discarded
sequence element must still be obtained through next_element_seed(ValueSeed), and
the unchanged nesting check must precede traversal. No warning suppressions, dummy
reads, parser bypasses, other semantic changes or additional files.

Read only active CURRENT prefix, this final review section, workflow docs if needed,
the two source files and named input hash checks. Separate HEAD/status, read/search,
hash and line-count commands only. No history, directory discovery, tests/builds,
formatter/syntax/lint, network/dependencies, docs/evidence or Git mutation. Main and
tests remain frozen. Report the runtime hash/lines and bounded changes, then stop.
Hermes execution/integration remains closed pending source acceptance.
