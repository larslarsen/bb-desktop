# BBD-WAL-007 Slice-3 Source Review 02

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Result: **REJECTED — BOUNDED TEST/PRODUCTION CORRECTION REQUIRED**

No formatter, test, build, policy command, or product binary was run. `git diff --check`
was clean. The worktree contained exactly the five authorized source paths, retained
all 15 named tests, and preserved the frozen process source.

Reviewed source identities:

- `wallet-broker/tests/xmr_rpc.rs` — 614 lines —
  `721636f1c4d8851c811193b74fa2085ce6d60f89d2ce540e68132f383de2e336`;
- `wallet-broker/src/xmr.rs` — 5 lines —
  `92af416c0ed6c5f2f8a6ff7011facb622da5e3f1ba8adaa2e96f600c09f31411`;
- `wallet-broker/src/xmr/model.rs` — 151 lines —
  `23a85dc216839cb936161845b52cacdca68a24181e5d31afeaaba30e62c77ca9`;
- `wallet-broker/src/xmr/rpc.rs` — 1,733 lines —
  `fa8b955d4ce63e07e5e07216c0f665ebb8027e62c18cfa7418447be65ca847ed`;
- `wallet-broker/src/xmr/test_support.rs` — 2,526 lines —
  `6894c757ea0a173b6d33d4780df9d2af97f57a64952d458e5e7359828b731f18`.

The frozen `wallet-broker/src/xmr/process.rs` remained 1,184 lines and SHA-256
`7533a86344c178ed25143d21bc3961a72ea7a1fcf85067f90b57134178928a1f`.

## Accepted corrections retained

- Wallet readiness now requires exact pinned RPC value `65567` and `release=true`,
  owns the ten-second monotonic deadline, and preserves the distribution-proof/RPC-
  proof bridge without claiming the CLI string came from RPC.
- Complete pinned wallet `get_balance` and `create_address` shapes are type-checked,
  while only sanitized typed values cross the core.
- The daemon fixtures now carry the base access-response fields, and the node policy no
  longer rejects target height zero/below current height or a future/disabled fork.
- Requests use closed typed values; unimplemented authority entries reject before the
  transport. Recording support delegates through the production request, framing,
  parser, and policy core and clears retained response/authentication observations.

## Blocking findings

1. `parse_node_info` requires `get_info.version` to be an unsigned integer and the fake
   supplies `196608`. Pinned v0.18.5.1 declares this member as `std::string`, and the
   official daemon RPC documentation likewise defines it as a software-version string.
   The current parser therefore rejects the real reviewed wire shape while the fake
   passes a synthetic incompatible one.
2. `block_weight_limit` and `block_weight_median` are serialized with
   `KV_SERIALIZE_OPT(..., 0)`. The pinned serializer omits either member when its value is
   zero. The current exact-key equality rejects those legal responses. The parser must
   accept the exact required inventory plus either optional member, validate it as
   `u64` when present, and continue rejecting every unknown member.
3. `hard_fork_info` declares `version` and `voting` as `uint8_t`, and `window`, `votes`,
   `threshold`, and `state` as `uint32_t`. Production currently accepts every one as an
   unconstrained `u64`; the successful fake also uses undocumented state `4` although
   pinned semantics define only states 0, 1, and 2. Exact typed parsing must reject
   width overflow and unknown state values. The documented default query with empty
   params remains accepted; this review does not require a request-shape change.
4. `SystemHttpPort` marks every `TcpStream::connect_timeout` error as
   `last_failure_not_listening`, and the recording port also treats a connect timeout as
   retryable. The authorized rule is narrower: only connection refusal before any
   successful exchange in that readiness attempt proves a transient not-yet-listening
   child. Timeout, permission/address errors, and refusal after a successful Digest
   challenge must stop immediately with `UNAVAILABLE`, not restart until the deadline.

Correction 02 is limited to the RPC test, production RPC source, and recording support.
The accepted `xmr.rs`, model, and process identities are frozen. Hermes execution and
integration remain unauthorized.
