# BBD-WAL-007 Test-Source Review 02

Decision: ACCEPTED FOR RESUMED DEPENDENCY RESOLUTION AND EXPECTED RED

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Protected governance HEAD before correction: `4440ef59`

Supersedes the manifest and Node-policy hashes in:
`BBD-WAL-007-TEST-SOURCE-REVIEW-01.md`

## Accepted source identity

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/Cargo.toml` | 113 | `189f067739afd8bc1e775e4d58b3ce7d4cfa6ab53e7b7acc5b02737396c5bff5` |
| `wallet-broker/tests/native_surface.rs` | 664 | `349f3a019a0e7a5c37aaae727192b0ddcecb02192a5fae6df9291eaf1357276d` |
| `wallet-broker/tests/xmr_distribution.rs` | 257 | `b17603919a4db88ff585e96ae590cbc7101687d787d9604de6a16599607d3e46` |
| `wallet-broker/tests/xmr_process.rs` | 336 | `0a2ed9cb452015861bf0b66a13a788c8221609be5de7880b3a454a37a3c97f17` |
| `wallet-broker/tests/xmr_rpc.rs` | 398 | `bd355d31d0ae64736e14f412293d54190d1c701f1cff2252b16d7ca03001ee18` |
| `wallet-broker/tests/xmr_account.rs` | 537 | `049eabad90979fcdbe3555460c047d8237900f9d8dc1d7ade7049996654afc3e` |
| `wallet-broker/tests/xmr_receiver.rs` | 553 | `e880fd2b3dfeadf412c2e44b85c17f9d7fd4d67ed691f07420bd09035b4d07cd` |
| `wallet-broker/tests/xmr_hygiene.rs` | 281 | `5c94c9452fea5229fdefc0568088ad4f95ee4ac61111fface59109540da87374` |
| `wallet-broker/tests/xmr_local_gate.rs` | 477 | `b3d558421cb0eb81aa13e525a9ea2cbb85e19c41425f927da050b6cc48935f1e` |
| `test/securityPolicy.node.js` | 3,067 | `ca7cc722c058870362bebb8c706e29a28a918711e0feed9680053b4f24e23d9d` |

## Correction review

Hermes's first resolution attempt correctly stopped before lock mutation: final
`md-5 0.11.0` requires stable `digest ^0.11`, which Cargo cannot select beside the
Zcash graph's exact compatible-line prerelease `digest 0.11.0-pre.9`.

The correction changes exactly one declaration in the manifest and its exact Node
policy mirror to:

```toml
md-5 = { version = "=0.11.0-pre.4", default-features = false, features = ["zeroize"] }
```

Official RustCrypto metadata for `md-5 0.11.0-pre.4` freezes
`digest = "=0.11.0-pre.9"`, `zeroize = ["digest/zeroize"]`, `build = false`, library
name `md5`, Rust 1.72, and `MIT OR Apache-2.0`. Defaults remain disabled. This should add
one registry package while reusing the already accepted Digest/zeroize closure; Hermes
must prove the exact lock result rather than assume it.

Every Rust test hash is unchanged. The Node file retains 86 tests and differs from the
accepted first review only in the exact dependency string. `git diff --check` is clean;
no production or lockfile changed. The 88-test ticket contract remains accepted.

No reviewer or Sol execution occurred. Hermes alone may resume the lock/source/license/
build-script inspection and focused expected red. Production and the real gate remain
unauthorized.
