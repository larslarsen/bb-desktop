# BBD-WAL-007 Slice-3 Compile Correction Source Review 02

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Result: **ACCEPTED FOR HERMES FOCUSED GREEN RESUME 03**

Sr Dev — Grok Build changed exactly two expressions in the two authorized paths and
ran the pinned Rust 1.98 formatter successfully.

In `build_authorization`, `hex_bytes(&source)` is now `hex_bytes(&*source)`. This views
the inner `[u8; 16]` as the byte slice required by `hex_bytes`; the entropy source stays
owned by the same `Zeroizing<[u8; 16]>` for the same lifetime. Entropy acquisition,
cnonce encoding, digest construction, secret ownership, zeroization, authorization
shape, and error behavior are unchanged.

In `apply_json_fault`, the unchanged `match fault` now completes into a local
`replacement` before the single `*body` assignment. This removes the overlapping borrow
without changing any match arm, byte sequence, `None` return, mutation count, or fault
behavior. The local contains only the same synthetic response bytes that the assignment
expression previously attempted to construct.

Reviewer inspection confirms no other expression or path changed, `git diff --check`
is clean, and the three other accepted paths retain their exact identities. Neither
repair widens an API, compiler setting, dependency, capability, secret lifetime, or
failure mode.

Accepted identities:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/tests/xmr_rpc.rs` | 694 | `7b11f6fa6ecf29aa5556f1afc4bb7e3cd5e29018e9ff9eb1f40c5da05c3fafda` |
| `wallet-broker/src/xmr.rs` | 5 | `92af416c0ed6c5f2f8a6ff7011facb622da5e3f1ba8adaa2e96f600c09f31411` |
| `wallet-broker/src/xmr/model.rs` | 151 | `23a85dc216839cb936161845b52cacdca68a24181e5d31afeaaba30e62c77ca9` |
| `wallet-broker/src/xmr/rpc.rs` | 1,896 | `2186f6fd56cfcce1edfbe00cd247ef2ab2177ba04dfd9b505416a1da33cbe4ed` |
| `wallet-broker/src/xmr/test_support.rs` | 2,703 | `0d7082233a638da3ea7bdf5ee6574cec28347e18cb532161c84dea43dcca1b8d` |

This acceptance is limited to Hermes execution. Hermes must independently prove the
formatter, exact runtime falsification and restoration, focused green, regressions,
evidence, and integration before final Slice-3 acceptance.
