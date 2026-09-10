# WAL-015 production authorization after reviewed red

Principal Dev Sol gpt-5.6-sol High is authorized for production paths listed in
tickets/BBD-WAL-015.md. Parent remains reviewer. No tests/builds/formatters, Cargo
resolution/fetching, Git, evidence or other actors. Complete the implementation,
including real tonic transport and native wiring, then stop for Hermes execution.

Reviewer inspected raw wal015-live-red-01.json and actual Hermes session
20260910_143940_a2efb3 (nous/meituan/longcat-2.0:free), actor50929 exit0. All three
Cargo commands exited101 solely for missing live APIs: core 2 import errors, manager
19 missing imports/type/method errors, UI 2 import +3 missing-trait-method errors.
All pinned source/test bytes are unchanged. The runner's verbal summary confused
targets; the raw command outputs are authoritative and independently reviewed.

Implement the test-defined API from the three approved files. Test semantics are
frozen. You may remove the new unused CHECKPOINT_HEIGHT constant or unused mut/imports
from those tests, but may not weaken assertions. Report concrete API conflicts instead
of inventing a fake production path. Test support must be a thin wrapper around the
actual generic live scanner and real WalletDb, with actual mutated responses/files.

## Dependency and transport choices

In Cargo.toml, preserve existing pins and add backend feature lightwalletd-tonic
only. Add exact direct pins, defaults=false:
- tonic =0.14.6 with channel,tls-ring,tls-webpki-roots;
- tokio =1.52.3 with rt,net,time,macros;
- prost =0.14.4 with std.
Do not add sync, Tor, transparent queries, tonic transport/server/router or defaults.
Missing crates/lock resolution are Hermes's responsibility after the manifest drop.
If a specific additional feature is needed, explain it to reviewer before adding it.
Suggested editable endpoint is https://testnet.zec.rocks:443, supported by the
operator's endpoint listing. Custom HTTPS remains allowed. No automatic fallback.

Use generated CompactTxStreamerClient::new(channel), not its feature-gated connect
convenience method. Bound connect + each entire RPC/stream to 15 seconds, including
waiting for all block messages. Set max_decoding_message_size and enforce measured
encoded byte and count limits before accumulation/scanning. Empty pool_types requests
legacy shielded data without transparent-address calls. Filter all dynamic diagnostics
to fixed codes. Never install an insecure verifier or use ambient proxy/CA settings.

Cancellation applies to connect, RPC and stream waits via the same cancellation
token. A worker-local current-thread Tokio runtime is suitable. Drop channels/futures
before bounded runtime shutdown. A platform DNS lookup may finish separately inside
Tokio's resolver; it must contain only a hostname, never wallet state/UFVK or database
handles. This is an explicit clarification of the ticket's no-detached-thread rule:
all wallet jobs must be joined, and runtime cleanup must not hang manager/EOF shutdown
waiting for a system resolver. Do not create a detached custom DNS/wallet worker.

## Implementation details fixed by review

Use separate live.sqlite3 as ticketed: the original receive store and fixture cache
remain untouched after account preparation. Factor WAL-014's authenticated helper so
sync preparation can validate/bootstrap viewing storage without fresh_receiver.
The manager's test seam always uses Testnet; only explicit engine test support uses
Local consensus. Worker carries no spending material. No session deadline extension.

The manager prepares/validates storage before starting the network worker; a blocked
metadata test therefore sees a valid 0600 live cache whose UFVK binding can be checked.
The upstream viewing account may be imported once the birthday ChainState arrives.
Support that recognized initial state, but never migrate/reset an arbitrary corrupt
existing cache. Exactly one account, fixed metadata version, owner/mode/nlink/sidecar
checks before SQLite open. No recursive cleanup or broad cache resets.

Use upstream transactionally_with_extension for scan + metadata checkpoint changes.
Validate complete batch shape/continuity before writes. Save target height with the
committed batch, derive snapshots from real upstream summary, and preserve bytes on
failure/idempotent replay. Perform necessary server checkpoint/tip revalidation; tests
allow repeated authorized RPCs. Retain/check the most recent 100 blocks for reorgs.
Fault hooks must interrupt the actual transaction after writes/before commit, not
short-circuit before the mechanism being tested. Byte-budget test seam calls the same
validator used for real decoded stream lengths.

Session expiry detected by list/status/tick must cancel/join work. Reject duplicate
jobs and wrong-job cancellation. Drop cancels/joins before releasing the root guard.
UI snapshots bind account/network/job; wrong/old results are ignored and cannot paint
balances. Clear/cancel on Back/lock/hide/quit and selection change; no network on paint.
Use exact decimal ZEC labels from integers. Keep all existing account/Receive tests
and native software-GLX rendering behavior intact. Tests still need real gRPC boundary
coverage after this first implementation; do not claim full acceptance yourself.

For efficiency, focused upstream/source reads and one coherent implementation drop.
If the manifest is ready before the source, message parent; Hermes can resolve/fetch
its dependencies while you finish source, without touching your source paths.

## Reviewer source split and fixture reuse

Live.rs ownership transfers exclusively to the core actor under
SOL_BBD_WAL_015_CORE_01.md; primary retains other authorized paths. Shared source
metadata now contains raw LightdInfo and BlockId, and tree_state returns raw
TreeState. Validation occurs in the real core before conversion. Authorize the
one-line scan.rs::derive_chain_states crate-private visibility change and direct
incrementalmerkletree =0.8.2, defaults=false, legacy-api solely for upstream fixture
TreeState serialization. This version and feature already occur transitively;
Hermes resolves the new root dependency edge and verifies graph equivalence.

AUDIT01 execution result accepted: zero vulnerabilities, existing atomic-polyfill
warning only. Hermes copied the embedded driver to a temporary script and added
read-only checks instead of the exact launcher. Reviewer compared the executed
script AST with the authorized driver: equivalent. The temporary script was removed
by exact path. This is a workflow deviation, not an unexecuted or failed audit.
