# WAL-015 helpers and real transport seam after expected RED

Principal Dev primary Sol High. Reviewer inspected COMPILE03 raw outputs/pins and
actual Hermes session20260910_152520_1e0148, CLI exit0. Production library and initial
three test targets compile. Only errors are the three named missing helpers.
Authorize primary changes ONLY test_support.rs, live_transport.rs, account_ui.rs
(removal of now-unused clear_receive_scene only), accounts.rs (cached snapshot
redaction on expiry/lock_all), and zec.rs reexports if needed.
No execution/formatting/Git/dependencies/evidence/test assertion changes.

Implement validate_live_batch_for_test as a thin call to live::validate_batch.
Implement LiveSyncHarness::reopen_via_preparation by deriving root from the capsule's
validated account directory and calling actual prepare_live_path with the bound
account/network/UFVK, then reconstructing the harness through the same reopen path.
Do not repair journals/reset files in the helper. Do not yet change the core's
header/txid guards or hot-journal ordering; next runtime pass must expose these bugs.

Implement probe_live_transport_for_test(endpoint:&str,ca_pem:Option<&[u8]>,
cancellation:LiveCancellation,deadline:Duration)->Result<LiveTransportProbe,ZecError>.
Result derives Debug and exposes tip_height:u32,block_count:usize. Use SAME real
TonicLiveSource through a private connection config helper. Product defaults remain
fixed15sec WebPKI roots. Only explicit hidden test seam may add runtime CA/short
positive deadline. No insecure certificate verifier, ambient roots or fallback.
Drop Option client before bounded runtime shutdown; early failures too. Calls exactly
metadata(GetLightdInfo/GetLatestBlock), raw TreeState(tip-1), blocks(tip,tip).
Use real batch size/shape validator; checked tip/height/hash validation and upstream
TreeState conversion. No fabricated success. Keep RPC bodies account-free.

All waits must use configured deadline for complete stream, not just headers;
map transport/TLS/timeouts UNAVAILABLE, oversized tonic message LIMIT and explicit
cancellation CANCELLED. Avoid new dependencies. Keep runtime-owned DNS clarification.
Return a coherent frozen source drop so Hermes can run actual focused tests.

Redact already-reaped last_live_snapshot when deadline expiry or lock_all leaves its
account locked, not just manual lock or cancellation of a currently active worker.
Preserve sanitized lifecycle/job status needed by the existing cancellation test.
