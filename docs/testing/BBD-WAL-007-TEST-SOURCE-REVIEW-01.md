# BBD-WAL-007 Test-Source Review 01

Decision: ACCEPTED FOR DEPENDENCY RESOLUTION AND EXPECTED-RED EXECUTION

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Source baseline: `c8b52ae69db8a563a0daf87af74bc09bd3ce97e2`

Protected governance HEAD at review: `433dbf9ba1233535a350569e803dbe3a60b176b8`

## Accepted uncommitted source identity

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/Cargo.toml` | 113 | `84e0e4eac1d64d10128334163b0ddbeaf2721aff429828929425e342d4573456` |
| `wallet-broker/tests/native_surface.rs` | 664 | `349f3a019a0e7a5c37aaae727192b0ddcecb02192a5fae6df9291eaf1357276d` |
| `wallet-broker/tests/xmr_distribution.rs` | 257 | `b17603919a4db88ff585e96ae590cbc7101687d787d9604de6a16599607d3e46` |
| `wallet-broker/tests/xmr_process.rs` | 336 | `0a2ed9cb452015861bf0b66a13a788c8221609be5de7880b3a454a37a3c97f17` |
| `wallet-broker/tests/xmr_rpc.rs` | 398 | `bd355d31d0ae64736e14f412293d54190d1c701f1cff2252b16d7ca03001ee18` |
| `wallet-broker/tests/xmr_account.rs` | 537 | `049eabad90979fcdbe3555460c047d8237900f9d8dc1d7ade7049996654afc3e` |
| `wallet-broker/tests/xmr_receiver.rs` | 553 | `e880fd2b3dfeadf412c2e44b85c17f9d7fd4d67ed691f07420bd09035b4d07cd` |
| `wallet-broker/tests/xmr_hygiene.rs` | 281 | `5c94c9452fea5229fdefc0568088ad4f95ee4ac61111fface59109540da87374` |
| `wallet-broker/tests/xmr_local_gate.rs` | 477 | `b3d558421cb0eb81aa13e525a9ea2cbb85e19c41425f927da050b6cc48935f1e` |
| `test/securityPolicy.node.js` | 3,067 | `41988b598fb73afd10eade38dd97527fd1db31ca430a6760bdde701a400da0fb` |

The listed Rust files contain 97 tests in total: 17 native-surface tests and 80 XMR
tests. BBD-WAL-007 adds 84 of those Rust tests. The Node policy file contains 86 tests,
including four BBD-WAL-007 additions. The complete ticket adds 88 named tests.

`git diff --check` is clean. The source actor changed exactly the ten authorized paths;
no production source, lockfile, fixture, package, workflow, Electron/Node production,
ZEC source, or other repository changed.

## Review outcome

The test source freezes the exact Linux release and inner-binary provenance, normal and
portable explicit selection, native-only authority, fail-closed enrollment and launch,
the complete authenticated wallet-RPC process plan, bounded Digest/HTTP/JSON transport,
fixed local-node and no-bootstrap policy, software/watch-only custody and recovery,
independent viewing state, and durable replay-safe subaddress issuance.

The high-value negative cases are non-vacuous: executable, record, process, node,
authentication, account-identity, database, receiver, teardown, and secret-hygiene
failures assert no returned authority, alternate endpoint, leaked secret, live child, or
uncommitted address. The closed capability inventory grants XMR viewing/private receive
only and keeps spend, broadcast, ZEC, hardware-device, mainnet, raw-RPC, remote-node,
download, and generic path authority absent.

The feature-gated real test owns its environment lookup, exact scratch leaf, official
binary verification, offline stagenet `monerod`, child reaping, and exact cleanup. No
ordinary test can run it, and no production feature conditional or extra XMR module is
reserved. Its execution remains prohibited during expected red.

The review corrected two genuine specification ambiguities in ticket governance:
receiver schema version is exactly `1`, and `hardware_backed` remains an unsupported
account kind rather than an invented capability field. Synthetic addresses are
invalid-checksum 95-byte fixtures and the mnemonic is explicitly non-spendable; no real
wallet, address, seed, installation path, or mainnet material is present.

## Remaining boundary

Neither Sol nor the reviewer ran Cargo, Rust, Node, npm, tests, formatters, builds,
dependency resolution, binaries, wallets, nodes, or the real gate. Compile and formatting
validity, the `md-5` lock/source/license/build-script delta, and expected red remain
unproved. Hermes alone may perform the exact Phase-B handoff.

Production source, the real local-Monero gate, falsification, broader regression,
packaging, mainnet, funds, hardware, and every other repository remain unauthorized.
