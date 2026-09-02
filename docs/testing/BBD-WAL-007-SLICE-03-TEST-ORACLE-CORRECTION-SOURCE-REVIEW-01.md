# BBD-WAL-007 Slice-3 Test Oracle Correction Source Review 01

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Result: **ACCEPTED FOR HERMES FOCUSED GREEN RESUME 06**

Sr Dev — Grok Build changed only the `RpcMethod::CreateAddress` expected-params literal
in `wallet-broker/tests/xmr_rpc.rs` and ran the pinned Rust 1.98 formatter successfully.
The expectation now uses the sorted member order produced by the existing
`serde_json::Value` observation: `account_index`, `count`, `label`.

Reviewer inspection confirms the production request remains byte-identical and retains
the reviewed raw order `account_index`, `label`, `count`. The test still validates the
exact normalized member set, values, and JSON types. Because JSON object order has no
protocol meaning, this corrects the observer oracle without weakening request authority
or changing production behavior. No assertion, method entry, other test, support code,
dependency, or compiler setting changed. `git diff --check` is clean.

Accepted identities:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/tests/xmr_rpc.rs` | 694 | `a1face1660ca0daf66002671ba3d794058b4acd497de2f0a3e25f2ca57597d0b` |
| `wallet-broker/src/xmr.rs` | 5 | `92af416c0ed6c5f2f8a6ff7011facb622da5e3f1ba8adaa2e96f600c09f31411` |
| `wallet-broker/src/xmr/model.rs` | 151 | `23a85dc216839cb936161845b52cacdca68a24181e5d31afeaaba30e62c77ca9` |
| `wallet-broker/src/xmr/rpc.rs` | 1,896 | `2186f6fd56cfcce1edfbe00cd247ef2ab2177ba04dfd9b505416a1da33cbe4ed` |
| `wallet-broker/src/xmr/test_support.rs` | 2,705 | `2c445494b36fd645240cc9a25405782eff6d3e8187b4762aee6a0fd7ca640a40` |

This acceptance is limited to Hermes execution. Hermes must independently prove the
formatter, exact runtime falsification and restoration, focused green, regressions,
evidence, and integration before final Slice-3 acceptance.
