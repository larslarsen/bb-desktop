# BBD-WAL-007 Slice-3 Source Review 03

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Result: **ACCEPTED FOR HERMES FOCUSED GREEN**

No formatter, test, build, policy command, or product binary was run by the reviewer.
The worktree contains exactly the accepted five-path Slice-3 test/production drop,
`git diff --check` is clean, all 15 named RPC tests remain, and the frozen process
source is byte-identical.

Accepted identities:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/tests/xmr_rpc.rs` | 691 | `67b745f4e951ad9acf473ca71153b99acd4ba5d3a387257e906de617e9052b49` |
| `wallet-broker/src/xmr.rs` | 5 | `92af416c0ed6c5f2f8a6ff7011facb622da5e3f1ba8adaa2e96f600c09f31411` |
| `wallet-broker/src/xmr/model.rs` | 151 | `23a85dc216839cb936161845b52cacdca68a24181e5d31afeaaba30e62c77ca9` |
| `wallet-broker/src/xmr/rpc.rs` | 1,789 | `0659a851a74f5fb5b62236d15feb5b74b82bd56e792b3eb31ad4a1c3bed74326` |
| `wallet-broker/src/xmr/test_support.rs` | 2,676 | `fdb5655e2531be8ef81f4f7254099c940cde02641df023aa4550ed710edad2c3` |

Frozen identity:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/xmr/process.rs` | 1,184 | `7533a86344c178ed25143d21bc3961a72ea7a1fcf85067f90b57134178928a1f` |

The production transport is a fresh, synchronous safe-Rust `TcpStream` to internally
constructed numeric IPv4 loopback endpoints. It retains fixed request/response,
connect/read/write, JSON nesting, HTTP framing, redirect, and no-DNS/proxy/TLS/generic-
method bounds. Wallet calls still require exactly one unauthenticated Digest challenge
and at most one authenticated exchange. Credentials, challenge components, digest
intermediates, authorization bytes, raw responses, and parsed raw strings retain their
bounded zeroizing ownership and sanitized public error surface.

The wallet identity gate now requires exact RPC `65567` plus `release=true` and bridges
that authenticated result to the already-verified executable capability without
misstating the upstream response. Startup owns the complete ten-second deadline and
retries only a pre-response `ConnectionRefused`; timeout, other connect failure, or
refusal after a successful challenge stops immediately.

The shared production/recording core enforces the reviewed full wallet response shapes,
closed typed request values, and the exact daemon inventories. `get_info.version` is a
bounded string; its two default-zero block-weight members are independently optional
without permitting any unknown member. Hard-fork integer widths and state domain are
exact. Node acceptance is limited to the ticket's network/status/offline/untrusted/
current-bootstrap/height policy; target ordering and future/disabled-fork gates remain
absent. Mainnet and unlisted methods reject before transport, and only sanitized typed
values cross the parser.

No account, store, receiver, real-local-gate, Electron, Node, ZEC, dependency, manifest,
lockfile, or cross-repository behavior is accepted by this review. Hermes must still
prove formatter cleanliness, the required bootstrap-policy falsification, focused RPC
green, and the named regressions before integration.
