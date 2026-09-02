# BBD-WAL-007 Slice-3 Green Resume 02 Stop Review 01

Reviewer: Lead Engineer/Reviewer — Codex at High

Result: **VALID STOP — TWO-PATH COMPILE CORRECTION REQUIRED**

Hermes's selected bootstrap-policy falsification command compiled the corrected fixture
far enough to expose two further errors in the accepted drop:

- `wallet-broker/src/xmr/rpc.rs`: the cnonce call passes `&Zeroizing<[u8; 16]>` to
  `hex_bytes`, which requires `&[u8]`;
- `wallet-broker/src/xmr/test_support.rs`: `apply_json_fault` assigns through `body`
  while the match expression used as the assignment's right-hand side also clones
  `body`, producing overlapping mutable and immutable borrows.

This was again a compile failure rather than the required runtime policy-test failure,
so Hermes stopped. It restored the temporary production falsification first. Reviewer
audit proves `HEAD == origin/master == ae89c562`, the index is clean, `git diff --check`
is clean, and the accepted source identities remain exact: `rpc.rs` is 1,896 lines with
SHA-256 `3f1f14972265fc79906c1f0f56f35b3ac55a2d68ffec7c0b91dbbea75a60c0b6` and
`test_support.rs` is 2,702 lines with SHA-256
`fbee13d0a646966359dd408bc8e9b6ab672c47ffbb81ea1c8fa5ac2bbaac7e80`. Hermes created
no evidence and performed no staging, commit, or push.

The bounded corrections are mechanical Rust type/borrow repairs. The cnonce source must
remain inside its existing `Zeroizing` owner while the inner fixed array is viewed as a
slice. `apply_json_fault` must finish computing the same replacement bytes before the
single assignment through `body`. No cryptographic, JSON-fault, production-policy, test,
API, dependency, or compiler-setting behavior is reopened.
