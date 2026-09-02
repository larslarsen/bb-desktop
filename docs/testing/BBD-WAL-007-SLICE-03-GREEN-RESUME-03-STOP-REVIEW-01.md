# BBD-WAL-007 Slice-3 Green Resume 03 Stop Review 01

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Result: **VALID STOP — ONE-EXPRESSION COMPILE CORRECTION REQUIRED**

Hermes's selected bootstrap-policy falsification command stopped on Rust E0502 at
`wallet-broker/src/xmr/test_support.rs:2035`. In `http_response`, the call
`body.resize(body.len() + target - response.len(), b' ')` mutably borrows `body` as the
receiver while also immutably borrowing it to compute the requested length. The test
therefore did not reach the required runtime policy failure.

Hermes restored the temporary production falsification before stopping. Reviewer audit
proves `HEAD == origin/master == e30ec51b`, the index is clean, `git diff --check` is
clean, and the accepted source identities remain exact: `rpc.rs` is 1,896 lines with
SHA-256 `2186f6fd56cfcce1edfbe00cd247ef2ab2177ba04dfd9b505416a1da33cbe4ed` and
`test_support.rs` is 2,703 lines with SHA-256
`0d7082233a638da3ea7bdf5ee6574cec28347e18cb532161c84dea43dcca1b8d`. Hermes created
no evidence and performed no staging, commit, or push.

The minimal repair is to compute the identical target body length in a local before the
`resize` call, then pass that local to `resize`. This changes borrow timing only. Body
bytes, total response sizing, loop behavior, faults, tests, production RPC behavior,
APIs, dependencies, and compiler settings remain frozen.
