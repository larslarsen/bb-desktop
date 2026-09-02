# BBD-WAL-007 Slice-1 Node Stop Review 01

Reviewer: Lead Engineer/Reviewer — Codex

Governance parent: `441ac6bb`

Result: **NODE STOP VALID — TWO-PATH POLICY CORRECTION REQUIRED**

Hermes completed the authorized mechanical formatting and the first four restarted gate
commands:

- `cargo fmt --check`: exit 0 with no further mutation;
- `native_surface`: 17 passed, 0 failed;
- `xmr_distribution`: 12 passed, 0 failed; and
- native-feature compile check: exit 0 without warning.

The fifth command, `node test/securityPolicy.node.js`, stopped with three failures that
all descend from one policy rejection. The sixth repository-policy command was correctly
not run. No commit or push occurred.

The precise cause is not the longstanding `eframe` or `rfd` imports described in the
provisional evidence. `checkRustWalletSource` applies its generic `monero` token ban to
`wallet-broker/src/native_ui.rs`; the newly accepted exact picker title
`"Select monero-wallet-rpc"` therefore fails before the WAL-007 checker can accept the
source. The existing broad ban remains valid for arbitrary Monero authority in legacy
wallet source. It needs one closed exception for zero or one exact approved picker-title
literal on the exact native-UI path, with duplicate and alternate Monero text still
rejected.

The formatter changed only the two authorized paths:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/xmr/distribution.rs` | 910 | `4135db83eca151185d8222498d2435ef56e2d564ff17bd3fa88481260758aed5` |
| `wallet-broker/src/xmr/test_support.rs` | 370 | `7819a2d7f20b49540346a16a53d94ea77e7acd21ceb17e48091e746eadc293b7` |

The provisional evidence is 87 lines, SHA-256
`7f13fccc3ac62024bb7a3fb97c42c0974a902189db3c0dbbe9ba56b729334416`.
It correctly preserves the stop chronology and corrected protected hashes, but final
evidence must replace its combined `Provider` value with separately queried provider
and model fields and correct its root-cause description.

No reviewer acceptance command was run. Only the two-path Sol correction in the linked
handoff is authorized. Hermes resume, integration, Slice 2, broader acceptance, and the
real local-Monero gate remain closed.
