# BBD-WAL-007 Test-Source Review 03

Decision: ACCEPTED FOR RESUMED EXPECTED RED

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Protected governance HEAD: `f327ee0f`

Supersedes the seven XMR Rust-test hashes in:
`BBD-WAL-007-TEST-SOURCE-REVIEW-02.md`

## Accepted source identity

| Path | Lines | Named tests | SHA-256 |
| --- | ---: | ---: | --- |
| `wallet-broker/Cargo.toml` | 113 | — | `189f067739afd8bc1e775e4d58b3ce7d4cfa6ab53e7b7acc5b02737396c5bff5` |
| `wallet-broker/tests/native_surface.rs` | 664 | 17 total / 4 WAL-007 | `349f3a019a0e7a5c37aaae727192b0ddcecb02192a5fae6df9291eaf1357276d` |
| `wallet-broker/tests/xmr_distribution.rs` | 297 | 12 | `481042e6dabe705fb7771c0cf692f0561298a9e0e8aa6d0cfd1d8f2ab9deb6f8` |
| `wallet-broker/tests/xmr_process.rs` | 370 | 12 | `f515181aea504fa76d739ad88ba6c8055df3abd8e768766480482f01ac3b42ce` |
| `wallet-broker/tests/xmr_rpc.rs` | 419 | 15 | `59b25c6fcb5c42b7bc72135479dced2f244c5e2bc5e7fe2e871a87076ee07ddd` |
| `wallet-broker/tests/xmr_account.rs` | 582 | 16 | `b87eb4e3d6cdd1941906705337ff34c3a1d899d715309dfc057f4b98912189e5` |
| `wallet-broker/tests/xmr_receiver.rs` | 588 | 15 | `d8656c533d7b2d08eb9fa8831cd70da6e1da632ed9563fc2af569cab754d3622` |
| `wallet-broker/tests/xmr_hygiene.rs` | 327 | 9 | `effa4a223c53d422c6cba97f1430d6e9b61141651e4df1227a132212921cddf7` |
| `wallet-broker/tests/xmr_local_gate.rs` | 458 | 1 gated | `aa70a6dec2b165257cef8da69770ec0649c1299d0e05d471706ebddf61568ed0` |
| `test/securityPolicy.node.js` | 3,067 | 86 total / 4 WAL-007 | `ca7cc722c058870362bebb8c706e29a28a918711e0feed9680053b4f24e23d9d` |

## Corrective review

Hermes's formatting stop was correct. Sol verified every pre-repair hash, then invoked
only standalone Rust 1.98.0 rustfmt on the seven authorized XMR files. The complete
80-test XMR inventory and every test name are unchanged. The manifest, native-surface
test, and Node policy remain byte-identical to Review 02. Source inspection found no
semantic repair, production addition, or path widening; `git diff --check` is clean.

The resolved lockfile SHA-256 is
`29a5e4a8b9fa40ca612f3b7c3b1f90d527544a519c06e50bc11aae7803f57420`.
Relative to protected HEAD it adds exactly the root `md-5` edge, one
`md-5 0.11.0-pre.4` package record, and the activated `zeroize` dependency edge in each
of existing `block-buffer 0.11.0-rc.3` and `digest 0.11.0-pre.9`. No existing package
version, checksum, or source changes. Those two feature-driven edges correct the overly
narrow delta stated in Resume 01.

No reviewer execution occurred. Sol ran no test, build, Cargo, Node, npm, network, or Git
operation. Hermes alone may run the corrected focused expected-red gate. Production and
the real local-Monero gate remain unauthorized.
