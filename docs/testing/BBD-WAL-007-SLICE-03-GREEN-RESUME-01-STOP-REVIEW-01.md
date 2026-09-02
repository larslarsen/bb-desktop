# BBD-WAL-007 Slice-3 Green Resume 01 Stop Review 01

Reviewer: Lead Engineer/Reviewer — Codex at High

Result: **VALID STOP — ONE-PATH COMPILE CORRECTION REQUIRED**

Hermes reached the authorized bootstrap-policy falsification after the formatter stage,
but the selected `xmr_rpc` test binary did not compile. Rust reported `recursion limit
reached while expanding $crate::json_internal!` at
`wallet-broker/src/xmr/test_support.rs:1908`. The expected falsification was a runtime
failure of the selected policy test, so the compile failure was the wrong failure mode
and required an immediate stop.

Hermes restored the temporary production mutation before stopping. Reviewer audit proves
`wallet-broker/src/xmr/rpc.rs` is 1,896 lines with SHA-256
`3f1f14972265fc79906c1f0f56f35b3ac55a2d68ffec7c0b91dbbea75a60c0b6`, the protected
governance parent remains `deb69908`, `HEAD == origin/master`, the index is clean, and
the worktree still contains exactly the accepted five-path source drop. Hermes ran no
green, evidence, staging, commit, or push action after the mismatch.

The diagnostic points to the single oversized `serde_json::json!` object used by
`valid_get_info_result`. The correction must remain local to
`wallet-broker/src/xmr/test_support.rs`: split that fixture construction into smaller
equivalent object constructions while preserving its exact fields, JSON types, optional
block-weight behavior, fault behavior, and serialized result. Increasing a crate-wide
recursion limit is not authorized because the fixture can be corrected without widening
the compiler setting. Production RPC behavior and all test assertions remain frozen.
