# BBD-WAL-006 Support-Dependency Gate Evidence 01

Protected governance parent: `18a9d77d`

## Accepted prior results

The prior Node policy red was accepted without rerun: exit `1`, exactly 71 `ok`,
3 `not ok`, and only the three deferred ZEC policy groups red. The prior offline
metadata command exited `0` but made no lock diff, so it was not treated as
resolution evidence.

## Resume commands and results

All commands used Rust/Cargo 1.98.0 via `/home/lars/.cargo/bin/rustup run 1.98.0`,
offline and without an external target/cache. The authorized non-locked check
exited `0` and finished successfully. Its complete lock diff was exactly:

```diff
@@ -271,7 +271,9 @@ dependencies = [
  "getrandom 0.4.3",
  "hkdf",
  "pczt",
+ "rand_core",
  "rfd",
+ "rusqlite",
  "secrecy 0.10.3",
```

No package block, version, checksum, source, transitive edge, or other lock byte
changed. Both locked/offline feature-tree commands exited `0`. The `rand_core`
tree showed the direct broker `rand_core feature "std"` path plus existing
upstream transitives; the `rusqlite` tree showed the direct package with no
direct feature and the existing transitive `array`, `bundled`, `default`,
`hooks`, `modern_sqlite`, `time`, `uuid`, and `vtab` union. No new package,
network/load-extension/SQLCipher/vendored-OpenSSL feature, or unexpected support
authority appeared.

The exact locked/offline custody command exited `0` and passed all 11 tests:

```text
test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

No warning altered the acceptance claim.

## Resolved inputs and provenance

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/Cargo.toml` | 81 | `6643435bdf59608b09e906c8b7010baf1c17bbaa785d8eb70e37039d4bb37632` |
| `wallet-broker/Cargo.lock` | 5369 | `ac454889b0796afea3f2a2ddfaf8c585bfff1080bec6d1428d0c227534ffb9dd` |
| `scripts/security-policy.js` | 2299 | `60e41a12462d77c6be875f1659e5ef8a86d2b8146bd25d37ec7777297847d767` |
| `test/securityPolicy.node.js` | 2454 | `f8340ae965a1f59e27594153cd3c9f3eca5ee1d8a78ed0a9a685faf6f1dc2647` |

The direct support pins are `rand_core = 0.6.4`, defaults off with `std`, and
`rusqlite = 0.37.0`, defaults off with no direct feature. Both are existing
crates.io packages with checksums `ec0be4795e2f6a28069bec0b5ff3e2ac9bafc99e6a9a7dc3547996c5c816922c`
and `165ca6e57b20e1351573e3729b958bc62f0e48025386970b6e4d29e7a7e71f3f`,
respectively. No new package/build-script/license authority was introduced.

No `wallet-broker/src/zec*` production path exists. ZEC source remains frozen;
No fixture, test, ZEC Rust source, policy beyond the accepted support-dependency
manifest checker, or unrelated path was changed. No network, wallet,
node, device, secret, or canary was used.

## Final state

This gate records the exact support dependency resolution and 11-test custody
result. Address production source review remains required; no ZEC source repair
or implementation is authorized.
