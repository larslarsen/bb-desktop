# BBD-WAL-007 Slice-3 Compile Correction Source Review 01

Reviewer: Lead Engineer/Reviewer — Codex at High

Result: **ACCEPTED FOR HERMES FOCUSED GREEN RESUME 02**

Sr Dev — Grok Build changed only `wallet-broker/src/xmr/test_support.rs`. The oversized
`valid_get_info_result` object literal is now two smaller `serde_json::json!` object
constructions. The second object's entries are extended into the first before the
existing optional block-weight removals and `RpcFault` mutations.

Reviewer inspection confirms every prior member name and value expression remains, the
JSON types are unchanged, and the split introduces no production-RPC, public-API,
assertion, fault, optional-member, or serialization change. No recursion-limit setting,
custom macro, helper API, dependency, test, allow attribute, or unrelated cleanup was
added. The other four accepted paths retain their exact identities. `git diff --check`
is clean.

Accepted identities:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/tests/xmr_rpc.rs` | 694 | `7b11f6fa6ecf29aa5556f1afc4bb7e3cd5e29018e9ff9eb1f40c5da05c3fafda` |
| `wallet-broker/src/xmr.rs` | 5 | `92af416c0ed6c5f2f8a6ff7011facb622da5e3f1ba8adaa2e96f600c09f31411` |
| `wallet-broker/src/xmr/model.rs` | 151 | `23a85dc216839cb936161845b52cacdca68a24181e5d31afeaaba30e62c77ca9` |
| `wallet-broker/src/xmr/rpc.rs` | 1,896 | `3f1f14972265fc79906c1f0f56f35b3ac55a2d68ffec7c0b91dbbea75a60c0b6` |
| `wallet-broker/src/xmr/test_support.rs` | 2,702 | `fbee13d0a646966359dd408bc8e9b6ab672c47ffbb81ea1c8fa5ac2bbaac7e80` |

This is acceptance for Hermes execution only. Hermes must independently prove the
formatter, exact runtime falsification and restoration, focused green, regressions,
evidence, and integration. Final Slice-3 acceptance remains XHigh-only.
