# BBD-WAL-006 Prepare Serde Lock Capture 01

Jr Dev — Hermes: offline lock-delta capture for the accepted prepare serde feature.

## Preconditions

- Hermes version: `0.18.2 (2026.7.7.2) · upstream f709bd88 · local 10b6d1a9 (+1 carried commit)`
- Provider: `nous`
- Model: `meituan/longcat-2.0:free`
- HEAD: `1592b6948b9e991ed7f3b391bccc593054dd34e0`
- origin/master: `1592b6948b9e991ed7f3b391bccc593054dd34e0`
- Source baseline: `432e69c0443dd5233609d578b43d5a43d83d2c3d`
- Integration commit: `4be931150583876fabadf5a6ffb52021c791fdb3`
- `git diff --check`: clean, no warnings
- Uncommitted worktree inventory (six paths):
  - `scripts/security-policy.js` (modified)
  - `wallet-broker/Cargo.toml` (modified)
  - `wallet-broker/src/zec.rs` (modified)
  - `wallet-broker/src/zec/store.rs` (modified)
  - `wallet-broker/src/zec/test_support.rs` (modified)
  - `wallet-broker/src/zec/prepare.rs` (untracked)

## Accepted source/manifest/policy identities

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/Cargo.toml` | 82 | `47667b8f1970856096c2451ef70ff562be02ca360facf94b043c71ac30072735` |
| `scripts/security-policy.js` | 2,306 | `2c3d859ddd246b38972c835e604c70bd2bbff7b9266629a9c5c39c1b4d967cea` |
| `wallet-broker/src/zec/store.rs` | 2,048 | `f9f66f98f33b8457c955125b77453be018397ab120f78618d52ed817200fcf34` |
| `wallet-broker/src/zec/prepare.rs` | 963 | `417178e0458a3a13e4f36331b8e17bb92148836631eefbdf1a0786501cd114e3` |
| `wallet-broker/src/zec.rs` | 252 | `1061adff987aefd8a641dfed11e06c85d0bc56ddb39f17a5c95d495d6aea387b` |
| `wallet-broker/src/zec/test_support.rs` | 1,830 | `5a1c30199874ad13eb753deaf83c3080534ba495fdfaf225a3d8d03c6ef1ac77` |

## Sole command

```text
TMPDIR="$PWD/wallet-broker/target/wal006-tmp" CARGO_TARGET_DIR="$PWD/wallet-broker/target/wal006-cargo" rustup run 1.98.0 cargo metadata --manifest-path wallet-broker/Cargo.toml --offline --format-version 1
```

## Result

- Exit code: **0**
- STDERR: empty (no warnings, no errors, no network)
- Only `wallet-broker/Cargo.lock` newly changed

## Lock delta

- Old lock: 5,379 lines, `9a6166ef2b39b47aa41b7a77cc3054dd8aee481f5a198a1ad4e4882111f97f59`
- New lock: 5,381 lines, `5c063b11e2882ab61bf126804cf8c3acef80ff5b4af16ac2fa59fe0ba6c85e01`
- Delta: exactly **2 lines** added, **0** removed

## Exact diff

```diff
diff --git a/wallet-broker/Cargo.lock b/wallet-broker/Cargo.lock
index 185a1a6e..60c50644 100644
--- a/wallet-broker/Cargo.lock
+++ b/wallet-broker/Cargo.lock
@@ -4168,6 +4168,7 @@ checksum = "b5772d71c9be8a8a6ac2117d949c5b224c1b72241bb611d9a3012edcf8af7812"
 dependencies = [
  "getrandom 0.4.3",
  "js-sys",
+ "serde_core",
  "wasm-bindgen",
 ]

@@ -5070,6 +5071,7 @@ dependencies = [
  "schemerz-rusqlite",
  "secp256k1",
  "secrecy 0.8.0",
+ "serde",
  "shardtree",
  "static_assertions",
  "subtle",
```

- `"serde"` added to the `zcash_client_sqlite 0.22.0` dependency array.
- `"serde_core"` added to the `uuid 1.26.0` dependency array.
- No package version, source, or checksum changed. No other path changed.

## ext4 paths

- `wallet-broker/target/wal006-tmp`: ext4
- `wallet-broker/target/wal006-cargo`: ext4

## Negative capability

The offline metadata run added exactly the two expected dependency-edge lines and nothing else. No formatter, Clippy, test, Node, or second Cargo command was run. No production source, manifest, policy implementation, Rust tests, fixtures, or other paths were modified. The six production/policy paths remain unstaged and uncommitted. The lock delta is the only new change.
