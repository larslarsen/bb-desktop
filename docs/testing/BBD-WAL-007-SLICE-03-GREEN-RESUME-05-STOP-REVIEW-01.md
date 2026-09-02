# BBD-WAL-007 Slice-3 Green Resume 05 Stop Review 01

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Result: **VALID STOP — ONE-LITERAL TEST ORACLE CORRECTION REQUIRED**

Hermes proved the Rust 1.98 formatter check at exit 0 without mutation and reached the
full `xmr_rpc` green command. That command ran all 15 tests: 14 passed and
`closed_typed_rpc_allowlist_rejects_every_unlisted_method_immediately` failed at line
620. Hermes stopped before every later green command, evidence, staging, commit, or
push. The temporary bootstrap-policy line was restored; reviewer audit proves `rpc.rs`
is 1,896 lines with SHA-256
`2186f6fd56cfcce1edfbe00cd247ef2ab2177ba04dfd9b505416a1da33cbe4ed`.

The retained report did not preserve the selected falsification command's middle
output, so this review does not claim its exact count or failure reason. A fresh Hermes
resume must rerun and record it; no result from Resume 05 may be reused.

The single green failure is a test-observation normalization mismatch. Production
`RpcRequest::CreateAddress` retains the reviewed raw parameter literal
`{"account_index":0,"label":"","count":1}`. `RecordingRpcPort::observe_request`
parses that object into `serde_json::Value` and calls `Value::to_string`; with the
crate's non-`preserve_order` `serde_json` configuration, the observation is serialized
in sorted member order as `{"account_index":0,"count":1,"label":""}`. The other
parameter expectations in the same table already use this normalized ordering.

JSON object member order has no protocol meaning. The minimal correction is to change
only the expected `create_address` observation string to the normalized ordering.
Production request bytes, parameter names/types/values, the closed allowlist, every
other test, dependencies, and runtime behavior remain frozen.
